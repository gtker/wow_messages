## Client Version 1.12

### Wowm Representation
```rust,ignore
enum BgTypeId : u32 {
    NOT_ELIGIBLE = 0;    
    QUEUED_FOR_AV = 1;    
    QUEUED_FOR_WSG = 2;    
    QUEUED_FOR_AB = 3;    
    REMOVE_FROM_QUEUE = 0xFFFFFFFE;    
}
```
### Type
The basic type is `u32`, a 4 byte (32 bit) little endian integer.
### Enumerators
| Enumerator | Original  | Description | Comment |
| --------- | -------- | ----------- | ------- |
| `NOT_ELIGIBLE` | 0 (0x00) |  | Your group has joined a battleground queue, but you are not eligible |
| `QUEUED_FOR_AV` | 1 (0x01) |  | Your group has joined the queue for AV |
| `QUEUED_FOR_WSG` | 2 (0x02) |  | Your group has joined the queue for WS |
| `QUEUED_FOR_AB` | 3 (0x03) |  | Your group has joined the queue for AB |
| `REMOVE_FROM_QUEUE` | 4294967294 (0xFFFFFFFE) |  | send bg command result to show nice message |
