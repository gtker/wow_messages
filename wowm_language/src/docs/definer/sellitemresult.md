## Client Version 1.12

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
