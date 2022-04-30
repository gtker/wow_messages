## Protocol Version 2, Protocol Version 3, Protocol Version 8

### Wowm Representation
```rust,ignore
enum Population : u32 {
    GREEN_RECOMMENDED = 0x43480000;    
    RED_FULL = 0x43c80000;    
    BLUE_RECOMMENDED = 0x44160000;    
    OTHER = self.value    
}

```
### Type
The basic type is `u32`, a 4 byte (32 bit) little endian integer.
### Enumerators
| Enumerator | Original  | Description | Comment |
| --------- | -------- | ----------- | ------- |
| `GREEN_RECOMMENDED` | 1128792064 (0x43480000) |  |  |
| `RED_FULL` | 1137180672 (0x43C80000) |  |  |
| `BLUE_RECOMMENDED` | 1142292480 (0x44160000) |  |  |
