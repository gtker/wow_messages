# PackedGuid

**NOT VALID FOR ALL VERSIONS. ONLY KNOWN VALID FOR VANILLA, TBC AND WRATH**

Some messages omit zero bytes from the GUID and instead send a byte mask as the first value.
The byte mask is then iterated through and every set bit indicates that a byte of the GUID has been sent.

Note that the bit positions and not the integer values determine how many GUID bytes are sent.
So for example a byte mask of `0b00000010` (`0x2`) indicates that the second least significant byte will be sent and the rest are zero, not that two bytes will be sent.

Since this format is impossible to model elegantly in the Wowm language it is treated as a built-in type that codegen units handle.

## Examples

Assume the following bytes are sent (first byte is shown as binary for ease of understanding)
```rust,ignore
0b00001010, 0xDE, 0xAD
```
This means that:
* The least significant (0th) byte is zero
* The second least significant byte (1st) byte is `0xDE`
* The third least significant (2nd) byte is zero
* The fourth least significant (3rd) byte is `0xAD`
* The rest are zero.

Giving a final GUID of `0x00000000AD00DE00`.

The following C code will parse packed GUIDs:
```c
#include <stdint.h>
#include <stdio.h>

// Assume data is a pointer to immediately after the byte mask
uint64_t parse_packed_guid(uint8_t byte_mask, uint8_t* data) {
    uint64_t guid = 0;
    // We can't use i because only set bits move the cursor
    uint8_t cursor = 0;

    for (uint8_t i = 0; i < 8; ++i) {
        if (byte_mask & (1 << i)) {
            guid |= (data[cursor] << (i * 8));

            ++cursor;
        }
    }

    return guid;
}

int main() {
    uint8_t byte_mask = 0b00001010;
    uint8_t data[] = { 0xDE, 0xAD };

    uint64_t guid = parse_packed_guid(byte_mask, (uint8_t*)&data);

    printf("%lx", guid);
}
```

The following Python code will parse packed GUIDs:
```python
# Assume data is a list of the remaining message data
def parse_packed_guid(byte_mask, data):
    guid = 0
    # We can't use i because only set bits move the cursor
    cursor = 0

    for i in range(0, 8):
        if byte_mask & (1 << i):
            guid |= (data[cursor] << (i * 8))
            cursor += 1

    return guid


byte_mask = 0b00001010
data = [0xDE, 0xAD]

print(hex(parse_packed_guid(byte_mask, data)))
```

# NamedGuid

A `u64` followed by a CString if the `u64` is not equal to 0.
