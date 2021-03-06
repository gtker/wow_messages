# MovementBlock

## Client Version 1.12

### Wowm Representation

Autogenerated from `wowm` file at [`wow_message_parser/wowm/world/gameobject/smsg_update_object.wowm:91`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/gameobject/smsg_update_object.wowm#L91).
```rust,ignore
struct MovementBlock {
    UpdateFlag update_flag;
    if (update_flag & LIVING) {
        MovementFlags flags;
        u32 timestamp;
        Vector3d living_position;
        f32 living_orientation;
        if (flags & ON_TRANSPORT) {
            TransportInfo transport;
        }
        if (flags & SWIMMING) {
            f32 pitch;
        }
        f32 fall_time;
        if (flags & JUMPING) {
            f32 z_speed;
            f32 cos_angle;
            f32 sin_angle;
            f32 xy_speed;
        }
        if (flags & SPLINE_ELEVATION) {
            f32 spline_elevation;
        }
        f32 walking_speed;
        f32 running_speed;
        f32 backwards_running_speed;
        f32 swimming_speed;
        f32 backwards_swimming_speed;
        f32 turn_rate;
        if (flags & SPLINE_ENABLED) {
            SplineFlag spline_flags;
            if (spline_flags & FINAL_ANGLE) {
                f32 angle;
            }
            else if (spline_flags & FINAL_TARGET) {
                u64 target;
            }
            else if (spline_flags & FINAL_POINT) {
                Vector3d spline_final_point;
            }
            u32 time_passed;
            u32 duration;
            u32 id;
            u32 amount_of_nodes;
            Vector3d[amount_of_nodes] nodes;
            Vector3d final_node;
        }
    }
    else if (update_flag & HAS_POSITION) {
        Vector3d position;
        f32 orientation;
    }
    if (update_flag & HIGH_GUID) {
        u32 unknown0;
    }
    if (update_flag & ALL) {
        u32 unknown1;
    }
    if (update_flag & MELEE_ATTACKING) {
        PackedGuid guid;
    }
    if (update_flag & TRANSPORT) {
        u32 transport_progress_in_ms;
    }
}
```
### Body

| Offset | Size / Endianness | Type | Name | Description | Comment |
| ------ | ----------------- | ---- | ---- | ----------- | ------- |
| 0x00 | ? / - | [UpdateFlag](updateflag.md) | update_flag |  |  |

If update_flag contains `LIVING`:

| Offset | Size / Endianness | Type | Name | Description | Comment |
| ------ | ----------------- | ---- | ---- | ----------- | ------- |
| - | ? / - | [MovementFlags](movementflags.md) | flags |  |  |
| - | 4 / Little | u32 | timestamp |  |  |
| - | ? / - | [Vector3d](vector3d.md) | living_position |  |  |
| - | 4 / Little | f32 | living_orientation |  |  |

If flags contains `ON_TRANSPORT`:

| Offset | Size / Endianness | Type | Name | Description | Comment |
| ------ | ----------------- | ---- | ---- | ----------- | ------- |
| - | ? / - | [TransportInfo](transportinfo.md) | transport |  |  |

If flags contains `SWIMMING`:

| Offset | Size / Endianness | Type | Name | Description | Comment |
| ------ | ----------------- | ---- | ---- | ----------- | ------- |
| - | 4 / Little | f32 | pitch |  |  |
| - | 4 / Little | f32 | fall_time |  |  |

If flags contains `JUMPING`:

| Offset | Size / Endianness | Type | Name | Description | Comment |
| ------ | ----------------- | ---- | ---- | ----------- | ------- |
| - | 4 / Little | f32 | z_speed |  |  |
| - | 4 / Little | f32 | cos_angle |  |  |
| - | 4 / Little | f32 | sin_angle |  |  |
| - | 4 / Little | f32 | xy_speed |  |  |

If flags contains `SPLINE_ELEVATION`:

| Offset | Size / Endianness | Type | Name | Description | Comment |
| ------ | ----------------- | ---- | ---- | ----------- | ------- |
| - | 4 / Little | f32 | spline_elevation |  |  |
| - | 4 / Little | f32 | walking_speed |  |  |
| - | 4 / Little | f32 | running_speed |  |  |
| - | 4 / Little | f32 | backwards_running_speed |  |  |
| - | 4 / Little | f32 | swimming_speed |  |  |
| - | 4 / Little | f32 | backwards_swimming_speed |  |  |
| - | 4 / Little | f32 | turn_rate |  |  |

If flags contains `SPLINE_ENABLED`:

| Offset | Size / Endianness | Type | Name | Description | Comment |
| ------ | ----------------- | ---- | ---- | ----------- | ------- |
| - | ? / - | [SplineFlag](splineflag.md) | spline_flags |  |  |

If spline_flags contains `FINAL_ANGLE`:

| Offset | Size / Endianness | Type | Name | Description | Comment |
| ------ | ----------------- | ---- | ---- | ----------- | ------- |
| - | 4 / Little | f32 | angle |  |  |

Else If spline_flags contains `FINAL_TARGET`:

| Offset | Size / Endianness | Type | Name | Description | Comment |
| ------ | ----------------- | ---- | ---- | ----------- | ------- |
| - | 8 / Little | u64 | target |  |  |

Else If spline_flags contains `FINAL_POINT`:

| Offset | Size / Endianness | Type | Name | Description | Comment |
| ------ | ----------------- | ---- | ---- | ----------- | ------- |
| - | ? / - | [Vector3d](vector3d.md) | spline_final_point |  |  |
| - | 4 / Little | u32 | time_passed |  |  |
| - | 4 / Little | u32 | duration |  |  |
| - | 4 / Little | u32 | id |  |  |
| - | 4 / Little | u32 | amount_of_nodes |  |  |
| - | ? / - | [Vector3d](vector3d.md)[amount_of_nodes] | nodes |  |  |
| - | ? / - | [Vector3d](vector3d.md) | final_node |  |  |

Else If update_flag contains `HAS_POSITION`:

| Offset | Size / Endianness | Type | Name | Description | Comment |
| ------ | ----------------- | ---- | ---- | ----------- | ------- |
| - | ? / - | [Vector3d](vector3d.md) | position |  |  |
| - | 4 / Little | f32 | orientation |  |  |

If update_flag contains `HIGH_GUID`:

| Offset | Size / Endianness | Type | Name | Description | Comment |
| ------ | ----------------- | ---- | ---- | ----------- | ------- |
| - | 4 / Little | u32 | unknown0 |  | vmangos statically sets to 0 |

If update_flag contains `ALL`:

| Offset | Size / Endianness | Type | Name | Description | Comment |
| ------ | ----------------- | ---- | ---- | ----------- | ------- |
| - | 4 / Little | u32 | unknown1 |  | vmangos sets statically to 1 |

If update_flag contains `MELEE_ATTACKING`:

| Offset | Size / Endianness | Type | Name | Description | Comment |
| ------ | ----------------- | ---- | ---- | ----------- | ------- |
| - | - / - | [PackedGuid](../spec/packed-guid.md) | guid |  |  |

If update_flag contains `TRANSPORT`:

| Offset | Size / Endianness | Type | Name | Description | Comment |
| ------ | ----------------- | ---- | ---- | ----------- | ------- |
| - | 4 / Little | u32 | transport_progress_in_ms |  |  |

