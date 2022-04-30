## Protocol Version 3

### Wowm Representation
```rust,ignore
enum SecurityFlag : u8 {
    NONE = 0x0;    
    PIN = 0x1;    
}
```
### Type
The basic type is `u8`, a 1 byte (8 bit) integer.
### Enumerators
| Enumerator | Original  | Description | Comment |
| --------- | -------- | ----------- | ------- |
| `NONE` | 0 (0x00) |  |  |
| `PIN` | 1 (0x01) |  |  |
## Protocol Version 8

### Wowm Representation
```rust,ignore
flag SecurityFlag : u8 {
    NONE = 0x00;    
    PIN = 0x01;    
    UNKNOWN0 = 0x02;    
    AUTHENTICATOR = 0x04;    
}
```
### Type
The basic type is `u8`, a 1 byte (8 bit) integer.
### Enumerators
| Enumerator | Original  | Description | Comment |
| --------- | -------- | ----------- | ------- |
| `NONE` | 0 (0x00) |  |  |
| `PIN` | 1 (0x01) |  |  |
| `UNKNOWN0` | 2 (0x02) |  |  |
| `AUTHENTICATOR` | 4 (0x04) |  |  |
