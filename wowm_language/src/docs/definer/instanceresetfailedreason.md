## Client Version 1.12

## Wowm Representation
```rust,ignore
enum InstanceResetFailedReason : u8 {
    GENERAL = 0;    
    OFFLINE = 1;    
    ZONING = 2;    
    SILENTLY = 3;    
}

```
## Enumerators
| Enumerator | Original | Decimal Value | Hex Value | Description | Comment |
| --------- | -------- | ------------- | --------- | ----------- | ------- |
| GENERAL | 0 | 0 | 0x0 |  | at least one player is in the instance |
| OFFLINE | 1 | 1 | 0x1 |  | at least one player is offline |
| ZONING | 2 | 2 | 0x2 |  | at least one player try to enter the instance (being teleported in) |
| SILENTLY | 3 | 3 | 0x3 |  |  |
