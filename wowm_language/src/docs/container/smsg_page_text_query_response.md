## Client Version 1.12

## Wowm Representation
```rust,ignore
smsg SMSG_PAGE_TEXT_QUERY_RESPONSE = 0x005B {
    u32 page_id;    
    CString text;    
    u32 next_page_id;    
}

```
