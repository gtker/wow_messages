## Client Version 1.12

## Wowm Representation
```rust,ignore
enum MailType : u8 {
    NORMAL = 0;    
    AUCTION = 2;    
    CREATURE = 3;    
    GAMEOBJECT = 4;    
    ITEM = 5;    
}

```
## Enumerators
| Enumerator | Original | Decimal Value | Hex Value | Description | Comment |
| --------- | -------- | ------------- | --------- | ----------- | ------- |
| NORMAL | 0 | 0 | 0x0 |  |  |
| AUCTION | 2 | 2 | 0x2 |  |  |
| CREATURE | 3 | 3 | 0x3 |  | client send CMSG_CREATURE_QUERY on this mailmessagetype |
| GAMEOBJECT | 4 | 4 | 0x4 |  | client send CMSG_GAMEOBJECT_QUERY on this mailmessagetype |
| ITEM | 5 | 5 | 0x5 |  | client send CMSG_ITEM_QUERY on this mailmessagetype |
