## Client Version 1.12

### Wowm Representation
```rust,ignore
cmsg CMSG_BUG = 0x01CA {
    u32 suggestion;    
    u32 content_length;    
    CString content;    
    u32 type_length;    
    CString bug_type;    
}

```
