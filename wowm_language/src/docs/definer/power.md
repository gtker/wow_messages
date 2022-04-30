## Client Version 1.12

## Wowm Representation
```rust,ignore
enum Power : u8 {
    MANA = 0;    
    RAGE = 1;    
    FOCUS = 2;    
    ENERGY = 3;    
    HAPPINESS = 4;    
    MAX_POWERS = 5;    
    ALL = 127;    
}

```
## Enumerators
| Enumerator | Original | Decimal Value | Hex Value | Description | Comment |
| --------- | -------- | ------------- | --------- | ----------- | ------- |
| MANA | 0 | 0 | 0x0 |  | The most common one, mobs usually have this or rage |
| RAGE | 1 | 1 | 0x1 |  | This is what warriors use to cast their spells |
| FOCUS | 2 | 2 | 0x2 |  | Used by hunters after Cataclysm (4.x) |
| ENERGY | 3 | 3 | 0x3 |  | Used by rouges to do their spells |
| HAPPINESS | 4 | 4 | 0x4 |  | Hunter's pet's happiness affect their damage |
| MAX_POWERS | 5 | 5 | 0x5 |  |  |
| ALL | 127 | 127 | 0x7F |  | default for class? - need check for TBC |
