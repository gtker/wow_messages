## Client Version 1.12

## Wowm Representation
```rust,ignore
cmsg CMSG_SEND_MAIL = 0x0238 {
    Guid mailbox;    
    CString receiver;    
    CString subject;    
    CString body;    
    u32 unknown1;    
    u32 unknown2;    
    Guid item;    
    u32 money;    
    u32 cash_on_delivery_amount;    
    u32 unknown3;    
    u32 unknown4;    
}

```
