import enum

class DisplayMode(enum.IntEnum):
    """
    Display modes for rendering images in the terminal.
    
    Each mode represents a different way of displaying images in the terminal,
    with varying levels of color support and terminal protocol requirements.
    """
    HalfColor = enum.auto()
    FullColor = enum.auto()
    FullNoColor = enum.auto()
    Ascii = enum.auto()
    WezTerm = enum.auto()
    WezTermNoColor = enum.auto()
    Kitty = enum.auto()
    KittyNoColor = enum.auto()
    Iterm2 = enum.auto()
    Iterm2NoColor = enum.auto()
    SixelHalf = enum.auto()
    SixelFull = enum.auto()

class Image:
    """
    The image object that can be displayed in the terminal.
    
    Represents an image loaded from file data that can be displayed using
    various terminal display protocols and modes.
    """
    def __init__(self, data: list[int]):
        """
        Initialize an Image from raw byte data.
        
        Args:
            data: Raw image data as a list of integers representing bytes
        """
        pass

    def display(
        self, display_mode: DisplayMode = DisplayMode.FullColor, center: bool = False
    ) -> DisplayImage:
        """
        Display the image using the specified mode and options.
        
        Args:
            display_mode: The terminal display mode to use. Defaults to FullColor.
            center: Whether to center the image in the terminal. Defaults to False.
            
        Returns:
            DisplayImage: An object representing the rendered image ready for display
        """
        pass

class DisplayImage:
    """
    Display image result that can be converted to string for terminal output.
    
    Represents a processed image that has been converted to a terminal-displayable
    format according to the chosen display mode and options.
    """

    pass

def open(path: str) -> Image:
    """
    Open an image file from the specified path.
    
    Args:
        path: Path to the image file to open
        
    Returns:
        Image: An Image object representing the opened image file
        
    Raises:
        FileNotFoundError: If the file at the specified path does not exist
        OSError: If there is an error reading or decoding the image file
    """
    pass
