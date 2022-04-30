## Client Version 1.12

```rust,ignore
smsg SMSG_NAME_QUERY_RESPONSE = 0x51 {
    Guid guid;    
    CString character_name;    
    CString realm_name;    
    Race race;    
    Gender gender;    
    Class class;    
}

```
