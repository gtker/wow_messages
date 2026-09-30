use std::io::ErrorKind;

use super::{
    packed_component, read_wrath_monster_move_spline, wrath_monster_move_spline_size,
    write_wrath_monster_move_spline,
};
use crate::wrath::Vector3d;

fn vector(x: f32, y: f32, z: f32) -> Vector3d {
    Vector3d { x, y, z }
}

fn append_vector(bytes: &mut Vec<u8>, vector: Vector3d) {
    bytes.extend(vector.x.to_le_bytes());
    bytes.extend(vector.y.to_le_bytes());
    bytes.extend(vector.z.to_le_bytes());
}

fn monster_move_body(move_type: u8) -> Vec<u8> {
    let mut body = vec![0; 1 + 1 + 12 + 4];
    body.push(move_type);
    body
}

fn monster_move_transport_body(move_type: u8) -> Vec<u8> {
    let mut body = vec![0; 1 + 1 + 1 + 1 + 12 + 4];
    body.push(move_type);
    body
}

fn read_body<M: crate::Message>(body: &[u8]) -> Result<M, crate::errors::ParseError> {
    let mut input = body;
    M::read_body::<crate::traits::private::Internal>(&mut input, body.len() as u32)
}

#[test]
fn linear_path_uses_full_destination_and_signed_quarter_unit_offsets() {
    let packed = 0x8000_37ff_u32;
    let mut bytes = 2_u32.to_le_bytes().to_vec();
    append_vector(&mut bytes, vector(14.0, 15.0, 16.0));
    bytes.extend(packed.to_le_bytes());

    let mut input = bytes.as_slice();
    let path = read_wrath_monster_move_spline(&mut input, false).expect("linear path");

    assert!(input.is_empty());
    assert_eq!(path, [vector(14.0, 15.0, 16.0), vector(-0.25, 1.5, -128.0)]);
    assert_eq!(wrath_monster_move_spline_size(&path, false), 20);
}

#[test]
fn smooth_path_uses_full_vectors_for_every_point() {
    let expected = [vector(1.25, -2.5, 3.75), vector(-4.0, 5.5, -6.25)];
    let mut bytes = 2_u32.to_le_bytes().to_vec();
    for point in expected {
        append_vector(&mut bytes, point);
    }

    let mut input = bytes.as_slice();
    let path = read_wrath_monster_move_spline(&mut input, true).expect("smooth path");

    assert!(input.is_empty());
    assert_eq!(path, expected);
    assert_eq!(wrath_monster_move_spline_size(&path, true), 28);
}

#[test]
fn empty_path_encodes_only_zero_count() {
    let mut bytes = Vec::new();
    write_wrath_monster_move_spline(&[], false, &mut bytes).expect("write empty linear path");
    assert_eq!(bytes, 0_u32.to_le_bytes());
    assert_eq!(wrath_monster_move_spline_size(&[], false), 4);

    let mut input = bytes.as_slice();
    assert!(
        read_wrath_monster_move_spline(&mut input, false)
            .expect("read empty linear path")
            .is_empty()
    );
    assert!(input.is_empty());
}

#[test]
fn linear_writer_matches_destination_first_reference_layout() {
    let path = [vector(14.0, 15.0, 16.0), vector(-0.25, 1.5, -128.0)];
    let mut expected = 2_u32.to_le_bytes().to_vec();
    append_vector(&mut expected, path[0]);
    expected.extend(0x8000_37ff_u32.to_le_bytes());

    let mut actual = Vec::new();
    write_wrath_monster_move_spline(&path, false, &mut actual).expect("write linear path");

    assert_eq!(actual, expected);
}

#[test]
fn smooth_writer_preserves_each_full_vector() {
    let path = [vector(1.25, -2.5, 3.75), vector(-4.0, 5.5, -6.25)];
    let mut expected = 2_u32.to_le_bytes().to_vec();
    for point in path {
        append_vector(&mut expected, point);
    }

    let mut actual = Vec::new();
    write_wrath_monster_move_spline(&path, true, &mut actual).expect("write smooth path");

    assert_eq!(actual, expected);
}

#[test]
fn packed_component_truncates_toward_zero_and_checks_signed_bounds() {
    assert_eq!(packed_component(0.49, 11).expect("positive component"), 1);
    assert_eq!(
        packed_component(-0.49, 11).expect("negative component"),
        0x7ff
    );
    assert_eq!(packed_component(-256.0, 11).expect("minimum x/y"), 0x400);
    assert_eq!(packed_component(255.75, 11).expect("maximum x/y"), 0x3ff);
    assert_eq!(packed_component(-128.0, 10).expect("minimum z"), 0x200);
    assert_eq!(packed_component(127.75, 10).expect("maximum z"), 0x1ff);

    assert_eq!(
        packed_component(256.0, 11)
            .expect_err("x/y overflow")
            .kind(),
        ErrorKind::InvalidInput
    );
    assert_eq!(
        packed_component(128.0, 10).expect_err("z overflow").kind(),
        ErrorKind::InvalidInput
    );
    assert_eq!(
        packed_component(f32::NAN, 11)
            .expect_err("non-finite component")
            .kind(),
        ErrorKind::InvalidInput
    );
}

#[test]
fn movement_tail_presence_matches_move_type_in_both_messages() {
    let mut monster_stop_with_tail = monster_move_body(1);
    monster_stop_with_tail.push(0);
    assert!(read_body::<crate::wrath::SMSG_MONSTER_MOVE>(&monster_stop_with_tail).is_err());
    assert!(read_body::<crate::wrath::SMSG_MONSTER_MOVE>(&monster_move_body(0)).is_err());

    let mut transport_stop_with_tail = monster_move_transport_body(1);
    transport_stop_with_tail.push(0);
    assert!(
        read_body::<crate::wrath::SMSG_MONSTER_MOVE_TRANSPORT>(&transport_stop_with_tail).is_err()
    );
    assert!(
        read_body::<crate::wrath::SMSG_MONSTER_MOVE_TRANSPORT>(&monster_move_transport_body(0))
            .is_err()
    );
}

#[test]
fn truncated_large_count_fails_before_reserving_points() {
    // This count requests 8,388,600 bytes, just below Wrath allocation limit.
    let truncated = 699_050_u32.to_le_bytes().to_vec();
    let error = read_wrath_monster_move_spline(&mut truncated.as_slice(), true)
        .expect_err("truncated large path");

    assert!(matches!(
        error,
        crate::errors::ParseErrorKind::Io(error) if error.kind() == ErrorKind::UnexpectedEof
    ));
}

#[test]
fn oversized_and_truncated_paths_fail_before_unbounded_reads_or_allocations() {
    let oversized = u32::MAX.to_le_bytes().to_vec();
    let error = read_wrath_monster_move_spline(&mut oversized.as_slice(), true)
        .expect_err("oversized allocation");
    assert!(matches!(
        error,
        crate::errors::ParseErrorKind::AllocationTooLargeError(_)
    ));

    let mut truncated = 2_u32.to_le_bytes().to_vec();
    append_vector(&mut truncated, vector(1.0, 2.0, 3.0));
    let error = read_wrath_monster_move_spline(&mut truncated.as_slice(), true)
        .expect_err("truncated smooth path");
    assert!(matches!(
        error,
        crate::errors::ParseErrorKind::Io(error) if error.kind() == ErrorKind::UnexpectedEof
    ));
}
