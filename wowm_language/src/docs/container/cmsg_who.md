## Client Version 1.12

```rust,ignore
cmsg CMSG_WHO = 0x62 {
    u32 minimum_level;    
    u32 maximum_level;    
    CString player_name;    
    CString guild_name;    
    u32 race_mask;    
    u32 class_mask;    
    u32 amount_of_zones;    
    u32[amount_of_zones] zones;    
    u32 amount_of_strings;    
    CString[amount_of_strings] search_strings;    
}

```
