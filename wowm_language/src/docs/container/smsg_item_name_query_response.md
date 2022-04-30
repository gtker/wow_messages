## Client Version 1.12

### Wowm Representation
```rust,ignore
smsg SMSG_ITEM_NAME_QUERY_RESPONSE = 0x02C5 {
    u32 item_id;    
    CString item_name;    
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
| 0x04 | 4 / Little | u32 | item_id |  |
| 0x08 | - / - | CString | item_name |  |
