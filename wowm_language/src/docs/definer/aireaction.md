## Client Version 1.12

### Wowm Representation
```rust,ignore
enum AiReaction : u32 {
    ALERT = 0;    
    FRIENDLY = 1;    
    HOSTILE = 2;    
    AFRAID = 3;    
    DESTROY = 4;    
}
```
### Type
The basic type is `u32`, a 4 byte (32 bit) little endian integer.
### Enumerators
| Enumerator | Original  | Description | Comment |
| --------- | -------- | ----------- | ------- |
| `ALERT` | 0 (0x00) |  | pre-aggro (used in client packet handler) |
| `FRIENDLY` | 1 (0x01) |  | (NOT used in client packet handler) |
| `HOSTILE` | 2 (0x02) |  | sent on every attack, triggers aggro sound (used in client packet handler) |
| `AFRAID` | 3 (0x03) |  | seen for polymorph (when AI not in control of self?) (NOT used in client packet handler) |
| `DESTROY` | 4 (0x04) |  | used on object destroy (NOT used in client packet handler) |
