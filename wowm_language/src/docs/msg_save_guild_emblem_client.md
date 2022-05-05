# MSG_SAVE_GUILD_EMBLEM_Client
## Client Version 1.12

### Wowm Representation
```rust,ignore
cmsg MSG_SAVE_GUILD_EMBLEM_Client = 0x01F1 {
    Guid vendor;
    u32 emblem_style;
    u32 emblem_color;
    u32 border_style;
    u32 border_color;
    u32 background_color;
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
| Offset | Size / Endianness | Type | Name | Description |
| ------ | ----------------- | ---- | ---- | ----------- |
| 0x06 | 8 / Little | [Guid](../spec/packed-guid.md) | vendor |  |
| 0x0E | 4 / Little | u32 | emblem_style |  |
| 0x12 | 4 / Little | u32 | emblem_color |  |
| 0x16 | 4 / Little | u32 | border_style |  |
| 0x1A | 4 / Little | u32 | border_color |  |
| 0x1E | 4 / Little | u32 | background_color |  |
