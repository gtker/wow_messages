## Client Version 1.12

### Wowm Representation
```rust,ignore
smsg SMSG_FRIEND_LIST = 0x0067 {
    u8 amount_of_friends;    
    Friend[amount_of_friends] friends;    
}

```
