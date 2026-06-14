use crate::{Error, VideoResult};

#[derive(Debug)]
pub struct VideoFrame {
    pub pts: Option<std::time::Duration>,
    pub frame: image::RgbImage,
}

impl VideoFrame {
    pub fn new(frame: image::RgbImage, pts: Option<std::time::Duration>) -> Self {
        Self { frame, pts }
    }
}

fn process_frame(frame: &ffmpeg_next::util::frame::Video) -> VideoResult<image::RgbImage> {
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
    image::RgbImage::from_raw(width as u32, height as u32, pixels).ok_or(Error::ToImageFailed)
}

pub struct VideoDecoder<'a> {
    decoder: ffmpeg_next::codec::decoder::Video,
    video_stream: usize,
    pockets: ffmpeg_next::format::context::input::PacketIter<'a>,
    video_frame: ffmpeg_next::frame::Video,
    width: u32,
    height: u32,
    rate: f32,
    time_base: f64,
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
        let rate = codec.frame_rate().ok_or(Error::GetVideoInfoFailed)?;
        let time_base = {
            let time_base = stream.time_base();
            time_base.0 as f64 / time_base.1 as f64
        };
        Ok(Self {
            video_stream,
            pockets: input.packets(),
            width: codec.width(),
            height: codec.height(),
            time_base,
            rate: rate.0 as f32 / rate.1 as f32,
            decoder: codec,
            video_frame: ffmpeg_next::frame::Video::empty(),
        })
    }

    pub fn size(&self) -> (u32, u32) {
        (self.width, self.height)
    }

    pub fn frame_rate(&self) -> f32 {
        self.rate
    }

    fn to_frames(
        self,
        pos: Option<std::sync::Arc<std::sync::atomic::AtomicU64>>,
    ) -> VideoResult<VideoFrames<'a>> {
        let codec = &self.decoder;
        let scaler = ffmpeg_next::software::scaling::Context::get(
            codec.format(),
            codec.width(),
            codec.height(),
            ffmpeg_next::format::Pixel::RGB24,
            codec.width(),
            codec.height(),
            ffmpeg_next::software::scaling::Flags::BILINEAR,
        )?;
        Ok(VideoFrames::new(self, scaler, pos))
    }

    #[inline]
    pub fn frames(self) -> VideoResult<VideoFrames<'a>> {
        self.to_frames(None)
    }

    #[inline]
    pub fn frames_with_pos(
        self,
        pos: std::sync::Arc<std::sync::atomic::AtomicU64>,
    ) -> VideoResult<VideoFrames<'a>> {
        self.to_frames(Some(pos))
    }

    fn read_frame(&mut self) -> VideoResult<Option<ffmpeg_next::util::frame::Video>> {
        if self.decoder.receive_frame(&mut self.video_frame).is_ok() {
            return Ok(Some(self.video_frame.clone()));
        }
        for (stream, packet) in self.pockets.by_ref() {
            if stream.index() == self.video_stream {
                match self.decoder.send_packet(&packet) {
                    Err(ffmpeg_next::Error::Other { errno })
                        if errno == ffmpeg_next::error::EAGAIN => {}
                    Err(e) => return Err(e.into()),
                    _ => {}
                }
                if self.decoder.receive_frame(&mut self.video_frame).is_ok() {
                    return Ok(Some(self.video_frame.clone()));
                }
            }
        }

        if self.decoder.receive_frame(&mut self.video_frame).is_ok() {
            return Ok(Some(self.video_frame.clone()));
        }
        Ok(None)
    }
}

impl<'a> Iterator for VideoDecoder<'a> {
    type Item = VideoResult<ffmpeg_next::util::frame::Video>;

    fn next(&mut self) -> Option<Self::Item> {
        self.read_frame().transpose()
    }
}

pub struct VideoFrames<'a> {
    decoder: VideoDecoder<'a>,
    scaler: ffmpeg_next::software::scaling::Context,
    rgb_frame: ffmpeg_next::frame::Video,
    pos: Option<std::sync::Arc<std::sync::atomic::AtomicU64>>,
}

impl<'a> VideoFrames<'a> {
    pub fn new(
        decoder: VideoDecoder<'a>,
        scaler: ffmpeg_next::software::scaling::Context,
        pos: Option<std::sync::Arc<std::sync::atomic::AtomicU64>>,
    ) -> Self {
        Self {
            pos,
            decoder,
            scaler,
            rgb_frame: ffmpeg_next::util::frame::Video::empty(),
        }
    }

    fn forward(&mut self) -> VideoResult<Option<VideoFrame>> {
        let Some(frame) = self.decoder.read_frame()? else {
            return Ok(None);
        };
        let pts = frame
            .pts()
            .map(|pts| std::time::Duration::from_secs_f64(pts as f64 * self.decoder.time_base));

        if let Some((pts, pos)) = pts.as_ref().zip(self.pos.as_ref()) {
            let pos =
                std::time::Duration::from_millis(pos.load(std::sync::atomic::Ordering::SeqCst));
            if pos.saturating_sub(*pts).as_millis() > 250 {
                return self.forward();
            }
        }

        self.scaler.run(&frame, &mut self.rgb_frame)?;
        let img = process_frame(&self.rgb_frame)?;
        // dbg!(self.decoder.time_base);
        Ok(Some(VideoFrame::new(img, pts)))
    }
}

impl<'a> Iterator for VideoFrames<'a> {
    type Item = VideoResult<VideoFrame>;

    fn next(&mut self) -> Option<Self::Item> {
        self.forward().transpose()
    }
}
