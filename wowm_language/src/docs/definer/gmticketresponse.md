## Client Version 1.12

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
