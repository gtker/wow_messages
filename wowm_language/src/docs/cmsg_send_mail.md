## Client Version 1.12

### Wowm Representation
```rust,ignore
cmsg CMSG_SEND_MAIL = 0x0238 {
    Guid mailbox;
    CString receiver;
    CString subject;
    CString body;
    u32 unknown1;
    u32 unknown2;
    Guid item;
    u32 money;
    u32 cash_on_delivery_amount;
    u32 unknown3;
    u32 unknown4;
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
| 0x06 | 8 / Little | [Guid](../spec/packed-guid.md) | mailbox |  |
| 0x0E | - / - | CString | receiver |  |
| - | - / - | CString | subject |  |
| - | - / - | CString | body |  |
| - | 4 / Little | u32 | unknown1 |  |
| - | 4 / Little | u32 | unknown2 |  |
| - | 8 / Little | [Guid](../spec/packed-guid.md) | item |  |
| - | 4 / Little | u32 | money |  |
| - | 4 / Little | u32 | cash_on_delivery_amount |  |
| - | 4 / Little | u32 | unknown3 |  |
| - | 4 / Little | u32 | unknown4 |  |
