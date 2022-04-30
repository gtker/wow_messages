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
