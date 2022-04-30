## Client Version 1.12

## Wowm Representation
```rust,ignore
enum RaidInstanceMessage : u32 {
    WARNING_HOURS = 1;    
    WARNING_MIN = 2;    
    WARNING_MIN_SOON = 3;    
    WELCOME = 4;    
}

```
## Enumerators
| Enumerator | Original | Decimal Value | Hex Value | Description | Comment |
| --------- | -------- | ------------- | --------- | ----------- | ------- |
| WARNING_HOURS | 1 | 1 | 0x1 |  | WARNING! %s is scheduled to reset in %d hour(s). |
| WARNING_MIN | 2 | 2 | 0x2 |  | WARNING! %s is scheduled to reset in %d minute(s)! |
| WARNING_MIN_SOON | 3 | 3 | 0x3 |  | WARNING! %s is scheduled to reset in %d minute(s). Please exit the zone or you will be returned to your bind location! |
| WELCOME | 4 | 4 | 0x4 |  | Welcome to %s. This raid instance is scheduled to reset in %s. |
