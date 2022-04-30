## Client Version 1.12

## Wowm Representation
```rust,ignore
enum PartyResult : u8 {
    SUCCESS = 0;    
    BAD_PLAYER_NAME = 1;    
    TARGET_NOT_IN_GROUP = 2;    
    GROUP_FULL = 3;    
    ALREADY_IN_GROUP = 4;    
    NOT_IN_GROUP = 5;    
    NOT_LEADER = 6;    
    PLAYER_WRONG_FACTION = 7;    
    IGNORING_YOU = 8;    
}

```
## Enumerators
| Enumerator | Original | Decimal Value | Hex Value | Description | Comment |
| --------- | -------- | ------------- | --------- | ----------- | ------- |
| SUCCESS | 0 | 0 | 0x0 |  |  |
| BAD_PLAYER_NAME | 1 | 1 | 0x1 |  |  |
| TARGET_NOT_IN_GROUP | 2 | 2 | 0x2 |  |  |
| GROUP_FULL | 3 | 3 | 0x3 |  |  |
| ALREADY_IN_GROUP | 4 | 4 | 0x4 |  |  |
| NOT_IN_GROUP | 5 | 5 | 0x5 |  |  |
| NOT_LEADER | 6 | 6 | 0x6 |  |  |
| PLAYER_WRONG_FACTION | 7 | 7 | 0x7 |  |  |
| IGNORING_YOU | 8 | 8 | 0x8 |  |  |
