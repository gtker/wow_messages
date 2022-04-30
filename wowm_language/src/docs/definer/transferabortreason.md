## Client Version 1.12

## Wowm Representation
```rust,ignore
enum TransferAbortReason : u8 {
    NONE = 0x00;    
    IS_FULL = 0x01;    
    NOT_FOUND = 0x02;    
    TOO_MANY_INSTANCES = 0x03;    
    ZONE_IS_IN_COMBAT = 0x05;    
}

```
## Enumerators
| Enumerator | Original | Decimal Value | Hex Value | Description | Comment |
| --------- | -------- | ------------- | --------- | ----------- | ------- |
| NONE | 0x00 | 0 | 0x0 |  |  |
| IS_FULL | 0x01 | 1 | 0x1 |  |  |
| NOT_FOUND | 0x02 | 2 | 0x2 |  |  |
| TOO_MANY_INSTANCES | 0x03 | 3 | 0x3 |  |  |
| ZONE_IS_IN_COMBAT | 0x05 | 5 | 0x5 |  |  |
