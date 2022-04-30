## Client Version 1.12

## Wowm Representation
```rust,ignore
enum ServerMessageType : u32 {
    SHUTDOWN_TIME = 1;    
    RESTART_TIME = 2;    
    CUSTOM = 3;    
    SHUTDOWN_CANCELLED = 4;    
    RESTART_CANCELLED = 5;    
}

```
## Enumerators
| Enumerator | Original | Decimal Value | Hex Value | Description | Comment |
| --------- | -------- | ------------- | --------- | ----------- | ------- |
| SHUTDOWN_TIME | 1 | 1 | 0x1 |  |  |
| RESTART_TIME | 2 | 2 | 0x2 |  |  |
| CUSTOM | 3 | 3 | 0x3 |  |  |
| SHUTDOWN_CANCELLED | 4 | 4 | 0x4 |  |  |
| RESTART_CANCELLED | 5 | 5 | 0x5 |  |  |
