## Client Version 1.12

### Wowm Representation
```rust,ignore
enum MailType : u8 {
    NORMAL = 0;
    AUCTION = 2;
    CREATURE = 3;
    GAMEOBJECT = 4;
    ITEM = 5;
}
```
### Type
The basic type is `u8`, a 1 byte (8 bit) integer.
### Enumerators
| Enumerator | Original  | Description | Comment |
| --------- | -------- | ----------- | ------- |
| `NORMAL` | 0 (0x00) |  |  |
| `AUCTION` | 2 (0x02) |  |  |
| `CREATURE` | 3 (0x03) |  | client send CMSG_CREATURE_QUERY on this mailmessagetype |
| `GAMEOBJECT` | 4 (0x04) |  | client send CMSG_GAMEOBJECT_QUERY on this mailmessagetype |
| `ITEM` | 5 (0x05) |  | client send CMSG_ITEM_QUERY on this mailmessagetype |
