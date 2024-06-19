# UpdateMask

An UpdateMask is a variable length way of sending known fields to the client.
It is represented by a byte mask that decides which fields are sent afterwards.

## Representation

The UpdateMask starts with a single u8 that decides how many **u32 mask blocks** will follow.
The bit pattern in these mask blocks determine how many additional u32s of data will follow and how to interpret that data.

## Examples

The absolute minimum amount of fields that need to be set to log a player into the world are:

* `OBJECT_GUID`, an Object field.
* `OBJECT_TYPE`, an Object field.
* `UNIT_HEALTH`, a Unit field.
* `UNIT_BYTES_0`, a Unit field.

To find out how many mask blocks we need to send we take the highest offset field (`UNIT_BYTES_0` with 36) and divide it by the amount of bits in a u32 and rounding up.
This gives us 2 mask blocks that we need to send.

To figure out which bits must be set on the mask blocks we look up the offset and sizes for our fields.
A size of 1 means that only the bit at the offset should be set.
A size of 2 means that the bit at the offset and the bit at the offset + 1 should be set, and so on.

Realistically you will have to store the mask blocks in an array of u32s.
The correct index and bit position can then be found by dividing to find the index, and modulus to find the bit position within that index.

For `UNIT_BYTES_0` with an offset of 36, this means that our index and bit position is:
```rust, ignore
index = 36 / 32 = 1
bit = 36 % 32 = 4
```

We do this for every field.

We then send, in order:

1. A `u8` with the amount of mask bytes.
2. The mask bytes as `u32`s.
3. The data values as `u32`s.

# Examples

The following Python example uses the `asyncio` module.
Additionally `read_int` is a function that reads a certain amount of bytes and returns it as an integer.
So `read_int(reader, 1)` would return a `u8` and `read_int(reader, 4)` would return a `u32`.

The `dict[int, int]` is a dictionary of integers, also called a hash map.
The keys are the offsets defined below, and the values are `u32` data values.

