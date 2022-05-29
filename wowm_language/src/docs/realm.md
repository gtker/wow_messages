# Realm

## Protocol Version 2, Protocol Version 3

### Wowm Representation
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
### Body

| Offset | Size / Endianness | Type | Name | Description | Comment |
| ------ | ----------------- | ---- | ---- | ----------- | ------- |
| 0x00 | ? / - | [RealmType](realmtype.md) | realm_type |  |  |
| - | ? / - | [RealmFlag](realmflag.md) | flag |  |  |
| - | - / - | CString | name |  |  |
| - | - / - | CString | address |  |  |
| - | ? / - | [Population](population.md) | population |  |  |
| - | 1 / - | u8 | number_of_characters_on_realm |  |  |
| - | ? / - | [RealmCategory](realmcategory.md) | category |  |  |
| - | 1 / - | u8 | realm_id |  |  |

# Realm

## Protocol Version 8

### Wowm Representation
```rust,ignore
struct Realm {
    RealmType realm_type;
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
### Body

| Offset | Size / Endianness | Type | Name | Description | Comment |
| ------ | ----------------- | ---- | ---- | ----------- | ------- |
| 0x00 | ? / - | [RealmType](realmtype.md) | realm_type |  | vmangos: this is the second column in Cfg_Configs.dbc |
| - | 1 / - | u8 | locked |  |  |
| - | ? / - | [RealmFlag](realmflag.md) | flag |  |  |
| - | - / - | CString | name |  |  |
| - | - / - | CString | address |  |  |
| - | ? / - | [Population](population.md) | population |  |  |
| - | 1 / - | u8 | number_of_characters_on_realm |  |  |
| - | ? / - | [RealmCategory](realmcategory.md) | category |  |  |
| - | 1 / - | u8 | realm_id |  |  |

If flag contains `SPECIFY_BUILD`:

| Offset | Size / Endianness | Type | Name | Description | Comment |
| ------ | ----------------- | ---- | ---- | ----------- | ------- |
| - | ? / - | [Version](version.md) | version |  |  |

