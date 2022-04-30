## Client Version 1.12

### Wowm Representation
```rust,ignore
enum BuyResult : u8 {
    CANT_FIND_ITEM = 0;    
    ITEM_ALREADY_SOLD = 1;    
    NOT_ENOUGHT_MONEY = 2;    
    SELLER_DONT_LIKE_YOU = 4;    
    DISTANCE_TOO_FAR = 5;    
    ITEM_SOLD_OUT = 7;    
    CANT_CARRY_MORE = 8;    
    RANK_REQUIRE = 11;    
    REPUTATION_REQUIRE = 12;    
}

```
### Type
The basic type is `u8`, a 1 byte (8 bit) integer.
### Enumerators
| Enumerator | Original  | Description | Comment |
| --------- | -------- | ----------- | ------- |
| `CANT_FIND_ITEM` | 0 (0x00) |  |  |
| `ITEM_ALREADY_SOLD` | 1 (0x01) |  |  |
| `NOT_ENOUGHT_MONEY` | 2 (0x02) |  |  |
| `SELLER_DONT_LIKE_YOU` | 4 (0x04) |  |  |
| `DISTANCE_TOO_FAR` | 5 (0x05) |  |  |
| `ITEM_SOLD_OUT` | 7 (0x07) |  |  |
| `CANT_CARRY_MORE` | 8 (0x08) |  |  |
| `RANK_REQUIRE` | 11 (0x0B) |  |  |
| `REPUTATION_REQUIRE` | 12 (0x0C) |  |  |
