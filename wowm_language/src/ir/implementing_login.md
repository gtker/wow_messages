# Implementing login messages

The login message data can be found in
the [`intermediate_representation.json`](https://github.com/gtker/wow_messages/blob/main/intermediate_representation.json)
file.
The [`intermediate_representation_schema.json`](https://github.com/gtker/wow_messages/blob/main/intermediate_representation_schema.json)
contains a [JSON type def](https://jsontypedef.com/) schema for the `intermediate_representation.json`.
The [`json-typedef-codegen`](https://github.com/jsontypedef/json-typedef-codegen) program can be used to generate
bindings for many popular languages.

The [`wow_messages_python` repository](https://github.com/gtker/wow_messages_python) contains the Python code used to
generate the Python message library.
It can be used as inspiration for your own implementation.
The [`generator` directory](https://github.com/gtker/wow_messages_python/tree/main/generator) contains the actual code
generation,
and the [`wow_login_messages` directory](https://github.com/gtker/wow_messages_python/tree/main/wow_login_messages)
contains the generated library.

## Message Layout

All interactions start with the client sending a message, then reading the reply from the server.

All login messages start with an [opcode field](https://en.wikipedia.org/wiki/Opcode)
field that specifies the message contents.
This is the only thing the messages have in common.

The only way to know how much data to expect in a message is by a combination of the protocol version sent
in the very first message by the
client, [`CMD_AUTH_LOGON_CHALLENGE_Client`](https://gtker.com/wow_messages/docs/cmd_auth_logon_challenge_client.html),
and the opcode.

## Design of types

### CMD_AUTH_LOGON_CHALLENGE_Client

* Automatically sent by client
* Has enums.
* Has String
* Has size
* Has `IpAddress` alias
* Has `Population` alias
* Requires a read for servers, write for clients

### CMD_AUTH_LOGON_CHALLENGE_Server

* Constant value
* Fields hidden behind if statement
* Same opcode as Client
* Requires a write for servers, read for clients

### CMD_AUTH_LOGON_PROOF_Client

* Array of structs

### CMD_REALM_LIST_Client

* Not empty, but has padding
* Will be empty if padding is ignored

### CMD_XFER_ACCEPT

* Empty body

### CMD_REALM_LIST_Server

* Array of more complex structs

### Realm

* CString
* Version 8 has if flag
