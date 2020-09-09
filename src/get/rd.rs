use futures_util::StreamExt as _;
use redis::AsyncCommands;

use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::Mutex;

pub async fn init(resource: &Arc<Mutex<HashMap<String, Vec<u8>>>>) {
    let connect: &str = "redis://redis";
    let pchannel: &str = "image/*";
    let wait: u64 = 5;

    let client_pubsub = redis::Client::open(connect).unwrap();
    let client = redis::Client::open(connect).unwrap();

    loop {
        match (client_pubsub.get_async_connection().await, client.get_async_connection().await) {
            (Ok(c_pubsub), Ok(mut c)) => {
                let mut pubsub = c_pubsub.into_pubsub();
                pubsub.psubscribe(pchannel).await.unwrap();

                let mut stream = pubsub.on_message();

                loop {
                    match stream.next().await {
                        Some(v) => {
                            let key: &str = v.get_channel_name();
                            let op: String = v.get_payload().unwrap();

                            if op == "insert".to_string() {
                                let val: Vec<u8> = c.get(key).await.unwrap();
                                resource.lock().await.insert(key.to_string(), val);
                            }
                        },
                        _ => {
                            break;
                        },
                    }
                }
            },
            _ => {
                tokio::time::delay_for(std::time::Duration::from_secs(wait)).await;
                continue;
            }
        };
    }
}
