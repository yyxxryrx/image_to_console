use gif::DisposalMethod;
use image::{DynamicImage, ImageBuffer, Rgba};

pub struct GifFrameProcessor {
    global_palette: Option<Vec<u8>>,
    last_disposal: DisposalMethod,
    last_frame_area: (u32, u32, u32, u32),
    canvas: ImageBuffer<Rgba<u8>, Vec<u8>>,
    previous_canvas: Option<ImageBuffer<Rgba<u8>, Vec<u8>>>,
}

impl GifFrameProcessor {
    pub fn new(width: u32, height: u32, global_palette: Option<Vec<u8>>) -> Self {
        Self {
            global_palette,
            previous_canvas: None,
            last_frame_area: (0, 0, 0, 0),
            last_disposal: DisposalMethod::Any,
            canvas: ImageBuffer::new(width, height),
        }
    }

    fn clean_canvas(&mut self) {
        match self.last_disposal {
            DisposalMethod::Background => {
                // Clean up the area occupied by the previous frame to be transparent
                let (left, top, w, h) = self.last_frame_area;
                for y in top..(top + h) {
                    for x in left..(left + w) {
                        self.canvas.put_pixel(x, y, Rgba([0, 0, 0, 0]));
                    }
                }
            }
            DisposalMethod::Previous => {
                // Reverts to the state it was in before the last frame was drawn
                if let Some(prev) = &self.previous_canvas {
                    self.canvas = prev.clone();
                }
            }
            _ => { /* Any or Keep，do nothing */ }
        }
    }

    pub fn process_frame(&mut self, frame: &gif::Frame) -> DynamicImage {
        self.clean_canvas();
        if frame.dispose == DisposalMethod::Previous {
            self.previous_canvas = Some(self.canvas.clone());
        }

        let palette = frame
            .palette.as_ref()
            .or(self.global_palette.as_ref())
            .unwrap();
        let transparent_index = frame.transparent.map(|i| i);
        for y in 0..frame.height {
            for x in 0..frame.width {
                let index_in_frame = (y as usize) * (frame.width as usize) + (x as usize);
                let color_index = frame.buffer[index_in_frame];

                // Skip the transparent color
                if Some(color_index) == transparent_index {
                    continue;
                }

                // Get RGBA colors from the palette
                let (r, g, b) = (
                    palette[color_index as usize * 3],
                    palette[color_index as usize * 3 + 1],
                    palette[color_index as usize * 3 + 2],
                );

                // Coordinates are calculated on the main canvas
                let canvas_x = frame.left as u32 + x as u32;
                let canvas_y = frame.top as u32 + y as u32;

                if canvas_x < self.canvas.width() && canvas_y < self.canvas.height() {
                    self.canvas
                        .put_pixel(canvas_x, canvas_y, Rgba([r, g, b, 255]));
                }
            }
        }
        self.last_disposal = frame.dispose;
        self.last_frame_area = (
            frame.left as u32,
            frame.top as u32,
            frame.width as u32,
            frame.height as u32,
        );
        DynamicImage::from(self.canvas.clone())
    }
}
