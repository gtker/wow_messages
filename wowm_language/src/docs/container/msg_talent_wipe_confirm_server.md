## Client Version 1.12

### Comment

cmangos/vmangos/mangoszero returns guid 0 and unknown 0 when talents can not be reset
cmangos/vmangos/mangoszero casts spell 14876 when resetting

### Wowm Representation
```rust,ignore
smsg MSG_TALENT_WIPE_CONFIRM_Server = 0x02AA {
    Guid wiping_npc;    
    u32 cost_in_copper;    
}

```
