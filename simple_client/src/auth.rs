use crate::{PASSWORD, USERNAME};
use std::net::TcpStream;
use wow_login_messages::all::{CMD_AUTH_LOGON_CHALLENGE_Client, Locale, Os, Platform, Version};
use wow_login_messages::helper::expect_server_message;
use wow_login_messages::version_2::{
    CMD_AUTH_LOGON_PROOF_Server, CMD_AUTH_LOGON_PROOF_Server_LoginResult, CMD_REALM_LIST_Client,
    CMD_REALM_LIST_Server,
};
use wow_login_messages::version_3::{
    CMD_AUTH_LOGON_CHALLENGE_Server, CMD_AUTH_LOGON_CHALLENGE_Server_LoginResult,
    CMD_AUTH_LOGON_PROOF_Client, CMD_AUTH_LOGON_PROOF_Client_SecurityFlag,
};
use wow_login_messages::ClientMessage;
use wow_srp::client::SrpClientUser;
use wow_srp::normalized_string::NormalizedString;
use wow_srp::{PublicKey, SESSION_KEY_LENGTH};

pub fn auth(
    auth_server: &mut TcpStream,
) -> ([u8; SESSION_KEY_LENGTH as usize], CMD_REALM_LIST_Server) {
    CMD_AUTH_LOGON_CHALLENGE_Client {
        protocol_version: 3, // We are pretending to be 1.12
        version: Version {
            major: 1,
            minor: 12,
            patch: 1,
            build: 5875,
        },
        platform: Platform::X86,
        os: Os::Windows,
        locale: Locale::EnGb,
        utc_timezone_offset: 180,
        client_ip_address: 0x7F000001,      // 127.0.0.1
        account_name: USERNAME.to_string(), //
    }
    .write(auth_server)
    .unwrap();

    let s = expect_server_message::<CMD_AUTH_LOGON_CHALLENGE_Server, _>(auth_server).unwrap();

    let c = if let CMD_AUTH_LOGON_CHALLENGE_Server_LoginResult::Success {
        generator,
        large_safe_prime,
        salt,
        server_public_key,
        ..
    } = s.result
    {
        let generator = generator[0];
        let large_safe_prime = large_safe_prime.try_into().unwrap();
        let server_public_key = PublicKey::from_le_bytes(&server_public_key).unwrap();

        SrpClientUser::new(
            NormalizedString::new(USERNAME).unwrap(),
            NormalizedString::new(PASSWORD).unwrap(),
        )
        .into_challenge(generator, large_safe_prime, server_public_key, salt)
    } else {
        panic!()
    };

    CMD_AUTH_LOGON_PROOF_Client {
        client_public_key: *c.client_public_key(),
        client_proof: *c.client_proof(),
        crc_hash: [0u8; 20],
        telemetry_keys: vec![],
        security_flag: CMD_AUTH_LOGON_PROOF_Client_SecurityFlag::None,
    }
    .write(auth_server)
    .unwrap();

    let s = expect_server_message::<CMD_AUTH_LOGON_PROOF_Server, _>(auth_server).unwrap();
    let c = if let CMD_AUTH_LOGON_PROOF_Server_LoginResult::Success { server_proof, .. } = s.result
    {
        c.verify_server_proof(server_proof).unwrap()
    } else {
        panic!()
    };

    CMD_REALM_LIST_Client {}.write(auth_server).unwrap();

    let realms = expect_server_message::<CMD_REALM_LIST_Server, _>(auth_server).unwrap();

    (c.session_key(), realms)
}
