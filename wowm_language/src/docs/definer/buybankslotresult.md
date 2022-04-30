## Client Version 1.12

## Wowm Representation
```rust,ignore
enum BuyBankSlotResult : u32 {
    FAILED_TOO_MANY = 0;    
    INSUFFICIENT_FUNDS = 1;    
    NOTBANKER = 2;    
    OK = 3;    
}

```
## Type
The basic type is `u32`, a 4 byte (32 bit) little endian integer.
## Enumerators
| Enumerator | Original  | Description | Comment |
| --------- | -------- | ----------- | ------- |
| `FAILED_TOO_MANY` | 0 (0x00) |  |  |
| `INSUFFICIENT_FUNDS` | 1 (0x01) |  |  |
| `NOTBANKER` | 2 (0x02) |  |  |
| `OK` | 3 (0x03) |  |  |
