## Client Version 1.12

```rust,ignore
smsg SMSG_WEATHER = 0x2F4 {
    WeatherType weather_type;    
    f32 grade;    
    u32 sound_id;    
    WeatherChangeType change;    
}

```
