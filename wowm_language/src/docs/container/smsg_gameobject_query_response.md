## Client Version 1.12

### Wowm Representation
```rust,ignore
smsg SMSG_GAMEOBJECT_QUERY_RESPONSE = 0x005F {
    u32 entry_id;    
    optional found {    
        u32 info_type;        
        u32 display_id;        
        CString name1;        
        CString name2;        
        CString name3;        
        CString name4;        
        CString name5;        
        u32[6] raw_data;        
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
