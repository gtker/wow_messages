## Client Version 1.12

### Wowm Representation
```rust,ignore
smsg SMSG_PARTY_MEMBER_STATS = 0x007E {
    PackedGuid guid;
    GroupUpdateFlags mask;
    if (mask & FLAG_STATUS) {
        GroupMemberOnlineStatus status;
    }
    if (mask & FLAG_CUR_HP) {
        u16 current_health;
    }
    if (mask & FLAG_MAX_HP) {
        u16 max_health;
    }
    if (mask & FLAG_POWER_TYPE) {
        Power power;
    }
    if (mask & FLAG_CUR_POWER) {
        u16 current_power;
    }
    if (mask & FLAG_MAX_POWER) {
        u16 max_power;
    }
    if (mask & FLAG_LEVEL) {
        u16 level;
    }
    if (mask & FLAG_ZONE) {
        Area area;
    }
    if (mask & FLAG_POSITION) {
        u16 position_x;
        u16 position_y;
    }
    if (mask & FLAG_AURAS) {
        AuraMask auras;
    }
    if (mask & FLAG_PET_NAME) {
        CString pet_name;
    }
    if (mask & FLAG_PET_MODEL_ID) {
        u16 pet_display_id;
    }
    if (mask & FLAG_PET_CUR_HP) {
        u16 pet_current_health;
    }
    if (mask & FLAG_PET_MAX_HP) {
        u16 pet_max_health;
    }
    if (mask & FLAG_PET_POWER_TYPE) {
        Power pet_power_type;
    }
    if (mask & FLAG_PET_CUR_POWER) {
        u16 pet_current_power;
    }
    if (mask & FLAG_PET_MAX_POWER) {
        u16 pet_max_power;
    }
    if (mask & FLAG_PET_AURAS) {
        AuraMask pet_auras;
    }
}
```
### Header
SMSG have a header of 4 bytes.

#### SMSG Header
| Offset | Size / Endianness | Type   | Name   | Description |
| ------ | ----------------- | ------ | ------ | ----------- |
| 0x00   | 2 / Big           | uint16 | size   | Size of the rest of the message including the opcode field but not including the size field.|
| 0x02   | 2 / Little        | uint16 | opcode | Opcode that determines which fields the message contains.|
### Body
| Offset | Size / Endianness | Type | Name | Description |
| ------ | ----------------- | ---- | ---- | ----------- |
| 0x04 | - / - | [PackedGuid](../spec/packed-guid.md) | guid |  |
| - | ? / - | [GroupUpdateFlags](groupupdateflags.md) | mask |  |

If mask contains `FLAG_STATUS`:

| Offset | Size / Endianness | Type | Name | Description |
| ------ | ----------------- | ---- | ---- | ----------- |
| - | ? / - | [GroupMemberOnlineStatus](groupmemberonlinestatus.md) | status |  |

If mask contains `FLAG_CUR_HP`:

| Offset | Size / Endianness | Type | Name | Description |
| ------ | ----------------- | ---- | ---- | ----------- |
| - | 2 / Little | u16 | current_health |  |

If mask contains `FLAG_MAX_HP`:

| Offset | Size / Endianness | Type | Name | Description |
| ------ | ----------------- | ---- | ---- | ----------- |
| - | 2 / Little | u16 | max_health |  |

If mask contains `FLAG_POWER_TYPE`:

| Offset | Size / Endianness | Type | Name | Description |
| ------ | ----------------- | ---- | ---- | ----------- |
| - | ? / - | [Power](power.md) | power |  |

If mask contains `FLAG_CUR_POWER`:

| Offset | Size / Endianness | Type | Name | Description |
| ------ | ----------------- | ---- | ---- | ----------- |
| - | 2 / Little | u16 | current_power |  |

If mask contains `FLAG_MAX_POWER`:

| Offset | Size / Endianness | Type | Name | Description |
| ------ | ----------------- | ---- | ---- | ----------- |
| - | 2 / Little | u16 | max_power |  |

If mask contains `FLAG_LEVEL`:

| Offset | Size / Endianness | Type | Name | Description |
| ------ | ----------------- | ---- | ---- | ----------- |
| - | 2 / Little | u16 | level |  |

If mask contains `FLAG_ZONE`:

| Offset | Size / Endianness | Type | Name | Description |
| ------ | ----------------- | ---- | ---- | ----------- |
| - | ? / - | [Area](area.md) | area |  |

If mask contains `FLAG_POSITION`:

| Offset | Size / Endianness | Type | Name | Description |
| ------ | ----------------- | ---- | ---- | ----------- |
| - | 2 / Little | u16 | position_x |  |
| - | 2 / Little | u16 | position_y |  |

If mask contains `FLAG_AURAS`:

| Offset | Size / Endianness | Type | Name | Description |
| ------ | ----------------- | ---- | ---- | ----------- |
| - | - / - | [AuraMask](../spec/aura-mask.md) | auras |  |

If mask contains `FLAG_PET_NAME`:

| Offset | Size / Endianness | Type | Name | Description |
| ------ | ----------------- | ---- | ---- | ----------- |
| - | - / - | CString | pet_name |  |

If mask contains `FLAG_PET_MODEL_ID`:

| Offset | Size / Endianness | Type | Name | Description |
| ------ | ----------------- | ---- | ---- | ----------- |
| - | 2 / Little | u16 | pet_display_id |  |

If mask contains `FLAG_PET_CUR_HP`:

| Offset | Size / Endianness | Type | Name | Description |
| ------ | ----------------- | ---- | ---- | ----------- |
| - | 2 / Little | u16 | pet_current_health |  |

If mask contains `FLAG_PET_MAX_HP`:

| Offset | Size / Endianness | Type | Name | Description |
| ------ | ----------------- | ---- | ---- | ----------- |
| - | 2 / Little | u16 | pet_max_health |  |

If mask contains `FLAG_PET_POWER_TYPE`:

| Offset | Size / Endianness | Type | Name | Description |
| ------ | ----------------- | ---- | ---- | ----------- |
| - | ? / - | [Power](power.md) | pet_power_type |  |

If mask contains `FLAG_PET_CUR_POWER`:

| Offset | Size / Endianness | Type | Name | Description |
| ------ | ----------------- | ---- | ---- | ----------- |
| - | 2 / Little | u16 | pet_current_power |  |

If mask contains `FLAG_PET_MAX_POWER`:

| Offset | Size / Endianness | Type | Name | Description |
| ------ | ----------------- | ---- | ---- | ----------- |
| - | 2 / Little | u16 | pet_max_power |  |

If mask contains `FLAG_PET_AURAS`:

| Offset | Size / Endianness | Type | Name | Description |
| ------ | ----------------- | ---- | ---- | ----------- |
| - | - / - | [AuraMask](../spec/aura-mask.md) | pet_auras |  |
