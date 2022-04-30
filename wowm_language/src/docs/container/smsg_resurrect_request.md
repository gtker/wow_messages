## Client Version 1.12

### Wowm Representation
```rust,ignore
smsg SMSG_RESURRECT_REQUEST = 0x015B {
    Guid guid;    
    u32 name_length;    
    CString name;    
    u8 caster_is_spirit_healer;    
    u8 respect_resurrection_timer;    
}

```
