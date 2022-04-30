## Client Version 1.12

## Wowm Representation
```rust,ignore
enum GmTicketResponse : u32 {
    NOT_EXIST = 0;    
    ALREADY_EXIST = 1;    
    CREATE_SUCCESS = 2;    
    CREATE_ERROR = 3;    
    UPDATE_SUCCESS = 4;    
    UPDATE_ERROR = 5;    
    TICKET_DELETED = 9;    
}

```
## Enumerators
| Enumerator | Original | Decimal Value | Hex Value | Description | Comment |
| --------- | -------- | ------------- | --------- | ----------- | ------- |
| NOT_EXIST | 0 | 0 | 0x0 |  |  |
| ALREADY_EXIST | 1 | 1 | 0x1 |  |  |
| CREATE_SUCCESS | 2 | 2 | 0x2 |  |  |
| CREATE_ERROR | 3 | 3 | 0x3 |  |  |
| UPDATE_SUCCESS | 4 | 4 | 0x4 |  |  |
| UPDATE_ERROR | 5 | 5 | 0x5 |  |  |
| TICKET_DELETED | 9 | 9 | 0x9 |  |  |
