# GroupUpdateFlags

## Client Version 1.12

### Wowm Representation

Autogenerated from `wowm` file at [`wow_message_parser/wowm/world/social/social_common.wowm:197`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/social/social_common.wowm#L197).

```rust,ignore
flag GroupUpdateFlags : u32 {
    FLAG_NONE = 0x00000000;
    FLAG_STATUS = 0x00000001;
    FLAG_CUR_HP = 0x00000002;
    FLAG_MAX_HP = 0x00000004;
    FLAG_POWER_TYPE = 0x00000008;
    FLAG_CUR_POWER = 0x00000010;
    FLAG_MAX_POWER = 0x00000020;
    FLAG_LEVEL = 0x00000040;
    FLAG_ZONE = 0x00000080;
    FLAG_POSITION = 0x00000100;
    FLAG_AURAS = 0x00000200;
    FLAG_AURAS_2 = 0x00000400;
    FLAG_PET_GUID = 0x00000800;
    FLAG_PET_NAME = 0x00001000;
    FLAG_PET_MODEL_ID = 0x00002000;
    FLAG_PET_CUR_HP = 0x00004000;
    FLAG_PET_MAX_HP = 0x00008000;
    FLAG_PET_POWER_TYPE = 0x00010000;
    FLAG_PET_CUR_POWER = 0x00020000;
    FLAG_PET_MAX_POWER = 0x00040000;
    FLAG_PET_AURAS = 0x00080000;
    FLAG_PET_AURAS_2 = 0x00100000;
    MODE_OFFLINE = 0x10000000;
}
```
### Type
The basic type is `u32`, a 4 byte (32 bit) little endian integer.
### Enumerators
| Enumerator | Value  | Description | Comment |
| --------- | -------- | ----------- | ------- |
| `FLAG_NONE` | 0 (0x00) |  | nothing |
| `FLAG_STATUS` | 1 (0x01) |  | uint8, enum `GroupMemberOnlineStatus` |
| `FLAG_CUR_HP` | 2 (0x02) |  | uint16 |
| `FLAG_MAX_HP` | 4 (0x04) |  | uint16 |
| `FLAG_POWER_TYPE` | 8 (0x08) |  | uint8, enum Powers |
| `FLAG_CUR_POWER` | 16 (0x10) |  | uint16 |
| `FLAG_MAX_POWER` | 32 (0x20) |  | uint16 |
| `FLAG_LEVEL` | 64 (0x40) |  | uint16 |
| `FLAG_ZONE` | 128 (0x80) |  | uint16 |
| `FLAG_POSITION` | 256 (0x100) |  | uint16, uint16 |
| `FLAG_AURAS` | 512 (0x200) |  | uint32 mask, for each bit set uint16 spellid |
| `FLAG_AURAS_2` | 1024 (0x400) |  | uint16 above mask continuation, giving max total of 48 auras possible |
| `FLAG_PET_GUID` | 2048 (0x800) |  | uint64 pet guid |
| `FLAG_PET_NAME` | 4096 (0x1000) |  | pet name, NULL terminated string |
| `FLAG_PET_MODEL_ID` | 8192 (0x2000) |  | uint16, model id |
| `FLAG_PET_CUR_HP` | 16384 (0x4000) |  | uint16 pet cur health |
| `FLAG_PET_MAX_HP` | 32768 (0x8000) |  | uint16 pet max health |
| `FLAG_PET_POWER_TYPE` | 65536 (0x10000) |  | uint8 pet power type |
| `FLAG_PET_CUR_POWER` | 131072 (0x20000) |  | uint16 pet cur power |
| `FLAG_PET_MAX_POWER` | 262144 (0x40000) |  | uint16 pet max power |
| `FLAG_PET_AURAS` | 524288 (0x80000) |  | uint32 mask, for each bit set uint16 spellid, pet auras... |
| `FLAG_PET_AURAS_2` | 1048576 (0x100000) |  | uint16 above mask continuation, giving max total of 48 auras possible |
| `MODE_OFFLINE` | 268435456 (0x10000000) |  |  |

Used in:
* [SMSG_PARTY_MEMBER_STATS](smsg_party_member_stats.md)
* [SMSG_PARTY_MEMBER_STATS_FULL](smsg_party_member_stats_full.md)
