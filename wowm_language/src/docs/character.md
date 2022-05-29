## Client Version 1.12

### Wowm Representation
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
### Body

| Offset | Size / Endianness | Type | Name | Description | Comment |
| ------ | ----------------- | ---- | ---- | ----------- | ------- |
| 0x00 | 8 / Little | [Guid](../spec/packed-guid.md) | guid |  |  |
| 0x08 | - / - | CString | name |  |  |
| - | ? / - | [Race](race.md) | race |  |  |
| - | ? / - | [Class](class.md) | class |  |  |
| - | ? / - | [Gender](gender.md) | gender |  |  |
| - | 1 / - | u8 | skin |  |  |
| - | 1 / - | u8 | face |  |  |
| - | 1 / - | u8 | hairstyle |  |  |
| - | 1 / - | u8 | haircolor |  |  |
| - | 1 / - | u8 | facialhair |  |  |
| - | 1 / - | u8 | level |  |  |
| - | ? / - | [Area](area.md) | area |  |  |
| - | ? / - | [Map](map.md) | map |  |  |
| - | 4 / Little | f32 | position_x |  |  |
| - | 4 / Little | f32 | position_y |  |  |
| - | 4 / Little | f32 | position_z |  |  |
| - | 4 / Little | u32 | guild_id |  |  |
| - | ? / - | [CharacterFlags](characterflags.md) | flags |  |  |
| - | 1 / - | u8 | first_login |  |  |
| - | 4 / Little | u32 | pet_display_id |  |  |
| - | 4 / Little | u32 | pet_level |  |  |
| - | 4 / Little | u32 | pet_familiy |  |  |
| - | ? / - | [CharacterGear](charactergear.md)[19] | equipment |  |  |
| - | 4 / Little | u32 | first_bag_display_id |  |  |
| - | 1 / - | u8 | first_bag_inventory_id |  |  |

