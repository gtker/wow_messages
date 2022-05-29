# SpellCastTargets

## Client Version 1.12

### Wowm Representation
```rust,ignore
struct SpellCastTargets {
    SpellCastTargetFlags target_flags;
    if (target_flags & UNIT) {
        PackedGuid unit_target1;
    }
    if (target_flags & UNIT_ENEMY) {
        PackedGuid unit_target2;
    }
    if (target_flags & GAMEOBJECT) {
        PackedGuid object_target1;
    }
    if (target_flags & LOCKED) {
        PackedGuid object_target2;
    }
    if (target_flags & ITEM) {
        PackedGuid item_target1;
    }
    if (target_flags & TRADE_ITEM) {
        PackedGuid item_target2;
    }
    if (target_flags & SOURCE_LOCATION) {
        f32 position_x1;
        f32 position_y1;
        f32 position_z1;
    }
    if (target_flags & DEST_LOCATION) {
        f32 position_x2;
        f32 position_y2;
        f32 position_z2;
    }
    if (target_flags & STRING) {
        CString target_string;
    }
    if (target_flags & CORPSE_ALLY) {
        PackedGuid corpse_target1;
    }
    if (target_flags & CORPSE_ENEMY) {
        PackedGuid corpse_target2;
    }
}
```
### Body

| Offset | Size / Endianness | Type | Name | Description | Comment |
| ------ | ----------------- | ---- | ---- | ----------- | ------- |
| 0x00 | ? / - | [SpellCastTargetFlags](spellcasttargetflags.md) | target_flags |  |  |

If target_flags contains `UNIT`:

| Offset | Size / Endianness | Type | Name | Description | Comment |
| ------ | ----------------- | ---- | ---- | ----------- | ------- |
| - | - / - | [PackedGuid](../spec/packed-guid.md) | unit_target1 |  |  |

If target_flags contains `UNIT_ENEMY`:

| Offset | Size / Endianness | Type | Name | Description | Comment |
| ------ | ----------------- | ---- | ---- | ----------- | ------- |
| - | - / - | [PackedGuid](../spec/packed-guid.md) | unit_target2 |  |  |

If target_flags contains `GAMEOBJECT`:

| Offset | Size / Endianness | Type | Name | Description | Comment |
| ------ | ----------------- | ---- | ---- | ----------- | ------- |
| - | - / - | [PackedGuid](../spec/packed-guid.md) | object_target1 |  |  |

If target_flags contains `LOCKED`:

| Offset | Size / Endianness | Type | Name | Description | Comment |
| ------ | ----------------- | ---- | ---- | ----------- | ------- |
| - | - / - | [PackedGuid](../spec/packed-guid.md) | object_target2 |  |  |

If target_flags contains `ITEM`:

| Offset | Size / Endianness | Type | Name | Description | Comment |
| ------ | ----------------- | ---- | ---- | ----------- | ------- |
| - | - / - | [PackedGuid](../spec/packed-guid.md) | item_target1 |  |  |

If target_flags contains `TRADE_ITEM`:

| Offset | Size / Endianness | Type | Name | Description | Comment |
| ------ | ----------------- | ---- | ---- | ----------- | ------- |
| - | - / - | [PackedGuid](../spec/packed-guid.md) | item_target2 |  |  |

If target_flags contains `SOURCE_LOCATION`:

| Offset | Size / Endianness | Type | Name | Description | Comment |
| ------ | ----------------- | ---- | ---- | ----------- | ------- |
| - | 4 / Little | f32 | position_x1 |  |  |
| - | 4 / Little | f32 | position_y1 |  |  |
| - | 4 / Little | f32 | position_z1 |  |  |

If target_flags contains `DEST_LOCATION`:

| Offset | Size / Endianness | Type | Name | Description | Comment |
| ------ | ----------------- | ---- | ---- | ----------- | ------- |
| - | 4 / Little | f32 | position_x2 |  |  |
| - | 4 / Little | f32 | position_y2 |  |  |
| - | 4 / Little | f32 | position_z2 |  |  |

If target_flags contains `STRING`:

| Offset | Size / Endianness | Type | Name | Description | Comment |
| ------ | ----------------- | ---- | ---- | ----------- | ------- |
| - | - / - | CString | target_string |  |  |

If target_flags contains `CORPSE_ALLY`:

| Offset | Size / Endianness | Type | Name | Description | Comment |
| ------ | ----------------- | ---- | ---- | ----------- | ------- |
| - | - / - | [PackedGuid](../spec/packed-guid.md) | corpse_target1 |  |  |

If target_flags contains `CORPSE_ENEMY`:

| Offset | Size / Endianness | Type | Name | Description | Comment |
| ------ | ----------------- | ---- | ---- | ----------- | ------- |
| - | - / - | [PackedGuid](../spec/packed-guid.md) | corpse_target2 |  |  |

