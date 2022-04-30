## Client Version 1.12

```rust,ignore
cmsg MSG_RAID_TARGET_UPDATE_Client = 0x321 {
    RaidTargetIndex index;    
    if (index != REQUEST_ICONS) {        
        Guid target;        
    }    
}

```
