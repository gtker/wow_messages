## Client Version 1.12

### Wowm Representation
```rust,ignore
smsg MSG_CORPSE_QUERY_Server = 0x0216 {
    CorpseQueryResult result;    
    if (result == FOUND) {        
        Map map;        
        f32 position_x;        
        f32 position_y;        
        f32 position_z;        
        Map corpse_map;        
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
| Offset | Size / Endianness | Type | Name | Description |
| ------ | ----------------- | ---- | ---- | ----------- |
| 0x04 | ? / - | CorpseQueryResult | result |  |

If result is equal to `FOUND`:

| Offset | Size / Endianness | Type | Name | Description |
| ------ | ----------------- | ---- | ---- | ----------- |
| - | ? / - | Map | map |  |
| - | 4 / Little | f32 | position_x |  |
| - | 4 / Little | f32 | position_y |  |
| - | 4 / Little | f32 | position_z |  |
| - | ? / - | Map | corpse_map |  |
