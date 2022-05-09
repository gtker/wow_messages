## Protocol Version *

### Wowm Representation
```rust,ignore
clogin TestStruct = 0xFF {
    TestFlag f;
    if (f & A) {
        u8 b_A1;
        InnerFlag i;
        if (i & H) {
            u8 b_H1;
        }
        else if (i & I) {
            u8 b_I1;
            ThirdFlag t;
            if (t & W) {
                u8 b_W1;
            }
            if (t & X) {
                u8 b_X1;
            }
            else if (t & Z) {
                u8 b_Z1;
            }
        }
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
| - | ? / - | [InnerFlag](innerflag.md) | i |  |

If i contains `H`:

| Offset | Size / Endianness | Type | Name | Description |
| ------ | ----------------- | ---- | ---- | ----------- |
| - | 1 / - | u8 | b_H1 |  |

Else If i contains `I`:

| Offset | Size / Endianness | Type | Name | Description |
| ------ | ----------------- | ---- | ---- | ----------- |
| - | 1 / - | u8 | b_I1 |  |
| - | ? / - | [ThirdFlag](thirdflag.md) | t |  |

If t contains `W`:

| Offset | Size / Endianness | Type | Name | Description |
| ------ | ----------------- | ---- | ---- | ----------- |
| - | 1 / - | u8 | b_W1 |  |

If t contains `X`:

| Offset | Size / Endianness | Type | Name | Description |
| ------ | ----------------- | ---- | ---- | ----------- |
| - | 1 / - | u8 | b_X1 |  |

Else If t contains `Z`:

| Offset | Size / Endianness | Type | Name | Description |
| ------ | ----------------- | ---- | ---- | ----------- |
| - | 1 / - | u8 | b_Z1 |  |

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
13, 0, // f: TestFlag  A| C| D (13)
1, // b_A1: u8
2, // i: InnerFlag  I (2)
3, // b_I1: u8
2, // t: ThirdFlag  Y (2)
2, // b_C1: u8
```
```c
255, // opcode (255)
12, 0, // f: TestFlag  C| D (12)
2, // b_C1: u8
```
