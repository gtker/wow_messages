## Client Version 1.12

### Wowm Representation
```rust,ignore
smsg SMSG_GUILD_EVENT = 0x0092 {
    GuildEvent event;
    u8 amount_of_events;
    CString[amount_of_events] event_descriptions;
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
| 0x04 | ? / - | [GuildEvent](guildevent.md) | event |  |  |
| - | 1 / - | u8 | amount_of_events |  |  |
| - | ? / - | CString[amount_of_events] | event_descriptions |  |  |

