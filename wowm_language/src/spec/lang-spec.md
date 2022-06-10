# Specification

A `.wowm` file starts with 0..* commands, and ends with 0..* statements.
Commands are prefix with `#` and statements start with an object keyword.

## Commands

The file must start with all commands.
A command appearing after a statement is invalid.

Commands take the form
```ignore
#<command> <command_parameters>;
```

For example:
```ignore
#tag_all versions "1.12";
```

## Built-in Types

The following types are not looked up but are expected to be handled by the compiler and code generator:

* The basic integer types `u8`, `u16`, `u32`, and `u64` are sent as little endian over the network.
* The basic integer types `u16_be`, `u32_be`, and `u64_be` are sent as big endian over the network.
* The basic floating point types `f32` and `f64` are sent as little endian over the network.
* The basic floating point types `f32_be` and `f64_be` are sent as big endian over the network.

The `String[len]` type is the same as a sized `u8` array (`u8[len]`) containing valid UTF-8 values, although it should be presented in the native string type.

The `CString` type is an array of valid UTF-8 `u8`s terminated by a null (0) byte.

| Type | Purpose | C Name |
| ---- | ------- | ------ |
| `u*` | Unsigned little endian `*` bit value. | `unsigned` `char`/`short`/`int`/`long long` |
| `u*_be` | Unsigned big endian `*` bit value. | `unsigned` `char`/`short`/`int`/`long long` |
| `i*` | Signed little endian `*` bit value. | `char`/`short`/`int`/`long long` |
| `i*_be` | Signed big endian `*` bit value. | `char`/`short`/`int`/`long long` |
| `f32` and `f64` | Floating point value. | `float`/`double` |
| `f*_be` | Floating point value sent as big endian. | `float`/`double` |
| `CString` | UTF-8 string type that is terminated by a zero byte value. | `char*` |
| `SizedCString` | A `u32` field that determines the size of the string followed by a UTF-8 string type that is terminated by a zero byte value. | `uint32_t` + `char*` |
| `String[len]` | UTF-8 string type of exactly length `len`. | - |
| `PackedGuid` | GUID sent in the "packed" format. See [PackedGuid](packed-guid.md). | - |
| `UpdateMask` | Update values sent in a relatively complex format. See [UpdateMask](update-mask.md). | - |
| `AuraMask` | Update values sent in a relatively complex format. See [AuraMask](aura-mask.md). | - |

### Arrays

Arrays are semi-built-in types of the form `<type>[<length>]` where `<type>` is any built-in or user defined type, and `<length>` is either a constant integer value, a previous integer field in the same object, or the character `-` for "endless" arrays.

#### Endless arrays

Endless arrays do not have a field that specifies how many items the array contains.
This information is instead deducted from the total size of the message minus the sizes of any previous fields.

## Alias Types

In order to give semantic type information to languages that can use it, the following aliases can be used in place of the built-in types.
Codegen units are free to simply substitute for the original built-in type if they want.
These are often used in places where enums do not make sense since almost every single value is valid, but where extra type information would be nice.

| Alias | Built-in | Why |
| ----- | -------- | --- |
| `Guid` | `u64` | Provides interoperability with [PackedGuid](packed-guid.md). |
| `Spell` | `u32` | Signifies a spell id. |
| `Seconds` | `u32` | Specifies that the time unit is seconds. |
| `Milliseconds` | `u32` | Specifies that the time unit is milliseconds. |
| `Copper` | `u32` | Specifies that the currency unit is coppers (one silver being 100 copper, one gold being 100 silver). |

## Statements

Statements start with one of the following keywords:

* `enum`, a _definer_, for descriptions of values where only one value is valid.
* `flag`, a _definer_, for descriptions of values where several values are valid at the same time.
* `struct`, a _container_, for collections of fields that do not fit into any of the below.
* `clogin`, a _container_, for login messages sent **from** the client.
* `slogin`, a _container_, for login messages sent **from** the server.
* `msg`, a _container_, for world messages that can be sent from both client and server.
* `smsg`, a _container_, for world messages sent **from** the server.
* `cmsg`, a _container_, for world messages sent **from** the client.
* `test`, a description of a full valid message and the expected values.

A definer creates a new type that gives names to basic integer values, like an enum would do in most programming languages.
A container is a collection of types that describes the order in which they are sent, as well as other information.

All statements can be followed by a tags block of the form
```ignore
{
    <tags>
}
```

Where `<tags>` is of the form

`<tag_name> = "<tag_value>";`

Where `<tag_name>` is a valid identifier, and `<tag_value>` is a string value.

A list of tags with meaning for the compiler can be found at [Tags](tags.md).

### Definer

