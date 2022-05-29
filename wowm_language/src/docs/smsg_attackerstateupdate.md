# SMSG_ATTACKERSTATEUPDATE

## Client Version 1.12

### Wowm Representation
```rust,ignore
smsg SMSG_ATTACKERSTATEUPDATE = 0x014A {
    u32 hit_info;
    PackedGuid attacker;
    PackedGuid target;
    u32 total_damage;
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

| Offset | Size / Endianness | Type | Name | Description | Comment |
| ------ | ----------------- | ---- | ---- | ----------- | ------- |
| 0x04 | 4 / Little | u32 | hit_info |  |  |
| 0x08 | - / - | [PackedGuid](../spec/packed-guid.md) | attacker |  |  |
| - | - / - | [PackedGuid](../spec/packed-guid.md) | target |  |  |
| - | 4 / Little | u32 | total_damage |  |  |

