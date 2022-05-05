# MSG_QUERY_NEXT_MAIL_TIME_Server
## Client Version 1.12

### Comment

mangoszero/vmangos: No idea when this is called.

### Wowm Representation
```rust,ignore
smsg MSG_QUERY_NEXT_MAIL_TIME_Server = 0x0284 {
    f32 unread_mails;
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
| 0x04 | 4 / Little | f32 | unread_mails |  |
