## Client Version 1.12

```rust,ignore
smsg MSG_INSPECT_HONOR_STATS_Server = 0x2D6 {
    Guid guid;    
    PvpRank highest_rank;    
    u32 today_honorable_and_dishonorable;    
    u16 yesterday_honorable;    
    u16 unknown1;    
    u16 last_week_honorable;    
    u16 unknown2;    
    u16 this_week_honorable;    
    u16 unknown3;    
    u32 lifetime_honorable;    
    u32 lifetime_dishonorable;    
    u32 yesterday_honor;    
    u32 last_week_honor;    
    u32 this_week_honor;    
    PvpRank last_week_standing;    
    u8 rank_progress_bar;    
}

```
