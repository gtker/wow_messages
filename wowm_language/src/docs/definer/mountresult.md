## Client Version 1.12

## Wowm Representation
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
## Enumerators
| Enumerator | Original | Decimal Value | Hex Value | Description | Comment |
| --------- | -------- | ------------- | --------- | ----------- | ------- |
| INVALIDMOUNTEE | 0 | 0 | 0x0 |  | You can't mount that unit! |
| TOOFARAWAY | 1 | 1 | 0x1 |  | That mount is too far away! |
| ALREADYMOUNTED | 2 | 2 | 0x2 |  | You're already mounted! |
| NOTMOUNTABLE | 3 | 3 | 0x3 |  | That unit can't be mounted! |
| NOTYOURPET | 4 | 4 | 0x4 |  | That mount isn't your pet! |
| OTHER | 5 | 5 | 0x5 |  | internal |
| LOOTING | 6 | 6 | 0x6 |  | You can't mount while looting! |
| RACECANTMOUNT | 7 | 7 | 0x7 |  | You can't mount because of your race! |
| SHAPESHIFTED | 8 | 8 | 0x8 |  | You can't mount while shapeshifted! |
| FORCEDDISMOUNT | 9 | 9 | 0x9 |  | You dismount before continuing. |
| OK | 10 | 10 | 0xA |  | no error |