The write function uses Python's `struct` module to pack the data into a byte array.
The [reference for `struct`](https://docs.python.org/3/library/struct.html#format-characters) lists the different format characters used.

The Python code below does not have the best API; users will have to fill out the `dict` using the offsets defined as constants without any type safety, merging of fields for Guids, or splitting of fields into smaller types.
When designing your own `UpdateMask` type consider what API it allows you to expose.

```python
class UpdateMask:
    fields: dict[int, int]

    @staticmethod
    async def read(reader: asyncio.StreamReader):
        amount_of_blocks = await read_int(reader, 1)

        blocks = []
        for _ in range(0, amount_of_blocks):
            blocks.append(await read_int(reader, 4))

        fields = {}
        for block_index, block in enumerate(blocks):
            for bit in range(0, 32):
                if block & 1 << bit:
                    value = await read_int(reader, 4)
                    key = block_index * 32 + bit
                    fields[key] = value

        return UpdateMask(fields=fields)

    def write(self, fmt, data):
        highest_key = max(self.fields)
        amount_of_blocks = highest_key // 32
        if highest_key % 32 != 0:
            amount_of_blocks += 1

        fmt += 'B'
        data.append(amount_of_blocks)

        blocks = [0] * amount_of_blocks

        for key in self.fields:
            block = key // 32
            index = key % 32
            blocks[block] |= 1 << index

        fmt += f'{len(blocks)}I'
        data.extend(blocks)

        for key in sorted(self.fields):
            fmt += 'I'
            data.append(self.fields[key])

        return fmt, data

    def size(self):
        highest_key = max(self.fields)
        amount_of_blocks = highest_key // 32

        extra = highest_key % 32
        if extra != 0:
            extra = 1
        else:
            extra = 0

        return 1 + (extra + amount_of_blocks + len(self.fields)) * 4
```

## Lookup Table
### Version 1.12

Taken from [vmangos](https://github.com/vmangos/core/blob/4b2a5173b0ca4917dfe91aa7b87d84232fd7203c/src/game/Objects/UpdateFields_1_12_1.cpp#L5) with some modifications.

Fields that all objects have:

| Name | Offset | Size | Type |
|------|--------|------|------|
|`OBJECT_GUID`| 0x0000 | 2 | GUID |
|`OBJECT_TYPE`| 0x0002 | 1 | INT |
|`OBJECT_ENTRY`| 0x0003 | 1 | INT |
|`OBJECT_SCALE_X`| 0x0004 | 1 | FLOAT |


Fields that all items have:

| Name | Offset | Size | Type |
|------|--------|------|------|
|`ITEM_OWNER`| 0x0006 | 2 | GUID |
|`ITEM_CONTAINED`| 0x0008 | 2 | GUID |
|`ITEM_CREATOR`| 0x000a | 2 | GUID |
|`ITEM_GIFTCREATOR`| 0x000c | 2 | GUID |
|`ITEM_STACK_COUNT`| 0x000e | 1 | INT |
|`ITEM_DURATION`| 0x000f | 1 | INT |
|`ITEM_SPELL_CHARGES`| 0x0010 | 5 | INT |
|`ITEM_FLAGS`| 0x0015 | 1 | INT |
|`ITEM_ENCHANTMENT`| 0x0016 | 21 | INT |
|`ITEM_PROPERTY_SEED`| 0x002b | 1 | INT |
|`ITEM_RANDOM_PROPERTIES_ID`| 0x002c | 1 | INT |
|`ITEM_ITEM_TEXT_ID`| 0x002d | 1 | INT |
|`ITEM_DURABILITY`| 0x002e | 1 | INT |
|`ITEM_MAXDURABILITY`| 0x002f | 1 | INT |


Fields that all containers have:

| Name | Offset | Size | Type |
|------|--------|------|------|
|`CONTAINER_NUM_SLOTS`| 0x0030 | 1 | INT |
|`CONTAINER_SLOT_1`| 0x0032 | 72 | GUID |


Fields that all units have:

| Name | Offset | Size | Type |
|------|--------|------|------|
|`UNIT_CHARM`| 0x0006 | 2 | GUID |
|`UNIT_SUMMON`| 0x0008 | 2 | GUID |
|`UNIT_CHARMEDBY`| 0x000a | 2 | GUID |
|`UNIT_SUMMONEDBY`| 0x000c | 2 | GUID |
|`UNIT_CREATEDBY`| 0x000e | 2 | GUID |
|`UNIT_TARGET`| 0x0010 | 2 | GUID |
|`UNIT_PERSUADED`| 0x0012 | 2 | GUID |
|`UNIT_CHANNEL_OBJECT`| 0x0014 | 2 | GUID |
|`UNIT_HEALTH`| 0x0016 | 1 | INT |
|`UNIT_POWER1`| 0x0017 | 1 | INT |
|`UNIT_POWER2`| 0x0018 | 1 | INT |
|`UNIT_POWER3`| 0x0019 | 1 | INT |
|`UNIT_POWER4`| 0x001a | 1 | INT |
|`UNIT_POWER5`| 0x001b | 1 | INT |
|`UNIT_MAXHEALTH`| 0x001c | 1 | INT |
|`UNIT_MAXPOWER1`| 0x001d | 1 | INT |
|`UNIT_MAXPOWER2`| 0x001e | 1 | INT |
|`UNIT_MAXPOWER3`| 0x001f | 1 | INT |
|`UNIT_MAXPOWER4`| 0x0020 | 1 | INT |
|`UNIT_MAXPOWER5`| 0x0021 | 1 | INT |
|`UNIT_LEVEL`| 0x0022 | 1 | INT |
|`UNIT_FACTIONTEMPLATE`| 0x0023 | 1 | INT |
|`UNIT_BYTES_0`| 0x0024 | 1 | BYTES |
|`UNIT_VIRTUAL_ITEM_SLOT_DISPLAY`| 0x0025 | 3 | INT |
|`UNIT_VIRTUAL_ITEM_INFO`| 0x0028 | 6 | BYTES |
|`UNIT_FLAGS`| 0x002e | 1 | INT |
|`UNIT_AURA`| 0x002f | 48 | INT |
|`UNIT_AURAFLAGS`| 0x005f | 6 | BYTES |
|`UNIT_AURALEVELS`| 0x0065 | 12 | BYTES |
|`UNIT_AURAAPPLICATIONS`| 0x0071 | 12 | BYTES |
|`UNIT_AURASTATE`| 0x007d | 1 | INT |
|`UNIT_BASEATTACKTIME`| 0x007e | 2 | INT |
|`UNIT_RANGEDATTACKTIME`| 0x0080 | 1 | INT |
|`UNIT_BOUNDINGRADIUS`| 0x0081 | 1 | FLOAT |
|`UNIT_COMBATREACH`| 0x0082 | 1 | FLOAT |
|`UNIT_DISPLAYID`| 0x0083 | 1 | INT |
|`UNIT_NATIVEDISPLAYID`| 0x0084 | 1 | INT |
|`UNIT_MOUNTDISPLAYID`| 0x0085 | 1 | INT |
|`UNIT_MINDAMAGE`| 0x0086 | 1 | FLOAT |
|`UNIT_MAXDAMAGE`| 0x0087 | 1 | FLOAT |
|`UNIT_MINOFFHANDDAMAGE`| 0x0088 | 1 | FLOAT |
|`UNIT_MAXOFFHANDDAMAGE`| 0x0089 | 1 | FLOAT |
|`UNIT_BYTES_1`| 0x008a | 1 | BYTES |
|`UNIT_PETNUMBER`| 0x008b | 1 | INT |
|`UNIT_PET_NAME_TIMESTAMP`| 0x008c | 1 | INT |
|`UNIT_PETEXPERIENCE`| 0x008d | 1 | INT |
|`UNIT_PETNEXTLEVELEXP`| 0x008e | 1 | INT |
|`UNIT_DYNAMIC_FLAGS`| 0x008f | 1 | INT |
|`UNIT_CHANNEL_SPELL`| 0x0090 | 1 | INT |
|`UNIT_MOD_CAST_SPEED`| 0x0091 | 1 | FLOAT |
|`UNIT_CREATED_BY_SPELL`| 0x0092 | 1 | INT |
|`UNIT_NPC_FLAGS`| 0x0093 | 1 | INT |
|`UNIT_NPC_EMOTESTATE`| 0x0094 | 1 | INT |
|`UNIT_TRAINING_POINTS`| 0x0095 | 1 | TWO_SHORT |
|`UNIT_STRENGTH`| 0x0096 | 1 | INT |
|`UNIT_AGILITY`| 0x0097 | 1 | INT |
|`UNIT_STAMINA`| 0x0098 | 1 | INT |
|`UNIT_INTELLECT`| 0x0099 | 1 | INT |
|`UNIT_SPIRIT`| 0x009a | 1 | INT |
|`UNIT_NORMAL_RESISTANCE`| 0x009b | 1 | INT |
|`UNIT_HOLY_RESISTANCE`| 0x009c | 1 | INT |
|`UNIT_FIRE_RESISTANCE`| 0x009d | 1 | INT |
|`UNIT_NATURE_RESISTANCE`| 0x009e | 1 | INT |
|`UNIT_FROST_RESISTANCE`| 0x009f | 1 | INT |
|`UNIT_SHADOW_RESISTANCE`| 0x00a0 | 1 | INT |
|`UNIT_ARCANE_RESISTANCE`| 0x00a1 | 1 | INT |
|`UNIT_BASE_MANA`| 0x00a2 | 1 | INT |
|`UNIT_BASE_HEALTH`| 0x00a3 | 1 | INT |
|`UNIT_BYTES_2`| 0x00a4 | 1 | BYTES |
|`UNIT_ATTACK_POWER`| 0x00a5 | 1 | INT |
|`UNIT_ATTACK_POWER_MODS`| 0x00a6 | 1 | TWO_SHORT |
|`UNIT_ATTACK_POWER_MULTIPLIER`| 0x00a7 | 1 | FLOAT |
|`UNIT_RANGED_ATTACK_POWER`| 0x00a8 | 1 | INT |
|`UNIT_RANGED_ATTACK_POWER_MODS`| 0x00a9 | 1 | TWO_SHORT |
|`UNIT_RANGED_ATTACK_POWER_MULTIPLIER`| 0x00aa | 1 | FLOAT |
|`UNIT_MINRANGEDDAMAGE`| 0x00ab | 1 | FLOAT |
|`UNIT_MAXRANGEDDAMAGE`| 0x00ac | 1 | FLOAT |
|`UNIT_POWER_COST_MODIFIER`| 0x00ad | 7 | INT |
|`UNIT_POWER_COST_MULTIPLIER`| 0x00b4 | 7 | FLOAT |


Fields that all players have:

| Name | Offset | Size | Type |
|------|--------|------|------|
|`PLAYER_DUEL_ARBITER`| 0x00bc | 2 | GUID |
|`PLAYER_FLAGS`| 0x00be | 1 | INT |
|`PLAYER_GUILDID`| 0x00bf | 1 | INT |
|`PLAYER_GUILDRANK`| 0x00c0 | 1 | INT |
|`PLAYER_FEATURES`| 0x00c1 | 1 | BYTES |
|`PLAYER_BYTES_2`| 0x00c2 | 1 | BYTES |
|`PLAYER_BYTES_3`| 0x00c3 | 1 | BYTES |
|`PLAYER_DUEL_TEAM`| 0x00c4 | 1 | INT |
|`PLAYER_GUILD_TIMESTAMP`| 0x00c5 | 1 | INT |
|`PLAYER_QUEST_LOG_1_1`| 0x00c6 | 1 | INT |
|`PLAYER_QUEST_LOG_1_2`| 0x00c7 | 2 | INT |
|`PLAYER_QUEST_LOG_2_1`| 0x00c9 | 1 | INT |
|`PLAYER_QUEST_LOG_2_2`| 0x00ca | 2 | INT |
|`PLAYER_QUEST_LOG_3_1`| 0x00cc | 1 | INT |
|`PLAYER_QUEST_LOG_3_2`| 0x00cd | 2 | INT |
|`PLAYER_QUEST_LOG_4_1`| 0x00cf | 1 | INT |
|`PLAYER_QUEST_LOG_4_2`| 0x00d0 | 2 | INT |
|`PLAYER_QUEST_LOG_5_1`| 0x00d2 | 1 | INT |
|`PLAYER_QUEST_LOG_5_2`| 0x00d3 | 2 | INT |
|`PLAYER_QUEST_LOG_6_1`| 0x00d5 | 1 | INT |
|`PLAYER_QUEST_LOG_6_2`| 0x00d6 | 2 | INT |
|`PLAYER_QUEST_LOG_7_1`| 0x00d8 | 1 | INT |
|`PLAYER_QUEST_LOG_7_2`| 0x00d9 | 2 | INT |
|`PLAYER_QUEST_LOG_8_1`| 0x00db | 1 | INT |
|`PLAYER_QUEST_LOG_8_2`| 0x00dc | 2 | INT |
|`PLAYER_QUEST_LOG_9_1`| 0x00de | 1 | INT |
|`PLAYER_QUEST_LOG_9_2`| 0x00df | 2 | INT |
|`PLAYER_QUEST_LOG_10_1`| 0x00e1 | 1 | INT |
|`PLAYER_QUEST_LOG_10_2`| 0x00e2 | 2 | INT |
|`PLAYER_QUEST_LOG_11_1`| 0x00e4 | 1 | INT |
|`PLAYER_QUEST_LOG_11_2`| 0x00e5 | 2 | INT |
|`PLAYER_QUEST_LOG_12_1`| 0x00e7 | 1 | INT |
|`PLAYER_QUEST_LOG_12_2`| 0x00e8 | 2 | INT |
|`PLAYER_QUEST_LOG_13_1`| 0x00ea | 1 | INT |
|`PLAYER_QUEST_LOG_13_2`| 0x00eb | 2 | INT |
|`PLAYER_QUEST_LOG_14_1`| 0x00ed | 1 | INT |
|`PLAYER_QUEST_LOG_14_2`| 0x00ee | 2 | INT |
|`PLAYER_QUEST_LOG_15_1`| 0x00f0 | 1 | INT |
|`PLAYER_QUEST_LOG_15_2`| 0x00f1 | 2 | INT |
|`PLAYER_QUEST_LOG_16_1`| 0x00f3 | 1 | INT |
|`PLAYER_QUEST_LOG_16_2`| 0x00f4 | 2 | INT |
|`PLAYER_QUEST_LOG_17_1`| 0x00f6 | 1 | INT |
|`PLAYER_QUEST_LOG_17_2`| 0x00f7 | 2 | INT |
|`PLAYER_QUEST_LOG_18_1`| 0x00f9 | 1 | INT |
|`PLAYER_QUEST_LOG_18_2`| 0x00fa | 2 | INT |
|`PLAYER_QUEST_LOG_19_1`| 0x00fc | 1 | INT |
|`PLAYER_QUEST_LOG_19_2`| 0x00fd | 2 | INT |
|`PLAYER_QUEST_LOG_20_1`| 0x00ff | 1 | INT |
|`PLAYER_QUEST_LOG_20_2`| 0x0100 | 2 | INT |
|`PLAYER_VISIBLE_ITEM`| 0x0102 | 228 | CUSTOM |
|`PLAYER_FIELD_INV`| 0x01e6 | 226 | CUSTOM |
|`PLAYER_FARSIGHT`| 0x02c8 | 2 | GUID |
|`PLAYER_FIELD_COMBO_TARGET`| 0x02ca | 2 | GUID |
|`PLAYER_XP`| 0x02cc | 1 | INT |
|`PLAYER_NEXT_LEVEL_XP`| 0x02cd | 1 | INT |
|`PLAYER_SKILL_INFO`| 0x02ce | 384 | CUSTOM |
|`PLAYER_CHARACTER_POINTS1`| 0x044e | 1 | INT |
|`PLAYER_CHARACTER_POINTS2`| 0x044f | 1 | INT |
|`PLAYER_TRACK_CREATURES`| 0x0450 | 1 | INT |
|`PLAYER_TRACK_RESOURCES`| 0x0451 | 1 | INT |
|`PLAYER_BLOCK_PERCENTAGE`| 0x0452 | 1 | FLOAT |
|`PLAYER_DODGE_PERCENTAGE`| 0x0453 | 1 | FLOAT |
|`PLAYER_PARRY_PERCENTAGE`| 0x0454 | 1 | FLOAT |
|`PLAYER_CRIT_PERCENTAGE`| 0x0455 | 1 | FLOAT |
|`PLAYER_RANGED_CRIT_PERCENTAGE`| 0x0456 | 1 | FLOAT |
|`PLAYER_EXPLORED_ZONES_1`| 0x0457 | 64 | BYTES |
|`PLAYER_REST_STATE_EXPERIENCE`| 0x0497 | 1 | INT |
|`PLAYER_FIELD_COINAGE`| 0x0498 | 1 | INT |
|`PLAYER_FIELD_POSSTAT0`| 0x0499 | 1 | INT |
|`PLAYER_FIELD_POSSTAT1`| 0x049a | 1 | INT |
|`PLAYER_FIELD_POSSTAT2`| 0x049b | 1 | INT |
|`PLAYER_FIELD_POSSTAT3`| 0x049c | 1 | INT |
|`PLAYER_FIELD_POSSTAT4`| 0x049d | 1 | INT |
|`PLAYER_FIELD_NEGSTAT0`| 0x049e | 1 | INT |
|`PLAYER_FIELD_NEGSTAT1`| 0x049f | 1 | INT |
|`PLAYER_FIELD_NEGSTAT2`| 0x04a0 | 1 | INT |
|`PLAYER_FIELD_NEGSTAT3`| 0x04a1 | 1 | INT |
|`PLAYER_FIELD_NEGSTAT4`| 0x04a2 | 1 | INT |
|`PLAYER_FIELD_RESISTANCEBUFFMODSPOSITIVE`| 0x04a3 | 7 | INT |
|`PLAYER_FIELD_RESISTANCEBUFFMODSNEGATIVE`| 0x04aa | 7 | INT |
|`PLAYER_FIELD_MOD_DAMAGE_DONE_POS`| 0x04b1 | 7 | INT |
|`PLAYER_FIELD_MOD_DAMAGE_DONE_NEG`| 0x04b8 | 7 | INT |
|`PLAYER_FIELD_MOD_DAMAGE_DONE_PCT`| 0x04bf | 7 | INT |
|`PLAYER_FIELD_BYTES`| 0x04c6 | 1 | BYTES |
|`PLAYER_AMMO_ID`| 0x04c7 | 1 | INT |
|`PLAYER_SELF_RES_SPELL`| 0x04c8 | 1 | INT |
|`PLAYER_FIELD_PVP_MEDALS`| 0x04c9 | 1 | INT |
|`PLAYER_FIELD_BUYBACK_PRICE_1`| 0x04ca | 12 | INT |
|`PLAYER_FIELD_BUYBACK_TIMESTAMP_1`| 0x04d6 | 12 | INT |
|`PLAYER_FIELD_SESSION_KILLS`| 0x04e2 | 1 | TWO_SHORT |
|`PLAYER_FIELD_YESTERDAY_KILLS`| 0x04e3 | 1 | TWO_SHORT |
|`PLAYER_FIELD_LAST_WEEK_KILLS`| 0x04e4 | 1 | TWO_SHORT |
|`PLAYER_FIELD_THIS_WEEK_KILLS`| 0x04e5 | 1 | TWO_SHORT |
|`PLAYER_FIELD_THIS_WEEK_CONTRIBUTION`| 0x04e6 | 1 | INT |
|`PLAYER_FIELD_LIFETIME_HONORBALE_KILLS`| 0x04e7 | 1 | INT |
|`PLAYER_FIELD_LIFETIME_DISHONORBALE_KILLS`| 0x04e8 | 1 | INT |
|`PLAYER_FIELD_YESTERDAY_CONTRIBUTION`| 0x04e9 | 1 | INT |
|`PLAYER_FIELD_LAST_WEEK_CONTRIBUTION`| 0x04ea | 1 | INT |
|`PLAYER_FIELD_LAST_WEEK_RANK`| 0x04eb | 1 | INT |
|`PLAYER_FIELD_BYTES2`| 0x04ec | 1 | BYTES |
|`PLAYER_FIELD_WATCHED_FACTION_INDEX`| 0x04ed | 1 | INT |
|`PLAYER_FIELD_COMBAT_RATING_1`| 0x04ee | 20 | INT |


Fields that all gameobjects have:

| Name | Offset | Size | Type |
|------|--------|------|------|
|`GAMEOBJECT_CREATED_BY`| 0x0006 | 2 | GUID |
|`GAMEOBJECT_DISPLAYID`| 0x0008 | 1 | INT |
|`GAMEOBJECT_FLAGS`| 0x0009 | 1 | INT |
|`GAMEOBJECT_ROTATION`| 0x000a | 4 | FLOAT |
|`GAMEOBJECT_STATE`| 0x000e | 1 | INT |
|`GAMEOBJECT_POS_X`| 0x000f | 1 | FLOAT |
|`GAMEOBJECT_POS_Y`| 0x0010 | 1 | FLOAT |
|`GAMEOBJECT_POS_Z`| 0x0011 | 1 | FLOAT |
|`GAMEOBJECT_FACING`| 0x0012 | 1 | FLOAT |
|`GAMEOBJECT_DYN_FLAGS`| 0x0013 | 1 | INT |
|`GAMEOBJECT_FACTION`| 0x0014 | 1 | INT |
|`GAMEOBJECT_TYPE_ID`| 0x0015 | 1 | INT |
|`GAMEOBJECT_LEVEL`| 0x0016 | 1 | INT |
|`GAMEOBJECT_ARTKIT`| 0x0017 | 1 | INT |
|`GAMEOBJECT_ANIMPROGRESS`| 0x0018 | 1 | INT |


Fields that all dynamicobjects have:

| Name | Offset | Size | Type |
|------|--------|------|------|
|`DYNAMICOBJECT_CASTER`| 0x0006 | 2 | GUID |
|`DYNAMICOBJECT_BYTES`| 0x0008 | 1 | BYTES |
|`DYNAMICOBJECT_SPELLID`| 0x0009 | 1 | INT |
|`DYNAMICOBJECT_RADIUS`| 0x000a | 1 | FLOAT |
|`DYNAMICOBJECT_POS_X`| 0x000b | 1 | FLOAT |
|`DYNAMICOBJECT_POS_Y`| 0x000c | 1 | FLOAT |
|`DYNAMICOBJECT_POS_Z`| 0x000d | 1 | FLOAT |
|`DYNAMICOBJECT_FACING`| 0x000e | 1 | FLOAT |


Fields that all corpses have:

| Name | Offset | Size | Type |
|------|--------|------|------|
|`CORPSE_OWNER`| 0x0006 | 2 | GUID |
|`CORPSE_FACING`| 0x0008 | 1 | FLOAT |
|`CORPSE_POS_X`| 0x0009 | 1 | FLOAT |
|`CORPSE_POS_Y`| 0x000a | 1 | FLOAT |
|`CORPSE_POS_Z`| 0x000b | 1 | FLOAT |
|`CORPSE_DISPLAY_ID`| 0x000c | 1 | INT |
|`CORPSE_ITEM`| 0x000d | 19 | INT |
|`CORPSE_BYTES_1`| 0x0020 | 1 | BYTES |
|`CORPSE_BYTES_2`| 0x0021 | 1 | BYTES |
|`CORPSE_GUILD`| 0x0022 | 1 | INT |
|`CORPSE_FLAGS`| 0x0023 | 1 | INT |
|`CORPSE_DYNAMIC_FLAGS`| 0x0024 | 1 | INT |


### Version 2.4.3

Taken from [mangosone](https://github.com/mangosone/server/blob/f441fc27a6430e79753bceb545f62fef90a79832/src/game/Object/UpdateFields.h#L30) with some modifications.

Fields that all objects have:

| Name | Offset | Size | Type |
|------|--------|------|------|
|`OBJECT_GUID`| 0x0000 | 2 | GUID |
|`OBJECT_TYPE`| 0x0002 | 1 | INT |
|`OBJECT_ENTRY`| 0x0003 | 1 | INT |
|`OBJECT_SCALE_X`| 0x0004 | 1 | FLOAT |
|`OBJECT_CREATED_BY`| 0x0006 | 2 | GUID |


Fields that all items have:

| Name | Offset | Size | Type |
|------|--------|------|------|
|`ITEM_OWNER`| 0x0006 | 2 | GUID |
|`ITEM_CONTAINED`| 0x0008 | 2 | GUID |
|`ITEM_CREATOR`| 0x000a | 2 | GUID |
|`ITEM_GIFTCREATOR`| 0x000c | 2 | GUID |
|`ITEM_STACK_COUNT`| 0x000e | 1 | INT |
|`ITEM_DURATION`| 0x000f | 1 | INT |
|`ITEM_SPELL_CHARGES`| 0x0010 | 5 | INT |
|`ITEM_FLAGS`| 0x0015 | 1 | INT |
|`ITEM_ENCHANTMENT_1_1`| 0x0016 | 33 | INT |
|`ITEM_PROPERTY_SEED`| 0x0037 | 1 | INT |
|`ITEM_RANDOM_PROPERTIES_ID`| 0x0038 | 1 | INT |
|`ITEM_ITEM_TEXT_ID`| 0x0039 | 1 | INT |
|`ITEM_DURABILITY`| 0x003a | 1 | INT |
|`ITEM_MAXDURABILITY`| 0x003b | 1 | INT |


Fields that all containers have:

| Name | Offset | Size | Type |
|------|--------|------|------|
|`CONTAINER_NUM_SLOTS`| 0x003c | 1 | INT |
|`CONTAINER_SLOT_1`| 0x003e | 72 | GUID |


Fields that all units have:

| Name | Offset | Size | Type |
|------|--------|------|------|
|`UNIT_CHARM`| 0x0006 | 2 | GUID |
|`UNIT_SUMMON`| 0x0008 | 2 | GUID |
|`UNIT_CHARMEDBY`| 0x000a | 2 | GUID |
|`UNIT_SUMMONEDBY`| 0x000c | 2 | GUID |
|`UNIT_CREATEDBY`| 0x000e | 2 | GUID |
|`UNIT_TARGET`| 0x0010 | 2 | GUID |
|`UNIT_PERSUADED`| 0x0012 | 2 | GUID |
|`UNIT_CHANNEL_OBJECT`| 0x0014 | 2 | GUID |
|`UNIT_HEALTH`| 0x0016 | 1 | INT |
|`UNIT_POWER1`| 0x0017 | 1 | INT |
|`UNIT_POWER2`| 0x0018 | 1 | INT |
|`UNIT_POWER3`| 0x0019 | 1 | INT |
|`UNIT_POWER4`| 0x001a | 1 | INT |
|`UNIT_POWER5`| 0x001b | 1 | INT |
|`UNIT_MAXHEALTH`| 0x001c | 1 | INT |
|`UNIT_MAXPOWER1`| 0x001d | 1 | INT |
|`UNIT_MAXPOWER2`| 0x001e | 1 | INT |
|`UNIT_MAXPOWER3`| 0x001f | 1 | INT |
|`UNIT_MAXPOWER4`| 0x0020 | 1 | INT |
|`UNIT_MAXPOWER5`| 0x0021 | 1 | INT |
|`UNIT_LEVEL`| 0x0022 | 1 | INT |
|`UNIT_FACTIONTEMPLATE`| 0x0023 | 1 | INT |
|`UNIT_BYTES_0`| 0x0024 | 1 | BYTES |
|`UNIT_VIRTUAL_ITEM_SLOT_DISPLAY`| 0x0025 | 3 | INT |
|`UNIT_VIRTUAL_ITEM_INFO`| 0x0028 | 6 | BYTES |
|`UNIT_FLAGS`| 0x002e | 1 | INT |
|`UNIT_FLAGS_2`| 0x002f | 1 | INT |
|`UNIT_AURA`| 0x0030 | 56 | INT |
|`UNIT_AURAFLAGS`| 0x0068 | 14 | BYTES |
|`UNIT_AURALEVELS`| 0x0076 | 14 | BYTES |
|`UNIT_AURAAPPLICATIONS`| 0x0084 | 14 | BYTES |
|`UNIT_AURASTATE`| 0x0092 | 1 | INT |
|`UNIT_BASEATTACKTIME`| 0x0093 | 2 | INT |
|`UNIT_RANGEDATTACKTIME`| 0x0095 | 1 | INT |
|`UNIT_BOUNDINGRADIUS`| 0x0096 | 1 | FLOAT |
|`UNIT_COMBATREACH`| 0x0097 | 1 | FLOAT |
|`UNIT_DISPLAYID`| 0x0098 | 1 | INT |
|`UNIT_NATIVEDISPLAYID`| 0x0099 | 1 | INT |
|`UNIT_MOUNTDISPLAYID`| 0x009a | 1 | INT |
|`UNIT_MINDAMAGE`| 0x009b | 1 | FLOAT |
|`UNIT_MAXDAMAGE`| 0x009c | 1 | FLOAT |
|`UNIT_MINOFFHANDDAMAGE`| 0x009d | 1 | FLOAT |
|`UNIT_MAXOFFHANDDAMAGE`| 0x009e | 1 | FLOAT |
|`UNIT_BYTES_1`| 0x009f | 1 | BYTES |
|`UNIT_PETNUMBER`| 0x00a0 | 1 | INT |
|`UNIT_PET_NAME_TIMESTAMP`| 0x00a1 | 1 | INT |
|`UNIT_PETEXPERIENCE`| 0x00a2 | 1 | INT |
|`UNIT_PETNEXTLEVELEXP`| 0x00a3 | 1 | INT |
|`UNIT_DYNAMIC_FLAGS`| 0x00a4 | 1 | INT |
|`UNIT_CHANNEL_SPELL`| 0x00a5 | 1 | INT |
|`UNIT_MOD_CAST_SPEED`| 0x00a6 | 1 | FLOAT |
|`UNIT_CREATED_BY_SPELL`| 0x00a7 | 1 | INT |
|`UNIT_NPC_FLAGS`| 0x00a8 | 1 | INT |
|`UNIT_NPC_EMOTESTATE`| 0x00a9 | 1 | INT |
|`UNIT_TRAINING_POINTS`| 0x00aa | 1 | TWO_SHORT |
|`UNIT_STRENGTH`| 0x00ab | 1 | INT |
|`UNIT_AGILITY`| 0x00ac | 1 | INT |
|`UNIT_STAMINA`| 0x00ad | 1 | INT |
|`UNIT_INTELLECT`| 0x00ae | 1 | INT |
|`UNIT_SPIRIT`| 0x00af | 1 | INT |
|`UNIT_POSSTAT1`| 0x00b1 | 1 | INT |
|`UNIT_POSSTAT2`| 0x00b2 | 1 | INT |
|`UNIT_POSSTAT3`| 0x00b3 | 1 | INT |
|`UNIT_NEGSTAT1`| 0x00b6 | 1 | INT |
|`UNIT_NEGSTAT2`| 0x00b7 | 1 | INT |
|`UNIT_NEGSTAT3`| 0x00b8 | 1 | INT |
|`UNIT_RESISTANCES`| 0x00ba | 7 | INT |
|`UNIT_BASE_MANA`| 0x00cf | 1 | INT |
|`UNIT_BASE_HEALTH`| 0x00d0 | 1 | INT |
|`UNIT_BYTES_2`| 0x00d1 | 1 | BYTES |
|`UNIT_ATTACK_POWER`| 0x00d2 | 1 | INT |
|`UNIT_ATTACK_POWER_MODS`| 0x00d3 | 1 | TWO_SHORT |
|`UNIT_ATTACK_POWER_MULTIPLIER`| 0x00d4 | 1 | FLOAT |
|`UNIT_RANGED_ATTACK_POWER`| 0x00d5 | 1 | INT |
|`UNIT_RANGED_ATTACK_POWER_MODS`| 0x00d6 | 1 | TWO_SHORT |
|`UNIT_RANGED_ATTACK_POWER_MULTIPLIER`| 0x00d7 | 1 | FLOAT |
|`UNIT_MINRANGEDDAMAGE`| 0x00d8 | 1 | FLOAT |
|`UNIT_MAXRANGEDDAMAGE`| 0x00d9 | 1 | FLOAT |
|`UNIT_POWER_COST_MODIFIER`| 0x00da | 7 | INT |
|`UNIT_POWER_COST_MULTIPLIER`| 0x00e1 | 7 | FLOAT |
|`UNIT_MAXHEALTHMODIFIER`| 0x00e8 | 1 | FLOAT |


Fields that all players have:

| Name | Offset | Size | Type |
|------|--------|------|------|
|`PLAYER_POSSTAT0`| 0x00b0 | 1 | INT |
|`PLAYER_POSSTAT4`| 0x00b4 | 1 | INT |
|`PLAYER_NEGSTAT0`| 0x00b5 | 1 | INT |
|`PLAYER_NEGSTAT4`| 0x00b9 | 1 | INT |
|`PLAYER_RESISTANCEBUFFMODSPOSITIVE`| 0x00c1 | 7 | INT |
|`PLAYER_RESISTANCEBUFFMODSNEGATIVE`| 0x00c8 | 7 | INT |
|`PLAYER_DUEL_ARBITER`| 0x00ea | 2 | GUID |
|`PLAYER_FLAGS`| 0x00ec | 1 | INT |
|`PLAYER_GUILDID`| 0x00ed | 1 | INT |
|`PLAYER_GUILDRANK`| 0x00ee | 1 | INT |
|`PLAYER_FIELD_BYTES`| 0x00ef | 1 | BYTES |
|`PLAYER_BYTES_2`| 0x00f0 | 1 | BYTES |
|`PLAYER_BYTES_3`| 0x00f1 | 1 | BYTES |
|`PLAYER_DUEL_TEAM`| 0x00f2 | 1 | INT |
|`PLAYER_GUILD_TIMESTAMP`| 0x00f3 | 1 | INT |
|`PLAYER_QUEST_LOG_1_1`| 0x00f4 | 1 | INT |
|`PLAYER_QUEST_LOG_1_2`| 0x00f5 | 1 | INT |
|`PLAYER_QUEST_LOG_1_3`| 0x00f6 | 1 | BYTES |
|`PLAYER_QUEST_LOG_1_4`| 0x00f7 | 1 | INT |
|`PLAYER_QUEST_LOG_2_1`| 0x00f8 | 1 | INT |
|`PLAYER_QUEST_LOG_2_2`| 0x00f9 | 1 | INT |
|`PLAYER_QUEST_LOG_2_3`| 0x00fa | 1 | BYTES |
|`PLAYER_QUEST_LOG_2_4`| 0x00fb | 1 | INT |
|`PLAYER_QUEST_LOG_3_1`| 0x00fc | 1 | INT |
|`PLAYER_QUEST_LOG_3_2`| 0x00fd | 1 | INT |
|`PLAYER_QUEST_LOG_3_3`| 0x00fe | 1 | BYTES |
|`PLAYER_QUEST_LOG_3_4`| 0x00ff | 1 | INT |
|`PLAYER_QUEST_LOG_4_1`| 0x0100 | 1 | INT |
|`PLAYER_QUEST_LOG_4_2`| 0x0101 | 1 | INT |
|`PLAYER_QUEST_LOG_4_3`| 0x0102 | 1 | BYTES |
|`PLAYER_QUEST_LOG_4_4`| 0x0103 | 1 | INT |
|`PLAYER_QUEST_LOG_5_1`| 0x0104 | 1 | INT |
|`PLAYER_QUEST_LOG_5_2`| 0x0105 | 1 | INT |
|`PLAYER_QUEST_LOG_5_3`| 0x0106 | 1 | BYTES |
|`PLAYER_QUEST_LOG_5_4`| 0x0107 | 1 | INT |
|`PLAYER_QUEST_LOG_6_1`| 0x0108 | 1 | INT |
|`PLAYER_QUEST_LOG_6_2`| 0x0109 | 1 | INT |
|`PLAYER_QUEST_LOG_6_3`| 0x010a | 1 | BYTES |
|`PLAYER_QUEST_LOG_6_4`| 0x010b | 1 | INT |
|`PLAYER_QUEST_LOG_7_1`| 0x010c | 1 | INT |
|`PLAYER_QUEST_LOG_7_2`| 0x010d | 1 | INT |
|`PLAYER_QUEST_LOG_7_3`| 0x010e | 1 | BYTES |
|`PLAYER_QUEST_LOG_7_4`| 0x010f | 1 | INT |
|`PLAYER_QUEST_LOG_8_1`| 0x0110 | 1 | INT |
|`PLAYER_QUEST_LOG_8_2`| 0x0111 | 1 | INT |
|`PLAYER_QUEST_LOG_8_3`| 0x0112 | 1 | BYTES |
|`PLAYER_QUEST_LOG_8_4`| 0x0113 | 1 | INT |
|`PLAYER_QUEST_LOG_9_1`| 0x0114 | 1 | INT |
|`PLAYER_QUEST_LOG_9_2`| 0x0115 | 1 | INT |
|`PLAYER_QUEST_LOG_9_3`| 0x0116 | 1 | BYTES |
|`PLAYER_QUEST_LOG_9_4`| 0x0117 | 1 | INT |
|`PLAYER_QUEST_LOG_10_1`| 0x0118 | 1 | INT |
|`PLAYER_QUEST_LOG_10_2`| 0x0119 | 1 | INT |
|`PLAYER_QUEST_LOG_10_3`| 0x011a | 1 | BYTES |
|`PLAYER_QUEST_LOG_10_4`| 0x011b | 1 | INT |
|`PLAYER_QUEST_LOG_11_1`| 0x011c | 1 | INT |
|`PLAYER_QUEST_LOG_11_2`| 0x011d | 1 | INT |
|`PLAYER_QUEST_LOG_11_3`| 0x011e | 1 | BYTES |
|`PLAYER_QUEST_LOG_11_4`| 0x011f | 1 | INT |
|`PLAYER_QUEST_LOG_12_1`| 0x0120 | 1 | INT |
|`PLAYER_QUEST_LOG_12_2`| 0x0121 | 1 | INT |
|`PLAYER_QUEST_LOG_12_3`| 0x0122 | 1 | BYTES |
|`PLAYER_QUEST_LOG_12_4`| 0x0123 | 1 | INT |
|`PLAYER_QUEST_LOG_13_1`| 0x0124 | 1 | INT |
|`PLAYER_QUEST_LOG_13_2`| 0x0125 | 1 | INT |
|`PLAYER_QUEST_LOG_13_3`| 0x0126 | 1 | BYTES |
|`PLAYER_QUEST_LOG_13_4`| 0x0127 | 1 | INT |
|`PLAYER_QUEST_LOG_14_1`| 0x0128 | 1 | INT |
|`PLAYER_QUEST_LOG_14_2`| 0x0129 | 1 | INT |
|`PLAYER_QUEST_LOG_14_3`| 0x012a | 1 | BYTES |
|`PLAYER_QUEST_LOG_14_4`| 0x012b | 1 | INT |
|`PLAYER_QUEST_LOG_15_1`| 0x012c | 1 | INT |
|`PLAYER_QUEST_LOG_15_2`| 0x012d | 1 | INT |
|`PLAYER_QUEST_LOG_15_3`| 0x012e | 1 | BYTES |
|`PLAYER_QUEST_LOG_15_4`| 0x012f | 1 | INT |
|`PLAYER_QUEST_LOG_16_1`| 0x0130 | 1 | INT |
|`PLAYER_QUEST_LOG_16_2`| 0x0131 | 1 | INT |
|`PLAYER_QUEST_LOG_16_3`| 0x0132 | 1 | BYTES |
|`PLAYER_QUEST_LOG_16_4`| 0x0133 | 1 | INT |
|`PLAYER_QUEST_LOG_17_1`| 0x0134 | 1 | INT |
|`PLAYER_QUEST_LOG_17_2`| 0x0135 | 1 | INT |
|`PLAYER_QUEST_LOG_17_3`| 0x0136 | 1 | BYTES |
|`PLAYER_QUEST_LOG_17_4`| 0x0137 | 1 | INT |
|`PLAYER_QUEST_LOG_18_1`| 0x0138 | 1 | INT |
|`PLAYER_QUEST_LOG_18_2`| 0x0139 | 1 | INT |
|`PLAYER_QUEST_LOG_18_3`| 0x013a | 1 | BYTES |
|`PLAYER_QUEST_LOG_18_4`| 0x013b | 1 | INT |
|`PLAYER_QUEST_LOG_19_1`| 0x013c | 1 | INT |
|`PLAYER_QUEST_LOG_19_2`| 0x013d | 1 | INT |
|`PLAYER_QUEST_LOG_19_3`| 0x013e | 1 | BYTES |
|`PLAYER_QUEST_LOG_19_4`| 0x013f | 1 | INT |
|`PLAYER_QUEST_LOG_20_1`| 0x0140 | 1 | INT |
|`PLAYER_QUEST_LOG_20_2`| 0x0141 | 1 | INT |
|`PLAYER_QUEST_LOG_20_3`| 0x0142 | 1 | BYTES |
|`PLAYER_QUEST_LOG_20_4`| 0x0143 | 1 | INT |
|`PLAYER_QUEST_LOG_21_1`| 0x0144 | 1 | INT |
|`PLAYER_QUEST_LOG_21_2`| 0x0145 | 1 | INT |
|`PLAYER_QUEST_LOG_21_3`| 0x0146 | 1 | BYTES |
|`PLAYER_QUEST_LOG_21_4`| 0x0147 | 1 | INT |
|`PLAYER_QUEST_LOG_22_1`| 0x0148 | 1 | INT |
|`PLAYER_QUEST_LOG_22_2`| 0x0149 | 1 | INT |
|`PLAYER_QUEST_LOG_22_3`| 0x014a | 1 | BYTES |
|`PLAYER_QUEST_LOG_22_4`| 0x014b | 1 | INT |
|`PLAYER_QUEST_LOG_23_1`| 0x014c | 1 | INT |
|`PLAYER_QUEST_LOG_23_2`| 0x014d | 1 | INT |
|`PLAYER_QUEST_LOG_23_3`| 0x014e | 1 | BYTES |
|`PLAYER_QUEST_LOG_23_4`| 0x014f | 1 | INT |
|`PLAYER_QUEST_LOG_24_1`| 0x0150 | 1 | INT |
|`PLAYER_QUEST_LOG_24_2`| 0x0151 | 1 | INT |
|`PLAYER_QUEST_LOG_24_3`| 0x0152 | 1 | BYTES |
|`PLAYER_QUEST_LOG_24_4`| 0x0153 | 1 | INT |
|`PLAYER_QUEST_LOG_25_1`| 0x0154 | 1 | INT |
|`PLAYER_QUEST_LOG_25_2`| 0x0155 | 1 | INT |
|`PLAYER_QUEST_LOG_25_3`| 0x0156 | 1 | BYTES |
|`PLAYER_QUEST_LOG_25_4`| 0x0157 | 1 | INT |
|`PLAYER_VISIBLE_ITEM`| 0x0158 | 228 | CUSTOM |
|`PLAYER_CHOSEN_TITLE`| 0x0288 | 1 | INT |
|`PLAYER_FIELD_INV`| 0x01e6 | 272 | CUSTOM |
|`PLAYER_FARSIGHT`| 0x039a | 2 | GUID |
|`PLAYER_KNOWN_TITLES`| 0x039c | 2 | GUID |
|`PLAYER_XP`| 0x039e | 1 | INT |
|`PLAYER_NEXT_LEVEL_XP`| 0x039f | 1 | INT |
|`PLAYER_SKILL_INFO`| 0x03a0 | 384 | CUSTOM |
|`PLAYER_CHARACTER_POINTS1`| 0x0520 | 1 | INT |
|`PLAYER_CHARACTER_POINTS2`| 0x0521 | 1 | INT |
|`PLAYER_TRACK_CREATURES`| 0x0522 | 1 | INT |
|`PLAYER_TRACK_RESOURCES`| 0x0523 | 1 | INT |
|`PLAYER_BLOCK_PERCENTAGE`| 0x0524 | 1 | FLOAT |
|`PLAYER_DODGE_PERCENTAGE`| 0x0525 | 1 | FLOAT |
|`PLAYER_PARRY_PERCENTAGE`| 0x0526 | 1 | FLOAT |
|`PLAYER_EXPERTISE`| 0x0527 | 1 | INT |
|`PLAYER_OFFHAND_EXPERTISE`| 0x0528 | 1 | INT |
|`PLAYER_CRIT_PERCENTAGE`| 0x0529 | 1 | FLOAT |
|`PLAYER_RANGED_CRIT_PERCENTAGE`| 0x052a | 1 | FLOAT |
|`PLAYER_OFFHAND_CRIT_PERCENTAGE`| 0x052b | 1 | FLOAT |
|`PLAYER_SPELL_CRIT_PERCENTAGE1`| 0x052c | 7 | FLOAT |
|`PLAYER_SHIELD_BLOCK`| 0x0533 | 1 | INT |
|`PLAYER_EXPLORED_ZONES_1`| 0x0534 | 128 | BYTES |
|`PLAYER_REST_STATE_EXPERIENCE`| 0x05b4 | 1 | INT |
|`PLAYER_COINAGE`| 0x05b5 | 1 | INT |
|`PLAYER_MOD_DAMAGE_DONE_POS`| 0x05b6 | 7 | INT |
|`PLAYER_MOD_DAMAGE_DONE_NEG`| 0x05bd | 7 | INT |
|`PLAYER_MOD_DAMAGE_DONE_PCT`| 0x05c4 | 7 | INT |
|`PLAYER_MOD_HEALING_DONE_POS`| 0x05cb | 1 | INT |
|`PLAYER_MOD_TARGET_RESISTANCE`| 0x05cc | 1 | INT |
|`PLAYER_MOD_TARGET_PHYSICAL_RESISTANCE`| 0x05cd | 1 | INT |
|`PLAYER_FEATURES`| 0x05ce | 1 | BYTES |
|`PLAYER_AMMO_ID`| 0x05cf | 1 | INT |
|`PLAYER_SELF_RES_SPELL`| 0x05d0 | 1 | INT |
|`PLAYER_PVP_MEDALS`| 0x05d1 | 1 | INT |
|`PLAYER_BUYBACK_PRICE_1`| 0x05d2 | 12 | INT |
|`PLAYER_BUYBACK_TIMESTAMP_1`| 0x05de | 12 | INT |
|`PLAYER_KILLS`| 0x05ea | 1 | TWO_SHORT |
|`PLAYER_TODAY_CONTRIBUTION`| 0x05eb | 1 | INT |
|`PLAYER_YESTERDAY_CONTRIBUTION`| 0x05ec | 1 | INT |
|`PLAYER_LIFETIME_HONORABLE_KILLS`| 0x05ed | 1 | INT |
|`PLAYER_BYTES2_GLOW`| 0x05ee | 1 | BYTES |
|`PLAYER_WATCHED_FACTION_INDEX`| 0x05ef | 1 | INT |
|`PLAYER_COMBAT_RATING_1`| 0x05f0 | 24 | INT |
|`PLAYER_ARENA_TEAM_INFO_1_1`| 0x0608 | 18 | INT |
|`PLAYER_HONOR_CURRENCY`| 0x061a | 1 | INT |
|`PLAYER_ARENA_CURRENCY`| 0x061b | 1 | INT |
|`PLAYER_MOD_MANA_REGEN`| 0x061c | 1 | FLOAT |
|`PLAYER_MOD_MANA_REGEN_INTERRUPT`| 0x061d | 1 | FLOAT |
|`PLAYER_MAX_LEVEL`| 0x061e | 1 | INT |
|`PLAYER_DAILY_QUESTS_1`| 0x061f | 25 | INT |


Fields that all gameobjects have:

| Name | Offset | Size | Type |
|------|--------|------|------|
|`GAMEOBJECT_DISPLAYID`| 0x0008 | 1 | INT |
|`GAMEOBJECT_FLAGS`| 0x0009 | 1 | INT |
|`GAMEOBJECT_ROTATION`| 0x000a | 4 | FLOAT |
|`GAMEOBJECT_STATE`| 0x000e | 1 | INT |
|`GAMEOBJECT_POS_X`| 0x000f | 1 | FLOAT |
|`GAMEOBJECT_POS_Y`| 0x0010 | 1 | FLOAT |
|`GAMEOBJECT_POS_Z`| 0x0011 | 1 | FLOAT |
|`GAMEOBJECT_FACING`| 0x0012 | 1 | FLOAT |
|`GAMEOBJECT_DYN_FLAGS`| 0x0013 | 1 | INT |
|`GAMEOBJECT_FACTION`| 0x0014 | 1 | INT |
|`GAMEOBJECT_TYPE_ID`| 0x0015 | 1 | INT |
|`GAMEOBJECT_LEVEL`| 0x0016 | 1 | INT |
|`GAMEOBJECT_ARTKIT`| 0x0017 | 1 | INT |
|`GAMEOBJECT_ANIMPROGRESS`| 0x0018 | 1 | INT |


Fields that all dynamicobjects have:

| Name | Offset | Size | Type |
|------|--------|------|------|
|`DYNAMICOBJECT_CASTER`| 0x0006 | 2 | GUID |
|`DYNAMICOBJECT_BYTES`| 0x0008 | 1 | BYTES |
|`DYNAMICOBJECT_SPELLID`| 0x0009 | 1 | INT |
|`DYNAMICOBJECT_RADIUS`| 0x000a | 1 | FLOAT |
|`DYNAMICOBJECT_POS_X`| 0x000b | 1 | FLOAT |
|`DYNAMICOBJECT_POS_Y`| 0x000c | 1 | FLOAT |
|`DYNAMICOBJECT_POS_Z`| 0x000d | 1 | FLOAT |
|`DYNAMICOBJECT_FACING`| 0x000e | 1 | FLOAT |
|`DYNAMICOBJECT_CASTTIME`| 0x000f | 1 | INT |


Fields that all corpses have:

| Name | Offset | Size | Type |
|------|--------|------|------|
|`CORPSE_OWNER`| 0x0006 | 2 | GUID |
|`CORPSE_PARTY`| 0x0008 | 2 | GUID |
|`CORPSE_FACING`| 0x000a | 1 | FLOAT |
|`CORPSE_POS_X`| 0x000b | 1 | FLOAT |
|`CORPSE_POS_Y`| 0x000c | 1 | FLOAT |
|`CORPSE_POS_Z`| 0x000d | 1 | FLOAT |
|`CORPSE_DISPLAY_ID`| 0x000e | 1 | INT |
|`CORPSE_ITEM`| 0x000f | 19 | INT |
|`CORPSE_BYTES_1`| 0x0022 | 1 | BYTES |
|`CORPSE_BYTES_2`| 0x0023 | 1 | BYTES |
|`CORPSE_GUILD`| 0x0024 | 1 | INT |
|`CORPSE_FLAGS`| 0x0025 | 1 | INT |
|`CORPSE_DYNAMIC_FLAGS`| 0x0026 | 1 | INT |


### Version 3.3.5

Taken from [ArcEmu](https://github.com/arcemu/arcemu/blob/1cb2b5248d050cb6fe413d7c42dd1817994b6366/src/world/Game/Entities/Update/UpdateFields.h#L26) with some modifications.

Fields that all objects have:

| Name | Offset | Size | Type |
|------|--------|------|------|
|`OBJECT_GUID`| 0x0000 | 2 | GUID |
|`OBJECT_TYPE`| 0x0002 | 1 | INT |
|`OBJECT_ENTRY`| 0x0003 | 1 | INT |
|`OBJECT_SCALE_X`| 0x0004 | 1 | FLOAT |
|`OBJECT_CREATED_BY`| 0x0006 | 2 | GUID |


Fields that all items have:

| Name | Offset | Size | Type |
|------|--------|------|------|
|`ITEM_OWNER`| 0x0006 | 2 | GUID |
|`ITEM_CONTAINED`| 0x0008 | 2 | GUID |
|`ITEM_CREATOR`| 0x000a | 2 | GUID |
|`ITEM_GIFTCREATOR`| 0x000c | 2 | GUID |
|`ITEM_STACK_COUNT`| 0x000e | 1 | INT |
|`ITEM_DURATION`| 0x000f | 1 | INT |
|`ITEM_SPELL_CHARGES`| 0x0010 | 5 | INT |
|`ITEM_FLAGS`| 0x0015 | 1 | INT |
|`ITEM_ENCHANTMENT_1_1`| 0x0016 | 2 | INT |
|`ITEM_ENCHANTMENT_1_3`| 0x0018 | 1 | TWO_SHORT |
|`ITEM_ENCHANTMENT_2_1`| 0x0019 | 2 | INT |
|`ITEM_ENCHANTMENT_2_3`| 0x001b | 1 | TWO_SHORT |
|`ITEM_ENCHANTMENT_3_1`| 0x001c | 2 | INT |
|`ITEM_ENCHANTMENT_3_3`| 0x001e | 1 | TWO_SHORT |
|`ITEM_ENCHANTMENT_4_1`| 0x001f | 2 | INT |
|`ITEM_ENCHANTMENT_4_3`| 0x0021 | 1 | TWO_SHORT |
|`ITEM_ENCHANTMENT_5_1`| 0x0022 | 2 | INT |
|`ITEM_ENCHANTMENT_5_3`| 0x0024 | 1 | TWO_SHORT |
|`ITEM_ENCHANTMENT_6_1`| 0x0025 | 2 | INT |
|`ITEM_ENCHANTMENT_6_3`| 0x0027 | 1 | TWO_SHORT |
|`ITEM_ENCHANTMENT_7_1`| 0x0028 | 2 | INT |
|`ITEM_ENCHANTMENT_7_3`| 0x002a | 1 | TWO_SHORT |
|`ITEM_ENCHANTMENT_8_1`| 0x002b | 2 | INT |
|`ITEM_ENCHANTMENT_8_3`| 0x002d | 1 | TWO_SHORT |
|`ITEM_ENCHANTMENT_9_1`| 0x002e | 2 | INT |
|`ITEM_ENCHANTMENT_9_3`| 0x0030 | 1 | TWO_SHORT |
|`ITEM_ENCHANTMENT_10_1`| 0x0031 | 2 | INT |
|`ITEM_ENCHANTMENT_10_3`| 0x0033 | 1 | TWO_SHORT |
|`ITEM_ENCHANTMENT_11_1`| 0x0034 | 2 | INT |
|`ITEM_ENCHANTMENT_11_3`| 0x0036 | 1 | TWO_SHORT |
|`ITEM_ENCHANTMENT_12_1`| 0x0037 | 2 | INT |
|`ITEM_ENCHANTMENT_12_3`| 0x0039 | 1 | TWO_SHORT |
|`ITEM_PROPERTY_SEED`| 0x003a | 1 | INT |
|`ITEM_RANDOM_PROPERTIES_ID`| 0x003b | 1 | INT |
|`ITEM_DURABILITY`| 0x003c | 1 | INT |
|`ITEM_MAXDURABILITY`| 0x003d | 1 | INT |
|`ITEM_CREATE_PLAYED_TIME`| 0x003e | 1 | INT |


Fields that all containers have:

| Name | Offset | Size | Type |
|------|--------|------|------|
|`CONTAINER_NUM_SLOTS`| 0x0040 | 1 | INT |
|`CONTAINER_SLOT_1`| 0x0042 | 72 | GUID |


Fields that all units have:

| Name | Offset | Size | Type |
|------|--------|------|------|
|`UNIT_CHARM`| 0x0006 | 2 | GUID |
|`UNIT_SUMMON`| 0x0008 | 2 | GUID |
|`UNIT_CRITTER`| 0x000a | 2 | GUID |
|`UNIT_CHARMEDBY`| 0x000c | 2 | GUID |
|`UNIT_SUMMONEDBY`| 0x000e | 2 | GUID |
|`UNIT_CREATEDBY`| 0x0010 | 2 | GUID |
|`UNIT_TARGET`| 0x0012 | 2 | GUID |
|`UNIT_CHANNEL_OBJECT`| 0x0014 | 2 | GUID |
|`UNIT_CHANNEL_SPELL`| 0x0016 | 1 | INT |
|`UNIT_BYTES_0`| 0x0017 | 1 | BYTES |
|`UNIT_HEALTH`| 0x0018 | 1 | INT |
|`UNIT_POWER1`| 0x0019 | 1 | INT |
|`UNIT_POWER2`| 0x001a | 1 | INT |
|`UNIT_POWER3`| 0x001b | 1 | INT |
|`UNIT_POWER4`| 0x001c | 1 | INT |
|`UNIT_POWER5`| 0x001d | 1 | INT |
|`UNIT_POWER6`| 0x001e | 1 | INT |
|`UNIT_POWER7`| 0x001f | 1 | INT |
|`UNIT_MAXHEALTH`| 0x0020 | 1 | INT |
|`UNIT_MAXPOWER1`| 0x0021 | 1 | INT |
|`UNIT_MAXPOWER2`| 0x0022 | 1 | INT |
|`UNIT_MAXPOWER3`| 0x0023 | 1 | INT |
|`UNIT_MAXPOWER4`| 0x0024 | 1 | INT |
|`UNIT_MAXPOWER5`| 0x0025 | 1 | INT |
|`UNIT_MAXPOWER6`| 0x0026 | 1 | INT |
|`UNIT_MAXPOWER7`| 0x0027 | 1 | INT |
|`UNIT_POWER_REGEN_FLAT_MODIFIER`| 0x0028 | 7 | FLOAT |
|`UNIT_POWER_REGEN_INTERRUPTED_FLAT_MODIFIER`| 0x002f | 7 | FLOAT |
|`UNIT_LEVEL`| 0x0036 | 1 | INT |
|`UNIT_FACTIONTEMPLATE`| 0x0037 | 1 | INT |
|`UNIT_VIRTUAL_ITEM_SLOT_ID`| 0x0038 | 3 | INT |
|`UNIT_FLAGS`| 0x003b | 1 | INT |
|`UNIT_FLAGS_2`| 0x003c | 1 | INT |
|`UNIT_AURASTATE`| 0x003d | 1 | INT |
|`UNIT_BASEATTACKTIME`| 0x003e | 2 | INT |
|`UNIT_RANGEDATTACKTIME`| 0x0040 | 1 | INT |
|`UNIT_BOUNDINGRADIUS`| 0x0041 | 1 | FLOAT |
|`UNIT_COMBATREACH`| 0x0042 | 1 | FLOAT |
|`UNIT_DISPLAYID`| 0x0043 | 1 | INT |
|`UNIT_NATIVEDISPLAYID`| 0x0044 | 1 | INT |
|`UNIT_MOUNTDISPLAYID`| 0x0045 | 1 | INT |
|`UNIT_MINDAMAGE`| 0x0046 | 1 | FLOAT |
|`UNIT_MAXDAMAGE`| 0x0047 | 1 | FLOAT |
|`UNIT_MINOFFHANDDAMAGE`| 0x0048 | 1 | FLOAT |
|`UNIT_MAXOFFHANDDAMAGE`| 0x0049 | 1 | FLOAT |
|`UNIT_BYTES_1`| 0x004a | 1 | BYTES |
|`UNIT_PETNUMBER`| 0x004b | 1 | INT |
|`UNIT_PET_NAME_TIMESTAMP`| 0x004c | 1 | INT |
|`UNIT_PETEXPERIENCE`| 0x004d | 1 | INT |
|`UNIT_PETNEXTLEVELEXP`| 0x004e | 1 | INT |
|`UNIT_DYNAMIC_FLAGS`| 0x004f | 1 | INT |
|`UNIT_MOD_CAST_SPEED`| 0x0050 | 1 | FLOAT |
|`UNIT_CREATED_BY_SPELL`| 0x0051 | 1 | INT |
|`UNIT_NPC_FLAGS`| 0x0052 | 1 | INT |
|`UNIT_NPC_EMOTESTATE`| 0x0053 | 1 | INT |
|`UNIT_STRENGTH`| 0x0054 | 1 | INT |
|`UNIT_AGILITY`| 0x0055 | 1 | INT |
|`UNIT_STAMINA`| 0x0056 | 1 | INT |
|`UNIT_INTELLECT`| 0x0057 | 1 | INT |
|`UNIT_SPIRIT`| 0x0058 | 1 | INT |
|`UNIT_POSSTAT0`| 0x0059 | 1 | INT |
|`UNIT_POSSTAT1`| 0x005a | 1 | INT |
|`UNIT_POSSTAT2`| 0x005b | 1 | INT |
|`UNIT_POSSTAT3`| 0x005c | 1 | INT |
|`UNIT_POSSTAT4`| 0x005d | 1 | INT |
|`UNIT_NEGSTAT0`| 0x005e | 1 | INT |
|`UNIT_NEGSTAT1`| 0x005f | 1 | INT |
|`UNIT_NEGSTAT2`| 0x0060 | 1 | INT |
|`UNIT_NEGSTAT3`| 0x0061 | 1 | INT |
|`UNIT_NEGSTAT4`| 0x0062 | 1 | INT |
|`UNIT_RESISTANCES`| 0x0063 | 7 | INT |
|`UNIT_RESISTANCEBUFFMODSPOSITIVE`| 0x006a | 7 | INT |
|`UNIT_RESISTANCEBUFFMODSNEGATIVE`| 0x0071 | 7 | INT |
|`UNIT_BASE_MANA`| 0x0078 | 1 | INT |
|`UNIT_BASE_HEALTH`| 0x0079 | 1 | INT |
|`UNIT_BYTES_2`| 0x007a | 1 | BYTES |
|`UNIT_ATTACK_POWER`| 0x007b | 1 | INT |
|`UNIT_ATTACK_POWER_MODS`| 0x007c | 1 | TWO_SHORT |
|`UNIT_ATTACK_POWER_MULTIPLIER`| 0x007d | 1 | FLOAT |
|`UNIT_RANGED_ATTACK_POWER`| 0x007e | 1 | INT |
|`UNIT_RANGED_ATTACK_POWER_MODS`| 0x007f | 1 | TWO_SHORT |
|`UNIT_RANGED_ATTACK_POWER_MULTIPLIER`| 0x0080 | 1 | FLOAT |
|`UNIT_MINRANGEDDAMAGE`| 0x0081 | 1 | FLOAT |
|`UNIT_MAXRANGEDDAMAGE`| 0x0082 | 1 | FLOAT |
|`UNIT_POWER_COST_MODIFIER`| 0x0083 | 7 | INT |
|`UNIT_POWER_COST_MULTIPLIER`| 0x008a | 7 | FLOAT |
|`UNIT_MAXHEALTHMODIFIER`| 0x0091 | 1 | FLOAT |
|`UNIT_HOVERHEIGHT`| 0x0092 | 1 | FLOAT |


Fields that all players have:

| Name | Offset | Size | Type |
|------|--------|------|------|
|`PLAYER_DUEL_ARBITER`| 0x0094 | 2 | GUID |
|`PLAYER_FLAGS`| 0x0096 | 1 | INT |
|`PLAYER_GUILDID`| 0x0097 | 1 | INT |
|`PLAYER_GUILDRANK`| 0x0098 | 1 | INT |
|`PLAYER_FIELD_BYTES`| 0x0099 | 1 | BYTES |
|`PLAYER_BYTES_2`| 0x009a | 1 | BYTES |
|`PLAYER_BYTES_3`| 0x009b | 1 | BYTES |
|`PLAYER_DUEL_TEAM`| 0x009c | 1 | INT |
|`PLAYER_GUILD_TIMESTAMP`| 0x009d | 1 | INT |
|`PLAYER_QUEST_LOG_1_1`| 0x009e | 1 | INT |
|`PLAYER_QUEST_LOG_1_2`| 0x009f | 1 | INT |
|`PLAYER_QUEST_LOG_1_3`| 0x00a0 | 2 | TWO_SHORT |
|`PLAYER_QUEST_LOG_1_4`| 0x00a2 | 1 | INT |
|`PLAYER_QUEST_LOG_2_1`| 0x00a3 | 1 | INT |
|`PLAYER_QUEST_LOG_2_2`| 0x00a4 | 1 | INT |
|`PLAYER_QUEST_LOG_2_3`| 0x00a5 | 2 | TWO_SHORT |
|`PLAYER_QUEST_LOG_2_5`| 0x00a7 | 1 | INT |
|`PLAYER_QUEST_LOG_3_1`| 0x00a8 | 1 | INT |
|`PLAYER_QUEST_LOG_3_2`| 0x00a9 | 1 | INT |
|`PLAYER_QUEST_LOG_3_3`| 0x00aa | 2 | TWO_SHORT |
|`PLAYER_QUEST_LOG_3_5`| 0x00ac | 1 | INT |
|`PLAYER_QUEST_LOG_4_1`| 0x00ad | 1 | INT |
|`PLAYER_QUEST_LOG_4_2`| 0x00ae | 1 | INT |
|`PLAYER_QUEST_LOG_4_3`| 0x00af | 2 | TWO_SHORT |
|`PLAYER_QUEST_LOG_4_5`| 0x00b1 | 1 | INT |
|`PLAYER_QUEST_LOG_5_1`| 0x00b2 | 1 | INT |
|`PLAYER_QUEST_LOG_5_2`| 0x00b3 | 1 | INT |
|`PLAYER_QUEST_LOG_5_3`| 0x00b4 | 2 | TWO_SHORT |
|`PLAYER_QUEST_LOG_5_5`| 0x00b6 | 1 | INT |
|`PLAYER_QUEST_LOG_6_1`| 0x00b7 | 1 | INT |
|`PLAYER_QUEST_LOG_6_2`| 0x00b8 | 1 | INT |
|`PLAYER_QUEST_LOG_6_3`| 0x00b9 | 2 | TWO_SHORT |
|`PLAYER_QUEST_LOG_6_5`| 0x00bb | 1 | INT |
|`PLAYER_QUEST_LOG_7_1`| 0x00bc | 1 | INT |
|`PLAYER_QUEST_LOG_7_2`| 0x00bd | 1 | INT |
|`PLAYER_QUEST_LOG_7_3`| 0x00be | 2 | TWO_SHORT |
|`PLAYER_QUEST_LOG_7_5`| 0x00c0 | 1 | INT |
|`PLAYER_QUEST_LOG_8_1`| 0x00c1 | 1 | INT |
|`PLAYER_QUEST_LOG_8_2`| 0x00c2 | 1 | INT |
|`PLAYER_QUEST_LOG_8_3`| 0x00c3 | 2 | TWO_SHORT |
|`PLAYER_QUEST_LOG_8_5`| 0x00c5 | 1 | INT |
|`PLAYER_QUEST_LOG_9_1`| 0x00c6 | 1 | INT |
|`PLAYER_QUEST_LOG_9_2`| 0x00c7 | 1 | INT |
|`PLAYER_QUEST_LOG_9_3`| 0x00c8 | 2 | TWO_SHORT |
|`PLAYER_QUEST_LOG_9_5`| 0x00ca | 1 | INT |
|`PLAYER_QUEST_LOG_10_1`| 0x00cb | 1 | INT |
|`PLAYER_QUEST_LOG_10_2`| 0x00cc | 1 | INT |
|`PLAYER_QUEST_LOG_10_3`| 0x00cd | 2 | TWO_SHORT |
|`PLAYER_QUEST_LOG_10_5`| 0x00cf | 1 | INT |
|`PLAYER_QUEST_LOG_11_1`| 0x00d0 | 1 | INT |
|`PLAYER_QUEST_LOG_11_2`| 0x00d1 | 1 | INT |
|`PLAYER_QUEST_LOG_11_3`| 0x00d2 | 2 | TWO_SHORT |
|`PLAYER_QUEST_LOG_11_5`| 0x00d4 | 1 | INT |
|`PLAYER_QUEST_LOG_12_1`| 0x00d5 | 1 | INT |
|`PLAYER_QUEST_LOG_12_2`| 0x00d6 | 1 | INT |
|`PLAYER_QUEST_LOG_12_3`| 0x00d7 | 2 | TWO_SHORT |
|`PLAYER_QUEST_LOG_12_5`| 0x00d9 | 1 | INT |
|`PLAYER_QUEST_LOG_13_1`| 0x00da | 1 | INT |
|`PLAYER_QUEST_LOG_13_2`| 0x00db | 1 | INT |
|`PLAYER_QUEST_LOG_13_3`| 0x00dc | 2 | TWO_SHORT |
|`PLAYER_QUEST_LOG_13_5`| 0x00de | 1 | INT |
|`PLAYER_QUEST_LOG_14_1`| 0x00df | 1 | INT |
|`PLAYER_QUEST_LOG_14_2`| 0x00e0 | 1 | INT |
|`PLAYER_QUEST_LOG_14_3`| 0x00e1 | 2 | TWO_SHORT |
|`PLAYER_QUEST_LOG_14_5`| 0x00e3 | 1 | INT |
|`PLAYER_QUEST_LOG_15_1`| 0x00e4 | 1 | INT |
|`PLAYER_QUEST_LOG_15_2`| 0x00e5 | 1 | INT |
|`PLAYER_QUEST_LOG_15_3`| 0x00e6 | 2 | TWO_SHORT |
|`PLAYER_QUEST_LOG_15_5`| 0x00e8 | 1 | INT |
|`PLAYER_QUEST_LOG_16_1`| 0x00e9 | 1 | INT |
|`PLAYER_QUEST_LOG_16_2`| 0x00ea | 1 | INT |
|`PLAYER_QUEST_LOG_16_3`| 0x00eb | 2 | TWO_SHORT |
|`PLAYER_QUEST_LOG_16_5`| 0x00ed | 1 | INT |
|`PLAYER_QUEST_LOG_17_1`| 0x00ee | 1 | INT |
|`PLAYER_QUEST_LOG_17_2`| 0x00ef | 1 | INT |
|`PLAYER_QUEST_LOG_17_3`| 0x00f0 | 2 | TWO_SHORT |
|`PLAYER_QUEST_LOG_17_5`| 0x00f2 | 1 | INT |
|`PLAYER_QUEST_LOG_18_1`| 0x00f3 | 1 | INT |
|`PLAYER_QUEST_LOG_18_2`| 0x00f4 | 1 | INT |
|`PLAYER_QUEST_LOG_18_3`| 0x00f5 | 2 | TWO_SHORT |
|`PLAYER_QUEST_LOG_18_5`| 0x00f7 | 1 | INT |
|`PLAYER_QUEST_LOG_19_1`| 0x00f8 | 1 | INT |
|`PLAYER_QUEST_LOG_19_2`| 0x00f9 | 1 | INT |
|`PLAYER_QUEST_LOG_19_3`| 0x00fa | 2 | TWO_SHORT |
|`PLAYER_QUEST_LOG_19_5`| 0x00fc | 1 | INT |
|`PLAYER_QUEST_LOG_20_1`| 0x00fd | 1 | INT |
|`PLAYER_QUEST_LOG_20_2`| 0x00fe | 1 | INT |
|`PLAYER_QUEST_LOG_20_3`| 0x00ff | 2 | TWO_SHORT |
|`PLAYER_QUEST_LOG_20_5`| 0x0101 | 1 | INT |
|`PLAYER_QUEST_LOG_21_1`| 0x0102 | 1 | INT |
|`PLAYER_QUEST_LOG_21_2`| 0x0103 | 1 | INT |
|`PLAYER_QUEST_LOG_21_3`| 0x0104 | 2 | TWO_SHORT |
|`PLAYER_QUEST_LOG_21_5`| 0x0106 | 1 | INT |
|`PLAYER_QUEST_LOG_22_1`| 0x0107 | 1 | INT |
|`PLAYER_QUEST_LOG_22_2`| 0x0108 | 1 | INT |
|`PLAYER_QUEST_LOG_22_3`| 0x0109 | 2 | TWO_SHORT |
|`PLAYER_QUEST_LOG_22_5`| 0x010b | 1 | INT |
|`PLAYER_QUEST_LOG_23_1`| 0x010c | 1 | INT |
|`PLAYER_QUEST_LOG_23_2`| 0x010d | 1 | INT |
|`PLAYER_QUEST_LOG_23_3`| 0x010e | 2 | TWO_SHORT |
|`PLAYER_QUEST_LOG_23_5`| 0x0110 | 1 | INT |
|`PLAYER_QUEST_LOG_24_1`| 0x0111 | 1 | INT |
|`PLAYER_QUEST_LOG_24_2`| 0x0112 | 1 | INT |
|`PLAYER_QUEST_LOG_24_3`| 0x0113 | 2 | TWO_SHORT |
|`PLAYER_QUEST_LOG_24_5`| 0x0115 | 1 | INT |
|`PLAYER_QUEST_LOG_25_1`| 0x0116 | 1 | INT |
|`PLAYER_QUEST_LOG_25_2`| 0x0117 | 1 | INT |
|`PLAYER_QUEST_LOG_25_3`| 0x0118 | 2 | TWO_SHORT |
|`PLAYER_QUEST_LOG_25_5`| 0x011a | 1 | INT |
|`PLAYER_VISIBLE_ITEM`| 0x011b | 38 | CUSTOM |
|`PLAYER_CHOSEN_TITLE`| 0x0141 | 1 | INT |
|`PLAYER_FAKE_INEBRIATION`| 0x0142 | 1 | INT |
|`PLAYER_FIELD_INV`| 0x0144 | 300 | CUSTOM |
|`PLAYER_FARSIGHT`| 0x0270 | 2 | GUID |
|`PLAYER_KNOWN_TITLES`| 0x0272 | 2 | GUID |
|`PLAYER_KNOWN_TITLES1`| 0x0274 | 2 | GUID |
|`PLAYER_KNOWN_TITLES2`| 0x0276 | 2 | GUID |
|`PLAYER_KNOWN_CURRENCIES`| 0x0278 | 2 | GUID |
|`PLAYER_XP`| 0x027a | 1 | INT |
|`PLAYER_NEXT_LEVEL_XP`| 0x027b | 1 | INT |
|`PLAYER_SKILL_INFO`| 0x027c | 384 | CUSTOM |
|`PLAYER_CHARACTER_POINTS1`| 0x03fc | 1 | INT |
|`PLAYER_CHARACTER_POINTS2`| 0x03fd | 1 | INT |
|`PLAYER_TRACK_CREATURES`| 0x03fe | 1 | INT |
|`PLAYER_TRACK_RESOURCES`| 0x03ff | 1 | INT |
|`PLAYER_BLOCK_PERCENTAGE`| 0x0400 | 1 | FLOAT |
|`PLAYER_DODGE_PERCENTAGE`| 0x0401 | 1 | FLOAT |
|`PLAYER_PARRY_PERCENTAGE`| 0x0402 | 1 | FLOAT |
|`PLAYER_EXPERTISE`| 0x0403 | 1 | INT |
|`PLAYER_OFFHAND_EXPERTISE`| 0x0404 | 1 | INT |
|`PLAYER_CRIT_PERCENTAGE`| 0x0405 | 1 | FLOAT |
|`PLAYER_RANGED_CRIT_PERCENTAGE`| 0x0406 | 1 | FLOAT |
|`PLAYER_OFFHAND_CRIT_PERCENTAGE`| 0x0407 | 1 | FLOAT |
|`PLAYER_SPELL_CRIT_PERCENTAGE1`| 0x0408 | 7 | FLOAT |
|`PLAYER_SHIELD_BLOCK`| 0x040f | 1 | INT |
|`PLAYER_SHIELD_BLOCK_CRIT_PERCENTAGE`| 0x0410 | 1 | FLOAT |
|`PLAYER_EXPLORED_ZONES_1`| 0x0411 | 128 | BYTES |
|`PLAYER_REST_STATE_EXPERIENCE`| 0x0491 | 1 | INT |
|`PLAYER_COINAGE`| 0x0492 | 1 | INT |
|`PLAYER_MOD_DAMAGE_DONE_POS`| 0x0493 | 7 | INT |
|`PLAYER_MOD_DAMAGE_DONE_NEG`| 0x049a | 7 | INT |
|`PLAYER_MOD_DAMAGE_DONE_PCT`| 0x04a1 | 7 | INT |
|`PLAYER_MOD_HEALING_DONE_POS`| 0x04a8 | 1 | INT |
|`PLAYER_MOD_HEALING_PCT`| 0x04a9 | 1 | FLOAT |
|`PLAYER_MOD_HEALING_DONE_PCT`| 0x04aa | 1 | FLOAT |
|`PLAYER_MOD_TARGET_RESISTANCE`| 0x04ab | 1 | INT |
|`PLAYER_MOD_TARGET_PHYSICAL_RESISTANCE`| 0x04ac | 1 | INT |
|`PLAYER_FEATURES`| 0x04ad | 1 | BYTES |
|`PLAYER_AMMO_ID`| 0x04ae | 1 | INT |
|`PLAYER_SELF_RES_SPELL`| 0x04af | 1 | INT |
|`PLAYER_PVP_MEDALS`| 0x04b0 | 1 | INT |
|`PLAYER_BUYBACK_PRICE_1`| 0x04b1 | 12 | INT |
|`PLAYER_BUYBACK_TIMESTAMP_1`| 0x04bd | 12 | INT |
|`PLAYER_KILLS`| 0x04c9 | 1 | TWO_SHORT |
|`PLAYER_TODAY_CONTRIBUTION`| 0x04ca | 1 | INT |
|`PLAYER_YESTERDAY_CONTRIBUTION`| 0x04cb | 1 | INT |
|`PLAYER_LIFETIME_HONORBALE_KILLS`| 0x04cc | 1 | INT |
|`PLAYER_BYTES2`| 0x04cd | 1 | BYTES |
|`PLAYER_WATCHED_FACTION_INDEX`| 0x04ce | 1 | INT |
|`PLAYER_COMBAT_RATING_1`| 0x04cf | 25 | INT |
|`PLAYER_ARENA_TEAM_INFO_1_1`| 0x04e8 | 21 | INT |
|`PLAYER_HONOR_CURRENCY`| 0x04fd | 1 | INT |
|`PLAYER_ARENA_CURRENCY`| 0x04fe | 1 | INT |
|`PLAYER_MAX_LEVEL`| 0x04ff | 1 | INT |
|`PLAYER_DAILY_QUESTS_1`| 0x0500 | 25 | INT |
|`PLAYER_RUNE_REGEN_1`| 0x0519 | 4 | FLOAT |
|`PLAYER_NO_REAGENT_COST_1`| 0x051d | 3 | INT |
|`PLAYER_GLYPH_SLOTS_1`| 0x0520 | 6 | INT |
|`PLAYER_GLYPHS_1`| 0x0526 | 6 | INT |
|`PLAYER_GLYPHS_ENABLED`| 0x052c | 1 | INT |
|`PLAYER_PET_SPELL_POWER`| 0x052d | 1 | INT |


Fields that all gameobjects have:

| Name | Offset | Size | Type |
|------|--------|------|------|
|`GAMEOBJECT_DISPLAYID`| 0x0008 | 1 | INT |
|`GAMEOBJECT_FLAGS`| 0x0009 | 1 | INT |
|`GAMEOBJECT_PARENTROTATION`| 0x000a | 4 | FLOAT |
|`GAMEOBJECT_DYNAMIC`| 0x000e | 1 | TWO_SHORT |
|`GAMEOBJECT_FACTION`| 0x000f | 1 | INT |
|`GAMEOBJECT_LEVEL`| 0x0010 | 1 | INT |
|`GAMEOBJECT_BYTES_1`| 0x0011 | 1 | BYTES |


Fields that all dynamicobjects have:

| Name | Offset | Size | Type |
|------|--------|------|------|
|`DYNAMICOBJECT_CASTER`| 0x0006 | 2 | GUID |
|`DYNAMICOBJECT_BYTES`| 0x0008 | 1 | BYTES |
|`DYNAMICOBJECT_SPELLID`| 0x0009 | 1 | INT |
|`DYNAMICOBJECT_RADIUS`| 0x000a | 1 | FLOAT |
|`DYNAMICOBJECT_CASTTIME`| 0x000b | 1 | INT |


Fields that all corpses have:

| Name | Offset | Size | Type |
|------|--------|------|------|
|`CORPSE_OWNER`| 0x0006 | 2 | GUID |
|`CORPSE_PARTY`| 0x0008 | 2 | GUID |
|`CORPSE_DISPLAY_ID`| 0x000a | 1 | INT |
|`CORPSE_ITEM`| 0x000b | 19 | INT |
|`CORPSE_BYTES_1`| 0x001e | 1 | BYTES |
|`CORPSE_BYTES_2`| 0x001f | 1 | BYTES |
|`CORPSE_GUILD`| 0x0020 | 1 | INT |
|`CORPSE_FLAGS`| 0x0021 | 1 | INT |
|`CORPSE_DYNAMIC_FLAGS`| 0x0022 | 1 | INT |


