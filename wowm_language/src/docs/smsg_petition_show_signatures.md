# SMSG_PETITION_SHOW_SIGNATURES

## Client Version 1.12

### Wowm Representation
```rust,ignore
smsg SMSG_PETITION_SHOW_SIGNATURES = 0x01BF {
    Guid item_guid;
    Guid owner_guid;
    Guid petition_guid;
    u8 amount_of_signatures;
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
| 0x04 | 8 / Little | [Guid](../spec/packed-guid.md) | item_guid |  |  |
| 0x0C | 8 / Little | [Guid](../spec/packed-guid.md) | owner_guid |  |  |
| 0x14 | 8 / Little | [Guid](../spec/packed-guid.md) | petition_guid |  |  |
| 0x1C | 1 / - | u8 | amount_of_signatures |  |  |

