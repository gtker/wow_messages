## Client Version 1.12

### Wowm Representation
```rust,ignore
smsg SMSG_CHANNEL_LIST = 0x009B {
    CString channel_name;    
    u8 channel_flags;    
    u32 amount_of_members;    
    ChannelMember[amount_of_members] members;    
}

```
