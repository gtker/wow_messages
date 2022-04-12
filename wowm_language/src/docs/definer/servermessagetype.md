## Client Version 1.12

```rust,ignore
enum ServerMessageType : u32 {
    SHUTDOWN_TIME = 1;    
    RESTART_TIME = 2;    
    CUSTOM = 3;    
    SHUTDOWN_CANCELLED = 4;    
    RESTART_CANCELLED = 5;    
}

```
