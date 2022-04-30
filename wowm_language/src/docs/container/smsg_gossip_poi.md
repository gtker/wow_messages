## Client Version 1.12

### Wowm Representation
```rust,ignore
smsg SMSG_GOSSIP_POI = 0x0224 {
    u32 flags;    
    f32 position_x;    
    f32 position_y;    
    u32 icon;    
    u32 data;    
    CString location_name;    
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
| 0x04 | 4 / Little | u32 | flags |  |
| 0x08 | 4 / Little | f32 | position_x |  |
| 0x0C | 4 / Little | f32 | position_y |  |
| 0x10 | 4 / Little | u32 | icon |  |
| 0x14 | 4 / Little | u32 | data |  |
| 0x18 | - / - | CString | location_name |  |
