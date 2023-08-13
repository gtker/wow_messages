# Implementing world messages

The world message data can be found in
the [`intermediate_representation.json`](https://github.com/gtker/wow_messages/blob/main/intermediate_representation.json)
file.
The [`intermediate_representation_schema.json`](https://github.com/gtker/wow_messages/blob/main/intermediate_representation_schema.json)
contains a [JSON type def](https://jsontypedef.com/) schema for the `intermediate_representation.json`.
The [`json-typedef-codegen`](https://github.com/jsontypedef/json-typedef-codegen) program can be used to generate
bindings for many popular languages.

The [`wow_messages_python` repository](https://github.com/gtker/wow_messages_python) contains the Python code used to
generate the Python message library.
It can be used as inspiration for your own implementation.
The [`generator` directory](https://github.com/gtker/wow_messages_python/tree/main/generator) contains the actual code
generation,
and the [`wow_world_messages` directory](https://github.com/gtker/wow_messages_python/tree/main/wow_world_messages)
contains the generated library.

Python code will be shown in this document to provide examples of how libraries could be implemented.
If you want to use the Python library then just bypass this and
[use the library directly](https://github.com/gtker/wow_messages_python/) instead.

## Message Layout

All messages sent from the server start with a 2 byte **big endian** size field, 

