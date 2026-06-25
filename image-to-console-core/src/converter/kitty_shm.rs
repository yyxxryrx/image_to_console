use base64::Engine;

pub struct KittyImage {
    data: crate::shm::SharedData,
    name: String,
    width: u32,
    height: u32,
    id: Option<u32>,
}

impl KittyImage {
    pub fn new(name: String, image: &image::RgbImage) -> crate::shm::error::ShmResult<Self> {
        let data = crate::shm::SharedData::new(&name, &image.as_raw()[..])?;
        Ok(Self {
            data,
            name,
            width: image.width(),
            height: image.height(),
            id: None,
        })
    }

    pub fn id(self, id: u32) -> Self {
        Self {
            id: Some(id),
            ..self
        }
    }
}

impl std::fmt::Display for KittyImage {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "\x1b_Ga=T,s={width},v={height},S={size},t=s,f=24{id};{payload}\x1b\\",
            width = self.width,
            height = self.height,
            size = self.data.size,
            payload = base64::engine::general_purpose::STANDARD.encode(self.name.as_bytes()),
            id = self
                .id
                .map_or_else(Default::default, |id| format!(",i={id}"))
        )
    }
}
