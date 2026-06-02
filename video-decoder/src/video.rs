use crate::VideoResult;

pub struct Video {
    input: ffmpeg_next::format::context::Input,
}

impl Video {
    pub fn new(input: ffmpeg_next::format::context::Input) -> Self {
        Self { input }
    }

    pub fn video_decoder<'a>(&mut self) -> VideoResult<crate::VideoDecoder<'a>> {
        crate::VideoDecoder::new(
            &mut self.input,
            self.input
                .streams()
                .best(ffmpeg_next::media::Type::Video)
                .ok_or(crate::Error::CannotFindVideoStream)?
                .index(),
        )
    }
}
