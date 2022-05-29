# UpdateMask

**NOT VALID FOR ALL VERSIONS. ONLY KNOWN VALID FOR 1.12.**

TODO

An UpdateMask is a variable length way of sending known fields to the client.
It is represented by a byte mask that decides which fields are sent afterwards.

## Representation

The UpdateMask starts with a single u8 that decides how many **u32 mask blocks** will follow.
The bit pattern in these mask blocks determine how many additional u32s of data will follow and how to interpret that data.

## Examples


## Lookup Table

Taken from [vmangos](https://github.com/vmangos/core/blob/4b2a5173b0ca4917dfe91aa7b87d84232fd7203c/src/game/Objects/UpdateFields_1_12_1.cpp#L5).

| Object Type | Name | Offset | Size | Type |
|-------------|------|--------|------|------|
|OBJECT       | `GUID`                               | 0x0  | 2  | GUID     |
|OBJECT       | `TYPE`                               | 0x2  | 1  | INT      |
|OBJECT       | `ENTRY`                              | 0x3  | 1  | INT      |
|OBJECT       | `SCALE_X`                            | 0x4  | 1  | FLOAT    |
|OBJECT       | `PADDING`                            | 0x5  | 1  | INT      |
|ITEM         | `OWNER`                                | 0x6  | 2  | GUID     |
|ITEM         | `CONTAINED`                            | 0x8  | 2  | GUID     |
|ITEM         | `CREATOR`                              | 0xA  | 2  | GUID     |
|ITEM         | `GIFTCREATOR`                          | 0xC  | 2  | GUID     |
|ITEM         | `STACK_COUNT`                          | 0xE  | 1  | INT      |
|ITEM         | `DURATION`                             | 0xF  | 1  | INT      |
|ITEM         | `SPELL_CHARGES`                        | 0x10 | 5  | INT      |
|ITEM         | `FLAGS`                                | 0x15 | 1  | INT      |
|ITEM         | `ENCHANTMENT`                          | 0x16 | 21 | INT      |
|ITEM         | `PROPERTY_SEED`                        | 0x2B | 1  | INT      |
|ITEM         | `RANDOM_PROPERTIES_ID`                 | 0x2C | 1  | INT      |
|ITEM         | `ITEM_TEXT_ID`                         | 0x2D | 1  | INT      |
|ITEM         | `DURABILITY`                           | 0x2E | 1  | INT      |
|ITEM         | `MAXDURABILITY`                        | 0x2F | 1  | INT      |
|CONTAINER    | `CONTAINER_FIELD_NUM_SLOTS`                       | 0x30 | 1  | INT      |
|CONTAINER    | `CONTAINER_ALIGN_PAD`                             | 0x31 | 1  | BYTES    |
|CONTAINER    | `CONTAINER_FIELD_SLOT_1`                          | 0x32 | 72 | GUID     |
|UNIT         | `CHARM`                                | 0x6  | 2  | GUID     |
|UNIT         | `SUMMON`                               | 0x8  | 2  | GUID     |
|UNIT         | `CHARMEDBY`                            | 0xA  | 2  | GUID     |
|UNIT         | `SUMMONEDBY`                           | 0xC  | 2  | GUID     |
|UNIT         | `CREATEDBY`                            | 0xE  | 2  | GUID     |
|UNIT         | `TARGET`                               | 0x10 | 2  | GUID     |
|UNIT         | `PERSUADED`                            | 0x12 | 2  | GUID     |
|UNIT         | `CHANNEL_OBJECT`                       | 0x14 | 2  | GUID     |
|UNIT         | `HEALTH`                               | 0x16 | 1  | INT      |
|UNIT         | `POWER1`                               | 0x17 | 1  | INT      |
|UNIT         | `POWER2`                               | 0x18 | 1  | INT      |
|UNIT         | `POWER3`                               | 0x19 | 1  | INT      |
|UNIT         | `POWER4`                               | 0x1A | 1  | INT      |
|UNIT         | `POWER5`                               | 0x1B | 1  | INT      |
|UNIT         | `MAXHEALTH`                            | 0x1C | 1  | INT      |
|UNIT         | `MAXPOWER1`                            | 0x1D | 1  | INT      |
|UNIT         | `MAXPOWER2`                            | 0x1E | 1  | INT      |
|UNIT         | `MAXPOWER3`                            | 0x1F | 1  | INT      |
|UNIT         | `MAXPOWER4`                            | 0x20 | 1  | INT      |
|UNIT         | `MAXPOWER5`                            | 0x21 | 1  | INT      |
|UNIT         | `LEVEL`                                | 0x22 | 1  | INT      |
|UNIT         | `FACTIONTEMPLATE`                      | 0x23 | 1  | INT      |
|UNIT         | `BYTES_0`                              | 0x24 | 1  | BYTES    |
|UNIT         | `UNIT_VIRTUAL_ITEM_SLOT_DISPLAY`                  | 0x25 | 3  | INT      |
|UNIT         | `UNIT_VIRTUAL_ITEM_INFO`                          | 0x28 | 6  | BYTES    |
|UNIT         | `FLAGS`                                | 0x2E | 1  | INT      |
|UNIT         | `AURA`                                 | 0x2F | 48 | INT      |
|UNIT         | `AURAFLAGS`                            | 0x5F | 6  | BYTES    |
|UNIT         | `AURALEVELS`                           | 0x65 | 12 | BYTES    |
|UNIT         | `AURAAPPLICATIONS`                     | 0x71 | 12 | BYTES    |
|UNIT         | `AURASTATE`                            | 0x7D | 1  | INT      |
|UNIT         | `BASEATTACKTIME`                       | 0x7E | 2  | INT      |
|UNIT         | `RANGEDATTACKTIME`                     | 0x80 | 1  | INT      |
|UNIT         | `BOUNDINGRADIUS`                       | 0x81 | 1  | FLOAT    |
|UNIT         | `COMBATREACH`                          | 0x82 | 1  | FLOAT    |
|UNIT         | `DISPLAYID`                            | 0x83 | 1  | INT      |
|UNIT         | `NATIVEDISPLAYID`                      | 0x84 | 1  | INT      |
|UNIT         | `MOUNTDISPLAYID`                       | 0x85 | 1  | INT      |
|UNIT         | `MINDAMAGE`                            | 0x86 | 1  | FLOAT    |
|UNIT         | `MAXDAMAGE`                            | 0x87 | 1  | FLOAT    |
|UNIT         | `MINOFFHANDDAMAGE`                     | 0x88 | 1  | FLOAT    |
|UNIT         | `MAXOFFHANDDAMAGE`                     | 0x89 | 1  | FLOAT    |
|UNIT         | `BYTES_1`                              | 0x8A | 1  | BYTES    |
|UNIT         | `PETNUMBER`                            | 0x8B | 1  | INT      |
|UNIT         | `PET_NAME_TIMESTAMP`                   | 0x8C | 1  | INT      |
|UNIT         | `PETEXPERIENCE`                        | 0x8D | 1  | INT      |
|UNIT         | `PETNEXTLEVELEXP`                      | 0x8E | 1  | INT      |
|UNIT         | `UNIT_DYNAMIC_FLAGS`                              | 0x8F | 1  | INT      |
|UNIT         | `UNIT_CHANNEL_SPELL`                              | 0x90 | 1  | INT      |
|UNIT         | `UNIT_MOD_CAST_SPEED`                             | 0x91 | 1  | FLOAT    |
|UNIT         | `UNIT_CREATED_BY_SPELL`                           | 0x92 | 1  | INT      |
|UNIT         | `UNIT_NPC_FLAGS`                                  | 0x93 | 1  | INT      |
|UNIT         | `UNIT_NPC_EMOTESTATE`                             | 0x94 | 1  | INT      |
|UNIT         | `UNIT_TRAINING_POINTS`                            | 0x95 | 1  | TWO_SHORT|
|UNIT         | `STAT0`                                | 0x96 | 1  | INT      |
|UNIT         | `STAT1`                                | 0x97 | 1  | INT      |
|UNIT         | `STAT2`                                | 0x98 | 1  | INT      |
|UNIT         | `STAT3`                                | 0x99 | 1  | INT      |
|UNIT         | `STAT4`                                | 0x9A | 1  | INT      |
|UNIT         | `RESISTANCES`                          | 0x9B | 7  | INT      |
|UNIT         | `BASE_MANA`                            | 0xA2 | 1  | INT      |
|UNIT         | `BASE_HEALTH`                          | 0xA3 | 1  | INT      |
|UNIT         | `BYTES_2`                              | 0xA4 | 1  | BYTES    |
|UNIT         | `ATTACK_POWER`                         | 0xA5 | 1  | INT      |
|UNIT         | `ATTACK_POWER_MODS`                    | 0xA6 | 1  | TWO_SHORT|
|UNIT         | `ATTACK_POWER_MULTIPLIER`              | 0xA7 | 1  | FLOAT    |
|UNIT         | `RANGED_ATTACK_POWER`                  | 0xA8 | 1  | INT      |
|UNIT         | `RANGED_ATTACK_POWER_MODS`             | 0xA9 | 1  | TWO_SHORT|
|UNIT         | `RANGED_ATTACK_POWER_MULTIPLIER`       | 0xAA | 1  | FLOAT    |
|UNIT         | `MINRANGEDDAMAGE`                      | 0xAB | 1  | FLOAT    |
|UNIT         | `MAXRANGEDDAMAGE`                      | 0xAC | 1  | FLOAT    |
|UNIT         | `POWER_COST_MODIFIER`                  | 0xAD | 7  | INT      |
|UNIT         | `POWER_COST_MULTIPLIER`                | 0xB4 | 7  | FLOAT    |
|UNIT         | `PADDING`                              | 0xBB | 1  | INT      |
|PLAYER       | `DUEL_ARBITER`                             | 0xBC | 2  | GUID     |
|PLAYER       | `FLAGS`                                    | 0xBE | 1  | INT      |
|PLAYER       | `GUILDID`                                  | 0xBF | 1  | INT      |
|PLAYER       | `GUILDRANK`                                | 0xC0 | 1  | INT      |
|PLAYER       | `BYTES`                                    | 0xC1 | 1  | BYTES    |
|PLAYER       | `BYTES_2`                                  | 0xC2 | 1  | BYTES    |
|PLAYER       | `BYTES_3`                                  | 0xC3 | 1  | BYTES    |
|PLAYER       | `DUEL_TEAM`                                | 0xC4 | 1  | INT      |
|PLAYER       | `GUILD_TIMESTAMP`                          | 0xC5 | 1  | INT      |
|PLAYER       | `QUEST_LOG_1_1`                            | 0xC6 | 1  | INT      |
|PLAYER       | `QUEST_LOG_1_2`                            | 0xC7 | 2  | INT      |
|PLAYER       | `QUEST_LOG_2_1`                            | 0xC9 | 1  | INT      |
|PLAYER       | `QUEST_LOG_2_2`                            | 0xCA | 2  | INT      |
|PLAYER       | `QUEST_LOG_3_1`                            | 0xCC | 1  | INT      |
|PLAYER       | `QUEST_LOG_3_2`                            | 0xCD | 2  | INT      |
|PLAYER       | `QUEST_LOG_4_1`                            | 0xCF | 1  | INT      |
|PLAYER       | `QUEST_LOG_4_2`                            | 0xD0 | 2  | INT      |
|PLAYER       | `QUEST_LOG_5_1`                            | 0xD2 | 1  | INT      |
|PLAYER       | `QUEST_LOG_5_2`                            | 0xD3 | 2  | INT      |
|PLAYER       | `QUEST_LOG_6_1`                            | 0xD5 | 1  | INT      |
|PLAYER       | `QUEST_LOG_6_2`                            | 0xD6 | 2  | INT      |
|PLAYER       | `QUEST_LOG_7_1`                            | 0xD8 | 1  | INT      |
|PLAYER       | `QUEST_LOG_7_2`                            | 0xD9 | 2  | INT      |
|PLAYER       | `QUEST_LOG_8_1`                            | 0xDB | 1  | INT      |
|PLAYER       | `QUEST_LOG_8_2`                            | 0xDC | 2  | INT      |
|PLAYER       | `QUEST_LOG_9_1`                            | 0xDE | 1  | INT      |
|PLAYER       | `QUEST_LOG_9_2`                            | 0xDF | 2  | INT      |
|PLAYER       | `QUEST_LOG_10_1`                           | 0xE1 | 1  | INT      |
|PLAYER       | `QUEST_LOG_10_2`                           | 0xE2 | 2  | INT      |
|PLAYER       | `QUEST_LOG_11_1`                           | 0xE4 | 1  | INT      |
|PLAYER       | `QUEST_LOG_11_2`                           | 0xE5 | 2  | INT      |
|PLAYER       | `QUEST_LOG_12_1`                           | 0xE7 | 1  | INT      |
|PLAYER       | `QUEST_LOG_12_2`                           | 0xE8 | 2  | INT      |
|PLAYER       | `QUEST_LOG_13_1`                           | 0xEA | 1  | INT      |
|PLAYER       | `QUEST_LOG_13_2`                           | 0xEB | 2  | INT      |
|PLAYER       | `QUEST_LOG_14_1`                           | 0xED | 1  | INT      |
|PLAYER       | `QUEST_LOG_14_2`                           | 0xEE | 2  | INT      |
|PLAYER       | `QUEST_LOG_15_1`                           | 0xF0 | 1  | INT      |
|PLAYER       | `QUEST_LOG_15_2`                           | 0xF1 | 2  | INT      |
|PLAYER       | `QUEST_LOG_16_1`                           | 0xF3 | 1  | INT      |
|PLAYER       | `QUEST_LOG_16_2`                           | 0xF4 | 2  | INT      |
|PLAYER       | `QUEST_LOG_17_1`                           | 0xF6 | 1  | INT      |
|PLAYER       | `QUEST_LOG_17_2`                           | 0xF7 | 2  | INT      |
|PLAYER       | `QUEST_LOG_18_1`                           | 0xF9 | 1  | INT      |
|PLAYER       | `QUEST_LOG_18_2`                           | 0xFA | 2  | INT      |
|PLAYER       | `QUEST_LOG_19_1`                           | 0xFC | 1  | INT      |
|PLAYER       | `QUEST_LOG_19_2`                           | 0xFD | 2  | INT      |
|PLAYER       | `QUEST_LOG_20_1`                           | 0xFF | 1  | INT      |
|PLAYER       | `QUEST_LOG_20_2`                           | 0x100| 2  | INT      |
|PLAYER       | `VISIBLE_ITEM_1_CREATOR`                   | 0x102| 2  | GUID     |
|PLAYER       | `VISIBLE_ITEM_1_0`                         | 0x104| 8  | INT      |
|PLAYER       | `VISIBLE_ITEM_1_PROPERTIES`                | 0x10C| 1  | TWO_SHORT|
|PLAYER       | `VISIBLE_ITEM_1_PAD`                       | 0x10D| 1  | INT      |
|PLAYER       | `VISIBLE_ITEM_2_CREATOR`                   | 0x10E| 2  | GUID     |
|PLAYER       | `VISIBLE_ITEM_2_0`                         | 0x110| 8  | INT      |
|PLAYER       | `VISIBLE_ITEM_2_PROPERTIES`                | 0x118| 1  | TWO_SHORT|
|PLAYER       | `VISIBLE_ITEM_2_PAD`                       | 0x119| 1  | INT      |
|PLAYER       | `VISIBLE_ITEM_3_CREATOR`                   | 0x11A| 2  | GUID     |
|PLAYER       | `VISIBLE_ITEM_3_0`                         | 0x11C| 8  | INT      |
|PLAYER       | `VISIBLE_ITEM_3_PROPERTIES`                | 0x124| 1  | TWO_SHORT|
|PLAYER       | `VISIBLE_ITEM_3_PAD`                       | 0x125| 1  | INT      |
|PLAYER       | `VISIBLE_ITEM_4_CREATOR`                   | 0x126| 2  | GUID     |
|PLAYER       | `VISIBLE_ITEM_4_0`                         | 0x128| 8  | INT      |
|PLAYER       | `VISIBLE_ITEM_4_PROPERTIES`                | 0x130| 1  | TWO_SHORT|
|PLAYER       | `VISIBLE_ITEM_4_PAD`                       | 0x131| 1  | INT      |
|PLAYER       | `VISIBLE_ITEM_5_CREATOR`                   | 0x132| 2  | GUID     |
|PLAYER       | `VISIBLE_ITEM_5_0`                         | 0x134| 8  | INT      |
|PLAYER       | `VISIBLE_ITEM_5_PROPERTIES`                | 0x13C| 1  | TWO_SHORT|
|PLAYER       | `VISIBLE_ITEM_5_PAD`                       | 0x13D| 1  | INT      |
|PLAYER       | `VISIBLE_ITEM_6_CREATOR`                   | 0x13E| 2  | GUID     |
|PLAYER       | `VISIBLE_ITEM_6_0`                         | 0x140| 8  | INT      |
|PLAYER       | `VISIBLE_ITEM_6_PROPERTIES`                | 0x148| 1  | TWO_SHORT|
|PLAYER       | `VISIBLE_ITEM_6_PAD`                       | 0x149| 1  | INT      |
|PLAYER       | `VISIBLE_ITEM_7_CREATOR`                   | 0x14A| 2  | GUID     |
|PLAYER       | `VISIBLE_ITEM_7_0`                         | 0x14C| 8  | INT      |
|PLAYER       | `VISIBLE_ITEM_7_PROPERTIES`                | 0x154| 1  | TWO_SHORT|
|PLAYER       | `VISIBLE_ITEM_7_PAD`                       | 0x155| 1  | INT      |
|PLAYER       | `VISIBLE_ITEM_8_CREATOR`                   | 0x156| 2  | GUID     |
|PLAYER       | `VISIBLE_ITEM_8_0`                         | 0x158| 8  | INT      |
|PLAYER       | `VISIBLE_ITEM_8_PROPERTIES`                | 0x160| 1  | TWO_SHORT|
|PLAYER       | `VISIBLE_ITEM_8_PAD`                       | 0x161| 1  | INT      |
|PLAYER       | `VISIBLE_ITEM_9_CREATOR`                   | 0x162| 2  | GUID     |
|PLAYER       | `VISIBLE_ITEM_9_0`                         | 0x164| 8  | INT      |
|PLAYER       | `VISIBLE_ITEM_9_PROPERTIES`                | 0x16C| 1  | TWO_SHORT|
|PLAYER       | `VISIBLE_ITEM_9_PAD`                       | 0x16D| 1  | INT      |
|PLAYER       | `VISIBLE_ITEM_10_CREATOR`                  | 0x16E| 2  | GUID     |
|PLAYER       | `VISIBLE_ITEM_10_0`                        | 0x170| 8  | INT      |
|PLAYER       | `VISIBLE_ITEM_10_PROPERTIES`               | 0x178| 1  | TWO_SHORT|
|PLAYER       | `VISIBLE_ITEM_10_PAD`                      | 0x179| 1  | INT      |
|PLAYER       | `VISIBLE_ITEM_11_CREATOR`                  | 0x17A| 2  | GUID     |
|PLAYER       | `VISIBLE_ITEM_11_0`                        | 0x17C| 8  | INT      |
|PLAYER       | `VISIBLE_ITEM_11_PROPERTIES`               | 0x184| 1  | TWO_SHORT|
|PLAYER       | `VISIBLE_ITEM_11_PAD`                      | 0x185| 1  | INT      |
|PLAYER       | `VISIBLE_ITEM_12_CREATOR`                  | 0x186| 2  | GUID     |
|PLAYER       | `VISIBLE_ITEM_12_0`                        | 0x188| 8  | INT      |
|PLAYER       | `VISIBLE_ITEM_12_PROPERTIES`               | 0x190| 1  | TWO_SHORT|
|PLAYER       | `VISIBLE_ITEM_12_PAD`                      | 0x191| 1  | INT      |
|PLAYER       | `VISIBLE_ITEM_13_CREATOR`                  | 0x192| 2  | GUID     |
|PLAYER       | `VISIBLE_ITEM_13_0`                        | 0x194| 8  | INT      |
|PLAYER       | `VISIBLE_ITEM_13_PROPERTIES`               | 0x19C| 1  | TWO_SHORT|
|PLAYER       | `VISIBLE_ITEM_13_PAD`                      | 0x19D| 1  | INT      |
|PLAYER       | `VISIBLE_ITEM_14_CREATOR`                  | 0x19E| 2  | GUID     |
|PLAYER       | `VISIBLE_ITEM_14_0`                        | 0x1A0| 8  | INT      |
|PLAYER       | `VISIBLE_ITEM_14_PROPERTIES`               | 0x1A8| 1  | TWO_SHORT|
|PLAYER       | `VISIBLE_ITEM_14_PAD`                      | 0x1A9| 1  | INT      |
|PLAYER       | `VISIBLE_ITEM_15_CREATOR`                  | 0x1AA| 2  | GUID     |
|PLAYER       | `VISIBLE_ITEM_15_0`                        | 0x1AC| 8  | INT      |
|PLAYER       | `VISIBLE_ITEM_15_PROPERTIES`               | 0x1B4| 1  | TWO_SHORT|
|PLAYER       | `VISIBLE_ITEM_15_PAD`                      | 0x1B5| 1  | INT      |
|PLAYER       | `VISIBLE_ITEM_16_CREATOR`                  | 0x1B6| 2  | GUID     |
|PLAYER       | `VISIBLE_ITEM_16_0`                        | 0x1B8| 8  | INT      |
|PLAYER       | `VISIBLE_ITEM_16_PROPERTIES`               | 0x1C0| 1  | TWO_SHORT|
|PLAYER       | `VISIBLE_ITEM_16_PAD`                      | 0x1C1| 1  | INT      |
|PLAYER       | `VISIBLE_ITEM_17_CREATOR`                  | 0x1C2| 2  | GUID     |
|PLAYER       | `VISIBLE_ITEM_17_0`                        | 0x1C4| 8  | INT      |
|PLAYER       | `VISIBLE_ITEM_17_PROPERTIES`               | 0x1CC| 1  | TWO_SHORT|
|PLAYER       | `VISIBLE_ITEM_17_PAD`                      | 0x1CD| 1  | INT      |
|PLAYER       | `VISIBLE_ITEM_18_CREATOR`                  | 0x1CE| 2  | GUID     |
|PLAYER       | `VISIBLE_ITEM_18_0`                        | 0x1D0| 8  | INT      |
|PLAYER       | `VISIBLE_ITEM_18_PROPERTIES`               | 0x1D8| 1  | TWO_SHORT|
|PLAYER       | `VISIBLE_ITEM_18_PAD`                      | 0x1D9| 1  | INT      |
|PLAYER       | `VISIBLE_ITEM_19_CREATOR`                  | 0x1DA| 2  | GUID     |
|PLAYER       | `VISIBLE_ITEM_19_0`                        | 0x1DC| 8  | INT      |
|PLAYER       | `VISIBLE_ITEM_19_PROPERTIES`               | 0x1E4| 1  | TWO_SHORT|
|PLAYER       | `VISIBLE_ITEM_19_PAD`                      | 0x1E5| 1  | INT      |
|PLAYER       | `FIELD_INV_SLOT_HEAD`                      | 0x1E6| 46 | GUID     |
|PLAYER       | `FIELD_PACK_SLOT_1`                        | 0x214| 32 | GUID     |
|PLAYER       | `FIELD_BANK_SLOT_1`                        | 0x234| 48 | GUID     |
|PLAYER       | `FIELD_BANKBAG_SLOT_1`                     | 0x264| 12 | GUID     |
|PLAYER       | `FIELD_VENDORBUYBACK_SLOT_1`               | 0x270| 24 | GUID     |
|PLAYER       | `FIELD_KEYRING_SLOT_1`                     | 0x288| 64 | GUID     |
|PLAYER       | `FARSIGHT`                                 | 0x2C8| 2  | GUID     |
|PLAYER       | `FIELD_COMBO_TARGET`                       | 0x2CA| 2  | GUID     |
|PLAYER       | `XP`                                       | 0x2CC| 1  | INT      |
|PLAYER       | `NEXT_LEVEL_XP`                            | 0x2CD| 1  | INT      |
|PLAYER       | `SKILL_INFO_1_1`                           | 0x2CE| 384| TWO_SHORT|
|PLAYER       | `CHARACTER_POINTS1`                        | 0x44E| 1  | INT      |
|PLAYER       | `CHARACTER_POINTS2`                        | 0x44F| 1  | INT      |
|PLAYER       | `TRACK_CREATURES`                          | 0x450| 1  | INT      |
|PLAYER       | `TRACK_RESOURCES`                          | 0x451| 1  | INT      |
|PLAYER       | `BLOCK_PERCENTAGE`                         | 0x452| 1  | FLOAT    |
|PLAYER       | `DODGE_PERCENTAGE`                         | 0x453| 1  | FLOAT    |
|PLAYER       | `PARRY_PERCENTAGE`                         | 0x454| 1  | FLOAT    |
|PLAYER       | `CRIT_PERCENTAGE`                          | 0x455| 1  | FLOAT    |
|PLAYER       | `RANGED_CRIT_PERCENTAGE`                   | 0x456| 1  | FLOAT    |
|PLAYER       | `EXPLORED_ZONES_1`                         | 0x457| 64 | BYTES    |
|PLAYER       | `REST_STATE_EXPERIENCE`                    | 0x497| 1  | INT      |
|PLAYER       | `FIELD_COINAGE`                            | 0x498| 1  | INT      |
|PLAYER       | `FIELD_POSSTAT0`                           | 0x499| 1  | INT      |
|PLAYER       | `FIELD_POSSTAT1`                           | 0x49A| 1  | INT      |
|PLAYER       | `FIELD_POSSTAT2`                           | 0x49B| 1  | INT      |
|PLAYER       | `FIELD_POSSTAT3`                           | 0x49C| 1  | INT      |
|PLAYER       | `FIELD_POSSTAT4`                           | 0x49D| 1  | INT      |
|PLAYER       | `FIELD_NEGSTAT0`                           | 0x49E| 1  | INT      |
|PLAYER       | `FIELD_NEGSTAT1`                           | 0x49F| 1  | INT      |
|PLAYER       | `FIELD_NEGSTAT2`                           | 0x4A0| 1  | INT      |
|PLAYER       | `FIELD_NEGSTAT3`                           | 0x4A1| 1  | INT      |
|PLAYER       | `FIELD_NEGSTAT4`                           | 0x4A2| 1  | INT      |
|PLAYER       | `FIELD_RESISTANCEBUFFMODSPOSITIVE`         | 0x4A3| 7  | INT      |
|PLAYER       | `FIELD_RESISTANCEBUFFMODSNEGATIVE`         | 0x4AA| 7  | INT      |
|PLAYER       | `FIELD_MOD_DAMAGE_DONE_POS`                | 0x4B1| 7  | INT      |
|PLAYER       | `FIELD_MOD_DAMAGE_DONE_NEG`                | 0x4B8| 7  | INT      |
|PLAYER       | `FIELD_MOD_DAMAGE_DONE_PCT`                | 0x4BF| 7  | INT      |
|PLAYER       | `FIELD_BYTES`                              | 0x4C6| 1  | BYTES    |
|PLAYER       | `AMMO_ID`                                  | 0x4C7| 1  | INT      |
|PLAYER       | `SELF_RES_SPELL`                           | 0x4C8| 1  | INT      |
|PLAYER       | `FIELD_PVP_MEDALS`                         | 0x4C9| 1  | INT      |
|PLAYER       | `FIELD_BUYBACK_PRICE_1`                    | 0x4CA| 12 | INT      |
|PLAYER       | `FIELD_BUYBACK_TIMESTAMP_1`                | 0x4D6| 12 | INT      |
|PLAYER       | `FIELD_SESSION_KILLS`                      | 0x4E2| 1  | TWO_SHORT|
|PLAYER       | `FIELD_YESTERDAY_KILLS`                    | 0x4E3| 1  | TWO_SHORT|
|PLAYER       | `FIELD_LAST_WEEK_KILLS`                    | 0x4E4| 1  | TWO_SHORT|
|PLAYER       | `FIELD_THIS_WEEK_KILLS`                    | 0x4E5| 1  | TWO_SHORT|
|PLAYER       | `FIELD_THIS_WEEK_CONTRIBUTION`             | 0x4E6| 1  | INT      |
|PLAYER       | `FIELD_LIFETIME_HONORBALE_KILLS`           | 0x4E7| 1  | INT      |
|PLAYER       | `FIELD_LIFETIME_DISHONORBALE_KILLS`        | 0x4E8| 1  | INT      |
|PLAYER       | `FIELD_YESTERDAY_CONTRIBUTION`             | 0x4E9| 1  | INT      |
|PLAYER       | `FIELD_LAST_WEEK_CONTRIBUTION`             | 0x4EA| 1  | INT      |
|PLAYER       | `FIELD_LAST_WEEK_RANK`                     | 0x4EB| 1  | INT      |
|PLAYER       | `FIELD_BYTES2`                             | 0x4EC| 1  | BYTES    |
|PLAYER       | `FIELD_WATCHED_FACTION_INDEX`              | 0x4ED| 1  | INT      |
|PLAYER       | `FIELD_COMBAT_RATING_1`                    | 0x4EE| 20 | INT      |
|GAMEOBJECT   | `CREATED_BY`                         | 0x6  | 2  | GUID     |
|GAMEOBJECT   | `DISPLAYID`                            | 0x8  | 1  | INT      |
|GAMEOBJECT   | `FLAGS`                                | 0x9  | 1  | INT      |
|GAMEOBJECT   | `ROTATION`                             | 0xA  | 4  | FLOAT    |
|GAMEOBJECT   | `STATE`                                | 0xE  | 1  | INT      |
|GAMEOBJECT   | `POS_X`                                | 0xF  | 1  | FLOAT    |
|GAMEOBJECT   | `POS_Y`                                | 0x10 | 1  | FLOAT    |
|GAMEOBJECT   | `POS_Z`                                | 0x11 | 1  | FLOAT    |
|GAMEOBJECT   | `FACING`                               | 0x12 | 1  | FLOAT    |
|GAMEOBJECT   | `DYN_FLAGS`                            | 0x13 | 1  | INT      |
|GAMEOBJECT   | `FACTION`                              | 0x14 | 1  | INT      |
|GAMEOBJECT   | `TYPE_ID`                              | 0x15 | 1  | INT      |
|GAMEOBJECT   | `LEVEL`                                | 0x16 | 1  | INT      |
|GAMEOBJECT   | `ARTKIT`                               | 0x17 | 1  | INT      |
|GAMEOBJECT   | `ANIMPROGRESS`                         | 0x18 | 1  | INT      |
|GAMEOBJECT   | `PADDING`                              | 0x19 | 1  | INT      |
|DYNAMICOBJECT| `CASTER`                            | 0x6  | 2  | GUID     |
|DYNAMICOBJECT| `BYTES`                             | 0x8  | 1  | BYTES    |
|DYNAMICOBJECT| `SPELLID`                           | 0x9  | 1  | INT      |
|DYNAMICOBJECT| `RADIUS`                            | 0xA  | 1  | FLOAT    |
|DYNAMICOBJECT| `POS_X`                             | 0xB  | 1  | FLOAT    |
|DYNAMICOBJECT| `POS_Y`                             | 0xC  | 1  | FLOAT    |
|DYNAMICOBJECT| `POS_Z`                             | 0xD  | 1  | FLOAT    |
|DYNAMICOBJECT| `FACING`                            | 0xE  | 1  | FLOAT    |
|DYNAMICOBJECT| `PAD`                               | 0xF  | 1  | BYTES    |
|CORPSE       | `OWNER`                              | 0x6  | 2  | GUID     |
|CORPSE       | `FACING`                             | 0x8  | 1  | FLOAT    |
|CORPSE       | `POS_X`                              | 0x9  | 1  | FLOAT    |
|CORPSE       | `POS_Y`                              | 0xA  | 1  | FLOAT    |
|CORPSE       | `POS_Z`                              | 0xB  | 1  | FLOAT    |
|CORPSE       | `DISPLAY_ID`                         | 0xC  | 1  | INT      |
|CORPSE       | `ITEM`                               | 0xD  | 19 | INT      |
|CORPSE       | `BYTES_1`                            | 0x20 | 1  | BYTES    |
|CORPSE       | `BYTES_2`                            | 0x21 | 1  | BYTES    |
|CORPSE       | `GUILD`                              | 0x22 | 1  | INT      |
|CORPSE       | `FLAGS`                              | 0x23 | 1  | INT      |
|CORPSE       | `DYNAMIC_FLAGS`                      | 0x24 | 1  | INT      |
|CORPSE       | `PAD`                                | 0x25 | 1  | INT      |
