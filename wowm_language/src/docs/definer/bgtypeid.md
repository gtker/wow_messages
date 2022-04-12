## Client Version 1.12

```rust,ignore
enum BgTypeId : u32 {
    NOT_ELIGIBLE = 0;    
    QUEUED_FOR_AV = 1;    
    QUEUED_FOR_WSG = 2;    
    QUEUED_FOR_AB = 3;    
    REMOVE_FROM_QUEUE = 0xFFFFFFFE;    
}

```
