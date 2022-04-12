## Client Version 1.12

```rust,ignore
enum PetTameFailureReason : u8 {
    INVALIDCREATURE = 1;    
    TOOMANY = 2;    
    CREATUREALREADYOWNED = 3;    
    NOTTAMEABLE = 4;    
    ANOTHERSUMMONACTIVE = 5;    
    UNITSCANTTAME = 6;    
    NOPETAVAILABLE = 7;    
    INTERNALERROR = 8;    
    TOOHIGHLEVEL = 9;    
    DEAD = 10;    
    NOTDEAD = 11;    
    UNKNOWNERROR = 12;    
}

```
