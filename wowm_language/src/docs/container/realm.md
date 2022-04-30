## Protocol Version 2, Protocol Version 3

```rust,ignore
struct Realm {
    RealmType realm_type;    
    RealmFlag flag;    
    CString name;    
    CString address;    
    Population population;    
    u8 number_of_characters_on_realm;    
    RealmCategory category;    
    u8 realm_id;    
}

```
## Protocol Version 8

```rust,ignore
struct Realm {
    u8 realm_type;    
    u8 locked;    
    RealmFlag flag;    
    CString name;    
    CString address;    
    Population population;    
    u8 number_of_characters_on_realm;    
    RealmCategory category;    
    u8 realm_id;    
    if (flag & SPECIFY_BUILD) {        
        Version version;        
    }    
}

```
