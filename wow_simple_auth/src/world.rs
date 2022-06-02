use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::time::Duration;
use tokio::net::TcpStream;
use tokio::time::sleep;
use wow_srp::header_crypto::ProofSeed;
use wow_srp::normalized_string::NormalizedString;
use wow_srp::server::SrpServer;
use wow_vanilla_messages::helper::tokio_expect_client_message;
use wow_vanilla_messages::version_1_12::opcodes::ClientOpcodeMessage;
use wow_vanilla_messages::version_1_12::*;
use wow_vanilla_messages::{Guid, ServerMessage, UpdateMask, UpdatePlayer};

pub async fn handle(mut stream: TcpStream, users: Arc<Mutex<HashMap<String, SrpServer>>>) {
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
                        race: Default::default(),
                        class: Default::default(),
                        gender: Default::default(),
                        skin: 0,
                        face: 0,
                        hairstyle: 0,
                        haircolor: 0,
                        facialhair: 0,
                        level: 0,
                        area: Default::default(),
                        map: Default::default(),
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
                    // Above fail
                    "SUSPENDED" => WorldResult::AUTH_SUSPENDED,
                    "PARENTAL" => WorldResult::AUTH_PARENTAL_CONTROL,
                    "INPROGRESS" => WorldResult::REALM_LIST_IN_PROGRESS,
                    "REALMSUC" => WorldResult::REALM_LIST_SUCCESS,
                    "RLFAILED" => WorldResult::REALM_LIST_FAILED,
                    "INVALID" => WorldResult::REALM_LIST_INVALID,
                    "REALMNO" => WorldResult::REALM_LIST_REALM_NOT_FOUND,
                    "CREATEI" => WorldResult::ACCOUNT_CREATE_IN_PROGRESS,
                    "CREATES" => WorldResult::ACCOUNT_CREATE_SUCCESS,
                    "CREATEF" => WorldResult::ACCOUNT_CREATE_FAILED,
                    "RETRIEV" => WorldResult::CHAR_LIST_RETRIEVING,
                    "RETRIED" => WorldResult::CHAR_LIST_RETRIEVED,
                    "CHARFA" => WorldResult::CHAR_LIST_FAILED,
                    "CCINPRO" => WorldResult::CHAR_CREATE_IN_PROGRESS,
                    "CCSUCC" => WorldResult::CHAR_CREATE_SUCCESS,
                    "ERROR" => WorldResult::CHAR_CREATE_ERROR,
                    "CCFAIL" => WorldResult::CHAR_CREATE_FAILED,
                    "NAMEIN" => WorldResult::CHAR_CREATE_NAME_IN_USE,
                    "DISABLED" => WorldResult::CHAR_CREATE_DISABLED,
                    "PVPTEAMS" => WorldResult::CHAR_CREATE_PVP_TEAMS_VIOLATION,
                    "SERVERL" => WorldResult::CHAR_CREATE_SERVER_LIMIT,
                    "ACCOUNTL" => WorldResult::CHAR_CREATE_ACCOUNT_LIMIT,
                    "SERVERQU" => WorldResult::CHAR_CREATE_SERVER_QUEUE,
                    "ONLYEXI" => WorldResult::CHAR_CREATE_ONLY_EXISTING,
                    "CDINPROG" => WorldResult::CHAR_DELETE_IN_PROGRESS,
                    "CDSUCCESS" => WorldResult::CHAR_DELETE_SUCCESS,
                    "CDFAIL" => WorldResult::CHAR_DELETE_FAILED,
                    "FAILEDLOC" => WorldResult::CHAR_DELETE_FAILED_LOCKED_FOR_TRANSFER,
                    "CLINPROG" => WorldResult::CHAR_LOGIN_IN_PROGRESS,
                    "CLSUCCE" => WorldResult::CHAR_LOGIN_SUCCESS,
                    "NOWORLD" => WorldResult::CHAR_LOGIN_NO_WORLD,
                    "DUPLICATEC" => WorldResult::CHAR_LOGIN_DUPLICATE_CHARACTER,
                    "NOINSTANC" => WorldResult::CHAR_LOGIN_NO_INSTANCES,
                    "CLFAILED" => WorldResult::CHAR_LOGIN_FAILED,
                    "CLDISABLED" => WorldResult::CHAR_LOGIN_DISABLED,
                    "NOCHARACTER" => WorldResult::CHAR_LOGIN_NO_CHARACTER,
                    "LOCKEDFORTR" => WorldResult::CHAR_LOGIN_LOCKED_FOR_TRANSFER,
                    "NONAME" => WorldResult::CHAR_NAME_NO_NAME,
                    "TOOSHORT" => WorldResult::CHAR_NAME_TOO_SHORT,
                    "TOOLONG" => WorldResult::CHAR_NAME_TOO_LONG,
                    "ONLYLETTE" => WorldResult::CHAR_NAME_ONLY_LETTERS,
                    "MIXEDLANG" => WorldResult::CHAR_NAME_MIXED_LANGUAGES,
                    "PROFANE" => WorldResult::CHAR_NAME_PROFANE,
                    "RESERVED" => WorldResult::CHAR_NAME_RESERVED,
                    "INVALIDAPO" => WorldResult::CHAR_NAME_INVALID_APOSTROPHE,
                    "MULTIPLEAP" => WorldResult::CHAR_NAME_MULTIPLE_APOSTROPHES,
                    "THREECONS" => WorldResult::CHAR_NAME_THREE_CONSECUTIVE,
                    "INVALIDSPA" => WorldResult::CHAR_NAME_INVALID_SPACE,
                    "CNSUCCESS" => WorldResult::CHAR_NAME_SUCCESS,
                    "CNFAILURE" => WorldResult::CHAR_NAME_FAILURE,
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
            _ => panic!(),
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
        .set_unit_HEALTH(100);

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
        sleep(Duration::new(10, 0)).await;
    }
}
