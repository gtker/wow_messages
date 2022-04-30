## Client Version 1.12

### Comment

vmangos: this opcode can be used in two ways: Either set explicit new status or toggle old status

### Wowm Representation
```rust,ignore
cmsg CMSG_TOGGLE_PVP = 0x0253 {
    optional set {    
        u8 enable_pvp;        
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
### Body

Optionally the following fields can be present. This can only be detected by looking at the size of the message.

| Offset | Size / Endianness | Type | Name | Description |
| ------ | ----------------- | ---- | ---- | ----------- |
| 0x06 | 1 / - | u8 | enable_pvp |  |
