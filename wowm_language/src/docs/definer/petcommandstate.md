## Client Version 1.12

### Wowm Representation
```rust,ignore
enum PetCommandState : u8 {
    STAY = 0;    
    FOLLOW = 1;    
    ATTACK = 2;    
    DISMISS = 3;    
}

```
### Type
The basic type is `u8`, a 1 byte (8 bit) integer.
### Enumerators
| Enumerator | Original  | Description | Comment |
| --------- | -------- | ----------- | ------- |
| `STAY` | 0 (0x00) |  |  |
| `FOLLOW` | 1 (0x01) |  |  |
| `ATTACK` | 2 (0x02) |  |  |
| `DISMISS` | 3 (0x03) |  |  |
