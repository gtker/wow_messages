## Protocol Version *

### Wowm Representation
```rust,ignore
enum Os : u32 {
    WINDOWS = "\0Win";
    OSX = "\0OSX";
    OTHER = self.value
}
```
### Type
The basic type is `u32`, a 4 byte (32 bit) little endian integer.
### Enumerators
| Enumerator | Value  | Description | Comment |
| --------- | -------- | ----------- | ------- |
| `WINDOWS` | 5728622 (0x57696E) |  |  |
| `OSX` | 5198680 (0x4F5358) |  |  |
