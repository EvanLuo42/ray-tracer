pub struct Image {
    aspect_ratio: f64,
    width: u64,
    height: u64
}

impl Image {
    pub fn with_aspect_ratio(aspect_ratio: f64, width: u64) -> Image {
        Image {
            aspect_ratio,
            width,
            height: (width as f64 / aspect_ratio) as u64
        }
    }
    
    pub fn new(width: u64, height: u64) -> Image {
        Image {
            aspect_ratio: (width / height) as f64,
            width,
            height,
        }
    }
}