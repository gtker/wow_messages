## Protocol Version 2

## Wowm Representation
```rust,ignore
clogin CMD_AUTH_LOGON_PROOF_Client = 0x01 {
    u8[32] client_public_key;    
    u8[20] client_proof;    
    u8[20] crc_hash;    
    u8 number_of_telemetry_keys;    
    TelemetryKey[number_of_telemetry_keys] telemetry_keys;    
}

```
## Protocol Version 3

## Wowm Representation
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
## Protocol Version 8

## Wowm Representation
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
