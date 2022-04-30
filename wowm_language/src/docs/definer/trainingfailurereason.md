## Client Version 1.12

## Wowm Representation
```rust,ignore
enum TrainingFailureReason : u32 {
    UNAVAILABLE = 0;    
    NOT_ENOUGH_MONEY = 1;    
    NOT_ENOUGH_SKILL = 2;    
}

```
## Enumerators
| Enumerator | Original | Decimal Value | Hex Value | Description | Comment |
| --------- | -------- | ------------- | --------- | ----------- | ------- |
| UNAVAILABLE | 0 | 0 | 0x0 |  | Trainer service %d unavailable. |
| NOT_ENOUGH_MONEY | 1 | 1 | 0x1 |  | Not enough money for trainer service %d. |
| NOT_ENOUGH_SKILL | 2 | 2 | 0x2 |  | Not enough skill points for trainer service %d. |
