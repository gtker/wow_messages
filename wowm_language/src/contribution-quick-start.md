# Contribution Quick Start Guide

## Background

Read the [introduction](introduction.md) and [versioning section](versioning-with-tags.md).
Skim the [specification](spec/lang-spec.md).

## Clone the repository

Run `git clone https://github.com/gtker/wow_messages.git` or equivalent to clone the repository.

## Touring the repository

Inside the `wow_message_parser` directory of the repository is the `wowm` directory.
This contains all the `wowm` definitions that are parsed by the compiler.
Go into the `world` directory and notice that it consists of subdirectories that contain `wowm` files.

The directory structure does not make any difference in the final output and is purely there for easier human consumption.
Do not stress over which directory a specific message is supposed to go into, and feel free to create new directories.

If messages are small, keep the different versions in the same `wowm` file.
If they are large keep them in separate files with the version appended, like `smsg_update_object_3_3_5.wowm` for 3.3.5.

## Install Rust

Use [the official site](https://www.rust-lang.org/tools/install) to install Rust.
It is not necessary to have knowledge about Rust in order to contribute, but it is necessary to have it install in order to run the compiler.
The compiler must be run after every change in the `wowm`.
The repository manually keeps the generated files up to date to avoid having to generate them each time they are used.

## Running the compiler

While inside the repository run `cargo run -p wow_message_parser --release` to run the compiler.
This will give you output like:
```text
    Finished release [optimized] target(s) in 0.03s
     Running `target/release/wow_message_parser`
1.12 Messages without definition:
    SMSG_COMPRESSED_UPDATE_OBJECT: Compressed
    [..]
    SMSG_COMPRESSED_MOVES: Compressed

1.12 Messages with definition: 601 / 607 (99.011536%) (6 left)
1.12 Total messages with tests: 60 / 607 (9.884679%)

2.4.3 Messages without definition:
cmsg CMSG_BOOTME = 0x0001 { unimplemented } { versions = "2.4.3"; }
[..]
smsg SMSG_SUMMON_CANCEL = 0x0423 { unimplemented } { versions = "2.4.3"; }

2.4.3 Messages with definition: 126 / 1058 (11.909263%) (932 left)
2.4.3 Total messages with tests: 17 / 1058 (1.6068052%)

3.3.5 Messages without definition:
cmsg CMSG_BOOTME = 0x0001 { unimplemented } { versions = "3.3.5"; }
[..]
smsg SMSG_MULTIPLE_MOVES = 0x051E { unimplemented } { versions = "3.3.5"; }

3.3.5 Messages with definition: 128 / 1309 (9.778457%) (1181 left)
3.3.5 Total messages with tests: 12 / 1309 (0.91673034%)
```
This contains a list of unimplemented messages for the versions that we currently want messages from.
Under 1.12, messages with a colon (`:`) are followed by a reason for them not being implemented currently.
So `SMSG_COMPRESSED_MOVES: Compressed` are not implemented because we are lacking support for compression.
1.12 messages are shown this way because are very few left for 1.12.

2.4.3 and 3.3.5 instead output a copy pastable `wowm` snippet that allows for less manual typing.

## Implementing a message

Let's implement a fictional message for training purposes.
Imagine that when running the compiler it told you that `MSG_FICTIONAL` was not implemented by outputting `msg MSG_FICTIONAL = 0x1337 { unimplemented } { versions = "3.3.5"; }`.

Let's break down what exactly that is for clarity:
```rust,ignore
msg MSG_FICTIONAL = 0x1337 { /* <-- Describes the message */
    unimplemented /* <-- Tells the compiler that this message is unimplemented. */
} { /* <-- Tags start */
    versions = "3.3.5"; /* <-- For version 3.3.5 */
} /* <-- Tags end */
```
The above is the preferred formatting for `wowm` files. Use 4 spaces instead of a tab, and add newlines like you see above.

Just putting the snippet into a `wowm` file inside the `wow_message_parser/wowm/world` directory would make the compiler compile it and add it to the generated code, documentation, and the Wireshark implementation, but it would not tell us anything useful about the message.

### Research

Before we can implement the message we need to figure out what the layout is and if it is even used by the client.

Use [SourceGraph](https://sourcegraph.com/search?q=context:global+CMSG_LOTTERY_QUERY_OBSOLETE&patternType=standard) to search through many emulators at the same time, as well as repositories you probably didn't know existed.

If the message does not appear to be used by the client, pick a new message.

If you are absolutely certain that the message is not used, remove it from the list in `wow_message_parser/src/parser/stats/wrath_messages.rs` (or `tbc_messages.rs` if it's for TBC). 

### Implementation

Let's pretend that the `MSG_FICTIONAL` message is used for something auction house related.

First search for `MSG_FICTIONAL` in the entire `wowm` folder for other versions of this message.
If you find any then place the new implementation next to the other ones, otherwise navigate into the `wow_message_parser/wowm/world/auction` directory and see that it's split into the directories:

* `cmsg`
* `msg`
* `smsg`

Recall that the directory layout of the `wowm` files do not matter for the final product, so this folder layout is not a requirement.

Go into the `msg` directory and create a new file called `msg_fictional.wowm` (all lowercase).

Paste the `wowm` output from before into the file with the correct formatting.

Lean on [the specification](spec/lang-spec.md), [versioning with tags](versioning-with-tags.md) and other messages inside the repository for examples of how to represent the chosen message.

**It is very possible that the message you are trying to implement can not be correctly represented in `wowm` or leads to compile errors when trying to test the libraries. In this case ask for help on the discord or go to another message.**

### Correct versioning

Currently you are only saying that specifically version `3.3.5` is supported by your message but it might be possible that `3.3.4` or even `3.1.0` also use the samme message.
Research the emulators and decide if it seems like the message has the same format in other version as well.

For example, if the message has the same layout in both a `2.4.3` TBC emulator and a `3.3.5` Wrath emulator, you are allowed to assume that any versions in between have the same layout.
You would therefore write `versions = "2.4.3 3";`, since you can't know if versions prior to `2.4.3` have the same layout, but you do know that `3.3.5` is the last Wrath release and that all previous releases have the same layout.

If other objects are preventing you from applying a larger version, for example because it depends on an `Emote` enum that only has version `3.3.5`, then simply give it the biggest possible gap you can and move on.

### Comments

During your research you may discover some things that are uncommon, surprising, or noteworthy.
In this case you can use the `comment` tag to pass on this knowledge.
If you end up writing a long comment you can use multiple tags and they will all appear in the final product.
Like so:
```rust,ignore
msg MSG_FICTIONAL = 0x1337 { /* <-- Describes the message */
    u8 thing; /* <-- Implementation that you should have here now. */
    u32 other_thing {
        comment = "mangoszero always sets this to 0."; /* <-- comment on this specific field */
    }
} {
    versions = "3.3.5";
    comment = "This message is entirely fictional so that the documentation doesn't go out of date as quickly.";
    comment = "But we still have some secret knowledge to share."; /* <-- Comment on the entire message. */
}
```
Remember that `wowm` comments (in between `/*` and `*/`) are not propagated further than the `wowm` sources, so put everything related to the messages in "real" comments and only use the `wowm` comments for `TODO`s or related items.

### Descriptions

If you know enough about the message or any particular fields you can also add `description` tags.

This step, along with the `comment` step, is not required but it would be nice if other people didn't have to go digging in the sources to uncover information that was already known.

### Testing

Run `cargo run -p wow_message_parser --release` and then `cargo test --all-features`.
This will ensure that

* Any changes to the `wowm` files are propagated.
* All tests compile and pass.

### Commit changes

Commit your changes and go on to the next message.

## Questions

If you have any questions ask on [the Discord channel](https://discord.gg/RjSytaEuuX).
