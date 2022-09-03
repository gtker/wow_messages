use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use tokio::net::{TcpListener, TcpStream};
use wow_srp::normalized_string::NormalizedString;
use wow_srp::server::SrpServer;
use wow_srp::wrath_header::ProofSeed;
use wow_world_messages::wrath::opcodes::ClientOpcodeMessage;
use wow_world_messages::wrath::tokio_expect_client_message;
use wow_world_messages::wrath::ServerMessage;
use wow_world_messages::wrath::*;
use wow_world_messages::Guid;

pub async fn world(users: Arc<Mutex<HashMap<String, SrpServer>>>) {
    let listener = TcpListener::bind("0.0.0.0:8085").await.unwrap();

    loop {
        let (stream, _) = listener.accept().await.unwrap();

        tokio::spawn(handle(stream, users.clone()));
    }
}

async fn handle(mut stream: TcpStream, users: Arc<Mutex<HashMap<String, SrpServer>>>) {
    let seed = ProofSeed::new();

    SMSG_AUTH_CHALLENGE {
        unknown1: 1,
        realm_seed: seed.seed(),
        seed: [0_u8; 32],
    }
    .tokio_write_unencrypted_server(&mut stream)
    .await
    .unwrap();

    let c = tokio_expect_client_message::<CMSG_AUTH_SESSION, _>(&mut stream)
        .await
        .unwrap();

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
        result: SMSG_AUTH_RESPONSE_WorldResult::AuthOk {
            billing_flags: 0,
            billing_rested: 0,
            billing_time: 0,
            expansion: Expansion::WrathOfTheLichLing,
        },
    }
    .tokio_write_encrypted_server(&mut stream, encryption.encrypter())
    .await
    .unwrap();

    loop {
        let opcode = ClientOpcodeMessage::tokio_read_encrypted(&mut stream, encryption.decrypter())
            .await
            .unwrap();

        match opcode {
            ClientOpcodeMessage::CMSG_CHAR_ENUM(_) => {
                SMSG_CHAR_ENUM {
                    characters: vec![Character {
                        guid: Guid::new(4),
                        name: "Warr".to_string(),
                        race: 1,
                        class: 1,
                        gender: Gender::Female,
                        skin: 0,
                        face: 0,
                        hair_style: 0,
                        hair_color: 0,
                        facial_hair: 0,
                        level: 0,
                        area: 1,
                        map: 0,
                        position: Vector3d {
                            x: 0.0,
                            y: 0.0,
                            z: 0.0,
                        },
                        guild_id: 0,
                        flags: Default::default(),
                        recustomization_flags: 0,
                        first_login: 0,
                        pet_display_id: 0,
                        pet_level: 0,
                        pet_family: 0,
                        gear: [Default::default(); 22],
                    }],
                }
                .tokio_write_encrypted_server(&mut stream, encryption.encrypter())
                .await
                .unwrap();
            }
            e => {
                dbg!(e);
            }
        }
    }
}
