use crate::USERNAME;
use std::net::TcpStream;
use wow_srp::normalized_string::NormalizedString;
use wow_srp::vanilla_header::ProofSeed;
use wow_srp::SESSION_KEY_LENGTH;
use wow_world_messages::vanilla::opcodes::ServerOpcodeMessage;
use wow_world_messages::vanilla::ClientMessage;
use wow_world_messages::vanilla::{expect_server_message, expect_server_message_encryption};
use wow_world_messages::vanilla::{
    SMSG_AUTH_RESPONSE_WorldResult, CMSG_AUTH_SESSION, CMSG_CHAR_ENUM, CMSG_PLAYER_LOGIN,
    SMSG_AUTH_CHALLENGE, SMSG_AUTH_RESPONSE, SMSG_CHAR_ENUM,
};

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
        decompressed_addon_info_size: 342,
        addon_info: vec![66, 108, 105, 122, 122, 97, 114, 100, 95, 65, 117, 99, 116, 105, 111, 110, 85, 73, 0, 1, 109, 119, 28, 76, 0, 0, 0, 0, 66, 108, 105, 122, 122, 97, 114, 100, 95, 66, 97, 116, 116, 108, 101, 102, 105, 101, 108, 100, 77, 105, 110, 105, 109, 97, 112, 0, 1, 109, 119, 28, 76, 27, 223, 5, 165, 66, 108, 105, 122, 122, 97, 114, 100, 95, 66, 105, 110, 100, 105, 110, 103, 85, 73, 0, 1, 109, 119, 28, 76, 0, 0, 0, 0, 66, 108, 105, 122, 122, 97, 114, 100, 95, 67, 111, 109, 98, 97, 116, 84, 101, 120, 116, 0, 1, 109, 119, 28, 76, 27, 223, 5, 165, 66, 108, 105, 122, 122, 97, 114, 100, 95, 67, 114, 97, 102, 116, 85, 73, 0, 1, 109, 119, 28, 76, 0, 0, 0, 0, 66, 108, 105, 122, 122, 97, 114, 100, 95, 71, 77, 83, 117, 114, 118, 101, 121, 85, 73, 0, 1, 109, 119, 28, 76, 27, 223, 5, 165, 66, 108, 105, 122, 122, 97, 114, 100, 95, 73, 110, 115, 112, 101, 99, 116, 85, 73, 0, 1, 109, 119, 28, 76, 0, 0, 0, 0, 66, 108, 105, 122, 122, 97, 114, 100, 95, 77, 97, 99, 114, 111, 85, 73, 0, 1, 109, 119, 28, 76, 27, 223, 5, 165, 66, 108, 105, 122, 122, 97, 114, 100, 95, 82, 97, 105, 100, 85, 73, 0, 1, 109, 119, 28, 76, 0, 0, 0, 0, 66, 108, 105, 122, 122, 97, 114, 100, 95, 84, 97, 108, 101, 110, 116, 85, 73, 0, 1, 109, 119, 28, 76, 27, 223, 5, 165, 66, 108, 105, 122, 122, 97, 114, 100, 95, 84, 114, 97, 100, 101, 83, 107, 105, 108, 108, 85, 73, 0, 1, 109, 119, 28, 76, 0, 0, 0, 0, 66, 108, 105, 122, 122, 97, 114, 100, 95, 84, 114, 97, 105, 110, 101, 114, 85, 73, 0, 1, 109, 119, 28, 76, 27, 223, 5, 165],
    }
    .write_unencrypted_client(stream)
    .unwrap();

    let s = expect_server_message_encryption::<SMSG_AUTH_RESPONSE, _>(stream, crypto.decrypter())
        .unwrap();

    if !matches!(s.result, SMSG_AUTH_RESPONSE_WorldResult::AuthOk { .. }) {
        panic!()
    }
    CMSG_CHAR_ENUM {}
        .write_encrypted_client(stream, crypto.encrypter())
        .unwrap();

    let s =
        expect_server_message_encryption::<SMSG_CHAR_ENUM, _>(stream, crypto.decrypter()).unwrap();

    CMSG_PLAYER_LOGIN {
        guid: s.characters[0].guid,
    }
    .write_encrypted_client(stream, crypto.encrypter())
    .unwrap();

    loop {
        let opcode = ServerOpcodeMessage::read_encrypted(stream, crypto.decrypter()).unwrap();

        dbg!(opcode);
    }
}
