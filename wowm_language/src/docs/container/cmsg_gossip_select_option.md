## Client Version 1.12

### Wowm Representation
```rust,ignore
cmsg CMSG_GOSSIP_SELECT_OPTION = 0x017C {
    Guid guid;    
    u32 gossip_list_id;    
    optional unknown {    
        CString code;        
    }    
}

```
### Header
CMSG have a header of 6 bytes.

#### CMSG Header
| Offset | Size / Endianness | Type   | Name   | Description |
| ------ | ----------------- | ------ | ------ | ----------- |
| 0x00   | 2 / Big           | uint16 | size   | Size of the rest of the message including the opcode field but not including the size field.|
| 0x02   | 4 / Little        | uint32 | opcode | Opcode that determines which fields the message contains.|
