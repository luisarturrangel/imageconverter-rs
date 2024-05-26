pub enum ErrorType {
    NoPathProvided,
    InvalidFileType,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum FormatType {
    Png,
    Jpeg,
    Bmp,
    WebP,
    Ico,
    Gif,
    Avif,
    Dds,
    Hdr,
    Tiff
}

impl ErrorType {
    pub fn error_menssage(error: &Option<ErrorType>) -> &str {
        match error {
            Some(ErrorType::NoPathProvided) => "Error: No path provided",
            Some(ErrorType::InvalidFileType) => "Error: Invlid file type",
            _ => "something went wrong",
        }
    }
}

impl FormatType {
    pub fn from_index(index: &FormatType) -> image::ImageFormat {
        match index {
            FormatType::Png => image::ImageFormat::Png,
            FormatType::Jpeg => image::ImageFormat::Jpeg,
            FormatType::Bmp => image::ImageFormat::Bmp,
            FormatType::WebP => image::ImageFormat::WebP,
            FormatType::Ico => image::ImageFormat::Ico,
            FormatType::Gif => image::ImageFormat::Gif,
            FormatType::Avif => image::ImageFormat::Avif,
            FormatType::Dds => image::ImageFormat::Dds,
            FormatType::Hdr => image::ImageFormat::Hdr,
            FormatType::Tiff => image::ImageFormat::Tiff,

        }
    }
    pub fn output_ext(index: &FormatType) -> &str {
        match index {
            FormatType::Png => ".png",
            FormatType::Jpeg => ".jpeg",
            FormatType::Bmp => ".bmp",
            FormatType::WebP => ".webP",
            FormatType::Ico => ".ico",
            FormatType::Gif => ".gif",
            FormatType::Avif => ".avif",
            FormatType::Dds => ".dds",
            FormatType::Hdr => ".hdr",
            FormatType::Tiff => ".tiff",

        }
    }
}

