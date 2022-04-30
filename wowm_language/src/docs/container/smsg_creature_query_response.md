## Client Version 1.12

```rust,ignore
smsg SMSG_CREATURE_QUERY_RESPONSE = 0x0061 {
    u32 creature_entry;    
    optional found {    
        CString name1;        
        CString name2;        
        CString name3;        
        CString name4;        
        CString sub_name;        
        u32 type_flags;        
        u32 creature_type;        
        u32 creature_family;        
        u32 creature_rank;        
        u32 unknown0;        
        u32 spell_data_id;        
        u32 display_id;        
        u8 civilian;        
        u8 racial_leader;        
    }    
}

```
