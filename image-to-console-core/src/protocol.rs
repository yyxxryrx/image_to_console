use crate::DisplayMode;

#[cfg(feature = "auto_select")]
/// Automatically detect the terminal protocol based on environment variables and terminal capabilities
///
/// This function attempts to detect which terminal protocol is supported by examining:
/// 1. Environment variables (`TERM_PROGRAM`, `TERM`, `ITERM_SESSION`)
/// 2. Terminal capability queries (for Sixel support when crossterm feature is enabled)
///
/// Detection priority:
/// 1. WezTerm - detected by "wezterm" in TERM_PROGRAM or TERM environment variables
/// 2. Kitty - detected by "kitty" in TERM_PROGRAM or TERM environment variables
/// 3. ITerm2 - detected by "iterm" in TERM_PROGRAM or TERM environment variables,
///             or presence of ITERM_SESSION environment variable
/// 4. Sixel - detected by querying terminal capabilities via escape sequences (requires crossterm feature)
/// 5. Normal - fallback if no specific terminal protocol is detected
///
/// # Returns
///
/// * `Protocol` - The detected terminal protocol
///
/// # Note
///
/// This function is only available when the `auto_check` feature is enabled.
pub fn get_terminal_protocol() -> Protocol {
    use std::io::Write;
    let term_program = std::env::var("TERM_PROGRAM")
        .unwrap_or_default()
        .to_lowercase();
    let term = std::env::var("TERM").unwrap_or_default().to_lowercase();
    if term_program.contains("wezterm") || term.contains("wezterm") {
        Protocol::WezTerm
    } else if term_program.contains("kitty") || term.contains("kitty") {
        Protocol::Kitty
    } else if term_program.contains("iterm")
        || term.contains("iterm")
        || std::env::var("ITERM_SESSION").is_ok()
    {
        Protocol::ITerm2
    } else {
        #[cfg(feature = "crossterm")]
        {
            fn check_sixel() -> std::io::Result<Protocol> {
                use std::io::BufRead;
                crossterm::terminal::enable_raw_mode()?;
                // Send the escape sequence
                std::io::stdout().write_all(b"\x1b[>c")?;
                // Flush the output
                std::io::stdout().flush()?;
                // Wait some time then try to get the result
                std::thread::sleep(std::time::Duration::from_millis(100));
                let (st, rt) = std::sync::mpsc::channel::<String>();
                // Spawn a thread to read the input
                std::thread::spawn(move || {
                    let mut buffer = Vec::new();
                    match std::io::stdin().lock().read_until(b'c', &mut buffer) {
                        Ok(_) => st.send(String::from_utf8(buffer).unwrap_or_default()),
                        Err(_) => st.send(String::default()),
                    }
                });
                // try to get the result
                let p = match rt.recv_timeout(std::time::Duration::from_millis(100)) {
                    Ok(s) => {
                        // The result should be "ESC [ > Ps ; Pv ; Pc c"
                        // So we skip to '>' then skip one again
                        // and take all characters before 'c'
                        let s = s
                            .chars()
                            .skip_while(|&c| c == '>')
                            .skip(1)
                            .take_while(|&c| c != 'c')
                            .collect::<String>();
                        // return the normal if we don't get anything
                        if s.is_empty() {
                            return Ok(Protocol::Normal);
                        }
                        // Parse the args
                        let args = s.split(";").collect::<Vec<&str>>();
                        // The Pc was ignored if the args length is 2
                        // We need Pc argument to determine whether we should use sixel or not
                        if args.len() <= 2 {
                            Protocol::Normal
                        } else if args
                            .last()
                            .map(|s| s.trim().parse::<u8>().unwrap_or_default() & 1)
                            .unwrap_or_default()
                            == 1
                        {
                            Protocol::Sixel
                        } else {
                            Protocol::Normal
                        }
                    }
                    Err(_) => {
                        // return normal if we cannot get the result
                        Protocol::Normal
                    }
                };
                crossterm::terminal::disable_raw_mode()?;
                Ok(p)
            }
            check_sixel().unwrap_or(Protocol::Normal)
        }
        #[cfg(not(feature = "crossterm"))]
        Protocol::Normal
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
/// Terminal protocol for displaying images
///
/// This enum represents different terminal protocols that can be used to display images.
/// Each variant corresponds to a specific terminal or image display protocol.
///
/// # Variants
///
/// * `Auto` - Automatically detect the best protocol based on terminal capabilities (requires `auto_check` feature)
/// * `Normal` - Standard terminal output using ASCII or Unicode block characters
/// * `WezTerm` - WezTerm terminal specific image protocol
/// * `Kitty` - Kitty terminal specific image protocol
/// * `ITerm2` - iTerm2 terminal specific image protocol
/// * `Sixel` - Sixel graphics protocol for compatible terminals (requires `sixel` feature)
pub enum Protocol {
    #[cfg(feature = "auto_select")]
    Auto,
    Normal,
    WezTerm,
    Kitty,
    ITerm2,
    #[cfg(feature = "sixel")]
    Sixel,
}

impl Default for Protocol {
    /// Returns the default protocol, which is `Normal`
    ///
    /// The default protocol uses standard terminal capabilities with ASCII or Unicode
    /// block characters to display images.
    fn default() -> Self {
        Self::Normal
    }
}

#[derive(Clone)]
/// Builder for constructing a DisplayMode based on protocol and options
///
/// This builder allows configuring how images should be displayed based on
/// the terminal protocol and desired display characteristics like full/half
/// block rendering and color/grayscale output.
///
/// # Fields
///
/// * `protocol` - The terminal protocol to use for display
/// * `is_full` - Whether to use full block characters (true) or half block (false)
/// * `has_color` - Whether to enable color output (true) or use grayscale (false)
pub struct DisplayModeBuilder {
    pub protocol: Protocol,
    pub is_full: bool,
    pub has_color: bool,
}

impl Default for DisplayModeBuilder {
    /// Creates a DisplayModeBuilder with default settings
    ///
    /// The default settings are:
    /// - protocol: Normal (via Protocol::default())
    /// - is_full: true (full block rendering)
    /// - has_color: true (color output)
    fn default() -> Self {
        Self {
            protocol: Protocol::default(),
            is_full: true,
            has_color: true,
        }
    }
}

impl DisplayModeBuilder {
    /// Creates a new DisplayModeBuilder with the specified protocol
    ///
    /// # Arguments
    ///
    /// * `protocol` - The terminal protocol to use for display
    ///
    /// # Returns
    ///
    /// A new DisplayModeBuilder instance with the specified protocol and default settings:
    /// - is_full: true (full block rendering)
    /// - has_color: false (grayscale output)
    pub fn new(protocol: Protocol) -> Self {
        Self {
            protocol,
            is_full: true,
            has_color: true,
        }
    }

    /// Sets the display mode to use full block characters
    ///
    /// # Returns
    ///
    /// A mutable reference to self for method chaining
    pub fn full(&mut self) -> &mut Self {
        self.is_full = true;
        self
    }

    /// Sets the display mode to use half block characters
    ///
    /// # Returns
    ///
    /// A mutable reference to self for method chaining
    pub fn no_full(&mut self) -> &mut Self {
        self.is_full = false;
        self
    }

    /// Sets whether to use full or half block characters based on the parameter
    ///
    /// # Arguments
    ///
    /// * `is_full` - true to use full block characters, false to use half block
    ///
    /// # Returns
    ///
    /// A mutable reference to self for method chaining
    pub fn option_is_full(&mut self, is_full: bool) -> &mut Self {
        self.is_full = is_full;
        self
    }

    /// Enables color output for the display mode
    ///
    /// # Returns
    ///
    /// A mutable reference to self for method chaining
    pub fn colored(&mut self) -> &mut Self {
        self.has_color = true;
        self
    }

    /// Disables color output, using grayscale instead
    ///
    /// # Returns
    ///
    /// A mutable reference to self for method chaining
    pub fn no_colored(&mut self) -> &mut Self {
        self.has_color = false;
        self
    }

    /// Sets whether to use color output based on the parameter
    ///
    /// # Arguments
    ///
    /// * `has_color` - true to enable color output, false for grayscale
    ///
    /// # Returns
    ///
    /// A mutable reference to self for method chaining
    pub fn option_has_color(&mut self, has_color: bool) -> &mut Self {
        self.has_color = has_color;
        self
    }

    /// Converts the builder configuration into a DisplayMode
    ///
    /// This method maps the protocol and display options (full/half block, color/grayscale)
    /// to the appropriate DisplayMode variant.
    ///
    /// # Returns
    ///
    /// A DisplayMode variant that corresponds to the configured protocol and options

    pub fn build(&self) -> DisplayMode {
        match self.protocol {
            Protocol::Normal => match (self.is_full, self.has_color) {
                (true, true) => DisplayMode::FullColor,
                (true, false) => DisplayMode::FullNoColor,
                (false, true) => DisplayMode::HalfColor,
                (false, false) => DisplayMode::Ascii,
            },
            Protocol::Kitty => match self.has_color {
                true => DisplayMode::Kitty,
                false => DisplayMode::KittyNoColor,
            },
            Protocol::ITerm2 => match self.has_color {
                true => DisplayMode::Iterm2,
                false => DisplayMode::Iterm2NoColor,
            },
            Protocol::WezTerm => match self.has_color {
                true => DisplayMode::WezTerm,
                false => DisplayMode::WezTermNoColor,
            },
            Protocol::Sixel => match self.is_full {
                true => DisplayMode::SixelFull,
                false => DisplayMode::SixelHalf,
            },
            #[cfg(feature = "auto_select")]
            Protocol::Auto => Self {
                protocol: get_terminal_protocol(),
                ..self.clone()
            }
            .build(),
        }
    }
}

impl Protocol {
    /// Creates a new DisplayModeBuilder with this protocol as the base
    ///
    /// This is a convenience method for creating a DisplayModeBuilder
    /// initialized with the current protocol.
    ///
    /// # Returns
    ///
    /// A new DisplayModeBuilder instance with this protocol
    pub fn builder(&self) -> DisplayModeBuilder {
        DisplayModeBuilder::new(self.clone())
    }
}

#[cfg(feature = "clap_support")]
/// Implementation of clap::ValueEnum for Protocol to enable command-line argument parsing
///
/// This implementation allows Protocol variants to be used as command-line arguments
/// with clap, mapping string values to enum variants.
///
/// Supported values:
/// - "auto" - Auto detection (requires `auto_check` feature)
/// - "normal" - Standard terminal output
/// - "wezterm" - WezTerm terminal protocol
/// - "kitty" - Kitty terminal protocol
/// - "iterm2" - iTerm2 terminal protocol
/// - "sixel" - Sixel graphics protocol (requires `sixel` feature)
impl clap::ValueEnum for Protocol {
    fn value_variants<'a>() -> &'a [Self] {
        &[
            #[cfg(feature = "auto_select")]
            Self::Auto,
            Self::Normal,
            Self::WezTerm,
            Self::Kitty,
            Self::ITerm2,
            #[cfg(feature = "sixel")]
            Self::Sixel,
        ]
    }

    fn to_possible_value(&self) -> Option<clap::builder::PossibleValue> {
        use clap::builder::PossibleValue;
        Some(match self {
            #[cfg(feature = "auto_select")]
            Self::Auto => PossibleValue::new("auto"),
            Self::Normal => PossibleValue::new("normal"),
            Self::WezTerm => PossibleValue::new("wezterm"),
            Self::Kitty => PossibleValue::new("kitty"),
            Self::ITerm2 => PossibleValue::new("iterm2"),
            #[cfg(feature = "sixel")]
            Self::Sixel => PossibleValue::new("sixel"),
        })
    }
}
