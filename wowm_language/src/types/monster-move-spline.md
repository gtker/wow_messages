# `MonsterMoveSpline`

A `u32` point count followed by spline points. This type represents the linear-path encoding; Wrath smooth paths use [`FullMonsterMoveSpline`](full-monster-move-spline.md).

## Vanilla and TBC

The first point is a full `Vector3d` (`x`, `y`, and `z` as `f32`). Remaining points use the legacy packed `u32` representation. Do not apply the Wrath signed-offset decoder below to Vanilla/TBC.

## Wrath (3.3.5a)

For linear paths, wire order is the count, a full `Vector3d` destination, then one packed `u32` for each intermediate point in path order. The first full vector is the destination, not the path origin. For each intermediate point `P`, the server packs the signed offset

`offset = (origin + destination) / 2 - P`.

After sign-extending the packed components and scaling by `0.25`, reconstruct the point as

`P = (origin + destination) / 2 - offset`.

Each packed `u32` contains signed quarter-unit components: signed 11-bit `x`, signed 11-bit `y`, and signed 10-bit `z`:

```c
#include <stdint.h>

int32_t sign_extend(uint32_t value, unsigned bits)
{
    uint32_t sign_bit = 1u << (bits - 1);
    return (value & sign_bit)
        ? (int32_t)value - (int32_t)(1u << bits)
        : (int32_t)value;
}

Vector3d decode_wrath_linear_offset(uint32_t packed)
{
    return (Vector3d) {
        (float)sign_extend(packed & 0x7FF, 11) * 0.25f,
        (float)sign_extend((packed >> 11) & 0x7FF, 11) * 0.25f,
        (float)sign_extend((packed >> 22) & 0x3FF, 10) * 0.25f,
    };
}
```

For example, with origin `(10, 20, 30)`, destination `(30, 40, 50)`, and intermediate point `(21, 28, 41)`, the midpoint is `(20, 30, 40)`. The packed offset is `(-1, 2, -1)`, or `(-4, 8, -4)` in quarter units. Its signed 11/11/10-bit fields are `0x7FC`, `0x008`, and `0x3FC`, producing packed value `0xFF0047FC`. Decoding gives `(-1, 2, -1)`; subtracting it from the midpoint recovers `(21, 28, 41)`.

Do not use this type for smooth paths: those contain a full `Vector3d` for every point.
