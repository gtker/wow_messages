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
