## Client Version 1.12

### Wowm Representation
```rust,ignore
smsg SMSG_PET_NAME_QUERY_RESPONSE = 0x0053 {
    u32 pet_number;    
    CString name;    
    u32 pet_name_timestamp;    
}

```
