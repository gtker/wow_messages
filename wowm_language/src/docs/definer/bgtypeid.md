## Client Version 1.12

## Wowm Representation
```rust,ignore
enum BgTypeId : u32 {
    NOT_ELIGIBLE = 0;    
    QUEUED_FOR_AV = 1;    
    QUEUED_FOR_WSG = 2;    
    QUEUED_FOR_AB = 3;    
    REMOVE_FROM_QUEUE = 0xFFFFFFFE;    
}

```
## Enumerators
| Enumerator | Original | Decimal Value | Hex Value | Description | Comment |
| --------- | -------- | ------------- | --------- | ----------- | ------- |
| NOT_ELIGIBLE | 0 | 0 | 0x0 |  | Your group has joined a battleground queue, but you are not eligible |
| QUEUED_FOR_AV | 1 | 1 | 0x1 |  | Your group has joined the queue for AV |
| QUEUED_FOR_WSG | 2 | 2 | 0x2 |  | Your group has joined the queue for WS |
| QUEUED_FOR_AB | 3 | 3 | 0x3 |  | Your group has joined the queue for AB |
| REMOVE_FROM_QUEUE | 0xFFFFFFFE | 4294967294 | 0xFFFFFFFE |  | send bg command result to show nice message |
