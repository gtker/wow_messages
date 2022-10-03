* Correct codegen
    * Else Ifs:
        - if a ==, else { if b == } (`SMSG_SEND_MAIL_RESULT`)
        - if a == { if b == } (`SMSG_AUCTION_COMMAND_RESULT`)
* UpdateMask docs
    * Improve Docs
* Add flags if a || b (`MovementInfo` wrath)
* Add support for a || b || c.d (`MovementInfo` wrath)
* Add support for if a & A { if b & B } (`MovementInfo` wrath)
* Add support for if on flags/enums inside structs in order to deduplicate SMSG_UPDATE_OBJECT MovementInfo nonsense
* Add support for messages with compression (`SMSG_COMPRESSED_MOVES`, `SMSG_COMPRESSED_UPDATE_OBJECT`
  , `SMSG_ADDON_INFO`)

# Low Priority

* Max size of vanilla message (10240) mangoszero/server/WorldSocket.cpp:367

* Feature has variants that are never constructed

* Do not allocate for statically sized messages
* More tests/definitions
* MSRV test
    * Login
    * World
