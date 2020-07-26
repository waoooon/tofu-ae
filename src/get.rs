use redis::AsyncCommands;

pub mod img;

pub fn ext(ext: Option<&str>) -> Option<image::ImageOutputFormat> {
    match ext {
        Some(e) if e == "png" => Some(image::ImageOutputFormat::Png),
        Some(e) if e == "jpeg" || e == "jpg" => Some(image::ImageOutputFormat::Jpeg(60)),
        Some(e) if e == "gif" => Some(image::ImageOutputFormat::Gif),
        Some(e) if e == "bmp" => Some(image::ImageOutputFormat::Bmp),
        _ => None,
    }
}

pub async fn redis(key: &str) -> Option<Vec<u8>> {
    let client = redis::Client::open("redis://redis/").expect("url error");
    let mut con = client.get_async_connection().await.expect("connect error");
    match con.get(key).await {
        Ok(res) => Some(res),
        _ => None,
    }
}
