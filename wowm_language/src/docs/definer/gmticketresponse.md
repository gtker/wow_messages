## Client Version 1.12

### Wowm Representation
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
### Type
The basic type is `u32`, a 4 byte (32 bit) little endian integer.
### Enumerators
| Enumerator | Original  | Description | Comment |
| --------- | -------- | ----------- | ------- |
| `NOT_EXIST` | 0 (0x00) |  |  |
| `ALREADY_EXIST` | 1 (0x01) |  |  |
| `CREATE_SUCCESS` | 2 (0x02) |  |  |
| `CREATE_ERROR` | 3 (0x03) |  |  |
| `UPDATE_SUCCESS` | 4 (0x04) |  |  |
| `UPDATE_ERROR` | 5 (0x05) |  |  |
| `TICKET_DELETED` | 9 (0x09) |  |  |
