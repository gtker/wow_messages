## Client Version 1.12

```rust,ignore
smsg SMSG_PETITION_QUERY_RESPONSE = 0x1C7 {
    Guid petition_guid;    
    Guid charter_owner;    
    CString guild_name;    
    CString body_text;    
    u32 unknown_flags;    
    u32 minimum_signatures;    
    u32 maximum_signatures;    
    u32 deadline;    
    u32 issue_date;    
    u32 allowed_guild_id;    
    u32 allowed_classes;    
    u32 allowed_races;    
    u16 allowed_genders;    
    u32 allowed_minimum_level;    
    u32 allowed_maximum_level;    
    u32 todo_amount_of_signers;    
    u32 number_of_choices;    
}

```
