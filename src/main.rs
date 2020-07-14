use hyper::service::{make_service_fn, service_fn};
use hyper::{Body, Request, Response, Server, StatusCode};
use redis::AsyncCommands;


async fn get_redis(key: &str) -> Option<Vec<u8>> {
    let client = redis::Client::open("redis://redis/").expect("url error");
    let mut con = client.get_async_connection().await.expect("connect error");
    match con.get(key).await {
        Ok(res) => Some(res),
        _ => None,
    }
}

fn image_convert(img: &[u8], ext: image::ImageOutputFormat) -> Result<Vec<u8>, String> {
    match image::load_from_memory(img) {
        Ok(res) => {
            let mut output = Vec::new();
            res.write_to(&mut output, ext).unwrap();
            Ok(output)
        },
        Err(err) => Err(err.to_string()),
    }
}

fn not_found() -> Result<Response<Body>, hyper::Error> {
    let mut not_found = Response::default();
    *not_found.status_mut() = StatusCode::NOT_FOUND;
    Ok(not_found)
}

fn get_ext(ext: Option<&str>) -> Option<image::ImageOutputFormat> {
    match ext {
        Some(e) if e == "png" => Some(image::ImageOutputFormat::Png),
        Some(e) if e == "jpeg" || e == "jpg" => Some(image::ImageOutputFormat::Jpeg(60)),
        Some(e) if e == "gif" => Some(image::ImageOutputFormat::Gif),
        Some(e) if e == "bmp" => Some(image::ImageOutputFormat::Bmp),
        _ => None,
    }
}

async fn echo(req: Request<Body>) -> Result<Response<Body>, hyper::Error> {
    let p = &req.uri().path_and_query().unwrap();
    let path = std::path::Path::new(p.path());

    let file_stem = path.file_stem().unwrap();
    let file_ext = path.extension().unwrap();

    match (get_redis(file_stem.to_str().unwrap()).await, get_ext(file_ext.to_str())) {
        (Some(res), Some(x)) if res.len() > 0 => {
            match image_convert(&res, x) {
                Ok(img) => Ok(Response::new(Body::from(img))),
                _ => not_found(),
            }
        },
        _ => not_found(),
    }
}

#[tokio::main]
pub async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    // For every connection, we must make a `Service` to handle all
    // incoming HTTP requests on said connection.
    let service = make_service_fn(|_| {
        // This is the `Service` that will handle the connection.
        // `service_fn` is a helper to convert a function that
        // returns a Response into a `Service`.
        async { Ok::<_, hyper::Error>(service_fn(echo)) }
    });

//    let addr = ([127, 0, 0, 1], 3000).into();
    let addr = ([0, 0, 0, 0], 3000).into();

    let server = Server::bind(&addr).serve(service);

    println!("Listening on http://{}", addr);

    server.await?;

    Ok(())
}
