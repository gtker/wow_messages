## Client Version 1.12

```rust,ignore
cmsg CMSG_PET_SET_ACTION = 0x0174 {
    Guid guid;    
    u32 position1;    
    u32 data1;    
    optional extra {    
        u32 position2;        
        u32 data2;        
    }    
}

```
