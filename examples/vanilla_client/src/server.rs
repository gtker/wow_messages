use crate::USERNAME;
use std::net::TcpStream;
use wow_srp::normalized_string::NormalizedString;
use wow_srp::vanilla_header::ProofSeed;
use wow_srp::SESSION_KEY_LENGTH;
use wow_world_messages::vanilla::opcodes::ServerOpcodeMessage;
use wow_world_messages::vanilla::AddonInfo;
use wow_world_messages::vanilla::ClientMessage;
use wow_world_messages::vanilla::{expect_server_message, expect_server_message_encryption};
use wow_world_messages::vanilla::{
    CMSG_AUTH_SESSION, CMSG_CHAR_ENUM, CMSG_PLAYER_LOGIN, SMSG_AUTH_CHALLENGE, SMSG_AUTH_RESPONSE,
    SMSG_CHAR_ENUM,
};

pub fn server(
    mut stream: &mut TcpStream,
    session_key: [u8; SESSION_KEY_LENGTH as usize],
    server_id: u8,
) {
    let s = expect_server_message::<SMSG_AUTH_CHALLENGE, _>(&mut stream).unwrap();

    let seed = ProofSeed::new();
    let seed_value = seed.seed();
    let (client_proof, mut crypto) = seed.into_client_header_crypto(
        &NormalizedString::new(USERNAME).unwrap(),
        session_key,
        s.server_seed,
    );

    CMSG_AUTH_SESSION {
        build: 5875,
        server_id: server_id as u32,
        username: USERNAME.to_string(),
        client_seed: seed_value,
        client_proof,
        addon_info: vec![AddonInfo {
            addon_name: "Test".to_string(),
            addon_crc: 0,
            addon_extra_crc: 0,
            addon_has_signature: 0,
        }],
    }
    .write_unencrypted_client(&mut stream)
    .unwrap();

    let s =
        expect_server_message_encryption::<SMSG_AUTH_RESPONSE, _>(&mut stream, crypto.decrypter())
            .unwrap();

    if !matches!(s, SMSG_AUTH_RESPONSE::AuthOk { .. }) {
        panic!()
    }
    CMSG_CHAR_ENUM {}
        .write_encrypted_client(&mut stream, crypto.encrypter())
        .unwrap();

    let s = expect_server_message_encryption::<SMSG_CHAR_ENUM, _>(&mut stream, crypto.decrypter())
        .unwrap();

    CMSG_PLAYER_LOGIN {
        guid: s.characters[0].guid,
    }
    .write_encrypted_client(&mut stream, crypto.encrypter())
    .unwrap();

    loop {
        let opcode = ServerOpcodeMessage::read_encrypted(&mut stream, crypto.decrypter()).unwrap();

        dbg!(opcode);
    }
}
