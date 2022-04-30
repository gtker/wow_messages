## Client Version 1.12

### Comment

vmangos: this opcode can be used in two ways: Either set explicit new status or toggle old status

### Wowm Representation
```rust,ignore
cmsg CMSG_TOGGLE_PVP = 0x0253 {
    optional set {    
        u8 enable_pvp;        
    }    
}

```
