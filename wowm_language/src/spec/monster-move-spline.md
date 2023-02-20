# `MonsterMoveSpline`

Array of splines used in `SMSG_MONSTER_MOVE` for Vanilla/TBC/Wrath.
Consists of a `u32` with the amount of splines, followed by the first spline as a `Vector3d` (x, y, z as floats) and then the remaining splines as packed `u32`s.

A C function for converting to and from the packed `u32`s would be:
```c
uint32_t to_packed_vector3d(float x, float y, float z)
{
    uint32_t packed = 0;
    packed |= ((uint32_t)(x / 0.25f) & 0x7FF);
    packed |= ((uint32_t)(y / 0.25f) & 0x7FF) << 11;
    packed |= ((uint32_t)(z / 0.25f) & 0x3FF) << 22;
    return packed;
}

Vector3d from_packed(uint32_t p)
{
    float x = (float)((p & 0x7FF) / 4);
    float y = (float)(((p >> 11) & 0x7FF) / 4);
    float z = (float)(((p >> 22) & 0x3FF) / 4);

    return Vector3d { x, y, z };
}
```
