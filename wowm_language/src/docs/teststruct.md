## Protocol Version *

### Wowm Representation
```rust,ignore
clogin TestStruct = 0xFF {
    TestFlag f;
    if (f & A) {
        u8 b_A1;
    }
    else if (f & B) {
        u8 b_B1;
    }
    if (f & C) {
        u8 b_C1;
    }
    if (f & E) {
        u8 b_E1;
    }
}
```
### Header
Login messages have a header of 1 byte with an opcode. Some messages also have a size field but this is not considered part of the header.

#### Login Header
| Offset | Size / Endianness | Type   | Name   | Description |
| ------ | ----------------- | ------ | ------ | ----------- |
| 0x00   | 1 / -             | uint8  | opcode | Opcode that determines which fields the message contains.|
### Body
| Offset | Size / Endianness | Type | Name | Description |
| ------ | ----------------- | ---- | ---- | ----------- |
| 0x01 | ? / - | [TestFlag](testflag.md) | f |  |

If f contains `A`:

| Offset | Size / Endianness | Type | Name | Description |
| ------ | ----------------- | ---- | ---- | ----------- |
| - | 1 / - | u8 | b_A1 |  |

Else If f contains `B`:

| Offset | Size / Endianness | Type | Name | Description |
| ------ | ----------------- | ---- | ---- | ----------- |
| - | 1 / - | u8 | b_B1 |  |

If f contains `C`:

| Offset | Size / Endianness | Type | Name | Description |
| ------ | ----------------- | ---- | ---- | ----------- |
| - | 1 / - | u8 | b_C1 |  |

If f contains `E`:

| Offset | Size / Endianness | Type | Name | Description |
| ------ | ----------------- | ---- | ---- | ----------- |
| - | 1 / - | u8 | b_E1 |  |
### Examples
```c
255, // opcode (255)
14, 0, // f: TestFlag  B| C| D (14)
1, // b_B1: u8
2, // b_C1: u8
```
```c
255, // opcode (255)
13, 0, // f: TestFlag  A| C| D (13)
1, // b_A1: u8
2, // b_C1: u8
```
