mod auth;
mod world;

use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use tokio::net::TcpListener;

#[tokio::main]
async fn main() {
    let users = Arc::new(Mutex::new(HashMap::new()));
    let users2 = users.clone();

    let auth_server = tokio::spawn(async move {
        let listener = TcpListener::bind("0.0.0.0:3724").await.unwrap();

        loop {
            let (stream, _) = listener.accept().await.unwrap();

            tokio::spawn(auth::handle(stream, users.clone()));
        }
    });

    let world_server = tokio::spawn(async move {
        let listener = TcpListener::bind("0.0.0.0:8085").await.unwrap();

        loop {
            let (stream, _) = listener.accept().await.unwrap();

            tokio::spawn(world::handle(stream, users2.clone()));
        }
    });

    let s = tokio::join!(auth_server, world_server);
    s.0.unwrap();
    s.1.unwrap();
}
