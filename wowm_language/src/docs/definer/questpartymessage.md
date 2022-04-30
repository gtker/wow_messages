## Client Version 1.12

## Wowm Representation
```rust,ignore
enum QuestPartyMessage : u8 {
    SHARING_QUEST = 0;    
    CANT_TAKE_QUEST = 1;    
    ACCEPT_QUEST = 2;    
    DECLINE_QUEST = 3;    
    TOO_FAR = 4;    
    BUSY = 5;    
    LOG_FULL = 6;    
    HAVE_QUEST = 7;    
    FINISH_QUEST = 8;    
}

```
## Enumerators
| Enumerator | Original | Decimal Value | Hex Value | Description | Comment |
| --------- | -------- | ------------- | --------- | ----------- | ------- |
| SHARING_QUEST | 0 | 0 | 0x0 |  | ERR_QUEST_PUSH_SUCCESS_S |
| CANT_TAKE_QUEST | 1 | 1 | 0x1 |  | ERR_QUEST_PUSH_INVALID_S |
| ACCEPT_QUEST | 2 | 2 | 0x2 |  | ERR_QUEST_PUSH_ACCEPTED_S |
| DECLINE_QUEST | 3 | 3 | 0x3 |  | ERR_QUEST_PUSH_DECLINED_S |
| TOO_FAR | 4 | 4 | 0x4 |  | removed in 3.x |
| BUSY | 5 | 5 | 0x5 |  | ERR_QUEST_PUSH_BUSY_S |
| LOG_FULL | 6 | 6 | 0x6 |  | ERR_QUEST_PUSH_LOG_FULL_S |
| HAVE_QUEST | 7 | 7 | 0x7 |  | ERR_QUEST_PUSH_ONQUEST_S |
| FINISH_QUEST | 8 | 8 | 0x8 |  | ERR_QUEST_PUSH_ALREADY_DONE_S |
