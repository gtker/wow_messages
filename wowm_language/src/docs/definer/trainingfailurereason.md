## Client Version 1.12

## Wowm Representation
```rust,ignore
enum TrainingFailureReason : u32 {
    UNAVAILABLE = 0;    
    NOT_ENOUGH_MONEY = 1;    
    NOT_ENOUGH_SKILL = 2;    
}

```
## Type
The basic type is `u32`, a 4 byte (32 bit) little endian integer.
## Enumerators
| Enumerator | Original  | Description | Comment |
| --------- | -------- | ----------- | ------- |
| `UNAVAILABLE` | 0 (0x00) |  | Trainer service %d unavailable. |
| `NOT_ENOUGH_MONEY` | 1 (0x01) |  | Not enough money for trainer service %d. |
| `NOT_ENOUGH_SKILL` | 2 (0x02) |  | Not enough skill points for trainer service %d. |
