## Client Version 1.12

```rust,ignore
smsg SMSG_TRANSFER_ABORTED = 0x40 {
    Map map;    
    TransferAbortReason reason;    
    u8 padding = 0;    
}

```
