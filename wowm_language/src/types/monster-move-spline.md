# `MonsterMoveSpline`

Packed linear spline payload used by `SMSG_MONSTER_MOVE` for Vanilla/TBC/Wrath. The wire layout is a `u32` point count, followed by the destination as a full `Vector3d`, then one packed `u32` for each remaining point.

Each packed component is a signed 11/11/10-bit integer in quarter units. For a linear spline, each interior point is encoded as the midpoint of start and destination minus that point:

```text
packed_offset = (start + destination) / 2 - interior_point
```

The in-memory `Vec<Vector3d>` representation stores destination first, followed by these signed offsets. Encoding truncates components toward zero to the nearest quarter unit. Decoding must sign-extend each component before multiplying by `0.25`.

```c
uint32_t to_packed_vector3d(float x, float y, float z)
{
    uint32_t packed = 0;
    packed |= ((int32_t)(x / 0.25f) & 0x7FF);
    packed |= ((int32_t)(y / 0.25f) & 0x7FF) << 11;
    packed |= ((int32_t)(z / 0.25f) & 0x3FF) << 22;
    return packed;
}

int32_t sign_extend(uint32_t value, uint32_t bits)
{
    uint32_t sign = 1u << (bits - 1);
    return (value & sign) ? (int32_t)value - (1 << bits) : (int32_t)value;
}

Vector3d from_packed(uint32_t packed)
{
    float x = (float)sign_extend(packed & 0x7FF, 11) * 0.25f;
    float y = (float)sign_extend((packed >> 11) & 0x7FF, 11) * 0.25f;
    float z = (float)sign_extend((packed >> 22) & 0x3FF, 10) * 0.25f;

    return Vector3d { x, y, z };
}
```
