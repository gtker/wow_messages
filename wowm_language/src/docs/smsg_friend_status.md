# SMSG_FRIEND_STATUS

## Client Version 1.12

### Wowm Representation
```rust,ignore
smsg SMSG_FRIEND_STATUS = 0x0068 {
    FriendResult result;
    Guid guid;
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
| 0x04 | ? / - | [FriendResult](friendresult.md) | result |  |  |
| - | 8 / Little | [Guid](../spec/packed-guid.md) | guid |  |  |

