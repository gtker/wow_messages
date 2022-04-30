## Client Version 1.12

### Wowm Representation
```rust,ignore
smsg SMSG_IGNORE_LIST = 0x006B {
    u8 amount_of_ignored;    
    u64[amount_of_ignored] ignored;    
}

```
