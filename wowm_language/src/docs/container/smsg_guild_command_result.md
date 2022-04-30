## Client Version 1.12

```rust,ignore
smsg SMSG_GUILD_COMMAND_RESULT = 0x0093 {
    GuildCommand command;    
    CString string;    
    GuildCommandResult result;    
}

```
