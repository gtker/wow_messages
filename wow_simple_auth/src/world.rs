use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::time::Duration;
use tokio::net::TcpStream;
use tokio::time::sleep;
use wow_srp::header_crypto::ProofSeed;
use wow_srp::normalized_string::NormalizedString;
use wow_srp::server::SrpServer;
use wow_vanilla_messages::helper::{
    tokio_read_expect_client_message, tokio_read_expect_client_message_encryption,
};
use wow_vanilla_messages::v1::v12::*;
use wow_vanilla_messages::ServerMessageWrite;

pub async fn handle(mut stream: TcpStream, users: Arc<Mutex<HashMap<String, SrpServer>>>) {
    let seed = ProofSeed::new();

    SMSG_AUTH_CHALLENGE {
        server_seed: seed.seed(),
    }
    .tokio_write_unencrypted_server(&mut stream)
    .await
    .unwrap();

    let c = tokio_read_expect_client_message::<CMSG_AUTH_SESSION, _>(&mut stream)
        .await
        .unwrap();
    dbg!(&c);

    let session_key = {
        let mut server = users.lock().unwrap();
        *server.get_mut(&c.username).unwrap().session_key()
    };

    let mut encryption = seed
        .into_header_crypto(
            &NormalizedString::new(&c.username).unwrap(),
            session_key,
            c.client_proof,
            c.client_seed,
        )
        .unwrap();

    SMSG_AUTH_RESPONSE {
        result: SMSG_AUTH_RESPONSEWorldResult::AUTH_OK {
            billing_flags: 0,
            billing_rested: 0,
            billing_time: 0,
        },
    }
    .tokio_write_encrypted_server(&mut stream, &mut encryption)
    .await
    .unwrap();

    let c = tokio_read_expect_client_message_encryption::<CMSG_CHAR_ENUM, _, _>(
        &mut stream,
        &mut encryption,
    )
    .await
    .unwrap();
    dbg!(&c);

    SMSG_CHAR_ENUM { characters: vec![] }
        .tokio_write_encrypted_server(&mut stream, &mut encryption)
        .await
        .unwrap();

    sleep(Duration::new(10, 0)).await;
}
