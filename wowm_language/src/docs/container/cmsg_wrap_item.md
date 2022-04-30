## Client Version 1.12

### Wowm Representation
```rust,ignore
cmsg CMSG_WRAP_ITEM = 0x01D3 {
    u8 gift_bag_index;    
    u8 gift_slot;    
    u8 item_bag_index;    
    u8 item_slot;    
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
| 0x06 | 1 / - | u8 | gift_bag_index |  |
| 0x07 | 1 / - | u8 | gift_slot |  |
| 0x08 | 1 / - | u8 | item_bag_index |  |
| 0x09 | 1 / - | u8 | item_slot |  |
