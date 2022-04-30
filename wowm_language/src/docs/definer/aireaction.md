## Client Version 1.12

## Wowm Representation
```rust,ignore
enum AiReaction : u32 {
    ALERT = 0;    
    FRIENDLY = 1;    
    HOSTILE = 2;    
    AFRAID = 3;    
    DESTROY = 4;    
}

```
## Enumerators
| Enumerator | Original | Decimal Value | Hex Value | Description | Comment |
| --------- | -------- | ------------- | --------- | ----------- | ------- |
| ALERT | 0 | 0 | 0x0 |  | pre-aggro (used in client packet handler) |
| FRIENDLY | 1 | 1 | 0x1 |  | (NOT used in client packet handler) |
| HOSTILE | 2 | 2 | 0x2 |  | sent on every attack, triggers aggro sound (used in client packet handler) |
| AFRAID | 3 | 3 | 0x3 |  | seen for polymorph (when AI not in control of self?) (NOT used in client packet handler) |
| DESTROY | 4 | 4 | 0x4 |  | used on object destroy (NOT used in client packet handler) |
