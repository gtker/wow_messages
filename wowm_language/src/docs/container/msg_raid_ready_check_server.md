## Client Version 1.12

```rust,ignore
smsg MSG_RAID_READY_CHECK_Server = 0x322 {
    optional state_check {    
        Guid guid;        
        u8 state;        
    }    
}

```
