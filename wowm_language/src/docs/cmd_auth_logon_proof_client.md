## Protocol Version 2

### Wowm Representation
```rust,ignore
clogin CMD_AUTH_LOGON_PROOF_Client = 0x01 {
    u8[32] client_public_key;
    u8[20] client_proof;
    u8[20] crc_hash;
    u8 number_of_telemetry_keys;
    TelemetryKey[number_of_telemetry_keys] telemetry_keys;
}
```
### Header

Login messages have a header of 1 byte with an opcode. Some messages also have a size field but this is not considered part of the header.

#### Login Header

| Offset | Size / Endianness | Type   | Name   | Description |
| ------ | ----------------- | ------ | ------ | ----------- |
| 0x00   | 1 / -             | uint8  | opcode | Opcode that determines which fields the message contains.|

### Body

| Offset | Size / Endianness | Type | Name | Description | Comment |
| ------ | ----------------- | ---- | ---- | ----------- | ------- |
| 0x01 | ? / - | u8[32] | client_public_key |  |  |
| - | ? / - | u8[20] | client_proof |  |  |
| - | ? / - | u8[20] | crc_hash |  |  |
| - | 1 / - | u8 | number_of_telemetry_keys |  |  |
| - | ? / - | [TelemetryKey](telemetrykey.md)[number_of_telemetry_keys] | telemetry_keys |  |  |

### Examples
```c
1, // opcode (1)
241, 62, 229, 209, 131, 196, 200, 169, 80, 14, 63, 90, 93, 138, 238, 78, 46, 69, 
225, 247, 204, 143, 28, 245, 238, 142, 17, 206, 211, 29, 215, 8, // client_public_key: u8[32]
107, 30, 72, 27, 77, 4, 161, 24, 216, 242, 222, 92, 89, 213, 92, 129, 46, 101, 236, 
62, // client_proof: u8[20]
78, 245, 45, 225, 128, 94, 26, 103, 21, 236, 200, 65, 238, 184, 144, 138, 88, 187, 
0, 208, // crc_hash: u8[20]
2, // number_of_telemetry_keys: u8
255, 0, // [0].TelemetryKey.unknown1: u16
239, 190, 173, 222, // [1].TelemetryKey.unknown2: u32
1, 2, 3, 4, // [2].TelemetryKey.unknown3: u8[4]
0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, // [3].TelemetryKey.unknown4: u8[20]
254, 0, // [0].TelemetryKey.unknown1: u16
238, 190, 173, 222, // [1].TelemetryKey.unknown2: u32
0, 1, 2, 3, // [2].TelemetryKey.unknown3: u8[4]
1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, // [3].TelemetryKey.unknown4: u8[20]
// telemetry_keys: TelemetryKey[number_of_telemetry_keys]
```
```c
1, // opcode (1)
241, 62, 229, 209, 131, 196, 200, 169, 80, 14, 63, 90, 93, 138, 238, 78, 46, 69, 
225, 247, 204, 143, 28, 245, 238, 142, 17, 206, 211, 29, 215, 8, // client_public_key: u8[32]
107, 30, 72, 27, 77, 4, 161, 24, 216, 242, 222, 92, 89, 213, 92, 129, 46, 101, 236, 
62, // client_proof: u8[20]
78, 245, 45, 225, 128, 94, 26, 103, 21, 236, 200, 65, 238, 184, 144, 138, 88, 187, 
0, 208, // crc_hash: u8[20]
1, // number_of_telemetry_keys: u8
255, 0, // [0].TelemetryKey.unknown1: u16
239, 190, 173, 222, // [1].TelemetryKey.unknown2: u32
1, 2, 3, 4, // [2].TelemetryKey.unknown3: u8[4]
0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, // [3].TelemetryKey.unknown4: u8[20]
// telemetry_keys: TelemetryKey[number_of_telemetry_keys]
```
```c
1, // opcode (1)
241, 62, 229, 209, 131, 196, 200, 169, 80, 14, 63, 90, 93, 138, 238, 78, 46, 69, 
225, 247, 204, 143, 28, 245, 238, 142, 17, 206, 211, 29, 215, 8, // client_public_key: u8[32]
107, 30, 72, 27, 77, 4, 161, 24, 216, 242, 222, 92, 89, 213, 92, 129, 46, 101, 236, 
62, // client_proof: u8[20]
78, 245, 45, 225, 128, 94, 26, 103, 21, 236, 200, 65, 238, 184, 144, 138, 88, 187, 
0, 208, // crc_hash: u8[20]
0, // number_of_telemetry_keys: u8
// telemetry_keys: TelemetryKey[number_of_telemetry_keys]
```
## Protocol Version 3

