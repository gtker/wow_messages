## Client Version 1.12

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
