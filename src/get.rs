pub mod img;
pub mod rd;

pub fn ext(ext: Option<&str>) -> Option<image::ImageOutputFormat> {
    match ext {
        Some(e) if e == "png" => Some(image::ImageOutputFormat::Png),
        Some(e) if e == "jpeg" || e == "jpg" => Some(image::ImageOutputFormat::Jpeg(60)),
        Some(e) if e == "gif" => Some(image::ImageOutputFormat::Gif),
        Some(e) if e == "bmp" => Some(image::ImageOutputFormat::Bmp),
        _ => None,
    }
}
