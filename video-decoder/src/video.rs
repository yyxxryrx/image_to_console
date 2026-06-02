use crate::VideoResult;

pub struct Video {
    input: ffmpeg_next::format::context::Input,
}

impl Video {
    pub fn new(input: ffmpeg_next::format::context::Input) -> Self {
        Self { input }
    }

    pub fn video_decoder<'a>(&'a mut self) -> VideoResult<crate::VideoDecoder<'a>> {
        let index = self
            .input
            .streams()
            .best(ffmpeg_next::media::Type::Video)
            .ok_or(crate::Error::CannotFindVideoStream)?
            .index();
        crate::VideoDecoder::new(&mut self.input, index)
    }
}
