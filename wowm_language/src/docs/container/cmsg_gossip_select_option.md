## Client Version 1.12

```rust,ignore
cmsg CMSG_GOSSIP_SELECT_OPTION = 0x017C {
    Guid guid;    
    u32 gossip_list_id;    
    optional unknown {    
        CString code;        
    }    
}

```
