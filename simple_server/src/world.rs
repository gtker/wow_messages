use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use tokio::net::{TcpListener, TcpStream};
use wow_srp::header_crypto::ProofSeed;
use wow_srp::normalized_string::NormalizedString;
use wow_srp::server::SrpServer;
use wow_vanilla_messages::helper::tokio_expect_client_message;
use wow_vanilla_messages::version_1_12::opcodes::ClientOpcodeMessage;
use wow_vanilla_messages::version_1_12::*;
use wow_vanilla_messages::{Guid, ServerMessage, UpdateMask, UpdatePlayer};

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
        result: SMSG_AUTH_RESPONSE_WorldResult::AUTH_OK {
            billing_flags: 0,
            billing_rested: 0,
            billing_time: 0,
        },
    }
    .tokio_write_encrypted_server(&mut stream, &mut encryption)
    .await
    .unwrap();

    loop {
        let opcode = ClientOpcodeMessage::tokio_read_encrypted(&mut stream, &mut encryption)
            .await
            .unwrap();

        match opcode {
            ClientOpcodeMessage::CMSG_PING(c) => {
                SMSG_PONG {
                    sequence_id: c.sequence_id,
                }
                .tokio_write_encrypted_server(&mut stream, &mut encryption)
                .await
                .unwrap();
            }
            ClientOpcodeMessage::CMSG_CHAR_ENUM(_) => {
                SMSG_CHAR_ENUM {
                    characters: vec![Character {
                        guid: Guid::new(4),
                        name: "Warr".to_string(),
                        race: Race::HUMAN,
                        class: Class::WARRIOR,
                        gender: Gender::FEMALE,
                        skin: 0,
                        face: 0,
                        hairstyle: 0,
                        haircolor: 0,
                        facialhair: 0,
                        level: 0,
                        area: Area::NORTHSHIRE_VALLEY,
                        map: Map::EASTERN_KINGDOMS,
                        position: Vector3d {
                            x: 0.0,
                            y: 0.0,
                            z: 0.0,
                        },
                        guild_id: 0,
                        flags: Default::default(),
                        first_login: 0,
                        pet_display_id: 0,
                        pet_level: 0,
                        pet_familiy: 0,
                        equipment: [Default::default(); 19],
                    }],
                }
                .tokio_write_encrypted_server(&mut stream, &mut encryption)
                .await
                .unwrap();
            }
            ClientOpcodeMessage::CMSG_CHAR_CREATE(c) => {
                let result = match c.name.to_uppercase().as_str() {
                    "SYSTEME" => WorldResult::AUTH_SYSTEM_ERROR,
                    "SERVERSH" => WorldResult::AUTH_SERVER_SHUTTING_DOWN,
                    "WAITQU" => WorldResult::AUTH_WAIT_QUEUE,
                    "ERROR" => WorldResult::CHAR_CREATE_ERROR,
                    "SERVERL" => WorldResult::CHAR_CREATE_SERVER_LIMIT,
                    // CCSUCC immediately returns to character screen
                    "CCSUCC" => WorldResult::CHAR_CREATE_SUCCESS,
                    "SERVERQU" => WorldResult::CHAR_CREATE_SERVER_QUEUE,
                    // Above fail
                    _ => WorldResult::CHAR_CREATE_ERROR,
                };

                SMSG_CHAR_CREATE { result }
                    .tokio_write_encrypted_server(&mut stream, &mut encryption)
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
        map: Map::EASTERN_KINGDOMS,
        position: Vector3d {
            x: 200.0,
            y: 200.0,
            z: 200.0,
        },
        orientation: 0.0,
    }
    .tokio_write_encrypted_server(&mut stream, &mut encryption)
    .await
    .unwrap();

    SMSG_TUTORIAL_FLAGS {
        tutorial_data0: 0xFFFFFFFF,
        tutorial_data1: 0xFFFFFFFF,
        tutorial_data2: 0xFFFFFFFF,
        tutorial_data3: 0xFFFFFFFF,
        tutorial_data4: 0xFFFFFFFF,
        tutorial_data5: 0xFFFFFFFF,
        tutorial_data6: 0xFFFFFFFF,
        tutorial_data7: 0xFFFFFFFF,
    }
    .tokio_write_encrypted_server(&mut stream, &mut encryption)
    .await
    .unwrap();

    let update_mask = UpdatePlayer::new()
        .set_object_GUID(Guid::new(4))
        .set_unit_BYTES_0(Race::HUMAN, Class::WARRIOR, Gender::FEMALE, Power::RAGE)
        .set_object_SCALE_X(1.0)
        .set_unit_HEALTH(100)
        .set_unit_MAXHEALTH(100)
        .set_unit_LEVEL(1)
        .set_unit_FACTIONTEMPLATE(1)
        .set_unit_DISPLAYID(50)
        .set_unit_NATIVEDISPLAYID(50);

    let update_flag = MovementBlock_UpdateFlag::empty()
        .set_LIVING(MovementBlock_UpdateFlag_LIVING::LIVING {
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
        .set_ALL(MovementBlock_UpdateFlag_ALL { unknown1: 1 })
        .set_SELF();

    SMSG_UPDATE_OBJECT {
        has_transport: 0,
        objects: vec![Object {
            update_type: Object_UpdateType::CREATE_OBJECT2 {
                guid3: Guid::new(4),
                mask2: UpdateMask::Player(update_mask),
                movement2: MovementBlock { update_flag },
                object_type: ObjectType::PLAYER,
            },
        }],
    }
    .tokio_write_encrypted_server(&mut stream, &mut encryption)
    .await
    .unwrap();

    loop {
        let opcode = ClientOpcodeMessage::tokio_read_encrypted(&mut stream, &mut encryption)
            .await
            .unwrap();
        dbg!(opcode);
    }
}
