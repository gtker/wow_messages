# Containers

All containers have:

* `name`, type name of the container.
* `object_type` contains:
    * `type`: either `struct`, `clogin`, `slogin`, `cmsg`, `smsg`, or `msg`.
    * `opcode`, only present if `type` is not struct.
* `constant`, `true` if all fields always have the same size, otherwise `false`.
* `members`, an array of Container Members.
* `tags`, a [Tags](tags.md) object.
* `tests`, an array of Container Tests.
* `file_info`, a [FileInfo](fileinfo.md) object.
* `only_has_io_error`, `true` if the contain does not contain an enum (which can error by having a non-existing enumerator value), a String or CString (which can contain invalid UTF-8 values) or a struct that has either, otherwise `false`.
* `features`, an array of strings with required [Features](../spec/features.md).

## Member

The member array is a representation of the layout of the container.

It contains:

* `type`, either `definition`, `if_statement`, or `optional`.
* `content`, the values that belong to a specific `type`.


### Definition

Definitions represent a specific value that is present in the container.
They contain:

* `name`, the variable name of the definition.
* `member_type`, the type of the definition, contains:
    * `type`, either `integer`, `packed_guid`, `guid`, `floating_point`, `cstring`, `sized_cstring`, `string`, `array`, `identifier`, `update_mask`, or `aura_mask`.
    * `content`, depends on the type.
* `constant_value`, `null` if the field is not always supposed to have the same value. Otherwise it contains:
    * `value`, the constant value it should always be represented as.
    * `original_string`, the original string that lead to the `value`. This can also be `self.size` to represent that the amount of remaining bytes in the message should be input instead of `value`.
* `used_as_size_in`, the field in the current container that the value of this field should be used as the size of, only for integer values. `null` if not used as a size.
* `used_in_if`, `true` if the definition is used in an `if_statement`, otherwise `false`.
* `tags`, a [Tags](tags.md) object.


#### Type Content

The following types do not contain any `content`:

* `packed_guid`
* `guid`
* `cstring`
* `sized_cstring`
* `update_mask`
* `aura_mask`


##### Integer

The `integer` type contains a `content` with:
* `type`, can be any of `u8`, `u16`, `u32` or `u64`.
* `endianness`, either `little` or `big`. Not present when `type` is `u8`.

##### Floating Point

The `floating_point` type contains a `content` with:
* `type`, either `f32` (float) or `f64` (double).
* `endianness`, either `little` or `big`.

##### String

The `string` type contains a `content` with:
* `length`, the variable name of the definition used to signify the length of the string.

##### Array

The `array` type contains a `content` with:
* `inner`, representing the inner type of the array. This contains the fields:
    * `type`, which is either `integer`, `complex`, `cstring`, `guid` or `packed_guid`.
    * `content`, which depends on the `type`.
        * `integer` has the fields:
            * `type`, can be any of `u8`, `u16`, `u32` or `u64`.
            * `endianness`, either `little` or `big`. Not present when `type` is `u8`.
        * `complex` has no fields, but the content is a string with the type name used in the array.
        * `cstring`, `guid`, and `packed_guid` do not have any `content`.
* `size`, representing how the size of the array is known. This contains the fields:
    * `type`, which is either `fixed`, `variable`, or `endless`.
    * `content`, which is:
        * The amount of array members if the size is `fixed`.
        * The variable name that determines amount of members if `variable`.
        * Not present if `endless`. Endless arrays take up all remaining bytes in the message.

##### Identifier

The `identifier` type contains a `content` with:
* `type_name`, the type name of the variable.
* `upcast`, contains the same values as an `integer` `type` field. Only present if the type is an enum where the type in the message is larger than the native type of the definer.

### If Statement

The if statement represents conditional presence of definitions.
They contain:

* `conditional`, contains:
    * `variable_name`, the variable used in the if statement.
    * `equations`, an array of values where one must be true in order to have the container contain the `members`. They have the fields
        * `type`, `equals` if the variable must be exactly this value, `not_equals` if the value must not equal this value, `bitwise_and` if the variable bitwise and should not equal zero.
        * `content`, always contains a `value` that contains the enumerator inside the variable name to check against.
* `members`, contains struct members that are present if the conditional is true.
* `else_ifs_statements`, if statements that are evaluated if the conditional is not true.
* `else_members`, contains struct members that are present if the conditinal is not true.
* `original_ty`, contains the original type of the variable being used in the if statement. It has the same fields as [Type Content](#type-content).

### Optional

The optional statement represents a block of members that do not necessarily have to be present in every version of the message. In order to determine if optional is present, look at the size of any present members, and the reported size from the header.

The optional statement consists of:

* `name`, an identifier for the optional block.
* `members`, an array of [Members](#member).
* `tags`, a [Tags](tags.md) object.

## Test

Tests represent a valid series of bytes, and how they should be interpreted.
A test consists of:

* `subject`, type name of the object being tested.
* `members`, an array of [Test Members](#test-member).
* `raw_bytes`, an array of byte values that the object should be serialized from.
* `tags`, a [Tags](tags.md) object.
* `file_info`, a [FileInfo](fileinfo.md) object.

### Test Member

A test member represents the value that a variable should parse as.
It contains:

* `variable_name`, identifies the variable in question.
* `value`, a [Test Value](#test-value).
* `tags`, a [Tags](tags.md) object.

#### Test Value

A specific value. It contains:

* `type`, any of:
    * `number`
    * `guid`
    * `floating_point`
    * `array`
    * `string`
    * `flag`
    * `enum`
    * `sub_object`
    * `array_of_sub_object`
    * `update_mask`
* `content`, depending on `type`, if `type` is:
    * `number`, `guid`, or `enum`:
        * `value`, an integer of the value.
        * `original_string`, the original string from the Wowm file.
    * `floating_point`
        * `value`, floating point value.
        * `original_string`, the original string from the Wowm file.
    * `array`
        * `values`, an array of the values.
        * `size`, having the same values as the regular [Array Size](#array).
    * `string`, containing the string value.
    * `flag`, containing an array of the flag enumerators that are active.
    * `sub_object`, containing:
        * `type_name`, the type name of the array item.
        * `members`, an array of [Test Members](#test-member).
    * `array_of_sub_object`, containing:
        * `type_name`, the type name of the objects.
        * `members`, an array containing [Test Members](#test-member).
    * `update_mask`, containing an array of:
        * `type`, the object type of the value (OBJECT, UNIT, etc)
        * `name`, the name of the value (GUID, TYPE, HEALTH, etc)
        * `value`, a string with the original value.

