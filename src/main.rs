extern crate tofu_ae as app;

use hyper::service::{make_service_fn, service_fn};
use hyper::{Body, Response, Server, StatusCode};

use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::Mutex;


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

#[tokio::main]
pub async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let resource: Arc<Mutex<HashMap<String, Vec<u8>>>> = Arc::new(Mutex::new(HashMap::new()));
    let res1 = Arc::clone(&resource);

    tokio::spawn(async move {
        app::get::rd::init(&res1).await;
    });


    let service = make_service_fn(|_| {
        let res2 = Arc::clone(&resource);
        async move {
            Ok::<_, hyper::Error>(service_fn(move |req| {
                let res2 = Arc::clone(&res2);
                async move {
                    let r = res2.lock().await;

                    let p = req.uri().path_and_query().unwrap();
                    let path = std::path::Path::new(p.path());

                    let dir_str = path.parent().unwrap().to_str().unwrap();
                    let stem = path.file_stem().unwrap();
                    let ext = path.extension().unwrap();

                    let d = if &dir_str[..1] == "/" {
                        &dir_str[1..]
                    } else {
                        &dir_str[..]
                    };

                    let f = if d == "" {
                        format!("{}", stem.to_str().unwrap())
                    } else {
                        format!("{}/{}", d, stem.to_str().unwrap())
                    };

                    let bin = r.get(&f);
                    match (bin, app::get::ext(ext.to_str())) {
                        (Some(res), Some(ext)) if res.len() > 0 => {
                            match app::get::img::load(&res) {
                                Ok(img) => {
                                    let get = to_hash(p.query());
                                    let img = app::get::img::op(img, get);
                                    Ok::<_, hyper::Error>(Response::new(Body::from(app::get::img::convert(img, ext))))
                                },
                                _ => not_found(),
                            }
                        },
                        _ => not_found(),
                    }
                }
            }))
        }
    });

    let addr = ([0, 0, 0, 0], 3000).into();
    let server = Server::bind(&addr).serve(service);

    println!("Listening on http://{}", addr);

    server.await?;

    Ok(())
}
