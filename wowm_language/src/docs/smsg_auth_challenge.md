# SMSG_AUTH_CHALLENGE

## Client Version 1.2, Client Version 1.12

### Description

Seed used by the client to prove in [CMSG_AUTH_SESSION](./cmsg_auth_session.md) that it has authenticated with the auth server.

### Comment

First thing sent when a client connects to the world server.
This message is always unencrypted.

### Wowm Representation
```rust,ignore
smsg SMSG_AUTH_CHALLENGE = 0x01EC {
    u32 server_seed;
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

| Offset | Size / Endianness | Type | Name | Description | Comment |
| ------ | ----------------- | ---- | ---- | ----------- | ------- |
| 0x04 | 4 / Little | u32 | server_seed |  |  |

### Examples
```c
0, 6, // size
236, 1, // opcode (492)
239, 190, 173, 222, // server_seed: u32
```
