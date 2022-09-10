* Correct codegen
    * Else Ifs:
        - if a ==, else { if b == } (`SMSG_SEND_MAIL_RESULT`)
        - if a == { if b == } (`SMSG_AUCTION_COMMAND_RESULT`)
* UpdateMask docs
    * Improve Docs
* Add Bool type
* Add flags if a || b (`MovementInfo` wrath)
* Add support for a || b || c.d (`MovementInfo` wrath)
* Add support for if a & A { if b & B } (`MovementInfo` wrath)

# Low Priority

* Max size of vanilla message (10240) mangoszero/server/WorldSocket.cpp:367

* Do not allocate for statically sized messages
* Special type for `SMSG_LOGIN_SETTIMESPEED.datetime`
* More tests/definitions
* MSRV test
    * Login
    * World
