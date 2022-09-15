use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use tokio::net::{TcpListener, TcpStream};
use wow_srp::normalized_string::NormalizedString;
use wow_srp::server::SrpServer;
use wow_srp::vanilla_header::ProofSeed;
use wow_world_messages::vanilla::opcodes::ClientOpcodeMessage;
use wow_world_messages::vanilla::tokio_expect_client_message;
use wow_world_messages::vanilla::ServerMessage;
use wow_world_messages::vanilla::*;
use wow_world_messages::vanilla::{UpdateMask, UpdatePlayer};
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
        server_seed: seed.seed(),
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
            ClientOpcodeMessage::CMSG_PING(c) => {
                SMSG_PONG {
                    sequence_id: c.sequence_id,
                }
                .tokio_write_encrypted_server(&mut stream, encryption.encrypter())
                .await
                .unwrap();
            }
            ClientOpcodeMessage::CMSG_CHAR_ENUM(_) => {
                SMSG_CHAR_ENUM {
                    characters: vec![Character {
                        guid: Guid::new(4),
                        name: "Warr".to_string(),
                        race: Race::Human,
                        class: Class::Warrior,
                        gender: Gender::Female,
                        skin: 0,
                        face: 0,
                        hair_style: 0,
                        hair_color: 0,
                        facial_hair: 0,
                        level: 0,
                        area: Area::NorthshireValley,
                        map: Map::EasternKingdoms,
                        position: Vector3d {
                            x: 0.0,
                            y: 0.0,
                            z: 0.0,
                        },
                        guild_id: 0,
                        flags: Default::default(),
                        first_login: false,
                        pet_display_id: 0,
                        pet_level: 0,
                        pet_family: 0,
                        equipment: [Default::default(); 19],
                    }],
                }
                .tokio_write_encrypted_server(&mut stream, encryption.encrypter())
                .await
                .unwrap();
            }
            ClientOpcodeMessage::CMSG_CHAR_CREATE(c) => {
                let result = match c.name.to_uppercase().as_str() {
                    "SYSTEME" => WorldResult::AuthSystemError,
                    "SERVERSH" => WorldResult::AuthServerShuttingDown,
                    "WAITQU" => WorldResult::AuthWaitQueue,
                    "ERROR" => WorldResult::CharCreateError,
                    "SERVERL" => WorldResult::CharCreateServerLimit,
                    // CCSUCC immediately returns to character screen
                    "CCSUCC" => WorldResult::CharCreateSuccess,
                    "SERVERQU" => WorldResult::CharCreateServerQueue,
                    // Above fail
                    _ => WorldResult::CharCreateError,
                };

                SMSG_CHAR_CREATE { result }
                    .tokio_write_encrypted_server(&mut stream, encryption.encrypter())
                    .await
                    .unwrap();
            }
            ClientOpcodeMessage::CMSG_PLAYER_LOGIN(_) => {
                break;
            }
            e => {
                dbg!(e);
            }
        }
    }

    SMSG_LOGIN_VERIFY_WORLD {
        map: Map::EasternKingdoms,
        position: Vector3d {
            x: 200.0,
            y: 200.0,
            z: 200.0,
        },
        orientation: 0.0,
    }
    .tokio_write_encrypted_server(&mut stream, encryption.encrypter())
    .await
    .unwrap();

    SMSG_TUTORIAL_FLAGS {
        tutorial_data: [
            0xFFFFFFFF, 0xFFFFFFFF, 0xFFFFFFFF, 0xFFFFFFFF, 0xFFFFFFFF, 0xFFFFFFFF, 0xFFFFFFFF,
            0xFFFFFFFF,
        ],
    }
    .tokio_write_encrypted_server(&mut stream, encryption.encrypter())
    .await
    .unwrap();

    let update_mask = UpdatePlayer::new()
        .set_object_GUID(Guid::new(4))
        .set_unit_BYTES_0(Race::Human, Class::Warrior, Gender::Female, Power::Rage)
        .set_object_SCALE_X(1.0)
        .set_unit_HEALTH(100)
        .set_unit_MAXHEALTH(100)
        .set_unit_LEVEL(1)
        .set_unit_FACTIONTEMPLATE(1)
        .set_unit_DISPLAYID(50)
        .set_unit_NATIVEDISPLAYID(50);

    let update_flag = MovementBlock_UpdateFlag::empty()
        .set_LIVING(MovementBlock_UpdateFlag_Living::Living {
            backwards_running_speed: 4.5,
            backwards_swimming_speed: 0.0,
            fall_time: 0.0,
            flags: MovementBlock_MovementFlags::empty(),
            living_orientation: 0.0,
            living_position: Vector3d {
                x: -8949.95,
                y: -132.493,
                z: 83.5312,
            },
            running_speed: 7.0,
            swimming_speed: 0.0,
            timestamp: 0,
            turn_rate: std::f32::consts::PI,
            walking_speed: 1.0,
        })
        .set_ALL(MovementBlock_UpdateFlag_All { unknown1: 1 })
        .set_SELF();

    SMSG_UPDATE_OBJECT {
        has_transport: 0,
        objects: vec![Object {
            update_type: Object_UpdateType::CreateObject2 {
                guid3: Guid::new(4),
                mask2: UpdateMask::Player(update_mask),
                movement2: MovementBlock { update_flag },
                object_type: ObjectType::Player,
            },
        }],
    }
    .tokio_write_encrypted_server(&mut stream, encryption.encrypter())
    .await
    .unwrap();

    loop {
        let opcode = ClientOpcodeMessage::tokio_read_encrypted(&mut stream, encryption.decrypter())
            .await
            .unwrap();
        dbg!(opcode);
    }
}
