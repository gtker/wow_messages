## Client Version 1.12

### Wowm Representation
```rust,ignore
smsg SMSG_TRANSFER_PENDING = 0x003F {
    Map map;    
    optional has_transport {    
        u32 transport;        
        Map transport_map;        
    }    
}

```
