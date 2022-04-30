## Client Version 1.12

## Wowm Representation
```rust,ignore
enum MeetingStoneStatus : u8 {
    LEAVE_QUEUE = 0;    
    JOINED_QUEUE = 1;    
    PARTY_MEMBER_LEFT_LFG = 2;    
    PARTY_MEMBER_REMOVED_PARTY_REMOVED = 3;    
    LOOKING_FOR_NEW_PARTY_IN_QUEUE = 4;    
    NONE = 5;    
}

```
## Enumerators
| Enumerator | Original | Decimal Value | Hex Value | Description | Comment |
| --------- | -------- | ------------- | --------- | ----------- | ------- |
| LEAVE_QUEUE | 0 | 0 | 0x0 |  |  |
| JOINED_QUEUE | 1 | 1 | 0x1 |  |  |
| PARTY_MEMBER_LEFT_LFG | 2 | 2 | 0x2 |  |  |
| PARTY_MEMBER_REMOVED_PARTY_REMOVED | 3 | 3 | 0x3 |  |  |
| LOOKING_FOR_NEW_PARTY_IN_QUEUE | 4 | 4 | 0x4 |  |  |
| NONE | 5 | 5 | 0x5 |  |  |
