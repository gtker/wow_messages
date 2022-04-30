## Client Version 1.12

## Wowm Representation
```rust,ignore
cmsg MSG_RAID_TARGET_UPDATE_Client = 0x0321 {
    RaidTargetIndex index;    
    if (index != REQUEST_ICONS) {        
        Guid target;        
    }    
}

```
