# MSG_QUERY_NEXT_MAIL_TIME_Server

## Client Version 1.12

mangoszero/vmangos: No idea when this is called.

### Wowm Representation

Autogenerated from `wowm` file at [`wow_message_parser/wowm/world/queries/msg_query_next_mail_time_server.wowm:2`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/queries/msg_query_next_mail_time_server.wowm#L2).
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

| Offset | Size / Endianness | Type | Name | Comment |
| ------ | ----------------- | ---- | ---- | ------- |
| 0x04 | 4 / Little | f32 | unread_mails | mangoszero sets 0 if has unread mail, -86400.0f (0xC7A8C000) if not<br/>vmangos sets 0 if has unread mail, -1.0f if not<br/>cmangos has the behavior of mangoszero except when there are unread mails. This is TODO. |

## Client Version 2.4.3, Client Version 3

### Wowm Representation

Autogenerated from `wowm` file at [`wow_message_parser/wowm/world/queries/msg_query_next_mail_time_server.wowm:37`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/queries/msg_query_next_mail_time_server.wowm#L37).
```rust,ignore
smsg MSG_QUERY_NEXT_MAIL_TIME_Server = 0x0284 {
    u32 float;
    u32 amount_of_mails;
    ReceivedMail[amount_of_mails] mails;
}
```
### Header

SMSG have a header of 4 bytes.

#### SMSG Header

| Offset | Size / Endianness | Type   | Name   | Description |
| ------ | ----------------- | ------ | ------ | ----------- |
| 0x00   | 2 **OR** 3 / Big           | uint16 **OR** uint16+uint8 | size | Size of the rest of the message including the opcode field but not including the size field. Wrath server messages **can** be 3 bytes. If the first (most significant) size byte has `0x80` set, the header will be 3 bytes, otherwise it is 2.|
| -      | 2 / Little| uint16 | opcode | Opcode that determines which fields the message contains. |

### Body

| Offset | Size / Endianness | Type | Name | Comment |
| ------ | ----------------- | ---- | ---- | ------- |
| - | 4 / Little | u32 | float |  |
| - | 4 / Little | u32 | amount_of_mails |  |
| - | ? / - | [ReceivedMail](receivedmail.md)[amount_of_mails] | mails |  |

