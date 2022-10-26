use crate::wrath::{ServerMessage, ClientMessage};
#[cfg(feature = "encryption")]
use wow_srp::wrath_header::{ClientEncrypterHalf, ClientDecrypterHalf, ServerEncrypterHalf, ServerDecrypterHalf};

#[cfg(feature = "tokio")]
use tokio::io::{AsyncReadExt, AsyncWriteExt};
#[cfg(feature = "async-std")]
use async_std::io::{ReadExt, WriteExt};
use crate::world::wrath::MovementInfo;
use crate::errors::ParseError;
use crate::world::wrath::MSG_MOVE_START_FORWARD;
use crate::world::wrath::MSG_MOVE_START_BACKWARD;
use crate::world::wrath::MSG_MOVE_STOP;
use crate::world::wrath::MSG_MOVE_START_STRAFE_LEFT;
use crate::world::wrath::MSG_MOVE_START_STRAFE_RIGHT;
use crate::world::wrath::MSG_MOVE_STOP_STRAFE;
use crate::world::wrath::MSG_MOVE_JUMP;
use crate::world::wrath::MSG_MOVE_START_TURN_LEFT;
use crate::world::wrath::MSG_MOVE_START_TURN_RIGHT;
use crate::world::wrath::MSG_MOVE_STOP_TURN;
use crate::world::wrath::MSG_MOVE_START_PITCH_UP;
use crate::world::wrath::MSG_MOVE_START_PITCH_DOWN;
use crate::world::wrath::MSG_MOVE_STOP_PITCH;
use crate::world::wrath::MSG_MOVE_SET_RUN_MODE;
use crate::world::wrath::MSG_MOVE_SET_WALK_MODE;
use crate::world::wrath::MSG_MOVE_FALL_LAND;
use crate::world::wrath::MSG_MOVE_START_SWIM;
use crate::world::wrath::MSG_MOVE_STOP_SWIM;
use crate::world::wrath::MSG_MOVE_SET_FACING;
use crate::world::wrath::MSG_MOVE_SET_PITCH;
use crate::world::wrath::MSG_MOVE_WORLDPORT_ACK;
use crate::world::wrath::MSG_MOVE_HEARTBEAT;
use crate::world::wrath::MSG_PETITION_DECLINE;
use crate::world::wrath::MSG_TABARDVENDOR_ACTIVATE;
use crate::world::wrath::MSG_PETITION_RENAME;
use crate::world::wrath::MSG_SET_DUNGEON_DIFFICULTY;
use crate::world::wrath::MSG_MOVE_START_ASCEND;
use crate::world::wrath::MSG_MOVE_STOP_ASCEND;
use crate::world::wrath::MSG_MOVE_START_DESCEND;
use crate::world::wrath::CMSG_CHAR_CREATE;
use crate::world::wrath::CMSG_CHAR_ENUM;
use crate::world::wrath::CMSG_CHAR_DELETE;
use crate::world::wrath::CMSG_PLAYER_LOGIN;
use crate::world::wrath::CMSG_PLAYER_LOGOUT;
use crate::world::wrath::CMSG_LOGOUT_REQUEST;
use crate::world::wrath::CMSG_LOGOUT_CANCEL;
use crate::world::wrath::CMSG_NAME_QUERY;
use crate::world::wrath::CMSG_GUILD_CREATE;
use crate::world::wrath::CMSG_GUILD_INVITE;
use crate::world::wrath::CMSG_GUILD_ACCEPT;
use crate::world::wrath::CMSG_GUILD_DECLINE;
use crate::world::wrath::CMSG_GUILD_INFO;
use crate::world::wrath::CMSG_GUILD_ROSTER;
use crate::world::wrath::CMSG_GUILD_PROMOTE;
use crate::world::wrath::CMSG_GUILD_DEMOTE;
use crate::world::wrath::CMSG_GUILD_LEAVE;
use crate::world::wrath::CMSG_GUILD_REMOVE;
use crate::world::wrath::CMSG_GUILD_DISBAND;
use crate::world::wrath::CMSG_GUILD_LEADER;
use crate::world::wrath::CMSG_GUILD_MOTD;
use crate::world::wrath::CMSG_MESSAGECHAT;
use crate::world::wrath::CMSG_JOIN_CHANNEL;
use crate::world::wrath::CMSG_LEAVE_CHANNEL;
use crate::world::wrath::CMSG_CHANNEL_LIST;
use crate::world::wrath::CMSG_CHANNEL_PASSWORD;
use crate::world::wrath::CMSG_CHANNEL_SET_OWNER;
use crate::world::wrath::CMSG_CHANNEL_OWNER;
use crate::world::wrath::CMSG_CHANNEL_MODERATOR;
use crate::world::wrath::CMSG_CHANNEL_UNMODERATOR;
use crate::world::wrath::CMSG_CHANNEL_MUTE;
use crate::world::wrath::CMSG_CHANNEL_UNMUTE;
use crate::world::wrath::CMSG_CHANNEL_INVITE;
use crate::world::wrath::CMSG_CHANNEL_KICK;
use crate::world::wrath::CMSG_CHANNEL_BAN;
use crate::world::wrath::CMSG_CHANNEL_UNBAN;
use crate::world::wrath::CMSG_CHANNEL_ANNOUNCEMENTS;
use crate::world::wrath::CMSG_CHANNEL_MODERATE;
use crate::world::wrath::CMSG_OPEN_ITEM;
use crate::world::wrath::CMSG_READ_ITEM;
use crate::world::wrath::CMSG_GAMEOBJ_USE;
use crate::world::wrath::CMSG_AREATRIGGER;
use crate::world::wrath::MSG_MOVE_TELEPORT_ACK_Client;
use crate::world::wrath::CMSG_FORCE_RUN_SPEED_CHANGE_ACK;
use crate::world::wrath::CMSG_FORCE_RUN_BACK_SPEED_CHANGE_ACK;
use crate::world::wrath::CMSG_FORCE_SWIM_SPEED_CHANGE_ACK;
use crate::world::wrath::CMSG_FORCE_MOVE_ROOT_ACK;
use crate::world::wrath::CMSG_FORCE_MOVE_UNROOT_ACK;
use crate::world::wrath::CMSG_NEXT_CINEMATIC_CAMERA;
use crate::world::wrath::CMSG_COMPLETE_CINEMATIC;
use crate::world::wrath::CMSG_TUTORIAL_FLAG;
use crate::world::wrath::CMSG_TUTORIAL_CLEAR;
use crate::world::wrath::CMSG_TUTORIAL_RESET;
use crate::world::wrath::CMSG_STANDSTATECHANGE;
use crate::world::wrath::CMSG_EMOTE;
use crate::world::wrath::CMSG_TEXT_EMOTE;
use crate::world::wrath::CMSG_AUTOSTORE_LOOT_ITEM;
use crate::world::wrath::CMSG_AUTOEQUIP_ITEM;
use crate::world::wrath::CMSG_AUTOSTORE_BAG_ITEM;
use crate::world::wrath::CMSG_SWAP_ITEM;
use crate::world::wrath::CMSG_SWAP_INV_ITEM;
use crate::world::wrath::CMSG_SPLIT_ITEM;
use crate::world::wrath::CMSG_AUTOEQUIP_ITEM_SLOT;
use crate::world::wrath::CMSG_DESTROYITEM;
use crate::world::wrath::CMSG_INSPECT;
use crate::world::wrath::CMSG_SET_FACTION_ATWAR;
use crate::world::wrath::CMSG_SET_ACTION_BUTTON;
use crate::world::wrath::CMSG_CANCEL_CHANNELLING;
use crate::world::wrath::CMSG_SET_SELECTION;
use crate::world::wrath::CMSG_SET_TARGET_OBSOLETE;
use crate::world::wrath::CMSG_ATTACKSWING;
use crate::world::wrath::CMSG_ATTACKSTOP;
use crate::world::wrath::CMSG_REPOP_REQUEST;
use crate::world::wrath::CMSG_LOOT;
use crate::world::wrath::CMSG_LOOT_MONEY;
use crate::world::wrath::CMSG_LOOT_RELEASE;
use crate::world::wrath::CMSG_DUEL_ACCEPTED;
use crate::world::wrath::CMSG_DUEL_CANCELLED;
use crate::world::wrath::CMSG_MOUNTSPECIAL_ANIM;
use crate::world::wrath::CMSG_PET_ACTION;
use crate::world::wrath::CMSG_PET_ABANDON;
use crate::world::wrath::CMSG_GOSSIP_HELLO;
use crate::world::wrath::CMSG_GOSSIP_SELECT_OPTION;
use crate::world::wrath::CMSG_LIST_INVENTORY;
use crate::world::wrath::CMSG_SELL_ITEM;
use crate::world::wrath::CMSG_BUY_ITEM;
use crate::world::wrath::CMSG_BUY_ITEM_IN_SLOT;
use crate::world::wrath::CMSG_ACTIVATETAXI;
use crate::world::wrath::CMSG_BINDER_ACTIVATE;
use crate::world::wrath::CMSG_BANKER_ACTIVATE;
use crate::world::wrath::CMSG_BUY_BANK_SLOT;
use crate::world::wrath::CMSG_PETITION_SHOWLIST;
use crate::world::wrath::CMSG_PETITION_BUY;
use crate::world::wrath::CMSG_PETITION_SHOW_SIGNATURES;
use crate::world::wrath::CMSG_PETITION_SIGN;
use crate::world::wrath::CMSG_OFFER_PETITION;
use crate::world::wrath::CMSG_TURN_IN_PETITION;
use crate::world::wrath::CMSG_BUG;
use crate::world::wrath::CMSG_PLAYED_TIME;
use crate::world::wrath::CMSG_QUERY_TIME;
use crate::world::wrath::CMSG_WRAP_ITEM;
use crate::world::wrath::CMSG_PING;
use crate::world::wrath::CMSG_SETSHEATHED;
use crate::world::wrath::CMSG_AUTH_SESSION;
use crate::world::wrath::MSG_SAVE_GUILD_EMBLEM_Client;
use crate::world::wrath::CMSG_ZONEUPDATE;
use crate::world::wrath::MSG_RANDOM_ROLL_Client;
use crate::world::wrath::CMSG_GMTICKET_UPDATETEXT;
use crate::world::wrath::CMSG_REQUEST_ACCOUNT_DATA;
use crate::world::wrath::CMSG_UPDATE_ACCOUNT_DATA;
use crate::world::wrath::CMSG_GMTICKET_GETTICKET;
use crate::world::wrath::CMSG_GMTICKET_DELETETICKET;
use crate::world::wrath::CMSG_GMTICKET_SYSTEMSTATUS;
use crate::world::wrath::CMSG_CHAT_IGNORED;
use crate::world::wrath::CMSG_GUILD_RANK;
use crate::world::wrath::CMSG_GUILD_ADD_RANK;
use crate::world::wrath::CMSG_GUILD_DEL_RANK;
use crate::world::wrath::CMSG_GUILD_SET_PUBLIC_NOTE;
use crate::world::wrath::CMSG_GUILD_SET_OFFICER_NOTE;
use crate::world::wrath::CMSG_SEND_MAIL;
use crate::world::wrath::CMSG_GET_MAIL_LIST;
use crate::world::wrath::CMSG_BATTLEFIELD_LIST;
use crate::world::wrath::CMSG_MAIL_TAKE_MONEY;
use crate::world::wrath::CMSG_MAIL_TAKE_ITEM;
use crate::world::wrath::CMSG_MAIL_MARK_AS_READ;
use crate::world::wrath::CMSG_MAIL_RETURN_TO_SENDER;
use crate::world::wrath::CMSG_MAIL_DELETE;
use crate::world::wrath::CMSG_MAIL_CREATE_TEXT_ITEM;
use crate::world::wrath::MSG_AUCTION_HELLO_Client;
use crate::world::wrath::CMSG_AUCTION_SELL_ITEM;
use crate::world::wrath::CMSG_AUCTION_REMOVE_ITEM;
use crate::world::wrath::CMSG_AUCTION_LIST_ITEMS;
use crate::world::wrath::CMSG_AUCTION_LIST_OWNER_ITEMS;
use crate::world::wrath::CMSG_AUCTION_PLACE_BID;
use crate::world::wrath::CMSG_AUCTION_LIST_BIDDER_ITEMS;
use crate::world::wrath::CMSG_SET_AMMO;
use crate::world::wrath::CMSG_SET_ACTIVE_MOVER;
use crate::world::wrath::CMSG_BUY_STABLE_SLOT;
use crate::world::wrath::CMSG_AUTOSTORE_BANK_ITEM;
use crate::world::wrath::CMSG_AUTOBANK_ITEM;
use crate::world::wrath::CMSG_BUYBACK_ITEM;
use crate::world::wrath::CMSG_LOOT_ROLL;
use crate::world::wrath::CMSG_LOOT_MASTER_GIVE;
use crate::world::wrath::CMSG_REPAIR_ITEM;
use crate::world::wrath::CMSG_TOGGLE_HELM;
use crate::world::wrath::CMSG_TOGGLE_CLOAK;
use crate::world::wrath::CMSG_SET_ACTIONBAR_TOGGLES;
use crate::world::wrath::CMSG_CHAR_RENAME;
use crate::world::wrath::CMSG_MOVE_FALL_RESET;
use crate::world::wrath::CMSG_BATTLEFIELD_STATUS;
use crate::world::wrath::CMSG_BATTLEFIELD_PORT;
use crate::world::wrath::CMSG_BATTLEMASTER_HELLO;
use crate::world::wrath::CMSG_FORCE_WALK_SPEED_CHANGE_ACK;
use crate::world::wrath::CMSG_FORCE_SWIM_BACK_SPEED_CHANGE_ACK;
use crate::world::wrath::CMSG_FORCE_TURN_RATE_CHANGE_ACK;
use crate::world::wrath::CMSG_LEAVE_BATTLEFIELD;
use crate::world::wrath::MSG_BATTLEGROUND_PLAYER_POSITIONS_Client;
use crate::world::wrath::CMSG_BATTLEMASTER_JOIN;
use crate::world::wrath::CMSG_GUILD_INFO_TEXT;
use crate::world::wrath::CMSG_ACTIVATETAXIEXPRESS;
use crate::world::wrath::CMSG_SET_WATCHED_FACTION;
use crate::world::wrath::CMSG_GMSURVEY_SUBMIT;
use crate::world::wrath::CMSG_MOVE_SET_FLY;
use crate::world::wrath::CMSG_REALM_SPLIT;
use crate::world::wrath::CMSG_MOVE_CHNG_TRANSPORT;
use crate::world::wrath::CMSG_TIME_SYNC_RESP;
use crate::world::wrath::CMSG_SET_ACTIVE_VOICE_CHANNEL;
use crate::world::wrath::CMSG_WORLD_STATE_UI_TIMER_UPDATE;
use crate::world::wrath::CMSG_READY_FOR_ACCOUNT_DATA_TIMES;

#[derive(Debug, Clone, PartialEq)]
pub enum ClientOpcodeMessage {
    MSG_MOVE_START_FORWARD(MSG_MOVE_START_FORWARD),
    MSG_MOVE_START_BACKWARD(MSG_MOVE_START_BACKWARD),
    MSG_MOVE_STOP(MSG_MOVE_STOP),
    MSG_MOVE_START_STRAFE_LEFT(MSG_MOVE_START_STRAFE_LEFT),
    MSG_MOVE_START_STRAFE_RIGHT(MSG_MOVE_START_STRAFE_RIGHT),
    MSG_MOVE_STOP_STRAFE(MSG_MOVE_STOP_STRAFE),
    MSG_MOVE_JUMP(MSG_MOVE_JUMP),
    MSG_MOVE_START_TURN_LEFT(MSG_MOVE_START_TURN_LEFT),
    MSG_MOVE_START_TURN_RIGHT(MSG_MOVE_START_TURN_RIGHT),
    MSG_MOVE_STOP_TURN(MSG_MOVE_STOP_TURN),
    MSG_MOVE_START_PITCH_UP(MSG_MOVE_START_PITCH_UP),
    MSG_MOVE_START_PITCH_DOWN(MSG_MOVE_START_PITCH_DOWN),
    MSG_MOVE_STOP_PITCH(MSG_MOVE_STOP_PITCH),
    MSG_MOVE_SET_RUN_MODE(MSG_MOVE_SET_RUN_MODE),
    MSG_MOVE_SET_WALK_MODE(MSG_MOVE_SET_WALK_MODE),
    MSG_MOVE_FALL_LAND(MSG_MOVE_FALL_LAND),
    MSG_MOVE_START_SWIM(MSG_MOVE_START_SWIM),
    MSG_MOVE_STOP_SWIM(MSG_MOVE_STOP_SWIM),
    MSG_MOVE_SET_FACING(MSG_MOVE_SET_FACING),
    MSG_MOVE_SET_PITCH(MSG_MOVE_SET_PITCH),
    MSG_MOVE_WORLDPORT_ACK(MSG_MOVE_WORLDPORT_ACK),
    MSG_MOVE_HEARTBEAT(MSG_MOVE_HEARTBEAT),
    MSG_PETITION_DECLINE(MSG_PETITION_DECLINE),
    MSG_TABARDVENDOR_ACTIVATE(MSG_TABARDVENDOR_ACTIVATE),
    MSG_PETITION_RENAME(MSG_PETITION_RENAME),
    MSG_SET_DUNGEON_DIFFICULTY(MSG_SET_DUNGEON_DIFFICULTY),
    MSG_MOVE_START_ASCEND(MSG_MOVE_START_ASCEND),
    MSG_MOVE_STOP_ASCEND(MSG_MOVE_STOP_ASCEND),
    MSG_MOVE_START_DESCEND(MSG_MOVE_START_DESCEND),
    CMSG_CHAR_CREATE(CMSG_CHAR_CREATE),
    CMSG_CHAR_ENUM(CMSG_CHAR_ENUM),
    CMSG_CHAR_DELETE(CMSG_CHAR_DELETE),
    CMSG_PLAYER_LOGIN(CMSG_PLAYER_LOGIN),
    CMSG_PLAYER_LOGOUT(CMSG_PLAYER_LOGOUT),
    CMSG_LOGOUT_REQUEST(CMSG_LOGOUT_REQUEST),
    CMSG_LOGOUT_CANCEL(CMSG_LOGOUT_CANCEL),
    CMSG_NAME_QUERY(CMSG_NAME_QUERY),
    CMSG_GUILD_CREATE(CMSG_GUILD_CREATE),
    CMSG_GUILD_INVITE(CMSG_GUILD_INVITE),
    CMSG_GUILD_ACCEPT(CMSG_GUILD_ACCEPT),
    CMSG_GUILD_DECLINE(CMSG_GUILD_DECLINE),
    CMSG_GUILD_INFO(CMSG_GUILD_INFO),
    CMSG_GUILD_ROSTER(CMSG_GUILD_ROSTER),
    CMSG_GUILD_PROMOTE(CMSG_GUILD_PROMOTE),
    CMSG_GUILD_DEMOTE(CMSG_GUILD_DEMOTE),
    CMSG_GUILD_LEAVE(CMSG_GUILD_LEAVE),
    CMSG_GUILD_REMOVE(CMSG_GUILD_REMOVE),
    CMSG_GUILD_DISBAND(CMSG_GUILD_DISBAND),
    CMSG_GUILD_LEADER(CMSG_GUILD_LEADER),
    CMSG_GUILD_MOTD(CMSG_GUILD_MOTD),
    CMSG_MESSAGECHAT(CMSG_MESSAGECHAT),
    CMSG_JOIN_CHANNEL(CMSG_JOIN_CHANNEL),
    CMSG_LEAVE_CHANNEL(CMSG_LEAVE_CHANNEL),
    CMSG_CHANNEL_LIST(CMSG_CHANNEL_LIST),
    CMSG_CHANNEL_PASSWORD(CMSG_CHANNEL_PASSWORD),
    CMSG_CHANNEL_SET_OWNER(CMSG_CHANNEL_SET_OWNER),
    CMSG_CHANNEL_OWNER(CMSG_CHANNEL_OWNER),
    CMSG_CHANNEL_MODERATOR(CMSG_CHANNEL_MODERATOR),
    CMSG_CHANNEL_UNMODERATOR(CMSG_CHANNEL_UNMODERATOR),
    CMSG_CHANNEL_MUTE(CMSG_CHANNEL_MUTE),
    CMSG_CHANNEL_UNMUTE(CMSG_CHANNEL_UNMUTE),
    CMSG_CHANNEL_INVITE(CMSG_CHANNEL_INVITE),
    CMSG_CHANNEL_KICK(CMSG_CHANNEL_KICK),
    CMSG_CHANNEL_BAN(CMSG_CHANNEL_BAN),
    CMSG_CHANNEL_UNBAN(CMSG_CHANNEL_UNBAN),
    CMSG_CHANNEL_ANNOUNCEMENTS(CMSG_CHANNEL_ANNOUNCEMENTS),
    CMSG_CHANNEL_MODERATE(CMSG_CHANNEL_MODERATE),
    CMSG_OPEN_ITEM(CMSG_OPEN_ITEM),
    CMSG_READ_ITEM(CMSG_READ_ITEM),
    CMSG_GAMEOBJ_USE(CMSG_GAMEOBJ_USE),
    CMSG_AREATRIGGER(CMSG_AREATRIGGER),
    MSG_MOVE_TELEPORT_ACK(MSG_MOVE_TELEPORT_ACK_Client),
    CMSG_FORCE_RUN_SPEED_CHANGE_ACK(CMSG_FORCE_RUN_SPEED_CHANGE_ACK),
    CMSG_FORCE_RUN_BACK_SPEED_CHANGE_ACK(CMSG_FORCE_RUN_BACK_SPEED_CHANGE_ACK),
    CMSG_FORCE_SWIM_SPEED_CHANGE_ACK(CMSG_FORCE_SWIM_SPEED_CHANGE_ACK),
    CMSG_FORCE_MOVE_ROOT_ACK(CMSG_FORCE_MOVE_ROOT_ACK),
    CMSG_FORCE_MOVE_UNROOT_ACK(CMSG_FORCE_MOVE_UNROOT_ACK),
    CMSG_NEXT_CINEMATIC_CAMERA(CMSG_NEXT_CINEMATIC_CAMERA),
    CMSG_COMPLETE_CINEMATIC(CMSG_COMPLETE_CINEMATIC),
    CMSG_TUTORIAL_FLAG(CMSG_TUTORIAL_FLAG),
    CMSG_TUTORIAL_CLEAR(CMSG_TUTORIAL_CLEAR),
    CMSG_TUTORIAL_RESET(CMSG_TUTORIAL_RESET),
    CMSG_STANDSTATECHANGE(CMSG_STANDSTATECHANGE),
    CMSG_EMOTE(CMSG_EMOTE),
    CMSG_TEXT_EMOTE(CMSG_TEXT_EMOTE),
    CMSG_AUTOSTORE_LOOT_ITEM(CMSG_AUTOSTORE_LOOT_ITEM),
    CMSG_AUTOEQUIP_ITEM(CMSG_AUTOEQUIP_ITEM),
    CMSG_AUTOSTORE_BAG_ITEM(CMSG_AUTOSTORE_BAG_ITEM),
    CMSG_SWAP_ITEM(CMSG_SWAP_ITEM),
    CMSG_SWAP_INV_ITEM(CMSG_SWAP_INV_ITEM),
    CMSG_SPLIT_ITEM(CMSG_SPLIT_ITEM),
    CMSG_AUTOEQUIP_ITEM_SLOT(CMSG_AUTOEQUIP_ITEM_SLOT),
    CMSG_DESTROYITEM(CMSG_DESTROYITEM),
    CMSG_INSPECT(CMSG_INSPECT),
    CMSG_SET_FACTION_ATWAR(CMSG_SET_FACTION_ATWAR),
    CMSG_SET_ACTION_BUTTON(CMSG_SET_ACTION_BUTTON),
    CMSG_CANCEL_CHANNELLING(CMSG_CANCEL_CHANNELLING),
    CMSG_SET_SELECTION(CMSG_SET_SELECTION),
    CMSG_SET_TARGET_OBSOLETE(CMSG_SET_TARGET_OBSOLETE),
    CMSG_ATTACKSWING(CMSG_ATTACKSWING),
    CMSG_ATTACKSTOP(CMSG_ATTACKSTOP),
    CMSG_REPOP_REQUEST(CMSG_REPOP_REQUEST),
    CMSG_LOOT(CMSG_LOOT),
    CMSG_LOOT_MONEY(CMSG_LOOT_MONEY),
    CMSG_LOOT_RELEASE(CMSG_LOOT_RELEASE),
    CMSG_DUEL_ACCEPTED(CMSG_DUEL_ACCEPTED),
    CMSG_DUEL_CANCELLED(CMSG_DUEL_CANCELLED),
    CMSG_MOUNTSPECIAL_ANIM(CMSG_MOUNTSPECIAL_ANIM),
    CMSG_PET_ACTION(CMSG_PET_ACTION),
    CMSG_PET_ABANDON(CMSG_PET_ABANDON),
    CMSG_GOSSIP_HELLO(CMSG_GOSSIP_HELLO),
    CMSG_GOSSIP_SELECT_OPTION(CMSG_GOSSIP_SELECT_OPTION),
    CMSG_LIST_INVENTORY(CMSG_LIST_INVENTORY),
    CMSG_SELL_ITEM(CMSG_SELL_ITEM),
    CMSG_BUY_ITEM(CMSG_BUY_ITEM),
    CMSG_BUY_ITEM_IN_SLOT(CMSG_BUY_ITEM_IN_SLOT),
    CMSG_ACTIVATETAXI(CMSG_ACTIVATETAXI),
    CMSG_BINDER_ACTIVATE(CMSG_BINDER_ACTIVATE),
    CMSG_BANKER_ACTIVATE(CMSG_BANKER_ACTIVATE),
    CMSG_BUY_BANK_SLOT(CMSG_BUY_BANK_SLOT),
    CMSG_PETITION_SHOWLIST(CMSG_PETITION_SHOWLIST),
    CMSG_PETITION_BUY(CMSG_PETITION_BUY),
    CMSG_PETITION_SHOW_SIGNATURES(CMSG_PETITION_SHOW_SIGNATURES),
    CMSG_PETITION_SIGN(CMSG_PETITION_SIGN),
    CMSG_OFFER_PETITION(CMSG_OFFER_PETITION),
    CMSG_TURN_IN_PETITION(CMSG_TURN_IN_PETITION),
    CMSG_BUG(CMSG_BUG),
    CMSG_PLAYED_TIME(CMSG_PLAYED_TIME),
    CMSG_QUERY_TIME(CMSG_QUERY_TIME),
    CMSG_WRAP_ITEM(CMSG_WRAP_ITEM),
    CMSG_PING(CMSG_PING),
    CMSG_SETSHEATHED(CMSG_SETSHEATHED),
    CMSG_AUTH_SESSION(CMSG_AUTH_SESSION),
    MSG_SAVE_GUILD_EMBLEM(MSG_SAVE_GUILD_EMBLEM_Client),
    CMSG_ZONEUPDATE(CMSG_ZONEUPDATE),
    MSG_RANDOM_ROLL(MSG_RANDOM_ROLL_Client),
    CMSG_GMTICKET_UPDATETEXT(CMSG_GMTICKET_UPDATETEXT),
    CMSG_REQUEST_ACCOUNT_DATA(CMSG_REQUEST_ACCOUNT_DATA),
    CMSG_UPDATE_ACCOUNT_DATA(CMSG_UPDATE_ACCOUNT_DATA),
    CMSG_GMTICKET_GETTICKET(CMSG_GMTICKET_GETTICKET),
    CMSG_GMTICKET_DELETETICKET(CMSG_GMTICKET_DELETETICKET),
    CMSG_GMTICKET_SYSTEMSTATUS(CMSG_GMTICKET_SYSTEMSTATUS),
    CMSG_CHAT_IGNORED(CMSG_CHAT_IGNORED),
    CMSG_GUILD_RANK(CMSG_GUILD_RANK),
    CMSG_GUILD_ADD_RANK(CMSG_GUILD_ADD_RANK),
    CMSG_GUILD_DEL_RANK(CMSG_GUILD_DEL_RANK),
    CMSG_GUILD_SET_PUBLIC_NOTE(CMSG_GUILD_SET_PUBLIC_NOTE),
    CMSG_GUILD_SET_OFFICER_NOTE(CMSG_GUILD_SET_OFFICER_NOTE),
    CMSG_SEND_MAIL(CMSG_SEND_MAIL),
    CMSG_GET_MAIL_LIST(CMSG_GET_MAIL_LIST),
    CMSG_BATTLEFIELD_LIST(CMSG_BATTLEFIELD_LIST),
    CMSG_MAIL_TAKE_MONEY(CMSG_MAIL_TAKE_MONEY),
    CMSG_MAIL_TAKE_ITEM(CMSG_MAIL_TAKE_ITEM),
    CMSG_MAIL_MARK_AS_READ(CMSG_MAIL_MARK_AS_READ),
    CMSG_MAIL_RETURN_TO_SENDER(CMSG_MAIL_RETURN_TO_SENDER),
    CMSG_MAIL_DELETE(CMSG_MAIL_DELETE),
    CMSG_MAIL_CREATE_TEXT_ITEM(CMSG_MAIL_CREATE_TEXT_ITEM),
    MSG_AUCTION_HELLO(MSG_AUCTION_HELLO_Client),
    CMSG_AUCTION_SELL_ITEM(CMSG_AUCTION_SELL_ITEM),
    CMSG_AUCTION_REMOVE_ITEM(CMSG_AUCTION_REMOVE_ITEM),
    CMSG_AUCTION_LIST_ITEMS(CMSG_AUCTION_LIST_ITEMS),
    CMSG_AUCTION_LIST_OWNER_ITEMS(CMSG_AUCTION_LIST_OWNER_ITEMS),
    CMSG_AUCTION_PLACE_BID(CMSG_AUCTION_PLACE_BID),
    CMSG_AUCTION_LIST_BIDDER_ITEMS(CMSG_AUCTION_LIST_BIDDER_ITEMS),
    CMSG_SET_AMMO(CMSG_SET_AMMO),
    CMSG_SET_ACTIVE_MOVER(CMSG_SET_ACTIVE_MOVER),
    CMSG_BUY_STABLE_SLOT(CMSG_BUY_STABLE_SLOT),
    CMSG_AUTOSTORE_BANK_ITEM(CMSG_AUTOSTORE_BANK_ITEM),
    CMSG_AUTOBANK_ITEM(CMSG_AUTOBANK_ITEM),
    CMSG_BUYBACK_ITEM(CMSG_BUYBACK_ITEM),
    CMSG_LOOT_ROLL(CMSG_LOOT_ROLL),
    CMSG_LOOT_MASTER_GIVE(CMSG_LOOT_MASTER_GIVE),
    CMSG_REPAIR_ITEM(CMSG_REPAIR_ITEM),
    CMSG_TOGGLE_HELM(CMSG_TOGGLE_HELM),
    CMSG_TOGGLE_CLOAK(CMSG_TOGGLE_CLOAK),
    CMSG_SET_ACTIONBAR_TOGGLES(CMSG_SET_ACTIONBAR_TOGGLES),
    CMSG_CHAR_RENAME(CMSG_CHAR_RENAME),
    CMSG_MOVE_FALL_RESET(CMSG_MOVE_FALL_RESET),
    CMSG_BATTLEFIELD_STATUS(CMSG_BATTLEFIELD_STATUS),
    CMSG_BATTLEFIELD_PORT(CMSG_BATTLEFIELD_PORT),
    CMSG_BATTLEMASTER_HELLO(CMSG_BATTLEMASTER_HELLO),
    CMSG_FORCE_WALK_SPEED_CHANGE_ACK(CMSG_FORCE_WALK_SPEED_CHANGE_ACK),
    CMSG_FORCE_SWIM_BACK_SPEED_CHANGE_ACK(CMSG_FORCE_SWIM_BACK_SPEED_CHANGE_ACK),
    CMSG_FORCE_TURN_RATE_CHANGE_ACK(CMSG_FORCE_TURN_RATE_CHANGE_ACK),
    CMSG_LEAVE_BATTLEFIELD(CMSG_LEAVE_BATTLEFIELD),
    MSG_BATTLEGROUND_PLAYER_POSITIONS(MSG_BATTLEGROUND_PLAYER_POSITIONS_Client),
    CMSG_BATTLEMASTER_JOIN(CMSG_BATTLEMASTER_JOIN),
    CMSG_GUILD_INFO_TEXT(CMSG_GUILD_INFO_TEXT),
    CMSG_ACTIVATETAXIEXPRESS(CMSG_ACTIVATETAXIEXPRESS),
    CMSG_SET_WATCHED_FACTION(CMSG_SET_WATCHED_FACTION),
    CMSG_GMSURVEY_SUBMIT(CMSG_GMSURVEY_SUBMIT),
    CMSG_MOVE_SET_FLY(CMSG_MOVE_SET_FLY),
    CMSG_REALM_SPLIT(CMSG_REALM_SPLIT),
    CMSG_MOVE_CHNG_TRANSPORT(CMSG_MOVE_CHNG_TRANSPORT),
    CMSG_TIME_SYNC_RESP(CMSG_TIME_SYNC_RESP),
    CMSG_SET_ACTIVE_VOICE_CHANNEL(CMSG_SET_ACTIVE_VOICE_CHANNEL),
    CMSG_WORLD_STATE_UI_TIMER_UPDATE(CMSG_WORLD_STATE_UI_TIMER_UPDATE),
    CMSG_READY_FOR_ACCOUNT_DATA_TIMES(CMSG_READY_FOR_ACCOUNT_DATA_TIMES),
}

impl ClientOpcodeMessage {
    fn read_opcodes(opcode: u32, body_size: u32, mut r: &[u8]) -> std::result::Result<Self, crate::errors::ExpectedOpcodeError> {
        match opcode {
            0x00B5 => Ok(Self::MSG_MOVE_START_FORWARD(<MSG_MOVE_START_FORWARD as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x00B5, size: body_size, io, } } else { a } })?)),
            0x00B6 => Ok(Self::MSG_MOVE_START_BACKWARD(<MSG_MOVE_START_BACKWARD as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x00B6, size: body_size, io, } } else { a } })?)),
            0x00B7 => Ok(Self::MSG_MOVE_STOP(<MSG_MOVE_STOP as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x00B7, size: body_size, io, } } else { a } })?)),
            0x00B8 => Ok(Self::MSG_MOVE_START_STRAFE_LEFT(<MSG_MOVE_START_STRAFE_LEFT as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x00B8, size: body_size, io, } } else { a } })?)),
            0x00B9 => Ok(Self::MSG_MOVE_START_STRAFE_RIGHT(<MSG_MOVE_START_STRAFE_RIGHT as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x00B9, size: body_size, io, } } else { a } })?)),
            0x00BA => Ok(Self::MSG_MOVE_STOP_STRAFE(<MSG_MOVE_STOP_STRAFE as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x00BA, size: body_size, io, } } else { a } })?)),
            0x00BB => Ok(Self::MSG_MOVE_JUMP(<MSG_MOVE_JUMP as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x00BB, size: body_size, io, } } else { a } })?)),
            0x00BC => Ok(Self::MSG_MOVE_START_TURN_LEFT(<MSG_MOVE_START_TURN_LEFT as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x00BC, size: body_size, io, } } else { a } })?)),
            0x00BD => Ok(Self::MSG_MOVE_START_TURN_RIGHT(<MSG_MOVE_START_TURN_RIGHT as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x00BD, size: body_size, io, } } else { a } })?)),
            0x00BE => Ok(Self::MSG_MOVE_STOP_TURN(<MSG_MOVE_STOP_TURN as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x00BE, size: body_size, io, } } else { a } })?)),
            0x00BF => Ok(Self::MSG_MOVE_START_PITCH_UP(<MSG_MOVE_START_PITCH_UP as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x00BF, size: body_size, io, } } else { a } })?)),
            0x00C0 => Ok(Self::MSG_MOVE_START_PITCH_DOWN(<MSG_MOVE_START_PITCH_DOWN as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x00C0, size: body_size, io, } } else { a } })?)),
            0x00C1 => Ok(Self::MSG_MOVE_STOP_PITCH(<MSG_MOVE_STOP_PITCH as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x00C1, size: body_size, io, } } else { a } })?)),
            0x00C2 => Ok(Self::MSG_MOVE_SET_RUN_MODE(<MSG_MOVE_SET_RUN_MODE as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x00C2, size: body_size, io, } } else { a } })?)),
            0x00C3 => Ok(Self::MSG_MOVE_SET_WALK_MODE(<MSG_MOVE_SET_WALK_MODE as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x00C3, size: body_size, io, } } else { a } })?)),
            0x00C9 => Ok(Self::MSG_MOVE_FALL_LAND(<MSG_MOVE_FALL_LAND as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x00C9, size: body_size, io, } } else { a } })?)),
            0x00CA => Ok(Self::MSG_MOVE_START_SWIM(<MSG_MOVE_START_SWIM as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x00CA, size: body_size, io, } } else { a } })?)),
            0x00CB => Ok(Self::MSG_MOVE_STOP_SWIM(<MSG_MOVE_STOP_SWIM as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x00CB, size: body_size, io, } } else { a } })?)),
            0x00DA => Ok(Self::MSG_MOVE_SET_FACING(<MSG_MOVE_SET_FACING as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x00DA, size: body_size, io, } } else { a } })?)),
            0x00DB => Ok(Self::MSG_MOVE_SET_PITCH(<MSG_MOVE_SET_PITCH as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x00DB, size: body_size, io, } } else { a } })?)),
            0x00DC => Ok(Self::MSG_MOVE_WORLDPORT_ACK(<MSG_MOVE_WORLDPORT_ACK as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x00DC, size: body_size, io, } } else { a } })?)),
            0x00EE => Ok(Self::MSG_MOVE_HEARTBEAT(<MSG_MOVE_HEARTBEAT as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x00EE, size: body_size, io, } } else { a } })?)),
            0x01C2 => Ok(Self::MSG_PETITION_DECLINE(<MSG_PETITION_DECLINE as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x01C2, size: body_size, io, } } else { a } })?)),
            0x01F2 => Ok(Self::MSG_TABARDVENDOR_ACTIVATE(<MSG_TABARDVENDOR_ACTIVATE as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x01F2, size: body_size, io, } } else { a } })?)),
            0x02C1 => Ok(Self::MSG_PETITION_RENAME(<MSG_PETITION_RENAME as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x02C1, size: body_size, io, } } else { a } })?)),
            0x0329 => Ok(Self::MSG_SET_DUNGEON_DIFFICULTY(<MSG_SET_DUNGEON_DIFFICULTY as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x0329, size: body_size, io, } } else { a } })?)),
            0x0359 => Ok(Self::MSG_MOVE_START_ASCEND(<MSG_MOVE_START_ASCEND as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x0359, size: body_size, io, } } else { a } })?)),
            0x035A => Ok(Self::MSG_MOVE_STOP_ASCEND(<MSG_MOVE_STOP_ASCEND as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x035A, size: body_size, io, } } else { a } })?)),
            0x03A7 => Ok(Self::MSG_MOVE_START_DESCEND(<MSG_MOVE_START_DESCEND as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x03A7, size: body_size, io, } } else { a } })?)),
            0x0036 => Ok(Self::CMSG_CHAR_CREATE(<CMSG_CHAR_CREATE as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x0036, size: body_size, io, } } else { a } })?)),
            0x0037 => Ok(Self::CMSG_CHAR_ENUM(<CMSG_CHAR_ENUM as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x0037, size: body_size, io, } } else { a } })?)),
            0x0038 => Ok(Self::CMSG_CHAR_DELETE(<CMSG_CHAR_DELETE as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x0038, size: body_size, io, } } else { a } })?)),
            0x003D => Ok(Self::CMSG_PLAYER_LOGIN(<CMSG_PLAYER_LOGIN as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x003D, size: body_size, io, } } else { a } })?)),
            0x004A => Ok(Self::CMSG_PLAYER_LOGOUT(<CMSG_PLAYER_LOGOUT as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x004A, size: body_size, io, } } else { a } })?)),
            0x004B => Ok(Self::CMSG_LOGOUT_REQUEST(<CMSG_LOGOUT_REQUEST as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x004B, size: body_size, io, } } else { a } })?)),
            0x004E => Ok(Self::CMSG_LOGOUT_CANCEL(<CMSG_LOGOUT_CANCEL as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x004E, size: body_size, io, } } else { a } })?)),
            0x0050 => Ok(Self::CMSG_NAME_QUERY(<CMSG_NAME_QUERY as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x0050, size: body_size, io, } } else { a } })?)),
            0x0081 => Ok(Self::CMSG_GUILD_CREATE(<CMSG_GUILD_CREATE as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x0081, size: body_size, io, } } else { a } })?)),
            0x0082 => Ok(Self::CMSG_GUILD_INVITE(<CMSG_GUILD_INVITE as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x0082, size: body_size, io, } } else { a } })?)),
            0x0084 => Ok(Self::CMSG_GUILD_ACCEPT(<CMSG_GUILD_ACCEPT as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x0084, size: body_size, io, } } else { a } })?)),
            0x0085 => Ok(Self::CMSG_GUILD_DECLINE(<CMSG_GUILD_DECLINE as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x0085, size: body_size, io, } } else { a } })?)),
            0x0087 => Ok(Self::CMSG_GUILD_INFO(<CMSG_GUILD_INFO as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x0087, size: body_size, io, } } else { a } })?)),
            0x0089 => Ok(Self::CMSG_GUILD_ROSTER(<CMSG_GUILD_ROSTER as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x0089, size: body_size, io, } } else { a } })?)),
            0x008B => Ok(Self::CMSG_GUILD_PROMOTE(<CMSG_GUILD_PROMOTE as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x008B, size: body_size, io, } } else { a } })?)),
            0x008C => Ok(Self::CMSG_GUILD_DEMOTE(<CMSG_GUILD_DEMOTE as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x008C, size: body_size, io, } } else { a } })?)),
            0x008D => Ok(Self::CMSG_GUILD_LEAVE(<CMSG_GUILD_LEAVE as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x008D, size: body_size, io, } } else { a } })?)),
            0x008E => Ok(Self::CMSG_GUILD_REMOVE(<CMSG_GUILD_REMOVE as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x008E, size: body_size, io, } } else { a } })?)),
            0x008F => Ok(Self::CMSG_GUILD_DISBAND(<CMSG_GUILD_DISBAND as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x008F, size: body_size, io, } } else { a } })?)),
            0x0090 => Ok(Self::CMSG_GUILD_LEADER(<CMSG_GUILD_LEADER as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x0090, size: body_size, io, } } else { a } })?)),
            0x0091 => Ok(Self::CMSG_GUILD_MOTD(<CMSG_GUILD_MOTD as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x0091, size: body_size, io, } } else { a } })?)),
            0x0095 => Ok(Self::CMSG_MESSAGECHAT(<CMSG_MESSAGECHAT as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x0095, size: body_size, io, } } else { a } })?)),
            0x0097 => Ok(Self::CMSG_JOIN_CHANNEL(<CMSG_JOIN_CHANNEL as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x0097, size: body_size, io, } } else { a } })?)),
            0x0098 => Ok(Self::CMSG_LEAVE_CHANNEL(<CMSG_LEAVE_CHANNEL as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x0098, size: body_size, io, } } else { a } })?)),
            0x009A => Ok(Self::CMSG_CHANNEL_LIST(<CMSG_CHANNEL_LIST as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x009A, size: body_size, io, } } else { a } })?)),
            0x009C => Ok(Self::CMSG_CHANNEL_PASSWORD(<CMSG_CHANNEL_PASSWORD as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x009C, size: body_size, io, } } else { a } })?)),
            0x009D => Ok(Self::CMSG_CHANNEL_SET_OWNER(<CMSG_CHANNEL_SET_OWNER as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x009D, size: body_size, io, } } else { a } })?)),
            0x009E => Ok(Self::CMSG_CHANNEL_OWNER(<CMSG_CHANNEL_OWNER as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x009E, size: body_size, io, } } else { a } })?)),
            0x009F => Ok(Self::CMSG_CHANNEL_MODERATOR(<CMSG_CHANNEL_MODERATOR as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x009F, size: body_size, io, } } else { a } })?)),
            0x00A0 => Ok(Self::CMSG_CHANNEL_UNMODERATOR(<CMSG_CHANNEL_UNMODERATOR as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x00A0, size: body_size, io, } } else { a } })?)),
            0x00A1 => Ok(Self::CMSG_CHANNEL_MUTE(<CMSG_CHANNEL_MUTE as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x00A1, size: body_size, io, } } else { a } })?)),
            0x00A2 => Ok(Self::CMSG_CHANNEL_UNMUTE(<CMSG_CHANNEL_UNMUTE as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x00A2, size: body_size, io, } } else { a } })?)),
            0x00A3 => Ok(Self::CMSG_CHANNEL_INVITE(<CMSG_CHANNEL_INVITE as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x00A3, size: body_size, io, } } else { a } })?)),
            0x00A4 => Ok(Self::CMSG_CHANNEL_KICK(<CMSG_CHANNEL_KICK as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x00A4, size: body_size, io, } } else { a } })?)),
            0x00A5 => Ok(Self::CMSG_CHANNEL_BAN(<CMSG_CHANNEL_BAN as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x00A5, size: body_size, io, } } else { a } })?)),
            0x00A6 => Ok(Self::CMSG_CHANNEL_UNBAN(<CMSG_CHANNEL_UNBAN as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x00A6, size: body_size, io, } } else { a } })?)),
            0x00A7 => Ok(Self::CMSG_CHANNEL_ANNOUNCEMENTS(<CMSG_CHANNEL_ANNOUNCEMENTS as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x00A7, size: body_size, io, } } else { a } })?)),
            0x00A8 => Ok(Self::CMSG_CHANNEL_MODERATE(<CMSG_CHANNEL_MODERATE as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x00A8, size: body_size, io, } } else { a } })?)),
            0x00AC => Ok(Self::CMSG_OPEN_ITEM(<CMSG_OPEN_ITEM as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x00AC, size: body_size, io, } } else { a } })?)),
            0x00AD => Ok(Self::CMSG_READ_ITEM(<CMSG_READ_ITEM as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x00AD, size: body_size, io, } } else { a } })?)),
            0x00B1 => Ok(Self::CMSG_GAMEOBJ_USE(<CMSG_GAMEOBJ_USE as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x00B1, size: body_size, io, } } else { a } })?)),
            0x00B4 => Ok(Self::CMSG_AREATRIGGER(<CMSG_AREATRIGGER as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x00B4, size: body_size, io, } } else { a } })?)),
            0x00C7 => Ok(Self::MSG_MOVE_TELEPORT_ACK(<MSG_MOVE_TELEPORT_ACK_Client as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x00C7, size: body_size, io, } } else { a } })?)),
            0x00E3 => Ok(Self::CMSG_FORCE_RUN_SPEED_CHANGE_ACK(<CMSG_FORCE_RUN_SPEED_CHANGE_ACK as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x00E3, size: body_size, io, } } else { a } })?)),
            0x00E5 => Ok(Self::CMSG_FORCE_RUN_BACK_SPEED_CHANGE_ACK(<CMSG_FORCE_RUN_BACK_SPEED_CHANGE_ACK as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x00E5, size: body_size, io, } } else { a } })?)),
            0x00E7 => Ok(Self::CMSG_FORCE_SWIM_SPEED_CHANGE_ACK(<CMSG_FORCE_SWIM_SPEED_CHANGE_ACK as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x00E7, size: body_size, io, } } else { a } })?)),
            0x00E9 => Ok(Self::CMSG_FORCE_MOVE_ROOT_ACK(<CMSG_FORCE_MOVE_ROOT_ACK as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x00E9, size: body_size, io, } } else { a } })?)),
            0x00EB => Ok(Self::CMSG_FORCE_MOVE_UNROOT_ACK(<CMSG_FORCE_MOVE_UNROOT_ACK as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x00EB, size: body_size, io, } } else { a } })?)),
            0x00FB => Ok(Self::CMSG_NEXT_CINEMATIC_CAMERA(<CMSG_NEXT_CINEMATIC_CAMERA as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x00FB, size: body_size, io, } } else { a } })?)),
            0x00FC => Ok(Self::CMSG_COMPLETE_CINEMATIC(<CMSG_COMPLETE_CINEMATIC as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x00FC, size: body_size, io, } } else { a } })?)),
            0x00FE => Ok(Self::CMSG_TUTORIAL_FLAG(<CMSG_TUTORIAL_FLAG as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x00FE, size: body_size, io, } } else { a } })?)),
            0x00FF => Ok(Self::CMSG_TUTORIAL_CLEAR(<CMSG_TUTORIAL_CLEAR as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x00FF, size: body_size, io, } } else { a } })?)),
            0x0100 => Ok(Self::CMSG_TUTORIAL_RESET(<CMSG_TUTORIAL_RESET as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x0100, size: body_size, io, } } else { a } })?)),
            0x0101 => Ok(Self::CMSG_STANDSTATECHANGE(<CMSG_STANDSTATECHANGE as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x0101, size: body_size, io, } } else { a } })?)),
            0x0102 => Ok(Self::CMSG_EMOTE(<CMSG_EMOTE as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x0102, size: body_size, io, } } else { a } })?)),
            0x0104 => Ok(Self::CMSG_TEXT_EMOTE(<CMSG_TEXT_EMOTE as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x0104, size: body_size, io, } } else { a } })?)),
            0x0108 => Ok(Self::CMSG_AUTOSTORE_LOOT_ITEM(<CMSG_AUTOSTORE_LOOT_ITEM as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x0108, size: body_size, io, } } else { a } })?)),
            0x010A => Ok(Self::CMSG_AUTOEQUIP_ITEM(<CMSG_AUTOEQUIP_ITEM as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x010A, size: body_size, io, } } else { a } })?)),
            0x010B => Ok(Self::CMSG_AUTOSTORE_BAG_ITEM(<CMSG_AUTOSTORE_BAG_ITEM as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x010B, size: body_size, io, } } else { a } })?)),
            0x010C => Ok(Self::CMSG_SWAP_ITEM(<CMSG_SWAP_ITEM as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x010C, size: body_size, io, } } else { a } })?)),
            0x010D => Ok(Self::CMSG_SWAP_INV_ITEM(<CMSG_SWAP_INV_ITEM as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x010D, size: body_size, io, } } else { a } })?)),
            0x010E => Ok(Self::CMSG_SPLIT_ITEM(<CMSG_SPLIT_ITEM as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x010E, size: body_size, io, } } else { a } })?)),
            0x010F => Ok(Self::CMSG_AUTOEQUIP_ITEM_SLOT(<CMSG_AUTOEQUIP_ITEM_SLOT as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x010F, size: body_size, io, } } else { a } })?)),
            0x0111 => Ok(Self::CMSG_DESTROYITEM(<CMSG_DESTROYITEM as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x0111, size: body_size, io, } } else { a } })?)),
            0x0114 => Ok(Self::CMSG_INSPECT(<CMSG_INSPECT as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x0114, size: body_size, io, } } else { a } })?)),
            0x0125 => Ok(Self::CMSG_SET_FACTION_ATWAR(<CMSG_SET_FACTION_ATWAR as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x0125, size: body_size, io, } } else { a } })?)),
            0x0128 => Ok(Self::CMSG_SET_ACTION_BUTTON(<CMSG_SET_ACTION_BUTTON as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x0128, size: body_size, io, } } else { a } })?)),
            0x013B => Ok(Self::CMSG_CANCEL_CHANNELLING(<CMSG_CANCEL_CHANNELLING as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x013B, size: body_size, io, } } else { a } })?)),
            0x013D => Ok(Self::CMSG_SET_SELECTION(<CMSG_SET_SELECTION as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x013D, size: body_size, io, } } else { a } })?)),
            0x013E => Ok(Self::CMSG_SET_TARGET_OBSOLETE(<CMSG_SET_TARGET_OBSOLETE as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x013E, size: body_size, io, } } else { a } })?)),
            0x0141 => Ok(Self::CMSG_ATTACKSWING(<CMSG_ATTACKSWING as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x0141, size: body_size, io, } } else { a } })?)),
            0x0142 => Ok(Self::CMSG_ATTACKSTOP(<CMSG_ATTACKSTOP as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x0142, size: body_size, io, } } else { a } })?)),
            0x015A => Ok(Self::CMSG_REPOP_REQUEST(<CMSG_REPOP_REQUEST as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x015A, size: body_size, io, } } else { a } })?)),
            0x015D => Ok(Self::CMSG_LOOT(<CMSG_LOOT as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x015D, size: body_size, io, } } else { a } })?)),
            0x015E => Ok(Self::CMSG_LOOT_MONEY(<CMSG_LOOT_MONEY as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x015E, size: body_size, io, } } else { a } })?)),
            0x015F => Ok(Self::CMSG_LOOT_RELEASE(<CMSG_LOOT_RELEASE as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x015F, size: body_size, io, } } else { a } })?)),
            0x016C => Ok(Self::CMSG_DUEL_ACCEPTED(<CMSG_DUEL_ACCEPTED as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x016C, size: body_size, io, } } else { a } })?)),
            0x016D => Ok(Self::CMSG_DUEL_CANCELLED(<CMSG_DUEL_CANCELLED as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x016D, size: body_size, io, } } else { a } })?)),
            0x0171 => Ok(Self::CMSG_MOUNTSPECIAL_ANIM(<CMSG_MOUNTSPECIAL_ANIM as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x0171, size: body_size, io, } } else { a } })?)),
            0x0175 => Ok(Self::CMSG_PET_ACTION(<CMSG_PET_ACTION as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x0175, size: body_size, io, } } else { a } })?)),
            0x0176 => Ok(Self::CMSG_PET_ABANDON(<CMSG_PET_ABANDON as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x0176, size: body_size, io, } } else { a } })?)),
            0x017B => Ok(Self::CMSG_GOSSIP_HELLO(<CMSG_GOSSIP_HELLO as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x017B, size: body_size, io, } } else { a } })?)),
            0x017C => Ok(Self::CMSG_GOSSIP_SELECT_OPTION(<CMSG_GOSSIP_SELECT_OPTION as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x017C, size: body_size, io, } } else { a } })?)),
            0x019E => Ok(Self::CMSG_LIST_INVENTORY(<CMSG_LIST_INVENTORY as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x019E, size: body_size, io, } } else { a } })?)),
            0x01A0 => Ok(Self::CMSG_SELL_ITEM(<CMSG_SELL_ITEM as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x01A0, size: body_size, io, } } else { a } })?)),
            0x01A2 => Ok(Self::CMSG_BUY_ITEM(<CMSG_BUY_ITEM as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x01A2, size: body_size, io, } } else { a } })?)),
            0x01A3 => Ok(Self::CMSG_BUY_ITEM_IN_SLOT(<CMSG_BUY_ITEM_IN_SLOT as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x01A3, size: body_size, io, } } else { a } })?)),
            0x01AD => Ok(Self::CMSG_ACTIVATETAXI(<CMSG_ACTIVATETAXI as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x01AD, size: body_size, io, } } else { a } })?)),
            0x01B5 => Ok(Self::CMSG_BINDER_ACTIVATE(<CMSG_BINDER_ACTIVATE as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x01B5, size: body_size, io, } } else { a } })?)),
            0x01B7 => Ok(Self::CMSG_BANKER_ACTIVATE(<CMSG_BANKER_ACTIVATE as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x01B7, size: body_size, io, } } else { a } })?)),
            0x01B9 => Ok(Self::CMSG_BUY_BANK_SLOT(<CMSG_BUY_BANK_SLOT as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x01B9, size: body_size, io, } } else { a } })?)),
            0x01BB => Ok(Self::CMSG_PETITION_SHOWLIST(<CMSG_PETITION_SHOWLIST as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x01BB, size: body_size, io, } } else { a } })?)),
            0x01BD => Ok(Self::CMSG_PETITION_BUY(<CMSG_PETITION_BUY as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x01BD, size: body_size, io, } } else { a } })?)),
            0x01BE => Ok(Self::CMSG_PETITION_SHOW_SIGNATURES(<CMSG_PETITION_SHOW_SIGNATURES as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x01BE, size: body_size, io, } } else { a } })?)),
            0x01C0 => Ok(Self::CMSG_PETITION_SIGN(<CMSG_PETITION_SIGN as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x01C0, size: body_size, io, } } else { a } })?)),
            0x01C3 => Ok(Self::CMSG_OFFER_PETITION(<CMSG_OFFER_PETITION as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x01C3, size: body_size, io, } } else { a } })?)),
            0x01C4 => Ok(Self::CMSG_TURN_IN_PETITION(<CMSG_TURN_IN_PETITION as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x01C4, size: body_size, io, } } else { a } })?)),
            0x01CA => Ok(Self::CMSG_BUG(<CMSG_BUG as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x01CA, size: body_size, io, } } else { a } })?)),
            0x01CC => Ok(Self::CMSG_PLAYED_TIME(<CMSG_PLAYED_TIME as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x01CC, size: body_size, io, } } else { a } })?)),
            0x01CE => Ok(Self::CMSG_QUERY_TIME(<CMSG_QUERY_TIME as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x01CE, size: body_size, io, } } else { a } })?)),
            0x01D3 => Ok(Self::CMSG_WRAP_ITEM(<CMSG_WRAP_ITEM as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x01D3, size: body_size, io, } } else { a } })?)),
            0x01DC => Ok(Self::CMSG_PING(<CMSG_PING as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x01DC, size: body_size, io, } } else { a } })?)),
            0x01E0 => Ok(Self::CMSG_SETSHEATHED(<CMSG_SETSHEATHED as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x01E0, size: body_size, io, } } else { a } })?)),
            0x01ED => Ok(Self::CMSG_AUTH_SESSION(<CMSG_AUTH_SESSION as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x01ED, size: body_size, io, } } else { a } })?)),
            0x01F1 => Ok(Self::MSG_SAVE_GUILD_EMBLEM(<MSG_SAVE_GUILD_EMBLEM_Client as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x01F1, size: body_size, io, } } else { a } })?)),
            0x01F4 => Ok(Self::CMSG_ZONEUPDATE(<CMSG_ZONEUPDATE as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x01F4, size: body_size, io, } } else { a } })?)),
            0x01FB => Ok(Self::MSG_RANDOM_ROLL(<MSG_RANDOM_ROLL_Client as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x01FB, size: body_size, io, } } else { a } })?)),
            0x0207 => Ok(Self::CMSG_GMTICKET_UPDATETEXT(<CMSG_GMTICKET_UPDATETEXT as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x0207, size: body_size, io, } } else { a } })?)),
            0x020A => Ok(Self::CMSG_REQUEST_ACCOUNT_DATA(<CMSG_REQUEST_ACCOUNT_DATA as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x020A, size: body_size, io, } } else { a } })?)),
            0x020B => Ok(Self::CMSG_UPDATE_ACCOUNT_DATA(<CMSG_UPDATE_ACCOUNT_DATA as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x020B, size: body_size, io, } } else { a } })?)),
            0x0211 => Ok(Self::CMSG_GMTICKET_GETTICKET(<CMSG_GMTICKET_GETTICKET as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x0211, size: body_size, io, } } else { a } })?)),
            0x0217 => Ok(Self::CMSG_GMTICKET_DELETETICKET(<CMSG_GMTICKET_DELETETICKET as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x0217, size: body_size, io, } } else { a } })?)),
            0x021A => Ok(Self::CMSG_GMTICKET_SYSTEMSTATUS(<CMSG_GMTICKET_SYSTEMSTATUS as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x021A, size: body_size, io, } } else { a } })?)),
            0x0225 => Ok(Self::CMSG_CHAT_IGNORED(<CMSG_CHAT_IGNORED as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x0225, size: body_size, io, } } else { a } })?)),
            0x0231 => Ok(Self::CMSG_GUILD_RANK(<CMSG_GUILD_RANK as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x0231, size: body_size, io, } } else { a } })?)),
            0x0232 => Ok(Self::CMSG_GUILD_ADD_RANK(<CMSG_GUILD_ADD_RANK as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x0232, size: body_size, io, } } else { a } })?)),
            0x0233 => Ok(Self::CMSG_GUILD_DEL_RANK(<CMSG_GUILD_DEL_RANK as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x0233, size: body_size, io, } } else { a } })?)),
            0x0234 => Ok(Self::CMSG_GUILD_SET_PUBLIC_NOTE(<CMSG_GUILD_SET_PUBLIC_NOTE as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x0234, size: body_size, io, } } else { a } })?)),
            0x0235 => Ok(Self::CMSG_GUILD_SET_OFFICER_NOTE(<CMSG_GUILD_SET_OFFICER_NOTE as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x0235, size: body_size, io, } } else { a } })?)),
            0x0238 => Ok(Self::CMSG_SEND_MAIL(<CMSG_SEND_MAIL as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x0238, size: body_size, io, } } else { a } })?)),
            0x023A => Ok(Self::CMSG_GET_MAIL_LIST(<CMSG_GET_MAIL_LIST as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x023A, size: body_size, io, } } else { a } })?)),
            0x023C => Ok(Self::CMSG_BATTLEFIELD_LIST(<CMSG_BATTLEFIELD_LIST as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x023C, size: body_size, io, } } else { a } })?)),
            0x0245 => Ok(Self::CMSG_MAIL_TAKE_MONEY(<CMSG_MAIL_TAKE_MONEY as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x0245, size: body_size, io, } } else { a } })?)),
            0x0246 => Ok(Self::CMSG_MAIL_TAKE_ITEM(<CMSG_MAIL_TAKE_ITEM as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x0246, size: body_size, io, } } else { a } })?)),
            0x0247 => Ok(Self::CMSG_MAIL_MARK_AS_READ(<CMSG_MAIL_MARK_AS_READ as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x0247, size: body_size, io, } } else { a } })?)),
            0x0248 => Ok(Self::CMSG_MAIL_RETURN_TO_SENDER(<CMSG_MAIL_RETURN_TO_SENDER as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x0248, size: body_size, io, } } else { a } })?)),
            0x0249 => Ok(Self::CMSG_MAIL_DELETE(<CMSG_MAIL_DELETE as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x0249, size: body_size, io, } } else { a } })?)),
            0x024A => Ok(Self::CMSG_MAIL_CREATE_TEXT_ITEM(<CMSG_MAIL_CREATE_TEXT_ITEM as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x024A, size: body_size, io, } } else { a } })?)),
            0x0255 => Ok(Self::MSG_AUCTION_HELLO(<MSG_AUCTION_HELLO_Client as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x0255, size: body_size, io, } } else { a } })?)),
            0x0256 => Ok(Self::CMSG_AUCTION_SELL_ITEM(<CMSG_AUCTION_SELL_ITEM as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x0256, size: body_size, io, } } else { a } })?)),
            0x0257 => Ok(Self::CMSG_AUCTION_REMOVE_ITEM(<CMSG_AUCTION_REMOVE_ITEM as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x0257, size: body_size, io, } } else { a } })?)),
            0x0258 => Ok(Self::CMSG_AUCTION_LIST_ITEMS(<CMSG_AUCTION_LIST_ITEMS as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x0258, size: body_size, io, } } else { a } })?)),
            0x0259 => Ok(Self::CMSG_AUCTION_LIST_OWNER_ITEMS(<CMSG_AUCTION_LIST_OWNER_ITEMS as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x0259, size: body_size, io, } } else { a } })?)),
            0x025A => Ok(Self::CMSG_AUCTION_PLACE_BID(<CMSG_AUCTION_PLACE_BID as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x025A, size: body_size, io, } } else { a } })?)),
            0x0264 => Ok(Self::CMSG_AUCTION_LIST_BIDDER_ITEMS(<CMSG_AUCTION_LIST_BIDDER_ITEMS as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x0264, size: body_size, io, } } else { a } })?)),
            0x0268 => Ok(Self::CMSG_SET_AMMO(<CMSG_SET_AMMO as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x0268, size: body_size, io, } } else { a } })?)),
            0x026A => Ok(Self::CMSG_SET_ACTIVE_MOVER(<CMSG_SET_ACTIVE_MOVER as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x026A, size: body_size, io, } } else { a } })?)),
            0x0272 => Ok(Self::CMSG_BUY_STABLE_SLOT(<CMSG_BUY_STABLE_SLOT as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x0272, size: body_size, io, } } else { a } })?)),
            0x0282 => Ok(Self::CMSG_AUTOSTORE_BANK_ITEM(<CMSG_AUTOSTORE_BANK_ITEM as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x0282, size: body_size, io, } } else { a } })?)),
            0x0283 => Ok(Self::CMSG_AUTOBANK_ITEM(<CMSG_AUTOBANK_ITEM as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x0283, size: body_size, io, } } else { a } })?)),
            0x0290 => Ok(Self::CMSG_BUYBACK_ITEM(<CMSG_BUYBACK_ITEM as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x0290, size: body_size, io, } } else { a } })?)),
            0x02A0 => Ok(Self::CMSG_LOOT_ROLL(<CMSG_LOOT_ROLL as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x02A0, size: body_size, io, } } else { a } })?)),
            0x02A3 => Ok(Self::CMSG_LOOT_MASTER_GIVE(<CMSG_LOOT_MASTER_GIVE as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x02A3, size: body_size, io, } } else { a } })?)),
            0x02A8 => Ok(Self::CMSG_REPAIR_ITEM(<CMSG_REPAIR_ITEM as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x02A8, size: body_size, io, } } else { a } })?)),
            0x02B9 => Ok(Self::CMSG_TOGGLE_HELM(<CMSG_TOGGLE_HELM as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x02B9, size: body_size, io, } } else { a } })?)),
            0x02BA => Ok(Self::CMSG_TOGGLE_CLOAK(<CMSG_TOGGLE_CLOAK as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x02BA, size: body_size, io, } } else { a } })?)),
            0x02BF => Ok(Self::CMSG_SET_ACTIONBAR_TOGGLES(<CMSG_SET_ACTIONBAR_TOGGLES as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x02BF, size: body_size, io, } } else { a } })?)),
            0x02C7 => Ok(Self::CMSG_CHAR_RENAME(<CMSG_CHAR_RENAME as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x02C7, size: body_size, io, } } else { a } })?)),
            0x02CA => Ok(Self::CMSG_MOVE_FALL_RESET(<CMSG_MOVE_FALL_RESET as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x02CA, size: body_size, io, } } else { a } })?)),
            0x02D3 => Ok(Self::CMSG_BATTLEFIELD_STATUS(<CMSG_BATTLEFIELD_STATUS as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x02D3, size: body_size, io, } } else { a } })?)),
            0x02D5 => Ok(Self::CMSG_BATTLEFIELD_PORT(<CMSG_BATTLEFIELD_PORT as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x02D5, size: body_size, io, } } else { a } })?)),
            0x02D7 => Ok(Self::CMSG_BATTLEMASTER_HELLO(<CMSG_BATTLEMASTER_HELLO as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x02D7, size: body_size, io, } } else { a } })?)),
            0x02DB => Ok(Self::CMSG_FORCE_WALK_SPEED_CHANGE_ACK(<CMSG_FORCE_WALK_SPEED_CHANGE_ACK as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x02DB, size: body_size, io, } } else { a } })?)),
            0x02DD => Ok(Self::CMSG_FORCE_SWIM_BACK_SPEED_CHANGE_ACK(<CMSG_FORCE_SWIM_BACK_SPEED_CHANGE_ACK as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x02DD, size: body_size, io, } } else { a } })?)),
            0x02DF => Ok(Self::CMSG_FORCE_TURN_RATE_CHANGE_ACK(<CMSG_FORCE_TURN_RATE_CHANGE_ACK as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x02DF, size: body_size, io, } } else { a } })?)),
            0x02E1 => Ok(Self::CMSG_LEAVE_BATTLEFIELD(<CMSG_LEAVE_BATTLEFIELD as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x02E1, size: body_size, io, } } else { a } })?)),
            0x02E9 => Ok(Self::MSG_BATTLEGROUND_PLAYER_POSITIONS(<MSG_BATTLEGROUND_PLAYER_POSITIONS_Client as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x02E9, size: body_size, io, } } else { a } })?)),
            0x02EE => Ok(Self::CMSG_BATTLEMASTER_JOIN(<CMSG_BATTLEMASTER_JOIN as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x02EE, size: body_size, io, } } else { a } })?)),
            0x02FC => Ok(Self::CMSG_GUILD_INFO_TEXT(<CMSG_GUILD_INFO_TEXT as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x02FC, size: body_size, io, } } else { a } })?)),
            0x0312 => Ok(Self::CMSG_ACTIVATETAXIEXPRESS(<CMSG_ACTIVATETAXIEXPRESS as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x0312, size: body_size, io, } } else { a } })?)),
            0x0318 => Ok(Self::CMSG_SET_WATCHED_FACTION(<CMSG_SET_WATCHED_FACTION as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x0318, size: body_size, io, } } else { a } })?)),
            0x032A => Ok(Self::CMSG_GMSURVEY_SUBMIT(<CMSG_GMSURVEY_SUBMIT as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x032A, size: body_size, io, } } else { a } })?)),
            0x0346 => Ok(Self::CMSG_MOVE_SET_FLY(<CMSG_MOVE_SET_FLY as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x0346, size: body_size, io, } } else { a } })?)),
            0x038C => Ok(Self::CMSG_REALM_SPLIT(<CMSG_REALM_SPLIT as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x038C, size: body_size, io, } } else { a } })?)),
            0x038D => Ok(Self::CMSG_MOVE_CHNG_TRANSPORT(<CMSG_MOVE_CHNG_TRANSPORT as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x038D, size: body_size, io, } } else { a } })?)),
            0x0391 => Ok(Self::CMSG_TIME_SYNC_RESP(<CMSG_TIME_SYNC_RESP as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x0391, size: body_size, io, } } else { a } })?)),
            0x03D3 => Ok(Self::CMSG_SET_ACTIVE_VOICE_CHANNEL(<CMSG_SET_ACTIVE_VOICE_CHANNEL as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x03D3, size: body_size, io, } } else { a } })?)),
            0x04F6 => Ok(Self::CMSG_WORLD_STATE_UI_TIMER_UPDATE(<CMSG_WORLD_STATE_UI_TIMER_UPDATE as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x04F6, size: body_size, io, } } else { a } })?)),
            0x04FF => Ok(Self::CMSG_READY_FOR_ACCOUNT_DATA_TIMES(<CMSG_READY_FOR_ACCOUNT_DATA_TIMES as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x04FF, size: body_size, io, } } else { a } })?)),
            _ => Err(crate::errors::ExpectedOpcodeError::Opcode{ opcode, size: body_size }),
        }
    }

    #[cfg(feature = "sync")]
    pub fn read_unencrypted<R: std::io::Read>(r: &mut R) -> std::result::Result<Self, crate::errors::ExpectedOpcodeError> {
        let size = (crate::util::read_u16_be(r)?.saturating_sub(4)) as u32;
        let opcode = crate::util::read_u32_le(r)?;

        let mut buf = vec![0; size as usize];
        r.read_exact(&mut buf)?;
        Self::read_opcodes(opcode, size, &buf)
    }
    #[cfg(all(feature = "sync", feature = "encryption"))]
    pub fn read_encrypted<R: std::io::Read>(r: &mut R, d: &mut ServerDecrypterHalf) -> std::result::Result<Self, crate::errors::ExpectedOpcodeError> {
        let mut header = [0_u8; 6];
        r.read_exact(&mut header)?;
        let header = d.decrypt_client_header(header);
        let opcode = header.opcode;
        let body_size = (header.size.saturating_sub(4)) as u32;

        let mut buf = vec![0; body_size as usize];
        r.read_exact(&mut buf)?;
        Self::read_opcodes(opcode, body_size, &buf)
    }

    #[cfg(feature = "tokio")]
    pub async fn tokio_read_unencrypted<R: tokio::io::AsyncReadExt + Unpin + Send>(r: &mut R) -> std::result::Result<Self, crate::errors::ExpectedOpcodeError> {
        let size = (crate::util::tokio_read_u16_be(r).await?.saturating_sub(4)) as u32;
        let opcode = crate::util::tokio_read_u32_le(r).await?;

        let mut buf = vec![0; size as usize];
        r.read_exact(&mut buf).await?;
        Self::read_opcodes(opcode, size, &buf)
    }
    #[cfg(all(feature = "tokio", feature = "encryption"))]
    pub async fn tokio_read_encrypted<R: tokio::io::AsyncReadExt + Unpin + Send>(r: &mut R, d: &mut ServerDecrypterHalf) -> std::result::Result<Self, crate::errors::ExpectedOpcodeError> {
        let mut header = [0_u8; 6];
        r.read_exact(&mut header).await?;
        let header = d.decrypt_client_header(header);
        let opcode = header.opcode;
        let body_size = (header.size.saturating_sub(4)) as u32;

        let mut buf = vec![0; body_size as usize];
        r.read_exact(&mut buf).await?;
        Self::read_opcodes(opcode, body_size, &buf)
    }

    #[cfg(feature = "async-std")]
    pub async fn astd_read_unencrypted<R: async_std::io::ReadExt + Unpin + Send>(r: &mut R) -> std::result::Result<Self, crate::errors::ExpectedOpcodeError> {
        let size = (crate::util::astd_read_u16_be(r).await?.saturating_sub(4)) as u32;
        let opcode = crate::util::astd_read_u32_le(r).await?;

        let mut buf = vec![0; size as usize];
        r.read_exact(&mut buf).await?;
        Self::read_opcodes(opcode, size, &buf)
    }
    #[cfg(all(feature = "async-std", feature = "encryption"))]
    pub async fn astd_read_encrypted<R: async_std::io::ReadExt + Unpin + Send>(r: &mut R, d: &mut ServerDecrypterHalf) -> std::result::Result<Self, crate::errors::ExpectedOpcodeError> {
        let mut header = [0_u8; 6];
        r.read_exact(&mut header).await?;
        let header = d.decrypt_client_header(header);
        let opcode = header.opcode;
        let body_size = (header.size.saturating_sub(4)) as u32;

        let mut buf = vec![0; body_size as usize];
        r.read_exact(&mut buf).await?;
        Self::read_opcodes(opcode, body_size, &buf)
    }

    #[cfg(all(feature = "sync", feature = "encryption"))]
    pub fn write_encrypted_client<W: std::io::Write>(&self, w: &mut W, e: &mut ClientEncrypterHalf) -> Result<(), std::io::Error> {
        match self {
            Self::MSG_MOVE_START_FORWARD(c) => c.write_encrypted_client(w, e),
            Self::MSG_MOVE_START_BACKWARD(c) => c.write_encrypted_client(w, e),
            Self::MSG_MOVE_STOP(c) => c.write_encrypted_client(w, e),
            Self::MSG_MOVE_START_STRAFE_LEFT(c) => c.write_encrypted_client(w, e),
            Self::MSG_MOVE_START_STRAFE_RIGHT(c) => c.write_encrypted_client(w, e),
            Self::MSG_MOVE_STOP_STRAFE(c) => c.write_encrypted_client(w, e),
            Self::MSG_MOVE_JUMP(c) => c.write_encrypted_client(w, e),
            Self::MSG_MOVE_START_TURN_LEFT(c) => c.write_encrypted_client(w, e),
            Self::MSG_MOVE_START_TURN_RIGHT(c) => c.write_encrypted_client(w, e),
            Self::MSG_MOVE_STOP_TURN(c) => c.write_encrypted_client(w, e),
            Self::MSG_MOVE_START_PITCH_UP(c) => c.write_encrypted_client(w, e),
            Self::MSG_MOVE_START_PITCH_DOWN(c) => c.write_encrypted_client(w, e),
            Self::MSG_MOVE_STOP_PITCH(c) => c.write_encrypted_client(w, e),
            Self::MSG_MOVE_SET_RUN_MODE(c) => c.write_encrypted_client(w, e),
            Self::MSG_MOVE_SET_WALK_MODE(c) => c.write_encrypted_client(w, e),
            Self::MSG_MOVE_FALL_LAND(c) => c.write_encrypted_client(w, e),
            Self::MSG_MOVE_START_SWIM(c) => c.write_encrypted_client(w, e),
            Self::MSG_MOVE_STOP_SWIM(c) => c.write_encrypted_client(w, e),
            Self::MSG_MOVE_SET_FACING(c) => c.write_encrypted_client(w, e),
            Self::MSG_MOVE_SET_PITCH(c) => c.write_encrypted_client(w, e),
            Self::MSG_MOVE_WORLDPORT_ACK(c) => c.write_encrypted_client(w, e),
            Self::MSG_MOVE_HEARTBEAT(c) => c.write_encrypted_client(w, e),
            Self::MSG_PETITION_DECLINE(c) => c.write_encrypted_client(w, e),
            Self::MSG_TABARDVENDOR_ACTIVATE(c) => c.write_encrypted_client(w, e),
            Self::MSG_PETITION_RENAME(c) => c.write_encrypted_client(w, e),
            Self::MSG_SET_DUNGEON_DIFFICULTY(c) => c.write_encrypted_client(w, e),
            Self::MSG_MOVE_START_ASCEND(c) => c.write_encrypted_client(w, e),
            Self::MSG_MOVE_STOP_ASCEND(c) => c.write_encrypted_client(w, e),
            Self::MSG_MOVE_START_DESCEND(c) => c.write_encrypted_client(w, e),
            Self::CMSG_CHAR_CREATE(c) => c.write_encrypted_client(w, e),
            Self::CMSG_CHAR_ENUM(c) => c.write_encrypted_client(w, e),
            Self::CMSG_CHAR_DELETE(c) => c.write_encrypted_client(w, e),
            Self::CMSG_PLAYER_LOGIN(c) => c.write_encrypted_client(w, e),
            Self::CMSG_PLAYER_LOGOUT(c) => c.write_encrypted_client(w, e),
            Self::CMSG_LOGOUT_REQUEST(c) => c.write_encrypted_client(w, e),
            Self::CMSG_LOGOUT_CANCEL(c) => c.write_encrypted_client(w, e),
            Self::CMSG_NAME_QUERY(c) => c.write_encrypted_client(w, e),
            Self::CMSG_GUILD_CREATE(c) => c.write_encrypted_client(w, e),
            Self::CMSG_GUILD_INVITE(c) => c.write_encrypted_client(w, e),
            Self::CMSG_GUILD_ACCEPT(c) => c.write_encrypted_client(w, e),
            Self::CMSG_GUILD_DECLINE(c) => c.write_encrypted_client(w, e),
            Self::CMSG_GUILD_INFO(c) => c.write_encrypted_client(w, e),
            Self::CMSG_GUILD_ROSTER(c) => c.write_encrypted_client(w, e),
            Self::CMSG_GUILD_PROMOTE(c) => c.write_encrypted_client(w, e),
            Self::CMSG_GUILD_DEMOTE(c) => c.write_encrypted_client(w, e),
            Self::CMSG_GUILD_LEAVE(c) => c.write_encrypted_client(w, e),
            Self::CMSG_GUILD_REMOVE(c) => c.write_encrypted_client(w, e),
            Self::CMSG_GUILD_DISBAND(c) => c.write_encrypted_client(w, e),
            Self::CMSG_GUILD_LEADER(c) => c.write_encrypted_client(w, e),
            Self::CMSG_GUILD_MOTD(c) => c.write_encrypted_client(w, e),
            Self::CMSG_MESSAGECHAT(c) => c.write_encrypted_client(w, e),
            Self::CMSG_JOIN_CHANNEL(c) => c.write_encrypted_client(w, e),
            Self::CMSG_LEAVE_CHANNEL(c) => c.write_encrypted_client(w, e),
            Self::CMSG_CHANNEL_LIST(c) => c.write_encrypted_client(w, e),
            Self::CMSG_CHANNEL_PASSWORD(c) => c.write_encrypted_client(w, e),
            Self::CMSG_CHANNEL_SET_OWNER(c) => c.write_encrypted_client(w, e),
            Self::CMSG_CHANNEL_OWNER(c) => c.write_encrypted_client(w, e),
            Self::CMSG_CHANNEL_MODERATOR(c) => c.write_encrypted_client(w, e),
            Self::CMSG_CHANNEL_UNMODERATOR(c) => c.write_encrypted_client(w, e),
            Self::CMSG_CHANNEL_MUTE(c) => c.write_encrypted_client(w, e),
            Self::CMSG_CHANNEL_UNMUTE(c) => c.write_encrypted_client(w, e),
            Self::CMSG_CHANNEL_INVITE(c) => c.write_encrypted_client(w, e),
            Self::CMSG_CHANNEL_KICK(c) => c.write_encrypted_client(w, e),
            Self::CMSG_CHANNEL_BAN(c) => c.write_encrypted_client(w, e),
            Self::CMSG_CHANNEL_UNBAN(c) => c.write_encrypted_client(w, e),
            Self::CMSG_CHANNEL_ANNOUNCEMENTS(c) => c.write_encrypted_client(w, e),
            Self::CMSG_CHANNEL_MODERATE(c) => c.write_encrypted_client(w, e),
            Self::CMSG_OPEN_ITEM(c) => c.write_encrypted_client(w, e),
            Self::CMSG_READ_ITEM(c) => c.write_encrypted_client(w, e),
            Self::CMSG_GAMEOBJ_USE(c) => c.write_encrypted_client(w, e),
            Self::CMSG_AREATRIGGER(c) => c.write_encrypted_client(w, e),
            Self::MSG_MOVE_TELEPORT_ACK(c) => c.write_encrypted_client(w, e),
            Self::CMSG_FORCE_RUN_SPEED_CHANGE_ACK(c) => c.write_encrypted_client(w, e),
            Self::CMSG_FORCE_RUN_BACK_SPEED_CHANGE_ACK(c) => c.write_encrypted_client(w, e),
            Self::CMSG_FORCE_SWIM_SPEED_CHANGE_ACK(c) => c.write_encrypted_client(w, e),
            Self::CMSG_FORCE_MOVE_ROOT_ACK(c) => c.write_encrypted_client(w, e),
            Self::CMSG_FORCE_MOVE_UNROOT_ACK(c) => c.write_encrypted_client(w, e),
            Self::CMSG_NEXT_CINEMATIC_CAMERA(c) => c.write_encrypted_client(w, e),
            Self::CMSG_COMPLETE_CINEMATIC(c) => c.write_encrypted_client(w, e),
            Self::CMSG_TUTORIAL_FLAG(c) => c.write_encrypted_client(w, e),
            Self::CMSG_TUTORIAL_CLEAR(c) => c.write_encrypted_client(w, e),
            Self::CMSG_TUTORIAL_RESET(c) => c.write_encrypted_client(w, e),
            Self::CMSG_STANDSTATECHANGE(c) => c.write_encrypted_client(w, e),
            Self::CMSG_EMOTE(c) => c.write_encrypted_client(w, e),
            Self::CMSG_TEXT_EMOTE(c) => c.write_encrypted_client(w, e),
            Self::CMSG_AUTOSTORE_LOOT_ITEM(c) => c.write_encrypted_client(w, e),
            Self::CMSG_AUTOEQUIP_ITEM(c) => c.write_encrypted_client(w, e),
            Self::CMSG_AUTOSTORE_BAG_ITEM(c) => c.write_encrypted_client(w, e),
            Self::CMSG_SWAP_ITEM(c) => c.write_encrypted_client(w, e),
            Self::CMSG_SWAP_INV_ITEM(c) => c.write_encrypted_client(w, e),
            Self::CMSG_SPLIT_ITEM(c) => c.write_encrypted_client(w, e),
            Self::CMSG_AUTOEQUIP_ITEM_SLOT(c) => c.write_encrypted_client(w, e),
            Self::CMSG_DESTROYITEM(c) => c.write_encrypted_client(w, e),
            Self::CMSG_INSPECT(c) => c.write_encrypted_client(w, e),
            Self::CMSG_SET_FACTION_ATWAR(c) => c.write_encrypted_client(w, e),
            Self::CMSG_SET_ACTION_BUTTON(c) => c.write_encrypted_client(w, e),
            Self::CMSG_CANCEL_CHANNELLING(c) => c.write_encrypted_client(w, e),
            Self::CMSG_SET_SELECTION(c) => c.write_encrypted_client(w, e),
            Self::CMSG_SET_TARGET_OBSOLETE(c) => c.write_encrypted_client(w, e),
            Self::CMSG_ATTACKSWING(c) => c.write_encrypted_client(w, e),
            Self::CMSG_ATTACKSTOP(c) => c.write_encrypted_client(w, e),
            Self::CMSG_REPOP_REQUEST(c) => c.write_encrypted_client(w, e),
            Self::CMSG_LOOT(c) => c.write_encrypted_client(w, e),
            Self::CMSG_LOOT_MONEY(c) => c.write_encrypted_client(w, e),
            Self::CMSG_LOOT_RELEASE(c) => c.write_encrypted_client(w, e),
            Self::CMSG_DUEL_ACCEPTED(c) => c.write_encrypted_client(w, e),
            Self::CMSG_DUEL_CANCELLED(c) => c.write_encrypted_client(w, e),
            Self::CMSG_MOUNTSPECIAL_ANIM(c) => c.write_encrypted_client(w, e),
            Self::CMSG_PET_ACTION(c) => c.write_encrypted_client(w, e),
            Self::CMSG_PET_ABANDON(c) => c.write_encrypted_client(w, e),
            Self::CMSG_GOSSIP_HELLO(c) => c.write_encrypted_client(w, e),
            Self::CMSG_GOSSIP_SELECT_OPTION(c) => c.write_encrypted_client(w, e),
            Self::CMSG_LIST_INVENTORY(c) => c.write_encrypted_client(w, e),
            Self::CMSG_SELL_ITEM(c) => c.write_encrypted_client(w, e),
            Self::CMSG_BUY_ITEM(c) => c.write_encrypted_client(w, e),
            Self::CMSG_BUY_ITEM_IN_SLOT(c) => c.write_encrypted_client(w, e),
            Self::CMSG_ACTIVATETAXI(c) => c.write_encrypted_client(w, e),
            Self::CMSG_BINDER_ACTIVATE(c) => c.write_encrypted_client(w, e),
            Self::CMSG_BANKER_ACTIVATE(c) => c.write_encrypted_client(w, e),
            Self::CMSG_BUY_BANK_SLOT(c) => c.write_encrypted_client(w, e),
            Self::CMSG_PETITION_SHOWLIST(c) => c.write_encrypted_client(w, e),
            Self::CMSG_PETITION_BUY(c) => c.write_encrypted_client(w, e),
            Self::CMSG_PETITION_SHOW_SIGNATURES(c) => c.write_encrypted_client(w, e),
            Self::CMSG_PETITION_SIGN(c) => c.write_encrypted_client(w, e),
            Self::CMSG_OFFER_PETITION(c) => c.write_encrypted_client(w, e),
            Self::CMSG_TURN_IN_PETITION(c) => c.write_encrypted_client(w, e),
            Self::CMSG_BUG(c) => c.write_encrypted_client(w, e),
            Self::CMSG_PLAYED_TIME(c) => c.write_encrypted_client(w, e),
            Self::CMSG_QUERY_TIME(c) => c.write_encrypted_client(w, e),
            Self::CMSG_WRAP_ITEM(c) => c.write_encrypted_client(w, e),
            Self::CMSG_PING(c) => c.write_encrypted_client(w, e),
            Self::CMSG_SETSHEATHED(c) => c.write_encrypted_client(w, e),
            Self::CMSG_AUTH_SESSION(c) => c.write_encrypted_client(w, e),
            Self::MSG_SAVE_GUILD_EMBLEM(c) => c.write_encrypted_client(w, e),
            Self::CMSG_ZONEUPDATE(c) => c.write_encrypted_client(w, e),
            Self::MSG_RANDOM_ROLL(c) => c.write_encrypted_client(w, e),
            Self::CMSG_GMTICKET_UPDATETEXT(c) => c.write_encrypted_client(w, e),
            Self::CMSG_REQUEST_ACCOUNT_DATA(c) => c.write_encrypted_client(w, e),
            Self::CMSG_UPDATE_ACCOUNT_DATA(c) => c.write_encrypted_client(w, e),
            Self::CMSG_GMTICKET_GETTICKET(c) => c.write_encrypted_client(w, e),
            Self::CMSG_GMTICKET_DELETETICKET(c) => c.write_encrypted_client(w, e),
            Self::CMSG_GMTICKET_SYSTEMSTATUS(c) => c.write_encrypted_client(w, e),
            Self::CMSG_CHAT_IGNORED(c) => c.write_encrypted_client(w, e),
            Self::CMSG_GUILD_RANK(c) => c.write_encrypted_client(w, e),
            Self::CMSG_GUILD_ADD_RANK(c) => c.write_encrypted_client(w, e),
            Self::CMSG_GUILD_DEL_RANK(c) => c.write_encrypted_client(w, e),
            Self::CMSG_GUILD_SET_PUBLIC_NOTE(c) => c.write_encrypted_client(w, e),
            Self::CMSG_GUILD_SET_OFFICER_NOTE(c) => c.write_encrypted_client(w, e),
            Self::CMSG_SEND_MAIL(c) => c.write_encrypted_client(w, e),
            Self::CMSG_GET_MAIL_LIST(c) => c.write_encrypted_client(w, e),
            Self::CMSG_BATTLEFIELD_LIST(c) => c.write_encrypted_client(w, e),
            Self::CMSG_MAIL_TAKE_MONEY(c) => c.write_encrypted_client(w, e),
            Self::CMSG_MAIL_TAKE_ITEM(c) => c.write_encrypted_client(w, e),
            Self::CMSG_MAIL_MARK_AS_READ(c) => c.write_encrypted_client(w, e),
            Self::CMSG_MAIL_RETURN_TO_SENDER(c) => c.write_encrypted_client(w, e),
            Self::CMSG_MAIL_DELETE(c) => c.write_encrypted_client(w, e),
            Self::CMSG_MAIL_CREATE_TEXT_ITEM(c) => c.write_encrypted_client(w, e),
            Self::MSG_AUCTION_HELLO(c) => c.write_encrypted_client(w, e),
            Self::CMSG_AUCTION_SELL_ITEM(c) => c.write_encrypted_client(w, e),
            Self::CMSG_AUCTION_REMOVE_ITEM(c) => c.write_encrypted_client(w, e),
            Self::CMSG_AUCTION_LIST_ITEMS(c) => c.write_encrypted_client(w, e),
            Self::CMSG_AUCTION_LIST_OWNER_ITEMS(c) => c.write_encrypted_client(w, e),
            Self::CMSG_AUCTION_PLACE_BID(c) => c.write_encrypted_client(w, e),
            Self::CMSG_AUCTION_LIST_BIDDER_ITEMS(c) => c.write_encrypted_client(w, e),
            Self::CMSG_SET_AMMO(c) => c.write_encrypted_client(w, e),
            Self::CMSG_SET_ACTIVE_MOVER(c) => c.write_encrypted_client(w, e),
            Self::CMSG_BUY_STABLE_SLOT(c) => c.write_encrypted_client(w, e),
            Self::CMSG_AUTOSTORE_BANK_ITEM(c) => c.write_encrypted_client(w, e),
            Self::CMSG_AUTOBANK_ITEM(c) => c.write_encrypted_client(w, e),
            Self::CMSG_BUYBACK_ITEM(c) => c.write_encrypted_client(w, e),
            Self::CMSG_LOOT_ROLL(c) => c.write_encrypted_client(w, e),
            Self::CMSG_LOOT_MASTER_GIVE(c) => c.write_encrypted_client(w, e),
            Self::CMSG_REPAIR_ITEM(c) => c.write_encrypted_client(w, e),
            Self::CMSG_TOGGLE_HELM(c) => c.write_encrypted_client(w, e),
            Self::CMSG_TOGGLE_CLOAK(c) => c.write_encrypted_client(w, e),
            Self::CMSG_SET_ACTIONBAR_TOGGLES(c) => c.write_encrypted_client(w, e),
            Self::CMSG_CHAR_RENAME(c) => c.write_encrypted_client(w, e),
            Self::CMSG_MOVE_FALL_RESET(c) => c.write_encrypted_client(w, e),
            Self::CMSG_BATTLEFIELD_STATUS(c) => c.write_encrypted_client(w, e),
            Self::CMSG_BATTLEFIELD_PORT(c) => c.write_encrypted_client(w, e),
            Self::CMSG_BATTLEMASTER_HELLO(c) => c.write_encrypted_client(w, e),
            Self::CMSG_FORCE_WALK_SPEED_CHANGE_ACK(c) => c.write_encrypted_client(w, e),
            Self::CMSG_FORCE_SWIM_BACK_SPEED_CHANGE_ACK(c) => c.write_encrypted_client(w, e),
            Self::CMSG_FORCE_TURN_RATE_CHANGE_ACK(c) => c.write_encrypted_client(w, e),
            Self::CMSG_LEAVE_BATTLEFIELD(c) => c.write_encrypted_client(w, e),
            Self::MSG_BATTLEGROUND_PLAYER_POSITIONS(c) => c.write_encrypted_client(w, e),
            Self::CMSG_BATTLEMASTER_JOIN(c) => c.write_encrypted_client(w, e),
            Self::CMSG_GUILD_INFO_TEXT(c) => c.write_encrypted_client(w, e),
            Self::CMSG_ACTIVATETAXIEXPRESS(c) => c.write_encrypted_client(w, e),
            Self::CMSG_SET_WATCHED_FACTION(c) => c.write_encrypted_client(w, e),
            Self::CMSG_GMSURVEY_SUBMIT(c) => c.write_encrypted_client(w, e),
            Self::CMSG_MOVE_SET_FLY(c) => c.write_encrypted_client(w, e),
            Self::CMSG_REALM_SPLIT(c) => c.write_encrypted_client(w, e),
            Self::CMSG_MOVE_CHNG_TRANSPORT(c) => c.write_encrypted_client(w, e),
            Self::CMSG_TIME_SYNC_RESP(c) => c.write_encrypted_client(w, e),
            Self::CMSG_SET_ACTIVE_VOICE_CHANNEL(c) => c.write_encrypted_client(w, e),
            Self::CMSG_WORLD_STATE_UI_TIMER_UPDATE(c) => c.write_encrypted_client(w, e),
            Self::CMSG_READY_FOR_ACCOUNT_DATA_TIMES(c) => c.write_encrypted_client(w, e),
        }
    }

    #[cfg(feature = "sync")]
    pub fn write_unencrypted_client<W: std::io::Write>(&self, w: &mut W) -> Result<(), std::io::Error> {
        match self {
            Self::MSG_MOVE_START_FORWARD(c) => c.write_unencrypted_client(w),
            Self::MSG_MOVE_START_BACKWARD(c) => c.write_unencrypted_client(w),
            Self::MSG_MOVE_STOP(c) => c.write_unencrypted_client(w),
            Self::MSG_MOVE_START_STRAFE_LEFT(c) => c.write_unencrypted_client(w),
            Self::MSG_MOVE_START_STRAFE_RIGHT(c) => c.write_unencrypted_client(w),
            Self::MSG_MOVE_STOP_STRAFE(c) => c.write_unencrypted_client(w),
            Self::MSG_MOVE_JUMP(c) => c.write_unencrypted_client(w),
            Self::MSG_MOVE_START_TURN_LEFT(c) => c.write_unencrypted_client(w),
            Self::MSG_MOVE_START_TURN_RIGHT(c) => c.write_unencrypted_client(w),
            Self::MSG_MOVE_STOP_TURN(c) => c.write_unencrypted_client(w),
            Self::MSG_MOVE_START_PITCH_UP(c) => c.write_unencrypted_client(w),
            Self::MSG_MOVE_START_PITCH_DOWN(c) => c.write_unencrypted_client(w),
            Self::MSG_MOVE_STOP_PITCH(c) => c.write_unencrypted_client(w),
            Self::MSG_MOVE_SET_RUN_MODE(c) => c.write_unencrypted_client(w),
            Self::MSG_MOVE_SET_WALK_MODE(c) => c.write_unencrypted_client(w),
            Self::MSG_MOVE_FALL_LAND(c) => c.write_unencrypted_client(w),
            Self::MSG_MOVE_START_SWIM(c) => c.write_unencrypted_client(w),
            Self::MSG_MOVE_STOP_SWIM(c) => c.write_unencrypted_client(w),
            Self::MSG_MOVE_SET_FACING(c) => c.write_unencrypted_client(w),
            Self::MSG_MOVE_SET_PITCH(c) => c.write_unencrypted_client(w),
            Self::MSG_MOVE_WORLDPORT_ACK(c) => c.write_unencrypted_client(w),
            Self::MSG_MOVE_HEARTBEAT(c) => c.write_unencrypted_client(w),
            Self::MSG_PETITION_DECLINE(c) => c.write_unencrypted_client(w),
            Self::MSG_TABARDVENDOR_ACTIVATE(c) => c.write_unencrypted_client(w),
            Self::MSG_PETITION_RENAME(c) => c.write_unencrypted_client(w),
            Self::MSG_SET_DUNGEON_DIFFICULTY(c) => c.write_unencrypted_client(w),
            Self::MSG_MOVE_START_ASCEND(c) => c.write_unencrypted_client(w),
            Self::MSG_MOVE_STOP_ASCEND(c) => c.write_unencrypted_client(w),
            Self::MSG_MOVE_START_DESCEND(c) => c.write_unencrypted_client(w),
            Self::CMSG_CHAR_CREATE(c) => c.write_unencrypted_client(w),
            Self::CMSG_CHAR_ENUM(c) => c.write_unencrypted_client(w),
            Self::CMSG_CHAR_DELETE(c) => c.write_unencrypted_client(w),
            Self::CMSG_PLAYER_LOGIN(c) => c.write_unencrypted_client(w),
            Self::CMSG_PLAYER_LOGOUT(c) => c.write_unencrypted_client(w),
            Self::CMSG_LOGOUT_REQUEST(c) => c.write_unencrypted_client(w),
            Self::CMSG_LOGOUT_CANCEL(c) => c.write_unencrypted_client(w),
            Self::CMSG_NAME_QUERY(c) => c.write_unencrypted_client(w),
            Self::CMSG_GUILD_CREATE(c) => c.write_unencrypted_client(w),
            Self::CMSG_GUILD_INVITE(c) => c.write_unencrypted_client(w),
            Self::CMSG_GUILD_ACCEPT(c) => c.write_unencrypted_client(w),
            Self::CMSG_GUILD_DECLINE(c) => c.write_unencrypted_client(w),
            Self::CMSG_GUILD_INFO(c) => c.write_unencrypted_client(w),
            Self::CMSG_GUILD_ROSTER(c) => c.write_unencrypted_client(w),
            Self::CMSG_GUILD_PROMOTE(c) => c.write_unencrypted_client(w),
            Self::CMSG_GUILD_DEMOTE(c) => c.write_unencrypted_client(w),
            Self::CMSG_GUILD_LEAVE(c) => c.write_unencrypted_client(w),
            Self::CMSG_GUILD_REMOVE(c) => c.write_unencrypted_client(w),
            Self::CMSG_GUILD_DISBAND(c) => c.write_unencrypted_client(w),
            Self::CMSG_GUILD_LEADER(c) => c.write_unencrypted_client(w),
            Self::CMSG_GUILD_MOTD(c) => c.write_unencrypted_client(w),
            Self::CMSG_MESSAGECHAT(c) => c.write_unencrypted_client(w),
            Self::CMSG_JOIN_CHANNEL(c) => c.write_unencrypted_client(w),
            Self::CMSG_LEAVE_CHANNEL(c) => c.write_unencrypted_client(w),
            Self::CMSG_CHANNEL_LIST(c) => c.write_unencrypted_client(w),
            Self::CMSG_CHANNEL_PASSWORD(c) => c.write_unencrypted_client(w),
            Self::CMSG_CHANNEL_SET_OWNER(c) => c.write_unencrypted_client(w),
            Self::CMSG_CHANNEL_OWNER(c) => c.write_unencrypted_client(w),
            Self::CMSG_CHANNEL_MODERATOR(c) => c.write_unencrypted_client(w),
            Self::CMSG_CHANNEL_UNMODERATOR(c) => c.write_unencrypted_client(w),
            Self::CMSG_CHANNEL_MUTE(c) => c.write_unencrypted_client(w),
            Self::CMSG_CHANNEL_UNMUTE(c) => c.write_unencrypted_client(w),
            Self::CMSG_CHANNEL_INVITE(c) => c.write_unencrypted_client(w),
            Self::CMSG_CHANNEL_KICK(c) => c.write_unencrypted_client(w),
            Self::CMSG_CHANNEL_BAN(c) => c.write_unencrypted_client(w),
            Self::CMSG_CHANNEL_UNBAN(c) => c.write_unencrypted_client(w),
            Self::CMSG_CHANNEL_ANNOUNCEMENTS(c) => c.write_unencrypted_client(w),
            Self::CMSG_CHANNEL_MODERATE(c) => c.write_unencrypted_client(w),
            Self::CMSG_OPEN_ITEM(c) => c.write_unencrypted_client(w),
            Self::CMSG_READ_ITEM(c) => c.write_unencrypted_client(w),
            Self::CMSG_GAMEOBJ_USE(c) => c.write_unencrypted_client(w),
            Self::CMSG_AREATRIGGER(c) => c.write_unencrypted_client(w),
            Self::MSG_MOVE_TELEPORT_ACK(c) => c.write_unencrypted_client(w),
            Self::CMSG_FORCE_RUN_SPEED_CHANGE_ACK(c) => c.write_unencrypted_client(w),
            Self::CMSG_FORCE_RUN_BACK_SPEED_CHANGE_ACK(c) => c.write_unencrypted_client(w),
            Self::CMSG_FORCE_SWIM_SPEED_CHANGE_ACK(c) => c.write_unencrypted_client(w),
            Self::CMSG_FORCE_MOVE_ROOT_ACK(c) => c.write_unencrypted_client(w),
            Self::CMSG_FORCE_MOVE_UNROOT_ACK(c) => c.write_unencrypted_client(w),
            Self::CMSG_NEXT_CINEMATIC_CAMERA(c) => c.write_unencrypted_client(w),
            Self::CMSG_COMPLETE_CINEMATIC(c) => c.write_unencrypted_client(w),
            Self::CMSG_TUTORIAL_FLAG(c) => c.write_unencrypted_client(w),
            Self::CMSG_TUTORIAL_CLEAR(c) => c.write_unencrypted_client(w),
            Self::CMSG_TUTORIAL_RESET(c) => c.write_unencrypted_client(w),
            Self::CMSG_STANDSTATECHANGE(c) => c.write_unencrypted_client(w),
            Self::CMSG_EMOTE(c) => c.write_unencrypted_client(w),
            Self::CMSG_TEXT_EMOTE(c) => c.write_unencrypted_client(w),
            Self::CMSG_AUTOSTORE_LOOT_ITEM(c) => c.write_unencrypted_client(w),
            Self::CMSG_AUTOEQUIP_ITEM(c) => c.write_unencrypted_client(w),
            Self::CMSG_AUTOSTORE_BAG_ITEM(c) => c.write_unencrypted_client(w),
            Self::CMSG_SWAP_ITEM(c) => c.write_unencrypted_client(w),
            Self::CMSG_SWAP_INV_ITEM(c) => c.write_unencrypted_client(w),
            Self::CMSG_SPLIT_ITEM(c) => c.write_unencrypted_client(w),
            Self::CMSG_AUTOEQUIP_ITEM_SLOT(c) => c.write_unencrypted_client(w),
            Self::CMSG_DESTROYITEM(c) => c.write_unencrypted_client(w),
            Self::CMSG_INSPECT(c) => c.write_unencrypted_client(w),
            Self::CMSG_SET_FACTION_ATWAR(c) => c.write_unencrypted_client(w),
            Self::CMSG_SET_ACTION_BUTTON(c) => c.write_unencrypted_client(w),
            Self::CMSG_CANCEL_CHANNELLING(c) => c.write_unencrypted_client(w),
            Self::CMSG_SET_SELECTION(c) => c.write_unencrypted_client(w),
            Self::CMSG_SET_TARGET_OBSOLETE(c) => c.write_unencrypted_client(w),
            Self::CMSG_ATTACKSWING(c) => c.write_unencrypted_client(w),
            Self::CMSG_ATTACKSTOP(c) => c.write_unencrypted_client(w),
            Self::CMSG_REPOP_REQUEST(c) => c.write_unencrypted_client(w),
            Self::CMSG_LOOT(c) => c.write_unencrypted_client(w),
            Self::CMSG_LOOT_MONEY(c) => c.write_unencrypted_client(w),
            Self::CMSG_LOOT_RELEASE(c) => c.write_unencrypted_client(w),
            Self::CMSG_DUEL_ACCEPTED(c) => c.write_unencrypted_client(w),
            Self::CMSG_DUEL_CANCELLED(c) => c.write_unencrypted_client(w),
            Self::CMSG_MOUNTSPECIAL_ANIM(c) => c.write_unencrypted_client(w),
            Self::CMSG_PET_ACTION(c) => c.write_unencrypted_client(w),
            Self::CMSG_PET_ABANDON(c) => c.write_unencrypted_client(w),
            Self::CMSG_GOSSIP_HELLO(c) => c.write_unencrypted_client(w),
            Self::CMSG_GOSSIP_SELECT_OPTION(c) => c.write_unencrypted_client(w),
            Self::CMSG_LIST_INVENTORY(c) => c.write_unencrypted_client(w),
            Self::CMSG_SELL_ITEM(c) => c.write_unencrypted_client(w),
            Self::CMSG_BUY_ITEM(c) => c.write_unencrypted_client(w),
            Self::CMSG_BUY_ITEM_IN_SLOT(c) => c.write_unencrypted_client(w),
            Self::CMSG_ACTIVATETAXI(c) => c.write_unencrypted_client(w),
            Self::CMSG_BINDER_ACTIVATE(c) => c.write_unencrypted_client(w),
            Self::CMSG_BANKER_ACTIVATE(c) => c.write_unencrypted_client(w),
            Self::CMSG_BUY_BANK_SLOT(c) => c.write_unencrypted_client(w),
            Self::CMSG_PETITION_SHOWLIST(c) => c.write_unencrypted_client(w),
            Self::CMSG_PETITION_BUY(c) => c.write_unencrypted_client(w),
            Self::CMSG_PETITION_SHOW_SIGNATURES(c) => c.write_unencrypted_client(w),
            Self::CMSG_PETITION_SIGN(c) => c.write_unencrypted_client(w),
            Self::CMSG_OFFER_PETITION(c) => c.write_unencrypted_client(w),
            Self::CMSG_TURN_IN_PETITION(c) => c.write_unencrypted_client(w),
            Self::CMSG_BUG(c) => c.write_unencrypted_client(w),
            Self::CMSG_PLAYED_TIME(c) => c.write_unencrypted_client(w),
            Self::CMSG_QUERY_TIME(c) => c.write_unencrypted_client(w),
            Self::CMSG_WRAP_ITEM(c) => c.write_unencrypted_client(w),
            Self::CMSG_PING(c) => c.write_unencrypted_client(w),
            Self::CMSG_SETSHEATHED(c) => c.write_unencrypted_client(w),
            Self::CMSG_AUTH_SESSION(c) => c.write_unencrypted_client(w),
            Self::MSG_SAVE_GUILD_EMBLEM(c) => c.write_unencrypted_client(w),
            Self::CMSG_ZONEUPDATE(c) => c.write_unencrypted_client(w),
            Self::MSG_RANDOM_ROLL(c) => c.write_unencrypted_client(w),
            Self::CMSG_GMTICKET_UPDATETEXT(c) => c.write_unencrypted_client(w),
            Self::CMSG_REQUEST_ACCOUNT_DATA(c) => c.write_unencrypted_client(w),
            Self::CMSG_UPDATE_ACCOUNT_DATA(c) => c.write_unencrypted_client(w),
            Self::CMSG_GMTICKET_GETTICKET(c) => c.write_unencrypted_client(w),
            Self::CMSG_GMTICKET_DELETETICKET(c) => c.write_unencrypted_client(w),
            Self::CMSG_GMTICKET_SYSTEMSTATUS(c) => c.write_unencrypted_client(w),
            Self::CMSG_CHAT_IGNORED(c) => c.write_unencrypted_client(w),
            Self::CMSG_GUILD_RANK(c) => c.write_unencrypted_client(w),
            Self::CMSG_GUILD_ADD_RANK(c) => c.write_unencrypted_client(w),
            Self::CMSG_GUILD_DEL_RANK(c) => c.write_unencrypted_client(w),
            Self::CMSG_GUILD_SET_PUBLIC_NOTE(c) => c.write_unencrypted_client(w),
            Self::CMSG_GUILD_SET_OFFICER_NOTE(c) => c.write_unencrypted_client(w),
            Self::CMSG_SEND_MAIL(c) => c.write_unencrypted_client(w),
            Self::CMSG_GET_MAIL_LIST(c) => c.write_unencrypted_client(w),
            Self::CMSG_BATTLEFIELD_LIST(c) => c.write_unencrypted_client(w),
            Self::CMSG_MAIL_TAKE_MONEY(c) => c.write_unencrypted_client(w),
            Self::CMSG_MAIL_TAKE_ITEM(c) => c.write_unencrypted_client(w),
            Self::CMSG_MAIL_MARK_AS_READ(c) => c.write_unencrypted_client(w),
            Self::CMSG_MAIL_RETURN_TO_SENDER(c) => c.write_unencrypted_client(w),
            Self::CMSG_MAIL_DELETE(c) => c.write_unencrypted_client(w),
            Self::CMSG_MAIL_CREATE_TEXT_ITEM(c) => c.write_unencrypted_client(w),
            Self::MSG_AUCTION_HELLO(c) => c.write_unencrypted_client(w),
            Self::CMSG_AUCTION_SELL_ITEM(c) => c.write_unencrypted_client(w),
            Self::CMSG_AUCTION_REMOVE_ITEM(c) => c.write_unencrypted_client(w),
            Self::CMSG_AUCTION_LIST_ITEMS(c) => c.write_unencrypted_client(w),
            Self::CMSG_AUCTION_LIST_OWNER_ITEMS(c) => c.write_unencrypted_client(w),
            Self::CMSG_AUCTION_PLACE_BID(c) => c.write_unencrypted_client(w),
            Self::CMSG_AUCTION_LIST_BIDDER_ITEMS(c) => c.write_unencrypted_client(w),
            Self::CMSG_SET_AMMO(c) => c.write_unencrypted_client(w),
            Self::CMSG_SET_ACTIVE_MOVER(c) => c.write_unencrypted_client(w),
            Self::CMSG_BUY_STABLE_SLOT(c) => c.write_unencrypted_client(w),
            Self::CMSG_AUTOSTORE_BANK_ITEM(c) => c.write_unencrypted_client(w),
            Self::CMSG_AUTOBANK_ITEM(c) => c.write_unencrypted_client(w),
            Self::CMSG_BUYBACK_ITEM(c) => c.write_unencrypted_client(w),
            Self::CMSG_LOOT_ROLL(c) => c.write_unencrypted_client(w),
            Self::CMSG_LOOT_MASTER_GIVE(c) => c.write_unencrypted_client(w),
            Self::CMSG_REPAIR_ITEM(c) => c.write_unencrypted_client(w),
            Self::CMSG_TOGGLE_HELM(c) => c.write_unencrypted_client(w),
            Self::CMSG_TOGGLE_CLOAK(c) => c.write_unencrypted_client(w),
            Self::CMSG_SET_ACTIONBAR_TOGGLES(c) => c.write_unencrypted_client(w),
            Self::CMSG_CHAR_RENAME(c) => c.write_unencrypted_client(w),
            Self::CMSG_MOVE_FALL_RESET(c) => c.write_unencrypted_client(w),
            Self::CMSG_BATTLEFIELD_STATUS(c) => c.write_unencrypted_client(w),
            Self::CMSG_BATTLEFIELD_PORT(c) => c.write_unencrypted_client(w),
            Self::CMSG_BATTLEMASTER_HELLO(c) => c.write_unencrypted_client(w),
            Self::CMSG_FORCE_WALK_SPEED_CHANGE_ACK(c) => c.write_unencrypted_client(w),
            Self::CMSG_FORCE_SWIM_BACK_SPEED_CHANGE_ACK(c) => c.write_unencrypted_client(w),
            Self::CMSG_FORCE_TURN_RATE_CHANGE_ACK(c) => c.write_unencrypted_client(w),
            Self::CMSG_LEAVE_BATTLEFIELD(c) => c.write_unencrypted_client(w),
            Self::MSG_BATTLEGROUND_PLAYER_POSITIONS(c) => c.write_unencrypted_client(w),
            Self::CMSG_BATTLEMASTER_JOIN(c) => c.write_unencrypted_client(w),
            Self::CMSG_GUILD_INFO_TEXT(c) => c.write_unencrypted_client(w),
            Self::CMSG_ACTIVATETAXIEXPRESS(c) => c.write_unencrypted_client(w),
            Self::CMSG_SET_WATCHED_FACTION(c) => c.write_unencrypted_client(w),
            Self::CMSG_GMSURVEY_SUBMIT(c) => c.write_unencrypted_client(w),
            Self::CMSG_MOVE_SET_FLY(c) => c.write_unencrypted_client(w),
            Self::CMSG_REALM_SPLIT(c) => c.write_unencrypted_client(w),
            Self::CMSG_MOVE_CHNG_TRANSPORT(c) => c.write_unencrypted_client(w),
            Self::CMSG_TIME_SYNC_RESP(c) => c.write_unencrypted_client(w),
            Self::CMSG_SET_ACTIVE_VOICE_CHANNEL(c) => c.write_unencrypted_client(w),
            Self::CMSG_WORLD_STATE_UI_TIMER_UPDATE(c) => c.write_unencrypted_client(w),
            Self::CMSG_READY_FOR_ACCOUNT_DATA_TIMES(c) => c.write_unencrypted_client(w),
        }
    }

    #[cfg(all(feature = "tokio", feature = "encryption"))]
    pub async fn tokio_write_encrypted_client<W: tokio::io::AsyncWriteExt + Unpin + Send>(&self, w: &mut W, e: &mut ClientEncrypterHalf) -> Result<(), std::io::Error> {
        match self {
            Self::MSG_MOVE_START_FORWARD(c) => c.tokio_write_encrypted_client(w, e).await,
            Self::MSG_MOVE_START_BACKWARD(c) => c.tokio_write_encrypted_client(w, e).await,
            Self::MSG_MOVE_STOP(c) => c.tokio_write_encrypted_client(w, e).await,
            Self::MSG_MOVE_START_STRAFE_LEFT(c) => c.tokio_write_encrypted_client(w, e).await,
            Self::MSG_MOVE_START_STRAFE_RIGHT(c) => c.tokio_write_encrypted_client(w, e).await,
            Self::MSG_MOVE_STOP_STRAFE(c) => c.tokio_write_encrypted_client(w, e).await,
            Self::MSG_MOVE_JUMP(c) => c.tokio_write_encrypted_client(w, e).await,
            Self::MSG_MOVE_START_TURN_LEFT(c) => c.tokio_write_encrypted_client(w, e).await,
            Self::MSG_MOVE_START_TURN_RIGHT(c) => c.tokio_write_encrypted_client(w, e).await,
            Self::MSG_MOVE_STOP_TURN(c) => c.tokio_write_encrypted_client(w, e).await,
            Self::MSG_MOVE_START_PITCH_UP(c) => c.tokio_write_encrypted_client(w, e).await,
            Self::MSG_MOVE_START_PITCH_DOWN(c) => c.tokio_write_encrypted_client(w, e).await,
            Self::MSG_MOVE_STOP_PITCH(c) => c.tokio_write_encrypted_client(w, e).await,
            Self::MSG_MOVE_SET_RUN_MODE(c) => c.tokio_write_encrypted_client(w, e).await,
            Self::MSG_MOVE_SET_WALK_MODE(c) => c.tokio_write_encrypted_client(w, e).await,
            Self::MSG_MOVE_FALL_LAND(c) => c.tokio_write_encrypted_client(w, e).await,
            Self::MSG_MOVE_START_SWIM(c) => c.tokio_write_encrypted_client(w, e).await,
            Self::MSG_MOVE_STOP_SWIM(c) => c.tokio_write_encrypted_client(w, e).await,
            Self::MSG_MOVE_SET_FACING(c) => c.tokio_write_encrypted_client(w, e).await,
            Self::MSG_MOVE_SET_PITCH(c) => c.tokio_write_encrypted_client(w, e).await,
            Self::MSG_MOVE_WORLDPORT_ACK(c) => c.tokio_write_encrypted_client(w, e).await,
            Self::MSG_MOVE_HEARTBEAT(c) => c.tokio_write_encrypted_client(w, e).await,
            Self::MSG_PETITION_DECLINE(c) => c.tokio_write_encrypted_client(w, e).await,
            Self::MSG_TABARDVENDOR_ACTIVATE(c) => c.tokio_write_encrypted_client(w, e).await,
            Self::MSG_PETITION_RENAME(c) => c.tokio_write_encrypted_client(w, e).await,
            Self::MSG_SET_DUNGEON_DIFFICULTY(c) => c.tokio_write_encrypted_client(w, e).await,
            Self::MSG_MOVE_START_ASCEND(c) => c.tokio_write_encrypted_client(w, e).await,
            Self::MSG_MOVE_STOP_ASCEND(c) => c.tokio_write_encrypted_client(w, e).await,
            Self::MSG_MOVE_START_DESCEND(c) => c.tokio_write_encrypted_client(w, e).await,
            Self::CMSG_CHAR_CREATE(c) => c.tokio_write_encrypted_client(w, e).await,
            Self::CMSG_CHAR_ENUM(c) => c.tokio_write_encrypted_client(w, e).await,
            Self::CMSG_CHAR_DELETE(c) => c.tokio_write_encrypted_client(w, e).await,
            Self::CMSG_PLAYER_LOGIN(c) => c.tokio_write_encrypted_client(w, e).await,
            Self::CMSG_PLAYER_LOGOUT(c) => c.tokio_write_encrypted_client(w, e).await,
            Self::CMSG_LOGOUT_REQUEST(c) => c.tokio_write_encrypted_client(w, e).await,
            Self::CMSG_LOGOUT_CANCEL(c) => c.tokio_write_encrypted_client(w, e).await,
            Self::CMSG_NAME_QUERY(c) => c.tokio_write_encrypted_client(w, e).await,
            Self::CMSG_GUILD_CREATE(c) => c.tokio_write_encrypted_client(w, e).await,
            Self::CMSG_GUILD_INVITE(c) => c.tokio_write_encrypted_client(w, e).await,
            Self::CMSG_GUILD_ACCEPT(c) => c.tokio_write_encrypted_client(w, e).await,
            Self::CMSG_GUILD_DECLINE(c) => c.tokio_write_encrypted_client(w, e).await,
            Self::CMSG_GUILD_INFO(c) => c.tokio_write_encrypted_client(w, e).await,
            Self::CMSG_GUILD_ROSTER(c) => c.tokio_write_encrypted_client(w, e).await,
            Self::CMSG_GUILD_PROMOTE(c) => c.tokio_write_encrypted_client(w, e).await,
            Self::CMSG_GUILD_DEMOTE(c) => c.tokio_write_encrypted_client(w, e).await,
            Self::CMSG_GUILD_LEAVE(c) => c.tokio_write_encrypted_client(w, e).await,
            Self::CMSG_GUILD_REMOVE(c) => c.tokio_write_encrypted_client(w, e).await,
            Self::CMSG_GUILD_DISBAND(c) => c.tokio_write_encrypted_client(w, e).await,
            Self::CMSG_GUILD_LEADER(c) => c.tokio_write_encrypted_client(w, e).await,
            Self::CMSG_GUILD_MOTD(c) => c.tokio_write_encrypted_client(w, e).await,
            Self::CMSG_MESSAGECHAT(c) => c.tokio_write_encrypted_client(w, e).await,
            Self::CMSG_JOIN_CHANNEL(c) => c.tokio_write_encrypted_client(w, e).await,
            Self::CMSG_LEAVE_CHANNEL(c) => c.tokio_write_encrypted_client(w, e).await,
            Self::CMSG_CHANNEL_LIST(c) => c.tokio_write_encrypted_client(w, e).await,
            Self::CMSG_CHANNEL_PASSWORD(c) => c.tokio_write_encrypted_client(w, e).await,
            Self::CMSG_CHANNEL_SET_OWNER(c) => c.tokio_write_encrypted_client(w, e).await,
            Self::CMSG_CHANNEL_OWNER(c) => c.tokio_write_encrypted_client(w, e).await,
            Self::CMSG_CHANNEL_MODERATOR(c) => c.tokio_write_encrypted_client(w, e).await,
            Self::CMSG_CHANNEL_UNMODERATOR(c) => c.tokio_write_encrypted_client(w, e).await,
            Self::CMSG_CHANNEL_MUTE(c) => c.tokio_write_encrypted_client(w, e).await,
            Self::CMSG_CHANNEL_UNMUTE(c) => c.tokio_write_encrypted_client(w, e).await,
            Self::CMSG_CHANNEL_INVITE(c) => c.tokio_write_encrypted_client(w, e).await,
            Self::CMSG_CHANNEL_KICK(c) => c.tokio_write_encrypted_client(w, e).await,
            Self::CMSG_CHANNEL_BAN(c) => c.tokio_write_encrypted_client(w, e).await,
            Self::CMSG_CHANNEL_UNBAN(c) => c.tokio_write_encrypted_client(w, e).await,
            Self::CMSG_CHANNEL_ANNOUNCEMENTS(c) => c.tokio_write_encrypted_client(w, e).await,
            Self::CMSG_CHANNEL_MODERATE(c) => c.tokio_write_encrypted_client(w, e).await,
            Self::CMSG_OPEN_ITEM(c) => c.tokio_write_encrypted_client(w, e).await,
            Self::CMSG_READ_ITEM(c) => c.tokio_write_encrypted_client(w, e).await,
            Self::CMSG_GAMEOBJ_USE(c) => c.tokio_write_encrypted_client(w, e).await,
            Self::CMSG_AREATRIGGER(c) => c.tokio_write_encrypted_client(w, e).await,
            Self::MSG_MOVE_TELEPORT_ACK(c) => c.tokio_write_encrypted_client(w, e).await,
            Self::CMSG_FORCE_RUN_SPEED_CHANGE_ACK(c) => c.tokio_write_encrypted_client(w, e).await,
            Self::CMSG_FORCE_RUN_BACK_SPEED_CHANGE_ACK(c) => c.tokio_write_encrypted_client(w, e).await,
            Self::CMSG_FORCE_SWIM_SPEED_CHANGE_ACK(c) => c.tokio_write_encrypted_client(w, e).await,
            Self::CMSG_FORCE_MOVE_ROOT_ACK(c) => c.tokio_write_encrypted_client(w, e).await,
            Self::CMSG_FORCE_MOVE_UNROOT_ACK(c) => c.tokio_write_encrypted_client(w, e).await,
            Self::CMSG_NEXT_CINEMATIC_CAMERA(c) => c.tokio_write_encrypted_client(w, e).await,
            Self::CMSG_COMPLETE_CINEMATIC(c) => c.tokio_write_encrypted_client(w, e).await,
            Self::CMSG_TUTORIAL_FLAG(c) => c.tokio_write_encrypted_client(w, e).await,
            Self::CMSG_TUTORIAL_CLEAR(c) => c.tokio_write_encrypted_client(w, e).await,
            Self::CMSG_TUTORIAL_RESET(c) => c.tokio_write_encrypted_client(w, e).await,
            Self::CMSG_STANDSTATECHANGE(c) => c.tokio_write_encrypted_client(w, e).await,
            Self::CMSG_EMOTE(c) => c.tokio_write_encrypted_client(w, e).await,
            Self::CMSG_TEXT_EMOTE(c) => c.tokio_write_encrypted_client(w, e).await,
            Self::CMSG_AUTOSTORE_LOOT_ITEM(c) => c.tokio_write_encrypted_client(w, e).await,
            Self::CMSG_AUTOEQUIP_ITEM(c) => c.tokio_write_encrypted_client(w, e).await,
            Self::CMSG_AUTOSTORE_BAG_ITEM(c) => c.tokio_write_encrypted_client(w, e).await,
            Self::CMSG_SWAP_ITEM(c) => c.tokio_write_encrypted_client(w, e).await,
            Self::CMSG_SWAP_INV_ITEM(c) => c.tokio_write_encrypted_client(w, e).await,
            Self::CMSG_SPLIT_ITEM(c) => c.tokio_write_encrypted_client(w, e).await,
            Self::CMSG_AUTOEQUIP_ITEM_SLOT(c) => c.tokio_write_encrypted_client(w, e).await,
            Self::CMSG_DESTROYITEM(c) => c.tokio_write_encrypted_client(w, e).await,
            Self::CMSG_INSPECT(c) => c.tokio_write_encrypted_client(w, e).await,
            Self::CMSG_SET_FACTION_ATWAR(c) => c.tokio_write_encrypted_client(w, e).await,
            Self::CMSG_SET_ACTION_BUTTON(c) => c.tokio_write_encrypted_client(w, e).await,
            Self::CMSG_CANCEL_CHANNELLING(c) => c.tokio_write_encrypted_client(w, e).await,
            Self::CMSG_SET_SELECTION(c) => c.tokio_write_encrypted_client(w, e).await,
            Self::CMSG_SET_TARGET_OBSOLETE(c) => c.tokio_write_encrypted_client(w, e).await,
            Self::CMSG_ATTACKSWING(c) => c.tokio_write_encrypted_client(w, e).await,
            Self::CMSG_ATTACKSTOP(c) => c.tokio_write_encrypted_client(w, e).await,
            Self::CMSG_REPOP_REQUEST(c) => c.tokio_write_encrypted_client(w, e).await,
            Self::CMSG_LOOT(c) => c.tokio_write_encrypted_client(w, e).await,
            Self::CMSG_LOOT_MONEY(c) => c.tokio_write_encrypted_client(w, e).await,
            Self::CMSG_LOOT_RELEASE(c) => c.tokio_write_encrypted_client(w, e).await,
            Self::CMSG_DUEL_ACCEPTED(c) => c.tokio_write_encrypted_client(w, e).await,
            Self::CMSG_DUEL_CANCELLED(c) => c.tokio_write_encrypted_client(w, e).await,
            Self::CMSG_MOUNTSPECIAL_ANIM(c) => c.tokio_write_encrypted_client(w, e).await,
            Self::CMSG_PET_ACTION(c) => c.tokio_write_encrypted_client(w, e).await,
            Self::CMSG_PET_ABANDON(c) => c.tokio_write_encrypted_client(w, e).await,
            Self::CMSG_GOSSIP_HELLO(c) => c.tokio_write_encrypted_client(w, e).await,
            Self::CMSG_GOSSIP_SELECT_OPTION(c) => c.tokio_write_encrypted_client(w, e).await,
            Self::CMSG_LIST_INVENTORY(c) => c.tokio_write_encrypted_client(w, e).await,
            Self::CMSG_SELL_ITEM(c) => c.tokio_write_encrypted_client(w, e).await,
            Self::CMSG_BUY_ITEM(c) => c.tokio_write_encrypted_client(w, e).await,
            Self::CMSG_BUY_ITEM_IN_SLOT(c) => c.tokio_write_encrypted_client(w, e).await,
            Self::CMSG_ACTIVATETAXI(c) => c.tokio_write_encrypted_client(w, e).await,
            Self::CMSG_BINDER_ACTIVATE(c) => c.tokio_write_encrypted_client(w, e).await,
            Self::CMSG_BANKER_ACTIVATE(c) => c.tokio_write_encrypted_client(w, e).await,
            Self::CMSG_BUY_BANK_SLOT(c) => c.tokio_write_encrypted_client(w, e).await,
            Self::CMSG_PETITION_SHOWLIST(c) => c.tokio_write_encrypted_client(w, e).await,
            Self::CMSG_PETITION_BUY(c) => c.tokio_write_encrypted_client(w, e).await,
            Self::CMSG_PETITION_SHOW_SIGNATURES(c) => c.tokio_write_encrypted_client(w, e).await,
            Self::CMSG_PETITION_SIGN(c) => c.tokio_write_encrypted_client(w, e).await,
            Self::CMSG_OFFER_PETITION(c) => c.tokio_write_encrypted_client(w, e).await,
            Self::CMSG_TURN_IN_PETITION(c) => c.tokio_write_encrypted_client(w, e).await,
            Self::CMSG_BUG(c) => c.tokio_write_encrypted_client(w, e).await,
            Self::CMSG_PLAYED_TIME(c) => c.tokio_write_encrypted_client(w, e).await,
            Self::CMSG_QUERY_TIME(c) => c.tokio_write_encrypted_client(w, e).await,
            Self::CMSG_WRAP_ITEM(c) => c.tokio_write_encrypted_client(w, e).await,
            Self::CMSG_PING(c) => c.tokio_write_encrypted_client(w, e).await,
            Self::CMSG_SETSHEATHED(c) => c.tokio_write_encrypted_client(w, e).await,
            Self::CMSG_AUTH_SESSION(c) => c.tokio_write_encrypted_client(w, e).await,
            Self::MSG_SAVE_GUILD_EMBLEM(c) => c.tokio_write_encrypted_client(w, e).await,
            Self::CMSG_ZONEUPDATE(c) => c.tokio_write_encrypted_client(w, e).await,
            Self::MSG_RANDOM_ROLL(c) => c.tokio_write_encrypted_client(w, e).await,
            Self::CMSG_GMTICKET_UPDATETEXT(c) => c.tokio_write_encrypted_client(w, e).await,
            Self::CMSG_REQUEST_ACCOUNT_DATA(c) => c.tokio_write_encrypted_client(w, e).await,
            Self::CMSG_UPDATE_ACCOUNT_DATA(c) => c.tokio_write_encrypted_client(w, e).await,
            Self::CMSG_GMTICKET_GETTICKET(c) => c.tokio_write_encrypted_client(w, e).await,
            Self::CMSG_GMTICKET_DELETETICKET(c) => c.tokio_write_encrypted_client(w, e).await,
            Self::CMSG_GMTICKET_SYSTEMSTATUS(c) => c.tokio_write_encrypted_client(w, e).await,
            Self::CMSG_CHAT_IGNORED(c) => c.tokio_write_encrypted_client(w, e).await,
            Self::CMSG_GUILD_RANK(c) => c.tokio_write_encrypted_client(w, e).await,
            Self::CMSG_GUILD_ADD_RANK(c) => c.tokio_write_encrypted_client(w, e).await,
            Self::CMSG_GUILD_DEL_RANK(c) => c.tokio_write_encrypted_client(w, e).await,
            Self::CMSG_GUILD_SET_PUBLIC_NOTE(c) => c.tokio_write_encrypted_client(w, e).await,
            Self::CMSG_GUILD_SET_OFFICER_NOTE(c) => c.tokio_write_encrypted_client(w, e).await,
            Self::CMSG_SEND_MAIL(c) => c.tokio_write_encrypted_client(w, e).await,
            Self::CMSG_GET_MAIL_LIST(c) => c.tokio_write_encrypted_client(w, e).await,
            Self::CMSG_BATTLEFIELD_LIST(c) => c.tokio_write_encrypted_client(w, e).await,
            Self::CMSG_MAIL_TAKE_MONEY(c) => c.tokio_write_encrypted_client(w, e).await,
            Self::CMSG_MAIL_TAKE_ITEM(c) => c.tokio_write_encrypted_client(w, e).await,
            Self::CMSG_MAIL_MARK_AS_READ(c) => c.tokio_write_encrypted_client(w, e).await,
            Self::CMSG_MAIL_RETURN_TO_SENDER(c) => c.tokio_write_encrypted_client(w, e).await,
            Self::CMSG_MAIL_DELETE(c) => c.tokio_write_encrypted_client(w, e).await,
            Self::CMSG_MAIL_CREATE_TEXT_ITEM(c) => c.tokio_write_encrypted_client(w, e).await,
            Self::MSG_AUCTION_HELLO(c) => c.tokio_write_encrypted_client(w, e).await,
            Self::CMSG_AUCTION_SELL_ITEM(c) => c.tokio_write_encrypted_client(w, e).await,
            Self::CMSG_AUCTION_REMOVE_ITEM(c) => c.tokio_write_encrypted_client(w, e).await,
            Self::CMSG_AUCTION_LIST_ITEMS(c) => c.tokio_write_encrypted_client(w, e).await,
            Self::CMSG_AUCTION_LIST_OWNER_ITEMS(c) => c.tokio_write_encrypted_client(w, e).await,
            Self::CMSG_AUCTION_PLACE_BID(c) => c.tokio_write_encrypted_client(w, e).await,
            Self::CMSG_AUCTION_LIST_BIDDER_ITEMS(c) => c.tokio_write_encrypted_client(w, e).await,
            Self::CMSG_SET_AMMO(c) => c.tokio_write_encrypted_client(w, e).await,
            Self::CMSG_SET_ACTIVE_MOVER(c) => c.tokio_write_encrypted_client(w, e).await,
            Self::CMSG_BUY_STABLE_SLOT(c) => c.tokio_write_encrypted_client(w, e).await,
            Self::CMSG_AUTOSTORE_BANK_ITEM(c) => c.tokio_write_encrypted_client(w, e).await,
            Self::CMSG_AUTOBANK_ITEM(c) => c.tokio_write_encrypted_client(w, e).await,
            Self::CMSG_BUYBACK_ITEM(c) => c.tokio_write_encrypted_client(w, e).await,
            Self::CMSG_LOOT_ROLL(c) => c.tokio_write_encrypted_client(w, e).await,
            Self::CMSG_LOOT_MASTER_GIVE(c) => c.tokio_write_encrypted_client(w, e).await,
            Self::CMSG_REPAIR_ITEM(c) => c.tokio_write_encrypted_client(w, e).await,
            Self::CMSG_TOGGLE_HELM(c) => c.tokio_write_encrypted_client(w, e).await,
            Self::CMSG_TOGGLE_CLOAK(c) => c.tokio_write_encrypted_client(w, e).await,
            Self::CMSG_SET_ACTIONBAR_TOGGLES(c) => c.tokio_write_encrypted_client(w, e).await,
            Self::CMSG_CHAR_RENAME(c) => c.tokio_write_encrypted_client(w, e).await,
            Self::CMSG_MOVE_FALL_RESET(c) => c.tokio_write_encrypted_client(w, e).await,
            Self::CMSG_BATTLEFIELD_STATUS(c) => c.tokio_write_encrypted_client(w, e).await,
            Self::CMSG_BATTLEFIELD_PORT(c) => c.tokio_write_encrypted_client(w, e).await,
            Self::CMSG_BATTLEMASTER_HELLO(c) => c.tokio_write_encrypted_client(w, e).await,
            Self::CMSG_FORCE_WALK_SPEED_CHANGE_ACK(c) => c.tokio_write_encrypted_client(w, e).await,
            Self::CMSG_FORCE_SWIM_BACK_SPEED_CHANGE_ACK(c) => c.tokio_write_encrypted_client(w, e).await,
            Self::CMSG_FORCE_TURN_RATE_CHANGE_ACK(c) => c.tokio_write_encrypted_client(w, e).await,
            Self::CMSG_LEAVE_BATTLEFIELD(c) => c.tokio_write_encrypted_client(w, e).await,
            Self::MSG_BATTLEGROUND_PLAYER_POSITIONS(c) => c.tokio_write_encrypted_client(w, e).await,
            Self::CMSG_BATTLEMASTER_JOIN(c) => c.tokio_write_encrypted_client(w, e).await,
            Self::CMSG_GUILD_INFO_TEXT(c) => c.tokio_write_encrypted_client(w, e).await,
            Self::CMSG_ACTIVATETAXIEXPRESS(c) => c.tokio_write_encrypted_client(w, e).await,
            Self::CMSG_SET_WATCHED_FACTION(c) => c.tokio_write_encrypted_client(w, e).await,
            Self::CMSG_GMSURVEY_SUBMIT(c) => c.tokio_write_encrypted_client(w, e).await,
            Self::CMSG_MOVE_SET_FLY(c) => c.tokio_write_encrypted_client(w, e).await,
            Self::CMSG_REALM_SPLIT(c) => c.tokio_write_encrypted_client(w, e).await,
            Self::CMSG_MOVE_CHNG_TRANSPORT(c) => c.tokio_write_encrypted_client(w, e).await,
            Self::CMSG_TIME_SYNC_RESP(c) => c.tokio_write_encrypted_client(w, e).await,
            Self::CMSG_SET_ACTIVE_VOICE_CHANNEL(c) => c.tokio_write_encrypted_client(w, e).await,
            Self::CMSG_WORLD_STATE_UI_TIMER_UPDATE(c) => c.tokio_write_encrypted_client(w, e).await,
            Self::CMSG_READY_FOR_ACCOUNT_DATA_TIMES(c) => c.tokio_write_encrypted_client(w, e).await,
        }
    }

    #[cfg(feature = "tokio")]
    pub async fn tokio_write_unencrypted_client<W: tokio::io::AsyncWriteExt + Unpin + Send>(&self, w: &mut W) -> Result<(), std::io::Error> {
        match self {
            Self::MSG_MOVE_START_FORWARD(c) => c.tokio_write_unencrypted_client(w).await,
            Self::MSG_MOVE_START_BACKWARD(c) => c.tokio_write_unencrypted_client(w).await,
            Self::MSG_MOVE_STOP(c) => c.tokio_write_unencrypted_client(w).await,
            Self::MSG_MOVE_START_STRAFE_LEFT(c) => c.tokio_write_unencrypted_client(w).await,
            Self::MSG_MOVE_START_STRAFE_RIGHT(c) => c.tokio_write_unencrypted_client(w).await,
            Self::MSG_MOVE_STOP_STRAFE(c) => c.tokio_write_unencrypted_client(w).await,
            Self::MSG_MOVE_JUMP(c) => c.tokio_write_unencrypted_client(w).await,
            Self::MSG_MOVE_START_TURN_LEFT(c) => c.tokio_write_unencrypted_client(w).await,
            Self::MSG_MOVE_START_TURN_RIGHT(c) => c.tokio_write_unencrypted_client(w).await,
            Self::MSG_MOVE_STOP_TURN(c) => c.tokio_write_unencrypted_client(w).await,
            Self::MSG_MOVE_START_PITCH_UP(c) => c.tokio_write_unencrypted_client(w).await,
            Self::MSG_MOVE_START_PITCH_DOWN(c) => c.tokio_write_unencrypted_client(w).await,
            Self::MSG_MOVE_STOP_PITCH(c) => c.tokio_write_unencrypted_client(w).await,
            Self::MSG_MOVE_SET_RUN_MODE(c) => c.tokio_write_unencrypted_client(w).await,
            Self::MSG_MOVE_SET_WALK_MODE(c) => c.tokio_write_unencrypted_client(w).await,
            Self::MSG_MOVE_FALL_LAND(c) => c.tokio_write_unencrypted_client(w).await,
            Self::MSG_MOVE_START_SWIM(c) => c.tokio_write_unencrypted_client(w).await,
            Self::MSG_MOVE_STOP_SWIM(c) => c.tokio_write_unencrypted_client(w).await,
            Self::MSG_MOVE_SET_FACING(c) => c.tokio_write_unencrypted_client(w).await,
            Self::MSG_MOVE_SET_PITCH(c) => c.tokio_write_unencrypted_client(w).await,
            Self::MSG_MOVE_WORLDPORT_ACK(c) => c.tokio_write_unencrypted_client(w).await,
            Self::MSG_MOVE_HEARTBEAT(c) => c.tokio_write_unencrypted_client(w).await,
            Self::MSG_PETITION_DECLINE(c) => c.tokio_write_unencrypted_client(w).await,
            Self::MSG_TABARDVENDOR_ACTIVATE(c) => c.tokio_write_unencrypted_client(w).await,
            Self::MSG_PETITION_RENAME(c) => c.tokio_write_unencrypted_client(w).await,
            Self::MSG_SET_DUNGEON_DIFFICULTY(c) => c.tokio_write_unencrypted_client(w).await,
            Self::MSG_MOVE_START_ASCEND(c) => c.tokio_write_unencrypted_client(w).await,
            Self::MSG_MOVE_STOP_ASCEND(c) => c.tokio_write_unencrypted_client(w).await,
            Self::MSG_MOVE_START_DESCEND(c) => c.tokio_write_unencrypted_client(w).await,
            Self::CMSG_CHAR_CREATE(c) => c.tokio_write_unencrypted_client(w).await,
            Self::CMSG_CHAR_ENUM(c) => c.tokio_write_unencrypted_client(w).await,
            Self::CMSG_CHAR_DELETE(c) => c.tokio_write_unencrypted_client(w).await,
            Self::CMSG_PLAYER_LOGIN(c) => c.tokio_write_unencrypted_client(w).await,
            Self::CMSG_PLAYER_LOGOUT(c) => c.tokio_write_unencrypted_client(w).await,
            Self::CMSG_LOGOUT_REQUEST(c) => c.tokio_write_unencrypted_client(w).await,
            Self::CMSG_LOGOUT_CANCEL(c) => c.tokio_write_unencrypted_client(w).await,
            Self::CMSG_NAME_QUERY(c) => c.tokio_write_unencrypted_client(w).await,
            Self::CMSG_GUILD_CREATE(c) => c.tokio_write_unencrypted_client(w).await,
            Self::CMSG_GUILD_INVITE(c) => c.tokio_write_unencrypted_client(w).await,
            Self::CMSG_GUILD_ACCEPT(c) => c.tokio_write_unencrypted_client(w).await,
            Self::CMSG_GUILD_DECLINE(c) => c.tokio_write_unencrypted_client(w).await,
            Self::CMSG_GUILD_INFO(c) => c.tokio_write_unencrypted_client(w).await,
            Self::CMSG_GUILD_ROSTER(c) => c.tokio_write_unencrypted_client(w).await,
            Self::CMSG_GUILD_PROMOTE(c) => c.tokio_write_unencrypted_client(w).await,
            Self::CMSG_GUILD_DEMOTE(c) => c.tokio_write_unencrypted_client(w).await,
            Self::CMSG_GUILD_LEAVE(c) => c.tokio_write_unencrypted_client(w).await,
            Self::CMSG_GUILD_REMOVE(c) => c.tokio_write_unencrypted_client(w).await,
            Self::CMSG_GUILD_DISBAND(c) => c.tokio_write_unencrypted_client(w).await,
            Self::CMSG_GUILD_LEADER(c) => c.tokio_write_unencrypted_client(w).await,
            Self::CMSG_GUILD_MOTD(c) => c.tokio_write_unencrypted_client(w).await,
            Self::CMSG_MESSAGECHAT(c) => c.tokio_write_unencrypted_client(w).await,
            Self::CMSG_JOIN_CHANNEL(c) => c.tokio_write_unencrypted_client(w).await,
            Self::CMSG_LEAVE_CHANNEL(c) => c.tokio_write_unencrypted_client(w).await,
            Self::CMSG_CHANNEL_LIST(c) => c.tokio_write_unencrypted_client(w).await,
            Self::CMSG_CHANNEL_PASSWORD(c) => c.tokio_write_unencrypted_client(w).await,
            Self::CMSG_CHANNEL_SET_OWNER(c) => c.tokio_write_unencrypted_client(w).await,
            Self::CMSG_CHANNEL_OWNER(c) => c.tokio_write_unencrypted_client(w).await,
            Self::CMSG_CHANNEL_MODERATOR(c) => c.tokio_write_unencrypted_client(w).await,
            Self::CMSG_CHANNEL_UNMODERATOR(c) => c.tokio_write_unencrypted_client(w).await,
            Self::CMSG_CHANNEL_MUTE(c) => c.tokio_write_unencrypted_client(w).await,
            Self::CMSG_CHANNEL_UNMUTE(c) => c.tokio_write_unencrypted_client(w).await,
            Self::CMSG_CHANNEL_INVITE(c) => c.tokio_write_unencrypted_client(w).await,
            Self::CMSG_CHANNEL_KICK(c) => c.tokio_write_unencrypted_client(w).await,
            Self::CMSG_CHANNEL_BAN(c) => c.tokio_write_unencrypted_client(w).await,
            Self::CMSG_CHANNEL_UNBAN(c) => c.tokio_write_unencrypted_client(w).await,
            Self::CMSG_CHANNEL_ANNOUNCEMENTS(c) => c.tokio_write_unencrypted_client(w).await,
            Self::CMSG_CHANNEL_MODERATE(c) => c.tokio_write_unencrypted_client(w).await,
            Self::CMSG_OPEN_ITEM(c) => c.tokio_write_unencrypted_client(w).await,
            Self::CMSG_READ_ITEM(c) => c.tokio_write_unencrypted_client(w).await,
            Self::CMSG_GAMEOBJ_USE(c) => c.tokio_write_unencrypted_client(w).await,
            Self::CMSG_AREATRIGGER(c) => c.tokio_write_unencrypted_client(w).await,
            Self::MSG_MOVE_TELEPORT_ACK(c) => c.tokio_write_unencrypted_client(w).await,
            Self::CMSG_FORCE_RUN_SPEED_CHANGE_ACK(c) => c.tokio_write_unencrypted_client(w).await,
            Self::CMSG_FORCE_RUN_BACK_SPEED_CHANGE_ACK(c) => c.tokio_write_unencrypted_client(w).await,
            Self::CMSG_FORCE_SWIM_SPEED_CHANGE_ACK(c) => c.tokio_write_unencrypted_client(w).await,
            Self::CMSG_FORCE_MOVE_ROOT_ACK(c) => c.tokio_write_unencrypted_client(w).await,
            Self::CMSG_FORCE_MOVE_UNROOT_ACK(c) => c.tokio_write_unencrypted_client(w).await,
            Self::CMSG_NEXT_CINEMATIC_CAMERA(c) => c.tokio_write_unencrypted_client(w).await,
            Self::CMSG_COMPLETE_CINEMATIC(c) => c.tokio_write_unencrypted_client(w).await,
            Self::CMSG_TUTORIAL_FLAG(c) => c.tokio_write_unencrypted_client(w).await,
            Self::CMSG_TUTORIAL_CLEAR(c) => c.tokio_write_unencrypted_client(w).await,
            Self::CMSG_TUTORIAL_RESET(c) => c.tokio_write_unencrypted_client(w).await,
            Self::CMSG_STANDSTATECHANGE(c) => c.tokio_write_unencrypted_client(w).await,
            Self::CMSG_EMOTE(c) => c.tokio_write_unencrypted_client(w).await,
            Self::CMSG_TEXT_EMOTE(c) => c.tokio_write_unencrypted_client(w).await,
            Self::CMSG_AUTOSTORE_LOOT_ITEM(c) => c.tokio_write_unencrypted_client(w).await,
            Self::CMSG_AUTOEQUIP_ITEM(c) => c.tokio_write_unencrypted_client(w).await,
            Self::CMSG_AUTOSTORE_BAG_ITEM(c) => c.tokio_write_unencrypted_client(w).await,
            Self::CMSG_SWAP_ITEM(c) => c.tokio_write_unencrypted_client(w).await,
            Self::CMSG_SWAP_INV_ITEM(c) => c.tokio_write_unencrypted_client(w).await,
            Self::CMSG_SPLIT_ITEM(c) => c.tokio_write_unencrypted_client(w).await,
            Self::CMSG_AUTOEQUIP_ITEM_SLOT(c) => c.tokio_write_unencrypted_client(w).await,
            Self::CMSG_DESTROYITEM(c) => c.tokio_write_unencrypted_client(w).await,
            Self::CMSG_INSPECT(c) => c.tokio_write_unencrypted_client(w).await,
            Self::CMSG_SET_FACTION_ATWAR(c) => c.tokio_write_unencrypted_client(w).await,
            Self::CMSG_SET_ACTION_BUTTON(c) => c.tokio_write_unencrypted_client(w).await,
            Self::CMSG_CANCEL_CHANNELLING(c) => c.tokio_write_unencrypted_client(w).await,
            Self::CMSG_SET_SELECTION(c) => c.tokio_write_unencrypted_client(w).await,
            Self::CMSG_SET_TARGET_OBSOLETE(c) => c.tokio_write_unencrypted_client(w).await,
            Self::CMSG_ATTACKSWING(c) => c.tokio_write_unencrypted_client(w).await,
            Self::CMSG_ATTACKSTOP(c) => c.tokio_write_unencrypted_client(w).await,
            Self::CMSG_REPOP_REQUEST(c) => c.tokio_write_unencrypted_client(w).await,
            Self::CMSG_LOOT(c) => c.tokio_write_unencrypted_client(w).await,
            Self::CMSG_LOOT_MONEY(c) => c.tokio_write_unencrypted_client(w).await,
            Self::CMSG_LOOT_RELEASE(c) => c.tokio_write_unencrypted_client(w).await,
            Self::CMSG_DUEL_ACCEPTED(c) => c.tokio_write_unencrypted_client(w).await,
            Self::CMSG_DUEL_CANCELLED(c) => c.tokio_write_unencrypted_client(w).await,
            Self::CMSG_MOUNTSPECIAL_ANIM(c) => c.tokio_write_unencrypted_client(w).await,
            Self::CMSG_PET_ACTION(c) => c.tokio_write_unencrypted_client(w).await,
            Self::CMSG_PET_ABANDON(c) => c.tokio_write_unencrypted_client(w).await,
            Self::CMSG_GOSSIP_HELLO(c) => c.tokio_write_unencrypted_client(w).await,
            Self::CMSG_GOSSIP_SELECT_OPTION(c) => c.tokio_write_unencrypted_client(w).await,
            Self::CMSG_LIST_INVENTORY(c) => c.tokio_write_unencrypted_client(w).await,
            Self::CMSG_SELL_ITEM(c) => c.tokio_write_unencrypted_client(w).await,
            Self::CMSG_BUY_ITEM(c) => c.tokio_write_unencrypted_client(w).await,
            Self::CMSG_BUY_ITEM_IN_SLOT(c) => c.tokio_write_unencrypted_client(w).await,
            Self::CMSG_ACTIVATETAXI(c) => c.tokio_write_unencrypted_client(w).await,
            Self::CMSG_BINDER_ACTIVATE(c) => c.tokio_write_unencrypted_client(w).await,
            Self::CMSG_BANKER_ACTIVATE(c) => c.tokio_write_unencrypted_client(w).await,
            Self::CMSG_BUY_BANK_SLOT(c) => c.tokio_write_unencrypted_client(w).await,
            Self::CMSG_PETITION_SHOWLIST(c) => c.tokio_write_unencrypted_client(w).await,
            Self::CMSG_PETITION_BUY(c) => c.tokio_write_unencrypted_client(w).await,
            Self::CMSG_PETITION_SHOW_SIGNATURES(c) => c.tokio_write_unencrypted_client(w).await,
            Self::CMSG_PETITION_SIGN(c) => c.tokio_write_unencrypted_client(w).await,
            Self::CMSG_OFFER_PETITION(c) => c.tokio_write_unencrypted_client(w).await,
            Self::CMSG_TURN_IN_PETITION(c) => c.tokio_write_unencrypted_client(w).await,
            Self::CMSG_BUG(c) => c.tokio_write_unencrypted_client(w).await,
            Self::CMSG_PLAYED_TIME(c) => c.tokio_write_unencrypted_client(w).await,
            Self::CMSG_QUERY_TIME(c) => c.tokio_write_unencrypted_client(w).await,
            Self::CMSG_WRAP_ITEM(c) => c.tokio_write_unencrypted_client(w).await,
            Self::CMSG_PING(c) => c.tokio_write_unencrypted_client(w).await,
            Self::CMSG_SETSHEATHED(c) => c.tokio_write_unencrypted_client(w).await,
            Self::CMSG_AUTH_SESSION(c) => c.tokio_write_unencrypted_client(w).await,
            Self::MSG_SAVE_GUILD_EMBLEM(c) => c.tokio_write_unencrypted_client(w).await,
            Self::CMSG_ZONEUPDATE(c) => c.tokio_write_unencrypted_client(w).await,
            Self::MSG_RANDOM_ROLL(c) => c.tokio_write_unencrypted_client(w).await,
            Self::CMSG_GMTICKET_UPDATETEXT(c) => c.tokio_write_unencrypted_client(w).await,
            Self::CMSG_REQUEST_ACCOUNT_DATA(c) => c.tokio_write_unencrypted_client(w).await,
            Self::CMSG_UPDATE_ACCOUNT_DATA(c) => c.tokio_write_unencrypted_client(w).await,
            Self::CMSG_GMTICKET_GETTICKET(c) => c.tokio_write_unencrypted_client(w).await,
            Self::CMSG_GMTICKET_DELETETICKET(c) => c.tokio_write_unencrypted_client(w).await,
            Self::CMSG_GMTICKET_SYSTEMSTATUS(c) => c.tokio_write_unencrypted_client(w).await,
            Self::CMSG_CHAT_IGNORED(c) => c.tokio_write_unencrypted_client(w).await,
            Self::CMSG_GUILD_RANK(c) => c.tokio_write_unencrypted_client(w).await,
            Self::CMSG_GUILD_ADD_RANK(c) => c.tokio_write_unencrypted_client(w).await,
            Self::CMSG_GUILD_DEL_RANK(c) => c.tokio_write_unencrypted_client(w).await,
            Self::CMSG_GUILD_SET_PUBLIC_NOTE(c) => c.tokio_write_unencrypted_client(w).await,
            Self::CMSG_GUILD_SET_OFFICER_NOTE(c) => c.tokio_write_unencrypted_client(w).await,
            Self::CMSG_SEND_MAIL(c) => c.tokio_write_unencrypted_client(w).await,
            Self::CMSG_GET_MAIL_LIST(c) => c.tokio_write_unencrypted_client(w).await,
            Self::CMSG_BATTLEFIELD_LIST(c) => c.tokio_write_unencrypted_client(w).await,
            Self::CMSG_MAIL_TAKE_MONEY(c) => c.tokio_write_unencrypted_client(w).await,
            Self::CMSG_MAIL_TAKE_ITEM(c) => c.tokio_write_unencrypted_client(w).await,
            Self::CMSG_MAIL_MARK_AS_READ(c) => c.tokio_write_unencrypted_client(w).await,
            Self::CMSG_MAIL_RETURN_TO_SENDER(c) => c.tokio_write_unencrypted_client(w).await,
            Self::CMSG_MAIL_DELETE(c) => c.tokio_write_unencrypted_client(w).await,
            Self::CMSG_MAIL_CREATE_TEXT_ITEM(c) => c.tokio_write_unencrypted_client(w).await,
            Self::MSG_AUCTION_HELLO(c) => c.tokio_write_unencrypted_client(w).await,
            Self::CMSG_AUCTION_SELL_ITEM(c) => c.tokio_write_unencrypted_client(w).await,
            Self::CMSG_AUCTION_REMOVE_ITEM(c) => c.tokio_write_unencrypted_client(w).await,
            Self::CMSG_AUCTION_LIST_ITEMS(c) => c.tokio_write_unencrypted_client(w).await,
            Self::CMSG_AUCTION_LIST_OWNER_ITEMS(c) => c.tokio_write_unencrypted_client(w).await,
            Self::CMSG_AUCTION_PLACE_BID(c) => c.tokio_write_unencrypted_client(w).await,
            Self::CMSG_AUCTION_LIST_BIDDER_ITEMS(c) => c.tokio_write_unencrypted_client(w).await,
            Self::CMSG_SET_AMMO(c) => c.tokio_write_unencrypted_client(w).await,
            Self::CMSG_SET_ACTIVE_MOVER(c) => c.tokio_write_unencrypted_client(w).await,
            Self::CMSG_BUY_STABLE_SLOT(c) => c.tokio_write_unencrypted_client(w).await,
            Self::CMSG_AUTOSTORE_BANK_ITEM(c) => c.tokio_write_unencrypted_client(w).await,
            Self::CMSG_AUTOBANK_ITEM(c) => c.tokio_write_unencrypted_client(w).await,
            Self::CMSG_BUYBACK_ITEM(c) => c.tokio_write_unencrypted_client(w).await,
            Self::CMSG_LOOT_ROLL(c) => c.tokio_write_unencrypted_client(w).await,
            Self::CMSG_LOOT_MASTER_GIVE(c) => c.tokio_write_unencrypted_client(w).await,
            Self::CMSG_REPAIR_ITEM(c) => c.tokio_write_unencrypted_client(w).await,
            Self::CMSG_TOGGLE_HELM(c) => c.tokio_write_unencrypted_client(w).await,
            Self::CMSG_TOGGLE_CLOAK(c) => c.tokio_write_unencrypted_client(w).await,
            Self::CMSG_SET_ACTIONBAR_TOGGLES(c) => c.tokio_write_unencrypted_client(w).await,
            Self::CMSG_CHAR_RENAME(c) => c.tokio_write_unencrypted_client(w).await,
            Self::CMSG_MOVE_FALL_RESET(c) => c.tokio_write_unencrypted_client(w).await,
            Self::CMSG_BATTLEFIELD_STATUS(c) => c.tokio_write_unencrypted_client(w).await,
            Self::CMSG_BATTLEFIELD_PORT(c) => c.tokio_write_unencrypted_client(w).await,
            Self::CMSG_BATTLEMASTER_HELLO(c) => c.tokio_write_unencrypted_client(w).await,
            Self::CMSG_FORCE_WALK_SPEED_CHANGE_ACK(c) => c.tokio_write_unencrypted_client(w).await,
            Self::CMSG_FORCE_SWIM_BACK_SPEED_CHANGE_ACK(c) => c.tokio_write_unencrypted_client(w).await,
            Self::CMSG_FORCE_TURN_RATE_CHANGE_ACK(c) => c.tokio_write_unencrypted_client(w).await,
            Self::CMSG_LEAVE_BATTLEFIELD(c) => c.tokio_write_unencrypted_client(w).await,
            Self::MSG_BATTLEGROUND_PLAYER_POSITIONS(c) => c.tokio_write_unencrypted_client(w).await,
            Self::CMSG_BATTLEMASTER_JOIN(c) => c.tokio_write_unencrypted_client(w).await,
            Self::CMSG_GUILD_INFO_TEXT(c) => c.tokio_write_unencrypted_client(w).await,
            Self::CMSG_ACTIVATETAXIEXPRESS(c) => c.tokio_write_unencrypted_client(w).await,
            Self::CMSG_SET_WATCHED_FACTION(c) => c.tokio_write_unencrypted_client(w).await,
            Self::CMSG_GMSURVEY_SUBMIT(c) => c.tokio_write_unencrypted_client(w).await,
            Self::CMSG_MOVE_SET_FLY(c) => c.tokio_write_unencrypted_client(w).await,
            Self::CMSG_REALM_SPLIT(c) => c.tokio_write_unencrypted_client(w).await,
            Self::CMSG_MOVE_CHNG_TRANSPORT(c) => c.tokio_write_unencrypted_client(w).await,
            Self::CMSG_TIME_SYNC_RESP(c) => c.tokio_write_unencrypted_client(w).await,
            Self::CMSG_SET_ACTIVE_VOICE_CHANNEL(c) => c.tokio_write_unencrypted_client(w).await,
            Self::CMSG_WORLD_STATE_UI_TIMER_UPDATE(c) => c.tokio_write_unencrypted_client(w).await,
            Self::CMSG_READY_FOR_ACCOUNT_DATA_TIMES(c) => c.tokio_write_unencrypted_client(w).await,
        }
    }

    #[cfg(all(feature = "async-std", feature = "encryption"))]
    pub async fn astd_write_encrypted_client<W: async_std::io::WriteExt + Unpin + Send>(&self, w: &mut W, e: &mut ClientEncrypterHalf) -> Result<(), std::io::Error> {
        match self {
            Self::MSG_MOVE_START_FORWARD(c) => c.astd_write_encrypted_client(w, e).await,
            Self::MSG_MOVE_START_BACKWARD(c) => c.astd_write_encrypted_client(w, e).await,
            Self::MSG_MOVE_STOP(c) => c.astd_write_encrypted_client(w, e).await,
            Self::MSG_MOVE_START_STRAFE_LEFT(c) => c.astd_write_encrypted_client(w, e).await,
            Self::MSG_MOVE_START_STRAFE_RIGHT(c) => c.astd_write_encrypted_client(w, e).await,
            Self::MSG_MOVE_STOP_STRAFE(c) => c.astd_write_encrypted_client(w, e).await,
            Self::MSG_MOVE_JUMP(c) => c.astd_write_encrypted_client(w, e).await,
            Self::MSG_MOVE_START_TURN_LEFT(c) => c.astd_write_encrypted_client(w, e).await,
            Self::MSG_MOVE_START_TURN_RIGHT(c) => c.astd_write_encrypted_client(w, e).await,
            Self::MSG_MOVE_STOP_TURN(c) => c.astd_write_encrypted_client(w, e).await,
            Self::MSG_MOVE_START_PITCH_UP(c) => c.astd_write_encrypted_client(w, e).await,
            Self::MSG_MOVE_START_PITCH_DOWN(c) => c.astd_write_encrypted_client(w, e).await,
            Self::MSG_MOVE_STOP_PITCH(c) => c.astd_write_encrypted_client(w, e).await,
            Self::MSG_MOVE_SET_RUN_MODE(c) => c.astd_write_encrypted_client(w, e).await,
            Self::MSG_MOVE_SET_WALK_MODE(c) => c.astd_write_encrypted_client(w, e).await,
            Self::MSG_MOVE_FALL_LAND(c) => c.astd_write_encrypted_client(w, e).await,
            Self::MSG_MOVE_START_SWIM(c) => c.astd_write_encrypted_client(w, e).await,
            Self::MSG_MOVE_STOP_SWIM(c) => c.astd_write_encrypted_client(w, e).await,
            Self::MSG_MOVE_SET_FACING(c) => c.astd_write_encrypted_client(w, e).await,
            Self::MSG_MOVE_SET_PITCH(c) => c.astd_write_encrypted_client(w, e).await,
            Self::MSG_MOVE_WORLDPORT_ACK(c) => c.astd_write_encrypted_client(w, e).await,
            Self::MSG_MOVE_HEARTBEAT(c) => c.astd_write_encrypted_client(w, e).await,
            Self::MSG_PETITION_DECLINE(c) => c.astd_write_encrypted_client(w, e).await,
            Self::MSG_TABARDVENDOR_ACTIVATE(c) => c.astd_write_encrypted_client(w, e).await,
            Self::MSG_PETITION_RENAME(c) => c.astd_write_encrypted_client(w, e).await,
            Self::MSG_SET_DUNGEON_DIFFICULTY(c) => c.astd_write_encrypted_client(w, e).await,
            Self::MSG_MOVE_START_ASCEND(c) => c.astd_write_encrypted_client(w, e).await,
            Self::MSG_MOVE_STOP_ASCEND(c) => c.astd_write_encrypted_client(w, e).await,
            Self::MSG_MOVE_START_DESCEND(c) => c.astd_write_encrypted_client(w, e).await,
            Self::CMSG_CHAR_CREATE(c) => c.astd_write_encrypted_client(w, e).await,
            Self::CMSG_CHAR_ENUM(c) => c.astd_write_encrypted_client(w, e).await,
            Self::CMSG_CHAR_DELETE(c) => c.astd_write_encrypted_client(w, e).await,
            Self::CMSG_PLAYER_LOGIN(c) => c.astd_write_encrypted_client(w, e).await,
            Self::CMSG_PLAYER_LOGOUT(c) => c.astd_write_encrypted_client(w, e).await,
            Self::CMSG_LOGOUT_REQUEST(c) => c.astd_write_encrypted_client(w, e).await,
            Self::CMSG_LOGOUT_CANCEL(c) => c.astd_write_encrypted_client(w, e).await,
            Self::CMSG_NAME_QUERY(c) => c.astd_write_encrypted_client(w, e).await,
            Self::CMSG_GUILD_CREATE(c) => c.astd_write_encrypted_client(w, e).await,
            Self::CMSG_GUILD_INVITE(c) => c.astd_write_encrypted_client(w, e).await,
            Self::CMSG_GUILD_ACCEPT(c) => c.astd_write_encrypted_client(w, e).await,
            Self::CMSG_GUILD_DECLINE(c) => c.astd_write_encrypted_client(w, e).await,
            Self::CMSG_GUILD_INFO(c) => c.astd_write_encrypted_client(w, e).await,
            Self::CMSG_GUILD_ROSTER(c) => c.astd_write_encrypted_client(w, e).await,
            Self::CMSG_GUILD_PROMOTE(c) => c.astd_write_encrypted_client(w, e).await,
            Self::CMSG_GUILD_DEMOTE(c) => c.astd_write_encrypted_client(w, e).await,
            Self::CMSG_GUILD_LEAVE(c) => c.astd_write_encrypted_client(w, e).await,
            Self::CMSG_GUILD_REMOVE(c) => c.astd_write_encrypted_client(w, e).await,
            Self::CMSG_GUILD_DISBAND(c) => c.astd_write_encrypted_client(w, e).await,
            Self::CMSG_GUILD_LEADER(c) => c.astd_write_encrypted_client(w, e).await,
            Self::CMSG_GUILD_MOTD(c) => c.astd_write_encrypted_client(w, e).await,
            Self::CMSG_MESSAGECHAT(c) => c.astd_write_encrypted_client(w, e).await,
            Self::CMSG_JOIN_CHANNEL(c) => c.astd_write_encrypted_client(w, e).await,
            Self::CMSG_LEAVE_CHANNEL(c) => c.astd_write_encrypted_client(w, e).await,
            Self::CMSG_CHANNEL_LIST(c) => c.astd_write_encrypted_client(w, e).await,
            Self::CMSG_CHANNEL_PASSWORD(c) => c.astd_write_encrypted_client(w, e).await,
            Self::CMSG_CHANNEL_SET_OWNER(c) => c.astd_write_encrypted_client(w, e).await,
            Self::CMSG_CHANNEL_OWNER(c) => c.astd_write_encrypted_client(w, e).await,
            Self::CMSG_CHANNEL_MODERATOR(c) => c.astd_write_encrypted_client(w, e).await,
            Self::CMSG_CHANNEL_UNMODERATOR(c) => c.astd_write_encrypted_client(w, e).await,
            Self::CMSG_CHANNEL_MUTE(c) => c.astd_write_encrypted_client(w, e).await,
            Self::CMSG_CHANNEL_UNMUTE(c) => c.astd_write_encrypted_client(w, e).await,
            Self::CMSG_CHANNEL_INVITE(c) => c.astd_write_encrypted_client(w, e).await,
            Self::CMSG_CHANNEL_KICK(c) => c.astd_write_encrypted_client(w, e).await,
            Self::CMSG_CHANNEL_BAN(c) => c.astd_write_encrypted_client(w, e).await,
            Self::CMSG_CHANNEL_UNBAN(c) => c.astd_write_encrypted_client(w, e).await,
            Self::CMSG_CHANNEL_ANNOUNCEMENTS(c) => c.astd_write_encrypted_client(w, e).await,
            Self::CMSG_CHANNEL_MODERATE(c) => c.astd_write_encrypted_client(w, e).await,
            Self::CMSG_OPEN_ITEM(c) => c.astd_write_encrypted_client(w, e).await,
            Self::CMSG_READ_ITEM(c) => c.astd_write_encrypted_client(w, e).await,
            Self::CMSG_GAMEOBJ_USE(c) => c.astd_write_encrypted_client(w, e).await,
            Self::CMSG_AREATRIGGER(c) => c.astd_write_encrypted_client(w, e).await,
            Self::MSG_MOVE_TELEPORT_ACK(c) => c.astd_write_encrypted_client(w, e).await,
            Self::CMSG_FORCE_RUN_SPEED_CHANGE_ACK(c) => c.astd_write_encrypted_client(w, e).await,
            Self::CMSG_FORCE_RUN_BACK_SPEED_CHANGE_ACK(c) => c.astd_write_encrypted_client(w, e).await,
            Self::CMSG_FORCE_SWIM_SPEED_CHANGE_ACK(c) => c.astd_write_encrypted_client(w, e).await,
            Self::CMSG_FORCE_MOVE_ROOT_ACK(c) => c.astd_write_encrypted_client(w, e).await,
            Self::CMSG_FORCE_MOVE_UNROOT_ACK(c) => c.astd_write_encrypted_client(w, e).await,
            Self::CMSG_NEXT_CINEMATIC_CAMERA(c) => c.astd_write_encrypted_client(w, e).await,
            Self::CMSG_COMPLETE_CINEMATIC(c) => c.astd_write_encrypted_client(w, e).await,
            Self::CMSG_TUTORIAL_FLAG(c) => c.astd_write_encrypted_client(w, e).await,
            Self::CMSG_TUTORIAL_CLEAR(c) => c.astd_write_encrypted_client(w, e).await,
            Self::CMSG_TUTORIAL_RESET(c) => c.astd_write_encrypted_client(w, e).await,
            Self::CMSG_STANDSTATECHANGE(c) => c.astd_write_encrypted_client(w, e).await,
            Self::CMSG_EMOTE(c) => c.astd_write_encrypted_client(w, e).await,
            Self::CMSG_TEXT_EMOTE(c) => c.astd_write_encrypted_client(w, e).await,
            Self::CMSG_AUTOSTORE_LOOT_ITEM(c) => c.astd_write_encrypted_client(w, e).await,
            Self::CMSG_AUTOEQUIP_ITEM(c) => c.astd_write_encrypted_client(w, e).await,
            Self::CMSG_AUTOSTORE_BAG_ITEM(c) => c.astd_write_encrypted_client(w, e).await,
            Self::CMSG_SWAP_ITEM(c) => c.astd_write_encrypted_client(w, e).await,
            Self::CMSG_SWAP_INV_ITEM(c) => c.astd_write_encrypted_client(w, e).await,
            Self::CMSG_SPLIT_ITEM(c) => c.astd_write_encrypted_client(w, e).await,
            Self::CMSG_AUTOEQUIP_ITEM_SLOT(c) => c.astd_write_encrypted_client(w, e).await,
            Self::CMSG_DESTROYITEM(c) => c.astd_write_encrypted_client(w, e).await,
            Self::CMSG_INSPECT(c) => c.astd_write_encrypted_client(w, e).await,
            Self::CMSG_SET_FACTION_ATWAR(c) => c.astd_write_encrypted_client(w, e).await,
            Self::CMSG_SET_ACTION_BUTTON(c) => c.astd_write_encrypted_client(w, e).await,
            Self::CMSG_CANCEL_CHANNELLING(c) => c.astd_write_encrypted_client(w, e).await,
            Self::CMSG_SET_SELECTION(c) => c.astd_write_encrypted_client(w, e).await,
            Self::CMSG_SET_TARGET_OBSOLETE(c) => c.astd_write_encrypted_client(w, e).await,
            Self::CMSG_ATTACKSWING(c) => c.astd_write_encrypted_client(w, e).await,
            Self::CMSG_ATTACKSTOP(c) => c.astd_write_encrypted_client(w, e).await,
            Self::CMSG_REPOP_REQUEST(c) => c.astd_write_encrypted_client(w, e).await,
            Self::CMSG_LOOT(c) => c.astd_write_encrypted_client(w, e).await,
            Self::CMSG_LOOT_MONEY(c) => c.astd_write_encrypted_client(w, e).await,
            Self::CMSG_LOOT_RELEASE(c) => c.astd_write_encrypted_client(w, e).await,
            Self::CMSG_DUEL_ACCEPTED(c) => c.astd_write_encrypted_client(w, e).await,
            Self::CMSG_DUEL_CANCELLED(c) => c.astd_write_encrypted_client(w, e).await,
            Self::CMSG_MOUNTSPECIAL_ANIM(c) => c.astd_write_encrypted_client(w, e).await,
            Self::CMSG_PET_ACTION(c) => c.astd_write_encrypted_client(w, e).await,
            Self::CMSG_PET_ABANDON(c) => c.astd_write_encrypted_client(w, e).await,
            Self::CMSG_GOSSIP_HELLO(c) => c.astd_write_encrypted_client(w, e).await,
            Self::CMSG_GOSSIP_SELECT_OPTION(c) => c.astd_write_encrypted_client(w, e).await,
            Self::CMSG_LIST_INVENTORY(c) => c.astd_write_encrypted_client(w, e).await,
            Self::CMSG_SELL_ITEM(c) => c.astd_write_encrypted_client(w, e).await,
            Self::CMSG_BUY_ITEM(c) => c.astd_write_encrypted_client(w, e).await,
            Self::CMSG_BUY_ITEM_IN_SLOT(c) => c.astd_write_encrypted_client(w, e).await,
            Self::CMSG_ACTIVATETAXI(c) => c.astd_write_encrypted_client(w, e).await,
            Self::CMSG_BINDER_ACTIVATE(c) => c.astd_write_encrypted_client(w, e).await,
            Self::CMSG_BANKER_ACTIVATE(c) => c.astd_write_encrypted_client(w, e).await,
            Self::CMSG_BUY_BANK_SLOT(c) => c.astd_write_encrypted_client(w, e).await,
            Self::CMSG_PETITION_SHOWLIST(c) => c.astd_write_encrypted_client(w, e).await,
            Self::CMSG_PETITION_BUY(c) => c.astd_write_encrypted_client(w, e).await,
            Self::CMSG_PETITION_SHOW_SIGNATURES(c) => c.astd_write_encrypted_client(w, e).await,
            Self::CMSG_PETITION_SIGN(c) => c.astd_write_encrypted_client(w, e).await,
            Self::CMSG_OFFER_PETITION(c) => c.astd_write_encrypted_client(w, e).await,
            Self::CMSG_TURN_IN_PETITION(c) => c.astd_write_encrypted_client(w, e).await,
            Self::CMSG_BUG(c) => c.astd_write_encrypted_client(w, e).await,
            Self::CMSG_PLAYED_TIME(c) => c.astd_write_encrypted_client(w, e).await,
            Self::CMSG_QUERY_TIME(c) => c.astd_write_encrypted_client(w, e).await,
            Self::CMSG_WRAP_ITEM(c) => c.astd_write_encrypted_client(w, e).await,
            Self::CMSG_PING(c) => c.astd_write_encrypted_client(w, e).await,
            Self::CMSG_SETSHEATHED(c) => c.astd_write_encrypted_client(w, e).await,
            Self::CMSG_AUTH_SESSION(c) => c.astd_write_encrypted_client(w, e).await,
            Self::MSG_SAVE_GUILD_EMBLEM(c) => c.astd_write_encrypted_client(w, e).await,
            Self::CMSG_ZONEUPDATE(c) => c.astd_write_encrypted_client(w, e).await,
            Self::MSG_RANDOM_ROLL(c) => c.astd_write_encrypted_client(w, e).await,
            Self::CMSG_GMTICKET_UPDATETEXT(c) => c.astd_write_encrypted_client(w, e).await,
            Self::CMSG_REQUEST_ACCOUNT_DATA(c) => c.astd_write_encrypted_client(w, e).await,
            Self::CMSG_UPDATE_ACCOUNT_DATA(c) => c.astd_write_encrypted_client(w, e).await,
            Self::CMSG_GMTICKET_GETTICKET(c) => c.astd_write_encrypted_client(w, e).await,
            Self::CMSG_GMTICKET_DELETETICKET(c) => c.astd_write_encrypted_client(w, e).await,
            Self::CMSG_GMTICKET_SYSTEMSTATUS(c) => c.astd_write_encrypted_client(w, e).await,
            Self::CMSG_CHAT_IGNORED(c) => c.astd_write_encrypted_client(w, e).await,
            Self::CMSG_GUILD_RANK(c) => c.astd_write_encrypted_client(w, e).await,
            Self::CMSG_GUILD_ADD_RANK(c) => c.astd_write_encrypted_client(w, e).await,
            Self::CMSG_GUILD_DEL_RANK(c) => c.astd_write_encrypted_client(w, e).await,
            Self::CMSG_GUILD_SET_PUBLIC_NOTE(c) => c.astd_write_encrypted_client(w, e).await,
            Self::CMSG_GUILD_SET_OFFICER_NOTE(c) => c.astd_write_encrypted_client(w, e).await,
            Self::CMSG_SEND_MAIL(c) => c.astd_write_encrypted_client(w, e).await,
            Self::CMSG_GET_MAIL_LIST(c) => c.astd_write_encrypted_client(w, e).await,
            Self::CMSG_BATTLEFIELD_LIST(c) => c.astd_write_encrypted_client(w, e).await,
            Self::CMSG_MAIL_TAKE_MONEY(c) => c.astd_write_encrypted_client(w, e).await,
            Self::CMSG_MAIL_TAKE_ITEM(c) => c.astd_write_encrypted_client(w, e).await,
            Self::CMSG_MAIL_MARK_AS_READ(c) => c.astd_write_encrypted_client(w, e).await,
            Self::CMSG_MAIL_RETURN_TO_SENDER(c) => c.astd_write_encrypted_client(w, e).await,
            Self::CMSG_MAIL_DELETE(c) => c.astd_write_encrypted_client(w, e).await,
            Self::CMSG_MAIL_CREATE_TEXT_ITEM(c) => c.astd_write_encrypted_client(w, e).await,
            Self::MSG_AUCTION_HELLO(c) => c.astd_write_encrypted_client(w, e).await,
            Self::CMSG_AUCTION_SELL_ITEM(c) => c.astd_write_encrypted_client(w, e).await,
            Self::CMSG_AUCTION_REMOVE_ITEM(c) => c.astd_write_encrypted_client(w, e).await,
            Self::CMSG_AUCTION_LIST_ITEMS(c) => c.astd_write_encrypted_client(w, e).await,
            Self::CMSG_AUCTION_LIST_OWNER_ITEMS(c) => c.astd_write_encrypted_client(w, e).await,
            Self::CMSG_AUCTION_PLACE_BID(c) => c.astd_write_encrypted_client(w, e).await,
            Self::CMSG_AUCTION_LIST_BIDDER_ITEMS(c) => c.astd_write_encrypted_client(w, e).await,
            Self::CMSG_SET_AMMO(c) => c.astd_write_encrypted_client(w, e).await,
            Self::CMSG_SET_ACTIVE_MOVER(c) => c.astd_write_encrypted_client(w, e).await,
            Self::CMSG_BUY_STABLE_SLOT(c) => c.astd_write_encrypted_client(w, e).await,
            Self::CMSG_AUTOSTORE_BANK_ITEM(c) => c.astd_write_encrypted_client(w, e).await,
            Self::CMSG_AUTOBANK_ITEM(c) => c.astd_write_encrypted_client(w, e).await,
            Self::CMSG_BUYBACK_ITEM(c) => c.astd_write_encrypted_client(w, e).await,
            Self::CMSG_LOOT_ROLL(c) => c.astd_write_encrypted_client(w, e).await,
            Self::CMSG_LOOT_MASTER_GIVE(c) => c.astd_write_encrypted_client(w, e).await,
            Self::CMSG_REPAIR_ITEM(c) => c.astd_write_encrypted_client(w, e).await,
            Self::CMSG_TOGGLE_HELM(c) => c.astd_write_encrypted_client(w, e).await,
            Self::CMSG_TOGGLE_CLOAK(c) => c.astd_write_encrypted_client(w, e).await,
            Self::CMSG_SET_ACTIONBAR_TOGGLES(c) => c.astd_write_encrypted_client(w, e).await,
            Self::CMSG_CHAR_RENAME(c) => c.astd_write_encrypted_client(w, e).await,
            Self::CMSG_MOVE_FALL_RESET(c) => c.astd_write_encrypted_client(w, e).await,
            Self::CMSG_BATTLEFIELD_STATUS(c) => c.astd_write_encrypted_client(w, e).await,
            Self::CMSG_BATTLEFIELD_PORT(c) => c.astd_write_encrypted_client(w, e).await,
            Self::CMSG_BATTLEMASTER_HELLO(c) => c.astd_write_encrypted_client(w, e).await,
            Self::CMSG_FORCE_WALK_SPEED_CHANGE_ACK(c) => c.astd_write_encrypted_client(w, e).await,
            Self::CMSG_FORCE_SWIM_BACK_SPEED_CHANGE_ACK(c) => c.astd_write_encrypted_client(w, e).await,
            Self::CMSG_FORCE_TURN_RATE_CHANGE_ACK(c) => c.astd_write_encrypted_client(w, e).await,
            Self::CMSG_LEAVE_BATTLEFIELD(c) => c.astd_write_encrypted_client(w, e).await,
            Self::MSG_BATTLEGROUND_PLAYER_POSITIONS(c) => c.astd_write_encrypted_client(w, e).await,
            Self::CMSG_BATTLEMASTER_JOIN(c) => c.astd_write_encrypted_client(w, e).await,
            Self::CMSG_GUILD_INFO_TEXT(c) => c.astd_write_encrypted_client(w, e).await,
            Self::CMSG_ACTIVATETAXIEXPRESS(c) => c.astd_write_encrypted_client(w, e).await,
            Self::CMSG_SET_WATCHED_FACTION(c) => c.astd_write_encrypted_client(w, e).await,
            Self::CMSG_GMSURVEY_SUBMIT(c) => c.astd_write_encrypted_client(w, e).await,
            Self::CMSG_MOVE_SET_FLY(c) => c.astd_write_encrypted_client(w, e).await,
            Self::CMSG_REALM_SPLIT(c) => c.astd_write_encrypted_client(w, e).await,
            Self::CMSG_MOVE_CHNG_TRANSPORT(c) => c.astd_write_encrypted_client(w, e).await,
            Self::CMSG_TIME_SYNC_RESP(c) => c.astd_write_encrypted_client(w, e).await,
            Self::CMSG_SET_ACTIVE_VOICE_CHANNEL(c) => c.astd_write_encrypted_client(w, e).await,
            Self::CMSG_WORLD_STATE_UI_TIMER_UPDATE(c) => c.astd_write_encrypted_client(w, e).await,
            Self::CMSG_READY_FOR_ACCOUNT_DATA_TIMES(c) => c.astd_write_encrypted_client(w, e).await,
        }
    }

    #[cfg(feature = "async-std")]
    pub async fn astd_write_unencrypted_client<W: async_std::io::WriteExt + Unpin + Send>(&self, w: &mut W) -> Result<(), std::io::Error> {
        match self {
            Self::MSG_MOVE_START_FORWARD(c) => c.astd_write_unencrypted_client(w).await,
            Self::MSG_MOVE_START_BACKWARD(c) => c.astd_write_unencrypted_client(w).await,
            Self::MSG_MOVE_STOP(c) => c.astd_write_unencrypted_client(w).await,
            Self::MSG_MOVE_START_STRAFE_LEFT(c) => c.astd_write_unencrypted_client(w).await,
            Self::MSG_MOVE_START_STRAFE_RIGHT(c) => c.astd_write_unencrypted_client(w).await,
            Self::MSG_MOVE_STOP_STRAFE(c) => c.astd_write_unencrypted_client(w).await,
            Self::MSG_MOVE_JUMP(c) => c.astd_write_unencrypted_client(w).await,
            Self::MSG_MOVE_START_TURN_LEFT(c) => c.astd_write_unencrypted_client(w).await,
            Self::MSG_MOVE_START_TURN_RIGHT(c) => c.astd_write_unencrypted_client(w).await,
            Self::MSG_MOVE_STOP_TURN(c) => c.astd_write_unencrypted_client(w).await,
            Self::MSG_MOVE_START_PITCH_UP(c) => c.astd_write_unencrypted_client(w).await,
            Self::MSG_MOVE_START_PITCH_DOWN(c) => c.astd_write_unencrypted_client(w).await,
            Self::MSG_MOVE_STOP_PITCH(c) => c.astd_write_unencrypted_client(w).await,
            Self::MSG_MOVE_SET_RUN_MODE(c) => c.astd_write_unencrypted_client(w).await,
            Self::MSG_MOVE_SET_WALK_MODE(c) => c.astd_write_unencrypted_client(w).await,
            Self::MSG_MOVE_FALL_LAND(c) => c.astd_write_unencrypted_client(w).await,
            Self::MSG_MOVE_START_SWIM(c) => c.astd_write_unencrypted_client(w).await,
            Self::MSG_MOVE_STOP_SWIM(c) => c.astd_write_unencrypted_client(w).await,
            Self::MSG_MOVE_SET_FACING(c) => c.astd_write_unencrypted_client(w).await,
            Self::MSG_MOVE_SET_PITCH(c) => c.astd_write_unencrypted_client(w).await,
            Self::MSG_MOVE_WORLDPORT_ACK(c) => c.astd_write_unencrypted_client(w).await,
            Self::MSG_MOVE_HEARTBEAT(c) => c.astd_write_unencrypted_client(w).await,
            Self::MSG_PETITION_DECLINE(c) => c.astd_write_unencrypted_client(w).await,
            Self::MSG_TABARDVENDOR_ACTIVATE(c) => c.astd_write_unencrypted_client(w).await,
            Self::MSG_PETITION_RENAME(c) => c.astd_write_unencrypted_client(w).await,
            Self::MSG_SET_DUNGEON_DIFFICULTY(c) => c.astd_write_unencrypted_client(w).await,
            Self::MSG_MOVE_START_ASCEND(c) => c.astd_write_unencrypted_client(w).await,
            Self::MSG_MOVE_STOP_ASCEND(c) => c.astd_write_unencrypted_client(w).await,
            Self::MSG_MOVE_START_DESCEND(c) => c.astd_write_unencrypted_client(w).await,
            Self::CMSG_CHAR_CREATE(c) => c.astd_write_unencrypted_client(w).await,
            Self::CMSG_CHAR_ENUM(c) => c.astd_write_unencrypted_client(w).await,
            Self::CMSG_CHAR_DELETE(c) => c.astd_write_unencrypted_client(w).await,
            Self::CMSG_PLAYER_LOGIN(c) => c.astd_write_unencrypted_client(w).await,
            Self::CMSG_PLAYER_LOGOUT(c) => c.astd_write_unencrypted_client(w).await,
            Self::CMSG_LOGOUT_REQUEST(c) => c.astd_write_unencrypted_client(w).await,
            Self::CMSG_LOGOUT_CANCEL(c) => c.astd_write_unencrypted_client(w).await,
            Self::CMSG_NAME_QUERY(c) => c.astd_write_unencrypted_client(w).await,
            Self::CMSG_GUILD_CREATE(c) => c.astd_write_unencrypted_client(w).await,
            Self::CMSG_GUILD_INVITE(c) => c.astd_write_unencrypted_client(w).await,
            Self::CMSG_GUILD_ACCEPT(c) => c.astd_write_unencrypted_client(w).await,
            Self::CMSG_GUILD_DECLINE(c) => c.astd_write_unencrypted_client(w).await,
            Self::CMSG_GUILD_INFO(c) => c.astd_write_unencrypted_client(w).await,
            Self::CMSG_GUILD_ROSTER(c) => c.astd_write_unencrypted_client(w).await,
            Self::CMSG_GUILD_PROMOTE(c) => c.astd_write_unencrypted_client(w).await,
            Self::CMSG_GUILD_DEMOTE(c) => c.astd_write_unencrypted_client(w).await,
            Self::CMSG_GUILD_LEAVE(c) => c.astd_write_unencrypted_client(w).await,
            Self::CMSG_GUILD_REMOVE(c) => c.astd_write_unencrypted_client(w).await,
            Self::CMSG_GUILD_DISBAND(c) => c.astd_write_unencrypted_client(w).await,
            Self::CMSG_GUILD_LEADER(c) => c.astd_write_unencrypted_client(w).await,
            Self::CMSG_GUILD_MOTD(c) => c.astd_write_unencrypted_client(w).await,
            Self::CMSG_MESSAGECHAT(c) => c.astd_write_unencrypted_client(w).await,
            Self::CMSG_JOIN_CHANNEL(c) => c.astd_write_unencrypted_client(w).await,
            Self::CMSG_LEAVE_CHANNEL(c) => c.astd_write_unencrypted_client(w).await,
            Self::CMSG_CHANNEL_LIST(c) => c.astd_write_unencrypted_client(w).await,
            Self::CMSG_CHANNEL_PASSWORD(c) => c.astd_write_unencrypted_client(w).await,
            Self::CMSG_CHANNEL_SET_OWNER(c) => c.astd_write_unencrypted_client(w).await,
            Self::CMSG_CHANNEL_OWNER(c) => c.astd_write_unencrypted_client(w).await,
            Self::CMSG_CHANNEL_MODERATOR(c) => c.astd_write_unencrypted_client(w).await,
            Self::CMSG_CHANNEL_UNMODERATOR(c) => c.astd_write_unencrypted_client(w).await,
            Self::CMSG_CHANNEL_MUTE(c) => c.astd_write_unencrypted_client(w).await,
            Self::CMSG_CHANNEL_UNMUTE(c) => c.astd_write_unencrypted_client(w).await,
            Self::CMSG_CHANNEL_INVITE(c) => c.astd_write_unencrypted_client(w).await,
            Self::CMSG_CHANNEL_KICK(c) => c.astd_write_unencrypted_client(w).await,
            Self::CMSG_CHANNEL_BAN(c) => c.astd_write_unencrypted_client(w).await,
            Self::CMSG_CHANNEL_UNBAN(c) => c.astd_write_unencrypted_client(w).await,
            Self::CMSG_CHANNEL_ANNOUNCEMENTS(c) => c.astd_write_unencrypted_client(w).await,
            Self::CMSG_CHANNEL_MODERATE(c) => c.astd_write_unencrypted_client(w).await,
            Self::CMSG_OPEN_ITEM(c) => c.astd_write_unencrypted_client(w).await,
            Self::CMSG_READ_ITEM(c) => c.astd_write_unencrypted_client(w).await,
            Self::CMSG_GAMEOBJ_USE(c) => c.astd_write_unencrypted_client(w).await,
            Self::CMSG_AREATRIGGER(c) => c.astd_write_unencrypted_client(w).await,
            Self::MSG_MOVE_TELEPORT_ACK(c) => c.astd_write_unencrypted_client(w).await,
            Self::CMSG_FORCE_RUN_SPEED_CHANGE_ACK(c) => c.astd_write_unencrypted_client(w).await,
            Self::CMSG_FORCE_RUN_BACK_SPEED_CHANGE_ACK(c) => c.astd_write_unencrypted_client(w).await,
            Self::CMSG_FORCE_SWIM_SPEED_CHANGE_ACK(c) => c.astd_write_unencrypted_client(w).await,
            Self::CMSG_FORCE_MOVE_ROOT_ACK(c) => c.astd_write_unencrypted_client(w).await,
            Self::CMSG_FORCE_MOVE_UNROOT_ACK(c) => c.astd_write_unencrypted_client(w).await,
            Self::CMSG_NEXT_CINEMATIC_CAMERA(c) => c.astd_write_unencrypted_client(w).await,
            Self::CMSG_COMPLETE_CINEMATIC(c) => c.astd_write_unencrypted_client(w).await,
            Self::CMSG_TUTORIAL_FLAG(c) => c.astd_write_unencrypted_client(w).await,
            Self::CMSG_TUTORIAL_CLEAR(c) => c.astd_write_unencrypted_client(w).await,
            Self::CMSG_TUTORIAL_RESET(c) => c.astd_write_unencrypted_client(w).await,
            Self::CMSG_STANDSTATECHANGE(c) => c.astd_write_unencrypted_client(w).await,
            Self::CMSG_EMOTE(c) => c.astd_write_unencrypted_client(w).await,
            Self::CMSG_TEXT_EMOTE(c) => c.astd_write_unencrypted_client(w).await,
            Self::CMSG_AUTOSTORE_LOOT_ITEM(c) => c.astd_write_unencrypted_client(w).await,
            Self::CMSG_AUTOEQUIP_ITEM(c) => c.astd_write_unencrypted_client(w).await,
            Self::CMSG_AUTOSTORE_BAG_ITEM(c) => c.astd_write_unencrypted_client(w).await,
            Self::CMSG_SWAP_ITEM(c) => c.astd_write_unencrypted_client(w).await,
            Self::CMSG_SWAP_INV_ITEM(c) => c.astd_write_unencrypted_client(w).await,
            Self::CMSG_SPLIT_ITEM(c) => c.astd_write_unencrypted_client(w).await,
            Self::CMSG_AUTOEQUIP_ITEM_SLOT(c) => c.astd_write_unencrypted_client(w).await,
            Self::CMSG_DESTROYITEM(c) => c.astd_write_unencrypted_client(w).await,
            Self::CMSG_INSPECT(c) => c.astd_write_unencrypted_client(w).await,
            Self::CMSG_SET_FACTION_ATWAR(c) => c.astd_write_unencrypted_client(w).await,
            Self::CMSG_SET_ACTION_BUTTON(c) => c.astd_write_unencrypted_client(w).await,
            Self::CMSG_CANCEL_CHANNELLING(c) => c.astd_write_unencrypted_client(w).await,
            Self::CMSG_SET_SELECTION(c) => c.astd_write_unencrypted_client(w).await,
            Self::CMSG_SET_TARGET_OBSOLETE(c) => c.astd_write_unencrypted_client(w).await,
            Self::CMSG_ATTACKSWING(c) => c.astd_write_unencrypted_client(w).await,
            Self::CMSG_ATTACKSTOP(c) => c.astd_write_unencrypted_client(w).await,
            Self::CMSG_REPOP_REQUEST(c) => c.astd_write_unencrypted_client(w).await,
            Self::CMSG_LOOT(c) => c.astd_write_unencrypted_client(w).await,
            Self::CMSG_LOOT_MONEY(c) => c.astd_write_unencrypted_client(w).await,
            Self::CMSG_LOOT_RELEASE(c) => c.astd_write_unencrypted_client(w).await,
            Self::CMSG_DUEL_ACCEPTED(c) => c.astd_write_unencrypted_client(w).await,
            Self::CMSG_DUEL_CANCELLED(c) => c.astd_write_unencrypted_client(w).await,
            Self::CMSG_MOUNTSPECIAL_ANIM(c) => c.astd_write_unencrypted_client(w).await,
            Self::CMSG_PET_ACTION(c) => c.astd_write_unencrypted_client(w).await,
            Self::CMSG_PET_ABANDON(c) => c.astd_write_unencrypted_client(w).await,
            Self::CMSG_GOSSIP_HELLO(c) => c.astd_write_unencrypted_client(w).await,
            Self::CMSG_GOSSIP_SELECT_OPTION(c) => c.astd_write_unencrypted_client(w).await,
            Self::CMSG_LIST_INVENTORY(c) => c.astd_write_unencrypted_client(w).await,
            Self::CMSG_SELL_ITEM(c) => c.astd_write_unencrypted_client(w).await,
            Self::CMSG_BUY_ITEM(c) => c.astd_write_unencrypted_client(w).await,
            Self::CMSG_BUY_ITEM_IN_SLOT(c) => c.astd_write_unencrypted_client(w).await,
            Self::CMSG_ACTIVATETAXI(c) => c.astd_write_unencrypted_client(w).await,
            Self::CMSG_BINDER_ACTIVATE(c) => c.astd_write_unencrypted_client(w).await,
            Self::CMSG_BANKER_ACTIVATE(c) => c.astd_write_unencrypted_client(w).await,
            Self::CMSG_BUY_BANK_SLOT(c) => c.astd_write_unencrypted_client(w).await,
            Self::CMSG_PETITION_SHOWLIST(c) => c.astd_write_unencrypted_client(w).await,
            Self::CMSG_PETITION_BUY(c) => c.astd_write_unencrypted_client(w).await,
            Self::CMSG_PETITION_SHOW_SIGNATURES(c) => c.astd_write_unencrypted_client(w).await,
            Self::CMSG_PETITION_SIGN(c) => c.astd_write_unencrypted_client(w).await,
            Self::CMSG_OFFER_PETITION(c) => c.astd_write_unencrypted_client(w).await,
            Self::CMSG_TURN_IN_PETITION(c) => c.astd_write_unencrypted_client(w).await,
            Self::CMSG_BUG(c) => c.astd_write_unencrypted_client(w).await,
            Self::CMSG_PLAYED_TIME(c) => c.astd_write_unencrypted_client(w).await,
            Self::CMSG_QUERY_TIME(c) => c.astd_write_unencrypted_client(w).await,
            Self::CMSG_WRAP_ITEM(c) => c.astd_write_unencrypted_client(w).await,
            Self::CMSG_PING(c) => c.astd_write_unencrypted_client(w).await,
            Self::CMSG_SETSHEATHED(c) => c.astd_write_unencrypted_client(w).await,
            Self::CMSG_AUTH_SESSION(c) => c.astd_write_unencrypted_client(w).await,
            Self::MSG_SAVE_GUILD_EMBLEM(c) => c.astd_write_unencrypted_client(w).await,
            Self::CMSG_ZONEUPDATE(c) => c.astd_write_unencrypted_client(w).await,
            Self::MSG_RANDOM_ROLL(c) => c.astd_write_unencrypted_client(w).await,
            Self::CMSG_GMTICKET_UPDATETEXT(c) => c.astd_write_unencrypted_client(w).await,
            Self::CMSG_REQUEST_ACCOUNT_DATA(c) => c.astd_write_unencrypted_client(w).await,
            Self::CMSG_UPDATE_ACCOUNT_DATA(c) => c.astd_write_unencrypted_client(w).await,
            Self::CMSG_GMTICKET_GETTICKET(c) => c.astd_write_unencrypted_client(w).await,
            Self::CMSG_GMTICKET_DELETETICKET(c) => c.astd_write_unencrypted_client(w).await,
            Self::CMSG_GMTICKET_SYSTEMSTATUS(c) => c.astd_write_unencrypted_client(w).await,
            Self::CMSG_CHAT_IGNORED(c) => c.astd_write_unencrypted_client(w).await,
            Self::CMSG_GUILD_RANK(c) => c.astd_write_unencrypted_client(w).await,
            Self::CMSG_GUILD_ADD_RANK(c) => c.astd_write_unencrypted_client(w).await,
            Self::CMSG_GUILD_DEL_RANK(c) => c.astd_write_unencrypted_client(w).await,
            Self::CMSG_GUILD_SET_PUBLIC_NOTE(c) => c.astd_write_unencrypted_client(w).await,
            Self::CMSG_GUILD_SET_OFFICER_NOTE(c) => c.astd_write_unencrypted_client(w).await,
            Self::CMSG_SEND_MAIL(c) => c.astd_write_unencrypted_client(w).await,
            Self::CMSG_GET_MAIL_LIST(c) => c.astd_write_unencrypted_client(w).await,
            Self::CMSG_BATTLEFIELD_LIST(c) => c.astd_write_unencrypted_client(w).await,
            Self::CMSG_MAIL_TAKE_MONEY(c) => c.astd_write_unencrypted_client(w).await,
            Self::CMSG_MAIL_TAKE_ITEM(c) => c.astd_write_unencrypted_client(w).await,
            Self::CMSG_MAIL_MARK_AS_READ(c) => c.astd_write_unencrypted_client(w).await,
            Self::CMSG_MAIL_RETURN_TO_SENDER(c) => c.astd_write_unencrypted_client(w).await,
            Self::CMSG_MAIL_DELETE(c) => c.astd_write_unencrypted_client(w).await,
            Self::CMSG_MAIL_CREATE_TEXT_ITEM(c) => c.astd_write_unencrypted_client(w).await,
            Self::MSG_AUCTION_HELLO(c) => c.astd_write_unencrypted_client(w).await,
            Self::CMSG_AUCTION_SELL_ITEM(c) => c.astd_write_unencrypted_client(w).await,
            Self::CMSG_AUCTION_REMOVE_ITEM(c) => c.astd_write_unencrypted_client(w).await,
            Self::CMSG_AUCTION_LIST_ITEMS(c) => c.astd_write_unencrypted_client(w).await,
            Self::CMSG_AUCTION_LIST_OWNER_ITEMS(c) => c.astd_write_unencrypted_client(w).await,
            Self::CMSG_AUCTION_PLACE_BID(c) => c.astd_write_unencrypted_client(w).await,
            Self::CMSG_AUCTION_LIST_BIDDER_ITEMS(c) => c.astd_write_unencrypted_client(w).await,
            Self::CMSG_SET_AMMO(c) => c.astd_write_unencrypted_client(w).await,
            Self::CMSG_SET_ACTIVE_MOVER(c) => c.astd_write_unencrypted_client(w).await,
            Self::CMSG_BUY_STABLE_SLOT(c) => c.astd_write_unencrypted_client(w).await,
            Self::CMSG_AUTOSTORE_BANK_ITEM(c) => c.astd_write_unencrypted_client(w).await,
            Self::CMSG_AUTOBANK_ITEM(c) => c.astd_write_unencrypted_client(w).await,
            Self::CMSG_BUYBACK_ITEM(c) => c.astd_write_unencrypted_client(w).await,
            Self::CMSG_LOOT_ROLL(c) => c.astd_write_unencrypted_client(w).await,
            Self::CMSG_LOOT_MASTER_GIVE(c) => c.astd_write_unencrypted_client(w).await,
            Self::CMSG_REPAIR_ITEM(c) => c.astd_write_unencrypted_client(w).await,
            Self::CMSG_TOGGLE_HELM(c) => c.astd_write_unencrypted_client(w).await,
            Self::CMSG_TOGGLE_CLOAK(c) => c.astd_write_unencrypted_client(w).await,
            Self::CMSG_SET_ACTIONBAR_TOGGLES(c) => c.astd_write_unencrypted_client(w).await,
            Self::CMSG_CHAR_RENAME(c) => c.astd_write_unencrypted_client(w).await,
            Self::CMSG_MOVE_FALL_RESET(c) => c.astd_write_unencrypted_client(w).await,
            Self::CMSG_BATTLEFIELD_STATUS(c) => c.astd_write_unencrypted_client(w).await,
            Self::CMSG_BATTLEFIELD_PORT(c) => c.astd_write_unencrypted_client(w).await,
            Self::CMSG_BATTLEMASTER_HELLO(c) => c.astd_write_unencrypted_client(w).await,
            Self::CMSG_FORCE_WALK_SPEED_CHANGE_ACK(c) => c.astd_write_unencrypted_client(w).await,
            Self::CMSG_FORCE_SWIM_BACK_SPEED_CHANGE_ACK(c) => c.astd_write_unencrypted_client(w).await,
            Self::CMSG_FORCE_TURN_RATE_CHANGE_ACK(c) => c.astd_write_unencrypted_client(w).await,
            Self::CMSG_LEAVE_BATTLEFIELD(c) => c.astd_write_unencrypted_client(w).await,
            Self::MSG_BATTLEGROUND_PLAYER_POSITIONS(c) => c.astd_write_unencrypted_client(w).await,
            Self::CMSG_BATTLEMASTER_JOIN(c) => c.astd_write_unencrypted_client(w).await,
            Self::CMSG_GUILD_INFO_TEXT(c) => c.astd_write_unencrypted_client(w).await,
            Self::CMSG_ACTIVATETAXIEXPRESS(c) => c.astd_write_unencrypted_client(w).await,
            Self::CMSG_SET_WATCHED_FACTION(c) => c.astd_write_unencrypted_client(w).await,
            Self::CMSG_GMSURVEY_SUBMIT(c) => c.astd_write_unencrypted_client(w).await,
            Self::CMSG_MOVE_SET_FLY(c) => c.astd_write_unencrypted_client(w).await,
            Self::CMSG_REALM_SPLIT(c) => c.astd_write_unencrypted_client(w).await,
            Self::CMSG_MOVE_CHNG_TRANSPORT(c) => c.astd_write_unencrypted_client(w).await,
            Self::CMSG_TIME_SYNC_RESP(c) => c.astd_write_unencrypted_client(w).await,
            Self::CMSG_SET_ACTIVE_VOICE_CHANNEL(c) => c.astd_write_unencrypted_client(w).await,
            Self::CMSG_WORLD_STATE_UI_TIMER_UPDATE(c) => c.astd_write_unencrypted_client(w).await,
            Self::CMSG_READY_FOR_ACCOUNT_DATA_TIMES(c) => c.astd_write_unencrypted_client(w).await,
        }
    }

    pub fn movement_info(&self) -> Option<&MovementInfo> {
        match self {
            Self::CMSG_MOVE_FALL_RESET(c) => Some(&c.info),
            Self::CMSG_MOVE_SET_FLY(c) => Some(&c.info),
            Self::CMSG_MOVE_CHNG_TRANSPORT(c) => Some(&c.info),
            _ => None,
        }
    }

}

impl std::fmt::Display for ClientOpcodeMessage {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            ClientOpcodeMessage::MSG_MOVE_START_FORWARD(_) => "MSG_MOVE_START_FORWARD",
            ClientOpcodeMessage::MSG_MOVE_START_BACKWARD(_) => "MSG_MOVE_START_BACKWARD",
            ClientOpcodeMessage::MSG_MOVE_STOP(_) => "MSG_MOVE_STOP",
            ClientOpcodeMessage::MSG_MOVE_START_STRAFE_LEFT(_) => "MSG_MOVE_START_STRAFE_LEFT",
            ClientOpcodeMessage::MSG_MOVE_START_STRAFE_RIGHT(_) => "MSG_MOVE_START_STRAFE_RIGHT",
            ClientOpcodeMessage::MSG_MOVE_STOP_STRAFE(_) => "MSG_MOVE_STOP_STRAFE",
            ClientOpcodeMessage::MSG_MOVE_JUMP(_) => "MSG_MOVE_JUMP",
            ClientOpcodeMessage::MSG_MOVE_START_TURN_LEFT(_) => "MSG_MOVE_START_TURN_LEFT",
            ClientOpcodeMessage::MSG_MOVE_START_TURN_RIGHT(_) => "MSG_MOVE_START_TURN_RIGHT",
            ClientOpcodeMessage::MSG_MOVE_STOP_TURN(_) => "MSG_MOVE_STOP_TURN",
            ClientOpcodeMessage::MSG_MOVE_START_PITCH_UP(_) => "MSG_MOVE_START_PITCH_UP",
            ClientOpcodeMessage::MSG_MOVE_START_PITCH_DOWN(_) => "MSG_MOVE_START_PITCH_DOWN",
            ClientOpcodeMessage::MSG_MOVE_STOP_PITCH(_) => "MSG_MOVE_STOP_PITCH",
            ClientOpcodeMessage::MSG_MOVE_SET_RUN_MODE(_) => "MSG_MOVE_SET_RUN_MODE",
            ClientOpcodeMessage::MSG_MOVE_SET_WALK_MODE(_) => "MSG_MOVE_SET_WALK_MODE",
            ClientOpcodeMessage::MSG_MOVE_FALL_LAND(_) => "MSG_MOVE_FALL_LAND",
            ClientOpcodeMessage::MSG_MOVE_START_SWIM(_) => "MSG_MOVE_START_SWIM",
            ClientOpcodeMessage::MSG_MOVE_STOP_SWIM(_) => "MSG_MOVE_STOP_SWIM",
            ClientOpcodeMessage::MSG_MOVE_SET_FACING(_) => "MSG_MOVE_SET_FACING",
            ClientOpcodeMessage::MSG_MOVE_SET_PITCH(_) => "MSG_MOVE_SET_PITCH",
            ClientOpcodeMessage::MSG_MOVE_WORLDPORT_ACK(_) => "MSG_MOVE_WORLDPORT_ACK",
            ClientOpcodeMessage::MSG_MOVE_HEARTBEAT(_) => "MSG_MOVE_HEARTBEAT",
            ClientOpcodeMessage::MSG_PETITION_DECLINE(_) => "MSG_PETITION_DECLINE",
            ClientOpcodeMessage::MSG_TABARDVENDOR_ACTIVATE(_) => "MSG_TABARDVENDOR_ACTIVATE",
            ClientOpcodeMessage::MSG_PETITION_RENAME(_) => "MSG_PETITION_RENAME",
            ClientOpcodeMessage::MSG_SET_DUNGEON_DIFFICULTY(_) => "MSG_SET_DUNGEON_DIFFICULTY",
            ClientOpcodeMessage::MSG_MOVE_START_ASCEND(_) => "MSG_MOVE_START_ASCEND",
            ClientOpcodeMessage::MSG_MOVE_STOP_ASCEND(_) => "MSG_MOVE_STOP_ASCEND",
            ClientOpcodeMessage::MSG_MOVE_START_DESCEND(_) => "MSG_MOVE_START_DESCEND",
            ClientOpcodeMessage::CMSG_CHAR_CREATE(_) => "CMSG_CHAR_CREATE",
            ClientOpcodeMessage::CMSG_CHAR_ENUM(_) => "CMSG_CHAR_ENUM",
            ClientOpcodeMessage::CMSG_CHAR_DELETE(_) => "CMSG_CHAR_DELETE",
            ClientOpcodeMessage::CMSG_PLAYER_LOGIN(_) => "CMSG_PLAYER_LOGIN",
            ClientOpcodeMessage::CMSG_PLAYER_LOGOUT(_) => "CMSG_PLAYER_LOGOUT",
            ClientOpcodeMessage::CMSG_LOGOUT_REQUEST(_) => "CMSG_LOGOUT_REQUEST",
            ClientOpcodeMessage::CMSG_LOGOUT_CANCEL(_) => "CMSG_LOGOUT_CANCEL",
            ClientOpcodeMessage::CMSG_NAME_QUERY(_) => "CMSG_NAME_QUERY",
            ClientOpcodeMessage::CMSG_GUILD_CREATE(_) => "CMSG_GUILD_CREATE",
            ClientOpcodeMessage::CMSG_GUILD_INVITE(_) => "CMSG_GUILD_INVITE",
            ClientOpcodeMessage::CMSG_GUILD_ACCEPT(_) => "CMSG_GUILD_ACCEPT",
            ClientOpcodeMessage::CMSG_GUILD_DECLINE(_) => "CMSG_GUILD_DECLINE",
            ClientOpcodeMessage::CMSG_GUILD_INFO(_) => "CMSG_GUILD_INFO",
            ClientOpcodeMessage::CMSG_GUILD_ROSTER(_) => "CMSG_GUILD_ROSTER",
            ClientOpcodeMessage::CMSG_GUILD_PROMOTE(_) => "CMSG_GUILD_PROMOTE",
            ClientOpcodeMessage::CMSG_GUILD_DEMOTE(_) => "CMSG_GUILD_DEMOTE",
            ClientOpcodeMessage::CMSG_GUILD_LEAVE(_) => "CMSG_GUILD_LEAVE",
            ClientOpcodeMessage::CMSG_GUILD_REMOVE(_) => "CMSG_GUILD_REMOVE",
            ClientOpcodeMessage::CMSG_GUILD_DISBAND(_) => "CMSG_GUILD_DISBAND",
            ClientOpcodeMessage::CMSG_GUILD_LEADER(_) => "CMSG_GUILD_LEADER",
            ClientOpcodeMessage::CMSG_GUILD_MOTD(_) => "CMSG_GUILD_MOTD",
            ClientOpcodeMessage::CMSG_MESSAGECHAT(_) => "CMSG_MESSAGECHAT",
            ClientOpcodeMessage::CMSG_JOIN_CHANNEL(_) => "CMSG_JOIN_CHANNEL",
            ClientOpcodeMessage::CMSG_LEAVE_CHANNEL(_) => "CMSG_LEAVE_CHANNEL",
            ClientOpcodeMessage::CMSG_CHANNEL_LIST(_) => "CMSG_CHANNEL_LIST",
            ClientOpcodeMessage::CMSG_CHANNEL_PASSWORD(_) => "CMSG_CHANNEL_PASSWORD",
            ClientOpcodeMessage::CMSG_CHANNEL_SET_OWNER(_) => "CMSG_CHANNEL_SET_OWNER",
            ClientOpcodeMessage::CMSG_CHANNEL_OWNER(_) => "CMSG_CHANNEL_OWNER",
            ClientOpcodeMessage::CMSG_CHANNEL_MODERATOR(_) => "CMSG_CHANNEL_MODERATOR",
            ClientOpcodeMessage::CMSG_CHANNEL_UNMODERATOR(_) => "CMSG_CHANNEL_UNMODERATOR",
            ClientOpcodeMessage::CMSG_CHANNEL_MUTE(_) => "CMSG_CHANNEL_MUTE",
            ClientOpcodeMessage::CMSG_CHANNEL_UNMUTE(_) => "CMSG_CHANNEL_UNMUTE",
            ClientOpcodeMessage::CMSG_CHANNEL_INVITE(_) => "CMSG_CHANNEL_INVITE",
            ClientOpcodeMessage::CMSG_CHANNEL_KICK(_) => "CMSG_CHANNEL_KICK",
            ClientOpcodeMessage::CMSG_CHANNEL_BAN(_) => "CMSG_CHANNEL_BAN",
            ClientOpcodeMessage::CMSG_CHANNEL_UNBAN(_) => "CMSG_CHANNEL_UNBAN",
            ClientOpcodeMessage::CMSG_CHANNEL_ANNOUNCEMENTS(_) => "CMSG_CHANNEL_ANNOUNCEMENTS",
            ClientOpcodeMessage::CMSG_CHANNEL_MODERATE(_) => "CMSG_CHANNEL_MODERATE",
            ClientOpcodeMessage::CMSG_OPEN_ITEM(_) => "CMSG_OPEN_ITEM",
            ClientOpcodeMessage::CMSG_READ_ITEM(_) => "CMSG_READ_ITEM",
            ClientOpcodeMessage::CMSG_GAMEOBJ_USE(_) => "CMSG_GAMEOBJ_USE",
            ClientOpcodeMessage::CMSG_AREATRIGGER(_) => "CMSG_AREATRIGGER",
            ClientOpcodeMessage::MSG_MOVE_TELEPORT_ACK(_) => "MSG_MOVE_TELEPORT_ACK_Client",
            ClientOpcodeMessage::CMSG_FORCE_RUN_SPEED_CHANGE_ACK(_) => "CMSG_FORCE_RUN_SPEED_CHANGE_ACK",
            ClientOpcodeMessage::CMSG_FORCE_RUN_BACK_SPEED_CHANGE_ACK(_) => "CMSG_FORCE_RUN_BACK_SPEED_CHANGE_ACK",
            ClientOpcodeMessage::CMSG_FORCE_SWIM_SPEED_CHANGE_ACK(_) => "CMSG_FORCE_SWIM_SPEED_CHANGE_ACK",
            ClientOpcodeMessage::CMSG_FORCE_MOVE_ROOT_ACK(_) => "CMSG_FORCE_MOVE_ROOT_ACK",
            ClientOpcodeMessage::CMSG_FORCE_MOVE_UNROOT_ACK(_) => "CMSG_FORCE_MOVE_UNROOT_ACK",
            ClientOpcodeMessage::CMSG_NEXT_CINEMATIC_CAMERA(_) => "CMSG_NEXT_CINEMATIC_CAMERA",
            ClientOpcodeMessage::CMSG_COMPLETE_CINEMATIC(_) => "CMSG_COMPLETE_CINEMATIC",
            ClientOpcodeMessage::CMSG_TUTORIAL_FLAG(_) => "CMSG_TUTORIAL_FLAG",
            ClientOpcodeMessage::CMSG_TUTORIAL_CLEAR(_) => "CMSG_TUTORIAL_CLEAR",
            ClientOpcodeMessage::CMSG_TUTORIAL_RESET(_) => "CMSG_TUTORIAL_RESET",
            ClientOpcodeMessage::CMSG_STANDSTATECHANGE(_) => "CMSG_STANDSTATECHANGE",
            ClientOpcodeMessage::CMSG_EMOTE(_) => "CMSG_EMOTE",
            ClientOpcodeMessage::CMSG_TEXT_EMOTE(_) => "CMSG_TEXT_EMOTE",
            ClientOpcodeMessage::CMSG_AUTOSTORE_LOOT_ITEM(_) => "CMSG_AUTOSTORE_LOOT_ITEM",
            ClientOpcodeMessage::CMSG_AUTOEQUIP_ITEM(_) => "CMSG_AUTOEQUIP_ITEM",
            ClientOpcodeMessage::CMSG_AUTOSTORE_BAG_ITEM(_) => "CMSG_AUTOSTORE_BAG_ITEM",
            ClientOpcodeMessage::CMSG_SWAP_ITEM(_) => "CMSG_SWAP_ITEM",
            ClientOpcodeMessage::CMSG_SWAP_INV_ITEM(_) => "CMSG_SWAP_INV_ITEM",
            ClientOpcodeMessage::CMSG_SPLIT_ITEM(_) => "CMSG_SPLIT_ITEM",
            ClientOpcodeMessage::CMSG_AUTOEQUIP_ITEM_SLOT(_) => "CMSG_AUTOEQUIP_ITEM_SLOT",
            ClientOpcodeMessage::CMSG_DESTROYITEM(_) => "CMSG_DESTROYITEM",
            ClientOpcodeMessage::CMSG_INSPECT(_) => "CMSG_INSPECT",
            ClientOpcodeMessage::CMSG_SET_FACTION_ATWAR(_) => "CMSG_SET_FACTION_ATWAR",
            ClientOpcodeMessage::CMSG_SET_ACTION_BUTTON(_) => "CMSG_SET_ACTION_BUTTON",
            ClientOpcodeMessage::CMSG_CANCEL_CHANNELLING(_) => "CMSG_CANCEL_CHANNELLING",
            ClientOpcodeMessage::CMSG_SET_SELECTION(_) => "CMSG_SET_SELECTION",
            ClientOpcodeMessage::CMSG_SET_TARGET_OBSOLETE(_) => "CMSG_SET_TARGET_OBSOLETE",
            ClientOpcodeMessage::CMSG_ATTACKSWING(_) => "CMSG_ATTACKSWING",
            ClientOpcodeMessage::CMSG_ATTACKSTOP(_) => "CMSG_ATTACKSTOP",
            ClientOpcodeMessage::CMSG_REPOP_REQUEST(_) => "CMSG_REPOP_REQUEST",
            ClientOpcodeMessage::CMSG_LOOT(_) => "CMSG_LOOT",
            ClientOpcodeMessage::CMSG_LOOT_MONEY(_) => "CMSG_LOOT_MONEY",
            ClientOpcodeMessage::CMSG_LOOT_RELEASE(_) => "CMSG_LOOT_RELEASE",
            ClientOpcodeMessage::CMSG_DUEL_ACCEPTED(_) => "CMSG_DUEL_ACCEPTED",
            ClientOpcodeMessage::CMSG_DUEL_CANCELLED(_) => "CMSG_DUEL_CANCELLED",
            ClientOpcodeMessage::CMSG_MOUNTSPECIAL_ANIM(_) => "CMSG_MOUNTSPECIAL_ANIM",
            ClientOpcodeMessage::CMSG_PET_ACTION(_) => "CMSG_PET_ACTION",
            ClientOpcodeMessage::CMSG_PET_ABANDON(_) => "CMSG_PET_ABANDON",
            ClientOpcodeMessage::CMSG_GOSSIP_HELLO(_) => "CMSG_GOSSIP_HELLO",
            ClientOpcodeMessage::CMSG_GOSSIP_SELECT_OPTION(_) => "CMSG_GOSSIP_SELECT_OPTION",
            ClientOpcodeMessage::CMSG_LIST_INVENTORY(_) => "CMSG_LIST_INVENTORY",
            ClientOpcodeMessage::CMSG_SELL_ITEM(_) => "CMSG_SELL_ITEM",
            ClientOpcodeMessage::CMSG_BUY_ITEM(_) => "CMSG_BUY_ITEM",
            ClientOpcodeMessage::CMSG_BUY_ITEM_IN_SLOT(_) => "CMSG_BUY_ITEM_IN_SLOT",
            ClientOpcodeMessage::CMSG_ACTIVATETAXI(_) => "CMSG_ACTIVATETAXI",
            ClientOpcodeMessage::CMSG_BINDER_ACTIVATE(_) => "CMSG_BINDER_ACTIVATE",
            ClientOpcodeMessage::CMSG_BANKER_ACTIVATE(_) => "CMSG_BANKER_ACTIVATE",
            ClientOpcodeMessage::CMSG_BUY_BANK_SLOT(_) => "CMSG_BUY_BANK_SLOT",
            ClientOpcodeMessage::CMSG_PETITION_SHOWLIST(_) => "CMSG_PETITION_SHOWLIST",
            ClientOpcodeMessage::CMSG_PETITION_BUY(_) => "CMSG_PETITION_BUY",
            ClientOpcodeMessage::CMSG_PETITION_SHOW_SIGNATURES(_) => "CMSG_PETITION_SHOW_SIGNATURES",
            ClientOpcodeMessage::CMSG_PETITION_SIGN(_) => "CMSG_PETITION_SIGN",
            ClientOpcodeMessage::CMSG_OFFER_PETITION(_) => "CMSG_OFFER_PETITION",
            ClientOpcodeMessage::CMSG_TURN_IN_PETITION(_) => "CMSG_TURN_IN_PETITION",
            ClientOpcodeMessage::CMSG_BUG(_) => "CMSG_BUG",
            ClientOpcodeMessage::CMSG_PLAYED_TIME(_) => "CMSG_PLAYED_TIME",
            ClientOpcodeMessage::CMSG_QUERY_TIME(_) => "CMSG_QUERY_TIME",
            ClientOpcodeMessage::CMSG_WRAP_ITEM(_) => "CMSG_WRAP_ITEM",
            ClientOpcodeMessage::CMSG_PING(_) => "CMSG_PING",
            ClientOpcodeMessage::CMSG_SETSHEATHED(_) => "CMSG_SETSHEATHED",
            ClientOpcodeMessage::CMSG_AUTH_SESSION(_) => "CMSG_AUTH_SESSION",
            ClientOpcodeMessage::MSG_SAVE_GUILD_EMBLEM(_) => "MSG_SAVE_GUILD_EMBLEM_Client",
            ClientOpcodeMessage::CMSG_ZONEUPDATE(_) => "CMSG_ZONEUPDATE",
            ClientOpcodeMessage::MSG_RANDOM_ROLL(_) => "MSG_RANDOM_ROLL_Client",
            ClientOpcodeMessage::CMSG_GMTICKET_UPDATETEXT(_) => "CMSG_GMTICKET_UPDATETEXT",
            ClientOpcodeMessage::CMSG_REQUEST_ACCOUNT_DATA(_) => "CMSG_REQUEST_ACCOUNT_DATA",
            ClientOpcodeMessage::CMSG_UPDATE_ACCOUNT_DATA(_) => "CMSG_UPDATE_ACCOUNT_DATA",
            ClientOpcodeMessage::CMSG_GMTICKET_GETTICKET(_) => "CMSG_GMTICKET_GETTICKET",
            ClientOpcodeMessage::CMSG_GMTICKET_DELETETICKET(_) => "CMSG_GMTICKET_DELETETICKET",
            ClientOpcodeMessage::CMSG_GMTICKET_SYSTEMSTATUS(_) => "CMSG_GMTICKET_SYSTEMSTATUS",
            ClientOpcodeMessage::CMSG_CHAT_IGNORED(_) => "CMSG_CHAT_IGNORED",
            ClientOpcodeMessage::CMSG_GUILD_RANK(_) => "CMSG_GUILD_RANK",
            ClientOpcodeMessage::CMSG_GUILD_ADD_RANK(_) => "CMSG_GUILD_ADD_RANK",
            ClientOpcodeMessage::CMSG_GUILD_DEL_RANK(_) => "CMSG_GUILD_DEL_RANK",
            ClientOpcodeMessage::CMSG_GUILD_SET_PUBLIC_NOTE(_) => "CMSG_GUILD_SET_PUBLIC_NOTE",
            ClientOpcodeMessage::CMSG_GUILD_SET_OFFICER_NOTE(_) => "CMSG_GUILD_SET_OFFICER_NOTE",
            ClientOpcodeMessage::CMSG_SEND_MAIL(_) => "CMSG_SEND_MAIL",
            ClientOpcodeMessage::CMSG_GET_MAIL_LIST(_) => "CMSG_GET_MAIL_LIST",
            ClientOpcodeMessage::CMSG_BATTLEFIELD_LIST(_) => "CMSG_BATTLEFIELD_LIST",
            ClientOpcodeMessage::CMSG_MAIL_TAKE_MONEY(_) => "CMSG_MAIL_TAKE_MONEY",
            ClientOpcodeMessage::CMSG_MAIL_TAKE_ITEM(_) => "CMSG_MAIL_TAKE_ITEM",
            ClientOpcodeMessage::CMSG_MAIL_MARK_AS_READ(_) => "CMSG_MAIL_MARK_AS_READ",
            ClientOpcodeMessage::CMSG_MAIL_RETURN_TO_SENDER(_) => "CMSG_MAIL_RETURN_TO_SENDER",
            ClientOpcodeMessage::CMSG_MAIL_DELETE(_) => "CMSG_MAIL_DELETE",
            ClientOpcodeMessage::CMSG_MAIL_CREATE_TEXT_ITEM(_) => "CMSG_MAIL_CREATE_TEXT_ITEM",
            ClientOpcodeMessage::MSG_AUCTION_HELLO(_) => "MSG_AUCTION_HELLO_Client",
            ClientOpcodeMessage::CMSG_AUCTION_SELL_ITEM(_) => "CMSG_AUCTION_SELL_ITEM",
            ClientOpcodeMessage::CMSG_AUCTION_REMOVE_ITEM(_) => "CMSG_AUCTION_REMOVE_ITEM",
            ClientOpcodeMessage::CMSG_AUCTION_LIST_ITEMS(_) => "CMSG_AUCTION_LIST_ITEMS",
            ClientOpcodeMessage::CMSG_AUCTION_LIST_OWNER_ITEMS(_) => "CMSG_AUCTION_LIST_OWNER_ITEMS",
            ClientOpcodeMessage::CMSG_AUCTION_PLACE_BID(_) => "CMSG_AUCTION_PLACE_BID",
            ClientOpcodeMessage::CMSG_AUCTION_LIST_BIDDER_ITEMS(_) => "CMSG_AUCTION_LIST_BIDDER_ITEMS",
            ClientOpcodeMessage::CMSG_SET_AMMO(_) => "CMSG_SET_AMMO",
            ClientOpcodeMessage::CMSG_SET_ACTIVE_MOVER(_) => "CMSG_SET_ACTIVE_MOVER",
            ClientOpcodeMessage::CMSG_BUY_STABLE_SLOT(_) => "CMSG_BUY_STABLE_SLOT",
            ClientOpcodeMessage::CMSG_AUTOSTORE_BANK_ITEM(_) => "CMSG_AUTOSTORE_BANK_ITEM",
            ClientOpcodeMessage::CMSG_AUTOBANK_ITEM(_) => "CMSG_AUTOBANK_ITEM",
            ClientOpcodeMessage::CMSG_BUYBACK_ITEM(_) => "CMSG_BUYBACK_ITEM",
            ClientOpcodeMessage::CMSG_LOOT_ROLL(_) => "CMSG_LOOT_ROLL",
            ClientOpcodeMessage::CMSG_LOOT_MASTER_GIVE(_) => "CMSG_LOOT_MASTER_GIVE",
            ClientOpcodeMessage::CMSG_REPAIR_ITEM(_) => "CMSG_REPAIR_ITEM",
            ClientOpcodeMessage::CMSG_TOGGLE_HELM(_) => "CMSG_TOGGLE_HELM",
            ClientOpcodeMessage::CMSG_TOGGLE_CLOAK(_) => "CMSG_TOGGLE_CLOAK",
            ClientOpcodeMessage::CMSG_SET_ACTIONBAR_TOGGLES(_) => "CMSG_SET_ACTIONBAR_TOGGLES",
            ClientOpcodeMessage::CMSG_CHAR_RENAME(_) => "CMSG_CHAR_RENAME",
            ClientOpcodeMessage::CMSG_MOVE_FALL_RESET(_) => "CMSG_MOVE_FALL_RESET",
            ClientOpcodeMessage::CMSG_BATTLEFIELD_STATUS(_) => "CMSG_BATTLEFIELD_STATUS",
            ClientOpcodeMessage::CMSG_BATTLEFIELD_PORT(_) => "CMSG_BATTLEFIELD_PORT",
            ClientOpcodeMessage::CMSG_BATTLEMASTER_HELLO(_) => "CMSG_BATTLEMASTER_HELLO",
            ClientOpcodeMessage::CMSG_FORCE_WALK_SPEED_CHANGE_ACK(_) => "CMSG_FORCE_WALK_SPEED_CHANGE_ACK",
            ClientOpcodeMessage::CMSG_FORCE_SWIM_BACK_SPEED_CHANGE_ACK(_) => "CMSG_FORCE_SWIM_BACK_SPEED_CHANGE_ACK",
            ClientOpcodeMessage::CMSG_FORCE_TURN_RATE_CHANGE_ACK(_) => "CMSG_FORCE_TURN_RATE_CHANGE_ACK",
            ClientOpcodeMessage::CMSG_LEAVE_BATTLEFIELD(_) => "CMSG_LEAVE_BATTLEFIELD",
            ClientOpcodeMessage::MSG_BATTLEGROUND_PLAYER_POSITIONS(_) => "MSG_BATTLEGROUND_PLAYER_POSITIONS_Client",
            ClientOpcodeMessage::CMSG_BATTLEMASTER_JOIN(_) => "CMSG_BATTLEMASTER_JOIN",
            ClientOpcodeMessage::CMSG_GUILD_INFO_TEXT(_) => "CMSG_GUILD_INFO_TEXT",
            ClientOpcodeMessage::CMSG_ACTIVATETAXIEXPRESS(_) => "CMSG_ACTIVATETAXIEXPRESS",
            ClientOpcodeMessage::CMSG_SET_WATCHED_FACTION(_) => "CMSG_SET_WATCHED_FACTION",
            ClientOpcodeMessage::CMSG_GMSURVEY_SUBMIT(_) => "CMSG_GMSURVEY_SUBMIT",
            ClientOpcodeMessage::CMSG_MOVE_SET_FLY(_) => "CMSG_MOVE_SET_FLY",
            ClientOpcodeMessage::CMSG_REALM_SPLIT(_) => "CMSG_REALM_SPLIT",
            ClientOpcodeMessage::CMSG_MOVE_CHNG_TRANSPORT(_) => "CMSG_MOVE_CHNG_TRANSPORT",
            ClientOpcodeMessage::CMSG_TIME_SYNC_RESP(_) => "CMSG_TIME_SYNC_RESP",
            ClientOpcodeMessage::CMSG_SET_ACTIVE_VOICE_CHANNEL(_) => "CMSG_SET_ACTIVE_VOICE_CHANNEL",
            ClientOpcodeMessage::CMSG_WORLD_STATE_UI_TIMER_UPDATE(_) => "CMSG_WORLD_STATE_UI_TIMER_UPDATE",
            ClientOpcodeMessage::CMSG_READY_FOR_ACCOUNT_DATA_TIMES(_) => "CMSG_READY_FOR_ACCOUNT_DATA_TIMES",
        })
    }
}

impl From<MSG_MOVE_START_FORWARD> for ClientOpcodeMessage {
    fn from(c: MSG_MOVE_START_FORWARD) -> Self {
        Self::MSG_MOVE_START_FORWARD(c)
    }
}

impl From<MSG_MOVE_START_BACKWARD> for ClientOpcodeMessage {
    fn from(c: MSG_MOVE_START_BACKWARD) -> Self {
        Self::MSG_MOVE_START_BACKWARD(c)
    }
}

impl From<MSG_MOVE_STOP> for ClientOpcodeMessage {
    fn from(c: MSG_MOVE_STOP) -> Self {
        Self::MSG_MOVE_STOP(c)
    }
}

impl From<MSG_MOVE_START_STRAFE_LEFT> for ClientOpcodeMessage {
    fn from(c: MSG_MOVE_START_STRAFE_LEFT) -> Self {
        Self::MSG_MOVE_START_STRAFE_LEFT(c)
    }
}

impl From<MSG_MOVE_START_STRAFE_RIGHT> for ClientOpcodeMessage {
    fn from(c: MSG_MOVE_START_STRAFE_RIGHT) -> Self {
        Self::MSG_MOVE_START_STRAFE_RIGHT(c)
    }
}

impl From<MSG_MOVE_STOP_STRAFE> for ClientOpcodeMessage {
    fn from(c: MSG_MOVE_STOP_STRAFE) -> Self {
        Self::MSG_MOVE_STOP_STRAFE(c)
    }
}

impl From<MSG_MOVE_JUMP> for ClientOpcodeMessage {
    fn from(c: MSG_MOVE_JUMP) -> Self {
        Self::MSG_MOVE_JUMP(c)
    }
}

impl From<MSG_MOVE_START_TURN_LEFT> for ClientOpcodeMessage {
    fn from(c: MSG_MOVE_START_TURN_LEFT) -> Self {
        Self::MSG_MOVE_START_TURN_LEFT(c)
    }
}

impl From<MSG_MOVE_START_TURN_RIGHT> for ClientOpcodeMessage {
    fn from(c: MSG_MOVE_START_TURN_RIGHT) -> Self {
        Self::MSG_MOVE_START_TURN_RIGHT(c)
    }
}

impl From<MSG_MOVE_STOP_TURN> for ClientOpcodeMessage {
    fn from(c: MSG_MOVE_STOP_TURN) -> Self {
        Self::MSG_MOVE_STOP_TURN(c)
    }
}

impl From<MSG_MOVE_START_PITCH_UP> for ClientOpcodeMessage {
    fn from(c: MSG_MOVE_START_PITCH_UP) -> Self {
        Self::MSG_MOVE_START_PITCH_UP(c)
    }
}

impl From<MSG_MOVE_START_PITCH_DOWN> for ClientOpcodeMessage {
    fn from(c: MSG_MOVE_START_PITCH_DOWN) -> Self {
        Self::MSG_MOVE_START_PITCH_DOWN(c)
    }
}

impl From<MSG_MOVE_STOP_PITCH> for ClientOpcodeMessage {
    fn from(c: MSG_MOVE_STOP_PITCH) -> Self {
        Self::MSG_MOVE_STOP_PITCH(c)
    }
}

impl From<MSG_MOVE_SET_RUN_MODE> for ClientOpcodeMessage {
    fn from(c: MSG_MOVE_SET_RUN_MODE) -> Self {
        Self::MSG_MOVE_SET_RUN_MODE(c)
    }
}

impl From<MSG_MOVE_SET_WALK_MODE> for ClientOpcodeMessage {
    fn from(c: MSG_MOVE_SET_WALK_MODE) -> Self {
        Self::MSG_MOVE_SET_WALK_MODE(c)
    }
}

impl From<MSG_MOVE_FALL_LAND> for ClientOpcodeMessage {
    fn from(c: MSG_MOVE_FALL_LAND) -> Self {
        Self::MSG_MOVE_FALL_LAND(c)
    }
}

impl From<MSG_MOVE_START_SWIM> for ClientOpcodeMessage {
    fn from(c: MSG_MOVE_START_SWIM) -> Self {
        Self::MSG_MOVE_START_SWIM(c)
    }
}

impl From<MSG_MOVE_STOP_SWIM> for ClientOpcodeMessage {
    fn from(c: MSG_MOVE_STOP_SWIM) -> Self {
        Self::MSG_MOVE_STOP_SWIM(c)
    }
}

impl From<MSG_MOVE_SET_FACING> for ClientOpcodeMessage {
    fn from(c: MSG_MOVE_SET_FACING) -> Self {
        Self::MSG_MOVE_SET_FACING(c)
    }
}

impl From<MSG_MOVE_SET_PITCH> for ClientOpcodeMessage {
    fn from(c: MSG_MOVE_SET_PITCH) -> Self {
        Self::MSG_MOVE_SET_PITCH(c)
    }
}

impl From<MSG_MOVE_WORLDPORT_ACK> for ClientOpcodeMessage {
    fn from(c: MSG_MOVE_WORLDPORT_ACK) -> Self {
        Self::MSG_MOVE_WORLDPORT_ACK(c)
    }
}

impl From<MSG_MOVE_HEARTBEAT> for ClientOpcodeMessage {
    fn from(c: MSG_MOVE_HEARTBEAT) -> Self {
        Self::MSG_MOVE_HEARTBEAT(c)
    }
}

impl From<MSG_PETITION_DECLINE> for ClientOpcodeMessage {
    fn from(c: MSG_PETITION_DECLINE) -> Self {
        Self::MSG_PETITION_DECLINE(c)
    }
}

impl From<MSG_TABARDVENDOR_ACTIVATE> for ClientOpcodeMessage {
    fn from(c: MSG_TABARDVENDOR_ACTIVATE) -> Self {
        Self::MSG_TABARDVENDOR_ACTIVATE(c)
    }
}

impl From<MSG_PETITION_RENAME> for ClientOpcodeMessage {
    fn from(c: MSG_PETITION_RENAME) -> Self {
        Self::MSG_PETITION_RENAME(c)
    }
}

impl From<MSG_SET_DUNGEON_DIFFICULTY> for ClientOpcodeMessage {
    fn from(c: MSG_SET_DUNGEON_DIFFICULTY) -> Self {
        Self::MSG_SET_DUNGEON_DIFFICULTY(c)
    }
}

impl From<MSG_MOVE_START_ASCEND> for ClientOpcodeMessage {
    fn from(c: MSG_MOVE_START_ASCEND) -> Self {
        Self::MSG_MOVE_START_ASCEND(c)
    }
}

impl From<MSG_MOVE_STOP_ASCEND> for ClientOpcodeMessage {
    fn from(c: MSG_MOVE_STOP_ASCEND) -> Self {
        Self::MSG_MOVE_STOP_ASCEND(c)
    }
}

impl From<MSG_MOVE_START_DESCEND> for ClientOpcodeMessage {
    fn from(c: MSG_MOVE_START_DESCEND) -> Self {
        Self::MSG_MOVE_START_DESCEND(c)
    }
}

impl From<CMSG_CHAR_CREATE> for ClientOpcodeMessage {
    fn from(c: CMSG_CHAR_CREATE) -> Self {
        Self::CMSG_CHAR_CREATE(c)
    }
}

impl From<CMSG_CHAR_ENUM> for ClientOpcodeMessage {
    fn from(c: CMSG_CHAR_ENUM) -> Self {
        Self::CMSG_CHAR_ENUM(c)
    }
}

impl From<CMSG_CHAR_DELETE> for ClientOpcodeMessage {
    fn from(c: CMSG_CHAR_DELETE) -> Self {
        Self::CMSG_CHAR_DELETE(c)
    }
}

impl From<CMSG_PLAYER_LOGIN> for ClientOpcodeMessage {
    fn from(c: CMSG_PLAYER_LOGIN) -> Self {
        Self::CMSG_PLAYER_LOGIN(c)
    }
}

impl From<CMSG_PLAYER_LOGOUT> for ClientOpcodeMessage {
    fn from(c: CMSG_PLAYER_LOGOUT) -> Self {
        Self::CMSG_PLAYER_LOGOUT(c)
    }
}

impl From<CMSG_LOGOUT_REQUEST> for ClientOpcodeMessage {
    fn from(c: CMSG_LOGOUT_REQUEST) -> Self {
        Self::CMSG_LOGOUT_REQUEST(c)
    }
}

impl From<CMSG_LOGOUT_CANCEL> for ClientOpcodeMessage {
    fn from(c: CMSG_LOGOUT_CANCEL) -> Self {
        Self::CMSG_LOGOUT_CANCEL(c)
    }
}

impl From<CMSG_NAME_QUERY> for ClientOpcodeMessage {
    fn from(c: CMSG_NAME_QUERY) -> Self {
        Self::CMSG_NAME_QUERY(c)
    }
}

impl From<CMSG_GUILD_CREATE> for ClientOpcodeMessage {
    fn from(c: CMSG_GUILD_CREATE) -> Self {
        Self::CMSG_GUILD_CREATE(c)
    }
}

impl From<CMSG_GUILD_INVITE> for ClientOpcodeMessage {
    fn from(c: CMSG_GUILD_INVITE) -> Self {
        Self::CMSG_GUILD_INVITE(c)
    }
}

impl From<CMSG_GUILD_ACCEPT> for ClientOpcodeMessage {
    fn from(c: CMSG_GUILD_ACCEPT) -> Self {
        Self::CMSG_GUILD_ACCEPT(c)
    }
}

impl From<CMSG_GUILD_DECLINE> for ClientOpcodeMessage {
    fn from(c: CMSG_GUILD_DECLINE) -> Self {
        Self::CMSG_GUILD_DECLINE(c)
    }
}

impl From<CMSG_GUILD_INFO> for ClientOpcodeMessage {
    fn from(c: CMSG_GUILD_INFO) -> Self {
        Self::CMSG_GUILD_INFO(c)
    }
}

impl From<CMSG_GUILD_ROSTER> for ClientOpcodeMessage {
    fn from(c: CMSG_GUILD_ROSTER) -> Self {
        Self::CMSG_GUILD_ROSTER(c)
    }
}

impl From<CMSG_GUILD_PROMOTE> for ClientOpcodeMessage {
    fn from(c: CMSG_GUILD_PROMOTE) -> Self {
        Self::CMSG_GUILD_PROMOTE(c)
    }
}

impl From<CMSG_GUILD_DEMOTE> for ClientOpcodeMessage {
    fn from(c: CMSG_GUILD_DEMOTE) -> Self {
        Self::CMSG_GUILD_DEMOTE(c)
    }
}

impl From<CMSG_GUILD_LEAVE> for ClientOpcodeMessage {
    fn from(c: CMSG_GUILD_LEAVE) -> Self {
        Self::CMSG_GUILD_LEAVE(c)
    }
}

impl From<CMSG_GUILD_REMOVE> for ClientOpcodeMessage {
    fn from(c: CMSG_GUILD_REMOVE) -> Self {
        Self::CMSG_GUILD_REMOVE(c)
    }
}

impl From<CMSG_GUILD_DISBAND> for ClientOpcodeMessage {
    fn from(c: CMSG_GUILD_DISBAND) -> Self {
        Self::CMSG_GUILD_DISBAND(c)
    }
}

impl From<CMSG_GUILD_LEADER> for ClientOpcodeMessage {
    fn from(c: CMSG_GUILD_LEADER) -> Self {
        Self::CMSG_GUILD_LEADER(c)
    }
}

impl From<CMSG_GUILD_MOTD> for ClientOpcodeMessage {
    fn from(c: CMSG_GUILD_MOTD) -> Self {
        Self::CMSG_GUILD_MOTD(c)
    }
}

impl From<CMSG_MESSAGECHAT> for ClientOpcodeMessage {
    fn from(c: CMSG_MESSAGECHAT) -> Self {
        Self::CMSG_MESSAGECHAT(c)
    }
}

impl From<CMSG_JOIN_CHANNEL> for ClientOpcodeMessage {
    fn from(c: CMSG_JOIN_CHANNEL) -> Self {
        Self::CMSG_JOIN_CHANNEL(c)
    }
}

impl From<CMSG_LEAVE_CHANNEL> for ClientOpcodeMessage {
    fn from(c: CMSG_LEAVE_CHANNEL) -> Self {
        Self::CMSG_LEAVE_CHANNEL(c)
    }
}

impl From<CMSG_CHANNEL_LIST> for ClientOpcodeMessage {
    fn from(c: CMSG_CHANNEL_LIST) -> Self {
        Self::CMSG_CHANNEL_LIST(c)
    }
}

impl From<CMSG_CHANNEL_PASSWORD> for ClientOpcodeMessage {
    fn from(c: CMSG_CHANNEL_PASSWORD) -> Self {
        Self::CMSG_CHANNEL_PASSWORD(c)
    }
}

impl From<CMSG_CHANNEL_SET_OWNER> for ClientOpcodeMessage {
    fn from(c: CMSG_CHANNEL_SET_OWNER) -> Self {
        Self::CMSG_CHANNEL_SET_OWNER(c)
    }
}

impl From<CMSG_CHANNEL_OWNER> for ClientOpcodeMessage {
    fn from(c: CMSG_CHANNEL_OWNER) -> Self {
        Self::CMSG_CHANNEL_OWNER(c)
    }
}

impl From<CMSG_CHANNEL_MODERATOR> for ClientOpcodeMessage {
    fn from(c: CMSG_CHANNEL_MODERATOR) -> Self {
        Self::CMSG_CHANNEL_MODERATOR(c)
    }
}

impl From<CMSG_CHANNEL_UNMODERATOR> for ClientOpcodeMessage {
    fn from(c: CMSG_CHANNEL_UNMODERATOR) -> Self {
        Self::CMSG_CHANNEL_UNMODERATOR(c)
    }
}

impl From<CMSG_CHANNEL_MUTE> for ClientOpcodeMessage {
    fn from(c: CMSG_CHANNEL_MUTE) -> Self {
        Self::CMSG_CHANNEL_MUTE(c)
    }
}

impl From<CMSG_CHANNEL_UNMUTE> for ClientOpcodeMessage {
    fn from(c: CMSG_CHANNEL_UNMUTE) -> Self {
        Self::CMSG_CHANNEL_UNMUTE(c)
    }
}

impl From<CMSG_CHANNEL_INVITE> for ClientOpcodeMessage {
    fn from(c: CMSG_CHANNEL_INVITE) -> Self {
        Self::CMSG_CHANNEL_INVITE(c)
    }
}

impl From<CMSG_CHANNEL_KICK> for ClientOpcodeMessage {
    fn from(c: CMSG_CHANNEL_KICK) -> Self {
        Self::CMSG_CHANNEL_KICK(c)
    }
}

impl From<CMSG_CHANNEL_BAN> for ClientOpcodeMessage {
    fn from(c: CMSG_CHANNEL_BAN) -> Self {
        Self::CMSG_CHANNEL_BAN(c)
    }
}

impl From<CMSG_CHANNEL_UNBAN> for ClientOpcodeMessage {
    fn from(c: CMSG_CHANNEL_UNBAN) -> Self {
        Self::CMSG_CHANNEL_UNBAN(c)
    }
}

impl From<CMSG_CHANNEL_ANNOUNCEMENTS> for ClientOpcodeMessage {
    fn from(c: CMSG_CHANNEL_ANNOUNCEMENTS) -> Self {
        Self::CMSG_CHANNEL_ANNOUNCEMENTS(c)
    }
}

impl From<CMSG_CHANNEL_MODERATE> for ClientOpcodeMessage {
    fn from(c: CMSG_CHANNEL_MODERATE) -> Self {
        Self::CMSG_CHANNEL_MODERATE(c)
    }
}

impl From<CMSG_OPEN_ITEM> for ClientOpcodeMessage {
    fn from(c: CMSG_OPEN_ITEM) -> Self {
        Self::CMSG_OPEN_ITEM(c)
    }
}

impl From<CMSG_READ_ITEM> for ClientOpcodeMessage {
    fn from(c: CMSG_READ_ITEM) -> Self {
        Self::CMSG_READ_ITEM(c)
    }
}

impl From<CMSG_GAMEOBJ_USE> for ClientOpcodeMessage {
    fn from(c: CMSG_GAMEOBJ_USE) -> Self {
        Self::CMSG_GAMEOBJ_USE(c)
    }
}

impl From<CMSG_AREATRIGGER> for ClientOpcodeMessage {
    fn from(c: CMSG_AREATRIGGER) -> Self {
        Self::CMSG_AREATRIGGER(c)
    }
}

impl From<MSG_MOVE_TELEPORT_ACK_Client> for ClientOpcodeMessage {
    fn from(c: MSG_MOVE_TELEPORT_ACK_Client) -> Self {
        Self::MSG_MOVE_TELEPORT_ACK(c)
    }
}

impl From<CMSG_FORCE_RUN_SPEED_CHANGE_ACK> for ClientOpcodeMessage {
    fn from(c: CMSG_FORCE_RUN_SPEED_CHANGE_ACK) -> Self {
        Self::CMSG_FORCE_RUN_SPEED_CHANGE_ACK(c)
    }
}

impl From<CMSG_FORCE_RUN_BACK_SPEED_CHANGE_ACK> for ClientOpcodeMessage {
    fn from(c: CMSG_FORCE_RUN_BACK_SPEED_CHANGE_ACK) -> Self {
        Self::CMSG_FORCE_RUN_BACK_SPEED_CHANGE_ACK(c)
    }
}

impl From<CMSG_FORCE_SWIM_SPEED_CHANGE_ACK> for ClientOpcodeMessage {
    fn from(c: CMSG_FORCE_SWIM_SPEED_CHANGE_ACK) -> Self {
        Self::CMSG_FORCE_SWIM_SPEED_CHANGE_ACK(c)
    }
}

impl From<CMSG_FORCE_MOVE_ROOT_ACK> for ClientOpcodeMessage {
    fn from(c: CMSG_FORCE_MOVE_ROOT_ACK) -> Self {
        Self::CMSG_FORCE_MOVE_ROOT_ACK(c)
    }
}

impl From<CMSG_FORCE_MOVE_UNROOT_ACK> for ClientOpcodeMessage {
    fn from(c: CMSG_FORCE_MOVE_UNROOT_ACK) -> Self {
        Self::CMSG_FORCE_MOVE_UNROOT_ACK(c)
    }
}

impl From<CMSG_NEXT_CINEMATIC_CAMERA> for ClientOpcodeMessage {
    fn from(c: CMSG_NEXT_CINEMATIC_CAMERA) -> Self {
        Self::CMSG_NEXT_CINEMATIC_CAMERA(c)
    }
}

impl From<CMSG_COMPLETE_CINEMATIC> for ClientOpcodeMessage {
    fn from(c: CMSG_COMPLETE_CINEMATIC) -> Self {
        Self::CMSG_COMPLETE_CINEMATIC(c)
    }
}

impl From<CMSG_TUTORIAL_FLAG> for ClientOpcodeMessage {
    fn from(c: CMSG_TUTORIAL_FLAG) -> Self {
        Self::CMSG_TUTORIAL_FLAG(c)
    }
}

impl From<CMSG_TUTORIAL_CLEAR> for ClientOpcodeMessage {
    fn from(c: CMSG_TUTORIAL_CLEAR) -> Self {
        Self::CMSG_TUTORIAL_CLEAR(c)
    }
}

impl From<CMSG_TUTORIAL_RESET> for ClientOpcodeMessage {
    fn from(c: CMSG_TUTORIAL_RESET) -> Self {
        Self::CMSG_TUTORIAL_RESET(c)
    }
}

impl From<CMSG_STANDSTATECHANGE> for ClientOpcodeMessage {
    fn from(c: CMSG_STANDSTATECHANGE) -> Self {
        Self::CMSG_STANDSTATECHANGE(c)
    }
}

impl From<CMSG_EMOTE> for ClientOpcodeMessage {
    fn from(c: CMSG_EMOTE) -> Self {
        Self::CMSG_EMOTE(c)
    }
}

impl From<CMSG_TEXT_EMOTE> for ClientOpcodeMessage {
    fn from(c: CMSG_TEXT_EMOTE) -> Self {
        Self::CMSG_TEXT_EMOTE(c)
    }
}

impl From<CMSG_AUTOSTORE_LOOT_ITEM> for ClientOpcodeMessage {
    fn from(c: CMSG_AUTOSTORE_LOOT_ITEM) -> Self {
        Self::CMSG_AUTOSTORE_LOOT_ITEM(c)
    }
}

impl From<CMSG_AUTOEQUIP_ITEM> for ClientOpcodeMessage {
    fn from(c: CMSG_AUTOEQUIP_ITEM) -> Self {
        Self::CMSG_AUTOEQUIP_ITEM(c)
    }
}

impl From<CMSG_AUTOSTORE_BAG_ITEM> for ClientOpcodeMessage {
    fn from(c: CMSG_AUTOSTORE_BAG_ITEM) -> Self {
        Self::CMSG_AUTOSTORE_BAG_ITEM(c)
    }
}

impl From<CMSG_SWAP_ITEM> for ClientOpcodeMessage {
    fn from(c: CMSG_SWAP_ITEM) -> Self {
        Self::CMSG_SWAP_ITEM(c)
    }
}

impl From<CMSG_SWAP_INV_ITEM> for ClientOpcodeMessage {
    fn from(c: CMSG_SWAP_INV_ITEM) -> Self {
        Self::CMSG_SWAP_INV_ITEM(c)
    }
}

impl From<CMSG_SPLIT_ITEM> for ClientOpcodeMessage {
    fn from(c: CMSG_SPLIT_ITEM) -> Self {
        Self::CMSG_SPLIT_ITEM(c)
    }
}

impl From<CMSG_AUTOEQUIP_ITEM_SLOT> for ClientOpcodeMessage {
    fn from(c: CMSG_AUTOEQUIP_ITEM_SLOT) -> Self {
        Self::CMSG_AUTOEQUIP_ITEM_SLOT(c)
    }
}

impl From<CMSG_DESTROYITEM> for ClientOpcodeMessage {
    fn from(c: CMSG_DESTROYITEM) -> Self {
        Self::CMSG_DESTROYITEM(c)
    }
}

impl From<CMSG_INSPECT> for ClientOpcodeMessage {
    fn from(c: CMSG_INSPECT) -> Self {
        Self::CMSG_INSPECT(c)
    }
}

impl From<CMSG_SET_FACTION_ATWAR> for ClientOpcodeMessage {
    fn from(c: CMSG_SET_FACTION_ATWAR) -> Self {
        Self::CMSG_SET_FACTION_ATWAR(c)
    }
}

impl From<CMSG_SET_ACTION_BUTTON> for ClientOpcodeMessage {
    fn from(c: CMSG_SET_ACTION_BUTTON) -> Self {
        Self::CMSG_SET_ACTION_BUTTON(c)
    }
}

impl From<CMSG_CANCEL_CHANNELLING> for ClientOpcodeMessage {
    fn from(c: CMSG_CANCEL_CHANNELLING) -> Self {
        Self::CMSG_CANCEL_CHANNELLING(c)
    }
}

impl From<CMSG_SET_SELECTION> for ClientOpcodeMessage {
    fn from(c: CMSG_SET_SELECTION) -> Self {
        Self::CMSG_SET_SELECTION(c)
    }
}

impl From<CMSG_SET_TARGET_OBSOLETE> for ClientOpcodeMessage {
    fn from(c: CMSG_SET_TARGET_OBSOLETE) -> Self {
        Self::CMSG_SET_TARGET_OBSOLETE(c)
    }
}

impl From<CMSG_ATTACKSWING> for ClientOpcodeMessage {
    fn from(c: CMSG_ATTACKSWING) -> Self {
        Self::CMSG_ATTACKSWING(c)
    }
}

impl From<CMSG_ATTACKSTOP> for ClientOpcodeMessage {
    fn from(c: CMSG_ATTACKSTOP) -> Self {
        Self::CMSG_ATTACKSTOP(c)
    }
}

impl From<CMSG_REPOP_REQUEST> for ClientOpcodeMessage {
    fn from(c: CMSG_REPOP_REQUEST) -> Self {
        Self::CMSG_REPOP_REQUEST(c)
    }
}

impl From<CMSG_LOOT> for ClientOpcodeMessage {
    fn from(c: CMSG_LOOT) -> Self {
        Self::CMSG_LOOT(c)
    }
}

impl From<CMSG_LOOT_MONEY> for ClientOpcodeMessage {
    fn from(c: CMSG_LOOT_MONEY) -> Self {
        Self::CMSG_LOOT_MONEY(c)
    }
}

impl From<CMSG_LOOT_RELEASE> for ClientOpcodeMessage {
    fn from(c: CMSG_LOOT_RELEASE) -> Self {
        Self::CMSG_LOOT_RELEASE(c)
    }
}

impl From<CMSG_DUEL_ACCEPTED> for ClientOpcodeMessage {
    fn from(c: CMSG_DUEL_ACCEPTED) -> Self {
        Self::CMSG_DUEL_ACCEPTED(c)
    }
}

impl From<CMSG_DUEL_CANCELLED> for ClientOpcodeMessage {
    fn from(c: CMSG_DUEL_CANCELLED) -> Self {
        Self::CMSG_DUEL_CANCELLED(c)
    }
}

impl From<CMSG_MOUNTSPECIAL_ANIM> for ClientOpcodeMessage {
    fn from(c: CMSG_MOUNTSPECIAL_ANIM) -> Self {
        Self::CMSG_MOUNTSPECIAL_ANIM(c)
    }
}

impl From<CMSG_PET_ACTION> for ClientOpcodeMessage {
    fn from(c: CMSG_PET_ACTION) -> Self {
        Self::CMSG_PET_ACTION(c)
    }
}

impl From<CMSG_PET_ABANDON> for ClientOpcodeMessage {
    fn from(c: CMSG_PET_ABANDON) -> Self {
        Self::CMSG_PET_ABANDON(c)
    }
}

impl From<CMSG_GOSSIP_HELLO> for ClientOpcodeMessage {
    fn from(c: CMSG_GOSSIP_HELLO) -> Self {
        Self::CMSG_GOSSIP_HELLO(c)
    }
}

impl From<CMSG_GOSSIP_SELECT_OPTION> for ClientOpcodeMessage {
    fn from(c: CMSG_GOSSIP_SELECT_OPTION) -> Self {
        Self::CMSG_GOSSIP_SELECT_OPTION(c)
    }
}

impl From<CMSG_LIST_INVENTORY> for ClientOpcodeMessage {
    fn from(c: CMSG_LIST_INVENTORY) -> Self {
        Self::CMSG_LIST_INVENTORY(c)
    }
}

impl From<CMSG_SELL_ITEM> for ClientOpcodeMessage {
    fn from(c: CMSG_SELL_ITEM) -> Self {
        Self::CMSG_SELL_ITEM(c)
    }
}

impl From<CMSG_BUY_ITEM> for ClientOpcodeMessage {
    fn from(c: CMSG_BUY_ITEM) -> Self {
        Self::CMSG_BUY_ITEM(c)
    }
}

impl From<CMSG_BUY_ITEM_IN_SLOT> for ClientOpcodeMessage {
    fn from(c: CMSG_BUY_ITEM_IN_SLOT) -> Self {
        Self::CMSG_BUY_ITEM_IN_SLOT(c)
    }
}

impl From<CMSG_ACTIVATETAXI> for ClientOpcodeMessage {
    fn from(c: CMSG_ACTIVATETAXI) -> Self {
        Self::CMSG_ACTIVATETAXI(c)
    }
}

impl From<CMSG_BINDER_ACTIVATE> for ClientOpcodeMessage {
    fn from(c: CMSG_BINDER_ACTIVATE) -> Self {
        Self::CMSG_BINDER_ACTIVATE(c)
    }
}

impl From<CMSG_BANKER_ACTIVATE> for ClientOpcodeMessage {
    fn from(c: CMSG_BANKER_ACTIVATE) -> Self {
        Self::CMSG_BANKER_ACTIVATE(c)
    }
}

impl From<CMSG_BUY_BANK_SLOT> for ClientOpcodeMessage {
    fn from(c: CMSG_BUY_BANK_SLOT) -> Self {
        Self::CMSG_BUY_BANK_SLOT(c)
    }
}

impl From<CMSG_PETITION_SHOWLIST> for ClientOpcodeMessage {
    fn from(c: CMSG_PETITION_SHOWLIST) -> Self {
        Self::CMSG_PETITION_SHOWLIST(c)
    }
}

impl From<CMSG_PETITION_BUY> for ClientOpcodeMessage {
    fn from(c: CMSG_PETITION_BUY) -> Self {
        Self::CMSG_PETITION_BUY(c)
    }
}

impl From<CMSG_PETITION_SHOW_SIGNATURES> for ClientOpcodeMessage {
    fn from(c: CMSG_PETITION_SHOW_SIGNATURES) -> Self {
        Self::CMSG_PETITION_SHOW_SIGNATURES(c)
    }
}

impl From<CMSG_PETITION_SIGN> for ClientOpcodeMessage {
    fn from(c: CMSG_PETITION_SIGN) -> Self {
        Self::CMSG_PETITION_SIGN(c)
    }
}

impl From<CMSG_OFFER_PETITION> for ClientOpcodeMessage {
    fn from(c: CMSG_OFFER_PETITION) -> Self {
        Self::CMSG_OFFER_PETITION(c)
    }
}

impl From<CMSG_TURN_IN_PETITION> for ClientOpcodeMessage {
    fn from(c: CMSG_TURN_IN_PETITION) -> Self {
        Self::CMSG_TURN_IN_PETITION(c)
    }
}

impl From<CMSG_BUG> for ClientOpcodeMessage {
    fn from(c: CMSG_BUG) -> Self {
        Self::CMSG_BUG(c)
    }
}

impl From<CMSG_PLAYED_TIME> for ClientOpcodeMessage {
    fn from(c: CMSG_PLAYED_TIME) -> Self {
        Self::CMSG_PLAYED_TIME(c)
    }
}

impl From<CMSG_QUERY_TIME> for ClientOpcodeMessage {
    fn from(c: CMSG_QUERY_TIME) -> Self {
        Self::CMSG_QUERY_TIME(c)
    }
}

impl From<CMSG_WRAP_ITEM> for ClientOpcodeMessage {
    fn from(c: CMSG_WRAP_ITEM) -> Self {
        Self::CMSG_WRAP_ITEM(c)
    }
}

impl From<CMSG_PING> for ClientOpcodeMessage {
    fn from(c: CMSG_PING) -> Self {
        Self::CMSG_PING(c)
    }
}

impl From<CMSG_SETSHEATHED> for ClientOpcodeMessage {
    fn from(c: CMSG_SETSHEATHED) -> Self {
        Self::CMSG_SETSHEATHED(c)
    }
}

impl From<CMSG_AUTH_SESSION> for ClientOpcodeMessage {
    fn from(c: CMSG_AUTH_SESSION) -> Self {
        Self::CMSG_AUTH_SESSION(c)
    }
}

impl From<MSG_SAVE_GUILD_EMBLEM_Client> for ClientOpcodeMessage {
    fn from(c: MSG_SAVE_GUILD_EMBLEM_Client) -> Self {
        Self::MSG_SAVE_GUILD_EMBLEM(c)
    }
}

impl From<CMSG_ZONEUPDATE> for ClientOpcodeMessage {
    fn from(c: CMSG_ZONEUPDATE) -> Self {
        Self::CMSG_ZONEUPDATE(c)
    }
}

impl From<MSG_RANDOM_ROLL_Client> for ClientOpcodeMessage {
    fn from(c: MSG_RANDOM_ROLL_Client) -> Self {
        Self::MSG_RANDOM_ROLL(c)
    }
}

impl From<CMSG_GMTICKET_UPDATETEXT> for ClientOpcodeMessage {
    fn from(c: CMSG_GMTICKET_UPDATETEXT) -> Self {
        Self::CMSG_GMTICKET_UPDATETEXT(c)
    }
}

impl From<CMSG_REQUEST_ACCOUNT_DATA> for ClientOpcodeMessage {
    fn from(c: CMSG_REQUEST_ACCOUNT_DATA) -> Self {
        Self::CMSG_REQUEST_ACCOUNT_DATA(c)
    }
}

impl From<CMSG_UPDATE_ACCOUNT_DATA> for ClientOpcodeMessage {
    fn from(c: CMSG_UPDATE_ACCOUNT_DATA) -> Self {
        Self::CMSG_UPDATE_ACCOUNT_DATA(c)
    }
}

impl From<CMSG_GMTICKET_GETTICKET> for ClientOpcodeMessage {
    fn from(c: CMSG_GMTICKET_GETTICKET) -> Self {
        Self::CMSG_GMTICKET_GETTICKET(c)
    }
}

impl From<CMSG_GMTICKET_DELETETICKET> for ClientOpcodeMessage {
    fn from(c: CMSG_GMTICKET_DELETETICKET) -> Self {
        Self::CMSG_GMTICKET_DELETETICKET(c)
    }
}

impl From<CMSG_GMTICKET_SYSTEMSTATUS> for ClientOpcodeMessage {
    fn from(c: CMSG_GMTICKET_SYSTEMSTATUS) -> Self {
        Self::CMSG_GMTICKET_SYSTEMSTATUS(c)
    }
}

impl From<CMSG_CHAT_IGNORED> for ClientOpcodeMessage {
    fn from(c: CMSG_CHAT_IGNORED) -> Self {
        Self::CMSG_CHAT_IGNORED(c)
    }
}

impl From<CMSG_GUILD_RANK> for ClientOpcodeMessage {
    fn from(c: CMSG_GUILD_RANK) -> Self {
        Self::CMSG_GUILD_RANK(c)
    }
}

impl From<CMSG_GUILD_ADD_RANK> for ClientOpcodeMessage {
    fn from(c: CMSG_GUILD_ADD_RANK) -> Self {
        Self::CMSG_GUILD_ADD_RANK(c)
    }
}

impl From<CMSG_GUILD_DEL_RANK> for ClientOpcodeMessage {
    fn from(c: CMSG_GUILD_DEL_RANK) -> Self {
        Self::CMSG_GUILD_DEL_RANK(c)
    }
}

impl From<CMSG_GUILD_SET_PUBLIC_NOTE> for ClientOpcodeMessage {
    fn from(c: CMSG_GUILD_SET_PUBLIC_NOTE) -> Self {
        Self::CMSG_GUILD_SET_PUBLIC_NOTE(c)
    }
}

impl From<CMSG_GUILD_SET_OFFICER_NOTE> for ClientOpcodeMessage {
    fn from(c: CMSG_GUILD_SET_OFFICER_NOTE) -> Self {
        Self::CMSG_GUILD_SET_OFFICER_NOTE(c)
    }
}

impl From<CMSG_SEND_MAIL> for ClientOpcodeMessage {
    fn from(c: CMSG_SEND_MAIL) -> Self {
        Self::CMSG_SEND_MAIL(c)
    }
}

impl From<CMSG_GET_MAIL_LIST> for ClientOpcodeMessage {
    fn from(c: CMSG_GET_MAIL_LIST) -> Self {
        Self::CMSG_GET_MAIL_LIST(c)
    }
}

impl From<CMSG_BATTLEFIELD_LIST> for ClientOpcodeMessage {
    fn from(c: CMSG_BATTLEFIELD_LIST) -> Self {
        Self::CMSG_BATTLEFIELD_LIST(c)
    }
}

impl From<CMSG_MAIL_TAKE_MONEY> for ClientOpcodeMessage {
    fn from(c: CMSG_MAIL_TAKE_MONEY) -> Self {
        Self::CMSG_MAIL_TAKE_MONEY(c)
    }
}

impl From<CMSG_MAIL_TAKE_ITEM> for ClientOpcodeMessage {
    fn from(c: CMSG_MAIL_TAKE_ITEM) -> Self {
        Self::CMSG_MAIL_TAKE_ITEM(c)
    }
}

impl From<CMSG_MAIL_MARK_AS_READ> for ClientOpcodeMessage {
    fn from(c: CMSG_MAIL_MARK_AS_READ) -> Self {
        Self::CMSG_MAIL_MARK_AS_READ(c)
    }
}

impl From<CMSG_MAIL_RETURN_TO_SENDER> for ClientOpcodeMessage {
    fn from(c: CMSG_MAIL_RETURN_TO_SENDER) -> Self {
        Self::CMSG_MAIL_RETURN_TO_SENDER(c)
    }
}

impl From<CMSG_MAIL_DELETE> for ClientOpcodeMessage {
    fn from(c: CMSG_MAIL_DELETE) -> Self {
        Self::CMSG_MAIL_DELETE(c)
    }
}

impl From<CMSG_MAIL_CREATE_TEXT_ITEM> for ClientOpcodeMessage {
    fn from(c: CMSG_MAIL_CREATE_TEXT_ITEM) -> Self {
        Self::CMSG_MAIL_CREATE_TEXT_ITEM(c)
    }
}

impl From<MSG_AUCTION_HELLO_Client> for ClientOpcodeMessage {
    fn from(c: MSG_AUCTION_HELLO_Client) -> Self {
        Self::MSG_AUCTION_HELLO(c)
    }
}

impl From<CMSG_AUCTION_SELL_ITEM> for ClientOpcodeMessage {
    fn from(c: CMSG_AUCTION_SELL_ITEM) -> Self {
        Self::CMSG_AUCTION_SELL_ITEM(c)
    }
}

impl From<CMSG_AUCTION_REMOVE_ITEM> for ClientOpcodeMessage {
    fn from(c: CMSG_AUCTION_REMOVE_ITEM) -> Self {
        Self::CMSG_AUCTION_REMOVE_ITEM(c)
    }
}

impl From<CMSG_AUCTION_LIST_ITEMS> for ClientOpcodeMessage {
    fn from(c: CMSG_AUCTION_LIST_ITEMS) -> Self {
        Self::CMSG_AUCTION_LIST_ITEMS(c)
    }
}

impl From<CMSG_AUCTION_LIST_OWNER_ITEMS> for ClientOpcodeMessage {
    fn from(c: CMSG_AUCTION_LIST_OWNER_ITEMS) -> Self {
        Self::CMSG_AUCTION_LIST_OWNER_ITEMS(c)
    }
}

impl From<CMSG_AUCTION_PLACE_BID> for ClientOpcodeMessage {
    fn from(c: CMSG_AUCTION_PLACE_BID) -> Self {
        Self::CMSG_AUCTION_PLACE_BID(c)
    }
}

impl From<CMSG_AUCTION_LIST_BIDDER_ITEMS> for ClientOpcodeMessage {
    fn from(c: CMSG_AUCTION_LIST_BIDDER_ITEMS) -> Self {
        Self::CMSG_AUCTION_LIST_BIDDER_ITEMS(c)
    }
}

impl From<CMSG_SET_AMMO> for ClientOpcodeMessage {
    fn from(c: CMSG_SET_AMMO) -> Self {
        Self::CMSG_SET_AMMO(c)
    }
}

impl From<CMSG_SET_ACTIVE_MOVER> for ClientOpcodeMessage {
    fn from(c: CMSG_SET_ACTIVE_MOVER) -> Self {
        Self::CMSG_SET_ACTIVE_MOVER(c)
    }
}

impl From<CMSG_BUY_STABLE_SLOT> for ClientOpcodeMessage {
    fn from(c: CMSG_BUY_STABLE_SLOT) -> Self {
        Self::CMSG_BUY_STABLE_SLOT(c)
    }
}

impl From<CMSG_AUTOSTORE_BANK_ITEM> for ClientOpcodeMessage {
    fn from(c: CMSG_AUTOSTORE_BANK_ITEM) -> Self {
        Self::CMSG_AUTOSTORE_BANK_ITEM(c)
    }
}

impl From<CMSG_AUTOBANK_ITEM> for ClientOpcodeMessage {
    fn from(c: CMSG_AUTOBANK_ITEM) -> Self {
        Self::CMSG_AUTOBANK_ITEM(c)
    }
}

impl From<CMSG_BUYBACK_ITEM> for ClientOpcodeMessage {
    fn from(c: CMSG_BUYBACK_ITEM) -> Self {
        Self::CMSG_BUYBACK_ITEM(c)
    }
}

impl From<CMSG_LOOT_ROLL> for ClientOpcodeMessage {
    fn from(c: CMSG_LOOT_ROLL) -> Self {
        Self::CMSG_LOOT_ROLL(c)
    }
}

impl From<CMSG_LOOT_MASTER_GIVE> for ClientOpcodeMessage {
    fn from(c: CMSG_LOOT_MASTER_GIVE) -> Self {
        Self::CMSG_LOOT_MASTER_GIVE(c)
    }
}

impl From<CMSG_REPAIR_ITEM> for ClientOpcodeMessage {
    fn from(c: CMSG_REPAIR_ITEM) -> Self {
        Self::CMSG_REPAIR_ITEM(c)
    }
}

impl From<CMSG_TOGGLE_HELM> for ClientOpcodeMessage {
    fn from(c: CMSG_TOGGLE_HELM) -> Self {
        Self::CMSG_TOGGLE_HELM(c)
    }
}

impl From<CMSG_TOGGLE_CLOAK> for ClientOpcodeMessage {
    fn from(c: CMSG_TOGGLE_CLOAK) -> Self {
        Self::CMSG_TOGGLE_CLOAK(c)
    }
}

impl From<CMSG_SET_ACTIONBAR_TOGGLES> for ClientOpcodeMessage {
    fn from(c: CMSG_SET_ACTIONBAR_TOGGLES) -> Self {
        Self::CMSG_SET_ACTIONBAR_TOGGLES(c)
    }
}

impl From<CMSG_CHAR_RENAME> for ClientOpcodeMessage {
    fn from(c: CMSG_CHAR_RENAME) -> Self {
        Self::CMSG_CHAR_RENAME(c)
    }
}

impl From<CMSG_MOVE_FALL_RESET> for ClientOpcodeMessage {
    fn from(c: CMSG_MOVE_FALL_RESET) -> Self {
        Self::CMSG_MOVE_FALL_RESET(c)
    }
}

impl From<CMSG_BATTLEFIELD_STATUS> for ClientOpcodeMessage {
    fn from(c: CMSG_BATTLEFIELD_STATUS) -> Self {
        Self::CMSG_BATTLEFIELD_STATUS(c)
    }
}

impl From<CMSG_BATTLEFIELD_PORT> for ClientOpcodeMessage {
    fn from(c: CMSG_BATTLEFIELD_PORT) -> Self {
        Self::CMSG_BATTLEFIELD_PORT(c)
    }
}

impl From<CMSG_BATTLEMASTER_HELLO> for ClientOpcodeMessage {
    fn from(c: CMSG_BATTLEMASTER_HELLO) -> Self {
        Self::CMSG_BATTLEMASTER_HELLO(c)
    }
}

impl From<CMSG_FORCE_WALK_SPEED_CHANGE_ACK> for ClientOpcodeMessage {
    fn from(c: CMSG_FORCE_WALK_SPEED_CHANGE_ACK) -> Self {
        Self::CMSG_FORCE_WALK_SPEED_CHANGE_ACK(c)
    }
}

impl From<CMSG_FORCE_SWIM_BACK_SPEED_CHANGE_ACK> for ClientOpcodeMessage {
    fn from(c: CMSG_FORCE_SWIM_BACK_SPEED_CHANGE_ACK) -> Self {
        Self::CMSG_FORCE_SWIM_BACK_SPEED_CHANGE_ACK(c)
    }
}

impl From<CMSG_FORCE_TURN_RATE_CHANGE_ACK> for ClientOpcodeMessage {
    fn from(c: CMSG_FORCE_TURN_RATE_CHANGE_ACK) -> Self {
        Self::CMSG_FORCE_TURN_RATE_CHANGE_ACK(c)
    }
}

impl From<CMSG_LEAVE_BATTLEFIELD> for ClientOpcodeMessage {
    fn from(c: CMSG_LEAVE_BATTLEFIELD) -> Self {
        Self::CMSG_LEAVE_BATTLEFIELD(c)
    }
}

impl From<MSG_BATTLEGROUND_PLAYER_POSITIONS_Client> for ClientOpcodeMessage {
    fn from(c: MSG_BATTLEGROUND_PLAYER_POSITIONS_Client) -> Self {
        Self::MSG_BATTLEGROUND_PLAYER_POSITIONS(c)
    }
}

impl From<CMSG_BATTLEMASTER_JOIN> for ClientOpcodeMessage {
    fn from(c: CMSG_BATTLEMASTER_JOIN) -> Self {
        Self::CMSG_BATTLEMASTER_JOIN(c)
    }
}

impl From<CMSG_GUILD_INFO_TEXT> for ClientOpcodeMessage {
    fn from(c: CMSG_GUILD_INFO_TEXT) -> Self {
        Self::CMSG_GUILD_INFO_TEXT(c)
    }
}

impl From<CMSG_ACTIVATETAXIEXPRESS> for ClientOpcodeMessage {
    fn from(c: CMSG_ACTIVATETAXIEXPRESS) -> Self {
        Self::CMSG_ACTIVATETAXIEXPRESS(c)
    }
}

impl From<CMSG_SET_WATCHED_FACTION> for ClientOpcodeMessage {
    fn from(c: CMSG_SET_WATCHED_FACTION) -> Self {
        Self::CMSG_SET_WATCHED_FACTION(c)
    }
}

impl From<CMSG_GMSURVEY_SUBMIT> for ClientOpcodeMessage {
    fn from(c: CMSG_GMSURVEY_SUBMIT) -> Self {
        Self::CMSG_GMSURVEY_SUBMIT(c)
    }
}

impl From<CMSG_MOVE_SET_FLY> for ClientOpcodeMessage {
    fn from(c: CMSG_MOVE_SET_FLY) -> Self {
        Self::CMSG_MOVE_SET_FLY(c)
    }
}

impl From<CMSG_REALM_SPLIT> for ClientOpcodeMessage {
    fn from(c: CMSG_REALM_SPLIT) -> Self {
        Self::CMSG_REALM_SPLIT(c)
    }
}

impl From<CMSG_MOVE_CHNG_TRANSPORT> for ClientOpcodeMessage {
    fn from(c: CMSG_MOVE_CHNG_TRANSPORT) -> Self {
        Self::CMSG_MOVE_CHNG_TRANSPORT(c)
    }
}

impl From<CMSG_TIME_SYNC_RESP> for ClientOpcodeMessage {
    fn from(c: CMSG_TIME_SYNC_RESP) -> Self {
        Self::CMSG_TIME_SYNC_RESP(c)
    }
}

impl From<CMSG_SET_ACTIVE_VOICE_CHANNEL> for ClientOpcodeMessage {
    fn from(c: CMSG_SET_ACTIVE_VOICE_CHANNEL) -> Self {
        Self::CMSG_SET_ACTIVE_VOICE_CHANNEL(c)
    }
}

impl From<CMSG_WORLD_STATE_UI_TIMER_UPDATE> for ClientOpcodeMessage {
    fn from(c: CMSG_WORLD_STATE_UI_TIMER_UPDATE) -> Self {
        Self::CMSG_WORLD_STATE_UI_TIMER_UPDATE(c)
    }
}

impl From<CMSG_READY_FOR_ACCOUNT_DATA_TIMES> for ClientOpcodeMessage {
    fn from(c: CMSG_READY_FOR_ACCOUNT_DATA_TIMES) -> Self {
        Self::CMSG_READY_FOR_ACCOUNT_DATA_TIMES(c)
    }
}

use crate::world::wrath::SMSG_CHAR_CREATE;
use crate::world::wrath::SMSG_CHAR_ENUM;
use crate::world::wrath::SMSG_CHAR_DELETE;
use crate::world::wrath::SMSG_NEW_WORLD;
use crate::world::wrath::SMSG_TRANSFER_PENDING;
use crate::world::wrath::SMSG_CHARACTER_LOGIN_FAILED;
use crate::world::wrath::SMSG_LOGIN_SETTIMESPEED;
use crate::world::wrath::SMSG_LOGOUT_RESPONSE;
use crate::world::wrath::SMSG_LOGOUT_COMPLETE;
use crate::world::wrath::SMSG_LOGOUT_CANCEL_ACK;
use crate::world::wrath::SMSG_NAME_QUERY_RESPONSE;
use crate::world::wrath::SMSG_CONTACT_LIST;
use crate::world::wrath::SMSG_GUILD_INVITE;
use crate::world::wrath::SMSG_GUILD_INFO;
use crate::world::wrath::SMSG_GUILD_EVENT;
use crate::world::wrath::SMSG_GUILD_COMMAND_RESULT;
use crate::world::wrath::SMSG_CHANNEL_NOTIFY;
use crate::world::wrath::SMSG_CHANNEL_LIST;
use crate::world::wrath::SMSG_UPDATE_OBJECT;
use crate::world::wrath::SMSG_DESTROY_OBJECT;
use crate::world::wrath::SMSG_READ_ITEM_OK;
use crate::world::wrath::SMSG_READ_ITEM_FAILED;
use crate::world::wrath::SMSG_ITEM_COOLDOWN;
use crate::world::wrath::SMSG_GAMEOBJECT_CUSTOM_ANIM;
use crate::world::wrath::MSG_MOVE_TELEPORT_ACK_Server;
use crate::world::wrath::SMSG_FORCE_MOVE_ROOT;
use crate::world::wrath::SMSG_FORCE_MOVE_UNROOT;
use crate::world::wrath::SMSG_TRIGGER_CINEMATIC;
use crate::world::wrath::SMSG_TUTORIAL_FLAGS;
use crate::world::wrath::SMSG_EMOTE;
use crate::world::wrath::SMSG_TEXT_EMOTE;
use crate::world::wrath::SMSG_INVENTORY_CHANGE_FAILURE;
use crate::world::wrath::SMSG_INITIALIZE_FACTIONS;
use crate::world::wrath::SMSG_SET_FACTION_VISIBLE;
use crate::world::wrath::SMSG_SET_FACTION_STANDING;
use crate::world::wrath::SMSG_SET_PROFICIENCY;
use crate::world::wrath::SMSG_ACTION_BUTTONS;
use crate::world::wrath::SMSG_AI_REACTION;
use crate::world::wrath::SMSG_ATTACKSTART;
use crate::world::wrath::SMSG_ATTACKSTOP;
use crate::world::wrath::SMSG_ATTACKSWING_NOTINRANGE;
use crate::world::wrath::SMSG_ATTACKSWING_BADFACING;
use crate::world::wrath::SMSG_ATTACKSWING_DEADTARGET;
use crate::world::wrath::SMSG_ATTACKSWING_CANT_ATTACK;
use crate::world::wrath::SMSG_ATTACKERSTATEUPDATE;
use crate::world::wrath::SMSG_CANCEL_COMBAT;
use crate::world::wrath::SMSG_BINDPOINTUPDATE;
use crate::world::wrath::SMSG_LOOT_RELEASE_RESPONSE;
use crate::world::wrath::SMSG_LOOT_REMOVED;
use crate::world::wrath::SMSG_LOOT_MONEY_NOTIFY;
use crate::world::wrath::SMSG_LOOT_CLEAR_MONEY;
use crate::world::wrath::SMSG_ITEM_PUSH_RESULT;
use crate::world::wrath::SMSG_DUEL_REQUESTED;
use crate::world::wrath::SMSG_DUEL_OUTOFBOUNDS;
use crate::world::wrath::SMSG_DUEL_INBOUNDS;
use crate::world::wrath::SMSG_DUEL_COMPLETE;
use crate::world::wrath::SMSG_DUEL_WINNER;
use crate::world::wrath::SMSG_MOUNTRESULT;
use crate::world::wrath::SMSG_MOUNTSPECIAL_ANIM;
use crate::world::wrath::SMSG_GOSSIP_MESSAGE;
use crate::world::wrath::SMSG_GOSSIP_COMPLETE;
use crate::world::wrath::SMSG_NPC_TEXT_UPDATE;
use crate::world::wrath::SMSG_LIST_INVENTORY;
use crate::world::wrath::SMSG_SELL_ITEM;
use crate::world::wrath::SMSG_BUY_ITEM;
use crate::world::wrath::SMSG_BUY_FAILED;
use crate::world::wrath::SMSG_SHOW_BANK;
use crate::world::wrath::SMSG_BUY_BANK_SLOT_RESULT;
use crate::world::wrath::SMSG_PETITION_SHOW_SIGNATURES;
use crate::world::wrath::SMSG_PETITION_SIGN_RESULTS;
use crate::world::wrath::SMSG_TURN_IN_PETITION_RESULTS;
use crate::world::wrath::SMSG_NOTIFICATION;
use crate::world::wrath::SMSG_PLAYED_TIME;
use crate::world::wrath::SMSG_QUERY_TIME_RESPONSE;
use crate::world::wrath::SMSG_LOG_XPGAIN;
use crate::world::wrath::SMSG_LEVELUP_INFO;
use crate::world::wrath::SMSG_PONG;
use crate::world::wrath::SMSG_GAMEOBJECT_PAGETEXT;
use crate::world::wrath::SMSG_ITEM_TIME_UPDATE;
use crate::world::wrath::SMSG_ITEM_ENCHANT_TIME_UPDATE;
use crate::world::wrath::SMSG_AUTH_CHALLENGE;
use crate::world::wrath::SMSG_AUTH_RESPONSE;
use crate::world::wrath::MSG_SAVE_GUILD_EMBLEM_Server;
use crate::world::wrath::SMSG_EXPLORATION_EXPERIENCE;
use crate::world::wrath::MSG_RANDOM_ROLL_Server;
use crate::world::wrath::SMSG_ENVIRONMENTALDAMAGELOG;
use crate::world::wrath::SMSG_GMTICKET_CREATE;
use crate::world::wrath::SMSG_GMTICKET_UPDATETEXT;
use crate::world::wrath::SMSG_ACCOUNT_DATA_TIMES;
use crate::world::wrath::SMSG_UPDATE_ACCOUNT_DATA;
use crate::world::wrath::SMSG_GAMEOBJECT_DESPAWN_ANIM;
use crate::world::wrath::SMSG_GMTICKET_DELETETICKET;
use crate::world::wrath::SMSG_CHAT_WRONG_FACTION;
use crate::world::wrath::SMSG_GMTICKET_SYSTEMSTATUS;
use crate::world::wrath::SMSG_SET_REST_START;
use crate::world::wrath::SMSG_GOSSIP_POI;
use crate::world::wrath::SMSG_LOGIN_VERIFY_WORLD;
use crate::world::wrath::SMSG_BATTLEFIELD_LIST;
use crate::world::wrath::SMSG_ZONE_UNDER_ATTACK;
use crate::world::wrath::MSG_AUCTION_HELLO_Server;
use crate::world::wrath::SMSG_AUCTION_LIST_RESULT;
use crate::world::wrath::SMSG_AUCTION_OWNER_LIST_RESULT;
use crate::world::wrath::SMSG_AUCTION_BIDDER_NOTIFICATION;
use crate::world::wrath::SMSG_AUCTION_OWNER_NOTIFICATION;
use crate::world::wrath::SMSG_PROCRESIST;
use crate::world::wrath::SMSG_AUCTION_BIDDER_LIST_RESULT;
use crate::world::wrath::SMSG_PLAY_OBJECT_SOUND;
use crate::world::wrath::SMSG_RECEIVED_MAIL;
use crate::world::wrath::SMSG_AUCTION_REMOVED_NOTIFICATION;
use crate::world::wrath::SMSG_SERVER_MESSAGE;
use crate::world::wrath::SMSG_STANDSTATE_UPDATE;
use crate::world::wrath::SMSG_LOOT_ALL_PASSED;
use crate::world::wrath::SMSG_LOOT_ROLL_WON;
use crate::world::wrath::SMSG_LOOT_START_ROLL;
use crate::world::wrath::SMSG_LOOT_ROLL;
use crate::world::wrath::SMSG_LOOT_MASTER_LIST;
use crate::world::wrath::SMSG_SET_FORCED_REACTIONS;
use crate::world::wrath::SMSG_CHAT_PLAYER_NOT_FOUND;
use crate::world::wrath::SMSG_DUEL_COUNTDOWN;
use crate::world::wrath::SMSG_AREA_TRIGGER_MESSAGE;
use crate::world::wrath::SMSG_DURABILITY_DAMAGE_DEATH;
use crate::world::wrath::SMSG_INIT_WORLD_STATES;
use crate::world::wrath::SMSG_UPDATE_WORLD_STATE;
use crate::world::wrath::SMSG_CHAR_RENAME;
use crate::world::wrath::MSG_BATTLEGROUND_PLAYER_POSITIONS_Server;
use crate::world::wrath::SMSG_BINDER_CONFIRM;
use crate::world::wrath::SMSG_BATTLEGROUND_PLAYER_JOINED;
use crate::world::wrath::SMSG_BATTLEGROUND_PLAYER_LEFT;
use crate::world::wrath::SMSG_ADDON_INFO;
use crate::world::wrath::SMSG_CHAT_RESTRICTED;
use crate::world::wrath::SMSG_GM_TICKET_STATUS_UPDATE;
use crate::world::wrath::SMSG_DEFENSE_MESSAGE;
use crate::world::wrath::SMSG_REALM_SPLIT;
use crate::world::wrath::SMSG_TIME_SYNC_REQ;
use crate::world::wrath::SMSG_FEATURE_SYSTEM_STATUS;
use crate::world::wrath::SMSG_UPDATE_ACCOUNT_DATA_COMPLETE;
use crate::world::wrath::SMSG_CLIENTCACHE_VERSION;
use crate::world::wrath::SMSG_WORLD_STATE_UI_TIMER_UPDATE;

#[derive(Debug, Clone, PartialEq)]
pub enum ServerOpcodeMessage {
    MSG_MOVE_START_FORWARD(MSG_MOVE_START_FORWARD),
    MSG_MOVE_START_BACKWARD(MSG_MOVE_START_BACKWARD),
    MSG_MOVE_STOP(MSG_MOVE_STOP),
    MSG_MOVE_START_STRAFE_LEFT(MSG_MOVE_START_STRAFE_LEFT),
    MSG_MOVE_START_STRAFE_RIGHT(MSG_MOVE_START_STRAFE_RIGHT),
    MSG_MOVE_STOP_STRAFE(MSG_MOVE_STOP_STRAFE),
    MSG_MOVE_JUMP(MSG_MOVE_JUMP),
    MSG_MOVE_START_TURN_LEFT(MSG_MOVE_START_TURN_LEFT),
    MSG_MOVE_START_TURN_RIGHT(MSG_MOVE_START_TURN_RIGHT),
    MSG_MOVE_STOP_TURN(MSG_MOVE_STOP_TURN),
    MSG_MOVE_START_PITCH_UP(MSG_MOVE_START_PITCH_UP),
    MSG_MOVE_START_PITCH_DOWN(MSG_MOVE_START_PITCH_DOWN),
    MSG_MOVE_STOP_PITCH(MSG_MOVE_STOP_PITCH),
    MSG_MOVE_SET_RUN_MODE(MSG_MOVE_SET_RUN_MODE),
    MSG_MOVE_SET_WALK_MODE(MSG_MOVE_SET_WALK_MODE),
    MSG_MOVE_FALL_LAND(MSG_MOVE_FALL_LAND),
    MSG_MOVE_START_SWIM(MSG_MOVE_START_SWIM),
    MSG_MOVE_STOP_SWIM(MSG_MOVE_STOP_SWIM),
    MSG_MOVE_SET_FACING(MSG_MOVE_SET_FACING),
    MSG_MOVE_SET_PITCH(MSG_MOVE_SET_PITCH),
    MSG_MOVE_WORLDPORT_ACK(MSG_MOVE_WORLDPORT_ACK),
    MSG_MOVE_HEARTBEAT(MSG_MOVE_HEARTBEAT),
    MSG_PETITION_DECLINE(MSG_PETITION_DECLINE),
    MSG_TABARDVENDOR_ACTIVATE(MSG_TABARDVENDOR_ACTIVATE),
    MSG_PETITION_RENAME(MSG_PETITION_RENAME),
    MSG_SET_DUNGEON_DIFFICULTY(MSG_SET_DUNGEON_DIFFICULTY),
    MSG_MOVE_START_ASCEND(MSG_MOVE_START_ASCEND),
    MSG_MOVE_STOP_ASCEND(MSG_MOVE_STOP_ASCEND),
    MSG_MOVE_START_DESCEND(MSG_MOVE_START_DESCEND),
    SMSG_CHAR_CREATE(SMSG_CHAR_CREATE),
    SMSG_CHAR_ENUM(SMSG_CHAR_ENUM),
    SMSG_CHAR_DELETE(SMSG_CHAR_DELETE),
    SMSG_NEW_WORLD(SMSG_NEW_WORLD),
    SMSG_TRANSFER_PENDING(SMSG_TRANSFER_PENDING),
    SMSG_CHARACTER_LOGIN_FAILED(SMSG_CHARACTER_LOGIN_FAILED),
    SMSG_LOGIN_SETTIMESPEED(SMSG_LOGIN_SETTIMESPEED),
    SMSG_LOGOUT_RESPONSE(SMSG_LOGOUT_RESPONSE),
    SMSG_LOGOUT_COMPLETE(SMSG_LOGOUT_COMPLETE),
    SMSG_LOGOUT_CANCEL_ACK(SMSG_LOGOUT_CANCEL_ACK),
    SMSG_NAME_QUERY_RESPONSE(SMSG_NAME_QUERY_RESPONSE),
    SMSG_CONTACT_LIST(SMSG_CONTACT_LIST),
    SMSG_GUILD_INVITE(SMSG_GUILD_INVITE),
    SMSG_GUILD_INFO(SMSG_GUILD_INFO),
    SMSG_GUILD_EVENT(SMSG_GUILD_EVENT),
    SMSG_GUILD_COMMAND_RESULT(SMSG_GUILD_COMMAND_RESULT),
    SMSG_CHANNEL_NOTIFY(SMSG_CHANNEL_NOTIFY),
    SMSG_CHANNEL_LIST(SMSG_CHANNEL_LIST),
    SMSG_UPDATE_OBJECT(SMSG_UPDATE_OBJECT),
    SMSG_DESTROY_OBJECT(SMSG_DESTROY_OBJECT),
    SMSG_READ_ITEM_OK(SMSG_READ_ITEM_OK),
    SMSG_READ_ITEM_FAILED(SMSG_READ_ITEM_FAILED),
    SMSG_ITEM_COOLDOWN(SMSG_ITEM_COOLDOWN),
    SMSG_GAMEOBJECT_CUSTOM_ANIM(SMSG_GAMEOBJECT_CUSTOM_ANIM),
    MSG_MOVE_TELEPORT_ACK(MSG_MOVE_TELEPORT_ACK_Server),
    SMSG_FORCE_MOVE_ROOT(SMSG_FORCE_MOVE_ROOT),
    SMSG_FORCE_MOVE_UNROOT(SMSG_FORCE_MOVE_UNROOT),
    SMSG_TRIGGER_CINEMATIC(SMSG_TRIGGER_CINEMATIC),
    SMSG_TUTORIAL_FLAGS(SMSG_TUTORIAL_FLAGS),
    SMSG_EMOTE(SMSG_EMOTE),
    SMSG_TEXT_EMOTE(SMSG_TEXT_EMOTE),
    SMSG_INVENTORY_CHANGE_FAILURE(SMSG_INVENTORY_CHANGE_FAILURE),
    SMSG_INITIALIZE_FACTIONS(SMSG_INITIALIZE_FACTIONS),
    SMSG_SET_FACTION_VISIBLE(SMSG_SET_FACTION_VISIBLE),
    SMSG_SET_FACTION_STANDING(SMSG_SET_FACTION_STANDING),
    SMSG_SET_PROFICIENCY(SMSG_SET_PROFICIENCY),
    SMSG_ACTION_BUTTONS(SMSG_ACTION_BUTTONS),
    SMSG_AI_REACTION(SMSG_AI_REACTION),
    SMSG_ATTACKSTART(SMSG_ATTACKSTART),
    SMSG_ATTACKSTOP(SMSG_ATTACKSTOP),
    SMSG_ATTACKSWING_NOTINRANGE(SMSG_ATTACKSWING_NOTINRANGE),
    SMSG_ATTACKSWING_BADFACING(SMSG_ATTACKSWING_BADFACING),
    SMSG_ATTACKSWING_DEADTARGET(SMSG_ATTACKSWING_DEADTARGET),
    SMSG_ATTACKSWING_CANT_ATTACK(SMSG_ATTACKSWING_CANT_ATTACK),
    SMSG_ATTACKERSTATEUPDATE(SMSG_ATTACKERSTATEUPDATE),
    SMSG_CANCEL_COMBAT(SMSG_CANCEL_COMBAT),
    SMSG_BINDPOINTUPDATE(SMSG_BINDPOINTUPDATE),
    SMSG_LOOT_RELEASE_RESPONSE(SMSG_LOOT_RELEASE_RESPONSE),
    SMSG_LOOT_REMOVED(SMSG_LOOT_REMOVED),
    SMSG_LOOT_MONEY_NOTIFY(SMSG_LOOT_MONEY_NOTIFY),
    SMSG_LOOT_CLEAR_MONEY(SMSG_LOOT_CLEAR_MONEY),
    SMSG_ITEM_PUSH_RESULT(SMSG_ITEM_PUSH_RESULT),
    SMSG_DUEL_REQUESTED(SMSG_DUEL_REQUESTED),
    SMSG_DUEL_OUTOFBOUNDS(SMSG_DUEL_OUTOFBOUNDS),
    SMSG_DUEL_INBOUNDS(SMSG_DUEL_INBOUNDS),
    SMSG_DUEL_COMPLETE(SMSG_DUEL_COMPLETE),
    SMSG_DUEL_WINNER(SMSG_DUEL_WINNER),
    SMSG_MOUNTRESULT(SMSG_MOUNTRESULT),
    SMSG_MOUNTSPECIAL_ANIM(SMSG_MOUNTSPECIAL_ANIM),
    SMSG_GOSSIP_MESSAGE(SMSG_GOSSIP_MESSAGE),
    SMSG_GOSSIP_COMPLETE(SMSG_GOSSIP_COMPLETE),
    SMSG_NPC_TEXT_UPDATE(SMSG_NPC_TEXT_UPDATE),
    SMSG_LIST_INVENTORY(SMSG_LIST_INVENTORY),
    SMSG_SELL_ITEM(SMSG_SELL_ITEM),
    SMSG_BUY_ITEM(SMSG_BUY_ITEM),
    SMSG_BUY_FAILED(SMSG_BUY_FAILED),
    SMSG_SHOW_BANK(SMSG_SHOW_BANK),
    SMSG_BUY_BANK_SLOT_RESULT(SMSG_BUY_BANK_SLOT_RESULT),
    SMSG_PETITION_SHOW_SIGNATURES(SMSG_PETITION_SHOW_SIGNATURES),
    SMSG_PETITION_SIGN_RESULTS(SMSG_PETITION_SIGN_RESULTS),
    SMSG_TURN_IN_PETITION_RESULTS(SMSG_TURN_IN_PETITION_RESULTS),
    SMSG_NOTIFICATION(SMSG_NOTIFICATION),
    SMSG_PLAYED_TIME(SMSG_PLAYED_TIME),
    SMSG_QUERY_TIME_RESPONSE(SMSG_QUERY_TIME_RESPONSE),
    SMSG_LOG_XPGAIN(SMSG_LOG_XPGAIN),
    SMSG_LEVELUP_INFO(SMSG_LEVELUP_INFO),
    SMSG_PONG(SMSG_PONG),
    SMSG_GAMEOBJECT_PAGETEXT(SMSG_GAMEOBJECT_PAGETEXT),
    SMSG_ITEM_TIME_UPDATE(SMSG_ITEM_TIME_UPDATE),
    SMSG_ITEM_ENCHANT_TIME_UPDATE(SMSG_ITEM_ENCHANT_TIME_UPDATE),
    SMSG_AUTH_CHALLENGE(SMSG_AUTH_CHALLENGE),
    SMSG_AUTH_RESPONSE(SMSG_AUTH_RESPONSE),
    MSG_SAVE_GUILD_EMBLEM(MSG_SAVE_GUILD_EMBLEM_Server),
    SMSG_EXPLORATION_EXPERIENCE(SMSG_EXPLORATION_EXPERIENCE),
    MSG_RANDOM_ROLL(MSG_RANDOM_ROLL_Server),
    SMSG_ENVIRONMENTALDAMAGELOG(SMSG_ENVIRONMENTALDAMAGELOG),
    SMSG_GMTICKET_CREATE(SMSG_GMTICKET_CREATE),
    SMSG_GMTICKET_UPDATETEXT(SMSG_GMTICKET_UPDATETEXT),
    SMSG_ACCOUNT_DATA_TIMES(SMSG_ACCOUNT_DATA_TIMES),
    SMSG_UPDATE_ACCOUNT_DATA(SMSG_UPDATE_ACCOUNT_DATA),
    SMSG_GAMEOBJECT_DESPAWN_ANIM(SMSG_GAMEOBJECT_DESPAWN_ANIM),
    SMSG_GMTICKET_DELETETICKET(SMSG_GMTICKET_DELETETICKET),
    SMSG_CHAT_WRONG_FACTION(SMSG_CHAT_WRONG_FACTION),
    SMSG_GMTICKET_SYSTEMSTATUS(SMSG_GMTICKET_SYSTEMSTATUS),
    SMSG_SET_REST_START(SMSG_SET_REST_START),
    SMSG_GOSSIP_POI(SMSG_GOSSIP_POI),
    SMSG_LOGIN_VERIFY_WORLD(SMSG_LOGIN_VERIFY_WORLD),
    SMSG_BATTLEFIELD_LIST(SMSG_BATTLEFIELD_LIST),
    SMSG_ZONE_UNDER_ATTACK(SMSG_ZONE_UNDER_ATTACK),
    MSG_AUCTION_HELLO(MSG_AUCTION_HELLO_Server),
    SMSG_AUCTION_LIST_RESULT(SMSG_AUCTION_LIST_RESULT),
    SMSG_AUCTION_OWNER_LIST_RESULT(SMSG_AUCTION_OWNER_LIST_RESULT),
    SMSG_AUCTION_BIDDER_NOTIFICATION(SMSG_AUCTION_BIDDER_NOTIFICATION),
    SMSG_AUCTION_OWNER_NOTIFICATION(SMSG_AUCTION_OWNER_NOTIFICATION),
    SMSG_PROCRESIST(SMSG_PROCRESIST),
    SMSG_AUCTION_BIDDER_LIST_RESULT(SMSG_AUCTION_BIDDER_LIST_RESULT),
    SMSG_PLAY_OBJECT_SOUND(SMSG_PLAY_OBJECT_SOUND),
    SMSG_RECEIVED_MAIL(SMSG_RECEIVED_MAIL),
    SMSG_AUCTION_REMOVED_NOTIFICATION(SMSG_AUCTION_REMOVED_NOTIFICATION),
    SMSG_SERVER_MESSAGE(SMSG_SERVER_MESSAGE),
    SMSG_STANDSTATE_UPDATE(SMSG_STANDSTATE_UPDATE),
    SMSG_LOOT_ALL_PASSED(SMSG_LOOT_ALL_PASSED),
    SMSG_LOOT_ROLL_WON(SMSG_LOOT_ROLL_WON),
    SMSG_LOOT_START_ROLL(SMSG_LOOT_START_ROLL),
    SMSG_LOOT_ROLL(SMSG_LOOT_ROLL),
    SMSG_LOOT_MASTER_LIST(SMSG_LOOT_MASTER_LIST),
    SMSG_SET_FORCED_REACTIONS(SMSG_SET_FORCED_REACTIONS),
    SMSG_CHAT_PLAYER_NOT_FOUND(SMSG_CHAT_PLAYER_NOT_FOUND),
    SMSG_DUEL_COUNTDOWN(SMSG_DUEL_COUNTDOWN),
    SMSG_AREA_TRIGGER_MESSAGE(SMSG_AREA_TRIGGER_MESSAGE),
    SMSG_DURABILITY_DAMAGE_DEATH(SMSG_DURABILITY_DAMAGE_DEATH),
    SMSG_INIT_WORLD_STATES(SMSG_INIT_WORLD_STATES),
    SMSG_UPDATE_WORLD_STATE(SMSG_UPDATE_WORLD_STATE),
    SMSG_CHAR_RENAME(SMSG_CHAR_RENAME),
    MSG_BATTLEGROUND_PLAYER_POSITIONS(MSG_BATTLEGROUND_PLAYER_POSITIONS_Server),
    SMSG_BINDER_CONFIRM(SMSG_BINDER_CONFIRM),
    SMSG_BATTLEGROUND_PLAYER_JOINED(SMSG_BATTLEGROUND_PLAYER_JOINED),
    SMSG_BATTLEGROUND_PLAYER_LEFT(SMSG_BATTLEGROUND_PLAYER_LEFT),
    SMSG_ADDON_INFO(SMSG_ADDON_INFO),
    SMSG_CHAT_RESTRICTED(SMSG_CHAT_RESTRICTED),
    SMSG_GM_TICKET_STATUS_UPDATE(SMSG_GM_TICKET_STATUS_UPDATE),
    SMSG_DEFENSE_MESSAGE(SMSG_DEFENSE_MESSAGE),
    SMSG_REALM_SPLIT(SMSG_REALM_SPLIT),
    SMSG_TIME_SYNC_REQ(SMSG_TIME_SYNC_REQ),
    SMSG_FEATURE_SYSTEM_STATUS(SMSG_FEATURE_SYSTEM_STATUS),
    SMSG_UPDATE_ACCOUNT_DATA_COMPLETE(SMSG_UPDATE_ACCOUNT_DATA_COMPLETE),
    SMSG_CLIENTCACHE_VERSION(SMSG_CLIENTCACHE_VERSION),
    SMSG_WORLD_STATE_UI_TIMER_UPDATE(SMSG_WORLD_STATE_UI_TIMER_UPDATE),
}

impl ServerOpcodeMessage {
    fn read_opcodes(opcode: u16, body_size: u32, mut r: &[u8]) -> std::result::Result<Self, crate::errors::ExpectedOpcodeError> {
        match opcode {
            0x00B5 => Ok(Self::MSG_MOVE_START_FORWARD(<MSG_MOVE_START_FORWARD as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x00B5, size: body_size, io, } } else { a } })?)),
            0x00B6 => Ok(Self::MSG_MOVE_START_BACKWARD(<MSG_MOVE_START_BACKWARD as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x00B6, size: body_size, io, } } else { a } })?)),
            0x00B7 => Ok(Self::MSG_MOVE_STOP(<MSG_MOVE_STOP as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x00B7, size: body_size, io, } } else { a } })?)),
            0x00B8 => Ok(Self::MSG_MOVE_START_STRAFE_LEFT(<MSG_MOVE_START_STRAFE_LEFT as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x00B8, size: body_size, io, } } else { a } })?)),
            0x00B9 => Ok(Self::MSG_MOVE_START_STRAFE_RIGHT(<MSG_MOVE_START_STRAFE_RIGHT as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x00B9, size: body_size, io, } } else { a } })?)),
            0x00BA => Ok(Self::MSG_MOVE_STOP_STRAFE(<MSG_MOVE_STOP_STRAFE as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x00BA, size: body_size, io, } } else { a } })?)),
            0x00BB => Ok(Self::MSG_MOVE_JUMP(<MSG_MOVE_JUMP as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x00BB, size: body_size, io, } } else { a } })?)),
            0x00BC => Ok(Self::MSG_MOVE_START_TURN_LEFT(<MSG_MOVE_START_TURN_LEFT as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x00BC, size: body_size, io, } } else { a } })?)),
            0x00BD => Ok(Self::MSG_MOVE_START_TURN_RIGHT(<MSG_MOVE_START_TURN_RIGHT as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x00BD, size: body_size, io, } } else { a } })?)),
            0x00BE => Ok(Self::MSG_MOVE_STOP_TURN(<MSG_MOVE_STOP_TURN as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x00BE, size: body_size, io, } } else { a } })?)),
            0x00BF => Ok(Self::MSG_MOVE_START_PITCH_UP(<MSG_MOVE_START_PITCH_UP as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x00BF, size: body_size, io, } } else { a } })?)),
            0x00C0 => Ok(Self::MSG_MOVE_START_PITCH_DOWN(<MSG_MOVE_START_PITCH_DOWN as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x00C0, size: body_size, io, } } else { a } })?)),
            0x00C1 => Ok(Self::MSG_MOVE_STOP_PITCH(<MSG_MOVE_STOP_PITCH as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x00C1, size: body_size, io, } } else { a } })?)),
            0x00C2 => Ok(Self::MSG_MOVE_SET_RUN_MODE(<MSG_MOVE_SET_RUN_MODE as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x00C2, size: body_size, io, } } else { a } })?)),
            0x00C3 => Ok(Self::MSG_MOVE_SET_WALK_MODE(<MSG_MOVE_SET_WALK_MODE as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x00C3, size: body_size, io, } } else { a } })?)),
            0x00C9 => Ok(Self::MSG_MOVE_FALL_LAND(<MSG_MOVE_FALL_LAND as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x00C9, size: body_size, io, } } else { a } })?)),
            0x00CA => Ok(Self::MSG_MOVE_START_SWIM(<MSG_MOVE_START_SWIM as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x00CA, size: body_size, io, } } else { a } })?)),
            0x00CB => Ok(Self::MSG_MOVE_STOP_SWIM(<MSG_MOVE_STOP_SWIM as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x00CB, size: body_size, io, } } else { a } })?)),
            0x00DA => Ok(Self::MSG_MOVE_SET_FACING(<MSG_MOVE_SET_FACING as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x00DA, size: body_size, io, } } else { a } })?)),
            0x00DB => Ok(Self::MSG_MOVE_SET_PITCH(<MSG_MOVE_SET_PITCH as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x00DB, size: body_size, io, } } else { a } })?)),
            0x00DC => Ok(Self::MSG_MOVE_WORLDPORT_ACK(<MSG_MOVE_WORLDPORT_ACK as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x00DC, size: body_size, io, } } else { a } })?)),
            0x00EE => Ok(Self::MSG_MOVE_HEARTBEAT(<MSG_MOVE_HEARTBEAT as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x00EE, size: body_size, io, } } else { a } })?)),
            0x01C2 => Ok(Self::MSG_PETITION_DECLINE(<MSG_PETITION_DECLINE as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x01C2, size: body_size, io, } } else { a } })?)),
            0x01F2 => Ok(Self::MSG_TABARDVENDOR_ACTIVATE(<MSG_TABARDVENDOR_ACTIVATE as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x01F2, size: body_size, io, } } else { a } })?)),
            0x02C1 => Ok(Self::MSG_PETITION_RENAME(<MSG_PETITION_RENAME as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x02C1, size: body_size, io, } } else { a } })?)),
            0x0329 => Ok(Self::MSG_SET_DUNGEON_DIFFICULTY(<MSG_SET_DUNGEON_DIFFICULTY as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x0329, size: body_size, io, } } else { a } })?)),
            0x0359 => Ok(Self::MSG_MOVE_START_ASCEND(<MSG_MOVE_START_ASCEND as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x0359, size: body_size, io, } } else { a } })?)),
            0x035A => Ok(Self::MSG_MOVE_STOP_ASCEND(<MSG_MOVE_STOP_ASCEND as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x035A, size: body_size, io, } } else { a } })?)),
            0x03A7 => Ok(Self::MSG_MOVE_START_DESCEND(<MSG_MOVE_START_DESCEND as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x03A7, size: body_size, io, } } else { a } })?)),
            0x003A => Ok(Self::SMSG_CHAR_CREATE(<SMSG_CHAR_CREATE as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x003A, size: body_size, io, } } else { a } })?)),
            0x003B => Ok(Self::SMSG_CHAR_ENUM(<SMSG_CHAR_ENUM as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x003B, size: body_size, io, } } else { a } })?)),
            0x003C => Ok(Self::SMSG_CHAR_DELETE(<SMSG_CHAR_DELETE as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x003C, size: body_size, io, } } else { a } })?)),
            0x003E => Ok(Self::SMSG_NEW_WORLD(<SMSG_NEW_WORLD as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x003E, size: body_size, io, } } else { a } })?)),
            0x003F => Ok(Self::SMSG_TRANSFER_PENDING(<SMSG_TRANSFER_PENDING as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x003F, size: body_size, io, } } else { a } })?)),
            0x0041 => Ok(Self::SMSG_CHARACTER_LOGIN_FAILED(<SMSG_CHARACTER_LOGIN_FAILED as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x0041, size: body_size, io, } } else { a } })?)),
            0x0042 => Ok(Self::SMSG_LOGIN_SETTIMESPEED(<SMSG_LOGIN_SETTIMESPEED as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x0042, size: body_size, io, } } else { a } })?)),
            0x004C => Ok(Self::SMSG_LOGOUT_RESPONSE(<SMSG_LOGOUT_RESPONSE as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x004C, size: body_size, io, } } else { a } })?)),
            0x004D => Ok(Self::SMSG_LOGOUT_COMPLETE(<SMSG_LOGOUT_COMPLETE as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x004D, size: body_size, io, } } else { a } })?)),
            0x004F => Ok(Self::SMSG_LOGOUT_CANCEL_ACK(<SMSG_LOGOUT_CANCEL_ACK as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x004F, size: body_size, io, } } else { a } })?)),
            0x0051 => Ok(Self::SMSG_NAME_QUERY_RESPONSE(<SMSG_NAME_QUERY_RESPONSE as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x0051, size: body_size, io, } } else { a } })?)),
            0x0067 => Ok(Self::SMSG_CONTACT_LIST(<SMSG_CONTACT_LIST as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x0067, size: body_size, io, } } else { a } })?)),
            0x0083 => Ok(Self::SMSG_GUILD_INVITE(<SMSG_GUILD_INVITE as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x0083, size: body_size, io, } } else { a } })?)),
            0x0088 => Ok(Self::SMSG_GUILD_INFO(<SMSG_GUILD_INFO as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x0088, size: body_size, io, } } else { a } })?)),
            0x0092 => Ok(Self::SMSG_GUILD_EVENT(<SMSG_GUILD_EVENT as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x0092, size: body_size, io, } } else { a } })?)),
            0x0093 => Ok(Self::SMSG_GUILD_COMMAND_RESULT(<SMSG_GUILD_COMMAND_RESULT as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x0093, size: body_size, io, } } else { a } })?)),
            0x0099 => Ok(Self::SMSG_CHANNEL_NOTIFY(<SMSG_CHANNEL_NOTIFY as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x0099, size: body_size, io, } } else { a } })?)),
            0x009B => Ok(Self::SMSG_CHANNEL_LIST(<SMSG_CHANNEL_LIST as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x009B, size: body_size, io, } } else { a } })?)),
            0x00A9 => Ok(Self::SMSG_UPDATE_OBJECT(<SMSG_UPDATE_OBJECT as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x00A9, size: body_size, io, } } else { a } })?)),
            0x00AA => Ok(Self::SMSG_DESTROY_OBJECT(<SMSG_DESTROY_OBJECT as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x00AA, size: body_size, io, } } else { a } })?)),
            0x00AE => Ok(Self::SMSG_READ_ITEM_OK(<SMSG_READ_ITEM_OK as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x00AE, size: body_size, io, } } else { a } })?)),
            0x00AF => Ok(Self::SMSG_READ_ITEM_FAILED(<SMSG_READ_ITEM_FAILED as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x00AF, size: body_size, io, } } else { a } })?)),
            0x00B0 => Ok(Self::SMSG_ITEM_COOLDOWN(<SMSG_ITEM_COOLDOWN as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x00B0, size: body_size, io, } } else { a } })?)),
            0x00B3 => Ok(Self::SMSG_GAMEOBJECT_CUSTOM_ANIM(<SMSG_GAMEOBJECT_CUSTOM_ANIM as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x00B3, size: body_size, io, } } else { a } })?)),
            0x00C7 => Ok(Self::MSG_MOVE_TELEPORT_ACK(<MSG_MOVE_TELEPORT_ACK_Server as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x00C7, size: body_size, io, } } else { a } })?)),
            0x00E8 => Ok(Self::SMSG_FORCE_MOVE_ROOT(<SMSG_FORCE_MOVE_ROOT as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x00E8, size: body_size, io, } } else { a } })?)),
            0x00EA => Ok(Self::SMSG_FORCE_MOVE_UNROOT(<SMSG_FORCE_MOVE_UNROOT as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x00EA, size: body_size, io, } } else { a } })?)),
            0x00FA => Ok(Self::SMSG_TRIGGER_CINEMATIC(<SMSG_TRIGGER_CINEMATIC as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x00FA, size: body_size, io, } } else { a } })?)),
            0x00FD => Ok(Self::SMSG_TUTORIAL_FLAGS(<SMSG_TUTORIAL_FLAGS as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x00FD, size: body_size, io, } } else { a } })?)),
            0x0103 => Ok(Self::SMSG_EMOTE(<SMSG_EMOTE as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x0103, size: body_size, io, } } else { a } })?)),
            0x0105 => Ok(Self::SMSG_TEXT_EMOTE(<SMSG_TEXT_EMOTE as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x0105, size: body_size, io, } } else { a } })?)),
            0x0112 => Ok(Self::SMSG_INVENTORY_CHANGE_FAILURE(<SMSG_INVENTORY_CHANGE_FAILURE as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x0112, size: body_size, io, } } else { a } })?)),
            0x0122 => Ok(Self::SMSG_INITIALIZE_FACTIONS(<SMSG_INITIALIZE_FACTIONS as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x0122, size: body_size, io, } } else { a } })?)),
            0x0123 => Ok(Self::SMSG_SET_FACTION_VISIBLE(<SMSG_SET_FACTION_VISIBLE as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x0123, size: body_size, io, } } else { a } })?)),
            0x0124 => Ok(Self::SMSG_SET_FACTION_STANDING(<SMSG_SET_FACTION_STANDING as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x0124, size: body_size, io, } } else { a } })?)),
            0x0127 => Ok(Self::SMSG_SET_PROFICIENCY(<SMSG_SET_PROFICIENCY as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x0127, size: body_size, io, } } else { a } })?)),
            0x0129 => Ok(Self::SMSG_ACTION_BUTTONS(<SMSG_ACTION_BUTTONS as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x0129, size: body_size, io, } } else { a } })?)),
            0x013C => Ok(Self::SMSG_AI_REACTION(<SMSG_AI_REACTION as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x013C, size: body_size, io, } } else { a } })?)),
            0x0143 => Ok(Self::SMSG_ATTACKSTART(<SMSG_ATTACKSTART as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x0143, size: body_size, io, } } else { a } })?)),
            0x0144 => Ok(Self::SMSG_ATTACKSTOP(<SMSG_ATTACKSTOP as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x0144, size: body_size, io, } } else { a } })?)),
            0x0145 => Ok(Self::SMSG_ATTACKSWING_NOTINRANGE(<SMSG_ATTACKSWING_NOTINRANGE as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x0145, size: body_size, io, } } else { a } })?)),
            0x0146 => Ok(Self::SMSG_ATTACKSWING_BADFACING(<SMSG_ATTACKSWING_BADFACING as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x0146, size: body_size, io, } } else { a } })?)),
            0x0148 => Ok(Self::SMSG_ATTACKSWING_DEADTARGET(<SMSG_ATTACKSWING_DEADTARGET as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x0148, size: body_size, io, } } else { a } })?)),
            0x0149 => Ok(Self::SMSG_ATTACKSWING_CANT_ATTACK(<SMSG_ATTACKSWING_CANT_ATTACK as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x0149, size: body_size, io, } } else { a } })?)),
            0x014A => Ok(Self::SMSG_ATTACKERSTATEUPDATE(<SMSG_ATTACKERSTATEUPDATE as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x014A, size: body_size, io, } } else { a } })?)),
            0x014E => Ok(Self::SMSG_CANCEL_COMBAT(<SMSG_CANCEL_COMBAT as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x014E, size: body_size, io, } } else { a } })?)),
            0x0155 => Ok(Self::SMSG_BINDPOINTUPDATE(<SMSG_BINDPOINTUPDATE as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x0155, size: body_size, io, } } else { a } })?)),
            0x0161 => Ok(Self::SMSG_LOOT_RELEASE_RESPONSE(<SMSG_LOOT_RELEASE_RESPONSE as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x0161, size: body_size, io, } } else { a } })?)),
            0x0162 => Ok(Self::SMSG_LOOT_REMOVED(<SMSG_LOOT_REMOVED as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x0162, size: body_size, io, } } else { a } })?)),
            0x0163 => Ok(Self::SMSG_LOOT_MONEY_NOTIFY(<SMSG_LOOT_MONEY_NOTIFY as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x0163, size: body_size, io, } } else { a } })?)),
            0x0165 => Ok(Self::SMSG_LOOT_CLEAR_MONEY(<SMSG_LOOT_CLEAR_MONEY as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x0165, size: body_size, io, } } else { a } })?)),
            0x0166 => Ok(Self::SMSG_ITEM_PUSH_RESULT(<SMSG_ITEM_PUSH_RESULT as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x0166, size: body_size, io, } } else { a } })?)),
            0x0167 => Ok(Self::SMSG_DUEL_REQUESTED(<SMSG_DUEL_REQUESTED as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x0167, size: body_size, io, } } else { a } })?)),
            0x0168 => Ok(Self::SMSG_DUEL_OUTOFBOUNDS(<SMSG_DUEL_OUTOFBOUNDS as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x0168, size: body_size, io, } } else { a } })?)),
            0x0169 => Ok(Self::SMSG_DUEL_INBOUNDS(<SMSG_DUEL_INBOUNDS as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x0169, size: body_size, io, } } else { a } })?)),
            0x016A => Ok(Self::SMSG_DUEL_COMPLETE(<SMSG_DUEL_COMPLETE as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x016A, size: body_size, io, } } else { a } })?)),
            0x016B => Ok(Self::SMSG_DUEL_WINNER(<SMSG_DUEL_WINNER as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x016B, size: body_size, io, } } else { a } })?)),
            0x016E => Ok(Self::SMSG_MOUNTRESULT(<SMSG_MOUNTRESULT as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x016E, size: body_size, io, } } else { a } })?)),
            0x0172 => Ok(Self::SMSG_MOUNTSPECIAL_ANIM(<SMSG_MOUNTSPECIAL_ANIM as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x0172, size: body_size, io, } } else { a } })?)),
            0x017D => Ok(Self::SMSG_GOSSIP_MESSAGE(<SMSG_GOSSIP_MESSAGE as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x017D, size: body_size, io, } } else { a } })?)),
            0x017E => Ok(Self::SMSG_GOSSIP_COMPLETE(<SMSG_GOSSIP_COMPLETE as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x017E, size: body_size, io, } } else { a } })?)),
            0x0180 => Ok(Self::SMSG_NPC_TEXT_UPDATE(<SMSG_NPC_TEXT_UPDATE as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x0180, size: body_size, io, } } else { a } })?)),
            0x019F => Ok(Self::SMSG_LIST_INVENTORY(<SMSG_LIST_INVENTORY as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x019F, size: body_size, io, } } else { a } })?)),
            0x01A1 => Ok(Self::SMSG_SELL_ITEM(<SMSG_SELL_ITEM as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x01A1, size: body_size, io, } } else { a } })?)),
            0x01A4 => Ok(Self::SMSG_BUY_ITEM(<SMSG_BUY_ITEM as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x01A4, size: body_size, io, } } else { a } })?)),
            0x01A5 => Ok(Self::SMSG_BUY_FAILED(<SMSG_BUY_FAILED as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x01A5, size: body_size, io, } } else { a } })?)),
            0x01B8 => Ok(Self::SMSG_SHOW_BANK(<SMSG_SHOW_BANK as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x01B8, size: body_size, io, } } else { a } })?)),
            0x01BA => Ok(Self::SMSG_BUY_BANK_SLOT_RESULT(<SMSG_BUY_BANK_SLOT_RESULT as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x01BA, size: body_size, io, } } else { a } })?)),
            0x01BF => Ok(Self::SMSG_PETITION_SHOW_SIGNATURES(<SMSG_PETITION_SHOW_SIGNATURES as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x01BF, size: body_size, io, } } else { a } })?)),
            0x01C1 => Ok(Self::SMSG_PETITION_SIGN_RESULTS(<SMSG_PETITION_SIGN_RESULTS as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x01C1, size: body_size, io, } } else { a } })?)),
            0x01C5 => Ok(Self::SMSG_TURN_IN_PETITION_RESULTS(<SMSG_TURN_IN_PETITION_RESULTS as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x01C5, size: body_size, io, } } else { a } })?)),
            0x01CB => Ok(Self::SMSG_NOTIFICATION(<SMSG_NOTIFICATION as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x01CB, size: body_size, io, } } else { a } })?)),
            0x01CD => Ok(Self::SMSG_PLAYED_TIME(<SMSG_PLAYED_TIME as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x01CD, size: body_size, io, } } else { a } })?)),
            0x01CF => Ok(Self::SMSG_QUERY_TIME_RESPONSE(<SMSG_QUERY_TIME_RESPONSE as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x01CF, size: body_size, io, } } else { a } })?)),
            0x01D0 => Ok(Self::SMSG_LOG_XPGAIN(<SMSG_LOG_XPGAIN as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x01D0, size: body_size, io, } } else { a } })?)),
            0x01D4 => Ok(Self::SMSG_LEVELUP_INFO(<SMSG_LEVELUP_INFO as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x01D4, size: body_size, io, } } else { a } })?)),
            0x01DD => Ok(Self::SMSG_PONG(<SMSG_PONG as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x01DD, size: body_size, io, } } else { a } })?)),
            0x01DF => Ok(Self::SMSG_GAMEOBJECT_PAGETEXT(<SMSG_GAMEOBJECT_PAGETEXT as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x01DF, size: body_size, io, } } else { a } })?)),
            0x01EA => Ok(Self::SMSG_ITEM_TIME_UPDATE(<SMSG_ITEM_TIME_UPDATE as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x01EA, size: body_size, io, } } else { a } })?)),
            0x01EB => Ok(Self::SMSG_ITEM_ENCHANT_TIME_UPDATE(<SMSG_ITEM_ENCHANT_TIME_UPDATE as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x01EB, size: body_size, io, } } else { a } })?)),
            0x01EC => Ok(Self::SMSG_AUTH_CHALLENGE(<SMSG_AUTH_CHALLENGE as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x01EC, size: body_size, io, } } else { a } })?)),
            0x01EE => Ok(Self::SMSG_AUTH_RESPONSE(<SMSG_AUTH_RESPONSE as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x01EE, size: body_size, io, } } else { a } })?)),
            0x01F1 => Ok(Self::MSG_SAVE_GUILD_EMBLEM(<MSG_SAVE_GUILD_EMBLEM_Server as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x01F1, size: body_size, io, } } else { a } })?)),
            0x01F8 => Ok(Self::SMSG_EXPLORATION_EXPERIENCE(<SMSG_EXPLORATION_EXPERIENCE as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x01F8, size: body_size, io, } } else { a } })?)),
            0x01FB => Ok(Self::MSG_RANDOM_ROLL(<MSG_RANDOM_ROLL_Server as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x01FB, size: body_size, io, } } else { a } })?)),
            0x01FC => Ok(Self::SMSG_ENVIRONMENTALDAMAGELOG(<SMSG_ENVIRONMENTALDAMAGELOG as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x01FC, size: body_size, io, } } else { a } })?)),
            0x0206 => Ok(Self::SMSG_GMTICKET_CREATE(<SMSG_GMTICKET_CREATE as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x0206, size: body_size, io, } } else { a } })?)),
            0x0208 => Ok(Self::SMSG_GMTICKET_UPDATETEXT(<SMSG_GMTICKET_UPDATETEXT as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x0208, size: body_size, io, } } else { a } })?)),
            0x0209 => Ok(Self::SMSG_ACCOUNT_DATA_TIMES(<SMSG_ACCOUNT_DATA_TIMES as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x0209, size: body_size, io, } } else { a } })?)),
            0x020C => Ok(Self::SMSG_UPDATE_ACCOUNT_DATA(<SMSG_UPDATE_ACCOUNT_DATA as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x020C, size: body_size, io, } } else { a } })?)),
            0x0215 => Ok(Self::SMSG_GAMEOBJECT_DESPAWN_ANIM(<SMSG_GAMEOBJECT_DESPAWN_ANIM as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x0215, size: body_size, io, } } else { a } })?)),
            0x0218 => Ok(Self::SMSG_GMTICKET_DELETETICKET(<SMSG_GMTICKET_DELETETICKET as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x0218, size: body_size, io, } } else { a } })?)),
            0x0219 => Ok(Self::SMSG_CHAT_WRONG_FACTION(<SMSG_CHAT_WRONG_FACTION as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x0219, size: body_size, io, } } else { a } })?)),
            0x021B => Ok(Self::SMSG_GMTICKET_SYSTEMSTATUS(<SMSG_GMTICKET_SYSTEMSTATUS as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x021B, size: body_size, io, } } else { a } })?)),
            0x021E => Ok(Self::SMSG_SET_REST_START(<SMSG_SET_REST_START as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x021E, size: body_size, io, } } else { a } })?)),
            0x0224 => Ok(Self::SMSG_GOSSIP_POI(<SMSG_GOSSIP_POI as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x0224, size: body_size, io, } } else { a } })?)),
            0x0236 => Ok(Self::SMSG_LOGIN_VERIFY_WORLD(<SMSG_LOGIN_VERIFY_WORLD as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x0236, size: body_size, io, } } else { a } })?)),
            0x023D => Ok(Self::SMSG_BATTLEFIELD_LIST(<SMSG_BATTLEFIELD_LIST as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x023D, size: body_size, io, } } else { a } })?)),
            0x0254 => Ok(Self::SMSG_ZONE_UNDER_ATTACK(<SMSG_ZONE_UNDER_ATTACK as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x0254, size: body_size, io, } } else { a } })?)),
            0x0255 => Ok(Self::MSG_AUCTION_HELLO(<MSG_AUCTION_HELLO_Server as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x0255, size: body_size, io, } } else { a } })?)),
            0x025C => Ok(Self::SMSG_AUCTION_LIST_RESULT(<SMSG_AUCTION_LIST_RESULT as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x025C, size: body_size, io, } } else { a } })?)),
            0x025D => Ok(Self::SMSG_AUCTION_OWNER_LIST_RESULT(<SMSG_AUCTION_OWNER_LIST_RESULT as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x025D, size: body_size, io, } } else { a } })?)),
            0x025E => Ok(Self::SMSG_AUCTION_BIDDER_NOTIFICATION(<SMSG_AUCTION_BIDDER_NOTIFICATION as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x025E, size: body_size, io, } } else { a } })?)),
            0x025F => Ok(Self::SMSG_AUCTION_OWNER_NOTIFICATION(<SMSG_AUCTION_OWNER_NOTIFICATION as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x025F, size: body_size, io, } } else { a } })?)),
            0x0260 => Ok(Self::SMSG_PROCRESIST(<SMSG_PROCRESIST as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x0260, size: body_size, io, } } else { a } })?)),
            0x0265 => Ok(Self::SMSG_AUCTION_BIDDER_LIST_RESULT(<SMSG_AUCTION_BIDDER_LIST_RESULT as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x0265, size: body_size, io, } } else { a } })?)),
            0x0278 => Ok(Self::SMSG_PLAY_OBJECT_SOUND(<SMSG_PLAY_OBJECT_SOUND as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x0278, size: body_size, io, } } else { a } })?)),
            0x0285 => Ok(Self::SMSG_RECEIVED_MAIL(<SMSG_RECEIVED_MAIL as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x0285, size: body_size, io, } } else { a } })?)),
            0x028D => Ok(Self::SMSG_AUCTION_REMOVED_NOTIFICATION(<SMSG_AUCTION_REMOVED_NOTIFICATION as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x028D, size: body_size, io, } } else { a } })?)),
            0x0291 => Ok(Self::SMSG_SERVER_MESSAGE(<SMSG_SERVER_MESSAGE as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x0291, size: body_size, io, } } else { a } })?)),
            0x029D => Ok(Self::SMSG_STANDSTATE_UPDATE(<SMSG_STANDSTATE_UPDATE as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x029D, size: body_size, io, } } else { a } })?)),
            0x029E => Ok(Self::SMSG_LOOT_ALL_PASSED(<SMSG_LOOT_ALL_PASSED as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x029E, size: body_size, io, } } else { a } })?)),
            0x029F => Ok(Self::SMSG_LOOT_ROLL_WON(<SMSG_LOOT_ROLL_WON as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x029F, size: body_size, io, } } else { a } })?)),
            0x02A1 => Ok(Self::SMSG_LOOT_START_ROLL(<SMSG_LOOT_START_ROLL as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x02A1, size: body_size, io, } } else { a } })?)),
            0x02A2 => Ok(Self::SMSG_LOOT_ROLL(<SMSG_LOOT_ROLL as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x02A2, size: body_size, io, } } else { a } })?)),
            0x02A4 => Ok(Self::SMSG_LOOT_MASTER_LIST(<SMSG_LOOT_MASTER_LIST as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x02A4, size: body_size, io, } } else { a } })?)),
            0x02A5 => Ok(Self::SMSG_SET_FORCED_REACTIONS(<SMSG_SET_FORCED_REACTIONS as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x02A5, size: body_size, io, } } else { a } })?)),
            0x02A9 => Ok(Self::SMSG_CHAT_PLAYER_NOT_FOUND(<SMSG_CHAT_PLAYER_NOT_FOUND as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x02A9, size: body_size, io, } } else { a } })?)),
            0x02B7 => Ok(Self::SMSG_DUEL_COUNTDOWN(<SMSG_DUEL_COUNTDOWN as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x02B7, size: body_size, io, } } else { a } })?)),
            0x02B8 => Ok(Self::SMSG_AREA_TRIGGER_MESSAGE(<SMSG_AREA_TRIGGER_MESSAGE as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x02B8, size: body_size, io, } } else { a } })?)),
            0x02BD => Ok(Self::SMSG_DURABILITY_DAMAGE_DEATH(<SMSG_DURABILITY_DAMAGE_DEATH as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x02BD, size: body_size, io, } } else { a } })?)),
            0x02C2 => Ok(Self::SMSG_INIT_WORLD_STATES(<SMSG_INIT_WORLD_STATES as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x02C2, size: body_size, io, } } else { a } })?)),
            0x02C3 => Ok(Self::SMSG_UPDATE_WORLD_STATE(<SMSG_UPDATE_WORLD_STATE as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x02C3, size: body_size, io, } } else { a } })?)),
            0x02C8 => Ok(Self::SMSG_CHAR_RENAME(<SMSG_CHAR_RENAME as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x02C8, size: body_size, io, } } else { a } })?)),
            0x02E9 => Ok(Self::MSG_BATTLEGROUND_PLAYER_POSITIONS(<MSG_BATTLEGROUND_PLAYER_POSITIONS_Server as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x02E9, size: body_size, io, } } else { a } })?)),
            0x02EB => Ok(Self::SMSG_BINDER_CONFIRM(<SMSG_BINDER_CONFIRM as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x02EB, size: body_size, io, } } else { a } })?)),
            0x02EC => Ok(Self::SMSG_BATTLEGROUND_PLAYER_JOINED(<SMSG_BATTLEGROUND_PLAYER_JOINED as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x02EC, size: body_size, io, } } else { a } })?)),
            0x02ED => Ok(Self::SMSG_BATTLEGROUND_PLAYER_LEFT(<SMSG_BATTLEGROUND_PLAYER_LEFT as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x02ED, size: body_size, io, } } else { a } })?)),
            0x02EF => Ok(Self::SMSG_ADDON_INFO(<SMSG_ADDON_INFO as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x02EF, size: body_size, io, } } else { a } })?)),
            0x02FD => Ok(Self::SMSG_CHAT_RESTRICTED(<SMSG_CHAT_RESTRICTED as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x02FD, size: body_size, io, } } else { a } })?)),
            0x0328 => Ok(Self::SMSG_GM_TICKET_STATUS_UPDATE(<SMSG_GM_TICKET_STATUS_UPDATE as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x0328, size: body_size, io, } } else { a } })?)),
            0x033A => Ok(Self::SMSG_DEFENSE_MESSAGE(<SMSG_DEFENSE_MESSAGE as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x033A, size: body_size, io, } } else { a } })?)),
            0x038B => Ok(Self::SMSG_REALM_SPLIT(<SMSG_REALM_SPLIT as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x038B, size: body_size, io, } } else { a } })?)),
            0x0390 => Ok(Self::SMSG_TIME_SYNC_REQ(<SMSG_TIME_SYNC_REQ as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x0390, size: body_size, io, } } else { a } })?)),
            0x03C9 => Ok(Self::SMSG_FEATURE_SYSTEM_STATUS(<SMSG_FEATURE_SYSTEM_STATUS as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x03C9, size: body_size, io, } } else { a } })?)),
            0x0463 => Ok(Self::SMSG_UPDATE_ACCOUNT_DATA_COMPLETE(<SMSG_UPDATE_ACCOUNT_DATA_COMPLETE as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x0463, size: body_size, io, } } else { a } })?)),
            0x04AB => Ok(Self::SMSG_CLIENTCACHE_VERSION(<SMSG_CLIENTCACHE_VERSION as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x04AB, size: body_size, io, } } else { a } })?)),
            0x04F7 => Ok(Self::SMSG_WORLD_STATE_UI_TIMER_UPDATE(<SMSG_WORLD_STATE_UI_TIMER_UPDATE as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x04F7, size: body_size, io, } } else { a } })?)),
            _ => Err(crate::errors::ExpectedOpcodeError::Opcode{ opcode: opcode.into(), size: body_size }),
        }
    }

    #[cfg(feature = "sync")]
    pub fn read_unencrypted<R: std::io::Read>(r: &mut R) -> std::result::Result<Self, crate::errors::ExpectedOpcodeError> {
        let mut header = [0_u8; 4];
        r.read_exact(&mut header)?;

        let (size, opcode) = if header[0] & 0x80 != 0 {
            let size = u32::from_be_bytes([0x00, header[0] & 0x7F, header[1], header[2]]).saturating_sub(3);

            let mut last_byte = [0_u8; 1];
            r.read_exact(&mut last_byte)?;
            let opcode = u16::from_le_bytes([header[3], last_byte[0]]);
            (size, opcode)
        }
        else {
            let size = u16::from_be_bytes([header[0], header[1]]).saturating_sub(2).into();
            let opcode = u16::from_le_bytes([header[2], header[3]]);
            (size, opcode)
        };

        let mut buf = vec![0; size as usize];
        r.read_exact(&mut buf)?;
        Self::read_opcodes(opcode, size, &buf)
    }
    #[cfg(all(feature = "sync", feature = "encryption"))]
    pub fn read_encrypted<R: std::io::Read>(r: &mut R, d: &mut ClientDecrypterHalf) -> std::result::Result<Self, crate::errors::ExpectedOpcodeError> {
        let mut header = [0_u8; 4];
        r.read_exact(&mut header)?;
        d.decrypt(&mut header);

        let (body_size, opcode) = if header[0] & 0x80 != 0 {
            let size = u32::from_be_bytes([0x00, header[0] & 0x7F, header[1], header[2]]).saturating_sub(3);

            let mut last_byte = [0_u8; 1];
            r.read_exact(&mut last_byte)?;
            d.decrypt(&mut last_byte);
            let opcode = u16::from_le_bytes([header[3], last_byte[0]]);
            (size, opcode)
        }
        else {
            let size = u16::from_be_bytes([header[0], header[1]]).saturating_sub(2).into();
            let opcode = u16::from_le_bytes([header[2], header[3]]);
            (size, opcode)
        };

        let mut buf = vec![0; body_size as usize];
        r.read_exact(&mut buf)?;
        Self::read_opcodes(opcode, body_size, &buf)
    }

    #[cfg(feature = "tokio")]
    pub async fn tokio_read_unencrypted<R: tokio::io::AsyncReadExt + Unpin + Send>(r: &mut R) -> std::result::Result<Self, crate::errors::ExpectedOpcodeError> {
        let mut header = [0_u8; 4];
        r.read_exact(&mut header).await?;

        let (size, opcode) = if header[0] & 0x80 != 0 {
            let size = u32::from_be_bytes([0x00, header[0] & 0x7F, header[1], header[2]]).saturating_sub(3);

            let mut last_byte = [0_u8; 1];
            r.read_exact(&mut last_byte).await?;
            let opcode = u16::from_le_bytes([header[3], last_byte[0]]);
            (size, opcode)
        }
        else {
            let size = u16::from_be_bytes([header[0], header[1]]).saturating_sub(2).into();
            let opcode = u16::from_le_bytes([header[2], header[3]]);
            (size, opcode)
        };

        let mut buf = vec![0; size as usize];
        r.read_exact(&mut buf).await?;
        Self::read_opcodes(opcode, size, &buf)
    }
    #[cfg(all(feature = "tokio", feature = "encryption"))]
    pub async fn tokio_read_encrypted<R: tokio::io::AsyncReadExt + Unpin + Send>(r: &mut R, d: &mut ClientDecrypterHalf) -> std::result::Result<Self, crate::errors::ExpectedOpcodeError> {
        let mut header = [0_u8; 4];
        r.read_exact(&mut header).await?;
        d.decrypt(&mut header);

        let (body_size, opcode) = if header[0] & 0x80 != 0 {
            let size = u32::from_be_bytes([0x00, header[0] & 0x7F, header[1], header[2]]).saturating_sub(3);

            let mut last_byte = [0_u8; 1];
            r.read_exact(&mut last_byte).await?;
            d.decrypt(&mut last_byte);
            let opcode = u16::from_le_bytes([header[3], last_byte[0]]);
            (size, opcode)
        }
        else {
            let size = u16::from_be_bytes([header[0], header[1]]).saturating_sub(2).into();
            let opcode = u16::from_le_bytes([header[2], header[3]]);
            (size, opcode)
        };

        let mut buf = vec![0; body_size as usize];
        r.read_exact(&mut buf).await?;
        Self::read_opcodes(opcode, body_size, &buf)
    }

    #[cfg(feature = "async-std")]
    pub async fn astd_read_unencrypted<R: async_std::io::ReadExt + Unpin + Send>(r: &mut R) -> std::result::Result<Self, crate::errors::ExpectedOpcodeError> {
        let mut header = [0_u8; 4];
        r.read_exact(&mut header).await?;

        let (size, opcode) = if header[0] & 0x80 != 0 {
            let size = u32::from_be_bytes([0x00, header[0] & 0x7F, header[1], header[2]]).saturating_sub(3);

            let mut last_byte = [0_u8; 1];
            r.read_exact(&mut last_byte).await?;
            let opcode = u16::from_le_bytes([header[3], last_byte[0]]);
            (size, opcode)
        }
        else {
            let size = u16::from_be_bytes([header[0], header[1]]).saturating_sub(2).into();
            let opcode = u16::from_le_bytes([header[2], header[3]]);
            (size, opcode)
        };

        let mut buf = vec![0; size as usize];
        r.read_exact(&mut buf).await?;
        Self::read_opcodes(opcode, size, &buf)
    }
    #[cfg(all(feature = "async-std", feature = "encryption"))]
    pub async fn astd_read_encrypted<R: async_std::io::ReadExt + Unpin + Send>(r: &mut R, d: &mut ClientDecrypterHalf) -> std::result::Result<Self, crate::errors::ExpectedOpcodeError> {
        let mut header = [0_u8; 4];
        r.read_exact(&mut header).await?;
        d.decrypt(&mut header);

        let (body_size, opcode) = if header[0] & 0x80 != 0 {
            let size = u32::from_be_bytes([0x00, header[0] & 0x7F, header[1], header[2]]).saturating_sub(3);

            let mut last_byte = [0_u8; 1];
            r.read_exact(&mut last_byte).await?;
            d.decrypt(&mut last_byte);
            let opcode = u16::from_le_bytes([header[3], last_byte[0]]);
            (size, opcode)
        }
        else {
            let size = u16::from_be_bytes([header[0], header[1]]).saturating_sub(2).into();
            let opcode = u16::from_le_bytes([header[2], header[3]]);
            (size, opcode)
        };

        let mut buf = vec![0; body_size as usize];
        r.read_exact(&mut buf).await?;
        Self::read_opcodes(opcode, body_size, &buf)
    }

    #[cfg(all(feature = "sync", feature = "encryption"))]
    pub fn write_encrypted_server<W: std::io::Write>(&self, w: &mut W, e: &mut ServerEncrypterHalf) -> Result<(), std::io::Error> {
        match self {
            Self::MSG_MOVE_START_FORWARD(c) => c.write_encrypted_server(w, e),
            Self::MSG_MOVE_START_BACKWARD(c) => c.write_encrypted_server(w, e),
            Self::MSG_MOVE_STOP(c) => c.write_encrypted_server(w, e),
            Self::MSG_MOVE_START_STRAFE_LEFT(c) => c.write_encrypted_server(w, e),
            Self::MSG_MOVE_START_STRAFE_RIGHT(c) => c.write_encrypted_server(w, e),
            Self::MSG_MOVE_STOP_STRAFE(c) => c.write_encrypted_server(w, e),
            Self::MSG_MOVE_JUMP(c) => c.write_encrypted_server(w, e),
            Self::MSG_MOVE_START_TURN_LEFT(c) => c.write_encrypted_server(w, e),
            Self::MSG_MOVE_START_TURN_RIGHT(c) => c.write_encrypted_server(w, e),
            Self::MSG_MOVE_STOP_TURN(c) => c.write_encrypted_server(w, e),
            Self::MSG_MOVE_START_PITCH_UP(c) => c.write_encrypted_server(w, e),
            Self::MSG_MOVE_START_PITCH_DOWN(c) => c.write_encrypted_server(w, e),
            Self::MSG_MOVE_STOP_PITCH(c) => c.write_encrypted_server(w, e),
            Self::MSG_MOVE_SET_RUN_MODE(c) => c.write_encrypted_server(w, e),
            Self::MSG_MOVE_SET_WALK_MODE(c) => c.write_encrypted_server(w, e),
            Self::MSG_MOVE_FALL_LAND(c) => c.write_encrypted_server(w, e),
            Self::MSG_MOVE_START_SWIM(c) => c.write_encrypted_server(w, e),
            Self::MSG_MOVE_STOP_SWIM(c) => c.write_encrypted_server(w, e),
            Self::MSG_MOVE_SET_FACING(c) => c.write_encrypted_server(w, e),
            Self::MSG_MOVE_SET_PITCH(c) => c.write_encrypted_server(w, e),
            Self::MSG_MOVE_WORLDPORT_ACK(c) => c.write_encrypted_server(w, e),
            Self::MSG_MOVE_HEARTBEAT(c) => c.write_encrypted_server(w, e),
            Self::MSG_PETITION_DECLINE(c) => c.write_encrypted_server(w, e),
            Self::MSG_TABARDVENDOR_ACTIVATE(c) => c.write_encrypted_server(w, e),
            Self::MSG_PETITION_RENAME(c) => c.write_encrypted_server(w, e),
            Self::MSG_SET_DUNGEON_DIFFICULTY(c) => c.write_encrypted_server(w, e),
            Self::MSG_MOVE_START_ASCEND(c) => c.write_encrypted_server(w, e),
            Self::MSG_MOVE_STOP_ASCEND(c) => c.write_encrypted_server(w, e),
            Self::MSG_MOVE_START_DESCEND(c) => c.write_encrypted_server(w, e),
            Self::SMSG_CHAR_CREATE(c) => c.write_encrypted_server(w, e),
            Self::SMSG_CHAR_ENUM(c) => c.write_encrypted_server(w, e),
            Self::SMSG_CHAR_DELETE(c) => c.write_encrypted_server(w, e),
            Self::SMSG_NEW_WORLD(c) => c.write_encrypted_server(w, e),
            Self::SMSG_TRANSFER_PENDING(c) => c.write_encrypted_server(w, e),
            Self::SMSG_CHARACTER_LOGIN_FAILED(c) => c.write_encrypted_server(w, e),
            Self::SMSG_LOGIN_SETTIMESPEED(c) => c.write_encrypted_server(w, e),
            Self::SMSG_LOGOUT_RESPONSE(c) => c.write_encrypted_server(w, e),
            Self::SMSG_LOGOUT_COMPLETE(c) => c.write_encrypted_server(w, e),
            Self::SMSG_LOGOUT_CANCEL_ACK(c) => c.write_encrypted_server(w, e),
            Self::SMSG_NAME_QUERY_RESPONSE(c) => c.write_encrypted_server(w, e),
            Self::SMSG_CONTACT_LIST(c) => c.write_encrypted_server(w, e),
            Self::SMSG_GUILD_INVITE(c) => c.write_encrypted_server(w, e),
            Self::SMSG_GUILD_INFO(c) => c.write_encrypted_server(w, e),
            Self::SMSG_GUILD_EVENT(c) => c.write_encrypted_server(w, e),
            Self::SMSG_GUILD_COMMAND_RESULT(c) => c.write_encrypted_server(w, e),
            Self::SMSG_CHANNEL_NOTIFY(c) => c.write_encrypted_server(w, e),
            Self::SMSG_CHANNEL_LIST(c) => c.write_encrypted_server(w, e),
            Self::SMSG_UPDATE_OBJECT(c) => c.write_encrypted_server(w, e),
            Self::SMSG_DESTROY_OBJECT(c) => c.write_encrypted_server(w, e),
            Self::SMSG_READ_ITEM_OK(c) => c.write_encrypted_server(w, e),
            Self::SMSG_READ_ITEM_FAILED(c) => c.write_encrypted_server(w, e),
            Self::SMSG_ITEM_COOLDOWN(c) => c.write_encrypted_server(w, e),
            Self::SMSG_GAMEOBJECT_CUSTOM_ANIM(c) => c.write_encrypted_server(w, e),
            Self::MSG_MOVE_TELEPORT_ACK(c) => c.write_encrypted_server(w, e),
            Self::SMSG_FORCE_MOVE_ROOT(c) => c.write_encrypted_server(w, e),
            Self::SMSG_FORCE_MOVE_UNROOT(c) => c.write_encrypted_server(w, e),
            Self::SMSG_TRIGGER_CINEMATIC(c) => c.write_encrypted_server(w, e),
            Self::SMSG_TUTORIAL_FLAGS(c) => c.write_encrypted_server(w, e),
            Self::SMSG_EMOTE(c) => c.write_encrypted_server(w, e),
            Self::SMSG_TEXT_EMOTE(c) => c.write_encrypted_server(w, e),
            Self::SMSG_INVENTORY_CHANGE_FAILURE(c) => c.write_encrypted_server(w, e),
            Self::SMSG_INITIALIZE_FACTIONS(c) => c.write_encrypted_server(w, e),
            Self::SMSG_SET_FACTION_VISIBLE(c) => c.write_encrypted_server(w, e),
            Self::SMSG_SET_FACTION_STANDING(c) => c.write_encrypted_server(w, e),
            Self::SMSG_SET_PROFICIENCY(c) => c.write_encrypted_server(w, e),
            Self::SMSG_ACTION_BUTTONS(c) => c.write_encrypted_server(w, e),
            Self::SMSG_AI_REACTION(c) => c.write_encrypted_server(w, e),
            Self::SMSG_ATTACKSTART(c) => c.write_encrypted_server(w, e),
            Self::SMSG_ATTACKSTOP(c) => c.write_encrypted_server(w, e),
            Self::SMSG_ATTACKSWING_NOTINRANGE(c) => c.write_encrypted_server(w, e),
            Self::SMSG_ATTACKSWING_BADFACING(c) => c.write_encrypted_server(w, e),
            Self::SMSG_ATTACKSWING_DEADTARGET(c) => c.write_encrypted_server(w, e),
            Self::SMSG_ATTACKSWING_CANT_ATTACK(c) => c.write_encrypted_server(w, e),
            Self::SMSG_ATTACKERSTATEUPDATE(c) => c.write_encrypted_server(w, e),
            Self::SMSG_CANCEL_COMBAT(c) => c.write_encrypted_server(w, e),
            Self::SMSG_BINDPOINTUPDATE(c) => c.write_encrypted_server(w, e),
            Self::SMSG_LOOT_RELEASE_RESPONSE(c) => c.write_encrypted_server(w, e),
            Self::SMSG_LOOT_REMOVED(c) => c.write_encrypted_server(w, e),
            Self::SMSG_LOOT_MONEY_NOTIFY(c) => c.write_encrypted_server(w, e),
            Self::SMSG_LOOT_CLEAR_MONEY(c) => c.write_encrypted_server(w, e),
            Self::SMSG_ITEM_PUSH_RESULT(c) => c.write_encrypted_server(w, e),
            Self::SMSG_DUEL_REQUESTED(c) => c.write_encrypted_server(w, e),
            Self::SMSG_DUEL_OUTOFBOUNDS(c) => c.write_encrypted_server(w, e),
            Self::SMSG_DUEL_INBOUNDS(c) => c.write_encrypted_server(w, e),
            Self::SMSG_DUEL_COMPLETE(c) => c.write_encrypted_server(w, e),
            Self::SMSG_DUEL_WINNER(c) => c.write_encrypted_server(w, e),
            Self::SMSG_MOUNTRESULT(c) => c.write_encrypted_server(w, e),
            Self::SMSG_MOUNTSPECIAL_ANIM(c) => c.write_encrypted_server(w, e),
            Self::SMSG_GOSSIP_MESSAGE(c) => c.write_encrypted_server(w, e),
            Self::SMSG_GOSSIP_COMPLETE(c) => c.write_encrypted_server(w, e),
            Self::SMSG_NPC_TEXT_UPDATE(c) => c.write_encrypted_server(w, e),
            Self::SMSG_LIST_INVENTORY(c) => c.write_encrypted_server(w, e),
            Self::SMSG_SELL_ITEM(c) => c.write_encrypted_server(w, e),
            Self::SMSG_BUY_ITEM(c) => c.write_encrypted_server(w, e),
            Self::SMSG_BUY_FAILED(c) => c.write_encrypted_server(w, e),
            Self::SMSG_SHOW_BANK(c) => c.write_encrypted_server(w, e),
            Self::SMSG_BUY_BANK_SLOT_RESULT(c) => c.write_encrypted_server(w, e),
            Self::SMSG_PETITION_SHOW_SIGNATURES(c) => c.write_encrypted_server(w, e),
            Self::SMSG_PETITION_SIGN_RESULTS(c) => c.write_encrypted_server(w, e),
            Self::SMSG_TURN_IN_PETITION_RESULTS(c) => c.write_encrypted_server(w, e),
            Self::SMSG_NOTIFICATION(c) => c.write_encrypted_server(w, e),
            Self::SMSG_PLAYED_TIME(c) => c.write_encrypted_server(w, e),
            Self::SMSG_QUERY_TIME_RESPONSE(c) => c.write_encrypted_server(w, e),
            Self::SMSG_LOG_XPGAIN(c) => c.write_encrypted_server(w, e),
            Self::SMSG_LEVELUP_INFO(c) => c.write_encrypted_server(w, e),
            Self::SMSG_PONG(c) => c.write_encrypted_server(w, e),
            Self::SMSG_GAMEOBJECT_PAGETEXT(c) => c.write_encrypted_server(w, e),
            Self::SMSG_ITEM_TIME_UPDATE(c) => c.write_encrypted_server(w, e),
            Self::SMSG_ITEM_ENCHANT_TIME_UPDATE(c) => c.write_encrypted_server(w, e),
            Self::SMSG_AUTH_CHALLENGE(c) => c.write_encrypted_server(w, e),
            Self::SMSG_AUTH_RESPONSE(c) => c.write_encrypted_server(w, e),
            Self::MSG_SAVE_GUILD_EMBLEM(c) => c.write_encrypted_server(w, e),
            Self::SMSG_EXPLORATION_EXPERIENCE(c) => c.write_encrypted_server(w, e),
            Self::MSG_RANDOM_ROLL(c) => c.write_encrypted_server(w, e),
            Self::SMSG_ENVIRONMENTALDAMAGELOG(c) => c.write_encrypted_server(w, e),
            Self::SMSG_GMTICKET_CREATE(c) => c.write_encrypted_server(w, e),
            Self::SMSG_GMTICKET_UPDATETEXT(c) => c.write_encrypted_server(w, e),
            Self::SMSG_ACCOUNT_DATA_TIMES(c) => c.write_encrypted_server(w, e),
            Self::SMSG_UPDATE_ACCOUNT_DATA(c) => c.write_encrypted_server(w, e),
            Self::SMSG_GAMEOBJECT_DESPAWN_ANIM(c) => c.write_encrypted_server(w, e),
            Self::SMSG_GMTICKET_DELETETICKET(c) => c.write_encrypted_server(w, e),
            Self::SMSG_CHAT_WRONG_FACTION(c) => c.write_encrypted_server(w, e),
            Self::SMSG_GMTICKET_SYSTEMSTATUS(c) => c.write_encrypted_server(w, e),
            Self::SMSG_SET_REST_START(c) => c.write_encrypted_server(w, e),
            Self::SMSG_GOSSIP_POI(c) => c.write_encrypted_server(w, e),
            Self::SMSG_LOGIN_VERIFY_WORLD(c) => c.write_encrypted_server(w, e),
            Self::SMSG_BATTLEFIELD_LIST(c) => c.write_encrypted_server(w, e),
            Self::SMSG_ZONE_UNDER_ATTACK(c) => c.write_encrypted_server(w, e),
            Self::MSG_AUCTION_HELLO(c) => c.write_encrypted_server(w, e),
            Self::SMSG_AUCTION_LIST_RESULT(c) => c.write_encrypted_server(w, e),
            Self::SMSG_AUCTION_OWNER_LIST_RESULT(c) => c.write_encrypted_server(w, e),
            Self::SMSG_AUCTION_BIDDER_NOTIFICATION(c) => c.write_encrypted_server(w, e),
            Self::SMSG_AUCTION_OWNER_NOTIFICATION(c) => c.write_encrypted_server(w, e),
            Self::SMSG_PROCRESIST(c) => c.write_encrypted_server(w, e),
            Self::SMSG_AUCTION_BIDDER_LIST_RESULT(c) => c.write_encrypted_server(w, e),
            Self::SMSG_PLAY_OBJECT_SOUND(c) => c.write_encrypted_server(w, e),
            Self::SMSG_RECEIVED_MAIL(c) => c.write_encrypted_server(w, e),
            Self::SMSG_AUCTION_REMOVED_NOTIFICATION(c) => c.write_encrypted_server(w, e),
            Self::SMSG_SERVER_MESSAGE(c) => c.write_encrypted_server(w, e),
            Self::SMSG_STANDSTATE_UPDATE(c) => c.write_encrypted_server(w, e),
            Self::SMSG_LOOT_ALL_PASSED(c) => c.write_encrypted_server(w, e),
            Self::SMSG_LOOT_ROLL_WON(c) => c.write_encrypted_server(w, e),
            Self::SMSG_LOOT_START_ROLL(c) => c.write_encrypted_server(w, e),
            Self::SMSG_LOOT_ROLL(c) => c.write_encrypted_server(w, e),
            Self::SMSG_LOOT_MASTER_LIST(c) => c.write_encrypted_server(w, e),
            Self::SMSG_SET_FORCED_REACTIONS(c) => c.write_encrypted_server(w, e),
            Self::SMSG_CHAT_PLAYER_NOT_FOUND(c) => c.write_encrypted_server(w, e),
            Self::SMSG_DUEL_COUNTDOWN(c) => c.write_encrypted_server(w, e),
            Self::SMSG_AREA_TRIGGER_MESSAGE(c) => c.write_encrypted_server(w, e),
            Self::SMSG_DURABILITY_DAMAGE_DEATH(c) => c.write_encrypted_server(w, e),
            Self::SMSG_INIT_WORLD_STATES(c) => c.write_encrypted_server(w, e),
            Self::SMSG_UPDATE_WORLD_STATE(c) => c.write_encrypted_server(w, e),
            Self::SMSG_CHAR_RENAME(c) => c.write_encrypted_server(w, e),
            Self::MSG_BATTLEGROUND_PLAYER_POSITIONS(c) => c.write_encrypted_server(w, e),
            Self::SMSG_BINDER_CONFIRM(c) => c.write_encrypted_server(w, e),
            Self::SMSG_BATTLEGROUND_PLAYER_JOINED(c) => c.write_encrypted_server(w, e),
            Self::SMSG_BATTLEGROUND_PLAYER_LEFT(c) => c.write_encrypted_server(w, e),
            Self::SMSG_ADDON_INFO(c) => c.write_encrypted_server(w, e),
            Self::SMSG_CHAT_RESTRICTED(c) => c.write_encrypted_server(w, e),
            Self::SMSG_GM_TICKET_STATUS_UPDATE(c) => c.write_encrypted_server(w, e),
            Self::SMSG_DEFENSE_MESSAGE(c) => c.write_encrypted_server(w, e),
            Self::SMSG_REALM_SPLIT(c) => c.write_encrypted_server(w, e),
            Self::SMSG_TIME_SYNC_REQ(c) => c.write_encrypted_server(w, e),
            Self::SMSG_FEATURE_SYSTEM_STATUS(c) => c.write_encrypted_server(w, e),
            Self::SMSG_UPDATE_ACCOUNT_DATA_COMPLETE(c) => c.write_encrypted_server(w, e),
            Self::SMSG_CLIENTCACHE_VERSION(c) => c.write_encrypted_server(w, e),
            Self::SMSG_WORLD_STATE_UI_TIMER_UPDATE(c) => c.write_encrypted_server(w, e),
        }
    }

    #[cfg(feature = "sync")]
    pub fn write_unencrypted_server<W: std::io::Write>(&self, w: &mut W) -> Result<(), std::io::Error> {
        match self {
            Self::MSG_MOVE_START_FORWARD(c) => c.write_unencrypted_server(w),
            Self::MSG_MOVE_START_BACKWARD(c) => c.write_unencrypted_server(w),
            Self::MSG_MOVE_STOP(c) => c.write_unencrypted_server(w),
            Self::MSG_MOVE_START_STRAFE_LEFT(c) => c.write_unencrypted_server(w),
            Self::MSG_MOVE_START_STRAFE_RIGHT(c) => c.write_unencrypted_server(w),
            Self::MSG_MOVE_STOP_STRAFE(c) => c.write_unencrypted_server(w),
            Self::MSG_MOVE_JUMP(c) => c.write_unencrypted_server(w),
            Self::MSG_MOVE_START_TURN_LEFT(c) => c.write_unencrypted_server(w),
            Self::MSG_MOVE_START_TURN_RIGHT(c) => c.write_unencrypted_server(w),
            Self::MSG_MOVE_STOP_TURN(c) => c.write_unencrypted_server(w),
            Self::MSG_MOVE_START_PITCH_UP(c) => c.write_unencrypted_server(w),
            Self::MSG_MOVE_START_PITCH_DOWN(c) => c.write_unencrypted_server(w),
            Self::MSG_MOVE_STOP_PITCH(c) => c.write_unencrypted_server(w),
            Self::MSG_MOVE_SET_RUN_MODE(c) => c.write_unencrypted_server(w),
            Self::MSG_MOVE_SET_WALK_MODE(c) => c.write_unencrypted_server(w),
            Self::MSG_MOVE_FALL_LAND(c) => c.write_unencrypted_server(w),
            Self::MSG_MOVE_START_SWIM(c) => c.write_unencrypted_server(w),
            Self::MSG_MOVE_STOP_SWIM(c) => c.write_unencrypted_server(w),
            Self::MSG_MOVE_SET_FACING(c) => c.write_unencrypted_server(w),
            Self::MSG_MOVE_SET_PITCH(c) => c.write_unencrypted_server(w),
            Self::MSG_MOVE_WORLDPORT_ACK(c) => c.write_unencrypted_server(w),
            Self::MSG_MOVE_HEARTBEAT(c) => c.write_unencrypted_server(w),
            Self::MSG_PETITION_DECLINE(c) => c.write_unencrypted_server(w),
            Self::MSG_TABARDVENDOR_ACTIVATE(c) => c.write_unencrypted_server(w),
            Self::MSG_PETITION_RENAME(c) => c.write_unencrypted_server(w),
            Self::MSG_SET_DUNGEON_DIFFICULTY(c) => c.write_unencrypted_server(w),
            Self::MSG_MOVE_START_ASCEND(c) => c.write_unencrypted_server(w),
            Self::MSG_MOVE_STOP_ASCEND(c) => c.write_unencrypted_server(w),
            Self::MSG_MOVE_START_DESCEND(c) => c.write_unencrypted_server(w),
            Self::SMSG_CHAR_CREATE(c) => c.write_unencrypted_server(w),
            Self::SMSG_CHAR_ENUM(c) => c.write_unencrypted_server(w),
            Self::SMSG_CHAR_DELETE(c) => c.write_unencrypted_server(w),
            Self::SMSG_NEW_WORLD(c) => c.write_unencrypted_server(w),
            Self::SMSG_TRANSFER_PENDING(c) => c.write_unencrypted_server(w),
            Self::SMSG_CHARACTER_LOGIN_FAILED(c) => c.write_unencrypted_server(w),
            Self::SMSG_LOGIN_SETTIMESPEED(c) => c.write_unencrypted_server(w),
            Self::SMSG_LOGOUT_RESPONSE(c) => c.write_unencrypted_server(w),
            Self::SMSG_LOGOUT_COMPLETE(c) => c.write_unencrypted_server(w),
            Self::SMSG_LOGOUT_CANCEL_ACK(c) => c.write_unencrypted_server(w),
            Self::SMSG_NAME_QUERY_RESPONSE(c) => c.write_unencrypted_server(w),
            Self::SMSG_CONTACT_LIST(c) => c.write_unencrypted_server(w),
            Self::SMSG_GUILD_INVITE(c) => c.write_unencrypted_server(w),
            Self::SMSG_GUILD_INFO(c) => c.write_unencrypted_server(w),
            Self::SMSG_GUILD_EVENT(c) => c.write_unencrypted_server(w),
            Self::SMSG_GUILD_COMMAND_RESULT(c) => c.write_unencrypted_server(w),
            Self::SMSG_CHANNEL_NOTIFY(c) => c.write_unencrypted_server(w),
            Self::SMSG_CHANNEL_LIST(c) => c.write_unencrypted_server(w),
            Self::SMSG_UPDATE_OBJECT(c) => c.write_unencrypted_server(w),
            Self::SMSG_DESTROY_OBJECT(c) => c.write_unencrypted_server(w),
            Self::SMSG_READ_ITEM_OK(c) => c.write_unencrypted_server(w),
            Self::SMSG_READ_ITEM_FAILED(c) => c.write_unencrypted_server(w),
            Self::SMSG_ITEM_COOLDOWN(c) => c.write_unencrypted_server(w),
            Self::SMSG_GAMEOBJECT_CUSTOM_ANIM(c) => c.write_unencrypted_server(w),
            Self::MSG_MOVE_TELEPORT_ACK(c) => c.write_unencrypted_server(w),
            Self::SMSG_FORCE_MOVE_ROOT(c) => c.write_unencrypted_server(w),
            Self::SMSG_FORCE_MOVE_UNROOT(c) => c.write_unencrypted_server(w),
            Self::SMSG_TRIGGER_CINEMATIC(c) => c.write_unencrypted_server(w),
            Self::SMSG_TUTORIAL_FLAGS(c) => c.write_unencrypted_server(w),
            Self::SMSG_EMOTE(c) => c.write_unencrypted_server(w),
            Self::SMSG_TEXT_EMOTE(c) => c.write_unencrypted_server(w),
            Self::SMSG_INVENTORY_CHANGE_FAILURE(c) => c.write_unencrypted_server(w),
            Self::SMSG_INITIALIZE_FACTIONS(c) => c.write_unencrypted_server(w),
            Self::SMSG_SET_FACTION_VISIBLE(c) => c.write_unencrypted_server(w),
            Self::SMSG_SET_FACTION_STANDING(c) => c.write_unencrypted_server(w),
            Self::SMSG_SET_PROFICIENCY(c) => c.write_unencrypted_server(w),
            Self::SMSG_ACTION_BUTTONS(c) => c.write_unencrypted_server(w),
            Self::SMSG_AI_REACTION(c) => c.write_unencrypted_server(w),
            Self::SMSG_ATTACKSTART(c) => c.write_unencrypted_server(w),
            Self::SMSG_ATTACKSTOP(c) => c.write_unencrypted_server(w),
            Self::SMSG_ATTACKSWING_NOTINRANGE(c) => c.write_unencrypted_server(w),
            Self::SMSG_ATTACKSWING_BADFACING(c) => c.write_unencrypted_server(w),
            Self::SMSG_ATTACKSWING_DEADTARGET(c) => c.write_unencrypted_server(w),
            Self::SMSG_ATTACKSWING_CANT_ATTACK(c) => c.write_unencrypted_server(w),
            Self::SMSG_ATTACKERSTATEUPDATE(c) => c.write_unencrypted_server(w),
            Self::SMSG_CANCEL_COMBAT(c) => c.write_unencrypted_server(w),
            Self::SMSG_BINDPOINTUPDATE(c) => c.write_unencrypted_server(w),
            Self::SMSG_LOOT_RELEASE_RESPONSE(c) => c.write_unencrypted_server(w),
            Self::SMSG_LOOT_REMOVED(c) => c.write_unencrypted_server(w),
            Self::SMSG_LOOT_MONEY_NOTIFY(c) => c.write_unencrypted_server(w),
            Self::SMSG_LOOT_CLEAR_MONEY(c) => c.write_unencrypted_server(w),
            Self::SMSG_ITEM_PUSH_RESULT(c) => c.write_unencrypted_server(w),
            Self::SMSG_DUEL_REQUESTED(c) => c.write_unencrypted_server(w),
            Self::SMSG_DUEL_OUTOFBOUNDS(c) => c.write_unencrypted_server(w),
            Self::SMSG_DUEL_INBOUNDS(c) => c.write_unencrypted_server(w),
            Self::SMSG_DUEL_COMPLETE(c) => c.write_unencrypted_server(w),
            Self::SMSG_DUEL_WINNER(c) => c.write_unencrypted_server(w),
            Self::SMSG_MOUNTRESULT(c) => c.write_unencrypted_server(w),
            Self::SMSG_MOUNTSPECIAL_ANIM(c) => c.write_unencrypted_server(w),
            Self::SMSG_GOSSIP_MESSAGE(c) => c.write_unencrypted_server(w),
            Self::SMSG_GOSSIP_COMPLETE(c) => c.write_unencrypted_server(w),
            Self::SMSG_NPC_TEXT_UPDATE(c) => c.write_unencrypted_server(w),
            Self::SMSG_LIST_INVENTORY(c) => c.write_unencrypted_server(w),
            Self::SMSG_SELL_ITEM(c) => c.write_unencrypted_server(w),
            Self::SMSG_BUY_ITEM(c) => c.write_unencrypted_server(w),
            Self::SMSG_BUY_FAILED(c) => c.write_unencrypted_server(w),
            Self::SMSG_SHOW_BANK(c) => c.write_unencrypted_server(w),
            Self::SMSG_BUY_BANK_SLOT_RESULT(c) => c.write_unencrypted_server(w),
            Self::SMSG_PETITION_SHOW_SIGNATURES(c) => c.write_unencrypted_server(w),
            Self::SMSG_PETITION_SIGN_RESULTS(c) => c.write_unencrypted_server(w),
            Self::SMSG_TURN_IN_PETITION_RESULTS(c) => c.write_unencrypted_server(w),
            Self::SMSG_NOTIFICATION(c) => c.write_unencrypted_server(w),
            Self::SMSG_PLAYED_TIME(c) => c.write_unencrypted_server(w),
            Self::SMSG_QUERY_TIME_RESPONSE(c) => c.write_unencrypted_server(w),
            Self::SMSG_LOG_XPGAIN(c) => c.write_unencrypted_server(w),
            Self::SMSG_LEVELUP_INFO(c) => c.write_unencrypted_server(w),
            Self::SMSG_PONG(c) => c.write_unencrypted_server(w),
            Self::SMSG_GAMEOBJECT_PAGETEXT(c) => c.write_unencrypted_server(w),
            Self::SMSG_ITEM_TIME_UPDATE(c) => c.write_unencrypted_server(w),
            Self::SMSG_ITEM_ENCHANT_TIME_UPDATE(c) => c.write_unencrypted_server(w),
            Self::SMSG_AUTH_CHALLENGE(c) => c.write_unencrypted_server(w),
            Self::SMSG_AUTH_RESPONSE(c) => c.write_unencrypted_server(w),
            Self::MSG_SAVE_GUILD_EMBLEM(c) => c.write_unencrypted_server(w),
            Self::SMSG_EXPLORATION_EXPERIENCE(c) => c.write_unencrypted_server(w),
            Self::MSG_RANDOM_ROLL(c) => c.write_unencrypted_server(w),
            Self::SMSG_ENVIRONMENTALDAMAGELOG(c) => c.write_unencrypted_server(w),
            Self::SMSG_GMTICKET_CREATE(c) => c.write_unencrypted_server(w),
            Self::SMSG_GMTICKET_UPDATETEXT(c) => c.write_unencrypted_server(w),
            Self::SMSG_ACCOUNT_DATA_TIMES(c) => c.write_unencrypted_server(w),
            Self::SMSG_UPDATE_ACCOUNT_DATA(c) => c.write_unencrypted_server(w),
            Self::SMSG_GAMEOBJECT_DESPAWN_ANIM(c) => c.write_unencrypted_server(w),
            Self::SMSG_GMTICKET_DELETETICKET(c) => c.write_unencrypted_server(w),
            Self::SMSG_CHAT_WRONG_FACTION(c) => c.write_unencrypted_server(w),
            Self::SMSG_GMTICKET_SYSTEMSTATUS(c) => c.write_unencrypted_server(w),
            Self::SMSG_SET_REST_START(c) => c.write_unencrypted_server(w),
            Self::SMSG_GOSSIP_POI(c) => c.write_unencrypted_server(w),
            Self::SMSG_LOGIN_VERIFY_WORLD(c) => c.write_unencrypted_server(w),
            Self::SMSG_BATTLEFIELD_LIST(c) => c.write_unencrypted_server(w),
            Self::SMSG_ZONE_UNDER_ATTACK(c) => c.write_unencrypted_server(w),
            Self::MSG_AUCTION_HELLO(c) => c.write_unencrypted_server(w),
            Self::SMSG_AUCTION_LIST_RESULT(c) => c.write_unencrypted_server(w),
            Self::SMSG_AUCTION_OWNER_LIST_RESULT(c) => c.write_unencrypted_server(w),
            Self::SMSG_AUCTION_BIDDER_NOTIFICATION(c) => c.write_unencrypted_server(w),
            Self::SMSG_AUCTION_OWNER_NOTIFICATION(c) => c.write_unencrypted_server(w),
            Self::SMSG_PROCRESIST(c) => c.write_unencrypted_server(w),
            Self::SMSG_AUCTION_BIDDER_LIST_RESULT(c) => c.write_unencrypted_server(w),
            Self::SMSG_PLAY_OBJECT_SOUND(c) => c.write_unencrypted_server(w),
            Self::SMSG_RECEIVED_MAIL(c) => c.write_unencrypted_server(w),
            Self::SMSG_AUCTION_REMOVED_NOTIFICATION(c) => c.write_unencrypted_server(w),
            Self::SMSG_SERVER_MESSAGE(c) => c.write_unencrypted_server(w),
            Self::SMSG_STANDSTATE_UPDATE(c) => c.write_unencrypted_server(w),
            Self::SMSG_LOOT_ALL_PASSED(c) => c.write_unencrypted_server(w),
            Self::SMSG_LOOT_ROLL_WON(c) => c.write_unencrypted_server(w),
            Self::SMSG_LOOT_START_ROLL(c) => c.write_unencrypted_server(w),
            Self::SMSG_LOOT_ROLL(c) => c.write_unencrypted_server(w),
            Self::SMSG_LOOT_MASTER_LIST(c) => c.write_unencrypted_server(w),
            Self::SMSG_SET_FORCED_REACTIONS(c) => c.write_unencrypted_server(w),
            Self::SMSG_CHAT_PLAYER_NOT_FOUND(c) => c.write_unencrypted_server(w),
            Self::SMSG_DUEL_COUNTDOWN(c) => c.write_unencrypted_server(w),
            Self::SMSG_AREA_TRIGGER_MESSAGE(c) => c.write_unencrypted_server(w),
            Self::SMSG_DURABILITY_DAMAGE_DEATH(c) => c.write_unencrypted_server(w),
            Self::SMSG_INIT_WORLD_STATES(c) => c.write_unencrypted_server(w),
            Self::SMSG_UPDATE_WORLD_STATE(c) => c.write_unencrypted_server(w),
            Self::SMSG_CHAR_RENAME(c) => c.write_unencrypted_server(w),
            Self::MSG_BATTLEGROUND_PLAYER_POSITIONS(c) => c.write_unencrypted_server(w),
            Self::SMSG_BINDER_CONFIRM(c) => c.write_unencrypted_server(w),
            Self::SMSG_BATTLEGROUND_PLAYER_JOINED(c) => c.write_unencrypted_server(w),
            Self::SMSG_BATTLEGROUND_PLAYER_LEFT(c) => c.write_unencrypted_server(w),
            Self::SMSG_ADDON_INFO(c) => c.write_unencrypted_server(w),
            Self::SMSG_CHAT_RESTRICTED(c) => c.write_unencrypted_server(w),
            Self::SMSG_GM_TICKET_STATUS_UPDATE(c) => c.write_unencrypted_server(w),
            Self::SMSG_DEFENSE_MESSAGE(c) => c.write_unencrypted_server(w),
            Self::SMSG_REALM_SPLIT(c) => c.write_unencrypted_server(w),
            Self::SMSG_TIME_SYNC_REQ(c) => c.write_unencrypted_server(w),
            Self::SMSG_FEATURE_SYSTEM_STATUS(c) => c.write_unencrypted_server(w),
            Self::SMSG_UPDATE_ACCOUNT_DATA_COMPLETE(c) => c.write_unencrypted_server(w),
            Self::SMSG_CLIENTCACHE_VERSION(c) => c.write_unencrypted_server(w),
            Self::SMSG_WORLD_STATE_UI_TIMER_UPDATE(c) => c.write_unencrypted_server(w),
        }
    }

    #[cfg(all(feature = "tokio", feature = "encryption"))]
    pub async fn tokio_write_encrypted_server<W: tokio::io::AsyncWriteExt + Unpin + Send>(&self, w: &mut W, e: &mut ServerEncrypterHalf) -> Result<(), std::io::Error> {
        match self {
            Self::MSG_MOVE_START_FORWARD(c) => c.tokio_write_encrypted_server(w, e).await,
            Self::MSG_MOVE_START_BACKWARD(c) => c.tokio_write_encrypted_server(w, e).await,
            Self::MSG_MOVE_STOP(c) => c.tokio_write_encrypted_server(w, e).await,
            Self::MSG_MOVE_START_STRAFE_LEFT(c) => c.tokio_write_encrypted_server(w, e).await,
            Self::MSG_MOVE_START_STRAFE_RIGHT(c) => c.tokio_write_encrypted_server(w, e).await,
            Self::MSG_MOVE_STOP_STRAFE(c) => c.tokio_write_encrypted_server(w, e).await,
            Self::MSG_MOVE_JUMP(c) => c.tokio_write_encrypted_server(w, e).await,
            Self::MSG_MOVE_START_TURN_LEFT(c) => c.tokio_write_encrypted_server(w, e).await,
            Self::MSG_MOVE_START_TURN_RIGHT(c) => c.tokio_write_encrypted_server(w, e).await,
            Self::MSG_MOVE_STOP_TURN(c) => c.tokio_write_encrypted_server(w, e).await,
            Self::MSG_MOVE_START_PITCH_UP(c) => c.tokio_write_encrypted_server(w, e).await,
            Self::MSG_MOVE_START_PITCH_DOWN(c) => c.tokio_write_encrypted_server(w, e).await,
            Self::MSG_MOVE_STOP_PITCH(c) => c.tokio_write_encrypted_server(w, e).await,
            Self::MSG_MOVE_SET_RUN_MODE(c) => c.tokio_write_encrypted_server(w, e).await,
            Self::MSG_MOVE_SET_WALK_MODE(c) => c.tokio_write_encrypted_server(w, e).await,
            Self::MSG_MOVE_FALL_LAND(c) => c.tokio_write_encrypted_server(w, e).await,
            Self::MSG_MOVE_START_SWIM(c) => c.tokio_write_encrypted_server(w, e).await,
            Self::MSG_MOVE_STOP_SWIM(c) => c.tokio_write_encrypted_server(w, e).await,
            Self::MSG_MOVE_SET_FACING(c) => c.tokio_write_encrypted_server(w, e).await,
            Self::MSG_MOVE_SET_PITCH(c) => c.tokio_write_encrypted_server(w, e).await,
            Self::MSG_MOVE_WORLDPORT_ACK(c) => c.tokio_write_encrypted_server(w, e).await,
            Self::MSG_MOVE_HEARTBEAT(c) => c.tokio_write_encrypted_server(w, e).await,
            Self::MSG_PETITION_DECLINE(c) => c.tokio_write_encrypted_server(w, e).await,
            Self::MSG_TABARDVENDOR_ACTIVATE(c) => c.tokio_write_encrypted_server(w, e).await,
            Self::MSG_PETITION_RENAME(c) => c.tokio_write_encrypted_server(w, e).await,
            Self::MSG_SET_DUNGEON_DIFFICULTY(c) => c.tokio_write_encrypted_server(w, e).await,
            Self::MSG_MOVE_START_ASCEND(c) => c.tokio_write_encrypted_server(w, e).await,
            Self::MSG_MOVE_STOP_ASCEND(c) => c.tokio_write_encrypted_server(w, e).await,
            Self::MSG_MOVE_START_DESCEND(c) => c.tokio_write_encrypted_server(w, e).await,
            Self::SMSG_CHAR_CREATE(c) => c.tokio_write_encrypted_server(w, e).await,
            Self::SMSG_CHAR_ENUM(c) => c.tokio_write_encrypted_server(w, e).await,
            Self::SMSG_CHAR_DELETE(c) => c.tokio_write_encrypted_server(w, e).await,
            Self::SMSG_NEW_WORLD(c) => c.tokio_write_encrypted_server(w, e).await,
            Self::SMSG_TRANSFER_PENDING(c) => c.tokio_write_encrypted_server(w, e).await,
            Self::SMSG_CHARACTER_LOGIN_FAILED(c) => c.tokio_write_encrypted_server(w, e).await,
            Self::SMSG_LOGIN_SETTIMESPEED(c) => c.tokio_write_encrypted_server(w, e).await,
            Self::SMSG_LOGOUT_RESPONSE(c) => c.tokio_write_encrypted_server(w, e).await,
            Self::SMSG_LOGOUT_COMPLETE(c) => c.tokio_write_encrypted_server(w, e).await,
            Self::SMSG_LOGOUT_CANCEL_ACK(c) => c.tokio_write_encrypted_server(w, e).await,
            Self::SMSG_NAME_QUERY_RESPONSE(c) => c.tokio_write_encrypted_server(w, e).await,
            Self::SMSG_CONTACT_LIST(c) => c.tokio_write_encrypted_server(w, e).await,
            Self::SMSG_GUILD_INVITE(c) => c.tokio_write_encrypted_server(w, e).await,
            Self::SMSG_GUILD_INFO(c) => c.tokio_write_encrypted_server(w, e).await,
            Self::SMSG_GUILD_EVENT(c) => c.tokio_write_encrypted_server(w, e).await,
            Self::SMSG_GUILD_COMMAND_RESULT(c) => c.tokio_write_encrypted_server(w, e).await,
            Self::SMSG_CHANNEL_NOTIFY(c) => c.tokio_write_encrypted_server(w, e).await,
            Self::SMSG_CHANNEL_LIST(c) => c.tokio_write_encrypted_server(w, e).await,
            Self::SMSG_UPDATE_OBJECT(c) => c.tokio_write_encrypted_server(w, e).await,
            Self::SMSG_DESTROY_OBJECT(c) => c.tokio_write_encrypted_server(w, e).await,
            Self::SMSG_READ_ITEM_OK(c) => c.tokio_write_encrypted_server(w, e).await,
            Self::SMSG_READ_ITEM_FAILED(c) => c.tokio_write_encrypted_server(w, e).await,
            Self::SMSG_ITEM_COOLDOWN(c) => c.tokio_write_encrypted_server(w, e).await,
            Self::SMSG_GAMEOBJECT_CUSTOM_ANIM(c) => c.tokio_write_encrypted_server(w, e).await,
            Self::MSG_MOVE_TELEPORT_ACK(c) => c.tokio_write_encrypted_server(w, e).await,
            Self::SMSG_FORCE_MOVE_ROOT(c) => c.tokio_write_encrypted_server(w, e).await,
            Self::SMSG_FORCE_MOVE_UNROOT(c) => c.tokio_write_encrypted_server(w, e).await,
            Self::SMSG_TRIGGER_CINEMATIC(c) => c.tokio_write_encrypted_server(w, e).await,
            Self::SMSG_TUTORIAL_FLAGS(c) => c.tokio_write_encrypted_server(w, e).await,
            Self::SMSG_EMOTE(c) => c.tokio_write_encrypted_server(w, e).await,
            Self::SMSG_TEXT_EMOTE(c) => c.tokio_write_encrypted_server(w, e).await,
            Self::SMSG_INVENTORY_CHANGE_FAILURE(c) => c.tokio_write_encrypted_server(w, e).await,
            Self::SMSG_INITIALIZE_FACTIONS(c) => c.tokio_write_encrypted_server(w, e).await,
            Self::SMSG_SET_FACTION_VISIBLE(c) => c.tokio_write_encrypted_server(w, e).await,
            Self::SMSG_SET_FACTION_STANDING(c) => c.tokio_write_encrypted_server(w, e).await,
            Self::SMSG_SET_PROFICIENCY(c) => c.tokio_write_encrypted_server(w, e).await,
            Self::SMSG_ACTION_BUTTONS(c) => c.tokio_write_encrypted_server(w, e).await,
            Self::SMSG_AI_REACTION(c) => c.tokio_write_encrypted_server(w, e).await,
            Self::SMSG_ATTACKSTART(c) => c.tokio_write_encrypted_server(w, e).await,
            Self::SMSG_ATTACKSTOP(c) => c.tokio_write_encrypted_server(w, e).await,
            Self::SMSG_ATTACKSWING_NOTINRANGE(c) => c.tokio_write_encrypted_server(w, e).await,
            Self::SMSG_ATTACKSWING_BADFACING(c) => c.tokio_write_encrypted_server(w, e).await,
            Self::SMSG_ATTACKSWING_DEADTARGET(c) => c.tokio_write_encrypted_server(w, e).await,
            Self::SMSG_ATTACKSWING_CANT_ATTACK(c) => c.tokio_write_encrypted_server(w, e).await,
            Self::SMSG_ATTACKERSTATEUPDATE(c) => c.tokio_write_encrypted_server(w, e).await,
            Self::SMSG_CANCEL_COMBAT(c) => c.tokio_write_encrypted_server(w, e).await,
            Self::SMSG_BINDPOINTUPDATE(c) => c.tokio_write_encrypted_server(w, e).await,
            Self::SMSG_LOOT_RELEASE_RESPONSE(c) => c.tokio_write_encrypted_server(w, e).await,
            Self::SMSG_LOOT_REMOVED(c) => c.tokio_write_encrypted_server(w, e).await,
            Self::SMSG_LOOT_MONEY_NOTIFY(c) => c.tokio_write_encrypted_server(w, e).await,
            Self::SMSG_LOOT_CLEAR_MONEY(c) => c.tokio_write_encrypted_server(w, e).await,
            Self::SMSG_ITEM_PUSH_RESULT(c) => c.tokio_write_encrypted_server(w, e).await,
            Self::SMSG_DUEL_REQUESTED(c) => c.tokio_write_encrypted_server(w, e).await,
            Self::SMSG_DUEL_OUTOFBOUNDS(c) => c.tokio_write_encrypted_server(w, e).await,
            Self::SMSG_DUEL_INBOUNDS(c) => c.tokio_write_encrypted_server(w, e).await,
            Self::SMSG_DUEL_COMPLETE(c) => c.tokio_write_encrypted_server(w, e).await,
            Self::SMSG_DUEL_WINNER(c) => c.tokio_write_encrypted_server(w, e).await,
            Self::SMSG_MOUNTRESULT(c) => c.tokio_write_encrypted_server(w, e).await,
            Self::SMSG_MOUNTSPECIAL_ANIM(c) => c.tokio_write_encrypted_server(w, e).await,
            Self::SMSG_GOSSIP_MESSAGE(c) => c.tokio_write_encrypted_server(w, e).await,
            Self::SMSG_GOSSIP_COMPLETE(c) => c.tokio_write_encrypted_server(w, e).await,
            Self::SMSG_NPC_TEXT_UPDATE(c) => c.tokio_write_encrypted_server(w, e).await,
            Self::SMSG_LIST_INVENTORY(c) => c.tokio_write_encrypted_server(w, e).await,
            Self::SMSG_SELL_ITEM(c) => c.tokio_write_encrypted_server(w, e).await,
            Self::SMSG_BUY_ITEM(c) => c.tokio_write_encrypted_server(w, e).await,
            Self::SMSG_BUY_FAILED(c) => c.tokio_write_encrypted_server(w, e).await,
            Self::SMSG_SHOW_BANK(c) => c.tokio_write_encrypted_server(w, e).await,
            Self::SMSG_BUY_BANK_SLOT_RESULT(c) => c.tokio_write_encrypted_server(w, e).await,
            Self::SMSG_PETITION_SHOW_SIGNATURES(c) => c.tokio_write_encrypted_server(w, e).await,
            Self::SMSG_PETITION_SIGN_RESULTS(c) => c.tokio_write_encrypted_server(w, e).await,
            Self::SMSG_TURN_IN_PETITION_RESULTS(c) => c.tokio_write_encrypted_server(w, e).await,
            Self::SMSG_NOTIFICATION(c) => c.tokio_write_encrypted_server(w, e).await,
            Self::SMSG_PLAYED_TIME(c) => c.tokio_write_encrypted_server(w, e).await,
            Self::SMSG_QUERY_TIME_RESPONSE(c) => c.tokio_write_encrypted_server(w, e).await,
            Self::SMSG_LOG_XPGAIN(c) => c.tokio_write_encrypted_server(w, e).await,
            Self::SMSG_LEVELUP_INFO(c) => c.tokio_write_encrypted_server(w, e).await,
            Self::SMSG_PONG(c) => c.tokio_write_encrypted_server(w, e).await,
            Self::SMSG_GAMEOBJECT_PAGETEXT(c) => c.tokio_write_encrypted_server(w, e).await,
            Self::SMSG_ITEM_TIME_UPDATE(c) => c.tokio_write_encrypted_server(w, e).await,
            Self::SMSG_ITEM_ENCHANT_TIME_UPDATE(c) => c.tokio_write_encrypted_server(w, e).await,
            Self::SMSG_AUTH_CHALLENGE(c) => c.tokio_write_encrypted_server(w, e).await,
            Self::SMSG_AUTH_RESPONSE(c) => c.tokio_write_encrypted_server(w, e).await,
            Self::MSG_SAVE_GUILD_EMBLEM(c) => c.tokio_write_encrypted_server(w, e).await,
            Self::SMSG_EXPLORATION_EXPERIENCE(c) => c.tokio_write_encrypted_server(w, e).await,
            Self::MSG_RANDOM_ROLL(c) => c.tokio_write_encrypted_server(w, e).await,
            Self::SMSG_ENVIRONMENTALDAMAGELOG(c) => c.tokio_write_encrypted_server(w, e).await,
            Self::SMSG_GMTICKET_CREATE(c) => c.tokio_write_encrypted_server(w, e).await,
            Self::SMSG_GMTICKET_UPDATETEXT(c) => c.tokio_write_encrypted_server(w, e).await,
            Self::SMSG_ACCOUNT_DATA_TIMES(c) => c.tokio_write_encrypted_server(w, e).await,
            Self::SMSG_UPDATE_ACCOUNT_DATA(c) => c.tokio_write_encrypted_server(w, e).await,
            Self::SMSG_GAMEOBJECT_DESPAWN_ANIM(c) => c.tokio_write_encrypted_server(w, e).await,
            Self::SMSG_GMTICKET_DELETETICKET(c) => c.tokio_write_encrypted_server(w, e).await,
            Self::SMSG_CHAT_WRONG_FACTION(c) => c.tokio_write_encrypted_server(w, e).await,
            Self::SMSG_GMTICKET_SYSTEMSTATUS(c) => c.tokio_write_encrypted_server(w, e).await,
            Self::SMSG_SET_REST_START(c) => c.tokio_write_encrypted_server(w, e).await,
            Self::SMSG_GOSSIP_POI(c) => c.tokio_write_encrypted_server(w, e).await,
            Self::SMSG_LOGIN_VERIFY_WORLD(c) => c.tokio_write_encrypted_server(w, e).await,
            Self::SMSG_BATTLEFIELD_LIST(c) => c.tokio_write_encrypted_server(w, e).await,
            Self::SMSG_ZONE_UNDER_ATTACK(c) => c.tokio_write_encrypted_server(w, e).await,
            Self::MSG_AUCTION_HELLO(c) => c.tokio_write_encrypted_server(w, e).await,
            Self::SMSG_AUCTION_LIST_RESULT(c) => c.tokio_write_encrypted_server(w, e).await,
            Self::SMSG_AUCTION_OWNER_LIST_RESULT(c) => c.tokio_write_encrypted_server(w, e).await,
            Self::SMSG_AUCTION_BIDDER_NOTIFICATION(c) => c.tokio_write_encrypted_server(w, e).await,
            Self::SMSG_AUCTION_OWNER_NOTIFICATION(c) => c.tokio_write_encrypted_server(w, e).await,
            Self::SMSG_PROCRESIST(c) => c.tokio_write_encrypted_server(w, e).await,
            Self::SMSG_AUCTION_BIDDER_LIST_RESULT(c) => c.tokio_write_encrypted_server(w, e).await,
            Self::SMSG_PLAY_OBJECT_SOUND(c) => c.tokio_write_encrypted_server(w, e).await,
            Self::SMSG_RECEIVED_MAIL(c) => c.tokio_write_encrypted_server(w, e).await,
            Self::SMSG_AUCTION_REMOVED_NOTIFICATION(c) => c.tokio_write_encrypted_server(w, e).await,
            Self::SMSG_SERVER_MESSAGE(c) => c.tokio_write_encrypted_server(w, e).await,
            Self::SMSG_STANDSTATE_UPDATE(c) => c.tokio_write_encrypted_server(w, e).await,
            Self::SMSG_LOOT_ALL_PASSED(c) => c.tokio_write_encrypted_server(w, e).await,
            Self::SMSG_LOOT_ROLL_WON(c) => c.tokio_write_encrypted_server(w, e).await,
            Self::SMSG_LOOT_START_ROLL(c) => c.tokio_write_encrypted_server(w, e).await,
            Self::SMSG_LOOT_ROLL(c) => c.tokio_write_encrypted_server(w, e).await,
            Self::SMSG_LOOT_MASTER_LIST(c) => c.tokio_write_encrypted_server(w, e).await,
            Self::SMSG_SET_FORCED_REACTIONS(c) => c.tokio_write_encrypted_server(w, e).await,
            Self::SMSG_CHAT_PLAYER_NOT_FOUND(c) => c.tokio_write_encrypted_server(w, e).await,
            Self::SMSG_DUEL_COUNTDOWN(c) => c.tokio_write_encrypted_server(w, e).await,
            Self::SMSG_AREA_TRIGGER_MESSAGE(c) => c.tokio_write_encrypted_server(w, e).await,
            Self::SMSG_DURABILITY_DAMAGE_DEATH(c) => c.tokio_write_encrypted_server(w, e).await,
            Self::SMSG_INIT_WORLD_STATES(c) => c.tokio_write_encrypted_server(w, e).await,
            Self::SMSG_UPDATE_WORLD_STATE(c) => c.tokio_write_encrypted_server(w, e).await,
            Self::SMSG_CHAR_RENAME(c) => c.tokio_write_encrypted_server(w, e).await,
            Self::MSG_BATTLEGROUND_PLAYER_POSITIONS(c) => c.tokio_write_encrypted_server(w, e).await,
            Self::SMSG_BINDER_CONFIRM(c) => c.tokio_write_encrypted_server(w, e).await,
            Self::SMSG_BATTLEGROUND_PLAYER_JOINED(c) => c.tokio_write_encrypted_server(w, e).await,
            Self::SMSG_BATTLEGROUND_PLAYER_LEFT(c) => c.tokio_write_encrypted_server(w, e).await,
            Self::SMSG_ADDON_INFO(c) => c.tokio_write_encrypted_server(w, e).await,
            Self::SMSG_CHAT_RESTRICTED(c) => c.tokio_write_encrypted_server(w, e).await,
            Self::SMSG_GM_TICKET_STATUS_UPDATE(c) => c.tokio_write_encrypted_server(w, e).await,
            Self::SMSG_DEFENSE_MESSAGE(c) => c.tokio_write_encrypted_server(w, e).await,
            Self::SMSG_REALM_SPLIT(c) => c.tokio_write_encrypted_server(w, e).await,
            Self::SMSG_TIME_SYNC_REQ(c) => c.tokio_write_encrypted_server(w, e).await,
            Self::SMSG_FEATURE_SYSTEM_STATUS(c) => c.tokio_write_encrypted_server(w, e).await,
            Self::SMSG_UPDATE_ACCOUNT_DATA_COMPLETE(c) => c.tokio_write_encrypted_server(w, e).await,
            Self::SMSG_CLIENTCACHE_VERSION(c) => c.tokio_write_encrypted_server(w, e).await,
            Self::SMSG_WORLD_STATE_UI_TIMER_UPDATE(c) => c.tokio_write_encrypted_server(w, e).await,
        }
    }

    #[cfg(feature = "tokio")]
    pub async fn tokio_write_unencrypted_server<W: tokio::io::AsyncWriteExt + Unpin + Send>(&self, w: &mut W) -> Result<(), std::io::Error> {
        match self {
            Self::MSG_MOVE_START_FORWARD(c) => c.tokio_write_unencrypted_server(w).await,
            Self::MSG_MOVE_START_BACKWARD(c) => c.tokio_write_unencrypted_server(w).await,
            Self::MSG_MOVE_STOP(c) => c.tokio_write_unencrypted_server(w).await,
            Self::MSG_MOVE_START_STRAFE_LEFT(c) => c.tokio_write_unencrypted_server(w).await,
            Self::MSG_MOVE_START_STRAFE_RIGHT(c) => c.tokio_write_unencrypted_server(w).await,
            Self::MSG_MOVE_STOP_STRAFE(c) => c.tokio_write_unencrypted_server(w).await,
            Self::MSG_MOVE_JUMP(c) => c.tokio_write_unencrypted_server(w).await,
            Self::MSG_MOVE_START_TURN_LEFT(c) => c.tokio_write_unencrypted_server(w).await,
            Self::MSG_MOVE_START_TURN_RIGHT(c) => c.tokio_write_unencrypted_server(w).await,
            Self::MSG_MOVE_STOP_TURN(c) => c.tokio_write_unencrypted_server(w).await,
            Self::MSG_MOVE_START_PITCH_UP(c) => c.tokio_write_unencrypted_server(w).await,
            Self::MSG_MOVE_START_PITCH_DOWN(c) => c.tokio_write_unencrypted_server(w).await,
            Self::MSG_MOVE_STOP_PITCH(c) => c.tokio_write_unencrypted_server(w).await,
            Self::MSG_MOVE_SET_RUN_MODE(c) => c.tokio_write_unencrypted_server(w).await,
            Self::MSG_MOVE_SET_WALK_MODE(c) => c.tokio_write_unencrypted_server(w).await,
            Self::MSG_MOVE_FALL_LAND(c) => c.tokio_write_unencrypted_server(w).await,
            Self::MSG_MOVE_START_SWIM(c) => c.tokio_write_unencrypted_server(w).await,
            Self::MSG_MOVE_STOP_SWIM(c) => c.tokio_write_unencrypted_server(w).await,
            Self::MSG_MOVE_SET_FACING(c) => c.tokio_write_unencrypted_server(w).await,
            Self::MSG_MOVE_SET_PITCH(c) => c.tokio_write_unencrypted_server(w).await,
            Self::MSG_MOVE_WORLDPORT_ACK(c) => c.tokio_write_unencrypted_server(w).await,
            Self::MSG_MOVE_HEARTBEAT(c) => c.tokio_write_unencrypted_server(w).await,
            Self::MSG_PETITION_DECLINE(c) => c.tokio_write_unencrypted_server(w).await,
            Self::MSG_TABARDVENDOR_ACTIVATE(c) => c.tokio_write_unencrypted_server(w).await,
            Self::MSG_PETITION_RENAME(c) => c.tokio_write_unencrypted_server(w).await,
            Self::MSG_SET_DUNGEON_DIFFICULTY(c) => c.tokio_write_unencrypted_server(w).await,
            Self::MSG_MOVE_START_ASCEND(c) => c.tokio_write_unencrypted_server(w).await,
            Self::MSG_MOVE_STOP_ASCEND(c) => c.tokio_write_unencrypted_server(w).await,
            Self::MSG_MOVE_START_DESCEND(c) => c.tokio_write_unencrypted_server(w).await,
            Self::SMSG_CHAR_CREATE(c) => c.tokio_write_unencrypted_server(w).await,
            Self::SMSG_CHAR_ENUM(c) => c.tokio_write_unencrypted_server(w).await,
            Self::SMSG_CHAR_DELETE(c) => c.tokio_write_unencrypted_server(w).await,
            Self::SMSG_NEW_WORLD(c) => c.tokio_write_unencrypted_server(w).await,
            Self::SMSG_TRANSFER_PENDING(c) => c.tokio_write_unencrypted_server(w).await,
            Self::SMSG_CHARACTER_LOGIN_FAILED(c) => c.tokio_write_unencrypted_server(w).await,
            Self::SMSG_LOGIN_SETTIMESPEED(c) => c.tokio_write_unencrypted_server(w).await,
            Self::SMSG_LOGOUT_RESPONSE(c) => c.tokio_write_unencrypted_server(w).await,
            Self::SMSG_LOGOUT_COMPLETE(c) => c.tokio_write_unencrypted_server(w).await,
            Self::SMSG_LOGOUT_CANCEL_ACK(c) => c.tokio_write_unencrypted_server(w).await,
            Self::SMSG_NAME_QUERY_RESPONSE(c) => c.tokio_write_unencrypted_server(w).await,
            Self::SMSG_CONTACT_LIST(c) => c.tokio_write_unencrypted_server(w).await,
            Self::SMSG_GUILD_INVITE(c) => c.tokio_write_unencrypted_server(w).await,
            Self::SMSG_GUILD_INFO(c) => c.tokio_write_unencrypted_server(w).await,
            Self::SMSG_GUILD_EVENT(c) => c.tokio_write_unencrypted_server(w).await,
            Self::SMSG_GUILD_COMMAND_RESULT(c) => c.tokio_write_unencrypted_server(w).await,
            Self::SMSG_CHANNEL_NOTIFY(c) => c.tokio_write_unencrypted_server(w).await,
            Self::SMSG_CHANNEL_LIST(c) => c.tokio_write_unencrypted_server(w).await,
            Self::SMSG_UPDATE_OBJECT(c) => c.tokio_write_unencrypted_server(w).await,
            Self::SMSG_DESTROY_OBJECT(c) => c.tokio_write_unencrypted_server(w).await,
            Self::SMSG_READ_ITEM_OK(c) => c.tokio_write_unencrypted_server(w).await,
            Self::SMSG_READ_ITEM_FAILED(c) => c.tokio_write_unencrypted_server(w).await,
            Self::SMSG_ITEM_COOLDOWN(c) => c.tokio_write_unencrypted_server(w).await,
            Self::SMSG_GAMEOBJECT_CUSTOM_ANIM(c) => c.tokio_write_unencrypted_server(w).await,
            Self::MSG_MOVE_TELEPORT_ACK(c) => c.tokio_write_unencrypted_server(w).await,
            Self::SMSG_FORCE_MOVE_ROOT(c) => c.tokio_write_unencrypted_server(w).await,
            Self::SMSG_FORCE_MOVE_UNROOT(c) => c.tokio_write_unencrypted_server(w).await,
            Self::SMSG_TRIGGER_CINEMATIC(c) => c.tokio_write_unencrypted_server(w).await,
            Self::SMSG_TUTORIAL_FLAGS(c) => c.tokio_write_unencrypted_server(w).await,
            Self::SMSG_EMOTE(c) => c.tokio_write_unencrypted_server(w).await,
            Self::SMSG_TEXT_EMOTE(c) => c.tokio_write_unencrypted_server(w).await,
            Self::SMSG_INVENTORY_CHANGE_FAILURE(c) => c.tokio_write_unencrypted_server(w).await,
            Self::SMSG_INITIALIZE_FACTIONS(c) => c.tokio_write_unencrypted_server(w).await,
            Self::SMSG_SET_FACTION_VISIBLE(c) => c.tokio_write_unencrypted_server(w).await,
            Self::SMSG_SET_FACTION_STANDING(c) => c.tokio_write_unencrypted_server(w).await,
            Self::SMSG_SET_PROFICIENCY(c) => c.tokio_write_unencrypted_server(w).await,
            Self::SMSG_ACTION_BUTTONS(c) => c.tokio_write_unencrypted_server(w).await,
            Self::SMSG_AI_REACTION(c) => c.tokio_write_unencrypted_server(w).await,
            Self::SMSG_ATTACKSTART(c) => c.tokio_write_unencrypted_server(w).await,
            Self::SMSG_ATTACKSTOP(c) => c.tokio_write_unencrypted_server(w).await,
            Self::SMSG_ATTACKSWING_NOTINRANGE(c) => c.tokio_write_unencrypted_server(w).await,
            Self::SMSG_ATTACKSWING_BADFACING(c) => c.tokio_write_unencrypted_server(w).await,
            Self::SMSG_ATTACKSWING_DEADTARGET(c) => c.tokio_write_unencrypted_server(w).await,
            Self::SMSG_ATTACKSWING_CANT_ATTACK(c) => c.tokio_write_unencrypted_server(w).await,
            Self::SMSG_ATTACKERSTATEUPDATE(c) => c.tokio_write_unencrypted_server(w).await,
            Self::SMSG_CANCEL_COMBAT(c) => c.tokio_write_unencrypted_server(w).await,
            Self::SMSG_BINDPOINTUPDATE(c) => c.tokio_write_unencrypted_server(w).await,
            Self::SMSG_LOOT_RELEASE_RESPONSE(c) => c.tokio_write_unencrypted_server(w).await,
            Self::SMSG_LOOT_REMOVED(c) => c.tokio_write_unencrypted_server(w).await,
            Self::SMSG_LOOT_MONEY_NOTIFY(c) => c.tokio_write_unencrypted_server(w).await,
            Self::SMSG_LOOT_CLEAR_MONEY(c) => c.tokio_write_unencrypted_server(w).await,
            Self::SMSG_ITEM_PUSH_RESULT(c) => c.tokio_write_unencrypted_server(w).await,
            Self::SMSG_DUEL_REQUESTED(c) => c.tokio_write_unencrypted_server(w).await,
            Self::SMSG_DUEL_OUTOFBOUNDS(c) => c.tokio_write_unencrypted_server(w).await,
            Self::SMSG_DUEL_INBOUNDS(c) => c.tokio_write_unencrypted_server(w).await,
            Self::SMSG_DUEL_COMPLETE(c) => c.tokio_write_unencrypted_server(w).await,
            Self::SMSG_DUEL_WINNER(c) => c.tokio_write_unencrypted_server(w).await,
            Self::SMSG_MOUNTRESULT(c) => c.tokio_write_unencrypted_server(w).await,
            Self::SMSG_MOUNTSPECIAL_ANIM(c) => c.tokio_write_unencrypted_server(w).await,
            Self::SMSG_GOSSIP_MESSAGE(c) => c.tokio_write_unencrypted_server(w).await,
            Self::SMSG_GOSSIP_COMPLETE(c) => c.tokio_write_unencrypted_server(w).await,
            Self::SMSG_NPC_TEXT_UPDATE(c) => c.tokio_write_unencrypted_server(w).await,
            Self::SMSG_LIST_INVENTORY(c) => c.tokio_write_unencrypted_server(w).await,
            Self::SMSG_SELL_ITEM(c) => c.tokio_write_unencrypted_server(w).await,
            Self::SMSG_BUY_ITEM(c) => c.tokio_write_unencrypted_server(w).await,
            Self::SMSG_BUY_FAILED(c) => c.tokio_write_unencrypted_server(w).await,
            Self::SMSG_SHOW_BANK(c) => c.tokio_write_unencrypted_server(w).await,
            Self::SMSG_BUY_BANK_SLOT_RESULT(c) => c.tokio_write_unencrypted_server(w).await,
            Self::SMSG_PETITION_SHOW_SIGNATURES(c) => c.tokio_write_unencrypted_server(w).await,
            Self::SMSG_PETITION_SIGN_RESULTS(c) => c.tokio_write_unencrypted_server(w).await,
            Self::SMSG_TURN_IN_PETITION_RESULTS(c) => c.tokio_write_unencrypted_server(w).await,
            Self::SMSG_NOTIFICATION(c) => c.tokio_write_unencrypted_server(w).await,
            Self::SMSG_PLAYED_TIME(c) => c.tokio_write_unencrypted_server(w).await,
            Self::SMSG_QUERY_TIME_RESPONSE(c) => c.tokio_write_unencrypted_server(w).await,
            Self::SMSG_LOG_XPGAIN(c) => c.tokio_write_unencrypted_server(w).await,
            Self::SMSG_LEVELUP_INFO(c) => c.tokio_write_unencrypted_server(w).await,
            Self::SMSG_PONG(c) => c.tokio_write_unencrypted_server(w).await,
            Self::SMSG_GAMEOBJECT_PAGETEXT(c) => c.tokio_write_unencrypted_server(w).await,
            Self::SMSG_ITEM_TIME_UPDATE(c) => c.tokio_write_unencrypted_server(w).await,
            Self::SMSG_ITEM_ENCHANT_TIME_UPDATE(c) => c.tokio_write_unencrypted_server(w).await,
            Self::SMSG_AUTH_CHALLENGE(c) => c.tokio_write_unencrypted_server(w).await,
            Self::SMSG_AUTH_RESPONSE(c) => c.tokio_write_unencrypted_server(w).await,
            Self::MSG_SAVE_GUILD_EMBLEM(c) => c.tokio_write_unencrypted_server(w).await,
            Self::SMSG_EXPLORATION_EXPERIENCE(c) => c.tokio_write_unencrypted_server(w).await,
            Self::MSG_RANDOM_ROLL(c) => c.tokio_write_unencrypted_server(w).await,
            Self::SMSG_ENVIRONMENTALDAMAGELOG(c) => c.tokio_write_unencrypted_server(w).await,
            Self::SMSG_GMTICKET_CREATE(c) => c.tokio_write_unencrypted_server(w).await,
            Self::SMSG_GMTICKET_UPDATETEXT(c) => c.tokio_write_unencrypted_server(w).await,
            Self::SMSG_ACCOUNT_DATA_TIMES(c) => c.tokio_write_unencrypted_server(w).await,
            Self::SMSG_UPDATE_ACCOUNT_DATA(c) => c.tokio_write_unencrypted_server(w).await,
            Self::SMSG_GAMEOBJECT_DESPAWN_ANIM(c) => c.tokio_write_unencrypted_server(w).await,
            Self::SMSG_GMTICKET_DELETETICKET(c) => c.tokio_write_unencrypted_server(w).await,
            Self::SMSG_CHAT_WRONG_FACTION(c) => c.tokio_write_unencrypted_server(w).await,
            Self::SMSG_GMTICKET_SYSTEMSTATUS(c) => c.tokio_write_unencrypted_server(w).await,
            Self::SMSG_SET_REST_START(c) => c.tokio_write_unencrypted_server(w).await,
            Self::SMSG_GOSSIP_POI(c) => c.tokio_write_unencrypted_server(w).await,
            Self::SMSG_LOGIN_VERIFY_WORLD(c) => c.tokio_write_unencrypted_server(w).await,
            Self::SMSG_BATTLEFIELD_LIST(c) => c.tokio_write_unencrypted_server(w).await,
            Self::SMSG_ZONE_UNDER_ATTACK(c) => c.tokio_write_unencrypted_server(w).await,
            Self::MSG_AUCTION_HELLO(c) => c.tokio_write_unencrypted_server(w).await,
            Self::SMSG_AUCTION_LIST_RESULT(c) => c.tokio_write_unencrypted_server(w).await,
            Self::SMSG_AUCTION_OWNER_LIST_RESULT(c) => c.tokio_write_unencrypted_server(w).await,
            Self::SMSG_AUCTION_BIDDER_NOTIFICATION(c) => c.tokio_write_unencrypted_server(w).await,
            Self::SMSG_AUCTION_OWNER_NOTIFICATION(c) => c.tokio_write_unencrypted_server(w).await,
            Self::SMSG_PROCRESIST(c) => c.tokio_write_unencrypted_server(w).await,
            Self::SMSG_AUCTION_BIDDER_LIST_RESULT(c) => c.tokio_write_unencrypted_server(w).await,
            Self::SMSG_PLAY_OBJECT_SOUND(c) => c.tokio_write_unencrypted_server(w).await,
            Self::SMSG_RECEIVED_MAIL(c) => c.tokio_write_unencrypted_server(w).await,
            Self::SMSG_AUCTION_REMOVED_NOTIFICATION(c) => c.tokio_write_unencrypted_server(w).await,
            Self::SMSG_SERVER_MESSAGE(c) => c.tokio_write_unencrypted_server(w).await,
            Self::SMSG_STANDSTATE_UPDATE(c) => c.tokio_write_unencrypted_server(w).await,
            Self::SMSG_LOOT_ALL_PASSED(c) => c.tokio_write_unencrypted_server(w).await,
            Self::SMSG_LOOT_ROLL_WON(c) => c.tokio_write_unencrypted_server(w).await,
            Self::SMSG_LOOT_START_ROLL(c) => c.tokio_write_unencrypted_server(w).await,
            Self::SMSG_LOOT_ROLL(c) => c.tokio_write_unencrypted_server(w).await,
            Self::SMSG_LOOT_MASTER_LIST(c) => c.tokio_write_unencrypted_server(w).await,
            Self::SMSG_SET_FORCED_REACTIONS(c) => c.tokio_write_unencrypted_server(w).await,
            Self::SMSG_CHAT_PLAYER_NOT_FOUND(c) => c.tokio_write_unencrypted_server(w).await,
            Self::SMSG_DUEL_COUNTDOWN(c) => c.tokio_write_unencrypted_server(w).await,
            Self::SMSG_AREA_TRIGGER_MESSAGE(c) => c.tokio_write_unencrypted_server(w).await,
            Self::SMSG_DURABILITY_DAMAGE_DEATH(c) => c.tokio_write_unencrypted_server(w).await,
            Self::SMSG_INIT_WORLD_STATES(c) => c.tokio_write_unencrypted_server(w).await,
            Self::SMSG_UPDATE_WORLD_STATE(c) => c.tokio_write_unencrypted_server(w).await,
            Self::SMSG_CHAR_RENAME(c) => c.tokio_write_unencrypted_server(w).await,
            Self::MSG_BATTLEGROUND_PLAYER_POSITIONS(c) => c.tokio_write_unencrypted_server(w).await,
            Self::SMSG_BINDER_CONFIRM(c) => c.tokio_write_unencrypted_server(w).await,
            Self::SMSG_BATTLEGROUND_PLAYER_JOINED(c) => c.tokio_write_unencrypted_server(w).await,
            Self::SMSG_BATTLEGROUND_PLAYER_LEFT(c) => c.tokio_write_unencrypted_server(w).await,
            Self::SMSG_ADDON_INFO(c) => c.tokio_write_unencrypted_server(w).await,
            Self::SMSG_CHAT_RESTRICTED(c) => c.tokio_write_unencrypted_server(w).await,
            Self::SMSG_GM_TICKET_STATUS_UPDATE(c) => c.tokio_write_unencrypted_server(w).await,
            Self::SMSG_DEFENSE_MESSAGE(c) => c.tokio_write_unencrypted_server(w).await,
            Self::SMSG_REALM_SPLIT(c) => c.tokio_write_unencrypted_server(w).await,
            Self::SMSG_TIME_SYNC_REQ(c) => c.tokio_write_unencrypted_server(w).await,
            Self::SMSG_FEATURE_SYSTEM_STATUS(c) => c.tokio_write_unencrypted_server(w).await,
            Self::SMSG_UPDATE_ACCOUNT_DATA_COMPLETE(c) => c.tokio_write_unencrypted_server(w).await,
            Self::SMSG_CLIENTCACHE_VERSION(c) => c.tokio_write_unencrypted_server(w).await,
            Self::SMSG_WORLD_STATE_UI_TIMER_UPDATE(c) => c.tokio_write_unencrypted_server(w).await,
        }
    }

    #[cfg(all(feature = "async-std", feature = "encryption"))]
    pub async fn astd_write_encrypted_server<W: async_std::io::WriteExt + Unpin + Send>(&self, w: &mut W, e: &mut ServerEncrypterHalf) -> Result<(), std::io::Error> {
        match self {
            Self::MSG_MOVE_START_FORWARD(c) => c.astd_write_encrypted_server(w, e).await,
            Self::MSG_MOVE_START_BACKWARD(c) => c.astd_write_encrypted_server(w, e).await,
            Self::MSG_MOVE_STOP(c) => c.astd_write_encrypted_server(w, e).await,
            Self::MSG_MOVE_START_STRAFE_LEFT(c) => c.astd_write_encrypted_server(w, e).await,
            Self::MSG_MOVE_START_STRAFE_RIGHT(c) => c.astd_write_encrypted_server(w, e).await,
            Self::MSG_MOVE_STOP_STRAFE(c) => c.astd_write_encrypted_server(w, e).await,
            Self::MSG_MOVE_JUMP(c) => c.astd_write_encrypted_server(w, e).await,
            Self::MSG_MOVE_START_TURN_LEFT(c) => c.astd_write_encrypted_server(w, e).await,
            Self::MSG_MOVE_START_TURN_RIGHT(c) => c.astd_write_encrypted_server(w, e).await,
            Self::MSG_MOVE_STOP_TURN(c) => c.astd_write_encrypted_server(w, e).await,
            Self::MSG_MOVE_START_PITCH_UP(c) => c.astd_write_encrypted_server(w, e).await,
            Self::MSG_MOVE_START_PITCH_DOWN(c) => c.astd_write_encrypted_server(w, e).await,
            Self::MSG_MOVE_STOP_PITCH(c) => c.astd_write_encrypted_server(w, e).await,
            Self::MSG_MOVE_SET_RUN_MODE(c) => c.astd_write_encrypted_server(w, e).await,
            Self::MSG_MOVE_SET_WALK_MODE(c) => c.astd_write_encrypted_server(w, e).await,
            Self::MSG_MOVE_FALL_LAND(c) => c.astd_write_encrypted_server(w, e).await,
            Self::MSG_MOVE_START_SWIM(c) => c.astd_write_encrypted_server(w, e).await,
            Self::MSG_MOVE_STOP_SWIM(c) => c.astd_write_encrypted_server(w, e).await,
            Self::MSG_MOVE_SET_FACING(c) => c.astd_write_encrypted_server(w, e).await,
            Self::MSG_MOVE_SET_PITCH(c) => c.astd_write_encrypted_server(w, e).await,
            Self::MSG_MOVE_WORLDPORT_ACK(c) => c.astd_write_encrypted_server(w, e).await,
            Self::MSG_MOVE_HEARTBEAT(c) => c.astd_write_encrypted_server(w, e).await,
            Self::MSG_PETITION_DECLINE(c) => c.astd_write_encrypted_server(w, e).await,
            Self::MSG_TABARDVENDOR_ACTIVATE(c) => c.astd_write_encrypted_server(w, e).await,
            Self::MSG_PETITION_RENAME(c) => c.astd_write_encrypted_server(w, e).await,
            Self::MSG_SET_DUNGEON_DIFFICULTY(c) => c.astd_write_encrypted_server(w, e).await,
            Self::MSG_MOVE_START_ASCEND(c) => c.astd_write_encrypted_server(w, e).await,
            Self::MSG_MOVE_STOP_ASCEND(c) => c.astd_write_encrypted_server(w, e).await,
            Self::MSG_MOVE_START_DESCEND(c) => c.astd_write_encrypted_server(w, e).await,
            Self::SMSG_CHAR_CREATE(c) => c.astd_write_encrypted_server(w, e).await,
            Self::SMSG_CHAR_ENUM(c) => c.astd_write_encrypted_server(w, e).await,
            Self::SMSG_CHAR_DELETE(c) => c.astd_write_encrypted_server(w, e).await,
            Self::SMSG_NEW_WORLD(c) => c.astd_write_encrypted_server(w, e).await,
            Self::SMSG_TRANSFER_PENDING(c) => c.astd_write_encrypted_server(w, e).await,
            Self::SMSG_CHARACTER_LOGIN_FAILED(c) => c.astd_write_encrypted_server(w, e).await,
            Self::SMSG_LOGIN_SETTIMESPEED(c) => c.astd_write_encrypted_server(w, e).await,
            Self::SMSG_LOGOUT_RESPONSE(c) => c.astd_write_encrypted_server(w, e).await,
            Self::SMSG_LOGOUT_COMPLETE(c) => c.astd_write_encrypted_server(w, e).await,
            Self::SMSG_LOGOUT_CANCEL_ACK(c) => c.astd_write_encrypted_server(w, e).await,
            Self::SMSG_NAME_QUERY_RESPONSE(c) => c.astd_write_encrypted_server(w, e).await,
            Self::SMSG_CONTACT_LIST(c) => c.astd_write_encrypted_server(w, e).await,
            Self::SMSG_GUILD_INVITE(c) => c.astd_write_encrypted_server(w, e).await,
            Self::SMSG_GUILD_INFO(c) => c.astd_write_encrypted_server(w, e).await,
            Self::SMSG_GUILD_EVENT(c) => c.astd_write_encrypted_server(w, e).await,
            Self::SMSG_GUILD_COMMAND_RESULT(c) => c.astd_write_encrypted_server(w, e).await,
            Self::SMSG_CHANNEL_NOTIFY(c) => c.astd_write_encrypted_server(w, e).await,
            Self::SMSG_CHANNEL_LIST(c) => c.astd_write_encrypted_server(w, e).await,
            Self::SMSG_UPDATE_OBJECT(c) => c.astd_write_encrypted_server(w, e).await,
            Self::SMSG_DESTROY_OBJECT(c) => c.astd_write_encrypted_server(w, e).await,
            Self::SMSG_READ_ITEM_OK(c) => c.astd_write_encrypted_server(w, e).await,
            Self::SMSG_READ_ITEM_FAILED(c) => c.astd_write_encrypted_server(w, e).await,
            Self::SMSG_ITEM_COOLDOWN(c) => c.astd_write_encrypted_server(w, e).await,
            Self::SMSG_GAMEOBJECT_CUSTOM_ANIM(c) => c.astd_write_encrypted_server(w, e).await,
            Self::MSG_MOVE_TELEPORT_ACK(c) => c.astd_write_encrypted_server(w, e).await,
            Self::SMSG_FORCE_MOVE_ROOT(c) => c.astd_write_encrypted_server(w, e).await,
            Self::SMSG_FORCE_MOVE_UNROOT(c) => c.astd_write_encrypted_server(w, e).await,
            Self::SMSG_TRIGGER_CINEMATIC(c) => c.astd_write_encrypted_server(w, e).await,
            Self::SMSG_TUTORIAL_FLAGS(c) => c.astd_write_encrypted_server(w, e).await,
            Self::SMSG_EMOTE(c) => c.astd_write_encrypted_server(w, e).await,
            Self::SMSG_TEXT_EMOTE(c) => c.astd_write_encrypted_server(w, e).await,
            Self::SMSG_INVENTORY_CHANGE_FAILURE(c) => c.astd_write_encrypted_server(w, e).await,
            Self::SMSG_INITIALIZE_FACTIONS(c) => c.astd_write_encrypted_server(w, e).await,
            Self::SMSG_SET_FACTION_VISIBLE(c) => c.astd_write_encrypted_server(w, e).await,
            Self::SMSG_SET_FACTION_STANDING(c) => c.astd_write_encrypted_server(w, e).await,
            Self::SMSG_SET_PROFICIENCY(c) => c.astd_write_encrypted_server(w, e).await,
            Self::SMSG_ACTION_BUTTONS(c) => c.astd_write_encrypted_server(w, e).await,
            Self::SMSG_AI_REACTION(c) => c.astd_write_encrypted_server(w, e).await,
            Self::SMSG_ATTACKSTART(c) => c.astd_write_encrypted_server(w, e).await,
            Self::SMSG_ATTACKSTOP(c) => c.astd_write_encrypted_server(w, e).await,
            Self::SMSG_ATTACKSWING_NOTINRANGE(c) => c.astd_write_encrypted_server(w, e).await,
            Self::SMSG_ATTACKSWING_BADFACING(c) => c.astd_write_encrypted_server(w, e).await,
            Self::SMSG_ATTACKSWING_DEADTARGET(c) => c.astd_write_encrypted_server(w, e).await,
            Self::SMSG_ATTACKSWING_CANT_ATTACK(c) => c.astd_write_encrypted_server(w, e).await,
            Self::SMSG_ATTACKERSTATEUPDATE(c) => c.astd_write_encrypted_server(w, e).await,
            Self::SMSG_CANCEL_COMBAT(c) => c.astd_write_encrypted_server(w, e).await,
            Self::SMSG_BINDPOINTUPDATE(c) => c.astd_write_encrypted_server(w, e).await,
            Self::SMSG_LOOT_RELEASE_RESPONSE(c) => c.astd_write_encrypted_server(w, e).await,
            Self::SMSG_LOOT_REMOVED(c) => c.astd_write_encrypted_server(w, e).await,
            Self::SMSG_LOOT_MONEY_NOTIFY(c) => c.astd_write_encrypted_server(w, e).await,
            Self::SMSG_LOOT_CLEAR_MONEY(c) => c.astd_write_encrypted_server(w, e).await,
            Self::SMSG_ITEM_PUSH_RESULT(c) => c.astd_write_encrypted_server(w, e).await,
            Self::SMSG_DUEL_REQUESTED(c) => c.astd_write_encrypted_server(w, e).await,
            Self::SMSG_DUEL_OUTOFBOUNDS(c) => c.astd_write_encrypted_server(w, e).await,
            Self::SMSG_DUEL_INBOUNDS(c) => c.astd_write_encrypted_server(w, e).await,
            Self::SMSG_DUEL_COMPLETE(c) => c.astd_write_encrypted_server(w, e).await,
            Self::SMSG_DUEL_WINNER(c) => c.astd_write_encrypted_server(w, e).await,
            Self::SMSG_MOUNTRESULT(c) => c.astd_write_encrypted_server(w, e).await,
            Self::SMSG_MOUNTSPECIAL_ANIM(c) => c.astd_write_encrypted_server(w, e).await,
            Self::SMSG_GOSSIP_MESSAGE(c) => c.astd_write_encrypted_server(w, e).await,
            Self::SMSG_GOSSIP_COMPLETE(c) => c.astd_write_encrypted_server(w, e).await,
            Self::SMSG_NPC_TEXT_UPDATE(c) => c.astd_write_encrypted_server(w, e).await,
            Self::SMSG_LIST_INVENTORY(c) => c.astd_write_encrypted_server(w, e).await,
            Self::SMSG_SELL_ITEM(c) => c.astd_write_encrypted_server(w, e).await,
            Self::SMSG_BUY_ITEM(c) => c.astd_write_encrypted_server(w, e).await,
            Self::SMSG_BUY_FAILED(c) => c.astd_write_encrypted_server(w, e).await,
            Self::SMSG_SHOW_BANK(c) => c.astd_write_encrypted_server(w, e).await,
            Self::SMSG_BUY_BANK_SLOT_RESULT(c) => c.astd_write_encrypted_server(w, e).await,
            Self::SMSG_PETITION_SHOW_SIGNATURES(c) => c.astd_write_encrypted_server(w, e).await,
            Self::SMSG_PETITION_SIGN_RESULTS(c) => c.astd_write_encrypted_server(w, e).await,
            Self::SMSG_TURN_IN_PETITION_RESULTS(c) => c.astd_write_encrypted_server(w, e).await,
            Self::SMSG_NOTIFICATION(c) => c.astd_write_encrypted_server(w, e).await,
            Self::SMSG_PLAYED_TIME(c) => c.astd_write_encrypted_server(w, e).await,
            Self::SMSG_QUERY_TIME_RESPONSE(c) => c.astd_write_encrypted_server(w, e).await,
            Self::SMSG_LOG_XPGAIN(c) => c.astd_write_encrypted_server(w, e).await,
            Self::SMSG_LEVELUP_INFO(c) => c.astd_write_encrypted_server(w, e).await,
            Self::SMSG_PONG(c) => c.astd_write_encrypted_server(w, e).await,
            Self::SMSG_GAMEOBJECT_PAGETEXT(c) => c.astd_write_encrypted_server(w, e).await,
            Self::SMSG_ITEM_TIME_UPDATE(c) => c.astd_write_encrypted_server(w, e).await,
            Self::SMSG_ITEM_ENCHANT_TIME_UPDATE(c) => c.astd_write_encrypted_server(w, e).await,
            Self::SMSG_AUTH_CHALLENGE(c) => c.astd_write_encrypted_server(w, e).await,
            Self::SMSG_AUTH_RESPONSE(c) => c.astd_write_encrypted_server(w, e).await,
            Self::MSG_SAVE_GUILD_EMBLEM(c) => c.astd_write_encrypted_server(w, e).await,
            Self::SMSG_EXPLORATION_EXPERIENCE(c) => c.astd_write_encrypted_server(w, e).await,
            Self::MSG_RANDOM_ROLL(c) => c.astd_write_encrypted_server(w, e).await,
            Self::SMSG_ENVIRONMENTALDAMAGELOG(c) => c.astd_write_encrypted_server(w, e).await,
            Self::SMSG_GMTICKET_CREATE(c) => c.astd_write_encrypted_server(w, e).await,
            Self::SMSG_GMTICKET_UPDATETEXT(c) => c.astd_write_encrypted_server(w, e).await,
            Self::SMSG_ACCOUNT_DATA_TIMES(c) => c.astd_write_encrypted_server(w, e).await,
            Self::SMSG_UPDATE_ACCOUNT_DATA(c) => c.astd_write_encrypted_server(w, e).await,
            Self::SMSG_GAMEOBJECT_DESPAWN_ANIM(c) => c.astd_write_encrypted_server(w, e).await,
            Self::SMSG_GMTICKET_DELETETICKET(c) => c.astd_write_encrypted_server(w, e).await,
            Self::SMSG_CHAT_WRONG_FACTION(c) => c.astd_write_encrypted_server(w, e).await,
            Self::SMSG_GMTICKET_SYSTEMSTATUS(c) => c.astd_write_encrypted_server(w, e).await,
            Self::SMSG_SET_REST_START(c) => c.astd_write_encrypted_server(w, e).await,
            Self::SMSG_GOSSIP_POI(c) => c.astd_write_encrypted_server(w, e).await,
            Self::SMSG_LOGIN_VERIFY_WORLD(c) => c.astd_write_encrypted_server(w, e).await,
            Self::SMSG_BATTLEFIELD_LIST(c) => c.astd_write_encrypted_server(w, e).await,
            Self::SMSG_ZONE_UNDER_ATTACK(c) => c.astd_write_encrypted_server(w, e).await,
            Self::MSG_AUCTION_HELLO(c) => c.astd_write_encrypted_server(w, e).await,
            Self::SMSG_AUCTION_LIST_RESULT(c) => c.astd_write_encrypted_server(w, e).await,
            Self::SMSG_AUCTION_OWNER_LIST_RESULT(c) => c.astd_write_encrypted_server(w, e).await,
            Self::SMSG_AUCTION_BIDDER_NOTIFICATION(c) => c.astd_write_encrypted_server(w, e).await,
            Self::SMSG_AUCTION_OWNER_NOTIFICATION(c) => c.astd_write_encrypted_server(w, e).await,
            Self::SMSG_PROCRESIST(c) => c.astd_write_encrypted_server(w, e).await,
            Self::SMSG_AUCTION_BIDDER_LIST_RESULT(c) => c.astd_write_encrypted_server(w, e).await,
            Self::SMSG_PLAY_OBJECT_SOUND(c) => c.astd_write_encrypted_server(w, e).await,
            Self::SMSG_RECEIVED_MAIL(c) => c.astd_write_encrypted_server(w, e).await,
            Self::SMSG_AUCTION_REMOVED_NOTIFICATION(c) => c.astd_write_encrypted_server(w, e).await,
            Self::SMSG_SERVER_MESSAGE(c) => c.astd_write_encrypted_server(w, e).await,
            Self::SMSG_STANDSTATE_UPDATE(c) => c.astd_write_encrypted_server(w, e).await,
            Self::SMSG_LOOT_ALL_PASSED(c) => c.astd_write_encrypted_server(w, e).await,
            Self::SMSG_LOOT_ROLL_WON(c) => c.astd_write_encrypted_server(w, e).await,
            Self::SMSG_LOOT_START_ROLL(c) => c.astd_write_encrypted_server(w, e).await,
            Self::SMSG_LOOT_ROLL(c) => c.astd_write_encrypted_server(w, e).await,
            Self::SMSG_LOOT_MASTER_LIST(c) => c.astd_write_encrypted_server(w, e).await,
            Self::SMSG_SET_FORCED_REACTIONS(c) => c.astd_write_encrypted_server(w, e).await,
            Self::SMSG_CHAT_PLAYER_NOT_FOUND(c) => c.astd_write_encrypted_server(w, e).await,
            Self::SMSG_DUEL_COUNTDOWN(c) => c.astd_write_encrypted_server(w, e).await,
            Self::SMSG_AREA_TRIGGER_MESSAGE(c) => c.astd_write_encrypted_server(w, e).await,
            Self::SMSG_DURABILITY_DAMAGE_DEATH(c) => c.astd_write_encrypted_server(w, e).await,
            Self::SMSG_INIT_WORLD_STATES(c) => c.astd_write_encrypted_server(w, e).await,
            Self::SMSG_UPDATE_WORLD_STATE(c) => c.astd_write_encrypted_server(w, e).await,
            Self::SMSG_CHAR_RENAME(c) => c.astd_write_encrypted_server(w, e).await,
            Self::MSG_BATTLEGROUND_PLAYER_POSITIONS(c) => c.astd_write_encrypted_server(w, e).await,
            Self::SMSG_BINDER_CONFIRM(c) => c.astd_write_encrypted_server(w, e).await,
            Self::SMSG_BATTLEGROUND_PLAYER_JOINED(c) => c.astd_write_encrypted_server(w, e).await,
            Self::SMSG_BATTLEGROUND_PLAYER_LEFT(c) => c.astd_write_encrypted_server(w, e).await,
            Self::SMSG_ADDON_INFO(c) => c.astd_write_encrypted_server(w, e).await,
            Self::SMSG_CHAT_RESTRICTED(c) => c.astd_write_encrypted_server(w, e).await,
            Self::SMSG_GM_TICKET_STATUS_UPDATE(c) => c.astd_write_encrypted_server(w, e).await,
            Self::SMSG_DEFENSE_MESSAGE(c) => c.astd_write_encrypted_server(w, e).await,
            Self::SMSG_REALM_SPLIT(c) => c.astd_write_encrypted_server(w, e).await,
            Self::SMSG_TIME_SYNC_REQ(c) => c.astd_write_encrypted_server(w, e).await,
            Self::SMSG_FEATURE_SYSTEM_STATUS(c) => c.astd_write_encrypted_server(w, e).await,
            Self::SMSG_UPDATE_ACCOUNT_DATA_COMPLETE(c) => c.astd_write_encrypted_server(w, e).await,
            Self::SMSG_CLIENTCACHE_VERSION(c) => c.astd_write_encrypted_server(w, e).await,
            Self::SMSG_WORLD_STATE_UI_TIMER_UPDATE(c) => c.astd_write_encrypted_server(w, e).await,
        }
    }

    #[cfg(feature = "async-std")]
    pub async fn astd_write_unencrypted_server<W: async_std::io::WriteExt + Unpin + Send>(&self, w: &mut W) -> Result<(), std::io::Error> {
        match self {
            Self::MSG_MOVE_START_FORWARD(c) => c.astd_write_unencrypted_server(w).await,
            Self::MSG_MOVE_START_BACKWARD(c) => c.astd_write_unencrypted_server(w).await,
            Self::MSG_MOVE_STOP(c) => c.astd_write_unencrypted_server(w).await,
            Self::MSG_MOVE_START_STRAFE_LEFT(c) => c.astd_write_unencrypted_server(w).await,
            Self::MSG_MOVE_START_STRAFE_RIGHT(c) => c.astd_write_unencrypted_server(w).await,
            Self::MSG_MOVE_STOP_STRAFE(c) => c.astd_write_unencrypted_server(w).await,
            Self::MSG_MOVE_JUMP(c) => c.astd_write_unencrypted_server(w).await,
            Self::MSG_MOVE_START_TURN_LEFT(c) => c.astd_write_unencrypted_server(w).await,
            Self::MSG_MOVE_START_TURN_RIGHT(c) => c.astd_write_unencrypted_server(w).await,
            Self::MSG_MOVE_STOP_TURN(c) => c.astd_write_unencrypted_server(w).await,
            Self::MSG_MOVE_START_PITCH_UP(c) => c.astd_write_unencrypted_server(w).await,
            Self::MSG_MOVE_START_PITCH_DOWN(c) => c.astd_write_unencrypted_server(w).await,
            Self::MSG_MOVE_STOP_PITCH(c) => c.astd_write_unencrypted_server(w).await,
            Self::MSG_MOVE_SET_RUN_MODE(c) => c.astd_write_unencrypted_server(w).await,
            Self::MSG_MOVE_SET_WALK_MODE(c) => c.astd_write_unencrypted_server(w).await,
            Self::MSG_MOVE_FALL_LAND(c) => c.astd_write_unencrypted_server(w).await,
            Self::MSG_MOVE_START_SWIM(c) => c.astd_write_unencrypted_server(w).await,
            Self::MSG_MOVE_STOP_SWIM(c) => c.astd_write_unencrypted_server(w).await,
            Self::MSG_MOVE_SET_FACING(c) => c.astd_write_unencrypted_server(w).await,
            Self::MSG_MOVE_SET_PITCH(c) => c.astd_write_unencrypted_server(w).await,
            Self::MSG_MOVE_WORLDPORT_ACK(c) => c.astd_write_unencrypted_server(w).await,
            Self::MSG_MOVE_HEARTBEAT(c) => c.astd_write_unencrypted_server(w).await,
            Self::MSG_PETITION_DECLINE(c) => c.astd_write_unencrypted_server(w).await,
            Self::MSG_TABARDVENDOR_ACTIVATE(c) => c.astd_write_unencrypted_server(w).await,
            Self::MSG_PETITION_RENAME(c) => c.astd_write_unencrypted_server(w).await,
            Self::MSG_SET_DUNGEON_DIFFICULTY(c) => c.astd_write_unencrypted_server(w).await,
            Self::MSG_MOVE_START_ASCEND(c) => c.astd_write_unencrypted_server(w).await,
            Self::MSG_MOVE_STOP_ASCEND(c) => c.astd_write_unencrypted_server(w).await,
            Self::MSG_MOVE_START_DESCEND(c) => c.astd_write_unencrypted_server(w).await,
            Self::SMSG_CHAR_CREATE(c) => c.astd_write_unencrypted_server(w).await,
            Self::SMSG_CHAR_ENUM(c) => c.astd_write_unencrypted_server(w).await,
            Self::SMSG_CHAR_DELETE(c) => c.astd_write_unencrypted_server(w).await,
            Self::SMSG_NEW_WORLD(c) => c.astd_write_unencrypted_server(w).await,
            Self::SMSG_TRANSFER_PENDING(c) => c.astd_write_unencrypted_server(w).await,
            Self::SMSG_CHARACTER_LOGIN_FAILED(c) => c.astd_write_unencrypted_server(w).await,
            Self::SMSG_LOGIN_SETTIMESPEED(c) => c.astd_write_unencrypted_server(w).await,
            Self::SMSG_LOGOUT_RESPONSE(c) => c.astd_write_unencrypted_server(w).await,
            Self::SMSG_LOGOUT_COMPLETE(c) => c.astd_write_unencrypted_server(w).await,
            Self::SMSG_LOGOUT_CANCEL_ACK(c) => c.astd_write_unencrypted_server(w).await,
            Self::SMSG_NAME_QUERY_RESPONSE(c) => c.astd_write_unencrypted_server(w).await,
            Self::SMSG_CONTACT_LIST(c) => c.astd_write_unencrypted_server(w).await,
            Self::SMSG_GUILD_INVITE(c) => c.astd_write_unencrypted_server(w).await,
            Self::SMSG_GUILD_INFO(c) => c.astd_write_unencrypted_server(w).await,
            Self::SMSG_GUILD_EVENT(c) => c.astd_write_unencrypted_server(w).await,
            Self::SMSG_GUILD_COMMAND_RESULT(c) => c.astd_write_unencrypted_server(w).await,
            Self::SMSG_CHANNEL_NOTIFY(c) => c.astd_write_unencrypted_server(w).await,
            Self::SMSG_CHANNEL_LIST(c) => c.astd_write_unencrypted_server(w).await,
            Self::SMSG_UPDATE_OBJECT(c) => c.astd_write_unencrypted_server(w).await,
            Self::SMSG_DESTROY_OBJECT(c) => c.astd_write_unencrypted_server(w).await,
            Self::SMSG_READ_ITEM_OK(c) => c.astd_write_unencrypted_server(w).await,
            Self::SMSG_READ_ITEM_FAILED(c) => c.astd_write_unencrypted_server(w).await,
            Self::SMSG_ITEM_COOLDOWN(c) => c.astd_write_unencrypted_server(w).await,
            Self::SMSG_GAMEOBJECT_CUSTOM_ANIM(c) => c.astd_write_unencrypted_server(w).await,
            Self::MSG_MOVE_TELEPORT_ACK(c) => c.astd_write_unencrypted_server(w).await,
            Self::SMSG_FORCE_MOVE_ROOT(c) => c.astd_write_unencrypted_server(w).await,
            Self::SMSG_FORCE_MOVE_UNROOT(c) => c.astd_write_unencrypted_server(w).await,
            Self::SMSG_TRIGGER_CINEMATIC(c) => c.astd_write_unencrypted_server(w).await,
            Self::SMSG_TUTORIAL_FLAGS(c) => c.astd_write_unencrypted_server(w).await,
            Self::SMSG_EMOTE(c) => c.astd_write_unencrypted_server(w).await,
            Self::SMSG_TEXT_EMOTE(c) => c.astd_write_unencrypted_server(w).await,
            Self::SMSG_INVENTORY_CHANGE_FAILURE(c) => c.astd_write_unencrypted_server(w).await,
            Self::SMSG_INITIALIZE_FACTIONS(c) => c.astd_write_unencrypted_server(w).await,
            Self::SMSG_SET_FACTION_VISIBLE(c) => c.astd_write_unencrypted_server(w).await,
            Self::SMSG_SET_FACTION_STANDING(c) => c.astd_write_unencrypted_server(w).await,
            Self::SMSG_SET_PROFICIENCY(c) => c.astd_write_unencrypted_server(w).await,
            Self::SMSG_ACTION_BUTTONS(c) => c.astd_write_unencrypted_server(w).await,
            Self::SMSG_AI_REACTION(c) => c.astd_write_unencrypted_server(w).await,
            Self::SMSG_ATTACKSTART(c) => c.astd_write_unencrypted_server(w).await,
            Self::SMSG_ATTACKSTOP(c) => c.astd_write_unencrypted_server(w).await,
            Self::SMSG_ATTACKSWING_NOTINRANGE(c) => c.astd_write_unencrypted_server(w).await,
            Self::SMSG_ATTACKSWING_BADFACING(c) => c.astd_write_unencrypted_server(w).await,
            Self::SMSG_ATTACKSWING_DEADTARGET(c) => c.astd_write_unencrypted_server(w).await,
            Self::SMSG_ATTACKSWING_CANT_ATTACK(c) => c.astd_write_unencrypted_server(w).await,
            Self::SMSG_ATTACKERSTATEUPDATE(c) => c.astd_write_unencrypted_server(w).await,
            Self::SMSG_CANCEL_COMBAT(c) => c.astd_write_unencrypted_server(w).await,
            Self::SMSG_BINDPOINTUPDATE(c) => c.astd_write_unencrypted_server(w).await,
            Self::SMSG_LOOT_RELEASE_RESPONSE(c) => c.astd_write_unencrypted_server(w).await,
            Self::SMSG_LOOT_REMOVED(c) => c.astd_write_unencrypted_server(w).await,
            Self::SMSG_LOOT_MONEY_NOTIFY(c) => c.astd_write_unencrypted_server(w).await,
            Self::SMSG_LOOT_CLEAR_MONEY(c) => c.astd_write_unencrypted_server(w).await,
            Self::SMSG_ITEM_PUSH_RESULT(c) => c.astd_write_unencrypted_server(w).await,
            Self::SMSG_DUEL_REQUESTED(c) => c.astd_write_unencrypted_server(w).await,
            Self::SMSG_DUEL_OUTOFBOUNDS(c) => c.astd_write_unencrypted_server(w).await,
            Self::SMSG_DUEL_INBOUNDS(c) => c.astd_write_unencrypted_server(w).await,
            Self::SMSG_DUEL_COMPLETE(c) => c.astd_write_unencrypted_server(w).await,
            Self::SMSG_DUEL_WINNER(c) => c.astd_write_unencrypted_server(w).await,
            Self::SMSG_MOUNTRESULT(c) => c.astd_write_unencrypted_server(w).await,
            Self::SMSG_MOUNTSPECIAL_ANIM(c) => c.astd_write_unencrypted_server(w).await,
            Self::SMSG_GOSSIP_MESSAGE(c) => c.astd_write_unencrypted_server(w).await,
            Self::SMSG_GOSSIP_COMPLETE(c) => c.astd_write_unencrypted_server(w).await,
            Self::SMSG_NPC_TEXT_UPDATE(c) => c.astd_write_unencrypted_server(w).await,
            Self::SMSG_LIST_INVENTORY(c) => c.astd_write_unencrypted_server(w).await,
            Self::SMSG_SELL_ITEM(c) => c.astd_write_unencrypted_server(w).await,
            Self::SMSG_BUY_ITEM(c) => c.astd_write_unencrypted_server(w).await,
            Self::SMSG_BUY_FAILED(c) => c.astd_write_unencrypted_server(w).await,
            Self::SMSG_SHOW_BANK(c) => c.astd_write_unencrypted_server(w).await,
            Self::SMSG_BUY_BANK_SLOT_RESULT(c) => c.astd_write_unencrypted_server(w).await,
            Self::SMSG_PETITION_SHOW_SIGNATURES(c) => c.astd_write_unencrypted_server(w).await,
            Self::SMSG_PETITION_SIGN_RESULTS(c) => c.astd_write_unencrypted_server(w).await,
            Self::SMSG_TURN_IN_PETITION_RESULTS(c) => c.astd_write_unencrypted_server(w).await,
            Self::SMSG_NOTIFICATION(c) => c.astd_write_unencrypted_server(w).await,
            Self::SMSG_PLAYED_TIME(c) => c.astd_write_unencrypted_server(w).await,
            Self::SMSG_QUERY_TIME_RESPONSE(c) => c.astd_write_unencrypted_server(w).await,
            Self::SMSG_LOG_XPGAIN(c) => c.astd_write_unencrypted_server(w).await,
            Self::SMSG_LEVELUP_INFO(c) => c.astd_write_unencrypted_server(w).await,
            Self::SMSG_PONG(c) => c.astd_write_unencrypted_server(w).await,
            Self::SMSG_GAMEOBJECT_PAGETEXT(c) => c.astd_write_unencrypted_server(w).await,
            Self::SMSG_ITEM_TIME_UPDATE(c) => c.astd_write_unencrypted_server(w).await,
            Self::SMSG_ITEM_ENCHANT_TIME_UPDATE(c) => c.astd_write_unencrypted_server(w).await,
            Self::SMSG_AUTH_CHALLENGE(c) => c.astd_write_unencrypted_server(w).await,
            Self::SMSG_AUTH_RESPONSE(c) => c.astd_write_unencrypted_server(w).await,
            Self::MSG_SAVE_GUILD_EMBLEM(c) => c.astd_write_unencrypted_server(w).await,
            Self::SMSG_EXPLORATION_EXPERIENCE(c) => c.astd_write_unencrypted_server(w).await,
            Self::MSG_RANDOM_ROLL(c) => c.astd_write_unencrypted_server(w).await,
            Self::SMSG_ENVIRONMENTALDAMAGELOG(c) => c.astd_write_unencrypted_server(w).await,
            Self::SMSG_GMTICKET_CREATE(c) => c.astd_write_unencrypted_server(w).await,
            Self::SMSG_GMTICKET_UPDATETEXT(c) => c.astd_write_unencrypted_server(w).await,
            Self::SMSG_ACCOUNT_DATA_TIMES(c) => c.astd_write_unencrypted_server(w).await,
            Self::SMSG_UPDATE_ACCOUNT_DATA(c) => c.astd_write_unencrypted_server(w).await,
            Self::SMSG_GAMEOBJECT_DESPAWN_ANIM(c) => c.astd_write_unencrypted_server(w).await,
            Self::SMSG_GMTICKET_DELETETICKET(c) => c.astd_write_unencrypted_server(w).await,
            Self::SMSG_CHAT_WRONG_FACTION(c) => c.astd_write_unencrypted_server(w).await,
            Self::SMSG_GMTICKET_SYSTEMSTATUS(c) => c.astd_write_unencrypted_server(w).await,
            Self::SMSG_SET_REST_START(c) => c.astd_write_unencrypted_server(w).await,
            Self::SMSG_GOSSIP_POI(c) => c.astd_write_unencrypted_server(w).await,
            Self::SMSG_LOGIN_VERIFY_WORLD(c) => c.astd_write_unencrypted_server(w).await,
            Self::SMSG_BATTLEFIELD_LIST(c) => c.astd_write_unencrypted_server(w).await,
            Self::SMSG_ZONE_UNDER_ATTACK(c) => c.astd_write_unencrypted_server(w).await,
            Self::MSG_AUCTION_HELLO(c) => c.astd_write_unencrypted_server(w).await,
            Self::SMSG_AUCTION_LIST_RESULT(c) => c.astd_write_unencrypted_server(w).await,
            Self::SMSG_AUCTION_OWNER_LIST_RESULT(c) => c.astd_write_unencrypted_server(w).await,
            Self::SMSG_AUCTION_BIDDER_NOTIFICATION(c) => c.astd_write_unencrypted_server(w).await,
            Self::SMSG_AUCTION_OWNER_NOTIFICATION(c) => c.astd_write_unencrypted_server(w).await,
            Self::SMSG_PROCRESIST(c) => c.astd_write_unencrypted_server(w).await,
            Self::SMSG_AUCTION_BIDDER_LIST_RESULT(c) => c.astd_write_unencrypted_server(w).await,
            Self::SMSG_PLAY_OBJECT_SOUND(c) => c.astd_write_unencrypted_server(w).await,
            Self::SMSG_RECEIVED_MAIL(c) => c.astd_write_unencrypted_server(w).await,
            Self::SMSG_AUCTION_REMOVED_NOTIFICATION(c) => c.astd_write_unencrypted_server(w).await,
            Self::SMSG_SERVER_MESSAGE(c) => c.astd_write_unencrypted_server(w).await,
            Self::SMSG_STANDSTATE_UPDATE(c) => c.astd_write_unencrypted_server(w).await,
            Self::SMSG_LOOT_ALL_PASSED(c) => c.astd_write_unencrypted_server(w).await,
            Self::SMSG_LOOT_ROLL_WON(c) => c.astd_write_unencrypted_server(w).await,
            Self::SMSG_LOOT_START_ROLL(c) => c.astd_write_unencrypted_server(w).await,
            Self::SMSG_LOOT_ROLL(c) => c.astd_write_unencrypted_server(w).await,
            Self::SMSG_LOOT_MASTER_LIST(c) => c.astd_write_unencrypted_server(w).await,
            Self::SMSG_SET_FORCED_REACTIONS(c) => c.astd_write_unencrypted_server(w).await,
            Self::SMSG_CHAT_PLAYER_NOT_FOUND(c) => c.astd_write_unencrypted_server(w).await,
            Self::SMSG_DUEL_COUNTDOWN(c) => c.astd_write_unencrypted_server(w).await,
            Self::SMSG_AREA_TRIGGER_MESSAGE(c) => c.astd_write_unencrypted_server(w).await,
            Self::SMSG_DURABILITY_DAMAGE_DEATH(c) => c.astd_write_unencrypted_server(w).await,
            Self::SMSG_INIT_WORLD_STATES(c) => c.astd_write_unencrypted_server(w).await,
            Self::SMSG_UPDATE_WORLD_STATE(c) => c.astd_write_unencrypted_server(w).await,
            Self::SMSG_CHAR_RENAME(c) => c.astd_write_unencrypted_server(w).await,
            Self::MSG_BATTLEGROUND_PLAYER_POSITIONS(c) => c.astd_write_unencrypted_server(w).await,
            Self::SMSG_BINDER_CONFIRM(c) => c.astd_write_unencrypted_server(w).await,
            Self::SMSG_BATTLEGROUND_PLAYER_JOINED(c) => c.astd_write_unencrypted_server(w).await,
            Self::SMSG_BATTLEGROUND_PLAYER_LEFT(c) => c.astd_write_unencrypted_server(w).await,
            Self::SMSG_ADDON_INFO(c) => c.astd_write_unencrypted_server(w).await,
            Self::SMSG_CHAT_RESTRICTED(c) => c.astd_write_unencrypted_server(w).await,
            Self::SMSG_GM_TICKET_STATUS_UPDATE(c) => c.astd_write_unencrypted_server(w).await,
            Self::SMSG_DEFENSE_MESSAGE(c) => c.astd_write_unencrypted_server(w).await,
            Self::SMSG_REALM_SPLIT(c) => c.astd_write_unencrypted_server(w).await,
            Self::SMSG_TIME_SYNC_REQ(c) => c.astd_write_unencrypted_server(w).await,
            Self::SMSG_FEATURE_SYSTEM_STATUS(c) => c.astd_write_unencrypted_server(w).await,
            Self::SMSG_UPDATE_ACCOUNT_DATA_COMPLETE(c) => c.astd_write_unencrypted_server(w).await,
            Self::SMSG_CLIENTCACHE_VERSION(c) => c.astd_write_unencrypted_server(w).await,
            Self::SMSG_WORLD_STATE_UI_TIMER_UPDATE(c) => c.astd_write_unencrypted_server(w).await,
        }
    }

}

impl std::fmt::Display for ServerOpcodeMessage {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            ServerOpcodeMessage::MSG_MOVE_START_FORWARD(_) => "MSG_MOVE_START_FORWARD",
            ServerOpcodeMessage::MSG_MOVE_START_BACKWARD(_) => "MSG_MOVE_START_BACKWARD",
            ServerOpcodeMessage::MSG_MOVE_STOP(_) => "MSG_MOVE_STOP",
            ServerOpcodeMessage::MSG_MOVE_START_STRAFE_LEFT(_) => "MSG_MOVE_START_STRAFE_LEFT",
            ServerOpcodeMessage::MSG_MOVE_START_STRAFE_RIGHT(_) => "MSG_MOVE_START_STRAFE_RIGHT",
            ServerOpcodeMessage::MSG_MOVE_STOP_STRAFE(_) => "MSG_MOVE_STOP_STRAFE",
            ServerOpcodeMessage::MSG_MOVE_JUMP(_) => "MSG_MOVE_JUMP",
            ServerOpcodeMessage::MSG_MOVE_START_TURN_LEFT(_) => "MSG_MOVE_START_TURN_LEFT",
            ServerOpcodeMessage::MSG_MOVE_START_TURN_RIGHT(_) => "MSG_MOVE_START_TURN_RIGHT",
            ServerOpcodeMessage::MSG_MOVE_STOP_TURN(_) => "MSG_MOVE_STOP_TURN",
            ServerOpcodeMessage::MSG_MOVE_START_PITCH_UP(_) => "MSG_MOVE_START_PITCH_UP",
            ServerOpcodeMessage::MSG_MOVE_START_PITCH_DOWN(_) => "MSG_MOVE_START_PITCH_DOWN",
            ServerOpcodeMessage::MSG_MOVE_STOP_PITCH(_) => "MSG_MOVE_STOP_PITCH",
            ServerOpcodeMessage::MSG_MOVE_SET_RUN_MODE(_) => "MSG_MOVE_SET_RUN_MODE",
            ServerOpcodeMessage::MSG_MOVE_SET_WALK_MODE(_) => "MSG_MOVE_SET_WALK_MODE",
            ServerOpcodeMessage::MSG_MOVE_FALL_LAND(_) => "MSG_MOVE_FALL_LAND",
            ServerOpcodeMessage::MSG_MOVE_START_SWIM(_) => "MSG_MOVE_START_SWIM",
            ServerOpcodeMessage::MSG_MOVE_STOP_SWIM(_) => "MSG_MOVE_STOP_SWIM",
            ServerOpcodeMessage::MSG_MOVE_SET_FACING(_) => "MSG_MOVE_SET_FACING",
            ServerOpcodeMessage::MSG_MOVE_SET_PITCH(_) => "MSG_MOVE_SET_PITCH",
            ServerOpcodeMessage::MSG_MOVE_WORLDPORT_ACK(_) => "MSG_MOVE_WORLDPORT_ACK",
            ServerOpcodeMessage::MSG_MOVE_HEARTBEAT(_) => "MSG_MOVE_HEARTBEAT",
            ServerOpcodeMessage::MSG_PETITION_DECLINE(_) => "MSG_PETITION_DECLINE",
            ServerOpcodeMessage::MSG_TABARDVENDOR_ACTIVATE(_) => "MSG_TABARDVENDOR_ACTIVATE",
            ServerOpcodeMessage::MSG_PETITION_RENAME(_) => "MSG_PETITION_RENAME",
            ServerOpcodeMessage::MSG_SET_DUNGEON_DIFFICULTY(_) => "MSG_SET_DUNGEON_DIFFICULTY",
            ServerOpcodeMessage::MSG_MOVE_START_ASCEND(_) => "MSG_MOVE_START_ASCEND",
            ServerOpcodeMessage::MSG_MOVE_STOP_ASCEND(_) => "MSG_MOVE_STOP_ASCEND",
            ServerOpcodeMessage::MSG_MOVE_START_DESCEND(_) => "MSG_MOVE_START_DESCEND",
            ServerOpcodeMessage::SMSG_CHAR_CREATE(_) => "SMSG_CHAR_CREATE",
            ServerOpcodeMessage::SMSG_CHAR_ENUM(_) => "SMSG_CHAR_ENUM",
            ServerOpcodeMessage::SMSG_CHAR_DELETE(_) => "SMSG_CHAR_DELETE",
            ServerOpcodeMessage::SMSG_NEW_WORLD(_) => "SMSG_NEW_WORLD",
            ServerOpcodeMessage::SMSG_TRANSFER_PENDING(_) => "SMSG_TRANSFER_PENDING",
            ServerOpcodeMessage::SMSG_CHARACTER_LOGIN_FAILED(_) => "SMSG_CHARACTER_LOGIN_FAILED",
            ServerOpcodeMessage::SMSG_LOGIN_SETTIMESPEED(_) => "SMSG_LOGIN_SETTIMESPEED",
            ServerOpcodeMessage::SMSG_LOGOUT_RESPONSE(_) => "SMSG_LOGOUT_RESPONSE",
            ServerOpcodeMessage::SMSG_LOGOUT_COMPLETE(_) => "SMSG_LOGOUT_COMPLETE",
            ServerOpcodeMessage::SMSG_LOGOUT_CANCEL_ACK(_) => "SMSG_LOGOUT_CANCEL_ACK",
            ServerOpcodeMessage::SMSG_NAME_QUERY_RESPONSE(_) => "SMSG_NAME_QUERY_RESPONSE",
            ServerOpcodeMessage::SMSG_CONTACT_LIST(_) => "SMSG_CONTACT_LIST",
            ServerOpcodeMessage::SMSG_GUILD_INVITE(_) => "SMSG_GUILD_INVITE",
            ServerOpcodeMessage::SMSG_GUILD_INFO(_) => "SMSG_GUILD_INFO",
            ServerOpcodeMessage::SMSG_GUILD_EVENT(_) => "SMSG_GUILD_EVENT",
            ServerOpcodeMessage::SMSG_GUILD_COMMAND_RESULT(_) => "SMSG_GUILD_COMMAND_RESULT",
            ServerOpcodeMessage::SMSG_CHANNEL_NOTIFY(_) => "SMSG_CHANNEL_NOTIFY",
            ServerOpcodeMessage::SMSG_CHANNEL_LIST(_) => "SMSG_CHANNEL_LIST",
            ServerOpcodeMessage::SMSG_UPDATE_OBJECT(_) => "SMSG_UPDATE_OBJECT",
            ServerOpcodeMessage::SMSG_DESTROY_OBJECT(_) => "SMSG_DESTROY_OBJECT",
            ServerOpcodeMessage::SMSG_READ_ITEM_OK(_) => "SMSG_READ_ITEM_OK",
            ServerOpcodeMessage::SMSG_READ_ITEM_FAILED(_) => "SMSG_READ_ITEM_FAILED",
            ServerOpcodeMessage::SMSG_ITEM_COOLDOWN(_) => "SMSG_ITEM_COOLDOWN",
            ServerOpcodeMessage::SMSG_GAMEOBJECT_CUSTOM_ANIM(_) => "SMSG_GAMEOBJECT_CUSTOM_ANIM",
            ServerOpcodeMessage::MSG_MOVE_TELEPORT_ACK(_) => "MSG_MOVE_TELEPORT_ACK_Server",
            ServerOpcodeMessage::SMSG_FORCE_MOVE_ROOT(_) => "SMSG_FORCE_MOVE_ROOT",
            ServerOpcodeMessage::SMSG_FORCE_MOVE_UNROOT(_) => "SMSG_FORCE_MOVE_UNROOT",
            ServerOpcodeMessage::SMSG_TRIGGER_CINEMATIC(_) => "SMSG_TRIGGER_CINEMATIC",
            ServerOpcodeMessage::SMSG_TUTORIAL_FLAGS(_) => "SMSG_TUTORIAL_FLAGS",
            ServerOpcodeMessage::SMSG_EMOTE(_) => "SMSG_EMOTE",
            ServerOpcodeMessage::SMSG_TEXT_EMOTE(_) => "SMSG_TEXT_EMOTE",
            ServerOpcodeMessage::SMSG_INVENTORY_CHANGE_FAILURE(_) => "SMSG_INVENTORY_CHANGE_FAILURE",
            ServerOpcodeMessage::SMSG_INITIALIZE_FACTIONS(_) => "SMSG_INITIALIZE_FACTIONS",
            ServerOpcodeMessage::SMSG_SET_FACTION_VISIBLE(_) => "SMSG_SET_FACTION_VISIBLE",
            ServerOpcodeMessage::SMSG_SET_FACTION_STANDING(_) => "SMSG_SET_FACTION_STANDING",
            ServerOpcodeMessage::SMSG_SET_PROFICIENCY(_) => "SMSG_SET_PROFICIENCY",
            ServerOpcodeMessage::SMSG_ACTION_BUTTONS(_) => "SMSG_ACTION_BUTTONS",
            ServerOpcodeMessage::SMSG_AI_REACTION(_) => "SMSG_AI_REACTION",
            ServerOpcodeMessage::SMSG_ATTACKSTART(_) => "SMSG_ATTACKSTART",
            ServerOpcodeMessage::SMSG_ATTACKSTOP(_) => "SMSG_ATTACKSTOP",
            ServerOpcodeMessage::SMSG_ATTACKSWING_NOTINRANGE(_) => "SMSG_ATTACKSWING_NOTINRANGE",
            ServerOpcodeMessage::SMSG_ATTACKSWING_BADFACING(_) => "SMSG_ATTACKSWING_BADFACING",
            ServerOpcodeMessage::SMSG_ATTACKSWING_DEADTARGET(_) => "SMSG_ATTACKSWING_DEADTARGET",
            ServerOpcodeMessage::SMSG_ATTACKSWING_CANT_ATTACK(_) => "SMSG_ATTACKSWING_CANT_ATTACK",
            ServerOpcodeMessage::SMSG_ATTACKERSTATEUPDATE(_) => "SMSG_ATTACKERSTATEUPDATE",
            ServerOpcodeMessage::SMSG_CANCEL_COMBAT(_) => "SMSG_CANCEL_COMBAT",
            ServerOpcodeMessage::SMSG_BINDPOINTUPDATE(_) => "SMSG_BINDPOINTUPDATE",
            ServerOpcodeMessage::SMSG_LOOT_RELEASE_RESPONSE(_) => "SMSG_LOOT_RELEASE_RESPONSE",
            ServerOpcodeMessage::SMSG_LOOT_REMOVED(_) => "SMSG_LOOT_REMOVED",
            ServerOpcodeMessage::SMSG_LOOT_MONEY_NOTIFY(_) => "SMSG_LOOT_MONEY_NOTIFY",
            ServerOpcodeMessage::SMSG_LOOT_CLEAR_MONEY(_) => "SMSG_LOOT_CLEAR_MONEY",
            ServerOpcodeMessage::SMSG_ITEM_PUSH_RESULT(_) => "SMSG_ITEM_PUSH_RESULT",
            ServerOpcodeMessage::SMSG_DUEL_REQUESTED(_) => "SMSG_DUEL_REQUESTED",
            ServerOpcodeMessage::SMSG_DUEL_OUTOFBOUNDS(_) => "SMSG_DUEL_OUTOFBOUNDS",
            ServerOpcodeMessage::SMSG_DUEL_INBOUNDS(_) => "SMSG_DUEL_INBOUNDS",
            ServerOpcodeMessage::SMSG_DUEL_COMPLETE(_) => "SMSG_DUEL_COMPLETE",
            ServerOpcodeMessage::SMSG_DUEL_WINNER(_) => "SMSG_DUEL_WINNER",
            ServerOpcodeMessage::SMSG_MOUNTRESULT(_) => "SMSG_MOUNTRESULT",
            ServerOpcodeMessage::SMSG_MOUNTSPECIAL_ANIM(_) => "SMSG_MOUNTSPECIAL_ANIM",
            ServerOpcodeMessage::SMSG_GOSSIP_MESSAGE(_) => "SMSG_GOSSIP_MESSAGE",
            ServerOpcodeMessage::SMSG_GOSSIP_COMPLETE(_) => "SMSG_GOSSIP_COMPLETE",
            ServerOpcodeMessage::SMSG_NPC_TEXT_UPDATE(_) => "SMSG_NPC_TEXT_UPDATE",
            ServerOpcodeMessage::SMSG_LIST_INVENTORY(_) => "SMSG_LIST_INVENTORY",
            ServerOpcodeMessage::SMSG_SELL_ITEM(_) => "SMSG_SELL_ITEM",
            ServerOpcodeMessage::SMSG_BUY_ITEM(_) => "SMSG_BUY_ITEM",
            ServerOpcodeMessage::SMSG_BUY_FAILED(_) => "SMSG_BUY_FAILED",
            ServerOpcodeMessage::SMSG_SHOW_BANK(_) => "SMSG_SHOW_BANK",
            ServerOpcodeMessage::SMSG_BUY_BANK_SLOT_RESULT(_) => "SMSG_BUY_BANK_SLOT_RESULT",
            ServerOpcodeMessage::SMSG_PETITION_SHOW_SIGNATURES(_) => "SMSG_PETITION_SHOW_SIGNATURES",
            ServerOpcodeMessage::SMSG_PETITION_SIGN_RESULTS(_) => "SMSG_PETITION_SIGN_RESULTS",
            ServerOpcodeMessage::SMSG_TURN_IN_PETITION_RESULTS(_) => "SMSG_TURN_IN_PETITION_RESULTS",
            ServerOpcodeMessage::SMSG_NOTIFICATION(_) => "SMSG_NOTIFICATION",
            ServerOpcodeMessage::SMSG_PLAYED_TIME(_) => "SMSG_PLAYED_TIME",
            ServerOpcodeMessage::SMSG_QUERY_TIME_RESPONSE(_) => "SMSG_QUERY_TIME_RESPONSE",
            ServerOpcodeMessage::SMSG_LOG_XPGAIN(_) => "SMSG_LOG_XPGAIN",
            ServerOpcodeMessage::SMSG_LEVELUP_INFO(_) => "SMSG_LEVELUP_INFO",
            ServerOpcodeMessage::SMSG_PONG(_) => "SMSG_PONG",
            ServerOpcodeMessage::SMSG_GAMEOBJECT_PAGETEXT(_) => "SMSG_GAMEOBJECT_PAGETEXT",
            ServerOpcodeMessage::SMSG_ITEM_TIME_UPDATE(_) => "SMSG_ITEM_TIME_UPDATE",
            ServerOpcodeMessage::SMSG_ITEM_ENCHANT_TIME_UPDATE(_) => "SMSG_ITEM_ENCHANT_TIME_UPDATE",
            ServerOpcodeMessage::SMSG_AUTH_CHALLENGE(_) => "SMSG_AUTH_CHALLENGE",
            ServerOpcodeMessage::SMSG_AUTH_RESPONSE(_) => "SMSG_AUTH_RESPONSE",
            ServerOpcodeMessage::MSG_SAVE_GUILD_EMBLEM(_) => "MSG_SAVE_GUILD_EMBLEM_Server",
            ServerOpcodeMessage::SMSG_EXPLORATION_EXPERIENCE(_) => "SMSG_EXPLORATION_EXPERIENCE",
            ServerOpcodeMessage::MSG_RANDOM_ROLL(_) => "MSG_RANDOM_ROLL_Server",
            ServerOpcodeMessage::SMSG_ENVIRONMENTALDAMAGELOG(_) => "SMSG_ENVIRONMENTALDAMAGELOG",
            ServerOpcodeMessage::SMSG_GMTICKET_CREATE(_) => "SMSG_GMTICKET_CREATE",
            ServerOpcodeMessage::SMSG_GMTICKET_UPDATETEXT(_) => "SMSG_GMTICKET_UPDATETEXT",
            ServerOpcodeMessage::SMSG_ACCOUNT_DATA_TIMES(_) => "SMSG_ACCOUNT_DATA_TIMES",
            ServerOpcodeMessage::SMSG_UPDATE_ACCOUNT_DATA(_) => "SMSG_UPDATE_ACCOUNT_DATA",
            ServerOpcodeMessage::SMSG_GAMEOBJECT_DESPAWN_ANIM(_) => "SMSG_GAMEOBJECT_DESPAWN_ANIM",
            ServerOpcodeMessage::SMSG_GMTICKET_DELETETICKET(_) => "SMSG_GMTICKET_DELETETICKET",
            ServerOpcodeMessage::SMSG_CHAT_WRONG_FACTION(_) => "SMSG_CHAT_WRONG_FACTION",
            ServerOpcodeMessage::SMSG_GMTICKET_SYSTEMSTATUS(_) => "SMSG_GMTICKET_SYSTEMSTATUS",
            ServerOpcodeMessage::SMSG_SET_REST_START(_) => "SMSG_SET_REST_START",
            ServerOpcodeMessage::SMSG_GOSSIP_POI(_) => "SMSG_GOSSIP_POI",
            ServerOpcodeMessage::SMSG_LOGIN_VERIFY_WORLD(_) => "SMSG_LOGIN_VERIFY_WORLD",
            ServerOpcodeMessage::SMSG_BATTLEFIELD_LIST(_) => "SMSG_BATTLEFIELD_LIST",
            ServerOpcodeMessage::SMSG_ZONE_UNDER_ATTACK(_) => "SMSG_ZONE_UNDER_ATTACK",
            ServerOpcodeMessage::MSG_AUCTION_HELLO(_) => "MSG_AUCTION_HELLO_Server",
            ServerOpcodeMessage::SMSG_AUCTION_LIST_RESULT(_) => "SMSG_AUCTION_LIST_RESULT",
            ServerOpcodeMessage::SMSG_AUCTION_OWNER_LIST_RESULT(_) => "SMSG_AUCTION_OWNER_LIST_RESULT",
            ServerOpcodeMessage::SMSG_AUCTION_BIDDER_NOTIFICATION(_) => "SMSG_AUCTION_BIDDER_NOTIFICATION",
            ServerOpcodeMessage::SMSG_AUCTION_OWNER_NOTIFICATION(_) => "SMSG_AUCTION_OWNER_NOTIFICATION",
            ServerOpcodeMessage::SMSG_PROCRESIST(_) => "SMSG_PROCRESIST",
            ServerOpcodeMessage::SMSG_AUCTION_BIDDER_LIST_RESULT(_) => "SMSG_AUCTION_BIDDER_LIST_RESULT",
            ServerOpcodeMessage::SMSG_PLAY_OBJECT_SOUND(_) => "SMSG_PLAY_OBJECT_SOUND",
            ServerOpcodeMessage::SMSG_RECEIVED_MAIL(_) => "SMSG_RECEIVED_MAIL",
            ServerOpcodeMessage::SMSG_AUCTION_REMOVED_NOTIFICATION(_) => "SMSG_AUCTION_REMOVED_NOTIFICATION",
            ServerOpcodeMessage::SMSG_SERVER_MESSAGE(_) => "SMSG_SERVER_MESSAGE",
            ServerOpcodeMessage::SMSG_STANDSTATE_UPDATE(_) => "SMSG_STANDSTATE_UPDATE",
            ServerOpcodeMessage::SMSG_LOOT_ALL_PASSED(_) => "SMSG_LOOT_ALL_PASSED",
            ServerOpcodeMessage::SMSG_LOOT_ROLL_WON(_) => "SMSG_LOOT_ROLL_WON",
            ServerOpcodeMessage::SMSG_LOOT_START_ROLL(_) => "SMSG_LOOT_START_ROLL",
            ServerOpcodeMessage::SMSG_LOOT_ROLL(_) => "SMSG_LOOT_ROLL",
            ServerOpcodeMessage::SMSG_LOOT_MASTER_LIST(_) => "SMSG_LOOT_MASTER_LIST",
            ServerOpcodeMessage::SMSG_SET_FORCED_REACTIONS(_) => "SMSG_SET_FORCED_REACTIONS",
            ServerOpcodeMessage::SMSG_CHAT_PLAYER_NOT_FOUND(_) => "SMSG_CHAT_PLAYER_NOT_FOUND",
            ServerOpcodeMessage::SMSG_DUEL_COUNTDOWN(_) => "SMSG_DUEL_COUNTDOWN",
            ServerOpcodeMessage::SMSG_AREA_TRIGGER_MESSAGE(_) => "SMSG_AREA_TRIGGER_MESSAGE",
            ServerOpcodeMessage::SMSG_DURABILITY_DAMAGE_DEATH(_) => "SMSG_DURABILITY_DAMAGE_DEATH",
            ServerOpcodeMessage::SMSG_INIT_WORLD_STATES(_) => "SMSG_INIT_WORLD_STATES",
            ServerOpcodeMessage::SMSG_UPDATE_WORLD_STATE(_) => "SMSG_UPDATE_WORLD_STATE",
            ServerOpcodeMessage::SMSG_CHAR_RENAME(_) => "SMSG_CHAR_RENAME",
            ServerOpcodeMessage::MSG_BATTLEGROUND_PLAYER_POSITIONS(_) => "MSG_BATTLEGROUND_PLAYER_POSITIONS_Server",
            ServerOpcodeMessage::SMSG_BINDER_CONFIRM(_) => "SMSG_BINDER_CONFIRM",
            ServerOpcodeMessage::SMSG_BATTLEGROUND_PLAYER_JOINED(_) => "SMSG_BATTLEGROUND_PLAYER_JOINED",
            ServerOpcodeMessage::SMSG_BATTLEGROUND_PLAYER_LEFT(_) => "SMSG_BATTLEGROUND_PLAYER_LEFT",
            ServerOpcodeMessage::SMSG_ADDON_INFO(_) => "SMSG_ADDON_INFO",
            ServerOpcodeMessage::SMSG_CHAT_RESTRICTED(_) => "SMSG_CHAT_RESTRICTED",
            ServerOpcodeMessage::SMSG_GM_TICKET_STATUS_UPDATE(_) => "SMSG_GM_TICKET_STATUS_UPDATE",
            ServerOpcodeMessage::SMSG_DEFENSE_MESSAGE(_) => "SMSG_DEFENSE_MESSAGE",
            ServerOpcodeMessage::SMSG_REALM_SPLIT(_) => "SMSG_REALM_SPLIT",
            ServerOpcodeMessage::SMSG_TIME_SYNC_REQ(_) => "SMSG_TIME_SYNC_REQ",
            ServerOpcodeMessage::SMSG_FEATURE_SYSTEM_STATUS(_) => "SMSG_FEATURE_SYSTEM_STATUS",
            ServerOpcodeMessage::SMSG_UPDATE_ACCOUNT_DATA_COMPLETE(_) => "SMSG_UPDATE_ACCOUNT_DATA_COMPLETE",
            ServerOpcodeMessage::SMSG_CLIENTCACHE_VERSION(_) => "SMSG_CLIENTCACHE_VERSION",
            ServerOpcodeMessage::SMSG_WORLD_STATE_UI_TIMER_UPDATE(_) => "SMSG_WORLD_STATE_UI_TIMER_UPDATE",
        })
    }
}

impl From<MSG_MOVE_START_FORWARD> for ServerOpcodeMessage {
    fn from(c: MSG_MOVE_START_FORWARD) -> Self {
        Self::MSG_MOVE_START_FORWARD(c)
    }
}

impl From<MSG_MOVE_START_BACKWARD> for ServerOpcodeMessage {
    fn from(c: MSG_MOVE_START_BACKWARD) -> Self {
        Self::MSG_MOVE_START_BACKWARD(c)
    }
}

impl From<MSG_MOVE_STOP> for ServerOpcodeMessage {
    fn from(c: MSG_MOVE_STOP) -> Self {
        Self::MSG_MOVE_STOP(c)
    }
}

impl From<MSG_MOVE_START_STRAFE_LEFT> for ServerOpcodeMessage {
    fn from(c: MSG_MOVE_START_STRAFE_LEFT) -> Self {
        Self::MSG_MOVE_START_STRAFE_LEFT(c)
    }
}

impl From<MSG_MOVE_START_STRAFE_RIGHT> for ServerOpcodeMessage {
    fn from(c: MSG_MOVE_START_STRAFE_RIGHT) -> Self {
        Self::MSG_MOVE_START_STRAFE_RIGHT(c)
    }
}

impl From<MSG_MOVE_STOP_STRAFE> for ServerOpcodeMessage {
    fn from(c: MSG_MOVE_STOP_STRAFE) -> Self {
        Self::MSG_MOVE_STOP_STRAFE(c)
    }
}

impl From<MSG_MOVE_JUMP> for ServerOpcodeMessage {
    fn from(c: MSG_MOVE_JUMP) -> Self {
        Self::MSG_MOVE_JUMP(c)
    }
}

impl From<MSG_MOVE_START_TURN_LEFT> for ServerOpcodeMessage {
    fn from(c: MSG_MOVE_START_TURN_LEFT) -> Self {
        Self::MSG_MOVE_START_TURN_LEFT(c)
    }
}

impl From<MSG_MOVE_START_TURN_RIGHT> for ServerOpcodeMessage {
    fn from(c: MSG_MOVE_START_TURN_RIGHT) -> Self {
        Self::MSG_MOVE_START_TURN_RIGHT(c)
    }
}

impl From<MSG_MOVE_STOP_TURN> for ServerOpcodeMessage {
    fn from(c: MSG_MOVE_STOP_TURN) -> Self {
        Self::MSG_MOVE_STOP_TURN(c)
    }
}

impl From<MSG_MOVE_START_PITCH_UP> for ServerOpcodeMessage {
    fn from(c: MSG_MOVE_START_PITCH_UP) -> Self {
        Self::MSG_MOVE_START_PITCH_UP(c)
    }
}

impl From<MSG_MOVE_START_PITCH_DOWN> for ServerOpcodeMessage {
    fn from(c: MSG_MOVE_START_PITCH_DOWN) -> Self {
        Self::MSG_MOVE_START_PITCH_DOWN(c)
    }
}

impl From<MSG_MOVE_STOP_PITCH> for ServerOpcodeMessage {
    fn from(c: MSG_MOVE_STOP_PITCH) -> Self {
        Self::MSG_MOVE_STOP_PITCH(c)
    }
}

impl From<MSG_MOVE_SET_RUN_MODE> for ServerOpcodeMessage {
    fn from(c: MSG_MOVE_SET_RUN_MODE) -> Self {
        Self::MSG_MOVE_SET_RUN_MODE(c)
    }
}

impl From<MSG_MOVE_SET_WALK_MODE> for ServerOpcodeMessage {
    fn from(c: MSG_MOVE_SET_WALK_MODE) -> Self {
        Self::MSG_MOVE_SET_WALK_MODE(c)
    }
}

impl From<MSG_MOVE_FALL_LAND> for ServerOpcodeMessage {
    fn from(c: MSG_MOVE_FALL_LAND) -> Self {
        Self::MSG_MOVE_FALL_LAND(c)
    }
}

impl From<MSG_MOVE_START_SWIM> for ServerOpcodeMessage {
    fn from(c: MSG_MOVE_START_SWIM) -> Self {
        Self::MSG_MOVE_START_SWIM(c)
    }
}

impl From<MSG_MOVE_STOP_SWIM> for ServerOpcodeMessage {
    fn from(c: MSG_MOVE_STOP_SWIM) -> Self {
        Self::MSG_MOVE_STOP_SWIM(c)
    }
}

impl From<MSG_MOVE_SET_FACING> for ServerOpcodeMessage {
    fn from(c: MSG_MOVE_SET_FACING) -> Self {
        Self::MSG_MOVE_SET_FACING(c)
    }
}

impl From<MSG_MOVE_SET_PITCH> for ServerOpcodeMessage {
    fn from(c: MSG_MOVE_SET_PITCH) -> Self {
        Self::MSG_MOVE_SET_PITCH(c)
    }
}

impl From<MSG_MOVE_WORLDPORT_ACK> for ServerOpcodeMessage {
    fn from(c: MSG_MOVE_WORLDPORT_ACK) -> Self {
        Self::MSG_MOVE_WORLDPORT_ACK(c)
    }
}

impl From<MSG_MOVE_HEARTBEAT> for ServerOpcodeMessage {
    fn from(c: MSG_MOVE_HEARTBEAT) -> Self {
        Self::MSG_MOVE_HEARTBEAT(c)
    }
}

impl From<MSG_PETITION_DECLINE> for ServerOpcodeMessage {
    fn from(c: MSG_PETITION_DECLINE) -> Self {
        Self::MSG_PETITION_DECLINE(c)
    }
}

impl From<MSG_TABARDVENDOR_ACTIVATE> for ServerOpcodeMessage {
    fn from(c: MSG_TABARDVENDOR_ACTIVATE) -> Self {
        Self::MSG_TABARDVENDOR_ACTIVATE(c)
    }
}

impl From<MSG_PETITION_RENAME> for ServerOpcodeMessage {
    fn from(c: MSG_PETITION_RENAME) -> Self {
        Self::MSG_PETITION_RENAME(c)
    }
}

impl From<MSG_SET_DUNGEON_DIFFICULTY> for ServerOpcodeMessage {
    fn from(c: MSG_SET_DUNGEON_DIFFICULTY) -> Self {
        Self::MSG_SET_DUNGEON_DIFFICULTY(c)
    }
}

impl From<MSG_MOVE_START_ASCEND> for ServerOpcodeMessage {
    fn from(c: MSG_MOVE_START_ASCEND) -> Self {
        Self::MSG_MOVE_START_ASCEND(c)
    }
}

impl From<MSG_MOVE_STOP_ASCEND> for ServerOpcodeMessage {
    fn from(c: MSG_MOVE_STOP_ASCEND) -> Self {
        Self::MSG_MOVE_STOP_ASCEND(c)
    }
}

impl From<MSG_MOVE_START_DESCEND> for ServerOpcodeMessage {
    fn from(c: MSG_MOVE_START_DESCEND) -> Self {
        Self::MSG_MOVE_START_DESCEND(c)
    }
}

impl From<SMSG_CHAR_CREATE> for ServerOpcodeMessage {
    fn from(c: SMSG_CHAR_CREATE) -> Self {
        Self::SMSG_CHAR_CREATE(c)
    }
}

impl From<SMSG_CHAR_ENUM> for ServerOpcodeMessage {
    fn from(c: SMSG_CHAR_ENUM) -> Self {
        Self::SMSG_CHAR_ENUM(c)
    }
}

impl From<SMSG_CHAR_DELETE> for ServerOpcodeMessage {
    fn from(c: SMSG_CHAR_DELETE) -> Self {
        Self::SMSG_CHAR_DELETE(c)
    }
}

impl From<SMSG_NEW_WORLD> for ServerOpcodeMessage {
    fn from(c: SMSG_NEW_WORLD) -> Self {
        Self::SMSG_NEW_WORLD(c)
    }
}

impl From<SMSG_TRANSFER_PENDING> for ServerOpcodeMessage {
    fn from(c: SMSG_TRANSFER_PENDING) -> Self {
        Self::SMSG_TRANSFER_PENDING(c)
    }
}

impl From<SMSG_CHARACTER_LOGIN_FAILED> for ServerOpcodeMessage {
    fn from(c: SMSG_CHARACTER_LOGIN_FAILED) -> Self {
        Self::SMSG_CHARACTER_LOGIN_FAILED(c)
    }
}

impl From<SMSG_LOGIN_SETTIMESPEED> for ServerOpcodeMessage {
    fn from(c: SMSG_LOGIN_SETTIMESPEED) -> Self {
        Self::SMSG_LOGIN_SETTIMESPEED(c)
    }
}

impl From<SMSG_LOGOUT_RESPONSE> for ServerOpcodeMessage {
    fn from(c: SMSG_LOGOUT_RESPONSE) -> Self {
        Self::SMSG_LOGOUT_RESPONSE(c)
    }
}

impl From<SMSG_LOGOUT_COMPLETE> for ServerOpcodeMessage {
    fn from(c: SMSG_LOGOUT_COMPLETE) -> Self {
        Self::SMSG_LOGOUT_COMPLETE(c)
    }
}

impl From<SMSG_LOGOUT_CANCEL_ACK> for ServerOpcodeMessage {
    fn from(c: SMSG_LOGOUT_CANCEL_ACK) -> Self {
        Self::SMSG_LOGOUT_CANCEL_ACK(c)
    }
}

impl From<SMSG_NAME_QUERY_RESPONSE> for ServerOpcodeMessage {
    fn from(c: SMSG_NAME_QUERY_RESPONSE) -> Self {
        Self::SMSG_NAME_QUERY_RESPONSE(c)
    }
}

impl From<SMSG_CONTACT_LIST> for ServerOpcodeMessage {
    fn from(c: SMSG_CONTACT_LIST) -> Self {
        Self::SMSG_CONTACT_LIST(c)
    }
}

impl From<SMSG_GUILD_INVITE> for ServerOpcodeMessage {
    fn from(c: SMSG_GUILD_INVITE) -> Self {
        Self::SMSG_GUILD_INVITE(c)
    }
}

impl From<SMSG_GUILD_INFO> for ServerOpcodeMessage {
    fn from(c: SMSG_GUILD_INFO) -> Self {
        Self::SMSG_GUILD_INFO(c)
    }
}

impl From<SMSG_GUILD_EVENT> for ServerOpcodeMessage {
    fn from(c: SMSG_GUILD_EVENT) -> Self {
        Self::SMSG_GUILD_EVENT(c)
    }
}

impl From<SMSG_GUILD_COMMAND_RESULT> for ServerOpcodeMessage {
    fn from(c: SMSG_GUILD_COMMAND_RESULT) -> Self {
        Self::SMSG_GUILD_COMMAND_RESULT(c)
    }
}

impl From<SMSG_CHANNEL_NOTIFY> for ServerOpcodeMessage {
    fn from(c: SMSG_CHANNEL_NOTIFY) -> Self {
        Self::SMSG_CHANNEL_NOTIFY(c)
    }
}

impl From<SMSG_CHANNEL_LIST> for ServerOpcodeMessage {
    fn from(c: SMSG_CHANNEL_LIST) -> Self {
        Self::SMSG_CHANNEL_LIST(c)
    }
}

impl From<SMSG_UPDATE_OBJECT> for ServerOpcodeMessage {
    fn from(c: SMSG_UPDATE_OBJECT) -> Self {
        Self::SMSG_UPDATE_OBJECT(c)
    }
}

impl From<SMSG_DESTROY_OBJECT> for ServerOpcodeMessage {
    fn from(c: SMSG_DESTROY_OBJECT) -> Self {
        Self::SMSG_DESTROY_OBJECT(c)
    }
}

impl From<SMSG_READ_ITEM_OK> for ServerOpcodeMessage {
    fn from(c: SMSG_READ_ITEM_OK) -> Self {
        Self::SMSG_READ_ITEM_OK(c)
    }
}

impl From<SMSG_READ_ITEM_FAILED> for ServerOpcodeMessage {
    fn from(c: SMSG_READ_ITEM_FAILED) -> Self {
        Self::SMSG_READ_ITEM_FAILED(c)
    }
}

impl From<SMSG_ITEM_COOLDOWN> for ServerOpcodeMessage {
    fn from(c: SMSG_ITEM_COOLDOWN) -> Self {
        Self::SMSG_ITEM_COOLDOWN(c)
    }
}

impl From<SMSG_GAMEOBJECT_CUSTOM_ANIM> for ServerOpcodeMessage {
    fn from(c: SMSG_GAMEOBJECT_CUSTOM_ANIM) -> Self {
        Self::SMSG_GAMEOBJECT_CUSTOM_ANIM(c)
    }
}

impl From<MSG_MOVE_TELEPORT_ACK_Server> for ServerOpcodeMessage {
    fn from(c: MSG_MOVE_TELEPORT_ACK_Server) -> Self {
        Self::MSG_MOVE_TELEPORT_ACK(c)
    }
}

impl From<SMSG_FORCE_MOVE_ROOT> for ServerOpcodeMessage {
    fn from(c: SMSG_FORCE_MOVE_ROOT) -> Self {
        Self::SMSG_FORCE_MOVE_ROOT(c)
    }
}

impl From<SMSG_FORCE_MOVE_UNROOT> for ServerOpcodeMessage {
    fn from(c: SMSG_FORCE_MOVE_UNROOT) -> Self {
        Self::SMSG_FORCE_MOVE_UNROOT(c)
    }
}

impl From<SMSG_TRIGGER_CINEMATIC> for ServerOpcodeMessage {
    fn from(c: SMSG_TRIGGER_CINEMATIC) -> Self {
        Self::SMSG_TRIGGER_CINEMATIC(c)
    }
}

impl From<SMSG_TUTORIAL_FLAGS> for ServerOpcodeMessage {
    fn from(c: SMSG_TUTORIAL_FLAGS) -> Self {
        Self::SMSG_TUTORIAL_FLAGS(c)
    }
}

impl From<SMSG_EMOTE> for ServerOpcodeMessage {
    fn from(c: SMSG_EMOTE) -> Self {
        Self::SMSG_EMOTE(c)
    }
}

impl From<SMSG_TEXT_EMOTE> for ServerOpcodeMessage {
    fn from(c: SMSG_TEXT_EMOTE) -> Self {
        Self::SMSG_TEXT_EMOTE(c)
    }
}

impl From<SMSG_INVENTORY_CHANGE_FAILURE> for ServerOpcodeMessage {
    fn from(c: SMSG_INVENTORY_CHANGE_FAILURE) -> Self {
        Self::SMSG_INVENTORY_CHANGE_FAILURE(c)
    }
}

impl From<SMSG_INITIALIZE_FACTIONS> for ServerOpcodeMessage {
    fn from(c: SMSG_INITIALIZE_FACTIONS) -> Self {
        Self::SMSG_INITIALIZE_FACTIONS(c)
    }
}

impl From<SMSG_SET_FACTION_VISIBLE> for ServerOpcodeMessage {
    fn from(c: SMSG_SET_FACTION_VISIBLE) -> Self {
        Self::SMSG_SET_FACTION_VISIBLE(c)
    }
}

impl From<SMSG_SET_FACTION_STANDING> for ServerOpcodeMessage {
    fn from(c: SMSG_SET_FACTION_STANDING) -> Self {
        Self::SMSG_SET_FACTION_STANDING(c)
    }
}

impl From<SMSG_SET_PROFICIENCY> for ServerOpcodeMessage {
    fn from(c: SMSG_SET_PROFICIENCY) -> Self {
        Self::SMSG_SET_PROFICIENCY(c)
    }
}

impl From<SMSG_ACTION_BUTTONS> for ServerOpcodeMessage {
    fn from(c: SMSG_ACTION_BUTTONS) -> Self {
        Self::SMSG_ACTION_BUTTONS(c)
    }
}

impl From<SMSG_AI_REACTION> for ServerOpcodeMessage {
    fn from(c: SMSG_AI_REACTION) -> Self {
        Self::SMSG_AI_REACTION(c)
    }
}

impl From<SMSG_ATTACKSTART> for ServerOpcodeMessage {
    fn from(c: SMSG_ATTACKSTART) -> Self {
        Self::SMSG_ATTACKSTART(c)
    }
}

impl From<SMSG_ATTACKSTOP> for ServerOpcodeMessage {
    fn from(c: SMSG_ATTACKSTOP) -> Self {
        Self::SMSG_ATTACKSTOP(c)
    }
}

impl From<SMSG_ATTACKSWING_NOTINRANGE> for ServerOpcodeMessage {
    fn from(c: SMSG_ATTACKSWING_NOTINRANGE) -> Self {
        Self::SMSG_ATTACKSWING_NOTINRANGE(c)
    }
}

impl From<SMSG_ATTACKSWING_BADFACING> for ServerOpcodeMessage {
    fn from(c: SMSG_ATTACKSWING_BADFACING) -> Self {
        Self::SMSG_ATTACKSWING_BADFACING(c)
    }
}

impl From<SMSG_ATTACKSWING_DEADTARGET> for ServerOpcodeMessage {
    fn from(c: SMSG_ATTACKSWING_DEADTARGET) -> Self {
        Self::SMSG_ATTACKSWING_DEADTARGET(c)
    }
}

impl From<SMSG_ATTACKSWING_CANT_ATTACK> for ServerOpcodeMessage {
    fn from(c: SMSG_ATTACKSWING_CANT_ATTACK) -> Self {
        Self::SMSG_ATTACKSWING_CANT_ATTACK(c)
    }
}

impl From<SMSG_ATTACKERSTATEUPDATE> for ServerOpcodeMessage {
    fn from(c: SMSG_ATTACKERSTATEUPDATE) -> Self {
        Self::SMSG_ATTACKERSTATEUPDATE(c)
    }
}

impl From<SMSG_CANCEL_COMBAT> for ServerOpcodeMessage {
    fn from(c: SMSG_CANCEL_COMBAT) -> Self {
        Self::SMSG_CANCEL_COMBAT(c)
    }
}

impl From<SMSG_BINDPOINTUPDATE> for ServerOpcodeMessage {
    fn from(c: SMSG_BINDPOINTUPDATE) -> Self {
        Self::SMSG_BINDPOINTUPDATE(c)
    }
}

impl From<SMSG_LOOT_RELEASE_RESPONSE> for ServerOpcodeMessage {
    fn from(c: SMSG_LOOT_RELEASE_RESPONSE) -> Self {
        Self::SMSG_LOOT_RELEASE_RESPONSE(c)
    }
}

impl From<SMSG_LOOT_REMOVED> for ServerOpcodeMessage {
    fn from(c: SMSG_LOOT_REMOVED) -> Self {
        Self::SMSG_LOOT_REMOVED(c)
    }
}

impl From<SMSG_LOOT_MONEY_NOTIFY> for ServerOpcodeMessage {
    fn from(c: SMSG_LOOT_MONEY_NOTIFY) -> Self {
        Self::SMSG_LOOT_MONEY_NOTIFY(c)
    }
}

impl From<SMSG_LOOT_CLEAR_MONEY> for ServerOpcodeMessage {
    fn from(c: SMSG_LOOT_CLEAR_MONEY) -> Self {
        Self::SMSG_LOOT_CLEAR_MONEY(c)
    }
}

impl From<SMSG_ITEM_PUSH_RESULT> for ServerOpcodeMessage {
    fn from(c: SMSG_ITEM_PUSH_RESULT) -> Self {
        Self::SMSG_ITEM_PUSH_RESULT(c)
    }
}

impl From<SMSG_DUEL_REQUESTED> for ServerOpcodeMessage {
    fn from(c: SMSG_DUEL_REQUESTED) -> Self {
        Self::SMSG_DUEL_REQUESTED(c)
    }
}

impl From<SMSG_DUEL_OUTOFBOUNDS> for ServerOpcodeMessage {
    fn from(c: SMSG_DUEL_OUTOFBOUNDS) -> Self {
        Self::SMSG_DUEL_OUTOFBOUNDS(c)
    }
}

impl From<SMSG_DUEL_INBOUNDS> for ServerOpcodeMessage {
    fn from(c: SMSG_DUEL_INBOUNDS) -> Self {
        Self::SMSG_DUEL_INBOUNDS(c)
    }
}

impl From<SMSG_DUEL_COMPLETE> for ServerOpcodeMessage {
    fn from(c: SMSG_DUEL_COMPLETE) -> Self {
        Self::SMSG_DUEL_COMPLETE(c)
    }
}

impl From<SMSG_DUEL_WINNER> for ServerOpcodeMessage {
    fn from(c: SMSG_DUEL_WINNER) -> Self {
        Self::SMSG_DUEL_WINNER(c)
    }
}

impl From<SMSG_MOUNTRESULT> for ServerOpcodeMessage {
    fn from(c: SMSG_MOUNTRESULT) -> Self {
        Self::SMSG_MOUNTRESULT(c)
    }
}

impl From<SMSG_MOUNTSPECIAL_ANIM> for ServerOpcodeMessage {
    fn from(c: SMSG_MOUNTSPECIAL_ANIM) -> Self {
        Self::SMSG_MOUNTSPECIAL_ANIM(c)
    }
}

impl From<SMSG_GOSSIP_MESSAGE> for ServerOpcodeMessage {
    fn from(c: SMSG_GOSSIP_MESSAGE) -> Self {
        Self::SMSG_GOSSIP_MESSAGE(c)
    }
}

impl From<SMSG_GOSSIP_COMPLETE> for ServerOpcodeMessage {
    fn from(c: SMSG_GOSSIP_COMPLETE) -> Self {
        Self::SMSG_GOSSIP_COMPLETE(c)
    }
}

impl From<SMSG_NPC_TEXT_UPDATE> for ServerOpcodeMessage {
    fn from(c: SMSG_NPC_TEXT_UPDATE) -> Self {
        Self::SMSG_NPC_TEXT_UPDATE(c)
    }
}

impl From<SMSG_LIST_INVENTORY> for ServerOpcodeMessage {
    fn from(c: SMSG_LIST_INVENTORY) -> Self {
        Self::SMSG_LIST_INVENTORY(c)
    }
}

impl From<SMSG_SELL_ITEM> for ServerOpcodeMessage {
    fn from(c: SMSG_SELL_ITEM) -> Self {
        Self::SMSG_SELL_ITEM(c)
    }
}

impl From<SMSG_BUY_ITEM> for ServerOpcodeMessage {
    fn from(c: SMSG_BUY_ITEM) -> Self {
        Self::SMSG_BUY_ITEM(c)
    }
}

impl From<SMSG_BUY_FAILED> for ServerOpcodeMessage {
    fn from(c: SMSG_BUY_FAILED) -> Self {
        Self::SMSG_BUY_FAILED(c)
    }
}

impl From<SMSG_SHOW_BANK> for ServerOpcodeMessage {
    fn from(c: SMSG_SHOW_BANK) -> Self {
        Self::SMSG_SHOW_BANK(c)
    }
}

impl From<SMSG_BUY_BANK_SLOT_RESULT> for ServerOpcodeMessage {
    fn from(c: SMSG_BUY_BANK_SLOT_RESULT) -> Self {
        Self::SMSG_BUY_BANK_SLOT_RESULT(c)
    }
}

impl From<SMSG_PETITION_SHOW_SIGNATURES> for ServerOpcodeMessage {
    fn from(c: SMSG_PETITION_SHOW_SIGNATURES) -> Self {
        Self::SMSG_PETITION_SHOW_SIGNATURES(c)
    }
}

impl From<SMSG_PETITION_SIGN_RESULTS> for ServerOpcodeMessage {
    fn from(c: SMSG_PETITION_SIGN_RESULTS) -> Self {
        Self::SMSG_PETITION_SIGN_RESULTS(c)
    }
}

impl From<SMSG_TURN_IN_PETITION_RESULTS> for ServerOpcodeMessage {
    fn from(c: SMSG_TURN_IN_PETITION_RESULTS) -> Self {
        Self::SMSG_TURN_IN_PETITION_RESULTS(c)
    }
}

impl From<SMSG_NOTIFICATION> for ServerOpcodeMessage {
    fn from(c: SMSG_NOTIFICATION) -> Self {
        Self::SMSG_NOTIFICATION(c)
    }
}

impl From<SMSG_PLAYED_TIME> for ServerOpcodeMessage {
    fn from(c: SMSG_PLAYED_TIME) -> Self {
        Self::SMSG_PLAYED_TIME(c)
    }
}

impl From<SMSG_QUERY_TIME_RESPONSE> for ServerOpcodeMessage {
    fn from(c: SMSG_QUERY_TIME_RESPONSE) -> Self {
        Self::SMSG_QUERY_TIME_RESPONSE(c)
    }
}

impl From<SMSG_LOG_XPGAIN> for ServerOpcodeMessage {
    fn from(c: SMSG_LOG_XPGAIN) -> Self {
        Self::SMSG_LOG_XPGAIN(c)
    }
}

impl From<SMSG_LEVELUP_INFO> for ServerOpcodeMessage {
    fn from(c: SMSG_LEVELUP_INFO) -> Self {
        Self::SMSG_LEVELUP_INFO(c)
    }
}

impl From<SMSG_PONG> for ServerOpcodeMessage {
    fn from(c: SMSG_PONG) -> Self {
        Self::SMSG_PONG(c)
    }
}

impl From<SMSG_GAMEOBJECT_PAGETEXT> for ServerOpcodeMessage {
    fn from(c: SMSG_GAMEOBJECT_PAGETEXT) -> Self {
        Self::SMSG_GAMEOBJECT_PAGETEXT(c)
    }
}

impl From<SMSG_ITEM_TIME_UPDATE> for ServerOpcodeMessage {
    fn from(c: SMSG_ITEM_TIME_UPDATE) -> Self {
        Self::SMSG_ITEM_TIME_UPDATE(c)
    }
}

impl From<SMSG_ITEM_ENCHANT_TIME_UPDATE> for ServerOpcodeMessage {
    fn from(c: SMSG_ITEM_ENCHANT_TIME_UPDATE) -> Self {
        Self::SMSG_ITEM_ENCHANT_TIME_UPDATE(c)
    }
}

impl From<SMSG_AUTH_CHALLENGE> for ServerOpcodeMessage {
    fn from(c: SMSG_AUTH_CHALLENGE) -> Self {
        Self::SMSG_AUTH_CHALLENGE(c)
    }
}

impl From<SMSG_AUTH_RESPONSE> for ServerOpcodeMessage {
    fn from(c: SMSG_AUTH_RESPONSE) -> Self {
        Self::SMSG_AUTH_RESPONSE(c)
    }
}

impl From<MSG_SAVE_GUILD_EMBLEM_Server> for ServerOpcodeMessage {
    fn from(c: MSG_SAVE_GUILD_EMBLEM_Server) -> Self {
        Self::MSG_SAVE_GUILD_EMBLEM(c)
    }
}

impl From<SMSG_EXPLORATION_EXPERIENCE> for ServerOpcodeMessage {
    fn from(c: SMSG_EXPLORATION_EXPERIENCE) -> Self {
        Self::SMSG_EXPLORATION_EXPERIENCE(c)
    }
}

impl From<MSG_RANDOM_ROLL_Server> for ServerOpcodeMessage {
    fn from(c: MSG_RANDOM_ROLL_Server) -> Self {
        Self::MSG_RANDOM_ROLL(c)
    }
}

impl From<SMSG_ENVIRONMENTALDAMAGELOG> for ServerOpcodeMessage {
    fn from(c: SMSG_ENVIRONMENTALDAMAGELOG) -> Self {
        Self::SMSG_ENVIRONMENTALDAMAGELOG(c)
    }
}

impl From<SMSG_GMTICKET_CREATE> for ServerOpcodeMessage {
    fn from(c: SMSG_GMTICKET_CREATE) -> Self {
        Self::SMSG_GMTICKET_CREATE(c)
    }
}

impl From<SMSG_GMTICKET_UPDATETEXT> for ServerOpcodeMessage {
    fn from(c: SMSG_GMTICKET_UPDATETEXT) -> Self {
        Self::SMSG_GMTICKET_UPDATETEXT(c)
    }
}

impl From<SMSG_ACCOUNT_DATA_TIMES> for ServerOpcodeMessage {
    fn from(c: SMSG_ACCOUNT_DATA_TIMES) -> Self {
        Self::SMSG_ACCOUNT_DATA_TIMES(c)
    }
}

impl From<SMSG_UPDATE_ACCOUNT_DATA> for ServerOpcodeMessage {
    fn from(c: SMSG_UPDATE_ACCOUNT_DATA) -> Self {
        Self::SMSG_UPDATE_ACCOUNT_DATA(c)
    }
}

impl From<SMSG_GAMEOBJECT_DESPAWN_ANIM> for ServerOpcodeMessage {
    fn from(c: SMSG_GAMEOBJECT_DESPAWN_ANIM) -> Self {
        Self::SMSG_GAMEOBJECT_DESPAWN_ANIM(c)
    }
}

impl From<SMSG_GMTICKET_DELETETICKET> for ServerOpcodeMessage {
    fn from(c: SMSG_GMTICKET_DELETETICKET) -> Self {
        Self::SMSG_GMTICKET_DELETETICKET(c)
    }
}

impl From<SMSG_CHAT_WRONG_FACTION> for ServerOpcodeMessage {
    fn from(c: SMSG_CHAT_WRONG_FACTION) -> Self {
        Self::SMSG_CHAT_WRONG_FACTION(c)
    }
}

impl From<SMSG_GMTICKET_SYSTEMSTATUS> for ServerOpcodeMessage {
    fn from(c: SMSG_GMTICKET_SYSTEMSTATUS) -> Self {
        Self::SMSG_GMTICKET_SYSTEMSTATUS(c)
    }
}

impl From<SMSG_SET_REST_START> for ServerOpcodeMessage {
    fn from(c: SMSG_SET_REST_START) -> Self {
        Self::SMSG_SET_REST_START(c)
    }
}

impl From<SMSG_GOSSIP_POI> for ServerOpcodeMessage {
    fn from(c: SMSG_GOSSIP_POI) -> Self {
        Self::SMSG_GOSSIP_POI(c)
    }
}

impl From<SMSG_LOGIN_VERIFY_WORLD> for ServerOpcodeMessage {
    fn from(c: SMSG_LOGIN_VERIFY_WORLD) -> Self {
        Self::SMSG_LOGIN_VERIFY_WORLD(c)
    }
}

impl From<SMSG_BATTLEFIELD_LIST> for ServerOpcodeMessage {
    fn from(c: SMSG_BATTLEFIELD_LIST) -> Self {
        Self::SMSG_BATTLEFIELD_LIST(c)
    }
}

impl From<SMSG_ZONE_UNDER_ATTACK> for ServerOpcodeMessage {
    fn from(c: SMSG_ZONE_UNDER_ATTACK) -> Self {
        Self::SMSG_ZONE_UNDER_ATTACK(c)
    }
}

impl From<MSG_AUCTION_HELLO_Server> for ServerOpcodeMessage {
    fn from(c: MSG_AUCTION_HELLO_Server) -> Self {
        Self::MSG_AUCTION_HELLO(c)
    }
}

impl From<SMSG_AUCTION_LIST_RESULT> for ServerOpcodeMessage {
    fn from(c: SMSG_AUCTION_LIST_RESULT) -> Self {
        Self::SMSG_AUCTION_LIST_RESULT(c)
    }
}

impl From<SMSG_AUCTION_OWNER_LIST_RESULT> for ServerOpcodeMessage {
    fn from(c: SMSG_AUCTION_OWNER_LIST_RESULT) -> Self {
        Self::SMSG_AUCTION_OWNER_LIST_RESULT(c)
    }
}

impl From<SMSG_AUCTION_BIDDER_NOTIFICATION> for ServerOpcodeMessage {
    fn from(c: SMSG_AUCTION_BIDDER_NOTIFICATION) -> Self {
        Self::SMSG_AUCTION_BIDDER_NOTIFICATION(c)
    }
}

impl From<SMSG_AUCTION_OWNER_NOTIFICATION> for ServerOpcodeMessage {
    fn from(c: SMSG_AUCTION_OWNER_NOTIFICATION) -> Self {
        Self::SMSG_AUCTION_OWNER_NOTIFICATION(c)
    }
}

impl From<SMSG_PROCRESIST> for ServerOpcodeMessage {
    fn from(c: SMSG_PROCRESIST) -> Self {
        Self::SMSG_PROCRESIST(c)
    }
}

impl From<SMSG_AUCTION_BIDDER_LIST_RESULT> for ServerOpcodeMessage {
    fn from(c: SMSG_AUCTION_BIDDER_LIST_RESULT) -> Self {
        Self::SMSG_AUCTION_BIDDER_LIST_RESULT(c)
    }
}

impl From<SMSG_PLAY_OBJECT_SOUND> for ServerOpcodeMessage {
    fn from(c: SMSG_PLAY_OBJECT_SOUND) -> Self {
        Self::SMSG_PLAY_OBJECT_SOUND(c)
    }
}

impl From<SMSG_RECEIVED_MAIL> for ServerOpcodeMessage {
    fn from(c: SMSG_RECEIVED_MAIL) -> Self {
        Self::SMSG_RECEIVED_MAIL(c)
    }
}

impl From<SMSG_AUCTION_REMOVED_NOTIFICATION> for ServerOpcodeMessage {
    fn from(c: SMSG_AUCTION_REMOVED_NOTIFICATION) -> Self {
        Self::SMSG_AUCTION_REMOVED_NOTIFICATION(c)
    }
}

impl From<SMSG_SERVER_MESSAGE> for ServerOpcodeMessage {
    fn from(c: SMSG_SERVER_MESSAGE) -> Self {
        Self::SMSG_SERVER_MESSAGE(c)
    }
}

impl From<SMSG_STANDSTATE_UPDATE> for ServerOpcodeMessage {
    fn from(c: SMSG_STANDSTATE_UPDATE) -> Self {
        Self::SMSG_STANDSTATE_UPDATE(c)
    }
}

impl From<SMSG_LOOT_ALL_PASSED> for ServerOpcodeMessage {
    fn from(c: SMSG_LOOT_ALL_PASSED) -> Self {
        Self::SMSG_LOOT_ALL_PASSED(c)
    }
}

impl From<SMSG_LOOT_ROLL_WON> for ServerOpcodeMessage {
    fn from(c: SMSG_LOOT_ROLL_WON) -> Self {
        Self::SMSG_LOOT_ROLL_WON(c)
    }
}

impl From<SMSG_LOOT_START_ROLL> for ServerOpcodeMessage {
    fn from(c: SMSG_LOOT_START_ROLL) -> Self {
        Self::SMSG_LOOT_START_ROLL(c)
    }
}

impl From<SMSG_LOOT_ROLL> for ServerOpcodeMessage {
    fn from(c: SMSG_LOOT_ROLL) -> Self {
        Self::SMSG_LOOT_ROLL(c)
    }
}

impl From<SMSG_LOOT_MASTER_LIST> for ServerOpcodeMessage {
    fn from(c: SMSG_LOOT_MASTER_LIST) -> Self {
        Self::SMSG_LOOT_MASTER_LIST(c)
    }
}

impl From<SMSG_SET_FORCED_REACTIONS> for ServerOpcodeMessage {
    fn from(c: SMSG_SET_FORCED_REACTIONS) -> Self {
        Self::SMSG_SET_FORCED_REACTIONS(c)
    }
}

impl From<SMSG_CHAT_PLAYER_NOT_FOUND> for ServerOpcodeMessage {
    fn from(c: SMSG_CHAT_PLAYER_NOT_FOUND) -> Self {
        Self::SMSG_CHAT_PLAYER_NOT_FOUND(c)
    }
}

impl From<SMSG_DUEL_COUNTDOWN> for ServerOpcodeMessage {
    fn from(c: SMSG_DUEL_COUNTDOWN) -> Self {
        Self::SMSG_DUEL_COUNTDOWN(c)
    }
}

impl From<SMSG_AREA_TRIGGER_MESSAGE> for ServerOpcodeMessage {
    fn from(c: SMSG_AREA_TRIGGER_MESSAGE) -> Self {
        Self::SMSG_AREA_TRIGGER_MESSAGE(c)
    }
}

impl From<SMSG_DURABILITY_DAMAGE_DEATH> for ServerOpcodeMessage {
    fn from(c: SMSG_DURABILITY_DAMAGE_DEATH) -> Self {
        Self::SMSG_DURABILITY_DAMAGE_DEATH(c)
    }
}

impl From<SMSG_INIT_WORLD_STATES> for ServerOpcodeMessage {
    fn from(c: SMSG_INIT_WORLD_STATES) -> Self {
        Self::SMSG_INIT_WORLD_STATES(c)
    }
}

impl From<SMSG_UPDATE_WORLD_STATE> for ServerOpcodeMessage {
    fn from(c: SMSG_UPDATE_WORLD_STATE) -> Self {
        Self::SMSG_UPDATE_WORLD_STATE(c)
    }
}

impl From<SMSG_CHAR_RENAME> for ServerOpcodeMessage {
    fn from(c: SMSG_CHAR_RENAME) -> Self {
        Self::SMSG_CHAR_RENAME(c)
    }
}

impl From<MSG_BATTLEGROUND_PLAYER_POSITIONS_Server> for ServerOpcodeMessage {
    fn from(c: MSG_BATTLEGROUND_PLAYER_POSITIONS_Server) -> Self {
        Self::MSG_BATTLEGROUND_PLAYER_POSITIONS(c)
    }
}

impl From<SMSG_BINDER_CONFIRM> for ServerOpcodeMessage {
    fn from(c: SMSG_BINDER_CONFIRM) -> Self {
        Self::SMSG_BINDER_CONFIRM(c)
    }
}

impl From<SMSG_BATTLEGROUND_PLAYER_JOINED> for ServerOpcodeMessage {
    fn from(c: SMSG_BATTLEGROUND_PLAYER_JOINED) -> Self {
        Self::SMSG_BATTLEGROUND_PLAYER_JOINED(c)
    }
}

impl From<SMSG_BATTLEGROUND_PLAYER_LEFT> for ServerOpcodeMessage {
    fn from(c: SMSG_BATTLEGROUND_PLAYER_LEFT) -> Self {
        Self::SMSG_BATTLEGROUND_PLAYER_LEFT(c)
    }
}

impl From<SMSG_ADDON_INFO> for ServerOpcodeMessage {
    fn from(c: SMSG_ADDON_INFO) -> Self {
        Self::SMSG_ADDON_INFO(c)
    }
}

impl From<SMSG_CHAT_RESTRICTED> for ServerOpcodeMessage {
    fn from(c: SMSG_CHAT_RESTRICTED) -> Self {
        Self::SMSG_CHAT_RESTRICTED(c)
    }
}

impl From<SMSG_GM_TICKET_STATUS_UPDATE> for ServerOpcodeMessage {
    fn from(c: SMSG_GM_TICKET_STATUS_UPDATE) -> Self {
        Self::SMSG_GM_TICKET_STATUS_UPDATE(c)
    }
}

impl From<SMSG_DEFENSE_MESSAGE> for ServerOpcodeMessage {
    fn from(c: SMSG_DEFENSE_MESSAGE) -> Self {
        Self::SMSG_DEFENSE_MESSAGE(c)
    }
}

impl From<SMSG_REALM_SPLIT> for ServerOpcodeMessage {
    fn from(c: SMSG_REALM_SPLIT) -> Self {
        Self::SMSG_REALM_SPLIT(c)
    }
}

impl From<SMSG_TIME_SYNC_REQ> for ServerOpcodeMessage {
    fn from(c: SMSG_TIME_SYNC_REQ) -> Self {
        Self::SMSG_TIME_SYNC_REQ(c)
    }
}

impl From<SMSG_FEATURE_SYSTEM_STATUS> for ServerOpcodeMessage {
    fn from(c: SMSG_FEATURE_SYSTEM_STATUS) -> Self {
        Self::SMSG_FEATURE_SYSTEM_STATUS(c)
    }
}

impl From<SMSG_UPDATE_ACCOUNT_DATA_COMPLETE> for ServerOpcodeMessage {
    fn from(c: SMSG_UPDATE_ACCOUNT_DATA_COMPLETE) -> Self {
        Self::SMSG_UPDATE_ACCOUNT_DATA_COMPLETE(c)
    }
}

impl From<SMSG_CLIENTCACHE_VERSION> for ServerOpcodeMessage {
    fn from(c: SMSG_CLIENTCACHE_VERSION) -> Self {
        Self::SMSG_CLIENTCACHE_VERSION(c)
    }
}

impl From<SMSG_WORLD_STATE_UI_TIMER_UPDATE> for ServerOpcodeMessage {
    fn from(c: SMSG_WORLD_STATE_UI_TIMER_UPDATE) -> Self {
        Self::SMSG_WORLD_STATE_UI_TIMER_UPDATE(c)
    }
}

