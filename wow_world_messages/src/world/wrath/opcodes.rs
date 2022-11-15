use crate::wrath::{ServerMessage, ClientMessage};
#[cfg(feature = "encryption")]
use wow_srp::wrath_header::{ClientEncrypterHalf, ClientDecrypterHalf, ServerEncrypterHalf, ServerDecrypterHalf};

#[cfg(feature = "tokio")]
use tokio::io::{AsyncReadExt, AsyncWriteExt};
#[cfg(feature = "async-std")]
use async_std::io::{ReadExt, WriteExt};
use crate::world::wrath::MovementInfo;
use crate::errors::ParseError;
use crate::wrath::opcode_to_name;
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
use crate::world::wrath::MSG_QUEST_PUSH_RESULT;
use crate::world::wrath::MSG_PETITION_RENAME;
use crate::world::wrath::MSG_SET_DUNGEON_DIFFICULTY;
use crate::world::wrath::MSG_MOVE_START_ASCEND;
use crate::world::wrath::MSG_MOVE_STOP_ASCEND;
use crate::world::wrath::MSG_MOVE_START_DESCEND;
use crate::world::wrath::MSG_GUILD_BANK_MONEY_WITHDRAWN;
use crate::world::wrath::CMSG_CALENDAR_GET_NUM_PENDING;
use crate::world::wrath::CMSG_BOOTME;
use crate::world::wrath::CMSG_WORLD_TELEPORT;
use crate::world::wrath::CMSG_TELEPORT_TO_UNIT;
use crate::world::wrath::CMSG_CHAR_CREATE;
use crate::world::wrath::CMSG_CHAR_ENUM;
use crate::world::wrath::CMSG_CHAR_DELETE;
use crate::world::wrath::CMSG_PLAYER_LOGIN;
use crate::world::wrath::CMSG_PLAYER_LOGOUT;
use crate::world::wrath::CMSG_LOGOUT_REQUEST;
use crate::world::wrath::CMSG_LOGOUT_CANCEL;
use crate::world::wrath::CMSG_NAME_QUERY;
use crate::world::wrath::CMSG_PET_NAME_QUERY;
use crate::world::wrath::CMSG_GUILD_QUERY;
use crate::world::wrath::CMSG_ITEM_QUERY_SINGLE;
use crate::world::wrath::CMSG_PAGE_TEXT_QUERY;
use crate::world::wrath::CMSG_QUEST_QUERY;
use crate::world::wrath::CMSG_GAMEOBJECT_QUERY;
use crate::world::wrath::CMSG_CREATURE_QUERY;
use crate::world::wrath::CMSG_WHO;
use crate::world::wrath::CMSG_WHOIS;
use crate::world::wrath::CMSG_CONTACT_LIST;
use crate::world::wrath::CMSG_ADD_FRIEND;
use crate::world::wrath::CMSG_DEL_FRIEND;
use crate::world::wrath::CMSG_ADD_IGNORE;
use crate::world::wrath::CMSG_DEL_IGNORE;
use crate::world::wrath::CMSG_GROUP_INVITE;
use crate::world::wrath::CMSG_GROUP_ACCEPT;
use crate::world::wrath::CMSG_GROUP_DECLINE;
use crate::world::wrath::CMSG_GROUP_UNINVITE;
use crate::world::wrath::CMSG_GROUP_UNINVITE_GUID;
use crate::world::wrath::CMSG_GROUP_SET_LEADER;
use crate::world::wrath::CMSG_LOOT_METHOD;
use crate::world::wrath::CMSG_GROUP_DISBAND;
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
use crate::world::wrath::CMSG_USE_ITEM;
use crate::world::wrath::CMSG_OPEN_ITEM;
use crate::world::wrath::CMSG_READ_ITEM;
use crate::world::wrath::CMSG_GAMEOBJ_USE;
use crate::world::wrath::CMSG_AREATRIGGER;
use crate::world::wrath::MSG_MOVE_TELEPORT_ACK_Client;
use crate::world::wrath::CMSG_MOVE_SET_RAW_POSITION;
use crate::world::wrath::CMSG_FORCE_RUN_SPEED_CHANGE_ACK;
use crate::world::wrath::CMSG_FORCE_RUN_BACK_SPEED_CHANGE_ACK;
use crate::world::wrath::CMSG_FORCE_SWIM_SPEED_CHANGE_ACK;
use crate::world::wrath::CMSG_FORCE_MOVE_ROOT_ACK;
use crate::world::wrath::CMSG_FORCE_MOVE_UNROOT_ACK;
use crate::world::wrath::CMSG_MOVE_KNOCK_BACK_ACK;
use crate::world::wrath::CMSG_MOVE_HOVER_ACK;
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
use crate::world::wrath::CMSG_INITIATE_TRADE;
use crate::world::wrath::CMSG_BEGIN_TRADE;
use crate::world::wrath::CMSG_BUSY_TRADE;
use crate::world::wrath::CMSG_IGNORE_TRADE;
use crate::world::wrath::CMSG_ACCEPT_TRADE;
use crate::world::wrath::CMSG_UNACCEPT_TRADE;
use crate::world::wrath::CMSG_CANCEL_TRADE;
use crate::world::wrath::CMSG_SET_TRADE_ITEM;
use crate::world::wrath::CMSG_CLEAR_TRADE_ITEM;
use crate::world::wrath::CMSG_SET_TRADE_GOLD;
use crate::world::wrath::CMSG_SET_FACTION_ATWAR;
use crate::world::wrath::CMSG_SET_ACTION_BUTTON;
use crate::world::wrath::CMSG_CAST_SPELL;
use crate::world::wrath::CMSG_CANCEL_CAST;
use crate::world::wrath::CMSG_CANCEL_AURA;
use crate::world::wrath::CMSG_CANCEL_CHANNELLING;
use crate::world::wrath::CMSG_SET_SELECTION;
use crate::world::wrath::CMSG_DELETEEQUIPMENT_SET;
use crate::world::wrath::CMSG_ATTACKSWING;
use crate::world::wrath::CMSG_ATTACKSTOP;
use crate::world::wrath::CMSG_REPOP_REQUEST;
use crate::world::wrath::CMSG_RESURRECT_RESPONSE;
use crate::world::wrath::CMSG_LOOT;
use crate::world::wrath::CMSG_LOOT_MONEY;
use crate::world::wrath::CMSG_LOOT_RELEASE;
use crate::world::wrath::CMSG_DUEL_ACCEPTED;
use crate::world::wrath::CMSG_DUEL_CANCELLED;
use crate::world::wrath::CMSG_MOUNTSPECIAL_ANIM;
use crate::world::wrath::CMSG_PET_SET_ACTION;
use crate::world::wrath::CMSG_PET_ACTION;
use crate::world::wrath::CMSG_PET_ABANDON;
use crate::world::wrath::CMSG_PET_RENAME;
use crate::world::wrath::CMSG_GOSSIP_HELLO;
use crate::world::wrath::CMSG_GOSSIP_SELECT_OPTION;
use crate::world::wrath::CMSG_NPC_TEXT_QUERY;
use crate::world::wrath::CMSG_QUESTGIVER_STATUS_QUERY;
use crate::world::wrath::CMSG_QUESTGIVER_HELLO;
use crate::world::wrath::CMSG_QUESTGIVER_QUERY_QUEST;
use crate::world::wrath::CMSG_QUESTGIVER_QUEST_AUTOLAUNCH;
use crate::world::wrath::CMSG_QUESTGIVER_ACCEPT_QUEST;
use crate::world::wrath::CMSG_QUESTGIVER_COMPLETE_QUEST;
use crate::world::wrath::CMSG_QUESTGIVER_REQUEST_REWARD;
use crate::world::wrath::CMSG_QUESTGIVER_CHOOSE_REWARD;
use crate::world::wrath::CMSG_QUESTGIVER_CANCEL;
use crate::world::wrath::CMSG_QUESTLOG_SWAP_QUEST;
use crate::world::wrath::CMSG_QUESTLOG_REMOVE_QUEST;
use crate::world::wrath::CMSG_QUEST_CONFIRM_ACCEPT;
use crate::world::wrath::CMSG_PUSHQUESTTOPARTY;
use crate::world::wrath::CMSG_LIST_INVENTORY;
use crate::world::wrath::CMSG_SELL_ITEM;
use crate::world::wrath::CMSG_BUY_ITEM;
use crate::world::wrath::CMSG_BUY_ITEM_IN_SLOT;
use crate::world::wrath::CMSG_TAXINODE_STATUS_QUERY;
use crate::world::wrath::CMSG_TAXIQUERYAVAILABLENODES;
use crate::world::wrath::CMSG_ACTIVATETAXI;
use crate::world::wrath::CMSG_TRAINER_LIST;
use crate::world::wrath::CMSG_TRAINER_BUY_SPELL;
use crate::world::wrath::CMSG_BINDER_ACTIVATE;
use crate::world::wrath::CMSG_BANKER_ACTIVATE;
use crate::world::wrath::CMSG_BUY_BANK_SLOT;
use crate::world::wrath::CMSG_PETITION_SHOWLIST;
use crate::world::wrath::CMSG_PETITION_BUY;
use crate::world::wrath::CMSG_PETITION_SHOW_SIGNATURES;
use crate::world::wrath::CMSG_PETITION_SIGN;
use crate::world::wrath::CMSG_OFFER_PETITION;
use crate::world::wrath::CMSG_TURN_IN_PETITION;
use crate::world::wrath::CMSG_PETITION_QUERY;
use crate::world::wrath::CMSG_BUG;
use crate::world::wrath::CMSG_PLAYED_TIME;
use crate::world::wrath::CMSG_QUERY_TIME;
use crate::world::wrath::CMSG_RECLAIM_CORPSE;
use crate::world::wrath::CMSG_WRAP_ITEM;
use crate::world::wrath::MSG_MINIMAP_PING_Client;
use crate::world::wrath::CMSG_PING;
use crate::world::wrath::CMSG_SETSHEATHED;
use crate::world::wrath::CMSG_AUTH_SESSION;
use crate::world::wrath::CMSG_PET_CAST_SPELL;
use crate::world::wrath::MSG_SAVE_GUILD_EMBLEM_Client;
use crate::world::wrath::CMSG_ZONEUPDATE;
use crate::world::wrath::MSG_RANDOM_ROLL_Client;
use crate::world::wrath::CMSG_UNLEARN_SKILL;
use crate::world::wrath::CMSG_GMTICKET_CREATE;
use crate::world::wrath::CMSG_GMTICKET_UPDATETEXT;
use crate::world::wrath::CMSG_REQUEST_ACCOUNT_DATA;
use crate::world::wrath::CMSG_UPDATE_ACCOUNT_DATA;
use crate::world::wrath::CMSG_GMTICKET_GETTICKET;
use crate::world::wrath::MSG_CORPSE_QUERY_Client;
use crate::world::wrath::CMSG_GMTICKET_DELETETICKET;
use crate::world::wrath::CMSG_GMTICKET_SYSTEMSTATUS;
use crate::world::wrath::CMSG_SPIRIT_HEALER_ACTIVATE;
use crate::world::wrath::CMSG_CHAT_IGNORED;
use crate::world::wrath::CMSG_GUILD_RANK;
use crate::world::wrath::CMSG_GUILD_ADD_RANK;
use crate::world::wrath::CMSG_GUILD_DEL_RANK;
use crate::world::wrath::CMSG_GUILD_SET_PUBLIC_NOTE;
use crate::world::wrath::CMSG_GUILD_SET_OFFICER_NOTE;
use crate::world::wrath::CMSG_SEND_MAIL;
use crate::world::wrath::CMSG_GET_MAIL_LIST;
use crate::world::wrath::CMSG_BATTLEFIELD_LIST;
use crate::world::wrath::CMSG_ITEM_TEXT_QUERY;
use crate::world::wrath::CMSG_MAIL_TAKE_MONEY;
use crate::world::wrath::CMSG_MAIL_TAKE_ITEM;
use crate::world::wrath::CMSG_MAIL_MARK_AS_READ;
use crate::world::wrath::CMSG_MAIL_RETURN_TO_SENDER;
use crate::world::wrath::CMSG_MAIL_DELETE;
use crate::world::wrath::CMSG_MAIL_CREATE_TEXT_ITEM;
use crate::world::wrath::CMSG_LEARN_TALENT;
use crate::world::wrath::CMSG_TOGGLE_PVP;
use crate::world::wrath::MSG_AUCTION_HELLO_Client;
use crate::world::wrath::CMSG_AUCTION_SELL_ITEM;
use crate::world::wrath::CMSG_AUCTION_REMOVE_ITEM;
use crate::world::wrath::CMSG_AUCTION_LIST_ITEMS;
use crate::world::wrath::CMSG_AUCTION_LIST_OWNER_ITEMS;
use crate::world::wrath::CMSG_AUCTION_PLACE_BID;
use crate::world::wrath::CMSG_AUCTION_LIST_BIDDER_ITEMS;
use crate::world::wrath::CMSG_SET_AMMO;
use crate::world::wrath::CMSG_SET_ACTIVE_MOVER;
use crate::world::wrath::CMSG_PET_CANCEL_AURA;
use crate::world::wrath::CMSG_CANCEL_AUTO_REPEAT_SPELL;
use crate::world::wrath::MSG_LIST_STABLED_PETS_Client;
use crate::world::wrath::CMSG_STABLE_PET;
use crate::world::wrath::CMSG_UNSTABLE_PET;
use crate::world::wrath::CMSG_BUY_STABLE_SLOT;
use crate::world::wrath::CMSG_STABLE_SWAP_PET;
use crate::world::wrath::CMSG_REQUEST_PET_INFO;
use crate::world::wrath::CMSG_FAR_SIGHT;
use crate::world::wrath::CMSG_GROUP_CHANGE_SUB_GROUP;
use crate::world::wrath::CMSG_REQUEST_PARTY_MEMBER_STATS;
use crate::world::wrath::CMSG_GROUP_SWAP_SUB_GROUP;
use crate::world::wrath::CMSG_AUTOSTORE_BANK_ITEM;
use crate::world::wrath::CMSG_AUTOBANK_ITEM;
use crate::world::wrath::MSG_QUERY_NEXT_MAIL_TIME_Client;
use crate::world::wrath::CMSG_GROUP_RAID_CONVERT;
use crate::world::wrath::CMSG_GROUP_ASSISTANT_LEADER;
use crate::world::wrath::CMSG_BUYBACK_ITEM;
use crate::world::wrath::CMSG_LFG_GET_STATUS;
use crate::world::wrath::CMSG_CANCEL_GROWTH_AURA;
use crate::world::wrath::CMSG_LOOT_ROLL;
use crate::world::wrath::CMSG_LOOT_MASTER_GIVE;
use crate::world::wrath::CMSG_REPAIR_ITEM;
use crate::world::wrath::CMSG_SUMMON_RESPONSE;
use crate::world::wrath::CMSG_SELF_RES;
use crate::world::wrath::CMSG_TOGGLE_HELM;
use crate::world::wrath::CMSG_TOGGLE_CLOAK;
use crate::world::wrath::CMSG_SET_ACTIONBAR_TOGGLES;
use crate::world::wrath::CMSG_ITEM_NAME_QUERY;
use crate::world::wrath::CMSG_CHAR_RENAME;
use crate::world::wrath::CMSG_MOVE_SPLINE_DONE;
use crate::world::wrath::CMSG_MOVE_FALL_RESET;
use crate::world::wrath::CMSG_REQUEST_RAID_INFO;
use crate::world::wrath::CMSG_MOVE_TIME_SKIPPED;
use crate::world::wrath::CMSG_MOVE_FEATHER_FALL_ACK;
use crate::world::wrath::CMSG_MOVE_WATER_WALK_ACK;
use crate::world::wrath::CMSG_MOVE_NOT_ACTIVE_MOVER;
use crate::world::wrath::CMSG_BATTLEFIELD_STATUS;
use crate::world::wrath::CMSG_BATTLEFIELD_PORT;
use crate::world::wrath::MSG_INSPECT_HONOR_STATS_Client;
use crate::world::wrath::CMSG_BATTLEMASTER_HELLO;
use crate::world::wrath::CMSG_FORCE_WALK_SPEED_CHANGE_ACK;
use crate::world::wrath::CMSG_FORCE_SWIM_BACK_SPEED_CHANGE_ACK;
use crate::world::wrath::CMSG_FORCE_TURN_RATE_CHANGE_ACK;
use crate::world::wrath::MSG_PVP_LOG_DATA_Client;
use crate::world::wrath::CMSG_LEAVE_BATTLEFIELD;
use crate::world::wrath::CMSG_AREA_SPIRIT_HEALER_QUERY;
use crate::world::wrath::CMSG_AREA_SPIRIT_HEALER_QUEUE;
use crate::world::wrath::MSG_BATTLEGROUND_PLAYER_POSITIONS_Client;
use crate::world::wrath::CMSG_PET_STOP_ATTACK;
use crate::world::wrath::CMSG_BATTLEMASTER_JOIN;
use crate::world::wrath::CMSG_PET_UNLEARN;
use crate::world::wrath::CMSG_PET_SPELL_AUTOCAST;
use crate::world::wrath::CMSG_GUILD_INFO_TEXT;
use crate::world::wrath::CMSG_ACTIVATETAXIEXPRESS;
use crate::world::wrath::CMSG_SET_FACTION_INACTIVE;
use crate::world::wrath::CMSG_SET_WATCHED_FACTION;
use crate::world::wrath::CMSG_RESET_INSTANCES;
use crate::world::wrath::MSG_RAID_READY_CHECK_Client;
use crate::world::wrath::CMSG_GMSURVEY_SUBMIT;
use crate::world::wrath::CMSG_MOVE_SET_FLY;
use crate::world::wrath::CMSG_LFD_PLAYER_LOCK_INFO_REQUEST;
use crate::world::wrath::CMSG_REALM_SPLIT;
use crate::world::wrath::CMSG_MOVE_CHNG_TRANSPORT;
use crate::world::wrath::CMSG_TIME_SYNC_RESP;
use crate::world::wrath::CMSG_VOICE_SESSION_ENABLE;
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
    MSG_QUEST_PUSH_RESULT(MSG_QUEST_PUSH_RESULT),
    MSG_PETITION_RENAME(MSG_PETITION_RENAME),
    MSG_SET_DUNGEON_DIFFICULTY(MSG_SET_DUNGEON_DIFFICULTY),
    MSG_MOVE_START_ASCEND(MSG_MOVE_START_ASCEND),
    MSG_MOVE_STOP_ASCEND(MSG_MOVE_STOP_ASCEND),
    MSG_MOVE_START_DESCEND(MSG_MOVE_START_DESCEND),
    MSG_GUILD_BANK_MONEY_WITHDRAWN(MSG_GUILD_BANK_MONEY_WITHDRAWN),
    CMSG_CALENDAR_GET_NUM_PENDING(CMSG_CALENDAR_GET_NUM_PENDING),
    CMSG_BOOTME(CMSG_BOOTME),
    CMSG_WORLD_TELEPORT(CMSG_WORLD_TELEPORT),
    CMSG_TELEPORT_TO_UNIT(CMSG_TELEPORT_TO_UNIT),
    CMSG_CHAR_CREATE(CMSG_CHAR_CREATE),
    CMSG_CHAR_ENUM(CMSG_CHAR_ENUM),
    CMSG_CHAR_DELETE(CMSG_CHAR_DELETE),
    CMSG_PLAYER_LOGIN(CMSG_PLAYER_LOGIN),
    CMSG_PLAYER_LOGOUT(CMSG_PLAYER_LOGOUT),
    CMSG_LOGOUT_REQUEST(CMSG_LOGOUT_REQUEST),
    CMSG_LOGOUT_CANCEL(CMSG_LOGOUT_CANCEL),
    CMSG_NAME_QUERY(CMSG_NAME_QUERY),
    CMSG_PET_NAME_QUERY(CMSG_PET_NAME_QUERY),
    CMSG_GUILD_QUERY(CMSG_GUILD_QUERY),
    CMSG_ITEM_QUERY_SINGLE(CMSG_ITEM_QUERY_SINGLE),
    CMSG_PAGE_TEXT_QUERY(CMSG_PAGE_TEXT_QUERY),
    CMSG_QUEST_QUERY(CMSG_QUEST_QUERY),
    CMSG_GAMEOBJECT_QUERY(CMSG_GAMEOBJECT_QUERY),
    CMSG_CREATURE_QUERY(CMSG_CREATURE_QUERY),
    CMSG_WHO(CMSG_WHO),
    CMSG_WHOIS(CMSG_WHOIS),
    CMSG_CONTACT_LIST(CMSG_CONTACT_LIST),
    CMSG_ADD_FRIEND(CMSG_ADD_FRIEND),
    CMSG_DEL_FRIEND(CMSG_DEL_FRIEND),
    CMSG_ADD_IGNORE(CMSG_ADD_IGNORE),
    CMSG_DEL_IGNORE(CMSG_DEL_IGNORE),
    CMSG_GROUP_INVITE(CMSG_GROUP_INVITE),
    CMSG_GROUP_ACCEPT(CMSG_GROUP_ACCEPT),
    CMSG_GROUP_DECLINE(CMSG_GROUP_DECLINE),
    CMSG_GROUP_UNINVITE(CMSG_GROUP_UNINVITE),
    CMSG_GROUP_UNINVITE_GUID(CMSG_GROUP_UNINVITE_GUID),
    CMSG_GROUP_SET_LEADER(CMSG_GROUP_SET_LEADER),
    CMSG_LOOT_METHOD(CMSG_LOOT_METHOD),
    CMSG_GROUP_DISBAND(CMSG_GROUP_DISBAND),
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
    CMSG_USE_ITEM(CMSG_USE_ITEM),
    CMSG_OPEN_ITEM(CMSG_OPEN_ITEM),
    CMSG_READ_ITEM(CMSG_READ_ITEM),
    CMSG_GAMEOBJ_USE(CMSG_GAMEOBJ_USE),
    CMSG_AREATRIGGER(CMSG_AREATRIGGER),
    MSG_MOVE_TELEPORT_ACK(MSG_MOVE_TELEPORT_ACK_Client),
    CMSG_MOVE_SET_RAW_POSITION(CMSG_MOVE_SET_RAW_POSITION),
    CMSG_FORCE_RUN_SPEED_CHANGE_ACK(CMSG_FORCE_RUN_SPEED_CHANGE_ACK),
    CMSG_FORCE_RUN_BACK_SPEED_CHANGE_ACK(CMSG_FORCE_RUN_BACK_SPEED_CHANGE_ACK),
    CMSG_FORCE_SWIM_SPEED_CHANGE_ACK(CMSG_FORCE_SWIM_SPEED_CHANGE_ACK),
    CMSG_FORCE_MOVE_ROOT_ACK(CMSG_FORCE_MOVE_ROOT_ACK),
    CMSG_FORCE_MOVE_UNROOT_ACK(CMSG_FORCE_MOVE_UNROOT_ACK),
    CMSG_MOVE_KNOCK_BACK_ACK(CMSG_MOVE_KNOCK_BACK_ACK),
    CMSG_MOVE_HOVER_ACK(CMSG_MOVE_HOVER_ACK),
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
    CMSG_INITIATE_TRADE(CMSG_INITIATE_TRADE),
    CMSG_BEGIN_TRADE(CMSG_BEGIN_TRADE),
    CMSG_BUSY_TRADE(CMSG_BUSY_TRADE),
    CMSG_IGNORE_TRADE(CMSG_IGNORE_TRADE),
    CMSG_ACCEPT_TRADE(CMSG_ACCEPT_TRADE),
    CMSG_UNACCEPT_TRADE(CMSG_UNACCEPT_TRADE),
    CMSG_CANCEL_TRADE(CMSG_CANCEL_TRADE),
    CMSG_SET_TRADE_ITEM(CMSG_SET_TRADE_ITEM),
    CMSG_CLEAR_TRADE_ITEM(CMSG_CLEAR_TRADE_ITEM),
    CMSG_SET_TRADE_GOLD(CMSG_SET_TRADE_GOLD),
    CMSG_SET_FACTION_ATWAR(CMSG_SET_FACTION_ATWAR),
    CMSG_SET_ACTION_BUTTON(CMSG_SET_ACTION_BUTTON),
    CMSG_CAST_SPELL(CMSG_CAST_SPELL),
    CMSG_CANCEL_CAST(CMSG_CANCEL_CAST),
    CMSG_CANCEL_AURA(CMSG_CANCEL_AURA),
    CMSG_CANCEL_CHANNELLING(CMSG_CANCEL_CHANNELLING),
    CMSG_SET_SELECTION(CMSG_SET_SELECTION),
    CMSG_DELETEEQUIPMENT_SET(CMSG_DELETEEQUIPMENT_SET),
    CMSG_ATTACKSWING(CMSG_ATTACKSWING),
    CMSG_ATTACKSTOP(CMSG_ATTACKSTOP),
    CMSG_REPOP_REQUEST(CMSG_REPOP_REQUEST),
    CMSG_RESURRECT_RESPONSE(CMSG_RESURRECT_RESPONSE),
    CMSG_LOOT(CMSG_LOOT),
    CMSG_LOOT_MONEY(CMSG_LOOT_MONEY),
    CMSG_LOOT_RELEASE(CMSG_LOOT_RELEASE),
    CMSG_DUEL_ACCEPTED(CMSG_DUEL_ACCEPTED),
    CMSG_DUEL_CANCELLED(CMSG_DUEL_CANCELLED),
    CMSG_MOUNTSPECIAL_ANIM(CMSG_MOUNTSPECIAL_ANIM),
    CMSG_PET_SET_ACTION(CMSG_PET_SET_ACTION),
    CMSG_PET_ACTION(CMSG_PET_ACTION),
    CMSG_PET_ABANDON(CMSG_PET_ABANDON),
    CMSG_PET_RENAME(CMSG_PET_RENAME),
    CMSG_GOSSIP_HELLO(CMSG_GOSSIP_HELLO),
    CMSG_GOSSIP_SELECT_OPTION(CMSG_GOSSIP_SELECT_OPTION),
    CMSG_NPC_TEXT_QUERY(CMSG_NPC_TEXT_QUERY),
    CMSG_QUESTGIVER_STATUS_QUERY(CMSG_QUESTGIVER_STATUS_QUERY),
    CMSG_QUESTGIVER_HELLO(CMSG_QUESTGIVER_HELLO),
    CMSG_QUESTGIVER_QUERY_QUEST(CMSG_QUESTGIVER_QUERY_QUEST),
    CMSG_QUESTGIVER_QUEST_AUTOLAUNCH(CMSG_QUESTGIVER_QUEST_AUTOLAUNCH),
    CMSG_QUESTGIVER_ACCEPT_QUEST(CMSG_QUESTGIVER_ACCEPT_QUEST),
    CMSG_QUESTGIVER_COMPLETE_QUEST(CMSG_QUESTGIVER_COMPLETE_QUEST),
    CMSG_QUESTGIVER_REQUEST_REWARD(CMSG_QUESTGIVER_REQUEST_REWARD),
    CMSG_QUESTGIVER_CHOOSE_REWARD(CMSG_QUESTGIVER_CHOOSE_REWARD),
    CMSG_QUESTGIVER_CANCEL(CMSG_QUESTGIVER_CANCEL),
    CMSG_QUESTLOG_SWAP_QUEST(CMSG_QUESTLOG_SWAP_QUEST),
    CMSG_QUESTLOG_REMOVE_QUEST(CMSG_QUESTLOG_REMOVE_QUEST),
    CMSG_QUEST_CONFIRM_ACCEPT(CMSG_QUEST_CONFIRM_ACCEPT),
    CMSG_PUSHQUESTTOPARTY(CMSG_PUSHQUESTTOPARTY),
    CMSG_LIST_INVENTORY(CMSG_LIST_INVENTORY),
    CMSG_SELL_ITEM(CMSG_SELL_ITEM),
    CMSG_BUY_ITEM(CMSG_BUY_ITEM),
    CMSG_BUY_ITEM_IN_SLOT(CMSG_BUY_ITEM_IN_SLOT),
    CMSG_TAXINODE_STATUS_QUERY(CMSG_TAXINODE_STATUS_QUERY),
    CMSG_TAXIQUERYAVAILABLENODES(CMSG_TAXIQUERYAVAILABLENODES),
    CMSG_ACTIVATETAXI(CMSG_ACTIVATETAXI),
    CMSG_TRAINER_LIST(CMSG_TRAINER_LIST),
    CMSG_TRAINER_BUY_SPELL(CMSG_TRAINER_BUY_SPELL),
    CMSG_BINDER_ACTIVATE(CMSG_BINDER_ACTIVATE),
    CMSG_BANKER_ACTIVATE(CMSG_BANKER_ACTIVATE),
    CMSG_BUY_BANK_SLOT(CMSG_BUY_BANK_SLOT),
    CMSG_PETITION_SHOWLIST(CMSG_PETITION_SHOWLIST),
    CMSG_PETITION_BUY(CMSG_PETITION_BUY),
    CMSG_PETITION_SHOW_SIGNATURES(CMSG_PETITION_SHOW_SIGNATURES),
    CMSG_PETITION_SIGN(CMSG_PETITION_SIGN),
    CMSG_OFFER_PETITION(CMSG_OFFER_PETITION),
    CMSG_TURN_IN_PETITION(CMSG_TURN_IN_PETITION),
    CMSG_PETITION_QUERY(CMSG_PETITION_QUERY),
    CMSG_BUG(CMSG_BUG),
    CMSG_PLAYED_TIME(CMSG_PLAYED_TIME),
    CMSG_QUERY_TIME(CMSG_QUERY_TIME),
    CMSG_RECLAIM_CORPSE(CMSG_RECLAIM_CORPSE),
    CMSG_WRAP_ITEM(CMSG_WRAP_ITEM),
    MSG_MINIMAP_PING(MSG_MINIMAP_PING_Client),
    CMSG_PING(CMSG_PING),
    CMSG_SETSHEATHED(CMSG_SETSHEATHED),
    CMSG_AUTH_SESSION(CMSG_AUTH_SESSION),
    CMSG_PET_CAST_SPELL(CMSG_PET_CAST_SPELL),
    MSG_SAVE_GUILD_EMBLEM(MSG_SAVE_GUILD_EMBLEM_Client),
    CMSG_ZONEUPDATE(CMSG_ZONEUPDATE),
    MSG_RANDOM_ROLL(MSG_RANDOM_ROLL_Client),
    CMSG_UNLEARN_SKILL(CMSG_UNLEARN_SKILL),
    CMSG_GMTICKET_CREATE(CMSG_GMTICKET_CREATE),
    CMSG_GMTICKET_UPDATETEXT(CMSG_GMTICKET_UPDATETEXT),
    CMSG_REQUEST_ACCOUNT_DATA(CMSG_REQUEST_ACCOUNT_DATA),
    CMSG_UPDATE_ACCOUNT_DATA(CMSG_UPDATE_ACCOUNT_DATA),
    CMSG_GMTICKET_GETTICKET(CMSG_GMTICKET_GETTICKET),
    MSG_CORPSE_QUERY(MSG_CORPSE_QUERY_Client),
    CMSG_GMTICKET_DELETETICKET(CMSG_GMTICKET_DELETETICKET),
    CMSG_GMTICKET_SYSTEMSTATUS(CMSG_GMTICKET_SYSTEMSTATUS),
    CMSG_SPIRIT_HEALER_ACTIVATE(CMSG_SPIRIT_HEALER_ACTIVATE),
    CMSG_CHAT_IGNORED(CMSG_CHAT_IGNORED),
    CMSG_GUILD_RANK(CMSG_GUILD_RANK),
    CMSG_GUILD_ADD_RANK(CMSG_GUILD_ADD_RANK),
    CMSG_GUILD_DEL_RANK(CMSG_GUILD_DEL_RANK),
    CMSG_GUILD_SET_PUBLIC_NOTE(CMSG_GUILD_SET_PUBLIC_NOTE),
    CMSG_GUILD_SET_OFFICER_NOTE(CMSG_GUILD_SET_OFFICER_NOTE),
    CMSG_SEND_MAIL(CMSG_SEND_MAIL),
    CMSG_GET_MAIL_LIST(CMSG_GET_MAIL_LIST),
    CMSG_BATTLEFIELD_LIST(CMSG_BATTLEFIELD_LIST),
    CMSG_ITEM_TEXT_QUERY(CMSG_ITEM_TEXT_QUERY),
    CMSG_MAIL_TAKE_MONEY(CMSG_MAIL_TAKE_MONEY),
    CMSG_MAIL_TAKE_ITEM(CMSG_MAIL_TAKE_ITEM),
    CMSG_MAIL_MARK_AS_READ(CMSG_MAIL_MARK_AS_READ),
    CMSG_MAIL_RETURN_TO_SENDER(CMSG_MAIL_RETURN_TO_SENDER),
    CMSG_MAIL_DELETE(CMSG_MAIL_DELETE),
    CMSG_MAIL_CREATE_TEXT_ITEM(CMSG_MAIL_CREATE_TEXT_ITEM),
    CMSG_LEARN_TALENT(CMSG_LEARN_TALENT),
    CMSG_TOGGLE_PVP(CMSG_TOGGLE_PVP),
    MSG_AUCTION_HELLO(MSG_AUCTION_HELLO_Client),
    CMSG_AUCTION_SELL_ITEM(CMSG_AUCTION_SELL_ITEM),
    CMSG_AUCTION_REMOVE_ITEM(CMSG_AUCTION_REMOVE_ITEM),
    CMSG_AUCTION_LIST_ITEMS(CMSG_AUCTION_LIST_ITEMS),
    CMSG_AUCTION_LIST_OWNER_ITEMS(CMSG_AUCTION_LIST_OWNER_ITEMS),
    CMSG_AUCTION_PLACE_BID(CMSG_AUCTION_PLACE_BID),
    CMSG_AUCTION_LIST_BIDDER_ITEMS(CMSG_AUCTION_LIST_BIDDER_ITEMS),
    CMSG_SET_AMMO(CMSG_SET_AMMO),
    CMSG_SET_ACTIVE_MOVER(CMSG_SET_ACTIVE_MOVER),
    CMSG_PET_CANCEL_AURA(CMSG_PET_CANCEL_AURA),
    CMSG_CANCEL_AUTO_REPEAT_SPELL(CMSG_CANCEL_AUTO_REPEAT_SPELL),
    MSG_LIST_STABLED_PETS(MSG_LIST_STABLED_PETS_Client),
    CMSG_STABLE_PET(CMSG_STABLE_PET),
    CMSG_UNSTABLE_PET(CMSG_UNSTABLE_PET),
    CMSG_BUY_STABLE_SLOT(CMSG_BUY_STABLE_SLOT),
    CMSG_STABLE_SWAP_PET(CMSG_STABLE_SWAP_PET),
    CMSG_REQUEST_PET_INFO(CMSG_REQUEST_PET_INFO),
    CMSG_FAR_SIGHT(CMSG_FAR_SIGHT),
    CMSG_GROUP_CHANGE_SUB_GROUP(CMSG_GROUP_CHANGE_SUB_GROUP),
    CMSG_REQUEST_PARTY_MEMBER_STATS(CMSG_REQUEST_PARTY_MEMBER_STATS),
    CMSG_GROUP_SWAP_SUB_GROUP(CMSG_GROUP_SWAP_SUB_GROUP),
    CMSG_AUTOSTORE_BANK_ITEM(CMSG_AUTOSTORE_BANK_ITEM),
    CMSG_AUTOBANK_ITEM(CMSG_AUTOBANK_ITEM),
    MSG_QUERY_NEXT_MAIL_TIME(MSG_QUERY_NEXT_MAIL_TIME_Client),
    CMSG_GROUP_RAID_CONVERT(CMSG_GROUP_RAID_CONVERT),
    CMSG_GROUP_ASSISTANT_LEADER(CMSG_GROUP_ASSISTANT_LEADER),
    CMSG_BUYBACK_ITEM(CMSG_BUYBACK_ITEM),
    CMSG_LFG_GET_STATUS(CMSG_LFG_GET_STATUS),
    CMSG_CANCEL_GROWTH_AURA(CMSG_CANCEL_GROWTH_AURA),
    CMSG_LOOT_ROLL(CMSG_LOOT_ROLL),
    CMSG_LOOT_MASTER_GIVE(CMSG_LOOT_MASTER_GIVE),
    CMSG_REPAIR_ITEM(CMSG_REPAIR_ITEM),
    CMSG_SUMMON_RESPONSE(CMSG_SUMMON_RESPONSE),
    CMSG_SELF_RES(CMSG_SELF_RES),
    CMSG_TOGGLE_HELM(CMSG_TOGGLE_HELM),
    CMSG_TOGGLE_CLOAK(CMSG_TOGGLE_CLOAK),
    CMSG_SET_ACTIONBAR_TOGGLES(CMSG_SET_ACTIONBAR_TOGGLES),
    CMSG_ITEM_NAME_QUERY(CMSG_ITEM_NAME_QUERY),
    CMSG_CHAR_RENAME(CMSG_CHAR_RENAME),
    CMSG_MOVE_SPLINE_DONE(CMSG_MOVE_SPLINE_DONE),
    CMSG_MOVE_FALL_RESET(CMSG_MOVE_FALL_RESET),
    CMSG_REQUEST_RAID_INFO(CMSG_REQUEST_RAID_INFO),
    CMSG_MOVE_TIME_SKIPPED(CMSG_MOVE_TIME_SKIPPED),
    CMSG_MOVE_FEATHER_FALL_ACK(CMSG_MOVE_FEATHER_FALL_ACK),
    CMSG_MOVE_WATER_WALK_ACK(CMSG_MOVE_WATER_WALK_ACK),
    CMSG_MOVE_NOT_ACTIVE_MOVER(CMSG_MOVE_NOT_ACTIVE_MOVER),
    CMSG_BATTLEFIELD_STATUS(CMSG_BATTLEFIELD_STATUS),
    CMSG_BATTLEFIELD_PORT(CMSG_BATTLEFIELD_PORT),
    MSG_INSPECT_HONOR_STATS(MSG_INSPECT_HONOR_STATS_Client),
    CMSG_BATTLEMASTER_HELLO(CMSG_BATTLEMASTER_HELLO),
    CMSG_FORCE_WALK_SPEED_CHANGE_ACK(CMSG_FORCE_WALK_SPEED_CHANGE_ACK),
    CMSG_FORCE_SWIM_BACK_SPEED_CHANGE_ACK(CMSG_FORCE_SWIM_BACK_SPEED_CHANGE_ACK),
    CMSG_FORCE_TURN_RATE_CHANGE_ACK(CMSG_FORCE_TURN_RATE_CHANGE_ACK),
    MSG_PVP_LOG_DATA(MSG_PVP_LOG_DATA_Client),
    CMSG_LEAVE_BATTLEFIELD(CMSG_LEAVE_BATTLEFIELD),
    CMSG_AREA_SPIRIT_HEALER_QUERY(CMSG_AREA_SPIRIT_HEALER_QUERY),
    CMSG_AREA_SPIRIT_HEALER_QUEUE(CMSG_AREA_SPIRIT_HEALER_QUEUE),
    MSG_BATTLEGROUND_PLAYER_POSITIONS(MSG_BATTLEGROUND_PLAYER_POSITIONS_Client),
    CMSG_PET_STOP_ATTACK(CMSG_PET_STOP_ATTACK),
    CMSG_BATTLEMASTER_JOIN(CMSG_BATTLEMASTER_JOIN),
    CMSG_PET_UNLEARN(CMSG_PET_UNLEARN),
    CMSG_PET_SPELL_AUTOCAST(CMSG_PET_SPELL_AUTOCAST),
    CMSG_GUILD_INFO_TEXT(CMSG_GUILD_INFO_TEXT),
    CMSG_ACTIVATETAXIEXPRESS(CMSG_ACTIVATETAXIEXPRESS),
    CMSG_SET_FACTION_INACTIVE(CMSG_SET_FACTION_INACTIVE),
    CMSG_SET_WATCHED_FACTION(CMSG_SET_WATCHED_FACTION),
    CMSG_RESET_INSTANCES(CMSG_RESET_INSTANCES),
    MSG_RAID_READY_CHECK(MSG_RAID_READY_CHECK_Client),
    CMSG_GMSURVEY_SUBMIT(CMSG_GMSURVEY_SUBMIT),
    CMSG_MOVE_SET_FLY(CMSG_MOVE_SET_FLY),
    CMSG_LFD_PLAYER_LOCK_INFO_REQUEST(CMSG_LFD_PLAYER_LOCK_INFO_REQUEST),
    CMSG_REALM_SPLIT(CMSG_REALM_SPLIT),
    CMSG_MOVE_CHNG_TRANSPORT(CMSG_MOVE_CHNG_TRANSPORT),
    CMSG_TIME_SYNC_RESP(CMSG_TIME_SYNC_RESP),
    CMSG_VOICE_SESSION_ENABLE(CMSG_VOICE_SESSION_ENABLE),
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
            0x0276 => Ok(Self::MSG_QUEST_PUSH_RESULT(<MSG_QUEST_PUSH_RESULT as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x0276, size: body_size, io, } } else { a } })?)),
            0x02C1 => Ok(Self::MSG_PETITION_RENAME(<MSG_PETITION_RENAME as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x02C1, size: body_size, io, } } else { a } })?)),
            0x0329 => Ok(Self::MSG_SET_DUNGEON_DIFFICULTY(<MSG_SET_DUNGEON_DIFFICULTY as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x0329, size: body_size, io, } } else { a } })?)),
            0x0359 => Ok(Self::MSG_MOVE_START_ASCEND(<MSG_MOVE_START_ASCEND as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x0359, size: body_size, io, } } else { a } })?)),
            0x035A => Ok(Self::MSG_MOVE_STOP_ASCEND(<MSG_MOVE_STOP_ASCEND as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x035A, size: body_size, io, } } else { a } })?)),
            0x03A7 => Ok(Self::MSG_MOVE_START_DESCEND(<MSG_MOVE_START_DESCEND as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x03A7, size: body_size, io, } } else { a } })?)),
            0x03FE => Ok(Self::MSG_GUILD_BANK_MONEY_WITHDRAWN(<MSG_GUILD_BANK_MONEY_WITHDRAWN as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x03FE, size: body_size, io, } } else { a } })?)),
            0x0447 => Ok(Self::CMSG_CALENDAR_GET_NUM_PENDING(<CMSG_CALENDAR_GET_NUM_PENDING as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x0447, size: body_size, io, } } else { a } })?)),
            0x0001 => Ok(Self::CMSG_BOOTME(<CMSG_BOOTME as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x0001, size: body_size, io, } } else { a } })?)),
            0x0008 => Ok(Self::CMSG_WORLD_TELEPORT(<CMSG_WORLD_TELEPORT as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x0008, size: body_size, io, } } else { a } })?)),
            0x0009 => Ok(Self::CMSG_TELEPORT_TO_UNIT(<CMSG_TELEPORT_TO_UNIT as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x0009, size: body_size, io, } } else { a } })?)),
            0x0036 => Ok(Self::CMSG_CHAR_CREATE(<CMSG_CHAR_CREATE as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x0036, size: body_size, io, } } else { a } })?)),
            0x0037 => Ok(Self::CMSG_CHAR_ENUM(<CMSG_CHAR_ENUM as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x0037, size: body_size, io, } } else { a } })?)),
            0x0038 => Ok(Self::CMSG_CHAR_DELETE(<CMSG_CHAR_DELETE as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x0038, size: body_size, io, } } else { a } })?)),
            0x003D => Ok(Self::CMSG_PLAYER_LOGIN(<CMSG_PLAYER_LOGIN as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x003D, size: body_size, io, } } else { a } })?)),
            0x004A => Ok(Self::CMSG_PLAYER_LOGOUT(<CMSG_PLAYER_LOGOUT as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x004A, size: body_size, io, } } else { a } })?)),
            0x004B => Ok(Self::CMSG_LOGOUT_REQUEST(<CMSG_LOGOUT_REQUEST as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x004B, size: body_size, io, } } else { a } })?)),
            0x004E => Ok(Self::CMSG_LOGOUT_CANCEL(<CMSG_LOGOUT_CANCEL as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x004E, size: body_size, io, } } else { a } })?)),
            0x0050 => Ok(Self::CMSG_NAME_QUERY(<CMSG_NAME_QUERY as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x0050, size: body_size, io, } } else { a } })?)),
            0x0052 => Ok(Self::CMSG_PET_NAME_QUERY(<CMSG_PET_NAME_QUERY as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x0052, size: body_size, io, } } else { a } })?)),
            0x0054 => Ok(Self::CMSG_GUILD_QUERY(<CMSG_GUILD_QUERY as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x0054, size: body_size, io, } } else { a } })?)),
            0x0056 => Ok(Self::CMSG_ITEM_QUERY_SINGLE(<CMSG_ITEM_QUERY_SINGLE as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x0056, size: body_size, io, } } else { a } })?)),
            0x005A => Ok(Self::CMSG_PAGE_TEXT_QUERY(<CMSG_PAGE_TEXT_QUERY as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x005A, size: body_size, io, } } else { a } })?)),
            0x005C => Ok(Self::CMSG_QUEST_QUERY(<CMSG_QUEST_QUERY as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x005C, size: body_size, io, } } else { a } })?)),
            0x005E => Ok(Self::CMSG_GAMEOBJECT_QUERY(<CMSG_GAMEOBJECT_QUERY as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x005E, size: body_size, io, } } else { a } })?)),
            0x0060 => Ok(Self::CMSG_CREATURE_QUERY(<CMSG_CREATURE_QUERY as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x0060, size: body_size, io, } } else { a } })?)),
            0x0062 => Ok(Self::CMSG_WHO(<CMSG_WHO as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x0062, size: body_size, io, } } else { a } })?)),
            0x0064 => Ok(Self::CMSG_WHOIS(<CMSG_WHOIS as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x0064, size: body_size, io, } } else { a } })?)),
            0x0066 => Ok(Self::CMSG_CONTACT_LIST(<CMSG_CONTACT_LIST as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x0066, size: body_size, io, } } else { a } })?)),
            0x0069 => Ok(Self::CMSG_ADD_FRIEND(<CMSG_ADD_FRIEND as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x0069, size: body_size, io, } } else { a } })?)),
            0x006A => Ok(Self::CMSG_DEL_FRIEND(<CMSG_DEL_FRIEND as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x006A, size: body_size, io, } } else { a } })?)),
            0x006C => Ok(Self::CMSG_ADD_IGNORE(<CMSG_ADD_IGNORE as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x006C, size: body_size, io, } } else { a } })?)),
            0x006D => Ok(Self::CMSG_DEL_IGNORE(<CMSG_DEL_IGNORE as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x006D, size: body_size, io, } } else { a } })?)),
            0x006E => Ok(Self::CMSG_GROUP_INVITE(<CMSG_GROUP_INVITE as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x006E, size: body_size, io, } } else { a } })?)),
            0x0072 => Ok(Self::CMSG_GROUP_ACCEPT(<CMSG_GROUP_ACCEPT as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x0072, size: body_size, io, } } else { a } })?)),
            0x0073 => Ok(Self::CMSG_GROUP_DECLINE(<CMSG_GROUP_DECLINE as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x0073, size: body_size, io, } } else { a } })?)),
            0x0075 => Ok(Self::CMSG_GROUP_UNINVITE(<CMSG_GROUP_UNINVITE as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x0075, size: body_size, io, } } else { a } })?)),
            0x0076 => Ok(Self::CMSG_GROUP_UNINVITE_GUID(<CMSG_GROUP_UNINVITE_GUID as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x0076, size: body_size, io, } } else { a } })?)),
            0x0078 => Ok(Self::CMSG_GROUP_SET_LEADER(<CMSG_GROUP_SET_LEADER as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x0078, size: body_size, io, } } else { a } })?)),
            0x007A => Ok(Self::CMSG_LOOT_METHOD(<CMSG_LOOT_METHOD as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x007A, size: body_size, io, } } else { a } })?)),
            0x007B => Ok(Self::CMSG_GROUP_DISBAND(<CMSG_GROUP_DISBAND as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x007B, size: body_size, io, } } else { a } })?)),
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
            0x00AB => Ok(Self::CMSG_USE_ITEM(<CMSG_USE_ITEM as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x00AB, size: body_size, io, } } else { a } })?)),
            0x00AC => Ok(Self::CMSG_OPEN_ITEM(<CMSG_OPEN_ITEM as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x00AC, size: body_size, io, } } else { a } })?)),
            0x00AD => Ok(Self::CMSG_READ_ITEM(<CMSG_READ_ITEM as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x00AD, size: body_size, io, } } else { a } })?)),
            0x00B1 => Ok(Self::CMSG_GAMEOBJ_USE(<CMSG_GAMEOBJ_USE as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x00B1, size: body_size, io, } } else { a } })?)),
            0x00B4 => Ok(Self::CMSG_AREATRIGGER(<CMSG_AREATRIGGER as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x00B4, size: body_size, io, } } else { a } })?)),
            0x00C7 => Ok(Self::MSG_MOVE_TELEPORT_ACK(<MSG_MOVE_TELEPORT_ACK_Client as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x00C7, size: body_size, io, } } else { a } })?)),
            0x00E1 => Ok(Self::CMSG_MOVE_SET_RAW_POSITION(<CMSG_MOVE_SET_RAW_POSITION as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x00E1, size: body_size, io, } } else { a } })?)),
            0x00E3 => Ok(Self::CMSG_FORCE_RUN_SPEED_CHANGE_ACK(<CMSG_FORCE_RUN_SPEED_CHANGE_ACK as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x00E3, size: body_size, io, } } else { a } })?)),
            0x00E5 => Ok(Self::CMSG_FORCE_RUN_BACK_SPEED_CHANGE_ACK(<CMSG_FORCE_RUN_BACK_SPEED_CHANGE_ACK as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x00E5, size: body_size, io, } } else { a } })?)),
            0x00E7 => Ok(Self::CMSG_FORCE_SWIM_SPEED_CHANGE_ACK(<CMSG_FORCE_SWIM_SPEED_CHANGE_ACK as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x00E7, size: body_size, io, } } else { a } })?)),
            0x00E9 => Ok(Self::CMSG_FORCE_MOVE_ROOT_ACK(<CMSG_FORCE_MOVE_ROOT_ACK as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x00E9, size: body_size, io, } } else { a } })?)),
            0x00EB => Ok(Self::CMSG_FORCE_MOVE_UNROOT_ACK(<CMSG_FORCE_MOVE_UNROOT_ACK as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x00EB, size: body_size, io, } } else { a } })?)),
            0x00F0 => Ok(Self::CMSG_MOVE_KNOCK_BACK_ACK(<CMSG_MOVE_KNOCK_BACK_ACK as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x00F0, size: body_size, io, } } else { a } })?)),
            0x00F6 => Ok(Self::CMSG_MOVE_HOVER_ACK(<CMSG_MOVE_HOVER_ACK as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x00F6, size: body_size, io, } } else { a } })?)),
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
            0x0116 => Ok(Self::CMSG_INITIATE_TRADE(<CMSG_INITIATE_TRADE as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x0116, size: body_size, io, } } else { a } })?)),
            0x0117 => Ok(Self::CMSG_BEGIN_TRADE(<CMSG_BEGIN_TRADE as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x0117, size: body_size, io, } } else { a } })?)),
            0x0118 => Ok(Self::CMSG_BUSY_TRADE(<CMSG_BUSY_TRADE as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x0118, size: body_size, io, } } else { a } })?)),
            0x0119 => Ok(Self::CMSG_IGNORE_TRADE(<CMSG_IGNORE_TRADE as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x0119, size: body_size, io, } } else { a } })?)),
            0x011A => Ok(Self::CMSG_ACCEPT_TRADE(<CMSG_ACCEPT_TRADE as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x011A, size: body_size, io, } } else { a } })?)),
            0x011B => Ok(Self::CMSG_UNACCEPT_TRADE(<CMSG_UNACCEPT_TRADE as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x011B, size: body_size, io, } } else { a } })?)),
            0x011C => Ok(Self::CMSG_CANCEL_TRADE(<CMSG_CANCEL_TRADE as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x011C, size: body_size, io, } } else { a } })?)),
            0x011D => Ok(Self::CMSG_SET_TRADE_ITEM(<CMSG_SET_TRADE_ITEM as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x011D, size: body_size, io, } } else { a } })?)),
            0x011E => Ok(Self::CMSG_CLEAR_TRADE_ITEM(<CMSG_CLEAR_TRADE_ITEM as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x011E, size: body_size, io, } } else { a } })?)),
            0x011F => Ok(Self::CMSG_SET_TRADE_GOLD(<CMSG_SET_TRADE_GOLD as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x011F, size: body_size, io, } } else { a } })?)),
            0x0125 => Ok(Self::CMSG_SET_FACTION_ATWAR(<CMSG_SET_FACTION_ATWAR as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x0125, size: body_size, io, } } else { a } })?)),
            0x0128 => Ok(Self::CMSG_SET_ACTION_BUTTON(<CMSG_SET_ACTION_BUTTON as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x0128, size: body_size, io, } } else { a } })?)),
            0x012E => Ok(Self::CMSG_CAST_SPELL(<CMSG_CAST_SPELL as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x012E, size: body_size, io, } } else { a } })?)),
            0x012F => Ok(Self::CMSG_CANCEL_CAST(<CMSG_CANCEL_CAST as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x012F, size: body_size, io, } } else { a } })?)),
            0x0136 => Ok(Self::CMSG_CANCEL_AURA(<CMSG_CANCEL_AURA as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x0136, size: body_size, io, } } else { a } })?)),
            0x013B => Ok(Self::CMSG_CANCEL_CHANNELLING(<CMSG_CANCEL_CHANNELLING as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x013B, size: body_size, io, } } else { a } })?)),
            0x013D => Ok(Self::CMSG_SET_SELECTION(<CMSG_SET_SELECTION as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x013D, size: body_size, io, } } else { a } })?)),
            0x013E => Ok(Self::CMSG_DELETEEQUIPMENT_SET(<CMSG_DELETEEQUIPMENT_SET as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x013E, size: body_size, io, } } else { a } })?)),
            0x0141 => Ok(Self::CMSG_ATTACKSWING(<CMSG_ATTACKSWING as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x0141, size: body_size, io, } } else { a } })?)),
            0x0142 => Ok(Self::CMSG_ATTACKSTOP(<CMSG_ATTACKSTOP as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x0142, size: body_size, io, } } else { a } })?)),
            0x015A => Ok(Self::CMSG_REPOP_REQUEST(<CMSG_REPOP_REQUEST as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x015A, size: body_size, io, } } else { a } })?)),
            0x015C => Ok(Self::CMSG_RESURRECT_RESPONSE(<CMSG_RESURRECT_RESPONSE as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x015C, size: body_size, io, } } else { a } })?)),
            0x015D => Ok(Self::CMSG_LOOT(<CMSG_LOOT as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x015D, size: body_size, io, } } else { a } })?)),
            0x015E => Ok(Self::CMSG_LOOT_MONEY(<CMSG_LOOT_MONEY as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x015E, size: body_size, io, } } else { a } })?)),
            0x015F => Ok(Self::CMSG_LOOT_RELEASE(<CMSG_LOOT_RELEASE as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x015F, size: body_size, io, } } else { a } })?)),
            0x016C => Ok(Self::CMSG_DUEL_ACCEPTED(<CMSG_DUEL_ACCEPTED as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x016C, size: body_size, io, } } else { a } })?)),
            0x016D => Ok(Self::CMSG_DUEL_CANCELLED(<CMSG_DUEL_CANCELLED as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x016D, size: body_size, io, } } else { a } })?)),
            0x0171 => Ok(Self::CMSG_MOUNTSPECIAL_ANIM(<CMSG_MOUNTSPECIAL_ANIM as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x0171, size: body_size, io, } } else { a } })?)),
            0x0174 => Ok(Self::CMSG_PET_SET_ACTION(<CMSG_PET_SET_ACTION as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x0174, size: body_size, io, } } else { a } })?)),
            0x0175 => Ok(Self::CMSG_PET_ACTION(<CMSG_PET_ACTION as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x0175, size: body_size, io, } } else { a } })?)),
            0x0176 => Ok(Self::CMSG_PET_ABANDON(<CMSG_PET_ABANDON as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x0176, size: body_size, io, } } else { a } })?)),
            0x0177 => Ok(Self::CMSG_PET_RENAME(<CMSG_PET_RENAME as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x0177, size: body_size, io, } } else { a } })?)),
            0x017B => Ok(Self::CMSG_GOSSIP_HELLO(<CMSG_GOSSIP_HELLO as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x017B, size: body_size, io, } } else { a } })?)),
            0x017C => Ok(Self::CMSG_GOSSIP_SELECT_OPTION(<CMSG_GOSSIP_SELECT_OPTION as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x017C, size: body_size, io, } } else { a } })?)),
            0x017F => Ok(Self::CMSG_NPC_TEXT_QUERY(<CMSG_NPC_TEXT_QUERY as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x017F, size: body_size, io, } } else { a } })?)),
            0x0182 => Ok(Self::CMSG_QUESTGIVER_STATUS_QUERY(<CMSG_QUESTGIVER_STATUS_QUERY as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x0182, size: body_size, io, } } else { a } })?)),
            0x0184 => Ok(Self::CMSG_QUESTGIVER_HELLO(<CMSG_QUESTGIVER_HELLO as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x0184, size: body_size, io, } } else { a } })?)),
            0x0186 => Ok(Self::CMSG_QUESTGIVER_QUERY_QUEST(<CMSG_QUESTGIVER_QUERY_QUEST as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x0186, size: body_size, io, } } else { a } })?)),
            0x0187 => Ok(Self::CMSG_QUESTGIVER_QUEST_AUTOLAUNCH(<CMSG_QUESTGIVER_QUEST_AUTOLAUNCH as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x0187, size: body_size, io, } } else { a } })?)),
            0x0189 => Ok(Self::CMSG_QUESTGIVER_ACCEPT_QUEST(<CMSG_QUESTGIVER_ACCEPT_QUEST as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x0189, size: body_size, io, } } else { a } })?)),
            0x018A => Ok(Self::CMSG_QUESTGIVER_COMPLETE_QUEST(<CMSG_QUESTGIVER_COMPLETE_QUEST as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x018A, size: body_size, io, } } else { a } })?)),
            0x018C => Ok(Self::CMSG_QUESTGIVER_REQUEST_REWARD(<CMSG_QUESTGIVER_REQUEST_REWARD as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x018C, size: body_size, io, } } else { a } })?)),
            0x018E => Ok(Self::CMSG_QUESTGIVER_CHOOSE_REWARD(<CMSG_QUESTGIVER_CHOOSE_REWARD as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x018E, size: body_size, io, } } else { a } })?)),
            0x0190 => Ok(Self::CMSG_QUESTGIVER_CANCEL(<CMSG_QUESTGIVER_CANCEL as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x0190, size: body_size, io, } } else { a } })?)),
            0x0193 => Ok(Self::CMSG_QUESTLOG_SWAP_QUEST(<CMSG_QUESTLOG_SWAP_QUEST as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x0193, size: body_size, io, } } else { a } })?)),
            0x0194 => Ok(Self::CMSG_QUESTLOG_REMOVE_QUEST(<CMSG_QUESTLOG_REMOVE_QUEST as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x0194, size: body_size, io, } } else { a } })?)),
            0x019B => Ok(Self::CMSG_QUEST_CONFIRM_ACCEPT(<CMSG_QUEST_CONFIRM_ACCEPT as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x019B, size: body_size, io, } } else { a } })?)),
            0x019D => Ok(Self::CMSG_PUSHQUESTTOPARTY(<CMSG_PUSHQUESTTOPARTY as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x019D, size: body_size, io, } } else { a } })?)),
            0x019E => Ok(Self::CMSG_LIST_INVENTORY(<CMSG_LIST_INVENTORY as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x019E, size: body_size, io, } } else { a } })?)),
            0x01A0 => Ok(Self::CMSG_SELL_ITEM(<CMSG_SELL_ITEM as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x01A0, size: body_size, io, } } else { a } })?)),
            0x01A2 => Ok(Self::CMSG_BUY_ITEM(<CMSG_BUY_ITEM as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x01A2, size: body_size, io, } } else { a } })?)),
            0x01A3 => Ok(Self::CMSG_BUY_ITEM_IN_SLOT(<CMSG_BUY_ITEM_IN_SLOT as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x01A3, size: body_size, io, } } else { a } })?)),
            0x01AA => Ok(Self::CMSG_TAXINODE_STATUS_QUERY(<CMSG_TAXINODE_STATUS_QUERY as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x01AA, size: body_size, io, } } else { a } })?)),
            0x01AC => Ok(Self::CMSG_TAXIQUERYAVAILABLENODES(<CMSG_TAXIQUERYAVAILABLENODES as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x01AC, size: body_size, io, } } else { a } })?)),
            0x01AD => Ok(Self::CMSG_ACTIVATETAXI(<CMSG_ACTIVATETAXI as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x01AD, size: body_size, io, } } else { a } })?)),
            0x01B0 => Ok(Self::CMSG_TRAINER_LIST(<CMSG_TRAINER_LIST as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x01B0, size: body_size, io, } } else { a } })?)),
            0x01B2 => Ok(Self::CMSG_TRAINER_BUY_SPELL(<CMSG_TRAINER_BUY_SPELL as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x01B2, size: body_size, io, } } else { a } })?)),
            0x01B5 => Ok(Self::CMSG_BINDER_ACTIVATE(<CMSG_BINDER_ACTIVATE as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x01B5, size: body_size, io, } } else { a } })?)),
            0x01B7 => Ok(Self::CMSG_BANKER_ACTIVATE(<CMSG_BANKER_ACTIVATE as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x01B7, size: body_size, io, } } else { a } })?)),
            0x01B9 => Ok(Self::CMSG_BUY_BANK_SLOT(<CMSG_BUY_BANK_SLOT as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x01B9, size: body_size, io, } } else { a } })?)),
            0x01BB => Ok(Self::CMSG_PETITION_SHOWLIST(<CMSG_PETITION_SHOWLIST as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x01BB, size: body_size, io, } } else { a } })?)),
            0x01BD => Ok(Self::CMSG_PETITION_BUY(<CMSG_PETITION_BUY as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x01BD, size: body_size, io, } } else { a } })?)),
            0x01BE => Ok(Self::CMSG_PETITION_SHOW_SIGNATURES(<CMSG_PETITION_SHOW_SIGNATURES as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x01BE, size: body_size, io, } } else { a } })?)),
            0x01C0 => Ok(Self::CMSG_PETITION_SIGN(<CMSG_PETITION_SIGN as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x01C0, size: body_size, io, } } else { a } })?)),
            0x01C3 => Ok(Self::CMSG_OFFER_PETITION(<CMSG_OFFER_PETITION as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x01C3, size: body_size, io, } } else { a } })?)),
            0x01C4 => Ok(Self::CMSG_TURN_IN_PETITION(<CMSG_TURN_IN_PETITION as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x01C4, size: body_size, io, } } else { a } })?)),
            0x01C6 => Ok(Self::CMSG_PETITION_QUERY(<CMSG_PETITION_QUERY as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x01C6, size: body_size, io, } } else { a } })?)),
            0x01CA => Ok(Self::CMSG_BUG(<CMSG_BUG as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x01CA, size: body_size, io, } } else { a } })?)),
            0x01CC => Ok(Self::CMSG_PLAYED_TIME(<CMSG_PLAYED_TIME as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x01CC, size: body_size, io, } } else { a } })?)),
            0x01CE => Ok(Self::CMSG_QUERY_TIME(<CMSG_QUERY_TIME as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x01CE, size: body_size, io, } } else { a } })?)),
            0x01D2 => Ok(Self::CMSG_RECLAIM_CORPSE(<CMSG_RECLAIM_CORPSE as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x01D2, size: body_size, io, } } else { a } })?)),
            0x01D3 => Ok(Self::CMSG_WRAP_ITEM(<CMSG_WRAP_ITEM as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x01D3, size: body_size, io, } } else { a } })?)),
            0x01D5 => Ok(Self::MSG_MINIMAP_PING(<MSG_MINIMAP_PING_Client as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x01D5, size: body_size, io, } } else { a } })?)),
            0x01DC => Ok(Self::CMSG_PING(<CMSG_PING as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x01DC, size: body_size, io, } } else { a } })?)),
            0x01E0 => Ok(Self::CMSG_SETSHEATHED(<CMSG_SETSHEATHED as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x01E0, size: body_size, io, } } else { a } })?)),
            0x01ED => Ok(Self::CMSG_AUTH_SESSION(<CMSG_AUTH_SESSION as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x01ED, size: body_size, io, } } else { a } })?)),
            0x01F0 => Ok(Self::CMSG_PET_CAST_SPELL(<CMSG_PET_CAST_SPELL as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x01F0, size: body_size, io, } } else { a } })?)),
            0x01F1 => Ok(Self::MSG_SAVE_GUILD_EMBLEM(<MSG_SAVE_GUILD_EMBLEM_Client as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x01F1, size: body_size, io, } } else { a } })?)),
            0x01F4 => Ok(Self::CMSG_ZONEUPDATE(<CMSG_ZONEUPDATE as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x01F4, size: body_size, io, } } else { a } })?)),
            0x01FB => Ok(Self::MSG_RANDOM_ROLL(<MSG_RANDOM_ROLL_Client as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x01FB, size: body_size, io, } } else { a } })?)),
            0x0202 => Ok(Self::CMSG_UNLEARN_SKILL(<CMSG_UNLEARN_SKILL as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x0202, size: body_size, io, } } else { a } })?)),
            0x0205 => Ok(Self::CMSG_GMTICKET_CREATE(<CMSG_GMTICKET_CREATE as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x0205, size: body_size, io, } } else { a } })?)),
            0x0207 => Ok(Self::CMSG_GMTICKET_UPDATETEXT(<CMSG_GMTICKET_UPDATETEXT as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x0207, size: body_size, io, } } else { a } })?)),
            0x020A => Ok(Self::CMSG_REQUEST_ACCOUNT_DATA(<CMSG_REQUEST_ACCOUNT_DATA as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x020A, size: body_size, io, } } else { a } })?)),
            0x020B => Ok(Self::CMSG_UPDATE_ACCOUNT_DATA(<CMSG_UPDATE_ACCOUNT_DATA as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x020B, size: body_size, io, } } else { a } })?)),
            0x0211 => Ok(Self::CMSG_GMTICKET_GETTICKET(<CMSG_GMTICKET_GETTICKET as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x0211, size: body_size, io, } } else { a } })?)),
            0x0216 => Ok(Self::MSG_CORPSE_QUERY(<MSG_CORPSE_QUERY_Client as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x0216, size: body_size, io, } } else { a } })?)),
            0x0217 => Ok(Self::CMSG_GMTICKET_DELETETICKET(<CMSG_GMTICKET_DELETETICKET as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x0217, size: body_size, io, } } else { a } })?)),
            0x021A => Ok(Self::CMSG_GMTICKET_SYSTEMSTATUS(<CMSG_GMTICKET_SYSTEMSTATUS as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x021A, size: body_size, io, } } else { a } })?)),
            0x021C => Ok(Self::CMSG_SPIRIT_HEALER_ACTIVATE(<CMSG_SPIRIT_HEALER_ACTIVATE as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x021C, size: body_size, io, } } else { a } })?)),
            0x0225 => Ok(Self::CMSG_CHAT_IGNORED(<CMSG_CHAT_IGNORED as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x0225, size: body_size, io, } } else { a } })?)),
            0x0231 => Ok(Self::CMSG_GUILD_RANK(<CMSG_GUILD_RANK as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x0231, size: body_size, io, } } else { a } })?)),
            0x0232 => Ok(Self::CMSG_GUILD_ADD_RANK(<CMSG_GUILD_ADD_RANK as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x0232, size: body_size, io, } } else { a } })?)),
            0x0233 => Ok(Self::CMSG_GUILD_DEL_RANK(<CMSG_GUILD_DEL_RANK as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x0233, size: body_size, io, } } else { a } })?)),
            0x0234 => Ok(Self::CMSG_GUILD_SET_PUBLIC_NOTE(<CMSG_GUILD_SET_PUBLIC_NOTE as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x0234, size: body_size, io, } } else { a } })?)),
            0x0235 => Ok(Self::CMSG_GUILD_SET_OFFICER_NOTE(<CMSG_GUILD_SET_OFFICER_NOTE as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x0235, size: body_size, io, } } else { a } })?)),
            0x0238 => Ok(Self::CMSG_SEND_MAIL(<CMSG_SEND_MAIL as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x0238, size: body_size, io, } } else { a } })?)),
            0x023A => Ok(Self::CMSG_GET_MAIL_LIST(<CMSG_GET_MAIL_LIST as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x023A, size: body_size, io, } } else { a } })?)),
            0x023C => Ok(Self::CMSG_BATTLEFIELD_LIST(<CMSG_BATTLEFIELD_LIST as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x023C, size: body_size, io, } } else { a } })?)),
            0x0243 => Ok(Self::CMSG_ITEM_TEXT_QUERY(<CMSG_ITEM_TEXT_QUERY as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x0243, size: body_size, io, } } else { a } })?)),
            0x0245 => Ok(Self::CMSG_MAIL_TAKE_MONEY(<CMSG_MAIL_TAKE_MONEY as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x0245, size: body_size, io, } } else { a } })?)),
            0x0246 => Ok(Self::CMSG_MAIL_TAKE_ITEM(<CMSG_MAIL_TAKE_ITEM as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x0246, size: body_size, io, } } else { a } })?)),
            0x0247 => Ok(Self::CMSG_MAIL_MARK_AS_READ(<CMSG_MAIL_MARK_AS_READ as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x0247, size: body_size, io, } } else { a } })?)),
            0x0248 => Ok(Self::CMSG_MAIL_RETURN_TO_SENDER(<CMSG_MAIL_RETURN_TO_SENDER as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x0248, size: body_size, io, } } else { a } })?)),
            0x0249 => Ok(Self::CMSG_MAIL_DELETE(<CMSG_MAIL_DELETE as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x0249, size: body_size, io, } } else { a } })?)),
            0x024A => Ok(Self::CMSG_MAIL_CREATE_TEXT_ITEM(<CMSG_MAIL_CREATE_TEXT_ITEM as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x024A, size: body_size, io, } } else { a } })?)),
            0x0251 => Ok(Self::CMSG_LEARN_TALENT(<CMSG_LEARN_TALENT as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x0251, size: body_size, io, } } else { a } })?)),
            0x0253 => Ok(Self::CMSG_TOGGLE_PVP(<CMSG_TOGGLE_PVP as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x0253, size: body_size, io, } } else { a } })?)),
            0x0255 => Ok(Self::MSG_AUCTION_HELLO(<MSG_AUCTION_HELLO_Client as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x0255, size: body_size, io, } } else { a } })?)),
            0x0256 => Ok(Self::CMSG_AUCTION_SELL_ITEM(<CMSG_AUCTION_SELL_ITEM as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x0256, size: body_size, io, } } else { a } })?)),
            0x0257 => Ok(Self::CMSG_AUCTION_REMOVE_ITEM(<CMSG_AUCTION_REMOVE_ITEM as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x0257, size: body_size, io, } } else { a } })?)),
            0x0258 => Ok(Self::CMSG_AUCTION_LIST_ITEMS(<CMSG_AUCTION_LIST_ITEMS as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x0258, size: body_size, io, } } else { a } })?)),
            0x0259 => Ok(Self::CMSG_AUCTION_LIST_OWNER_ITEMS(<CMSG_AUCTION_LIST_OWNER_ITEMS as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x0259, size: body_size, io, } } else { a } })?)),
            0x025A => Ok(Self::CMSG_AUCTION_PLACE_BID(<CMSG_AUCTION_PLACE_BID as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x025A, size: body_size, io, } } else { a } })?)),
            0x0264 => Ok(Self::CMSG_AUCTION_LIST_BIDDER_ITEMS(<CMSG_AUCTION_LIST_BIDDER_ITEMS as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x0264, size: body_size, io, } } else { a } })?)),
            0x0268 => Ok(Self::CMSG_SET_AMMO(<CMSG_SET_AMMO as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x0268, size: body_size, io, } } else { a } })?)),
            0x026A => Ok(Self::CMSG_SET_ACTIVE_MOVER(<CMSG_SET_ACTIVE_MOVER as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x026A, size: body_size, io, } } else { a } })?)),
            0x026B => Ok(Self::CMSG_PET_CANCEL_AURA(<CMSG_PET_CANCEL_AURA as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x026B, size: body_size, io, } } else { a } })?)),
            0x026D => Ok(Self::CMSG_CANCEL_AUTO_REPEAT_SPELL(<CMSG_CANCEL_AUTO_REPEAT_SPELL as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x026D, size: body_size, io, } } else { a } })?)),
            0x026F => Ok(Self::MSG_LIST_STABLED_PETS(<MSG_LIST_STABLED_PETS_Client as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x026F, size: body_size, io, } } else { a } })?)),
            0x0270 => Ok(Self::CMSG_STABLE_PET(<CMSG_STABLE_PET as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x0270, size: body_size, io, } } else { a } })?)),
            0x0271 => Ok(Self::CMSG_UNSTABLE_PET(<CMSG_UNSTABLE_PET as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x0271, size: body_size, io, } } else { a } })?)),
            0x0272 => Ok(Self::CMSG_BUY_STABLE_SLOT(<CMSG_BUY_STABLE_SLOT as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x0272, size: body_size, io, } } else { a } })?)),
            0x0275 => Ok(Self::CMSG_STABLE_SWAP_PET(<CMSG_STABLE_SWAP_PET as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x0275, size: body_size, io, } } else { a } })?)),
            0x0279 => Ok(Self::CMSG_REQUEST_PET_INFO(<CMSG_REQUEST_PET_INFO as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x0279, size: body_size, io, } } else { a } })?)),
            0x027A => Ok(Self::CMSG_FAR_SIGHT(<CMSG_FAR_SIGHT as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x027A, size: body_size, io, } } else { a } })?)),
            0x027E => Ok(Self::CMSG_GROUP_CHANGE_SUB_GROUP(<CMSG_GROUP_CHANGE_SUB_GROUP as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x027E, size: body_size, io, } } else { a } })?)),
            0x027F => Ok(Self::CMSG_REQUEST_PARTY_MEMBER_STATS(<CMSG_REQUEST_PARTY_MEMBER_STATS as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x027F, size: body_size, io, } } else { a } })?)),
            0x0280 => Ok(Self::CMSG_GROUP_SWAP_SUB_GROUP(<CMSG_GROUP_SWAP_SUB_GROUP as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x0280, size: body_size, io, } } else { a } })?)),
            0x0282 => Ok(Self::CMSG_AUTOSTORE_BANK_ITEM(<CMSG_AUTOSTORE_BANK_ITEM as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x0282, size: body_size, io, } } else { a } })?)),
            0x0283 => Ok(Self::CMSG_AUTOBANK_ITEM(<CMSG_AUTOBANK_ITEM as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x0283, size: body_size, io, } } else { a } })?)),
            0x0284 => Ok(Self::MSG_QUERY_NEXT_MAIL_TIME(<MSG_QUERY_NEXT_MAIL_TIME_Client as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x0284, size: body_size, io, } } else { a } })?)),
            0x028E => Ok(Self::CMSG_GROUP_RAID_CONVERT(<CMSG_GROUP_RAID_CONVERT as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x028E, size: body_size, io, } } else { a } })?)),
            0x028F => Ok(Self::CMSG_GROUP_ASSISTANT_LEADER(<CMSG_GROUP_ASSISTANT_LEADER as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x028F, size: body_size, io, } } else { a } })?)),
            0x0290 => Ok(Self::CMSG_BUYBACK_ITEM(<CMSG_BUYBACK_ITEM as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x0290, size: body_size, io, } } else { a } })?)),
            0x0296 => Ok(Self::CMSG_LFG_GET_STATUS(<CMSG_LFG_GET_STATUS as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x0296, size: body_size, io, } } else { a } })?)),
            0x029B => Ok(Self::CMSG_CANCEL_GROWTH_AURA(<CMSG_CANCEL_GROWTH_AURA as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x029B, size: body_size, io, } } else { a } })?)),
            0x02A0 => Ok(Self::CMSG_LOOT_ROLL(<CMSG_LOOT_ROLL as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x02A0, size: body_size, io, } } else { a } })?)),
            0x02A3 => Ok(Self::CMSG_LOOT_MASTER_GIVE(<CMSG_LOOT_MASTER_GIVE as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x02A3, size: body_size, io, } } else { a } })?)),
            0x02A8 => Ok(Self::CMSG_REPAIR_ITEM(<CMSG_REPAIR_ITEM as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x02A8, size: body_size, io, } } else { a } })?)),
            0x02AC => Ok(Self::CMSG_SUMMON_RESPONSE(<CMSG_SUMMON_RESPONSE as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x02AC, size: body_size, io, } } else { a } })?)),
            0x02B3 => Ok(Self::CMSG_SELF_RES(<CMSG_SELF_RES as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x02B3, size: body_size, io, } } else { a } })?)),
            0x02B9 => Ok(Self::CMSG_TOGGLE_HELM(<CMSG_TOGGLE_HELM as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x02B9, size: body_size, io, } } else { a } })?)),
            0x02BA => Ok(Self::CMSG_TOGGLE_CLOAK(<CMSG_TOGGLE_CLOAK as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x02BA, size: body_size, io, } } else { a } })?)),
            0x02BF => Ok(Self::CMSG_SET_ACTIONBAR_TOGGLES(<CMSG_SET_ACTIONBAR_TOGGLES as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x02BF, size: body_size, io, } } else { a } })?)),
            0x02C4 => Ok(Self::CMSG_ITEM_NAME_QUERY(<CMSG_ITEM_NAME_QUERY as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x02C4, size: body_size, io, } } else { a } })?)),
            0x02C7 => Ok(Self::CMSG_CHAR_RENAME(<CMSG_CHAR_RENAME as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x02C7, size: body_size, io, } } else { a } })?)),
            0x02C9 => Ok(Self::CMSG_MOVE_SPLINE_DONE(<CMSG_MOVE_SPLINE_DONE as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x02C9, size: body_size, io, } } else { a } })?)),
            0x02CA => Ok(Self::CMSG_MOVE_FALL_RESET(<CMSG_MOVE_FALL_RESET as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x02CA, size: body_size, io, } } else { a } })?)),
            0x02CD => Ok(Self::CMSG_REQUEST_RAID_INFO(<CMSG_REQUEST_RAID_INFO as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x02CD, size: body_size, io, } } else { a } })?)),
            0x02CE => Ok(Self::CMSG_MOVE_TIME_SKIPPED(<CMSG_MOVE_TIME_SKIPPED as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x02CE, size: body_size, io, } } else { a } })?)),
            0x02CF => Ok(Self::CMSG_MOVE_FEATHER_FALL_ACK(<CMSG_MOVE_FEATHER_FALL_ACK as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x02CF, size: body_size, io, } } else { a } })?)),
            0x02D0 => Ok(Self::CMSG_MOVE_WATER_WALK_ACK(<CMSG_MOVE_WATER_WALK_ACK as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x02D0, size: body_size, io, } } else { a } })?)),
            0x02D1 => Ok(Self::CMSG_MOVE_NOT_ACTIVE_MOVER(<CMSG_MOVE_NOT_ACTIVE_MOVER as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x02D1, size: body_size, io, } } else { a } })?)),
            0x02D3 => Ok(Self::CMSG_BATTLEFIELD_STATUS(<CMSG_BATTLEFIELD_STATUS as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x02D3, size: body_size, io, } } else { a } })?)),
            0x02D5 => Ok(Self::CMSG_BATTLEFIELD_PORT(<CMSG_BATTLEFIELD_PORT as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x02D5, size: body_size, io, } } else { a } })?)),
            0x02D6 => Ok(Self::MSG_INSPECT_HONOR_STATS(<MSG_INSPECT_HONOR_STATS_Client as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x02D6, size: body_size, io, } } else { a } })?)),
            0x02D7 => Ok(Self::CMSG_BATTLEMASTER_HELLO(<CMSG_BATTLEMASTER_HELLO as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x02D7, size: body_size, io, } } else { a } })?)),
            0x02DB => Ok(Self::CMSG_FORCE_WALK_SPEED_CHANGE_ACK(<CMSG_FORCE_WALK_SPEED_CHANGE_ACK as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x02DB, size: body_size, io, } } else { a } })?)),
            0x02DD => Ok(Self::CMSG_FORCE_SWIM_BACK_SPEED_CHANGE_ACK(<CMSG_FORCE_SWIM_BACK_SPEED_CHANGE_ACK as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x02DD, size: body_size, io, } } else { a } })?)),
            0x02DF => Ok(Self::CMSG_FORCE_TURN_RATE_CHANGE_ACK(<CMSG_FORCE_TURN_RATE_CHANGE_ACK as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x02DF, size: body_size, io, } } else { a } })?)),
            0x02E0 => Ok(Self::MSG_PVP_LOG_DATA(<MSG_PVP_LOG_DATA_Client as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x02E0, size: body_size, io, } } else { a } })?)),
            0x02E1 => Ok(Self::CMSG_LEAVE_BATTLEFIELD(<CMSG_LEAVE_BATTLEFIELD as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x02E1, size: body_size, io, } } else { a } })?)),
            0x02E2 => Ok(Self::CMSG_AREA_SPIRIT_HEALER_QUERY(<CMSG_AREA_SPIRIT_HEALER_QUERY as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x02E2, size: body_size, io, } } else { a } })?)),
            0x02E3 => Ok(Self::CMSG_AREA_SPIRIT_HEALER_QUEUE(<CMSG_AREA_SPIRIT_HEALER_QUEUE as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x02E3, size: body_size, io, } } else { a } })?)),
            0x02E9 => Ok(Self::MSG_BATTLEGROUND_PLAYER_POSITIONS(<MSG_BATTLEGROUND_PLAYER_POSITIONS_Client as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x02E9, size: body_size, io, } } else { a } })?)),
            0x02EA => Ok(Self::CMSG_PET_STOP_ATTACK(<CMSG_PET_STOP_ATTACK as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x02EA, size: body_size, io, } } else { a } })?)),
            0x02EE => Ok(Self::CMSG_BATTLEMASTER_JOIN(<CMSG_BATTLEMASTER_JOIN as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x02EE, size: body_size, io, } } else { a } })?)),
            0x02F0 => Ok(Self::CMSG_PET_UNLEARN(<CMSG_PET_UNLEARN as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x02F0, size: body_size, io, } } else { a } })?)),
            0x02F3 => Ok(Self::CMSG_PET_SPELL_AUTOCAST(<CMSG_PET_SPELL_AUTOCAST as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x02F3, size: body_size, io, } } else { a } })?)),
            0x02FC => Ok(Self::CMSG_GUILD_INFO_TEXT(<CMSG_GUILD_INFO_TEXT as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x02FC, size: body_size, io, } } else { a } })?)),
            0x0312 => Ok(Self::CMSG_ACTIVATETAXIEXPRESS(<CMSG_ACTIVATETAXIEXPRESS as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x0312, size: body_size, io, } } else { a } })?)),
            0x0317 => Ok(Self::CMSG_SET_FACTION_INACTIVE(<CMSG_SET_FACTION_INACTIVE as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x0317, size: body_size, io, } } else { a } })?)),
            0x0318 => Ok(Self::CMSG_SET_WATCHED_FACTION(<CMSG_SET_WATCHED_FACTION as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x0318, size: body_size, io, } } else { a } })?)),
            0x031D => Ok(Self::CMSG_RESET_INSTANCES(<CMSG_RESET_INSTANCES as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x031D, size: body_size, io, } } else { a } })?)),
            0x0322 => Ok(Self::MSG_RAID_READY_CHECK(<MSG_RAID_READY_CHECK_Client as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x0322, size: body_size, io, } } else { a } })?)),
            0x032A => Ok(Self::CMSG_GMSURVEY_SUBMIT(<CMSG_GMSURVEY_SUBMIT as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x032A, size: body_size, io, } } else { a } })?)),
            0x0346 => Ok(Self::CMSG_MOVE_SET_FLY(<CMSG_MOVE_SET_FLY as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x0346, size: body_size, io, } } else { a } })?)),
            0x036E => Ok(Self::CMSG_LFD_PLAYER_LOCK_INFO_REQUEST(<CMSG_LFD_PLAYER_LOCK_INFO_REQUEST as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x036E, size: body_size, io, } } else { a } })?)),
            0x038C => Ok(Self::CMSG_REALM_SPLIT(<CMSG_REALM_SPLIT as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x038C, size: body_size, io, } } else { a } })?)),
            0x038D => Ok(Self::CMSG_MOVE_CHNG_TRANSPORT(<CMSG_MOVE_CHNG_TRANSPORT as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x038D, size: body_size, io, } } else { a } })?)),
            0x0391 => Ok(Self::CMSG_TIME_SYNC_RESP(<CMSG_TIME_SYNC_RESP as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x0391, size: body_size, io, } } else { a } })?)),
            0x03AF => Ok(Self::CMSG_VOICE_SESSION_ENABLE(<CMSG_VOICE_SESSION_ENABLE as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x03AF, size: body_size, io, } } else { a } })?)),
            0x03D3 => Ok(Self::CMSG_SET_ACTIVE_VOICE_CHANNEL(<CMSG_SET_ACTIVE_VOICE_CHANNEL as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x03D3, size: body_size, io, } } else { a } })?)),
            0x04F6 => Ok(Self::CMSG_WORLD_STATE_UI_TIMER_UPDATE(<CMSG_WORLD_STATE_UI_TIMER_UPDATE as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x04F6, size: body_size, io, } } else { a } })?)),
            0x04FF => Ok(Self::CMSG_READY_FOR_ACCOUNT_DATA_TIMES(<CMSG_READY_FOR_ACCOUNT_DATA_TIMES as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x04FF, size: body_size, io, } } else { a } })?)),
            _ => Err(crate::errors::ExpectedOpcodeError::Opcode{ opcode, name: opcode_to_name(opcode), size: body_size }),
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
            Self::MSG_QUEST_PUSH_RESULT(c) => c.write_encrypted_client(w, e),
            Self::MSG_PETITION_RENAME(c) => c.write_encrypted_client(w, e),
            Self::MSG_SET_DUNGEON_DIFFICULTY(c) => c.write_encrypted_client(w, e),
            Self::MSG_MOVE_START_ASCEND(c) => c.write_encrypted_client(w, e),
            Self::MSG_MOVE_STOP_ASCEND(c) => c.write_encrypted_client(w, e),
            Self::MSG_MOVE_START_DESCEND(c) => c.write_encrypted_client(w, e),
            Self::MSG_GUILD_BANK_MONEY_WITHDRAWN(c) => c.write_encrypted_client(w, e),
            Self::CMSG_CALENDAR_GET_NUM_PENDING(c) => c.write_encrypted_client(w, e),
            Self::CMSG_BOOTME(c) => c.write_encrypted_client(w, e),
            Self::CMSG_WORLD_TELEPORT(c) => c.write_encrypted_client(w, e),
            Self::CMSG_TELEPORT_TO_UNIT(c) => c.write_encrypted_client(w, e),
            Self::CMSG_CHAR_CREATE(c) => c.write_encrypted_client(w, e),
            Self::CMSG_CHAR_ENUM(c) => c.write_encrypted_client(w, e),
            Self::CMSG_CHAR_DELETE(c) => c.write_encrypted_client(w, e),
            Self::CMSG_PLAYER_LOGIN(c) => c.write_encrypted_client(w, e),
            Self::CMSG_PLAYER_LOGOUT(c) => c.write_encrypted_client(w, e),
            Self::CMSG_LOGOUT_REQUEST(c) => c.write_encrypted_client(w, e),
            Self::CMSG_LOGOUT_CANCEL(c) => c.write_encrypted_client(w, e),
            Self::CMSG_NAME_QUERY(c) => c.write_encrypted_client(w, e),
            Self::CMSG_PET_NAME_QUERY(c) => c.write_encrypted_client(w, e),
            Self::CMSG_GUILD_QUERY(c) => c.write_encrypted_client(w, e),
            Self::CMSG_ITEM_QUERY_SINGLE(c) => c.write_encrypted_client(w, e),
            Self::CMSG_PAGE_TEXT_QUERY(c) => c.write_encrypted_client(w, e),
            Self::CMSG_QUEST_QUERY(c) => c.write_encrypted_client(w, e),
            Self::CMSG_GAMEOBJECT_QUERY(c) => c.write_encrypted_client(w, e),
            Self::CMSG_CREATURE_QUERY(c) => c.write_encrypted_client(w, e),
            Self::CMSG_WHO(c) => c.write_encrypted_client(w, e),
            Self::CMSG_WHOIS(c) => c.write_encrypted_client(w, e),
            Self::CMSG_CONTACT_LIST(c) => c.write_encrypted_client(w, e),
            Self::CMSG_ADD_FRIEND(c) => c.write_encrypted_client(w, e),
            Self::CMSG_DEL_FRIEND(c) => c.write_encrypted_client(w, e),
            Self::CMSG_ADD_IGNORE(c) => c.write_encrypted_client(w, e),
            Self::CMSG_DEL_IGNORE(c) => c.write_encrypted_client(w, e),
            Self::CMSG_GROUP_INVITE(c) => c.write_encrypted_client(w, e),
            Self::CMSG_GROUP_ACCEPT(c) => c.write_encrypted_client(w, e),
            Self::CMSG_GROUP_DECLINE(c) => c.write_encrypted_client(w, e),
            Self::CMSG_GROUP_UNINVITE(c) => c.write_encrypted_client(w, e),
            Self::CMSG_GROUP_UNINVITE_GUID(c) => c.write_encrypted_client(w, e),
            Self::CMSG_GROUP_SET_LEADER(c) => c.write_encrypted_client(w, e),
            Self::CMSG_LOOT_METHOD(c) => c.write_encrypted_client(w, e),
            Self::CMSG_GROUP_DISBAND(c) => c.write_encrypted_client(w, e),
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
            Self::CMSG_USE_ITEM(c) => c.write_encrypted_client(w, e),
            Self::CMSG_OPEN_ITEM(c) => c.write_encrypted_client(w, e),
            Self::CMSG_READ_ITEM(c) => c.write_encrypted_client(w, e),
            Self::CMSG_GAMEOBJ_USE(c) => c.write_encrypted_client(w, e),
            Self::CMSG_AREATRIGGER(c) => c.write_encrypted_client(w, e),
            Self::MSG_MOVE_TELEPORT_ACK(c) => c.write_encrypted_client(w, e),
            Self::CMSG_MOVE_SET_RAW_POSITION(c) => c.write_encrypted_client(w, e),
            Self::CMSG_FORCE_RUN_SPEED_CHANGE_ACK(c) => c.write_encrypted_client(w, e),
            Self::CMSG_FORCE_RUN_BACK_SPEED_CHANGE_ACK(c) => c.write_encrypted_client(w, e),
            Self::CMSG_FORCE_SWIM_SPEED_CHANGE_ACK(c) => c.write_encrypted_client(w, e),
            Self::CMSG_FORCE_MOVE_ROOT_ACK(c) => c.write_encrypted_client(w, e),
            Self::CMSG_FORCE_MOVE_UNROOT_ACK(c) => c.write_encrypted_client(w, e),
            Self::CMSG_MOVE_KNOCK_BACK_ACK(c) => c.write_encrypted_client(w, e),
            Self::CMSG_MOVE_HOVER_ACK(c) => c.write_encrypted_client(w, e),
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
            Self::CMSG_INITIATE_TRADE(c) => c.write_encrypted_client(w, e),
            Self::CMSG_BEGIN_TRADE(c) => c.write_encrypted_client(w, e),
            Self::CMSG_BUSY_TRADE(c) => c.write_encrypted_client(w, e),
            Self::CMSG_IGNORE_TRADE(c) => c.write_encrypted_client(w, e),
            Self::CMSG_ACCEPT_TRADE(c) => c.write_encrypted_client(w, e),
            Self::CMSG_UNACCEPT_TRADE(c) => c.write_encrypted_client(w, e),
            Self::CMSG_CANCEL_TRADE(c) => c.write_encrypted_client(w, e),
            Self::CMSG_SET_TRADE_ITEM(c) => c.write_encrypted_client(w, e),
            Self::CMSG_CLEAR_TRADE_ITEM(c) => c.write_encrypted_client(w, e),
            Self::CMSG_SET_TRADE_GOLD(c) => c.write_encrypted_client(w, e),
            Self::CMSG_SET_FACTION_ATWAR(c) => c.write_encrypted_client(w, e),
            Self::CMSG_SET_ACTION_BUTTON(c) => c.write_encrypted_client(w, e),
            Self::CMSG_CAST_SPELL(c) => c.write_encrypted_client(w, e),
            Self::CMSG_CANCEL_CAST(c) => c.write_encrypted_client(w, e),
            Self::CMSG_CANCEL_AURA(c) => c.write_encrypted_client(w, e),
            Self::CMSG_CANCEL_CHANNELLING(c) => c.write_encrypted_client(w, e),
            Self::CMSG_SET_SELECTION(c) => c.write_encrypted_client(w, e),
            Self::CMSG_DELETEEQUIPMENT_SET(c) => c.write_encrypted_client(w, e),
            Self::CMSG_ATTACKSWING(c) => c.write_encrypted_client(w, e),
            Self::CMSG_ATTACKSTOP(c) => c.write_encrypted_client(w, e),
            Self::CMSG_REPOP_REQUEST(c) => c.write_encrypted_client(w, e),
            Self::CMSG_RESURRECT_RESPONSE(c) => c.write_encrypted_client(w, e),
            Self::CMSG_LOOT(c) => c.write_encrypted_client(w, e),
            Self::CMSG_LOOT_MONEY(c) => c.write_encrypted_client(w, e),
            Self::CMSG_LOOT_RELEASE(c) => c.write_encrypted_client(w, e),
            Self::CMSG_DUEL_ACCEPTED(c) => c.write_encrypted_client(w, e),
            Self::CMSG_DUEL_CANCELLED(c) => c.write_encrypted_client(w, e),
            Self::CMSG_MOUNTSPECIAL_ANIM(c) => c.write_encrypted_client(w, e),
            Self::CMSG_PET_SET_ACTION(c) => c.write_encrypted_client(w, e),
            Self::CMSG_PET_ACTION(c) => c.write_encrypted_client(w, e),
            Self::CMSG_PET_ABANDON(c) => c.write_encrypted_client(w, e),
            Self::CMSG_PET_RENAME(c) => c.write_encrypted_client(w, e),
            Self::CMSG_GOSSIP_HELLO(c) => c.write_encrypted_client(w, e),
            Self::CMSG_GOSSIP_SELECT_OPTION(c) => c.write_encrypted_client(w, e),
            Self::CMSG_NPC_TEXT_QUERY(c) => c.write_encrypted_client(w, e),
            Self::CMSG_QUESTGIVER_STATUS_QUERY(c) => c.write_encrypted_client(w, e),
            Self::CMSG_QUESTGIVER_HELLO(c) => c.write_encrypted_client(w, e),
            Self::CMSG_QUESTGIVER_QUERY_QUEST(c) => c.write_encrypted_client(w, e),
            Self::CMSG_QUESTGIVER_QUEST_AUTOLAUNCH(c) => c.write_encrypted_client(w, e),
            Self::CMSG_QUESTGIVER_ACCEPT_QUEST(c) => c.write_encrypted_client(w, e),
            Self::CMSG_QUESTGIVER_COMPLETE_QUEST(c) => c.write_encrypted_client(w, e),
            Self::CMSG_QUESTGIVER_REQUEST_REWARD(c) => c.write_encrypted_client(w, e),
            Self::CMSG_QUESTGIVER_CHOOSE_REWARD(c) => c.write_encrypted_client(w, e),
            Self::CMSG_QUESTGIVER_CANCEL(c) => c.write_encrypted_client(w, e),
            Self::CMSG_QUESTLOG_SWAP_QUEST(c) => c.write_encrypted_client(w, e),
            Self::CMSG_QUESTLOG_REMOVE_QUEST(c) => c.write_encrypted_client(w, e),
            Self::CMSG_QUEST_CONFIRM_ACCEPT(c) => c.write_encrypted_client(w, e),
            Self::CMSG_PUSHQUESTTOPARTY(c) => c.write_encrypted_client(w, e),
            Self::CMSG_LIST_INVENTORY(c) => c.write_encrypted_client(w, e),
            Self::CMSG_SELL_ITEM(c) => c.write_encrypted_client(w, e),
            Self::CMSG_BUY_ITEM(c) => c.write_encrypted_client(w, e),
            Self::CMSG_BUY_ITEM_IN_SLOT(c) => c.write_encrypted_client(w, e),
            Self::CMSG_TAXINODE_STATUS_QUERY(c) => c.write_encrypted_client(w, e),
            Self::CMSG_TAXIQUERYAVAILABLENODES(c) => c.write_encrypted_client(w, e),
            Self::CMSG_ACTIVATETAXI(c) => c.write_encrypted_client(w, e),
            Self::CMSG_TRAINER_LIST(c) => c.write_encrypted_client(w, e),
            Self::CMSG_TRAINER_BUY_SPELL(c) => c.write_encrypted_client(w, e),
            Self::CMSG_BINDER_ACTIVATE(c) => c.write_encrypted_client(w, e),
            Self::CMSG_BANKER_ACTIVATE(c) => c.write_encrypted_client(w, e),
            Self::CMSG_BUY_BANK_SLOT(c) => c.write_encrypted_client(w, e),
            Self::CMSG_PETITION_SHOWLIST(c) => c.write_encrypted_client(w, e),
            Self::CMSG_PETITION_BUY(c) => c.write_encrypted_client(w, e),
            Self::CMSG_PETITION_SHOW_SIGNATURES(c) => c.write_encrypted_client(w, e),
            Self::CMSG_PETITION_SIGN(c) => c.write_encrypted_client(w, e),
            Self::CMSG_OFFER_PETITION(c) => c.write_encrypted_client(w, e),
            Self::CMSG_TURN_IN_PETITION(c) => c.write_encrypted_client(w, e),
            Self::CMSG_PETITION_QUERY(c) => c.write_encrypted_client(w, e),
            Self::CMSG_BUG(c) => c.write_encrypted_client(w, e),
            Self::CMSG_PLAYED_TIME(c) => c.write_encrypted_client(w, e),
            Self::CMSG_QUERY_TIME(c) => c.write_encrypted_client(w, e),
            Self::CMSG_RECLAIM_CORPSE(c) => c.write_encrypted_client(w, e),
            Self::CMSG_WRAP_ITEM(c) => c.write_encrypted_client(w, e),
            Self::MSG_MINIMAP_PING(c) => c.write_encrypted_client(w, e),
            Self::CMSG_PING(c) => c.write_encrypted_client(w, e),
            Self::CMSG_SETSHEATHED(c) => c.write_encrypted_client(w, e),
            Self::CMSG_AUTH_SESSION(c) => c.write_encrypted_client(w, e),
            Self::CMSG_PET_CAST_SPELL(c) => c.write_encrypted_client(w, e),
            Self::MSG_SAVE_GUILD_EMBLEM(c) => c.write_encrypted_client(w, e),
            Self::CMSG_ZONEUPDATE(c) => c.write_encrypted_client(w, e),
            Self::MSG_RANDOM_ROLL(c) => c.write_encrypted_client(w, e),
            Self::CMSG_UNLEARN_SKILL(c) => c.write_encrypted_client(w, e),
            Self::CMSG_GMTICKET_CREATE(c) => c.write_encrypted_client(w, e),
            Self::CMSG_GMTICKET_UPDATETEXT(c) => c.write_encrypted_client(w, e),
            Self::CMSG_REQUEST_ACCOUNT_DATA(c) => c.write_encrypted_client(w, e),
            Self::CMSG_UPDATE_ACCOUNT_DATA(c) => c.write_encrypted_client(w, e),
            Self::CMSG_GMTICKET_GETTICKET(c) => c.write_encrypted_client(w, e),
            Self::MSG_CORPSE_QUERY(c) => c.write_encrypted_client(w, e),
            Self::CMSG_GMTICKET_DELETETICKET(c) => c.write_encrypted_client(w, e),
            Self::CMSG_GMTICKET_SYSTEMSTATUS(c) => c.write_encrypted_client(w, e),
            Self::CMSG_SPIRIT_HEALER_ACTIVATE(c) => c.write_encrypted_client(w, e),
            Self::CMSG_CHAT_IGNORED(c) => c.write_encrypted_client(w, e),
            Self::CMSG_GUILD_RANK(c) => c.write_encrypted_client(w, e),
            Self::CMSG_GUILD_ADD_RANK(c) => c.write_encrypted_client(w, e),
            Self::CMSG_GUILD_DEL_RANK(c) => c.write_encrypted_client(w, e),
            Self::CMSG_GUILD_SET_PUBLIC_NOTE(c) => c.write_encrypted_client(w, e),
            Self::CMSG_GUILD_SET_OFFICER_NOTE(c) => c.write_encrypted_client(w, e),
            Self::CMSG_SEND_MAIL(c) => c.write_encrypted_client(w, e),
            Self::CMSG_GET_MAIL_LIST(c) => c.write_encrypted_client(w, e),
            Self::CMSG_BATTLEFIELD_LIST(c) => c.write_encrypted_client(w, e),
            Self::CMSG_ITEM_TEXT_QUERY(c) => c.write_encrypted_client(w, e),
            Self::CMSG_MAIL_TAKE_MONEY(c) => c.write_encrypted_client(w, e),
            Self::CMSG_MAIL_TAKE_ITEM(c) => c.write_encrypted_client(w, e),
            Self::CMSG_MAIL_MARK_AS_READ(c) => c.write_encrypted_client(w, e),
            Self::CMSG_MAIL_RETURN_TO_SENDER(c) => c.write_encrypted_client(w, e),
            Self::CMSG_MAIL_DELETE(c) => c.write_encrypted_client(w, e),
            Self::CMSG_MAIL_CREATE_TEXT_ITEM(c) => c.write_encrypted_client(w, e),
            Self::CMSG_LEARN_TALENT(c) => c.write_encrypted_client(w, e),
            Self::CMSG_TOGGLE_PVP(c) => c.write_encrypted_client(w, e),
            Self::MSG_AUCTION_HELLO(c) => c.write_encrypted_client(w, e),
            Self::CMSG_AUCTION_SELL_ITEM(c) => c.write_encrypted_client(w, e),
            Self::CMSG_AUCTION_REMOVE_ITEM(c) => c.write_encrypted_client(w, e),
            Self::CMSG_AUCTION_LIST_ITEMS(c) => c.write_encrypted_client(w, e),
            Self::CMSG_AUCTION_LIST_OWNER_ITEMS(c) => c.write_encrypted_client(w, e),
            Self::CMSG_AUCTION_PLACE_BID(c) => c.write_encrypted_client(w, e),
            Self::CMSG_AUCTION_LIST_BIDDER_ITEMS(c) => c.write_encrypted_client(w, e),
            Self::CMSG_SET_AMMO(c) => c.write_encrypted_client(w, e),
            Self::CMSG_SET_ACTIVE_MOVER(c) => c.write_encrypted_client(w, e),
            Self::CMSG_PET_CANCEL_AURA(c) => c.write_encrypted_client(w, e),
            Self::CMSG_CANCEL_AUTO_REPEAT_SPELL(c) => c.write_encrypted_client(w, e),
            Self::MSG_LIST_STABLED_PETS(c) => c.write_encrypted_client(w, e),
            Self::CMSG_STABLE_PET(c) => c.write_encrypted_client(w, e),
            Self::CMSG_UNSTABLE_PET(c) => c.write_encrypted_client(w, e),
            Self::CMSG_BUY_STABLE_SLOT(c) => c.write_encrypted_client(w, e),
            Self::CMSG_STABLE_SWAP_PET(c) => c.write_encrypted_client(w, e),
            Self::CMSG_REQUEST_PET_INFO(c) => c.write_encrypted_client(w, e),
            Self::CMSG_FAR_SIGHT(c) => c.write_encrypted_client(w, e),
            Self::CMSG_GROUP_CHANGE_SUB_GROUP(c) => c.write_encrypted_client(w, e),
            Self::CMSG_REQUEST_PARTY_MEMBER_STATS(c) => c.write_encrypted_client(w, e),
            Self::CMSG_GROUP_SWAP_SUB_GROUP(c) => c.write_encrypted_client(w, e),
            Self::CMSG_AUTOSTORE_BANK_ITEM(c) => c.write_encrypted_client(w, e),
            Self::CMSG_AUTOBANK_ITEM(c) => c.write_encrypted_client(w, e),
            Self::MSG_QUERY_NEXT_MAIL_TIME(c) => c.write_encrypted_client(w, e),
            Self::CMSG_GROUP_RAID_CONVERT(c) => c.write_encrypted_client(w, e),
            Self::CMSG_GROUP_ASSISTANT_LEADER(c) => c.write_encrypted_client(w, e),
            Self::CMSG_BUYBACK_ITEM(c) => c.write_encrypted_client(w, e),
            Self::CMSG_LFG_GET_STATUS(c) => c.write_encrypted_client(w, e),
            Self::CMSG_CANCEL_GROWTH_AURA(c) => c.write_encrypted_client(w, e),
            Self::CMSG_LOOT_ROLL(c) => c.write_encrypted_client(w, e),
            Self::CMSG_LOOT_MASTER_GIVE(c) => c.write_encrypted_client(w, e),
            Self::CMSG_REPAIR_ITEM(c) => c.write_encrypted_client(w, e),
            Self::CMSG_SUMMON_RESPONSE(c) => c.write_encrypted_client(w, e),
            Self::CMSG_SELF_RES(c) => c.write_encrypted_client(w, e),
            Self::CMSG_TOGGLE_HELM(c) => c.write_encrypted_client(w, e),
            Self::CMSG_TOGGLE_CLOAK(c) => c.write_encrypted_client(w, e),
            Self::CMSG_SET_ACTIONBAR_TOGGLES(c) => c.write_encrypted_client(w, e),
            Self::CMSG_ITEM_NAME_QUERY(c) => c.write_encrypted_client(w, e),
            Self::CMSG_CHAR_RENAME(c) => c.write_encrypted_client(w, e),
            Self::CMSG_MOVE_SPLINE_DONE(c) => c.write_encrypted_client(w, e),
            Self::CMSG_MOVE_FALL_RESET(c) => c.write_encrypted_client(w, e),
            Self::CMSG_REQUEST_RAID_INFO(c) => c.write_encrypted_client(w, e),
            Self::CMSG_MOVE_TIME_SKIPPED(c) => c.write_encrypted_client(w, e),
            Self::CMSG_MOVE_FEATHER_FALL_ACK(c) => c.write_encrypted_client(w, e),
            Self::CMSG_MOVE_WATER_WALK_ACK(c) => c.write_encrypted_client(w, e),
            Self::CMSG_MOVE_NOT_ACTIVE_MOVER(c) => c.write_encrypted_client(w, e),
            Self::CMSG_BATTLEFIELD_STATUS(c) => c.write_encrypted_client(w, e),
            Self::CMSG_BATTLEFIELD_PORT(c) => c.write_encrypted_client(w, e),
            Self::MSG_INSPECT_HONOR_STATS(c) => c.write_encrypted_client(w, e),
            Self::CMSG_BATTLEMASTER_HELLO(c) => c.write_encrypted_client(w, e),
            Self::CMSG_FORCE_WALK_SPEED_CHANGE_ACK(c) => c.write_encrypted_client(w, e),
            Self::CMSG_FORCE_SWIM_BACK_SPEED_CHANGE_ACK(c) => c.write_encrypted_client(w, e),
            Self::CMSG_FORCE_TURN_RATE_CHANGE_ACK(c) => c.write_encrypted_client(w, e),
            Self::MSG_PVP_LOG_DATA(c) => c.write_encrypted_client(w, e),
            Self::CMSG_LEAVE_BATTLEFIELD(c) => c.write_encrypted_client(w, e),
            Self::CMSG_AREA_SPIRIT_HEALER_QUERY(c) => c.write_encrypted_client(w, e),
            Self::CMSG_AREA_SPIRIT_HEALER_QUEUE(c) => c.write_encrypted_client(w, e),
            Self::MSG_BATTLEGROUND_PLAYER_POSITIONS(c) => c.write_encrypted_client(w, e),
            Self::CMSG_PET_STOP_ATTACK(c) => c.write_encrypted_client(w, e),
            Self::CMSG_BATTLEMASTER_JOIN(c) => c.write_encrypted_client(w, e),
            Self::CMSG_PET_UNLEARN(c) => c.write_encrypted_client(w, e),
            Self::CMSG_PET_SPELL_AUTOCAST(c) => c.write_encrypted_client(w, e),
            Self::CMSG_GUILD_INFO_TEXT(c) => c.write_encrypted_client(w, e),
            Self::CMSG_ACTIVATETAXIEXPRESS(c) => c.write_encrypted_client(w, e),
            Self::CMSG_SET_FACTION_INACTIVE(c) => c.write_encrypted_client(w, e),
            Self::CMSG_SET_WATCHED_FACTION(c) => c.write_encrypted_client(w, e),
            Self::CMSG_RESET_INSTANCES(c) => c.write_encrypted_client(w, e),
            Self::MSG_RAID_READY_CHECK(c) => c.write_encrypted_client(w, e),
            Self::CMSG_GMSURVEY_SUBMIT(c) => c.write_encrypted_client(w, e),
            Self::CMSG_MOVE_SET_FLY(c) => c.write_encrypted_client(w, e),
            Self::CMSG_LFD_PLAYER_LOCK_INFO_REQUEST(c) => c.write_encrypted_client(w, e),
            Self::CMSG_REALM_SPLIT(c) => c.write_encrypted_client(w, e),
            Self::CMSG_MOVE_CHNG_TRANSPORT(c) => c.write_encrypted_client(w, e),
            Self::CMSG_TIME_SYNC_RESP(c) => c.write_encrypted_client(w, e),
            Self::CMSG_VOICE_SESSION_ENABLE(c) => c.write_encrypted_client(w, e),
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
            Self::MSG_QUEST_PUSH_RESULT(c) => c.write_unencrypted_client(w),
            Self::MSG_PETITION_RENAME(c) => c.write_unencrypted_client(w),
            Self::MSG_SET_DUNGEON_DIFFICULTY(c) => c.write_unencrypted_client(w),
            Self::MSG_MOVE_START_ASCEND(c) => c.write_unencrypted_client(w),
            Self::MSG_MOVE_STOP_ASCEND(c) => c.write_unencrypted_client(w),
            Self::MSG_MOVE_START_DESCEND(c) => c.write_unencrypted_client(w),
            Self::MSG_GUILD_BANK_MONEY_WITHDRAWN(c) => c.write_unencrypted_client(w),
            Self::CMSG_CALENDAR_GET_NUM_PENDING(c) => c.write_unencrypted_client(w),
            Self::CMSG_BOOTME(c) => c.write_unencrypted_client(w),
            Self::CMSG_WORLD_TELEPORT(c) => c.write_unencrypted_client(w),
            Self::CMSG_TELEPORT_TO_UNIT(c) => c.write_unencrypted_client(w),
            Self::CMSG_CHAR_CREATE(c) => c.write_unencrypted_client(w),
            Self::CMSG_CHAR_ENUM(c) => c.write_unencrypted_client(w),
            Self::CMSG_CHAR_DELETE(c) => c.write_unencrypted_client(w),
            Self::CMSG_PLAYER_LOGIN(c) => c.write_unencrypted_client(w),
            Self::CMSG_PLAYER_LOGOUT(c) => c.write_unencrypted_client(w),
            Self::CMSG_LOGOUT_REQUEST(c) => c.write_unencrypted_client(w),
            Self::CMSG_LOGOUT_CANCEL(c) => c.write_unencrypted_client(w),
            Self::CMSG_NAME_QUERY(c) => c.write_unencrypted_client(w),
            Self::CMSG_PET_NAME_QUERY(c) => c.write_unencrypted_client(w),
            Self::CMSG_GUILD_QUERY(c) => c.write_unencrypted_client(w),
            Self::CMSG_ITEM_QUERY_SINGLE(c) => c.write_unencrypted_client(w),
            Self::CMSG_PAGE_TEXT_QUERY(c) => c.write_unencrypted_client(w),
            Self::CMSG_QUEST_QUERY(c) => c.write_unencrypted_client(w),
            Self::CMSG_GAMEOBJECT_QUERY(c) => c.write_unencrypted_client(w),
            Self::CMSG_CREATURE_QUERY(c) => c.write_unencrypted_client(w),
            Self::CMSG_WHO(c) => c.write_unencrypted_client(w),
            Self::CMSG_WHOIS(c) => c.write_unencrypted_client(w),
            Self::CMSG_CONTACT_LIST(c) => c.write_unencrypted_client(w),
            Self::CMSG_ADD_FRIEND(c) => c.write_unencrypted_client(w),
            Self::CMSG_DEL_FRIEND(c) => c.write_unencrypted_client(w),
            Self::CMSG_ADD_IGNORE(c) => c.write_unencrypted_client(w),
            Self::CMSG_DEL_IGNORE(c) => c.write_unencrypted_client(w),
            Self::CMSG_GROUP_INVITE(c) => c.write_unencrypted_client(w),
            Self::CMSG_GROUP_ACCEPT(c) => c.write_unencrypted_client(w),
            Self::CMSG_GROUP_DECLINE(c) => c.write_unencrypted_client(w),
            Self::CMSG_GROUP_UNINVITE(c) => c.write_unencrypted_client(w),
            Self::CMSG_GROUP_UNINVITE_GUID(c) => c.write_unencrypted_client(w),
            Self::CMSG_GROUP_SET_LEADER(c) => c.write_unencrypted_client(w),
            Self::CMSG_LOOT_METHOD(c) => c.write_unencrypted_client(w),
            Self::CMSG_GROUP_DISBAND(c) => c.write_unencrypted_client(w),
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
            Self::CMSG_USE_ITEM(c) => c.write_unencrypted_client(w),
            Self::CMSG_OPEN_ITEM(c) => c.write_unencrypted_client(w),
            Self::CMSG_READ_ITEM(c) => c.write_unencrypted_client(w),
            Self::CMSG_GAMEOBJ_USE(c) => c.write_unencrypted_client(w),
            Self::CMSG_AREATRIGGER(c) => c.write_unencrypted_client(w),
            Self::MSG_MOVE_TELEPORT_ACK(c) => c.write_unencrypted_client(w),
            Self::CMSG_MOVE_SET_RAW_POSITION(c) => c.write_unencrypted_client(w),
            Self::CMSG_FORCE_RUN_SPEED_CHANGE_ACK(c) => c.write_unencrypted_client(w),
            Self::CMSG_FORCE_RUN_BACK_SPEED_CHANGE_ACK(c) => c.write_unencrypted_client(w),
            Self::CMSG_FORCE_SWIM_SPEED_CHANGE_ACK(c) => c.write_unencrypted_client(w),
            Self::CMSG_FORCE_MOVE_ROOT_ACK(c) => c.write_unencrypted_client(w),
            Self::CMSG_FORCE_MOVE_UNROOT_ACK(c) => c.write_unencrypted_client(w),
            Self::CMSG_MOVE_KNOCK_BACK_ACK(c) => c.write_unencrypted_client(w),
            Self::CMSG_MOVE_HOVER_ACK(c) => c.write_unencrypted_client(w),
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
            Self::CMSG_INITIATE_TRADE(c) => c.write_unencrypted_client(w),
            Self::CMSG_BEGIN_TRADE(c) => c.write_unencrypted_client(w),
            Self::CMSG_BUSY_TRADE(c) => c.write_unencrypted_client(w),
            Self::CMSG_IGNORE_TRADE(c) => c.write_unencrypted_client(w),
            Self::CMSG_ACCEPT_TRADE(c) => c.write_unencrypted_client(w),
            Self::CMSG_UNACCEPT_TRADE(c) => c.write_unencrypted_client(w),
            Self::CMSG_CANCEL_TRADE(c) => c.write_unencrypted_client(w),
            Self::CMSG_SET_TRADE_ITEM(c) => c.write_unencrypted_client(w),
            Self::CMSG_CLEAR_TRADE_ITEM(c) => c.write_unencrypted_client(w),
            Self::CMSG_SET_TRADE_GOLD(c) => c.write_unencrypted_client(w),
            Self::CMSG_SET_FACTION_ATWAR(c) => c.write_unencrypted_client(w),
            Self::CMSG_SET_ACTION_BUTTON(c) => c.write_unencrypted_client(w),
            Self::CMSG_CAST_SPELL(c) => c.write_unencrypted_client(w),
            Self::CMSG_CANCEL_CAST(c) => c.write_unencrypted_client(w),
            Self::CMSG_CANCEL_AURA(c) => c.write_unencrypted_client(w),
            Self::CMSG_CANCEL_CHANNELLING(c) => c.write_unencrypted_client(w),
            Self::CMSG_SET_SELECTION(c) => c.write_unencrypted_client(w),
            Self::CMSG_DELETEEQUIPMENT_SET(c) => c.write_unencrypted_client(w),
            Self::CMSG_ATTACKSWING(c) => c.write_unencrypted_client(w),
            Self::CMSG_ATTACKSTOP(c) => c.write_unencrypted_client(w),
            Self::CMSG_REPOP_REQUEST(c) => c.write_unencrypted_client(w),
            Self::CMSG_RESURRECT_RESPONSE(c) => c.write_unencrypted_client(w),
            Self::CMSG_LOOT(c) => c.write_unencrypted_client(w),
            Self::CMSG_LOOT_MONEY(c) => c.write_unencrypted_client(w),
            Self::CMSG_LOOT_RELEASE(c) => c.write_unencrypted_client(w),
            Self::CMSG_DUEL_ACCEPTED(c) => c.write_unencrypted_client(w),
            Self::CMSG_DUEL_CANCELLED(c) => c.write_unencrypted_client(w),
            Self::CMSG_MOUNTSPECIAL_ANIM(c) => c.write_unencrypted_client(w),
            Self::CMSG_PET_SET_ACTION(c) => c.write_unencrypted_client(w),
            Self::CMSG_PET_ACTION(c) => c.write_unencrypted_client(w),
            Self::CMSG_PET_ABANDON(c) => c.write_unencrypted_client(w),
            Self::CMSG_PET_RENAME(c) => c.write_unencrypted_client(w),
            Self::CMSG_GOSSIP_HELLO(c) => c.write_unencrypted_client(w),
            Self::CMSG_GOSSIP_SELECT_OPTION(c) => c.write_unencrypted_client(w),
            Self::CMSG_NPC_TEXT_QUERY(c) => c.write_unencrypted_client(w),
            Self::CMSG_QUESTGIVER_STATUS_QUERY(c) => c.write_unencrypted_client(w),
            Self::CMSG_QUESTGIVER_HELLO(c) => c.write_unencrypted_client(w),
            Self::CMSG_QUESTGIVER_QUERY_QUEST(c) => c.write_unencrypted_client(w),
            Self::CMSG_QUESTGIVER_QUEST_AUTOLAUNCH(c) => c.write_unencrypted_client(w),
            Self::CMSG_QUESTGIVER_ACCEPT_QUEST(c) => c.write_unencrypted_client(w),
            Self::CMSG_QUESTGIVER_COMPLETE_QUEST(c) => c.write_unencrypted_client(w),
            Self::CMSG_QUESTGIVER_REQUEST_REWARD(c) => c.write_unencrypted_client(w),
            Self::CMSG_QUESTGIVER_CHOOSE_REWARD(c) => c.write_unencrypted_client(w),
            Self::CMSG_QUESTGIVER_CANCEL(c) => c.write_unencrypted_client(w),
            Self::CMSG_QUESTLOG_SWAP_QUEST(c) => c.write_unencrypted_client(w),
            Self::CMSG_QUESTLOG_REMOVE_QUEST(c) => c.write_unencrypted_client(w),
            Self::CMSG_QUEST_CONFIRM_ACCEPT(c) => c.write_unencrypted_client(w),
            Self::CMSG_PUSHQUESTTOPARTY(c) => c.write_unencrypted_client(w),
            Self::CMSG_LIST_INVENTORY(c) => c.write_unencrypted_client(w),
            Self::CMSG_SELL_ITEM(c) => c.write_unencrypted_client(w),
            Self::CMSG_BUY_ITEM(c) => c.write_unencrypted_client(w),
            Self::CMSG_BUY_ITEM_IN_SLOT(c) => c.write_unencrypted_client(w),
            Self::CMSG_TAXINODE_STATUS_QUERY(c) => c.write_unencrypted_client(w),
            Self::CMSG_TAXIQUERYAVAILABLENODES(c) => c.write_unencrypted_client(w),
            Self::CMSG_ACTIVATETAXI(c) => c.write_unencrypted_client(w),
            Self::CMSG_TRAINER_LIST(c) => c.write_unencrypted_client(w),
            Self::CMSG_TRAINER_BUY_SPELL(c) => c.write_unencrypted_client(w),
            Self::CMSG_BINDER_ACTIVATE(c) => c.write_unencrypted_client(w),
            Self::CMSG_BANKER_ACTIVATE(c) => c.write_unencrypted_client(w),
            Self::CMSG_BUY_BANK_SLOT(c) => c.write_unencrypted_client(w),
            Self::CMSG_PETITION_SHOWLIST(c) => c.write_unencrypted_client(w),
            Self::CMSG_PETITION_BUY(c) => c.write_unencrypted_client(w),
            Self::CMSG_PETITION_SHOW_SIGNATURES(c) => c.write_unencrypted_client(w),
            Self::CMSG_PETITION_SIGN(c) => c.write_unencrypted_client(w),
            Self::CMSG_OFFER_PETITION(c) => c.write_unencrypted_client(w),
            Self::CMSG_TURN_IN_PETITION(c) => c.write_unencrypted_client(w),
            Self::CMSG_PETITION_QUERY(c) => c.write_unencrypted_client(w),
            Self::CMSG_BUG(c) => c.write_unencrypted_client(w),
            Self::CMSG_PLAYED_TIME(c) => c.write_unencrypted_client(w),
            Self::CMSG_QUERY_TIME(c) => c.write_unencrypted_client(w),
            Self::CMSG_RECLAIM_CORPSE(c) => c.write_unencrypted_client(w),
            Self::CMSG_WRAP_ITEM(c) => c.write_unencrypted_client(w),
            Self::MSG_MINIMAP_PING(c) => c.write_unencrypted_client(w),
            Self::CMSG_PING(c) => c.write_unencrypted_client(w),
            Self::CMSG_SETSHEATHED(c) => c.write_unencrypted_client(w),
            Self::CMSG_AUTH_SESSION(c) => c.write_unencrypted_client(w),
            Self::CMSG_PET_CAST_SPELL(c) => c.write_unencrypted_client(w),
            Self::MSG_SAVE_GUILD_EMBLEM(c) => c.write_unencrypted_client(w),
            Self::CMSG_ZONEUPDATE(c) => c.write_unencrypted_client(w),
            Self::MSG_RANDOM_ROLL(c) => c.write_unencrypted_client(w),
            Self::CMSG_UNLEARN_SKILL(c) => c.write_unencrypted_client(w),
            Self::CMSG_GMTICKET_CREATE(c) => c.write_unencrypted_client(w),
            Self::CMSG_GMTICKET_UPDATETEXT(c) => c.write_unencrypted_client(w),
            Self::CMSG_REQUEST_ACCOUNT_DATA(c) => c.write_unencrypted_client(w),
            Self::CMSG_UPDATE_ACCOUNT_DATA(c) => c.write_unencrypted_client(w),
            Self::CMSG_GMTICKET_GETTICKET(c) => c.write_unencrypted_client(w),
            Self::MSG_CORPSE_QUERY(c) => c.write_unencrypted_client(w),
            Self::CMSG_GMTICKET_DELETETICKET(c) => c.write_unencrypted_client(w),
            Self::CMSG_GMTICKET_SYSTEMSTATUS(c) => c.write_unencrypted_client(w),
            Self::CMSG_SPIRIT_HEALER_ACTIVATE(c) => c.write_unencrypted_client(w),
            Self::CMSG_CHAT_IGNORED(c) => c.write_unencrypted_client(w),
            Self::CMSG_GUILD_RANK(c) => c.write_unencrypted_client(w),
            Self::CMSG_GUILD_ADD_RANK(c) => c.write_unencrypted_client(w),
            Self::CMSG_GUILD_DEL_RANK(c) => c.write_unencrypted_client(w),
            Self::CMSG_GUILD_SET_PUBLIC_NOTE(c) => c.write_unencrypted_client(w),
            Self::CMSG_GUILD_SET_OFFICER_NOTE(c) => c.write_unencrypted_client(w),
            Self::CMSG_SEND_MAIL(c) => c.write_unencrypted_client(w),
            Self::CMSG_GET_MAIL_LIST(c) => c.write_unencrypted_client(w),
            Self::CMSG_BATTLEFIELD_LIST(c) => c.write_unencrypted_client(w),
            Self::CMSG_ITEM_TEXT_QUERY(c) => c.write_unencrypted_client(w),
            Self::CMSG_MAIL_TAKE_MONEY(c) => c.write_unencrypted_client(w),
            Self::CMSG_MAIL_TAKE_ITEM(c) => c.write_unencrypted_client(w),
            Self::CMSG_MAIL_MARK_AS_READ(c) => c.write_unencrypted_client(w),
            Self::CMSG_MAIL_RETURN_TO_SENDER(c) => c.write_unencrypted_client(w),
            Self::CMSG_MAIL_DELETE(c) => c.write_unencrypted_client(w),
            Self::CMSG_MAIL_CREATE_TEXT_ITEM(c) => c.write_unencrypted_client(w),
            Self::CMSG_LEARN_TALENT(c) => c.write_unencrypted_client(w),
            Self::CMSG_TOGGLE_PVP(c) => c.write_unencrypted_client(w),
            Self::MSG_AUCTION_HELLO(c) => c.write_unencrypted_client(w),
            Self::CMSG_AUCTION_SELL_ITEM(c) => c.write_unencrypted_client(w),
            Self::CMSG_AUCTION_REMOVE_ITEM(c) => c.write_unencrypted_client(w),
            Self::CMSG_AUCTION_LIST_ITEMS(c) => c.write_unencrypted_client(w),
            Self::CMSG_AUCTION_LIST_OWNER_ITEMS(c) => c.write_unencrypted_client(w),
            Self::CMSG_AUCTION_PLACE_BID(c) => c.write_unencrypted_client(w),
            Self::CMSG_AUCTION_LIST_BIDDER_ITEMS(c) => c.write_unencrypted_client(w),
            Self::CMSG_SET_AMMO(c) => c.write_unencrypted_client(w),
            Self::CMSG_SET_ACTIVE_MOVER(c) => c.write_unencrypted_client(w),
            Self::CMSG_PET_CANCEL_AURA(c) => c.write_unencrypted_client(w),
            Self::CMSG_CANCEL_AUTO_REPEAT_SPELL(c) => c.write_unencrypted_client(w),
            Self::MSG_LIST_STABLED_PETS(c) => c.write_unencrypted_client(w),
            Self::CMSG_STABLE_PET(c) => c.write_unencrypted_client(w),
            Self::CMSG_UNSTABLE_PET(c) => c.write_unencrypted_client(w),
            Self::CMSG_BUY_STABLE_SLOT(c) => c.write_unencrypted_client(w),
            Self::CMSG_STABLE_SWAP_PET(c) => c.write_unencrypted_client(w),
            Self::CMSG_REQUEST_PET_INFO(c) => c.write_unencrypted_client(w),
            Self::CMSG_FAR_SIGHT(c) => c.write_unencrypted_client(w),
            Self::CMSG_GROUP_CHANGE_SUB_GROUP(c) => c.write_unencrypted_client(w),
            Self::CMSG_REQUEST_PARTY_MEMBER_STATS(c) => c.write_unencrypted_client(w),
            Self::CMSG_GROUP_SWAP_SUB_GROUP(c) => c.write_unencrypted_client(w),
            Self::CMSG_AUTOSTORE_BANK_ITEM(c) => c.write_unencrypted_client(w),
            Self::CMSG_AUTOBANK_ITEM(c) => c.write_unencrypted_client(w),
            Self::MSG_QUERY_NEXT_MAIL_TIME(c) => c.write_unencrypted_client(w),
            Self::CMSG_GROUP_RAID_CONVERT(c) => c.write_unencrypted_client(w),
            Self::CMSG_GROUP_ASSISTANT_LEADER(c) => c.write_unencrypted_client(w),
            Self::CMSG_BUYBACK_ITEM(c) => c.write_unencrypted_client(w),
            Self::CMSG_LFG_GET_STATUS(c) => c.write_unencrypted_client(w),
            Self::CMSG_CANCEL_GROWTH_AURA(c) => c.write_unencrypted_client(w),
            Self::CMSG_LOOT_ROLL(c) => c.write_unencrypted_client(w),
            Self::CMSG_LOOT_MASTER_GIVE(c) => c.write_unencrypted_client(w),
            Self::CMSG_REPAIR_ITEM(c) => c.write_unencrypted_client(w),
            Self::CMSG_SUMMON_RESPONSE(c) => c.write_unencrypted_client(w),
            Self::CMSG_SELF_RES(c) => c.write_unencrypted_client(w),
            Self::CMSG_TOGGLE_HELM(c) => c.write_unencrypted_client(w),
            Self::CMSG_TOGGLE_CLOAK(c) => c.write_unencrypted_client(w),
            Self::CMSG_SET_ACTIONBAR_TOGGLES(c) => c.write_unencrypted_client(w),
            Self::CMSG_ITEM_NAME_QUERY(c) => c.write_unencrypted_client(w),
            Self::CMSG_CHAR_RENAME(c) => c.write_unencrypted_client(w),
            Self::CMSG_MOVE_SPLINE_DONE(c) => c.write_unencrypted_client(w),
            Self::CMSG_MOVE_FALL_RESET(c) => c.write_unencrypted_client(w),
            Self::CMSG_REQUEST_RAID_INFO(c) => c.write_unencrypted_client(w),
            Self::CMSG_MOVE_TIME_SKIPPED(c) => c.write_unencrypted_client(w),
            Self::CMSG_MOVE_FEATHER_FALL_ACK(c) => c.write_unencrypted_client(w),
            Self::CMSG_MOVE_WATER_WALK_ACK(c) => c.write_unencrypted_client(w),
            Self::CMSG_MOVE_NOT_ACTIVE_MOVER(c) => c.write_unencrypted_client(w),
            Self::CMSG_BATTLEFIELD_STATUS(c) => c.write_unencrypted_client(w),
            Self::CMSG_BATTLEFIELD_PORT(c) => c.write_unencrypted_client(w),
            Self::MSG_INSPECT_HONOR_STATS(c) => c.write_unencrypted_client(w),
            Self::CMSG_BATTLEMASTER_HELLO(c) => c.write_unencrypted_client(w),
            Self::CMSG_FORCE_WALK_SPEED_CHANGE_ACK(c) => c.write_unencrypted_client(w),
            Self::CMSG_FORCE_SWIM_BACK_SPEED_CHANGE_ACK(c) => c.write_unencrypted_client(w),
            Self::CMSG_FORCE_TURN_RATE_CHANGE_ACK(c) => c.write_unencrypted_client(w),
            Self::MSG_PVP_LOG_DATA(c) => c.write_unencrypted_client(w),
            Self::CMSG_LEAVE_BATTLEFIELD(c) => c.write_unencrypted_client(w),
            Self::CMSG_AREA_SPIRIT_HEALER_QUERY(c) => c.write_unencrypted_client(w),
            Self::CMSG_AREA_SPIRIT_HEALER_QUEUE(c) => c.write_unencrypted_client(w),
            Self::MSG_BATTLEGROUND_PLAYER_POSITIONS(c) => c.write_unencrypted_client(w),
            Self::CMSG_PET_STOP_ATTACK(c) => c.write_unencrypted_client(w),
            Self::CMSG_BATTLEMASTER_JOIN(c) => c.write_unencrypted_client(w),
            Self::CMSG_PET_UNLEARN(c) => c.write_unencrypted_client(w),
            Self::CMSG_PET_SPELL_AUTOCAST(c) => c.write_unencrypted_client(w),
            Self::CMSG_GUILD_INFO_TEXT(c) => c.write_unencrypted_client(w),
            Self::CMSG_ACTIVATETAXIEXPRESS(c) => c.write_unencrypted_client(w),
            Self::CMSG_SET_FACTION_INACTIVE(c) => c.write_unencrypted_client(w),
            Self::CMSG_SET_WATCHED_FACTION(c) => c.write_unencrypted_client(w),
            Self::CMSG_RESET_INSTANCES(c) => c.write_unencrypted_client(w),
            Self::MSG_RAID_READY_CHECK(c) => c.write_unencrypted_client(w),
            Self::CMSG_GMSURVEY_SUBMIT(c) => c.write_unencrypted_client(w),
            Self::CMSG_MOVE_SET_FLY(c) => c.write_unencrypted_client(w),
            Self::CMSG_LFD_PLAYER_LOCK_INFO_REQUEST(c) => c.write_unencrypted_client(w),
            Self::CMSG_REALM_SPLIT(c) => c.write_unencrypted_client(w),
            Self::CMSG_MOVE_CHNG_TRANSPORT(c) => c.write_unencrypted_client(w),
            Self::CMSG_TIME_SYNC_RESP(c) => c.write_unencrypted_client(w),
            Self::CMSG_VOICE_SESSION_ENABLE(c) => c.write_unencrypted_client(w),
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
            Self::MSG_QUEST_PUSH_RESULT(c) => c.tokio_write_encrypted_client(w, e).await,
            Self::MSG_PETITION_RENAME(c) => c.tokio_write_encrypted_client(w, e).await,
            Self::MSG_SET_DUNGEON_DIFFICULTY(c) => c.tokio_write_encrypted_client(w, e).await,
            Self::MSG_MOVE_START_ASCEND(c) => c.tokio_write_encrypted_client(w, e).await,
            Self::MSG_MOVE_STOP_ASCEND(c) => c.tokio_write_encrypted_client(w, e).await,
            Self::MSG_MOVE_START_DESCEND(c) => c.tokio_write_encrypted_client(w, e).await,
            Self::MSG_GUILD_BANK_MONEY_WITHDRAWN(c) => c.tokio_write_encrypted_client(w, e).await,
            Self::CMSG_CALENDAR_GET_NUM_PENDING(c) => c.tokio_write_encrypted_client(w, e).await,
            Self::CMSG_BOOTME(c) => c.tokio_write_encrypted_client(w, e).await,
            Self::CMSG_WORLD_TELEPORT(c) => c.tokio_write_encrypted_client(w, e).await,
            Self::CMSG_TELEPORT_TO_UNIT(c) => c.tokio_write_encrypted_client(w, e).await,
            Self::CMSG_CHAR_CREATE(c) => c.tokio_write_encrypted_client(w, e).await,
            Self::CMSG_CHAR_ENUM(c) => c.tokio_write_encrypted_client(w, e).await,
            Self::CMSG_CHAR_DELETE(c) => c.tokio_write_encrypted_client(w, e).await,
            Self::CMSG_PLAYER_LOGIN(c) => c.tokio_write_encrypted_client(w, e).await,
            Self::CMSG_PLAYER_LOGOUT(c) => c.tokio_write_encrypted_client(w, e).await,
            Self::CMSG_LOGOUT_REQUEST(c) => c.tokio_write_encrypted_client(w, e).await,
            Self::CMSG_LOGOUT_CANCEL(c) => c.tokio_write_encrypted_client(w, e).await,
            Self::CMSG_NAME_QUERY(c) => c.tokio_write_encrypted_client(w, e).await,
            Self::CMSG_PET_NAME_QUERY(c) => c.tokio_write_encrypted_client(w, e).await,
            Self::CMSG_GUILD_QUERY(c) => c.tokio_write_encrypted_client(w, e).await,
            Self::CMSG_ITEM_QUERY_SINGLE(c) => c.tokio_write_encrypted_client(w, e).await,
            Self::CMSG_PAGE_TEXT_QUERY(c) => c.tokio_write_encrypted_client(w, e).await,
            Self::CMSG_QUEST_QUERY(c) => c.tokio_write_encrypted_client(w, e).await,
            Self::CMSG_GAMEOBJECT_QUERY(c) => c.tokio_write_encrypted_client(w, e).await,
            Self::CMSG_CREATURE_QUERY(c) => c.tokio_write_encrypted_client(w, e).await,
            Self::CMSG_WHO(c) => c.tokio_write_encrypted_client(w, e).await,
            Self::CMSG_WHOIS(c) => c.tokio_write_encrypted_client(w, e).await,
            Self::CMSG_CONTACT_LIST(c) => c.tokio_write_encrypted_client(w, e).await,
            Self::CMSG_ADD_FRIEND(c) => c.tokio_write_encrypted_client(w, e).await,
            Self::CMSG_DEL_FRIEND(c) => c.tokio_write_encrypted_client(w, e).await,
            Self::CMSG_ADD_IGNORE(c) => c.tokio_write_encrypted_client(w, e).await,
            Self::CMSG_DEL_IGNORE(c) => c.tokio_write_encrypted_client(w, e).await,
            Self::CMSG_GROUP_INVITE(c) => c.tokio_write_encrypted_client(w, e).await,
            Self::CMSG_GROUP_ACCEPT(c) => c.tokio_write_encrypted_client(w, e).await,
            Self::CMSG_GROUP_DECLINE(c) => c.tokio_write_encrypted_client(w, e).await,
            Self::CMSG_GROUP_UNINVITE(c) => c.tokio_write_encrypted_client(w, e).await,
            Self::CMSG_GROUP_UNINVITE_GUID(c) => c.tokio_write_encrypted_client(w, e).await,
            Self::CMSG_GROUP_SET_LEADER(c) => c.tokio_write_encrypted_client(w, e).await,
            Self::CMSG_LOOT_METHOD(c) => c.tokio_write_encrypted_client(w, e).await,
            Self::CMSG_GROUP_DISBAND(c) => c.tokio_write_encrypted_client(w, e).await,
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
            Self::CMSG_USE_ITEM(c) => c.tokio_write_encrypted_client(w, e).await,
            Self::CMSG_OPEN_ITEM(c) => c.tokio_write_encrypted_client(w, e).await,
            Self::CMSG_READ_ITEM(c) => c.tokio_write_encrypted_client(w, e).await,
            Self::CMSG_GAMEOBJ_USE(c) => c.tokio_write_encrypted_client(w, e).await,
            Self::CMSG_AREATRIGGER(c) => c.tokio_write_encrypted_client(w, e).await,
            Self::MSG_MOVE_TELEPORT_ACK(c) => c.tokio_write_encrypted_client(w, e).await,
            Self::CMSG_MOVE_SET_RAW_POSITION(c) => c.tokio_write_encrypted_client(w, e).await,
            Self::CMSG_FORCE_RUN_SPEED_CHANGE_ACK(c) => c.tokio_write_encrypted_client(w, e).await,
            Self::CMSG_FORCE_RUN_BACK_SPEED_CHANGE_ACK(c) => c.tokio_write_encrypted_client(w, e).await,
            Self::CMSG_FORCE_SWIM_SPEED_CHANGE_ACK(c) => c.tokio_write_encrypted_client(w, e).await,
            Self::CMSG_FORCE_MOVE_ROOT_ACK(c) => c.tokio_write_encrypted_client(w, e).await,
            Self::CMSG_FORCE_MOVE_UNROOT_ACK(c) => c.tokio_write_encrypted_client(w, e).await,
            Self::CMSG_MOVE_KNOCK_BACK_ACK(c) => c.tokio_write_encrypted_client(w, e).await,
            Self::CMSG_MOVE_HOVER_ACK(c) => c.tokio_write_encrypted_client(w, e).await,
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
            Self::CMSG_INITIATE_TRADE(c) => c.tokio_write_encrypted_client(w, e).await,
            Self::CMSG_BEGIN_TRADE(c) => c.tokio_write_encrypted_client(w, e).await,
            Self::CMSG_BUSY_TRADE(c) => c.tokio_write_encrypted_client(w, e).await,
            Self::CMSG_IGNORE_TRADE(c) => c.tokio_write_encrypted_client(w, e).await,
            Self::CMSG_ACCEPT_TRADE(c) => c.tokio_write_encrypted_client(w, e).await,
            Self::CMSG_UNACCEPT_TRADE(c) => c.tokio_write_encrypted_client(w, e).await,
            Self::CMSG_CANCEL_TRADE(c) => c.tokio_write_encrypted_client(w, e).await,
            Self::CMSG_SET_TRADE_ITEM(c) => c.tokio_write_encrypted_client(w, e).await,
            Self::CMSG_CLEAR_TRADE_ITEM(c) => c.tokio_write_encrypted_client(w, e).await,
            Self::CMSG_SET_TRADE_GOLD(c) => c.tokio_write_encrypted_client(w, e).await,
            Self::CMSG_SET_FACTION_ATWAR(c) => c.tokio_write_encrypted_client(w, e).await,
            Self::CMSG_SET_ACTION_BUTTON(c) => c.tokio_write_encrypted_client(w, e).await,
            Self::CMSG_CAST_SPELL(c) => c.tokio_write_encrypted_client(w, e).await,
            Self::CMSG_CANCEL_CAST(c) => c.tokio_write_encrypted_client(w, e).await,
            Self::CMSG_CANCEL_AURA(c) => c.tokio_write_encrypted_client(w, e).await,
            Self::CMSG_CANCEL_CHANNELLING(c) => c.tokio_write_encrypted_client(w, e).await,
            Self::CMSG_SET_SELECTION(c) => c.tokio_write_encrypted_client(w, e).await,
            Self::CMSG_DELETEEQUIPMENT_SET(c) => c.tokio_write_encrypted_client(w, e).await,
            Self::CMSG_ATTACKSWING(c) => c.tokio_write_encrypted_client(w, e).await,
            Self::CMSG_ATTACKSTOP(c) => c.tokio_write_encrypted_client(w, e).await,
            Self::CMSG_REPOP_REQUEST(c) => c.tokio_write_encrypted_client(w, e).await,
            Self::CMSG_RESURRECT_RESPONSE(c) => c.tokio_write_encrypted_client(w, e).await,
            Self::CMSG_LOOT(c) => c.tokio_write_encrypted_client(w, e).await,
            Self::CMSG_LOOT_MONEY(c) => c.tokio_write_encrypted_client(w, e).await,
            Self::CMSG_LOOT_RELEASE(c) => c.tokio_write_encrypted_client(w, e).await,
            Self::CMSG_DUEL_ACCEPTED(c) => c.tokio_write_encrypted_client(w, e).await,
            Self::CMSG_DUEL_CANCELLED(c) => c.tokio_write_encrypted_client(w, e).await,
            Self::CMSG_MOUNTSPECIAL_ANIM(c) => c.tokio_write_encrypted_client(w, e).await,
            Self::CMSG_PET_SET_ACTION(c) => c.tokio_write_encrypted_client(w, e).await,
            Self::CMSG_PET_ACTION(c) => c.tokio_write_encrypted_client(w, e).await,
            Self::CMSG_PET_ABANDON(c) => c.tokio_write_encrypted_client(w, e).await,
            Self::CMSG_PET_RENAME(c) => c.tokio_write_encrypted_client(w, e).await,
            Self::CMSG_GOSSIP_HELLO(c) => c.tokio_write_encrypted_client(w, e).await,
            Self::CMSG_GOSSIP_SELECT_OPTION(c) => c.tokio_write_encrypted_client(w, e).await,
            Self::CMSG_NPC_TEXT_QUERY(c) => c.tokio_write_encrypted_client(w, e).await,
            Self::CMSG_QUESTGIVER_STATUS_QUERY(c) => c.tokio_write_encrypted_client(w, e).await,
            Self::CMSG_QUESTGIVER_HELLO(c) => c.tokio_write_encrypted_client(w, e).await,
            Self::CMSG_QUESTGIVER_QUERY_QUEST(c) => c.tokio_write_encrypted_client(w, e).await,
            Self::CMSG_QUESTGIVER_QUEST_AUTOLAUNCH(c) => c.tokio_write_encrypted_client(w, e).await,
            Self::CMSG_QUESTGIVER_ACCEPT_QUEST(c) => c.tokio_write_encrypted_client(w, e).await,
            Self::CMSG_QUESTGIVER_COMPLETE_QUEST(c) => c.tokio_write_encrypted_client(w, e).await,
            Self::CMSG_QUESTGIVER_REQUEST_REWARD(c) => c.tokio_write_encrypted_client(w, e).await,
            Self::CMSG_QUESTGIVER_CHOOSE_REWARD(c) => c.tokio_write_encrypted_client(w, e).await,
            Self::CMSG_QUESTGIVER_CANCEL(c) => c.tokio_write_encrypted_client(w, e).await,
            Self::CMSG_QUESTLOG_SWAP_QUEST(c) => c.tokio_write_encrypted_client(w, e).await,
            Self::CMSG_QUESTLOG_REMOVE_QUEST(c) => c.tokio_write_encrypted_client(w, e).await,
            Self::CMSG_QUEST_CONFIRM_ACCEPT(c) => c.tokio_write_encrypted_client(w, e).await,
            Self::CMSG_PUSHQUESTTOPARTY(c) => c.tokio_write_encrypted_client(w, e).await,
            Self::CMSG_LIST_INVENTORY(c) => c.tokio_write_encrypted_client(w, e).await,
            Self::CMSG_SELL_ITEM(c) => c.tokio_write_encrypted_client(w, e).await,
            Self::CMSG_BUY_ITEM(c) => c.tokio_write_encrypted_client(w, e).await,
            Self::CMSG_BUY_ITEM_IN_SLOT(c) => c.tokio_write_encrypted_client(w, e).await,
            Self::CMSG_TAXINODE_STATUS_QUERY(c) => c.tokio_write_encrypted_client(w, e).await,
            Self::CMSG_TAXIQUERYAVAILABLENODES(c) => c.tokio_write_encrypted_client(w, e).await,
            Self::CMSG_ACTIVATETAXI(c) => c.tokio_write_encrypted_client(w, e).await,
            Self::CMSG_TRAINER_LIST(c) => c.tokio_write_encrypted_client(w, e).await,
            Self::CMSG_TRAINER_BUY_SPELL(c) => c.tokio_write_encrypted_client(w, e).await,
            Self::CMSG_BINDER_ACTIVATE(c) => c.tokio_write_encrypted_client(w, e).await,
            Self::CMSG_BANKER_ACTIVATE(c) => c.tokio_write_encrypted_client(w, e).await,
            Self::CMSG_BUY_BANK_SLOT(c) => c.tokio_write_encrypted_client(w, e).await,
            Self::CMSG_PETITION_SHOWLIST(c) => c.tokio_write_encrypted_client(w, e).await,
            Self::CMSG_PETITION_BUY(c) => c.tokio_write_encrypted_client(w, e).await,
            Self::CMSG_PETITION_SHOW_SIGNATURES(c) => c.tokio_write_encrypted_client(w, e).await,
            Self::CMSG_PETITION_SIGN(c) => c.tokio_write_encrypted_client(w, e).await,
            Self::CMSG_OFFER_PETITION(c) => c.tokio_write_encrypted_client(w, e).await,
            Self::CMSG_TURN_IN_PETITION(c) => c.tokio_write_encrypted_client(w, e).await,
            Self::CMSG_PETITION_QUERY(c) => c.tokio_write_encrypted_client(w, e).await,
            Self::CMSG_BUG(c) => c.tokio_write_encrypted_client(w, e).await,
            Self::CMSG_PLAYED_TIME(c) => c.tokio_write_encrypted_client(w, e).await,
            Self::CMSG_QUERY_TIME(c) => c.tokio_write_encrypted_client(w, e).await,
            Self::CMSG_RECLAIM_CORPSE(c) => c.tokio_write_encrypted_client(w, e).await,
            Self::CMSG_WRAP_ITEM(c) => c.tokio_write_encrypted_client(w, e).await,
            Self::MSG_MINIMAP_PING(c) => c.tokio_write_encrypted_client(w, e).await,
            Self::CMSG_PING(c) => c.tokio_write_encrypted_client(w, e).await,
            Self::CMSG_SETSHEATHED(c) => c.tokio_write_encrypted_client(w, e).await,
            Self::CMSG_AUTH_SESSION(c) => c.tokio_write_encrypted_client(w, e).await,
            Self::CMSG_PET_CAST_SPELL(c) => c.tokio_write_encrypted_client(w, e).await,
            Self::MSG_SAVE_GUILD_EMBLEM(c) => c.tokio_write_encrypted_client(w, e).await,
            Self::CMSG_ZONEUPDATE(c) => c.tokio_write_encrypted_client(w, e).await,
            Self::MSG_RANDOM_ROLL(c) => c.tokio_write_encrypted_client(w, e).await,
            Self::CMSG_UNLEARN_SKILL(c) => c.tokio_write_encrypted_client(w, e).await,
            Self::CMSG_GMTICKET_CREATE(c) => c.tokio_write_encrypted_client(w, e).await,
            Self::CMSG_GMTICKET_UPDATETEXT(c) => c.tokio_write_encrypted_client(w, e).await,
            Self::CMSG_REQUEST_ACCOUNT_DATA(c) => c.tokio_write_encrypted_client(w, e).await,
            Self::CMSG_UPDATE_ACCOUNT_DATA(c) => c.tokio_write_encrypted_client(w, e).await,
            Self::CMSG_GMTICKET_GETTICKET(c) => c.tokio_write_encrypted_client(w, e).await,
            Self::MSG_CORPSE_QUERY(c) => c.tokio_write_encrypted_client(w, e).await,
            Self::CMSG_GMTICKET_DELETETICKET(c) => c.tokio_write_encrypted_client(w, e).await,
            Self::CMSG_GMTICKET_SYSTEMSTATUS(c) => c.tokio_write_encrypted_client(w, e).await,
            Self::CMSG_SPIRIT_HEALER_ACTIVATE(c) => c.tokio_write_encrypted_client(w, e).await,
            Self::CMSG_CHAT_IGNORED(c) => c.tokio_write_encrypted_client(w, e).await,
            Self::CMSG_GUILD_RANK(c) => c.tokio_write_encrypted_client(w, e).await,
            Self::CMSG_GUILD_ADD_RANK(c) => c.tokio_write_encrypted_client(w, e).await,
            Self::CMSG_GUILD_DEL_RANK(c) => c.tokio_write_encrypted_client(w, e).await,
            Self::CMSG_GUILD_SET_PUBLIC_NOTE(c) => c.tokio_write_encrypted_client(w, e).await,
            Self::CMSG_GUILD_SET_OFFICER_NOTE(c) => c.tokio_write_encrypted_client(w, e).await,
            Self::CMSG_SEND_MAIL(c) => c.tokio_write_encrypted_client(w, e).await,
            Self::CMSG_GET_MAIL_LIST(c) => c.tokio_write_encrypted_client(w, e).await,
            Self::CMSG_BATTLEFIELD_LIST(c) => c.tokio_write_encrypted_client(w, e).await,
            Self::CMSG_ITEM_TEXT_QUERY(c) => c.tokio_write_encrypted_client(w, e).await,
            Self::CMSG_MAIL_TAKE_MONEY(c) => c.tokio_write_encrypted_client(w, e).await,
            Self::CMSG_MAIL_TAKE_ITEM(c) => c.tokio_write_encrypted_client(w, e).await,
            Self::CMSG_MAIL_MARK_AS_READ(c) => c.tokio_write_encrypted_client(w, e).await,
            Self::CMSG_MAIL_RETURN_TO_SENDER(c) => c.tokio_write_encrypted_client(w, e).await,
            Self::CMSG_MAIL_DELETE(c) => c.tokio_write_encrypted_client(w, e).await,
            Self::CMSG_MAIL_CREATE_TEXT_ITEM(c) => c.tokio_write_encrypted_client(w, e).await,
            Self::CMSG_LEARN_TALENT(c) => c.tokio_write_encrypted_client(w, e).await,
            Self::CMSG_TOGGLE_PVP(c) => c.tokio_write_encrypted_client(w, e).await,
            Self::MSG_AUCTION_HELLO(c) => c.tokio_write_encrypted_client(w, e).await,
            Self::CMSG_AUCTION_SELL_ITEM(c) => c.tokio_write_encrypted_client(w, e).await,
            Self::CMSG_AUCTION_REMOVE_ITEM(c) => c.tokio_write_encrypted_client(w, e).await,
            Self::CMSG_AUCTION_LIST_ITEMS(c) => c.tokio_write_encrypted_client(w, e).await,
            Self::CMSG_AUCTION_LIST_OWNER_ITEMS(c) => c.tokio_write_encrypted_client(w, e).await,
            Self::CMSG_AUCTION_PLACE_BID(c) => c.tokio_write_encrypted_client(w, e).await,
            Self::CMSG_AUCTION_LIST_BIDDER_ITEMS(c) => c.tokio_write_encrypted_client(w, e).await,
            Self::CMSG_SET_AMMO(c) => c.tokio_write_encrypted_client(w, e).await,
            Self::CMSG_SET_ACTIVE_MOVER(c) => c.tokio_write_encrypted_client(w, e).await,
            Self::CMSG_PET_CANCEL_AURA(c) => c.tokio_write_encrypted_client(w, e).await,
            Self::CMSG_CANCEL_AUTO_REPEAT_SPELL(c) => c.tokio_write_encrypted_client(w, e).await,
            Self::MSG_LIST_STABLED_PETS(c) => c.tokio_write_encrypted_client(w, e).await,
            Self::CMSG_STABLE_PET(c) => c.tokio_write_encrypted_client(w, e).await,
            Self::CMSG_UNSTABLE_PET(c) => c.tokio_write_encrypted_client(w, e).await,
            Self::CMSG_BUY_STABLE_SLOT(c) => c.tokio_write_encrypted_client(w, e).await,
            Self::CMSG_STABLE_SWAP_PET(c) => c.tokio_write_encrypted_client(w, e).await,
            Self::CMSG_REQUEST_PET_INFO(c) => c.tokio_write_encrypted_client(w, e).await,
            Self::CMSG_FAR_SIGHT(c) => c.tokio_write_encrypted_client(w, e).await,
            Self::CMSG_GROUP_CHANGE_SUB_GROUP(c) => c.tokio_write_encrypted_client(w, e).await,
            Self::CMSG_REQUEST_PARTY_MEMBER_STATS(c) => c.tokio_write_encrypted_client(w, e).await,
            Self::CMSG_GROUP_SWAP_SUB_GROUP(c) => c.tokio_write_encrypted_client(w, e).await,
            Self::CMSG_AUTOSTORE_BANK_ITEM(c) => c.tokio_write_encrypted_client(w, e).await,
            Self::CMSG_AUTOBANK_ITEM(c) => c.tokio_write_encrypted_client(w, e).await,
            Self::MSG_QUERY_NEXT_MAIL_TIME(c) => c.tokio_write_encrypted_client(w, e).await,
            Self::CMSG_GROUP_RAID_CONVERT(c) => c.tokio_write_encrypted_client(w, e).await,
            Self::CMSG_GROUP_ASSISTANT_LEADER(c) => c.tokio_write_encrypted_client(w, e).await,
            Self::CMSG_BUYBACK_ITEM(c) => c.tokio_write_encrypted_client(w, e).await,
            Self::CMSG_LFG_GET_STATUS(c) => c.tokio_write_encrypted_client(w, e).await,
            Self::CMSG_CANCEL_GROWTH_AURA(c) => c.tokio_write_encrypted_client(w, e).await,
            Self::CMSG_LOOT_ROLL(c) => c.tokio_write_encrypted_client(w, e).await,
            Self::CMSG_LOOT_MASTER_GIVE(c) => c.tokio_write_encrypted_client(w, e).await,
            Self::CMSG_REPAIR_ITEM(c) => c.tokio_write_encrypted_client(w, e).await,
            Self::CMSG_SUMMON_RESPONSE(c) => c.tokio_write_encrypted_client(w, e).await,
            Self::CMSG_SELF_RES(c) => c.tokio_write_encrypted_client(w, e).await,
            Self::CMSG_TOGGLE_HELM(c) => c.tokio_write_encrypted_client(w, e).await,
            Self::CMSG_TOGGLE_CLOAK(c) => c.tokio_write_encrypted_client(w, e).await,
            Self::CMSG_SET_ACTIONBAR_TOGGLES(c) => c.tokio_write_encrypted_client(w, e).await,
            Self::CMSG_ITEM_NAME_QUERY(c) => c.tokio_write_encrypted_client(w, e).await,
            Self::CMSG_CHAR_RENAME(c) => c.tokio_write_encrypted_client(w, e).await,
            Self::CMSG_MOVE_SPLINE_DONE(c) => c.tokio_write_encrypted_client(w, e).await,
            Self::CMSG_MOVE_FALL_RESET(c) => c.tokio_write_encrypted_client(w, e).await,
            Self::CMSG_REQUEST_RAID_INFO(c) => c.tokio_write_encrypted_client(w, e).await,
            Self::CMSG_MOVE_TIME_SKIPPED(c) => c.tokio_write_encrypted_client(w, e).await,
            Self::CMSG_MOVE_FEATHER_FALL_ACK(c) => c.tokio_write_encrypted_client(w, e).await,
            Self::CMSG_MOVE_WATER_WALK_ACK(c) => c.tokio_write_encrypted_client(w, e).await,
            Self::CMSG_MOVE_NOT_ACTIVE_MOVER(c) => c.tokio_write_encrypted_client(w, e).await,
            Self::CMSG_BATTLEFIELD_STATUS(c) => c.tokio_write_encrypted_client(w, e).await,
            Self::CMSG_BATTLEFIELD_PORT(c) => c.tokio_write_encrypted_client(w, e).await,
            Self::MSG_INSPECT_HONOR_STATS(c) => c.tokio_write_encrypted_client(w, e).await,
            Self::CMSG_BATTLEMASTER_HELLO(c) => c.tokio_write_encrypted_client(w, e).await,
            Self::CMSG_FORCE_WALK_SPEED_CHANGE_ACK(c) => c.tokio_write_encrypted_client(w, e).await,
            Self::CMSG_FORCE_SWIM_BACK_SPEED_CHANGE_ACK(c) => c.tokio_write_encrypted_client(w, e).await,
            Self::CMSG_FORCE_TURN_RATE_CHANGE_ACK(c) => c.tokio_write_encrypted_client(w, e).await,
            Self::MSG_PVP_LOG_DATA(c) => c.tokio_write_encrypted_client(w, e).await,
            Self::CMSG_LEAVE_BATTLEFIELD(c) => c.tokio_write_encrypted_client(w, e).await,
            Self::CMSG_AREA_SPIRIT_HEALER_QUERY(c) => c.tokio_write_encrypted_client(w, e).await,
            Self::CMSG_AREA_SPIRIT_HEALER_QUEUE(c) => c.tokio_write_encrypted_client(w, e).await,
            Self::MSG_BATTLEGROUND_PLAYER_POSITIONS(c) => c.tokio_write_encrypted_client(w, e).await,
            Self::CMSG_PET_STOP_ATTACK(c) => c.tokio_write_encrypted_client(w, e).await,
            Self::CMSG_BATTLEMASTER_JOIN(c) => c.tokio_write_encrypted_client(w, e).await,
            Self::CMSG_PET_UNLEARN(c) => c.tokio_write_encrypted_client(w, e).await,
            Self::CMSG_PET_SPELL_AUTOCAST(c) => c.tokio_write_encrypted_client(w, e).await,
            Self::CMSG_GUILD_INFO_TEXT(c) => c.tokio_write_encrypted_client(w, e).await,
            Self::CMSG_ACTIVATETAXIEXPRESS(c) => c.tokio_write_encrypted_client(w, e).await,
            Self::CMSG_SET_FACTION_INACTIVE(c) => c.tokio_write_encrypted_client(w, e).await,
            Self::CMSG_SET_WATCHED_FACTION(c) => c.tokio_write_encrypted_client(w, e).await,
            Self::CMSG_RESET_INSTANCES(c) => c.tokio_write_encrypted_client(w, e).await,
            Self::MSG_RAID_READY_CHECK(c) => c.tokio_write_encrypted_client(w, e).await,
            Self::CMSG_GMSURVEY_SUBMIT(c) => c.tokio_write_encrypted_client(w, e).await,
            Self::CMSG_MOVE_SET_FLY(c) => c.tokio_write_encrypted_client(w, e).await,
            Self::CMSG_LFD_PLAYER_LOCK_INFO_REQUEST(c) => c.tokio_write_encrypted_client(w, e).await,
            Self::CMSG_REALM_SPLIT(c) => c.tokio_write_encrypted_client(w, e).await,
            Self::CMSG_MOVE_CHNG_TRANSPORT(c) => c.tokio_write_encrypted_client(w, e).await,
            Self::CMSG_TIME_SYNC_RESP(c) => c.tokio_write_encrypted_client(w, e).await,
            Self::CMSG_VOICE_SESSION_ENABLE(c) => c.tokio_write_encrypted_client(w, e).await,
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
            Self::MSG_QUEST_PUSH_RESULT(c) => c.tokio_write_unencrypted_client(w).await,
            Self::MSG_PETITION_RENAME(c) => c.tokio_write_unencrypted_client(w).await,
            Self::MSG_SET_DUNGEON_DIFFICULTY(c) => c.tokio_write_unencrypted_client(w).await,
            Self::MSG_MOVE_START_ASCEND(c) => c.tokio_write_unencrypted_client(w).await,
            Self::MSG_MOVE_STOP_ASCEND(c) => c.tokio_write_unencrypted_client(w).await,
            Self::MSG_MOVE_START_DESCEND(c) => c.tokio_write_unencrypted_client(w).await,
            Self::MSG_GUILD_BANK_MONEY_WITHDRAWN(c) => c.tokio_write_unencrypted_client(w).await,
            Self::CMSG_CALENDAR_GET_NUM_PENDING(c) => c.tokio_write_unencrypted_client(w).await,
            Self::CMSG_BOOTME(c) => c.tokio_write_unencrypted_client(w).await,
            Self::CMSG_WORLD_TELEPORT(c) => c.tokio_write_unencrypted_client(w).await,
            Self::CMSG_TELEPORT_TO_UNIT(c) => c.tokio_write_unencrypted_client(w).await,
            Self::CMSG_CHAR_CREATE(c) => c.tokio_write_unencrypted_client(w).await,
            Self::CMSG_CHAR_ENUM(c) => c.tokio_write_unencrypted_client(w).await,
            Self::CMSG_CHAR_DELETE(c) => c.tokio_write_unencrypted_client(w).await,
            Self::CMSG_PLAYER_LOGIN(c) => c.tokio_write_unencrypted_client(w).await,
            Self::CMSG_PLAYER_LOGOUT(c) => c.tokio_write_unencrypted_client(w).await,
            Self::CMSG_LOGOUT_REQUEST(c) => c.tokio_write_unencrypted_client(w).await,
            Self::CMSG_LOGOUT_CANCEL(c) => c.tokio_write_unencrypted_client(w).await,
            Self::CMSG_NAME_QUERY(c) => c.tokio_write_unencrypted_client(w).await,
            Self::CMSG_PET_NAME_QUERY(c) => c.tokio_write_unencrypted_client(w).await,
            Self::CMSG_GUILD_QUERY(c) => c.tokio_write_unencrypted_client(w).await,
            Self::CMSG_ITEM_QUERY_SINGLE(c) => c.tokio_write_unencrypted_client(w).await,
            Self::CMSG_PAGE_TEXT_QUERY(c) => c.tokio_write_unencrypted_client(w).await,
            Self::CMSG_QUEST_QUERY(c) => c.tokio_write_unencrypted_client(w).await,
            Self::CMSG_GAMEOBJECT_QUERY(c) => c.tokio_write_unencrypted_client(w).await,
            Self::CMSG_CREATURE_QUERY(c) => c.tokio_write_unencrypted_client(w).await,
            Self::CMSG_WHO(c) => c.tokio_write_unencrypted_client(w).await,
            Self::CMSG_WHOIS(c) => c.tokio_write_unencrypted_client(w).await,
            Self::CMSG_CONTACT_LIST(c) => c.tokio_write_unencrypted_client(w).await,
            Self::CMSG_ADD_FRIEND(c) => c.tokio_write_unencrypted_client(w).await,
            Self::CMSG_DEL_FRIEND(c) => c.tokio_write_unencrypted_client(w).await,
            Self::CMSG_ADD_IGNORE(c) => c.tokio_write_unencrypted_client(w).await,
            Self::CMSG_DEL_IGNORE(c) => c.tokio_write_unencrypted_client(w).await,
            Self::CMSG_GROUP_INVITE(c) => c.tokio_write_unencrypted_client(w).await,
            Self::CMSG_GROUP_ACCEPT(c) => c.tokio_write_unencrypted_client(w).await,
            Self::CMSG_GROUP_DECLINE(c) => c.tokio_write_unencrypted_client(w).await,
            Self::CMSG_GROUP_UNINVITE(c) => c.tokio_write_unencrypted_client(w).await,
            Self::CMSG_GROUP_UNINVITE_GUID(c) => c.tokio_write_unencrypted_client(w).await,
            Self::CMSG_GROUP_SET_LEADER(c) => c.tokio_write_unencrypted_client(w).await,
            Self::CMSG_LOOT_METHOD(c) => c.tokio_write_unencrypted_client(w).await,
            Self::CMSG_GROUP_DISBAND(c) => c.tokio_write_unencrypted_client(w).await,
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
            Self::CMSG_USE_ITEM(c) => c.tokio_write_unencrypted_client(w).await,
            Self::CMSG_OPEN_ITEM(c) => c.tokio_write_unencrypted_client(w).await,
            Self::CMSG_READ_ITEM(c) => c.tokio_write_unencrypted_client(w).await,
            Self::CMSG_GAMEOBJ_USE(c) => c.tokio_write_unencrypted_client(w).await,
            Self::CMSG_AREATRIGGER(c) => c.tokio_write_unencrypted_client(w).await,
            Self::MSG_MOVE_TELEPORT_ACK(c) => c.tokio_write_unencrypted_client(w).await,
            Self::CMSG_MOVE_SET_RAW_POSITION(c) => c.tokio_write_unencrypted_client(w).await,
            Self::CMSG_FORCE_RUN_SPEED_CHANGE_ACK(c) => c.tokio_write_unencrypted_client(w).await,
            Self::CMSG_FORCE_RUN_BACK_SPEED_CHANGE_ACK(c) => c.tokio_write_unencrypted_client(w).await,
            Self::CMSG_FORCE_SWIM_SPEED_CHANGE_ACK(c) => c.tokio_write_unencrypted_client(w).await,
            Self::CMSG_FORCE_MOVE_ROOT_ACK(c) => c.tokio_write_unencrypted_client(w).await,
            Self::CMSG_FORCE_MOVE_UNROOT_ACK(c) => c.tokio_write_unencrypted_client(w).await,
            Self::CMSG_MOVE_KNOCK_BACK_ACK(c) => c.tokio_write_unencrypted_client(w).await,
            Self::CMSG_MOVE_HOVER_ACK(c) => c.tokio_write_unencrypted_client(w).await,
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
            Self::CMSG_INITIATE_TRADE(c) => c.tokio_write_unencrypted_client(w).await,
            Self::CMSG_BEGIN_TRADE(c) => c.tokio_write_unencrypted_client(w).await,
            Self::CMSG_BUSY_TRADE(c) => c.tokio_write_unencrypted_client(w).await,
            Self::CMSG_IGNORE_TRADE(c) => c.tokio_write_unencrypted_client(w).await,
            Self::CMSG_ACCEPT_TRADE(c) => c.tokio_write_unencrypted_client(w).await,
            Self::CMSG_UNACCEPT_TRADE(c) => c.tokio_write_unencrypted_client(w).await,
            Self::CMSG_CANCEL_TRADE(c) => c.tokio_write_unencrypted_client(w).await,
            Self::CMSG_SET_TRADE_ITEM(c) => c.tokio_write_unencrypted_client(w).await,
            Self::CMSG_CLEAR_TRADE_ITEM(c) => c.tokio_write_unencrypted_client(w).await,
            Self::CMSG_SET_TRADE_GOLD(c) => c.tokio_write_unencrypted_client(w).await,
            Self::CMSG_SET_FACTION_ATWAR(c) => c.tokio_write_unencrypted_client(w).await,
            Self::CMSG_SET_ACTION_BUTTON(c) => c.tokio_write_unencrypted_client(w).await,
            Self::CMSG_CAST_SPELL(c) => c.tokio_write_unencrypted_client(w).await,
            Self::CMSG_CANCEL_CAST(c) => c.tokio_write_unencrypted_client(w).await,
            Self::CMSG_CANCEL_AURA(c) => c.tokio_write_unencrypted_client(w).await,
            Self::CMSG_CANCEL_CHANNELLING(c) => c.tokio_write_unencrypted_client(w).await,
            Self::CMSG_SET_SELECTION(c) => c.tokio_write_unencrypted_client(w).await,
            Self::CMSG_DELETEEQUIPMENT_SET(c) => c.tokio_write_unencrypted_client(w).await,
            Self::CMSG_ATTACKSWING(c) => c.tokio_write_unencrypted_client(w).await,
            Self::CMSG_ATTACKSTOP(c) => c.tokio_write_unencrypted_client(w).await,
            Self::CMSG_REPOP_REQUEST(c) => c.tokio_write_unencrypted_client(w).await,
            Self::CMSG_RESURRECT_RESPONSE(c) => c.tokio_write_unencrypted_client(w).await,
            Self::CMSG_LOOT(c) => c.tokio_write_unencrypted_client(w).await,
            Self::CMSG_LOOT_MONEY(c) => c.tokio_write_unencrypted_client(w).await,
            Self::CMSG_LOOT_RELEASE(c) => c.tokio_write_unencrypted_client(w).await,
            Self::CMSG_DUEL_ACCEPTED(c) => c.tokio_write_unencrypted_client(w).await,
            Self::CMSG_DUEL_CANCELLED(c) => c.tokio_write_unencrypted_client(w).await,
            Self::CMSG_MOUNTSPECIAL_ANIM(c) => c.tokio_write_unencrypted_client(w).await,
            Self::CMSG_PET_SET_ACTION(c) => c.tokio_write_unencrypted_client(w).await,
            Self::CMSG_PET_ACTION(c) => c.tokio_write_unencrypted_client(w).await,
            Self::CMSG_PET_ABANDON(c) => c.tokio_write_unencrypted_client(w).await,
            Self::CMSG_PET_RENAME(c) => c.tokio_write_unencrypted_client(w).await,
            Self::CMSG_GOSSIP_HELLO(c) => c.tokio_write_unencrypted_client(w).await,
            Self::CMSG_GOSSIP_SELECT_OPTION(c) => c.tokio_write_unencrypted_client(w).await,
            Self::CMSG_NPC_TEXT_QUERY(c) => c.tokio_write_unencrypted_client(w).await,
            Self::CMSG_QUESTGIVER_STATUS_QUERY(c) => c.tokio_write_unencrypted_client(w).await,
            Self::CMSG_QUESTGIVER_HELLO(c) => c.tokio_write_unencrypted_client(w).await,
            Self::CMSG_QUESTGIVER_QUERY_QUEST(c) => c.tokio_write_unencrypted_client(w).await,
            Self::CMSG_QUESTGIVER_QUEST_AUTOLAUNCH(c) => c.tokio_write_unencrypted_client(w).await,
            Self::CMSG_QUESTGIVER_ACCEPT_QUEST(c) => c.tokio_write_unencrypted_client(w).await,
            Self::CMSG_QUESTGIVER_COMPLETE_QUEST(c) => c.tokio_write_unencrypted_client(w).await,
            Self::CMSG_QUESTGIVER_REQUEST_REWARD(c) => c.tokio_write_unencrypted_client(w).await,
            Self::CMSG_QUESTGIVER_CHOOSE_REWARD(c) => c.tokio_write_unencrypted_client(w).await,
            Self::CMSG_QUESTGIVER_CANCEL(c) => c.tokio_write_unencrypted_client(w).await,
            Self::CMSG_QUESTLOG_SWAP_QUEST(c) => c.tokio_write_unencrypted_client(w).await,
            Self::CMSG_QUESTLOG_REMOVE_QUEST(c) => c.tokio_write_unencrypted_client(w).await,
            Self::CMSG_QUEST_CONFIRM_ACCEPT(c) => c.tokio_write_unencrypted_client(w).await,
            Self::CMSG_PUSHQUESTTOPARTY(c) => c.tokio_write_unencrypted_client(w).await,
            Self::CMSG_LIST_INVENTORY(c) => c.tokio_write_unencrypted_client(w).await,
            Self::CMSG_SELL_ITEM(c) => c.tokio_write_unencrypted_client(w).await,
            Self::CMSG_BUY_ITEM(c) => c.tokio_write_unencrypted_client(w).await,
            Self::CMSG_BUY_ITEM_IN_SLOT(c) => c.tokio_write_unencrypted_client(w).await,
            Self::CMSG_TAXINODE_STATUS_QUERY(c) => c.tokio_write_unencrypted_client(w).await,
            Self::CMSG_TAXIQUERYAVAILABLENODES(c) => c.tokio_write_unencrypted_client(w).await,
            Self::CMSG_ACTIVATETAXI(c) => c.tokio_write_unencrypted_client(w).await,
            Self::CMSG_TRAINER_LIST(c) => c.tokio_write_unencrypted_client(w).await,
            Self::CMSG_TRAINER_BUY_SPELL(c) => c.tokio_write_unencrypted_client(w).await,
            Self::CMSG_BINDER_ACTIVATE(c) => c.tokio_write_unencrypted_client(w).await,
            Self::CMSG_BANKER_ACTIVATE(c) => c.tokio_write_unencrypted_client(w).await,
            Self::CMSG_BUY_BANK_SLOT(c) => c.tokio_write_unencrypted_client(w).await,
            Self::CMSG_PETITION_SHOWLIST(c) => c.tokio_write_unencrypted_client(w).await,
            Self::CMSG_PETITION_BUY(c) => c.tokio_write_unencrypted_client(w).await,
            Self::CMSG_PETITION_SHOW_SIGNATURES(c) => c.tokio_write_unencrypted_client(w).await,
            Self::CMSG_PETITION_SIGN(c) => c.tokio_write_unencrypted_client(w).await,
            Self::CMSG_OFFER_PETITION(c) => c.tokio_write_unencrypted_client(w).await,
            Self::CMSG_TURN_IN_PETITION(c) => c.tokio_write_unencrypted_client(w).await,
            Self::CMSG_PETITION_QUERY(c) => c.tokio_write_unencrypted_client(w).await,
            Self::CMSG_BUG(c) => c.tokio_write_unencrypted_client(w).await,
            Self::CMSG_PLAYED_TIME(c) => c.tokio_write_unencrypted_client(w).await,
            Self::CMSG_QUERY_TIME(c) => c.tokio_write_unencrypted_client(w).await,
            Self::CMSG_RECLAIM_CORPSE(c) => c.tokio_write_unencrypted_client(w).await,
            Self::CMSG_WRAP_ITEM(c) => c.tokio_write_unencrypted_client(w).await,
            Self::MSG_MINIMAP_PING(c) => c.tokio_write_unencrypted_client(w).await,
            Self::CMSG_PING(c) => c.tokio_write_unencrypted_client(w).await,
            Self::CMSG_SETSHEATHED(c) => c.tokio_write_unencrypted_client(w).await,
            Self::CMSG_AUTH_SESSION(c) => c.tokio_write_unencrypted_client(w).await,
            Self::CMSG_PET_CAST_SPELL(c) => c.tokio_write_unencrypted_client(w).await,
            Self::MSG_SAVE_GUILD_EMBLEM(c) => c.tokio_write_unencrypted_client(w).await,
            Self::CMSG_ZONEUPDATE(c) => c.tokio_write_unencrypted_client(w).await,
            Self::MSG_RANDOM_ROLL(c) => c.tokio_write_unencrypted_client(w).await,
            Self::CMSG_UNLEARN_SKILL(c) => c.tokio_write_unencrypted_client(w).await,
            Self::CMSG_GMTICKET_CREATE(c) => c.tokio_write_unencrypted_client(w).await,
            Self::CMSG_GMTICKET_UPDATETEXT(c) => c.tokio_write_unencrypted_client(w).await,
            Self::CMSG_REQUEST_ACCOUNT_DATA(c) => c.tokio_write_unencrypted_client(w).await,
            Self::CMSG_UPDATE_ACCOUNT_DATA(c) => c.tokio_write_unencrypted_client(w).await,
            Self::CMSG_GMTICKET_GETTICKET(c) => c.tokio_write_unencrypted_client(w).await,
            Self::MSG_CORPSE_QUERY(c) => c.tokio_write_unencrypted_client(w).await,
            Self::CMSG_GMTICKET_DELETETICKET(c) => c.tokio_write_unencrypted_client(w).await,
            Self::CMSG_GMTICKET_SYSTEMSTATUS(c) => c.tokio_write_unencrypted_client(w).await,
            Self::CMSG_SPIRIT_HEALER_ACTIVATE(c) => c.tokio_write_unencrypted_client(w).await,
            Self::CMSG_CHAT_IGNORED(c) => c.tokio_write_unencrypted_client(w).await,
            Self::CMSG_GUILD_RANK(c) => c.tokio_write_unencrypted_client(w).await,
            Self::CMSG_GUILD_ADD_RANK(c) => c.tokio_write_unencrypted_client(w).await,
            Self::CMSG_GUILD_DEL_RANK(c) => c.tokio_write_unencrypted_client(w).await,
            Self::CMSG_GUILD_SET_PUBLIC_NOTE(c) => c.tokio_write_unencrypted_client(w).await,
            Self::CMSG_GUILD_SET_OFFICER_NOTE(c) => c.tokio_write_unencrypted_client(w).await,
            Self::CMSG_SEND_MAIL(c) => c.tokio_write_unencrypted_client(w).await,
            Self::CMSG_GET_MAIL_LIST(c) => c.tokio_write_unencrypted_client(w).await,
            Self::CMSG_BATTLEFIELD_LIST(c) => c.tokio_write_unencrypted_client(w).await,
            Self::CMSG_ITEM_TEXT_QUERY(c) => c.tokio_write_unencrypted_client(w).await,
            Self::CMSG_MAIL_TAKE_MONEY(c) => c.tokio_write_unencrypted_client(w).await,
            Self::CMSG_MAIL_TAKE_ITEM(c) => c.tokio_write_unencrypted_client(w).await,
            Self::CMSG_MAIL_MARK_AS_READ(c) => c.tokio_write_unencrypted_client(w).await,
            Self::CMSG_MAIL_RETURN_TO_SENDER(c) => c.tokio_write_unencrypted_client(w).await,
            Self::CMSG_MAIL_DELETE(c) => c.tokio_write_unencrypted_client(w).await,
            Self::CMSG_MAIL_CREATE_TEXT_ITEM(c) => c.tokio_write_unencrypted_client(w).await,
            Self::CMSG_LEARN_TALENT(c) => c.tokio_write_unencrypted_client(w).await,
            Self::CMSG_TOGGLE_PVP(c) => c.tokio_write_unencrypted_client(w).await,
            Self::MSG_AUCTION_HELLO(c) => c.tokio_write_unencrypted_client(w).await,
            Self::CMSG_AUCTION_SELL_ITEM(c) => c.tokio_write_unencrypted_client(w).await,
            Self::CMSG_AUCTION_REMOVE_ITEM(c) => c.tokio_write_unencrypted_client(w).await,
            Self::CMSG_AUCTION_LIST_ITEMS(c) => c.tokio_write_unencrypted_client(w).await,
            Self::CMSG_AUCTION_LIST_OWNER_ITEMS(c) => c.tokio_write_unencrypted_client(w).await,
            Self::CMSG_AUCTION_PLACE_BID(c) => c.tokio_write_unencrypted_client(w).await,
            Self::CMSG_AUCTION_LIST_BIDDER_ITEMS(c) => c.tokio_write_unencrypted_client(w).await,
            Self::CMSG_SET_AMMO(c) => c.tokio_write_unencrypted_client(w).await,
            Self::CMSG_SET_ACTIVE_MOVER(c) => c.tokio_write_unencrypted_client(w).await,
            Self::CMSG_PET_CANCEL_AURA(c) => c.tokio_write_unencrypted_client(w).await,
            Self::CMSG_CANCEL_AUTO_REPEAT_SPELL(c) => c.tokio_write_unencrypted_client(w).await,
            Self::MSG_LIST_STABLED_PETS(c) => c.tokio_write_unencrypted_client(w).await,
            Self::CMSG_STABLE_PET(c) => c.tokio_write_unencrypted_client(w).await,
            Self::CMSG_UNSTABLE_PET(c) => c.tokio_write_unencrypted_client(w).await,
            Self::CMSG_BUY_STABLE_SLOT(c) => c.tokio_write_unencrypted_client(w).await,
            Self::CMSG_STABLE_SWAP_PET(c) => c.tokio_write_unencrypted_client(w).await,
            Self::CMSG_REQUEST_PET_INFO(c) => c.tokio_write_unencrypted_client(w).await,
            Self::CMSG_FAR_SIGHT(c) => c.tokio_write_unencrypted_client(w).await,
            Self::CMSG_GROUP_CHANGE_SUB_GROUP(c) => c.tokio_write_unencrypted_client(w).await,
            Self::CMSG_REQUEST_PARTY_MEMBER_STATS(c) => c.tokio_write_unencrypted_client(w).await,
            Self::CMSG_GROUP_SWAP_SUB_GROUP(c) => c.tokio_write_unencrypted_client(w).await,
            Self::CMSG_AUTOSTORE_BANK_ITEM(c) => c.tokio_write_unencrypted_client(w).await,
            Self::CMSG_AUTOBANK_ITEM(c) => c.tokio_write_unencrypted_client(w).await,
            Self::MSG_QUERY_NEXT_MAIL_TIME(c) => c.tokio_write_unencrypted_client(w).await,
            Self::CMSG_GROUP_RAID_CONVERT(c) => c.tokio_write_unencrypted_client(w).await,
            Self::CMSG_GROUP_ASSISTANT_LEADER(c) => c.tokio_write_unencrypted_client(w).await,
            Self::CMSG_BUYBACK_ITEM(c) => c.tokio_write_unencrypted_client(w).await,
            Self::CMSG_LFG_GET_STATUS(c) => c.tokio_write_unencrypted_client(w).await,
            Self::CMSG_CANCEL_GROWTH_AURA(c) => c.tokio_write_unencrypted_client(w).await,
            Self::CMSG_LOOT_ROLL(c) => c.tokio_write_unencrypted_client(w).await,
            Self::CMSG_LOOT_MASTER_GIVE(c) => c.tokio_write_unencrypted_client(w).await,
            Self::CMSG_REPAIR_ITEM(c) => c.tokio_write_unencrypted_client(w).await,
            Self::CMSG_SUMMON_RESPONSE(c) => c.tokio_write_unencrypted_client(w).await,
            Self::CMSG_SELF_RES(c) => c.tokio_write_unencrypted_client(w).await,
            Self::CMSG_TOGGLE_HELM(c) => c.tokio_write_unencrypted_client(w).await,
            Self::CMSG_TOGGLE_CLOAK(c) => c.tokio_write_unencrypted_client(w).await,
            Self::CMSG_SET_ACTIONBAR_TOGGLES(c) => c.tokio_write_unencrypted_client(w).await,
            Self::CMSG_ITEM_NAME_QUERY(c) => c.tokio_write_unencrypted_client(w).await,
            Self::CMSG_CHAR_RENAME(c) => c.tokio_write_unencrypted_client(w).await,
            Self::CMSG_MOVE_SPLINE_DONE(c) => c.tokio_write_unencrypted_client(w).await,
            Self::CMSG_MOVE_FALL_RESET(c) => c.tokio_write_unencrypted_client(w).await,
            Self::CMSG_REQUEST_RAID_INFO(c) => c.tokio_write_unencrypted_client(w).await,
            Self::CMSG_MOVE_TIME_SKIPPED(c) => c.tokio_write_unencrypted_client(w).await,
            Self::CMSG_MOVE_FEATHER_FALL_ACK(c) => c.tokio_write_unencrypted_client(w).await,
            Self::CMSG_MOVE_WATER_WALK_ACK(c) => c.tokio_write_unencrypted_client(w).await,
            Self::CMSG_MOVE_NOT_ACTIVE_MOVER(c) => c.tokio_write_unencrypted_client(w).await,
            Self::CMSG_BATTLEFIELD_STATUS(c) => c.tokio_write_unencrypted_client(w).await,
            Self::CMSG_BATTLEFIELD_PORT(c) => c.tokio_write_unencrypted_client(w).await,
            Self::MSG_INSPECT_HONOR_STATS(c) => c.tokio_write_unencrypted_client(w).await,
            Self::CMSG_BATTLEMASTER_HELLO(c) => c.tokio_write_unencrypted_client(w).await,
            Self::CMSG_FORCE_WALK_SPEED_CHANGE_ACK(c) => c.tokio_write_unencrypted_client(w).await,
            Self::CMSG_FORCE_SWIM_BACK_SPEED_CHANGE_ACK(c) => c.tokio_write_unencrypted_client(w).await,
            Self::CMSG_FORCE_TURN_RATE_CHANGE_ACK(c) => c.tokio_write_unencrypted_client(w).await,
            Self::MSG_PVP_LOG_DATA(c) => c.tokio_write_unencrypted_client(w).await,
            Self::CMSG_LEAVE_BATTLEFIELD(c) => c.tokio_write_unencrypted_client(w).await,
            Self::CMSG_AREA_SPIRIT_HEALER_QUERY(c) => c.tokio_write_unencrypted_client(w).await,
            Self::CMSG_AREA_SPIRIT_HEALER_QUEUE(c) => c.tokio_write_unencrypted_client(w).await,
            Self::MSG_BATTLEGROUND_PLAYER_POSITIONS(c) => c.tokio_write_unencrypted_client(w).await,
            Self::CMSG_PET_STOP_ATTACK(c) => c.tokio_write_unencrypted_client(w).await,
            Self::CMSG_BATTLEMASTER_JOIN(c) => c.tokio_write_unencrypted_client(w).await,
            Self::CMSG_PET_UNLEARN(c) => c.tokio_write_unencrypted_client(w).await,
            Self::CMSG_PET_SPELL_AUTOCAST(c) => c.tokio_write_unencrypted_client(w).await,
            Self::CMSG_GUILD_INFO_TEXT(c) => c.tokio_write_unencrypted_client(w).await,
            Self::CMSG_ACTIVATETAXIEXPRESS(c) => c.tokio_write_unencrypted_client(w).await,
            Self::CMSG_SET_FACTION_INACTIVE(c) => c.tokio_write_unencrypted_client(w).await,
            Self::CMSG_SET_WATCHED_FACTION(c) => c.tokio_write_unencrypted_client(w).await,
            Self::CMSG_RESET_INSTANCES(c) => c.tokio_write_unencrypted_client(w).await,
            Self::MSG_RAID_READY_CHECK(c) => c.tokio_write_unencrypted_client(w).await,
            Self::CMSG_GMSURVEY_SUBMIT(c) => c.tokio_write_unencrypted_client(w).await,
            Self::CMSG_MOVE_SET_FLY(c) => c.tokio_write_unencrypted_client(w).await,
            Self::CMSG_LFD_PLAYER_LOCK_INFO_REQUEST(c) => c.tokio_write_unencrypted_client(w).await,
            Self::CMSG_REALM_SPLIT(c) => c.tokio_write_unencrypted_client(w).await,
            Self::CMSG_MOVE_CHNG_TRANSPORT(c) => c.tokio_write_unencrypted_client(w).await,
            Self::CMSG_TIME_SYNC_RESP(c) => c.tokio_write_unencrypted_client(w).await,
            Self::CMSG_VOICE_SESSION_ENABLE(c) => c.tokio_write_unencrypted_client(w).await,
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
            Self::MSG_QUEST_PUSH_RESULT(c) => c.astd_write_encrypted_client(w, e).await,
            Self::MSG_PETITION_RENAME(c) => c.astd_write_encrypted_client(w, e).await,
            Self::MSG_SET_DUNGEON_DIFFICULTY(c) => c.astd_write_encrypted_client(w, e).await,
            Self::MSG_MOVE_START_ASCEND(c) => c.astd_write_encrypted_client(w, e).await,
            Self::MSG_MOVE_STOP_ASCEND(c) => c.astd_write_encrypted_client(w, e).await,
            Self::MSG_MOVE_START_DESCEND(c) => c.astd_write_encrypted_client(w, e).await,
            Self::MSG_GUILD_BANK_MONEY_WITHDRAWN(c) => c.astd_write_encrypted_client(w, e).await,
            Self::CMSG_CALENDAR_GET_NUM_PENDING(c) => c.astd_write_encrypted_client(w, e).await,
            Self::CMSG_BOOTME(c) => c.astd_write_encrypted_client(w, e).await,
            Self::CMSG_WORLD_TELEPORT(c) => c.astd_write_encrypted_client(w, e).await,
            Self::CMSG_TELEPORT_TO_UNIT(c) => c.astd_write_encrypted_client(w, e).await,
            Self::CMSG_CHAR_CREATE(c) => c.astd_write_encrypted_client(w, e).await,
            Self::CMSG_CHAR_ENUM(c) => c.astd_write_encrypted_client(w, e).await,
            Self::CMSG_CHAR_DELETE(c) => c.astd_write_encrypted_client(w, e).await,
            Self::CMSG_PLAYER_LOGIN(c) => c.astd_write_encrypted_client(w, e).await,
            Self::CMSG_PLAYER_LOGOUT(c) => c.astd_write_encrypted_client(w, e).await,
            Self::CMSG_LOGOUT_REQUEST(c) => c.astd_write_encrypted_client(w, e).await,
            Self::CMSG_LOGOUT_CANCEL(c) => c.astd_write_encrypted_client(w, e).await,
            Self::CMSG_NAME_QUERY(c) => c.astd_write_encrypted_client(w, e).await,
            Self::CMSG_PET_NAME_QUERY(c) => c.astd_write_encrypted_client(w, e).await,
            Self::CMSG_GUILD_QUERY(c) => c.astd_write_encrypted_client(w, e).await,
            Self::CMSG_ITEM_QUERY_SINGLE(c) => c.astd_write_encrypted_client(w, e).await,
            Self::CMSG_PAGE_TEXT_QUERY(c) => c.astd_write_encrypted_client(w, e).await,
            Self::CMSG_QUEST_QUERY(c) => c.astd_write_encrypted_client(w, e).await,
            Self::CMSG_GAMEOBJECT_QUERY(c) => c.astd_write_encrypted_client(w, e).await,
            Self::CMSG_CREATURE_QUERY(c) => c.astd_write_encrypted_client(w, e).await,
            Self::CMSG_WHO(c) => c.astd_write_encrypted_client(w, e).await,
            Self::CMSG_WHOIS(c) => c.astd_write_encrypted_client(w, e).await,
            Self::CMSG_CONTACT_LIST(c) => c.astd_write_encrypted_client(w, e).await,
            Self::CMSG_ADD_FRIEND(c) => c.astd_write_encrypted_client(w, e).await,
            Self::CMSG_DEL_FRIEND(c) => c.astd_write_encrypted_client(w, e).await,
            Self::CMSG_ADD_IGNORE(c) => c.astd_write_encrypted_client(w, e).await,
            Self::CMSG_DEL_IGNORE(c) => c.astd_write_encrypted_client(w, e).await,
            Self::CMSG_GROUP_INVITE(c) => c.astd_write_encrypted_client(w, e).await,
            Self::CMSG_GROUP_ACCEPT(c) => c.astd_write_encrypted_client(w, e).await,
            Self::CMSG_GROUP_DECLINE(c) => c.astd_write_encrypted_client(w, e).await,
            Self::CMSG_GROUP_UNINVITE(c) => c.astd_write_encrypted_client(w, e).await,
            Self::CMSG_GROUP_UNINVITE_GUID(c) => c.astd_write_encrypted_client(w, e).await,
            Self::CMSG_GROUP_SET_LEADER(c) => c.astd_write_encrypted_client(w, e).await,
            Self::CMSG_LOOT_METHOD(c) => c.astd_write_encrypted_client(w, e).await,
            Self::CMSG_GROUP_DISBAND(c) => c.astd_write_encrypted_client(w, e).await,
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
            Self::CMSG_USE_ITEM(c) => c.astd_write_encrypted_client(w, e).await,
            Self::CMSG_OPEN_ITEM(c) => c.astd_write_encrypted_client(w, e).await,
            Self::CMSG_READ_ITEM(c) => c.astd_write_encrypted_client(w, e).await,
            Self::CMSG_GAMEOBJ_USE(c) => c.astd_write_encrypted_client(w, e).await,
            Self::CMSG_AREATRIGGER(c) => c.astd_write_encrypted_client(w, e).await,
            Self::MSG_MOVE_TELEPORT_ACK(c) => c.astd_write_encrypted_client(w, e).await,
            Self::CMSG_MOVE_SET_RAW_POSITION(c) => c.astd_write_encrypted_client(w, e).await,
            Self::CMSG_FORCE_RUN_SPEED_CHANGE_ACK(c) => c.astd_write_encrypted_client(w, e).await,
            Self::CMSG_FORCE_RUN_BACK_SPEED_CHANGE_ACK(c) => c.astd_write_encrypted_client(w, e).await,
            Self::CMSG_FORCE_SWIM_SPEED_CHANGE_ACK(c) => c.astd_write_encrypted_client(w, e).await,
            Self::CMSG_FORCE_MOVE_ROOT_ACK(c) => c.astd_write_encrypted_client(w, e).await,
            Self::CMSG_FORCE_MOVE_UNROOT_ACK(c) => c.astd_write_encrypted_client(w, e).await,
            Self::CMSG_MOVE_KNOCK_BACK_ACK(c) => c.astd_write_encrypted_client(w, e).await,
            Self::CMSG_MOVE_HOVER_ACK(c) => c.astd_write_encrypted_client(w, e).await,
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
            Self::CMSG_INITIATE_TRADE(c) => c.astd_write_encrypted_client(w, e).await,
            Self::CMSG_BEGIN_TRADE(c) => c.astd_write_encrypted_client(w, e).await,
            Self::CMSG_BUSY_TRADE(c) => c.astd_write_encrypted_client(w, e).await,
            Self::CMSG_IGNORE_TRADE(c) => c.astd_write_encrypted_client(w, e).await,
            Self::CMSG_ACCEPT_TRADE(c) => c.astd_write_encrypted_client(w, e).await,
            Self::CMSG_UNACCEPT_TRADE(c) => c.astd_write_encrypted_client(w, e).await,
            Self::CMSG_CANCEL_TRADE(c) => c.astd_write_encrypted_client(w, e).await,
            Self::CMSG_SET_TRADE_ITEM(c) => c.astd_write_encrypted_client(w, e).await,
            Self::CMSG_CLEAR_TRADE_ITEM(c) => c.astd_write_encrypted_client(w, e).await,
            Self::CMSG_SET_TRADE_GOLD(c) => c.astd_write_encrypted_client(w, e).await,
            Self::CMSG_SET_FACTION_ATWAR(c) => c.astd_write_encrypted_client(w, e).await,
            Self::CMSG_SET_ACTION_BUTTON(c) => c.astd_write_encrypted_client(w, e).await,
            Self::CMSG_CAST_SPELL(c) => c.astd_write_encrypted_client(w, e).await,
            Self::CMSG_CANCEL_CAST(c) => c.astd_write_encrypted_client(w, e).await,
            Self::CMSG_CANCEL_AURA(c) => c.astd_write_encrypted_client(w, e).await,
            Self::CMSG_CANCEL_CHANNELLING(c) => c.astd_write_encrypted_client(w, e).await,
            Self::CMSG_SET_SELECTION(c) => c.astd_write_encrypted_client(w, e).await,
            Self::CMSG_DELETEEQUIPMENT_SET(c) => c.astd_write_encrypted_client(w, e).await,
            Self::CMSG_ATTACKSWING(c) => c.astd_write_encrypted_client(w, e).await,
            Self::CMSG_ATTACKSTOP(c) => c.astd_write_encrypted_client(w, e).await,
            Self::CMSG_REPOP_REQUEST(c) => c.astd_write_encrypted_client(w, e).await,
            Self::CMSG_RESURRECT_RESPONSE(c) => c.astd_write_encrypted_client(w, e).await,
            Self::CMSG_LOOT(c) => c.astd_write_encrypted_client(w, e).await,
            Self::CMSG_LOOT_MONEY(c) => c.astd_write_encrypted_client(w, e).await,
            Self::CMSG_LOOT_RELEASE(c) => c.astd_write_encrypted_client(w, e).await,
            Self::CMSG_DUEL_ACCEPTED(c) => c.astd_write_encrypted_client(w, e).await,
            Self::CMSG_DUEL_CANCELLED(c) => c.astd_write_encrypted_client(w, e).await,
            Self::CMSG_MOUNTSPECIAL_ANIM(c) => c.astd_write_encrypted_client(w, e).await,
            Self::CMSG_PET_SET_ACTION(c) => c.astd_write_encrypted_client(w, e).await,
            Self::CMSG_PET_ACTION(c) => c.astd_write_encrypted_client(w, e).await,
            Self::CMSG_PET_ABANDON(c) => c.astd_write_encrypted_client(w, e).await,
            Self::CMSG_PET_RENAME(c) => c.astd_write_encrypted_client(w, e).await,
            Self::CMSG_GOSSIP_HELLO(c) => c.astd_write_encrypted_client(w, e).await,
            Self::CMSG_GOSSIP_SELECT_OPTION(c) => c.astd_write_encrypted_client(w, e).await,
            Self::CMSG_NPC_TEXT_QUERY(c) => c.astd_write_encrypted_client(w, e).await,
            Self::CMSG_QUESTGIVER_STATUS_QUERY(c) => c.astd_write_encrypted_client(w, e).await,
            Self::CMSG_QUESTGIVER_HELLO(c) => c.astd_write_encrypted_client(w, e).await,
            Self::CMSG_QUESTGIVER_QUERY_QUEST(c) => c.astd_write_encrypted_client(w, e).await,
            Self::CMSG_QUESTGIVER_QUEST_AUTOLAUNCH(c) => c.astd_write_encrypted_client(w, e).await,
            Self::CMSG_QUESTGIVER_ACCEPT_QUEST(c) => c.astd_write_encrypted_client(w, e).await,
            Self::CMSG_QUESTGIVER_COMPLETE_QUEST(c) => c.astd_write_encrypted_client(w, e).await,
            Self::CMSG_QUESTGIVER_REQUEST_REWARD(c) => c.astd_write_encrypted_client(w, e).await,
            Self::CMSG_QUESTGIVER_CHOOSE_REWARD(c) => c.astd_write_encrypted_client(w, e).await,
            Self::CMSG_QUESTGIVER_CANCEL(c) => c.astd_write_encrypted_client(w, e).await,
            Self::CMSG_QUESTLOG_SWAP_QUEST(c) => c.astd_write_encrypted_client(w, e).await,
            Self::CMSG_QUESTLOG_REMOVE_QUEST(c) => c.astd_write_encrypted_client(w, e).await,
            Self::CMSG_QUEST_CONFIRM_ACCEPT(c) => c.astd_write_encrypted_client(w, e).await,
            Self::CMSG_PUSHQUESTTOPARTY(c) => c.astd_write_encrypted_client(w, e).await,
            Self::CMSG_LIST_INVENTORY(c) => c.astd_write_encrypted_client(w, e).await,
            Self::CMSG_SELL_ITEM(c) => c.astd_write_encrypted_client(w, e).await,
            Self::CMSG_BUY_ITEM(c) => c.astd_write_encrypted_client(w, e).await,
            Self::CMSG_BUY_ITEM_IN_SLOT(c) => c.astd_write_encrypted_client(w, e).await,
            Self::CMSG_TAXINODE_STATUS_QUERY(c) => c.astd_write_encrypted_client(w, e).await,
            Self::CMSG_TAXIQUERYAVAILABLENODES(c) => c.astd_write_encrypted_client(w, e).await,
            Self::CMSG_ACTIVATETAXI(c) => c.astd_write_encrypted_client(w, e).await,
            Self::CMSG_TRAINER_LIST(c) => c.astd_write_encrypted_client(w, e).await,
            Self::CMSG_TRAINER_BUY_SPELL(c) => c.astd_write_encrypted_client(w, e).await,
            Self::CMSG_BINDER_ACTIVATE(c) => c.astd_write_encrypted_client(w, e).await,
            Self::CMSG_BANKER_ACTIVATE(c) => c.astd_write_encrypted_client(w, e).await,
            Self::CMSG_BUY_BANK_SLOT(c) => c.astd_write_encrypted_client(w, e).await,
            Self::CMSG_PETITION_SHOWLIST(c) => c.astd_write_encrypted_client(w, e).await,
            Self::CMSG_PETITION_BUY(c) => c.astd_write_encrypted_client(w, e).await,
            Self::CMSG_PETITION_SHOW_SIGNATURES(c) => c.astd_write_encrypted_client(w, e).await,
            Self::CMSG_PETITION_SIGN(c) => c.astd_write_encrypted_client(w, e).await,
            Self::CMSG_OFFER_PETITION(c) => c.astd_write_encrypted_client(w, e).await,
            Self::CMSG_TURN_IN_PETITION(c) => c.astd_write_encrypted_client(w, e).await,
            Self::CMSG_PETITION_QUERY(c) => c.astd_write_encrypted_client(w, e).await,
            Self::CMSG_BUG(c) => c.astd_write_encrypted_client(w, e).await,
            Self::CMSG_PLAYED_TIME(c) => c.astd_write_encrypted_client(w, e).await,
            Self::CMSG_QUERY_TIME(c) => c.astd_write_encrypted_client(w, e).await,
            Self::CMSG_RECLAIM_CORPSE(c) => c.astd_write_encrypted_client(w, e).await,
            Self::CMSG_WRAP_ITEM(c) => c.astd_write_encrypted_client(w, e).await,
            Self::MSG_MINIMAP_PING(c) => c.astd_write_encrypted_client(w, e).await,
            Self::CMSG_PING(c) => c.astd_write_encrypted_client(w, e).await,
            Self::CMSG_SETSHEATHED(c) => c.astd_write_encrypted_client(w, e).await,
            Self::CMSG_AUTH_SESSION(c) => c.astd_write_encrypted_client(w, e).await,
            Self::CMSG_PET_CAST_SPELL(c) => c.astd_write_encrypted_client(w, e).await,
            Self::MSG_SAVE_GUILD_EMBLEM(c) => c.astd_write_encrypted_client(w, e).await,
            Self::CMSG_ZONEUPDATE(c) => c.astd_write_encrypted_client(w, e).await,
            Self::MSG_RANDOM_ROLL(c) => c.astd_write_encrypted_client(w, e).await,
            Self::CMSG_UNLEARN_SKILL(c) => c.astd_write_encrypted_client(w, e).await,
            Self::CMSG_GMTICKET_CREATE(c) => c.astd_write_encrypted_client(w, e).await,
            Self::CMSG_GMTICKET_UPDATETEXT(c) => c.astd_write_encrypted_client(w, e).await,
            Self::CMSG_REQUEST_ACCOUNT_DATA(c) => c.astd_write_encrypted_client(w, e).await,
            Self::CMSG_UPDATE_ACCOUNT_DATA(c) => c.astd_write_encrypted_client(w, e).await,
            Self::CMSG_GMTICKET_GETTICKET(c) => c.astd_write_encrypted_client(w, e).await,
            Self::MSG_CORPSE_QUERY(c) => c.astd_write_encrypted_client(w, e).await,
            Self::CMSG_GMTICKET_DELETETICKET(c) => c.astd_write_encrypted_client(w, e).await,
            Self::CMSG_GMTICKET_SYSTEMSTATUS(c) => c.astd_write_encrypted_client(w, e).await,
            Self::CMSG_SPIRIT_HEALER_ACTIVATE(c) => c.astd_write_encrypted_client(w, e).await,
            Self::CMSG_CHAT_IGNORED(c) => c.astd_write_encrypted_client(w, e).await,
            Self::CMSG_GUILD_RANK(c) => c.astd_write_encrypted_client(w, e).await,
            Self::CMSG_GUILD_ADD_RANK(c) => c.astd_write_encrypted_client(w, e).await,
            Self::CMSG_GUILD_DEL_RANK(c) => c.astd_write_encrypted_client(w, e).await,
            Self::CMSG_GUILD_SET_PUBLIC_NOTE(c) => c.astd_write_encrypted_client(w, e).await,
            Self::CMSG_GUILD_SET_OFFICER_NOTE(c) => c.astd_write_encrypted_client(w, e).await,
            Self::CMSG_SEND_MAIL(c) => c.astd_write_encrypted_client(w, e).await,
            Self::CMSG_GET_MAIL_LIST(c) => c.astd_write_encrypted_client(w, e).await,
            Self::CMSG_BATTLEFIELD_LIST(c) => c.astd_write_encrypted_client(w, e).await,
            Self::CMSG_ITEM_TEXT_QUERY(c) => c.astd_write_encrypted_client(w, e).await,
            Self::CMSG_MAIL_TAKE_MONEY(c) => c.astd_write_encrypted_client(w, e).await,
            Self::CMSG_MAIL_TAKE_ITEM(c) => c.astd_write_encrypted_client(w, e).await,
            Self::CMSG_MAIL_MARK_AS_READ(c) => c.astd_write_encrypted_client(w, e).await,
            Self::CMSG_MAIL_RETURN_TO_SENDER(c) => c.astd_write_encrypted_client(w, e).await,
            Self::CMSG_MAIL_DELETE(c) => c.astd_write_encrypted_client(w, e).await,
            Self::CMSG_MAIL_CREATE_TEXT_ITEM(c) => c.astd_write_encrypted_client(w, e).await,
            Self::CMSG_LEARN_TALENT(c) => c.astd_write_encrypted_client(w, e).await,
            Self::CMSG_TOGGLE_PVP(c) => c.astd_write_encrypted_client(w, e).await,
            Self::MSG_AUCTION_HELLO(c) => c.astd_write_encrypted_client(w, e).await,
            Self::CMSG_AUCTION_SELL_ITEM(c) => c.astd_write_encrypted_client(w, e).await,
            Self::CMSG_AUCTION_REMOVE_ITEM(c) => c.astd_write_encrypted_client(w, e).await,
            Self::CMSG_AUCTION_LIST_ITEMS(c) => c.astd_write_encrypted_client(w, e).await,
            Self::CMSG_AUCTION_LIST_OWNER_ITEMS(c) => c.astd_write_encrypted_client(w, e).await,
            Self::CMSG_AUCTION_PLACE_BID(c) => c.astd_write_encrypted_client(w, e).await,
            Self::CMSG_AUCTION_LIST_BIDDER_ITEMS(c) => c.astd_write_encrypted_client(w, e).await,
            Self::CMSG_SET_AMMO(c) => c.astd_write_encrypted_client(w, e).await,
            Self::CMSG_SET_ACTIVE_MOVER(c) => c.astd_write_encrypted_client(w, e).await,
            Self::CMSG_PET_CANCEL_AURA(c) => c.astd_write_encrypted_client(w, e).await,
            Self::CMSG_CANCEL_AUTO_REPEAT_SPELL(c) => c.astd_write_encrypted_client(w, e).await,
            Self::MSG_LIST_STABLED_PETS(c) => c.astd_write_encrypted_client(w, e).await,
            Self::CMSG_STABLE_PET(c) => c.astd_write_encrypted_client(w, e).await,
            Self::CMSG_UNSTABLE_PET(c) => c.astd_write_encrypted_client(w, e).await,
            Self::CMSG_BUY_STABLE_SLOT(c) => c.astd_write_encrypted_client(w, e).await,
            Self::CMSG_STABLE_SWAP_PET(c) => c.astd_write_encrypted_client(w, e).await,
            Self::CMSG_REQUEST_PET_INFO(c) => c.astd_write_encrypted_client(w, e).await,
            Self::CMSG_FAR_SIGHT(c) => c.astd_write_encrypted_client(w, e).await,
            Self::CMSG_GROUP_CHANGE_SUB_GROUP(c) => c.astd_write_encrypted_client(w, e).await,
            Self::CMSG_REQUEST_PARTY_MEMBER_STATS(c) => c.astd_write_encrypted_client(w, e).await,
            Self::CMSG_GROUP_SWAP_SUB_GROUP(c) => c.astd_write_encrypted_client(w, e).await,
            Self::CMSG_AUTOSTORE_BANK_ITEM(c) => c.astd_write_encrypted_client(w, e).await,
            Self::CMSG_AUTOBANK_ITEM(c) => c.astd_write_encrypted_client(w, e).await,
            Self::MSG_QUERY_NEXT_MAIL_TIME(c) => c.astd_write_encrypted_client(w, e).await,
            Self::CMSG_GROUP_RAID_CONVERT(c) => c.astd_write_encrypted_client(w, e).await,
            Self::CMSG_GROUP_ASSISTANT_LEADER(c) => c.astd_write_encrypted_client(w, e).await,
            Self::CMSG_BUYBACK_ITEM(c) => c.astd_write_encrypted_client(w, e).await,
            Self::CMSG_LFG_GET_STATUS(c) => c.astd_write_encrypted_client(w, e).await,
            Self::CMSG_CANCEL_GROWTH_AURA(c) => c.astd_write_encrypted_client(w, e).await,
            Self::CMSG_LOOT_ROLL(c) => c.astd_write_encrypted_client(w, e).await,
            Self::CMSG_LOOT_MASTER_GIVE(c) => c.astd_write_encrypted_client(w, e).await,
            Self::CMSG_REPAIR_ITEM(c) => c.astd_write_encrypted_client(w, e).await,
            Self::CMSG_SUMMON_RESPONSE(c) => c.astd_write_encrypted_client(w, e).await,
            Self::CMSG_SELF_RES(c) => c.astd_write_encrypted_client(w, e).await,
            Self::CMSG_TOGGLE_HELM(c) => c.astd_write_encrypted_client(w, e).await,
            Self::CMSG_TOGGLE_CLOAK(c) => c.astd_write_encrypted_client(w, e).await,
            Self::CMSG_SET_ACTIONBAR_TOGGLES(c) => c.astd_write_encrypted_client(w, e).await,
            Self::CMSG_ITEM_NAME_QUERY(c) => c.astd_write_encrypted_client(w, e).await,
            Self::CMSG_CHAR_RENAME(c) => c.astd_write_encrypted_client(w, e).await,
            Self::CMSG_MOVE_SPLINE_DONE(c) => c.astd_write_encrypted_client(w, e).await,
            Self::CMSG_MOVE_FALL_RESET(c) => c.astd_write_encrypted_client(w, e).await,
            Self::CMSG_REQUEST_RAID_INFO(c) => c.astd_write_encrypted_client(w, e).await,
            Self::CMSG_MOVE_TIME_SKIPPED(c) => c.astd_write_encrypted_client(w, e).await,
            Self::CMSG_MOVE_FEATHER_FALL_ACK(c) => c.astd_write_encrypted_client(w, e).await,
            Self::CMSG_MOVE_WATER_WALK_ACK(c) => c.astd_write_encrypted_client(w, e).await,
            Self::CMSG_MOVE_NOT_ACTIVE_MOVER(c) => c.astd_write_encrypted_client(w, e).await,
            Self::CMSG_BATTLEFIELD_STATUS(c) => c.astd_write_encrypted_client(w, e).await,
            Self::CMSG_BATTLEFIELD_PORT(c) => c.astd_write_encrypted_client(w, e).await,
            Self::MSG_INSPECT_HONOR_STATS(c) => c.astd_write_encrypted_client(w, e).await,
            Self::CMSG_BATTLEMASTER_HELLO(c) => c.astd_write_encrypted_client(w, e).await,
            Self::CMSG_FORCE_WALK_SPEED_CHANGE_ACK(c) => c.astd_write_encrypted_client(w, e).await,
            Self::CMSG_FORCE_SWIM_BACK_SPEED_CHANGE_ACK(c) => c.astd_write_encrypted_client(w, e).await,
            Self::CMSG_FORCE_TURN_RATE_CHANGE_ACK(c) => c.astd_write_encrypted_client(w, e).await,
            Self::MSG_PVP_LOG_DATA(c) => c.astd_write_encrypted_client(w, e).await,
            Self::CMSG_LEAVE_BATTLEFIELD(c) => c.astd_write_encrypted_client(w, e).await,
            Self::CMSG_AREA_SPIRIT_HEALER_QUERY(c) => c.astd_write_encrypted_client(w, e).await,
            Self::CMSG_AREA_SPIRIT_HEALER_QUEUE(c) => c.astd_write_encrypted_client(w, e).await,
            Self::MSG_BATTLEGROUND_PLAYER_POSITIONS(c) => c.astd_write_encrypted_client(w, e).await,
            Self::CMSG_PET_STOP_ATTACK(c) => c.astd_write_encrypted_client(w, e).await,
            Self::CMSG_BATTLEMASTER_JOIN(c) => c.astd_write_encrypted_client(w, e).await,
            Self::CMSG_PET_UNLEARN(c) => c.astd_write_encrypted_client(w, e).await,
            Self::CMSG_PET_SPELL_AUTOCAST(c) => c.astd_write_encrypted_client(w, e).await,
            Self::CMSG_GUILD_INFO_TEXT(c) => c.astd_write_encrypted_client(w, e).await,
            Self::CMSG_ACTIVATETAXIEXPRESS(c) => c.astd_write_encrypted_client(w, e).await,
            Self::CMSG_SET_FACTION_INACTIVE(c) => c.astd_write_encrypted_client(w, e).await,
            Self::CMSG_SET_WATCHED_FACTION(c) => c.astd_write_encrypted_client(w, e).await,
            Self::CMSG_RESET_INSTANCES(c) => c.astd_write_encrypted_client(w, e).await,
            Self::MSG_RAID_READY_CHECK(c) => c.astd_write_encrypted_client(w, e).await,
            Self::CMSG_GMSURVEY_SUBMIT(c) => c.astd_write_encrypted_client(w, e).await,
            Self::CMSG_MOVE_SET_FLY(c) => c.astd_write_encrypted_client(w, e).await,
            Self::CMSG_LFD_PLAYER_LOCK_INFO_REQUEST(c) => c.astd_write_encrypted_client(w, e).await,
            Self::CMSG_REALM_SPLIT(c) => c.astd_write_encrypted_client(w, e).await,
            Self::CMSG_MOVE_CHNG_TRANSPORT(c) => c.astd_write_encrypted_client(w, e).await,
            Self::CMSG_TIME_SYNC_RESP(c) => c.astd_write_encrypted_client(w, e).await,
            Self::CMSG_VOICE_SESSION_ENABLE(c) => c.astd_write_encrypted_client(w, e).await,
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
            Self::MSG_QUEST_PUSH_RESULT(c) => c.astd_write_unencrypted_client(w).await,
            Self::MSG_PETITION_RENAME(c) => c.astd_write_unencrypted_client(w).await,
            Self::MSG_SET_DUNGEON_DIFFICULTY(c) => c.astd_write_unencrypted_client(w).await,
            Self::MSG_MOVE_START_ASCEND(c) => c.astd_write_unencrypted_client(w).await,
            Self::MSG_MOVE_STOP_ASCEND(c) => c.astd_write_unencrypted_client(w).await,
            Self::MSG_MOVE_START_DESCEND(c) => c.astd_write_unencrypted_client(w).await,
            Self::MSG_GUILD_BANK_MONEY_WITHDRAWN(c) => c.astd_write_unencrypted_client(w).await,
            Self::CMSG_CALENDAR_GET_NUM_PENDING(c) => c.astd_write_unencrypted_client(w).await,
            Self::CMSG_BOOTME(c) => c.astd_write_unencrypted_client(w).await,
            Self::CMSG_WORLD_TELEPORT(c) => c.astd_write_unencrypted_client(w).await,
            Self::CMSG_TELEPORT_TO_UNIT(c) => c.astd_write_unencrypted_client(w).await,
            Self::CMSG_CHAR_CREATE(c) => c.astd_write_unencrypted_client(w).await,
            Self::CMSG_CHAR_ENUM(c) => c.astd_write_unencrypted_client(w).await,
            Self::CMSG_CHAR_DELETE(c) => c.astd_write_unencrypted_client(w).await,
            Self::CMSG_PLAYER_LOGIN(c) => c.astd_write_unencrypted_client(w).await,
            Self::CMSG_PLAYER_LOGOUT(c) => c.astd_write_unencrypted_client(w).await,
            Self::CMSG_LOGOUT_REQUEST(c) => c.astd_write_unencrypted_client(w).await,
            Self::CMSG_LOGOUT_CANCEL(c) => c.astd_write_unencrypted_client(w).await,
            Self::CMSG_NAME_QUERY(c) => c.astd_write_unencrypted_client(w).await,
            Self::CMSG_PET_NAME_QUERY(c) => c.astd_write_unencrypted_client(w).await,
            Self::CMSG_GUILD_QUERY(c) => c.astd_write_unencrypted_client(w).await,
            Self::CMSG_ITEM_QUERY_SINGLE(c) => c.astd_write_unencrypted_client(w).await,
            Self::CMSG_PAGE_TEXT_QUERY(c) => c.astd_write_unencrypted_client(w).await,
            Self::CMSG_QUEST_QUERY(c) => c.astd_write_unencrypted_client(w).await,
            Self::CMSG_GAMEOBJECT_QUERY(c) => c.astd_write_unencrypted_client(w).await,
            Self::CMSG_CREATURE_QUERY(c) => c.astd_write_unencrypted_client(w).await,
            Self::CMSG_WHO(c) => c.astd_write_unencrypted_client(w).await,
            Self::CMSG_WHOIS(c) => c.astd_write_unencrypted_client(w).await,
            Self::CMSG_CONTACT_LIST(c) => c.astd_write_unencrypted_client(w).await,
            Self::CMSG_ADD_FRIEND(c) => c.astd_write_unencrypted_client(w).await,
            Self::CMSG_DEL_FRIEND(c) => c.astd_write_unencrypted_client(w).await,
            Self::CMSG_ADD_IGNORE(c) => c.astd_write_unencrypted_client(w).await,
            Self::CMSG_DEL_IGNORE(c) => c.astd_write_unencrypted_client(w).await,
            Self::CMSG_GROUP_INVITE(c) => c.astd_write_unencrypted_client(w).await,
            Self::CMSG_GROUP_ACCEPT(c) => c.astd_write_unencrypted_client(w).await,
            Self::CMSG_GROUP_DECLINE(c) => c.astd_write_unencrypted_client(w).await,
            Self::CMSG_GROUP_UNINVITE(c) => c.astd_write_unencrypted_client(w).await,
            Self::CMSG_GROUP_UNINVITE_GUID(c) => c.astd_write_unencrypted_client(w).await,
            Self::CMSG_GROUP_SET_LEADER(c) => c.astd_write_unencrypted_client(w).await,
            Self::CMSG_LOOT_METHOD(c) => c.astd_write_unencrypted_client(w).await,
            Self::CMSG_GROUP_DISBAND(c) => c.astd_write_unencrypted_client(w).await,
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
            Self::CMSG_USE_ITEM(c) => c.astd_write_unencrypted_client(w).await,
            Self::CMSG_OPEN_ITEM(c) => c.astd_write_unencrypted_client(w).await,
            Self::CMSG_READ_ITEM(c) => c.astd_write_unencrypted_client(w).await,
            Self::CMSG_GAMEOBJ_USE(c) => c.astd_write_unencrypted_client(w).await,
            Self::CMSG_AREATRIGGER(c) => c.astd_write_unencrypted_client(w).await,
            Self::MSG_MOVE_TELEPORT_ACK(c) => c.astd_write_unencrypted_client(w).await,
            Self::CMSG_MOVE_SET_RAW_POSITION(c) => c.astd_write_unencrypted_client(w).await,
            Self::CMSG_FORCE_RUN_SPEED_CHANGE_ACK(c) => c.astd_write_unencrypted_client(w).await,
            Self::CMSG_FORCE_RUN_BACK_SPEED_CHANGE_ACK(c) => c.astd_write_unencrypted_client(w).await,
            Self::CMSG_FORCE_SWIM_SPEED_CHANGE_ACK(c) => c.astd_write_unencrypted_client(w).await,
            Self::CMSG_FORCE_MOVE_ROOT_ACK(c) => c.astd_write_unencrypted_client(w).await,
            Self::CMSG_FORCE_MOVE_UNROOT_ACK(c) => c.astd_write_unencrypted_client(w).await,
            Self::CMSG_MOVE_KNOCK_BACK_ACK(c) => c.astd_write_unencrypted_client(w).await,
            Self::CMSG_MOVE_HOVER_ACK(c) => c.astd_write_unencrypted_client(w).await,
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
            Self::CMSG_INITIATE_TRADE(c) => c.astd_write_unencrypted_client(w).await,
            Self::CMSG_BEGIN_TRADE(c) => c.astd_write_unencrypted_client(w).await,
            Self::CMSG_BUSY_TRADE(c) => c.astd_write_unencrypted_client(w).await,
            Self::CMSG_IGNORE_TRADE(c) => c.astd_write_unencrypted_client(w).await,
            Self::CMSG_ACCEPT_TRADE(c) => c.astd_write_unencrypted_client(w).await,
            Self::CMSG_UNACCEPT_TRADE(c) => c.astd_write_unencrypted_client(w).await,
            Self::CMSG_CANCEL_TRADE(c) => c.astd_write_unencrypted_client(w).await,
            Self::CMSG_SET_TRADE_ITEM(c) => c.astd_write_unencrypted_client(w).await,
            Self::CMSG_CLEAR_TRADE_ITEM(c) => c.astd_write_unencrypted_client(w).await,
            Self::CMSG_SET_TRADE_GOLD(c) => c.astd_write_unencrypted_client(w).await,
            Self::CMSG_SET_FACTION_ATWAR(c) => c.astd_write_unencrypted_client(w).await,
            Self::CMSG_SET_ACTION_BUTTON(c) => c.astd_write_unencrypted_client(w).await,
            Self::CMSG_CAST_SPELL(c) => c.astd_write_unencrypted_client(w).await,
            Self::CMSG_CANCEL_CAST(c) => c.astd_write_unencrypted_client(w).await,
            Self::CMSG_CANCEL_AURA(c) => c.astd_write_unencrypted_client(w).await,
            Self::CMSG_CANCEL_CHANNELLING(c) => c.astd_write_unencrypted_client(w).await,
            Self::CMSG_SET_SELECTION(c) => c.astd_write_unencrypted_client(w).await,
            Self::CMSG_DELETEEQUIPMENT_SET(c) => c.astd_write_unencrypted_client(w).await,
            Self::CMSG_ATTACKSWING(c) => c.astd_write_unencrypted_client(w).await,
            Self::CMSG_ATTACKSTOP(c) => c.astd_write_unencrypted_client(w).await,
            Self::CMSG_REPOP_REQUEST(c) => c.astd_write_unencrypted_client(w).await,
            Self::CMSG_RESURRECT_RESPONSE(c) => c.astd_write_unencrypted_client(w).await,
            Self::CMSG_LOOT(c) => c.astd_write_unencrypted_client(w).await,
            Self::CMSG_LOOT_MONEY(c) => c.astd_write_unencrypted_client(w).await,
            Self::CMSG_LOOT_RELEASE(c) => c.astd_write_unencrypted_client(w).await,
            Self::CMSG_DUEL_ACCEPTED(c) => c.astd_write_unencrypted_client(w).await,
            Self::CMSG_DUEL_CANCELLED(c) => c.astd_write_unencrypted_client(w).await,
            Self::CMSG_MOUNTSPECIAL_ANIM(c) => c.astd_write_unencrypted_client(w).await,
            Self::CMSG_PET_SET_ACTION(c) => c.astd_write_unencrypted_client(w).await,
            Self::CMSG_PET_ACTION(c) => c.astd_write_unencrypted_client(w).await,
            Self::CMSG_PET_ABANDON(c) => c.astd_write_unencrypted_client(w).await,
            Self::CMSG_PET_RENAME(c) => c.astd_write_unencrypted_client(w).await,
            Self::CMSG_GOSSIP_HELLO(c) => c.astd_write_unencrypted_client(w).await,
            Self::CMSG_GOSSIP_SELECT_OPTION(c) => c.astd_write_unencrypted_client(w).await,
            Self::CMSG_NPC_TEXT_QUERY(c) => c.astd_write_unencrypted_client(w).await,
            Self::CMSG_QUESTGIVER_STATUS_QUERY(c) => c.astd_write_unencrypted_client(w).await,
            Self::CMSG_QUESTGIVER_HELLO(c) => c.astd_write_unencrypted_client(w).await,
            Self::CMSG_QUESTGIVER_QUERY_QUEST(c) => c.astd_write_unencrypted_client(w).await,
            Self::CMSG_QUESTGIVER_QUEST_AUTOLAUNCH(c) => c.astd_write_unencrypted_client(w).await,
            Self::CMSG_QUESTGIVER_ACCEPT_QUEST(c) => c.astd_write_unencrypted_client(w).await,
            Self::CMSG_QUESTGIVER_COMPLETE_QUEST(c) => c.astd_write_unencrypted_client(w).await,
            Self::CMSG_QUESTGIVER_REQUEST_REWARD(c) => c.astd_write_unencrypted_client(w).await,
            Self::CMSG_QUESTGIVER_CHOOSE_REWARD(c) => c.astd_write_unencrypted_client(w).await,
            Self::CMSG_QUESTGIVER_CANCEL(c) => c.astd_write_unencrypted_client(w).await,
            Self::CMSG_QUESTLOG_SWAP_QUEST(c) => c.astd_write_unencrypted_client(w).await,
            Self::CMSG_QUESTLOG_REMOVE_QUEST(c) => c.astd_write_unencrypted_client(w).await,
            Self::CMSG_QUEST_CONFIRM_ACCEPT(c) => c.astd_write_unencrypted_client(w).await,
            Self::CMSG_PUSHQUESTTOPARTY(c) => c.astd_write_unencrypted_client(w).await,
            Self::CMSG_LIST_INVENTORY(c) => c.astd_write_unencrypted_client(w).await,
            Self::CMSG_SELL_ITEM(c) => c.astd_write_unencrypted_client(w).await,
            Self::CMSG_BUY_ITEM(c) => c.astd_write_unencrypted_client(w).await,
            Self::CMSG_BUY_ITEM_IN_SLOT(c) => c.astd_write_unencrypted_client(w).await,
            Self::CMSG_TAXINODE_STATUS_QUERY(c) => c.astd_write_unencrypted_client(w).await,
            Self::CMSG_TAXIQUERYAVAILABLENODES(c) => c.astd_write_unencrypted_client(w).await,
            Self::CMSG_ACTIVATETAXI(c) => c.astd_write_unencrypted_client(w).await,
            Self::CMSG_TRAINER_LIST(c) => c.astd_write_unencrypted_client(w).await,
            Self::CMSG_TRAINER_BUY_SPELL(c) => c.astd_write_unencrypted_client(w).await,
            Self::CMSG_BINDER_ACTIVATE(c) => c.astd_write_unencrypted_client(w).await,
            Self::CMSG_BANKER_ACTIVATE(c) => c.astd_write_unencrypted_client(w).await,
            Self::CMSG_BUY_BANK_SLOT(c) => c.astd_write_unencrypted_client(w).await,
            Self::CMSG_PETITION_SHOWLIST(c) => c.astd_write_unencrypted_client(w).await,
            Self::CMSG_PETITION_BUY(c) => c.astd_write_unencrypted_client(w).await,
            Self::CMSG_PETITION_SHOW_SIGNATURES(c) => c.astd_write_unencrypted_client(w).await,
            Self::CMSG_PETITION_SIGN(c) => c.astd_write_unencrypted_client(w).await,
            Self::CMSG_OFFER_PETITION(c) => c.astd_write_unencrypted_client(w).await,
            Self::CMSG_TURN_IN_PETITION(c) => c.astd_write_unencrypted_client(w).await,
            Self::CMSG_PETITION_QUERY(c) => c.astd_write_unencrypted_client(w).await,
            Self::CMSG_BUG(c) => c.astd_write_unencrypted_client(w).await,
            Self::CMSG_PLAYED_TIME(c) => c.astd_write_unencrypted_client(w).await,
            Self::CMSG_QUERY_TIME(c) => c.astd_write_unencrypted_client(w).await,
            Self::CMSG_RECLAIM_CORPSE(c) => c.astd_write_unencrypted_client(w).await,
            Self::CMSG_WRAP_ITEM(c) => c.astd_write_unencrypted_client(w).await,
            Self::MSG_MINIMAP_PING(c) => c.astd_write_unencrypted_client(w).await,
            Self::CMSG_PING(c) => c.astd_write_unencrypted_client(w).await,
            Self::CMSG_SETSHEATHED(c) => c.astd_write_unencrypted_client(w).await,
            Self::CMSG_AUTH_SESSION(c) => c.astd_write_unencrypted_client(w).await,
            Self::CMSG_PET_CAST_SPELL(c) => c.astd_write_unencrypted_client(w).await,
            Self::MSG_SAVE_GUILD_EMBLEM(c) => c.astd_write_unencrypted_client(w).await,
            Self::CMSG_ZONEUPDATE(c) => c.astd_write_unencrypted_client(w).await,
            Self::MSG_RANDOM_ROLL(c) => c.astd_write_unencrypted_client(w).await,
            Self::CMSG_UNLEARN_SKILL(c) => c.astd_write_unencrypted_client(w).await,
            Self::CMSG_GMTICKET_CREATE(c) => c.astd_write_unencrypted_client(w).await,
            Self::CMSG_GMTICKET_UPDATETEXT(c) => c.astd_write_unencrypted_client(w).await,
            Self::CMSG_REQUEST_ACCOUNT_DATA(c) => c.astd_write_unencrypted_client(w).await,
            Self::CMSG_UPDATE_ACCOUNT_DATA(c) => c.astd_write_unencrypted_client(w).await,
            Self::CMSG_GMTICKET_GETTICKET(c) => c.astd_write_unencrypted_client(w).await,
            Self::MSG_CORPSE_QUERY(c) => c.astd_write_unencrypted_client(w).await,
            Self::CMSG_GMTICKET_DELETETICKET(c) => c.astd_write_unencrypted_client(w).await,
            Self::CMSG_GMTICKET_SYSTEMSTATUS(c) => c.astd_write_unencrypted_client(w).await,
            Self::CMSG_SPIRIT_HEALER_ACTIVATE(c) => c.astd_write_unencrypted_client(w).await,
            Self::CMSG_CHAT_IGNORED(c) => c.astd_write_unencrypted_client(w).await,
            Self::CMSG_GUILD_RANK(c) => c.astd_write_unencrypted_client(w).await,
            Self::CMSG_GUILD_ADD_RANK(c) => c.astd_write_unencrypted_client(w).await,
            Self::CMSG_GUILD_DEL_RANK(c) => c.astd_write_unencrypted_client(w).await,
            Self::CMSG_GUILD_SET_PUBLIC_NOTE(c) => c.astd_write_unencrypted_client(w).await,
            Self::CMSG_GUILD_SET_OFFICER_NOTE(c) => c.astd_write_unencrypted_client(w).await,
            Self::CMSG_SEND_MAIL(c) => c.astd_write_unencrypted_client(w).await,
            Self::CMSG_GET_MAIL_LIST(c) => c.astd_write_unencrypted_client(w).await,
            Self::CMSG_BATTLEFIELD_LIST(c) => c.astd_write_unencrypted_client(w).await,
            Self::CMSG_ITEM_TEXT_QUERY(c) => c.astd_write_unencrypted_client(w).await,
            Self::CMSG_MAIL_TAKE_MONEY(c) => c.astd_write_unencrypted_client(w).await,
            Self::CMSG_MAIL_TAKE_ITEM(c) => c.astd_write_unencrypted_client(w).await,
            Self::CMSG_MAIL_MARK_AS_READ(c) => c.astd_write_unencrypted_client(w).await,
            Self::CMSG_MAIL_RETURN_TO_SENDER(c) => c.astd_write_unencrypted_client(w).await,
            Self::CMSG_MAIL_DELETE(c) => c.astd_write_unencrypted_client(w).await,
            Self::CMSG_MAIL_CREATE_TEXT_ITEM(c) => c.astd_write_unencrypted_client(w).await,
            Self::CMSG_LEARN_TALENT(c) => c.astd_write_unencrypted_client(w).await,
            Self::CMSG_TOGGLE_PVP(c) => c.astd_write_unencrypted_client(w).await,
            Self::MSG_AUCTION_HELLO(c) => c.astd_write_unencrypted_client(w).await,
            Self::CMSG_AUCTION_SELL_ITEM(c) => c.astd_write_unencrypted_client(w).await,
            Self::CMSG_AUCTION_REMOVE_ITEM(c) => c.astd_write_unencrypted_client(w).await,
            Self::CMSG_AUCTION_LIST_ITEMS(c) => c.astd_write_unencrypted_client(w).await,
            Self::CMSG_AUCTION_LIST_OWNER_ITEMS(c) => c.astd_write_unencrypted_client(w).await,
            Self::CMSG_AUCTION_PLACE_BID(c) => c.astd_write_unencrypted_client(w).await,
            Self::CMSG_AUCTION_LIST_BIDDER_ITEMS(c) => c.astd_write_unencrypted_client(w).await,
            Self::CMSG_SET_AMMO(c) => c.astd_write_unencrypted_client(w).await,
            Self::CMSG_SET_ACTIVE_MOVER(c) => c.astd_write_unencrypted_client(w).await,
            Self::CMSG_PET_CANCEL_AURA(c) => c.astd_write_unencrypted_client(w).await,
            Self::CMSG_CANCEL_AUTO_REPEAT_SPELL(c) => c.astd_write_unencrypted_client(w).await,
            Self::MSG_LIST_STABLED_PETS(c) => c.astd_write_unencrypted_client(w).await,
            Self::CMSG_STABLE_PET(c) => c.astd_write_unencrypted_client(w).await,
            Self::CMSG_UNSTABLE_PET(c) => c.astd_write_unencrypted_client(w).await,
            Self::CMSG_BUY_STABLE_SLOT(c) => c.astd_write_unencrypted_client(w).await,
            Self::CMSG_STABLE_SWAP_PET(c) => c.astd_write_unencrypted_client(w).await,
            Self::CMSG_REQUEST_PET_INFO(c) => c.astd_write_unencrypted_client(w).await,
            Self::CMSG_FAR_SIGHT(c) => c.astd_write_unencrypted_client(w).await,
            Self::CMSG_GROUP_CHANGE_SUB_GROUP(c) => c.astd_write_unencrypted_client(w).await,
            Self::CMSG_REQUEST_PARTY_MEMBER_STATS(c) => c.astd_write_unencrypted_client(w).await,
            Self::CMSG_GROUP_SWAP_SUB_GROUP(c) => c.astd_write_unencrypted_client(w).await,
            Self::CMSG_AUTOSTORE_BANK_ITEM(c) => c.astd_write_unencrypted_client(w).await,
            Self::CMSG_AUTOBANK_ITEM(c) => c.astd_write_unencrypted_client(w).await,
            Self::MSG_QUERY_NEXT_MAIL_TIME(c) => c.astd_write_unencrypted_client(w).await,
            Self::CMSG_GROUP_RAID_CONVERT(c) => c.astd_write_unencrypted_client(w).await,
            Self::CMSG_GROUP_ASSISTANT_LEADER(c) => c.astd_write_unencrypted_client(w).await,
            Self::CMSG_BUYBACK_ITEM(c) => c.astd_write_unencrypted_client(w).await,
            Self::CMSG_LFG_GET_STATUS(c) => c.astd_write_unencrypted_client(w).await,
            Self::CMSG_CANCEL_GROWTH_AURA(c) => c.astd_write_unencrypted_client(w).await,
            Self::CMSG_LOOT_ROLL(c) => c.astd_write_unencrypted_client(w).await,
            Self::CMSG_LOOT_MASTER_GIVE(c) => c.astd_write_unencrypted_client(w).await,
            Self::CMSG_REPAIR_ITEM(c) => c.astd_write_unencrypted_client(w).await,
            Self::CMSG_SUMMON_RESPONSE(c) => c.astd_write_unencrypted_client(w).await,
            Self::CMSG_SELF_RES(c) => c.astd_write_unencrypted_client(w).await,
            Self::CMSG_TOGGLE_HELM(c) => c.astd_write_unencrypted_client(w).await,
            Self::CMSG_TOGGLE_CLOAK(c) => c.astd_write_unencrypted_client(w).await,
            Self::CMSG_SET_ACTIONBAR_TOGGLES(c) => c.astd_write_unencrypted_client(w).await,
            Self::CMSG_ITEM_NAME_QUERY(c) => c.astd_write_unencrypted_client(w).await,
            Self::CMSG_CHAR_RENAME(c) => c.astd_write_unencrypted_client(w).await,
            Self::CMSG_MOVE_SPLINE_DONE(c) => c.astd_write_unencrypted_client(w).await,
            Self::CMSG_MOVE_FALL_RESET(c) => c.astd_write_unencrypted_client(w).await,
            Self::CMSG_REQUEST_RAID_INFO(c) => c.astd_write_unencrypted_client(w).await,
            Self::CMSG_MOVE_TIME_SKIPPED(c) => c.astd_write_unencrypted_client(w).await,
            Self::CMSG_MOVE_FEATHER_FALL_ACK(c) => c.astd_write_unencrypted_client(w).await,
            Self::CMSG_MOVE_WATER_WALK_ACK(c) => c.astd_write_unencrypted_client(w).await,
            Self::CMSG_MOVE_NOT_ACTIVE_MOVER(c) => c.astd_write_unencrypted_client(w).await,
            Self::CMSG_BATTLEFIELD_STATUS(c) => c.astd_write_unencrypted_client(w).await,
            Self::CMSG_BATTLEFIELD_PORT(c) => c.astd_write_unencrypted_client(w).await,
            Self::MSG_INSPECT_HONOR_STATS(c) => c.astd_write_unencrypted_client(w).await,
            Self::CMSG_BATTLEMASTER_HELLO(c) => c.astd_write_unencrypted_client(w).await,
            Self::CMSG_FORCE_WALK_SPEED_CHANGE_ACK(c) => c.astd_write_unencrypted_client(w).await,
            Self::CMSG_FORCE_SWIM_BACK_SPEED_CHANGE_ACK(c) => c.astd_write_unencrypted_client(w).await,
            Self::CMSG_FORCE_TURN_RATE_CHANGE_ACK(c) => c.astd_write_unencrypted_client(w).await,
            Self::MSG_PVP_LOG_DATA(c) => c.astd_write_unencrypted_client(w).await,
            Self::CMSG_LEAVE_BATTLEFIELD(c) => c.astd_write_unencrypted_client(w).await,
            Self::CMSG_AREA_SPIRIT_HEALER_QUERY(c) => c.astd_write_unencrypted_client(w).await,
            Self::CMSG_AREA_SPIRIT_HEALER_QUEUE(c) => c.astd_write_unencrypted_client(w).await,
            Self::MSG_BATTLEGROUND_PLAYER_POSITIONS(c) => c.astd_write_unencrypted_client(w).await,
            Self::CMSG_PET_STOP_ATTACK(c) => c.astd_write_unencrypted_client(w).await,
            Self::CMSG_BATTLEMASTER_JOIN(c) => c.astd_write_unencrypted_client(w).await,
            Self::CMSG_PET_UNLEARN(c) => c.astd_write_unencrypted_client(w).await,
            Self::CMSG_PET_SPELL_AUTOCAST(c) => c.astd_write_unencrypted_client(w).await,
            Self::CMSG_GUILD_INFO_TEXT(c) => c.astd_write_unencrypted_client(w).await,
            Self::CMSG_ACTIVATETAXIEXPRESS(c) => c.astd_write_unencrypted_client(w).await,
            Self::CMSG_SET_FACTION_INACTIVE(c) => c.astd_write_unencrypted_client(w).await,
            Self::CMSG_SET_WATCHED_FACTION(c) => c.astd_write_unencrypted_client(w).await,
            Self::CMSG_RESET_INSTANCES(c) => c.astd_write_unencrypted_client(w).await,
            Self::MSG_RAID_READY_CHECK(c) => c.astd_write_unencrypted_client(w).await,
            Self::CMSG_GMSURVEY_SUBMIT(c) => c.astd_write_unencrypted_client(w).await,
            Self::CMSG_MOVE_SET_FLY(c) => c.astd_write_unencrypted_client(w).await,
            Self::CMSG_LFD_PLAYER_LOCK_INFO_REQUEST(c) => c.astd_write_unencrypted_client(w).await,
            Self::CMSG_REALM_SPLIT(c) => c.astd_write_unencrypted_client(w).await,
            Self::CMSG_MOVE_CHNG_TRANSPORT(c) => c.astd_write_unencrypted_client(w).await,
            Self::CMSG_TIME_SYNC_RESP(c) => c.astd_write_unencrypted_client(w).await,
            Self::CMSG_VOICE_SESSION_ENABLE(c) => c.astd_write_unencrypted_client(w).await,
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
            ClientOpcodeMessage::MSG_QUEST_PUSH_RESULT(_) => "MSG_QUEST_PUSH_RESULT",
            ClientOpcodeMessage::MSG_PETITION_RENAME(_) => "MSG_PETITION_RENAME",
            ClientOpcodeMessage::MSG_SET_DUNGEON_DIFFICULTY(_) => "MSG_SET_DUNGEON_DIFFICULTY",
            ClientOpcodeMessage::MSG_MOVE_START_ASCEND(_) => "MSG_MOVE_START_ASCEND",
            ClientOpcodeMessage::MSG_MOVE_STOP_ASCEND(_) => "MSG_MOVE_STOP_ASCEND",
            ClientOpcodeMessage::MSG_MOVE_START_DESCEND(_) => "MSG_MOVE_START_DESCEND",
            ClientOpcodeMessage::MSG_GUILD_BANK_MONEY_WITHDRAWN(_) => "MSG_GUILD_BANK_MONEY_WITHDRAWN",
            ClientOpcodeMessage::CMSG_CALENDAR_GET_NUM_PENDING(_) => "CMSG_CALENDAR_GET_NUM_PENDING",
            ClientOpcodeMessage::CMSG_BOOTME(_) => "CMSG_BOOTME",
            ClientOpcodeMessage::CMSG_WORLD_TELEPORT(_) => "CMSG_WORLD_TELEPORT",
            ClientOpcodeMessage::CMSG_TELEPORT_TO_UNIT(_) => "CMSG_TELEPORT_TO_UNIT",
            ClientOpcodeMessage::CMSG_CHAR_CREATE(_) => "CMSG_CHAR_CREATE",
            ClientOpcodeMessage::CMSG_CHAR_ENUM(_) => "CMSG_CHAR_ENUM",
            ClientOpcodeMessage::CMSG_CHAR_DELETE(_) => "CMSG_CHAR_DELETE",
            ClientOpcodeMessage::CMSG_PLAYER_LOGIN(_) => "CMSG_PLAYER_LOGIN",
            ClientOpcodeMessage::CMSG_PLAYER_LOGOUT(_) => "CMSG_PLAYER_LOGOUT",
            ClientOpcodeMessage::CMSG_LOGOUT_REQUEST(_) => "CMSG_LOGOUT_REQUEST",
            ClientOpcodeMessage::CMSG_LOGOUT_CANCEL(_) => "CMSG_LOGOUT_CANCEL",
            ClientOpcodeMessage::CMSG_NAME_QUERY(_) => "CMSG_NAME_QUERY",
            ClientOpcodeMessage::CMSG_PET_NAME_QUERY(_) => "CMSG_PET_NAME_QUERY",
            ClientOpcodeMessage::CMSG_GUILD_QUERY(_) => "CMSG_GUILD_QUERY",
            ClientOpcodeMessage::CMSG_ITEM_QUERY_SINGLE(_) => "CMSG_ITEM_QUERY_SINGLE",
            ClientOpcodeMessage::CMSG_PAGE_TEXT_QUERY(_) => "CMSG_PAGE_TEXT_QUERY",
            ClientOpcodeMessage::CMSG_QUEST_QUERY(_) => "CMSG_QUEST_QUERY",
            ClientOpcodeMessage::CMSG_GAMEOBJECT_QUERY(_) => "CMSG_GAMEOBJECT_QUERY",
            ClientOpcodeMessage::CMSG_CREATURE_QUERY(_) => "CMSG_CREATURE_QUERY",
            ClientOpcodeMessage::CMSG_WHO(_) => "CMSG_WHO",
            ClientOpcodeMessage::CMSG_WHOIS(_) => "CMSG_WHOIS",
            ClientOpcodeMessage::CMSG_CONTACT_LIST(_) => "CMSG_CONTACT_LIST",
            ClientOpcodeMessage::CMSG_ADD_FRIEND(_) => "CMSG_ADD_FRIEND",
            ClientOpcodeMessage::CMSG_DEL_FRIEND(_) => "CMSG_DEL_FRIEND",
            ClientOpcodeMessage::CMSG_ADD_IGNORE(_) => "CMSG_ADD_IGNORE",
            ClientOpcodeMessage::CMSG_DEL_IGNORE(_) => "CMSG_DEL_IGNORE",
            ClientOpcodeMessage::CMSG_GROUP_INVITE(_) => "CMSG_GROUP_INVITE",
            ClientOpcodeMessage::CMSG_GROUP_ACCEPT(_) => "CMSG_GROUP_ACCEPT",
            ClientOpcodeMessage::CMSG_GROUP_DECLINE(_) => "CMSG_GROUP_DECLINE",
            ClientOpcodeMessage::CMSG_GROUP_UNINVITE(_) => "CMSG_GROUP_UNINVITE",
            ClientOpcodeMessage::CMSG_GROUP_UNINVITE_GUID(_) => "CMSG_GROUP_UNINVITE_GUID",
            ClientOpcodeMessage::CMSG_GROUP_SET_LEADER(_) => "CMSG_GROUP_SET_LEADER",
            ClientOpcodeMessage::CMSG_LOOT_METHOD(_) => "CMSG_LOOT_METHOD",
            ClientOpcodeMessage::CMSG_GROUP_DISBAND(_) => "CMSG_GROUP_DISBAND",
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
            ClientOpcodeMessage::CMSG_USE_ITEM(_) => "CMSG_USE_ITEM",
            ClientOpcodeMessage::CMSG_OPEN_ITEM(_) => "CMSG_OPEN_ITEM",
            ClientOpcodeMessage::CMSG_READ_ITEM(_) => "CMSG_READ_ITEM",
            ClientOpcodeMessage::CMSG_GAMEOBJ_USE(_) => "CMSG_GAMEOBJ_USE",
            ClientOpcodeMessage::CMSG_AREATRIGGER(_) => "CMSG_AREATRIGGER",
            ClientOpcodeMessage::MSG_MOVE_TELEPORT_ACK(_) => "MSG_MOVE_TELEPORT_ACK_Client",
            ClientOpcodeMessage::CMSG_MOVE_SET_RAW_POSITION(_) => "CMSG_MOVE_SET_RAW_POSITION",
            ClientOpcodeMessage::CMSG_FORCE_RUN_SPEED_CHANGE_ACK(_) => "CMSG_FORCE_RUN_SPEED_CHANGE_ACK",
            ClientOpcodeMessage::CMSG_FORCE_RUN_BACK_SPEED_CHANGE_ACK(_) => "CMSG_FORCE_RUN_BACK_SPEED_CHANGE_ACK",
            ClientOpcodeMessage::CMSG_FORCE_SWIM_SPEED_CHANGE_ACK(_) => "CMSG_FORCE_SWIM_SPEED_CHANGE_ACK",
            ClientOpcodeMessage::CMSG_FORCE_MOVE_ROOT_ACK(_) => "CMSG_FORCE_MOVE_ROOT_ACK",
            ClientOpcodeMessage::CMSG_FORCE_MOVE_UNROOT_ACK(_) => "CMSG_FORCE_MOVE_UNROOT_ACK",
            ClientOpcodeMessage::CMSG_MOVE_KNOCK_BACK_ACK(_) => "CMSG_MOVE_KNOCK_BACK_ACK",
            ClientOpcodeMessage::CMSG_MOVE_HOVER_ACK(_) => "CMSG_MOVE_HOVER_ACK",
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
            ClientOpcodeMessage::CMSG_INITIATE_TRADE(_) => "CMSG_INITIATE_TRADE",
            ClientOpcodeMessage::CMSG_BEGIN_TRADE(_) => "CMSG_BEGIN_TRADE",
            ClientOpcodeMessage::CMSG_BUSY_TRADE(_) => "CMSG_BUSY_TRADE",
            ClientOpcodeMessage::CMSG_IGNORE_TRADE(_) => "CMSG_IGNORE_TRADE",
            ClientOpcodeMessage::CMSG_ACCEPT_TRADE(_) => "CMSG_ACCEPT_TRADE",
            ClientOpcodeMessage::CMSG_UNACCEPT_TRADE(_) => "CMSG_UNACCEPT_TRADE",
            ClientOpcodeMessage::CMSG_CANCEL_TRADE(_) => "CMSG_CANCEL_TRADE",
            ClientOpcodeMessage::CMSG_SET_TRADE_ITEM(_) => "CMSG_SET_TRADE_ITEM",
            ClientOpcodeMessage::CMSG_CLEAR_TRADE_ITEM(_) => "CMSG_CLEAR_TRADE_ITEM",
            ClientOpcodeMessage::CMSG_SET_TRADE_GOLD(_) => "CMSG_SET_TRADE_GOLD",
            ClientOpcodeMessage::CMSG_SET_FACTION_ATWAR(_) => "CMSG_SET_FACTION_ATWAR",
            ClientOpcodeMessage::CMSG_SET_ACTION_BUTTON(_) => "CMSG_SET_ACTION_BUTTON",
            ClientOpcodeMessage::CMSG_CAST_SPELL(_) => "CMSG_CAST_SPELL",
            ClientOpcodeMessage::CMSG_CANCEL_CAST(_) => "CMSG_CANCEL_CAST",
            ClientOpcodeMessage::CMSG_CANCEL_AURA(_) => "CMSG_CANCEL_AURA",
            ClientOpcodeMessage::CMSG_CANCEL_CHANNELLING(_) => "CMSG_CANCEL_CHANNELLING",
            ClientOpcodeMessage::CMSG_SET_SELECTION(_) => "CMSG_SET_SELECTION",
            ClientOpcodeMessage::CMSG_DELETEEQUIPMENT_SET(_) => "CMSG_DELETEEQUIPMENT_SET",
            ClientOpcodeMessage::CMSG_ATTACKSWING(_) => "CMSG_ATTACKSWING",
            ClientOpcodeMessage::CMSG_ATTACKSTOP(_) => "CMSG_ATTACKSTOP",
            ClientOpcodeMessage::CMSG_REPOP_REQUEST(_) => "CMSG_REPOP_REQUEST",
            ClientOpcodeMessage::CMSG_RESURRECT_RESPONSE(_) => "CMSG_RESURRECT_RESPONSE",
            ClientOpcodeMessage::CMSG_LOOT(_) => "CMSG_LOOT",
            ClientOpcodeMessage::CMSG_LOOT_MONEY(_) => "CMSG_LOOT_MONEY",
            ClientOpcodeMessage::CMSG_LOOT_RELEASE(_) => "CMSG_LOOT_RELEASE",
            ClientOpcodeMessage::CMSG_DUEL_ACCEPTED(_) => "CMSG_DUEL_ACCEPTED",
            ClientOpcodeMessage::CMSG_DUEL_CANCELLED(_) => "CMSG_DUEL_CANCELLED",
            ClientOpcodeMessage::CMSG_MOUNTSPECIAL_ANIM(_) => "CMSG_MOUNTSPECIAL_ANIM",
            ClientOpcodeMessage::CMSG_PET_SET_ACTION(_) => "CMSG_PET_SET_ACTION",
            ClientOpcodeMessage::CMSG_PET_ACTION(_) => "CMSG_PET_ACTION",
            ClientOpcodeMessage::CMSG_PET_ABANDON(_) => "CMSG_PET_ABANDON",
            ClientOpcodeMessage::CMSG_PET_RENAME(_) => "CMSG_PET_RENAME",
            ClientOpcodeMessage::CMSG_GOSSIP_HELLO(_) => "CMSG_GOSSIP_HELLO",
            ClientOpcodeMessage::CMSG_GOSSIP_SELECT_OPTION(_) => "CMSG_GOSSIP_SELECT_OPTION",
            ClientOpcodeMessage::CMSG_NPC_TEXT_QUERY(_) => "CMSG_NPC_TEXT_QUERY",
            ClientOpcodeMessage::CMSG_QUESTGIVER_STATUS_QUERY(_) => "CMSG_QUESTGIVER_STATUS_QUERY",
            ClientOpcodeMessage::CMSG_QUESTGIVER_HELLO(_) => "CMSG_QUESTGIVER_HELLO",
            ClientOpcodeMessage::CMSG_QUESTGIVER_QUERY_QUEST(_) => "CMSG_QUESTGIVER_QUERY_QUEST",
            ClientOpcodeMessage::CMSG_QUESTGIVER_QUEST_AUTOLAUNCH(_) => "CMSG_QUESTGIVER_QUEST_AUTOLAUNCH",
            ClientOpcodeMessage::CMSG_QUESTGIVER_ACCEPT_QUEST(_) => "CMSG_QUESTGIVER_ACCEPT_QUEST",
            ClientOpcodeMessage::CMSG_QUESTGIVER_COMPLETE_QUEST(_) => "CMSG_QUESTGIVER_COMPLETE_QUEST",
            ClientOpcodeMessage::CMSG_QUESTGIVER_REQUEST_REWARD(_) => "CMSG_QUESTGIVER_REQUEST_REWARD",
            ClientOpcodeMessage::CMSG_QUESTGIVER_CHOOSE_REWARD(_) => "CMSG_QUESTGIVER_CHOOSE_REWARD",
            ClientOpcodeMessage::CMSG_QUESTGIVER_CANCEL(_) => "CMSG_QUESTGIVER_CANCEL",
            ClientOpcodeMessage::CMSG_QUESTLOG_SWAP_QUEST(_) => "CMSG_QUESTLOG_SWAP_QUEST",
            ClientOpcodeMessage::CMSG_QUESTLOG_REMOVE_QUEST(_) => "CMSG_QUESTLOG_REMOVE_QUEST",
            ClientOpcodeMessage::CMSG_QUEST_CONFIRM_ACCEPT(_) => "CMSG_QUEST_CONFIRM_ACCEPT",
            ClientOpcodeMessage::CMSG_PUSHQUESTTOPARTY(_) => "CMSG_PUSHQUESTTOPARTY",
            ClientOpcodeMessage::CMSG_LIST_INVENTORY(_) => "CMSG_LIST_INVENTORY",
            ClientOpcodeMessage::CMSG_SELL_ITEM(_) => "CMSG_SELL_ITEM",
            ClientOpcodeMessage::CMSG_BUY_ITEM(_) => "CMSG_BUY_ITEM",
            ClientOpcodeMessage::CMSG_BUY_ITEM_IN_SLOT(_) => "CMSG_BUY_ITEM_IN_SLOT",
            ClientOpcodeMessage::CMSG_TAXINODE_STATUS_QUERY(_) => "CMSG_TAXINODE_STATUS_QUERY",
            ClientOpcodeMessage::CMSG_TAXIQUERYAVAILABLENODES(_) => "CMSG_TAXIQUERYAVAILABLENODES",
            ClientOpcodeMessage::CMSG_ACTIVATETAXI(_) => "CMSG_ACTIVATETAXI",
            ClientOpcodeMessage::CMSG_TRAINER_LIST(_) => "CMSG_TRAINER_LIST",
            ClientOpcodeMessage::CMSG_TRAINER_BUY_SPELL(_) => "CMSG_TRAINER_BUY_SPELL",
            ClientOpcodeMessage::CMSG_BINDER_ACTIVATE(_) => "CMSG_BINDER_ACTIVATE",
            ClientOpcodeMessage::CMSG_BANKER_ACTIVATE(_) => "CMSG_BANKER_ACTIVATE",
            ClientOpcodeMessage::CMSG_BUY_BANK_SLOT(_) => "CMSG_BUY_BANK_SLOT",
            ClientOpcodeMessage::CMSG_PETITION_SHOWLIST(_) => "CMSG_PETITION_SHOWLIST",
            ClientOpcodeMessage::CMSG_PETITION_BUY(_) => "CMSG_PETITION_BUY",
            ClientOpcodeMessage::CMSG_PETITION_SHOW_SIGNATURES(_) => "CMSG_PETITION_SHOW_SIGNATURES",
            ClientOpcodeMessage::CMSG_PETITION_SIGN(_) => "CMSG_PETITION_SIGN",
            ClientOpcodeMessage::CMSG_OFFER_PETITION(_) => "CMSG_OFFER_PETITION",
            ClientOpcodeMessage::CMSG_TURN_IN_PETITION(_) => "CMSG_TURN_IN_PETITION",
            ClientOpcodeMessage::CMSG_PETITION_QUERY(_) => "CMSG_PETITION_QUERY",
            ClientOpcodeMessage::CMSG_BUG(_) => "CMSG_BUG",
            ClientOpcodeMessage::CMSG_PLAYED_TIME(_) => "CMSG_PLAYED_TIME",
            ClientOpcodeMessage::CMSG_QUERY_TIME(_) => "CMSG_QUERY_TIME",
            ClientOpcodeMessage::CMSG_RECLAIM_CORPSE(_) => "CMSG_RECLAIM_CORPSE",
            ClientOpcodeMessage::CMSG_WRAP_ITEM(_) => "CMSG_WRAP_ITEM",
            ClientOpcodeMessage::MSG_MINIMAP_PING(_) => "MSG_MINIMAP_PING_Client",
            ClientOpcodeMessage::CMSG_PING(_) => "CMSG_PING",
            ClientOpcodeMessage::CMSG_SETSHEATHED(_) => "CMSG_SETSHEATHED",
            ClientOpcodeMessage::CMSG_AUTH_SESSION(_) => "CMSG_AUTH_SESSION",
            ClientOpcodeMessage::CMSG_PET_CAST_SPELL(_) => "CMSG_PET_CAST_SPELL",
            ClientOpcodeMessage::MSG_SAVE_GUILD_EMBLEM(_) => "MSG_SAVE_GUILD_EMBLEM_Client",
            ClientOpcodeMessage::CMSG_ZONEUPDATE(_) => "CMSG_ZONEUPDATE",
            ClientOpcodeMessage::MSG_RANDOM_ROLL(_) => "MSG_RANDOM_ROLL_Client",
            ClientOpcodeMessage::CMSG_UNLEARN_SKILL(_) => "CMSG_UNLEARN_SKILL",
            ClientOpcodeMessage::CMSG_GMTICKET_CREATE(_) => "CMSG_GMTICKET_CREATE",
            ClientOpcodeMessage::CMSG_GMTICKET_UPDATETEXT(_) => "CMSG_GMTICKET_UPDATETEXT",
            ClientOpcodeMessage::CMSG_REQUEST_ACCOUNT_DATA(_) => "CMSG_REQUEST_ACCOUNT_DATA",
            ClientOpcodeMessage::CMSG_UPDATE_ACCOUNT_DATA(_) => "CMSG_UPDATE_ACCOUNT_DATA",
            ClientOpcodeMessage::CMSG_GMTICKET_GETTICKET(_) => "CMSG_GMTICKET_GETTICKET",
            ClientOpcodeMessage::MSG_CORPSE_QUERY(_) => "MSG_CORPSE_QUERY_Client",
            ClientOpcodeMessage::CMSG_GMTICKET_DELETETICKET(_) => "CMSG_GMTICKET_DELETETICKET",
            ClientOpcodeMessage::CMSG_GMTICKET_SYSTEMSTATUS(_) => "CMSG_GMTICKET_SYSTEMSTATUS",
            ClientOpcodeMessage::CMSG_SPIRIT_HEALER_ACTIVATE(_) => "CMSG_SPIRIT_HEALER_ACTIVATE",
            ClientOpcodeMessage::CMSG_CHAT_IGNORED(_) => "CMSG_CHAT_IGNORED",
            ClientOpcodeMessage::CMSG_GUILD_RANK(_) => "CMSG_GUILD_RANK",
            ClientOpcodeMessage::CMSG_GUILD_ADD_RANK(_) => "CMSG_GUILD_ADD_RANK",
            ClientOpcodeMessage::CMSG_GUILD_DEL_RANK(_) => "CMSG_GUILD_DEL_RANK",
            ClientOpcodeMessage::CMSG_GUILD_SET_PUBLIC_NOTE(_) => "CMSG_GUILD_SET_PUBLIC_NOTE",
            ClientOpcodeMessage::CMSG_GUILD_SET_OFFICER_NOTE(_) => "CMSG_GUILD_SET_OFFICER_NOTE",
            ClientOpcodeMessage::CMSG_SEND_MAIL(_) => "CMSG_SEND_MAIL",
            ClientOpcodeMessage::CMSG_GET_MAIL_LIST(_) => "CMSG_GET_MAIL_LIST",
            ClientOpcodeMessage::CMSG_BATTLEFIELD_LIST(_) => "CMSG_BATTLEFIELD_LIST",
            ClientOpcodeMessage::CMSG_ITEM_TEXT_QUERY(_) => "CMSG_ITEM_TEXT_QUERY",
            ClientOpcodeMessage::CMSG_MAIL_TAKE_MONEY(_) => "CMSG_MAIL_TAKE_MONEY",
            ClientOpcodeMessage::CMSG_MAIL_TAKE_ITEM(_) => "CMSG_MAIL_TAKE_ITEM",
            ClientOpcodeMessage::CMSG_MAIL_MARK_AS_READ(_) => "CMSG_MAIL_MARK_AS_READ",
            ClientOpcodeMessage::CMSG_MAIL_RETURN_TO_SENDER(_) => "CMSG_MAIL_RETURN_TO_SENDER",
            ClientOpcodeMessage::CMSG_MAIL_DELETE(_) => "CMSG_MAIL_DELETE",
            ClientOpcodeMessage::CMSG_MAIL_CREATE_TEXT_ITEM(_) => "CMSG_MAIL_CREATE_TEXT_ITEM",
            ClientOpcodeMessage::CMSG_LEARN_TALENT(_) => "CMSG_LEARN_TALENT",
            ClientOpcodeMessage::CMSG_TOGGLE_PVP(_) => "CMSG_TOGGLE_PVP",
            ClientOpcodeMessage::MSG_AUCTION_HELLO(_) => "MSG_AUCTION_HELLO_Client",
            ClientOpcodeMessage::CMSG_AUCTION_SELL_ITEM(_) => "CMSG_AUCTION_SELL_ITEM",
            ClientOpcodeMessage::CMSG_AUCTION_REMOVE_ITEM(_) => "CMSG_AUCTION_REMOVE_ITEM",
            ClientOpcodeMessage::CMSG_AUCTION_LIST_ITEMS(_) => "CMSG_AUCTION_LIST_ITEMS",
            ClientOpcodeMessage::CMSG_AUCTION_LIST_OWNER_ITEMS(_) => "CMSG_AUCTION_LIST_OWNER_ITEMS",
            ClientOpcodeMessage::CMSG_AUCTION_PLACE_BID(_) => "CMSG_AUCTION_PLACE_BID",
            ClientOpcodeMessage::CMSG_AUCTION_LIST_BIDDER_ITEMS(_) => "CMSG_AUCTION_LIST_BIDDER_ITEMS",
            ClientOpcodeMessage::CMSG_SET_AMMO(_) => "CMSG_SET_AMMO",
            ClientOpcodeMessage::CMSG_SET_ACTIVE_MOVER(_) => "CMSG_SET_ACTIVE_MOVER",
            ClientOpcodeMessage::CMSG_PET_CANCEL_AURA(_) => "CMSG_PET_CANCEL_AURA",
            ClientOpcodeMessage::CMSG_CANCEL_AUTO_REPEAT_SPELL(_) => "CMSG_CANCEL_AUTO_REPEAT_SPELL",
            ClientOpcodeMessage::MSG_LIST_STABLED_PETS(_) => "MSG_LIST_STABLED_PETS_Client",
            ClientOpcodeMessage::CMSG_STABLE_PET(_) => "CMSG_STABLE_PET",
            ClientOpcodeMessage::CMSG_UNSTABLE_PET(_) => "CMSG_UNSTABLE_PET",
            ClientOpcodeMessage::CMSG_BUY_STABLE_SLOT(_) => "CMSG_BUY_STABLE_SLOT",
            ClientOpcodeMessage::CMSG_STABLE_SWAP_PET(_) => "CMSG_STABLE_SWAP_PET",
            ClientOpcodeMessage::CMSG_REQUEST_PET_INFO(_) => "CMSG_REQUEST_PET_INFO",
            ClientOpcodeMessage::CMSG_FAR_SIGHT(_) => "CMSG_FAR_SIGHT",
            ClientOpcodeMessage::CMSG_GROUP_CHANGE_SUB_GROUP(_) => "CMSG_GROUP_CHANGE_SUB_GROUP",
            ClientOpcodeMessage::CMSG_REQUEST_PARTY_MEMBER_STATS(_) => "CMSG_REQUEST_PARTY_MEMBER_STATS",
            ClientOpcodeMessage::CMSG_GROUP_SWAP_SUB_GROUP(_) => "CMSG_GROUP_SWAP_SUB_GROUP",
            ClientOpcodeMessage::CMSG_AUTOSTORE_BANK_ITEM(_) => "CMSG_AUTOSTORE_BANK_ITEM",
            ClientOpcodeMessage::CMSG_AUTOBANK_ITEM(_) => "CMSG_AUTOBANK_ITEM",
            ClientOpcodeMessage::MSG_QUERY_NEXT_MAIL_TIME(_) => "MSG_QUERY_NEXT_MAIL_TIME_Client",
            ClientOpcodeMessage::CMSG_GROUP_RAID_CONVERT(_) => "CMSG_GROUP_RAID_CONVERT",
            ClientOpcodeMessage::CMSG_GROUP_ASSISTANT_LEADER(_) => "CMSG_GROUP_ASSISTANT_LEADER",
            ClientOpcodeMessage::CMSG_BUYBACK_ITEM(_) => "CMSG_BUYBACK_ITEM",
            ClientOpcodeMessage::CMSG_LFG_GET_STATUS(_) => "CMSG_LFG_GET_STATUS",
            ClientOpcodeMessage::CMSG_CANCEL_GROWTH_AURA(_) => "CMSG_CANCEL_GROWTH_AURA",
            ClientOpcodeMessage::CMSG_LOOT_ROLL(_) => "CMSG_LOOT_ROLL",
            ClientOpcodeMessage::CMSG_LOOT_MASTER_GIVE(_) => "CMSG_LOOT_MASTER_GIVE",
            ClientOpcodeMessage::CMSG_REPAIR_ITEM(_) => "CMSG_REPAIR_ITEM",
            ClientOpcodeMessage::CMSG_SUMMON_RESPONSE(_) => "CMSG_SUMMON_RESPONSE",
            ClientOpcodeMessage::CMSG_SELF_RES(_) => "CMSG_SELF_RES",
            ClientOpcodeMessage::CMSG_TOGGLE_HELM(_) => "CMSG_TOGGLE_HELM",
            ClientOpcodeMessage::CMSG_TOGGLE_CLOAK(_) => "CMSG_TOGGLE_CLOAK",
            ClientOpcodeMessage::CMSG_SET_ACTIONBAR_TOGGLES(_) => "CMSG_SET_ACTIONBAR_TOGGLES",
            ClientOpcodeMessage::CMSG_ITEM_NAME_QUERY(_) => "CMSG_ITEM_NAME_QUERY",
            ClientOpcodeMessage::CMSG_CHAR_RENAME(_) => "CMSG_CHAR_RENAME",
            ClientOpcodeMessage::CMSG_MOVE_SPLINE_DONE(_) => "CMSG_MOVE_SPLINE_DONE",
            ClientOpcodeMessage::CMSG_MOVE_FALL_RESET(_) => "CMSG_MOVE_FALL_RESET",
            ClientOpcodeMessage::CMSG_REQUEST_RAID_INFO(_) => "CMSG_REQUEST_RAID_INFO",
            ClientOpcodeMessage::CMSG_MOVE_TIME_SKIPPED(_) => "CMSG_MOVE_TIME_SKIPPED",
            ClientOpcodeMessage::CMSG_MOVE_FEATHER_FALL_ACK(_) => "CMSG_MOVE_FEATHER_FALL_ACK",
            ClientOpcodeMessage::CMSG_MOVE_WATER_WALK_ACK(_) => "CMSG_MOVE_WATER_WALK_ACK",
            ClientOpcodeMessage::CMSG_MOVE_NOT_ACTIVE_MOVER(_) => "CMSG_MOVE_NOT_ACTIVE_MOVER",
            ClientOpcodeMessage::CMSG_BATTLEFIELD_STATUS(_) => "CMSG_BATTLEFIELD_STATUS",
            ClientOpcodeMessage::CMSG_BATTLEFIELD_PORT(_) => "CMSG_BATTLEFIELD_PORT",
            ClientOpcodeMessage::MSG_INSPECT_HONOR_STATS(_) => "MSG_INSPECT_HONOR_STATS_Client",
            ClientOpcodeMessage::CMSG_BATTLEMASTER_HELLO(_) => "CMSG_BATTLEMASTER_HELLO",
            ClientOpcodeMessage::CMSG_FORCE_WALK_SPEED_CHANGE_ACK(_) => "CMSG_FORCE_WALK_SPEED_CHANGE_ACK",
            ClientOpcodeMessage::CMSG_FORCE_SWIM_BACK_SPEED_CHANGE_ACK(_) => "CMSG_FORCE_SWIM_BACK_SPEED_CHANGE_ACK",
            ClientOpcodeMessage::CMSG_FORCE_TURN_RATE_CHANGE_ACK(_) => "CMSG_FORCE_TURN_RATE_CHANGE_ACK",
            ClientOpcodeMessage::MSG_PVP_LOG_DATA(_) => "MSG_PVP_LOG_DATA_Client",
            ClientOpcodeMessage::CMSG_LEAVE_BATTLEFIELD(_) => "CMSG_LEAVE_BATTLEFIELD",
            ClientOpcodeMessage::CMSG_AREA_SPIRIT_HEALER_QUERY(_) => "CMSG_AREA_SPIRIT_HEALER_QUERY",
            ClientOpcodeMessage::CMSG_AREA_SPIRIT_HEALER_QUEUE(_) => "CMSG_AREA_SPIRIT_HEALER_QUEUE",
            ClientOpcodeMessage::MSG_BATTLEGROUND_PLAYER_POSITIONS(_) => "MSG_BATTLEGROUND_PLAYER_POSITIONS_Client",
            ClientOpcodeMessage::CMSG_PET_STOP_ATTACK(_) => "CMSG_PET_STOP_ATTACK",
            ClientOpcodeMessage::CMSG_BATTLEMASTER_JOIN(_) => "CMSG_BATTLEMASTER_JOIN",
            ClientOpcodeMessage::CMSG_PET_UNLEARN(_) => "CMSG_PET_UNLEARN",
            ClientOpcodeMessage::CMSG_PET_SPELL_AUTOCAST(_) => "CMSG_PET_SPELL_AUTOCAST",
            ClientOpcodeMessage::CMSG_GUILD_INFO_TEXT(_) => "CMSG_GUILD_INFO_TEXT",
            ClientOpcodeMessage::CMSG_ACTIVATETAXIEXPRESS(_) => "CMSG_ACTIVATETAXIEXPRESS",
            ClientOpcodeMessage::CMSG_SET_FACTION_INACTIVE(_) => "CMSG_SET_FACTION_INACTIVE",
            ClientOpcodeMessage::CMSG_SET_WATCHED_FACTION(_) => "CMSG_SET_WATCHED_FACTION",
            ClientOpcodeMessage::CMSG_RESET_INSTANCES(_) => "CMSG_RESET_INSTANCES",
            ClientOpcodeMessage::MSG_RAID_READY_CHECK(_) => "MSG_RAID_READY_CHECK_Client",
            ClientOpcodeMessage::CMSG_GMSURVEY_SUBMIT(_) => "CMSG_GMSURVEY_SUBMIT",
            ClientOpcodeMessage::CMSG_MOVE_SET_FLY(_) => "CMSG_MOVE_SET_FLY",
            ClientOpcodeMessage::CMSG_LFD_PLAYER_LOCK_INFO_REQUEST(_) => "CMSG_LFD_PLAYER_LOCK_INFO_REQUEST",
            ClientOpcodeMessage::CMSG_REALM_SPLIT(_) => "CMSG_REALM_SPLIT",
            ClientOpcodeMessage::CMSG_MOVE_CHNG_TRANSPORT(_) => "CMSG_MOVE_CHNG_TRANSPORT",
            ClientOpcodeMessage::CMSG_TIME_SYNC_RESP(_) => "CMSG_TIME_SYNC_RESP",
            ClientOpcodeMessage::CMSG_VOICE_SESSION_ENABLE(_) => "CMSG_VOICE_SESSION_ENABLE",
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

impl From<MSG_QUEST_PUSH_RESULT> for ClientOpcodeMessage {
    fn from(c: MSG_QUEST_PUSH_RESULT) -> Self {
        Self::MSG_QUEST_PUSH_RESULT(c)
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

impl From<MSG_GUILD_BANK_MONEY_WITHDRAWN> for ClientOpcodeMessage {
    fn from(c: MSG_GUILD_BANK_MONEY_WITHDRAWN) -> Self {
        Self::MSG_GUILD_BANK_MONEY_WITHDRAWN(c)
    }
}

impl From<CMSG_CALENDAR_GET_NUM_PENDING> for ClientOpcodeMessage {
    fn from(c: CMSG_CALENDAR_GET_NUM_PENDING) -> Self {
        Self::CMSG_CALENDAR_GET_NUM_PENDING(c)
    }
}

impl From<CMSG_BOOTME> for ClientOpcodeMessage {
    fn from(c: CMSG_BOOTME) -> Self {
        Self::CMSG_BOOTME(c)
    }
}

impl From<CMSG_WORLD_TELEPORT> for ClientOpcodeMessage {
    fn from(c: CMSG_WORLD_TELEPORT) -> Self {
        Self::CMSG_WORLD_TELEPORT(c)
    }
}

impl From<CMSG_TELEPORT_TO_UNIT> for ClientOpcodeMessage {
    fn from(c: CMSG_TELEPORT_TO_UNIT) -> Self {
        Self::CMSG_TELEPORT_TO_UNIT(c)
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

impl From<CMSG_PET_NAME_QUERY> for ClientOpcodeMessage {
    fn from(c: CMSG_PET_NAME_QUERY) -> Self {
        Self::CMSG_PET_NAME_QUERY(c)
    }
}

impl From<CMSG_GUILD_QUERY> for ClientOpcodeMessage {
    fn from(c: CMSG_GUILD_QUERY) -> Self {
        Self::CMSG_GUILD_QUERY(c)
    }
}

impl From<CMSG_ITEM_QUERY_SINGLE> for ClientOpcodeMessage {
    fn from(c: CMSG_ITEM_QUERY_SINGLE) -> Self {
        Self::CMSG_ITEM_QUERY_SINGLE(c)
    }
}

impl From<CMSG_PAGE_TEXT_QUERY> for ClientOpcodeMessage {
    fn from(c: CMSG_PAGE_TEXT_QUERY) -> Self {
        Self::CMSG_PAGE_TEXT_QUERY(c)
    }
}

impl From<CMSG_QUEST_QUERY> for ClientOpcodeMessage {
    fn from(c: CMSG_QUEST_QUERY) -> Self {
        Self::CMSG_QUEST_QUERY(c)
    }
}

impl From<CMSG_GAMEOBJECT_QUERY> for ClientOpcodeMessage {
    fn from(c: CMSG_GAMEOBJECT_QUERY) -> Self {
        Self::CMSG_GAMEOBJECT_QUERY(c)
    }
}

impl From<CMSG_CREATURE_QUERY> for ClientOpcodeMessage {
    fn from(c: CMSG_CREATURE_QUERY) -> Self {
        Self::CMSG_CREATURE_QUERY(c)
    }
}

impl From<CMSG_WHO> for ClientOpcodeMessage {
    fn from(c: CMSG_WHO) -> Self {
        Self::CMSG_WHO(c)
    }
}

impl From<CMSG_WHOIS> for ClientOpcodeMessage {
    fn from(c: CMSG_WHOIS) -> Self {
        Self::CMSG_WHOIS(c)
    }
}

impl From<CMSG_CONTACT_LIST> for ClientOpcodeMessage {
    fn from(c: CMSG_CONTACT_LIST) -> Self {
        Self::CMSG_CONTACT_LIST(c)
    }
}

impl From<CMSG_ADD_FRIEND> for ClientOpcodeMessage {
    fn from(c: CMSG_ADD_FRIEND) -> Self {
        Self::CMSG_ADD_FRIEND(c)
    }
}

impl From<CMSG_DEL_FRIEND> for ClientOpcodeMessage {
    fn from(c: CMSG_DEL_FRIEND) -> Self {
        Self::CMSG_DEL_FRIEND(c)
    }
}

impl From<CMSG_ADD_IGNORE> for ClientOpcodeMessage {
    fn from(c: CMSG_ADD_IGNORE) -> Self {
        Self::CMSG_ADD_IGNORE(c)
    }
}

impl From<CMSG_DEL_IGNORE> for ClientOpcodeMessage {
    fn from(c: CMSG_DEL_IGNORE) -> Self {
        Self::CMSG_DEL_IGNORE(c)
    }
}

impl From<CMSG_GROUP_INVITE> for ClientOpcodeMessage {
    fn from(c: CMSG_GROUP_INVITE) -> Self {
        Self::CMSG_GROUP_INVITE(c)
    }
}

impl From<CMSG_GROUP_ACCEPT> for ClientOpcodeMessage {
    fn from(c: CMSG_GROUP_ACCEPT) -> Self {
        Self::CMSG_GROUP_ACCEPT(c)
    }
}

impl From<CMSG_GROUP_DECLINE> for ClientOpcodeMessage {
    fn from(c: CMSG_GROUP_DECLINE) -> Self {
        Self::CMSG_GROUP_DECLINE(c)
    }
}

impl From<CMSG_GROUP_UNINVITE> for ClientOpcodeMessage {
    fn from(c: CMSG_GROUP_UNINVITE) -> Self {
        Self::CMSG_GROUP_UNINVITE(c)
    }
}

impl From<CMSG_GROUP_UNINVITE_GUID> for ClientOpcodeMessage {
    fn from(c: CMSG_GROUP_UNINVITE_GUID) -> Self {
        Self::CMSG_GROUP_UNINVITE_GUID(c)
    }
}

impl From<CMSG_GROUP_SET_LEADER> for ClientOpcodeMessage {
    fn from(c: CMSG_GROUP_SET_LEADER) -> Self {
        Self::CMSG_GROUP_SET_LEADER(c)
    }
}

impl From<CMSG_LOOT_METHOD> for ClientOpcodeMessage {
    fn from(c: CMSG_LOOT_METHOD) -> Self {
        Self::CMSG_LOOT_METHOD(c)
    }
}

impl From<CMSG_GROUP_DISBAND> for ClientOpcodeMessage {
    fn from(c: CMSG_GROUP_DISBAND) -> Self {
        Self::CMSG_GROUP_DISBAND(c)
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

impl From<CMSG_USE_ITEM> for ClientOpcodeMessage {
    fn from(c: CMSG_USE_ITEM) -> Self {
        Self::CMSG_USE_ITEM(c)
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

impl From<CMSG_MOVE_SET_RAW_POSITION> for ClientOpcodeMessage {
    fn from(c: CMSG_MOVE_SET_RAW_POSITION) -> Self {
        Self::CMSG_MOVE_SET_RAW_POSITION(c)
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

impl From<CMSG_MOVE_KNOCK_BACK_ACK> for ClientOpcodeMessage {
    fn from(c: CMSG_MOVE_KNOCK_BACK_ACK) -> Self {
        Self::CMSG_MOVE_KNOCK_BACK_ACK(c)
    }
}

impl From<CMSG_MOVE_HOVER_ACK> for ClientOpcodeMessage {
    fn from(c: CMSG_MOVE_HOVER_ACK) -> Self {
        Self::CMSG_MOVE_HOVER_ACK(c)
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

impl From<CMSG_INITIATE_TRADE> for ClientOpcodeMessage {
    fn from(c: CMSG_INITIATE_TRADE) -> Self {
        Self::CMSG_INITIATE_TRADE(c)
    }
}

impl From<CMSG_BEGIN_TRADE> for ClientOpcodeMessage {
    fn from(c: CMSG_BEGIN_TRADE) -> Self {
        Self::CMSG_BEGIN_TRADE(c)
    }
}

impl From<CMSG_BUSY_TRADE> for ClientOpcodeMessage {
    fn from(c: CMSG_BUSY_TRADE) -> Self {
        Self::CMSG_BUSY_TRADE(c)
    }
}

impl From<CMSG_IGNORE_TRADE> for ClientOpcodeMessage {
    fn from(c: CMSG_IGNORE_TRADE) -> Self {
        Self::CMSG_IGNORE_TRADE(c)
    }
}

impl From<CMSG_ACCEPT_TRADE> for ClientOpcodeMessage {
    fn from(c: CMSG_ACCEPT_TRADE) -> Self {
        Self::CMSG_ACCEPT_TRADE(c)
    }
}

impl From<CMSG_UNACCEPT_TRADE> for ClientOpcodeMessage {
    fn from(c: CMSG_UNACCEPT_TRADE) -> Self {
        Self::CMSG_UNACCEPT_TRADE(c)
    }
}

impl From<CMSG_CANCEL_TRADE> for ClientOpcodeMessage {
    fn from(c: CMSG_CANCEL_TRADE) -> Self {
        Self::CMSG_CANCEL_TRADE(c)
    }
}

impl From<CMSG_SET_TRADE_ITEM> for ClientOpcodeMessage {
    fn from(c: CMSG_SET_TRADE_ITEM) -> Self {
        Self::CMSG_SET_TRADE_ITEM(c)
    }
}

impl From<CMSG_CLEAR_TRADE_ITEM> for ClientOpcodeMessage {
    fn from(c: CMSG_CLEAR_TRADE_ITEM) -> Self {
        Self::CMSG_CLEAR_TRADE_ITEM(c)
    }
}

impl From<CMSG_SET_TRADE_GOLD> for ClientOpcodeMessage {
    fn from(c: CMSG_SET_TRADE_GOLD) -> Self {
        Self::CMSG_SET_TRADE_GOLD(c)
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

impl From<CMSG_CAST_SPELL> for ClientOpcodeMessage {
    fn from(c: CMSG_CAST_SPELL) -> Self {
        Self::CMSG_CAST_SPELL(c)
    }
}

impl From<CMSG_CANCEL_CAST> for ClientOpcodeMessage {
    fn from(c: CMSG_CANCEL_CAST) -> Self {
        Self::CMSG_CANCEL_CAST(c)
    }
}

impl From<CMSG_CANCEL_AURA> for ClientOpcodeMessage {
    fn from(c: CMSG_CANCEL_AURA) -> Self {
        Self::CMSG_CANCEL_AURA(c)
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

impl From<CMSG_DELETEEQUIPMENT_SET> for ClientOpcodeMessage {
    fn from(c: CMSG_DELETEEQUIPMENT_SET) -> Self {
        Self::CMSG_DELETEEQUIPMENT_SET(c)
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

impl From<CMSG_RESURRECT_RESPONSE> for ClientOpcodeMessage {
    fn from(c: CMSG_RESURRECT_RESPONSE) -> Self {
        Self::CMSG_RESURRECT_RESPONSE(c)
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

impl From<CMSG_PET_SET_ACTION> for ClientOpcodeMessage {
    fn from(c: CMSG_PET_SET_ACTION) -> Self {
        Self::CMSG_PET_SET_ACTION(c)
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

impl From<CMSG_PET_RENAME> for ClientOpcodeMessage {
    fn from(c: CMSG_PET_RENAME) -> Self {
        Self::CMSG_PET_RENAME(c)
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

impl From<CMSG_NPC_TEXT_QUERY> for ClientOpcodeMessage {
    fn from(c: CMSG_NPC_TEXT_QUERY) -> Self {
        Self::CMSG_NPC_TEXT_QUERY(c)
    }
}

impl From<CMSG_QUESTGIVER_STATUS_QUERY> for ClientOpcodeMessage {
    fn from(c: CMSG_QUESTGIVER_STATUS_QUERY) -> Self {
        Self::CMSG_QUESTGIVER_STATUS_QUERY(c)
    }
}

impl From<CMSG_QUESTGIVER_HELLO> for ClientOpcodeMessage {
    fn from(c: CMSG_QUESTGIVER_HELLO) -> Self {
        Self::CMSG_QUESTGIVER_HELLO(c)
    }
}

impl From<CMSG_QUESTGIVER_QUERY_QUEST> for ClientOpcodeMessage {
    fn from(c: CMSG_QUESTGIVER_QUERY_QUEST) -> Self {
        Self::CMSG_QUESTGIVER_QUERY_QUEST(c)
    }
}

impl From<CMSG_QUESTGIVER_QUEST_AUTOLAUNCH> for ClientOpcodeMessage {
    fn from(c: CMSG_QUESTGIVER_QUEST_AUTOLAUNCH) -> Self {
        Self::CMSG_QUESTGIVER_QUEST_AUTOLAUNCH(c)
    }
}

impl From<CMSG_QUESTGIVER_ACCEPT_QUEST> for ClientOpcodeMessage {
    fn from(c: CMSG_QUESTGIVER_ACCEPT_QUEST) -> Self {
        Self::CMSG_QUESTGIVER_ACCEPT_QUEST(c)
    }
}

impl From<CMSG_QUESTGIVER_COMPLETE_QUEST> for ClientOpcodeMessage {
    fn from(c: CMSG_QUESTGIVER_COMPLETE_QUEST) -> Self {
        Self::CMSG_QUESTGIVER_COMPLETE_QUEST(c)
    }
}

impl From<CMSG_QUESTGIVER_REQUEST_REWARD> for ClientOpcodeMessage {
    fn from(c: CMSG_QUESTGIVER_REQUEST_REWARD) -> Self {
        Self::CMSG_QUESTGIVER_REQUEST_REWARD(c)
    }
}

impl From<CMSG_QUESTGIVER_CHOOSE_REWARD> for ClientOpcodeMessage {
    fn from(c: CMSG_QUESTGIVER_CHOOSE_REWARD) -> Self {
        Self::CMSG_QUESTGIVER_CHOOSE_REWARD(c)
    }
}

impl From<CMSG_QUESTGIVER_CANCEL> for ClientOpcodeMessage {
    fn from(c: CMSG_QUESTGIVER_CANCEL) -> Self {
        Self::CMSG_QUESTGIVER_CANCEL(c)
    }
}

impl From<CMSG_QUESTLOG_SWAP_QUEST> for ClientOpcodeMessage {
    fn from(c: CMSG_QUESTLOG_SWAP_QUEST) -> Self {
        Self::CMSG_QUESTLOG_SWAP_QUEST(c)
    }
}

impl From<CMSG_QUESTLOG_REMOVE_QUEST> for ClientOpcodeMessage {
    fn from(c: CMSG_QUESTLOG_REMOVE_QUEST) -> Self {
        Self::CMSG_QUESTLOG_REMOVE_QUEST(c)
    }
}

impl From<CMSG_QUEST_CONFIRM_ACCEPT> for ClientOpcodeMessage {
    fn from(c: CMSG_QUEST_CONFIRM_ACCEPT) -> Self {
        Self::CMSG_QUEST_CONFIRM_ACCEPT(c)
    }
}

impl From<CMSG_PUSHQUESTTOPARTY> for ClientOpcodeMessage {
    fn from(c: CMSG_PUSHQUESTTOPARTY) -> Self {
        Self::CMSG_PUSHQUESTTOPARTY(c)
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

impl From<CMSG_TAXINODE_STATUS_QUERY> for ClientOpcodeMessage {
    fn from(c: CMSG_TAXINODE_STATUS_QUERY) -> Self {
        Self::CMSG_TAXINODE_STATUS_QUERY(c)
    }
}

impl From<CMSG_TAXIQUERYAVAILABLENODES> for ClientOpcodeMessage {
    fn from(c: CMSG_TAXIQUERYAVAILABLENODES) -> Self {
        Self::CMSG_TAXIQUERYAVAILABLENODES(c)
    }
}

impl From<CMSG_ACTIVATETAXI> for ClientOpcodeMessage {
    fn from(c: CMSG_ACTIVATETAXI) -> Self {
        Self::CMSG_ACTIVATETAXI(c)
    }
}

impl From<CMSG_TRAINER_LIST> for ClientOpcodeMessage {
    fn from(c: CMSG_TRAINER_LIST) -> Self {
        Self::CMSG_TRAINER_LIST(c)
    }
}

impl From<CMSG_TRAINER_BUY_SPELL> for ClientOpcodeMessage {
    fn from(c: CMSG_TRAINER_BUY_SPELL) -> Self {
        Self::CMSG_TRAINER_BUY_SPELL(c)
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

impl From<CMSG_PETITION_QUERY> for ClientOpcodeMessage {
    fn from(c: CMSG_PETITION_QUERY) -> Self {
        Self::CMSG_PETITION_QUERY(c)
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

impl From<CMSG_RECLAIM_CORPSE> for ClientOpcodeMessage {
    fn from(c: CMSG_RECLAIM_CORPSE) -> Self {
        Self::CMSG_RECLAIM_CORPSE(c)
    }
}

impl From<CMSG_WRAP_ITEM> for ClientOpcodeMessage {
    fn from(c: CMSG_WRAP_ITEM) -> Self {
        Self::CMSG_WRAP_ITEM(c)
    }
}

impl From<MSG_MINIMAP_PING_Client> for ClientOpcodeMessage {
    fn from(c: MSG_MINIMAP_PING_Client) -> Self {
        Self::MSG_MINIMAP_PING(c)
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

impl From<CMSG_PET_CAST_SPELL> for ClientOpcodeMessage {
    fn from(c: CMSG_PET_CAST_SPELL) -> Self {
        Self::CMSG_PET_CAST_SPELL(c)
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

impl From<CMSG_UNLEARN_SKILL> for ClientOpcodeMessage {
    fn from(c: CMSG_UNLEARN_SKILL) -> Self {
        Self::CMSG_UNLEARN_SKILL(c)
    }
}

impl From<CMSG_GMTICKET_CREATE> for ClientOpcodeMessage {
    fn from(c: CMSG_GMTICKET_CREATE) -> Self {
        Self::CMSG_GMTICKET_CREATE(c)
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

impl From<MSG_CORPSE_QUERY_Client> for ClientOpcodeMessage {
    fn from(c: MSG_CORPSE_QUERY_Client) -> Self {
        Self::MSG_CORPSE_QUERY(c)
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

impl From<CMSG_SPIRIT_HEALER_ACTIVATE> for ClientOpcodeMessage {
    fn from(c: CMSG_SPIRIT_HEALER_ACTIVATE) -> Self {
        Self::CMSG_SPIRIT_HEALER_ACTIVATE(c)
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

impl From<CMSG_ITEM_TEXT_QUERY> for ClientOpcodeMessage {
    fn from(c: CMSG_ITEM_TEXT_QUERY) -> Self {
        Self::CMSG_ITEM_TEXT_QUERY(c)
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

impl From<CMSG_LEARN_TALENT> for ClientOpcodeMessage {
    fn from(c: CMSG_LEARN_TALENT) -> Self {
        Self::CMSG_LEARN_TALENT(c)
    }
}

impl From<CMSG_TOGGLE_PVP> for ClientOpcodeMessage {
    fn from(c: CMSG_TOGGLE_PVP) -> Self {
        Self::CMSG_TOGGLE_PVP(c)
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

impl From<CMSG_PET_CANCEL_AURA> for ClientOpcodeMessage {
    fn from(c: CMSG_PET_CANCEL_AURA) -> Self {
        Self::CMSG_PET_CANCEL_AURA(c)
    }
}

impl From<CMSG_CANCEL_AUTO_REPEAT_SPELL> for ClientOpcodeMessage {
    fn from(c: CMSG_CANCEL_AUTO_REPEAT_SPELL) -> Self {
        Self::CMSG_CANCEL_AUTO_REPEAT_SPELL(c)
    }
}

impl From<MSG_LIST_STABLED_PETS_Client> for ClientOpcodeMessage {
    fn from(c: MSG_LIST_STABLED_PETS_Client) -> Self {
        Self::MSG_LIST_STABLED_PETS(c)
    }
}

impl From<CMSG_STABLE_PET> for ClientOpcodeMessage {
    fn from(c: CMSG_STABLE_PET) -> Self {
        Self::CMSG_STABLE_PET(c)
    }
}

impl From<CMSG_UNSTABLE_PET> for ClientOpcodeMessage {
    fn from(c: CMSG_UNSTABLE_PET) -> Self {
        Self::CMSG_UNSTABLE_PET(c)
    }
}

impl From<CMSG_BUY_STABLE_SLOT> for ClientOpcodeMessage {
    fn from(c: CMSG_BUY_STABLE_SLOT) -> Self {
        Self::CMSG_BUY_STABLE_SLOT(c)
    }
}

impl From<CMSG_STABLE_SWAP_PET> for ClientOpcodeMessage {
    fn from(c: CMSG_STABLE_SWAP_PET) -> Self {
        Self::CMSG_STABLE_SWAP_PET(c)
    }
}

impl From<CMSG_REQUEST_PET_INFO> for ClientOpcodeMessage {
    fn from(c: CMSG_REQUEST_PET_INFO) -> Self {
        Self::CMSG_REQUEST_PET_INFO(c)
    }
}

impl From<CMSG_FAR_SIGHT> for ClientOpcodeMessage {
    fn from(c: CMSG_FAR_SIGHT) -> Self {
        Self::CMSG_FAR_SIGHT(c)
    }
}

impl From<CMSG_GROUP_CHANGE_SUB_GROUP> for ClientOpcodeMessage {
    fn from(c: CMSG_GROUP_CHANGE_SUB_GROUP) -> Self {
        Self::CMSG_GROUP_CHANGE_SUB_GROUP(c)
    }
}

impl From<CMSG_REQUEST_PARTY_MEMBER_STATS> for ClientOpcodeMessage {
    fn from(c: CMSG_REQUEST_PARTY_MEMBER_STATS) -> Self {
        Self::CMSG_REQUEST_PARTY_MEMBER_STATS(c)
    }
}

impl From<CMSG_GROUP_SWAP_SUB_GROUP> for ClientOpcodeMessage {
    fn from(c: CMSG_GROUP_SWAP_SUB_GROUP) -> Self {
        Self::CMSG_GROUP_SWAP_SUB_GROUP(c)
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

impl From<MSG_QUERY_NEXT_MAIL_TIME_Client> for ClientOpcodeMessage {
    fn from(c: MSG_QUERY_NEXT_MAIL_TIME_Client) -> Self {
        Self::MSG_QUERY_NEXT_MAIL_TIME(c)
    }
}

impl From<CMSG_GROUP_RAID_CONVERT> for ClientOpcodeMessage {
    fn from(c: CMSG_GROUP_RAID_CONVERT) -> Self {
        Self::CMSG_GROUP_RAID_CONVERT(c)
    }
}

impl From<CMSG_GROUP_ASSISTANT_LEADER> for ClientOpcodeMessage {
    fn from(c: CMSG_GROUP_ASSISTANT_LEADER) -> Self {
        Self::CMSG_GROUP_ASSISTANT_LEADER(c)
    }
}

impl From<CMSG_BUYBACK_ITEM> for ClientOpcodeMessage {
    fn from(c: CMSG_BUYBACK_ITEM) -> Self {
        Self::CMSG_BUYBACK_ITEM(c)
    }
}

impl From<CMSG_LFG_GET_STATUS> for ClientOpcodeMessage {
    fn from(c: CMSG_LFG_GET_STATUS) -> Self {
        Self::CMSG_LFG_GET_STATUS(c)
    }
}

impl From<CMSG_CANCEL_GROWTH_AURA> for ClientOpcodeMessage {
    fn from(c: CMSG_CANCEL_GROWTH_AURA) -> Self {
        Self::CMSG_CANCEL_GROWTH_AURA(c)
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

impl From<CMSG_SUMMON_RESPONSE> for ClientOpcodeMessage {
    fn from(c: CMSG_SUMMON_RESPONSE) -> Self {
        Self::CMSG_SUMMON_RESPONSE(c)
    }
}

impl From<CMSG_SELF_RES> for ClientOpcodeMessage {
    fn from(c: CMSG_SELF_RES) -> Self {
        Self::CMSG_SELF_RES(c)
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

impl From<CMSG_ITEM_NAME_QUERY> for ClientOpcodeMessage {
    fn from(c: CMSG_ITEM_NAME_QUERY) -> Self {
        Self::CMSG_ITEM_NAME_QUERY(c)
    }
}

impl From<CMSG_CHAR_RENAME> for ClientOpcodeMessage {
    fn from(c: CMSG_CHAR_RENAME) -> Self {
        Self::CMSG_CHAR_RENAME(c)
    }
}

impl From<CMSG_MOVE_SPLINE_DONE> for ClientOpcodeMessage {
    fn from(c: CMSG_MOVE_SPLINE_DONE) -> Self {
        Self::CMSG_MOVE_SPLINE_DONE(c)
    }
}

impl From<CMSG_MOVE_FALL_RESET> for ClientOpcodeMessage {
    fn from(c: CMSG_MOVE_FALL_RESET) -> Self {
        Self::CMSG_MOVE_FALL_RESET(c)
    }
}

impl From<CMSG_REQUEST_RAID_INFO> for ClientOpcodeMessage {
    fn from(c: CMSG_REQUEST_RAID_INFO) -> Self {
        Self::CMSG_REQUEST_RAID_INFO(c)
    }
}

impl From<CMSG_MOVE_TIME_SKIPPED> for ClientOpcodeMessage {
    fn from(c: CMSG_MOVE_TIME_SKIPPED) -> Self {
        Self::CMSG_MOVE_TIME_SKIPPED(c)
    }
}

impl From<CMSG_MOVE_FEATHER_FALL_ACK> for ClientOpcodeMessage {
    fn from(c: CMSG_MOVE_FEATHER_FALL_ACK) -> Self {
        Self::CMSG_MOVE_FEATHER_FALL_ACK(c)
    }
}

impl From<CMSG_MOVE_WATER_WALK_ACK> for ClientOpcodeMessage {
    fn from(c: CMSG_MOVE_WATER_WALK_ACK) -> Self {
        Self::CMSG_MOVE_WATER_WALK_ACK(c)
    }
}

impl From<CMSG_MOVE_NOT_ACTIVE_MOVER> for ClientOpcodeMessage {
    fn from(c: CMSG_MOVE_NOT_ACTIVE_MOVER) -> Self {
        Self::CMSG_MOVE_NOT_ACTIVE_MOVER(c)
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

impl From<MSG_INSPECT_HONOR_STATS_Client> for ClientOpcodeMessage {
    fn from(c: MSG_INSPECT_HONOR_STATS_Client) -> Self {
        Self::MSG_INSPECT_HONOR_STATS(c)
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

impl From<MSG_PVP_LOG_DATA_Client> for ClientOpcodeMessage {
    fn from(c: MSG_PVP_LOG_DATA_Client) -> Self {
        Self::MSG_PVP_LOG_DATA(c)
    }
}

impl From<CMSG_LEAVE_BATTLEFIELD> for ClientOpcodeMessage {
    fn from(c: CMSG_LEAVE_BATTLEFIELD) -> Self {
        Self::CMSG_LEAVE_BATTLEFIELD(c)
    }
}

impl From<CMSG_AREA_SPIRIT_HEALER_QUERY> for ClientOpcodeMessage {
    fn from(c: CMSG_AREA_SPIRIT_HEALER_QUERY) -> Self {
        Self::CMSG_AREA_SPIRIT_HEALER_QUERY(c)
    }
}

impl From<CMSG_AREA_SPIRIT_HEALER_QUEUE> for ClientOpcodeMessage {
    fn from(c: CMSG_AREA_SPIRIT_HEALER_QUEUE) -> Self {
        Self::CMSG_AREA_SPIRIT_HEALER_QUEUE(c)
    }
}

impl From<MSG_BATTLEGROUND_PLAYER_POSITIONS_Client> for ClientOpcodeMessage {
    fn from(c: MSG_BATTLEGROUND_PLAYER_POSITIONS_Client) -> Self {
        Self::MSG_BATTLEGROUND_PLAYER_POSITIONS(c)
    }
}

impl From<CMSG_PET_STOP_ATTACK> for ClientOpcodeMessage {
    fn from(c: CMSG_PET_STOP_ATTACK) -> Self {
        Self::CMSG_PET_STOP_ATTACK(c)
    }
}

impl From<CMSG_BATTLEMASTER_JOIN> for ClientOpcodeMessage {
    fn from(c: CMSG_BATTLEMASTER_JOIN) -> Self {
        Self::CMSG_BATTLEMASTER_JOIN(c)
    }
}

impl From<CMSG_PET_UNLEARN> for ClientOpcodeMessage {
    fn from(c: CMSG_PET_UNLEARN) -> Self {
        Self::CMSG_PET_UNLEARN(c)
    }
}

impl From<CMSG_PET_SPELL_AUTOCAST> for ClientOpcodeMessage {
    fn from(c: CMSG_PET_SPELL_AUTOCAST) -> Self {
        Self::CMSG_PET_SPELL_AUTOCAST(c)
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

impl From<CMSG_SET_FACTION_INACTIVE> for ClientOpcodeMessage {
    fn from(c: CMSG_SET_FACTION_INACTIVE) -> Self {
        Self::CMSG_SET_FACTION_INACTIVE(c)
    }
}

impl From<CMSG_SET_WATCHED_FACTION> for ClientOpcodeMessage {
    fn from(c: CMSG_SET_WATCHED_FACTION) -> Self {
        Self::CMSG_SET_WATCHED_FACTION(c)
    }
}

impl From<CMSG_RESET_INSTANCES> for ClientOpcodeMessage {
    fn from(c: CMSG_RESET_INSTANCES) -> Self {
        Self::CMSG_RESET_INSTANCES(c)
    }
}

impl From<MSG_RAID_READY_CHECK_Client> for ClientOpcodeMessage {
    fn from(c: MSG_RAID_READY_CHECK_Client) -> Self {
        Self::MSG_RAID_READY_CHECK(c)
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

impl From<CMSG_LFD_PLAYER_LOCK_INFO_REQUEST> for ClientOpcodeMessage {
    fn from(c: CMSG_LFD_PLAYER_LOCK_INFO_REQUEST) -> Self {
        Self::CMSG_LFD_PLAYER_LOCK_INFO_REQUEST(c)
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

impl From<CMSG_VOICE_SESSION_ENABLE> for ClientOpcodeMessage {
    fn from(c: CMSG_VOICE_SESSION_ENABLE) -> Self {
        Self::CMSG_VOICE_SESSION_ENABLE(c)
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
use crate::world::wrath::SMSG_TRANSFER_ABORTED;
use crate::world::wrath::SMSG_CHARACTER_LOGIN_FAILED;
use crate::world::wrath::SMSG_LOGIN_SETTIMESPEED;
use crate::world::wrath::SMSG_LOGOUT_RESPONSE;
use crate::world::wrath::SMSG_LOGOUT_COMPLETE;
use crate::world::wrath::SMSG_LOGOUT_CANCEL_ACK;
use crate::world::wrath::SMSG_NAME_QUERY_RESPONSE;
use crate::world::wrath::SMSG_PET_NAME_QUERY_RESPONSE;
use crate::world::wrath::SMSG_GUILD_QUERY_RESPONSE;
use crate::world::wrath::SMSG_ITEM_QUERY_SINGLE_RESPONSE;
use crate::world::wrath::SMSG_PAGE_TEXT_QUERY_RESPONSE;
use crate::world::wrath::SMSG_QUEST_QUERY_RESPONSE;
use crate::world::wrath::SMSG_GAMEOBJECT_QUERY_RESPONSE;
use crate::world::wrath::SMSG_CREATURE_QUERY_RESPONSE;
use crate::world::wrath::SMSG_WHO;
use crate::world::wrath::SMSG_WHOIS;
use crate::world::wrath::SMSG_CONTACT_LIST;
use crate::world::wrath::SMSG_FRIEND_STATUS;
use crate::world::wrath::SMSG_GROUP_INVITE;
use crate::world::wrath::SMSG_GROUP_DECLINE;
use crate::world::wrath::SMSG_GROUP_UNINVITE;
use crate::world::wrath::SMSG_GROUP_SET_LEADER;
use crate::world::wrath::SMSG_GROUP_DESTROYED;
use crate::world::wrath::SMSG_GROUP_LIST;
use crate::world::wrath::SMSG_PARTY_COMMAND_RESULT;
use crate::world::wrath::SMSG_GUILD_INVITE;
use crate::world::wrath::SMSG_GUILD_INFO;
use crate::world::wrath::SMSG_GUILD_ROSTER;
use crate::world::wrath::SMSG_GUILD_EVENT;
use crate::world::wrath::SMSG_GUILD_COMMAND_RESULT;
use crate::world::wrath::SMSG_MESSAGECHAT;
use crate::world::wrath::SMSG_CHANNEL_NOTIFY;
use crate::world::wrath::SMSG_CHANNEL_LIST;
use crate::world::wrath::SMSG_UPDATE_OBJECT;
use crate::world::wrath::SMSG_DESTROY_OBJECT;
use crate::world::wrath::SMSG_READ_ITEM_OK;
use crate::world::wrath::SMSG_READ_ITEM_FAILED;
use crate::world::wrath::SMSG_ITEM_COOLDOWN;
use crate::world::wrath::SMSG_GAMEOBJECT_CUSTOM_ANIM;
use crate::world::wrath::MSG_MOVE_TELEPORT_ACK_Server;
use crate::world::wrath::SMSG_MOVE_WATER_WALK;
use crate::world::wrath::SMSG_MOVE_LAND_WALK;
use crate::world::wrath::SMSG_FORCE_RUN_SPEED_CHANGE;
use crate::world::wrath::SMSG_FORCE_RUN_BACK_SPEED_CHANGE;
use crate::world::wrath::SMSG_FORCE_SWIM_SPEED_CHANGE;
use crate::world::wrath::SMSG_FORCE_MOVE_ROOT;
use crate::world::wrath::SMSG_FORCE_MOVE_UNROOT;
use crate::world::wrath::SMSG_MOVE_KNOCK_BACK;
use crate::world::wrath::SMSG_MOVE_FEATHER_FALL;
use crate::world::wrath::SMSG_MOVE_NORMAL_FALL;
use crate::world::wrath::SMSG_MOVE_SET_HOVER;
use crate::world::wrath::SMSG_MOVE_UNSET_HOVER;
use crate::world::wrath::SMSG_TRIGGER_CINEMATIC;
use crate::world::wrath::SMSG_TUTORIAL_FLAGS;
use crate::world::wrath::SMSG_EMOTE;
use crate::world::wrath::SMSG_TEXT_EMOTE;
use crate::world::wrath::SMSG_INVENTORY_CHANGE_FAILURE;
use crate::world::wrath::SMSG_TRADE_STATUS;
use crate::world::wrath::SMSG_TRADE_STATUS_EXTENDED;
use crate::world::wrath::SMSG_INITIALIZE_FACTIONS;
use crate::world::wrath::SMSG_SET_FACTION_VISIBLE;
use crate::world::wrath::SMSG_SET_FACTION_STANDING;
use crate::world::wrath::SMSG_SET_PROFICIENCY;
use crate::world::wrath::SMSG_ACTION_BUTTONS;
use crate::world::wrath::SMSG_INITIAL_SPELLS;
use crate::world::wrath::SMSG_LEARNED_SPELL;
use crate::world::wrath::SMSG_SUPERCEDED_SPELL;
use crate::world::wrath::SMSG_SPELL_START;
use crate::world::wrath::SMSG_SPELL_GO;
use crate::world::wrath::SMSG_SPELL_FAILURE;
use crate::world::wrath::SMSG_SPELL_COOLDOWN;
use crate::world::wrath::SMSG_COOLDOWN_EVENT;
use crate::world::wrath::SMSG_PET_CAST_FAILED;
use crate::world::wrath::SMSG_AI_REACTION;
use crate::world::wrath::SMSG_ATTACKSTART;
use crate::world::wrath::SMSG_ATTACKSTOP;
use crate::world::wrath::SMSG_ATTACKSWING_NOTINRANGE;
use crate::world::wrath::SMSG_ATTACKSWING_BADFACING;
use crate::world::wrath::SMSG_ATTACKSWING_DEADTARGET;
use crate::world::wrath::SMSG_ATTACKSWING_CANT_ATTACK;
use crate::world::wrath::SMSG_ATTACKERSTATEUPDATE;
use crate::world::wrath::SMSG_CANCEL_COMBAT;
use crate::world::wrath::SMSG_SPELLHEALLOG;
use crate::world::wrath::SMSG_SPELLENERGIZELOG;
use crate::world::wrath::SMSG_BINDPOINTUPDATE;
use crate::world::wrath::SMSG_PLAYERBOUND;
use crate::world::wrath::SMSG_CLIENT_CONTROL_UPDATE;
use crate::world::wrath::SMSG_RESURRECT_REQUEST;
use crate::world::wrath::SMSG_LOOT_RESPONSE;
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
use crate::world::wrath::SMSG_PET_TAME_FAILURE;
use crate::world::wrath::SMSG_PET_NAME_INVALID;
use crate::world::wrath::SMSG_PET_SPELLS;
use crate::world::wrath::SMSG_PET_MODE;
use crate::world::wrath::SMSG_GOSSIP_MESSAGE;
use crate::world::wrath::SMSG_GOSSIP_COMPLETE;
use crate::world::wrath::SMSG_NPC_TEXT_UPDATE;
use crate::world::wrath::SMSG_QUESTGIVER_STATUS;
use crate::world::wrath::SMSG_QUESTGIVER_QUEST_LIST;
use crate::world::wrath::SMSG_QUESTGIVER_QUEST_DETAILS;
use crate::world::wrath::SMSG_QUESTGIVER_REQUEST_ITEMS;
use crate::world::wrath::SMSG_QUESTGIVER_OFFER_REWARD;
use crate::world::wrath::SMSG_QUESTGIVER_QUEST_INVALID;
use crate::world::wrath::SMSG_QUESTGIVER_QUEST_COMPLETE;
use crate::world::wrath::SMSG_QUESTGIVER_QUEST_FAILED;
use crate::world::wrath::SMSG_QUESTLOG_FULL;
use crate::world::wrath::SMSG_QUESTUPDATE_FAILED;
use crate::world::wrath::SMSG_QUESTUPDATE_FAILEDTIMER;
use crate::world::wrath::SMSG_QUESTUPDATE_COMPLETE;
use crate::world::wrath::SMSG_QUESTUPDATE_ADD_KILL;
use crate::world::wrath::SMSG_QUESTUPDATE_ADD_ITEM;
use crate::world::wrath::SMSG_QUEST_CONFIRM_ACCEPT;
use crate::world::wrath::SMSG_LIST_INVENTORY;
use crate::world::wrath::SMSG_SELL_ITEM;
use crate::world::wrath::SMSG_BUY_ITEM;
use crate::world::wrath::SMSG_BUY_FAILED;
use crate::world::wrath::SMSG_SHOWTAXINODES;
use crate::world::wrath::SMSG_TAXINODE_STATUS;
use crate::world::wrath::SMSG_ACTIVATETAXIREPLY;
use crate::world::wrath::SMSG_NEW_TAXI_PATH;
use crate::world::wrath::SMSG_TRAINER_LIST;
use crate::world::wrath::SMSG_TRAINER_BUY_SUCCEEDED;
use crate::world::wrath::SMSG_TRAINER_BUY_FAILED;
use crate::world::wrath::SMSG_SHOW_BANK;
use crate::world::wrath::SMSG_BUY_BANK_SLOT_RESULT;
use crate::world::wrath::SMSG_PETITION_SHOWLIST;
use crate::world::wrath::SMSG_PETITION_SHOW_SIGNATURES;
use crate::world::wrath::SMSG_PETITION_SIGN_RESULTS;
use crate::world::wrath::SMSG_TURN_IN_PETITION_RESULTS;
use crate::world::wrath::SMSG_PETITION_QUERY_RESPONSE;
use crate::world::wrath::SMSG_FISH_NOT_HOOKED;
use crate::world::wrath::SMSG_FISH_ESCAPED;
use crate::world::wrath::SMSG_NOTIFICATION;
use crate::world::wrath::SMSG_PLAYED_TIME;
use crate::world::wrath::SMSG_QUERY_TIME_RESPONSE;
use crate::world::wrath::SMSG_LOG_XPGAIN;
use crate::world::wrath::SMSG_LEVELUP_INFO;
use crate::world::wrath::MSG_MINIMAP_PING_Server;
use crate::world::wrath::SMSG_PONG;
use crate::world::wrath::SMSG_GAMEOBJECT_PAGETEXT;
use crate::world::wrath::SMSG_ITEM_TIME_UPDATE;
use crate::world::wrath::SMSG_ITEM_ENCHANT_TIME_UPDATE;
use crate::world::wrath::SMSG_AUTH_CHALLENGE;
use crate::world::wrath::SMSG_AUTH_RESPONSE;
use crate::world::wrath::MSG_SAVE_GUILD_EMBLEM_Server;
use crate::world::wrath::SMSG_EXPLORATION_EXPERIENCE;
use crate::world::wrath::MSG_RANDOM_ROLL_Server;
use crate::world::wrath::SMSG_ENVIRONMENTAL_DAMAGE_LOG;
use crate::world::wrath::SMSG_GMTICKET_CREATE;
use crate::world::wrath::SMSG_GMTICKET_UPDATETEXT;
use crate::world::wrath::SMSG_ACCOUNT_DATA_TIMES;
use crate::world::wrath::SMSG_UPDATE_ACCOUNT_DATA;
use crate::world::wrath::SMSG_GMTICKET_GETTICKET;
use crate::world::wrath::SMSG_GAMEOBJECT_DESPAWN_ANIM;
use crate::world::wrath::MSG_CORPSE_QUERY_Server;
use crate::world::wrath::SMSG_GMTICKET_DELETETICKET;
use crate::world::wrath::SMSG_CHAT_WRONG_FACTION;
use crate::world::wrath::SMSG_GMTICKET_SYSTEMSTATUS;
use crate::world::wrath::SMSG_QUEST_FORCE_REMOVE;
use crate::world::wrath::SMSG_GOSSIP_POI;
use crate::world::wrath::SMSG_LOGIN_VERIFY_WORLD;
use crate::world::wrath::SMSG_BATTLEFIELD_LIST;
use crate::world::wrath::SMSG_ITEM_TEXT_QUERY_RESPONSE;
use crate::world::wrath::SMSG_ZONE_UNDER_ATTACK;
use crate::world::wrath::MSG_AUCTION_HELLO_Server;
use crate::world::wrath::SMSG_AUCTION_LIST_RESULT;
use crate::world::wrath::SMSG_AUCTION_OWNER_LIST_RESULT;
use crate::world::wrath::SMSG_AUCTION_BIDDER_NOTIFICATION;
use crate::world::wrath::SMSG_AUCTION_OWNER_NOTIFICATION;
use crate::world::wrath::SMSG_PROCRESIST;
use crate::world::wrath::SMSG_AUCTION_BIDDER_LIST_RESULT;
use crate::world::wrath::MSG_LIST_STABLED_PETS_Server;
use crate::world::wrath::SMSG_STABLE_RESULT;
use crate::world::wrath::SMSG_PLAY_OBJECT_SOUND;
use crate::world::wrath::MSG_QUERY_NEXT_MAIL_TIME_Server;
use crate::world::wrath::SMSG_RECEIVED_MAIL;
use crate::world::wrath::SMSG_PVP_CREDIT;
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
use crate::world::wrath::SMSG_PET_BROKEN;
use crate::world::wrath::SMSG_DUEL_COUNTDOWN;
use crate::world::wrath::SMSG_AREA_TRIGGER_MESSAGE;
use crate::world::wrath::SMSG_DURABILITY_DAMAGE_DEATH;
use crate::world::wrath::SMSG_INIT_WORLD_STATES;
use crate::world::wrath::SMSG_UPDATE_WORLD_STATE;
use crate::world::wrath::SMSG_ITEM_NAME_QUERY_RESPONSE;
use crate::world::wrath::SMSG_PET_ACTION_FEEDBACK;
use crate::world::wrath::SMSG_CHAR_RENAME;
use crate::world::wrath::SMSG_RAID_INSTANCE_INFO;
use crate::world::wrath::MSG_INSPECT_HONOR_STATS_Server;
use crate::world::wrath::SMSG_FORCE_WALK_SPEED_CHANGE;
use crate::world::wrath::SMSG_FORCE_SWIM_BACK_SPEED_CHANGE;
use crate::world::wrath::SMSG_FORCE_TURN_RATE_CHANGE;
use crate::world::wrath::MSG_BATTLEGROUND_PLAYER_POSITIONS_Server;
use crate::world::wrath::SMSG_BINDER_CONFIRM;
use crate::world::wrath::SMSG_BATTLEGROUND_PLAYER_JOINED;
use crate::world::wrath::SMSG_BATTLEGROUND_PLAYER_LEFT;
use crate::world::wrath::SMSG_ADDON_INFO;
use crate::world::wrath::SMSG_PET_UNLEARN_CONFIRM;
use crate::world::wrath::SMSG_CHAT_RESTRICTED;
use crate::world::wrath::SMSG_SPLINE_SET_RUN_SPEED;
use crate::world::wrath::SMSG_SPLINE_SET_RUN_BACK_SPEED;
use crate::world::wrath::SMSG_SPLINE_SET_SWIM_SPEED;
use crate::world::wrath::SMSG_SPLINE_SET_WALK_SPEED;
use crate::world::wrath::SMSG_SPLINE_SET_SWIM_BACK_SPEED;
use crate::world::wrath::SMSG_SPLINE_SET_TURN_RATE;
use crate::world::wrath::SMSG_SPLINE_MOVE_UNROOT;
use crate::world::wrath::SMSG_SPLINE_MOVE_FEATHER_FALL;
use crate::world::wrath::SMSG_SPLINE_MOVE_NORMAL_FALL;
use crate::world::wrath::SMSG_SPLINE_MOVE_SET_HOVER;
use crate::world::wrath::SMSG_SPLINE_MOVE_UNSET_HOVER;
use crate::world::wrath::SMSG_SPLINE_MOVE_WATER_WALK;
use crate::world::wrath::SMSG_SPLINE_MOVE_LAND_WALK;
use crate::world::wrath::SMSG_SPLINE_MOVE_START_SWIM;
use crate::world::wrath::SMSG_SPLINE_MOVE_STOP_SWIM;
use crate::world::wrath::SMSG_SPLINE_MOVE_SET_RUN_MODE;
use crate::world::wrath::SMSG_SPLINE_MOVE_SET_WALK_MODE;
use crate::world::wrath::SMSG_SPLINE_MOVE_ROOT;
use crate::world::wrath::MSG_RAID_READY_CHECK_Server;
use crate::world::wrath::SMSG_PET_ACTION_SOUND;
use crate::world::wrath::SMSG_PET_DISMISS_SOUND;
use crate::world::wrath::SMSG_GM_TICKET_STATUS_UPDATE;
use crate::world::wrath::SMSG_DEFENSE_MESSAGE;
use crate::world::wrath::SMSG_REALM_SPLIT;
use crate::world::wrath::SMSG_TIME_SYNC_REQ;
use crate::world::wrath::SMSG_GM_MESSAGECHAT;
use crate::world::wrath::SMSG_FEATURE_SYSTEM_STATUS;
use crate::world::wrath::SMSG_CALENDAR_SEND_NUM_PENDING;
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
    MSG_QUEST_PUSH_RESULT(MSG_QUEST_PUSH_RESULT),
    MSG_PETITION_RENAME(MSG_PETITION_RENAME),
    MSG_SET_DUNGEON_DIFFICULTY(MSG_SET_DUNGEON_DIFFICULTY),
    MSG_MOVE_START_ASCEND(MSG_MOVE_START_ASCEND),
    MSG_MOVE_STOP_ASCEND(MSG_MOVE_STOP_ASCEND),
    MSG_MOVE_START_DESCEND(MSG_MOVE_START_DESCEND),
    MSG_GUILD_BANK_MONEY_WITHDRAWN(MSG_GUILD_BANK_MONEY_WITHDRAWN),
    CMSG_CALENDAR_GET_NUM_PENDING(CMSG_CALENDAR_GET_NUM_PENDING),
    SMSG_CHAR_CREATE(SMSG_CHAR_CREATE),
    SMSG_CHAR_ENUM(SMSG_CHAR_ENUM),
    SMSG_CHAR_DELETE(SMSG_CHAR_DELETE),
    SMSG_NEW_WORLD(SMSG_NEW_WORLD),
    SMSG_TRANSFER_PENDING(SMSG_TRANSFER_PENDING),
    SMSG_TRANSFER_ABORTED(SMSG_TRANSFER_ABORTED),
    SMSG_CHARACTER_LOGIN_FAILED(SMSG_CHARACTER_LOGIN_FAILED),
    SMSG_LOGIN_SETTIMESPEED(SMSG_LOGIN_SETTIMESPEED),
    SMSG_LOGOUT_RESPONSE(SMSG_LOGOUT_RESPONSE),
    SMSG_LOGOUT_COMPLETE(SMSG_LOGOUT_COMPLETE),
    SMSG_LOGOUT_CANCEL_ACK(SMSG_LOGOUT_CANCEL_ACK),
    SMSG_NAME_QUERY_RESPONSE(SMSG_NAME_QUERY_RESPONSE),
    SMSG_PET_NAME_QUERY_RESPONSE(SMSG_PET_NAME_QUERY_RESPONSE),
    SMSG_GUILD_QUERY_RESPONSE(SMSG_GUILD_QUERY_RESPONSE),
    SMSG_ITEM_QUERY_SINGLE_RESPONSE(SMSG_ITEM_QUERY_SINGLE_RESPONSE),
    SMSG_PAGE_TEXT_QUERY_RESPONSE(SMSG_PAGE_TEXT_QUERY_RESPONSE),
    SMSG_QUEST_QUERY_RESPONSE(SMSG_QUEST_QUERY_RESPONSE),
    SMSG_GAMEOBJECT_QUERY_RESPONSE(SMSG_GAMEOBJECT_QUERY_RESPONSE),
    SMSG_CREATURE_QUERY_RESPONSE(SMSG_CREATURE_QUERY_RESPONSE),
    SMSG_WHO(SMSG_WHO),
    SMSG_WHOIS(SMSG_WHOIS),
    SMSG_CONTACT_LIST(SMSG_CONTACT_LIST),
    SMSG_FRIEND_STATUS(SMSG_FRIEND_STATUS),
    SMSG_GROUP_INVITE(SMSG_GROUP_INVITE),
    SMSG_GROUP_DECLINE(SMSG_GROUP_DECLINE),
    SMSG_GROUP_UNINVITE(SMSG_GROUP_UNINVITE),
    SMSG_GROUP_SET_LEADER(SMSG_GROUP_SET_LEADER),
    SMSG_GROUP_DESTROYED(SMSG_GROUP_DESTROYED),
    SMSG_GROUP_LIST(SMSG_GROUP_LIST),
    SMSG_PARTY_COMMAND_RESULT(SMSG_PARTY_COMMAND_RESULT),
    SMSG_GUILD_INVITE(SMSG_GUILD_INVITE),
    SMSG_GUILD_INFO(SMSG_GUILD_INFO),
    SMSG_GUILD_ROSTER(SMSG_GUILD_ROSTER),
    SMSG_GUILD_EVENT(SMSG_GUILD_EVENT),
    SMSG_GUILD_COMMAND_RESULT(SMSG_GUILD_COMMAND_RESULT),
    SMSG_MESSAGECHAT(SMSG_MESSAGECHAT),
    SMSG_CHANNEL_NOTIFY(SMSG_CHANNEL_NOTIFY),
    SMSG_CHANNEL_LIST(SMSG_CHANNEL_LIST),
    SMSG_UPDATE_OBJECT(SMSG_UPDATE_OBJECT),
    SMSG_DESTROY_OBJECT(SMSG_DESTROY_OBJECT),
    SMSG_READ_ITEM_OK(SMSG_READ_ITEM_OK),
    SMSG_READ_ITEM_FAILED(SMSG_READ_ITEM_FAILED),
    SMSG_ITEM_COOLDOWN(SMSG_ITEM_COOLDOWN),
    SMSG_GAMEOBJECT_CUSTOM_ANIM(SMSG_GAMEOBJECT_CUSTOM_ANIM),
    MSG_MOVE_TELEPORT_ACK(MSG_MOVE_TELEPORT_ACK_Server),
    SMSG_MOVE_WATER_WALK(SMSG_MOVE_WATER_WALK),
    SMSG_MOVE_LAND_WALK(SMSG_MOVE_LAND_WALK),
    SMSG_FORCE_RUN_SPEED_CHANGE(SMSG_FORCE_RUN_SPEED_CHANGE),
    SMSG_FORCE_RUN_BACK_SPEED_CHANGE(SMSG_FORCE_RUN_BACK_SPEED_CHANGE),
    SMSG_FORCE_SWIM_SPEED_CHANGE(SMSG_FORCE_SWIM_SPEED_CHANGE),
    SMSG_FORCE_MOVE_ROOT(SMSG_FORCE_MOVE_ROOT),
    SMSG_FORCE_MOVE_UNROOT(SMSG_FORCE_MOVE_UNROOT),
    SMSG_MOVE_KNOCK_BACK(SMSG_MOVE_KNOCK_BACK),
    SMSG_MOVE_FEATHER_FALL(SMSG_MOVE_FEATHER_FALL),
    SMSG_MOVE_NORMAL_FALL(SMSG_MOVE_NORMAL_FALL),
    SMSG_MOVE_SET_HOVER(SMSG_MOVE_SET_HOVER),
    SMSG_MOVE_UNSET_HOVER(SMSG_MOVE_UNSET_HOVER),
    SMSG_TRIGGER_CINEMATIC(SMSG_TRIGGER_CINEMATIC),
    SMSG_TUTORIAL_FLAGS(SMSG_TUTORIAL_FLAGS),
    SMSG_EMOTE(SMSG_EMOTE),
    SMSG_TEXT_EMOTE(SMSG_TEXT_EMOTE),
    SMSG_INVENTORY_CHANGE_FAILURE(SMSG_INVENTORY_CHANGE_FAILURE),
    SMSG_TRADE_STATUS(SMSG_TRADE_STATUS),
    SMSG_TRADE_STATUS_EXTENDED(SMSG_TRADE_STATUS_EXTENDED),
    SMSG_INITIALIZE_FACTIONS(SMSG_INITIALIZE_FACTIONS),
    SMSG_SET_FACTION_VISIBLE(SMSG_SET_FACTION_VISIBLE),
    SMSG_SET_FACTION_STANDING(SMSG_SET_FACTION_STANDING),
    SMSG_SET_PROFICIENCY(SMSG_SET_PROFICIENCY),
    SMSG_ACTION_BUTTONS(SMSG_ACTION_BUTTONS),
    SMSG_INITIAL_SPELLS(SMSG_INITIAL_SPELLS),
    SMSG_LEARNED_SPELL(SMSG_LEARNED_SPELL),
    SMSG_SUPERCEDED_SPELL(SMSG_SUPERCEDED_SPELL),
    SMSG_SPELL_START(SMSG_SPELL_START),
    SMSG_SPELL_GO(SMSG_SPELL_GO),
    SMSG_SPELL_FAILURE(SMSG_SPELL_FAILURE),
    SMSG_SPELL_COOLDOWN(SMSG_SPELL_COOLDOWN),
    SMSG_COOLDOWN_EVENT(SMSG_COOLDOWN_EVENT),
    SMSG_PET_CAST_FAILED(SMSG_PET_CAST_FAILED),
    SMSG_AI_REACTION(SMSG_AI_REACTION),
    SMSG_ATTACKSTART(SMSG_ATTACKSTART),
    SMSG_ATTACKSTOP(SMSG_ATTACKSTOP),
    SMSG_ATTACKSWING_NOTINRANGE(SMSG_ATTACKSWING_NOTINRANGE),
    SMSG_ATTACKSWING_BADFACING(SMSG_ATTACKSWING_BADFACING),
    SMSG_ATTACKSWING_DEADTARGET(SMSG_ATTACKSWING_DEADTARGET),
    SMSG_ATTACKSWING_CANT_ATTACK(SMSG_ATTACKSWING_CANT_ATTACK),
    SMSG_ATTACKERSTATEUPDATE(SMSG_ATTACKERSTATEUPDATE),
    SMSG_CANCEL_COMBAT(SMSG_CANCEL_COMBAT),
    SMSG_SPELLHEALLOG(SMSG_SPELLHEALLOG),
    SMSG_SPELLENERGIZELOG(SMSG_SPELLENERGIZELOG),
    SMSG_BINDPOINTUPDATE(SMSG_BINDPOINTUPDATE),
    SMSG_PLAYERBOUND(SMSG_PLAYERBOUND),
    SMSG_CLIENT_CONTROL_UPDATE(SMSG_CLIENT_CONTROL_UPDATE),
    SMSG_RESURRECT_REQUEST(SMSG_RESURRECT_REQUEST),
    SMSG_LOOT_RESPONSE(SMSG_LOOT_RESPONSE),
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
    SMSG_PET_TAME_FAILURE(SMSG_PET_TAME_FAILURE),
    SMSG_PET_NAME_INVALID(SMSG_PET_NAME_INVALID),
    SMSG_PET_SPELLS(SMSG_PET_SPELLS),
    SMSG_PET_MODE(SMSG_PET_MODE),
    SMSG_GOSSIP_MESSAGE(SMSG_GOSSIP_MESSAGE),
    SMSG_GOSSIP_COMPLETE(SMSG_GOSSIP_COMPLETE),
    SMSG_NPC_TEXT_UPDATE(SMSG_NPC_TEXT_UPDATE),
    SMSG_QUESTGIVER_STATUS(SMSG_QUESTGIVER_STATUS),
    SMSG_QUESTGIVER_QUEST_LIST(SMSG_QUESTGIVER_QUEST_LIST),
    SMSG_QUESTGIVER_QUEST_DETAILS(SMSG_QUESTGIVER_QUEST_DETAILS),
    SMSG_QUESTGIVER_REQUEST_ITEMS(SMSG_QUESTGIVER_REQUEST_ITEMS),
    SMSG_QUESTGIVER_OFFER_REWARD(SMSG_QUESTGIVER_OFFER_REWARD),
    SMSG_QUESTGIVER_QUEST_INVALID(SMSG_QUESTGIVER_QUEST_INVALID),
    SMSG_QUESTGIVER_QUEST_COMPLETE(SMSG_QUESTGIVER_QUEST_COMPLETE),
    SMSG_QUESTGIVER_QUEST_FAILED(SMSG_QUESTGIVER_QUEST_FAILED),
    SMSG_QUESTLOG_FULL(SMSG_QUESTLOG_FULL),
    SMSG_QUESTUPDATE_FAILED(SMSG_QUESTUPDATE_FAILED),
    SMSG_QUESTUPDATE_FAILEDTIMER(SMSG_QUESTUPDATE_FAILEDTIMER),
    SMSG_QUESTUPDATE_COMPLETE(SMSG_QUESTUPDATE_COMPLETE),
    SMSG_QUESTUPDATE_ADD_KILL(SMSG_QUESTUPDATE_ADD_KILL),
    SMSG_QUESTUPDATE_ADD_ITEM(SMSG_QUESTUPDATE_ADD_ITEM),
    SMSG_QUEST_CONFIRM_ACCEPT(SMSG_QUEST_CONFIRM_ACCEPT),
    SMSG_LIST_INVENTORY(SMSG_LIST_INVENTORY),
    SMSG_SELL_ITEM(SMSG_SELL_ITEM),
    SMSG_BUY_ITEM(SMSG_BUY_ITEM),
    SMSG_BUY_FAILED(SMSG_BUY_FAILED),
    SMSG_SHOWTAXINODES(SMSG_SHOWTAXINODES),
    SMSG_TAXINODE_STATUS(SMSG_TAXINODE_STATUS),
    SMSG_ACTIVATETAXIREPLY(SMSG_ACTIVATETAXIREPLY),
    SMSG_NEW_TAXI_PATH(SMSG_NEW_TAXI_PATH),
    SMSG_TRAINER_LIST(SMSG_TRAINER_LIST),
    SMSG_TRAINER_BUY_SUCCEEDED(SMSG_TRAINER_BUY_SUCCEEDED),
    SMSG_TRAINER_BUY_FAILED(SMSG_TRAINER_BUY_FAILED),
    SMSG_SHOW_BANK(SMSG_SHOW_BANK),
    SMSG_BUY_BANK_SLOT_RESULT(SMSG_BUY_BANK_SLOT_RESULT),
    SMSG_PETITION_SHOWLIST(SMSG_PETITION_SHOWLIST),
    SMSG_PETITION_SHOW_SIGNATURES(SMSG_PETITION_SHOW_SIGNATURES),
    SMSG_PETITION_SIGN_RESULTS(SMSG_PETITION_SIGN_RESULTS),
    SMSG_TURN_IN_PETITION_RESULTS(SMSG_TURN_IN_PETITION_RESULTS),
    SMSG_PETITION_QUERY_RESPONSE(SMSG_PETITION_QUERY_RESPONSE),
    SMSG_FISH_NOT_HOOKED(SMSG_FISH_NOT_HOOKED),
    SMSG_FISH_ESCAPED(SMSG_FISH_ESCAPED),
    SMSG_NOTIFICATION(SMSG_NOTIFICATION),
    SMSG_PLAYED_TIME(SMSG_PLAYED_TIME),
    SMSG_QUERY_TIME_RESPONSE(SMSG_QUERY_TIME_RESPONSE),
    SMSG_LOG_XPGAIN(SMSG_LOG_XPGAIN),
    SMSG_LEVELUP_INFO(SMSG_LEVELUP_INFO),
    MSG_MINIMAP_PING(MSG_MINIMAP_PING_Server),
    SMSG_PONG(SMSG_PONG),
    SMSG_GAMEOBJECT_PAGETEXT(SMSG_GAMEOBJECT_PAGETEXT),
    SMSG_ITEM_TIME_UPDATE(SMSG_ITEM_TIME_UPDATE),
    SMSG_ITEM_ENCHANT_TIME_UPDATE(SMSG_ITEM_ENCHANT_TIME_UPDATE),
    SMSG_AUTH_CHALLENGE(SMSG_AUTH_CHALLENGE),
    SMSG_AUTH_RESPONSE(SMSG_AUTH_RESPONSE),
    MSG_SAVE_GUILD_EMBLEM(MSG_SAVE_GUILD_EMBLEM_Server),
    SMSG_EXPLORATION_EXPERIENCE(SMSG_EXPLORATION_EXPERIENCE),
    MSG_RANDOM_ROLL(MSG_RANDOM_ROLL_Server),
    SMSG_ENVIRONMENTAL_DAMAGE_LOG(SMSG_ENVIRONMENTAL_DAMAGE_LOG),
    SMSG_GMTICKET_CREATE(SMSG_GMTICKET_CREATE),
    SMSG_GMTICKET_UPDATETEXT(SMSG_GMTICKET_UPDATETEXT),
    SMSG_ACCOUNT_DATA_TIMES(SMSG_ACCOUNT_DATA_TIMES),
    SMSG_UPDATE_ACCOUNT_DATA(SMSG_UPDATE_ACCOUNT_DATA),
    SMSG_GMTICKET_GETTICKET(SMSG_GMTICKET_GETTICKET),
    SMSG_GAMEOBJECT_DESPAWN_ANIM(SMSG_GAMEOBJECT_DESPAWN_ANIM),
    MSG_CORPSE_QUERY(MSG_CORPSE_QUERY_Server),
    SMSG_GMTICKET_DELETETICKET(SMSG_GMTICKET_DELETETICKET),
    SMSG_CHAT_WRONG_FACTION(SMSG_CHAT_WRONG_FACTION),
    SMSG_GMTICKET_SYSTEMSTATUS(SMSG_GMTICKET_SYSTEMSTATUS),
    SMSG_QUEST_FORCE_REMOVE(SMSG_QUEST_FORCE_REMOVE),
    SMSG_GOSSIP_POI(SMSG_GOSSIP_POI),
    SMSG_LOGIN_VERIFY_WORLD(SMSG_LOGIN_VERIFY_WORLD),
    SMSG_BATTLEFIELD_LIST(SMSG_BATTLEFIELD_LIST),
    SMSG_ITEM_TEXT_QUERY_RESPONSE(SMSG_ITEM_TEXT_QUERY_RESPONSE),
    SMSG_ZONE_UNDER_ATTACK(SMSG_ZONE_UNDER_ATTACK),
    MSG_AUCTION_HELLO(MSG_AUCTION_HELLO_Server),
    SMSG_AUCTION_LIST_RESULT(SMSG_AUCTION_LIST_RESULT),
    SMSG_AUCTION_OWNER_LIST_RESULT(SMSG_AUCTION_OWNER_LIST_RESULT),
    SMSG_AUCTION_BIDDER_NOTIFICATION(SMSG_AUCTION_BIDDER_NOTIFICATION),
    SMSG_AUCTION_OWNER_NOTIFICATION(SMSG_AUCTION_OWNER_NOTIFICATION),
    SMSG_PROCRESIST(SMSG_PROCRESIST),
    SMSG_AUCTION_BIDDER_LIST_RESULT(SMSG_AUCTION_BIDDER_LIST_RESULT),
    MSG_LIST_STABLED_PETS(MSG_LIST_STABLED_PETS_Server),
    SMSG_STABLE_RESULT(SMSG_STABLE_RESULT),
    SMSG_PLAY_OBJECT_SOUND(SMSG_PLAY_OBJECT_SOUND),
    MSG_QUERY_NEXT_MAIL_TIME(MSG_QUERY_NEXT_MAIL_TIME_Server),
    SMSG_RECEIVED_MAIL(SMSG_RECEIVED_MAIL),
    SMSG_PVP_CREDIT(SMSG_PVP_CREDIT),
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
    SMSG_PET_BROKEN(SMSG_PET_BROKEN),
    SMSG_DUEL_COUNTDOWN(SMSG_DUEL_COUNTDOWN),
    SMSG_AREA_TRIGGER_MESSAGE(SMSG_AREA_TRIGGER_MESSAGE),
    SMSG_DURABILITY_DAMAGE_DEATH(SMSG_DURABILITY_DAMAGE_DEATH),
    SMSG_INIT_WORLD_STATES(SMSG_INIT_WORLD_STATES),
    SMSG_UPDATE_WORLD_STATE(SMSG_UPDATE_WORLD_STATE),
    SMSG_ITEM_NAME_QUERY_RESPONSE(SMSG_ITEM_NAME_QUERY_RESPONSE),
    SMSG_PET_ACTION_FEEDBACK(SMSG_PET_ACTION_FEEDBACK),
    SMSG_CHAR_RENAME(SMSG_CHAR_RENAME),
    SMSG_RAID_INSTANCE_INFO(SMSG_RAID_INSTANCE_INFO),
    MSG_INSPECT_HONOR_STATS(MSG_INSPECT_HONOR_STATS_Server),
    SMSG_FORCE_WALK_SPEED_CHANGE(SMSG_FORCE_WALK_SPEED_CHANGE),
    SMSG_FORCE_SWIM_BACK_SPEED_CHANGE(SMSG_FORCE_SWIM_BACK_SPEED_CHANGE),
    SMSG_FORCE_TURN_RATE_CHANGE(SMSG_FORCE_TURN_RATE_CHANGE),
    MSG_BATTLEGROUND_PLAYER_POSITIONS(MSG_BATTLEGROUND_PLAYER_POSITIONS_Server),
    SMSG_BINDER_CONFIRM(SMSG_BINDER_CONFIRM),
    SMSG_BATTLEGROUND_PLAYER_JOINED(SMSG_BATTLEGROUND_PLAYER_JOINED),
    SMSG_BATTLEGROUND_PLAYER_LEFT(SMSG_BATTLEGROUND_PLAYER_LEFT),
    SMSG_ADDON_INFO(SMSG_ADDON_INFO),
    SMSG_PET_UNLEARN_CONFIRM(SMSG_PET_UNLEARN_CONFIRM),
    SMSG_CHAT_RESTRICTED(SMSG_CHAT_RESTRICTED),
    SMSG_SPLINE_SET_RUN_SPEED(SMSG_SPLINE_SET_RUN_SPEED),
    SMSG_SPLINE_SET_RUN_BACK_SPEED(SMSG_SPLINE_SET_RUN_BACK_SPEED),
    SMSG_SPLINE_SET_SWIM_SPEED(SMSG_SPLINE_SET_SWIM_SPEED),
    SMSG_SPLINE_SET_WALK_SPEED(SMSG_SPLINE_SET_WALK_SPEED),
    SMSG_SPLINE_SET_SWIM_BACK_SPEED(SMSG_SPLINE_SET_SWIM_BACK_SPEED),
    SMSG_SPLINE_SET_TURN_RATE(SMSG_SPLINE_SET_TURN_RATE),
    SMSG_SPLINE_MOVE_UNROOT(SMSG_SPLINE_MOVE_UNROOT),
    SMSG_SPLINE_MOVE_FEATHER_FALL(SMSG_SPLINE_MOVE_FEATHER_FALL),
    SMSG_SPLINE_MOVE_NORMAL_FALL(SMSG_SPLINE_MOVE_NORMAL_FALL),
    SMSG_SPLINE_MOVE_SET_HOVER(SMSG_SPLINE_MOVE_SET_HOVER),
    SMSG_SPLINE_MOVE_UNSET_HOVER(SMSG_SPLINE_MOVE_UNSET_HOVER),
    SMSG_SPLINE_MOVE_WATER_WALK(SMSG_SPLINE_MOVE_WATER_WALK),
    SMSG_SPLINE_MOVE_LAND_WALK(SMSG_SPLINE_MOVE_LAND_WALK),
    SMSG_SPLINE_MOVE_START_SWIM(SMSG_SPLINE_MOVE_START_SWIM),
    SMSG_SPLINE_MOVE_STOP_SWIM(SMSG_SPLINE_MOVE_STOP_SWIM),
    SMSG_SPLINE_MOVE_SET_RUN_MODE(SMSG_SPLINE_MOVE_SET_RUN_MODE),
    SMSG_SPLINE_MOVE_SET_WALK_MODE(SMSG_SPLINE_MOVE_SET_WALK_MODE),
    SMSG_SPLINE_MOVE_ROOT(SMSG_SPLINE_MOVE_ROOT),
    MSG_RAID_READY_CHECK(MSG_RAID_READY_CHECK_Server),
    SMSG_PET_ACTION_SOUND(SMSG_PET_ACTION_SOUND),
    SMSG_PET_DISMISS_SOUND(SMSG_PET_DISMISS_SOUND),
    SMSG_GM_TICKET_STATUS_UPDATE(SMSG_GM_TICKET_STATUS_UPDATE),
    SMSG_DEFENSE_MESSAGE(SMSG_DEFENSE_MESSAGE),
    SMSG_REALM_SPLIT(SMSG_REALM_SPLIT),
    SMSG_TIME_SYNC_REQ(SMSG_TIME_SYNC_REQ),
    SMSG_GM_MESSAGECHAT(SMSG_GM_MESSAGECHAT),
    SMSG_FEATURE_SYSTEM_STATUS(SMSG_FEATURE_SYSTEM_STATUS),
    SMSG_CALENDAR_SEND_NUM_PENDING(SMSG_CALENDAR_SEND_NUM_PENDING),
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
            0x0276 => Ok(Self::MSG_QUEST_PUSH_RESULT(<MSG_QUEST_PUSH_RESULT as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x0276, size: body_size, io, } } else { a } })?)),
            0x02C1 => Ok(Self::MSG_PETITION_RENAME(<MSG_PETITION_RENAME as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x02C1, size: body_size, io, } } else { a } })?)),
            0x0329 => Ok(Self::MSG_SET_DUNGEON_DIFFICULTY(<MSG_SET_DUNGEON_DIFFICULTY as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x0329, size: body_size, io, } } else { a } })?)),
            0x0359 => Ok(Self::MSG_MOVE_START_ASCEND(<MSG_MOVE_START_ASCEND as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x0359, size: body_size, io, } } else { a } })?)),
            0x035A => Ok(Self::MSG_MOVE_STOP_ASCEND(<MSG_MOVE_STOP_ASCEND as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x035A, size: body_size, io, } } else { a } })?)),
            0x03A7 => Ok(Self::MSG_MOVE_START_DESCEND(<MSG_MOVE_START_DESCEND as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x03A7, size: body_size, io, } } else { a } })?)),
            0x03FE => Ok(Self::MSG_GUILD_BANK_MONEY_WITHDRAWN(<MSG_GUILD_BANK_MONEY_WITHDRAWN as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x03FE, size: body_size, io, } } else { a } })?)),
            0x0447 => Ok(Self::CMSG_CALENDAR_GET_NUM_PENDING(<CMSG_CALENDAR_GET_NUM_PENDING as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x0447, size: body_size, io, } } else { a } })?)),
            0x003A => Ok(Self::SMSG_CHAR_CREATE(<SMSG_CHAR_CREATE as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x003A, size: body_size, io, } } else { a } })?)),
            0x003B => Ok(Self::SMSG_CHAR_ENUM(<SMSG_CHAR_ENUM as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x003B, size: body_size, io, } } else { a } })?)),
            0x003C => Ok(Self::SMSG_CHAR_DELETE(<SMSG_CHAR_DELETE as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x003C, size: body_size, io, } } else { a } })?)),
            0x003E => Ok(Self::SMSG_NEW_WORLD(<SMSG_NEW_WORLD as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x003E, size: body_size, io, } } else { a } })?)),
            0x003F => Ok(Self::SMSG_TRANSFER_PENDING(<SMSG_TRANSFER_PENDING as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x003F, size: body_size, io, } } else { a } })?)),
            0x0040 => Ok(Self::SMSG_TRANSFER_ABORTED(<SMSG_TRANSFER_ABORTED as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x0040, size: body_size, io, } } else { a } })?)),
            0x0041 => Ok(Self::SMSG_CHARACTER_LOGIN_FAILED(<SMSG_CHARACTER_LOGIN_FAILED as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x0041, size: body_size, io, } } else { a } })?)),
            0x0042 => Ok(Self::SMSG_LOGIN_SETTIMESPEED(<SMSG_LOGIN_SETTIMESPEED as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x0042, size: body_size, io, } } else { a } })?)),
            0x004C => Ok(Self::SMSG_LOGOUT_RESPONSE(<SMSG_LOGOUT_RESPONSE as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x004C, size: body_size, io, } } else { a } })?)),
            0x004D => Ok(Self::SMSG_LOGOUT_COMPLETE(<SMSG_LOGOUT_COMPLETE as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x004D, size: body_size, io, } } else { a } })?)),
            0x004F => Ok(Self::SMSG_LOGOUT_CANCEL_ACK(<SMSG_LOGOUT_CANCEL_ACK as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x004F, size: body_size, io, } } else { a } })?)),
            0x0051 => Ok(Self::SMSG_NAME_QUERY_RESPONSE(<SMSG_NAME_QUERY_RESPONSE as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x0051, size: body_size, io, } } else { a } })?)),
            0x0053 => Ok(Self::SMSG_PET_NAME_QUERY_RESPONSE(<SMSG_PET_NAME_QUERY_RESPONSE as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x0053, size: body_size, io, } } else { a } })?)),
            0x0055 => Ok(Self::SMSG_GUILD_QUERY_RESPONSE(<SMSG_GUILD_QUERY_RESPONSE as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x0055, size: body_size, io, } } else { a } })?)),
            0x0058 => Ok(Self::SMSG_ITEM_QUERY_SINGLE_RESPONSE(<SMSG_ITEM_QUERY_SINGLE_RESPONSE as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x0058, size: body_size, io, } } else { a } })?)),
            0x005B => Ok(Self::SMSG_PAGE_TEXT_QUERY_RESPONSE(<SMSG_PAGE_TEXT_QUERY_RESPONSE as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x005B, size: body_size, io, } } else { a } })?)),
            0x005D => Ok(Self::SMSG_QUEST_QUERY_RESPONSE(<SMSG_QUEST_QUERY_RESPONSE as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x005D, size: body_size, io, } } else { a } })?)),
            0x005F => Ok(Self::SMSG_GAMEOBJECT_QUERY_RESPONSE(<SMSG_GAMEOBJECT_QUERY_RESPONSE as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x005F, size: body_size, io, } } else { a } })?)),
            0x0061 => Ok(Self::SMSG_CREATURE_QUERY_RESPONSE(<SMSG_CREATURE_QUERY_RESPONSE as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x0061, size: body_size, io, } } else { a } })?)),
            0x0063 => Ok(Self::SMSG_WHO(<SMSG_WHO as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x0063, size: body_size, io, } } else { a } })?)),
            0x0065 => Ok(Self::SMSG_WHOIS(<SMSG_WHOIS as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x0065, size: body_size, io, } } else { a } })?)),
            0x0067 => Ok(Self::SMSG_CONTACT_LIST(<SMSG_CONTACT_LIST as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x0067, size: body_size, io, } } else { a } })?)),
            0x0068 => Ok(Self::SMSG_FRIEND_STATUS(<SMSG_FRIEND_STATUS as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x0068, size: body_size, io, } } else { a } })?)),
            0x006F => Ok(Self::SMSG_GROUP_INVITE(<SMSG_GROUP_INVITE as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x006F, size: body_size, io, } } else { a } })?)),
            0x0074 => Ok(Self::SMSG_GROUP_DECLINE(<SMSG_GROUP_DECLINE as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x0074, size: body_size, io, } } else { a } })?)),
            0x0077 => Ok(Self::SMSG_GROUP_UNINVITE(<SMSG_GROUP_UNINVITE as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x0077, size: body_size, io, } } else { a } })?)),
            0x0079 => Ok(Self::SMSG_GROUP_SET_LEADER(<SMSG_GROUP_SET_LEADER as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x0079, size: body_size, io, } } else { a } })?)),
            0x007C => Ok(Self::SMSG_GROUP_DESTROYED(<SMSG_GROUP_DESTROYED as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x007C, size: body_size, io, } } else { a } })?)),
            0x007D => Ok(Self::SMSG_GROUP_LIST(<SMSG_GROUP_LIST as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x007D, size: body_size, io, } } else { a } })?)),
            0x007F => Ok(Self::SMSG_PARTY_COMMAND_RESULT(<SMSG_PARTY_COMMAND_RESULT as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x007F, size: body_size, io, } } else { a } })?)),
            0x0083 => Ok(Self::SMSG_GUILD_INVITE(<SMSG_GUILD_INVITE as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x0083, size: body_size, io, } } else { a } })?)),
            0x0088 => Ok(Self::SMSG_GUILD_INFO(<SMSG_GUILD_INFO as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x0088, size: body_size, io, } } else { a } })?)),
            0x008A => Ok(Self::SMSG_GUILD_ROSTER(<SMSG_GUILD_ROSTER as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x008A, size: body_size, io, } } else { a } })?)),
            0x0092 => Ok(Self::SMSG_GUILD_EVENT(<SMSG_GUILD_EVENT as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x0092, size: body_size, io, } } else { a } })?)),
            0x0093 => Ok(Self::SMSG_GUILD_COMMAND_RESULT(<SMSG_GUILD_COMMAND_RESULT as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x0093, size: body_size, io, } } else { a } })?)),
            0x0096 => Ok(Self::SMSG_MESSAGECHAT(<SMSG_MESSAGECHAT as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x0096, size: body_size, io, } } else { a } })?)),
            0x0099 => Ok(Self::SMSG_CHANNEL_NOTIFY(<SMSG_CHANNEL_NOTIFY as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x0099, size: body_size, io, } } else { a } })?)),
            0x009B => Ok(Self::SMSG_CHANNEL_LIST(<SMSG_CHANNEL_LIST as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x009B, size: body_size, io, } } else { a } })?)),
            0x00A9 => Ok(Self::SMSG_UPDATE_OBJECT(<SMSG_UPDATE_OBJECT as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x00A9, size: body_size, io, } } else { a } })?)),
            0x00AA => Ok(Self::SMSG_DESTROY_OBJECT(<SMSG_DESTROY_OBJECT as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x00AA, size: body_size, io, } } else { a } })?)),
            0x00AE => Ok(Self::SMSG_READ_ITEM_OK(<SMSG_READ_ITEM_OK as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x00AE, size: body_size, io, } } else { a } })?)),
            0x00AF => Ok(Self::SMSG_READ_ITEM_FAILED(<SMSG_READ_ITEM_FAILED as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x00AF, size: body_size, io, } } else { a } })?)),
            0x00B0 => Ok(Self::SMSG_ITEM_COOLDOWN(<SMSG_ITEM_COOLDOWN as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x00B0, size: body_size, io, } } else { a } })?)),
            0x00B3 => Ok(Self::SMSG_GAMEOBJECT_CUSTOM_ANIM(<SMSG_GAMEOBJECT_CUSTOM_ANIM as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x00B3, size: body_size, io, } } else { a } })?)),
            0x00C7 => Ok(Self::MSG_MOVE_TELEPORT_ACK(<MSG_MOVE_TELEPORT_ACK_Server as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x00C7, size: body_size, io, } } else { a } })?)),
            0x00DE => Ok(Self::SMSG_MOVE_WATER_WALK(<SMSG_MOVE_WATER_WALK as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x00DE, size: body_size, io, } } else { a } })?)),
            0x00DF => Ok(Self::SMSG_MOVE_LAND_WALK(<SMSG_MOVE_LAND_WALK as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x00DF, size: body_size, io, } } else { a } })?)),
            0x00E2 => Ok(Self::SMSG_FORCE_RUN_SPEED_CHANGE(<SMSG_FORCE_RUN_SPEED_CHANGE as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x00E2, size: body_size, io, } } else { a } })?)),
            0x00E4 => Ok(Self::SMSG_FORCE_RUN_BACK_SPEED_CHANGE(<SMSG_FORCE_RUN_BACK_SPEED_CHANGE as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x00E4, size: body_size, io, } } else { a } })?)),
            0x00E6 => Ok(Self::SMSG_FORCE_SWIM_SPEED_CHANGE(<SMSG_FORCE_SWIM_SPEED_CHANGE as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x00E6, size: body_size, io, } } else { a } })?)),
            0x00E8 => Ok(Self::SMSG_FORCE_MOVE_ROOT(<SMSG_FORCE_MOVE_ROOT as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x00E8, size: body_size, io, } } else { a } })?)),
            0x00EA => Ok(Self::SMSG_FORCE_MOVE_UNROOT(<SMSG_FORCE_MOVE_UNROOT as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x00EA, size: body_size, io, } } else { a } })?)),
            0x00EF => Ok(Self::SMSG_MOVE_KNOCK_BACK(<SMSG_MOVE_KNOCK_BACK as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x00EF, size: body_size, io, } } else { a } })?)),
            0x00F2 => Ok(Self::SMSG_MOVE_FEATHER_FALL(<SMSG_MOVE_FEATHER_FALL as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x00F2, size: body_size, io, } } else { a } })?)),
            0x00F3 => Ok(Self::SMSG_MOVE_NORMAL_FALL(<SMSG_MOVE_NORMAL_FALL as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x00F3, size: body_size, io, } } else { a } })?)),
            0x00F4 => Ok(Self::SMSG_MOVE_SET_HOVER(<SMSG_MOVE_SET_HOVER as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x00F4, size: body_size, io, } } else { a } })?)),
            0x00F5 => Ok(Self::SMSG_MOVE_UNSET_HOVER(<SMSG_MOVE_UNSET_HOVER as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x00F5, size: body_size, io, } } else { a } })?)),
            0x00FA => Ok(Self::SMSG_TRIGGER_CINEMATIC(<SMSG_TRIGGER_CINEMATIC as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x00FA, size: body_size, io, } } else { a } })?)),
            0x00FD => Ok(Self::SMSG_TUTORIAL_FLAGS(<SMSG_TUTORIAL_FLAGS as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x00FD, size: body_size, io, } } else { a } })?)),
            0x0103 => Ok(Self::SMSG_EMOTE(<SMSG_EMOTE as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x0103, size: body_size, io, } } else { a } })?)),
            0x0105 => Ok(Self::SMSG_TEXT_EMOTE(<SMSG_TEXT_EMOTE as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x0105, size: body_size, io, } } else { a } })?)),
            0x0112 => Ok(Self::SMSG_INVENTORY_CHANGE_FAILURE(<SMSG_INVENTORY_CHANGE_FAILURE as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x0112, size: body_size, io, } } else { a } })?)),
            0x0120 => Ok(Self::SMSG_TRADE_STATUS(<SMSG_TRADE_STATUS as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x0120, size: body_size, io, } } else { a } })?)),
            0x0121 => Ok(Self::SMSG_TRADE_STATUS_EXTENDED(<SMSG_TRADE_STATUS_EXTENDED as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x0121, size: body_size, io, } } else { a } })?)),
            0x0122 => Ok(Self::SMSG_INITIALIZE_FACTIONS(<SMSG_INITIALIZE_FACTIONS as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x0122, size: body_size, io, } } else { a } })?)),
            0x0123 => Ok(Self::SMSG_SET_FACTION_VISIBLE(<SMSG_SET_FACTION_VISIBLE as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x0123, size: body_size, io, } } else { a } })?)),
            0x0124 => Ok(Self::SMSG_SET_FACTION_STANDING(<SMSG_SET_FACTION_STANDING as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x0124, size: body_size, io, } } else { a } })?)),
            0x0127 => Ok(Self::SMSG_SET_PROFICIENCY(<SMSG_SET_PROFICIENCY as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x0127, size: body_size, io, } } else { a } })?)),
            0x0129 => Ok(Self::SMSG_ACTION_BUTTONS(<SMSG_ACTION_BUTTONS as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x0129, size: body_size, io, } } else { a } })?)),
            0x012A => Ok(Self::SMSG_INITIAL_SPELLS(<SMSG_INITIAL_SPELLS as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x012A, size: body_size, io, } } else { a } })?)),
            0x012B => Ok(Self::SMSG_LEARNED_SPELL(<SMSG_LEARNED_SPELL as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x012B, size: body_size, io, } } else { a } })?)),
            0x012C => Ok(Self::SMSG_SUPERCEDED_SPELL(<SMSG_SUPERCEDED_SPELL as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x012C, size: body_size, io, } } else { a } })?)),
            0x0131 => Ok(Self::SMSG_SPELL_START(<SMSG_SPELL_START as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x0131, size: body_size, io, } } else { a } })?)),
            0x0132 => Ok(Self::SMSG_SPELL_GO(<SMSG_SPELL_GO as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x0132, size: body_size, io, } } else { a } })?)),
            0x0133 => Ok(Self::SMSG_SPELL_FAILURE(<SMSG_SPELL_FAILURE as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x0133, size: body_size, io, } } else { a } })?)),
            0x0134 => Ok(Self::SMSG_SPELL_COOLDOWN(<SMSG_SPELL_COOLDOWN as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x0134, size: body_size, io, } } else { a } })?)),
            0x0135 => Ok(Self::SMSG_COOLDOWN_EVENT(<SMSG_COOLDOWN_EVENT as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x0135, size: body_size, io, } } else { a } })?)),
            0x0138 => Ok(Self::SMSG_PET_CAST_FAILED(<SMSG_PET_CAST_FAILED as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x0138, size: body_size, io, } } else { a } })?)),
            0x013C => Ok(Self::SMSG_AI_REACTION(<SMSG_AI_REACTION as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x013C, size: body_size, io, } } else { a } })?)),
            0x0143 => Ok(Self::SMSG_ATTACKSTART(<SMSG_ATTACKSTART as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x0143, size: body_size, io, } } else { a } })?)),
            0x0144 => Ok(Self::SMSG_ATTACKSTOP(<SMSG_ATTACKSTOP as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x0144, size: body_size, io, } } else { a } })?)),
            0x0145 => Ok(Self::SMSG_ATTACKSWING_NOTINRANGE(<SMSG_ATTACKSWING_NOTINRANGE as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x0145, size: body_size, io, } } else { a } })?)),
            0x0146 => Ok(Self::SMSG_ATTACKSWING_BADFACING(<SMSG_ATTACKSWING_BADFACING as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x0146, size: body_size, io, } } else { a } })?)),
            0x0148 => Ok(Self::SMSG_ATTACKSWING_DEADTARGET(<SMSG_ATTACKSWING_DEADTARGET as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x0148, size: body_size, io, } } else { a } })?)),
            0x0149 => Ok(Self::SMSG_ATTACKSWING_CANT_ATTACK(<SMSG_ATTACKSWING_CANT_ATTACK as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x0149, size: body_size, io, } } else { a } })?)),
            0x014A => Ok(Self::SMSG_ATTACKERSTATEUPDATE(<SMSG_ATTACKERSTATEUPDATE as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x014A, size: body_size, io, } } else { a } })?)),
            0x014E => Ok(Self::SMSG_CANCEL_COMBAT(<SMSG_CANCEL_COMBAT as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x014E, size: body_size, io, } } else { a } })?)),
            0x0150 => Ok(Self::SMSG_SPELLHEALLOG(<SMSG_SPELLHEALLOG as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x0150, size: body_size, io, } } else { a } })?)),
            0x0151 => Ok(Self::SMSG_SPELLENERGIZELOG(<SMSG_SPELLENERGIZELOG as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x0151, size: body_size, io, } } else { a } })?)),
            0x0155 => Ok(Self::SMSG_BINDPOINTUPDATE(<SMSG_BINDPOINTUPDATE as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x0155, size: body_size, io, } } else { a } })?)),
            0x0158 => Ok(Self::SMSG_PLAYERBOUND(<SMSG_PLAYERBOUND as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x0158, size: body_size, io, } } else { a } })?)),
            0x0159 => Ok(Self::SMSG_CLIENT_CONTROL_UPDATE(<SMSG_CLIENT_CONTROL_UPDATE as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x0159, size: body_size, io, } } else { a } })?)),
            0x015B => Ok(Self::SMSG_RESURRECT_REQUEST(<SMSG_RESURRECT_REQUEST as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x015B, size: body_size, io, } } else { a } })?)),
            0x0160 => Ok(Self::SMSG_LOOT_RESPONSE(<SMSG_LOOT_RESPONSE as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x0160, size: body_size, io, } } else { a } })?)),
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
            0x0173 => Ok(Self::SMSG_PET_TAME_FAILURE(<SMSG_PET_TAME_FAILURE as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x0173, size: body_size, io, } } else { a } })?)),
            0x0178 => Ok(Self::SMSG_PET_NAME_INVALID(<SMSG_PET_NAME_INVALID as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x0178, size: body_size, io, } } else { a } })?)),
            0x0179 => Ok(Self::SMSG_PET_SPELLS(<SMSG_PET_SPELLS as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x0179, size: body_size, io, } } else { a } })?)),
            0x017A => Ok(Self::SMSG_PET_MODE(<SMSG_PET_MODE as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x017A, size: body_size, io, } } else { a } })?)),
            0x017D => Ok(Self::SMSG_GOSSIP_MESSAGE(<SMSG_GOSSIP_MESSAGE as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x017D, size: body_size, io, } } else { a } })?)),
            0x017E => Ok(Self::SMSG_GOSSIP_COMPLETE(<SMSG_GOSSIP_COMPLETE as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x017E, size: body_size, io, } } else { a } })?)),
            0x0180 => Ok(Self::SMSG_NPC_TEXT_UPDATE(<SMSG_NPC_TEXT_UPDATE as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x0180, size: body_size, io, } } else { a } })?)),
            0x0183 => Ok(Self::SMSG_QUESTGIVER_STATUS(<SMSG_QUESTGIVER_STATUS as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x0183, size: body_size, io, } } else { a } })?)),
            0x0185 => Ok(Self::SMSG_QUESTGIVER_QUEST_LIST(<SMSG_QUESTGIVER_QUEST_LIST as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x0185, size: body_size, io, } } else { a } })?)),
            0x0188 => Ok(Self::SMSG_QUESTGIVER_QUEST_DETAILS(<SMSG_QUESTGIVER_QUEST_DETAILS as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x0188, size: body_size, io, } } else { a } })?)),
            0x018B => Ok(Self::SMSG_QUESTGIVER_REQUEST_ITEMS(<SMSG_QUESTGIVER_REQUEST_ITEMS as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x018B, size: body_size, io, } } else { a } })?)),
            0x018D => Ok(Self::SMSG_QUESTGIVER_OFFER_REWARD(<SMSG_QUESTGIVER_OFFER_REWARD as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x018D, size: body_size, io, } } else { a } })?)),
            0x018F => Ok(Self::SMSG_QUESTGIVER_QUEST_INVALID(<SMSG_QUESTGIVER_QUEST_INVALID as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x018F, size: body_size, io, } } else { a } })?)),
            0x0191 => Ok(Self::SMSG_QUESTGIVER_QUEST_COMPLETE(<SMSG_QUESTGIVER_QUEST_COMPLETE as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x0191, size: body_size, io, } } else { a } })?)),
            0x0192 => Ok(Self::SMSG_QUESTGIVER_QUEST_FAILED(<SMSG_QUESTGIVER_QUEST_FAILED as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x0192, size: body_size, io, } } else { a } })?)),
            0x0195 => Ok(Self::SMSG_QUESTLOG_FULL(<SMSG_QUESTLOG_FULL as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x0195, size: body_size, io, } } else { a } })?)),
            0x0196 => Ok(Self::SMSG_QUESTUPDATE_FAILED(<SMSG_QUESTUPDATE_FAILED as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x0196, size: body_size, io, } } else { a } })?)),
            0x0197 => Ok(Self::SMSG_QUESTUPDATE_FAILEDTIMER(<SMSG_QUESTUPDATE_FAILEDTIMER as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x0197, size: body_size, io, } } else { a } })?)),
            0x0198 => Ok(Self::SMSG_QUESTUPDATE_COMPLETE(<SMSG_QUESTUPDATE_COMPLETE as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x0198, size: body_size, io, } } else { a } })?)),
            0x0199 => Ok(Self::SMSG_QUESTUPDATE_ADD_KILL(<SMSG_QUESTUPDATE_ADD_KILL as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x0199, size: body_size, io, } } else { a } })?)),
            0x019A => Ok(Self::SMSG_QUESTUPDATE_ADD_ITEM(<SMSG_QUESTUPDATE_ADD_ITEM as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x019A, size: body_size, io, } } else { a } })?)),
            0x019C => Ok(Self::SMSG_QUEST_CONFIRM_ACCEPT(<SMSG_QUEST_CONFIRM_ACCEPT as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x019C, size: body_size, io, } } else { a } })?)),
            0x019F => Ok(Self::SMSG_LIST_INVENTORY(<SMSG_LIST_INVENTORY as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x019F, size: body_size, io, } } else { a } })?)),
            0x01A1 => Ok(Self::SMSG_SELL_ITEM(<SMSG_SELL_ITEM as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x01A1, size: body_size, io, } } else { a } })?)),
            0x01A4 => Ok(Self::SMSG_BUY_ITEM(<SMSG_BUY_ITEM as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x01A4, size: body_size, io, } } else { a } })?)),
            0x01A5 => Ok(Self::SMSG_BUY_FAILED(<SMSG_BUY_FAILED as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x01A5, size: body_size, io, } } else { a } })?)),
            0x01A9 => Ok(Self::SMSG_SHOWTAXINODES(<SMSG_SHOWTAXINODES as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x01A9, size: body_size, io, } } else { a } })?)),
            0x01AB => Ok(Self::SMSG_TAXINODE_STATUS(<SMSG_TAXINODE_STATUS as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x01AB, size: body_size, io, } } else { a } })?)),
            0x01AE => Ok(Self::SMSG_ACTIVATETAXIREPLY(<SMSG_ACTIVATETAXIREPLY as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x01AE, size: body_size, io, } } else { a } })?)),
            0x01AF => Ok(Self::SMSG_NEW_TAXI_PATH(<SMSG_NEW_TAXI_PATH as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x01AF, size: body_size, io, } } else { a } })?)),
            0x01B1 => Ok(Self::SMSG_TRAINER_LIST(<SMSG_TRAINER_LIST as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x01B1, size: body_size, io, } } else { a } })?)),
            0x01B3 => Ok(Self::SMSG_TRAINER_BUY_SUCCEEDED(<SMSG_TRAINER_BUY_SUCCEEDED as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x01B3, size: body_size, io, } } else { a } })?)),
            0x01B4 => Ok(Self::SMSG_TRAINER_BUY_FAILED(<SMSG_TRAINER_BUY_FAILED as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x01B4, size: body_size, io, } } else { a } })?)),
            0x01B8 => Ok(Self::SMSG_SHOW_BANK(<SMSG_SHOW_BANK as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x01B8, size: body_size, io, } } else { a } })?)),
            0x01BA => Ok(Self::SMSG_BUY_BANK_SLOT_RESULT(<SMSG_BUY_BANK_SLOT_RESULT as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x01BA, size: body_size, io, } } else { a } })?)),
            0x01BC => Ok(Self::SMSG_PETITION_SHOWLIST(<SMSG_PETITION_SHOWLIST as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x01BC, size: body_size, io, } } else { a } })?)),
            0x01BF => Ok(Self::SMSG_PETITION_SHOW_SIGNATURES(<SMSG_PETITION_SHOW_SIGNATURES as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x01BF, size: body_size, io, } } else { a } })?)),
            0x01C1 => Ok(Self::SMSG_PETITION_SIGN_RESULTS(<SMSG_PETITION_SIGN_RESULTS as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x01C1, size: body_size, io, } } else { a } })?)),
            0x01C5 => Ok(Self::SMSG_TURN_IN_PETITION_RESULTS(<SMSG_TURN_IN_PETITION_RESULTS as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x01C5, size: body_size, io, } } else { a } })?)),
            0x01C7 => Ok(Self::SMSG_PETITION_QUERY_RESPONSE(<SMSG_PETITION_QUERY_RESPONSE as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x01C7, size: body_size, io, } } else { a } })?)),
            0x01C8 => Ok(Self::SMSG_FISH_NOT_HOOKED(<SMSG_FISH_NOT_HOOKED as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x01C8, size: body_size, io, } } else { a } })?)),
            0x01C9 => Ok(Self::SMSG_FISH_ESCAPED(<SMSG_FISH_ESCAPED as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x01C9, size: body_size, io, } } else { a } })?)),
            0x01CB => Ok(Self::SMSG_NOTIFICATION(<SMSG_NOTIFICATION as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x01CB, size: body_size, io, } } else { a } })?)),
            0x01CD => Ok(Self::SMSG_PLAYED_TIME(<SMSG_PLAYED_TIME as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x01CD, size: body_size, io, } } else { a } })?)),
            0x01CF => Ok(Self::SMSG_QUERY_TIME_RESPONSE(<SMSG_QUERY_TIME_RESPONSE as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x01CF, size: body_size, io, } } else { a } })?)),
            0x01D0 => Ok(Self::SMSG_LOG_XPGAIN(<SMSG_LOG_XPGAIN as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x01D0, size: body_size, io, } } else { a } })?)),
            0x01D4 => Ok(Self::SMSG_LEVELUP_INFO(<SMSG_LEVELUP_INFO as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x01D4, size: body_size, io, } } else { a } })?)),
            0x01D5 => Ok(Self::MSG_MINIMAP_PING(<MSG_MINIMAP_PING_Server as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x01D5, size: body_size, io, } } else { a } })?)),
            0x01DD => Ok(Self::SMSG_PONG(<SMSG_PONG as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x01DD, size: body_size, io, } } else { a } })?)),
            0x01DF => Ok(Self::SMSG_GAMEOBJECT_PAGETEXT(<SMSG_GAMEOBJECT_PAGETEXT as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x01DF, size: body_size, io, } } else { a } })?)),
            0x01EA => Ok(Self::SMSG_ITEM_TIME_UPDATE(<SMSG_ITEM_TIME_UPDATE as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x01EA, size: body_size, io, } } else { a } })?)),
            0x01EB => Ok(Self::SMSG_ITEM_ENCHANT_TIME_UPDATE(<SMSG_ITEM_ENCHANT_TIME_UPDATE as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x01EB, size: body_size, io, } } else { a } })?)),
            0x01EC => Ok(Self::SMSG_AUTH_CHALLENGE(<SMSG_AUTH_CHALLENGE as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x01EC, size: body_size, io, } } else { a } })?)),
            0x01EE => Ok(Self::SMSG_AUTH_RESPONSE(<SMSG_AUTH_RESPONSE as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x01EE, size: body_size, io, } } else { a } })?)),
            0x01F1 => Ok(Self::MSG_SAVE_GUILD_EMBLEM(<MSG_SAVE_GUILD_EMBLEM_Server as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x01F1, size: body_size, io, } } else { a } })?)),
            0x01F8 => Ok(Self::SMSG_EXPLORATION_EXPERIENCE(<SMSG_EXPLORATION_EXPERIENCE as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x01F8, size: body_size, io, } } else { a } })?)),
            0x01FB => Ok(Self::MSG_RANDOM_ROLL(<MSG_RANDOM_ROLL_Server as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x01FB, size: body_size, io, } } else { a } })?)),
            0x01FC => Ok(Self::SMSG_ENVIRONMENTAL_DAMAGE_LOG(<SMSG_ENVIRONMENTAL_DAMAGE_LOG as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x01FC, size: body_size, io, } } else { a } })?)),
            0x0206 => Ok(Self::SMSG_GMTICKET_CREATE(<SMSG_GMTICKET_CREATE as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x0206, size: body_size, io, } } else { a } })?)),
            0x0208 => Ok(Self::SMSG_GMTICKET_UPDATETEXT(<SMSG_GMTICKET_UPDATETEXT as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x0208, size: body_size, io, } } else { a } })?)),
            0x0209 => Ok(Self::SMSG_ACCOUNT_DATA_TIMES(<SMSG_ACCOUNT_DATA_TIMES as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x0209, size: body_size, io, } } else { a } })?)),
            0x020C => Ok(Self::SMSG_UPDATE_ACCOUNT_DATA(<SMSG_UPDATE_ACCOUNT_DATA as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x020C, size: body_size, io, } } else { a } })?)),
            0x0212 => Ok(Self::SMSG_GMTICKET_GETTICKET(<SMSG_GMTICKET_GETTICKET as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x0212, size: body_size, io, } } else { a } })?)),
            0x0215 => Ok(Self::SMSG_GAMEOBJECT_DESPAWN_ANIM(<SMSG_GAMEOBJECT_DESPAWN_ANIM as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x0215, size: body_size, io, } } else { a } })?)),
            0x0216 => Ok(Self::MSG_CORPSE_QUERY(<MSG_CORPSE_QUERY_Server as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x0216, size: body_size, io, } } else { a } })?)),
            0x0218 => Ok(Self::SMSG_GMTICKET_DELETETICKET(<SMSG_GMTICKET_DELETETICKET as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x0218, size: body_size, io, } } else { a } })?)),
            0x0219 => Ok(Self::SMSG_CHAT_WRONG_FACTION(<SMSG_CHAT_WRONG_FACTION as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x0219, size: body_size, io, } } else { a } })?)),
            0x021B => Ok(Self::SMSG_GMTICKET_SYSTEMSTATUS(<SMSG_GMTICKET_SYSTEMSTATUS as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x021B, size: body_size, io, } } else { a } })?)),
            0x021E => Ok(Self::SMSG_QUEST_FORCE_REMOVE(<SMSG_QUEST_FORCE_REMOVE as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x021E, size: body_size, io, } } else { a } })?)),
            0x0224 => Ok(Self::SMSG_GOSSIP_POI(<SMSG_GOSSIP_POI as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x0224, size: body_size, io, } } else { a } })?)),
            0x0236 => Ok(Self::SMSG_LOGIN_VERIFY_WORLD(<SMSG_LOGIN_VERIFY_WORLD as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x0236, size: body_size, io, } } else { a } })?)),
            0x023D => Ok(Self::SMSG_BATTLEFIELD_LIST(<SMSG_BATTLEFIELD_LIST as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x023D, size: body_size, io, } } else { a } })?)),
            0x0244 => Ok(Self::SMSG_ITEM_TEXT_QUERY_RESPONSE(<SMSG_ITEM_TEXT_QUERY_RESPONSE as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x0244, size: body_size, io, } } else { a } })?)),
            0x0254 => Ok(Self::SMSG_ZONE_UNDER_ATTACK(<SMSG_ZONE_UNDER_ATTACK as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x0254, size: body_size, io, } } else { a } })?)),
            0x0255 => Ok(Self::MSG_AUCTION_HELLO(<MSG_AUCTION_HELLO_Server as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x0255, size: body_size, io, } } else { a } })?)),
            0x025C => Ok(Self::SMSG_AUCTION_LIST_RESULT(<SMSG_AUCTION_LIST_RESULT as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x025C, size: body_size, io, } } else { a } })?)),
            0x025D => Ok(Self::SMSG_AUCTION_OWNER_LIST_RESULT(<SMSG_AUCTION_OWNER_LIST_RESULT as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x025D, size: body_size, io, } } else { a } })?)),
            0x025E => Ok(Self::SMSG_AUCTION_BIDDER_NOTIFICATION(<SMSG_AUCTION_BIDDER_NOTIFICATION as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x025E, size: body_size, io, } } else { a } })?)),
            0x025F => Ok(Self::SMSG_AUCTION_OWNER_NOTIFICATION(<SMSG_AUCTION_OWNER_NOTIFICATION as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x025F, size: body_size, io, } } else { a } })?)),
            0x0260 => Ok(Self::SMSG_PROCRESIST(<SMSG_PROCRESIST as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x0260, size: body_size, io, } } else { a } })?)),
            0x0265 => Ok(Self::SMSG_AUCTION_BIDDER_LIST_RESULT(<SMSG_AUCTION_BIDDER_LIST_RESULT as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x0265, size: body_size, io, } } else { a } })?)),
            0x026F => Ok(Self::MSG_LIST_STABLED_PETS(<MSG_LIST_STABLED_PETS_Server as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x026F, size: body_size, io, } } else { a } })?)),
            0x0273 => Ok(Self::SMSG_STABLE_RESULT(<SMSG_STABLE_RESULT as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x0273, size: body_size, io, } } else { a } })?)),
            0x0278 => Ok(Self::SMSG_PLAY_OBJECT_SOUND(<SMSG_PLAY_OBJECT_SOUND as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x0278, size: body_size, io, } } else { a } })?)),
            0x0284 => Ok(Self::MSG_QUERY_NEXT_MAIL_TIME(<MSG_QUERY_NEXT_MAIL_TIME_Server as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x0284, size: body_size, io, } } else { a } })?)),
            0x0285 => Ok(Self::SMSG_RECEIVED_MAIL(<SMSG_RECEIVED_MAIL as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x0285, size: body_size, io, } } else { a } })?)),
            0x028C => Ok(Self::SMSG_PVP_CREDIT(<SMSG_PVP_CREDIT as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x028C, size: body_size, io, } } else { a } })?)),
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
            0x02AF => Ok(Self::SMSG_PET_BROKEN(<SMSG_PET_BROKEN as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x02AF, size: body_size, io, } } else { a } })?)),
            0x02B7 => Ok(Self::SMSG_DUEL_COUNTDOWN(<SMSG_DUEL_COUNTDOWN as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x02B7, size: body_size, io, } } else { a } })?)),
            0x02B8 => Ok(Self::SMSG_AREA_TRIGGER_MESSAGE(<SMSG_AREA_TRIGGER_MESSAGE as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x02B8, size: body_size, io, } } else { a } })?)),
            0x02BD => Ok(Self::SMSG_DURABILITY_DAMAGE_DEATH(<SMSG_DURABILITY_DAMAGE_DEATH as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x02BD, size: body_size, io, } } else { a } })?)),
            0x02C2 => Ok(Self::SMSG_INIT_WORLD_STATES(<SMSG_INIT_WORLD_STATES as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x02C2, size: body_size, io, } } else { a } })?)),
            0x02C3 => Ok(Self::SMSG_UPDATE_WORLD_STATE(<SMSG_UPDATE_WORLD_STATE as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x02C3, size: body_size, io, } } else { a } })?)),
            0x02C5 => Ok(Self::SMSG_ITEM_NAME_QUERY_RESPONSE(<SMSG_ITEM_NAME_QUERY_RESPONSE as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x02C5, size: body_size, io, } } else { a } })?)),
            0x02C6 => Ok(Self::SMSG_PET_ACTION_FEEDBACK(<SMSG_PET_ACTION_FEEDBACK as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x02C6, size: body_size, io, } } else { a } })?)),
            0x02C8 => Ok(Self::SMSG_CHAR_RENAME(<SMSG_CHAR_RENAME as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x02C8, size: body_size, io, } } else { a } })?)),
            0x02CC => Ok(Self::SMSG_RAID_INSTANCE_INFO(<SMSG_RAID_INSTANCE_INFO as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x02CC, size: body_size, io, } } else { a } })?)),
            0x02D6 => Ok(Self::MSG_INSPECT_HONOR_STATS(<MSG_INSPECT_HONOR_STATS_Server as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x02D6, size: body_size, io, } } else { a } })?)),
            0x02DA => Ok(Self::SMSG_FORCE_WALK_SPEED_CHANGE(<SMSG_FORCE_WALK_SPEED_CHANGE as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x02DA, size: body_size, io, } } else { a } })?)),
            0x02DC => Ok(Self::SMSG_FORCE_SWIM_BACK_SPEED_CHANGE(<SMSG_FORCE_SWIM_BACK_SPEED_CHANGE as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x02DC, size: body_size, io, } } else { a } })?)),
            0x02DE => Ok(Self::SMSG_FORCE_TURN_RATE_CHANGE(<SMSG_FORCE_TURN_RATE_CHANGE as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x02DE, size: body_size, io, } } else { a } })?)),
            0x02E9 => Ok(Self::MSG_BATTLEGROUND_PLAYER_POSITIONS(<MSG_BATTLEGROUND_PLAYER_POSITIONS_Server as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x02E9, size: body_size, io, } } else { a } })?)),
            0x02EB => Ok(Self::SMSG_BINDER_CONFIRM(<SMSG_BINDER_CONFIRM as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x02EB, size: body_size, io, } } else { a } })?)),
            0x02EC => Ok(Self::SMSG_BATTLEGROUND_PLAYER_JOINED(<SMSG_BATTLEGROUND_PLAYER_JOINED as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x02EC, size: body_size, io, } } else { a } })?)),
            0x02ED => Ok(Self::SMSG_BATTLEGROUND_PLAYER_LEFT(<SMSG_BATTLEGROUND_PLAYER_LEFT as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x02ED, size: body_size, io, } } else { a } })?)),
            0x02EF => Ok(Self::SMSG_ADDON_INFO(<SMSG_ADDON_INFO as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x02EF, size: body_size, io, } } else { a } })?)),
            0x02F1 => Ok(Self::SMSG_PET_UNLEARN_CONFIRM(<SMSG_PET_UNLEARN_CONFIRM as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x02F1, size: body_size, io, } } else { a } })?)),
            0x02FD => Ok(Self::SMSG_CHAT_RESTRICTED(<SMSG_CHAT_RESTRICTED as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x02FD, size: body_size, io, } } else { a } })?)),
            0x02FE => Ok(Self::SMSG_SPLINE_SET_RUN_SPEED(<SMSG_SPLINE_SET_RUN_SPEED as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x02FE, size: body_size, io, } } else { a } })?)),
            0x02FF => Ok(Self::SMSG_SPLINE_SET_RUN_BACK_SPEED(<SMSG_SPLINE_SET_RUN_BACK_SPEED as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x02FF, size: body_size, io, } } else { a } })?)),
            0x0300 => Ok(Self::SMSG_SPLINE_SET_SWIM_SPEED(<SMSG_SPLINE_SET_SWIM_SPEED as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x0300, size: body_size, io, } } else { a } })?)),
            0x0301 => Ok(Self::SMSG_SPLINE_SET_WALK_SPEED(<SMSG_SPLINE_SET_WALK_SPEED as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x0301, size: body_size, io, } } else { a } })?)),
            0x0302 => Ok(Self::SMSG_SPLINE_SET_SWIM_BACK_SPEED(<SMSG_SPLINE_SET_SWIM_BACK_SPEED as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x0302, size: body_size, io, } } else { a } })?)),
            0x0303 => Ok(Self::SMSG_SPLINE_SET_TURN_RATE(<SMSG_SPLINE_SET_TURN_RATE as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x0303, size: body_size, io, } } else { a } })?)),
            0x0304 => Ok(Self::SMSG_SPLINE_MOVE_UNROOT(<SMSG_SPLINE_MOVE_UNROOT as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x0304, size: body_size, io, } } else { a } })?)),
            0x0305 => Ok(Self::SMSG_SPLINE_MOVE_FEATHER_FALL(<SMSG_SPLINE_MOVE_FEATHER_FALL as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x0305, size: body_size, io, } } else { a } })?)),
            0x0306 => Ok(Self::SMSG_SPLINE_MOVE_NORMAL_FALL(<SMSG_SPLINE_MOVE_NORMAL_FALL as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x0306, size: body_size, io, } } else { a } })?)),
            0x0307 => Ok(Self::SMSG_SPLINE_MOVE_SET_HOVER(<SMSG_SPLINE_MOVE_SET_HOVER as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x0307, size: body_size, io, } } else { a } })?)),
            0x0308 => Ok(Self::SMSG_SPLINE_MOVE_UNSET_HOVER(<SMSG_SPLINE_MOVE_UNSET_HOVER as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x0308, size: body_size, io, } } else { a } })?)),
            0x0309 => Ok(Self::SMSG_SPLINE_MOVE_WATER_WALK(<SMSG_SPLINE_MOVE_WATER_WALK as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x0309, size: body_size, io, } } else { a } })?)),
            0x030A => Ok(Self::SMSG_SPLINE_MOVE_LAND_WALK(<SMSG_SPLINE_MOVE_LAND_WALK as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x030A, size: body_size, io, } } else { a } })?)),
            0x030B => Ok(Self::SMSG_SPLINE_MOVE_START_SWIM(<SMSG_SPLINE_MOVE_START_SWIM as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x030B, size: body_size, io, } } else { a } })?)),
            0x030C => Ok(Self::SMSG_SPLINE_MOVE_STOP_SWIM(<SMSG_SPLINE_MOVE_STOP_SWIM as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x030C, size: body_size, io, } } else { a } })?)),
            0x030D => Ok(Self::SMSG_SPLINE_MOVE_SET_RUN_MODE(<SMSG_SPLINE_MOVE_SET_RUN_MODE as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x030D, size: body_size, io, } } else { a } })?)),
            0x030E => Ok(Self::SMSG_SPLINE_MOVE_SET_WALK_MODE(<SMSG_SPLINE_MOVE_SET_WALK_MODE as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x030E, size: body_size, io, } } else { a } })?)),
            0x031A => Ok(Self::SMSG_SPLINE_MOVE_ROOT(<SMSG_SPLINE_MOVE_ROOT as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x031A, size: body_size, io, } } else { a } })?)),
            0x0322 => Ok(Self::MSG_RAID_READY_CHECK(<MSG_RAID_READY_CHECK_Server as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x0322, size: body_size, io, } } else { a } })?)),
            0x0324 => Ok(Self::SMSG_PET_ACTION_SOUND(<SMSG_PET_ACTION_SOUND as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x0324, size: body_size, io, } } else { a } })?)),
            0x0325 => Ok(Self::SMSG_PET_DISMISS_SOUND(<SMSG_PET_DISMISS_SOUND as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x0325, size: body_size, io, } } else { a } })?)),
            0x0328 => Ok(Self::SMSG_GM_TICKET_STATUS_UPDATE(<SMSG_GM_TICKET_STATUS_UPDATE as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x0328, size: body_size, io, } } else { a } })?)),
            0x033A => Ok(Self::SMSG_DEFENSE_MESSAGE(<SMSG_DEFENSE_MESSAGE as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x033A, size: body_size, io, } } else { a } })?)),
            0x038B => Ok(Self::SMSG_REALM_SPLIT(<SMSG_REALM_SPLIT as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x038B, size: body_size, io, } } else { a } })?)),
            0x0390 => Ok(Self::SMSG_TIME_SYNC_REQ(<SMSG_TIME_SYNC_REQ as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x0390, size: body_size, io, } } else { a } })?)),
            0x03B3 => Ok(Self::SMSG_GM_MESSAGECHAT(<SMSG_GM_MESSAGECHAT as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x03B3, size: body_size, io, } } else { a } })?)),
            0x03C9 => Ok(Self::SMSG_FEATURE_SYSTEM_STATUS(<SMSG_FEATURE_SYSTEM_STATUS as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x03C9, size: body_size, io, } } else { a } })?)),
            0x0448 => Ok(Self::SMSG_CALENDAR_SEND_NUM_PENDING(<SMSG_CALENDAR_SEND_NUM_PENDING as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x0448, size: body_size, io, } } else { a } })?)),
            0x0463 => Ok(Self::SMSG_UPDATE_ACCOUNT_DATA_COMPLETE(<SMSG_UPDATE_ACCOUNT_DATA_COMPLETE as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x0463, size: body_size, io, } } else { a } })?)),
            0x04AB => Ok(Self::SMSG_CLIENTCACHE_VERSION(<SMSG_CLIENTCACHE_VERSION as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x04AB, size: body_size, io, } } else { a } })?)),
            0x04F7 => Ok(Self::SMSG_WORLD_STATE_UI_TIMER_UPDATE(<SMSG_WORLD_STATE_UI_TIMER_UPDATE as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x04F7, size: body_size, io, } } else { a } })?)),
            _ => Err(crate::errors::ExpectedOpcodeError::Opcode{ opcode: opcode.into(), name: opcode_to_name(opcode.into()), size: body_size }),
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
            Self::MSG_QUEST_PUSH_RESULT(c) => c.write_encrypted_server(w, e),
            Self::MSG_PETITION_RENAME(c) => c.write_encrypted_server(w, e),
            Self::MSG_SET_DUNGEON_DIFFICULTY(c) => c.write_encrypted_server(w, e),
            Self::MSG_MOVE_START_ASCEND(c) => c.write_encrypted_server(w, e),
            Self::MSG_MOVE_STOP_ASCEND(c) => c.write_encrypted_server(w, e),
            Self::MSG_MOVE_START_DESCEND(c) => c.write_encrypted_server(w, e),
            Self::MSG_GUILD_BANK_MONEY_WITHDRAWN(c) => c.write_encrypted_server(w, e),
            Self::CMSG_CALENDAR_GET_NUM_PENDING(c) => c.write_encrypted_server(w, e),
            Self::SMSG_CHAR_CREATE(c) => c.write_encrypted_server(w, e),
            Self::SMSG_CHAR_ENUM(c) => c.write_encrypted_server(w, e),
            Self::SMSG_CHAR_DELETE(c) => c.write_encrypted_server(w, e),
            Self::SMSG_NEW_WORLD(c) => c.write_encrypted_server(w, e),
            Self::SMSG_TRANSFER_PENDING(c) => c.write_encrypted_server(w, e),
            Self::SMSG_TRANSFER_ABORTED(c) => c.write_encrypted_server(w, e),
            Self::SMSG_CHARACTER_LOGIN_FAILED(c) => c.write_encrypted_server(w, e),
            Self::SMSG_LOGIN_SETTIMESPEED(c) => c.write_encrypted_server(w, e),
            Self::SMSG_LOGOUT_RESPONSE(c) => c.write_encrypted_server(w, e),
            Self::SMSG_LOGOUT_COMPLETE(c) => c.write_encrypted_server(w, e),
            Self::SMSG_LOGOUT_CANCEL_ACK(c) => c.write_encrypted_server(w, e),
            Self::SMSG_NAME_QUERY_RESPONSE(c) => c.write_encrypted_server(w, e),
            Self::SMSG_PET_NAME_QUERY_RESPONSE(c) => c.write_encrypted_server(w, e),
            Self::SMSG_GUILD_QUERY_RESPONSE(c) => c.write_encrypted_server(w, e),
            Self::SMSG_ITEM_QUERY_SINGLE_RESPONSE(c) => c.write_encrypted_server(w, e),
            Self::SMSG_PAGE_TEXT_QUERY_RESPONSE(c) => c.write_encrypted_server(w, e),
            Self::SMSG_QUEST_QUERY_RESPONSE(c) => c.write_encrypted_server(w, e),
            Self::SMSG_GAMEOBJECT_QUERY_RESPONSE(c) => c.write_encrypted_server(w, e),
            Self::SMSG_CREATURE_QUERY_RESPONSE(c) => c.write_encrypted_server(w, e),
            Self::SMSG_WHO(c) => c.write_encrypted_server(w, e),
            Self::SMSG_WHOIS(c) => c.write_encrypted_server(w, e),
            Self::SMSG_CONTACT_LIST(c) => c.write_encrypted_server(w, e),
            Self::SMSG_FRIEND_STATUS(c) => c.write_encrypted_server(w, e),
            Self::SMSG_GROUP_INVITE(c) => c.write_encrypted_server(w, e),
            Self::SMSG_GROUP_DECLINE(c) => c.write_encrypted_server(w, e),
            Self::SMSG_GROUP_UNINVITE(c) => c.write_encrypted_server(w, e),
            Self::SMSG_GROUP_SET_LEADER(c) => c.write_encrypted_server(w, e),
            Self::SMSG_GROUP_DESTROYED(c) => c.write_encrypted_server(w, e),
            Self::SMSG_GROUP_LIST(c) => c.write_encrypted_server(w, e),
            Self::SMSG_PARTY_COMMAND_RESULT(c) => c.write_encrypted_server(w, e),
            Self::SMSG_GUILD_INVITE(c) => c.write_encrypted_server(w, e),
            Self::SMSG_GUILD_INFO(c) => c.write_encrypted_server(w, e),
            Self::SMSG_GUILD_ROSTER(c) => c.write_encrypted_server(w, e),
            Self::SMSG_GUILD_EVENT(c) => c.write_encrypted_server(w, e),
            Self::SMSG_GUILD_COMMAND_RESULT(c) => c.write_encrypted_server(w, e),
            Self::SMSG_MESSAGECHAT(c) => c.write_encrypted_server(w, e),
            Self::SMSG_CHANNEL_NOTIFY(c) => c.write_encrypted_server(w, e),
            Self::SMSG_CHANNEL_LIST(c) => c.write_encrypted_server(w, e),
            Self::SMSG_UPDATE_OBJECT(c) => c.write_encrypted_server(w, e),
            Self::SMSG_DESTROY_OBJECT(c) => c.write_encrypted_server(w, e),
            Self::SMSG_READ_ITEM_OK(c) => c.write_encrypted_server(w, e),
            Self::SMSG_READ_ITEM_FAILED(c) => c.write_encrypted_server(w, e),
            Self::SMSG_ITEM_COOLDOWN(c) => c.write_encrypted_server(w, e),
            Self::SMSG_GAMEOBJECT_CUSTOM_ANIM(c) => c.write_encrypted_server(w, e),
            Self::MSG_MOVE_TELEPORT_ACK(c) => c.write_encrypted_server(w, e),
            Self::SMSG_MOVE_WATER_WALK(c) => c.write_encrypted_server(w, e),
            Self::SMSG_MOVE_LAND_WALK(c) => c.write_encrypted_server(w, e),
            Self::SMSG_FORCE_RUN_SPEED_CHANGE(c) => c.write_encrypted_server(w, e),
            Self::SMSG_FORCE_RUN_BACK_SPEED_CHANGE(c) => c.write_encrypted_server(w, e),
            Self::SMSG_FORCE_SWIM_SPEED_CHANGE(c) => c.write_encrypted_server(w, e),
            Self::SMSG_FORCE_MOVE_ROOT(c) => c.write_encrypted_server(w, e),
            Self::SMSG_FORCE_MOVE_UNROOT(c) => c.write_encrypted_server(w, e),
            Self::SMSG_MOVE_KNOCK_BACK(c) => c.write_encrypted_server(w, e),
            Self::SMSG_MOVE_FEATHER_FALL(c) => c.write_encrypted_server(w, e),
            Self::SMSG_MOVE_NORMAL_FALL(c) => c.write_encrypted_server(w, e),
            Self::SMSG_MOVE_SET_HOVER(c) => c.write_encrypted_server(w, e),
            Self::SMSG_MOVE_UNSET_HOVER(c) => c.write_encrypted_server(w, e),
            Self::SMSG_TRIGGER_CINEMATIC(c) => c.write_encrypted_server(w, e),
            Self::SMSG_TUTORIAL_FLAGS(c) => c.write_encrypted_server(w, e),
            Self::SMSG_EMOTE(c) => c.write_encrypted_server(w, e),
            Self::SMSG_TEXT_EMOTE(c) => c.write_encrypted_server(w, e),
            Self::SMSG_INVENTORY_CHANGE_FAILURE(c) => c.write_encrypted_server(w, e),
            Self::SMSG_TRADE_STATUS(c) => c.write_encrypted_server(w, e),
            Self::SMSG_TRADE_STATUS_EXTENDED(c) => c.write_encrypted_server(w, e),
            Self::SMSG_INITIALIZE_FACTIONS(c) => c.write_encrypted_server(w, e),
            Self::SMSG_SET_FACTION_VISIBLE(c) => c.write_encrypted_server(w, e),
            Self::SMSG_SET_FACTION_STANDING(c) => c.write_encrypted_server(w, e),
            Self::SMSG_SET_PROFICIENCY(c) => c.write_encrypted_server(w, e),
            Self::SMSG_ACTION_BUTTONS(c) => c.write_encrypted_server(w, e),
            Self::SMSG_INITIAL_SPELLS(c) => c.write_encrypted_server(w, e),
            Self::SMSG_LEARNED_SPELL(c) => c.write_encrypted_server(w, e),
            Self::SMSG_SUPERCEDED_SPELL(c) => c.write_encrypted_server(w, e),
            Self::SMSG_SPELL_START(c) => c.write_encrypted_server(w, e),
            Self::SMSG_SPELL_GO(c) => c.write_encrypted_server(w, e),
            Self::SMSG_SPELL_FAILURE(c) => c.write_encrypted_server(w, e),
            Self::SMSG_SPELL_COOLDOWN(c) => c.write_encrypted_server(w, e),
            Self::SMSG_COOLDOWN_EVENT(c) => c.write_encrypted_server(w, e),
            Self::SMSG_PET_CAST_FAILED(c) => c.write_encrypted_server(w, e),
            Self::SMSG_AI_REACTION(c) => c.write_encrypted_server(w, e),
            Self::SMSG_ATTACKSTART(c) => c.write_encrypted_server(w, e),
            Self::SMSG_ATTACKSTOP(c) => c.write_encrypted_server(w, e),
            Self::SMSG_ATTACKSWING_NOTINRANGE(c) => c.write_encrypted_server(w, e),
            Self::SMSG_ATTACKSWING_BADFACING(c) => c.write_encrypted_server(w, e),
            Self::SMSG_ATTACKSWING_DEADTARGET(c) => c.write_encrypted_server(w, e),
            Self::SMSG_ATTACKSWING_CANT_ATTACK(c) => c.write_encrypted_server(w, e),
            Self::SMSG_ATTACKERSTATEUPDATE(c) => c.write_encrypted_server(w, e),
            Self::SMSG_CANCEL_COMBAT(c) => c.write_encrypted_server(w, e),
            Self::SMSG_SPELLHEALLOG(c) => c.write_encrypted_server(w, e),
            Self::SMSG_SPELLENERGIZELOG(c) => c.write_encrypted_server(w, e),
            Self::SMSG_BINDPOINTUPDATE(c) => c.write_encrypted_server(w, e),
            Self::SMSG_PLAYERBOUND(c) => c.write_encrypted_server(w, e),
            Self::SMSG_CLIENT_CONTROL_UPDATE(c) => c.write_encrypted_server(w, e),
            Self::SMSG_RESURRECT_REQUEST(c) => c.write_encrypted_server(w, e),
            Self::SMSG_LOOT_RESPONSE(c) => c.write_encrypted_server(w, e),
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
            Self::SMSG_PET_TAME_FAILURE(c) => c.write_encrypted_server(w, e),
            Self::SMSG_PET_NAME_INVALID(c) => c.write_encrypted_server(w, e),
            Self::SMSG_PET_SPELLS(c) => c.write_encrypted_server(w, e),
            Self::SMSG_PET_MODE(c) => c.write_encrypted_server(w, e),
            Self::SMSG_GOSSIP_MESSAGE(c) => c.write_encrypted_server(w, e),
            Self::SMSG_GOSSIP_COMPLETE(c) => c.write_encrypted_server(w, e),
            Self::SMSG_NPC_TEXT_UPDATE(c) => c.write_encrypted_server(w, e),
            Self::SMSG_QUESTGIVER_STATUS(c) => c.write_encrypted_server(w, e),
            Self::SMSG_QUESTGIVER_QUEST_LIST(c) => c.write_encrypted_server(w, e),
            Self::SMSG_QUESTGIVER_QUEST_DETAILS(c) => c.write_encrypted_server(w, e),
            Self::SMSG_QUESTGIVER_REQUEST_ITEMS(c) => c.write_encrypted_server(w, e),
            Self::SMSG_QUESTGIVER_OFFER_REWARD(c) => c.write_encrypted_server(w, e),
            Self::SMSG_QUESTGIVER_QUEST_INVALID(c) => c.write_encrypted_server(w, e),
            Self::SMSG_QUESTGIVER_QUEST_COMPLETE(c) => c.write_encrypted_server(w, e),
            Self::SMSG_QUESTGIVER_QUEST_FAILED(c) => c.write_encrypted_server(w, e),
            Self::SMSG_QUESTLOG_FULL(c) => c.write_encrypted_server(w, e),
            Self::SMSG_QUESTUPDATE_FAILED(c) => c.write_encrypted_server(w, e),
            Self::SMSG_QUESTUPDATE_FAILEDTIMER(c) => c.write_encrypted_server(w, e),
            Self::SMSG_QUESTUPDATE_COMPLETE(c) => c.write_encrypted_server(w, e),
            Self::SMSG_QUESTUPDATE_ADD_KILL(c) => c.write_encrypted_server(w, e),
            Self::SMSG_QUESTUPDATE_ADD_ITEM(c) => c.write_encrypted_server(w, e),
            Self::SMSG_QUEST_CONFIRM_ACCEPT(c) => c.write_encrypted_server(w, e),
            Self::SMSG_LIST_INVENTORY(c) => c.write_encrypted_server(w, e),
            Self::SMSG_SELL_ITEM(c) => c.write_encrypted_server(w, e),
            Self::SMSG_BUY_ITEM(c) => c.write_encrypted_server(w, e),
            Self::SMSG_BUY_FAILED(c) => c.write_encrypted_server(w, e),
            Self::SMSG_SHOWTAXINODES(c) => c.write_encrypted_server(w, e),
            Self::SMSG_TAXINODE_STATUS(c) => c.write_encrypted_server(w, e),
            Self::SMSG_ACTIVATETAXIREPLY(c) => c.write_encrypted_server(w, e),
            Self::SMSG_NEW_TAXI_PATH(c) => c.write_encrypted_server(w, e),
            Self::SMSG_TRAINER_LIST(c) => c.write_encrypted_server(w, e),
            Self::SMSG_TRAINER_BUY_SUCCEEDED(c) => c.write_encrypted_server(w, e),
            Self::SMSG_TRAINER_BUY_FAILED(c) => c.write_encrypted_server(w, e),
            Self::SMSG_SHOW_BANK(c) => c.write_encrypted_server(w, e),
            Self::SMSG_BUY_BANK_SLOT_RESULT(c) => c.write_encrypted_server(w, e),
            Self::SMSG_PETITION_SHOWLIST(c) => c.write_encrypted_server(w, e),
            Self::SMSG_PETITION_SHOW_SIGNATURES(c) => c.write_encrypted_server(w, e),
            Self::SMSG_PETITION_SIGN_RESULTS(c) => c.write_encrypted_server(w, e),
            Self::SMSG_TURN_IN_PETITION_RESULTS(c) => c.write_encrypted_server(w, e),
            Self::SMSG_PETITION_QUERY_RESPONSE(c) => c.write_encrypted_server(w, e),
            Self::SMSG_FISH_NOT_HOOKED(c) => c.write_encrypted_server(w, e),
            Self::SMSG_FISH_ESCAPED(c) => c.write_encrypted_server(w, e),
            Self::SMSG_NOTIFICATION(c) => c.write_encrypted_server(w, e),
            Self::SMSG_PLAYED_TIME(c) => c.write_encrypted_server(w, e),
            Self::SMSG_QUERY_TIME_RESPONSE(c) => c.write_encrypted_server(w, e),
            Self::SMSG_LOG_XPGAIN(c) => c.write_encrypted_server(w, e),
            Self::SMSG_LEVELUP_INFO(c) => c.write_encrypted_server(w, e),
            Self::MSG_MINIMAP_PING(c) => c.write_encrypted_server(w, e),
            Self::SMSG_PONG(c) => c.write_encrypted_server(w, e),
            Self::SMSG_GAMEOBJECT_PAGETEXT(c) => c.write_encrypted_server(w, e),
            Self::SMSG_ITEM_TIME_UPDATE(c) => c.write_encrypted_server(w, e),
            Self::SMSG_ITEM_ENCHANT_TIME_UPDATE(c) => c.write_encrypted_server(w, e),
            Self::SMSG_AUTH_CHALLENGE(c) => c.write_encrypted_server(w, e),
            Self::SMSG_AUTH_RESPONSE(c) => c.write_encrypted_server(w, e),
            Self::MSG_SAVE_GUILD_EMBLEM(c) => c.write_encrypted_server(w, e),
            Self::SMSG_EXPLORATION_EXPERIENCE(c) => c.write_encrypted_server(w, e),
            Self::MSG_RANDOM_ROLL(c) => c.write_encrypted_server(w, e),
            Self::SMSG_ENVIRONMENTAL_DAMAGE_LOG(c) => c.write_encrypted_server(w, e),
            Self::SMSG_GMTICKET_CREATE(c) => c.write_encrypted_server(w, e),
            Self::SMSG_GMTICKET_UPDATETEXT(c) => c.write_encrypted_server(w, e),
            Self::SMSG_ACCOUNT_DATA_TIMES(c) => c.write_encrypted_server(w, e),
            Self::SMSG_UPDATE_ACCOUNT_DATA(c) => c.write_encrypted_server(w, e),
            Self::SMSG_GMTICKET_GETTICKET(c) => c.write_encrypted_server(w, e),
            Self::SMSG_GAMEOBJECT_DESPAWN_ANIM(c) => c.write_encrypted_server(w, e),
            Self::MSG_CORPSE_QUERY(c) => c.write_encrypted_server(w, e),
            Self::SMSG_GMTICKET_DELETETICKET(c) => c.write_encrypted_server(w, e),
            Self::SMSG_CHAT_WRONG_FACTION(c) => c.write_encrypted_server(w, e),
            Self::SMSG_GMTICKET_SYSTEMSTATUS(c) => c.write_encrypted_server(w, e),
            Self::SMSG_QUEST_FORCE_REMOVE(c) => c.write_encrypted_server(w, e),
            Self::SMSG_GOSSIP_POI(c) => c.write_encrypted_server(w, e),
            Self::SMSG_LOGIN_VERIFY_WORLD(c) => c.write_encrypted_server(w, e),
            Self::SMSG_BATTLEFIELD_LIST(c) => c.write_encrypted_server(w, e),
            Self::SMSG_ITEM_TEXT_QUERY_RESPONSE(c) => c.write_encrypted_server(w, e),
            Self::SMSG_ZONE_UNDER_ATTACK(c) => c.write_encrypted_server(w, e),
            Self::MSG_AUCTION_HELLO(c) => c.write_encrypted_server(w, e),
            Self::SMSG_AUCTION_LIST_RESULT(c) => c.write_encrypted_server(w, e),
            Self::SMSG_AUCTION_OWNER_LIST_RESULT(c) => c.write_encrypted_server(w, e),
            Self::SMSG_AUCTION_BIDDER_NOTIFICATION(c) => c.write_encrypted_server(w, e),
            Self::SMSG_AUCTION_OWNER_NOTIFICATION(c) => c.write_encrypted_server(w, e),
            Self::SMSG_PROCRESIST(c) => c.write_encrypted_server(w, e),
            Self::SMSG_AUCTION_BIDDER_LIST_RESULT(c) => c.write_encrypted_server(w, e),
            Self::MSG_LIST_STABLED_PETS(c) => c.write_encrypted_server(w, e),
            Self::SMSG_STABLE_RESULT(c) => c.write_encrypted_server(w, e),
            Self::SMSG_PLAY_OBJECT_SOUND(c) => c.write_encrypted_server(w, e),
            Self::MSG_QUERY_NEXT_MAIL_TIME(c) => c.write_encrypted_server(w, e),
            Self::SMSG_RECEIVED_MAIL(c) => c.write_encrypted_server(w, e),
            Self::SMSG_PVP_CREDIT(c) => c.write_encrypted_server(w, e),
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
            Self::SMSG_PET_BROKEN(c) => c.write_encrypted_server(w, e),
            Self::SMSG_DUEL_COUNTDOWN(c) => c.write_encrypted_server(w, e),
            Self::SMSG_AREA_TRIGGER_MESSAGE(c) => c.write_encrypted_server(w, e),
            Self::SMSG_DURABILITY_DAMAGE_DEATH(c) => c.write_encrypted_server(w, e),
            Self::SMSG_INIT_WORLD_STATES(c) => c.write_encrypted_server(w, e),
            Self::SMSG_UPDATE_WORLD_STATE(c) => c.write_encrypted_server(w, e),
            Self::SMSG_ITEM_NAME_QUERY_RESPONSE(c) => c.write_encrypted_server(w, e),
            Self::SMSG_PET_ACTION_FEEDBACK(c) => c.write_encrypted_server(w, e),
            Self::SMSG_CHAR_RENAME(c) => c.write_encrypted_server(w, e),
            Self::SMSG_RAID_INSTANCE_INFO(c) => c.write_encrypted_server(w, e),
            Self::MSG_INSPECT_HONOR_STATS(c) => c.write_encrypted_server(w, e),
            Self::SMSG_FORCE_WALK_SPEED_CHANGE(c) => c.write_encrypted_server(w, e),
            Self::SMSG_FORCE_SWIM_BACK_SPEED_CHANGE(c) => c.write_encrypted_server(w, e),
            Self::SMSG_FORCE_TURN_RATE_CHANGE(c) => c.write_encrypted_server(w, e),
            Self::MSG_BATTLEGROUND_PLAYER_POSITIONS(c) => c.write_encrypted_server(w, e),
            Self::SMSG_BINDER_CONFIRM(c) => c.write_encrypted_server(w, e),
            Self::SMSG_BATTLEGROUND_PLAYER_JOINED(c) => c.write_encrypted_server(w, e),
            Self::SMSG_BATTLEGROUND_PLAYER_LEFT(c) => c.write_encrypted_server(w, e),
            Self::SMSG_ADDON_INFO(c) => c.write_encrypted_server(w, e),
            Self::SMSG_PET_UNLEARN_CONFIRM(c) => c.write_encrypted_server(w, e),
            Self::SMSG_CHAT_RESTRICTED(c) => c.write_encrypted_server(w, e),
            Self::SMSG_SPLINE_SET_RUN_SPEED(c) => c.write_encrypted_server(w, e),
            Self::SMSG_SPLINE_SET_RUN_BACK_SPEED(c) => c.write_encrypted_server(w, e),
            Self::SMSG_SPLINE_SET_SWIM_SPEED(c) => c.write_encrypted_server(w, e),
            Self::SMSG_SPLINE_SET_WALK_SPEED(c) => c.write_encrypted_server(w, e),
            Self::SMSG_SPLINE_SET_SWIM_BACK_SPEED(c) => c.write_encrypted_server(w, e),
            Self::SMSG_SPLINE_SET_TURN_RATE(c) => c.write_encrypted_server(w, e),
            Self::SMSG_SPLINE_MOVE_UNROOT(c) => c.write_encrypted_server(w, e),
            Self::SMSG_SPLINE_MOVE_FEATHER_FALL(c) => c.write_encrypted_server(w, e),
            Self::SMSG_SPLINE_MOVE_NORMAL_FALL(c) => c.write_encrypted_server(w, e),
            Self::SMSG_SPLINE_MOVE_SET_HOVER(c) => c.write_encrypted_server(w, e),
            Self::SMSG_SPLINE_MOVE_UNSET_HOVER(c) => c.write_encrypted_server(w, e),
            Self::SMSG_SPLINE_MOVE_WATER_WALK(c) => c.write_encrypted_server(w, e),
            Self::SMSG_SPLINE_MOVE_LAND_WALK(c) => c.write_encrypted_server(w, e),
            Self::SMSG_SPLINE_MOVE_START_SWIM(c) => c.write_encrypted_server(w, e),
            Self::SMSG_SPLINE_MOVE_STOP_SWIM(c) => c.write_encrypted_server(w, e),
            Self::SMSG_SPLINE_MOVE_SET_RUN_MODE(c) => c.write_encrypted_server(w, e),
            Self::SMSG_SPLINE_MOVE_SET_WALK_MODE(c) => c.write_encrypted_server(w, e),
            Self::SMSG_SPLINE_MOVE_ROOT(c) => c.write_encrypted_server(w, e),
            Self::MSG_RAID_READY_CHECK(c) => c.write_encrypted_server(w, e),
            Self::SMSG_PET_ACTION_SOUND(c) => c.write_encrypted_server(w, e),
            Self::SMSG_PET_DISMISS_SOUND(c) => c.write_encrypted_server(w, e),
            Self::SMSG_GM_TICKET_STATUS_UPDATE(c) => c.write_encrypted_server(w, e),
            Self::SMSG_DEFENSE_MESSAGE(c) => c.write_encrypted_server(w, e),
            Self::SMSG_REALM_SPLIT(c) => c.write_encrypted_server(w, e),
            Self::SMSG_TIME_SYNC_REQ(c) => c.write_encrypted_server(w, e),
            Self::SMSG_GM_MESSAGECHAT(c) => c.write_encrypted_server(w, e),
            Self::SMSG_FEATURE_SYSTEM_STATUS(c) => c.write_encrypted_server(w, e),
            Self::SMSG_CALENDAR_SEND_NUM_PENDING(c) => c.write_encrypted_server(w, e),
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
            Self::MSG_QUEST_PUSH_RESULT(c) => c.write_unencrypted_server(w),
            Self::MSG_PETITION_RENAME(c) => c.write_unencrypted_server(w),
            Self::MSG_SET_DUNGEON_DIFFICULTY(c) => c.write_unencrypted_server(w),
            Self::MSG_MOVE_START_ASCEND(c) => c.write_unencrypted_server(w),
            Self::MSG_MOVE_STOP_ASCEND(c) => c.write_unencrypted_server(w),
            Self::MSG_MOVE_START_DESCEND(c) => c.write_unencrypted_server(w),
            Self::MSG_GUILD_BANK_MONEY_WITHDRAWN(c) => c.write_unencrypted_server(w),
            Self::CMSG_CALENDAR_GET_NUM_PENDING(c) => c.write_unencrypted_server(w),
            Self::SMSG_CHAR_CREATE(c) => c.write_unencrypted_server(w),
            Self::SMSG_CHAR_ENUM(c) => c.write_unencrypted_server(w),
            Self::SMSG_CHAR_DELETE(c) => c.write_unencrypted_server(w),
            Self::SMSG_NEW_WORLD(c) => c.write_unencrypted_server(w),
            Self::SMSG_TRANSFER_PENDING(c) => c.write_unencrypted_server(w),
            Self::SMSG_TRANSFER_ABORTED(c) => c.write_unencrypted_server(w),
            Self::SMSG_CHARACTER_LOGIN_FAILED(c) => c.write_unencrypted_server(w),
            Self::SMSG_LOGIN_SETTIMESPEED(c) => c.write_unencrypted_server(w),
            Self::SMSG_LOGOUT_RESPONSE(c) => c.write_unencrypted_server(w),
            Self::SMSG_LOGOUT_COMPLETE(c) => c.write_unencrypted_server(w),
            Self::SMSG_LOGOUT_CANCEL_ACK(c) => c.write_unencrypted_server(w),
            Self::SMSG_NAME_QUERY_RESPONSE(c) => c.write_unencrypted_server(w),
            Self::SMSG_PET_NAME_QUERY_RESPONSE(c) => c.write_unencrypted_server(w),
            Self::SMSG_GUILD_QUERY_RESPONSE(c) => c.write_unencrypted_server(w),
            Self::SMSG_ITEM_QUERY_SINGLE_RESPONSE(c) => c.write_unencrypted_server(w),
            Self::SMSG_PAGE_TEXT_QUERY_RESPONSE(c) => c.write_unencrypted_server(w),
            Self::SMSG_QUEST_QUERY_RESPONSE(c) => c.write_unencrypted_server(w),
            Self::SMSG_GAMEOBJECT_QUERY_RESPONSE(c) => c.write_unencrypted_server(w),
            Self::SMSG_CREATURE_QUERY_RESPONSE(c) => c.write_unencrypted_server(w),
            Self::SMSG_WHO(c) => c.write_unencrypted_server(w),
            Self::SMSG_WHOIS(c) => c.write_unencrypted_server(w),
            Self::SMSG_CONTACT_LIST(c) => c.write_unencrypted_server(w),
            Self::SMSG_FRIEND_STATUS(c) => c.write_unencrypted_server(w),
            Self::SMSG_GROUP_INVITE(c) => c.write_unencrypted_server(w),
            Self::SMSG_GROUP_DECLINE(c) => c.write_unencrypted_server(w),
            Self::SMSG_GROUP_UNINVITE(c) => c.write_unencrypted_server(w),
            Self::SMSG_GROUP_SET_LEADER(c) => c.write_unencrypted_server(w),
            Self::SMSG_GROUP_DESTROYED(c) => c.write_unencrypted_server(w),
            Self::SMSG_GROUP_LIST(c) => c.write_unencrypted_server(w),
            Self::SMSG_PARTY_COMMAND_RESULT(c) => c.write_unencrypted_server(w),
            Self::SMSG_GUILD_INVITE(c) => c.write_unencrypted_server(w),
            Self::SMSG_GUILD_INFO(c) => c.write_unencrypted_server(w),
            Self::SMSG_GUILD_ROSTER(c) => c.write_unencrypted_server(w),
            Self::SMSG_GUILD_EVENT(c) => c.write_unencrypted_server(w),
            Self::SMSG_GUILD_COMMAND_RESULT(c) => c.write_unencrypted_server(w),
            Self::SMSG_MESSAGECHAT(c) => c.write_unencrypted_server(w),
            Self::SMSG_CHANNEL_NOTIFY(c) => c.write_unencrypted_server(w),
            Self::SMSG_CHANNEL_LIST(c) => c.write_unencrypted_server(w),
            Self::SMSG_UPDATE_OBJECT(c) => c.write_unencrypted_server(w),
            Self::SMSG_DESTROY_OBJECT(c) => c.write_unencrypted_server(w),
            Self::SMSG_READ_ITEM_OK(c) => c.write_unencrypted_server(w),
            Self::SMSG_READ_ITEM_FAILED(c) => c.write_unencrypted_server(w),
            Self::SMSG_ITEM_COOLDOWN(c) => c.write_unencrypted_server(w),
            Self::SMSG_GAMEOBJECT_CUSTOM_ANIM(c) => c.write_unencrypted_server(w),
            Self::MSG_MOVE_TELEPORT_ACK(c) => c.write_unencrypted_server(w),
            Self::SMSG_MOVE_WATER_WALK(c) => c.write_unencrypted_server(w),
            Self::SMSG_MOVE_LAND_WALK(c) => c.write_unencrypted_server(w),
            Self::SMSG_FORCE_RUN_SPEED_CHANGE(c) => c.write_unencrypted_server(w),
            Self::SMSG_FORCE_RUN_BACK_SPEED_CHANGE(c) => c.write_unencrypted_server(w),
            Self::SMSG_FORCE_SWIM_SPEED_CHANGE(c) => c.write_unencrypted_server(w),
            Self::SMSG_FORCE_MOVE_ROOT(c) => c.write_unencrypted_server(w),
            Self::SMSG_FORCE_MOVE_UNROOT(c) => c.write_unencrypted_server(w),
            Self::SMSG_MOVE_KNOCK_BACK(c) => c.write_unencrypted_server(w),
            Self::SMSG_MOVE_FEATHER_FALL(c) => c.write_unencrypted_server(w),
            Self::SMSG_MOVE_NORMAL_FALL(c) => c.write_unencrypted_server(w),
            Self::SMSG_MOVE_SET_HOVER(c) => c.write_unencrypted_server(w),
            Self::SMSG_MOVE_UNSET_HOVER(c) => c.write_unencrypted_server(w),
            Self::SMSG_TRIGGER_CINEMATIC(c) => c.write_unencrypted_server(w),
            Self::SMSG_TUTORIAL_FLAGS(c) => c.write_unencrypted_server(w),
            Self::SMSG_EMOTE(c) => c.write_unencrypted_server(w),
            Self::SMSG_TEXT_EMOTE(c) => c.write_unencrypted_server(w),
            Self::SMSG_INVENTORY_CHANGE_FAILURE(c) => c.write_unencrypted_server(w),
            Self::SMSG_TRADE_STATUS(c) => c.write_unencrypted_server(w),
            Self::SMSG_TRADE_STATUS_EXTENDED(c) => c.write_unencrypted_server(w),
            Self::SMSG_INITIALIZE_FACTIONS(c) => c.write_unencrypted_server(w),
            Self::SMSG_SET_FACTION_VISIBLE(c) => c.write_unencrypted_server(w),
            Self::SMSG_SET_FACTION_STANDING(c) => c.write_unencrypted_server(w),
            Self::SMSG_SET_PROFICIENCY(c) => c.write_unencrypted_server(w),
            Self::SMSG_ACTION_BUTTONS(c) => c.write_unencrypted_server(w),
            Self::SMSG_INITIAL_SPELLS(c) => c.write_unencrypted_server(w),
            Self::SMSG_LEARNED_SPELL(c) => c.write_unencrypted_server(w),
            Self::SMSG_SUPERCEDED_SPELL(c) => c.write_unencrypted_server(w),
            Self::SMSG_SPELL_START(c) => c.write_unencrypted_server(w),
            Self::SMSG_SPELL_GO(c) => c.write_unencrypted_server(w),
            Self::SMSG_SPELL_FAILURE(c) => c.write_unencrypted_server(w),
            Self::SMSG_SPELL_COOLDOWN(c) => c.write_unencrypted_server(w),
            Self::SMSG_COOLDOWN_EVENT(c) => c.write_unencrypted_server(w),
            Self::SMSG_PET_CAST_FAILED(c) => c.write_unencrypted_server(w),
            Self::SMSG_AI_REACTION(c) => c.write_unencrypted_server(w),
            Self::SMSG_ATTACKSTART(c) => c.write_unencrypted_server(w),
            Self::SMSG_ATTACKSTOP(c) => c.write_unencrypted_server(w),
            Self::SMSG_ATTACKSWING_NOTINRANGE(c) => c.write_unencrypted_server(w),
            Self::SMSG_ATTACKSWING_BADFACING(c) => c.write_unencrypted_server(w),
            Self::SMSG_ATTACKSWING_DEADTARGET(c) => c.write_unencrypted_server(w),
            Self::SMSG_ATTACKSWING_CANT_ATTACK(c) => c.write_unencrypted_server(w),
            Self::SMSG_ATTACKERSTATEUPDATE(c) => c.write_unencrypted_server(w),
            Self::SMSG_CANCEL_COMBAT(c) => c.write_unencrypted_server(w),
            Self::SMSG_SPELLHEALLOG(c) => c.write_unencrypted_server(w),
            Self::SMSG_SPELLENERGIZELOG(c) => c.write_unencrypted_server(w),
            Self::SMSG_BINDPOINTUPDATE(c) => c.write_unencrypted_server(w),
            Self::SMSG_PLAYERBOUND(c) => c.write_unencrypted_server(w),
            Self::SMSG_CLIENT_CONTROL_UPDATE(c) => c.write_unencrypted_server(w),
            Self::SMSG_RESURRECT_REQUEST(c) => c.write_unencrypted_server(w),
            Self::SMSG_LOOT_RESPONSE(c) => c.write_unencrypted_server(w),
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
            Self::SMSG_PET_TAME_FAILURE(c) => c.write_unencrypted_server(w),
            Self::SMSG_PET_NAME_INVALID(c) => c.write_unencrypted_server(w),
            Self::SMSG_PET_SPELLS(c) => c.write_unencrypted_server(w),
            Self::SMSG_PET_MODE(c) => c.write_unencrypted_server(w),
            Self::SMSG_GOSSIP_MESSAGE(c) => c.write_unencrypted_server(w),
            Self::SMSG_GOSSIP_COMPLETE(c) => c.write_unencrypted_server(w),
            Self::SMSG_NPC_TEXT_UPDATE(c) => c.write_unencrypted_server(w),
            Self::SMSG_QUESTGIVER_STATUS(c) => c.write_unencrypted_server(w),
            Self::SMSG_QUESTGIVER_QUEST_LIST(c) => c.write_unencrypted_server(w),
            Self::SMSG_QUESTGIVER_QUEST_DETAILS(c) => c.write_unencrypted_server(w),
            Self::SMSG_QUESTGIVER_REQUEST_ITEMS(c) => c.write_unencrypted_server(w),
            Self::SMSG_QUESTGIVER_OFFER_REWARD(c) => c.write_unencrypted_server(w),
            Self::SMSG_QUESTGIVER_QUEST_INVALID(c) => c.write_unencrypted_server(w),
            Self::SMSG_QUESTGIVER_QUEST_COMPLETE(c) => c.write_unencrypted_server(w),
            Self::SMSG_QUESTGIVER_QUEST_FAILED(c) => c.write_unencrypted_server(w),
            Self::SMSG_QUESTLOG_FULL(c) => c.write_unencrypted_server(w),
            Self::SMSG_QUESTUPDATE_FAILED(c) => c.write_unencrypted_server(w),
            Self::SMSG_QUESTUPDATE_FAILEDTIMER(c) => c.write_unencrypted_server(w),
            Self::SMSG_QUESTUPDATE_COMPLETE(c) => c.write_unencrypted_server(w),
            Self::SMSG_QUESTUPDATE_ADD_KILL(c) => c.write_unencrypted_server(w),
            Self::SMSG_QUESTUPDATE_ADD_ITEM(c) => c.write_unencrypted_server(w),
            Self::SMSG_QUEST_CONFIRM_ACCEPT(c) => c.write_unencrypted_server(w),
            Self::SMSG_LIST_INVENTORY(c) => c.write_unencrypted_server(w),
            Self::SMSG_SELL_ITEM(c) => c.write_unencrypted_server(w),
            Self::SMSG_BUY_ITEM(c) => c.write_unencrypted_server(w),
            Self::SMSG_BUY_FAILED(c) => c.write_unencrypted_server(w),
            Self::SMSG_SHOWTAXINODES(c) => c.write_unencrypted_server(w),
            Self::SMSG_TAXINODE_STATUS(c) => c.write_unencrypted_server(w),
            Self::SMSG_ACTIVATETAXIREPLY(c) => c.write_unencrypted_server(w),
            Self::SMSG_NEW_TAXI_PATH(c) => c.write_unencrypted_server(w),
            Self::SMSG_TRAINER_LIST(c) => c.write_unencrypted_server(w),
            Self::SMSG_TRAINER_BUY_SUCCEEDED(c) => c.write_unencrypted_server(w),
            Self::SMSG_TRAINER_BUY_FAILED(c) => c.write_unencrypted_server(w),
            Self::SMSG_SHOW_BANK(c) => c.write_unencrypted_server(w),
            Self::SMSG_BUY_BANK_SLOT_RESULT(c) => c.write_unencrypted_server(w),
            Self::SMSG_PETITION_SHOWLIST(c) => c.write_unencrypted_server(w),
            Self::SMSG_PETITION_SHOW_SIGNATURES(c) => c.write_unencrypted_server(w),
            Self::SMSG_PETITION_SIGN_RESULTS(c) => c.write_unencrypted_server(w),
            Self::SMSG_TURN_IN_PETITION_RESULTS(c) => c.write_unencrypted_server(w),
            Self::SMSG_PETITION_QUERY_RESPONSE(c) => c.write_unencrypted_server(w),
            Self::SMSG_FISH_NOT_HOOKED(c) => c.write_unencrypted_server(w),
            Self::SMSG_FISH_ESCAPED(c) => c.write_unencrypted_server(w),
            Self::SMSG_NOTIFICATION(c) => c.write_unencrypted_server(w),
            Self::SMSG_PLAYED_TIME(c) => c.write_unencrypted_server(w),
            Self::SMSG_QUERY_TIME_RESPONSE(c) => c.write_unencrypted_server(w),
            Self::SMSG_LOG_XPGAIN(c) => c.write_unencrypted_server(w),
            Self::SMSG_LEVELUP_INFO(c) => c.write_unencrypted_server(w),
            Self::MSG_MINIMAP_PING(c) => c.write_unencrypted_server(w),
            Self::SMSG_PONG(c) => c.write_unencrypted_server(w),
            Self::SMSG_GAMEOBJECT_PAGETEXT(c) => c.write_unencrypted_server(w),
            Self::SMSG_ITEM_TIME_UPDATE(c) => c.write_unencrypted_server(w),
            Self::SMSG_ITEM_ENCHANT_TIME_UPDATE(c) => c.write_unencrypted_server(w),
            Self::SMSG_AUTH_CHALLENGE(c) => c.write_unencrypted_server(w),
            Self::SMSG_AUTH_RESPONSE(c) => c.write_unencrypted_server(w),
            Self::MSG_SAVE_GUILD_EMBLEM(c) => c.write_unencrypted_server(w),
            Self::SMSG_EXPLORATION_EXPERIENCE(c) => c.write_unencrypted_server(w),
            Self::MSG_RANDOM_ROLL(c) => c.write_unencrypted_server(w),
            Self::SMSG_ENVIRONMENTAL_DAMAGE_LOG(c) => c.write_unencrypted_server(w),
            Self::SMSG_GMTICKET_CREATE(c) => c.write_unencrypted_server(w),
            Self::SMSG_GMTICKET_UPDATETEXT(c) => c.write_unencrypted_server(w),
            Self::SMSG_ACCOUNT_DATA_TIMES(c) => c.write_unencrypted_server(w),
            Self::SMSG_UPDATE_ACCOUNT_DATA(c) => c.write_unencrypted_server(w),
            Self::SMSG_GMTICKET_GETTICKET(c) => c.write_unencrypted_server(w),
            Self::SMSG_GAMEOBJECT_DESPAWN_ANIM(c) => c.write_unencrypted_server(w),
            Self::MSG_CORPSE_QUERY(c) => c.write_unencrypted_server(w),
            Self::SMSG_GMTICKET_DELETETICKET(c) => c.write_unencrypted_server(w),
            Self::SMSG_CHAT_WRONG_FACTION(c) => c.write_unencrypted_server(w),
            Self::SMSG_GMTICKET_SYSTEMSTATUS(c) => c.write_unencrypted_server(w),
            Self::SMSG_QUEST_FORCE_REMOVE(c) => c.write_unencrypted_server(w),
            Self::SMSG_GOSSIP_POI(c) => c.write_unencrypted_server(w),
            Self::SMSG_LOGIN_VERIFY_WORLD(c) => c.write_unencrypted_server(w),
            Self::SMSG_BATTLEFIELD_LIST(c) => c.write_unencrypted_server(w),
            Self::SMSG_ITEM_TEXT_QUERY_RESPONSE(c) => c.write_unencrypted_server(w),
            Self::SMSG_ZONE_UNDER_ATTACK(c) => c.write_unencrypted_server(w),
            Self::MSG_AUCTION_HELLO(c) => c.write_unencrypted_server(w),
            Self::SMSG_AUCTION_LIST_RESULT(c) => c.write_unencrypted_server(w),
            Self::SMSG_AUCTION_OWNER_LIST_RESULT(c) => c.write_unencrypted_server(w),
            Self::SMSG_AUCTION_BIDDER_NOTIFICATION(c) => c.write_unencrypted_server(w),
            Self::SMSG_AUCTION_OWNER_NOTIFICATION(c) => c.write_unencrypted_server(w),
            Self::SMSG_PROCRESIST(c) => c.write_unencrypted_server(w),
            Self::SMSG_AUCTION_BIDDER_LIST_RESULT(c) => c.write_unencrypted_server(w),
            Self::MSG_LIST_STABLED_PETS(c) => c.write_unencrypted_server(w),
            Self::SMSG_STABLE_RESULT(c) => c.write_unencrypted_server(w),
            Self::SMSG_PLAY_OBJECT_SOUND(c) => c.write_unencrypted_server(w),
            Self::MSG_QUERY_NEXT_MAIL_TIME(c) => c.write_unencrypted_server(w),
            Self::SMSG_RECEIVED_MAIL(c) => c.write_unencrypted_server(w),
            Self::SMSG_PVP_CREDIT(c) => c.write_unencrypted_server(w),
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
            Self::SMSG_PET_BROKEN(c) => c.write_unencrypted_server(w),
            Self::SMSG_DUEL_COUNTDOWN(c) => c.write_unencrypted_server(w),
            Self::SMSG_AREA_TRIGGER_MESSAGE(c) => c.write_unencrypted_server(w),
            Self::SMSG_DURABILITY_DAMAGE_DEATH(c) => c.write_unencrypted_server(w),
            Self::SMSG_INIT_WORLD_STATES(c) => c.write_unencrypted_server(w),
            Self::SMSG_UPDATE_WORLD_STATE(c) => c.write_unencrypted_server(w),
            Self::SMSG_ITEM_NAME_QUERY_RESPONSE(c) => c.write_unencrypted_server(w),
            Self::SMSG_PET_ACTION_FEEDBACK(c) => c.write_unencrypted_server(w),
            Self::SMSG_CHAR_RENAME(c) => c.write_unencrypted_server(w),
            Self::SMSG_RAID_INSTANCE_INFO(c) => c.write_unencrypted_server(w),
            Self::MSG_INSPECT_HONOR_STATS(c) => c.write_unencrypted_server(w),
            Self::SMSG_FORCE_WALK_SPEED_CHANGE(c) => c.write_unencrypted_server(w),
            Self::SMSG_FORCE_SWIM_BACK_SPEED_CHANGE(c) => c.write_unencrypted_server(w),
            Self::SMSG_FORCE_TURN_RATE_CHANGE(c) => c.write_unencrypted_server(w),
            Self::MSG_BATTLEGROUND_PLAYER_POSITIONS(c) => c.write_unencrypted_server(w),
            Self::SMSG_BINDER_CONFIRM(c) => c.write_unencrypted_server(w),
            Self::SMSG_BATTLEGROUND_PLAYER_JOINED(c) => c.write_unencrypted_server(w),
            Self::SMSG_BATTLEGROUND_PLAYER_LEFT(c) => c.write_unencrypted_server(w),
            Self::SMSG_ADDON_INFO(c) => c.write_unencrypted_server(w),
            Self::SMSG_PET_UNLEARN_CONFIRM(c) => c.write_unencrypted_server(w),
            Self::SMSG_CHAT_RESTRICTED(c) => c.write_unencrypted_server(w),
            Self::SMSG_SPLINE_SET_RUN_SPEED(c) => c.write_unencrypted_server(w),
            Self::SMSG_SPLINE_SET_RUN_BACK_SPEED(c) => c.write_unencrypted_server(w),
            Self::SMSG_SPLINE_SET_SWIM_SPEED(c) => c.write_unencrypted_server(w),
            Self::SMSG_SPLINE_SET_WALK_SPEED(c) => c.write_unencrypted_server(w),
            Self::SMSG_SPLINE_SET_SWIM_BACK_SPEED(c) => c.write_unencrypted_server(w),
            Self::SMSG_SPLINE_SET_TURN_RATE(c) => c.write_unencrypted_server(w),
            Self::SMSG_SPLINE_MOVE_UNROOT(c) => c.write_unencrypted_server(w),
            Self::SMSG_SPLINE_MOVE_FEATHER_FALL(c) => c.write_unencrypted_server(w),
            Self::SMSG_SPLINE_MOVE_NORMAL_FALL(c) => c.write_unencrypted_server(w),
            Self::SMSG_SPLINE_MOVE_SET_HOVER(c) => c.write_unencrypted_server(w),
            Self::SMSG_SPLINE_MOVE_UNSET_HOVER(c) => c.write_unencrypted_server(w),
            Self::SMSG_SPLINE_MOVE_WATER_WALK(c) => c.write_unencrypted_server(w),
            Self::SMSG_SPLINE_MOVE_LAND_WALK(c) => c.write_unencrypted_server(w),
            Self::SMSG_SPLINE_MOVE_START_SWIM(c) => c.write_unencrypted_server(w),
            Self::SMSG_SPLINE_MOVE_STOP_SWIM(c) => c.write_unencrypted_server(w),
            Self::SMSG_SPLINE_MOVE_SET_RUN_MODE(c) => c.write_unencrypted_server(w),
            Self::SMSG_SPLINE_MOVE_SET_WALK_MODE(c) => c.write_unencrypted_server(w),
            Self::SMSG_SPLINE_MOVE_ROOT(c) => c.write_unencrypted_server(w),
            Self::MSG_RAID_READY_CHECK(c) => c.write_unencrypted_server(w),
            Self::SMSG_PET_ACTION_SOUND(c) => c.write_unencrypted_server(w),
            Self::SMSG_PET_DISMISS_SOUND(c) => c.write_unencrypted_server(w),
            Self::SMSG_GM_TICKET_STATUS_UPDATE(c) => c.write_unencrypted_server(w),
            Self::SMSG_DEFENSE_MESSAGE(c) => c.write_unencrypted_server(w),
            Self::SMSG_REALM_SPLIT(c) => c.write_unencrypted_server(w),
            Self::SMSG_TIME_SYNC_REQ(c) => c.write_unencrypted_server(w),
            Self::SMSG_GM_MESSAGECHAT(c) => c.write_unencrypted_server(w),
            Self::SMSG_FEATURE_SYSTEM_STATUS(c) => c.write_unencrypted_server(w),
            Self::SMSG_CALENDAR_SEND_NUM_PENDING(c) => c.write_unencrypted_server(w),
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
            Self::MSG_QUEST_PUSH_RESULT(c) => c.tokio_write_encrypted_server(w, e).await,
            Self::MSG_PETITION_RENAME(c) => c.tokio_write_encrypted_server(w, e).await,
            Self::MSG_SET_DUNGEON_DIFFICULTY(c) => c.tokio_write_encrypted_server(w, e).await,
            Self::MSG_MOVE_START_ASCEND(c) => c.tokio_write_encrypted_server(w, e).await,
            Self::MSG_MOVE_STOP_ASCEND(c) => c.tokio_write_encrypted_server(w, e).await,
            Self::MSG_MOVE_START_DESCEND(c) => c.tokio_write_encrypted_server(w, e).await,
            Self::MSG_GUILD_BANK_MONEY_WITHDRAWN(c) => c.tokio_write_encrypted_server(w, e).await,
            Self::CMSG_CALENDAR_GET_NUM_PENDING(c) => c.tokio_write_encrypted_server(w, e).await,
            Self::SMSG_CHAR_CREATE(c) => c.tokio_write_encrypted_server(w, e).await,
            Self::SMSG_CHAR_ENUM(c) => c.tokio_write_encrypted_server(w, e).await,
            Self::SMSG_CHAR_DELETE(c) => c.tokio_write_encrypted_server(w, e).await,
            Self::SMSG_NEW_WORLD(c) => c.tokio_write_encrypted_server(w, e).await,
            Self::SMSG_TRANSFER_PENDING(c) => c.tokio_write_encrypted_server(w, e).await,
            Self::SMSG_TRANSFER_ABORTED(c) => c.tokio_write_encrypted_server(w, e).await,
            Self::SMSG_CHARACTER_LOGIN_FAILED(c) => c.tokio_write_encrypted_server(w, e).await,
            Self::SMSG_LOGIN_SETTIMESPEED(c) => c.tokio_write_encrypted_server(w, e).await,
            Self::SMSG_LOGOUT_RESPONSE(c) => c.tokio_write_encrypted_server(w, e).await,
            Self::SMSG_LOGOUT_COMPLETE(c) => c.tokio_write_encrypted_server(w, e).await,
            Self::SMSG_LOGOUT_CANCEL_ACK(c) => c.tokio_write_encrypted_server(w, e).await,
            Self::SMSG_NAME_QUERY_RESPONSE(c) => c.tokio_write_encrypted_server(w, e).await,
            Self::SMSG_PET_NAME_QUERY_RESPONSE(c) => c.tokio_write_encrypted_server(w, e).await,
            Self::SMSG_GUILD_QUERY_RESPONSE(c) => c.tokio_write_encrypted_server(w, e).await,
            Self::SMSG_ITEM_QUERY_SINGLE_RESPONSE(c) => c.tokio_write_encrypted_server(w, e).await,
            Self::SMSG_PAGE_TEXT_QUERY_RESPONSE(c) => c.tokio_write_encrypted_server(w, e).await,
            Self::SMSG_QUEST_QUERY_RESPONSE(c) => c.tokio_write_encrypted_server(w, e).await,
            Self::SMSG_GAMEOBJECT_QUERY_RESPONSE(c) => c.tokio_write_encrypted_server(w, e).await,
            Self::SMSG_CREATURE_QUERY_RESPONSE(c) => c.tokio_write_encrypted_server(w, e).await,
            Self::SMSG_WHO(c) => c.tokio_write_encrypted_server(w, e).await,
            Self::SMSG_WHOIS(c) => c.tokio_write_encrypted_server(w, e).await,
            Self::SMSG_CONTACT_LIST(c) => c.tokio_write_encrypted_server(w, e).await,
            Self::SMSG_FRIEND_STATUS(c) => c.tokio_write_encrypted_server(w, e).await,
            Self::SMSG_GROUP_INVITE(c) => c.tokio_write_encrypted_server(w, e).await,
            Self::SMSG_GROUP_DECLINE(c) => c.tokio_write_encrypted_server(w, e).await,
            Self::SMSG_GROUP_UNINVITE(c) => c.tokio_write_encrypted_server(w, e).await,
            Self::SMSG_GROUP_SET_LEADER(c) => c.tokio_write_encrypted_server(w, e).await,
            Self::SMSG_GROUP_DESTROYED(c) => c.tokio_write_encrypted_server(w, e).await,
            Self::SMSG_GROUP_LIST(c) => c.tokio_write_encrypted_server(w, e).await,
            Self::SMSG_PARTY_COMMAND_RESULT(c) => c.tokio_write_encrypted_server(w, e).await,
            Self::SMSG_GUILD_INVITE(c) => c.tokio_write_encrypted_server(w, e).await,
            Self::SMSG_GUILD_INFO(c) => c.tokio_write_encrypted_server(w, e).await,
            Self::SMSG_GUILD_ROSTER(c) => c.tokio_write_encrypted_server(w, e).await,
            Self::SMSG_GUILD_EVENT(c) => c.tokio_write_encrypted_server(w, e).await,
            Self::SMSG_GUILD_COMMAND_RESULT(c) => c.tokio_write_encrypted_server(w, e).await,
            Self::SMSG_MESSAGECHAT(c) => c.tokio_write_encrypted_server(w, e).await,
            Self::SMSG_CHANNEL_NOTIFY(c) => c.tokio_write_encrypted_server(w, e).await,
            Self::SMSG_CHANNEL_LIST(c) => c.tokio_write_encrypted_server(w, e).await,
            Self::SMSG_UPDATE_OBJECT(c) => c.tokio_write_encrypted_server(w, e).await,
            Self::SMSG_DESTROY_OBJECT(c) => c.tokio_write_encrypted_server(w, e).await,
            Self::SMSG_READ_ITEM_OK(c) => c.tokio_write_encrypted_server(w, e).await,
            Self::SMSG_READ_ITEM_FAILED(c) => c.tokio_write_encrypted_server(w, e).await,
            Self::SMSG_ITEM_COOLDOWN(c) => c.tokio_write_encrypted_server(w, e).await,
            Self::SMSG_GAMEOBJECT_CUSTOM_ANIM(c) => c.tokio_write_encrypted_server(w, e).await,
            Self::MSG_MOVE_TELEPORT_ACK(c) => c.tokio_write_encrypted_server(w, e).await,
            Self::SMSG_MOVE_WATER_WALK(c) => c.tokio_write_encrypted_server(w, e).await,
            Self::SMSG_MOVE_LAND_WALK(c) => c.tokio_write_encrypted_server(w, e).await,
            Self::SMSG_FORCE_RUN_SPEED_CHANGE(c) => c.tokio_write_encrypted_server(w, e).await,
            Self::SMSG_FORCE_RUN_BACK_SPEED_CHANGE(c) => c.tokio_write_encrypted_server(w, e).await,
            Self::SMSG_FORCE_SWIM_SPEED_CHANGE(c) => c.tokio_write_encrypted_server(w, e).await,
            Self::SMSG_FORCE_MOVE_ROOT(c) => c.tokio_write_encrypted_server(w, e).await,
            Self::SMSG_FORCE_MOVE_UNROOT(c) => c.tokio_write_encrypted_server(w, e).await,
            Self::SMSG_MOVE_KNOCK_BACK(c) => c.tokio_write_encrypted_server(w, e).await,
            Self::SMSG_MOVE_FEATHER_FALL(c) => c.tokio_write_encrypted_server(w, e).await,
            Self::SMSG_MOVE_NORMAL_FALL(c) => c.tokio_write_encrypted_server(w, e).await,
            Self::SMSG_MOVE_SET_HOVER(c) => c.tokio_write_encrypted_server(w, e).await,
            Self::SMSG_MOVE_UNSET_HOVER(c) => c.tokio_write_encrypted_server(w, e).await,
            Self::SMSG_TRIGGER_CINEMATIC(c) => c.tokio_write_encrypted_server(w, e).await,
            Self::SMSG_TUTORIAL_FLAGS(c) => c.tokio_write_encrypted_server(w, e).await,
            Self::SMSG_EMOTE(c) => c.tokio_write_encrypted_server(w, e).await,
            Self::SMSG_TEXT_EMOTE(c) => c.tokio_write_encrypted_server(w, e).await,
            Self::SMSG_INVENTORY_CHANGE_FAILURE(c) => c.tokio_write_encrypted_server(w, e).await,
            Self::SMSG_TRADE_STATUS(c) => c.tokio_write_encrypted_server(w, e).await,
            Self::SMSG_TRADE_STATUS_EXTENDED(c) => c.tokio_write_encrypted_server(w, e).await,
            Self::SMSG_INITIALIZE_FACTIONS(c) => c.tokio_write_encrypted_server(w, e).await,
            Self::SMSG_SET_FACTION_VISIBLE(c) => c.tokio_write_encrypted_server(w, e).await,
            Self::SMSG_SET_FACTION_STANDING(c) => c.tokio_write_encrypted_server(w, e).await,
            Self::SMSG_SET_PROFICIENCY(c) => c.tokio_write_encrypted_server(w, e).await,
            Self::SMSG_ACTION_BUTTONS(c) => c.tokio_write_encrypted_server(w, e).await,
            Self::SMSG_INITIAL_SPELLS(c) => c.tokio_write_encrypted_server(w, e).await,
            Self::SMSG_LEARNED_SPELL(c) => c.tokio_write_encrypted_server(w, e).await,
            Self::SMSG_SUPERCEDED_SPELL(c) => c.tokio_write_encrypted_server(w, e).await,
            Self::SMSG_SPELL_START(c) => c.tokio_write_encrypted_server(w, e).await,
            Self::SMSG_SPELL_GO(c) => c.tokio_write_encrypted_server(w, e).await,
            Self::SMSG_SPELL_FAILURE(c) => c.tokio_write_encrypted_server(w, e).await,
            Self::SMSG_SPELL_COOLDOWN(c) => c.tokio_write_encrypted_server(w, e).await,
            Self::SMSG_COOLDOWN_EVENT(c) => c.tokio_write_encrypted_server(w, e).await,
            Self::SMSG_PET_CAST_FAILED(c) => c.tokio_write_encrypted_server(w, e).await,
            Self::SMSG_AI_REACTION(c) => c.tokio_write_encrypted_server(w, e).await,
            Self::SMSG_ATTACKSTART(c) => c.tokio_write_encrypted_server(w, e).await,
            Self::SMSG_ATTACKSTOP(c) => c.tokio_write_encrypted_server(w, e).await,
            Self::SMSG_ATTACKSWING_NOTINRANGE(c) => c.tokio_write_encrypted_server(w, e).await,
            Self::SMSG_ATTACKSWING_BADFACING(c) => c.tokio_write_encrypted_server(w, e).await,
            Self::SMSG_ATTACKSWING_DEADTARGET(c) => c.tokio_write_encrypted_server(w, e).await,
            Self::SMSG_ATTACKSWING_CANT_ATTACK(c) => c.tokio_write_encrypted_server(w, e).await,
            Self::SMSG_ATTACKERSTATEUPDATE(c) => c.tokio_write_encrypted_server(w, e).await,
            Self::SMSG_CANCEL_COMBAT(c) => c.tokio_write_encrypted_server(w, e).await,
            Self::SMSG_SPELLHEALLOG(c) => c.tokio_write_encrypted_server(w, e).await,
            Self::SMSG_SPELLENERGIZELOG(c) => c.tokio_write_encrypted_server(w, e).await,
            Self::SMSG_BINDPOINTUPDATE(c) => c.tokio_write_encrypted_server(w, e).await,
            Self::SMSG_PLAYERBOUND(c) => c.tokio_write_encrypted_server(w, e).await,
            Self::SMSG_CLIENT_CONTROL_UPDATE(c) => c.tokio_write_encrypted_server(w, e).await,
            Self::SMSG_RESURRECT_REQUEST(c) => c.tokio_write_encrypted_server(w, e).await,
            Self::SMSG_LOOT_RESPONSE(c) => c.tokio_write_encrypted_server(w, e).await,
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
            Self::SMSG_PET_TAME_FAILURE(c) => c.tokio_write_encrypted_server(w, e).await,
            Self::SMSG_PET_NAME_INVALID(c) => c.tokio_write_encrypted_server(w, e).await,
            Self::SMSG_PET_SPELLS(c) => c.tokio_write_encrypted_server(w, e).await,
            Self::SMSG_PET_MODE(c) => c.tokio_write_encrypted_server(w, e).await,
            Self::SMSG_GOSSIP_MESSAGE(c) => c.tokio_write_encrypted_server(w, e).await,
            Self::SMSG_GOSSIP_COMPLETE(c) => c.tokio_write_encrypted_server(w, e).await,
            Self::SMSG_NPC_TEXT_UPDATE(c) => c.tokio_write_encrypted_server(w, e).await,
            Self::SMSG_QUESTGIVER_STATUS(c) => c.tokio_write_encrypted_server(w, e).await,
            Self::SMSG_QUESTGIVER_QUEST_LIST(c) => c.tokio_write_encrypted_server(w, e).await,
            Self::SMSG_QUESTGIVER_QUEST_DETAILS(c) => c.tokio_write_encrypted_server(w, e).await,
            Self::SMSG_QUESTGIVER_REQUEST_ITEMS(c) => c.tokio_write_encrypted_server(w, e).await,
            Self::SMSG_QUESTGIVER_OFFER_REWARD(c) => c.tokio_write_encrypted_server(w, e).await,
            Self::SMSG_QUESTGIVER_QUEST_INVALID(c) => c.tokio_write_encrypted_server(w, e).await,
            Self::SMSG_QUESTGIVER_QUEST_COMPLETE(c) => c.tokio_write_encrypted_server(w, e).await,
            Self::SMSG_QUESTGIVER_QUEST_FAILED(c) => c.tokio_write_encrypted_server(w, e).await,
            Self::SMSG_QUESTLOG_FULL(c) => c.tokio_write_encrypted_server(w, e).await,
            Self::SMSG_QUESTUPDATE_FAILED(c) => c.tokio_write_encrypted_server(w, e).await,
            Self::SMSG_QUESTUPDATE_FAILEDTIMER(c) => c.tokio_write_encrypted_server(w, e).await,
            Self::SMSG_QUESTUPDATE_COMPLETE(c) => c.tokio_write_encrypted_server(w, e).await,
            Self::SMSG_QUESTUPDATE_ADD_KILL(c) => c.tokio_write_encrypted_server(w, e).await,
            Self::SMSG_QUESTUPDATE_ADD_ITEM(c) => c.tokio_write_encrypted_server(w, e).await,
            Self::SMSG_QUEST_CONFIRM_ACCEPT(c) => c.tokio_write_encrypted_server(w, e).await,
            Self::SMSG_LIST_INVENTORY(c) => c.tokio_write_encrypted_server(w, e).await,
            Self::SMSG_SELL_ITEM(c) => c.tokio_write_encrypted_server(w, e).await,
            Self::SMSG_BUY_ITEM(c) => c.tokio_write_encrypted_server(w, e).await,
            Self::SMSG_BUY_FAILED(c) => c.tokio_write_encrypted_server(w, e).await,
            Self::SMSG_SHOWTAXINODES(c) => c.tokio_write_encrypted_server(w, e).await,
            Self::SMSG_TAXINODE_STATUS(c) => c.tokio_write_encrypted_server(w, e).await,
            Self::SMSG_ACTIVATETAXIREPLY(c) => c.tokio_write_encrypted_server(w, e).await,
            Self::SMSG_NEW_TAXI_PATH(c) => c.tokio_write_encrypted_server(w, e).await,
            Self::SMSG_TRAINER_LIST(c) => c.tokio_write_encrypted_server(w, e).await,
            Self::SMSG_TRAINER_BUY_SUCCEEDED(c) => c.tokio_write_encrypted_server(w, e).await,
            Self::SMSG_TRAINER_BUY_FAILED(c) => c.tokio_write_encrypted_server(w, e).await,
            Self::SMSG_SHOW_BANK(c) => c.tokio_write_encrypted_server(w, e).await,
            Self::SMSG_BUY_BANK_SLOT_RESULT(c) => c.tokio_write_encrypted_server(w, e).await,
            Self::SMSG_PETITION_SHOWLIST(c) => c.tokio_write_encrypted_server(w, e).await,
            Self::SMSG_PETITION_SHOW_SIGNATURES(c) => c.tokio_write_encrypted_server(w, e).await,
            Self::SMSG_PETITION_SIGN_RESULTS(c) => c.tokio_write_encrypted_server(w, e).await,
            Self::SMSG_TURN_IN_PETITION_RESULTS(c) => c.tokio_write_encrypted_server(w, e).await,
            Self::SMSG_PETITION_QUERY_RESPONSE(c) => c.tokio_write_encrypted_server(w, e).await,
            Self::SMSG_FISH_NOT_HOOKED(c) => c.tokio_write_encrypted_server(w, e).await,
            Self::SMSG_FISH_ESCAPED(c) => c.tokio_write_encrypted_server(w, e).await,
            Self::SMSG_NOTIFICATION(c) => c.tokio_write_encrypted_server(w, e).await,
            Self::SMSG_PLAYED_TIME(c) => c.tokio_write_encrypted_server(w, e).await,
            Self::SMSG_QUERY_TIME_RESPONSE(c) => c.tokio_write_encrypted_server(w, e).await,
            Self::SMSG_LOG_XPGAIN(c) => c.tokio_write_encrypted_server(w, e).await,
            Self::SMSG_LEVELUP_INFO(c) => c.tokio_write_encrypted_server(w, e).await,
            Self::MSG_MINIMAP_PING(c) => c.tokio_write_encrypted_server(w, e).await,
            Self::SMSG_PONG(c) => c.tokio_write_encrypted_server(w, e).await,
            Self::SMSG_GAMEOBJECT_PAGETEXT(c) => c.tokio_write_encrypted_server(w, e).await,
            Self::SMSG_ITEM_TIME_UPDATE(c) => c.tokio_write_encrypted_server(w, e).await,
            Self::SMSG_ITEM_ENCHANT_TIME_UPDATE(c) => c.tokio_write_encrypted_server(w, e).await,
            Self::SMSG_AUTH_CHALLENGE(c) => c.tokio_write_encrypted_server(w, e).await,
            Self::SMSG_AUTH_RESPONSE(c) => c.tokio_write_encrypted_server(w, e).await,
            Self::MSG_SAVE_GUILD_EMBLEM(c) => c.tokio_write_encrypted_server(w, e).await,
            Self::SMSG_EXPLORATION_EXPERIENCE(c) => c.tokio_write_encrypted_server(w, e).await,
            Self::MSG_RANDOM_ROLL(c) => c.tokio_write_encrypted_server(w, e).await,
            Self::SMSG_ENVIRONMENTAL_DAMAGE_LOG(c) => c.tokio_write_encrypted_server(w, e).await,
            Self::SMSG_GMTICKET_CREATE(c) => c.tokio_write_encrypted_server(w, e).await,
            Self::SMSG_GMTICKET_UPDATETEXT(c) => c.tokio_write_encrypted_server(w, e).await,
            Self::SMSG_ACCOUNT_DATA_TIMES(c) => c.tokio_write_encrypted_server(w, e).await,
            Self::SMSG_UPDATE_ACCOUNT_DATA(c) => c.tokio_write_encrypted_server(w, e).await,
            Self::SMSG_GMTICKET_GETTICKET(c) => c.tokio_write_encrypted_server(w, e).await,
            Self::SMSG_GAMEOBJECT_DESPAWN_ANIM(c) => c.tokio_write_encrypted_server(w, e).await,
            Self::MSG_CORPSE_QUERY(c) => c.tokio_write_encrypted_server(w, e).await,
            Self::SMSG_GMTICKET_DELETETICKET(c) => c.tokio_write_encrypted_server(w, e).await,
            Self::SMSG_CHAT_WRONG_FACTION(c) => c.tokio_write_encrypted_server(w, e).await,
            Self::SMSG_GMTICKET_SYSTEMSTATUS(c) => c.tokio_write_encrypted_server(w, e).await,
            Self::SMSG_QUEST_FORCE_REMOVE(c) => c.tokio_write_encrypted_server(w, e).await,
            Self::SMSG_GOSSIP_POI(c) => c.tokio_write_encrypted_server(w, e).await,
            Self::SMSG_LOGIN_VERIFY_WORLD(c) => c.tokio_write_encrypted_server(w, e).await,
            Self::SMSG_BATTLEFIELD_LIST(c) => c.tokio_write_encrypted_server(w, e).await,
            Self::SMSG_ITEM_TEXT_QUERY_RESPONSE(c) => c.tokio_write_encrypted_server(w, e).await,
            Self::SMSG_ZONE_UNDER_ATTACK(c) => c.tokio_write_encrypted_server(w, e).await,
            Self::MSG_AUCTION_HELLO(c) => c.tokio_write_encrypted_server(w, e).await,
            Self::SMSG_AUCTION_LIST_RESULT(c) => c.tokio_write_encrypted_server(w, e).await,
            Self::SMSG_AUCTION_OWNER_LIST_RESULT(c) => c.tokio_write_encrypted_server(w, e).await,
            Self::SMSG_AUCTION_BIDDER_NOTIFICATION(c) => c.tokio_write_encrypted_server(w, e).await,
            Self::SMSG_AUCTION_OWNER_NOTIFICATION(c) => c.tokio_write_encrypted_server(w, e).await,
            Self::SMSG_PROCRESIST(c) => c.tokio_write_encrypted_server(w, e).await,
            Self::SMSG_AUCTION_BIDDER_LIST_RESULT(c) => c.tokio_write_encrypted_server(w, e).await,
            Self::MSG_LIST_STABLED_PETS(c) => c.tokio_write_encrypted_server(w, e).await,
            Self::SMSG_STABLE_RESULT(c) => c.tokio_write_encrypted_server(w, e).await,
            Self::SMSG_PLAY_OBJECT_SOUND(c) => c.tokio_write_encrypted_server(w, e).await,
            Self::MSG_QUERY_NEXT_MAIL_TIME(c) => c.tokio_write_encrypted_server(w, e).await,
            Self::SMSG_RECEIVED_MAIL(c) => c.tokio_write_encrypted_server(w, e).await,
            Self::SMSG_PVP_CREDIT(c) => c.tokio_write_encrypted_server(w, e).await,
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
            Self::SMSG_PET_BROKEN(c) => c.tokio_write_encrypted_server(w, e).await,
            Self::SMSG_DUEL_COUNTDOWN(c) => c.tokio_write_encrypted_server(w, e).await,
            Self::SMSG_AREA_TRIGGER_MESSAGE(c) => c.tokio_write_encrypted_server(w, e).await,
            Self::SMSG_DURABILITY_DAMAGE_DEATH(c) => c.tokio_write_encrypted_server(w, e).await,
            Self::SMSG_INIT_WORLD_STATES(c) => c.tokio_write_encrypted_server(w, e).await,
            Self::SMSG_UPDATE_WORLD_STATE(c) => c.tokio_write_encrypted_server(w, e).await,
            Self::SMSG_ITEM_NAME_QUERY_RESPONSE(c) => c.tokio_write_encrypted_server(w, e).await,
            Self::SMSG_PET_ACTION_FEEDBACK(c) => c.tokio_write_encrypted_server(w, e).await,
            Self::SMSG_CHAR_RENAME(c) => c.tokio_write_encrypted_server(w, e).await,
            Self::SMSG_RAID_INSTANCE_INFO(c) => c.tokio_write_encrypted_server(w, e).await,
            Self::MSG_INSPECT_HONOR_STATS(c) => c.tokio_write_encrypted_server(w, e).await,
            Self::SMSG_FORCE_WALK_SPEED_CHANGE(c) => c.tokio_write_encrypted_server(w, e).await,
            Self::SMSG_FORCE_SWIM_BACK_SPEED_CHANGE(c) => c.tokio_write_encrypted_server(w, e).await,
            Self::SMSG_FORCE_TURN_RATE_CHANGE(c) => c.tokio_write_encrypted_server(w, e).await,
            Self::MSG_BATTLEGROUND_PLAYER_POSITIONS(c) => c.tokio_write_encrypted_server(w, e).await,
            Self::SMSG_BINDER_CONFIRM(c) => c.tokio_write_encrypted_server(w, e).await,
            Self::SMSG_BATTLEGROUND_PLAYER_JOINED(c) => c.tokio_write_encrypted_server(w, e).await,
            Self::SMSG_BATTLEGROUND_PLAYER_LEFT(c) => c.tokio_write_encrypted_server(w, e).await,
            Self::SMSG_ADDON_INFO(c) => c.tokio_write_encrypted_server(w, e).await,
            Self::SMSG_PET_UNLEARN_CONFIRM(c) => c.tokio_write_encrypted_server(w, e).await,
            Self::SMSG_CHAT_RESTRICTED(c) => c.tokio_write_encrypted_server(w, e).await,
            Self::SMSG_SPLINE_SET_RUN_SPEED(c) => c.tokio_write_encrypted_server(w, e).await,
            Self::SMSG_SPLINE_SET_RUN_BACK_SPEED(c) => c.tokio_write_encrypted_server(w, e).await,
            Self::SMSG_SPLINE_SET_SWIM_SPEED(c) => c.tokio_write_encrypted_server(w, e).await,
            Self::SMSG_SPLINE_SET_WALK_SPEED(c) => c.tokio_write_encrypted_server(w, e).await,
            Self::SMSG_SPLINE_SET_SWIM_BACK_SPEED(c) => c.tokio_write_encrypted_server(w, e).await,
            Self::SMSG_SPLINE_SET_TURN_RATE(c) => c.tokio_write_encrypted_server(w, e).await,
            Self::SMSG_SPLINE_MOVE_UNROOT(c) => c.tokio_write_encrypted_server(w, e).await,
            Self::SMSG_SPLINE_MOVE_FEATHER_FALL(c) => c.tokio_write_encrypted_server(w, e).await,
            Self::SMSG_SPLINE_MOVE_NORMAL_FALL(c) => c.tokio_write_encrypted_server(w, e).await,
            Self::SMSG_SPLINE_MOVE_SET_HOVER(c) => c.tokio_write_encrypted_server(w, e).await,
            Self::SMSG_SPLINE_MOVE_UNSET_HOVER(c) => c.tokio_write_encrypted_server(w, e).await,
            Self::SMSG_SPLINE_MOVE_WATER_WALK(c) => c.tokio_write_encrypted_server(w, e).await,
            Self::SMSG_SPLINE_MOVE_LAND_WALK(c) => c.tokio_write_encrypted_server(w, e).await,
            Self::SMSG_SPLINE_MOVE_START_SWIM(c) => c.tokio_write_encrypted_server(w, e).await,
            Self::SMSG_SPLINE_MOVE_STOP_SWIM(c) => c.tokio_write_encrypted_server(w, e).await,
            Self::SMSG_SPLINE_MOVE_SET_RUN_MODE(c) => c.tokio_write_encrypted_server(w, e).await,
            Self::SMSG_SPLINE_MOVE_SET_WALK_MODE(c) => c.tokio_write_encrypted_server(w, e).await,
            Self::SMSG_SPLINE_MOVE_ROOT(c) => c.tokio_write_encrypted_server(w, e).await,
            Self::MSG_RAID_READY_CHECK(c) => c.tokio_write_encrypted_server(w, e).await,
            Self::SMSG_PET_ACTION_SOUND(c) => c.tokio_write_encrypted_server(w, e).await,
            Self::SMSG_PET_DISMISS_SOUND(c) => c.tokio_write_encrypted_server(w, e).await,
            Self::SMSG_GM_TICKET_STATUS_UPDATE(c) => c.tokio_write_encrypted_server(w, e).await,
            Self::SMSG_DEFENSE_MESSAGE(c) => c.tokio_write_encrypted_server(w, e).await,
            Self::SMSG_REALM_SPLIT(c) => c.tokio_write_encrypted_server(w, e).await,
            Self::SMSG_TIME_SYNC_REQ(c) => c.tokio_write_encrypted_server(w, e).await,
            Self::SMSG_GM_MESSAGECHAT(c) => c.tokio_write_encrypted_server(w, e).await,
            Self::SMSG_FEATURE_SYSTEM_STATUS(c) => c.tokio_write_encrypted_server(w, e).await,
            Self::SMSG_CALENDAR_SEND_NUM_PENDING(c) => c.tokio_write_encrypted_server(w, e).await,
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
            Self::MSG_QUEST_PUSH_RESULT(c) => c.tokio_write_unencrypted_server(w).await,
            Self::MSG_PETITION_RENAME(c) => c.tokio_write_unencrypted_server(w).await,
            Self::MSG_SET_DUNGEON_DIFFICULTY(c) => c.tokio_write_unencrypted_server(w).await,
            Self::MSG_MOVE_START_ASCEND(c) => c.tokio_write_unencrypted_server(w).await,
            Self::MSG_MOVE_STOP_ASCEND(c) => c.tokio_write_unencrypted_server(w).await,
            Self::MSG_MOVE_START_DESCEND(c) => c.tokio_write_unencrypted_server(w).await,
            Self::MSG_GUILD_BANK_MONEY_WITHDRAWN(c) => c.tokio_write_unencrypted_server(w).await,
            Self::CMSG_CALENDAR_GET_NUM_PENDING(c) => c.tokio_write_unencrypted_server(w).await,
            Self::SMSG_CHAR_CREATE(c) => c.tokio_write_unencrypted_server(w).await,
            Self::SMSG_CHAR_ENUM(c) => c.tokio_write_unencrypted_server(w).await,
            Self::SMSG_CHAR_DELETE(c) => c.tokio_write_unencrypted_server(w).await,
            Self::SMSG_NEW_WORLD(c) => c.tokio_write_unencrypted_server(w).await,
            Self::SMSG_TRANSFER_PENDING(c) => c.tokio_write_unencrypted_server(w).await,
            Self::SMSG_TRANSFER_ABORTED(c) => c.tokio_write_unencrypted_server(w).await,
            Self::SMSG_CHARACTER_LOGIN_FAILED(c) => c.tokio_write_unencrypted_server(w).await,
            Self::SMSG_LOGIN_SETTIMESPEED(c) => c.tokio_write_unencrypted_server(w).await,
            Self::SMSG_LOGOUT_RESPONSE(c) => c.tokio_write_unencrypted_server(w).await,
            Self::SMSG_LOGOUT_COMPLETE(c) => c.tokio_write_unencrypted_server(w).await,
            Self::SMSG_LOGOUT_CANCEL_ACK(c) => c.tokio_write_unencrypted_server(w).await,
            Self::SMSG_NAME_QUERY_RESPONSE(c) => c.tokio_write_unencrypted_server(w).await,
            Self::SMSG_PET_NAME_QUERY_RESPONSE(c) => c.tokio_write_unencrypted_server(w).await,
            Self::SMSG_GUILD_QUERY_RESPONSE(c) => c.tokio_write_unencrypted_server(w).await,
            Self::SMSG_ITEM_QUERY_SINGLE_RESPONSE(c) => c.tokio_write_unencrypted_server(w).await,
            Self::SMSG_PAGE_TEXT_QUERY_RESPONSE(c) => c.tokio_write_unencrypted_server(w).await,
            Self::SMSG_QUEST_QUERY_RESPONSE(c) => c.tokio_write_unencrypted_server(w).await,
            Self::SMSG_GAMEOBJECT_QUERY_RESPONSE(c) => c.tokio_write_unencrypted_server(w).await,
            Self::SMSG_CREATURE_QUERY_RESPONSE(c) => c.tokio_write_unencrypted_server(w).await,
            Self::SMSG_WHO(c) => c.tokio_write_unencrypted_server(w).await,
            Self::SMSG_WHOIS(c) => c.tokio_write_unencrypted_server(w).await,
            Self::SMSG_CONTACT_LIST(c) => c.tokio_write_unencrypted_server(w).await,
            Self::SMSG_FRIEND_STATUS(c) => c.tokio_write_unencrypted_server(w).await,
            Self::SMSG_GROUP_INVITE(c) => c.tokio_write_unencrypted_server(w).await,
            Self::SMSG_GROUP_DECLINE(c) => c.tokio_write_unencrypted_server(w).await,
            Self::SMSG_GROUP_UNINVITE(c) => c.tokio_write_unencrypted_server(w).await,
            Self::SMSG_GROUP_SET_LEADER(c) => c.tokio_write_unencrypted_server(w).await,
            Self::SMSG_GROUP_DESTROYED(c) => c.tokio_write_unencrypted_server(w).await,
            Self::SMSG_GROUP_LIST(c) => c.tokio_write_unencrypted_server(w).await,
            Self::SMSG_PARTY_COMMAND_RESULT(c) => c.tokio_write_unencrypted_server(w).await,
            Self::SMSG_GUILD_INVITE(c) => c.tokio_write_unencrypted_server(w).await,
            Self::SMSG_GUILD_INFO(c) => c.tokio_write_unencrypted_server(w).await,
            Self::SMSG_GUILD_ROSTER(c) => c.tokio_write_unencrypted_server(w).await,
            Self::SMSG_GUILD_EVENT(c) => c.tokio_write_unencrypted_server(w).await,
            Self::SMSG_GUILD_COMMAND_RESULT(c) => c.tokio_write_unencrypted_server(w).await,
            Self::SMSG_MESSAGECHAT(c) => c.tokio_write_unencrypted_server(w).await,
            Self::SMSG_CHANNEL_NOTIFY(c) => c.tokio_write_unencrypted_server(w).await,
            Self::SMSG_CHANNEL_LIST(c) => c.tokio_write_unencrypted_server(w).await,
            Self::SMSG_UPDATE_OBJECT(c) => c.tokio_write_unencrypted_server(w).await,
            Self::SMSG_DESTROY_OBJECT(c) => c.tokio_write_unencrypted_server(w).await,
            Self::SMSG_READ_ITEM_OK(c) => c.tokio_write_unencrypted_server(w).await,
            Self::SMSG_READ_ITEM_FAILED(c) => c.tokio_write_unencrypted_server(w).await,
            Self::SMSG_ITEM_COOLDOWN(c) => c.tokio_write_unencrypted_server(w).await,
            Self::SMSG_GAMEOBJECT_CUSTOM_ANIM(c) => c.tokio_write_unencrypted_server(w).await,
            Self::MSG_MOVE_TELEPORT_ACK(c) => c.tokio_write_unencrypted_server(w).await,
            Self::SMSG_MOVE_WATER_WALK(c) => c.tokio_write_unencrypted_server(w).await,
            Self::SMSG_MOVE_LAND_WALK(c) => c.tokio_write_unencrypted_server(w).await,
            Self::SMSG_FORCE_RUN_SPEED_CHANGE(c) => c.tokio_write_unencrypted_server(w).await,
            Self::SMSG_FORCE_RUN_BACK_SPEED_CHANGE(c) => c.tokio_write_unencrypted_server(w).await,
            Self::SMSG_FORCE_SWIM_SPEED_CHANGE(c) => c.tokio_write_unencrypted_server(w).await,
            Self::SMSG_FORCE_MOVE_ROOT(c) => c.tokio_write_unencrypted_server(w).await,
            Self::SMSG_FORCE_MOVE_UNROOT(c) => c.tokio_write_unencrypted_server(w).await,
            Self::SMSG_MOVE_KNOCK_BACK(c) => c.tokio_write_unencrypted_server(w).await,
            Self::SMSG_MOVE_FEATHER_FALL(c) => c.tokio_write_unencrypted_server(w).await,
            Self::SMSG_MOVE_NORMAL_FALL(c) => c.tokio_write_unencrypted_server(w).await,
            Self::SMSG_MOVE_SET_HOVER(c) => c.tokio_write_unencrypted_server(w).await,
            Self::SMSG_MOVE_UNSET_HOVER(c) => c.tokio_write_unencrypted_server(w).await,
            Self::SMSG_TRIGGER_CINEMATIC(c) => c.tokio_write_unencrypted_server(w).await,
            Self::SMSG_TUTORIAL_FLAGS(c) => c.tokio_write_unencrypted_server(w).await,
            Self::SMSG_EMOTE(c) => c.tokio_write_unencrypted_server(w).await,
            Self::SMSG_TEXT_EMOTE(c) => c.tokio_write_unencrypted_server(w).await,
            Self::SMSG_INVENTORY_CHANGE_FAILURE(c) => c.tokio_write_unencrypted_server(w).await,
            Self::SMSG_TRADE_STATUS(c) => c.tokio_write_unencrypted_server(w).await,
            Self::SMSG_TRADE_STATUS_EXTENDED(c) => c.tokio_write_unencrypted_server(w).await,
            Self::SMSG_INITIALIZE_FACTIONS(c) => c.tokio_write_unencrypted_server(w).await,
            Self::SMSG_SET_FACTION_VISIBLE(c) => c.tokio_write_unencrypted_server(w).await,
            Self::SMSG_SET_FACTION_STANDING(c) => c.tokio_write_unencrypted_server(w).await,
            Self::SMSG_SET_PROFICIENCY(c) => c.tokio_write_unencrypted_server(w).await,
            Self::SMSG_ACTION_BUTTONS(c) => c.tokio_write_unencrypted_server(w).await,
            Self::SMSG_INITIAL_SPELLS(c) => c.tokio_write_unencrypted_server(w).await,
            Self::SMSG_LEARNED_SPELL(c) => c.tokio_write_unencrypted_server(w).await,
            Self::SMSG_SUPERCEDED_SPELL(c) => c.tokio_write_unencrypted_server(w).await,
            Self::SMSG_SPELL_START(c) => c.tokio_write_unencrypted_server(w).await,
            Self::SMSG_SPELL_GO(c) => c.tokio_write_unencrypted_server(w).await,
            Self::SMSG_SPELL_FAILURE(c) => c.tokio_write_unencrypted_server(w).await,
            Self::SMSG_SPELL_COOLDOWN(c) => c.tokio_write_unencrypted_server(w).await,
            Self::SMSG_COOLDOWN_EVENT(c) => c.tokio_write_unencrypted_server(w).await,
            Self::SMSG_PET_CAST_FAILED(c) => c.tokio_write_unencrypted_server(w).await,
            Self::SMSG_AI_REACTION(c) => c.tokio_write_unencrypted_server(w).await,
            Self::SMSG_ATTACKSTART(c) => c.tokio_write_unencrypted_server(w).await,
            Self::SMSG_ATTACKSTOP(c) => c.tokio_write_unencrypted_server(w).await,
            Self::SMSG_ATTACKSWING_NOTINRANGE(c) => c.tokio_write_unencrypted_server(w).await,
            Self::SMSG_ATTACKSWING_BADFACING(c) => c.tokio_write_unencrypted_server(w).await,
            Self::SMSG_ATTACKSWING_DEADTARGET(c) => c.tokio_write_unencrypted_server(w).await,
            Self::SMSG_ATTACKSWING_CANT_ATTACK(c) => c.tokio_write_unencrypted_server(w).await,
            Self::SMSG_ATTACKERSTATEUPDATE(c) => c.tokio_write_unencrypted_server(w).await,
            Self::SMSG_CANCEL_COMBAT(c) => c.tokio_write_unencrypted_server(w).await,
            Self::SMSG_SPELLHEALLOG(c) => c.tokio_write_unencrypted_server(w).await,
            Self::SMSG_SPELLENERGIZELOG(c) => c.tokio_write_unencrypted_server(w).await,
            Self::SMSG_BINDPOINTUPDATE(c) => c.tokio_write_unencrypted_server(w).await,
            Self::SMSG_PLAYERBOUND(c) => c.tokio_write_unencrypted_server(w).await,
            Self::SMSG_CLIENT_CONTROL_UPDATE(c) => c.tokio_write_unencrypted_server(w).await,
            Self::SMSG_RESURRECT_REQUEST(c) => c.tokio_write_unencrypted_server(w).await,
            Self::SMSG_LOOT_RESPONSE(c) => c.tokio_write_unencrypted_server(w).await,
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
            Self::SMSG_PET_TAME_FAILURE(c) => c.tokio_write_unencrypted_server(w).await,
            Self::SMSG_PET_NAME_INVALID(c) => c.tokio_write_unencrypted_server(w).await,
            Self::SMSG_PET_SPELLS(c) => c.tokio_write_unencrypted_server(w).await,
            Self::SMSG_PET_MODE(c) => c.tokio_write_unencrypted_server(w).await,
            Self::SMSG_GOSSIP_MESSAGE(c) => c.tokio_write_unencrypted_server(w).await,
            Self::SMSG_GOSSIP_COMPLETE(c) => c.tokio_write_unencrypted_server(w).await,
            Self::SMSG_NPC_TEXT_UPDATE(c) => c.tokio_write_unencrypted_server(w).await,
            Self::SMSG_QUESTGIVER_STATUS(c) => c.tokio_write_unencrypted_server(w).await,
            Self::SMSG_QUESTGIVER_QUEST_LIST(c) => c.tokio_write_unencrypted_server(w).await,
            Self::SMSG_QUESTGIVER_QUEST_DETAILS(c) => c.tokio_write_unencrypted_server(w).await,
            Self::SMSG_QUESTGIVER_REQUEST_ITEMS(c) => c.tokio_write_unencrypted_server(w).await,
            Self::SMSG_QUESTGIVER_OFFER_REWARD(c) => c.tokio_write_unencrypted_server(w).await,
            Self::SMSG_QUESTGIVER_QUEST_INVALID(c) => c.tokio_write_unencrypted_server(w).await,
            Self::SMSG_QUESTGIVER_QUEST_COMPLETE(c) => c.tokio_write_unencrypted_server(w).await,
            Self::SMSG_QUESTGIVER_QUEST_FAILED(c) => c.tokio_write_unencrypted_server(w).await,
            Self::SMSG_QUESTLOG_FULL(c) => c.tokio_write_unencrypted_server(w).await,
            Self::SMSG_QUESTUPDATE_FAILED(c) => c.tokio_write_unencrypted_server(w).await,
            Self::SMSG_QUESTUPDATE_FAILEDTIMER(c) => c.tokio_write_unencrypted_server(w).await,
            Self::SMSG_QUESTUPDATE_COMPLETE(c) => c.tokio_write_unencrypted_server(w).await,
            Self::SMSG_QUESTUPDATE_ADD_KILL(c) => c.tokio_write_unencrypted_server(w).await,
            Self::SMSG_QUESTUPDATE_ADD_ITEM(c) => c.tokio_write_unencrypted_server(w).await,
            Self::SMSG_QUEST_CONFIRM_ACCEPT(c) => c.tokio_write_unencrypted_server(w).await,
            Self::SMSG_LIST_INVENTORY(c) => c.tokio_write_unencrypted_server(w).await,
            Self::SMSG_SELL_ITEM(c) => c.tokio_write_unencrypted_server(w).await,
            Self::SMSG_BUY_ITEM(c) => c.tokio_write_unencrypted_server(w).await,
            Self::SMSG_BUY_FAILED(c) => c.tokio_write_unencrypted_server(w).await,
            Self::SMSG_SHOWTAXINODES(c) => c.tokio_write_unencrypted_server(w).await,
            Self::SMSG_TAXINODE_STATUS(c) => c.tokio_write_unencrypted_server(w).await,
            Self::SMSG_ACTIVATETAXIREPLY(c) => c.tokio_write_unencrypted_server(w).await,
            Self::SMSG_NEW_TAXI_PATH(c) => c.tokio_write_unencrypted_server(w).await,
            Self::SMSG_TRAINER_LIST(c) => c.tokio_write_unencrypted_server(w).await,
            Self::SMSG_TRAINER_BUY_SUCCEEDED(c) => c.tokio_write_unencrypted_server(w).await,
            Self::SMSG_TRAINER_BUY_FAILED(c) => c.tokio_write_unencrypted_server(w).await,
            Self::SMSG_SHOW_BANK(c) => c.tokio_write_unencrypted_server(w).await,
            Self::SMSG_BUY_BANK_SLOT_RESULT(c) => c.tokio_write_unencrypted_server(w).await,
            Self::SMSG_PETITION_SHOWLIST(c) => c.tokio_write_unencrypted_server(w).await,
            Self::SMSG_PETITION_SHOW_SIGNATURES(c) => c.tokio_write_unencrypted_server(w).await,
            Self::SMSG_PETITION_SIGN_RESULTS(c) => c.tokio_write_unencrypted_server(w).await,
            Self::SMSG_TURN_IN_PETITION_RESULTS(c) => c.tokio_write_unencrypted_server(w).await,
            Self::SMSG_PETITION_QUERY_RESPONSE(c) => c.tokio_write_unencrypted_server(w).await,
            Self::SMSG_FISH_NOT_HOOKED(c) => c.tokio_write_unencrypted_server(w).await,
            Self::SMSG_FISH_ESCAPED(c) => c.tokio_write_unencrypted_server(w).await,
            Self::SMSG_NOTIFICATION(c) => c.tokio_write_unencrypted_server(w).await,
            Self::SMSG_PLAYED_TIME(c) => c.tokio_write_unencrypted_server(w).await,
            Self::SMSG_QUERY_TIME_RESPONSE(c) => c.tokio_write_unencrypted_server(w).await,
            Self::SMSG_LOG_XPGAIN(c) => c.tokio_write_unencrypted_server(w).await,
            Self::SMSG_LEVELUP_INFO(c) => c.tokio_write_unencrypted_server(w).await,
            Self::MSG_MINIMAP_PING(c) => c.tokio_write_unencrypted_server(w).await,
            Self::SMSG_PONG(c) => c.tokio_write_unencrypted_server(w).await,
            Self::SMSG_GAMEOBJECT_PAGETEXT(c) => c.tokio_write_unencrypted_server(w).await,
            Self::SMSG_ITEM_TIME_UPDATE(c) => c.tokio_write_unencrypted_server(w).await,
            Self::SMSG_ITEM_ENCHANT_TIME_UPDATE(c) => c.tokio_write_unencrypted_server(w).await,
            Self::SMSG_AUTH_CHALLENGE(c) => c.tokio_write_unencrypted_server(w).await,
            Self::SMSG_AUTH_RESPONSE(c) => c.tokio_write_unencrypted_server(w).await,
            Self::MSG_SAVE_GUILD_EMBLEM(c) => c.tokio_write_unencrypted_server(w).await,
            Self::SMSG_EXPLORATION_EXPERIENCE(c) => c.tokio_write_unencrypted_server(w).await,
            Self::MSG_RANDOM_ROLL(c) => c.tokio_write_unencrypted_server(w).await,
            Self::SMSG_ENVIRONMENTAL_DAMAGE_LOG(c) => c.tokio_write_unencrypted_server(w).await,
            Self::SMSG_GMTICKET_CREATE(c) => c.tokio_write_unencrypted_server(w).await,
            Self::SMSG_GMTICKET_UPDATETEXT(c) => c.tokio_write_unencrypted_server(w).await,
            Self::SMSG_ACCOUNT_DATA_TIMES(c) => c.tokio_write_unencrypted_server(w).await,
            Self::SMSG_UPDATE_ACCOUNT_DATA(c) => c.tokio_write_unencrypted_server(w).await,
            Self::SMSG_GMTICKET_GETTICKET(c) => c.tokio_write_unencrypted_server(w).await,
            Self::SMSG_GAMEOBJECT_DESPAWN_ANIM(c) => c.tokio_write_unencrypted_server(w).await,
            Self::MSG_CORPSE_QUERY(c) => c.tokio_write_unencrypted_server(w).await,
            Self::SMSG_GMTICKET_DELETETICKET(c) => c.tokio_write_unencrypted_server(w).await,
            Self::SMSG_CHAT_WRONG_FACTION(c) => c.tokio_write_unencrypted_server(w).await,
            Self::SMSG_GMTICKET_SYSTEMSTATUS(c) => c.tokio_write_unencrypted_server(w).await,
            Self::SMSG_QUEST_FORCE_REMOVE(c) => c.tokio_write_unencrypted_server(w).await,
            Self::SMSG_GOSSIP_POI(c) => c.tokio_write_unencrypted_server(w).await,
            Self::SMSG_LOGIN_VERIFY_WORLD(c) => c.tokio_write_unencrypted_server(w).await,
            Self::SMSG_BATTLEFIELD_LIST(c) => c.tokio_write_unencrypted_server(w).await,
            Self::SMSG_ITEM_TEXT_QUERY_RESPONSE(c) => c.tokio_write_unencrypted_server(w).await,
            Self::SMSG_ZONE_UNDER_ATTACK(c) => c.tokio_write_unencrypted_server(w).await,
            Self::MSG_AUCTION_HELLO(c) => c.tokio_write_unencrypted_server(w).await,
            Self::SMSG_AUCTION_LIST_RESULT(c) => c.tokio_write_unencrypted_server(w).await,
            Self::SMSG_AUCTION_OWNER_LIST_RESULT(c) => c.tokio_write_unencrypted_server(w).await,
            Self::SMSG_AUCTION_BIDDER_NOTIFICATION(c) => c.tokio_write_unencrypted_server(w).await,
            Self::SMSG_AUCTION_OWNER_NOTIFICATION(c) => c.tokio_write_unencrypted_server(w).await,
            Self::SMSG_PROCRESIST(c) => c.tokio_write_unencrypted_server(w).await,
            Self::SMSG_AUCTION_BIDDER_LIST_RESULT(c) => c.tokio_write_unencrypted_server(w).await,
            Self::MSG_LIST_STABLED_PETS(c) => c.tokio_write_unencrypted_server(w).await,
            Self::SMSG_STABLE_RESULT(c) => c.tokio_write_unencrypted_server(w).await,
            Self::SMSG_PLAY_OBJECT_SOUND(c) => c.tokio_write_unencrypted_server(w).await,
            Self::MSG_QUERY_NEXT_MAIL_TIME(c) => c.tokio_write_unencrypted_server(w).await,
            Self::SMSG_RECEIVED_MAIL(c) => c.tokio_write_unencrypted_server(w).await,
            Self::SMSG_PVP_CREDIT(c) => c.tokio_write_unencrypted_server(w).await,
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
            Self::SMSG_PET_BROKEN(c) => c.tokio_write_unencrypted_server(w).await,
            Self::SMSG_DUEL_COUNTDOWN(c) => c.tokio_write_unencrypted_server(w).await,
            Self::SMSG_AREA_TRIGGER_MESSAGE(c) => c.tokio_write_unencrypted_server(w).await,
            Self::SMSG_DURABILITY_DAMAGE_DEATH(c) => c.tokio_write_unencrypted_server(w).await,
            Self::SMSG_INIT_WORLD_STATES(c) => c.tokio_write_unencrypted_server(w).await,
            Self::SMSG_UPDATE_WORLD_STATE(c) => c.tokio_write_unencrypted_server(w).await,
            Self::SMSG_ITEM_NAME_QUERY_RESPONSE(c) => c.tokio_write_unencrypted_server(w).await,
            Self::SMSG_PET_ACTION_FEEDBACK(c) => c.tokio_write_unencrypted_server(w).await,
            Self::SMSG_CHAR_RENAME(c) => c.tokio_write_unencrypted_server(w).await,
            Self::SMSG_RAID_INSTANCE_INFO(c) => c.tokio_write_unencrypted_server(w).await,
            Self::MSG_INSPECT_HONOR_STATS(c) => c.tokio_write_unencrypted_server(w).await,
            Self::SMSG_FORCE_WALK_SPEED_CHANGE(c) => c.tokio_write_unencrypted_server(w).await,
            Self::SMSG_FORCE_SWIM_BACK_SPEED_CHANGE(c) => c.tokio_write_unencrypted_server(w).await,
            Self::SMSG_FORCE_TURN_RATE_CHANGE(c) => c.tokio_write_unencrypted_server(w).await,
            Self::MSG_BATTLEGROUND_PLAYER_POSITIONS(c) => c.tokio_write_unencrypted_server(w).await,
            Self::SMSG_BINDER_CONFIRM(c) => c.tokio_write_unencrypted_server(w).await,
            Self::SMSG_BATTLEGROUND_PLAYER_JOINED(c) => c.tokio_write_unencrypted_server(w).await,
            Self::SMSG_BATTLEGROUND_PLAYER_LEFT(c) => c.tokio_write_unencrypted_server(w).await,
            Self::SMSG_ADDON_INFO(c) => c.tokio_write_unencrypted_server(w).await,
            Self::SMSG_PET_UNLEARN_CONFIRM(c) => c.tokio_write_unencrypted_server(w).await,
            Self::SMSG_CHAT_RESTRICTED(c) => c.tokio_write_unencrypted_server(w).await,
            Self::SMSG_SPLINE_SET_RUN_SPEED(c) => c.tokio_write_unencrypted_server(w).await,
            Self::SMSG_SPLINE_SET_RUN_BACK_SPEED(c) => c.tokio_write_unencrypted_server(w).await,
            Self::SMSG_SPLINE_SET_SWIM_SPEED(c) => c.tokio_write_unencrypted_server(w).await,
            Self::SMSG_SPLINE_SET_WALK_SPEED(c) => c.tokio_write_unencrypted_server(w).await,
            Self::SMSG_SPLINE_SET_SWIM_BACK_SPEED(c) => c.tokio_write_unencrypted_server(w).await,
            Self::SMSG_SPLINE_SET_TURN_RATE(c) => c.tokio_write_unencrypted_server(w).await,
            Self::SMSG_SPLINE_MOVE_UNROOT(c) => c.tokio_write_unencrypted_server(w).await,
            Self::SMSG_SPLINE_MOVE_FEATHER_FALL(c) => c.tokio_write_unencrypted_server(w).await,
            Self::SMSG_SPLINE_MOVE_NORMAL_FALL(c) => c.tokio_write_unencrypted_server(w).await,
            Self::SMSG_SPLINE_MOVE_SET_HOVER(c) => c.tokio_write_unencrypted_server(w).await,
            Self::SMSG_SPLINE_MOVE_UNSET_HOVER(c) => c.tokio_write_unencrypted_server(w).await,
            Self::SMSG_SPLINE_MOVE_WATER_WALK(c) => c.tokio_write_unencrypted_server(w).await,
            Self::SMSG_SPLINE_MOVE_LAND_WALK(c) => c.tokio_write_unencrypted_server(w).await,
            Self::SMSG_SPLINE_MOVE_START_SWIM(c) => c.tokio_write_unencrypted_server(w).await,
            Self::SMSG_SPLINE_MOVE_STOP_SWIM(c) => c.tokio_write_unencrypted_server(w).await,
            Self::SMSG_SPLINE_MOVE_SET_RUN_MODE(c) => c.tokio_write_unencrypted_server(w).await,
            Self::SMSG_SPLINE_MOVE_SET_WALK_MODE(c) => c.tokio_write_unencrypted_server(w).await,
            Self::SMSG_SPLINE_MOVE_ROOT(c) => c.tokio_write_unencrypted_server(w).await,
            Self::MSG_RAID_READY_CHECK(c) => c.tokio_write_unencrypted_server(w).await,
            Self::SMSG_PET_ACTION_SOUND(c) => c.tokio_write_unencrypted_server(w).await,
            Self::SMSG_PET_DISMISS_SOUND(c) => c.tokio_write_unencrypted_server(w).await,
            Self::SMSG_GM_TICKET_STATUS_UPDATE(c) => c.tokio_write_unencrypted_server(w).await,
            Self::SMSG_DEFENSE_MESSAGE(c) => c.tokio_write_unencrypted_server(w).await,
            Self::SMSG_REALM_SPLIT(c) => c.tokio_write_unencrypted_server(w).await,
            Self::SMSG_TIME_SYNC_REQ(c) => c.tokio_write_unencrypted_server(w).await,
            Self::SMSG_GM_MESSAGECHAT(c) => c.tokio_write_unencrypted_server(w).await,
            Self::SMSG_FEATURE_SYSTEM_STATUS(c) => c.tokio_write_unencrypted_server(w).await,
            Self::SMSG_CALENDAR_SEND_NUM_PENDING(c) => c.tokio_write_unencrypted_server(w).await,
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
            Self::MSG_QUEST_PUSH_RESULT(c) => c.astd_write_encrypted_server(w, e).await,
            Self::MSG_PETITION_RENAME(c) => c.astd_write_encrypted_server(w, e).await,
            Self::MSG_SET_DUNGEON_DIFFICULTY(c) => c.astd_write_encrypted_server(w, e).await,
            Self::MSG_MOVE_START_ASCEND(c) => c.astd_write_encrypted_server(w, e).await,
            Self::MSG_MOVE_STOP_ASCEND(c) => c.astd_write_encrypted_server(w, e).await,
            Self::MSG_MOVE_START_DESCEND(c) => c.astd_write_encrypted_server(w, e).await,
            Self::MSG_GUILD_BANK_MONEY_WITHDRAWN(c) => c.astd_write_encrypted_server(w, e).await,
            Self::CMSG_CALENDAR_GET_NUM_PENDING(c) => c.astd_write_encrypted_server(w, e).await,
            Self::SMSG_CHAR_CREATE(c) => c.astd_write_encrypted_server(w, e).await,
            Self::SMSG_CHAR_ENUM(c) => c.astd_write_encrypted_server(w, e).await,
            Self::SMSG_CHAR_DELETE(c) => c.astd_write_encrypted_server(w, e).await,
            Self::SMSG_NEW_WORLD(c) => c.astd_write_encrypted_server(w, e).await,
            Self::SMSG_TRANSFER_PENDING(c) => c.astd_write_encrypted_server(w, e).await,
            Self::SMSG_TRANSFER_ABORTED(c) => c.astd_write_encrypted_server(w, e).await,
            Self::SMSG_CHARACTER_LOGIN_FAILED(c) => c.astd_write_encrypted_server(w, e).await,
            Self::SMSG_LOGIN_SETTIMESPEED(c) => c.astd_write_encrypted_server(w, e).await,
            Self::SMSG_LOGOUT_RESPONSE(c) => c.astd_write_encrypted_server(w, e).await,
            Self::SMSG_LOGOUT_COMPLETE(c) => c.astd_write_encrypted_server(w, e).await,
            Self::SMSG_LOGOUT_CANCEL_ACK(c) => c.astd_write_encrypted_server(w, e).await,
            Self::SMSG_NAME_QUERY_RESPONSE(c) => c.astd_write_encrypted_server(w, e).await,
            Self::SMSG_PET_NAME_QUERY_RESPONSE(c) => c.astd_write_encrypted_server(w, e).await,
            Self::SMSG_GUILD_QUERY_RESPONSE(c) => c.astd_write_encrypted_server(w, e).await,
            Self::SMSG_ITEM_QUERY_SINGLE_RESPONSE(c) => c.astd_write_encrypted_server(w, e).await,
            Self::SMSG_PAGE_TEXT_QUERY_RESPONSE(c) => c.astd_write_encrypted_server(w, e).await,
            Self::SMSG_QUEST_QUERY_RESPONSE(c) => c.astd_write_encrypted_server(w, e).await,
            Self::SMSG_GAMEOBJECT_QUERY_RESPONSE(c) => c.astd_write_encrypted_server(w, e).await,
            Self::SMSG_CREATURE_QUERY_RESPONSE(c) => c.astd_write_encrypted_server(w, e).await,
            Self::SMSG_WHO(c) => c.astd_write_encrypted_server(w, e).await,
            Self::SMSG_WHOIS(c) => c.astd_write_encrypted_server(w, e).await,
            Self::SMSG_CONTACT_LIST(c) => c.astd_write_encrypted_server(w, e).await,
            Self::SMSG_FRIEND_STATUS(c) => c.astd_write_encrypted_server(w, e).await,
            Self::SMSG_GROUP_INVITE(c) => c.astd_write_encrypted_server(w, e).await,
            Self::SMSG_GROUP_DECLINE(c) => c.astd_write_encrypted_server(w, e).await,
            Self::SMSG_GROUP_UNINVITE(c) => c.astd_write_encrypted_server(w, e).await,
            Self::SMSG_GROUP_SET_LEADER(c) => c.astd_write_encrypted_server(w, e).await,
            Self::SMSG_GROUP_DESTROYED(c) => c.astd_write_encrypted_server(w, e).await,
            Self::SMSG_GROUP_LIST(c) => c.astd_write_encrypted_server(w, e).await,
            Self::SMSG_PARTY_COMMAND_RESULT(c) => c.astd_write_encrypted_server(w, e).await,
            Self::SMSG_GUILD_INVITE(c) => c.astd_write_encrypted_server(w, e).await,
            Self::SMSG_GUILD_INFO(c) => c.astd_write_encrypted_server(w, e).await,
            Self::SMSG_GUILD_ROSTER(c) => c.astd_write_encrypted_server(w, e).await,
            Self::SMSG_GUILD_EVENT(c) => c.astd_write_encrypted_server(w, e).await,
            Self::SMSG_GUILD_COMMAND_RESULT(c) => c.astd_write_encrypted_server(w, e).await,
            Self::SMSG_MESSAGECHAT(c) => c.astd_write_encrypted_server(w, e).await,
            Self::SMSG_CHANNEL_NOTIFY(c) => c.astd_write_encrypted_server(w, e).await,
            Self::SMSG_CHANNEL_LIST(c) => c.astd_write_encrypted_server(w, e).await,
            Self::SMSG_UPDATE_OBJECT(c) => c.astd_write_encrypted_server(w, e).await,
            Self::SMSG_DESTROY_OBJECT(c) => c.astd_write_encrypted_server(w, e).await,
            Self::SMSG_READ_ITEM_OK(c) => c.astd_write_encrypted_server(w, e).await,
            Self::SMSG_READ_ITEM_FAILED(c) => c.astd_write_encrypted_server(w, e).await,
            Self::SMSG_ITEM_COOLDOWN(c) => c.astd_write_encrypted_server(w, e).await,
            Self::SMSG_GAMEOBJECT_CUSTOM_ANIM(c) => c.astd_write_encrypted_server(w, e).await,
            Self::MSG_MOVE_TELEPORT_ACK(c) => c.astd_write_encrypted_server(w, e).await,
            Self::SMSG_MOVE_WATER_WALK(c) => c.astd_write_encrypted_server(w, e).await,
            Self::SMSG_MOVE_LAND_WALK(c) => c.astd_write_encrypted_server(w, e).await,
            Self::SMSG_FORCE_RUN_SPEED_CHANGE(c) => c.astd_write_encrypted_server(w, e).await,
            Self::SMSG_FORCE_RUN_BACK_SPEED_CHANGE(c) => c.astd_write_encrypted_server(w, e).await,
            Self::SMSG_FORCE_SWIM_SPEED_CHANGE(c) => c.astd_write_encrypted_server(w, e).await,
            Self::SMSG_FORCE_MOVE_ROOT(c) => c.astd_write_encrypted_server(w, e).await,
            Self::SMSG_FORCE_MOVE_UNROOT(c) => c.astd_write_encrypted_server(w, e).await,
            Self::SMSG_MOVE_KNOCK_BACK(c) => c.astd_write_encrypted_server(w, e).await,
            Self::SMSG_MOVE_FEATHER_FALL(c) => c.astd_write_encrypted_server(w, e).await,
            Self::SMSG_MOVE_NORMAL_FALL(c) => c.astd_write_encrypted_server(w, e).await,
            Self::SMSG_MOVE_SET_HOVER(c) => c.astd_write_encrypted_server(w, e).await,
            Self::SMSG_MOVE_UNSET_HOVER(c) => c.astd_write_encrypted_server(w, e).await,
            Self::SMSG_TRIGGER_CINEMATIC(c) => c.astd_write_encrypted_server(w, e).await,
            Self::SMSG_TUTORIAL_FLAGS(c) => c.astd_write_encrypted_server(w, e).await,
            Self::SMSG_EMOTE(c) => c.astd_write_encrypted_server(w, e).await,
            Self::SMSG_TEXT_EMOTE(c) => c.astd_write_encrypted_server(w, e).await,
            Self::SMSG_INVENTORY_CHANGE_FAILURE(c) => c.astd_write_encrypted_server(w, e).await,
            Self::SMSG_TRADE_STATUS(c) => c.astd_write_encrypted_server(w, e).await,
            Self::SMSG_TRADE_STATUS_EXTENDED(c) => c.astd_write_encrypted_server(w, e).await,
            Self::SMSG_INITIALIZE_FACTIONS(c) => c.astd_write_encrypted_server(w, e).await,
            Self::SMSG_SET_FACTION_VISIBLE(c) => c.astd_write_encrypted_server(w, e).await,
            Self::SMSG_SET_FACTION_STANDING(c) => c.astd_write_encrypted_server(w, e).await,
            Self::SMSG_SET_PROFICIENCY(c) => c.astd_write_encrypted_server(w, e).await,
            Self::SMSG_ACTION_BUTTONS(c) => c.astd_write_encrypted_server(w, e).await,
            Self::SMSG_INITIAL_SPELLS(c) => c.astd_write_encrypted_server(w, e).await,
            Self::SMSG_LEARNED_SPELL(c) => c.astd_write_encrypted_server(w, e).await,
            Self::SMSG_SUPERCEDED_SPELL(c) => c.astd_write_encrypted_server(w, e).await,
            Self::SMSG_SPELL_START(c) => c.astd_write_encrypted_server(w, e).await,
            Self::SMSG_SPELL_GO(c) => c.astd_write_encrypted_server(w, e).await,
            Self::SMSG_SPELL_FAILURE(c) => c.astd_write_encrypted_server(w, e).await,
            Self::SMSG_SPELL_COOLDOWN(c) => c.astd_write_encrypted_server(w, e).await,
            Self::SMSG_COOLDOWN_EVENT(c) => c.astd_write_encrypted_server(w, e).await,
            Self::SMSG_PET_CAST_FAILED(c) => c.astd_write_encrypted_server(w, e).await,
            Self::SMSG_AI_REACTION(c) => c.astd_write_encrypted_server(w, e).await,
            Self::SMSG_ATTACKSTART(c) => c.astd_write_encrypted_server(w, e).await,
            Self::SMSG_ATTACKSTOP(c) => c.astd_write_encrypted_server(w, e).await,
            Self::SMSG_ATTACKSWING_NOTINRANGE(c) => c.astd_write_encrypted_server(w, e).await,
            Self::SMSG_ATTACKSWING_BADFACING(c) => c.astd_write_encrypted_server(w, e).await,
            Self::SMSG_ATTACKSWING_DEADTARGET(c) => c.astd_write_encrypted_server(w, e).await,
            Self::SMSG_ATTACKSWING_CANT_ATTACK(c) => c.astd_write_encrypted_server(w, e).await,
            Self::SMSG_ATTACKERSTATEUPDATE(c) => c.astd_write_encrypted_server(w, e).await,
            Self::SMSG_CANCEL_COMBAT(c) => c.astd_write_encrypted_server(w, e).await,
            Self::SMSG_SPELLHEALLOG(c) => c.astd_write_encrypted_server(w, e).await,
            Self::SMSG_SPELLENERGIZELOG(c) => c.astd_write_encrypted_server(w, e).await,
            Self::SMSG_BINDPOINTUPDATE(c) => c.astd_write_encrypted_server(w, e).await,
            Self::SMSG_PLAYERBOUND(c) => c.astd_write_encrypted_server(w, e).await,
            Self::SMSG_CLIENT_CONTROL_UPDATE(c) => c.astd_write_encrypted_server(w, e).await,
            Self::SMSG_RESURRECT_REQUEST(c) => c.astd_write_encrypted_server(w, e).await,
            Self::SMSG_LOOT_RESPONSE(c) => c.astd_write_encrypted_server(w, e).await,
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
            Self::SMSG_PET_TAME_FAILURE(c) => c.astd_write_encrypted_server(w, e).await,
            Self::SMSG_PET_NAME_INVALID(c) => c.astd_write_encrypted_server(w, e).await,
            Self::SMSG_PET_SPELLS(c) => c.astd_write_encrypted_server(w, e).await,
            Self::SMSG_PET_MODE(c) => c.astd_write_encrypted_server(w, e).await,
            Self::SMSG_GOSSIP_MESSAGE(c) => c.astd_write_encrypted_server(w, e).await,
            Self::SMSG_GOSSIP_COMPLETE(c) => c.astd_write_encrypted_server(w, e).await,
            Self::SMSG_NPC_TEXT_UPDATE(c) => c.astd_write_encrypted_server(w, e).await,
            Self::SMSG_QUESTGIVER_STATUS(c) => c.astd_write_encrypted_server(w, e).await,
            Self::SMSG_QUESTGIVER_QUEST_LIST(c) => c.astd_write_encrypted_server(w, e).await,
            Self::SMSG_QUESTGIVER_QUEST_DETAILS(c) => c.astd_write_encrypted_server(w, e).await,
            Self::SMSG_QUESTGIVER_REQUEST_ITEMS(c) => c.astd_write_encrypted_server(w, e).await,
            Self::SMSG_QUESTGIVER_OFFER_REWARD(c) => c.astd_write_encrypted_server(w, e).await,
            Self::SMSG_QUESTGIVER_QUEST_INVALID(c) => c.astd_write_encrypted_server(w, e).await,
            Self::SMSG_QUESTGIVER_QUEST_COMPLETE(c) => c.astd_write_encrypted_server(w, e).await,
            Self::SMSG_QUESTGIVER_QUEST_FAILED(c) => c.astd_write_encrypted_server(w, e).await,
            Self::SMSG_QUESTLOG_FULL(c) => c.astd_write_encrypted_server(w, e).await,
            Self::SMSG_QUESTUPDATE_FAILED(c) => c.astd_write_encrypted_server(w, e).await,
            Self::SMSG_QUESTUPDATE_FAILEDTIMER(c) => c.astd_write_encrypted_server(w, e).await,
            Self::SMSG_QUESTUPDATE_COMPLETE(c) => c.astd_write_encrypted_server(w, e).await,
            Self::SMSG_QUESTUPDATE_ADD_KILL(c) => c.astd_write_encrypted_server(w, e).await,
            Self::SMSG_QUESTUPDATE_ADD_ITEM(c) => c.astd_write_encrypted_server(w, e).await,
            Self::SMSG_QUEST_CONFIRM_ACCEPT(c) => c.astd_write_encrypted_server(w, e).await,
            Self::SMSG_LIST_INVENTORY(c) => c.astd_write_encrypted_server(w, e).await,
            Self::SMSG_SELL_ITEM(c) => c.astd_write_encrypted_server(w, e).await,
            Self::SMSG_BUY_ITEM(c) => c.astd_write_encrypted_server(w, e).await,
            Self::SMSG_BUY_FAILED(c) => c.astd_write_encrypted_server(w, e).await,
            Self::SMSG_SHOWTAXINODES(c) => c.astd_write_encrypted_server(w, e).await,
            Self::SMSG_TAXINODE_STATUS(c) => c.astd_write_encrypted_server(w, e).await,
            Self::SMSG_ACTIVATETAXIREPLY(c) => c.astd_write_encrypted_server(w, e).await,
            Self::SMSG_NEW_TAXI_PATH(c) => c.astd_write_encrypted_server(w, e).await,
            Self::SMSG_TRAINER_LIST(c) => c.astd_write_encrypted_server(w, e).await,
            Self::SMSG_TRAINER_BUY_SUCCEEDED(c) => c.astd_write_encrypted_server(w, e).await,
            Self::SMSG_TRAINER_BUY_FAILED(c) => c.astd_write_encrypted_server(w, e).await,
            Self::SMSG_SHOW_BANK(c) => c.astd_write_encrypted_server(w, e).await,
            Self::SMSG_BUY_BANK_SLOT_RESULT(c) => c.astd_write_encrypted_server(w, e).await,
            Self::SMSG_PETITION_SHOWLIST(c) => c.astd_write_encrypted_server(w, e).await,
            Self::SMSG_PETITION_SHOW_SIGNATURES(c) => c.astd_write_encrypted_server(w, e).await,
            Self::SMSG_PETITION_SIGN_RESULTS(c) => c.astd_write_encrypted_server(w, e).await,
            Self::SMSG_TURN_IN_PETITION_RESULTS(c) => c.astd_write_encrypted_server(w, e).await,
            Self::SMSG_PETITION_QUERY_RESPONSE(c) => c.astd_write_encrypted_server(w, e).await,
            Self::SMSG_FISH_NOT_HOOKED(c) => c.astd_write_encrypted_server(w, e).await,
            Self::SMSG_FISH_ESCAPED(c) => c.astd_write_encrypted_server(w, e).await,
            Self::SMSG_NOTIFICATION(c) => c.astd_write_encrypted_server(w, e).await,
            Self::SMSG_PLAYED_TIME(c) => c.astd_write_encrypted_server(w, e).await,
            Self::SMSG_QUERY_TIME_RESPONSE(c) => c.astd_write_encrypted_server(w, e).await,
            Self::SMSG_LOG_XPGAIN(c) => c.astd_write_encrypted_server(w, e).await,
            Self::SMSG_LEVELUP_INFO(c) => c.astd_write_encrypted_server(w, e).await,
            Self::MSG_MINIMAP_PING(c) => c.astd_write_encrypted_server(w, e).await,
            Self::SMSG_PONG(c) => c.astd_write_encrypted_server(w, e).await,
            Self::SMSG_GAMEOBJECT_PAGETEXT(c) => c.astd_write_encrypted_server(w, e).await,
            Self::SMSG_ITEM_TIME_UPDATE(c) => c.astd_write_encrypted_server(w, e).await,
            Self::SMSG_ITEM_ENCHANT_TIME_UPDATE(c) => c.astd_write_encrypted_server(w, e).await,
            Self::SMSG_AUTH_CHALLENGE(c) => c.astd_write_encrypted_server(w, e).await,
            Self::SMSG_AUTH_RESPONSE(c) => c.astd_write_encrypted_server(w, e).await,
            Self::MSG_SAVE_GUILD_EMBLEM(c) => c.astd_write_encrypted_server(w, e).await,
            Self::SMSG_EXPLORATION_EXPERIENCE(c) => c.astd_write_encrypted_server(w, e).await,
            Self::MSG_RANDOM_ROLL(c) => c.astd_write_encrypted_server(w, e).await,
            Self::SMSG_ENVIRONMENTAL_DAMAGE_LOG(c) => c.astd_write_encrypted_server(w, e).await,
            Self::SMSG_GMTICKET_CREATE(c) => c.astd_write_encrypted_server(w, e).await,
            Self::SMSG_GMTICKET_UPDATETEXT(c) => c.astd_write_encrypted_server(w, e).await,
            Self::SMSG_ACCOUNT_DATA_TIMES(c) => c.astd_write_encrypted_server(w, e).await,
            Self::SMSG_UPDATE_ACCOUNT_DATA(c) => c.astd_write_encrypted_server(w, e).await,
            Self::SMSG_GMTICKET_GETTICKET(c) => c.astd_write_encrypted_server(w, e).await,
            Self::SMSG_GAMEOBJECT_DESPAWN_ANIM(c) => c.astd_write_encrypted_server(w, e).await,
            Self::MSG_CORPSE_QUERY(c) => c.astd_write_encrypted_server(w, e).await,
            Self::SMSG_GMTICKET_DELETETICKET(c) => c.astd_write_encrypted_server(w, e).await,
            Self::SMSG_CHAT_WRONG_FACTION(c) => c.astd_write_encrypted_server(w, e).await,
            Self::SMSG_GMTICKET_SYSTEMSTATUS(c) => c.astd_write_encrypted_server(w, e).await,
            Self::SMSG_QUEST_FORCE_REMOVE(c) => c.astd_write_encrypted_server(w, e).await,
            Self::SMSG_GOSSIP_POI(c) => c.astd_write_encrypted_server(w, e).await,
            Self::SMSG_LOGIN_VERIFY_WORLD(c) => c.astd_write_encrypted_server(w, e).await,
            Self::SMSG_BATTLEFIELD_LIST(c) => c.astd_write_encrypted_server(w, e).await,
            Self::SMSG_ITEM_TEXT_QUERY_RESPONSE(c) => c.astd_write_encrypted_server(w, e).await,
            Self::SMSG_ZONE_UNDER_ATTACK(c) => c.astd_write_encrypted_server(w, e).await,
            Self::MSG_AUCTION_HELLO(c) => c.astd_write_encrypted_server(w, e).await,
            Self::SMSG_AUCTION_LIST_RESULT(c) => c.astd_write_encrypted_server(w, e).await,
            Self::SMSG_AUCTION_OWNER_LIST_RESULT(c) => c.astd_write_encrypted_server(w, e).await,
            Self::SMSG_AUCTION_BIDDER_NOTIFICATION(c) => c.astd_write_encrypted_server(w, e).await,
            Self::SMSG_AUCTION_OWNER_NOTIFICATION(c) => c.astd_write_encrypted_server(w, e).await,
            Self::SMSG_PROCRESIST(c) => c.astd_write_encrypted_server(w, e).await,
            Self::SMSG_AUCTION_BIDDER_LIST_RESULT(c) => c.astd_write_encrypted_server(w, e).await,
            Self::MSG_LIST_STABLED_PETS(c) => c.astd_write_encrypted_server(w, e).await,
            Self::SMSG_STABLE_RESULT(c) => c.astd_write_encrypted_server(w, e).await,
            Self::SMSG_PLAY_OBJECT_SOUND(c) => c.astd_write_encrypted_server(w, e).await,
            Self::MSG_QUERY_NEXT_MAIL_TIME(c) => c.astd_write_encrypted_server(w, e).await,
            Self::SMSG_RECEIVED_MAIL(c) => c.astd_write_encrypted_server(w, e).await,
            Self::SMSG_PVP_CREDIT(c) => c.astd_write_encrypted_server(w, e).await,
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
            Self::SMSG_PET_BROKEN(c) => c.astd_write_encrypted_server(w, e).await,
            Self::SMSG_DUEL_COUNTDOWN(c) => c.astd_write_encrypted_server(w, e).await,
            Self::SMSG_AREA_TRIGGER_MESSAGE(c) => c.astd_write_encrypted_server(w, e).await,
            Self::SMSG_DURABILITY_DAMAGE_DEATH(c) => c.astd_write_encrypted_server(w, e).await,
            Self::SMSG_INIT_WORLD_STATES(c) => c.astd_write_encrypted_server(w, e).await,
            Self::SMSG_UPDATE_WORLD_STATE(c) => c.astd_write_encrypted_server(w, e).await,
            Self::SMSG_ITEM_NAME_QUERY_RESPONSE(c) => c.astd_write_encrypted_server(w, e).await,
            Self::SMSG_PET_ACTION_FEEDBACK(c) => c.astd_write_encrypted_server(w, e).await,
            Self::SMSG_CHAR_RENAME(c) => c.astd_write_encrypted_server(w, e).await,
            Self::SMSG_RAID_INSTANCE_INFO(c) => c.astd_write_encrypted_server(w, e).await,
            Self::MSG_INSPECT_HONOR_STATS(c) => c.astd_write_encrypted_server(w, e).await,
            Self::SMSG_FORCE_WALK_SPEED_CHANGE(c) => c.astd_write_encrypted_server(w, e).await,
            Self::SMSG_FORCE_SWIM_BACK_SPEED_CHANGE(c) => c.astd_write_encrypted_server(w, e).await,
            Self::SMSG_FORCE_TURN_RATE_CHANGE(c) => c.astd_write_encrypted_server(w, e).await,
            Self::MSG_BATTLEGROUND_PLAYER_POSITIONS(c) => c.astd_write_encrypted_server(w, e).await,
            Self::SMSG_BINDER_CONFIRM(c) => c.astd_write_encrypted_server(w, e).await,
            Self::SMSG_BATTLEGROUND_PLAYER_JOINED(c) => c.astd_write_encrypted_server(w, e).await,
            Self::SMSG_BATTLEGROUND_PLAYER_LEFT(c) => c.astd_write_encrypted_server(w, e).await,
            Self::SMSG_ADDON_INFO(c) => c.astd_write_encrypted_server(w, e).await,
            Self::SMSG_PET_UNLEARN_CONFIRM(c) => c.astd_write_encrypted_server(w, e).await,
            Self::SMSG_CHAT_RESTRICTED(c) => c.astd_write_encrypted_server(w, e).await,
            Self::SMSG_SPLINE_SET_RUN_SPEED(c) => c.astd_write_encrypted_server(w, e).await,
            Self::SMSG_SPLINE_SET_RUN_BACK_SPEED(c) => c.astd_write_encrypted_server(w, e).await,
            Self::SMSG_SPLINE_SET_SWIM_SPEED(c) => c.astd_write_encrypted_server(w, e).await,
            Self::SMSG_SPLINE_SET_WALK_SPEED(c) => c.astd_write_encrypted_server(w, e).await,
            Self::SMSG_SPLINE_SET_SWIM_BACK_SPEED(c) => c.astd_write_encrypted_server(w, e).await,
            Self::SMSG_SPLINE_SET_TURN_RATE(c) => c.astd_write_encrypted_server(w, e).await,
            Self::SMSG_SPLINE_MOVE_UNROOT(c) => c.astd_write_encrypted_server(w, e).await,
            Self::SMSG_SPLINE_MOVE_FEATHER_FALL(c) => c.astd_write_encrypted_server(w, e).await,
            Self::SMSG_SPLINE_MOVE_NORMAL_FALL(c) => c.astd_write_encrypted_server(w, e).await,
            Self::SMSG_SPLINE_MOVE_SET_HOVER(c) => c.astd_write_encrypted_server(w, e).await,
            Self::SMSG_SPLINE_MOVE_UNSET_HOVER(c) => c.astd_write_encrypted_server(w, e).await,
            Self::SMSG_SPLINE_MOVE_WATER_WALK(c) => c.astd_write_encrypted_server(w, e).await,
            Self::SMSG_SPLINE_MOVE_LAND_WALK(c) => c.astd_write_encrypted_server(w, e).await,
            Self::SMSG_SPLINE_MOVE_START_SWIM(c) => c.astd_write_encrypted_server(w, e).await,
            Self::SMSG_SPLINE_MOVE_STOP_SWIM(c) => c.astd_write_encrypted_server(w, e).await,
            Self::SMSG_SPLINE_MOVE_SET_RUN_MODE(c) => c.astd_write_encrypted_server(w, e).await,
            Self::SMSG_SPLINE_MOVE_SET_WALK_MODE(c) => c.astd_write_encrypted_server(w, e).await,
            Self::SMSG_SPLINE_MOVE_ROOT(c) => c.astd_write_encrypted_server(w, e).await,
            Self::MSG_RAID_READY_CHECK(c) => c.astd_write_encrypted_server(w, e).await,
            Self::SMSG_PET_ACTION_SOUND(c) => c.astd_write_encrypted_server(w, e).await,
            Self::SMSG_PET_DISMISS_SOUND(c) => c.astd_write_encrypted_server(w, e).await,
            Self::SMSG_GM_TICKET_STATUS_UPDATE(c) => c.astd_write_encrypted_server(w, e).await,
            Self::SMSG_DEFENSE_MESSAGE(c) => c.astd_write_encrypted_server(w, e).await,
            Self::SMSG_REALM_SPLIT(c) => c.astd_write_encrypted_server(w, e).await,
            Self::SMSG_TIME_SYNC_REQ(c) => c.astd_write_encrypted_server(w, e).await,
            Self::SMSG_GM_MESSAGECHAT(c) => c.astd_write_encrypted_server(w, e).await,
            Self::SMSG_FEATURE_SYSTEM_STATUS(c) => c.astd_write_encrypted_server(w, e).await,
            Self::SMSG_CALENDAR_SEND_NUM_PENDING(c) => c.astd_write_encrypted_server(w, e).await,
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
            Self::MSG_QUEST_PUSH_RESULT(c) => c.astd_write_unencrypted_server(w).await,
            Self::MSG_PETITION_RENAME(c) => c.astd_write_unencrypted_server(w).await,
            Self::MSG_SET_DUNGEON_DIFFICULTY(c) => c.astd_write_unencrypted_server(w).await,
            Self::MSG_MOVE_START_ASCEND(c) => c.astd_write_unencrypted_server(w).await,
            Self::MSG_MOVE_STOP_ASCEND(c) => c.astd_write_unencrypted_server(w).await,
            Self::MSG_MOVE_START_DESCEND(c) => c.astd_write_unencrypted_server(w).await,
            Self::MSG_GUILD_BANK_MONEY_WITHDRAWN(c) => c.astd_write_unencrypted_server(w).await,
            Self::CMSG_CALENDAR_GET_NUM_PENDING(c) => c.astd_write_unencrypted_server(w).await,
            Self::SMSG_CHAR_CREATE(c) => c.astd_write_unencrypted_server(w).await,
            Self::SMSG_CHAR_ENUM(c) => c.astd_write_unencrypted_server(w).await,
            Self::SMSG_CHAR_DELETE(c) => c.astd_write_unencrypted_server(w).await,
            Self::SMSG_NEW_WORLD(c) => c.astd_write_unencrypted_server(w).await,
            Self::SMSG_TRANSFER_PENDING(c) => c.astd_write_unencrypted_server(w).await,
            Self::SMSG_TRANSFER_ABORTED(c) => c.astd_write_unencrypted_server(w).await,
            Self::SMSG_CHARACTER_LOGIN_FAILED(c) => c.astd_write_unencrypted_server(w).await,
            Self::SMSG_LOGIN_SETTIMESPEED(c) => c.astd_write_unencrypted_server(w).await,
            Self::SMSG_LOGOUT_RESPONSE(c) => c.astd_write_unencrypted_server(w).await,
            Self::SMSG_LOGOUT_COMPLETE(c) => c.astd_write_unencrypted_server(w).await,
            Self::SMSG_LOGOUT_CANCEL_ACK(c) => c.astd_write_unencrypted_server(w).await,
            Self::SMSG_NAME_QUERY_RESPONSE(c) => c.astd_write_unencrypted_server(w).await,
            Self::SMSG_PET_NAME_QUERY_RESPONSE(c) => c.astd_write_unencrypted_server(w).await,
            Self::SMSG_GUILD_QUERY_RESPONSE(c) => c.astd_write_unencrypted_server(w).await,
            Self::SMSG_ITEM_QUERY_SINGLE_RESPONSE(c) => c.astd_write_unencrypted_server(w).await,
            Self::SMSG_PAGE_TEXT_QUERY_RESPONSE(c) => c.astd_write_unencrypted_server(w).await,
            Self::SMSG_QUEST_QUERY_RESPONSE(c) => c.astd_write_unencrypted_server(w).await,
            Self::SMSG_GAMEOBJECT_QUERY_RESPONSE(c) => c.astd_write_unencrypted_server(w).await,
            Self::SMSG_CREATURE_QUERY_RESPONSE(c) => c.astd_write_unencrypted_server(w).await,
            Self::SMSG_WHO(c) => c.astd_write_unencrypted_server(w).await,
            Self::SMSG_WHOIS(c) => c.astd_write_unencrypted_server(w).await,
            Self::SMSG_CONTACT_LIST(c) => c.astd_write_unencrypted_server(w).await,
            Self::SMSG_FRIEND_STATUS(c) => c.astd_write_unencrypted_server(w).await,
            Self::SMSG_GROUP_INVITE(c) => c.astd_write_unencrypted_server(w).await,
            Self::SMSG_GROUP_DECLINE(c) => c.astd_write_unencrypted_server(w).await,
            Self::SMSG_GROUP_UNINVITE(c) => c.astd_write_unencrypted_server(w).await,
            Self::SMSG_GROUP_SET_LEADER(c) => c.astd_write_unencrypted_server(w).await,
            Self::SMSG_GROUP_DESTROYED(c) => c.astd_write_unencrypted_server(w).await,
            Self::SMSG_GROUP_LIST(c) => c.astd_write_unencrypted_server(w).await,
            Self::SMSG_PARTY_COMMAND_RESULT(c) => c.astd_write_unencrypted_server(w).await,
            Self::SMSG_GUILD_INVITE(c) => c.astd_write_unencrypted_server(w).await,
            Self::SMSG_GUILD_INFO(c) => c.astd_write_unencrypted_server(w).await,
            Self::SMSG_GUILD_ROSTER(c) => c.astd_write_unencrypted_server(w).await,
            Self::SMSG_GUILD_EVENT(c) => c.astd_write_unencrypted_server(w).await,
            Self::SMSG_GUILD_COMMAND_RESULT(c) => c.astd_write_unencrypted_server(w).await,
            Self::SMSG_MESSAGECHAT(c) => c.astd_write_unencrypted_server(w).await,
            Self::SMSG_CHANNEL_NOTIFY(c) => c.astd_write_unencrypted_server(w).await,
            Self::SMSG_CHANNEL_LIST(c) => c.astd_write_unencrypted_server(w).await,
            Self::SMSG_UPDATE_OBJECT(c) => c.astd_write_unencrypted_server(w).await,
            Self::SMSG_DESTROY_OBJECT(c) => c.astd_write_unencrypted_server(w).await,
            Self::SMSG_READ_ITEM_OK(c) => c.astd_write_unencrypted_server(w).await,
            Self::SMSG_READ_ITEM_FAILED(c) => c.astd_write_unencrypted_server(w).await,
            Self::SMSG_ITEM_COOLDOWN(c) => c.astd_write_unencrypted_server(w).await,
            Self::SMSG_GAMEOBJECT_CUSTOM_ANIM(c) => c.astd_write_unencrypted_server(w).await,
            Self::MSG_MOVE_TELEPORT_ACK(c) => c.astd_write_unencrypted_server(w).await,
            Self::SMSG_MOVE_WATER_WALK(c) => c.astd_write_unencrypted_server(w).await,
            Self::SMSG_MOVE_LAND_WALK(c) => c.astd_write_unencrypted_server(w).await,
            Self::SMSG_FORCE_RUN_SPEED_CHANGE(c) => c.astd_write_unencrypted_server(w).await,
            Self::SMSG_FORCE_RUN_BACK_SPEED_CHANGE(c) => c.astd_write_unencrypted_server(w).await,
            Self::SMSG_FORCE_SWIM_SPEED_CHANGE(c) => c.astd_write_unencrypted_server(w).await,
            Self::SMSG_FORCE_MOVE_ROOT(c) => c.astd_write_unencrypted_server(w).await,
            Self::SMSG_FORCE_MOVE_UNROOT(c) => c.astd_write_unencrypted_server(w).await,
            Self::SMSG_MOVE_KNOCK_BACK(c) => c.astd_write_unencrypted_server(w).await,
            Self::SMSG_MOVE_FEATHER_FALL(c) => c.astd_write_unencrypted_server(w).await,
            Self::SMSG_MOVE_NORMAL_FALL(c) => c.astd_write_unencrypted_server(w).await,
            Self::SMSG_MOVE_SET_HOVER(c) => c.astd_write_unencrypted_server(w).await,
            Self::SMSG_MOVE_UNSET_HOVER(c) => c.astd_write_unencrypted_server(w).await,
            Self::SMSG_TRIGGER_CINEMATIC(c) => c.astd_write_unencrypted_server(w).await,
            Self::SMSG_TUTORIAL_FLAGS(c) => c.astd_write_unencrypted_server(w).await,
            Self::SMSG_EMOTE(c) => c.astd_write_unencrypted_server(w).await,
            Self::SMSG_TEXT_EMOTE(c) => c.astd_write_unencrypted_server(w).await,
            Self::SMSG_INVENTORY_CHANGE_FAILURE(c) => c.astd_write_unencrypted_server(w).await,
            Self::SMSG_TRADE_STATUS(c) => c.astd_write_unencrypted_server(w).await,
            Self::SMSG_TRADE_STATUS_EXTENDED(c) => c.astd_write_unencrypted_server(w).await,
            Self::SMSG_INITIALIZE_FACTIONS(c) => c.astd_write_unencrypted_server(w).await,
            Self::SMSG_SET_FACTION_VISIBLE(c) => c.astd_write_unencrypted_server(w).await,
            Self::SMSG_SET_FACTION_STANDING(c) => c.astd_write_unencrypted_server(w).await,
            Self::SMSG_SET_PROFICIENCY(c) => c.astd_write_unencrypted_server(w).await,
            Self::SMSG_ACTION_BUTTONS(c) => c.astd_write_unencrypted_server(w).await,
            Self::SMSG_INITIAL_SPELLS(c) => c.astd_write_unencrypted_server(w).await,
            Self::SMSG_LEARNED_SPELL(c) => c.astd_write_unencrypted_server(w).await,
            Self::SMSG_SUPERCEDED_SPELL(c) => c.astd_write_unencrypted_server(w).await,
            Self::SMSG_SPELL_START(c) => c.astd_write_unencrypted_server(w).await,
            Self::SMSG_SPELL_GO(c) => c.astd_write_unencrypted_server(w).await,
            Self::SMSG_SPELL_FAILURE(c) => c.astd_write_unencrypted_server(w).await,
            Self::SMSG_SPELL_COOLDOWN(c) => c.astd_write_unencrypted_server(w).await,
            Self::SMSG_COOLDOWN_EVENT(c) => c.astd_write_unencrypted_server(w).await,
            Self::SMSG_PET_CAST_FAILED(c) => c.astd_write_unencrypted_server(w).await,
            Self::SMSG_AI_REACTION(c) => c.astd_write_unencrypted_server(w).await,
            Self::SMSG_ATTACKSTART(c) => c.astd_write_unencrypted_server(w).await,
            Self::SMSG_ATTACKSTOP(c) => c.astd_write_unencrypted_server(w).await,
            Self::SMSG_ATTACKSWING_NOTINRANGE(c) => c.astd_write_unencrypted_server(w).await,
            Self::SMSG_ATTACKSWING_BADFACING(c) => c.astd_write_unencrypted_server(w).await,
            Self::SMSG_ATTACKSWING_DEADTARGET(c) => c.astd_write_unencrypted_server(w).await,
            Self::SMSG_ATTACKSWING_CANT_ATTACK(c) => c.astd_write_unencrypted_server(w).await,
            Self::SMSG_ATTACKERSTATEUPDATE(c) => c.astd_write_unencrypted_server(w).await,
            Self::SMSG_CANCEL_COMBAT(c) => c.astd_write_unencrypted_server(w).await,
            Self::SMSG_SPELLHEALLOG(c) => c.astd_write_unencrypted_server(w).await,
            Self::SMSG_SPELLENERGIZELOG(c) => c.astd_write_unencrypted_server(w).await,
            Self::SMSG_BINDPOINTUPDATE(c) => c.astd_write_unencrypted_server(w).await,
            Self::SMSG_PLAYERBOUND(c) => c.astd_write_unencrypted_server(w).await,
            Self::SMSG_CLIENT_CONTROL_UPDATE(c) => c.astd_write_unencrypted_server(w).await,
            Self::SMSG_RESURRECT_REQUEST(c) => c.astd_write_unencrypted_server(w).await,
            Self::SMSG_LOOT_RESPONSE(c) => c.astd_write_unencrypted_server(w).await,
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
            Self::SMSG_PET_TAME_FAILURE(c) => c.astd_write_unencrypted_server(w).await,
            Self::SMSG_PET_NAME_INVALID(c) => c.astd_write_unencrypted_server(w).await,
            Self::SMSG_PET_SPELLS(c) => c.astd_write_unencrypted_server(w).await,
            Self::SMSG_PET_MODE(c) => c.astd_write_unencrypted_server(w).await,
            Self::SMSG_GOSSIP_MESSAGE(c) => c.astd_write_unencrypted_server(w).await,
            Self::SMSG_GOSSIP_COMPLETE(c) => c.astd_write_unencrypted_server(w).await,
            Self::SMSG_NPC_TEXT_UPDATE(c) => c.astd_write_unencrypted_server(w).await,
            Self::SMSG_QUESTGIVER_STATUS(c) => c.astd_write_unencrypted_server(w).await,
            Self::SMSG_QUESTGIVER_QUEST_LIST(c) => c.astd_write_unencrypted_server(w).await,
            Self::SMSG_QUESTGIVER_QUEST_DETAILS(c) => c.astd_write_unencrypted_server(w).await,
            Self::SMSG_QUESTGIVER_REQUEST_ITEMS(c) => c.astd_write_unencrypted_server(w).await,
            Self::SMSG_QUESTGIVER_OFFER_REWARD(c) => c.astd_write_unencrypted_server(w).await,
            Self::SMSG_QUESTGIVER_QUEST_INVALID(c) => c.astd_write_unencrypted_server(w).await,
            Self::SMSG_QUESTGIVER_QUEST_COMPLETE(c) => c.astd_write_unencrypted_server(w).await,
            Self::SMSG_QUESTGIVER_QUEST_FAILED(c) => c.astd_write_unencrypted_server(w).await,
            Self::SMSG_QUESTLOG_FULL(c) => c.astd_write_unencrypted_server(w).await,
            Self::SMSG_QUESTUPDATE_FAILED(c) => c.astd_write_unencrypted_server(w).await,
            Self::SMSG_QUESTUPDATE_FAILEDTIMER(c) => c.astd_write_unencrypted_server(w).await,
            Self::SMSG_QUESTUPDATE_COMPLETE(c) => c.astd_write_unencrypted_server(w).await,
            Self::SMSG_QUESTUPDATE_ADD_KILL(c) => c.astd_write_unencrypted_server(w).await,
            Self::SMSG_QUESTUPDATE_ADD_ITEM(c) => c.astd_write_unencrypted_server(w).await,
            Self::SMSG_QUEST_CONFIRM_ACCEPT(c) => c.astd_write_unencrypted_server(w).await,
            Self::SMSG_LIST_INVENTORY(c) => c.astd_write_unencrypted_server(w).await,
            Self::SMSG_SELL_ITEM(c) => c.astd_write_unencrypted_server(w).await,
            Self::SMSG_BUY_ITEM(c) => c.astd_write_unencrypted_server(w).await,
            Self::SMSG_BUY_FAILED(c) => c.astd_write_unencrypted_server(w).await,
            Self::SMSG_SHOWTAXINODES(c) => c.astd_write_unencrypted_server(w).await,
            Self::SMSG_TAXINODE_STATUS(c) => c.astd_write_unencrypted_server(w).await,
            Self::SMSG_ACTIVATETAXIREPLY(c) => c.astd_write_unencrypted_server(w).await,
            Self::SMSG_NEW_TAXI_PATH(c) => c.astd_write_unencrypted_server(w).await,
            Self::SMSG_TRAINER_LIST(c) => c.astd_write_unencrypted_server(w).await,
            Self::SMSG_TRAINER_BUY_SUCCEEDED(c) => c.astd_write_unencrypted_server(w).await,
            Self::SMSG_TRAINER_BUY_FAILED(c) => c.astd_write_unencrypted_server(w).await,
            Self::SMSG_SHOW_BANK(c) => c.astd_write_unencrypted_server(w).await,
            Self::SMSG_BUY_BANK_SLOT_RESULT(c) => c.astd_write_unencrypted_server(w).await,
            Self::SMSG_PETITION_SHOWLIST(c) => c.astd_write_unencrypted_server(w).await,
            Self::SMSG_PETITION_SHOW_SIGNATURES(c) => c.astd_write_unencrypted_server(w).await,
            Self::SMSG_PETITION_SIGN_RESULTS(c) => c.astd_write_unencrypted_server(w).await,
            Self::SMSG_TURN_IN_PETITION_RESULTS(c) => c.astd_write_unencrypted_server(w).await,
            Self::SMSG_PETITION_QUERY_RESPONSE(c) => c.astd_write_unencrypted_server(w).await,
            Self::SMSG_FISH_NOT_HOOKED(c) => c.astd_write_unencrypted_server(w).await,
            Self::SMSG_FISH_ESCAPED(c) => c.astd_write_unencrypted_server(w).await,
            Self::SMSG_NOTIFICATION(c) => c.astd_write_unencrypted_server(w).await,
            Self::SMSG_PLAYED_TIME(c) => c.astd_write_unencrypted_server(w).await,
            Self::SMSG_QUERY_TIME_RESPONSE(c) => c.astd_write_unencrypted_server(w).await,
            Self::SMSG_LOG_XPGAIN(c) => c.astd_write_unencrypted_server(w).await,
            Self::SMSG_LEVELUP_INFO(c) => c.astd_write_unencrypted_server(w).await,
            Self::MSG_MINIMAP_PING(c) => c.astd_write_unencrypted_server(w).await,
            Self::SMSG_PONG(c) => c.astd_write_unencrypted_server(w).await,
            Self::SMSG_GAMEOBJECT_PAGETEXT(c) => c.astd_write_unencrypted_server(w).await,
            Self::SMSG_ITEM_TIME_UPDATE(c) => c.astd_write_unencrypted_server(w).await,
            Self::SMSG_ITEM_ENCHANT_TIME_UPDATE(c) => c.astd_write_unencrypted_server(w).await,
            Self::SMSG_AUTH_CHALLENGE(c) => c.astd_write_unencrypted_server(w).await,
            Self::SMSG_AUTH_RESPONSE(c) => c.astd_write_unencrypted_server(w).await,
            Self::MSG_SAVE_GUILD_EMBLEM(c) => c.astd_write_unencrypted_server(w).await,
            Self::SMSG_EXPLORATION_EXPERIENCE(c) => c.astd_write_unencrypted_server(w).await,
            Self::MSG_RANDOM_ROLL(c) => c.astd_write_unencrypted_server(w).await,
            Self::SMSG_ENVIRONMENTAL_DAMAGE_LOG(c) => c.astd_write_unencrypted_server(w).await,
            Self::SMSG_GMTICKET_CREATE(c) => c.astd_write_unencrypted_server(w).await,
            Self::SMSG_GMTICKET_UPDATETEXT(c) => c.astd_write_unencrypted_server(w).await,
            Self::SMSG_ACCOUNT_DATA_TIMES(c) => c.astd_write_unencrypted_server(w).await,
            Self::SMSG_UPDATE_ACCOUNT_DATA(c) => c.astd_write_unencrypted_server(w).await,
            Self::SMSG_GMTICKET_GETTICKET(c) => c.astd_write_unencrypted_server(w).await,
            Self::SMSG_GAMEOBJECT_DESPAWN_ANIM(c) => c.astd_write_unencrypted_server(w).await,
            Self::MSG_CORPSE_QUERY(c) => c.astd_write_unencrypted_server(w).await,
            Self::SMSG_GMTICKET_DELETETICKET(c) => c.astd_write_unencrypted_server(w).await,
            Self::SMSG_CHAT_WRONG_FACTION(c) => c.astd_write_unencrypted_server(w).await,
            Self::SMSG_GMTICKET_SYSTEMSTATUS(c) => c.astd_write_unencrypted_server(w).await,
            Self::SMSG_QUEST_FORCE_REMOVE(c) => c.astd_write_unencrypted_server(w).await,
            Self::SMSG_GOSSIP_POI(c) => c.astd_write_unencrypted_server(w).await,
            Self::SMSG_LOGIN_VERIFY_WORLD(c) => c.astd_write_unencrypted_server(w).await,
            Self::SMSG_BATTLEFIELD_LIST(c) => c.astd_write_unencrypted_server(w).await,
            Self::SMSG_ITEM_TEXT_QUERY_RESPONSE(c) => c.astd_write_unencrypted_server(w).await,
            Self::SMSG_ZONE_UNDER_ATTACK(c) => c.astd_write_unencrypted_server(w).await,
            Self::MSG_AUCTION_HELLO(c) => c.astd_write_unencrypted_server(w).await,
            Self::SMSG_AUCTION_LIST_RESULT(c) => c.astd_write_unencrypted_server(w).await,
            Self::SMSG_AUCTION_OWNER_LIST_RESULT(c) => c.astd_write_unencrypted_server(w).await,
            Self::SMSG_AUCTION_BIDDER_NOTIFICATION(c) => c.astd_write_unencrypted_server(w).await,
            Self::SMSG_AUCTION_OWNER_NOTIFICATION(c) => c.astd_write_unencrypted_server(w).await,
            Self::SMSG_PROCRESIST(c) => c.astd_write_unencrypted_server(w).await,
            Self::SMSG_AUCTION_BIDDER_LIST_RESULT(c) => c.astd_write_unencrypted_server(w).await,
            Self::MSG_LIST_STABLED_PETS(c) => c.astd_write_unencrypted_server(w).await,
            Self::SMSG_STABLE_RESULT(c) => c.astd_write_unencrypted_server(w).await,
            Self::SMSG_PLAY_OBJECT_SOUND(c) => c.astd_write_unencrypted_server(w).await,
            Self::MSG_QUERY_NEXT_MAIL_TIME(c) => c.astd_write_unencrypted_server(w).await,
            Self::SMSG_RECEIVED_MAIL(c) => c.astd_write_unencrypted_server(w).await,
            Self::SMSG_PVP_CREDIT(c) => c.astd_write_unencrypted_server(w).await,
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
            Self::SMSG_PET_BROKEN(c) => c.astd_write_unencrypted_server(w).await,
            Self::SMSG_DUEL_COUNTDOWN(c) => c.astd_write_unencrypted_server(w).await,
            Self::SMSG_AREA_TRIGGER_MESSAGE(c) => c.astd_write_unencrypted_server(w).await,
            Self::SMSG_DURABILITY_DAMAGE_DEATH(c) => c.astd_write_unencrypted_server(w).await,
            Self::SMSG_INIT_WORLD_STATES(c) => c.astd_write_unencrypted_server(w).await,
            Self::SMSG_UPDATE_WORLD_STATE(c) => c.astd_write_unencrypted_server(w).await,
            Self::SMSG_ITEM_NAME_QUERY_RESPONSE(c) => c.astd_write_unencrypted_server(w).await,
            Self::SMSG_PET_ACTION_FEEDBACK(c) => c.astd_write_unencrypted_server(w).await,
            Self::SMSG_CHAR_RENAME(c) => c.astd_write_unencrypted_server(w).await,
            Self::SMSG_RAID_INSTANCE_INFO(c) => c.astd_write_unencrypted_server(w).await,
            Self::MSG_INSPECT_HONOR_STATS(c) => c.astd_write_unencrypted_server(w).await,
            Self::SMSG_FORCE_WALK_SPEED_CHANGE(c) => c.astd_write_unencrypted_server(w).await,
            Self::SMSG_FORCE_SWIM_BACK_SPEED_CHANGE(c) => c.astd_write_unencrypted_server(w).await,
            Self::SMSG_FORCE_TURN_RATE_CHANGE(c) => c.astd_write_unencrypted_server(w).await,
            Self::MSG_BATTLEGROUND_PLAYER_POSITIONS(c) => c.astd_write_unencrypted_server(w).await,
            Self::SMSG_BINDER_CONFIRM(c) => c.astd_write_unencrypted_server(w).await,
            Self::SMSG_BATTLEGROUND_PLAYER_JOINED(c) => c.astd_write_unencrypted_server(w).await,
            Self::SMSG_BATTLEGROUND_PLAYER_LEFT(c) => c.astd_write_unencrypted_server(w).await,
            Self::SMSG_ADDON_INFO(c) => c.astd_write_unencrypted_server(w).await,
            Self::SMSG_PET_UNLEARN_CONFIRM(c) => c.astd_write_unencrypted_server(w).await,
            Self::SMSG_CHAT_RESTRICTED(c) => c.astd_write_unencrypted_server(w).await,
            Self::SMSG_SPLINE_SET_RUN_SPEED(c) => c.astd_write_unencrypted_server(w).await,
            Self::SMSG_SPLINE_SET_RUN_BACK_SPEED(c) => c.astd_write_unencrypted_server(w).await,
            Self::SMSG_SPLINE_SET_SWIM_SPEED(c) => c.astd_write_unencrypted_server(w).await,
            Self::SMSG_SPLINE_SET_WALK_SPEED(c) => c.astd_write_unencrypted_server(w).await,
            Self::SMSG_SPLINE_SET_SWIM_BACK_SPEED(c) => c.astd_write_unencrypted_server(w).await,
            Self::SMSG_SPLINE_SET_TURN_RATE(c) => c.astd_write_unencrypted_server(w).await,
            Self::SMSG_SPLINE_MOVE_UNROOT(c) => c.astd_write_unencrypted_server(w).await,
            Self::SMSG_SPLINE_MOVE_FEATHER_FALL(c) => c.astd_write_unencrypted_server(w).await,
            Self::SMSG_SPLINE_MOVE_NORMAL_FALL(c) => c.astd_write_unencrypted_server(w).await,
            Self::SMSG_SPLINE_MOVE_SET_HOVER(c) => c.astd_write_unencrypted_server(w).await,
            Self::SMSG_SPLINE_MOVE_UNSET_HOVER(c) => c.astd_write_unencrypted_server(w).await,
            Self::SMSG_SPLINE_MOVE_WATER_WALK(c) => c.astd_write_unencrypted_server(w).await,
            Self::SMSG_SPLINE_MOVE_LAND_WALK(c) => c.astd_write_unencrypted_server(w).await,
            Self::SMSG_SPLINE_MOVE_START_SWIM(c) => c.astd_write_unencrypted_server(w).await,
            Self::SMSG_SPLINE_MOVE_STOP_SWIM(c) => c.astd_write_unencrypted_server(w).await,
            Self::SMSG_SPLINE_MOVE_SET_RUN_MODE(c) => c.astd_write_unencrypted_server(w).await,
            Self::SMSG_SPLINE_MOVE_SET_WALK_MODE(c) => c.astd_write_unencrypted_server(w).await,
            Self::SMSG_SPLINE_MOVE_ROOT(c) => c.astd_write_unencrypted_server(w).await,
            Self::MSG_RAID_READY_CHECK(c) => c.astd_write_unencrypted_server(w).await,
            Self::SMSG_PET_ACTION_SOUND(c) => c.astd_write_unencrypted_server(w).await,
            Self::SMSG_PET_DISMISS_SOUND(c) => c.astd_write_unencrypted_server(w).await,
            Self::SMSG_GM_TICKET_STATUS_UPDATE(c) => c.astd_write_unencrypted_server(w).await,
            Self::SMSG_DEFENSE_MESSAGE(c) => c.astd_write_unencrypted_server(w).await,
            Self::SMSG_REALM_SPLIT(c) => c.astd_write_unencrypted_server(w).await,
            Self::SMSG_TIME_SYNC_REQ(c) => c.astd_write_unencrypted_server(w).await,
            Self::SMSG_GM_MESSAGECHAT(c) => c.astd_write_unencrypted_server(w).await,
            Self::SMSG_FEATURE_SYSTEM_STATUS(c) => c.astd_write_unencrypted_server(w).await,
            Self::SMSG_CALENDAR_SEND_NUM_PENDING(c) => c.astd_write_unencrypted_server(w).await,
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
            ServerOpcodeMessage::MSG_QUEST_PUSH_RESULT(_) => "MSG_QUEST_PUSH_RESULT",
            ServerOpcodeMessage::MSG_PETITION_RENAME(_) => "MSG_PETITION_RENAME",
            ServerOpcodeMessage::MSG_SET_DUNGEON_DIFFICULTY(_) => "MSG_SET_DUNGEON_DIFFICULTY",
            ServerOpcodeMessage::MSG_MOVE_START_ASCEND(_) => "MSG_MOVE_START_ASCEND",
            ServerOpcodeMessage::MSG_MOVE_STOP_ASCEND(_) => "MSG_MOVE_STOP_ASCEND",
            ServerOpcodeMessage::MSG_MOVE_START_DESCEND(_) => "MSG_MOVE_START_DESCEND",
            ServerOpcodeMessage::MSG_GUILD_BANK_MONEY_WITHDRAWN(_) => "MSG_GUILD_BANK_MONEY_WITHDRAWN",
            ServerOpcodeMessage::CMSG_CALENDAR_GET_NUM_PENDING(_) => "CMSG_CALENDAR_GET_NUM_PENDING",
            ServerOpcodeMessage::SMSG_CHAR_CREATE(_) => "SMSG_CHAR_CREATE",
            ServerOpcodeMessage::SMSG_CHAR_ENUM(_) => "SMSG_CHAR_ENUM",
            ServerOpcodeMessage::SMSG_CHAR_DELETE(_) => "SMSG_CHAR_DELETE",
            ServerOpcodeMessage::SMSG_NEW_WORLD(_) => "SMSG_NEW_WORLD",
            ServerOpcodeMessage::SMSG_TRANSFER_PENDING(_) => "SMSG_TRANSFER_PENDING",
            ServerOpcodeMessage::SMSG_TRANSFER_ABORTED(_) => "SMSG_TRANSFER_ABORTED",
            ServerOpcodeMessage::SMSG_CHARACTER_LOGIN_FAILED(_) => "SMSG_CHARACTER_LOGIN_FAILED",
            ServerOpcodeMessage::SMSG_LOGIN_SETTIMESPEED(_) => "SMSG_LOGIN_SETTIMESPEED",
            ServerOpcodeMessage::SMSG_LOGOUT_RESPONSE(_) => "SMSG_LOGOUT_RESPONSE",
            ServerOpcodeMessage::SMSG_LOGOUT_COMPLETE(_) => "SMSG_LOGOUT_COMPLETE",
            ServerOpcodeMessage::SMSG_LOGOUT_CANCEL_ACK(_) => "SMSG_LOGOUT_CANCEL_ACK",
            ServerOpcodeMessage::SMSG_NAME_QUERY_RESPONSE(_) => "SMSG_NAME_QUERY_RESPONSE",
            ServerOpcodeMessage::SMSG_PET_NAME_QUERY_RESPONSE(_) => "SMSG_PET_NAME_QUERY_RESPONSE",
            ServerOpcodeMessage::SMSG_GUILD_QUERY_RESPONSE(_) => "SMSG_GUILD_QUERY_RESPONSE",
            ServerOpcodeMessage::SMSG_ITEM_QUERY_SINGLE_RESPONSE(_) => "SMSG_ITEM_QUERY_SINGLE_RESPONSE",
            ServerOpcodeMessage::SMSG_PAGE_TEXT_QUERY_RESPONSE(_) => "SMSG_PAGE_TEXT_QUERY_RESPONSE",
            ServerOpcodeMessage::SMSG_QUEST_QUERY_RESPONSE(_) => "SMSG_QUEST_QUERY_RESPONSE",
            ServerOpcodeMessage::SMSG_GAMEOBJECT_QUERY_RESPONSE(_) => "SMSG_GAMEOBJECT_QUERY_RESPONSE",
            ServerOpcodeMessage::SMSG_CREATURE_QUERY_RESPONSE(_) => "SMSG_CREATURE_QUERY_RESPONSE",
            ServerOpcodeMessage::SMSG_WHO(_) => "SMSG_WHO",
            ServerOpcodeMessage::SMSG_WHOIS(_) => "SMSG_WHOIS",
            ServerOpcodeMessage::SMSG_CONTACT_LIST(_) => "SMSG_CONTACT_LIST",
            ServerOpcodeMessage::SMSG_FRIEND_STATUS(_) => "SMSG_FRIEND_STATUS",
            ServerOpcodeMessage::SMSG_GROUP_INVITE(_) => "SMSG_GROUP_INVITE",
            ServerOpcodeMessage::SMSG_GROUP_DECLINE(_) => "SMSG_GROUP_DECLINE",
            ServerOpcodeMessage::SMSG_GROUP_UNINVITE(_) => "SMSG_GROUP_UNINVITE",
            ServerOpcodeMessage::SMSG_GROUP_SET_LEADER(_) => "SMSG_GROUP_SET_LEADER",
            ServerOpcodeMessage::SMSG_GROUP_DESTROYED(_) => "SMSG_GROUP_DESTROYED",
            ServerOpcodeMessage::SMSG_GROUP_LIST(_) => "SMSG_GROUP_LIST",
            ServerOpcodeMessage::SMSG_PARTY_COMMAND_RESULT(_) => "SMSG_PARTY_COMMAND_RESULT",
            ServerOpcodeMessage::SMSG_GUILD_INVITE(_) => "SMSG_GUILD_INVITE",
            ServerOpcodeMessage::SMSG_GUILD_INFO(_) => "SMSG_GUILD_INFO",
            ServerOpcodeMessage::SMSG_GUILD_ROSTER(_) => "SMSG_GUILD_ROSTER",
            ServerOpcodeMessage::SMSG_GUILD_EVENT(_) => "SMSG_GUILD_EVENT",
            ServerOpcodeMessage::SMSG_GUILD_COMMAND_RESULT(_) => "SMSG_GUILD_COMMAND_RESULT",
            ServerOpcodeMessage::SMSG_MESSAGECHAT(_) => "SMSG_MESSAGECHAT",
            ServerOpcodeMessage::SMSG_CHANNEL_NOTIFY(_) => "SMSG_CHANNEL_NOTIFY",
            ServerOpcodeMessage::SMSG_CHANNEL_LIST(_) => "SMSG_CHANNEL_LIST",
            ServerOpcodeMessage::SMSG_UPDATE_OBJECT(_) => "SMSG_UPDATE_OBJECT",
            ServerOpcodeMessage::SMSG_DESTROY_OBJECT(_) => "SMSG_DESTROY_OBJECT",
            ServerOpcodeMessage::SMSG_READ_ITEM_OK(_) => "SMSG_READ_ITEM_OK",
            ServerOpcodeMessage::SMSG_READ_ITEM_FAILED(_) => "SMSG_READ_ITEM_FAILED",
            ServerOpcodeMessage::SMSG_ITEM_COOLDOWN(_) => "SMSG_ITEM_COOLDOWN",
            ServerOpcodeMessage::SMSG_GAMEOBJECT_CUSTOM_ANIM(_) => "SMSG_GAMEOBJECT_CUSTOM_ANIM",
            ServerOpcodeMessage::MSG_MOVE_TELEPORT_ACK(_) => "MSG_MOVE_TELEPORT_ACK_Server",
            ServerOpcodeMessage::SMSG_MOVE_WATER_WALK(_) => "SMSG_MOVE_WATER_WALK",
            ServerOpcodeMessage::SMSG_MOVE_LAND_WALK(_) => "SMSG_MOVE_LAND_WALK",
            ServerOpcodeMessage::SMSG_FORCE_RUN_SPEED_CHANGE(_) => "SMSG_FORCE_RUN_SPEED_CHANGE",
            ServerOpcodeMessage::SMSG_FORCE_RUN_BACK_SPEED_CHANGE(_) => "SMSG_FORCE_RUN_BACK_SPEED_CHANGE",
            ServerOpcodeMessage::SMSG_FORCE_SWIM_SPEED_CHANGE(_) => "SMSG_FORCE_SWIM_SPEED_CHANGE",
            ServerOpcodeMessage::SMSG_FORCE_MOVE_ROOT(_) => "SMSG_FORCE_MOVE_ROOT",
            ServerOpcodeMessage::SMSG_FORCE_MOVE_UNROOT(_) => "SMSG_FORCE_MOVE_UNROOT",
            ServerOpcodeMessage::SMSG_MOVE_KNOCK_BACK(_) => "SMSG_MOVE_KNOCK_BACK",
            ServerOpcodeMessage::SMSG_MOVE_FEATHER_FALL(_) => "SMSG_MOVE_FEATHER_FALL",
            ServerOpcodeMessage::SMSG_MOVE_NORMAL_FALL(_) => "SMSG_MOVE_NORMAL_FALL",
            ServerOpcodeMessage::SMSG_MOVE_SET_HOVER(_) => "SMSG_MOVE_SET_HOVER",
            ServerOpcodeMessage::SMSG_MOVE_UNSET_HOVER(_) => "SMSG_MOVE_UNSET_HOVER",
            ServerOpcodeMessage::SMSG_TRIGGER_CINEMATIC(_) => "SMSG_TRIGGER_CINEMATIC",
            ServerOpcodeMessage::SMSG_TUTORIAL_FLAGS(_) => "SMSG_TUTORIAL_FLAGS",
            ServerOpcodeMessage::SMSG_EMOTE(_) => "SMSG_EMOTE",
            ServerOpcodeMessage::SMSG_TEXT_EMOTE(_) => "SMSG_TEXT_EMOTE",
            ServerOpcodeMessage::SMSG_INVENTORY_CHANGE_FAILURE(_) => "SMSG_INVENTORY_CHANGE_FAILURE",
            ServerOpcodeMessage::SMSG_TRADE_STATUS(_) => "SMSG_TRADE_STATUS",
            ServerOpcodeMessage::SMSG_TRADE_STATUS_EXTENDED(_) => "SMSG_TRADE_STATUS_EXTENDED",
            ServerOpcodeMessage::SMSG_INITIALIZE_FACTIONS(_) => "SMSG_INITIALIZE_FACTIONS",
            ServerOpcodeMessage::SMSG_SET_FACTION_VISIBLE(_) => "SMSG_SET_FACTION_VISIBLE",
            ServerOpcodeMessage::SMSG_SET_FACTION_STANDING(_) => "SMSG_SET_FACTION_STANDING",
            ServerOpcodeMessage::SMSG_SET_PROFICIENCY(_) => "SMSG_SET_PROFICIENCY",
            ServerOpcodeMessage::SMSG_ACTION_BUTTONS(_) => "SMSG_ACTION_BUTTONS",
            ServerOpcodeMessage::SMSG_INITIAL_SPELLS(_) => "SMSG_INITIAL_SPELLS",
            ServerOpcodeMessage::SMSG_LEARNED_SPELL(_) => "SMSG_LEARNED_SPELL",
            ServerOpcodeMessage::SMSG_SUPERCEDED_SPELL(_) => "SMSG_SUPERCEDED_SPELL",
            ServerOpcodeMessage::SMSG_SPELL_START(_) => "SMSG_SPELL_START",
            ServerOpcodeMessage::SMSG_SPELL_GO(_) => "SMSG_SPELL_GO",
            ServerOpcodeMessage::SMSG_SPELL_FAILURE(_) => "SMSG_SPELL_FAILURE",
            ServerOpcodeMessage::SMSG_SPELL_COOLDOWN(_) => "SMSG_SPELL_COOLDOWN",
            ServerOpcodeMessage::SMSG_COOLDOWN_EVENT(_) => "SMSG_COOLDOWN_EVENT",
            ServerOpcodeMessage::SMSG_PET_CAST_FAILED(_) => "SMSG_PET_CAST_FAILED",
            ServerOpcodeMessage::SMSG_AI_REACTION(_) => "SMSG_AI_REACTION",
            ServerOpcodeMessage::SMSG_ATTACKSTART(_) => "SMSG_ATTACKSTART",
            ServerOpcodeMessage::SMSG_ATTACKSTOP(_) => "SMSG_ATTACKSTOP",
            ServerOpcodeMessage::SMSG_ATTACKSWING_NOTINRANGE(_) => "SMSG_ATTACKSWING_NOTINRANGE",
            ServerOpcodeMessage::SMSG_ATTACKSWING_BADFACING(_) => "SMSG_ATTACKSWING_BADFACING",
            ServerOpcodeMessage::SMSG_ATTACKSWING_DEADTARGET(_) => "SMSG_ATTACKSWING_DEADTARGET",
            ServerOpcodeMessage::SMSG_ATTACKSWING_CANT_ATTACK(_) => "SMSG_ATTACKSWING_CANT_ATTACK",
            ServerOpcodeMessage::SMSG_ATTACKERSTATEUPDATE(_) => "SMSG_ATTACKERSTATEUPDATE",
            ServerOpcodeMessage::SMSG_CANCEL_COMBAT(_) => "SMSG_CANCEL_COMBAT",
            ServerOpcodeMessage::SMSG_SPELLHEALLOG(_) => "SMSG_SPELLHEALLOG",
            ServerOpcodeMessage::SMSG_SPELLENERGIZELOG(_) => "SMSG_SPELLENERGIZELOG",
            ServerOpcodeMessage::SMSG_BINDPOINTUPDATE(_) => "SMSG_BINDPOINTUPDATE",
            ServerOpcodeMessage::SMSG_PLAYERBOUND(_) => "SMSG_PLAYERBOUND",
            ServerOpcodeMessage::SMSG_CLIENT_CONTROL_UPDATE(_) => "SMSG_CLIENT_CONTROL_UPDATE",
            ServerOpcodeMessage::SMSG_RESURRECT_REQUEST(_) => "SMSG_RESURRECT_REQUEST",
            ServerOpcodeMessage::SMSG_LOOT_RESPONSE(_) => "SMSG_LOOT_RESPONSE",
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
            ServerOpcodeMessage::SMSG_PET_TAME_FAILURE(_) => "SMSG_PET_TAME_FAILURE",
            ServerOpcodeMessage::SMSG_PET_NAME_INVALID(_) => "SMSG_PET_NAME_INVALID",
            ServerOpcodeMessage::SMSG_PET_SPELLS(_) => "SMSG_PET_SPELLS",
            ServerOpcodeMessage::SMSG_PET_MODE(_) => "SMSG_PET_MODE",
            ServerOpcodeMessage::SMSG_GOSSIP_MESSAGE(_) => "SMSG_GOSSIP_MESSAGE",
            ServerOpcodeMessage::SMSG_GOSSIP_COMPLETE(_) => "SMSG_GOSSIP_COMPLETE",
            ServerOpcodeMessage::SMSG_NPC_TEXT_UPDATE(_) => "SMSG_NPC_TEXT_UPDATE",
            ServerOpcodeMessage::SMSG_QUESTGIVER_STATUS(_) => "SMSG_QUESTGIVER_STATUS",
            ServerOpcodeMessage::SMSG_QUESTGIVER_QUEST_LIST(_) => "SMSG_QUESTGIVER_QUEST_LIST",
            ServerOpcodeMessage::SMSG_QUESTGIVER_QUEST_DETAILS(_) => "SMSG_QUESTGIVER_QUEST_DETAILS",
            ServerOpcodeMessage::SMSG_QUESTGIVER_REQUEST_ITEMS(_) => "SMSG_QUESTGIVER_REQUEST_ITEMS",
            ServerOpcodeMessage::SMSG_QUESTGIVER_OFFER_REWARD(_) => "SMSG_QUESTGIVER_OFFER_REWARD",
            ServerOpcodeMessage::SMSG_QUESTGIVER_QUEST_INVALID(_) => "SMSG_QUESTGIVER_QUEST_INVALID",
            ServerOpcodeMessage::SMSG_QUESTGIVER_QUEST_COMPLETE(_) => "SMSG_QUESTGIVER_QUEST_COMPLETE",
            ServerOpcodeMessage::SMSG_QUESTGIVER_QUEST_FAILED(_) => "SMSG_QUESTGIVER_QUEST_FAILED",
            ServerOpcodeMessage::SMSG_QUESTLOG_FULL(_) => "SMSG_QUESTLOG_FULL",
            ServerOpcodeMessage::SMSG_QUESTUPDATE_FAILED(_) => "SMSG_QUESTUPDATE_FAILED",
            ServerOpcodeMessage::SMSG_QUESTUPDATE_FAILEDTIMER(_) => "SMSG_QUESTUPDATE_FAILEDTIMER",
            ServerOpcodeMessage::SMSG_QUESTUPDATE_COMPLETE(_) => "SMSG_QUESTUPDATE_COMPLETE",
            ServerOpcodeMessage::SMSG_QUESTUPDATE_ADD_KILL(_) => "SMSG_QUESTUPDATE_ADD_KILL",
            ServerOpcodeMessage::SMSG_QUESTUPDATE_ADD_ITEM(_) => "SMSG_QUESTUPDATE_ADD_ITEM",
            ServerOpcodeMessage::SMSG_QUEST_CONFIRM_ACCEPT(_) => "SMSG_QUEST_CONFIRM_ACCEPT",
            ServerOpcodeMessage::SMSG_LIST_INVENTORY(_) => "SMSG_LIST_INVENTORY",
            ServerOpcodeMessage::SMSG_SELL_ITEM(_) => "SMSG_SELL_ITEM",
            ServerOpcodeMessage::SMSG_BUY_ITEM(_) => "SMSG_BUY_ITEM",
            ServerOpcodeMessage::SMSG_BUY_FAILED(_) => "SMSG_BUY_FAILED",
            ServerOpcodeMessage::SMSG_SHOWTAXINODES(_) => "SMSG_SHOWTAXINODES",
            ServerOpcodeMessage::SMSG_TAXINODE_STATUS(_) => "SMSG_TAXINODE_STATUS",
            ServerOpcodeMessage::SMSG_ACTIVATETAXIREPLY(_) => "SMSG_ACTIVATETAXIREPLY",
            ServerOpcodeMessage::SMSG_NEW_TAXI_PATH(_) => "SMSG_NEW_TAXI_PATH",
            ServerOpcodeMessage::SMSG_TRAINER_LIST(_) => "SMSG_TRAINER_LIST",
            ServerOpcodeMessage::SMSG_TRAINER_BUY_SUCCEEDED(_) => "SMSG_TRAINER_BUY_SUCCEEDED",
            ServerOpcodeMessage::SMSG_TRAINER_BUY_FAILED(_) => "SMSG_TRAINER_BUY_FAILED",
            ServerOpcodeMessage::SMSG_SHOW_BANK(_) => "SMSG_SHOW_BANK",
            ServerOpcodeMessage::SMSG_BUY_BANK_SLOT_RESULT(_) => "SMSG_BUY_BANK_SLOT_RESULT",
            ServerOpcodeMessage::SMSG_PETITION_SHOWLIST(_) => "SMSG_PETITION_SHOWLIST",
            ServerOpcodeMessage::SMSG_PETITION_SHOW_SIGNATURES(_) => "SMSG_PETITION_SHOW_SIGNATURES",
            ServerOpcodeMessage::SMSG_PETITION_SIGN_RESULTS(_) => "SMSG_PETITION_SIGN_RESULTS",
            ServerOpcodeMessage::SMSG_TURN_IN_PETITION_RESULTS(_) => "SMSG_TURN_IN_PETITION_RESULTS",
            ServerOpcodeMessage::SMSG_PETITION_QUERY_RESPONSE(_) => "SMSG_PETITION_QUERY_RESPONSE",
            ServerOpcodeMessage::SMSG_FISH_NOT_HOOKED(_) => "SMSG_FISH_NOT_HOOKED",
            ServerOpcodeMessage::SMSG_FISH_ESCAPED(_) => "SMSG_FISH_ESCAPED",
            ServerOpcodeMessage::SMSG_NOTIFICATION(_) => "SMSG_NOTIFICATION",
            ServerOpcodeMessage::SMSG_PLAYED_TIME(_) => "SMSG_PLAYED_TIME",
            ServerOpcodeMessage::SMSG_QUERY_TIME_RESPONSE(_) => "SMSG_QUERY_TIME_RESPONSE",
            ServerOpcodeMessage::SMSG_LOG_XPGAIN(_) => "SMSG_LOG_XPGAIN",
            ServerOpcodeMessage::SMSG_LEVELUP_INFO(_) => "SMSG_LEVELUP_INFO",
            ServerOpcodeMessage::MSG_MINIMAP_PING(_) => "MSG_MINIMAP_PING_Server",
            ServerOpcodeMessage::SMSG_PONG(_) => "SMSG_PONG",
            ServerOpcodeMessage::SMSG_GAMEOBJECT_PAGETEXT(_) => "SMSG_GAMEOBJECT_PAGETEXT",
            ServerOpcodeMessage::SMSG_ITEM_TIME_UPDATE(_) => "SMSG_ITEM_TIME_UPDATE",
            ServerOpcodeMessage::SMSG_ITEM_ENCHANT_TIME_UPDATE(_) => "SMSG_ITEM_ENCHANT_TIME_UPDATE",
            ServerOpcodeMessage::SMSG_AUTH_CHALLENGE(_) => "SMSG_AUTH_CHALLENGE",
            ServerOpcodeMessage::SMSG_AUTH_RESPONSE(_) => "SMSG_AUTH_RESPONSE",
            ServerOpcodeMessage::MSG_SAVE_GUILD_EMBLEM(_) => "MSG_SAVE_GUILD_EMBLEM_Server",
            ServerOpcodeMessage::SMSG_EXPLORATION_EXPERIENCE(_) => "SMSG_EXPLORATION_EXPERIENCE",
            ServerOpcodeMessage::MSG_RANDOM_ROLL(_) => "MSG_RANDOM_ROLL_Server",
            ServerOpcodeMessage::SMSG_ENVIRONMENTAL_DAMAGE_LOG(_) => "SMSG_ENVIRONMENTAL_DAMAGE_LOG",
            ServerOpcodeMessage::SMSG_GMTICKET_CREATE(_) => "SMSG_GMTICKET_CREATE",
            ServerOpcodeMessage::SMSG_GMTICKET_UPDATETEXT(_) => "SMSG_GMTICKET_UPDATETEXT",
            ServerOpcodeMessage::SMSG_ACCOUNT_DATA_TIMES(_) => "SMSG_ACCOUNT_DATA_TIMES",
            ServerOpcodeMessage::SMSG_UPDATE_ACCOUNT_DATA(_) => "SMSG_UPDATE_ACCOUNT_DATA",
            ServerOpcodeMessage::SMSG_GMTICKET_GETTICKET(_) => "SMSG_GMTICKET_GETTICKET",
            ServerOpcodeMessage::SMSG_GAMEOBJECT_DESPAWN_ANIM(_) => "SMSG_GAMEOBJECT_DESPAWN_ANIM",
            ServerOpcodeMessage::MSG_CORPSE_QUERY(_) => "MSG_CORPSE_QUERY_Server",
            ServerOpcodeMessage::SMSG_GMTICKET_DELETETICKET(_) => "SMSG_GMTICKET_DELETETICKET",
            ServerOpcodeMessage::SMSG_CHAT_WRONG_FACTION(_) => "SMSG_CHAT_WRONG_FACTION",
            ServerOpcodeMessage::SMSG_GMTICKET_SYSTEMSTATUS(_) => "SMSG_GMTICKET_SYSTEMSTATUS",
            ServerOpcodeMessage::SMSG_QUEST_FORCE_REMOVE(_) => "SMSG_QUEST_FORCE_REMOVE",
            ServerOpcodeMessage::SMSG_GOSSIP_POI(_) => "SMSG_GOSSIP_POI",
            ServerOpcodeMessage::SMSG_LOGIN_VERIFY_WORLD(_) => "SMSG_LOGIN_VERIFY_WORLD",
            ServerOpcodeMessage::SMSG_BATTLEFIELD_LIST(_) => "SMSG_BATTLEFIELD_LIST",
            ServerOpcodeMessage::SMSG_ITEM_TEXT_QUERY_RESPONSE(_) => "SMSG_ITEM_TEXT_QUERY_RESPONSE",
            ServerOpcodeMessage::SMSG_ZONE_UNDER_ATTACK(_) => "SMSG_ZONE_UNDER_ATTACK",
            ServerOpcodeMessage::MSG_AUCTION_HELLO(_) => "MSG_AUCTION_HELLO_Server",
            ServerOpcodeMessage::SMSG_AUCTION_LIST_RESULT(_) => "SMSG_AUCTION_LIST_RESULT",
            ServerOpcodeMessage::SMSG_AUCTION_OWNER_LIST_RESULT(_) => "SMSG_AUCTION_OWNER_LIST_RESULT",
            ServerOpcodeMessage::SMSG_AUCTION_BIDDER_NOTIFICATION(_) => "SMSG_AUCTION_BIDDER_NOTIFICATION",
            ServerOpcodeMessage::SMSG_AUCTION_OWNER_NOTIFICATION(_) => "SMSG_AUCTION_OWNER_NOTIFICATION",
            ServerOpcodeMessage::SMSG_PROCRESIST(_) => "SMSG_PROCRESIST",
            ServerOpcodeMessage::SMSG_AUCTION_BIDDER_LIST_RESULT(_) => "SMSG_AUCTION_BIDDER_LIST_RESULT",
            ServerOpcodeMessage::MSG_LIST_STABLED_PETS(_) => "MSG_LIST_STABLED_PETS_Server",
            ServerOpcodeMessage::SMSG_STABLE_RESULT(_) => "SMSG_STABLE_RESULT",
            ServerOpcodeMessage::SMSG_PLAY_OBJECT_SOUND(_) => "SMSG_PLAY_OBJECT_SOUND",
            ServerOpcodeMessage::MSG_QUERY_NEXT_MAIL_TIME(_) => "MSG_QUERY_NEXT_MAIL_TIME_Server",
            ServerOpcodeMessage::SMSG_RECEIVED_MAIL(_) => "SMSG_RECEIVED_MAIL",
            ServerOpcodeMessage::SMSG_PVP_CREDIT(_) => "SMSG_PVP_CREDIT",
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
            ServerOpcodeMessage::SMSG_PET_BROKEN(_) => "SMSG_PET_BROKEN",
            ServerOpcodeMessage::SMSG_DUEL_COUNTDOWN(_) => "SMSG_DUEL_COUNTDOWN",
            ServerOpcodeMessage::SMSG_AREA_TRIGGER_MESSAGE(_) => "SMSG_AREA_TRIGGER_MESSAGE",
            ServerOpcodeMessage::SMSG_DURABILITY_DAMAGE_DEATH(_) => "SMSG_DURABILITY_DAMAGE_DEATH",
            ServerOpcodeMessage::SMSG_INIT_WORLD_STATES(_) => "SMSG_INIT_WORLD_STATES",
            ServerOpcodeMessage::SMSG_UPDATE_WORLD_STATE(_) => "SMSG_UPDATE_WORLD_STATE",
            ServerOpcodeMessage::SMSG_ITEM_NAME_QUERY_RESPONSE(_) => "SMSG_ITEM_NAME_QUERY_RESPONSE",
            ServerOpcodeMessage::SMSG_PET_ACTION_FEEDBACK(_) => "SMSG_PET_ACTION_FEEDBACK",
            ServerOpcodeMessage::SMSG_CHAR_RENAME(_) => "SMSG_CHAR_RENAME",
            ServerOpcodeMessage::SMSG_RAID_INSTANCE_INFO(_) => "SMSG_RAID_INSTANCE_INFO",
            ServerOpcodeMessage::MSG_INSPECT_HONOR_STATS(_) => "MSG_INSPECT_HONOR_STATS_Server",
            ServerOpcodeMessage::SMSG_FORCE_WALK_SPEED_CHANGE(_) => "SMSG_FORCE_WALK_SPEED_CHANGE",
            ServerOpcodeMessage::SMSG_FORCE_SWIM_BACK_SPEED_CHANGE(_) => "SMSG_FORCE_SWIM_BACK_SPEED_CHANGE",
            ServerOpcodeMessage::SMSG_FORCE_TURN_RATE_CHANGE(_) => "SMSG_FORCE_TURN_RATE_CHANGE",
            ServerOpcodeMessage::MSG_BATTLEGROUND_PLAYER_POSITIONS(_) => "MSG_BATTLEGROUND_PLAYER_POSITIONS_Server",
            ServerOpcodeMessage::SMSG_BINDER_CONFIRM(_) => "SMSG_BINDER_CONFIRM",
            ServerOpcodeMessage::SMSG_BATTLEGROUND_PLAYER_JOINED(_) => "SMSG_BATTLEGROUND_PLAYER_JOINED",
            ServerOpcodeMessage::SMSG_BATTLEGROUND_PLAYER_LEFT(_) => "SMSG_BATTLEGROUND_PLAYER_LEFT",
            ServerOpcodeMessage::SMSG_ADDON_INFO(_) => "SMSG_ADDON_INFO",
            ServerOpcodeMessage::SMSG_PET_UNLEARN_CONFIRM(_) => "SMSG_PET_UNLEARN_CONFIRM",
            ServerOpcodeMessage::SMSG_CHAT_RESTRICTED(_) => "SMSG_CHAT_RESTRICTED",
            ServerOpcodeMessage::SMSG_SPLINE_SET_RUN_SPEED(_) => "SMSG_SPLINE_SET_RUN_SPEED",
            ServerOpcodeMessage::SMSG_SPLINE_SET_RUN_BACK_SPEED(_) => "SMSG_SPLINE_SET_RUN_BACK_SPEED",
            ServerOpcodeMessage::SMSG_SPLINE_SET_SWIM_SPEED(_) => "SMSG_SPLINE_SET_SWIM_SPEED",
            ServerOpcodeMessage::SMSG_SPLINE_SET_WALK_SPEED(_) => "SMSG_SPLINE_SET_WALK_SPEED",
            ServerOpcodeMessage::SMSG_SPLINE_SET_SWIM_BACK_SPEED(_) => "SMSG_SPLINE_SET_SWIM_BACK_SPEED",
            ServerOpcodeMessage::SMSG_SPLINE_SET_TURN_RATE(_) => "SMSG_SPLINE_SET_TURN_RATE",
            ServerOpcodeMessage::SMSG_SPLINE_MOVE_UNROOT(_) => "SMSG_SPLINE_MOVE_UNROOT",
            ServerOpcodeMessage::SMSG_SPLINE_MOVE_FEATHER_FALL(_) => "SMSG_SPLINE_MOVE_FEATHER_FALL",
            ServerOpcodeMessage::SMSG_SPLINE_MOVE_NORMAL_FALL(_) => "SMSG_SPLINE_MOVE_NORMAL_FALL",
            ServerOpcodeMessage::SMSG_SPLINE_MOVE_SET_HOVER(_) => "SMSG_SPLINE_MOVE_SET_HOVER",
            ServerOpcodeMessage::SMSG_SPLINE_MOVE_UNSET_HOVER(_) => "SMSG_SPLINE_MOVE_UNSET_HOVER",
            ServerOpcodeMessage::SMSG_SPLINE_MOVE_WATER_WALK(_) => "SMSG_SPLINE_MOVE_WATER_WALK",
            ServerOpcodeMessage::SMSG_SPLINE_MOVE_LAND_WALK(_) => "SMSG_SPLINE_MOVE_LAND_WALK",
            ServerOpcodeMessage::SMSG_SPLINE_MOVE_START_SWIM(_) => "SMSG_SPLINE_MOVE_START_SWIM",
            ServerOpcodeMessage::SMSG_SPLINE_MOVE_STOP_SWIM(_) => "SMSG_SPLINE_MOVE_STOP_SWIM",
            ServerOpcodeMessage::SMSG_SPLINE_MOVE_SET_RUN_MODE(_) => "SMSG_SPLINE_MOVE_SET_RUN_MODE",
            ServerOpcodeMessage::SMSG_SPLINE_MOVE_SET_WALK_MODE(_) => "SMSG_SPLINE_MOVE_SET_WALK_MODE",
            ServerOpcodeMessage::SMSG_SPLINE_MOVE_ROOT(_) => "SMSG_SPLINE_MOVE_ROOT",
            ServerOpcodeMessage::MSG_RAID_READY_CHECK(_) => "MSG_RAID_READY_CHECK_Server",
            ServerOpcodeMessage::SMSG_PET_ACTION_SOUND(_) => "SMSG_PET_ACTION_SOUND",
            ServerOpcodeMessage::SMSG_PET_DISMISS_SOUND(_) => "SMSG_PET_DISMISS_SOUND",
            ServerOpcodeMessage::SMSG_GM_TICKET_STATUS_UPDATE(_) => "SMSG_GM_TICKET_STATUS_UPDATE",
            ServerOpcodeMessage::SMSG_DEFENSE_MESSAGE(_) => "SMSG_DEFENSE_MESSAGE",
            ServerOpcodeMessage::SMSG_REALM_SPLIT(_) => "SMSG_REALM_SPLIT",
            ServerOpcodeMessage::SMSG_TIME_SYNC_REQ(_) => "SMSG_TIME_SYNC_REQ",
            ServerOpcodeMessage::SMSG_GM_MESSAGECHAT(_) => "SMSG_GM_MESSAGECHAT",
            ServerOpcodeMessage::SMSG_FEATURE_SYSTEM_STATUS(_) => "SMSG_FEATURE_SYSTEM_STATUS",
            ServerOpcodeMessage::SMSG_CALENDAR_SEND_NUM_PENDING(_) => "SMSG_CALENDAR_SEND_NUM_PENDING",
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

impl From<MSG_QUEST_PUSH_RESULT> for ServerOpcodeMessage {
    fn from(c: MSG_QUEST_PUSH_RESULT) -> Self {
        Self::MSG_QUEST_PUSH_RESULT(c)
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

impl From<MSG_GUILD_BANK_MONEY_WITHDRAWN> for ServerOpcodeMessage {
    fn from(c: MSG_GUILD_BANK_MONEY_WITHDRAWN) -> Self {
        Self::MSG_GUILD_BANK_MONEY_WITHDRAWN(c)
    }
}

impl From<CMSG_CALENDAR_GET_NUM_PENDING> for ServerOpcodeMessage {
    fn from(c: CMSG_CALENDAR_GET_NUM_PENDING) -> Self {
        Self::CMSG_CALENDAR_GET_NUM_PENDING(c)
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

impl From<SMSG_TRANSFER_ABORTED> for ServerOpcodeMessage {
    fn from(c: SMSG_TRANSFER_ABORTED) -> Self {
        Self::SMSG_TRANSFER_ABORTED(c)
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

impl From<SMSG_PET_NAME_QUERY_RESPONSE> for ServerOpcodeMessage {
    fn from(c: SMSG_PET_NAME_QUERY_RESPONSE) -> Self {
        Self::SMSG_PET_NAME_QUERY_RESPONSE(c)
    }
}

impl From<SMSG_GUILD_QUERY_RESPONSE> for ServerOpcodeMessage {
    fn from(c: SMSG_GUILD_QUERY_RESPONSE) -> Self {
        Self::SMSG_GUILD_QUERY_RESPONSE(c)
    }
}

impl From<SMSG_ITEM_QUERY_SINGLE_RESPONSE> for ServerOpcodeMessage {
    fn from(c: SMSG_ITEM_QUERY_SINGLE_RESPONSE) -> Self {
        Self::SMSG_ITEM_QUERY_SINGLE_RESPONSE(c)
    }
}

impl From<SMSG_PAGE_TEXT_QUERY_RESPONSE> for ServerOpcodeMessage {
    fn from(c: SMSG_PAGE_TEXT_QUERY_RESPONSE) -> Self {
        Self::SMSG_PAGE_TEXT_QUERY_RESPONSE(c)
    }
}

impl From<SMSG_QUEST_QUERY_RESPONSE> for ServerOpcodeMessage {
    fn from(c: SMSG_QUEST_QUERY_RESPONSE) -> Self {
        Self::SMSG_QUEST_QUERY_RESPONSE(c)
    }
}

impl From<SMSG_GAMEOBJECT_QUERY_RESPONSE> for ServerOpcodeMessage {
    fn from(c: SMSG_GAMEOBJECT_QUERY_RESPONSE) -> Self {
        Self::SMSG_GAMEOBJECT_QUERY_RESPONSE(c)
    }
}

impl From<SMSG_CREATURE_QUERY_RESPONSE> for ServerOpcodeMessage {
    fn from(c: SMSG_CREATURE_QUERY_RESPONSE) -> Self {
        Self::SMSG_CREATURE_QUERY_RESPONSE(c)
    }
}

impl From<SMSG_WHO> for ServerOpcodeMessage {
    fn from(c: SMSG_WHO) -> Self {
        Self::SMSG_WHO(c)
    }
}

impl From<SMSG_WHOIS> for ServerOpcodeMessage {
    fn from(c: SMSG_WHOIS) -> Self {
        Self::SMSG_WHOIS(c)
    }
}

impl From<SMSG_CONTACT_LIST> for ServerOpcodeMessage {
    fn from(c: SMSG_CONTACT_LIST) -> Self {
        Self::SMSG_CONTACT_LIST(c)
    }
}

impl From<SMSG_FRIEND_STATUS> for ServerOpcodeMessage {
    fn from(c: SMSG_FRIEND_STATUS) -> Self {
        Self::SMSG_FRIEND_STATUS(c)
    }
}

impl From<SMSG_GROUP_INVITE> for ServerOpcodeMessage {
    fn from(c: SMSG_GROUP_INVITE) -> Self {
        Self::SMSG_GROUP_INVITE(c)
    }
}

impl From<SMSG_GROUP_DECLINE> for ServerOpcodeMessage {
    fn from(c: SMSG_GROUP_DECLINE) -> Self {
        Self::SMSG_GROUP_DECLINE(c)
    }
}

impl From<SMSG_GROUP_UNINVITE> for ServerOpcodeMessage {
    fn from(c: SMSG_GROUP_UNINVITE) -> Self {
        Self::SMSG_GROUP_UNINVITE(c)
    }
}

impl From<SMSG_GROUP_SET_LEADER> for ServerOpcodeMessage {
    fn from(c: SMSG_GROUP_SET_LEADER) -> Self {
        Self::SMSG_GROUP_SET_LEADER(c)
    }
}

impl From<SMSG_GROUP_DESTROYED> for ServerOpcodeMessage {
    fn from(c: SMSG_GROUP_DESTROYED) -> Self {
        Self::SMSG_GROUP_DESTROYED(c)
    }
}

impl From<SMSG_GROUP_LIST> for ServerOpcodeMessage {
    fn from(c: SMSG_GROUP_LIST) -> Self {
        Self::SMSG_GROUP_LIST(c)
    }
}

impl From<SMSG_PARTY_COMMAND_RESULT> for ServerOpcodeMessage {
    fn from(c: SMSG_PARTY_COMMAND_RESULT) -> Self {
        Self::SMSG_PARTY_COMMAND_RESULT(c)
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

impl From<SMSG_GUILD_ROSTER> for ServerOpcodeMessage {
    fn from(c: SMSG_GUILD_ROSTER) -> Self {
        Self::SMSG_GUILD_ROSTER(c)
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

impl From<SMSG_MESSAGECHAT> for ServerOpcodeMessage {
    fn from(c: SMSG_MESSAGECHAT) -> Self {
        Self::SMSG_MESSAGECHAT(c)
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

impl From<SMSG_MOVE_WATER_WALK> for ServerOpcodeMessage {
    fn from(c: SMSG_MOVE_WATER_WALK) -> Self {
        Self::SMSG_MOVE_WATER_WALK(c)
    }
}

impl From<SMSG_MOVE_LAND_WALK> for ServerOpcodeMessage {
    fn from(c: SMSG_MOVE_LAND_WALK) -> Self {
        Self::SMSG_MOVE_LAND_WALK(c)
    }
}

impl From<SMSG_FORCE_RUN_SPEED_CHANGE> for ServerOpcodeMessage {
    fn from(c: SMSG_FORCE_RUN_SPEED_CHANGE) -> Self {
        Self::SMSG_FORCE_RUN_SPEED_CHANGE(c)
    }
}

impl From<SMSG_FORCE_RUN_BACK_SPEED_CHANGE> for ServerOpcodeMessage {
    fn from(c: SMSG_FORCE_RUN_BACK_SPEED_CHANGE) -> Self {
        Self::SMSG_FORCE_RUN_BACK_SPEED_CHANGE(c)
    }
}

impl From<SMSG_FORCE_SWIM_SPEED_CHANGE> for ServerOpcodeMessage {
    fn from(c: SMSG_FORCE_SWIM_SPEED_CHANGE) -> Self {
        Self::SMSG_FORCE_SWIM_SPEED_CHANGE(c)
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

impl From<SMSG_MOVE_KNOCK_BACK> for ServerOpcodeMessage {
    fn from(c: SMSG_MOVE_KNOCK_BACK) -> Self {
        Self::SMSG_MOVE_KNOCK_BACK(c)
    }
}

impl From<SMSG_MOVE_FEATHER_FALL> for ServerOpcodeMessage {
    fn from(c: SMSG_MOVE_FEATHER_FALL) -> Self {
        Self::SMSG_MOVE_FEATHER_FALL(c)
    }
}

impl From<SMSG_MOVE_NORMAL_FALL> for ServerOpcodeMessage {
    fn from(c: SMSG_MOVE_NORMAL_FALL) -> Self {
        Self::SMSG_MOVE_NORMAL_FALL(c)
    }
}

impl From<SMSG_MOVE_SET_HOVER> for ServerOpcodeMessage {
    fn from(c: SMSG_MOVE_SET_HOVER) -> Self {
        Self::SMSG_MOVE_SET_HOVER(c)
    }
}

impl From<SMSG_MOVE_UNSET_HOVER> for ServerOpcodeMessage {
    fn from(c: SMSG_MOVE_UNSET_HOVER) -> Self {
        Self::SMSG_MOVE_UNSET_HOVER(c)
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

impl From<SMSG_TRADE_STATUS> for ServerOpcodeMessage {
    fn from(c: SMSG_TRADE_STATUS) -> Self {
        Self::SMSG_TRADE_STATUS(c)
    }
}

impl From<SMSG_TRADE_STATUS_EXTENDED> for ServerOpcodeMessage {
    fn from(c: SMSG_TRADE_STATUS_EXTENDED) -> Self {
        Self::SMSG_TRADE_STATUS_EXTENDED(c)
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

impl From<SMSG_INITIAL_SPELLS> for ServerOpcodeMessage {
    fn from(c: SMSG_INITIAL_SPELLS) -> Self {
        Self::SMSG_INITIAL_SPELLS(c)
    }
}

impl From<SMSG_LEARNED_SPELL> for ServerOpcodeMessage {
    fn from(c: SMSG_LEARNED_SPELL) -> Self {
        Self::SMSG_LEARNED_SPELL(c)
    }
}

impl From<SMSG_SUPERCEDED_SPELL> for ServerOpcodeMessage {
    fn from(c: SMSG_SUPERCEDED_SPELL) -> Self {
        Self::SMSG_SUPERCEDED_SPELL(c)
    }
}

impl From<SMSG_SPELL_START> for ServerOpcodeMessage {
    fn from(c: SMSG_SPELL_START) -> Self {
        Self::SMSG_SPELL_START(c)
    }
}

impl From<SMSG_SPELL_GO> for ServerOpcodeMessage {
    fn from(c: SMSG_SPELL_GO) -> Self {
        Self::SMSG_SPELL_GO(c)
    }
}

impl From<SMSG_SPELL_FAILURE> for ServerOpcodeMessage {
    fn from(c: SMSG_SPELL_FAILURE) -> Self {
        Self::SMSG_SPELL_FAILURE(c)
    }
}

impl From<SMSG_SPELL_COOLDOWN> for ServerOpcodeMessage {
    fn from(c: SMSG_SPELL_COOLDOWN) -> Self {
        Self::SMSG_SPELL_COOLDOWN(c)
    }
}

impl From<SMSG_COOLDOWN_EVENT> for ServerOpcodeMessage {
    fn from(c: SMSG_COOLDOWN_EVENT) -> Self {
        Self::SMSG_COOLDOWN_EVENT(c)
    }
}

impl From<SMSG_PET_CAST_FAILED> for ServerOpcodeMessage {
    fn from(c: SMSG_PET_CAST_FAILED) -> Self {
        Self::SMSG_PET_CAST_FAILED(c)
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

impl From<SMSG_SPELLHEALLOG> for ServerOpcodeMessage {
    fn from(c: SMSG_SPELLHEALLOG) -> Self {
        Self::SMSG_SPELLHEALLOG(c)
    }
}

impl From<SMSG_SPELLENERGIZELOG> for ServerOpcodeMessage {
    fn from(c: SMSG_SPELLENERGIZELOG) -> Self {
        Self::SMSG_SPELLENERGIZELOG(c)
    }
}

impl From<SMSG_BINDPOINTUPDATE> for ServerOpcodeMessage {
    fn from(c: SMSG_BINDPOINTUPDATE) -> Self {
        Self::SMSG_BINDPOINTUPDATE(c)
    }
}

impl From<SMSG_PLAYERBOUND> for ServerOpcodeMessage {
    fn from(c: SMSG_PLAYERBOUND) -> Self {
        Self::SMSG_PLAYERBOUND(c)
    }
}

impl From<SMSG_CLIENT_CONTROL_UPDATE> for ServerOpcodeMessage {
    fn from(c: SMSG_CLIENT_CONTROL_UPDATE) -> Self {
        Self::SMSG_CLIENT_CONTROL_UPDATE(c)
    }
}

impl From<SMSG_RESURRECT_REQUEST> for ServerOpcodeMessage {
    fn from(c: SMSG_RESURRECT_REQUEST) -> Self {
        Self::SMSG_RESURRECT_REQUEST(c)
    }
}

impl From<SMSG_LOOT_RESPONSE> for ServerOpcodeMessage {
    fn from(c: SMSG_LOOT_RESPONSE) -> Self {
        Self::SMSG_LOOT_RESPONSE(c)
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

impl From<SMSG_PET_TAME_FAILURE> for ServerOpcodeMessage {
    fn from(c: SMSG_PET_TAME_FAILURE) -> Self {
        Self::SMSG_PET_TAME_FAILURE(c)
    }
}

impl From<SMSG_PET_NAME_INVALID> for ServerOpcodeMessage {
    fn from(c: SMSG_PET_NAME_INVALID) -> Self {
        Self::SMSG_PET_NAME_INVALID(c)
    }
}

impl From<SMSG_PET_SPELLS> for ServerOpcodeMessage {
    fn from(c: SMSG_PET_SPELLS) -> Self {
        Self::SMSG_PET_SPELLS(c)
    }
}

impl From<SMSG_PET_MODE> for ServerOpcodeMessage {
    fn from(c: SMSG_PET_MODE) -> Self {
        Self::SMSG_PET_MODE(c)
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

impl From<SMSG_QUESTGIVER_STATUS> for ServerOpcodeMessage {
    fn from(c: SMSG_QUESTGIVER_STATUS) -> Self {
        Self::SMSG_QUESTGIVER_STATUS(c)
    }
}

impl From<SMSG_QUESTGIVER_QUEST_LIST> for ServerOpcodeMessage {
    fn from(c: SMSG_QUESTGIVER_QUEST_LIST) -> Self {
        Self::SMSG_QUESTGIVER_QUEST_LIST(c)
    }
}

impl From<SMSG_QUESTGIVER_QUEST_DETAILS> for ServerOpcodeMessage {
    fn from(c: SMSG_QUESTGIVER_QUEST_DETAILS) -> Self {
        Self::SMSG_QUESTGIVER_QUEST_DETAILS(c)
    }
}

impl From<SMSG_QUESTGIVER_REQUEST_ITEMS> for ServerOpcodeMessage {
    fn from(c: SMSG_QUESTGIVER_REQUEST_ITEMS) -> Self {
        Self::SMSG_QUESTGIVER_REQUEST_ITEMS(c)
    }
}

impl From<SMSG_QUESTGIVER_OFFER_REWARD> for ServerOpcodeMessage {
    fn from(c: SMSG_QUESTGIVER_OFFER_REWARD) -> Self {
        Self::SMSG_QUESTGIVER_OFFER_REWARD(c)
    }
}

impl From<SMSG_QUESTGIVER_QUEST_INVALID> for ServerOpcodeMessage {
    fn from(c: SMSG_QUESTGIVER_QUEST_INVALID) -> Self {
        Self::SMSG_QUESTGIVER_QUEST_INVALID(c)
    }
}

impl From<SMSG_QUESTGIVER_QUEST_COMPLETE> for ServerOpcodeMessage {
    fn from(c: SMSG_QUESTGIVER_QUEST_COMPLETE) -> Self {
        Self::SMSG_QUESTGIVER_QUEST_COMPLETE(c)
    }
}

impl From<SMSG_QUESTGIVER_QUEST_FAILED> for ServerOpcodeMessage {
    fn from(c: SMSG_QUESTGIVER_QUEST_FAILED) -> Self {
        Self::SMSG_QUESTGIVER_QUEST_FAILED(c)
    }
}

impl From<SMSG_QUESTLOG_FULL> for ServerOpcodeMessage {
    fn from(c: SMSG_QUESTLOG_FULL) -> Self {
        Self::SMSG_QUESTLOG_FULL(c)
    }
}

impl From<SMSG_QUESTUPDATE_FAILED> for ServerOpcodeMessage {
    fn from(c: SMSG_QUESTUPDATE_FAILED) -> Self {
        Self::SMSG_QUESTUPDATE_FAILED(c)
    }
}

impl From<SMSG_QUESTUPDATE_FAILEDTIMER> for ServerOpcodeMessage {
    fn from(c: SMSG_QUESTUPDATE_FAILEDTIMER) -> Self {
        Self::SMSG_QUESTUPDATE_FAILEDTIMER(c)
    }
}

impl From<SMSG_QUESTUPDATE_COMPLETE> for ServerOpcodeMessage {
    fn from(c: SMSG_QUESTUPDATE_COMPLETE) -> Self {
        Self::SMSG_QUESTUPDATE_COMPLETE(c)
    }
}

impl From<SMSG_QUESTUPDATE_ADD_KILL> for ServerOpcodeMessage {
    fn from(c: SMSG_QUESTUPDATE_ADD_KILL) -> Self {
        Self::SMSG_QUESTUPDATE_ADD_KILL(c)
    }
}

impl From<SMSG_QUESTUPDATE_ADD_ITEM> for ServerOpcodeMessage {
    fn from(c: SMSG_QUESTUPDATE_ADD_ITEM) -> Self {
        Self::SMSG_QUESTUPDATE_ADD_ITEM(c)
    }
}

impl From<SMSG_QUEST_CONFIRM_ACCEPT> for ServerOpcodeMessage {
    fn from(c: SMSG_QUEST_CONFIRM_ACCEPT) -> Self {
        Self::SMSG_QUEST_CONFIRM_ACCEPT(c)
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

impl From<SMSG_SHOWTAXINODES> for ServerOpcodeMessage {
    fn from(c: SMSG_SHOWTAXINODES) -> Self {
        Self::SMSG_SHOWTAXINODES(c)
    }
}

impl From<SMSG_TAXINODE_STATUS> for ServerOpcodeMessage {
    fn from(c: SMSG_TAXINODE_STATUS) -> Self {
        Self::SMSG_TAXINODE_STATUS(c)
    }
}

impl From<SMSG_ACTIVATETAXIREPLY> for ServerOpcodeMessage {
    fn from(c: SMSG_ACTIVATETAXIREPLY) -> Self {
        Self::SMSG_ACTIVATETAXIREPLY(c)
    }
}

impl From<SMSG_NEW_TAXI_PATH> for ServerOpcodeMessage {
    fn from(c: SMSG_NEW_TAXI_PATH) -> Self {
        Self::SMSG_NEW_TAXI_PATH(c)
    }
}

impl From<SMSG_TRAINER_LIST> for ServerOpcodeMessage {
    fn from(c: SMSG_TRAINER_LIST) -> Self {
        Self::SMSG_TRAINER_LIST(c)
    }
}

impl From<SMSG_TRAINER_BUY_SUCCEEDED> for ServerOpcodeMessage {
    fn from(c: SMSG_TRAINER_BUY_SUCCEEDED) -> Self {
        Self::SMSG_TRAINER_BUY_SUCCEEDED(c)
    }
}

impl From<SMSG_TRAINER_BUY_FAILED> for ServerOpcodeMessage {
    fn from(c: SMSG_TRAINER_BUY_FAILED) -> Self {
        Self::SMSG_TRAINER_BUY_FAILED(c)
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

impl From<SMSG_PETITION_SHOWLIST> for ServerOpcodeMessage {
    fn from(c: SMSG_PETITION_SHOWLIST) -> Self {
        Self::SMSG_PETITION_SHOWLIST(c)
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

impl From<SMSG_PETITION_QUERY_RESPONSE> for ServerOpcodeMessage {
    fn from(c: SMSG_PETITION_QUERY_RESPONSE) -> Self {
        Self::SMSG_PETITION_QUERY_RESPONSE(c)
    }
}

impl From<SMSG_FISH_NOT_HOOKED> for ServerOpcodeMessage {
    fn from(c: SMSG_FISH_NOT_HOOKED) -> Self {
        Self::SMSG_FISH_NOT_HOOKED(c)
    }
}

impl From<SMSG_FISH_ESCAPED> for ServerOpcodeMessage {
    fn from(c: SMSG_FISH_ESCAPED) -> Self {
        Self::SMSG_FISH_ESCAPED(c)
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

impl From<MSG_MINIMAP_PING_Server> for ServerOpcodeMessage {
    fn from(c: MSG_MINIMAP_PING_Server) -> Self {
        Self::MSG_MINIMAP_PING(c)
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

impl From<SMSG_ENVIRONMENTAL_DAMAGE_LOG> for ServerOpcodeMessage {
    fn from(c: SMSG_ENVIRONMENTAL_DAMAGE_LOG) -> Self {
        Self::SMSG_ENVIRONMENTAL_DAMAGE_LOG(c)
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

impl From<SMSG_GMTICKET_GETTICKET> for ServerOpcodeMessage {
    fn from(c: SMSG_GMTICKET_GETTICKET) -> Self {
        Self::SMSG_GMTICKET_GETTICKET(c)
    }
}

impl From<SMSG_GAMEOBJECT_DESPAWN_ANIM> for ServerOpcodeMessage {
    fn from(c: SMSG_GAMEOBJECT_DESPAWN_ANIM) -> Self {
        Self::SMSG_GAMEOBJECT_DESPAWN_ANIM(c)
    }
}

impl From<MSG_CORPSE_QUERY_Server> for ServerOpcodeMessage {
    fn from(c: MSG_CORPSE_QUERY_Server) -> Self {
        Self::MSG_CORPSE_QUERY(c)
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

impl From<SMSG_QUEST_FORCE_REMOVE> for ServerOpcodeMessage {
    fn from(c: SMSG_QUEST_FORCE_REMOVE) -> Self {
        Self::SMSG_QUEST_FORCE_REMOVE(c)
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

impl From<SMSG_ITEM_TEXT_QUERY_RESPONSE> for ServerOpcodeMessage {
    fn from(c: SMSG_ITEM_TEXT_QUERY_RESPONSE) -> Self {
        Self::SMSG_ITEM_TEXT_QUERY_RESPONSE(c)
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

impl From<MSG_LIST_STABLED_PETS_Server> for ServerOpcodeMessage {
    fn from(c: MSG_LIST_STABLED_PETS_Server) -> Self {
        Self::MSG_LIST_STABLED_PETS(c)
    }
}

impl From<SMSG_STABLE_RESULT> for ServerOpcodeMessage {
    fn from(c: SMSG_STABLE_RESULT) -> Self {
        Self::SMSG_STABLE_RESULT(c)
    }
}

impl From<SMSG_PLAY_OBJECT_SOUND> for ServerOpcodeMessage {
    fn from(c: SMSG_PLAY_OBJECT_SOUND) -> Self {
        Self::SMSG_PLAY_OBJECT_SOUND(c)
    }
}

impl From<MSG_QUERY_NEXT_MAIL_TIME_Server> for ServerOpcodeMessage {
    fn from(c: MSG_QUERY_NEXT_MAIL_TIME_Server) -> Self {
        Self::MSG_QUERY_NEXT_MAIL_TIME(c)
    }
}

impl From<SMSG_RECEIVED_MAIL> for ServerOpcodeMessage {
    fn from(c: SMSG_RECEIVED_MAIL) -> Self {
        Self::SMSG_RECEIVED_MAIL(c)
    }
}

impl From<SMSG_PVP_CREDIT> for ServerOpcodeMessage {
    fn from(c: SMSG_PVP_CREDIT) -> Self {
        Self::SMSG_PVP_CREDIT(c)
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

impl From<SMSG_PET_BROKEN> for ServerOpcodeMessage {
    fn from(c: SMSG_PET_BROKEN) -> Self {
        Self::SMSG_PET_BROKEN(c)
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

impl From<SMSG_ITEM_NAME_QUERY_RESPONSE> for ServerOpcodeMessage {
    fn from(c: SMSG_ITEM_NAME_QUERY_RESPONSE) -> Self {
        Self::SMSG_ITEM_NAME_QUERY_RESPONSE(c)
    }
}

impl From<SMSG_PET_ACTION_FEEDBACK> for ServerOpcodeMessage {
    fn from(c: SMSG_PET_ACTION_FEEDBACK) -> Self {
        Self::SMSG_PET_ACTION_FEEDBACK(c)
    }
}

impl From<SMSG_CHAR_RENAME> for ServerOpcodeMessage {
    fn from(c: SMSG_CHAR_RENAME) -> Self {
        Self::SMSG_CHAR_RENAME(c)
    }
}

impl From<SMSG_RAID_INSTANCE_INFO> for ServerOpcodeMessage {
    fn from(c: SMSG_RAID_INSTANCE_INFO) -> Self {
        Self::SMSG_RAID_INSTANCE_INFO(c)
    }
}

impl From<MSG_INSPECT_HONOR_STATS_Server> for ServerOpcodeMessage {
    fn from(c: MSG_INSPECT_HONOR_STATS_Server) -> Self {
        Self::MSG_INSPECT_HONOR_STATS(c)
    }
}

impl From<SMSG_FORCE_WALK_SPEED_CHANGE> for ServerOpcodeMessage {
    fn from(c: SMSG_FORCE_WALK_SPEED_CHANGE) -> Self {
        Self::SMSG_FORCE_WALK_SPEED_CHANGE(c)
    }
}

impl From<SMSG_FORCE_SWIM_BACK_SPEED_CHANGE> for ServerOpcodeMessage {
    fn from(c: SMSG_FORCE_SWIM_BACK_SPEED_CHANGE) -> Self {
        Self::SMSG_FORCE_SWIM_BACK_SPEED_CHANGE(c)
    }
}

impl From<SMSG_FORCE_TURN_RATE_CHANGE> for ServerOpcodeMessage {
    fn from(c: SMSG_FORCE_TURN_RATE_CHANGE) -> Self {
        Self::SMSG_FORCE_TURN_RATE_CHANGE(c)
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

impl From<SMSG_PET_UNLEARN_CONFIRM> for ServerOpcodeMessage {
    fn from(c: SMSG_PET_UNLEARN_CONFIRM) -> Self {
        Self::SMSG_PET_UNLEARN_CONFIRM(c)
    }
}

impl From<SMSG_CHAT_RESTRICTED> for ServerOpcodeMessage {
    fn from(c: SMSG_CHAT_RESTRICTED) -> Self {
        Self::SMSG_CHAT_RESTRICTED(c)
    }
}

impl From<SMSG_SPLINE_SET_RUN_SPEED> for ServerOpcodeMessage {
    fn from(c: SMSG_SPLINE_SET_RUN_SPEED) -> Self {
        Self::SMSG_SPLINE_SET_RUN_SPEED(c)
    }
}

impl From<SMSG_SPLINE_SET_RUN_BACK_SPEED> for ServerOpcodeMessage {
    fn from(c: SMSG_SPLINE_SET_RUN_BACK_SPEED) -> Self {
        Self::SMSG_SPLINE_SET_RUN_BACK_SPEED(c)
    }
}

impl From<SMSG_SPLINE_SET_SWIM_SPEED> for ServerOpcodeMessage {
    fn from(c: SMSG_SPLINE_SET_SWIM_SPEED) -> Self {
        Self::SMSG_SPLINE_SET_SWIM_SPEED(c)
    }
}

impl From<SMSG_SPLINE_SET_WALK_SPEED> for ServerOpcodeMessage {
    fn from(c: SMSG_SPLINE_SET_WALK_SPEED) -> Self {
        Self::SMSG_SPLINE_SET_WALK_SPEED(c)
    }
}

impl From<SMSG_SPLINE_SET_SWIM_BACK_SPEED> for ServerOpcodeMessage {
    fn from(c: SMSG_SPLINE_SET_SWIM_BACK_SPEED) -> Self {
        Self::SMSG_SPLINE_SET_SWIM_BACK_SPEED(c)
    }
}

impl From<SMSG_SPLINE_SET_TURN_RATE> for ServerOpcodeMessage {
    fn from(c: SMSG_SPLINE_SET_TURN_RATE) -> Self {
        Self::SMSG_SPLINE_SET_TURN_RATE(c)
    }
}

impl From<SMSG_SPLINE_MOVE_UNROOT> for ServerOpcodeMessage {
    fn from(c: SMSG_SPLINE_MOVE_UNROOT) -> Self {
        Self::SMSG_SPLINE_MOVE_UNROOT(c)
    }
}

impl From<SMSG_SPLINE_MOVE_FEATHER_FALL> for ServerOpcodeMessage {
    fn from(c: SMSG_SPLINE_MOVE_FEATHER_FALL) -> Self {
        Self::SMSG_SPLINE_MOVE_FEATHER_FALL(c)
    }
}

impl From<SMSG_SPLINE_MOVE_NORMAL_FALL> for ServerOpcodeMessage {
    fn from(c: SMSG_SPLINE_MOVE_NORMAL_FALL) -> Self {
        Self::SMSG_SPLINE_MOVE_NORMAL_FALL(c)
    }
}

impl From<SMSG_SPLINE_MOVE_SET_HOVER> for ServerOpcodeMessage {
    fn from(c: SMSG_SPLINE_MOVE_SET_HOVER) -> Self {
        Self::SMSG_SPLINE_MOVE_SET_HOVER(c)
    }
}

impl From<SMSG_SPLINE_MOVE_UNSET_HOVER> for ServerOpcodeMessage {
    fn from(c: SMSG_SPLINE_MOVE_UNSET_HOVER) -> Self {
        Self::SMSG_SPLINE_MOVE_UNSET_HOVER(c)
    }
}

impl From<SMSG_SPLINE_MOVE_WATER_WALK> for ServerOpcodeMessage {
    fn from(c: SMSG_SPLINE_MOVE_WATER_WALK) -> Self {
        Self::SMSG_SPLINE_MOVE_WATER_WALK(c)
    }
}

impl From<SMSG_SPLINE_MOVE_LAND_WALK> for ServerOpcodeMessage {
    fn from(c: SMSG_SPLINE_MOVE_LAND_WALK) -> Self {
        Self::SMSG_SPLINE_MOVE_LAND_WALK(c)
    }
}

impl From<SMSG_SPLINE_MOVE_START_SWIM> for ServerOpcodeMessage {
    fn from(c: SMSG_SPLINE_MOVE_START_SWIM) -> Self {
        Self::SMSG_SPLINE_MOVE_START_SWIM(c)
    }
}

impl From<SMSG_SPLINE_MOVE_STOP_SWIM> for ServerOpcodeMessage {
    fn from(c: SMSG_SPLINE_MOVE_STOP_SWIM) -> Self {
        Self::SMSG_SPLINE_MOVE_STOP_SWIM(c)
    }
}

impl From<SMSG_SPLINE_MOVE_SET_RUN_MODE> for ServerOpcodeMessage {
    fn from(c: SMSG_SPLINE_MOVE_SET_RUN_MODE) -> Self {
        Self::SMSG_SPLINE_MOVE_SET_RUN_MODE(c)
    }
}

impl From<SMSG_SPLINE_MOVE_SET_WALK_MODE> for ServerOpcodeMessage {
    fn from(c: SMSG_SPLINE_MOVE_SET_WALK_MODE) -> Self {
        Self::SMSG_SPLINE_MOVE_SET_WALK_MODE(c)
    }
}

impl From<SMSG_SPLINE_MOVE_ROOT> for ServerOpcodeMessage {
    fn from(c: SMSG_SPLINE_MOVE_ROOT) -> Self {
        Self::SMSG_SPLINE_MOVE_ROOT(c)
    }
}

impl From<MSG_RAID_READY_CHECK_Server> for ServerOpcodeMessage {
    fn from(c: MSG_RAID_READY_CHECK_Server) -> Self {
        Self::MSG_RAID_READY_CHECK(c)
    }
}

impl From<SMSG_PET_ACTION_SOUND> for ServerOpcodeMessage {
    fn from(c: SMSG_PET_ACTION_SOUND) -> Self {
        Self::SMSG_PET_ACTION_SOUND(c)
    }
}

impl From<SMSG_PET_DISMISS_SOUND> for ServerOpcodeMessage {
    fn from(c: SMSG_PET_DISMISS_SOUND) -> Self {
        Self::SMSG_PET_DISMISS_SOUND(c)
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

impl From<SMSG_GM_MESSAGECHAT> for ServerOpcodeMessage {
    fn from(c: SMSG_GM_MESSAGECHAT) -> Self {
        Self::SMSG_GM_MESSAGECHAT(c)
    }
}

impl From<SMSG_FEATURE_SYSTEM_STATUS> for ServerOpcodeMessage {
    fn from(c: SMSG_FEATURE_SYSTEM_STATUS) -> Self {
        Self::SMSG_FEATURE_SYSTEM_STATUS(c)
    }
}

impl From<SMSG_CALENDAR_SEND_NUM_PENDING> for ServerOpcodeMessage {
    fn from(c: SMSG_CALENDAR_SEND_NUM_PENDING) -> Self {
        Self::SMSG_CALENDAR_SEND_NUM_PENDING(c)
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

