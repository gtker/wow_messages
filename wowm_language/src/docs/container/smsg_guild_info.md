## Client Version 1.12

```rust,ignore
smsg SMSG_GUILD_INFO = 0x88 {
    CString guild_name;    
    u32 created_day;    
    u32 created_month;    
    u32 created_year;    
    u32 amount_of_characters_in_guild;    
    u32 amount_of_accounts_in_guild;    
}

```
