## Client Version 1.12

```rust,ignore
smsg SMSG_CHAR_RENAME = 0x2C8 {
    WorldResult result;    
    if (result == RESPONSE_SUCCESS) {        
        Guid guid;        
        CString name;        
    }    
}

```
