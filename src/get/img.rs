use std::collections::HashMap;

pub fn load(img: &[u8]) -> image::ImageResult<image::DynamicImage> {
    image::load_from_memory(img)
}

pub fn convert(img: image::DynamicImage, ext: image::ImageOutputFormat) -> Vec<u8> {
    let mut output = Vec::new();
    img.write_to(&mut output, ext).unwrap();
    output
}

pub fn op(img: image::DynamicImage, hm: HashMap<&str, &str>) -> image::DynamicImage {
    match hm.get("op") {
        Some(op) if op == &"r" => {
            if let (Some(x), Some(y)) = (hm.get("x"), hm.get("y")) {
                img.resize(x.parse().unwrap(), y.parse().unwrap(), image::imageops::FilterType::Lanczos3)
            } else {
                img
            }
        },
        Some(op) if op == &"r_ex" => {
            if let (Some(x), Some(y)) = (hm.get("x"), hm.get("y")) {
                img.resize_exact(x.parse().unwrap(), y.parse().unwrap(), image::imageops::FilterType::Lanczos3)
            } else {
                img
            }
        },
        Some(op) if op == &"r_fill" => {
            if let (Some(x), Some(y)) = (hm.get("x"), hm.get("y")) {
                img.resize_to_fill(x.parse().unwrap(), y.parse().unwrap(), image::imageops::FilterType::Lanczos3)
            } else {
                img
            }
        },
        _ => img,
    }
}
