#![deny(clippy::unwrap_used)]
mod error;

pub use error::*;

pub type FFmpegResult<T> = Result<T, ffmpeg_next::Error>;

pub struct VideoFrame {
    pub pts: Option<i64>,
    pub frame: image::RgbImage,
}

impl VideoFrame {
    pub fn new(frame: image::RgbImage, pts: Option<i64>) -> Self {
        Self { frame, pts }
    }
}

fn process_frame(frame: &ffmpeg_next::util::frame::Video) -> Option<image::RgbImage> {
    let width = frame.width() as usize;
    let height = frame.height() as usize;
    let stride = frame.stride(0);
    let row_size = width * 3;

    let data = frame.data(0);

    let mut pixels = Vec::with_capacity(width * height * 3);
    if stride == row_size {
        pixels.extend_from_slice(data);
    } else {
        for i in 0..height {
            let start = stride * i;
            let end = start + row_size;
            pixels.extend_from_slice(&data[start..end]);
        }
    }
    image::RgbImage::from_raw(width as u32, height as u32, pixels)
}

struct VideoDecoder<'a> {
    decoder: ffmpeg_next::codec::decoder::Video,
    video_stream: usize,
    pockets: ffmpeg_next::format::context::input::PacketIter<'a>,
    scaler: ffmpeg_next::software::scaling::Context,
    video_frame: ffmpeg_next::frame::Video,
    rgb_frame: ffmpeg_next::frame::Video,
}

impl<'a> VideoDecoder<'a> {
    pub fn new(
        input: &'a mut ffmpeg_next::format::context::Input,
        video_stream: usize,
    ) -> VideoResult<Self> {
        let Some(stream) = input.stream(video_stream) else {
            return Err(Error::InvalidStream);
        };
        let codec = ffmpeg_next::codec::Context::from_parameters(stream.parameters())?
            .decoder()
            .video()?;
        let scaler = ffmpeg_next::software::scaling::Context::get(
            codec.format(),
            codec.width(),
            codec.height(),
            ffmpeg_next::format::Pixel::RGB24,
            codec.width(),
            codec.height(),
            ffmpeg_next::software::scaling::Flags::BILINEAR,
        )?;
        Ok(Self {
            video_stream,
            pockets: input.packets(),
            scaler,
            decoder: codec,
            rgb_frame: ffmpeg_next::frame::Video::empty(),
            video_frame: ffmpeg_next::frame::Video::empty(),
        })
    }

    pub fn read_frame(&mut self) -> FFmpegResult<Option<VideoFrame>> {
        if self.decoder.receive_frame(&mut self.video_frame).is_ok() {
            self.scaler.run(&self.video_frame, &mut self.rgb_frame)?;
            return Ok(process_frame(&self.rgb_frame)
                .map(|img| VideoFrame::new(img, self.rgb_frame.pts())));
        }
        while let Some((stream, packet)) = self.pockets.next() {
            if stream.index() == self.video_stream {
                self.decoder.send_packet(&packet)?;
                if self.decoder.receive_frame(&mut self.video_frame).is_ok() {
                    self.scaler.run(&self.video_frame, &mut self.rgb_frame)?;
                    return Ok(process_frame(&self.rgb_frame)
                        .map(|img| VideoFrame::new(img, packet.pts())));
                }
            }
        }
        Ok(None)
    }
}
