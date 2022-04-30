## Client Version 1.12

## Wowm Representation
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
## Enumerators
| Enumerator | Original | Decimal Value | Hex Value | Description | Comment |
| --------- | -------- | ------------- | --------- | ----------- | ------- |
| CANT_FIND_ITEM | 0 | 0 | 0x0 |  |  |
| ITEM_ALREADY_SOLD | 1 | 1 | 0x1 |  |  |
| NOT_ENOUGHT_MONEY | 2 | 2 | 0x2 |  |  |
| SELLER_DONT_LIKE_YOU | 4 | 4 | 0x4 |  |  |
| DISTANCE_TOO_FAR | 5 | 5 | 0x5 |  |  |
| ITEM_SOLD_OUT | 7 | 7 | 0x7 |  |  |
| CANT_CARRY_MORE | 8 | 8 | 0x8 |  |  |
| RANK_REQUIRE | 11 | 11 | 0xB |  |  |
| REPUTATION_REQUIRE | 12 | 12 | 0xC |  |  |
