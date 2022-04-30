## Client Version 1.12

## Wowm Representation
```rust,ignore
enum StableResult : u8 {
    ERR_MONEY = 0x01;    
    ERR_STABLE = 0x06;    
    SUCCESS_STABLE = 0x08;    
    SUCCESS_UNSTABLE = 0x09;    
    SUCCESS_BUY_SLOT = 0x0A;    
}

```
## Enumerators
| Enumerator | Original | Decimal Value | Hex Value | Description | Comment |
| --------- | -------- | ------------- | --------- | ----------- | ------- |
| ERR_MONEY | 0x01 | 1 | 0x1 |  | you don't have enough money |
| ERR_STABLE | 0x06 | 6 | 0x6 |  | currently used in most fail cases |
| SUCCESS_STABLE | 0x08 | 8 | 0x8 |  | table success |
| SUCCESS_UNSTABLE | 0x09 | 9 | 0x9 |  | unstable/swap success |
| SUCCESS_BUY_SLOT | 0x0A | 10 | 0xA |  | buy slot success |
