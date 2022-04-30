## Client Version 1.12

### Wowm Representation
```rust,ignore
smsg MSG_CORPSE_QUERY_Server = 0x0216 {
    CorpseQueryResult result;    
    if (result == FOUND) {        
        Map map;        
        f32 position_x;        
        f32 position_y;        
        f32 position_z;        
        Map corpse_map;        
    }    
}

```
