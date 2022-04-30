## Client Version 1.12

## Wowm Representation
```rust,ignore
struct Friend {
    Guid guid;    
    FriendStatus status;    
    if (status != OFFLINE) {        
        Area area;        
        u32 level;        
        Class class;        
    }    
}

```
