# SMSG_ITEM_QUERY_SINGLE_RESPONSE

## Client Version 1.12

### Wowm Representation
```rust,ignore
smsg SMSG_ITEM_QUERY_SINGLE_RESPONSE = 0x0058 {
    u32 item;
    optional found {
        ItemClass item_class;
        u32 item_sub_class;
        CString name1;
        CString name2;
        CString name3;
        CString name4;
        u32 item_display_info;
        ItemQuality quality;
        u32 flags;
        f64 buy_price;
        f64 sell_price;
        InventoryType inventory_type;
        u32 allowed_class;
        u32 allowed_race;
        u32 item_level;
        u32 required_level;
        u32 required_skill;
        u32 required_skill_rank;
        u32 required_spell;
        u32 required_honor_rank;
        u32 required_city_rank;
        u32 required_reputation_faction;
        u32 required_reputation_rank;
        u32 max_count;
        u32 stackable;
        u32 container_slots;
        ItemStat[10] stats;
        ItemDamageType[5] damages;
        u32 armor;
        u32 holy_resistance;
        u32 fire_resistance;
        u32 nature_resistance;
        u32 frost_resistance;
        u32 shadow_resistance;
        u32 arcane_resistance;
        u32 delay;
        u32 ammo_type;
        f32 ranged_range_modification;
        ItemSpells[5] spells;
        u32 bonding;
        CString description;
        u32 page_text;
        u32 language_id;
        u32 page_material;
        u32 start_quest;
        u32 lock_id;
        u32 material;
        u32 sheath;
        u32 random_property;
        u32 block;
        u32 item_set;
        u32 max_durability;
        Area area;
        Map map;
        u32 bag_family;
    }
}
```
### Header

SMSG have a header of 4 bytes.

#### SMSG Header

| Offset | Size / Endianness | Type   | Name   | Description |
| ------ | ----------------- | ------ | ------ | ----------- |
| 0x00   | 2 / Big           | uint16 | size   | Size of the rest of the message including the opcode field but not including the size field.|
| 0x02   | 2 / Little        | uint16 | opcode | Opcode that determines which fields the message contains.|

### Body

| Offset | Size / Endianness | Type | Name | Description | Comment |
| ------ | ----------------- | ---- | ---- | ----------- | ------- |
| 0x04 | 4 / Little | u32 | item |  |  |

Optionally the following fields can be present. This can only be detected by looking at the size of the message.

| Offset | Size / Endianness | Type | Name | Description | Comment |
| ------ | ----------------- | ---- | ---- | ----------- | ------- |
| 0x08 | ? / - | [ItemClass](itemclass.md) | item_class |  |  |
| - | 4 / Little | u32 | item_sub_class |  | mangoszero/vmangos/cmangos: client known only 0 subclass (and 1-2 obsolute subclasses)<br/>mangoszero/vmangos/cmangos: id from ItemSubClass.dbc |
| - | - / - | CString | name1 |  |  |
| - | - / - | CString | name2 |  |  |
| - | - / - | CString | name3 |  |  |
| - | - / - | CString | name4 |  |  |
| - | 4 / Little | u32 | item_display_info |  | mangoszero/vmangos/cmangos: id from ItemDisplayInfo.dbc |
| - | ? / - | [ItemQuality](itemquality.md) | quality |  |  |
| - | 4 / Little | u32 | flags |  |  |
| - | 8 / Little | f64 | buy_price |  |  |
| - | 8 / Little | f64 | sell_price |  |  |
| - | ? / - | [InventoryType](inventorytype.md) | inventory_type |  |  |
| - | 4 / Little | u32 | allowed_class |  |  |
| - | 4 / Little | u32 | allowed_race |  |  |
| - | 4 / Little | u32 | item_level |  |  |
| - | 4 / Little | u32 | required_level |  |  |
| - | 4 / Little | u32 | required_skill |  | cmangos/vmangos/mangoszero: id from Spell.dbc |
| - | 4 / Little | u32 | required_skill_rank |  |  |
| - | 4 / Little | u32 | required_spell |  |  |
| - | 4 / Little | u32 | required_honor_rank |  |  |
| - | 4 / Little | u32 | required_city_rank |  |  |
| - | 4 / Little | u32 | required_reputation_faction |  | cmangos/vmangos/mangoszero: id from Faction.dbc |
| - | 4 / Little | u32 | required_reputation_rank |  | cmangos/vmangos/mangoszero: send value only if reputation faction id setted ( needed for some items) |
| - | 4 / Little | u32 | max_count |  |  |
| - | 4 / Little | u32 | stackable |  |  |
| - | 4 / Little | u32 | container_slots |  |  |
| - | ? / - | [ItemStat](itemstat.md)[10] | stats |  |  |
| - | ? / - | [ItemDamageType](itemdamagetype.md)[5] | damages |  |  |
| - | 4 / Little | u32 | armor |  |  |
| - | 4 / Little | u32 | holy_resistance |  |  |
| - | 4 / Little | u32 | fire_resistance |  |  |
| - | 4 / Little | u32 | nature_resistance |  |  |
| - | 4 / Little | u32 | frost_resistance |  |  |
| - | 4 / Little | u32 | shadow_resistance |  |  |
| - | 4 / Little | u32 | arcane_resistance |  |  |
| - | 4 / Little | u32 | delay |  |  |
| - | 4 / Little | u32 | ammo_type |  |  |
| - | 4 / Little | f32 | ranged_range_modification |  |  |
| - | ? / - | [ItemSpells](itemspells.md)[5] | spells |  |  |
| - | 4 / Little | u32 | bonding |  |  |
| - | - / - | CString | description |  |  |
| - | 4 / Little | u32 | page_text |  |  |
| - | 4 / Little | u32 | language_id |  |  |
| - | 4 / Little | u32 | page_material |  |  |
| - | 4 / Little | u32 | start_quest |  | cmangos/vmangos/mangoszero: id from QuestCache.wdb |
| - | 4 / Little | u32 | lock_id |  |  |
| - | 4 / Little | u32 | material |  | cmangos/vmangos/mangoszero: id from Material.dbc |
| - | 4 / Little | u32 | sheath |  |  |
| - | 4 / Little | u32 | random_property |  | cmangos/vmangos/mangoszero: id from ItemRandomProperties.dbc |
| - | 4 / Little | u32 | block |  |  |
| - | 4 / Little | u32 | item_set |  | cmangos/vmangos/mangoszero: id from ItemSet.dbc |
| - | 4 / Little | u32 | max_durability |  |  |
| - | ? / - | [Area](area.md) | area |  |  |
| - | ? / - | [Map](map.md) | map |  |  |
| - | 4 / Little | u32 | bag_family |  |  |

