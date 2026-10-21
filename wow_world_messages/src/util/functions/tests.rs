use std::io::Cursor;

use wow_world_base::shared::vector3d_vanilla_tbc_wrath::Vector3d;

use crate::errors::ParseErrorKind;
use crate::util::{
    monster_move_spline_size, read_wrath_monster_move_spline, wrath_packed_to_vector3d,
    wrath_vector3d_to_packed, write_wrath_monster_move_spline,
};
#[cfg(all(feature = "wrath", feature = "sync"))]
use crate::wrath::{
    opcodes::ServerOpcodeMessage, SMSG_MONSTER_MOVE_MonsterMoveType, ServerMessage,
    SMSG_MONSTER_MOVE,
};
use crate::wrath::{
    FullMonsterMoveData, MonsterMoveData, MonsterMoveDataVariant,
    MonsterMoveDataVariant_SplineFlag, SplineFlag,
};

struct FailFirstWrite {
    failed: bool,
}

impl std::io::Write for FailFirstWrite {
    fn write(&mut self, bytes: &[u8]) -> Result<usize, std::io::Error> {
        if !self.failed {
            self.failed = true;
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                "injected first-write failure",
            ));
        }

        Ok(bytes.len())
    }

    fn flush(&mut self) -> Result<(), std::io::Error> {
        Ok(())
    }
}

#[test]
fn packed_linear_spline_round_trips_signed_quarter_unit_offsets() {
    let splines = vec![
        Vector3d {
            x: 1.25,
            y: -2.5,
            z: 3.75,
        },
        Vector3d {
            x: -0.25,
            y: 0.25,
            z: -1.0,
        },
    ];
    let mut bytes = Vec::new();

    write_wrath_monster_move_spline(&splines, &mut bytes).unwrap();

    let mut expected = vec![2, 0, 0, 0];
    for component in [1.25_f32, -2.5, 3.75] {
        expected.extend_from_slice(&component.to_le_bytes());
    }
    expected.extend_from_slice(&0xFF00_0FFF_u32.to_le_bytes());
    assert_eq!(bytes, expected);
    assert_eq!(monster_move_spline_size(&[]), 4);
    assert_eq!(monster_move_spline_size(&splines[..1]), 16);
    assert_eq!(monster_move_spline_size(&splines), expected.len());

    let decoded = read_wrath_monster_move_spline(
        &mut Cursor::new(bytes),
        crate::errors::MAX_ALLOCATION_SIZE_WRATH,
    )
    .unwrap();
    assert_eq!(decoded, splines);
    assert_eq!(wrath_vector3d_to_packed(&splines[1]).unwrap(), 0xFF00_0FFF);
    assert_eq!(
        wrath_packed_to_vector3d(0xFF00_0FFF),
        Vector3d {
            x: -0.25,
            y: 0.25,
            z: -1.0,
        }
    );
}

#[test]
fn movement_writer_masks_monster_only_flags_from_both_spline_variants() {
    let excluded_flags = SplineFlag::DONE
        | SplineFlag::FINAL_POINT
        | SplineFlag::FINAL_TARGET
        | SplineFlag::FINAL_ANGLE
        | 0xFF;
    let linear = MonsterMoveDataVariant::Linear(MonsterMoveData {
        spline_flags: MonsterMoveDataVariant_SplineFlag::new(
            excluded_flags | SplineFlag::CYCLIC,
            None,
            None,
        ),
        duration: 0,
        splines: Vec::new(),
    });
    let full = MonsterMoveDataVariant::Full(FullMonsterMoveData {
        spline_flags: MonsterMoveDataVariant_SplineFlag::new(
            excluded_flags | SplineFlag::FLYING,
            None,
            None,
        ),
        duration: 0,
        full_splines: Vec::new(),
    });

    for (movement, expected_flags) in [(linear, SplineFlag::CYCLIC), (full, SplineFlag::FLYING)] {
        let mut bytes = Vec::new();
        movement.write_into_vec(&mut bytes).unwrap();

        assert_eq!(
            u32::from_le_bytes(bytes[..4].try_into().unwrap()),
            expected_flags
        );
    }
}

#[test]
fn movement_writer_propagates_flag_write_failures_for_both_variants() {
    let movements = [
        MonsterMoveDataVariant::Linear(MonsterMoveData {
            spline_flags: MonsterMoveDataVariant_SplineFlag::empty(),
            duration: 0,
            splines: Vec::new(),
        }),
        MonsterMoveDataVariant::Full(FullMonsterMoveData {
            spline_flags: MonsterMoveDataVariant_SplineFlag::new(SplineFlag::FLYING, None, None),
            duration: 0,
            full_splines: Vec::new(),
        }),
    ];

    for movement in movements {
        assert!(movement
            .write_into_vec(FailFirstWrite { failed: false })
            .is_err());
    }
}

#[test]
fn packed_components_truncate_toward_zero_to_quarter_units() {
    let packed = wrath_vector3d_to_packed(&Vector3d {
        x: -0.49,
        y: 0.49,
        z: -0.49,
    })
    .unwrap();

    assert_eq!(
        wrath_packed_to_vector3d(packed),
        Vector3d {
            x: -0.25,
            y: 0.25,
            z: -0.25,
        }
    );
}

