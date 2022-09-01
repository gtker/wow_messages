use crate::USERNAME;
use std::net::TcpStream;
use wow_srp::header_crypto::ProofSeed;
use wow_srp::normalized_string::NormalizedString;
use wow_srp::SESSION_KEY_LENGTH;
use wow_world_messages::vanilla::opcodes::ServerOpcodeMessage;
use wow_world_messages::vanilla::{expect_server_message, expect_server_message_encryption};
use wow_world_messages::vanilla::{
    SMSG_AUTH_RESPONSE_WorldResult, CMSG_AUTH_SESSION, CMSG_CHAR_ENUM, CMSG_PLAYER_LOGIN,
    SMSG_AUTH_CHALLENGE, SMSG_AUTH_RESPONSE, SMSG_CHAR_ENUM,
};
use wow_world_messages::ClientMessage;

pub fn server(
    stream: &mut TcpStream,
    session_key: [u8; SESSION_KEY_LENGTH as usize],
    server_id: u8,
) {
    let s = expect_server_message::<SMSG_AUTH_CHALLENGE, _>(stream).unwrap();

    let seed = ProofSeed::new();
    let seed_value = seed.seed();
    let (client_proof, mut crypto) = seed.into_proof_and_header_crypto(
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
        decompressed_addon_info_size: 0,
        compressed_addon_info: vec![],
    }
    .write_unencrypted_client(stream)
    .unwrap();

    let s =
        expect_server_message_encryption::<SMSG_AUTH_RESPONSE, _, _>(stream, &mut crypto).unwrap();

    if !matches!(s.result, SMSG_AUTH_RESPONSE_WorldResult::AuthOk { .. }) {
        panic!()
    }
    CMSG_CHAR_ENUM {}
        .write_encrypted_client(stream, &mut crypto)
        .unwrap();

    let s = expect_server_message_encryption::<SMSG_CHAR_ENUM, _, _>(stream, &mut crypto).unwrap();

    CMSG_PLAYER_LOGIN {
        guid: s.characters[0].guid,
    }
    .write_encrypted_client(stream, &mut crypto)
    .unwrap();

    loop {
        let opcode = ServerOpcodeMessage::read_encrypted(stream, &mut crypto).unwrap();

        dbg!(opcode);
    }
}
