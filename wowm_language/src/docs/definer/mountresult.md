## Client Version 1.12

### Wowm Representation
```rust,ignore
enum MountResult : u32 {
    INVALIDMOUNTEE = 0;    
    TOOFARAWAY = 1;    
    ALREADYMOUNTED = 2;    
    NOTMOUNTABLE = 3;    
    NOTYOURPET = 4;    
    OTHER = 5;    
    LOOTING = 6;    
    RACECANTMOUNT = 7;    
    SHAPESHIFTED = 8;    
    FORCEDDISMOUNT = 9;    
    OK = 10;    
}

```
### Type
The basic type is `u32`, a 4 byte (32 bit) little endian integer.
### Enumerators
| Enumerator | Original  | Description | Comment |
| --------- | -------- | ----------- | ------- |
| `INVALIDMOUNTEE` | 0 (0x00) |  | You can't mount that unit! |
| `TOOFARAWAY` | 1 (0x01) |  | That mount is too far away! |
| `ALREADYMOUNTED` | 2 (0x02) |  | You're already mounted! |
| `NOTMOUNTABLE` | 3 (0x03) |  | That unit can't be mounted! |
| `NOTYOURPET` | 4 (0x04) |  | That mount isn't your pet! |
| `OTHER` | 5 (0x05) |  | internal |
| `LOOTING` | 6 (0x06) |  | You can't mount while looting! |
| `RACECANTMOUNT` | 7 (0x07) |  | You can't mount because of your race! |
| `SHAPESHIFTED` | 8 (0x08) |  | You can't mount while shapeshifted! |
| `FORCEDDISMOUNT` | 9 (0x09) |  | You dismount before continuing. |
| `OK` | 10 (0x0A) |  | no error |
