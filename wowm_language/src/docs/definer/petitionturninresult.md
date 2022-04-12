## Client Version 1.12

```rust,ignore
enum PetitionTurnInResult : u32 {
    OK = 0;    
    ALREADY_SIGNED = 1;    
    ALREADY_IN_GUILD = 2;    
    CANT_SIGN_OWN = 3;    
    NEED_MORE = 4;    
    NOT_SERVER = 5;    
}

```
