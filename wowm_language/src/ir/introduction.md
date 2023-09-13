# Introduction

The `wowm` data contained in this repository can be read in a structured manner from the [intermediate_representation.json](https://github.com/gtker/wow_messages/blob/main/intermediate_representation.json) file.
It uses the [JSON Typedef](https://jsontypedef.com/) schema in the [intermediate_representation_schema.json](https://github.com/gtker/wow_messages/blob/main/intermediate_representation_schema.json) file.

[`jtd-codegen`](https://github.com/jsontypedef/json-typedef-codegen) can be used to automatically create language bindings in some modern languages, including TypeScript and Python.

It is highly recommended to automatically generate bindings rather than manually parsing the JSON.

If `JSON Typedef` doesn't work for you for whatever reason, it is also possible to deduce a [JSON Schema](https://json-schema.org/) of the `intermediate_representation.json` file and autogenerate bindings from that.
They will likely be of much lower quality though.

After generating the bindings, take a look at [Implementing login messages](implementing_login.md).

## Tests

Some containers have associated tests that consist of binary data as well as what is expected to be parsed.
Creating simple tests that read the raw bytes, asserts that the read went successfully,
writing the bytes back and asserting that raw bytes match, as well as testing that the size is
calculated correctly will save you a lot of effort.

## Design of types

Before writing the auto generator it's a good idea to know what you actually want to generate.
For this reason I suggest manually writing the implementations for a few select types, and using it to have a client
connect.
This will allow you to detect things that need to be changed as early as possible as well as having executable code
that can be used to test your auto generated code later, even in interpreted languages.
