## Client Version 1.12

### Wowm Representation
```rust,ignore
smsg SMSG_GAMEOBJECT_QUERY_RESPONSE = 0x005F {
    u32 entry_id;    
    optional found {    
        u32 info_type;        
        u32 display_id;        
        CString name1;        
        CString name2;        
        CString name3;        
        CString name4;        
        CString name5;        
        u32[6] raw_data;        
    }    
}

```
