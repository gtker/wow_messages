## Client Version 1.12

## Wowm Representation
```rust,ignore
enum PetTameFailureReason : u8 {
    INVALIDCREATURE = 1;    
    TOOMANY = 2;    
    CREATUREALREADYOWNED = 3;    
    NOTTAMEABLE = 4;    
    ANOTHERSUMMONACTIVE = 5;    
    UNITSCANTTAME = 6;    
    NOPETAVAILABLE = 7;    
    INTERNALERROR = 8;    
    TOOHIGHLEVEL = 9;    
    DEAD = 10;    
    NOTDEAD = 11;    
    UNKNOWNERROR = 12;    
}

```
## Enumerators
| Enumerator | Original | Decimal Value | Hex Value | Description | Comment |
| --------- | -------- | ------------- | --------- | ----------- | ------- |
| INVALIDCREATURE | 1 | 1 | 0x1 |  |  |
| TOOMANY | 2 | 2 | 0x2 |  |  |
| CREATUREALREADYOWNED | 3 | 3 | 0x3 |  |  |
| NOTTAMEABLE | 4 | 4 | 0x4 |  |  |
| ANOTHERSUMMONACTIVE | 5 | 5 | 0x5 |  |  |
| UNITSCANTTAME | 6 | 6 | 0x6 |  |  |
| NOPETAVAILABLE | 7 | 7 | 0x7 |  | not used in taming |
| INTERNALERROR | 8 | 8 | 0x8 |  |  |
| TOOHIGHLEVEL | 9 | 9 | 0x9 |  |  |
| DEAD | 10 | 10 | 0xA |  | not used in taming |
| NOTDEAD | 11 | 11 | 0xB |  | not used in taming |
| UNKNOWNERROR | 12 | 12 | 0xC |  |  |
