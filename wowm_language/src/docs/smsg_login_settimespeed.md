# SMSG_LOGIN_SETTIMESPEED

## Client Version 1.12

### Wowm Representation
```rust,ignore
smsg SMSG_LOGIN_SETTIMESPEED = 0x0042 {
    u32 secs_to_time_bit_field;
    f32 game_speed;
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
| 0x04 | 4 / Little | u32 | secs_to_time_bit_field |  |  |
| 0x08 | 4 / Little | f32 | game_speed |  | Set to 0.01666667f in cmangos. |

