extern crate tofu_ae as app;

use hyper::service::{make_service_fn, service_fn};
use hyper::{Body, Request, Response, Server, StatusCode};

use std::collections::HashMap;


fn not_found() -> Result<Response<Body>, hyper::Error> {
    let mut not_found = Response::default();
    *not_found.status_mut() = StatusCode::NOT_FOUND;
    Ok(not_found)
}

fn to_hash(opt: Option<&str>) -> HashMap<&str, &str> {
    match opt {
        Some(st) => st.split("&").collect::<Vec<_>>().iter().map(|s| {
            let t = s.split("=").collect::<Vec<_>>();
            (t[0], t[1])
        }).collect::<HashMap<_, _>>(),
        _ => HashMap::new(),
    }
}

async fn echo(req: Request<Body>) -> Result<Response<Body>, hyper::Error> {
    let p = &req.uri().path_and_query().unwrap();
    let path = std::path::Path::new(p.path());

    let file_stem = path.file_stem().unwrap();
    let file_ext = path.extension().unwrap();

    match (app::get::redis(file_stem.to_str().unwrap()).await, app::get::ext(file_ext.to_str())) {
        (Some(res), Some(ext)) if res.len() > 0 => {
            match app::get::img::load(&res) {
                Ok(img) => {
                    let get = to_hash(p.query());
                    let img = app::get::img::op(img, get);
                    Ok(Response::new(Body::from(app::get::img::convert(img, ext))))
                },
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
