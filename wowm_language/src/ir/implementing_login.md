# Implementing login messages

## Message Layout

## Design of types

### CMD_AUTH_LOGON_CHALLENGE_Client

* Automatically sent by client
* Has enums, with and without self.value
* Has String
* Has size
* Has `IpAddress` alias
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
