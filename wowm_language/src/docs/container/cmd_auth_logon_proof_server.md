## Protocol Version 2, Protocol Version 3

```rust,ignore
slogin CMD_AUTH_LOGON_PROOF_Server = 0x1 {
    LoginResult login_result;    
    if (login_result == SUCCESS) {        
        u8[20] server_proof;        
        u32 hardware_survey_id;        
    }    
}

```
## Protocol Version 8

```rust,ignore
slogin CMD_AUTH_LOGON_PROOF_Server = 0x1 {
    LoginResult login_result;    
    if (login_result == SUCCESS) {        
        u8[20] server_proof;        
        AccountFlag account_flag;        
        u32 hardware_survey_id;        
        u16 unknown_flags;        
    }    
    else {    
        u16 padding = 0;        
    }    
}

```
