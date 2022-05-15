# Containers

All containers have:

* `name`, type name of the container.
* `object_type` contains:
    * `type`: either `struct`, `clogin`, `slogin`, `cmsg`, `smsg`, or `msg`.
    * `opcode`, only present if `type` is not struct.
* `constant`, `true` if all fields always have the same size, otherwise `false`.
* `members`, an array of Container Members.

TODO
