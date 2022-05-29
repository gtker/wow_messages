# SMSG_SERVER_MESSAGE

## Client Version 1.12

### Wowm Representation
```rust,ignore
smsg SMSG_SERVER_MESSAGE = 0x0291 {
    ServerMessageType message_type;
    CString message;
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
| 0x04 | ? / - | [ServerMessageType](servermessagetype.md) | message_type |  |  |
| - | - / - | CString | message |  |  |

