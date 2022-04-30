## Client Version 1.12

## Wowm Representation
```rust,ignore
enum PetitionTurnInResult : u32 {
    OK = 0;    
    ALREADY_SIGNED = 1;    
    ALREADY_IN_GUILD = 2;    
    CANT_SIGN_OWN = 3;    
    NEED_MORE = 4;    
    NOT_SERVER = 5;    
}

```
## Enumerators
| Enumerator | Original | Decimal Value | Hex Value | Description | Comment |
| --------- | -------- | ------------- | --------- | ----------- | ------- |
| OK | 0 | 0 | 0x0 |  |  |
| ALREADY_SIGNED | 1 | 1 | 0x1 |  |  |
| ALREADY_IN_GUILD | 2 | 2 | 0x2 |  |  |
| CANT_SIGN_OWN | 3 | 3 | 0x3 |  |  |
| NEED_MORE | 4 | 4 | 0x4 |  |  |
| NOT_SERVER | 5 | 5 | 0x5 |  |  |
