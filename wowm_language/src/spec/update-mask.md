# UpdateMask

**NOT VALID FOR ALL VERSIONS. ONLY KNOWN VALID FOR 1.12.**

TODO

An UpdateMask is a variable length way of sending known fields to the client.
It is represented by a byte mask that decides which fields are sent afterwards.

## Representation

The UpdateMask starts with a single u8 that decides how many **u32 mask blocks** will follow.
The bit pattern in these mask blocks determine how many additional u32s of data will follow.

## Examples
