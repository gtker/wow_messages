## Client Version 1.12

## Wowm Representation
```rust,ignore
enum QuestGiverStatus : u8 {
    NONE = 0;    
    UNAVAILABLE = 1;    
    CHAT = 2;    
    INCOMPLETE = 3;    
    REWARD_REP = 4;    
    AVAILABLE = 5;    
    REWARD_OLD = 6;    
    REWARD2 = 7;    
}

```
## Enumerators
| Enumerator | Original | Decimal Value | Hex Value | Description | Comment |
| --------- | -------- | ------------- | --------- | ----------- | ------- |
| NONE | 0 | 0 | 0x0 |  |  |
| UNAVAILABLE | 1 | 1 | 0x1 |  |  |
| CHAT | 2 | 2 | 0x2 |  |  |
| INCOMPLETE | 3 | 3 | 0x3 |  |  |
| REWARD_REP | 4 | 4 | 0x4 |  |  |
| AVAILABLE | 5 | 5 | 0x5 |  |  |
| REWARD_OLD | 6 | 6 | 0x6 |  | red dot on minimap |
| REWARD2 | 7 | 7 | 0x7 |  | yellow dot on minimap |