### Wowm Representation
```rust,ignore
clogin CMD_AUTH_LOGON_PROOF_Client = 0x01 {
    u8[32] client_public_key;
    u8[20] client_proof;
    u8[20] crc_hash;
    u8 number_of_telemetry_keys;
    TelemetryKey[number_of_telemetry_keys] telemetry_keys;
    SecurityFlag security_flag;
    if (security_flag == PIN) {
        u8[16] pin_salt;
        u8[20] pin_hash;
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

| Offset | Size / Endianness | Type | Name | Description | Comment |
| ------ | ----------------- | ---- | ---- | ----------- | ------- |
| 0x01 | ? / - | u8[32] | client_public_key |  |  |
| - | ? / - | u8[20] | client_proof |  |  |
| - | ? / - | u8[20] | crc_hash |  |  |
| - | 1 / - | u8 | number_of_telemetry_keys |  |  |
| - | ? / - | [TelemetryKey](telemetrykey.md)[number_of_telemetry_keys] | telemetry_keys |  |  |
| - | ? / - | [SecurityFlag](securityflag.md) | security_flag |  |  |

If security_flag is equal to `PIN`:

| Offset | Size / Endianness | Type | Name | Description | Comment |
| ------ | ----------------- | ---- | ---- | ----------- | ------- |
| - | ? / - | u8[16] | pin_salt |  |  |
| - | ? / - | u8[20] | pin_hash |  |  |

### Examples
```c
1, // opcode (1)
241, 62, 229, 209, 131, 196, 200, 169, 80, 14, 63, 90, 93, 138, 238, 78, 46, 69, 
225, 247, 204, 143, 28, 245, 238, 142, 17, 206, 211, 29, 215, 8, // client_public_key: u8[32]
107, 30, 72, 27, 77, 4, 161, 24, 216, 242, 222, 92, 89, 213, 92, 129, 46, 101, 236, 
62, // client_proof: u8[20]
78, 245, 45, 225, 128, 94, 26, 103, 21, 236, 200, 65, 238, 184, 144, 138, 88, 187, 
0, 208, // crc_hash: u8[20]
2, // number_of_telemetry_keys: u8
255, 0, // [0].TelemetryKey.unknown1: u16
239, 190, 173, 222, // [1].TelemetryKey.unknown2: u32
1, 2, 3, 4, // [2].TelemetryKey.unknown3: u8[4]
0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, // [3].TelemetryKey.unknown4: u8[20]
254, 0, // [0].TelemetryKey.unknown1: u16
238, 190, 173, 222, // [1].TelemetryKey.unknown2: u32
0, 1, 2, 3, // [2].TelemetryKey.unknown3: u8[4]
1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, // [3].TelemetryKey.unknown4: u8[20]
// telemetry_keys: TelemetryKey[number_of_telemetry_keys]
0, // security_flag: SecurityFlag NONE (0x0)
```
```c
1, // opcode (1)
241, 62, 229, 209, 131, 196, 200, 169, 80, 14, 63, 90, 93, 138, 238, 78, 46, 69, 
225, 247, 204, 143, 28, 245, 238, 142, 17, 206, 211, 29, 215, 8, // client_public_key: u8[32]
107, 30, 72, 27, 77, 4, 161, 24, 216, 242, 222, 92, 89, 213, 92, 129, 46, 101, 236, 
62, // client_proof: u8[20]
78, 245, 45, 225, 128, 94, 26, 103, 21, 236, 200, 65, 238, 184, 144, 138, 88, 187, 
0, 208, // crc_hash: u8[20]
1, // number_of_telemetry_keys: u8
255, 0, // [0].TelemetryKey.unknown1: u16
239, 190, 173, 222, // [1].TelemetryKey.unknown2: u32
1, 2, 3, 4, // [2].TelemetryKey.unknown3: u8[4]
0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, // [3].TelemetryKey.unknown4: u8[20]
// telemetry_keys: TelemetryKey[number_of_telemetry_keys]
0, // security_flag: SecurityFlag NONE (0x0)
```
```c
1, // opcode (1)
241, 62, 229, 209, 131, 196, 200, 169, 80, 14, 63, 90, 93, 138, 238, 78, 46, 69, 
225, 247, 204, 143, 28, 245, 238, 142, 17, 206, 211, 29, 215, 8, // client_public_key: u8[32]
107, 30, 72, 27, 77, 4, 161, 24, 216, 242, 222, 92, 89, 213, 92, 129, 46, 101, 236, 
62, // client_proof: u8[20]
78, 245, 45, 225, 128, 94, 26, 103, 21, 236, 200, 65, 238, 184, 144, 138, 88, 187, 
0, 208, // crc_hash: u8[20]
0, // number_of_telemetry_keys: u8
// telemetry_keys: TelemetryKey[number_of_telemetry_keys]
0, // security_flag: SecurityFlag NONE (0x0)
```
```c
1, // opcode (1)
241, 62, 229, 209, 131, 196, 200, 169, 80, 14, 63, 90, 93, 138, 238, 78, 46, 69, 
225, 247, 204, 143, 28, 245, 238, 142, 17, 206, 211, 29, 215, 8, // client_public_key: u8[32]
107, 30, 72, 27, 77, 4, 161, 24, 216, 242, 222, 92, 89, 213, 92, 129, 46, 101, 236, 
62, // client_proof: u8[20]
78, 245, 45, 225, 128, 94, 26, 103, 21, 236, 200, 65, 238, 184, 144, 138, 88, 187, 
0, 208, // crc_hash: u8[20]
0, // number_of_telemetry_keys: u8
// telemetry_keys: TelemetryKey[number_of_telemetry_keys]
1, // security_flag: SecurityFlag PIN (0x1)
0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, // pin_salt: u8[16]
0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, // pin_hash: u8[20]
```
## Protocol Version 8

### Wowm Representation
```rust,ignore
clogin CMD_AUTH_LOGON_PROOF_Client = 0x01 {
    u8[32] client_public_key;
    u8[20] client_proof;
    u8[20] crc_hash;
    u8 number_of_telemetry_keys;
    TelemetryKey[number_of_telemetry_keys] telemetry_keys;
    SecurityFlag security_flag;
    if (security_flag & PIN) {
        u8[16] pin_salt;
        u8[20] pin_hash;
    }
    if (security_flag & UNKNOWN0) {
        u8 unknown0;
        u8 unknown1;
        u8 unknown2;
        u8 unknown3;
        u64 unknown4;
    }
    if (security_flag & AUTHENTICATOR) {
        u8 unknown5;
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

| Offset | Size / Endianness | Type | Name | Description | Comment |
| ------ | ----------------- | ---- | ---- | ----------- | ------- |
| 0x01 | ? / - | u8[32] | client_public_key |  |  |
| - | ? / - | u8[20] | client_proof |  |  |
| - | ? / - | u8[20] | crc_hash |  |  |
| - | 1 / - | u8 | number_of_telemetry_keys |  |  |
| - | ? / - | [TelemetryKey](telemetrykey.md)[number_of_telemetry_keys] | telemetry_keys |  |  |
| - | ? / - | [SecurityFlag](securityflag.md) | security_flag |  |  |

If security_flag contains `PIN`:

| Offset | Size / Endianness | Type | Name | Description | Comment |
| ------ | ----------------- | ---- | ---- | ----------- | ------- |
| - | ? / - | u8[16] | pin_salt |  |  |
| - | ? / - | u8[20] | pin_hash |  |  |

If security_flag contains `UNKNOWN0`:

| Offset | Size / Endianness | Type | Name | Description | Comment |
| ------ | ----------------- | ---- | ---- | ----------- | ------- |
| - | 1 / - | u8 | unknown0 |  |  |
| - | 1 / - | u8 | unknown1 |  |  |
| - | 1 / - | u8 | unknown2 |  |  |
| - | 1 / - | u8 | unknown3 |  |  |
| - | 8 / Little | u64 | unknown4 |  |  |

If security_flag contains `AUTHENTICATOR`:

| Offset | Size / Endianness | Type | Name | Description | Comment |
| ------ | ----------------- | ---- | ---- | ----------- | ------- |
| - | 1 / - | u8 | unknown5 |  |  |

### Examples
```c
1, // opcode (1)
241, 62, 229, 209, 131, 196, 200, 169, 80, 14, 63, 90, 93, 138, 238, 78, 46, 69, 
225, 247, 204, 143, 28, 245, 238, 142, 17, 206, 211, 29, 215, 8, // client_public_key: u8[32]
107, 30, 72, 27, 77, 4, 161, 24, 216, 242, 222, 92, 89, 213, 92, 129, 46, 101, 236, 
62, // client_proof: u8[20]
78, 245, 45, 225, 128, 94, 26, 103, 21, 236, 200, 65, 238, 184, 144, 138, 88, 187, 
0, 208, // crc_hash: u8[20]
2, // number_of_telemetry_keys: u8
255, 0, // [0].TelemetryKey.unknown1: u16
239, 190, 173, 222, // [1].TelemetryKey.unknown2: u32
1, 2, 3, 4, // [2].TelemetryKey.unknown3: u8[4]
0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, // [3].TelemetryKey.unknown4: u8[20]
254, 0, // [0].TelemetryKey.unknown1: u16
238, 190, 173, 222, // [1].TelemetryKey.unknown2: u32
0, 1, 2, 3, // [2].TelemetryKey.unknown3: u8[4]
1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, // [3].TelemetryKey.unknown4: u8[20]
// telemetry_keys: TelemetryKey[number_of_telemetry_keys]
0, // security_flag: SecurityFlag  NONE (0)
```
```c
1, // opcode (1)
241, 62, 229, 209, 131, 196, 200, 169, 80, 14, 63, 90, 93, 138, 238, 78, 46, 69, 
225, 247, 204, 143, 28, 245, 238, 142, 17, 206, 211, 29, 215, 8, // client_public_key: u8[32]
107, 30, 72, 27, 77, 4, 161, 24, 216, 242, 222, 92, 89, 213, 92, 129, 46, 101, 236, 
62, // client_proof: u8[20]
78, 245, 45, 225, 128, 94, 26, 103, 21, 236, 200, 65, 238, 184, 144, 138, 88, 187, 
0, 208, // crc_hash: u8[20]
1, // number_of_telemetry_keys: u8
255, 0, // [0].TelemetryKey.unknown1: u16
239, 190, 173, 222, // [1].TelemetryKey.unknown2: u32
1, 2, 3, 4, // [2].TelemetryKey.unknown3: u8[4]
0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, // [3].TelemetryKey.unknown4: u8[20]
// telemetry_keys: TelemetryKey[number_of_telemetry_keys]
0, // security_flag: SecurityFlag  NONE (0)
```
```c
1, // opcode (1)
241, 62, 229, 209, 131, 196, 200, 169, 80, 14, 63, 90, 93, 138, 238, 78, 46, 69, 
225, 247, 204, 143, 28, 245, 238, 142, 17, 206, 211, 29, 215, 8, // client_public_key: u8[32]
107, 30, 72, 27, 77, 4, 161, 24, 216, 242, 222, 92, 89, 213, 92, 129, 46, 101, 236, 
62, // client_proof: u8[20]
78, 245, 45, 225, 128, 94, 26, 103, 21, 236, 200, 65, 238, 184, 144, 138, 88, 187, 
0, 208, // crc_hash: u8[20]
0, // number_of_telemetry_keys: u8
// telemetry_keys: TelemetryKey[number_of_telemetry_keys]
0, // security_flag: SecurityFlag  NONE (0)
```
```c
1, // opcode (1)
241, 62, 229, 209, 131, 196, 200, 169, 80, 14, 63, 90, 93, 138, 238, 78, 46, 69, 
225, 247, 204, 143, 28, 245, 238, 142, 17, 206, 211, 29, 215, 8, // client_public_key: u8[32]
107, 30, 72, 27, 77, 4, 161, 24, 216, 242, 222, 92, 89, 213, 92, 129, 46, 101, 236, 
62, // client_proof: u8[20]
78, 245, 45, 225, 128, 94, 26, 103, 21, 236, 200, 65, 238, 184, 144, 138, 88, 187, 
0, 208, // crc_hash: u8[20]
0, // number_of_telemetry_keys: u8
// telemetry_keys: TelemetryKey[number_of_telemetry_keys]
1, // security_flag: SecurityFlag  PIN (1)
0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, // pin_salt: u8[16]
0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, // pin_hash: u8[20]
```
```c
1, // opcode (1)
241, 62, 229, 209, 131, 196, 200, 169, 80, 14, 63, 90, 93, 138, 238, 78, 46, 69, 
225, 247, 204, 143, 28, 245, 238, 142, 17, 206, 211, 29, 215, 8, // client_public_key: u8[32]
107, 30, 72, 27, 77, 4, 161, 24, 216, 242, 222, 92, 89, 213, 92, 129, 46, 101, 236, 
62, // client_proof: u8[20]
78, 245, 45, 225, 128, 94, 26, 103, 21, 236, 200, 65, 238, 184, 144, 138, 88, 187, 
0, 208, // crc_hash: u8[20]
0, // number_of_telemetry_keys: u8
// telemetry_keys: TelemetryKey[number_of_telemetry_keys]
3, // security_flag: SecurityFlag  PIN| UNKNOWN0 (3)
0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, // pin_salt: u8[16]
0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, // pin_hash: u8[20]
16, // unknown0: u8
32, // unknown1: u8
48, // unknown2: u8
64, // unknown3: u8
239, 190, 173, 222, 0, 0, 0, 0, // unknown4: u64
```
```c
1, // opcode (1)
241, 62, 229, 209, 131, 196, 200, 169, 80, 14, 63, 90, 93, 138, 238, 78, 46, 69, 
225, 247, 204, 143, 28, 245, 238, 142, 17, 206, 211, 29, 215, 8, // client_public_key: u8[32]
107, 30, 72, 27, 77, 4, 161, 24, 216, 242, 222, 92, 89, 213, 92, 129, 46, 101, 236, 
62, // client_proof: u8[20]
78, 245, 45, 225, 128, 94, 26, 103, 21, 236, 200, 65, 238, 184, 144, 138, 88, 187, 
0, 208, // crc_hash: u8[20]
0, // number_of_telemetry_keys: u8
// telemetry_keys: TelemetryKey[number_of_telemetry_keys]
7, // security_flag: SecurityFlag  PIN| UNKNOWN0| AUTHENTICATOR (7)
0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, // pin_salt: u8[16]
0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, // pin_hash: u8[20]
16, // unknown0: u8
32, // unknown1: u8
48, // unknown2: u8
64, // unknown3: u8
239, 190, 173, 222, 0, 0, 0, 0, // unknown4: u64
1, // unknown5: u8
```
