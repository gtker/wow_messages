# SMSG_WEATHER

## Client Version 1.12

### Wowm Representation
```rust,ignore
smsg SMSG_WEATHER = 0x02F4 {
    WeatherType weather_type;
    f32 grade;
    u32 sound_id;
    WeatherChangeType change;
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
| 0x04 | ? / - | [WeatherType](weathertype.md) | weather_type |  |  |
| - | 4 / Little | f32 | grade |  |  |
| - | 4 / Little | u32 | sound_id |  |  |
| - | ? / - | [WeatherChangeType](weatherchangetype.md) | change |  |  |