Definers take the form of
```ignore
enum <name> : <basic_type> {
    <enumerators>
}
```

Where `<name>` is a unique identifier, `<basic_type>` is an integer type `u8`, `u16`, `u32`, `u64`, `u16_be`, `u32_be`, `u64_be`.

`<enumerators>` is one or more enumerators of the form
```ignore
<name> = <value>;
```
where `<name>` is a unique identifier inside the definer, and `<value>` is a valid value.
Enums can not have multiple names with the same value, while flags can.
Enums can have a `<value>` of `self.value`, while flags can not.

A `<value>` of `self.value` means that the enumerator is not inclusive, so any values that do not exactly match an existing enumerator should be allowed and should not lead to failed parsing.

The allowed number formats in definers and how they are sent over the network are:
```rust,ignore
enum EnumAllowedInteger : u32 {
    INTEGER = 22; /* 22, [22, 0, 0, 0]  */
    HEXADECIMAL = 0xFF; /* 255, [255, 0, 0, 0] */
    BINARY = 0b0101; /* 5, [5, 0, 0, 0] */
    STRING = "\0AB"; /* 0x4142, [66, 65, 0, 0] */
    OTHER = self.value; /* Accept values other than those explicitly here */
}
```

The string syntax has the special case of `\0` which is replaced with a single zero byte.

### Container

Containers take the form of
```ignore
<keyword> <name> [= <opcode>] {
    <declaration | if_statement | optional_statement>*
}
```

Where `<keyword>` is one of 
* `struct`.
* `clogin`, for a login message sent **from** the client.
* `slogin`, for a login message sent **from** the server.
* `msg`, for a world message sent by both the client and server.
* `smsg`, for a world message sent **from** the server.
* `cmsg`, for a world message sent **from** the client.

`<name>` is a valid identifier.

`[= <opcode>]` is an allowed value in the same format as for definer values that defines the unique opcode value for the container.
The `<opcode>` is required for every container except for `struct`s, which have no opcodes.

For `msg`, `smsg`, and `cmsg` the size field is implicitly added as part of the message header.
`clogin` and `slogin` messages that require a specific size field must set the field equal to `self.size`.

#### Declaration

`<declaration>` is of the form
```ignore
[<upcast>]<type> <identifier> [= <constant_value>];
```

Where `<type>` is either a built-in or user defined type.

`<identifier>` is a legal identifier. Two declarations or optional statements in the same object must not have identical identifiers, even across if statement blocks.

The optional `<constant_value>` defines which value this field should always be sent as, used for padding values.
Fields received with a different value will not lead to failed parsing.

The optional `<upcast>` is used for an enum which should be sent over the network as a different type than it is defined with.
This is in order to prevent needing multiple enums for the same concept.
Upcasts have the form `( <integer_type> )` where `integer_type` is an integer type of larger size or different endianness from the type in the original enum.

#### If Statement

`<if_statement>` is of the form
```ignore
if (<variable_name> <operator> <definer_enumerator>
    [|| <variable_name> <operator> <definer_enumerator>]*) {
    <declaration | if_statement | optional_statement>*
} [ else if (<variable_name> <operator> <definer_enumerator> 
    [|| <variable_name> <operator> <definer_enumerator>]* {
    <declaration | if_statement | optional_statement>*
}]* [ else {
    <declaration | if_statement | optional_statement>*
}]?
```

Where:
* `<variable_name>` is the name of a variable from a declaration that has previously appeared. The variable name must be the same in all statements.
* `<operator>` is either `==`, `&`, or `!=`. Restrictions apply to `!=`.
* `<definer_enumerator>` is a valid enumerator in the type of `<variable_name>`.

If statements that use `!=` can not have any `else if` or `||` options, but they can have the `else` option.

#### Optional Statement

`<optional_statement>` is of the form
```ignore
optional <identifier> {
    <declaration | if_statement | optional_statement>*
}
```

Where `<identifier>` is a legal identifier.
Two declarations or optional statements in the same object must not have identical identifiers, even across if statement blocks.

Optional statements can only occur as the last element of a message.

### Test

Tests take the form of
```ignore
test <name> {
    <fields>
} [
    <bytes>
]
```

Where `<name>` is a valid name of an existing container.

`<fields>` is zero or more field definitions of the form
```ignore
<name> = <value> | "[" <array_fields>[","] "]" | "{" <subobject_fields> "}" ;
```
that describe which value to expect for a specific field.

`<array_fields>` and `<subobject_fields>` consist of 1 or more `<fields>`.

`<bytes>` are the raw byte values sent over the network, including the size and opcode fields in unencrypted format.
The allowed formats are the same as those in definer values.

