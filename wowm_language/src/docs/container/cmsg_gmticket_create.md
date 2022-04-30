## Client Version 1.12

### Wowm Representation
```rust,ignore
cmsg CMSG_GMTICKET_CREATE = 0x0205 {
    GmTicketType category;    
    Map map;    
    f32 position_x;    
    f32 position_y;    
    f32 position_z;    
    CString message;    
    CString reserved_for_future_use;    
    if (category == BEHAVIOR_HARASSMENT) {        
        u32 chat_data_line_count;        
        u32 chat_data_size_uncompressed;        
        u8[-] compressed_chat_data;        
    }    
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
| 0x06 | ? / - | GmTicketType | category |  |
| - | ? / - | Map | map |  |
| - | 4 / Little | f32 | position_x |  |
| - | 4 / Little | f32 | position_y |  |
| - | 4 / Little | f32 | position_z |  |
| - | - / - | CString | message |  |
| - | - / - | CString | reserved_for_future_use |  |

If category is equal to `BEHAVIOR_HARASSMENT`:

| Offset | Size / Endianness | Type | Name | Description |
| ------ | ----------------- | ---- | ---- | ----------- |
| - | 4 / Little | u32 | chat_data_line_count |  |
| - | 4 / Little | u32 | chat_data_size_uncompressed |  |
| - | ? / - | u8[-] | compressed_chat_data |  |
