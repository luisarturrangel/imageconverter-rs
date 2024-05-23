pub enum ErrorType {
    NoPathProvided,
    InvalidFileType,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum FormatType {
    PNG,
    JPEG,
    BMP,
    WEBP,
    ICO,
}

impl ErrorType {
    pub fn error_menssage(error: &Option<ErrorType>) -> &str {
        let error = match error {
            Some(ErrorType::NoPathProvided) => "Error: No path provided",
            Some(ErrorType::InvalidFileType) => "Error: Invlid file type",
            _ => panic!("something"),
        };
        error
    }
}

impl FormatType {
    pub fn from_index(index: &FormatType) -> image::ImageFormat {
        match index {
            FormatType::PNG => image::ImageFormat::Png,
            FormatType::JPEG => image::ImageFormat::Jpeg,
            FormatType::BMP => image::ImageFormat::Bmp,
            FormatType::WEBP => image::ImageFormat::WebP,
            FormatType::ICO => image::ImageFormat::Ico,
        }
    }
}