# SMSG_COMPRESSED_MOVES

## Client Version 1.12

### Wowm Representation

Autogenerated from `wowm` file at [`wow_message_parser/wowm/world/movement/smsg/smsg_compressed_moves.wowm:48`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/movement/smsg/smsg_compressed_moves.wowm#L48).
```rust,ignore
smsg SMSG_COMPRESSED_MOVES = 0x02FB {
    CompressedMove[-] moves;
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

| Offset | Size / Endianness | Type | Name | Comment |
| ------ | ----------------- | ---- | ---- | ------- |
| 0x04 | ? / - | [CompressedMove](compressedmove.md)[-] | moves |  |

### Examples

#### Example 1

```c
0, 50, // size
251, 2, // opcode (763)
46, 0, 0, 0, // uncompressed size
120, 1, 211, 189, 203, 192, 40, 145, 183, 154, 251, 216, 186, 88, 230, 195, 43, 
212, 151, 59, 49, 32, 3, 70, 32, 167, 100, 57, 247, 177, 245, 239, 95, 29, 58, 121, 
102, 137, 19, 0, 38, 30, 14, 73, // compressed data
```
#### Example 2

```c
0, 70, // size
251, 2, // opcode (763)
59, 0, 0, 0, // uncompressed size
120, 1, 179, 186, 203, 112, 59, 95, 198, 65, 220, 224, 99, 245, 27, 177, 35, 215, 
23, 55, 31, 109, 80, 148, 113, 210, 87, 100, 0, 2, 70, 6, 134, 99, 28, 12, 12, 204, 
64, 102, 235, 107, 177, 35, 92, 229, 205, 71, 21, 183, 203, 58, 49, 20, 0, 5, 12, 
24, 24, 0, 88, 227, 17, 4, // compressed data
```
## Client Version 3.3.5

### Wowm Representation

Autogenerated from `wowm` file at [`wow_message_parser/wowm/world/movement/smsg/smsg_multiple_moves.wowm:27`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/movement/smsg/smsg_multiple_moves.wowm#L27).
```rust,ignore
smsg SMSG_COMPRESSED_MOVES = 0x02FB {
    u32 size = self.size;
    MiniMoveMessage[-] moves;
}
```
### Header

SMSG have a header of 4 bytes.

#### SMSG Header

| Offset | Size / Endianness | Type   | Name   | Description |
| ------ | ----------------- | ------ | ------ | ----------- |
| 0x00   | 2 **OR** 3 / Big           | uint16 **OR** uint16+uint8 | size | Size of the rest of the message including the opcode field but not including the size field. Wrath server messages **can** be 3 bytes. If the first (most significant) size byte has `0x80` set, the header will be 3 bytes, otherwise it is 2.|
| -      | 2 / Little| uint16 | opcode | Opcode that determines which fields the message contains. |

### Body

| Offset | Size / Endianness | Type | Name | Comment |
| ------ | ----------------- | ---- | ---- | ------- |
| - | 4 / Little | u32 | size |  |
| - | ? / - | [MiniMoveMessage](minimovemessage.md)[-] | moves |  |

