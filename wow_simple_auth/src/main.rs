use std::collections::HashMap;
use std::io::{BufWriter, Write};
use std::net::{TcpListener, TcpStream};
use wow_login_messages::all::CMD_AUTH_LOGON_CHALLENGE_Client;
use wow_login_messages::all::CMD_AUTH_RECONNECT_CHALLENGE_Client;
use wow_login_messages::helper::{
    read_expect_client_login_message, read_initial_opcode, InitialLoginOpcode,
    InitialLoginOpcodeError,
};
use wow_login_messages::version_2::CMD_AUTH_LOGON_CHALLENGE_Server;
use wow_login_messages::version_2::CMD_AUTH_LOGON_CHALLENGE_ServerLoginResult;
use wow_login_messages::version_2::CMD_AUTH_LOGON_PROOF_Client;
use wow_login_messages::version_2::CMD_AUTH_RECONNECT_PROOF_Client;
use wow_login_messages::version_2::CMD_AUTH_RECONNECT_PROOF_Server;
use wow_login_messages::version_2::CMD_REALM_LIST_Client;
use wow_login_messages::version_2::CMD_REALM_LIST_Server;
use wow_login_messages::version_2::LoginResult::SUCCESS;
use wow_login_messages::version_2::Realm;
use wow_login_messages::version_2::RealmFlag;
use wow_login_messages::version_2::RealmType;
use wow_login_messages::version_2::{
    CMD_AUTH_LOGON_PROOF_Server, CMD_AUTH_LOGON_PROOF_ServerLoginResult,
};
use wow_login_messages::version_2::{
    CMD_AUTH_RECONNECT_CHALLENGE_Server, CMD_AUTH_RECONNECT_CHALLENGE_ServerLoginResult,
};
use wow_login_messages::ReadableAndWritable;
use wow_srp::normalized_string::NormalizedString;
use wow_srp::server::{SrpServer, SrpVerifier};
use wow_srp::{PublicKey, GENERATOR, LARGE_SAFE_PRIME_LITTLE_ENDIAN};

fn main() -> std::io::Result<()> {
    let listener = TcpListener::bind("0.0.0.0:3724")?;

    let mut users: HashMap<String, SrpServer> = HashMap::new();

    for stream in listener.incoming() {
        handle(stream?, &mut users);
    }

    Ok(())
}

fn handle(mut stream: TcpStream, users: &mut HashMap<String, SrpServer>) {
    let opcode = read_initial_opcode(&mut stream);
    let opcode = match opcode {
        Ok(o) => o,
        Err(e) => {
            match e {
                InitialLoginOpcodeError::Io(_) => {}
                InitialLoginOpcodeError::InvalidOpcode(o) => {
                    println!("Got invalid opcode {}", o);
                }
                InitialLoginOpcodeError::CMD_AUTH_LOGON_CHALLENGE(_) => {}
                InitialLoginOpcodeError::CMD_AUTH_RECONNECT_CHALLENGE(_) => {}
            }
            return;
        }
    };

    match opcode {
        InitialLoginOpcode::Logon(l) => {
            login(stream, l, users);
        }
        InitialLoginOpcode::Reconnect(r) => {
            reconnect(stream, r, users);
        }
    }
}
fn reconnect(
    mut stream: TcpStream,
    r: CMD_AUTH_RECONNECT_CHALLENGE_Client,
    users: &mut HashMap<String, SrpServer>,
) {
    println!("Reconnect version: {}", r.protocol_version);

    let server = users.get_mut(&r.account_name).unwrap();

    CMD_AUTH_RECONNECT_CHALLENGE_Server {
        result: CMD_AUTH_RECONNECT_CHALLENGE_ServerLoginResult::SUCCESS {
            challenge_data: *server.reconnect_challenge_data(),
            checksum_salt: [0; 16],
        },
    }
    .write(&mut stream)
    .unwrap();

    let l = read_expect_client_login_message::<CMD_AUTH_RECONNECT_PROOF_Client, _>(&mut stream)
        .unwrap();

    server.verify_reconnection_attempt(l.proof_data, l.client_proof);

    CMD_AUTH_RECONNECT_PROOF_Server { result: SUCCESS }
        .write(&mut stream)
        .unwrap();

    while let Ok(_) = read_expect_client_login_message::<CMD_REALM_LIST_Client, _>(&mut stream) {
        let mut b = BufWriter::new(&mut stream);
        CMD_REALM_LIST_Server {
            realms: vec![Realm {
                realm_type: RealmType::PLAYER_VS_ENVIRONMENT,
                flag: RealmFlag::new_OFFLINE(),
                name: "Tester".to_string(),
                address: "127.0.0.1:8085".to_string(),
                population: Default::default(),
                number_of_characters_on_realm: 0,
                category: Default::default(),
                realm_id: 0,
            }],
        }
        .write(&mut b)
        .unwrap();
        b.flush().unwrap();
        println!("Sent Logon Proof");
    }
}

fn login(
    mut stream: TcpStream,
    l: CMD_AUTH_LOGON_CHALLENGE_Client,
    users: &mut HashMap<String, SrpServer>,
) {
    println!("Login version: {}", l.protocol_version);
    let username = NormalizedString::new(l.account_name.clone()).unwrap();
    let password = NormalizedString::new(l.account_name).unwrap();

    let p = SrpVerifier::from_username_and_password(username.clone(), password).into_proof();

    CMD_AUTH_LOGON_CHALLENGE_Server {
        login_result: CMD_AUTH_LOGON_CHALLENGE_ServerLoginResult::SUCCESS {
            server_public_key: *p.server_public_key(),
            generator: vec![GENERATOR],
            large_safe_prime: LARGE_SAFE_PRIME_LITTLE_ENDIAN.into(),
            salt: *p.salt(),
            crc_salt: [0; 16],
        },
    }
    .write(&mut stream)
    .unwrap();
    println!("Sent Logon Challenge");

    let l =
        read_expect_client_login_message::<CMD_AUTH_LOGON_PROOF_Client, _>(&mut stream).unwrap();

    let (p, proof) = p
        .into_server(
            PublicKey::from_le_bytes(&l.client_public_key).unwrap(),
            l.client_proof,
        )
        .unwrap();

    CMD_AUTH_LOGON_PROOF_Server {
        login_result: CMD_AUTH_LOGON_PROOF_ServerLoginResult::SUCCESS {
            server_proof: proof,
            hardware_survey_id: 0,
        },
    }
    .write(&mut stream)
    .unwrap();
    println!("Sent Logon Proof");

    users.insert(username.as_ref().to_string(), p);

    while let Ok(_) = read_expect_client_login_message::<CMD_REALM_LIST_Client, _>(&mut stream) {
        let mut b = BufWriter::new(&mut stream);
        CMD_REALM_LIST_Server {
            realms: vec![Realm {
                realm_type: RealmType::PLAYER_VS_ENVIRONMENT,
                flag: RealmFlag::new_OFFLINE(),
                name: "Tester".to_string(),
                address: "127.0.0.1:8085".to_string(),
                population: Default::default(),
                number_of_characters_on_realm: 0,
                category: Default::default(),
                realm_id: 0,
            }],
        }
        .write(&mut b)
        .unwrap();
        b.flush().unwrap();
        println!("Sent Logon Proof");
    }
}