#[test]
fn packed_components_accept_signed_boundaries_and_reject_invalid_offsets() {
    let minimum_and_maximum = Vector3d {
        x: -256.0,
        y: 255.75,
        z: -128.0,
    };
    let packed = wrath_vector3d_to_packed(&minimum_and_maximum).unwrap();
    assert_eq!(packed, 0x801F_FC00);
    assert_eq!(wrath_packed_to_vector3d(packed), minimum_and_maximum);

    let invalid_offsets = [
        (
            "x above range",
            Vector3d {
                x: 256.0,
                y: 0.0,
                z: 0.0,
            },
        ),
        (
            "x below range",
            Vector3d {
                x: -256.25,
                y: 0.0,
                z: 0.0,
            },
        ),
        (
            "y above range",
            Vector3d {
                x: 0.0,
                y: 256.0,
                z: 0.0,
            },
        ),
        (
            "z above range",
            Vector3d {
                x: 0.0,
                y: 0.0,
                z: 128.0,
            },
        ),
        (
            "z below range",
            Vector3d {
                x: 0.0,
                y: 0.0,
                z: -128.25,
            },
        ),
        (
            "non-finite component",
            Vector3d {
                x: f32::NAN,
                y: 0.0,
                z: 0.0,
            },
        ),
        (
            "infinite component",
            Vector3d {
                x: 0.0,
                y: f32::INFINITY,
                z: 0.0,
            },
        ),
    ];

    for (label, offset) in invalid_offsets {
        let error = wrath_vector3d_to_packed(&offset).expect_err(label);
        assert_eq!(error.kind(), std::io::ErrorKind::InvalidInput, "{label}");
    }
}

#[test]
fn packed_linear_spline_rejects_allocation_before_reading_points() {
    let maximum = crate::errors::MAX_ALLOCATION_SIZE_WRATH;
    let point_count = u32::try_from(maximum / core::mem::size_of::<Vector3d>() as u64 + 1).unwrap();
    let mut reader = Cursor::new(point_count.to_le_bytes());

    let error = read_wrath_monster_move_spline(&mut reader, maximum).unwrap_err();
    assert!(matches!(
        error,
        ParseErrorKind::AllocationTooLargeError(size)
            if size > maximum
    ));
    assert_eq!(reader.position(), 4);
}

#[test]
fn full_spline_read_rejects_allocation_before_reserving_points() {
    let maximum = crate::errors::MAX_ALLOCATION_SIZE_WRATH;
    let point_count = u32::try_from(maximum / core::mem::size_of::<Vector3d>() as u64 + 1).unwrap();
    let mut bytes = SplineFlag::FLYING.to_le_bytes().to_vec();
    bytes.extend_from_slice(&0_u32.to_le_bytes());
    bytes.extend_from_slice(&point_count.to_le_bytes());
    let mut reader = Cursor::new(bytes);

    let error = MonsterMoveDataVariant::read(&mut reader).unwrap_err();
    assert!(matches!(
        error,
        ParseErrorKind::AllocationTooLargeError(size)
            if size > maximum
    ));
    assert_eq!(reader.position(), 12);
}

#[test]
fn full_spline_read_reports_truncated_vectors() {
    let mut bytes = SplineFlag::FLYING.to_le_bytes().to_vec();
    bytes.extend_from_slice(&0_u32.to_le_bytes());
    bytes.extend_from_slice(&1_u32.to_le_bytes());

    let error = MonsterMoveDataVariant::read(&mut Cursor::new(bytes)).unwrap_err();
    assert!(matches!(error, ParseErrorKind::Io(_)));
}

#[test]
fn packed_linear_spline_reports_truncated_offsets() {
    let mut bytes = vec![2, 0, 0, 0];
    for component in [1.0_f32, 2.0, 3.0] {
        bytes.extend_from_slice(&component.to_le_bytes());
    }

    let error = read_wrath_monster_move_spline(
        &mut Cursor::new(bytes),
        crate::errors::MAX_ALLOCATION_SIZE_WRATH,
    )
    .unwrap_err();
    assert!(matches!(error, ParseErrorKind::Io(_)));
}

#[cfg(all(feature = "wrath", feature = "sync"))]
#[test]
fn wrath_facing_target_is_not_repeated_in_spline_flags() {
    let message = SMSG_MONSTER_MOVE {
        guid: crate::Guid::new(0),
        unknown: 0,
        spline_point: Vector3d {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        },
        spline_id: 0,
        move_type: SMSG_MONSTER_MOVE_MonsterMoveType::FacingTarget {
            movement: MonsterMoveDataVariant::Linear(MonsterMoveData {
                spline_flags: MonsterMoveDataVariant_SplineFlag::new(
                    SplineFlag::FINAL_TARGET,
                    None,
                    None,
                ),
                duration: 0,
                splines: Vec::new(),
            }),
            target: crate::Guid::new(1),
        },
    };
    let mut bytes = Vec::new();
    message
        .write_unencrypted_server(&mut Cursor::new(&mut bytes))
        .unwrap();

    assert_eq!(u32::from_le_bytes(bytes[31..35].try_into().unwrap()), 0);
}

#[cfg(all(feature = "wrath", feature = "sync"))]
#[test]
fn wrath_monster_move_stop_has_no_payload_and_rejects_framed_trailing_bytes() {
    let mut packet = vec![0, 21, 0xDD, 0];
    packet.extend_from_slice(&[0; 18]);
    packet.push(1);

    let parsed = ServerOpcodeMessage::read_unencrypted(packet.as_slice()).unwrap();
    let ServerOpcodeMessage::SMSG_MONSTER_MOVE(message) = parsed else {
        panic!("expected SMSG_MONSTER_MOVE");
    };
    assert!(matches!(
        message.move_type,
        SMSG_MONSTER_MOVE_MonsterMoveType::Stop
    ));

    packet[1] += 1;
    packet.push(0);
    assert!(matches!(
        ServerOpcodeMessage::read_unencrypted(packet.as_slice()),
        Err(crate::errors::ExpectedOpcodeError::Parse(_))
    ));
}
