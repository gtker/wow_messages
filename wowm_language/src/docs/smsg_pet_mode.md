# SMSG_PET_MODE
## Client Version 1.12

### Wowm Representation
```rust,ignore
smsg SMSG_PET_MODE = 0x017A {
    Guid guid;
    PetReactState react_state;
    PetCommandState command_state;
    u8 unknown1;
    u8 pet_enabled;
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
| 0x04 | 8 / Little | [Guid](../spec/packed-guid.md) | guid |  |
| 0x0C | ? / - | [PetReactState](petreactstate.md) | react_state |  |
| - | ? / - | [PetCommandState](petcommandstate.md) | command_state |  |
| - | 1 / - | u8 | unknown1 |  |
| - | 1 / - | u8 | pet_enabled |  |
