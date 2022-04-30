## Client Version 1.12

## Wowm Representation
```rust,ignore
enum StatusId : u8 {
    NONE = 0;    
    WAIT_QUEUE = 1;    
    WAIT_JOIN = 2;    
    IN_PROGRESS = 3;    
    WAIT_LEAVE = 4;    
}

```
## Enumerators
| Enumerator | Original | Decimal Value | Hex Value | Description | Comment |
| --------- | -------- | ------------- | --------- | ----------- | ------- |
| NONE | 0 | 0 | 0x0 |  | first status, should mean bg is not instance |
| WAIT_QUEUE | 1 | 1 | 0x1 |  | means bg is empty and waiting for queue |
| WAIT_JOIN | 2 | 2 | 0x2 |  | this means, that BG has already started and it is waiting for more players |
| IN_PROGRESS | 3 | 3 | 0x3 |  | means bg is running |
| WAIT_LEAVE | 4 | 4 | 0x4 |  | means some faction has won BG and it is ending |
