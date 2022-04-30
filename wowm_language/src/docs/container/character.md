## Client Version 1.12

```rust,ignore
struct Character {
    Guid guid;    
    CString name;    
    Race race;    
    Class class;    
    Gender gender;    
    u8 skin;    
    u8 face;    
    u8 hairstyle;    
    u8 haircolor;    
    u8 facialhair;    
    u8 level;    
    Area area;    
    Map map;    
    f32 position_x;    
    f32 position_y;    
    f32 position_z;    
    u32 guild_id;    
    CharacterFlags flags;    
    u8 first_login;    
    u32 pet_display_id;    
    u32 pet_level;    
    u32 pet_familiy;    
    CharacterGear[19] equipment;    
    u32 first_bag_display_id = 0;    
    u8 first_bag_inventory_id = 0;    
}

```
