## Client Version 1.12

### Wowm Representation
```rust,ignore
msg MSG_QUEST_PUSH_RESULT = 0x0276 {
    Guid guid;    
    QuestPartyMessage message;    
}

```
### Header
MSG have a header of either 6 bytes if they are sent from the client (CMSG), or 4 bytes if they are sent from the server (SMSG).

#### CMSG Header
| Offset | Size / Endianness | Type   | Name   | Description |
| ------ | ----------------- | ------ | ------ | ----------- |
| 0x00   | 2 / Big           | uint16 | size   | Size of the rest of the message including the opcode field but not including the size field.|
| 0x02   | 4 / Little        | uint32 | opcode | Opcode that determines which fields the message contains.|
#### SMSG Header
| Offset | Size / Endianness | Type   | Name   | Description |
| ------ | ----------------- | ------ | ------ | ----------- |
| 0x00   | 2 / Big           | uint16 | size   | Size of the rest of the message including the opcode field but not including the size field.|
| 0x02   | 2 / Little        | uint16 | opcode | Opcode that determines which fields the message contains.|
### Body
| Offset | Size / Endianness | Type | Name | Description |
| ------ | ----------------- | ---- | ---- | ----------- |
| 0x00 | 8 / Little | Guid | guid |  |
| 0x08 | ? / - | QuestPartyMessage | message |  |
