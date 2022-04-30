## Client Version 1.12

```rust,ignore
smsg MSG_BATTLEGROUND_PLAYER_POSITIONS_Server = 0x02E9 {
    u32 amount_of_carriers = 0;    
    u32 amount_of_flag_carriers;    
    BattlegroundPlayerPosition[amount_of_flag_carriers] flag_carriers;    
}

```
