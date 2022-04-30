## Client Version 1.12

### Wowm Representation
```rust,ignore
enum ActivateTaxiReply : u32 {
    OK = 0;    
    UNSPECIFIEDSERVERERROR = 1;    
    NOSUCHPATH = 2;    
    NOTENOUGHMONEY = 3;    
    TOOFARAWAY = 4;    
    NOVENDORNEARBY = 5;    
    NOTVISITED = 6;    
    PLAYERBUSY = 7;    
    PLAYERALREADYMOUNTED = 8;    
    PLAYERSHAPESHIFTED = 9;    
    PLAYERMOVING = 10;    
    SAMENODE = 11;    
    NOTSTANDING = 12;    
}

```
### Type
The basic type is `u32`, a 4 byte (32 bit) little endian integer.
### Enumerators
| Enumerator | Original  | Description | Comment |
| --------- | -------- | ----------- | ------- |
| `OK` | 0 (0x00) |  |  |
| `UNSPECIFIEDSERVERERROR` | 1 (0x01) |  |  |
| `NOSUCHPATH` | 2 (0x02) |  |  |
| `NOTENOUGHMONEY` | 3 (0x03) |  |  |
| `TOOFARAWAY` | 4 (0x04) |  |  |
| `NOVENDORNEARBY` | 5 (0x05) |  |  |
| `NOTVISITED` | 6 (0x06) |  |  |
| `PLAYERBUSY` | 7 (0x07) |  |  |
| `PLAYERALREADYMOUNTED` | 8 (0x08) |  |  |
| `PLAYERSHAPESHIFTED` | 9 (0x09) |  |  |
| `PLAYERMOVING` | 10 (0x0A) |  |  |
| `SAMENODE` | 11 (0x0B) |  |  |
| `NOTSTANDING` | 12 (0x0C) |  |  |
