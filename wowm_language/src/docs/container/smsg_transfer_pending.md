## Client Version 1.12

```rust,ignore
smsg SMSG_TRANSFER_PENDING = 0x3F {
    Map map;    
    optional has_transport {    
        u32 transport;        
        Map transport_map;        
    }    
}

```
