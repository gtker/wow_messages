## Client Version 1.12

```rust,ignore
smsg SMSG_RAID_INSTANCE_INFO = 0x2CC {
    u32 amount_of_raid_infos;    
    RaidInfo[amount_of_raid_infos] raid_infos;    
}

```
