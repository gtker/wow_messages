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
* `features`, an array of [Features](features.md) required.

## Member

The member array is a representation of the layout of the container.

It contains:

* `type`, either `definition`, `if_statement`, or `optional`.
* `content`, the values that belong to a specific `type`.


### Definition

### If Statement

### Optional



## Test
