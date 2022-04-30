## Client Version 1.12

## Wowm Representation
```rust,ignore
enum SellItemResult : u8 {
    CANT_FIND_ITEM = 1;    
    CANT_SELL_ITEM = 2;    
    CANT_FIND_VENDOR = 3;    
    YOU_DONT_OWN_THAT_ITEM = 4;    
    UNK = 5;    
    ONLY_EMPTY_BAG = 6;    
}

```
## Enumerators
| Enumerator | Original | Decimal Value | Hex Value | Description | Comment |
| --------- | -------- | ------------- | --------- | ----------- | ------- |
| CANT_FIND_ITEM | 1 | 1 | 0x1 |  |  |
| CANT_SELL_ITEM | 2 | 2 | 0x2 |  | cmangos/vmangos/mangoszero: merchant doesn't like that item |
| CANT_FIND_VENDOR | 3 | 3 | 0x3 |  | cmangos/vmangos/mangoszero: merchant doesn't like you |
| YOU_DONT_OWN_THAT_ITEM | 4 | 4 | 0x4 |  | cmangos/vmangos/mangoszero: you don't own that item |
| UNK | 5 | 5 | 0x5 |  | cmangos/vmangos/mangoszero: nothing appears... |
| ONLY_EMPTY_BAG | 6 | 6 | 0x6 |  | cmangos/vmangos/mangoszero: can only do with empty bags |
