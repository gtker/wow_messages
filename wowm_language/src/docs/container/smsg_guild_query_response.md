## Client Version 1.12

## Wowm Representation
```rust,ignore
smsg SMSG_GUILD_QUERY_RESPONSE = 0x0055 {
    u32 id;    
    CString name;    
    CString[10] rank_names;    
    u32 emblem_style;    
    u32 emblem_color;    
    u32 border_style;    
    u32 border_color;    
    u32 background_color;    
}

```
