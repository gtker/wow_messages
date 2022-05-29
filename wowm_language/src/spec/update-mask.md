# UpdateMask

**NOT VALID FOR ALL VERSIONS. ONLY KNOWN VALID FOR 1.12.**

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

After the mask blocks we simply send the data in the order of their offsets.

## Lookup Table

Taken from [vmangos](https://github.com/vmangos/core/blob/4b2a5173b0ca4917dfe91aa7b87d84232fd7203c/src/game/Objects/UpdateFields_1_12_1.cpp#L5).

Fields that all objects have:

| Name | Offset | Size | Type |
|------|--------|------|------|
|`OBJECT_GUID`                               | 0x0  | 2  | GUID     |
|`OBJECT_TYPE`                               | 0x2  | 1  | INT      |
|`OBJECT_ENTRY`                              | 0x3  | 1  | INT      |
|`OBJECT_SCALE_X`                            | 0x4  | 1  | FLOAT    |
|`OBJECT_PADDING`                            | 0x5  | 1  | INT      |

Fields that items have (+ the object fields):

| Name | Offset | Size | Type |
|------|--------|------|------|
|`ITEM_OWNER`                                | 0x6  | 2  | GUID     |
|`ITEM_CONTAINED`                            | 0x8  | 2  | GUID     |
|`ITEM_CREATOR`                              | 0xA  | 2  | GUID     |
|`ITEM_GIFTCREATOR`                          | 0xC  | 2  | GUID     |
|`ITEM_STACK_COUNT`                          | 0xE  | 1  | INT      |
|`ITEM_DURATION`                             | 0xF  | 1  | INT      |
|`ITEM_SPELL_CHARGES`                        | 0x10 | 5  | INT      |
|`ITEM_FLAGS`                                | 0x15 | 1  | INT      |
|`ITEM_ENCHANTMENT`                          | 0x16 | 21 | INT      |
|`ITEM_PROPERTY_SEED`                        | 0x2B | 1  | INT      |
|`ITEM_RANDOM_PROPERTIES_ID`                 | 0x2C | 1  | INT      |
|`ITEM_ITEM_TEXT_ID`                         | 0x2D | 1  | INT      |
|`ITEM_DURABILITY`                           | 0x2E | 1  | INT      |
|`ITEM_MAXDURABILITY`                        | 0x2F | 1  | INT      |

Fields that containers (bags) have (+ the item and object fields):

| Name | Offset | Size | Type |
|------|--------|------|------|
|`CONTAINER_CONTAINER_FIELD_NUM_SLOTS`                       | 0x30 | 1  | INT      |
|`CONTAINER_CONTAINER_ALIGN_PAD`                             | 0x31 | 1  | BYTES    |
|`CONTAINER_CONTAINER_FIELD_SLOT_1`                          | 0x32 | 72 | GUID     |

Fields that units have (+ the object fields):

| Name | Offset | Size | Type |
|------|--------|------|------|
|`UNIT_CHARM`                                | 0x6  | 2  | GUID     |
|`UNIT_SUMMON`                               | 0x8  | 2  | GUID     |
|`UNIT_CHARMEDBY`                            | 0xA  | 2  | GUID     |
|`UNIT_SUMMONEDBY`                           | 0xC  | 2  | GUID     |
|`UNIT_CREATEDBY`                            | 0xE  | 2  | GUID     |
|`UNIT_TARGET`                               | 0x10 | 2  | GUID     |
|`UNIT_PERSUADED`                            | 0x12 | 2  | GUID     |
|`UNIT_CHANNEL_OBJECT`                       | 0x14 | 2  | GUID     |
|`UNIT_HEALTH`                               | 0x16 | 1  | INT      |
|`UNIT_POWER1`                               | 0x17 | 1  | INT      |
|`UNIT_POWER2`                               | 0x18 | 1  | INT      |
|`UNIT_POWER3`                               | 0x19 | 1  | INT      |
|`UNIT_POWER4`                               | 0x1A | 1  | INT      |
|`UNIT_POWER5`                               | 0x1B | 1  | INT      |
|`UNIT_MAXHEALTH`                            | 0x1C | 1  | INT      |
|`UNIT_MAXPOWER1`                            | 0x1D | 1  | INT      |
|`UNIT_MAXPOWER2`                            | 0x1E | 1  | INT      |
|`UNIT_MAXPOWER3`                            | 0x1F | 1  | INT      |
|`UNIT_MAXPOWER4`                            | 0x20 | 1  | INT      |
|`UNIT_MAXPOWER5`                            | 0x21 | 1  | INT      |
|`UNIT_LEVEL`                                | 0x22 | 1  | INT      |
|`UNIT_FACTIONTEMPLATE`                      | 0x23 | 1  | INT      |
|`UNIT_BYTES_0`                              | 0x24 | 1  | BYTES    |
|`UNIT_UNIT_VIRTUAL_ITEM_SLOT_DISPLAY`       | 0x25 | 3  | INT      |
|`UNIT_UNIT_VIRTUAL_ITEM_INFO`               | 0x28 | 6  | BYTES    |
|`UNIT_FLAGS`                                | 0x2E | 1  | INT      |
|`UNIT_AURA`                                 | 0x2F | 48 | INT      |
|`UNIT_AURAFLAGS`                            | 0x5F | 6  | BYTES    |
|`UNIT_AURALEVELS`                           | 0x65 | 12 | BYTES    |
|`UNIT_AURAAPPLICATIONS`                     | 0x71 | 12 | BYTES    |
|`UNIT_AURASTATE`                            | 0x7D | 1  | INT      |
|`UNIT_BASEATTACKTIME`                       | 0x7E | 2  | INT      |
|`UNIT_RANGEDATTACKTIME`                     | 0x80 | 1  | INT      |
|`UNIT_BOUNDINGRADIUS`                       | 0x81 | 1  | FLOAT    |
|`UNIT_COMBATREACH`                          | 0x82 | 1  | FLOAT    |
|`UNIT_DISPLAYID`                            | 0x83 | 1  | INT      |
|`UNIT_NATIVEDISPLAYID`                      | 0x84 | 1  | INT      |
|`UNIT_MOUNTDISPLAYID`                       | 0x85 | 1  | INT      |
|`UNIT_MINDAMAGE`                            | 0x86 | 1  | FLOAT    |
|`UNIT_MAXDAMAGE`                            | 0x87 | 1  | FLOAT    |
|`UNIT_MINOFFHANDDAMAGE`                     | 0x88 | 1  | FLOAT    |
|`UNIT_MAXOFFHANDDAMAGE`                     | 0x89 | 1  | FLOAT    |
|`UNIT_BYTES_1`                              | 0x8A | 1  | BYTES    |
|`UNIT_PETNUMBER`                            | 0x8B | 1  | INT      |
|`UNIT_PET_NAME_TIMESTAMP`                   | 0x8C | 1  | INT      |
|`UNIT_PETEXPERIENCE`                        | 0x8D | 1  | INT      |
|`UNIT_PETNEXTLEVELEXP`                      | 0x8E | 1  | INT      |
|`UNIT_UNIT_DYNAMIC_FLAGS`                   | 0x8F | 1  | INT      |
|`UNIT_UNIT_CHANNEL_SPELL`                   | 0x90 | 1  | INT      |
|`UNIT_UNIT_MOD_CAST_SPEED`                  | 0x91 | 1  | FLOAT    |
|`UNIT_UNIT_CREATED_BY_SPELL`                | 0x92 | 1  | INT      |
|`UNIT_UNIT_NPC_FLAGS`                       | 0x93 | 1  | INT      |
|`UNIT_UNIT_NPC_EMOTESTATE`                  | 0x94 | 1  | INT      |
|`UNIT_UNIT_TRAINING_POINTS`                 | 0x95 | 1  | TWO_SHORT|
|`UNIT_STAT0`                                | 0x96 | 1  | INT      |
|`UNIT_STAT1`                                | 0x97 | 1  | INT      |
|`UNIT_STAT2`                                | 0x98 | 1  | INT      |
|`UNIT_STAT3`                                | 0x99 | 1  | INT      |
|`UNIT_STAT4`                                | 0x9A | 1  | INT      |
|`UNIT_RESISTANCES`                          | 0x9B | 7  | INT      |
|`UNIT_BASE_MANA`                            | 0xA2 | 1  | INT      |
|`UNIT_BASE_HEALTH`                          | 0xA3 | 1  | INT      |
|`UNIT_BYTES_2`                              | 0xA4 | 1  | BYTES    |
|`UNIT_ATTACK_POWER`                         | 0xA5 | 1  | INT      |
|`UNIT_ATTACK_POWER_MODS`                    | 0xA6 | 1  | TWO_SHORT|
|`UNIT_ATTACK_POWER_MULTIPLIER`              | 0xA7 | 1  | FLOAT    |
|`UNIT_RANGED_ATTACK_POWER`                  | 0xA8 | 1  | INT      |
|`UNIT_RANGED_ATTACK_POWER_MODS`             | 0xA9 | 1  | TWO_SHORT|
|`UNIT_RANGED_ATTACK_POWER_MULTIPLIER`       | 0xAA | 1  | FLOAT    |
|`UNIT_MINRANGEDDAMAGE`                      | 0xAB | 1  | FLOAT    |
|`UNIT_MAXRANGEDDAMAGE`                      | 0xAC | 1  | FLOAT    |
|`UNIT_POWER_COST_MODIFIER`                  | 0xAD | 7  | INT      |
|`UNIT_POWER_COST_MULTIPLIER`                | 0xB4 | 7  | FLOAT    |
|`UNIT_PADDING`                              | 0xBB | 1  | INT      |

Fields that players have (+ the object and unit fields):

| Name | Offset | Size | Type |
|------|--------|------|------|
|`PLAYER_DUEL_ARBITER`                             | 0xBC | 2  | GUID     |
|`PLAYER_FLAGS`                                    | 0xBE | 1  | INT      |
|`PLAYER_GUILDID`                                  | 0xBF | 1  | INT      |
|`PLAYER_GUILDRANK`                                | 0xC0 | 1  | INT      |
|`PLAYER_BYTES`                                    | 0xC1 | 1  | BYTES    |
|`PLAYER_BYTES_2`                                  | 0xC2 | 1  | BYTES    |
|`PLAYER_BYTES_3`                                  | 0xC3 | 1  | BYTES    |
|`PLAYER_DUEL_TEAM`                                | 0xC4 | 1  | INT      |
|`PLAYER_GUILD_TIMESTAMP`                          | 0xC5 | 1  | INT      |
|`PLAYER_QUEST_LOG_1_1`                            | 0xC6 | 1  | INT      |
|`PLAYER_QUEST_LOG_1_2`                            | 0xC7 | 2  | INT      |
|`PLAYER_QUEST_LOG_2_1`                            | 0xC9 | 1  | INT      |
|`PLAYER_QUEST_LOG_2_2`                            | 0xCA | 2  | INT      |
|`PLAYER_QUEST_LOG_3_1`                            | 0xCC | 1  | INT      |
|`PLAYER_QUEST_LOG_3_2`                            | 0xCD | 2  | INT      |
|`PLAYER_QUEST_LOG_4_1`                            | 0xCF | 1  | INT      |
|`PLAYER_QUEST_LOG_4_2`                            | 0xD0 | 2  | INT      |
|`PLAYER_QUEST_LOG_5_1`                            | 0xD2 | 1  | INT      |
|`PLAYER_QUEST_LOG_5_2`                            | 0xD3 | 2  | INT      |
|`PLAYER_QUEST_LOG_6_1`                            | 0xD5 | 1  | INT      |
|`PLAYER_QUEST_LOG_6_2`                            | 0xD6 | 2  | INT      |
|`PLAYER_QUEST_LOG_7_1`                            | 0xD8 | 1  | INT      |
|`PLAYER_QUEST_LOG_7_2`                            | 0xD9 | 2  | INT      |
|`PLAYER_QUEST_LOG_8_1`                            | 0xDB | 1  | INT      |
|`PLAYER_QUEST_LOG_8_2`                            | 0xDC | 2  | INT      |
|`PLAYER_QUEST_LOG_9_1`                            | 0xDE | 1  | INT      |
|`PLAYER_QUEST_LOG_9_2`                            | 0xDF | 2  | INT      |
|`PLAYER_QUEST_LOG_10_1`                           | 0xE1 | 1  | INT      |
|`PLAYER_QUEST_LOG_10_2`                           | 0xE2 | 2  | INT      |
|`PLAYER_QUEST_LOG_11_1`                           | 0xE4 | 1  | INT      |
|`PLAYER_QUEST_LOG_11_2`                           | 0xE5 | 2  | INT      |
|`PLAYER_QUEST_LOG_12_1`                           | 0xE7 | 1  | INT      |
|`PLAYER_QUEST_LOG_12_2`                           | 0xE8 | 2  | INT      |
|`PLAYER_QUEST_LOG_13_1`                           | 0xEA | 1  | INT      |
|`PLAYER_QUEST_LOG_13_2`                           | 0xEB | 2  | INT      |
|`PLAYER_QUEST_LOG_14_1`                           | 0xED | 1  | INT      |
|`PLAYER_QUEST_LOG_14_2`                           | 0xEE | 2  | INT      |
|`PLAYER_QUEST_LOG_15_1`                           | 0xF0 | 1  | INT      |
|`PLAYER_QUEST_LOG_15_2`                           | 0xF1 | 2  | INT      |
|`PLAYER_QUEST_LOG_16_1`                           | 0xF3 | 1  | INT      |
|`PLAYER_QUEST_LOG_16_2`                           | 0xF4 | 2  | INT      |
|`PLAYER_QUEST_LOG_17_1`                           | 0xF6 | 1  | INT      |
|`PLAYER_QUEST_LOG_17_2`                           | 0xF7 | 2  | INT      |
|`PLAYER_QUEST_LOG_18_1`                           | 0xF9 | 1  | INT      |
|`PLAYER_QUEST_LOG_18_2`                           | 0xFA | 2  | INT      |
|`PLAYER_QUEST_LOG_19_1`                           | 0xFC | 1  | INT      |
|`PLAYER_QUEST_LOG_19_2`                           | 0xFD | 2  | INT      |
|`PLAYER_QUEST_LOG_20_1`                           | 0xFF | 1  | INT      |
|`PLAYER_QUEST_LOG_20_2`                           | 0x100| 2  | INT      |
|`PLAYER_VISIBLE_ITEM_1_CREATOR`                   | 0x102| 2  | GUID     |
|`PLAYER_VISIBLE_ITEM_1_0`                         | 0x104| 8  | INT      |
|`PLAYER_VISIBLE_ITEM_1_PROPERTIES`                | 0x10C| 1  | TWO_SHORT|
|`PLAYER_VISIBLE_ITEM_1_PAD`                       | 0x10D| 1  | INT      |
|`PLAYER_VISIBLE_ITEM_2_CREATOR`                   | 0x10E| 2  | GUID     |
|`PLAYER_VISIBLE_ITEM_2_0`                         | 0x110| 8  | INT      |
|`PLAYER_VISIBLE_ITEM_2_PROPERTIES`                | 0x118| 1  | TWO_SHORT|
|`PLAYER_VISIBLE_ITEM_2_PAD`                       | 0x119| 1  | INT      |
|`PLAYER_VISIBLE_ITEM_3_CREATOR`                   | 0x11A| 2  | GUID     |
|`PLAYER_VISIBLE_ITEM_3_0`                         | 0x11C| 8  | INT      |
|`PLAYER_VISIBLE_ITEM_3_PROPERTIES`                | 0x124| 1  | TWO_SHORT|
|`PLAYER_VISIBLE_ITEM_3_PAD`                       | 0x125| 1  | INT      |
|`PLAYER_VISIBLE_ITEM_4_CREATOR`                   | 0x126| 2  | GUID     |
|`PLAYER_VISIBLE_ITEM_4_0`                         | 0x128| 8  | INT      |
|`PLAYER_VISIBLE_ITEM_4_PROPERTIES`                | 0x130| 1  | TWO_SHORT|
|`PLAYER_VISIBLE_ITEM_4_PAD`                       | 0x131| 1  | INT      |
|`PLAYER_VISIBLE_ITEM_5_CREATOR`                   | 0x132| 2  | GUID     |
|`PLAYER_VISIBLE_ITEM_5_0`                         | 0x134| 8  | INT      |
|`PLAYER_VISIBLE_ITEM_5_PROPERTIES`                | 0x13C| 1  | TWO_SHORT|
|`PLAYER_VISIBLE_ITEM_5_PAD`                       | 0x13D| 1  | INT      |
|`PLAYER_VISIBLE_ITEM_6_CREATOR`                   | 0x13E| 2  | GUID     |
|`PLAYER_VISIBLE_ITEM_6_0`                         | 0x140| 8  | INT      |
|`PLAYER_VISIBLE_ITEM_6_PROPERTIES`                | 0x148| 1  | TWO_SHORT|
|`PLAYER_VISIBLE_ITEM_6_PAD`                       | 0x149| 1  | INT      |
|`PLAYER_VISIBLE_ITEM_7_CREATOR`                   | 0x14A| 2  | GUID     |
|`PLAYER_VISIBLE_ITEM_7_0`                         | 0x14C| 8  | INT      |
|`PLAYER_VISIBLE_ITEM_7_PROPERTIES`                | 0x154| 1  | TWO_SHORT|
|`PLAYER_VISIBLE_ITEM_7_PAD`                       | 0x155| 1  | INT      |
|`PLAYER_VISIBLE_ITEM_8_CREATOR`                   | 0x156| 2  | GUID     |
|`PLAYER_VISIBLE_ITEM_8_0`                         | 0x158| 8  | INT      |
|`PLAYER_VISIBLE_ITEM_8_PROPERTIES`                | 0x160| 1  | TWO_SHORT|
|`PLAYER_VISIBLE_ITEM_8_PAD`                       | 0x161| 1  | INT      |
|`PLAYER_VISIBLE_ITEM_9_CREATOR`                   | 0x162| 2  | GUID     |
|`PLAYER_VISIBLE_ITEM_9_0`                         | 0x164| 8  | INT      |
|`PLAYER_VISIBLE_ITEM_9_PROPERTIES`                | 0x16C| 1  | TWO_SHORT|
|`PLAYER_VISIBLE_ITEM_9_PAD`                       | 0x16D| 1  | INT      |
|`PLAYER_VISIBLE_ITEM_10_CREATOR`                  | 0x16E| 2  | GUID     |
|`PLAYER_VISIBLE_ITEM_10_0`                        | 0x170| 8  | INT      |
|`PLAYER_VISIBLE_ITEM_10_PROPERTIES`               | 0x178| 1  | TWO_SHORT|
|`PLAYER_VISIBLE_ITEM_10_PAD`                      | 0x179| 1  | INT      |
|`PLAYER_VISIBLE_ITEM_11_CREATOR`                  | 0x17A| 2  | GUID     |
|`PLAYER_VISIBLE_ITEM_11_0`                        | 0x17C| 8  | INT      |
|`PLAYER_VISIBLE_ITEM_11_PROPERTIES`               | 0x184| 1  | TWO_SHORT|
|`PLAYER_VISIBLE_ITEM_11_PAD`                      | 0x185| 1  | INT      |
|`PLAYER_VISIBLE_ITEM_12_CREATOR`                  | 0x186| 2  | GUID     |
|`PLAYER_VISIBLE_ITEM_12_0`                        | 0x188| 8  | INT      |
|`PLAYER_VISIBLE_ITEM_12_PROPERTIES`               | 0x190| 1  | TWO_SHORT|
|`PLAYER_VISIBLE_ITEM_12_PAD`                      | 0x191| 1  | INT      |
|`PLAYER_VISIBLE_ITEM_13_CREATOR`                  | 0x192| 2  | GUID     |
|`PLAYER_VISIBLE_ITEM_13_0`                        | 0x194| 8  | INT      |
|`PLAYER_VISIBLE_ITEM_13_PROPERTIES`               | 0x19C| 1  | TWO_SHORT|
|`PLAYER_VISIBLE_ITEM_13_PAD`                      | 0x19D| 1  | INT      |
|`PLAYER_VISIBLE_ITEM_14_CREATOR`                  | 0x19E| 2  | GUID     |
|`PLAYER_VISIBLE_ITEM_14_0`                        | 0x1A0| 8  | INT      |
|`PLAYER_VISIBLE_ITEM_14_PROPERTIES`               | 0x1A8| 1  | TWO_SHORT|
|`PLAYER_VISIBLE_ITEM_14_PAD`                      | 0x1A9| 1  | INT      |
|`PLAYER_VISIBLE_ITEM_15_CREATOR`                  | 0x1AA| 2  | GUID     |
|`PLAYER_VISIBLE_ITEM_15_0`                        | 0x1AC| 8  | INT      |
|`PLAYER_VISIBLE_ITEM_15_PROPERTIES`               | 0x1B4| 1  | TWO_SHORT|
|`PLAYER_VISIBLE_ITEM_15_PAD`                      | 0x1B5| 1  | INT      |
|`PLAYER_VISIBLE_ITEM_16_CREATOR`                  | 0x1B6| 2  | GUID     |
|`PLAYER_VISIBLE_ITEM_16_0`                        | 0x1B8| 8  | INT      |
|`PLAYER_VISIBLE_ITEM_16_PROPERTIES`               | 0x1C0| 1  | TWO_SHORT|
|`PLAYER_VISIBLE_ITEM_16_PAD`                      | 0x1C1| 1  | INT      |
|`PLAYER_VISIBLE_ITEM_17_CREATOR`                  | 0x1C2| 2  | GUID     |
|`PLAYER_VISIBLE_ITEM_17_0`                        | 0x1C4| 8  | INT      |
|`PLAYER_VISIBLE_ITEM_17_PROPERTIES`               | 0x1CC| 1  | TWO_SHORT|
|`PLAYER_VISIBLE_ITEM_17_PAD`                      | 0x1CD| 1  | INT      |
|`PLAYER_VISIBLE_ITEM_18_CREATOR`                  | 0x1CE| 2  | GUID     |
|`PLAYER_VISIBLE_ITEM_18_0`                        | 0x1D0| 8  | INT      |
|`PLAYER_VISIBLE_ITEM_18_PROPERTIES`               | 0x1D8| 1  | TWO_SHORT|
|`PLAYER_VISIBLE_ITEM_18_PAD`                      | 0x1D9| 1  | INT      |
|`PLAYER_VISIBLE_ITEM_19_CREATOR`                  | 0x1DA| 2  | GUID     |
|`PLAYER_VISIBLE_ITEM_19_0`                        | 0x1DC| 8  | INT      |
|`PLAYER_VISIBLE_ITEM_19_PROPERTIES`               | 0x1E4| 1  | TWO_SHORT|
|`PLAYER_VISIBLE_ITEM_19_PAD`                      | 0x1E5| 1  | INT      |
|`PLAYER_FIELD_INV_SLOT_HEAD`                      | 0x1E6| 46 | GUID     |
|`PLAYER_FIELD_PACK_SLOT_1`                        | 0x214| 32 | GUID     |
|`PLAYER_FIELD_BANK_SLOT_1`                        | 0x234| 48 | GUID     |
|`PLAYER_FIELD_BANKBAG_SLOT_1`                     | 0x264| 12 | GUID     |
|`PLAYER_FIELD_VENDORBUYBACK_SLOT_1`               | 0x270| 24 | GUID     |
|`PLAYER_FIELD_KEYRING_SLOT_1`                     | 0x288| 64 | GUID     |
|`PLAYER_FARSIGHT`                                 | 0x2C8| 2  | GUID     |
|`PLAYER_FIELD_COMBO_TARGET`                       | 0x2CA| 2  | GUID     |
|`PLAYER_XP`                                       | 0x2CC| 1  | INT      |
|`PLAYER_NEXT_LEVEL_XP`                            | 0x2CD| 1  | INT      |
|`PLAYER_SKILL_INFO_1_1`                           | 0x2CE| 384| TWO_SHORT|
|`PLAYER_CHARACTER_POINTS1`                        | 0x44E| 1  | INT      |
|`PLAYER_CHARACTER_POINTS2`                        | 0x44F| 1  | INT      |
|`PLAYER_TRACK_CREATURES`                          | 0x450| 1  | INT      |
|`PLAYER_TRACK_RESOURCES`                          | 0x451| 1  | INT      |
|`PLAYER_BLOCK_PERCENTAGE`                         | 0x452| 1  | FLOAT    |
|`PLAYER_DODGE_PERCENTAGE`                         | 0x453| 1  | FLOAT    |
|`PLAYER_PARRY_PERCENTAGE`                         | 0x454| 1  | FLOAT    |
|`PLAYER_CRIT_PERCENTAGE`                          | 0x455| 1  | FLOAT    |
|`PLAYER_RANGED_CRIT_PERCENTAGE`                   | 0x456| 1  | FLOAT    |
|`PLAYER_EXPLORED_ZONES_1`                         | 0x457| 64 | BYTES    |
|`PLAYER_REST_STATE_EXPERIENCE`                    | 0x497| 1  | INT      |
|`PLAYER_FIELD_COINAGE`                            | 0x498| 1  | INT      |
|`PLAYER_FIELD_POSSTAT0`                           | 0x499| 1  | INT      |
|`PLAYER_FIELD_POSSTAT1`                           | 0x49A| 1  | INT      |
|`PLAYER_FIELD_POSSTAT2`                           | 0x49B| 1  | INT      |
|`PLAYER_FIELD_POSSTAT3`                           | 0x49C| 1  | INT      |
|`PLAYER_FIELD_POSSTAT4`                           | 0x49D| 1  | INT      |
|`PLAYER_FIELD_NEGSTAT0`                           | 0x49E| 1  | INT      |
|`PLAYER_FIELD_NEGSTAT1`                           | 0x49F| 1  | INT      |
|`PLAYER_FIELD_NEGSTAT2`                           | 0x4A0| 1  | INT      |
|`PLAYER_FIELD_NEGSTAT3`                           | 0x4A1| 1  | INT      |
|`PLAYER_FIELD_NEGSTAT4`                           | 0x4A2| 1  | INT      |
|`PLAYER_FIELD_RESISTANCEBUFFMODSPOSITIVE`         | 0x4A3| 7  | INT      |
|`PLAYER_FIELD_RESISTANCEBUFFMODSNEGATIVE`         | 0x4AA| 7  | INT      |
|`PLAYER_FIELD_MOD_DAMAGE_DONE_POS`                | 0x4B1| 7  | INT      |
|`PLAYER_FIELD_MOD_DAMAGE_DONE_NEG`                | 0x4B8| 7  | INT      |
|`PLAYER_FIELD_MOD_DAMAGE_DONE_PCT`                | 0x4BF| 7  | INT      |
|`PLAYER_FIELD_BYTES`                              | 0x4C6| 1  | BYTES    |
|`PLAYER_AMMO_ID`                                  | 0x4C7| 1  | INT      |
|`PLAYER_SELF_RES_SPELL`                           | 0x4C8| 1  | INT      |
|`PLAYER_FIELD_PVP_MEDALS`                         | 0x4C9| 1  | INT      |
|`PLAYER_FIELD_BUYBACK_PRICE_1`                    | 0x4CA| 12 | INT      |
|`PLAYER_FIELD_BUYBACK_TIMESTAMP_1`                | 0x4D6| 12 | INT      |
|`PLAYER_FIELD_SESSION_KILLS`                      | 0x4E2| 1  | TWO_SHORT|
|`PLAYER_FIELD_YESTERDAY_KILLS`                    | 0x4E3| 1  | TWO_SHORT|
|`PLAYER_FIELD_LAST_WEEK_KILLS`                    | 0x4E4| 1  | TWO_SHORT|
|`PLAYER_FIELD_THIS_WEEK_KILLS`                    | 0x4E5| 1  | TWO_SHORT|
|`PLAYER_FIELD_THIS_WEEK_CONTRIBUTION`             | 0x4E6| 1  | INT      |
|`PLAYER_FIELD_LIFETIME_HONORBALE_KILLS`           | 0x4E7| 1  | INT      |
|`PLAYER_FIELD_LIFETIME_DISHONORBALE_KILLS`        | 0x4E8| 1  | INT      |
|`PLAYER_FIELD_YESTERDAY_CONTRIBUTION`             | 0x4E9| 1  | INT      |
|`PLAYER_FIELD_LAST_WEEK_CONTRIBUTION`             | 0x4EA| 1  | INT      |
|`PLAYER_FIELD_LAST_WEEK_RANK`                     | 0x4EB| 1  | INT      |
|`PLAYER_FIELD_BYTES2`                             | 0x4EC| 1  | BYTES    |
|`PLAYER_FIELD_WATCHED_FACTION_INDEX`              | 0x4ED| 1  | INT      |
|`PLAYER_FIELD_COMBAT_RATING_1`                    | 0x4EE| 20 | INT      |

Fields that gameobjects have (+ the object fields):

| Name | Offset | Size | Type |
|------|--------|------|------|
|`GAMEOBJECT_CREATED_BY`                         | 0x6  | 2  | GUID     |
|`GAMEOBJECT_DISPLAYID`                            | 0x8  | 1  | INT      |
|`GAMEOBJECT_FLAGS`                                | 0x9  | 1  | INT      |
|`GAMEOBJECT_ROTATION`                             | 0xA  | 4  | FLOAT    |
|`GAMEOBJECT_STATE`                                | 0xE  | 1  | INT      |
|`GAMEOBJECT_POS_X`                                | 0xF  | 1  | FLOAT    |
|`GAMEOBJECT_POS_Y`                                | 0x10 | 1  | FLOAT    |
|`GAMEOBJECT_POS_Z`                                | 0x11 | 1  | FLOAT    |
|`GAMEOBJECT_FACING`                               | 0x12 | 1  | FLOAT    |
|`GAMEOBJECT_DYN_FLAGS`                            | 0x13 | 1  | INT      |
|`GAMEOBJECT_FACTION`                              | 0x14 | 1  | INT      |
|`GAMEOBJECT_TYPE_ID`                              | 0x15 | 1  | INT      |
|`GAMEOBJECT_LEVEL`                                | 0x16 | 1  | INT      |
|`GAMEOBJECT_ARTKIT`                               | 0x17 | 1  | INT      |
|`GAMEOBJECT_ANIMPROGRESS`                         | 0x18 | 1  | INT      |
|`GAMEOBJECT_PADDING`                              | 0x19 | 1  | INT      |

Fields that dynamic objects have (+ the object fields):

| Name | Offset | Size | Type |
|------|--------|------|------|
|`DYNAMICOBJECT_CASTER`                            | 0x6  | 2  | GUID     |
|`DYNAMICOBJECT_BYTES`                             | 0x8  | 1  | BYTES    |
|`DYNAMICOBJECT_SPELLID`                           | 0x9  | 1  | INT      |
|`DYNAMICOBJECT_RADIUS`                            | 0xA  | 1  | FLOAT    |
|`DYNAMICOBJECT_POS_X`                             | 0xB  | 1  | FLOAT    |
|`DYNAMICOBJECT_POS_Y`                             | 0xC  | 1  | FLOAT    |
|`DYNAMICOBJECT_POS_Z`                             | 0xD  | 1  | FLOAT    |
|`DYNAMICOBJECT_FACING`                            | 0xE  | 1  | FLOAT    |
|`DYNAMICOBJECT_PAD`                               | 0xF  | 1  | BYTES    |

Fields that corpses have (+ the object fields):

| Name | Offset | Size | Type |
|------|--------|------|------|
|`CORPSE_OWNER`                              | 0x6  | 2  | GUID     |
|`CORPSE_FACING`                             | 0x8  | 1  | FLOAT    |
|`CORPSE_POS_X`                              | 0x9  | 1  | FLOAT    |
|`CORPSE_POS_Y`                              | 0xA  | 1  | FLOAT    |
|`CORPSE_POS_Z`                              | 0xB  | 1  | FLOAT    |
|`CORPSE_DISPLAY_ID`                         | 0xC  | 1  | INT      |
|`CORPSE_ITEM`                               | 0xD  | 19 | INT      |
|`CORPSE_BYTES_1`                            | 0x20 | 1  | BYTES    |
|`CORPSE_BYTES_2`                            | 0x21 | 1  | BYTES    |
|`CORPSE_GUILD`                              | 0x22 | 1  | INT      |
|`CORPSE_FLAGS`                              | 0x23 | 1  | INT      |
|`CORPSE_DYNAMIC_FLAGS`                      | 0x24 | 1  | INT      |
|`CORPSE_PAD`                                | 0x25 | 1  | INT      |
