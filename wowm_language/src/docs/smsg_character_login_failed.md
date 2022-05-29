## Client Version 1.12

### Description

Response if [CMSG_PLAYER_LOGIN](./cmsg_player_login.md) fails. If successful it should instead be [SMSG_LOGIN_VERIFY_WORLD](./smsg_login_verify_world.md).

### Wowm Representation
```rust,ignore
smsg SMSG_CHARACTER_LOGIN_FAILED = 0x0041 {
    WorldResult result;
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
| 0x04 | ? / - | [WorldResult](worldresult.md) | result |  |  |

### Examples
```c
0, 6, // size
65, 0, // opcode (65)
65, 0, 0, 0, // result: WorldResult CHAR_LOGIN_FAILED (0x41)
```
