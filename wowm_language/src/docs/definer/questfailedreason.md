## Client Version 1.12

```rust,ignore
enum QuestFailedReason : u32 {
    DONT_HAVE_REQ = 0;    
    QUEST_FAILED_LOW_LEVEL = 1;    
    QUEST_FAILED_REQS = 2;    
    QUEST_FAILED_INVENTORY_FULL = 4;    
    QUEST_FAILED_WRONG_RACE = 6;    
    QUEST_ONLY_ONE_TIMED = 12;    
    QUEST_ALREADY_ON = 13;    
    QUEST_FAILED_DUPLICATE_ITEM = 17;    
    QUEST_FAILED_MISSING_ITEMS = 20;    
    QUEST_FAILED_NOT_ENOUGH_MONEY = 22;    
}

```
