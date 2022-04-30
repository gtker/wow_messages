## Client Version 1.12

### Wowm Representation
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
### Type
The basic type is `u8`, a 1 byte (8 bit) integer.
### Enumerators
| Enumerator | Original  | Description | Comment |
| --------- | -------- | ----------- | ------- |
| `CANT_FIND_ITEM` | 1 (0x01) |  |  |
| `CANT_SELL_ITEM` | 2 (0x02) |  | cmangos/vmangos/mangoszero: merchant doesn't like that item |
| `CANT_FIND_VENDOR` | 3 (0x03) |  | cmangos/vmangos/mangoszero: merchant doesn't like you |
| `YOU_DONT_OWN_THAT_ITEM` | 4 (0x04) |  | cmangos/vmangos/mangoszero: you don't own that item |
| `UNK` | 5 (0x05) |  | cmangos/vmangos/mangoszero: nothing appears... |
| `ONLY_EMPTY_BAG` | 6 (0x06) |  | cmangos/vmangos/mangoszero: can only do with empty bags |
