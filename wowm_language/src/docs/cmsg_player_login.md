## Client Version 1.12

### Description

Command to log into the specified character.

### Comment

This is sent after the client has been authenticated and served the character list with [SMSG_CHAR_ENUM].
If the player receives a [SMSG_CHARACTER_LOGIN_FAILED] it will return to the character screen and send a [CMSG_CHAR_ENUM].

### Wowm Representation
```rust,ignore
cmsg CMSG_PLAYER_LOGIN = 0x003D {
    Guid guid;
}
```
### Header
CMSG have a header of 6 bytes.

#### CMSG Header
| Offset | Size / Endianness | Type   | Name   | Description |
| ------ | ----------------- | ------ | ------ | ----------- |
| 0x00   | 2 / Big           | uint16 | size   | Size of the rest of the message including the opcode field but not including the size field.|
| 0x02   | 4 / Little        | uint32 | opcode | Opcode that determines which fields the message contains.|
### Body
| Offset | Size / Endianness | Type | Name | Description | Comment |
| ------ | ----------------- | ---- | ---- | ----------- | ------- |
| 0x06 | 8 / Little | [Guid](../spec/packed-guid.md) | guid |  |  |
### Examples
```c
0, 12, // size
61, 0, 0, 0, // opcode (61)
239, 190, 173, 222, 0, 0, 0, 0, // guid: Guid
```
