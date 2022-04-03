# `SMSG_AUTH_SESSION`

# 1.12

```rust,ignore
smsg SMSG_AUTH_SESSION {
    u8 amount_of_things;
    u16 b_u16;
    Thing[amount_of_things] things;
}
```

## Packet Layout

| Offset | Size/Endianness |    Type    | Name               | Description | Comments |
| ------ | --------------- | ---------- | ---------- | ------------------ | -------- |
| 0x0    |  1 / -          | uint8_t    | `amount_of_things` | | |
| 0x1    |  2 / Little     | uint16_t   | `b_u16`              |  |  |
| 0x3    |  0 - 255 / -    | [Thing](thing.md)[amount_of_things] | `things` |  |  |

**if `t == OFFLINE`**

| Offset | Size/Endianness |    Type    | Name               | Description | Comments |
| ------ | --------------- | ---------- | ---------- | ------------------ | -------- |
| -    |  1 / -          | uint8_t    | `amount_of_things` | | |

else

| Offset | Size/Endianness |    Type    | Name               | Description | Comments |
| ------ | --------------- | ---------- | ---------- | ------------------ | -------- |
| -    |  1 / -          | uint8_t    | `amount_of_things` | | |

## Size

Minimum size is **3** bytes:
```rust,ignore
1 // amount_of_things: uint8_t
+ 2 // b_u16: uint16_t
+ 0 * 1 // Thing[amount_of_things]
```

Maximum size is **258** bytes:
```rust,ignore
1 // amount_of_things: uint8_t
+ 2 // b_u16: uint16_t
+ 255 * 1 // Thing[amount_of_things]
```

## Example

```c,ignore
0x01, 0x00, // size: u16 (0x0001)
0x13, 0x00, // opcode: u16 (0x0013)
0x01, // amount_of_things: u8 (0x01)
0x00, 0x01, // b_u16: u16 (0x0100)

```
