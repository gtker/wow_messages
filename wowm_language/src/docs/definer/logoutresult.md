## Client Version 1.12

## Wowm Representation
```rust,ignore
enum LogoutResult : u32 {
    SUCCESS = 0;    
    FAILURE_IN_COMBAT = 1;    
    FAILURE_FROZEN_BY_GM = 2;    
    FAILURE_JUMPING_OR_FALLING = 3;    
}

```
## Enumerators
| Enumerator | Original | Decimal Value | Hex Value | Description | Comment |
| --------- | -------- | ------------- | --------- | ----------- | ------- |
| SUCCESS | 0 | 0 | 0x0 |  |  |
| FAILURE_IN_COMBAT | 1 | 1 | 0x1 |  |  |
| FAILURE_FROZEN_BY_GM | 2 | 2 | 0x2 |  | vmangos checks for aura 9454. Has FIXME - Need the correct value. |
| FAILURE_JUMPING_OR_FALLING | 3 | 3 | 0x3 |  |  |
