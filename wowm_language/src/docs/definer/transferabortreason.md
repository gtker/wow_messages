## Client Version 1.12

```rust,ignore
enum TransferAbortReason : u8 {
    NONE = 0x00;    
    IS_FULL = 0x01;    
    NOT_FOUND = 0x02;    
    TOO_MANY_INSTANCES = 0x03;    
    ZONE_IS_IN_COMBAT = 0x05;    
}

```
