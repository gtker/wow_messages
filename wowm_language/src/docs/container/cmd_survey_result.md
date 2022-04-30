## Protocol Version 3

```rust,ignore
clogin CMD_SURVEY_RESULT = 0x04 {
    u32 survey_id;    
    u8 error;    
    u16 compressed_data_length;    
    u8[compressed_data_length] data;    
}

```
