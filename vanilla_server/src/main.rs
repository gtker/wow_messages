mod auth;
mod world;

use std::collections::HashMap;
use std::sync::{Arc, Mutex};

#[tokio::main]
async fn main() {
    let users = Arc::new(Mutex::new(HashMap::new()));

    let auth_server = tokio::spawn(auth::auth(users.clone()));

    let world_server = tokio::spawn(world::world(users.clone()));

    let s = tokio::join!(auth_server, world_server);
    s.0.unwrap();
    s.1.unwrap();
}
