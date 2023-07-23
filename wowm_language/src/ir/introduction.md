# Introduction

The `wowm` data contained in this repository can be read in a structured manner from the [intermediate_representation.json](https://github.com/gtker/wow_messages/blob/main/intermediate_representation.json) file.
It uses the [JSON Typedef](https://jsontypedef.com/) schema in the [intermediate_representation_schema.json](https://github.com/gtker/wow_messages/blob/main/intermediate_representation_schema.json) file.

[`jtd-codegen`](https://github.com/jsontypedef/json-typedef-codegen) can be used to automatically create language bindings in some modern languages, including TypeScript and Python.

It is highly recommended to automatically generate bindings rather than manually parsing the JSON.

If `JSON Typedef` doesn't work for you for whatever reason, it is also possible to deduce a [JSON Schema](https://json-schema.org/) of the `intermediate_representation.json` file and autogenerate bindings from that.
They will likely be of much lower quality though.

After generating the bindings, take a look at [Implementing login messages](implementing_login.md).
