## Client Version 1.12

## Wowm Representation
```rust,ignore
enum PowerType : u32 {
    MANA = 0;    
    RAGE = 1;    
    FOCUS = 2;    
    ENERGY = 3;    
    HAPPINESS = 4;    
    HEALTH = 0xFFFFFFFE;    
}

```
## Enumerators
| Enumerator | Original | Decimal Value | Hex Value | Description | Comment |
| --------- | -------- | ------------- | --------- | ----------- | ------- |
| MANA | 0 | 0 | 0x0 |  | UNIT_FIELD_POWER1 |
| RAGE | 1 | 1 | 0x1 |  | UNIT_FIELD_POWER2 |
| FOCUS | 2 | 2 | 0x2 |  | UNIT_FIELD_POWER3 |
| ENERGY | 3 | 3 | 0x3 |  | UNIT_FIELD_POWER4 |
| HAPPINESS | 4 | 4 | 0x4 |  | UNIT_FIELD_POWER5 |
| HEALTH | 0xFFFFFFFE | 4294967294 | 0xFFFFFFFE |  | (-2 as signed value) |
