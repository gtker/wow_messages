## Client Version 1.2, Client Version 1.12

### Wowm Representation
```rust,ignore
smsg SMSG_AUTH_RESPONSE = 0x01EE {
    WorldResult result;    
    if (result == AUTH_OK) {        
        u32 billing_time;        
        u8 billing_flags;        
        u32 billing_rested;        
    }    
    else if (result == AUTH_WAIT_QUEUE) {        
        u32 queue_position;        
    }    
}

```
