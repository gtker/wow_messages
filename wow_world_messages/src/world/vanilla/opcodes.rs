use crate::vanilla::{ServerMessage, ClientMessage};
#[cfg(feature = "encryption")]
use wow_srp::vanilla_header::{DecrypterHalf, EncrypterHalf};

#[cfg(feature = "tokio")]
use tokio::io::{AsyncReadExt, AsyncWriteExt};
#[cfg(feature = "async-std")]
use async_std::io::{ReadExt, WriteExt};
use crate::vanilla::MovementInfo;
use crate::errors::ParseError;
use crate::vanilla::opcode_to_name;
use crate::vanilla::MSG_MOVE_WORLDPORT_ACK;
use crate::vanilla::MSG_PETITION_DECLINE;
use crate::vanilla::MSG_TABARDVENDOR_ACTIVATE;
use crate::vanilla::MSG_QUEST_PUSH_RESULT;
use crate::vanilla::MSG_MOVE_WATER_WALK;
use crate::vanilla::MSG_PETITION_RENAME;
use crate::vanilla::CMSG_BOOTME;
use crate::vanilla::CMSG_DBLOOKUP;
use crate::vanilla::CMSG_WORLD_TELEPORT;
use crate::vanilla::CMSG_TELEPORT_TO_UNIT;
use crate::vanilla::CMSG_CHAR_CREATE;
use crate::vanilla::CMSG_CHAR_ENUM;
use crate::vanilla::CMSG_CHAR_DELETE;
use crate::vanilla::CMSG_PLAYER_LOGIN;
use crate::vanilla::CMSG_PLAYER_LOGOUT;
use crate::vanilla::CMSG_LOGOUT_REQUEST;
use crate::vanilla::CMSG_LOGOUT_CANCEL;
use crate::vanilla::CMSG_NAME_QUERY;
use crate::vanilla::CMSG_PET_NAME_QUERY;
use crate::vanilla::CMSG_GUILD_QUERY;
use crate::vanilla::CMSG_ITEM_QUERY_SINGLE;
use crate::vanilla::CMSG_PAGE_TEXT_QUERY;
use crate::vanilla::CMSG_QUEST_QUERY;
use crate::vanilla::CMSG_GAMEOBJECT_QUERY;
use crate::vanilla::CMSG_CREATURE_QUERY;
use crate::vanilla::CMSG_WHO;
use crate::vanilla::CMSG_WHOIS;
use crate::vanilla::CMSG_FRIEND_LIST;
use crate::vanilla::CMSG_ADD_FRIEND;
use crate::vanilla::CMSG_DEL_FRIEND;
use crate::vanilla::CMSG_ADD_IGNORE;
use crate::vanilla::CMSG_DEL_IGNORE;
use crate::vanilla::CMSG_GROUP_INVITE;
use crate::vanilla::CMSG_GROUP_ACCEPT;
use crate::vanilla::CMSG_GROUP_DECLINE;
use crate::vanilla::CMSG_GROUP_UNINVITE;
use crate::vanilla::CMSG_GROUP_UNINVITE_GUID;
use crate::vanilla::CMSG_GROUP_SET_LEADER;
use crate::vanilla::CMSG_LOOT_METHOD;
use crate::vanilla::CMSG_GROUP_DISBAND;
use crate::vanilla::CMSG_GUILD_CREATE;
use crate::vanilla::CMSG_GUILD_INVITE;
use crate::vanilla::CMSG_GUILD_ACCEPT;
use crate::vanilla::CMSG_GUILD_DECLINE;
use crate::vanilla::CMSG_GUILD_INFO;
use crate::vanilla::CMSG_GUILD_ROSTER;
use crate::vanilla::CMSG_GUILD_PROMOTE;
use crate::vanilla::CMSG_GUILD_DEMOTE;
use crate::vanilla::CMSG_GUILD_LEAVE;
use crate::vanilla::CMSG_GUILD_REMOVE;
use crate::vanilla::CMSG_GUILD_DISBAND;
use crate::vanilla::CMSG_GUILD_LEADER;
use crate::vanilla::CMSG_GUILD_MOTD;
use crate::vanilla::CMSG_MESSAGECHAT;
use crate::vanilla::CMSG_JOIN_CHANNEL;
use crate::vanilla::CMSG_LEAVE_CHANNEL;
use crate::vanilla::CMSG_CHANNEL_LIST;
use crate::vanilla::CMSG_CHANNEL_PASSWORD;
use crate::vanilla::CMSG_CHANNEL_SET_OWNER;
use crate::vanilla::CMSG_CHANNEL_OWNER;
use crate::vanilla::CMSG_CHANNEL_MODERATOR;
use crate::vanilla::CMSG_CHANNEL_UNMODERATOR;
use crate::vanilla::CMSG_CHANNEL_MUTE;
use crate::vanilla::CMSG_CHANNEL_UNMUTE;
use crate::vanilla::CMSG_CHANNEL_INVITE;
use crate::vanilla::CMSG_CHANNEL_KICK;
use crate::vanilla::CMSG_CHANNEL_BAN;
use crate::vanilla::CMSG_CHANNEL_UNBAN;
use crate::vanilla::CMSG_CHANNEL_ANNOUNCEMENTS;
use crate::vanilla::CMSG_CHANNEL_MODERATE;
use crate::vanilla::CMSG_USE_ITEM;
use crate::vanilla::CMSG_OPEN_ITEM;
use crate::vanilla::CMSG_READ_ITEM;
use crate::vanilla::CMSG_GAMEOBJ_USE;
use crate::vanilla::CMSG_AREATRIGGER;
use crate::vanilla::MSG_MOVE_START_FORWARD_Client;
use crate::vanilla::MSG_MOVE_START_BACKWARD_Client;
use crate::vanilla::MSG_MOVE_STOP_Client;
use crate::vanilla::MSG_MOVE_START_STRAFE_LEFT_Client;
use crate::vanilla::MSG_MOVE_START_STRAFE_RIGHT_Client;
use crate::vanilla::MSG_MOVE_STOP_STRAFE_Client;
use crate::vanilla::MSG_MOVE_JUMP_Client;
use crate::vanilla::MSG_MOVE_START_TURN_LEFT_Client;
use crate::vanilla::MSG_MOVE_START_TURN_RIGHT_Client;
use crate::vanilla::MSG_MOVE_STOP_TURN_Client;
use crate::vanilla::MSG_MOVE_START_PITCH_UP_Client;
use crate::vanilla::MSG_MOVE_START_PITCH_DOWN_Client;
use crate::vanilla::MSG_MOVE_STOP_PITCH_Client;
use crate::vanilla::MSG_MOVE_SET_RUN_MODE_Client;
use crate::vanilla::MSG_MOVE_SET_WALK_MODE_Client;
use crate::vanilla::MSG_MOVE_TELEPORT_ACK_Client;
use crate::vanilla::MSG_MOVE_FALL_LAND_Client;
use crate::vanilla::MSG_MOVE_START_SWIM_Client;
use crate::vanilla::MSG_MOVE_STOP_SWIM_Client;
use crate::vanilla::MSG_MOVE_SET_FACING_Client;
use crate::vanilla::MSG_MOVE_SET_PITCH_Client;
use crate::vanilla::CMSG_MOVE_SET_RAW_POSITION;
use crate::vanilla::CMSG_FORCE_RUN_SPEED_CHANGE_ACK;
use crate::vanilla::CMSG_FORCE_RUN_BACK_SPEED_CHANGE_ACK;
use crate::vanilla::CMSG_FORCE_SWIM_SPEED_CHANGE_ACK;
use crate::vanilla::CMSG_FORCE_MOVE_ROOT_ACK;
use crate::vanilla::CMSG_FORCE_MOVE_UNROOT_ACK;
use crate::vanilla::MSG_MOVE_HEARTBEAT_Client;
use crate::vanilla::CMSG_MOVE_KNOCK_BACK_ACK;
use crate::vanilla::CMSG_MOVE_HOVER_ACK;
use crate::vanilla::CMSG_NEXT_CINEMATIC_CAMERA;
use crate::vanilla::CMSG_COMPLETE_CINEMATIC;
use crate::vanilla::CMSG_TUTORIAL_FLAG;
use crate::vanilla::CMSG_TUTORIAL_CLEAR;
use crate::vanilla::CMSG_TUTORIAL_RESET;
use crate::vanilla::CMSG_STANDSTATECHANGE;
use crate::vanilla::CMSG_EMOTE;
use crate::vanilla::CMSG_TEXT_EMOTE;
use crate::vanilla::CMSG_AUTOSTORE_LOOT_ITEM;
use crate::vanilla::CMSG_AUTOEQUIP_ITEM;
use crate::vanilla::CMSG_AUTOSTORE_BAG_ITEM;
use crate::vanilla::CMSG_SWAP_ITEM;
use crate::vanilla::CMSG_SWAP_INV_ITEM;
use crate::vanilla::CMSG_SPLIT_ITEM;
use crate::vanilla::CMSG_AUTOEQUIP_ITEM_SLOT;
use crate::vanilla::CMSG_DESTROYITEM;
use crate::vanilla::CMSG_INSPECT;
use crate::vanilla::CMSG_INITIATE_TRADE;
use crate::vanilla::CMSG_BEGIN_TRADE;
use crate::vanilla::CMSG_BUSY_TRADE;
use crate::vanilla::CMSG_IGNORE_TRADE;
use crate::vanilla::CMSG_ACCEPT_TRADE;
use crate::vanilla::CMSG_UNACCEPT_TRADE;
use crate::vanilla::CMSG_CANCEL_TRADE;
use crate::vanilla::CMSG_SET_TRADE_ITEM;
use crate::vanilla::CMSG_CLEAR_TRADE_ITEM;
use crate::vanilla::CMSG_SET_TRADE_GOLD;
use crate::vanilla::CMSG_SET_FACTION_ATWAR;
use crate::vanilla::CMSG_SET_ACTION_BUTTON;
use crate::vanilla::CMSG_CAST_SPELL;
use crate::vanilla::CMSG_CANCEL_CAST;
use crate::vanilla::CMSG_CANCEL_AURA;
use crate::vanilla::CMSG_CANCEL_CHANNELLING;
use crate::vanilla::CMSG_SET_SELECTION;
use crate::vanilla::CMSG_SET_TARGET_OBSOLETE;
use crate::vanilla::CMSG_ATTACKSWING;
use crate::vanilla::CMSG_ATTACKSTOP;
use crate::vanilla::CMSG_REPOP_REQUEST;
use crate::vanilla::CMSG_RESURRECT_RESPONSE;
use crate::vanilla::CMSG_LOOT;
use crate::vanilla::CMSG_LOOT_MONEY;
use crate::vanilla::CMSG_LOOT_RELEASE;
use crate::vanilla::CMSG_DUEL_ACCEPTED;
use crate::vanilla::CMSG_DUEL_CANCELLED;
use crate::vanilla::CMSG_MOUNTSPECIAL_ANIM;
use crate::vanilla::CMSG_PET_SET_ACTION;
use crate::vanilla::CMSG_PET_ACTION;
use crate::vanilla::CMSG_PET_ABANDON;
use crate::vanilla::CMSG_PET_RENAME;
use crate::vanilla::CMSG_GOSSIP_HELLO;
use crate::vanilla::CMSG_GOSSIP_SELECT_OPTION;
use crate::vanilla::CMSG_NPC_TEXT_QUERY;
use crate::vanilla::CMSG_QUESTGIVER_STATUS_QUERY;
use crate::vanilla::CMSG_QUESTGIVER_HELLO;
use crate::vanilla::CMSG_QUESTGIVER_QUERY_QUEST;
use crate::vanilla::CMSG_QUESTGIVER_QUEST_AUTOLAUNCH;
use crate::vanilla::CMSG_QUESTGIVER_ACCEPT_QUEST;
use crate::vanilla::CMSG_QUESTGIVER_COMPLETE_QUEST;
use crate::vanilla::CMSG_QUESTGIVER_REQUEST_REWARD;
use crate::vanilla::CMSG_QUESTGIVER_CHOOSE_REWARD;
use crate::vanilla::CMSG_QUESTGIVER_CANCEL;
use crate::vanilla::CMSG_QUESTLOG_SWAP_QUEST;
use crate::vanilla::CMSG_QUESTLOG_REMOVE_QUEST;
use crate::vanilla::CMSG_QUEST_CONFIRM_ACCEPT;
use crate::vanilla::CMSG_PUSHQUESTTOPARTY;
use crate::vanilla::CMSG_LIST_INVENTORY;
use crate::vanilla::CMSG_SELL_ITEM;
use crate::vanilla::CMSG_BUY_ITEM;
use crate::vanilla::CMSG_BUY_ITEM_IN_SLOT;
use crate::vanilla::CMSG_TAXINODE_STATUS_QUERY;
use crate::vanilla::CMSG_TAXIQUERYAVAILABLENODES;
use crate::vanilla::CMSG_ACTIVATETAXI;
use crate::vanilla::CMSG_TRAINER_LIST;
use crate::vanilla::CMSG_TRAINER_BUY_SPELL;
use crate::vanilla::CMSG_BINDER_ACTIVATE;
use crate::vanilla::CMSG_BANKER_ACTIVATE;
use crate::vanilla::CMSG_BUY_BANK_SLOT;
use crate::vanilla::CMSG_PETITION_SHOWLIST;
use crate::vanilla::CMSG_PETITION_BUY;
use crate::vanilla::CMSG_PETITION_SHOW_SIGNATURES;
use crate::vanilla::CMSG_PETITION_SIGN;
use crate::vanilla::CMSG_OFFER_PETITION;
use crate::vanilla::CMSG_TURN_IN_PETITION;
use crate::vanilla::CMSG_PETITION_QUERY;
use crate::vanilla::CMSG_BUG;
use crate::vanilla::CMSG_PLAYED_TIME;
use crate::vanilla::CMSG_QUERY_TIME;
use crate::vanilla::CMSG_RECLAIM_CORPSE;
use crate::vanilla::CMSG_WRAP_ITEM;
use crate::vanilla::MSG_MINIMAP_PING_Client;
use crate::vanilla::CMSG_PING;
use crate::vanilla::CMSG_SETSHEATHED;
use crate::vanilla::CMSG_AUTH_SESSION;
use crate::vanilla::CMSG_PET_CAST_SPELL;
use crate::vanilla::MSG_SAVE_GUILD_EMBLEM_Client;
use crate::vanilla::CMSG_ZONEUPDATE;
use crate::vanilla::MSG_RANDOM_ROLL_Client;
use crate::vanilla::MSG_LOOKING_FOR_GROUP_Client;
use crate::vanilla::CMSG_UNLEARN_SKILL;
use crate::vanilla::CMSG_GMTICKET_CREATE;
use crate::vanilla::CMSG_GMTICKET_UPDATETEXT;
use crate::vanilla::CMSG_REQUEST_ACCOUNT_DATA;
use crate::vanilla::CMSG_UPDATE_ACCOUNT_DATA;
use crate::vanilla::CMSG_GMTICKET_GETTICKET;
use crate::vanilla::MSG_CORPSE_QUERY_Client;
use crate::vanilla::CMSG_GMTICKET_DELETETICKET;
use crate::vanilla::CMSG_GMTICKET_SYSTEMSTATUS;
use crate::vanilla::CMSG_SPIRIT_HEALER_ACTIVATE;
use crate::vanilla::CMSG_CHAT_IGNORED;
use crate::vanilla::CMSG_GUILD_RANK;
use crate::vanilla::CMSG_GUILD_ADD_RANK;
use crate::vanilla::CMSG_GUILD_DEL_RANK;
use crate::vanilla::CMSG_GUILD_SET_PUBLIC_NOTE;
use crate::vanilla::CMSG_GUILD_SET_OFFICER_NOTE;
use crate::vanilla::CMSG_SEND_MAIL;
use crate::vanilla::CMSG_GET_MAIL_LIST;
use crate::vanilla::CMSG_BATTLEFIELD_LIST;
use crate::vanilla::CMSG_BATTLEFIELD_JOIN;
use crate::vanilla::CMSG_ITEM_TEXT_QUERY;
use crate::vanilla::CMSG_MAIL_TAKE_MONEY;
use crate::vanilla::CMSG_MAIL_TAKE_ITEM;
use crate::vanilla::CMSG_MAIL_MARK_AS_READ;
use crate::vanilla::CMSG_MAIL_RETURN_TO_SENDER;
use crate::vanilla::CMSG_MAIL_DELETE;
use crate::vanilla::CMSG_MAIL_CREATE_TEXT_ITEM;
use crate::vanilla::CMSG_LEARN_TALENT;
use crate::vanilla::CMSG_TOGGLE_PVP;
use crate::vanilla::MSG_AUCTION_HELLO_Client;
use crate::vanilla::CMSG_AUCTION_SELL_ITEM;
use crate::vanilla::CMSG_AUCTION_REMOVE_ITEM;
use crate::vanilla::CMSG_AUCTION_LIST_ITEMS;
use crate::vanilla::CMSG_AUCTION_LIST_OWNER_ITEMS;
use crate::vanilla::CMSG_AUCTION_PLACE_BID;
use crate::vanilla::CMSG_AUCTION_LIST_BIDDER_ITEMS;
use crate::vanilla::CMSG_SET_AMMO;
use crate::vanilla::CMSG_SET_ACTIVE_MOVER;
use crate::vanilla::CMSG_PET_CANCEL_AURA;
use crate::vanilla::CMSG_CANCEL_AUTO_REPEAT_SPELL;
use crate::vanilla::MSG_LIST_STABLED_PETS_Client;
use crate::vanilla::CMSG_STABLE_PET;
use crate::vanilla::CMSG_UNSTABLE_PET;
use crate::vanilla::CMSG_BUY_STABLE_SLOT;
use crate::vanilla::CMSG_STABLE_SWAP_PET;
use crate::vanilla::CMSG_REQUEST_PET_INFO;
use crate::vanilla::CMSG_FAR_SIGHT;
use crate::vanilla::CMSG_GROUP_CHANGE_SUB_GROUP;
use crate::vanilla::CMSG_REQUEST_PARTY_MEMBER_STATS;
use crate::vanilla::CMSG_GROUP_SWAP_SUB_GROUP;
use crate::vanilla::CMSG_AUTOSTORE_BANK_ITEM;
use crate::vanilla::CMSG_AUTOBANK_ITEM;
use crate::vanilla::MSG_QUERY_NEXT_MAIL_TIME_Client;
use crate::vanilla::CMSG_GROUP_RAID_CONVERT;
use crate::vanilla::CMSG_GROUP_ASSISTANT_LEADER;
use crate::vanilla::CMSG_BUYBACK_ITEM;
use crate::vanilla::CMSG_MEETINGSTONE_JOIN;
use crate::vanilla::CMSG_MEETINGSTONE_LEAVE;
use crate::vanilla::CMSG_MEETINGSTONE_INFO;
use crate::vanilla::CMSG_CANCEL_GROWTH_AURA;
use crate::vanilla::CMSG_LOOT_ROLL;
use crate::vanilla::CMSG_LOOT_MASTER_GIVE;
use crate::vanilla::CMSG_REPAIR_ITEM;
use crate::vanilla::MSG_TALENT_WIPE_CONFIRM_Client;
use crate::vanilla::CMSG_SUMMON_RESPONSE;
use crate::vanilla::CMSG_SELF_RES;
use crate::vanilla::CMSG_TOGGLE_HELM;
use crate::vanilla::CMSG_TOGGLE_CLOAK;
use crate::vanilla::CMSG_SET_ACTIONBAR_TOGGLES;
use crate::vanilla::CMSG_ITEM_NAME_QUERY;
use crate::vanilla::CMSG_CHAR_RENAME;
use crate::vanilla::CMSG_MOVE_SPLINE_DONE;
use crate::vanilla::CMSG_MOVE_FALL_RESET;
use crate::vanilla::CMSG_REQUEST_RAID_INFO;
use crate::vanilla::CMSG_MOVE_TIME_SKIPPED;
use crate::vanilla::CMSG_MOVE_FEATHER_FALL_ACK;
use crate::vanilla::CMSG_MOVE_WATER_WALK_ACK;
use crate::vanilla::CMSG_MOVE_NOT_ACTIVE_MOVER;
use crate::vanilla::CMSG_BATTLEFIELD_STATUS;
use crate::vanilla::CMSG_BATTLEFIELD_PORT;
use crate::vanilla::MSG_INSPECT_HONOR_STATS_Client;
use crate::vanilla::CMSG_BATTLEMASTER_HELLO;
use crate::vanilla::CMSG_FORCE_WALK_SPEED_CHANGE_ACK;
use crate::vanilla::CMSG_FORCE_SWIM_BACK_SPEED_CHANGE_ACK;
use crate::vanilla::CMSG_FORCE_TURN_RATE_CHANGE_ACK;
use crate::vanilla::MSG_PVP_LOG_DATA_Client;
use crate::vanilla::CMSG_LEAVE_BATTLEFIELD;
use crate::vanilla::CMSG_AREA_SPIRIT_HEALER_QUERY;
use crate::vanilla::CMSG_AREA_SPIRIT_HEALER_QUEUE;
use crate::vanilla::CMSG_WARDEN_DATA;
use crate::vanilla::MSG_BATTLEGROUND_PLAYER_POSITIONS_Client;
use crate::vanilla::CMSG_PET_STOP_ATTACK;
use crate::vanilla::CMSG_BATTLEMASTER_JOIN;
use crate::vanilla::CMSG_PET_UNLEARN;
use crate::vanilla::CMSG_PET_SPELL_AUTOCAST;
use crate::vanilla::CMSG_GUILD_INFO_TEXT;
use crate::vanilla::CMSG_ACTIVATETAXIEXPRESS;
use crate::vanilla::CMSG_SET_FACTION_INACTIVE;
use crate::vanilla::CMSG_SET_WATCHED_FACTION;
use crate::vanilla::CMSG_RESET_INSTANCES;
use crate::vanilla::MSG_RAID_TARGET_UPDATE_Client;
use crate::vanilla::MSG_RAID_READY_CHECK_Client;
use crate::vanilla::CMSG_GMSURVEY_SUBMIT;

#[derive(Debug, Clone, PartialEq)]
pub enum ClientOpcodeMessage {
    MSG_MOVE_WORLDPORT_ACK,
    MSG_PETITION_DECLINE(MSG_PETITION_DECLINE),
    MSG_TABARDVENDOR_ACTIVATE(MSG_TABARDVENDOR_ACTIVATE),
    MSG_QUEST_PUSH_RESULT(MSG_QUEST_PUSH_RESULT),
    MSG_MOVE_WATER_WALK(MSG_MOVE_WATER_WALK),
    MSG_PETITION_RENAME(MSG_PETITION_RENAME),
    CMSG_BOOTME,
    CMSG_DBLOOKUP(CMSG_DBLOOKUP),
    CMSG_WORLD_TELEPORT(CMSG_WORLD_TELEPORT),
    CMSG_TELEPORT_TO_UNIT(CMSG_TELEPORT_TO_UNIT),
    CMSG_CHAR_CREATE(CMSG_CHAR_CREATE),
    CMSG_CHAR_ENUM,
    CMSG_CHAR_DELETE(CMSG_CHAR_DELETE),
    CMSG_PLAYER_LOGIN(CMSG_PLAYER_LOGIN),
    CMSG_PLAYER_LOGOUT,
    CMSG_LOGOUT_REQUEST,
    CMSG_LOGOUT_CANCEL,
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
    CMSG_FRIEND_LIST,
    CMSG_ADD_FRIEND(CMSG_ADD_FRIEND),
    CMSG_DEL_FRIEND(CMSG_DEL_FRIEND),
    CMSG_ADD_IGNORE(CMSG_ADD_IGNORE),
    CMSG_DEL_IGNORE(CMSG_DEL_IGNORE),
    CMSG_GROUP_INVITE(CMSG_GROUP_INVITE),
    CMSG_GROUP_ACCEPT,
    CMSG_GROUP_DECLINE,
    CMSG_GROUP_UNINVITE(CMSG_GROUP_UNINVITE),
    CMSG_GROUP_UNINVITE_GUID(CMSG_GROUP_UNINVITE_GUID),
    CMSG_GROUP_SET_LEADER(CMSG_GROUP_SET_LEADER),
    CMSG_LOOT_METHOD(CMSG_LOOT_METHOD),
    CMSG_GROUP_DISBAND,
    CMSG_GUILD_CREATE(CMSG_GUILD_CREATE),
    CMSG_GUILD_INVITE(CMSG_GUILD_INVITE),
    CMSG_GUILD_ACCEPT,
    CMSG_GUILD_DECLINE,
    CMSG_GUILD_INFO,
    CMSG_GUILD_ROSTER,
    CMSG_GUILD_PROMOTE(CMSG_GUILD_PROMOTE),
    CMSG_GUILD_DEMOTE(CMSG_GUILD_DEMOTE),
    CMSG_GUILD_LEAVE,
    CMSG_GUILD_REMOVE(CMSG_GUILD_REMOVE),
    CMSG_GUILD_DISBAND,
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
    MSG_MOVE_START_FORWARD(MSG_MOVE_START_FORWARD_Client),
    MSG_MOVE_START_BACKWARD(MSG_MOVE_START_BACKWARD_Client),
    MSG_MOVE_STOP(MSG_MOVE_STOP_Client),
    MSG_MOVE_START_STRAFE_LEFT(MSG_MOVE_START_STRAFE_LEFT_Client),
    MSG_MOVE_START_STRAFE_RIGHT(MSG_MOVE_START_STRAFE_RIGHT_Client),
    MSG_MOVE_STOP_STRAFE(MSG_MOVE_STOP_STRAFE_Client),
    MSG_MOVE_JUMP(MSG_MOVE_JUMP_Client),
    MSG_MOVE_START_TURN_LEFT(MSG_MOVE_START_TURN_LEFT_Client),
    MSG_MOVE_START_TURN_RIGHT(MSG_MOVE_START_TURN_RIGHT_Client),
    MSG_MOVE_STOP_TURN(MSG_MOVE_STOP_TURN_Client),
    MSG_MOVE_START_PITCH_UP(MSG_MOVE_START_PITCH_UP_Client),
    MSG_MOVE_START_PITCH_DOWN(MSG_MOVE_START_PITCH_DOWN_Client),
    MSG_MOVE_STOP_PITCH(MSG_MOVE_STOP_PITCH_Client),
    MSG_MOVE_SET_RUN_MODE(MSG_MOVE_SET_RUN_MODE_Client),
    MSG_MOVE_SET_WALK_MODE(MSG_MOVE_SET_WALK_MODE_Client),
    MSG_MOVE_TELEPORT_ACK(MSG_MOVE_TELEPORT_ACK_Client),
    MSG_MOVE_FALL_LAND(MSG_MOVE_FALL_LAND_Client),
    MSG_MOVE_START_SWIM(MSG_MOVE_START_SWIM_Client),
    MSG_MOVE_STOP_SWIM(MSG_MOVE_STOP_SWIM_Client),
    MSG_MOVE_SET_FACING(MSG_MOVE_SET_FACING_Client),
    MSG_MOVE_SET_PITCH(MSG_MOVE_SET_PITCH_Client),
    CMSG_MOVE_SET_RAW_POSITION(CMSG_MOVE_SET_RAW_POSITION),
    CMSG_FORCE_RUN_SPEED_CHANGE_ACK(CMSG_FORCE_RUN_SPEED_CHANGE_ACK),
    CMSG_FORCE_RUN_BACK_SPEED_CHANGE_ACK(CMSG_FORCE_RUN_BACK_SPEED_CHANGE_ACK),
    CMSG_FORCE_SWIM_SPEED_CHANGE_ACK(CMSG_FORCE_SWIM_SPEED_CHANGE_ACK),
    CMSG_FORCE_MOVE_ROOT_ACK(CMSG_FORCE_MOVE_ROOT_ACK),
    CMSG_FORCE_MOVE_UNROOT_ACK(CMSG_FORCE_MOVE_UNROOT_ACK),
    MSG_MOVE_HEARTBEAT(MSG_MOVE_HEARTBEAT_Client),
    CMSG_MOVE_KNOCK_BACK_ACK(CMSG_MOVE_KNOCK_BACK_ACK),
    CMSG_MOVE_HOVER_ACK(CMSG_MOVE_HOVER_ACK),
    CMSG_NEXT_CINEMATIC_CAMERA,
    CMSG_COMPLETE_CINEMATIC,
    CMSG_TUTORIAL_FLAG(CMSG_TUTORIAL_FLAG),
    CMSG_TUTORIAL_CLEAR,
    CMSG_TUTORIAL_RESET,
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
    CMSG_BEGIN_TRADE,
    CMSG_BUSY_TRADE,
    CMSG_IGNORE_TRADE,
    CMSG_ACCEPT_TRADE(CMSG_ACCEPT_TRADE),
    CMSG_UNACCEPT_TRADE,
    CMSG_CANCEL_TRADE,
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
    CMSG_SET_TARGET_OBSOLETE(CMSG_SET_TARGET_OBSOLETE),
    CMSG_ATTACKSWING(CMSG_ATTACKSWING),
    CMSG_ATTACKSTOP,
    CMSG_REPOP_REQUEST,
    CMSG_RESURRECT_RESPONSE(CMSG_RESURRECT_RESPONSE),
    CMSG_LOOT(CMSG_LOOT),
    CMSG_LOOT_MONEY,
    CMSG_LOOT_RELEASE(CMSG_LOOT_RELEASE),
    CMSG_DUEL_ACCEPTED(CMSG_DUEL_ACCEPTED),
    CMSG_DUEL_CANCELLED(CMSG_DUEL_CANCELLED),
    CMSG_MOUNTSPECIAL_ANIM,
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
    CMSG_QUESTGIVER_QUEST_AUTOLAUNCH,
    CMSG_QUESTGIVER_ACCEPT_QUEST(CMSG_QUESTGIVER_ACCEPT_QUEST),
    CMSG_QUESTGIVER_COMPLETE_QUEST(CMSG_QUESTGIVER_COMPLETE_QUEST),
    CMSG_QUESTGIVER_REQUEST_REWARD(CMSG_QUESTGIVER_REQUEST_REWARD),
    CMSG_QUESTGIVER_CHOOSE_REWARD(CMSG_QUESTGIVER_CHOOSE_REWARD),
    CMSG_QUESTGIVER_CANCEL,
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
    CMSG_PLAYED_TIME,
    CMSG_QUERY_TIME,
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
    MSG_LOOKING_FOR_GROUP,
    CMSG_UNLEARN_SKILL(CMSG_UNLEARN_SKILL),
    CMSG_GMTICKET_CREATE(CMSG_GMTICKET_CREATE),
    CMSG_GMTICKET_UPDATETEXT(CMSG_GMTICKET_UPDATETEXT),
    CMSG_REQUEST_ACCOUNT_DATA(CMSG_REQUEST_ACCOUNT_DATA),
    CMSG_UPDATE_ACCOUNT_DATA(CMSG_UPDATE_ACCOUNT_DATA),
    CMSG_GMTICKET_GETTICKET,
    MSG_CORPSE_QUERY,
    CMSG_GMTICKET_DELETETICKET,
    CMSG_GMTICKET_SYSTEMSTATUS,
    CMSG_SPIRIT_HEALER_ACTIVATE(CMSG_SPIRIT_HEALER_ACTIVATE),
    CMSG_CHAT_IGNORED(CMSG_CHAT_IGNORED),
    CMSG_GUILD_RANK(CMSG_GUILD_RANK),
    CMSG_GUILD_ADD_RANK(CMSG_GUILD_ADD_RANK),
    CMSG_GUILD_DEL_RANK,
    CMSG_GUILD_SET_PUBLIC_NOTE(CMSG_GUILD_SET_PUBLIC_NOTE),
    CMSG_GUILD_SET_OFFICER_NOTE(CMSG_GUILD_SET_OFFICER_NOTE),
    CMSG_SEND_MAIL(CMSG_SEND_MAIL),
    CMSG_GET_MAIL_LIST(CMSG_GET_MAIL_LIST),
    CMSG_BATTLEFIELD_LIST(CMSG_BATTLEFIELD_LIST),
    CMSG_BATTLEFIELD_JOIN(CMSG_BATTLEFIELD_JOIN),
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
    CMSG_CANCEL_AUTO_REPEAT_SPELL,
    MSG_LIST_STABLED_PETS(MSG_LIST_STABLED_PETS_Client),
    CMSG_STABLE_PET(CMSG_STABLE_PET),
    CMSG_UNSTABLE_PET(CMSG_UNSTABLE_PET),
    CMSG_BUY_STABLE_SLOT(CMSG_BUY_STABLE_SLOT),
    CMSG_STABLE_SWAP_PET(CMSG_STABLE_SWAP_PET),
    CMSG_REQUEST_PET_INFO,
    CMSG_FAR_SIGHT(CMSG_FAR_SIGHT),
    CMSG_GROUP_CHANGE_SUB_GROUP(CMSG_GROUP_CHANGE_SUB_GROUP),
    CMSG_REQUEST_PARTY_MEMBER_STATS(CMSG_REQUEST_PARTY_MEMBER_STATS),
    CMSG_GROUP_SWAP_SUB_GROUP(CMSG_GROUP_SWAP_SUB_GROUP),
    CMSG_AUTOSTORE_BANK_ITEM(CMSG_AUTOSTORE_BANK_ITEM),
    CMSG_AUTOBANK_ITEM(CMSG_AUTOBANK_ITEM),
    MSG_QUERY_NEXT_MAIL_TIME,
    CMSG_GROUP_RAID_CONVERT,
    CMSG_GROUP_ASSISTANT_LEADER(CMSG_GROUP_ASSISTANT_LEADER),
    CMSG_BUYBACK_ITEM(CMSG_BUYBACK_ITEM),
    CMSG_MEETINGSTONE_JOIN(CMSG_MEETINGSTONE_JOIN),
    CMSG_MEETINGSTONE_LEAVE,
    CMSG_MEETINGSTONE_INFO,
    CMSG_CANCEL_GROWTH_AURA,
    CMSG_LOOT_ROLL(CMSG_LOOT_ROLL),
    CMSG_LOOT_MASTER_GIVE(CMSG_LOOT_MASTER_GIVE),
    CMSG_REPAIR_ITEM(CMSG_REPAIR_ITEM),
    MSG_TALENT_WIPE_CONFIRM(MSG_TALENT_WIPE_CONFIRM_Client),
    CMSG_SUMMON_RESPONSE(CMSG_SUMMON_RESPONSE),
    CMSG_SELF_RES,
    CMSG_TOGGLE_HELM,
    CMSG_TOGGLE_CLOAK,
    CMSG_SET_ACTIONBAR_TOGGLES(CMSG_SET_ACTIONBAR_TOGGLES),
    CMSG_ITEM_NAME_QUERY(CMSG_ITEM_NAME_QUERY),
    CMSG_CHAR_RENAME(CMSG_CHAR_RENAME),
    CMSG_MOVE_SPLINE_DONE(CMSG_MOVE_SPLINE_DONE),
    CMSG_MOVE_FALL_RESET(CMSG_MOVE_FALL_RESET),
    CMSG_REQUEST_RAID_INFO,
    CMSG_MOVE_TIME_SKIPPED(CMSG_MOVE_TIME_SKIPPED),
    CMSG_MOVE_FEATHER_FALL_ACK(CMSG_MOVE_FEATHER_FALL_ACK),
    CMSG_MOVE_WATER_WALK_ACK(CMSG_MOVE_WATER_WALK_ACK),
    CMSG_MOVE_NOT_ACTIVE_MOVER(CMSG_MOVE_NOT_ACTIVE_MOVER),
    CMSG_BATTLEFIELD_STATUS,
    CMSG_BATTLEFIELD_PORT(CMSG_BATTLEFIELD_PORT),
    MSG_INSPECT_HONOR_STATS(MSG_INSPECT_HONOR_STATS_Client),
    CMSG_BATTLEMASTER_HELLO(CMSG_BATTLEMASTER_HELLO),
    CMSG_FORCE_WALK_SPEED_CHANGE_ACK(CMSG_FORCE_WALK_SPEED_CHANGE_ACK),
    CMSG_FORCE_SWIM_BACK_SPEED_CHANGE_ACK(CMSG_FORCE_SWIM_BACK_SPEED_CHANGE_ACK),
    CMSG_FORCE_TURN_RATE_CHANGE_ACK(CMSG_FORCE_TURN_RATE_CHANGE_ACK),
    MSG_PVP_LOG_DATA,
    CMSG_LEAVE_BATTLEFIELD(CMSG_LEAVE_BATTLEFIELD),
    CMSG_AREA_SPIRIT_HEALER_QUERY(CMSG_AREA_SPIRIT_HEALER_QUERY),
    CMSG_AREA_SPIRIT_HEALER_QUEUE(CMSG_AREA_SPIRIT_HEALER_QUEUE),
    CMSG_WARDEN_DATA(CMSG_WARDEN_DATA),
    MSG_BATTLEGROUND_PLAYER_POSITIONS,
    CMSG_PET_STOP_ATTACK(CMSG_PET_STOP_ATTACK),
    CMSG_BATTLEMASTER_JOIN(CMSG_BATTLEMASTER_JOIN),
    CMSG_PET_UNLEARN(CMSG_PET_UNLEARN),
    CMSG_PET_SPELL_AUTOCAST(CMSG_PET_SPELL_AUTOCAST),
    CMSG_GUILD_INFO_TEXT(CMSG_GUILD_INFO_TEXT),
    CMSG_ACTIVATETAXIEXPRESS(CMSG_ACTIVATETAXIEXPRESS),
    CMSG_SET_FACTION_INACTIVE(CMSG_SET_FACTION_INACTIVE),
    CMSG_SET_WATCHED_FACTION(CMSG_SET_WATCHED_FACTION),
    CMSG_RESET_INSTANCES,
    MSG_RAID_TARGET_UPDATE(MSG_RAID_TARGET_UPDATE_Client),
    MSG_RAID_READY_CHECK(MSG_RAID_READY_CHECK_Client),
    CMSG_GMSURVEY_SUBMIT(CMSG_GMSURVEY_SUBMIT),
}

impl ClientOpcodeMessage {
    fn read_opcodes(opcode: u32, body_size: u32, mut r: &[u8]) -> std::result::Result<Self, crate::errors::ExpectedOpcodeError> {
        match opcode {
            0x00DC => crate::util::assert_empty(body_size, opcode).map(|_| Self::MSG_MOVE_WORLDPORT_ACK),
            0x01C2 => Ok(Self::MSG_PETITION_DECLINE(<MSG_PETITION_DECLINE as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x01C2, size: body_size, io, } } else { a } })?)),
            0x01F2 => Ok(Self::MSG_TABARDVENDOR_ACTIVATE(<MSG_TABARDVENDOR_ACTIVATE as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x01F2, size: body_size, io, } } else { a } })?)),
            0x0276 => Ok(Self::MSG_QUEST_PUSH_RESULT(<MSG_QUEST_PUSH_RESULT as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x0276, size: body_size, io, } } else { a } })?)),
            0x02B1 => Ok(Self::MSG_MOVE_WATER_WALK(<MSG_MOVE_WATER_WALK as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x02B1, size: body_size, io, } } else { a } })?)),
            0x02C1 => Ok(Self::MSG_PETITION_RENAME(<MSG_PETITION_RENAME as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x02C1, size: body_size, io, } } else { a } })?)),
            0x0001 => crate::util::assert_empty(body_size, opcode).map(|_| Self::CMSG_BOOTME),
            0x0002 => Ok(Self::CMSG_DBLOOKUP(<CMSG_DBLOOKUP as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x0002, size: body_size, io, } } else { a } })?)),
            0x0008 => Ok(Self::CMSG_WORLD_TELEPORT(<CMSG_WORLD_TELEPORT as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x0008, size: body_size, io, } } else { a } })?)),
            0x0009 => Ok(Self::CMSG_TELEPORT_TO_UNIT(<CMSG_TELEPORT_TO_UNIT as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x0009, size: body_size, io, } } else { a } })?)),
            0x0036 => Ok(Self::CMSG_CHAR_CREATE(<CMSG_CHAR_CREATE as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x0036, size: body_size, io, } } else { a } })?)),
            0x0037 => crate::util::assert_empty(body_size, opcode).map(|_| Self::CMSG_CHAR_ENUM),
            0x0038 => Ok(Self::CMSG_CHAR_DELETE(<CMSG_CHAR_DELETE as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x0038, size: body_size, io, } } else { a } })?)),
            0x003D => Ok(Self::CMSG_PLAYER_LOGIN(<CMSG_PLAYER_LOGIN as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x003D, size: body_size, io, } } else { a } })?)),
            0x004A => crate::util::assert_empty(body_size, opcode).map(|_| Self::CMSG_PLAYER_LOGOUT),
            0x004B => crate::util::assert_empty(body_size, opcode).map(|_| Self::CMSG_LOGOUT_REQUEST),
            0x004E => crate::util::assert_empty(body_size, opcode).map(|_| Self::CMSG_LOGOUT_CANCEL),
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
            0x0066 => crate::util::assert_empty(body_size, opcode).map(|_| Self::CMSG_FRIEND_LIST),
            0x0069 => Ok(Self::CMSG_ADD_FRIEND(<CMSG_ADD_FRIEND as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x0069, size: body_size, io, } } else { a } })?)),
            0x006A => Ok(Self::CMSG_DEL_FRIEND(<CMSG_DEL_FRIEND as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x006A, size: body_size, io, } } else { a } })?)),
            0x006C => Ok(Self::CMSG_ADD_IGNORE(<CMSG_ADD_IGNORE as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x006C, size: body_size, io, } } else { a } })?)),
            0x006D => Ok(Self::CMSG_DEL_IGNORE(<CMSG_DEL_IGNORE as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x006D, size: body_size, io, } } else { a } })?)),
            0x006E => Ok(Self::CMSG_GROUP_INVITE(<CMSG_GROUP_INVITE as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x006E, size: body_size, io, } } else { a } })?)),
            0x0072 => crate::util::assert_empty(body_size, opcode).map(|_| Self::CMSG_GROUP_ACCEPT),
            0x0073 => crate::util::assert_empty(body_size, opcode).map(|_| Self::CMSG_GROUP_DECLINE),
            0x0075 => Ok(Self::CMSG_GROUP_UNINVITE(<CMSG_GROUP_UNINVITE as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x0075, size: body_size, io, } } else { a } })?)),
            0x0076 => Ok(Self::CMSG_GROUP_UNINVITE_GUID(<CMSG_GROUP_UNINVITE_GUID as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x0076, size: body_size, io, } } else { a } })?)),
            0x0078 => Ok(Self::CMSG_GROUP_SET_LEADER(<CMSG_GROUP_SET_LEADER as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x0078, size: body_size, io, } } else { a } })?)),
            0x007A => Ok(Self::CMSG_LOOT_METHOD(<CMSG_LOOT_METHOD as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x007A, size: body_size, io, } } else { a } })?)),
            0x007B => crate::util::assert_empty(body_size, opcode).map(|_| Self::CMSG_GROUP_DISBAND),
            0x0081 => Ok(Self::CMSG_GUILD_CREATE(<CMSG_GUILD_CREATE as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x0081, size: body_size, io, } } else { a } })?)),
            0x0082 => Ok(Self::CMSG_GUILD_INVITE(<CMSG_GUILD_INVITE as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x0082, size: body_size, io, } } else { a } })?)),
            0x0084 => crate::util::assert_empty(body_size, opcode).map(|_| Self::CMSG_GUILD_ACCEPT),
            0x0085 => crate::util::assert_empty(body_size, opcode).map(|_| Self::CMSG_GUILD_DECLINE),
            0x0087 => crate::util::assert_empty(body_size, opcode).map(|_| Self::CMSG_GUILD_INFO),
            0x0089 => crate::util::assert_empty(body_size, opcode).map(|_| Self::CMSG_GUILD_ROSTER),
            0x008B => Ok(Self::CMSG_GUILD_PROMOTE(<CMSG_GUILD_PROMOTE as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x008B, size: body_size, io, } } else { a } })?)),
            0x008C => Ok(Self::CMSG_GUILD_DEMOTE(<CMSG_GUILD_DEMOTE as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x008C, size: body_size, io, } } else { a } })?)),
            0x008D => crate::util::assert_empty(body_size, opcode).map(|_| Self::CMSG_GUILD_LEAVE),
            0x008E => Ok(Self::CMSG_GUILD_REMOVE(<CMSG_GUILD_REMOVE as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x008E, size: body_size, io, } } else { a } })?)),
            0x008F => crate::util::assert_empty(body_size, opcode).map(|_| Self::CMSG_GUILD_DISBAND),
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
            0x00B5 => Ok(Self::MSG_MOVE_START_FORWARD(<MSG_MOVE_START_FORWARD_Client as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x00B5, size: body_size, io, } } else { a } })?)),
            0x00B6 => Ok(Self::MSG_MOVE_START_BACKWARD(<MSG_MOVE_START_BACKWARD_Client as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x00B6, size: body_size, io, } } else { a } })?)),
            0x00B7 => Ok(Self::MSG_MOVE_STOP(<MSG_MOVE_STOP_Client as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x00B7, size: body_size, io, } } else { a } })?)),
            0x00B8 => Ok(Self::MSG_MOVE_START_STRAFE_LEFT(<MSG_MOVE_START_STRAFE_LEFT_Client as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x00B8, size: body_size, io, } } else { a } })?)),
            0x00B9 => Ok(Self::MSG_MOVE_START_STRAFE_RIGHT(<MSG_MOVE_START_STRAFE_RIGHT_Client as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x00B9, size: body_size, io, } } else { a } })?)),
            0x00BA => Ok(Self::MSG_MOVE_STOP_STRAFE(<MSG_MOVE_STOP_STRAFE_Client as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x00BA, size: body_size, io, } } else { a } })?)),
            0x00BB => Ok(Self::MSG_MOVE_JUMP(<MSG_MOVE_JUMP_Client as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x00BB, size: body_size, io, } } else { a } })?)),
            0x00BC => Ok(Self::MSG_MOVE_START_TURN_LEFT(<MSG_MOVE_START_TURN_LEFT_Client as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x00BC, size: body_size, io, } } else { a } })?)),
            0x00BD => Ok(Self::MSG_MOVE_START_TURN_RIGHT(<MSG_MOVE_START_TURN_RIGHT_Client as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x00BD, size: body_size, io, } } else { a } })?)),
            0x00BE => Ok(Self::MSG_MOVE_STOP_TURN(<MSG_MOVE_STOP_TURN_Client as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x00BE, size: body_size, io, } } else { a } })?)),
            0x00BF => Ok(Self::MSG_MOVE_START_PITCH_UP(<MSG_MOVE_START_PITCH_UP_Client as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x00BF, size: body_size, io, } } else { a } })?)),
            0x00C0 => Ok(Self::MSG_MOVE_START_PITCH_DOWN(<MSG_MOVE_START_PITCH_DOWN_Client as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x00C0, size: body_size, io, } } else { a } })?)),
            0x00C1 => Ok(Self::MSG_MOVE_STOP_PITCH(<MSG_MOVE_STOP_PITCH_Client as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x00C1, size: body_size, io, } } else { a } })?)),
            0x00C2 => Ok(Self::MSG_MOVE_SET_RUN_MODE(<MSG_MOVE_SET_RUN_MODE_Client as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x00C2, size: body_size, io, } } else { a } })?)),
            0x00C3 => Ok(Self::MSG_MOVE_SET_WALK_MODE(<MSG_MOVE_SET_WALK_MODE_Client as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x00C3, size: body_size, io, } } else { a } })?)),
            0x00C7 => Ok(Self::MSG_MOVE_TELEPORT_ACK(<MSG_MOVE_TELEPORT_ACK_Client as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x00C7, size: body_size, io, } } else { a } })?)),
            0x00C9 => Ok(Self::MSG_MOVE_FALL_LAND(<MSG_MOVE_FALL_LAND_Client as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x00C9, size: body_size, io, } } else { a } })?)),
            0x00CA => Ok(Self::MSG_MOVE_START_SWIM(<MSG_MOVE_START_SWIM_Client as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x00CA, size: body_size, io, } } else { a } })?)),
            0x00CB => Ok(Self::MSG_MOVE_STOP_SWIM(<MSG_MOVE_STOP_SWIM_Client as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x00CB, size: body_size, io, } } else { a } })?)),
            0x00DA => Ok(Self::MSG_MOVE_SET_FACING(<MSG_MOVE_SET_FACING_Client as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x00DA, size: body_size, io, } } else { a } })?)),
            0x00DB => Ok(Self::MSG_MOVE_SET_PITCH(<MSG_MOVE_SET_PITCH_Client as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x00DB, size: body_size, io, } } else { a } })?)),
            0x00E1 => Ok(Self::CMSG_MOVE_SET_RAW_POSITION(<CMSG_MOVE_SET_RAW_POSITION as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x00E1, size: body_size, io, } } else { a } })?)),
            0x00E3 => Ok(Self::CMSG_FORCE_RUN_SPEED_CHANGE_ACK(<CMSG_FORCE_RUN_SPEED_CHANGE_ACK as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x00E3, size: body_size, io, } } else { a } })?)),
            0x00E5 => Ok(Self::CMSG_FORCE_RUN_BACK_SPEED_CHANGE_ACK(<CMSG_FORCE_RUN_BACK_SPEED_CHANGE_ACK as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x00E5, size: body_size, io, } } else { a } })?)),
            0x00E7 => Ok(Self::CMSG_FORCE_SWIM_SPEED_CHANGE_ACK(<CMSG_FORCE_SWIM_SPEED_CHANGE_ACK as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x00E7, size: body_size, io, } } else { a } })?)),
            0x00E9 => Ok(Self::CMSG_FORCE_MOVE_ROOT_ACK(<CMSG_FORCE_MOVE_ROOT_ACK as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x00E9, size: body_size, io, } } else { a } })?)),
            0x00EB => Ok(Self::CMSG_FORCE_MOVE_UNROOT_ACK(<CMSG_FORCE_MOVE_UNROOT_ACK as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x00EB, size: body_size, io, } } else { a } })?)),
            0x00EE => Ok(Self::MSG_MOVE_HEARTBEAT(<MSG_MOVE_HEARTBEAT_Client as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x00EE, size: body_size, io, } } else { a } })?)),
            0x00F0 => Ok(Self::CMSG_MOVE_KNOCK_BACK_ACK(<CMSG_MOVE_KNOCK_BACK_ACK as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x00F0, size: body_size, io, } } else { a } })?)),
            0x00F6 => Ok(Self::CMSG_MOVE_HOVER_ACK(<CMSG_MOVE_HOVER_ACK as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x00F6, size: body_size, io, } } else { a } })?)),
            0x00FB => crate::util::assert_empty(body_size, opcode).map(|_| Self::CMSG_NEXT_CINEMATIC_CAMERA),
            0x00FC => crate::util::assert_empty(body_size, opcode).map(|_| Self::CMSG_COMPLETE_CINEMATIC),
            0x00FE => Ok(Self::CMSG_TUTORIAL_FLAG(<CMSG_TUTORIAL_FLAG as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x00FE, size: body_size, io, } } else { a } })?)),
            0x00FF => crate::util::assert_empty(body_size, opcode).map(|_| Self::CMSG_TUTORIAL_CLEAR),
            0x0100 => crate::util::assert_empty(body_size, opcode).map(|_| Self::CMSG_TUTORIAL_RESET),
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
            0x0117 => crate::util::assert_empty(body_size, opcode).map(|_| Self::CMSG_BEGIN_TRADE),
            0x0118 => crate::util::assert_empty(body_size, opcode).map(|_| Self::CMSG_BUSY_TRADE),
            0x0119 => crate::util::assert_empty(body_size, opcode).map(|_| Self::CMSG_IGNORE_TRADE),
            0x011A => Ok(Self::CMSG_ACCEPT_TRADE(<CMSG_ACCEPT_TRADE as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x011A, size: body_size, io, } } else { a } })?)),
            0x011B => crate::util::assert_empty(body_size, opcode).map(|_| Self::CMSG_UNACCEPT_TRADE),
            0x011C => crate::util::assert_empty(body_size, opcode).map(|_| Self::CMSG_CANCEL_TRADE),
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
            0x013E => Ok(Self::CMSG_SET_TARGET_OBSOLETE(<CMSG_SET_TARGET_OBSOLETE as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x013E, size: body_size, io, } } else { a } })?)),
            0x0141 => Ok(Self::CMSG_ATTACKSWING(<CMSG_ATTACKSWING as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x0141, size: body_size, io, } } else { a } })?)),
            0x0142 => crate::util::assert_empty(body_size, opcode).map(|_| Self::CMSG_ATTACKSTOP),
            0x015A => crate::util::assert_empty(body_size, opcode).map(|_| Self::CMSG_REPOP_REQUEST),
            0x015C => Ok(Self::CMSG_RESURRECT_RESPONSE(<CMSG_RESURRECT_RESPONSE as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x015C, size: body_size, io, } } else { a } })?)),
            0x015D => Ok(Self::CMSG_LOOT(<CMSG_LOOT as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x015D, size: body_size, io, } } else { a } })?)),
            0x015E => crate::util::assert_empty(body_size, opcode).map(|_| Self::CMSG_LOOT_MONEY),
            0x015F => Ok(Self::CMSG_LOOT_RELEASE(<CMSG_LOOT_RELEASE as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x015F, size: body_size, io, } } else { a } })?)),
            0x016C => Ok(Self::CMSG_DUEL_ACCEPTED(<CMSG_DUEL_ACCEPTED as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x016C, size: body_size, io, } } else { a } })?)),
            0x016D => Ok(Self::CMSG_DUEL_CANCELLED(<CMSG_DUEL_CANCELLED as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x016D, size: body_size, io, } } else { a } })?)),
            0x0171 => crate::util::assert_empty(body_size, opcode).map(|_| Self::CMSG_MOUNTSPECIAL_ANIM),
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
            0x0187 => crate::util::assert_empty(body_size, opcode).map(|_| Self::CMSG_QUESTGIVER_QUEST_AUTOLAUNCH),
            0x0189 => Ok(Self::CMSG_QUESTGIVER_ACCEPT_QUEST(<CMSG_QUESTGIVER_ACCEPT_QUEST as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x0189, size: body_size, io, } } else { a } })?)),
            0x018A => Ok(Self::CMSG_QUESTGIVER_COMPLETE_QUEST(<CMSG_QUESTGIVER_COMPLETE_QUEST as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x018A, size: body_size, io, } } else { a } })?)),
            0x018C => Ok(Self::CMSG_QUESTGIVER_REQUEST_REWARD(<CMSG_QUESTGIVER_REQUEST_REWARD as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x018C, size: body_size, io, } } else { a } })?)),
            0x018E => Ok(Self::CMSG_QUESTGIVER_CHOOSE_REWARD(<CMSG_QUESTGIVER_CHOOSE_REWARD as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x018E, size: body_size, io, } } else { a } })?)),
            0x0190 => crate::util::assert_empty(body_size, opcode).map(|_| Self::CMSG_QUESTGIVER_CANCEL),
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
            0x01CC => crate::util::assert_empty(body_size, opcode).map(|_| Self::CMSG_PLAYED_TIME),
            0x01CE => crate::util::assert_empty(body_size, opcode).map(|_| Self::CMSG_QUERY_TIME),
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
            0x01FF => crate::util::assert_empty(body_size, opcode).map(|_| Self::MSG_LOOKING_FOR_GROUP),
            0x0202 => Ok(Self::CMSG_UNLEARN_SKILL(<CMSG_UNLEARN_SKILL as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x0202, size: body_size, io, } } else { a } })?)),
            0x0205 => Ok(Self::CMSG_GMTICKET_CREATE(<CMSG_GMTICKET_CREATE as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x0205, size: body_size, io, } } else { a } })?)),
            0x0207 => Ok(Self::CMSG_GMTICKET_UPDATETEXT(<CMSG_GMTICKET_UPDATETEXT as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x0207, size: body_size, io, } } else { a } })?)),
            0x020A => Ok(Self::CMSG_REQUEST_ACCOUNT_DATA(<CMSG_REQUEST_ACCOUNT_DATA as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x020A, size: body_size, io, } } else { a } })?)),
            0x020B => Ok(Self::CMSG_UPDATE_ACCOUNT_DATA(<CMSG_UPDATE_ACCOUNT_DATA as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x020B, size: body_size, io, } } else { a } })?)),
            0x0211 => crate::util::assert_empty(body_size, opcode).map(|_| Self::CMSG_GMTICKET_GETTICKET),
            0x0216 => crate::util::assert_empty(body_size, opcode).map(|_| Self::MSG_CORPSE_QUERY),
            0x0217 => crate::util::assert_empty(body_size, opcode).map(|_| Self::CMSG_GMTICKET_DELETETICKET),
            0x021A => crate::util::assert_empty(body_size, opcode).map(|_| Self::CMSG_GMTICKET_SYSTEMSTATUS),
            0x021C => Ok(Self::CMSG_SPIRIT_HEALER_ACTIVATE(<CMSG_SPIRIT_HEALER_ACTIVATE as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x021C, size: body_size, io, } } else { a } })?)),
            0x0225 => Ok(Self::CMSG_CHAT_IGNORED(<CMSG_CHAT_IGNORED as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x0225, size: body_size, io, } } else { a } })?)),
            0x0231 => Ok(Self::CMSG_GUILD_RANK(<CMSG_GUILD_RANK as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x0231, size: body_size, io, } } else { a } })?)),
            0x0232 => Ok(Self::CMSG_GUILD_ADD_RANK(<CMSG_GUILD_ADD_RANK as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x0232, size: body_size, io, } } else { a } })?)),
            0x0233 => crate::util::assert_empty(body_size, opcode).map(|_| Self::CMSG_GUILD_DEL_RANK),
            0x0234 => Ok(Self::CMSG_GUILD_SET_PUBLIC_NOTE(<CMSG_GUILD_SET_PUBLIC_NOTE as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x0234, size: body_size, io, } } else { a } })?)),
            0x0235 => Ok(Self::CMSG_GUILD_SET_OFFICER_NOTE(<CMSG_GUILD_SET_OFFICER_NOTE as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x0235, size: body_size, io, } } else { a } })?)),
            0x0238 => Ok(Self::CMSG_SEND_MAIL(<CMSG_SEND_MAIL as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x0238, size: body_size, io, } } else { a } })?)),
            0x023A => Ok(Self::CMSG_GET_MAIL_LIST(<CMSG_GET_MAIL_LIST as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x023A, size: body_size, io, } } else { a } })?)),
            0x023C => Ok(Self::CMSG_BATTLEFIELD_LIST(<CMSG_BATTLEFIELD_LIST as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x023C, size: body_size, io, } } else { a } })?)),
            0x023E => Ok(Self::CMSG_BATTLEFIELD_JOIN(<CMSG_BATTLEFIELD_JOIN as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x023E, size: body_size, io, } } else { a } })?)),
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
            0x026D => crate::util::assert_empty(body_size, opcode).map(|_| Self::CMSG_CANCEL_AUTO_REPEAT_SPELL),
            0x026F => Ok(Self::MSG_LIST_STABLED_PETS(<MSG_LIST_STABLED_PETS_Client as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x026F, size: body_size, io, } } else { a } })?)),
            0x0270 => Ok(Self::CMSG_STABLE_PET(<CMSG_STABLE_PET as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x0270, size: body_size, io, } } else { a } })?)),
            0x0271 => Ok(Self::CMSG_UNSTABLE_PET(<CMSG_UNSTABLE_PET as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x0271, size: body_size, io, } } else { a } })?)),
            0x0272 => Ok(Self::CMSG_BUY_STABLE_SLOT(<CMSG_BUY_STABLE_SLOT as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x0272, size: body_size, io, } } else { a } })?)),
            0x0275 => Ok(Self::CMSG_STABLE_SWAP_PET(<CMSG_STABLE_SWAP_PET as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x0275, size: body_size, io, } } else { a } })?)),
            0x0279 => crate::util::assert_empty(body_size, opcode).map(|_| Self::CMSG_REQUEST_PET_INFO),
            0x027A => Ok(Self::CMSG_FAR_SIGHT(<CMSG_FAR_SIGHT as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x027A, size: body_size, io, } } else { a } })?)),
            0x027E => Ok(Self::CMSG_GROUP_CHANGE_SUB_GROUP(<CMSG_GROUP_CHANGE_SUB_GROUP as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x027E, size: body_size, io, } } else { a } })?)),
            0x027F => Ok(Self::CMSG_REQUEST_PARTY_MEMBER_STATS(<CMSG_REQUEST_PARTY_MEMBER_STATS as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x027F, size: body_size, io, } } else { a } })?)),
            0x0280 => Ok(Self::CMSG_GROUP_SWAP_SUB_GROUP(<CMSG_GROUP_SWAP_SUB_GROUP as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x0280, size: body_size, io, } } else { a } })?)),
            0x0282 => Ok(Self::CMSG_AUTOSTORE_BANK_ITEM(<CMSG_AUTOSTORE_BANK_ITEM as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x0282, size: body_size, io, } } else { a } })?)),
            0x0283 => Ok(Self::CMSG_AUTOBANK_ITEM(<CMSG_AUTOBANK_ITEM as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x0283, size: body_size, io, } } else { a } })?)),
            0x0284 => crate::util::assert_empty(body_size, opcode).map(|_| Self::MSG_QUERY_NEXT_MAIL_TIME),
            0x028E => crate::util::assert_empty(body_size, opcode).map(|_| Self::CMSG_GROUP_RAID_CONVERT),
            0x028F => Ok(Self::CMSG_GROUP_ASSISTANT_LEADER(<CMSG_GROUP_ASSISTANT_LEADER as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x028F, size: body_size, io, } } else { a } })?)),
            0x0290 => Ok(Self::CMSG_BUYBACK_ITEM(<CMSG_BUYBACK_ITEM as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x0290, size: body_size, io, } } else { a } })?)),
            0x0292 => Ok(Self::CMSG_MEETINGSTONE_JOIN(<CMSG_MEETINGSTONE_JOIN as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x0292, size: body_size, io, } } else { a } })?)),
            0x0293 => crate::util::assert_empty(body_size, opcode).map(|_| Self::CMSG_MEETINGSTONE_LEAVE),
            0x0296 => crate::util::assert_empty(body_size, opcode).map(|_| Self::CMSG_MEETINGSTONE_INFO),
            0x029B => crate::util::assert_empty(body_size, opcode).map(|_| Self::CMSG_CANCEL_GROWTH_AURA),
            0x02A0 => Ok(Self::CMSG_LOOT_ROLL(<CMSG_LOOT_ROLL as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x02A0, size: body_size, io, } } else { a } })?)),
            0x02A3 => Ok(Self::CMSG_LOOT_MASTER_GIVE(<CMSG_LOOT_MASTER_GIVE as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x02A3, size: body_size, io, } } else { a } })?)),
            0x02A8 => Ok(Self::CMSG_REPAIR_ITEM(<CMSG_REPAIR_ITEM as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x02A8, size: body_size, io, } } else { a } })?)),
            0x02AA => Ok(Self::MSG_TALENT_WIPE_CONFIRM(<MSG_TALENT_WIPE_CONFIRM_Client as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x02AA, size: body_size, io, } } else { a } })?)),
            0x02AC => Ok(Self::CMSG_SUMMON_RESPONSE(<CMSG_SUMMON_RESPONSE as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x02AC, size: body_size, io, } } else { a } })?)),
            0x02B3 => crate::util::assert_empty(body_size, opcode).map(|_| Self::CMSG_SELF_RES),
            0x02B9 => crate::util::assert_empty(body_size, opcode).map(|_| Self::CMSG_TOGGLE_HELM),
            0x02BA => crate::util::assert_empty(body_size, opcode).map(|_| Self::CMSG_TOGGLE_CLOAK),
            0x02BF => Ok(Self::CMSG_SET_ACTIONBAR_TOGGLES(<CMSG_SET_ACTIONBAR_TOGGLES as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x02BF, size: body_size, io, } } else { a } })?)),
            0x02C4 => Ok(Self::CMSG_ITEM_NAME_QUERY(<CMSG_ITEM_NAME_QUERY as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x02C4, size: body_size, io, } } else { a } })?)),
            0x02C7 => Ok(Self::CMSG_CHAR_RENAME(<CMSG_CHAR_RENAME as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x02C7, size: body_size, io, } } else { a } })?)),
            0x02C9 => Ok(Self::CMSG_MOVE_SPLINE_DONE(<CMSG_MOVE_SPLINE_DONE as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x02C9, size: body_size, io, } } else { a } })?)),
            0x02CA => Ok(Self::CMSG_MOVE_FALL_RESET(<CMSG_MOVE_FALL_RESET as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x02CA, size: body_size, io, } } else { a } })?)),
            0x02CD => crate::util::assert_empty(body_size, opcode).map(|_| Self::CMSG_REQUEST_RAID_INFO),
            0x02CE => Ok(Self::CMSG_MOVE_TIME_SKIPPED(<CMSG_MOVE_TIME_SKIPPED as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x02CE, size: body_size, io, } } else { a } })?)),
            0x02CF => Ok(Self::CMSG_MOVE_FEATHER_FALL_ACK(<CMSG_MOVE_FEATHER_FALL_ACK as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x02CF, size: body_size, io, } } else { a } })?)),
            0x02D0 => Ok(Self::CMSG_MOVE_WATER_WALK_ACK(<CMSG_MOVE_WATER_WALK_ACK as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x02D0, size: body_size, io, } } else { a } })?)),
            0x02D1 => Ok(Self::CMSG_MOVE_NOT_ACTIVE_MOVER(<CMSG_MOVE_NOT_ACTIVE_MOVER as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x02D1, size: body_size, io, } } else { a } })?)),
            0x02D3 => crate::util::assert_empty(body_size, opcode).map(|_| Self::CMSG_BATTLEFIELD_STATUS),
            0x02D5 => Ok(Self::CMSG_BATTLEFIELD_PORT(<CMSG_BATTLEFIELD_PORT as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x02D5, size: body_size, io, } } else { a } })?)),
            0x02D6 => Ok(Self::MSG_INSPECT_HONOR_STATS(<MSG_INSPECT_HONOR_STATS_Client as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x02D6, size: body_size, io, } } else { a } })?)),
            0x02D7 => Ok(Self::CMSG_BATTLEMASTER_HELLO(<CMSG_BATTLEMASTER_HELLO as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x02D7, size: body_size, io, } } else { a } })?)),
            0x02DB => Ok(Self::CMSG_FORCE_WALK_SPEED_CHANGE_ACK(<CMSG_FORCE_WALK_SPEED_CHANGE_ACK as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x02DB, size: body_size, io, } } else { a } })?)),
            0x02DD => Ok(Self::CMSG_FORCE_SWIM_BACK_SPEED_CHANGE_ACK(<CMSG_FORCE_SWIM_BACK_SPEED_CHANGE_ACK as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x02DD, size: body_size, io, } } else { a } })?)),
            0x02DF => Ok(Self::CMSG_FORCE_TURN_RATE_CHANGE_ACK(<CMSG_FORCE_TURN_RATE_CHANGE_ACK as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x02DF, size: body_size, io, } } else { a } })?)),
            0x02E0 => crate::util::assert_empty(body_size, opcode).map(|_| Self::MSG_PVP_LOG_DATA),
            0x02E1 => Ok(Self::CMSG_LEAVE_BATTLEFIELD(<CMSG_LEAVE_BATTLEFIELD as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x02E1, size: body_size, io, } } else { a } })?)),
            0x02E2 => Ok(Self::CMSG_AREA_SPIRIT_HEALER_QUERY(<CMSG_AREA_SPIRIT_HEALER_QUERY as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x02E2, size: body_size, io, } } else { a } })?)),
            0x02E3 => Ok(Self::CMSG_AREA_SPIRIT_HEALER_QUEUE(<CMSG_AREA_SPIRIT_HEALER_QUEUE as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x02E3, size: body_size, io, } } else { a } })?)),
            0x02E7 => Ok(Self::CMSG_WARDEN_DATA(<CMSG_WARDEN_DATA as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x02E7, size: body_size, io, } } else { a } })?)),
            0x02E9 => crate::util::assert_empty(body_size, opcode).map(|_| Self::MSG_BATTLEGROUND_PLAYER_POSITIONS),
            0x02EA => Ok(Self::CMSG_PET_STOP_ATTACK(<CMSG_PET_STOP_ATTACK as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x02EA, size: body_size, io, } } else { a } })?)),
            0x02EE => Ok(Self::CMSG_BATTLEMASTER_JOIN(<CMSG_BATTLEMASTER_JOIN as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x02EE, size: body_size, io, } } else { a } })?)),
            0x02F0 => Ok(Self::CMSG_PET_UNLEARN(<CMSG_PET_UNLEARN as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x02F0, size: body_size, io, } } else { a } })?)),
            0x02F3 => Ok(Self::CMSG_PET_SPELL_AUTOCAST(<CMSG_PET_SPELL_AUTOCAST as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x02F3, size: body_size, io, } } else { a } })?)),
            0x02FC => Ok(Self::CMSG_GUILD_INFO_TEXT(<CMSG_GUILD_INFO_TEXT as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x02FC, size: body_size, io, } } else { a } })?)),
            0x0312 => Ok(Self::CMSG_ACTIVATETAXIEXPRESS(<CMSG_ACTIVATETAXIEXPRESS as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x0312, size: body_size, io, } } else { a } })?)),
            0x0317 => Ok(Self::CMSG_SET_FACTION_INACTIVE(<CMSG_SET_FACTION_INACTIVE as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x0317, size: body_size, io, } } else { a } })?)),
            0x0318 => Ok(Self::CMSG_SET_WATCHED_FACTION(<CMSG_SET_WATCHED_FACTION as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x0318, size: body_size, io, } } else { a } })?)),
            0x031D => crate::util::assert_empty(body_size, opcode).map(|_| Self::CMSG_RESET_INSTANCES),
            0x0321 => Ok(Self::MSG_RAID_TARGET_UPDATE(<MSG_RAID_TARGET_UPDATE_Client as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x0321, size: body_size, io, } } else { a } })?)),
            0x0322 => Ok(Self::MSG_RAID_READY_CHECK(<MSG_RAID_READY_CHECK_Client as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x0322, size: body_size, io, } } else { a } })?)),
            0x032A => Ok(Self::CMSG_GMSURVEY_SUBMIT(<CMSG_GMSURVEY_SUBMIT as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x032A, size: body_size, io, } } else { a } })?)),
            _ => Err(crate::errors::ExpectedOpcodeError::Opcode{ opcode, name: opcode_to_name(opcode), size: body_size }),
        }
    }

    #[cfg(feature = "sync")]
    pub fn read_unencrypted<R: std::io::Read>(mut r: R) -> std::result::Result<Self, crate::errors::ExpectedOpcodeError> {
        let size = (crate::util::read_u16_be(&mut r)?.saturating_sub(4)) as u32;
        let opcode = crate::util::read_u32_le(&mut r)?;

        let mut buf = vec![0; size as usize];
        r.read_exact(&mut buf)?;
        Self::read_opcodes(opcode, size, &buf)
    }
    #[cfg(all(feature = "sync", feature = "encryption"))]
    pub fn read_encrypted<R: std::io::Read>(mut r: R, d: &mut DecrypterHalf) -> std::result::Result<Self, crate::errors::ExpectedOpcodeError> {
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
    pub async fn tokio_read_unencrypted<R: tokio::io::AsyncReadExt + Unpin + Send>(mut r: R) -> std::result::Result<Self, crate::errors::ExpectedOpcodeError> {
        let size = (crate::util::tokio_read_u16_be(&mut r).await?.saturating_sub(4)) as u32;
        let opcode = crate::util::tokio_read_u32_le(&mut r).await?;

        let mut buf = vec![0; size as usize];
        r.read_exact(&mut buf).await?;
        Self::read_opcodes(opcode, size, &buf)
    }
    #[cfg(all(feature = "tokio", feature = "encryption"))]
    pub async fn tokio_read_encrypted<R: tokio::io::AsyncReadExt + Unpin + Send>(mut r: R, d: &mut DecrypterHalf) -> std::result::Result<Self, crate::errors::ExpectedOpcodeError> {
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
    pub async fn astd_read_unencrypted<R: async_std::io::ReadExt + Unpin + Send>(mut r: R) -> std::result::Result<Self, crate::errors::ExpectedOpcodeError> {
        let size = (crate::util::astd_read_u16_be(&mut r).await?.saturating_sub(4)) as u32;
        let opcode = crate::util::astd_read_u32_le(&mut r).await?;

        let mut buf = vec![0; size as usize];
        r.read_exact(&mut buf).await?;
        Self::read_opcodes(opcode, size, &buf)
    }
    #[cfg(all(feature = "async-std", feature = "encryption"))]
    pub async fn astd_read_encrypted<R: async_std::io::ReadExt + Unpin + Send>(mut r: R, d: &mut DecrypterHalf) -> std::result::Result<Self, crate::errors::ExpectedOpcodeError> {
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
    pub fn write_encrypted_client<W: std::io::Write>(&self, mut w: W, e: &mut EncrypterHalf) -> Result<(), std::io::Error> {
        match self {
            Self::MSG_MOVE_WORLDPORT_ACK => MSG_MOVE_WORLDPORT_ACK{}.write_encrypted_client(w, e),
            Self::MSG_PETITION_DECLINE(c) => c.write_encrypted_client(w, e),
            Self::MSG_TABARDVENDOR_ACTIVATE(c) => c.write_encrypted_client(w, e),
            Self::MSG_QUEST_PUSH_RESULT(c) => c.write_encrypted_client(w, e),
            Self::MSG_MOVE_WATER_WALK(c) => c.write_encrypted_client(w, e),
            Self::MSG_PETITION_RENAME(c) => c.write_encrypted_client(w, e),
            Self::CMSG_BOOTME => CMSG_BOOTME{}.write_encrypted_client(w, e),
            Self::CMSG_DBLOOKUP(c) => c.write_encrypted_client(w, e),
            Self::CMSG_WORLD_TELEPORT(c) => c.write_encrypted_client(w, e),
            Self::CMSG_TELEPORT_TO_UNIT(c) => c.write_encrypted_client(w, e),
            Self::CMSG_CHAR_CREATE(c) => c.write_encrypted_client(w, e),
            Self::CMSG_CHAR_ENUM => CMSG_CHAR_ENUM{}.write_encrypted_client(w, e),
            Self::CMSG_CHAR_DELETE(c) => c.write_encrypted_client(w, e),
            Self::CMSG_PLAYER_LOGIN(c) => c.write_encrypted_client(w, e),
            Self::CMSG_PLAYER_LOGOUT => CMSG_PLAYER_LOGOUT{}.write_encrypted_client(w, e),
            Self::CMSG_LOGOUT_REQUEST => CMSG_LOGOUT_REQUEST{}.write_encrypted_client(w, e),
            Self::CMSG_LOGOUT_CANCEL => CMSG_LOGOUT_CANCEL{}.write_encrypted_client(w, e),
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
            Self::CMSG_FRIEND_LIST => CMSG_FRIEND_LIST{}.write_encrypted_client(w, e),
            Self::CMSG_ADD_FRIEND(c) => c.write_encrypted_client(w, e),
            Self::CMSG_DEL_FRIEND(c) => c.write_encrypted_client(w, e),
            Self::CMSG_ADD_IGNORE(c) => c.write_encrypted_client(w, e),
            Self::CMSG_DEL_IGNORE(c) => c.write_encrypted_client(w, e),
            Self::CMSG_GROUP_INVITE(c) => c.write_encrypted_client(w, e),
            Self::CMSG_GROUP_ACCEPT => CMSG_GROUP_ACCEPT{}.write_encrypted_client(w, e),
            Self::CMSG_GROUP_DECLINE => CMSG_GROUP_DECLINE{}.write_encrypted_client(w, e),
            Self::CMSG_GROUP_UNINVITE(c) => c.write_encrypted_client(w, e),
            Self::CMSG_GROUP_UNINVITE_GUID(c) => c.write_encrypted_client(w, e),
            Self::CMSG_GROUP_SET_LEADER(c) => c.write_encrypted_client(w, e),
            Self::CMSG_LOOT_METHOD(c) => c.write_encrypted_client(w, e),
            Self::CMSG_GROUP_DISBAND => CMSG_GROUP_DISBAND{}.write_encrypted_client(w, e),
            Self::CMSG_GUILD_CREATE(c) => c.write_encrypted_client(w, e),
            Self::CMSG_GUILD_INVITE(c) => c.write_encrypted_client(w, e),
            Self::CMSG_GUILD_ACCEPT => CMSG_GUILD_ACCEPT{}.write_encrypted_client(w, e),
            Self::CMSG_GUILD_DECLINE => CMSG_GUILD_DECLINE{}.write_encrypted_client(w, e),
            Self::CMSG_GUILD_INFO => CMSG_GUILD_INFO{}.write_encrypted_client(w, e),
            Self::CMSG_GUILD_ROSTER => CMSG_GUILD_ROSTER{}.write_encrypted_client(w, e),
            Self::CMSG_GUILD_PROMOTE(c) => c.write_encrypted_client(w, e),
            Self::CMSG_GUILD_DEMOTE(c) => c.write_encrypted_client(w, e),
            Self::CMSG_GUILD_LEAVE => CMSG_GUILD_LEAVE{}.write_encrypted_client(w, e),
            Self::CMSG_GUILD_REMOVE(c) => c.write_encrypted_client(w, e),
            Self::CMSG_GUILD_DISBAND => CMSG_GUILD_DISBAND{}.write_encrypted_client(w, e),
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
            Self::MSG_MOVE_TELEPORT_ACK(c) => c.write_encrypted_client(w, e),
            Self::MSG_MOVE_FALL_LAND(c) => c.write_encrypted_client(w, e),
            Self::MSG_MOVE_START_SWIM(c) => c.write_encrypted_client(w, e),
            Self::MSG_MOVE_STOP_SWIM(c) => c.write_encrypted_client(w, e),
            Self::MSG_MOVE_SET_FACING(c) => c.write_encrypted_client(w, e),
            Self::MSG_MOVE_SET_PITCH(c) => c.write_encrypted_client(w, e),
            Self::CMSG_MOVE_SET_RAW_POSITION(c) => c.write_encrypted_client(w, e),
            Self::CMSG_FORCE_RUN_SPEED_CHANGE_ACK(c) => c.write_encrypted_client(w, e),
            Self::CMSG_FORCE_RUN_BACK_SPEED_CHANGE_ACK(c) => c.write_encrypted_client(w, e),
            Self::CMSG_FORCE_SWIM_SPEED_CHANGE_ACK(c) => c.write_encrypted_client(w, e),
            Self::CMSG_FORCE_MOVE_ROOT_ACK(c) => c.write_encrypted_client(w, e),
            Self::CMSG_FORCE_MOVE_UNROOT_ACK(c) => c.write_encrypted_client(w, e),
            Self::MSG_MOVE_HEARTBEAT(c) => c.write_encrypted_client(w, e),
            Self::CMSG_MOVE_KNOCK_BACK_ACK(c) => c.write_encrypted_client(w, e),
            Self::CMSG_MOVE_HOVER_ACK(c) => c.write_encrypted_client(w, e),
            Self::CMSG_NEXT_CINEMATIC_CAMERA => CMSG_NEXT_CINEMATIC_CAMERA{}.write_encrypted_client(w, e),
            Self::CMSG_COMPLETE_CINEMATIC => CMSG_COMPLETE_CINEMATIC{}.write_encrypted_client(w, e),
            Self::CMSG_TUTORIAL_FLAG(c) => c.write_encrypted_client(w, e),
            Self::CMSG_TUTORIAL_CLEAR => CMSG_TUTORIAL_CLEAR{}.write_encrypted_client(w, e),
            Self::CMSG_TUTORIAL_RESET => CMSG_TUTORIAL_RESET{}.write_encrypted_client(w, e),
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
            Self::CMSG_BEGIN_TRADE => CMSG_BEGIN_TRADE{}.write_encrypted_client(w, e),
            Self::CMSG_BUSY_TRADE => CMSG_BUSY_TRADE{}.write_encrypted_client(w, e),
            Self::CMSG_IGNORE_TRADE => CMSG_IGNORE_TRADE{}.write_encrypted_client(w, e),
            Self::CMSG_ACCEPT_TRADE(c) => c.write_encrypted_client(w, e),
            Self::CMSG_UNACCEPT_TRADE => CMSG_UNACCEPT_TRADE{}.write_encrypted_client(w, e),
            Self::CMSG_CANCEL_TRADE => CMSG_CANCEL_TRADE{}.write_encrypted_client(w, e),
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
            Self::CMSG_SET_TARGET_OBSOLETE(c) => c.write_encrypted_client(w, e),
            Self::CMSG_ATTACKSWING(c) => c.write_encrypted_client(w, e),
            Self::CMSG_ATTACKSTOP => CMSG_ATTACKSTOP{}.write_encrypted_client(w, e),
            Self::CMSG_REPOP_REQUEST => CMSG_REPOP_REQUEST{}.write_encrypted_client(w, e),
            Self::CMSG_RESURRECT_RESPONSE(c) => c.write_encrypted_client(w, e),
            Self::CMSG_LOOT(c) => c.write_encrypted_client(w, e),
            Self::CMSG_LOOT_MONEY => CMSG_LOOT_MONEY{}.write_encrypted_client(w, e),
            Self::CMSG_LOOT_RELEASE(c) => c.write_encrypted_client(w, e),
            Self::CMSG_DUEL_ACCEPTED(c) => c.write_encrypted_client(w, e),
            Self::CMSG_DUEL_CANCELLED(c) => c.write_encrypted_client(w, e),
            Self::CMSG_MOUNTSPECIAL_ANIM => CMSG_MOUNTSPECIAL_ANIM{}.write_encrypted_client(w, e),
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
            Self::CMSG_QUESTGIVER_QUEST_AUTOLAUNCH => CMSG_QUESTGIVER_QUEST_AUTOLAUNCH{}.write_encrypted_client(w, e),
            Self::CMSG_QUESTGIVER_ACCEPT_QUEST(c) => c.write_encrypted_client(w, e),
            Self::CMSG_QUESTGIVER_COMPLETE_QUEST(c) => c.write_encrypted_client(w, e),
            Self::CMSG_QUESTGIVER_REQUEST_REWARD(c) => c.write_encrypted_client(w, e),
            Self::CMSG_QUESTGIVER_CHOOSE_REWARD(c) => c.write_encrypted_client(w, e),
            Self::CMSG_QUESTGIVER_CANCEL => CMSG_QUESTGIVER_CANCEL{}.write_encrypted_client(w, e),
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
            Self::CMSG_PLAYED_TIME => CMSG_PLAYED_TIME{}.write_encrypted_client(w, e),
            Self::CMSG_QUERY_TIME => CMSG_QUERY_TIME{}.write_encrypted_client(w, e),
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
            Self::MSG_LOOKING_FOR_GROUP => MSG_LOOKING_FOR_GROUP_Client{}.write_encrypted_client(w, e),
            Self::CMSG_UNLEARN_SKILL(c) => c.write_encrypted_client(w, e),
            Self::CMSG_GMTICKET_CREATE(c) => c.write_encrypted_client(w, e),
            Self::CMSG_GMTICKET_UPDATETEXT(c) => c.write_encrypted_client(w, e),
            Self::CMSG_REQUEST_ACCOUNT_DATA(c) => c.write_encrypted_client(w, e),
            Self::CMSG_UPDATE_ACCOUNT_DATA(c) => c.write_encrypted_client(w, e),
            Self::CMSG_GMTICKET_GETTICKET => CMSG_GMTICKET_GETTICKET{}.write_encrypted_client(w, e),
            Self::MSG_CORPSE_QUERY => MSG_CORPSE_QUERY_Client{}.write_encrypted_client(w, e),
            Self::CMSG_GMTICKET_DELETETICKET => CMSG_GMTICKET_DELETETICKET{}.write_encrypted_client(w, e),
            Self::CMSG_GMTICKET_SYSTEMSTATUS => CMSG_GMTICKET_SYSTEMSTATUS{}.write_encrypted_client(w, e),
            Self::CMSG_SPIRIT_HEALER_ACTIVATE(c) => c.write_encrypted_client(w, e),
            Self::CMSG_CHAT_IGNORED(c) => c.write_encrypted_client(w, e),
            Self::CMSG_GUILD_RANK(c) => c.write_encrypted_client(w, e),
            Self::CMSG_GUILD_ADD_RANK(c) => c.write_encrypted_client(w, e),
            Self::CMSG_GUILD_DEL_RANK => CMSG_GUILD_DEL_RANK{}.write_encrypted_client(w, e),
            Self::CMSG_GUILD_SET_PUBLIC_NOTE(c) => c.write_encrypted_client(w, e),
            Self::CMSG_GUILD_SET_OFFICER_NOTE(c) => c.write_encrypted_client(w, e),
            Self::CMSG_SEND_MAIL(c) => c.write_encrypted_client(w, e),
            Self::CMSG_GET_MAIL_LIST(c) => c.write_encrypted_client(w, e),
            Self::CMSG_BATTLEFIELD_LIST(c) => c.write_encrypted_client(w, e),
            Self::CMSG_BATTLEFIELD_JOIN(c) => c.write_encrypted_client(w, e),
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
            Self::CMSG_CANCEL_AUTO_REPEAT_SPELL => CMSG_CANCEL_AUTO_REPEAT_SPELL{}.write_encrypted_client(w, e),
            Self::MSG_LIST_STABLED_PETS(c) => c.write_encrypted_client(w, e),
            Self::CMSG_STABLE_PET(c) => c.write_encrypted_client(w, e),
            Self::CMSG_UNSTABLE_PET(c) => c.write_encrypted_client(w, e),
            Self::CMSG_BUY_STABLE_SLOT(c) => c.write_encrypted_client(w, e),
            Self::CMSG_STABLE_SWAP_PET(c) => c.write_encrypted_client(w, e),
            Self::CMSG_REQUEST_PET_INFO => CMSG_REQUEST_PET_INFO{}.write_encrypted_client(w, e),
            Self::CMSG_FAR_SIGHT(c) => c.write_encrypted_client(w, e),
            Self::CMSG_GROUP_CHANGE_SUB_GROUP(c) => c.write_encrypted_client(w, e),
            Self::CMSG_REQUEST_PARTY_MEMBER_STATS(c) => c.write_encrypted_client(w, e),
            Self::CMSG_GROUP_SWAP_SUB_GROUP(c) => c.write_encrypted_client(w, e),
            Self::CMSG_AUTOSTORE_BANK_ITEM(c) => c.write_encrypted_client(w, e),
            Self::CMSG_AUTOBANK_ITEM(c) => c.write_encrypted_client(w, e),
            Self::MSG_QUERY_NEXT_MAIL_TIME => MSG_QUERY_NEXT_MAIL_TIME_Client{}.write_encrypted_client(w, e),
            Self::CMSG_GROUP_RAID_CONVERT => CMSG_GROUP_RAID_CONVERT{}.write_encrypted_client(w, e),
            Self::CMSG_GROUP_ASSISTANT_LEADER(c) => c.write_encrypted_client(w, e),
            Self::CMSG_BUYBACK_ITEM(c) => c.write_encrypted_client(w, e),
            Self::CMSG_MEETINGSTONE_JOIN(c) => c.write_encrypted_client(w, e),
            Self::CMSG_MEETINGSTONE_LEAVE => CMSG_MEETINGSTONE_LEAVE{}.write_encrypted_client(w, e),
            Self::CMSG_MEETINGSTONE_INFO => CMSG_MEETINGSTONE_INFO{}.write_encrypted_client(w, e),
            Self::CMSG_CANCEL_GROWTH_AURA => CMSG_CANCEL_GROWTH_AURA{}.write_encrypted_client(w, e),
            Self::CMSG_LOOT_ROLL(c) => c.write_encrypted_client(w, e),
            Self::CMSG_LOOT_MASTER_GIVE(c) => c.write_encrypted_client(w, e),
            Self::CMSG_REPAIR_ITEM(c) => c.write_encrypted_client(w, e),
            Self::MSG_TALENT_WIPE_CONFIRM(c) => c.write_encrypted_client(w, e),
            Self::CMSG_SUMMON_RESPONSE(c) => c.write_encrypted_client(w, e),
            Self::CMSG_SELF_RES => CMSG_SELF_RES{}.write_encrypted_client(w, e),
            Self::CMSG_TOGGLE_HELM => CMSG_TOGGLE_HELM{}.write_encrypted_client(w, e),
            Self::CMSG_TOGGLE_CLOAK => CMSG_TOGGLE_CLOAK{}.write_encrypted_client(w, e),
            Self::CMSG_SET_ACTIONBAR_TOGGLES(c) => c.write_encrypted_client(w, e),
            Self::CMSG_ITEM_NAME_QUERY(c) => c.write_encrypted_client(w, e),
            Self::CMSG_CHAR_RENAME(c) => c.write_encrypted_client(w, e),
            Self::CMSG_MOVE_SPLINE_DONE(c) => c.write_encrypted_client(w, e),
            Self::CMSG_MOVE_FALL_RESET(c) => c.write_encrypted_client(w, e),
            Self::CMSG_REQUEST_RAID_INFO => CMSG_REQUEST_RAID_INFO{}.write_encrypted_client(w, e),
            Self::CMSG_MOVE_TIME_SKIPPED(c) => c.write_encrypted_client(w, e),
            Self::CMSG_MOVE_FEATHER_FALL_ACK(c) => c.write_encrypted_client(w, e),
            Self::CMSG_MOVE_WATER_WALK_ACK(c) => c.write_encrypted_client(w, e),
            Self::CMSG_MOVE_NOT_ACTIVE_MOVER(c) => c.write_encrypted_client(w, e),
            Self::CMSG_BATTLEFIELD_STATUS => CMSG_BATTLEFIELD_STATUS{}.write_encrypted_client(w, e),
            Self::CMSG_BATTLEFIELD_PORT(c) => c.write_encrypted_client(w, e),
            Self::MSG_INSPECT_HONOR_STATS(c) => c.write_encrypted_client(w, e),
            Self::CMSG_BATTLEMASTER_HELLO(c) => c.write_encrypted_client(w, e),
            Self::CMSG_FORCE_WALK_SPEED_CHANGE_ACK(c) => c.write_encrypted_client(w, e),
            Self::CMSG_FORCE_SWIM_BACK_SPEED_CHANGE_ACK(c) => c.write_encrypted_client(w, e),
            Self::CMSG_FORCE_TURN_RATE_CHANGE_ACK(c) => c.write_encrypted_client(w, e),
            Self::MSG_PVP_LOG_DATA => MSG_PVP_LOG_DATA_Client{}.write_encrypted_client(w, e),
            Self::CMSG_LEAVE_BATTLEFIELD(c) => c.write_encrypted_client(w, e),
            Self::CMSG_AREA_SPIRIT_HEALER_QUERY(c) => c.write_encrypted_client(w, e),
            Self::CMSG_AREA_SPIRIT_HEALER_QUEUE(c) => c.write_encrypted_client(w, e),
            Self::CMSG_WARDEN_DATA(c) => c.write_encrypted_client(w, e),
            Self::MSG_BATTLEGROUND_PLAYER_POSITIONS => MSG_BATTLEGROUND_PLAYER_POSITIONS_Client{}.write_encrypted_client(w, e),
            Self::CMSG_PET_STOP_ATTACK(c) => c.write_encrypted_client(w, e),
            Self::CMSG_BATTLEMASTER_JOIN(c) => c.write_encrypted_client(w, e),
            Self::CMSG_PET_UNLEARN(c) => c.write_encrypted_client(w, e),
            Self::CMSG_PET_SPELL_AUTOCAST(c) => c.write_encrypted_client(w, e),
            Self::CMSG_GUILD_INFO_TEXT(c) => c.write_encrypted_client(w, e),
            Self::CMSG_ACTIVATETAXIEXPRESS(c) => c.write_encrypted_client(w, e),
            Self::CMSG_SET_FACTION_INACTIVE(c) => c.write_encrypted_client(w, e),
            Self::CMSG_SET_WATCHED_FACTION(c) => c.write_encrypted_client(w, e),
            Self::CMSG_RESET_INSTANCES => CMSG_RESET_INSTANCES{}.write_encrypted_client(w, e),
            Self::MSG_RAID_TARGET_UPDATE(c) => c.write_encrypted_client(w, e),
            Self::MSG_RAID_READY_CHECK(c) => c.write_encrypted_client(w, e),
            Self::CMSG_GMSURVEY_SUBMIT(c) => c.write_encrypted_client(w, e),
        }
    }

    #[cfg(feature = "sync")]
    pub fn write_unencrypted_client<W: std::io::Write>(&self, mut w: W) -> Result<(), std::io::Error> {
        match self {
            Self::MSG_MOVE_WORLDPORT_ACK => MSG_MOVE_WORLDPORT_ACK{}.write_unencrypted_client(w),
            Self::MSG_PETITION_DECLINE(c) => c.write_unencrypted_client(w),
            Self::MSG_TABARDVENDOR_ACTIVATE(c) => c.write_unencrypted_client(w),
            Self::MSG_QUEST_PUSH_RESULT(c) => c.write_unencrypted_client(w),
            Self::MSG_MOVE_WATER_WALK(c) => c.write_unencrypted_client(w),
            Self::MSG_PETITION_RENAME(c) => c.write_unencrypted_client(w),
            Self::CMSG_BOOTME => CMSG_BOOTME{}.write_unencrypted_client(w),
            Self::CMSG_DBLOOKUP(c) => c.write_unencrypted_client(w),
            Self::CMSG_WORLD_TELEPORT(c) => c.write_unencrypted_client(w),
            Self::CMSG_TELEPORT_TO_UNIT(c) => c.write_unencrypted_client(w),
            Self::CMSG_CHAR_CREATE(c) => c.write_unencrypted_client(w),
            Self::CMSG_CHAR_ENUM => CMSG_CHAR_ENUM{}.write_unencrypted_client(w),
            Self::CMSG_CHAR_DELETE(c) => c.write_unencrypted_client(w),
            Self::CMSG_PLAYER_LOGIN(c) => c.write_unencrypted_client(w),
            Self::CMSG_PLAYER_LOGOUT => CMSG_PLAYER_LOGOUT{}.write_unencrypted_client(w),
            Self::CMSG_LOGOUT_REQUEST => CMSG_LOGOUT_REQUEST{}.write_unencrypted_client(w),
            Self::CMSG_LOGOUT_CANCEL => CMSG_LOGOUT_CANCEL{}.write_unencrypted_client(w),
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
            Self::CMSG_FRIEND_LIST => CMSG_FRIEND_LIST{}.write_unencrypted_client(w),
            Self::CMSG_ADD_FRIEND(c) => c.write_unencrypted_client(w),
            Self::CMSG_DEL_FRIEND(c) => c.write_unencrypted_client(w),
            Self::CMSG_ADD_IGNORE(c) => c.write_unencrypted_client(w),
            Self::CMSG_DEL_IGNORE(c) => c.write_unencrypted_client(w),
            Self::CMSG_GROUP_INVITE(c) => c.write_unencrypted_client(w),
            Self::CMSG_GROUP_ACCEPT => CMSG_GROUP_ACCEPT{}.write_unencrypted_client(w),
            Self::CMSG_GROUP_DECLINE => CMSG_GROUP_DECLINE{}.write_unencrypted_client(w),
            Self::CMSG_GROUP_UNINVITE(c) => c.write_unencrypted_client(w),
            Self::CMSG_GROUP_UNINVITE_GUID(c) => c.write_unencrypted_client(w),
            Self::CMSG_GROUP_SET_LEADER(c) => c.write_unencrypted_client(w),
            Self::CMSG_LOOT_METHOD(c) => c.write_unencrypted_client(w),
            Self::CMSG_GROUP_DISBAND => CMSG_GROUP_DISBAND{}.write_unencrypted_client(w),
            Self::CMSG_GUILD_CREATE(c) => c.write_unencrypted_client(w),
            Self::CMSG_GUILD_INVITE(c) => c.write_unencrypted_client(w),
            Self::CMSG_GUILD_ACCEPT => CMSG_GUILD_ACCEPT{}.write_unencrypted_client(w),
            Self::CMSG_GUILD_DECLINE => CMSG_GUILD_DECLINE{}.write_unencrypted_client(w),
            Self::CMSG_GUILD_INFO => CMSG_GUILD_INFO{}.write_unencrypted_client(w),
            Self::CMSG_GUILD_ROSTER => CMSG_GUILD_ROSTER{}.write_unencrypted_client(w),
            Self::CMSG_GUILD_PROMOTE(c) => c.write_unencrypted_client(w),
            Self::CMSG_GUILD_DEMOTE(c) => c.write_unencrypted_client(w),
            Self::CMSG_GUILD_LEAVE => CMSG_GUILD_LEAVE{}.write_unencrypted_client(w),
            Self::CMSG_GUILD_REMOVE(c) => c.write_unencrypted_client(w),
            Self::CMSG_GUILD_DISBAND => CMSG_GUILD_DISBAND{}.write_unencrypted_client(w),
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
            Self::MSG_MOVE_TELEPORT_ACK(c) => c.write_unencrypted_client(w),
            Self::MSG_MOVE_FALL_LAND(c) => c.write_unencrypted_client(w),
            Self::MSG_MOVE_START_SWIM(c) => c.write_unencrypted_client(w),
            Self::MSG_MOVE_STOP_SWIM(c) => c.write_unencrypted_client(w),
            Self::MSG_MOVE_SET_FACING(c) => c.write_unencrypted_client(w),
            Self::MSG_MOVE_SET_PITCH(c) => c.write_unencrypted_client(w),
            Self::CMSG_MOVE_SET_RAW_POSITION(c) => c.write_unencrypted_client(w),
            Self::CMSG_FORCE_RUN_SPEED_CHANGE_ACK(c) => c.write_unencrypted_client(w),
            Self::CMSG_FORCE_RUN_BACK_SPEED_CHANGE_ACK(c) => c.write_unencrypted_client(w),
            Self::CMSG_FORCE_SWIM_SPEED_CHANGE_ACK(c) => c.write_unencrypted_client(w),
            Self::CMSG_FORCE_MOVE_ROOT_ACK(c) => c.write_unencrypted_client(w),
            Self::CMSG_FORCE_MOVE_UNROOT_ACK(c) => c.write_unencrypted_client(w),
            Self::MSG_MOVE_HEARTBEAT(c) => c.write_unencrypted_client(w),
            Self::CMSG_MOVE_KNOCK_BACK_ACK(c) => c.write_unencrypted_client(w),
            Self::CMSG_MOVE_HOVER_ACK(c) => c.write_unencrypted_client(w),
            Self::CMSG_NEXT_CINEMATIC_CAMERA => CMSG_NEXT_CINEMATIC_CAMERA{}.write_unencrypted_client(w),
            Self::CMSG_COMPLETE_CINEMATIC => CMSG_COMPLETE_CINEMATIC{}.write_unencrypted_client(w),
            Self::CMSG_TUTORIAL_FLAG(c) => c.write_unencrypted_client(w),
            Self::CMSG_TUTORIAL_CLEAR => CMSG_TUTORIAL_CLEAR{}.write_unencrypted_client(w),
            Self::CMSG_TUTORIAL_RESET => CMSG_TUTORIAL_RESET{}.write_unencrypted_client(w),
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
            Self::CMSG_BEGIN_TRADE => CMSG_BEGIN_TRADE{}.write_unencrypted_client(w),
            Self::CMSG_BUSY_TRADE => CMSG_BUSY_TRADE{}.write_unencrypted_client(w),
            Self::CMSG_IGNORE_TRADE => CMSG_IGNORE_TRADE{}.write_unencrypted_client(w),
            Self::CMSG_ACCEPT_TRADE(c) => c.write_unencrypted_client(w),
            Self::CMSG_UNACCEPT_TRADE => CMSG_UNACCEPT_TRADE{}.write_unencrypted_client(w),
            Self::CMSG_CANCEL_TRADE => CMSG_CANCEL_TRADE{}.write_unencrypted_client(w),
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
            Self::CMSG_SET_TARGET_OBSOLETE(c) => c.write_unencrypted_client(w),
            Self::CMSG_ATTACKSWING(c) => c.write_unencrypted_client(w),
            Self::CMSG_ATTACKSTOP => CMSG_ATTACKSTOP{}.write_unencrypted_client(w),
            Self::CMSG_REPOP_REQUEST => CMSG_REPOP_REQUEST{}.write_unencrypted_client(w),
            Self::CMSG_RESURRECT_RESPONSE(c) => c.write_unencrypted_client(w),
            Self::CMSG_LOOT(c) => c.write_unencrypted_client(w),
            Self::CMSG_LOOT_MONEY => CMSG_LOOT_MONEY{}.write_unencrypted_client(w),
            Self::CMSG_LOOT_RELEASE(c) => c.write_unencrypted_client(w),
            Self::CMSG_DUEL_ACCEPTED(c) => c.write_unencrypted_client(w),
            Self::CMSG_DUEL_CANCELLED(c) => c.write_unencrypted_client(w),
            Self::CMSG_MOUNTSPECIAL_ANIM => CMSG_MOUNTSPECIAL_ANIM{}.write_unencrypted_client(w),
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
            Self::CMSG_QUESTGIVER_QUEST_AUTOLAUNCH => CMSG_QUESTGIVER_QUEST_AUTOLAUNCH{}.write_unencrypted_client(w),
            Self::CMSG_QUESTGIVER_ACCEPT_QUEST(c) => c.write_unencrypted_client(w),
            Self::CMSG_QUESTGIVER_COMPLETE_QUEST(c) => c.write_unencrypted_client(w),
            Self::CMSG_QUESTGIVER_REQUEST_REWARD(c) => c.write_unencrypted_client(w),
            Self::CMSG_QUESTGIVER_CHOOSE_REWARD(c) => c.write_unencrypted_client(w),
            Self::CMSG_QUESTGIVER_CANCEL => CMSG_QUESTGIVER_CANCEL{}.write_unencrypted_client(w),
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
            Self::CMSG_PLAYED_TIME => CMSG_PLAYED_TIME{}.write_unencrypted_client(w),
            Self::CMSG_QUERY_TIME => CMSG_QUERY_TIME{}.write_unencrypted_client(w),
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
            Self::MSG_LOOKING_FOR_GROUP => MSG_LOOKING_FOR_GROUP_Client{}.write_unencrypted_client(w),
            Self::CMSG_UNLEARN_SKILL(c) => c.write_unencrypted_client(w),
            Self::CMSG_GMTICKET_CREATE(c) => c.write_unencrypted_client(w),
            Self::CMSG_GMTICKET_UPDATETEXT(c) => c.write_unencrypted_client(w),
            Self::CMSG_REQUEST_ACCOUNT_DATA(c) => c.write_unencrypted_client(w),
            Self::CMSG_UPDATE_ACCOUNT_DATA(c) => c.write_unencrypted_client(w),
            Self::CMSG_GMTICKET_GETTICKET => CMSG_GMTICKET_GETTICKET{}.write_unencrypted_client(w),
            Self::MSG_CORPSE_QUERY => MSG_CORPSE_QUERY_Client{}.write_unencrypted_client(w),
            Self::CMSG_GMTICKET_DELETETICKET => CMSG_GMTICKET_DELETETICKET{}.write_unencrypted_client(w),
            Self::CMSG_GMTICKET_SYSTEMSTATUS => CMSG_GMTICKET_SYSTEMSTATUS{}.write_unencrypted_client(w),
            Self::CMSG_SPIRIT_HEALER_ACTIVATE(c) => c.write_unencrypted_client(w),
            Self::CMSG_CHAT_IGNORED(c) => c.write_unencrypted_client(w),
            Self::CMSG_GUILD_RANK(c) => c.write_unencrypted_client(w),
            Self::CMSG_GUILD_ADD_RANK(c) => c.write_unencrypted_client(w),
            Self::CMSG_GUILD_DEL_RANK => CMSG_GUILD_DEL_RANK{}.write_unencrypted_client(w),
            Self::CMSG_GUILD_SET_PUBLIC_NOTE(c) => c.write_unencrypted_client(w),
            Self::CMSG_GUILD_SET_OFFICER_NOTE(c) => c.write_unencrypted_client(w),
            Self::CMSG_SEND_MAIL(c) => c.write_unencrypted_client(w),
            Self::CMSG_GET_MAIL_LIST(c) => c.write_unencrypted_client(w),
            Self::CMSG_BATTLEFIELD_LIST(c) => c.write_unencrypted_client(w),
            Self::CMSG_BATTLEFIELD_JOIN(c) => c.write_unencrypted_client(w),
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
            Self::CMSG_CANCEL_AUTO_REPEAT_SPELL => CMSG_CANCEL_AUTO_REPEAT_SPELL{}.write_unencrypted_client(w),
            Self::MSG_LIST_STABLED_PETS(c) => c.write_unencrypted_client(w),
            Self::CMSG_STABLE_PET(c) => c.write_unencrypted_client(w),
            Self::CMSG_UNSTABLE_PET(c) => c.write_unencrypted_client(w),
            Self::CMSG_BUY_STABLE_SLOT(c) => c.write_unencrypted_client(w),
            Self::CMSG_STABLE_SWAP_PET(c) => c.write_unencrypted_client(w),
            Self::CMSG_REQUEST_PET_INFO => CMSG_REQUEST_PET_INFO{}.write_unencrypted_client(w),
            Self::CMSG_FAR_SIGHT(c) => c.write_unencrypted_client(w),
            Self::CMSG_GROUP_CHANGE_SUB_GROUP(c) => c.write_unencrypted_client(w),
            Self::CMSG_REQUEST_PARTY_MEMBER_STATS(c) => c.write_unencrypted_client(w),
            Self::CMSG_GROUP_SWAP_SUB_GROUP(c) => c.write_unencrypted_client(w),
            Self::CMSG_AUTOSTORE_BANK_ITEM(c) => c.write_unencrypted_client(w),
            Self::CMSG_AUTOBANK_ITEM(c) => c.write_unencrypted_client(w),
            Self::MSG_QUERY_NEXT_MAIL_TIME => MSG_QUERY_NEXT_MAIL_TIME_Client{}.write_unencrypted_client(w),
            Self::CMSG_GROUP_RAID_CONVERT => CMSG_GROUP_RAID_CONVERT{}.write_unencrypted_client(w),
            Self::CMSG_GROUP_ASSISTANT_LEADER(c) => c.write_unencrypted_client(w),
            Self::CMSG_BUYBACK_ITEM(c) => c.write_unencrypted_client(w),
            Self::CMSG_MEETINGSTONE_JOIN(c) => c.write_unencrypted_client(w),
            Self::CMSG_MEETINGSTONE_LEAVE => CMSG_MEETINGSTONE_LEAVE{}.write_unencrypted_client(w),
            Self::CMSG_MEETINGSTONE_INFO => CMSG_MEETINGSTONE_INFO{}.write_unencrypted_client(w),
            Self::CMSG_CANCEL_GROWTH_AURA => CMSG_CANCEL_GROWTH_AURA{}.write_unencrypted_client(w),
            Self::CMSG_LOOT_ROLL(c) => c.write_unencrypted_client(w),
            Self::CMSG_LOOT_MASTER_GIVE(c) => c.write_unencrypted_client(w),
            Self::CMSG_REPAIR_ITEM(c) => c.write_unencrypted_client(w),
            Self::MSG_TALENT_WIPE_CONFIRM(c) => c.write_unencrypted_client(w),
            Self::CMSG_SUMMON_RESPONSE(c) => c.write_unencrypted_client(w),
            Self::CMSG_SELF_RES => CMSG_SELF_RES{}.write_unencrypted_client(w),
            Self::CMSG_TOGGLE_HELM => CMSG_TOGGLE_HELM{}.write_unencrypted_client(w),
            Self::CMSG_TOGGLE_CLOAK => CMSG_TOGGLE_CLOAK{}.write_unencrypted_client(w),
            Self::CMSG_SET_ACTIONBAR_TOGGLES(c) => c.write_unencrypted_client(w),
            Self::CMSG_ITEM_NAME_QUERY(c) => c.write_unencrypted_client(w),
            Self::CMSG_CHAR_RENAME(c) => c.write_unencrypted_client(w),
            Self::CMSG_MOVE_SPLINE_DONE(c) => c.write_unencrypted_client(w),
            Self::CMSG_MOVE_FALL_RESET(c) => c.write_unencrypted_client(w),
            Self::CMSG_REQUEST_RAID_INFO => CMSG_REQUEST_RAID_INFO{}.write_unencrypted_client(w),
            Self::CMSG_MOVE_TIME_SKIPPED(c) => c.write_unencrypted_client(w),
            Self::CMSG_MOVE_FEATHER_FALL_ACK(c) => c.write_unencrypted_client(w),
            Self::CMSG_MOVE_WATER_WALK_ACK(c) => c.write_unencrypted_client(w),
            Self::CMSG_MOVE_NOT_ACTIVE_MOVER(c) => c.write_unencrypted_client(w),
            Self::CMSG_BATTLEFIELD_STATUS => CMSG_BATTLEFIELD_STATUS{}.write_unencrypted_client(w),
            Self::CMSG_BATTLEFIELD_PORT(c) => c.write_unencrypted_client(w),
            Self::MSG_INSPECT_HONOR_STATS(c) => c.write_unencrypted_client(w),
            Self::CMSG_BATTLEMASTER_HELLO(c) => c.write_unencrypted_client(w),
            Self::CMSG_FORCE_WALK_SPEED_CHANGE_ACK(c) => c.write_unencrypted_client(w),
            Self::CMSG_FORCE_SWIM_BACK_SPEED_CHANGE_ACK(c) => c.write_unencrypted_client(w),
            Self::CMSG_FORCE_TURN_RATE_CHANGE_ACK(c) => c.write_unencrypted_client(w),
            Self::MSG_PVP_LOG_DATA => MSG_PVP_LOG_DATA_Client{}.write_unencrypted_client(w),
            Self::CMSG_LEAVE_BATTLEFIELD(c) => c.write_unencrypted_client(w),
            Self::CMSG_AREA_SPIRIT_HEALER_QUERY(c) => c.write_unencrypted_client(w),
            Self::CMSG_AREA_SPIRIT_HEALER_QUEUE(c) => c.write_unencrypted_client(w),
            Self::CMSG_WARDEN_DATA(c) => c.write_unencrypted_client(w),
            Self::MSG_BATTLEGROUND_PLAYER_POSITIONS => MSG_BATTLEGROUND_PLAYER_POSITIONS_Client{}.write_unencrypted_client(w),
            Self::CMSG_PET_STOP_ATTACK(c) => c.write_unencrypted_client(w),
            Self::CMSG_BATTLEMASTER_JOIN(c) => c.write_unencrypted_client(w),
            Self::CMSG_PET_UNLEARN(c) => c.write_unencrypted_client(w),
            Self::CMSG_PET_SPELL_AUTOCAST(c) => c.write_unencrypted_client(w),
            Self::CMSG_GUILD_INFO_TEXT(c) => c.write_unencrypted_client(w),
            Self::CMSG_ACTIVATETAXIEXPRESS(c) => c.write_unencrypted_client(w),
            Self::CMSG_SET_FACTION_INACTIVE(c) => c.write_unencrypted_client(w),
            Self::CMSG_SET_WATCHED_FACTION(c) => c.write_unencrypted_client(w),
            Self::CMSG_RESET_INSTANCES => CMSG_RESET_INSTANCES{}.write_unencrypted_client(w),
            Self::MSG_RAID_TARGET_UPDATE(c) => c.write_unencrypted_client(w),
            Self::MSG_RAID_READY_CHECK(c) => c.write_unencrypted_client(w),
            Self::CMSG_GMSURVEY_SUBMIT(c) => c.write_unencrypted_client(w),
        }
    }

    #[cfg(all(feature = "tokio", feature = "encryption"))]
    pub async fn tokio_write_encrypted_client<W: tokio::io::AsyncWriteExt + Unpin + Send>(&self, mut w: W, e: &mut EncrypterHalf) -> Result<(), std::io::Error> {
        match self {
            Self::MSG_MOVE_WORLDPORT_ACK => MSG_MOVE_WORLDPORT_ACK{}.tokio_write_encrypted_client(w, e).await,
            Self::MSG_PETITION_DECLINE(c) => c.tokio_write_encrypted_client(w, e).await,
            Self::MSG_TABARDVENDOR_ACTIVATE(c) => c.tokio_write_encrypted_client(w, e).await,
            Self::MSG_QUEST_PUSH_RESULT(c) => c.tokio_write_encrypted_client(w, e).await,
            Self::MSG_MOVE_WATER_WALK(c) => c.tokio_write_encrypted_client(w, e).await,
            Self::MSG_PETITION_RENAME(c) => c.tokio_write_encrypted_client(w, e).await,
            Self::CMSG_BOOTME => CMSG_BOOTME{}.tokio_write_encrypted_client(w, e).await,
            Self::CMSG_DBLOOKUP(c) => c.tokio_write_encrypted_client(w, e).await,
            Self::CMSG_WORLD_TELEPORT(c) => c.tokio_write_encrypted_client(w, e).await,
            Self::CMSG_TELEPORT_TO_UNIT(c) => c.tokio_write_encrypted_client(w, e).await,
            Self::CMSG_CHAR_CREATE(c) => c.tokio_write_encrypted_client(w, e).await,
            Self::CMSG_CHAR_ENUM => CMSG_CHAR_ENUM{}.tokio_write_encrypted_client(w, e).await,
            Self::CMSG_CHAR_DELETE(c) => c.tokio_write_encrypted_client(w, e).await,
            Self::CMSG_PLAYER_LOGIN(c) => c.tokio_write_encrypted_client(w, e).await,
            Self::CMSG_PLAYER_LOGOUT => CMSG_PLAYER_LOGOUT{}.tokio_write_encrypted_client(w, e).await,
            Self::CMSG_LOGOUT_REQUEST => CMSG_LOGOUT_REQUEST{}.tokio_write_encrypted_client(w, e).await,
            Self::CMSG_LOGOUT_CANCEL => CMSG_LOGOUT_CANCEL{}.tokio_write_encrypted_client(w, e).await,
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
            Self::CMSG_FRIEND_LIST => CMSG_FRIEND_LIST{}.tokio_write_encrypted_client(w, e).await,
            Self::CMSG_ADD_FRIEND(c) => c.tokio_write_encrypted_client(w, e).await,
            Self::CMSG_DEL_FRIEND(c) => c.tokio_write_encrypted_client(w, e).await,
            Self::CMSG_ADD_IGNORE(c) => c.tokio_write_encrypted_client(w, e).await,
            Self::CMSG_DEL_IGNORE(c) => c.tokio_write_encrypted_client(w, e).await,
            Self::CMSG_GROUP_INVITE(c) => c.tokio_write_encrypted_client(w, e).await,
            Self::CMSG_GROUP_ACCEPT => CMSG_GROUP_ACCEPT{}.tokio_write_encrypted_client(w, e).await,
            Self::CMSG_GROUP_DECLINE => CMSG_GROUP_DECLINE{}.tokio_write_encrypted_client(w, e).await,
            Self::CMSG_GROUP_UNINVITE(c) => c.tokio_write_encrypted_client(w, e).await,
            Self::CMSG_GROUP_UNINVITE_GUID(c) => c.tokio_write_encrypted_client(w, e).await,
            Self::CMSG_GROUP_SET_LEADER(c) => c.tokio_write_encrypted_client(w, e).await,
            Self::CMSG_LOOT_METHOD(c) => c.tokio_write_encrypted_client(w, e).await,
            Self::CMSG_GROUP_DISBAND => CMSG_GROUP_DISBAND{}.tokio_write_encrypted_client(w, e).await,
            Self::CMSG_GUILD_CREATE(c) => c.tokio_write_encrypted_client(w, e).await,
            Self::CMSG_GUILD_INVITE(c) => c.tokio_write_encrypted_client(w, e).await,
            Self::CMSG_GUILD_ACCEPT => CMSG_GUILD_ACCEPT{}.tokio_write_encrypted_client(w, e).await,
            Self::CMSG_GUILD_DECLINE => CMSG_GUILD_DECLINE{}.tokio_write_encrypted_client(w, e).await,
            Self::CMSG_GUILD_INFO => CMSG_GUILD_INFO{}.tokio_write_encrypted_client(w, e).await,
            Self::CMSG_GUILD_ROSTER => CMSG_GUILD_ROSTER{}.tokio_write_encrypted_client(w, e).await,
            Self::CMSG_GUILD_PROMOTE(c) => c.tokio_write_encrypted_client(w, e).await,
            Self::CMSG_GUILD_DEMOTE(c) => c.tokio_write_encrypted_client(w, e).await,
            Self::CMSG_GUILD_LEAVE => CMSG_GUILD_LEAVE{}.tokio_write_encrypted_client(w, e).await,
            Self::CMSG_GUILD_REMOVE(c) => c.tokio_write_encrypted_client(w, e).await,
            Self::CMSG_GUILD_DISBAND => CMSG_GUILD_DISBAND{}.tokio_write_encrypted_client(w, e).await,
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
            Self::MSG_MOVE_TELEPORT_ACK(c) => c.tokio_write_encrypted_client(w, e).await,
            Self::MSG_MOVE_FALL_LAND(c) => c.tokio_write_encrypted_client(w, e).await,
            Self::MSG_MOVE_START_SWIM(c) => c.tokio_write_encrypted_client(w, e).await,
            Self::MSG_MOVE_STOP_SWIM(c) => c.tokio_write_encrypted_client(w, e).await,
            Self::MSG_MOVE_SET_FACING(c) => c.tokio_write_encrypted_client(w, e).await,
            Self::MSG_MOVE_SET_PITCH(c) => c.tokio_write_encrypted_client(w, e).await,
            Self::CMSG_MOVE_SET_RAW_POSITION(c) => c.tokio_write_encrypted_client(w, e).await,
            Self::CMSG_FORCE_RUN_SPEED_CHANGE_ACK(c) => c.tokio_write_encrypted_client(w, e).await,
            Self::CMSG_FORCE_RUN_BACK_SPEED_CHANGE_ACK(c) => c.tokio_write_encrypted_client(w, e).await,
            Self::CMSG_FORCE_SWIM_SPEED_CHANGE_ACK(c) => c.tokio_write_encrypted_client(w, e).await,
            Self::CMSG_FORCE_MOVE_ROOT_ACK(c) => c.tokio_write_encrypted_client(w, e).await,
            Self::CMSG_FORCE_MOVE_UNROOT_ACK(c) => c.tokio_write_encrypted_client(w, e).await,
            Self::MSG_MOVE_HEARTBEAT(c) => c.tokio_write_encrypted_client(w, e).await,
            Self::CMSG_MOVE_KNOCK_BACK_ACK(c) => c.tokio_write_encrypted_client(w, e).await,
            Self::CMSG_MOVE_HOVER_ACK(c) => c.tokio_write_encrypted_client(w, e).await,
            Self::CMSG_NEXT_CINEMATIC_CAMERA => CMSG_NEXT_CINEMATIC_CAMERA{}.tokio_write_encrypted_client(w, e).await,
            Self::CMSG_COMPLETE_CINEMATIC => CMSG_COMPLETE_CINEMATIC{}.tokio_write_encrypted_client(w, e).await,
            Self::CMSG_TUTORIAL_FLAG(c) => c.tokio_write_encrypted_client(w, e).await,
            Self::CMSG_TUTORIAL_CLEAR => CMSG_TUTORIAL_CLEAR{}.tokio_write_encrypted_client(w, e).await,
            Self::CMSG_TUTORIAL_RESET => CMSG_TUTORIAL_RESET{}.tokio_write_encrypted_client(w, e).await,
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
            Self::CMSG_BEGIN_TRADE => CMSG_BEGIN_TRADE{}.tokio_write_encrypted_client(w, e).await,
            Self::CMSG_BUSY_TRADE => CMSG_BUSY_TRADE{}.tokio_write_encrypted_client(w, e).await,
            Self::CMSG_IGNORE_TRADE => CMSG_IGNORE_TRADE{}.tokio_write_encrypted_client(w, e).await,
            Self::CMSG_ACCEPT_TRADE(c) => c.tokio_write_encrypted_client(w, e).await,
            Self::CMSG_UNACCEPT_TRADE => CMSG_UNACCEPT_TRADE{}.tokio_write_encrypted_client(w, e).await,
            Self::CMSG_CANCEL_TRADE => CMSG_CANCEL_TRADE{}.tokio_write_encrypted_client(w, e).await,
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
            Self::CMSG_SET_TARGET_OBSOLETE(c) => c.tokio_write_encrypted_client(w, e).await,
            Self::CMSG_ATTACKSWING(c) => c.tokio_write_encrypted_client(w, e).await,
            Self::CMSG_ATTACKSTOP => CMSG_ATTACKSTOP{}.tokio_write_encrypted_client(w, e).await,
            Self::CMSG_REPOP_REQUEST => CMSG_REPOP_REQUEST{}.tokio_write_encrypted_client(w, e).await,
            Self::CMSG_RESURRECT_RESPONSE(c) => c.tokio_write_encrypted_client(w, e).await,
            Self::CMSG_LOOT(c) => c.tokio_write_encrypted_client(w, e).await,
            Self::CMSG_LOOT_MONEY => CMSG_LOOT_MONEY{}.tokio_write_encrypted_client(w, e).await,
            Self::CMSG_LOOT_RELEASE(c) => c.tokio_write_encrypted_client(w, e).await,
            Self::CMSG_DUEL_ACCEPTED(c) => c.tokio_write_encrypted_client(w, e).await,
            Self::CMSG_DUEL_CANCELLED(c) => c.tokio_write_encrypted_client(w, e).await,
            Self::CMSG_MOUNTSPECIAL_ANIM => CMSG_MOUNTSPECIAL_ANIM{}.tokio_write_encrypted_client(w, e).await,
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
            Self::CMSG_QUESTGIVER_QUEST_AUTOLAUNCH => CMSG_QUESTGIVER_QUEST_AUTOLAUNCH{}.tokio_write_encrypted_client(w, e).await,
            Self::CMSG_QUESTGIVER_ACCEPT_QUEST(c) => c.tokio_write_encrypted_client(w, e).await,
            Self::CMSG_QUESTGIVER_COMPLETE_QUEST(c) => c.tokio_write_encrypted_client(w, e).await,
            Self::CMSG_QUESTGIVER_REQUEST_REWARD(c) => c.tokio_write_encrypted_client(w, e).await,
            Self::CMSG_QUESTGIVER_CHOOSE_REWARD(c) => c.tokio_write_encrypted_client(w, e).await,
            Self::CMSG_QUESTGIVER_CANCEL => CMSG_QUESTGIVER_CANCEL{}.tokio_write_encrypted_client(w, e).await,
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
            Self::CMSG_PLAYED_TIME => CMSG_PLAYED_TIME{}.tokio_write_encrypted_client(w, e).await,
            Self::CMSG_QUERY_TIME => CMSG_QUERY_TIME{}.tokio_write_encrypted_client(w, e).await,
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
            Self::MSG_LOOKING_FOR_GROUP => MSG_LOOKING_FOR_GROUP_Client{}.tokio_write_encrypted_client(w, e).await,
            Self::CMSG_UNLEARN_SKILL(c) => c.tokio_write_encrypted_client(w, e).await,
            Self::CMSG_GMTICKET_CREATE(c) => c.tokio_write_encrypted_client(w, e).await,
            Self::CMSG_GMTICKET_UPDATETEXT(c) => c.tokio_write_encrypted_client(w, e).await,
            Self::CMSG_REQUEST_ACCOUNT_DATA(c) => c.tokio_write_encrypted_client(w, e).await,
            Self::CMSG_UPDATE_ACCOUNT_DATA(c) => c.tokio_write_encrypted_client(w, e).await,
            Self::CMSG_GMTICKET_GETTICKET => CMSG_GMTICKET_GETTICKET{}.tokio_write_encrypted_client(w, e).await,
            Self::MSG_CORPSE_QUERY => MSG_CORPSE_QUERY_Client{}.tokio_write_encrypted_client(w, e).await,
            Self::CMSG_GMTICKET_DELETETICKET => CMSG_GMTICKET_DELETETICKET{}.tokio_write_encrypted_client(w, e).await,
            Self::CMSG_GMTICKET_SYSTEMSTATUS => CMSG_GMTICKET_SYSTEMSTATUS{}.tokio_write_encrypted_client(w, e).await,
            Self::CMSG_SPIRIT_HEALER_ACTIVATE(c) => c.tokio_write_encrypted_client(w, e).await,
            Self::CMSG_CHAT_IGNORED(c) => c.tokio_write_encrypted_client(w, e).await,
            Self::CMSG_GUILD_RANK(c) => c.tokio_write_encrypted_client(w, e).await,
            Self::CMSG_GUILD_ADD_RANK(c) => c.tokio_write_encrypted_client(w, e).await,
            Self::CMSG_GUILD_DEL_RANK => CMSG_GUILD_DEL_RANK{}.tokio_write_encrypted_client(w, e).await,
            Self::CMSG_GUILD_SET_PUBLIC_NOTE(c) => c.tokio_write_encrypted_client(w, e).await,
            Self::CMSG_GUILD_SET_OFFICER_NOTE(c) => c.tokio_write_encrypted_client(w, e).await,
            Self::CMSG_SEND_MAIL(c) => c.tokio_write_encrypted_client(w, e).await,
            Self::CMSG_GET_MAIL_LIST(c) => c.tokio_write_encrypted_client(w, e).await,
            Self::CMSG_BATTLEFIELD_LIST(c) => c.tokio_write_encrypted_client(w, e).await,
            Self::CMSG_BATTLEFIELD_JOIN(c) => c.tokio_write_encrypted_client(w, e).await,
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
            Self::CMSG_CANCEL_AUTO_REPEAT_SPELL => CMSG_CANCEL_AUTO_REPEAT_SPELL{}.tokio_write_encrypted_client(w, e).await,
            Self::MSG_LIST_STABLED_PETS(c) => c.tokio_write_encrypted_client(w, e).await,
            Self::CMSG_STABLE_PET(c) => c.tokio_write_encrypted_client(w, e).await,
            Self::CMSG_UNSTABLE_PET(c) => c.tokio_write_encrypted_client(w, e).await,
            Self::CMSG_BUY_STABLE_SLOT(c) => c.tokio_write_encrypted_client(w, e).await,
            Self::CMSG_STABLE_SWAP_PET(c) => c.tokio_write_encrypted_client(w, e).await,
            Self::CMSG_REQUEST_PET_INFO => CMSG_REQUEST_PET_INFO{}.tokio_write_encrypted_client(w, e).await,
            Self::CMSG_FAR_SIGHT(c) => c.tokio_write_encrypted_client(w, e).await,
            Self::CMSG_GROUP_CHANGE_SUB_GROUP(c) => c.tokio_write_encrypted_client(w, e).await,
            Self::CMSG_REQUEST_PARTY_MEMBER_STATS(c) => c.tokio_write_encrypted_client(w, e).await,
            Self::CMSG_GROUP_SWAP_SUB_GROUP(c) => c.tokio_write_encrypted_client(w, e).await,
            Self::CMSG_AUTOSTORE_BANK_ITEM(c) => c.tokio_write_encrypted_client(w, e).await,
            Self::CMSG_AUTOBANK_ITEM(c) => c.tokio_write_encrypted_client(w, e).await,
            Self::MSG_QUERY_NEXT_MAIL_TIME => MSG_QUERY_NEXT_MAIL_TIME_Client{}.tokio_write_encrypted_client(w, e).await,
            Self::CMSG_GROUP_RAID_CONVERT => CMSG_GROUP_RAID_CONVERT{}.tokio_write_encrypted_client(w, e).await,
            Self::CMSG_GROUP_ASSISTANT_LEADER(c) => c.tokio_write_encrypted_client(w, e).await,
            Self::CMSG_BUYBACK_ITEM(c) => c.tokio_write_encrypted_client(w, e).await,
            Self::CMSG_MEETINGSTONE_JOIN(c) => c.tokio_write_encrypted_client(w, e).await,
            Self::CMSG_MEETINGSTONE_LEAVE => CMSG_MEETINGSTONE_LEAVE{}.tokio_write_encrypted_client(w, e).await,
            Self::CMSG_MEETINGSTONE_INFO => CMSG_MEETINGSTONE_INFO{}.tokio_write_encrypted_client(w, e).await,
            Self::CMSG_CANCEL_GROWTH_AURA => CMSG_CANCEL_GROWTH_AURA{}.tokio_write_encrypted_client(w, e).await,
            Self::CMSG_LOOT_ROLL(c) => c.tokio_write_encrypted_client(w, e).await,
            Self::CMSG_LOOT_MASTER_GIVE(c) => c.tokio_write_encrypted_client(w, e).await,
            Self::CMSG_REPAIR_ITEM(c) => c.tokio_write_encrypted_client(w, e).await,
            Self::MSG_TALENT_WIPE_CONFIRM(c) => c.tokio_write_encrypted_client(w, e).await,
            Self::CMSG_SUMMON_RESPONSE(c) => c.tokio_write_encrypted_client(w, e).await,
            Self::CMSG_SELF_RES => CMSG_SELF_RES{}.tokio_write_encrypted_client(w, e).await,
            Self::CMSG_TOGGLE_HELM => CMSG_TOGGLE_HELM{}.tokio_write_encrypted_client(w, e).await,
            Self::CMSG_TOGGLE_CLOAK => CMSG_TOGGLE_CLOAK{}.tokio_write_encrypted_client(w, e).await,
            Self::CMSG_SET_ACTIONBAR_TOGGLES(c) => c.tokio_write_encrypted_client(w, e).await,
            Self::CMSG_ITEM_NAME_QUERY(c) => c.tokio_write_encrypted_client(w, e).await,
            Self::CMSG_CHAR_RENAME(c) => c.tokio_write_encrypted_client(w, e).await,
            Self::CMSG_MOVE_SPLINE_DONE(c) => c.tokio_write_encrypted_client(w, e).await,
            Self::CMSG_MOVE_FALL_RESET(c) => c.tokio_write_encrypted_client(w, e).await,
            Self::CMSG_REQUEST_RAID_INFO => CMSG_REQUEST_RAID_INFO{}.tokio_write_encrypted_client(w, e).await,
            Self::CMSG_MOVE_TIME_SKIPPED(c) => c.tokio_write_encrypted_client(w, e).await,
            Self::CMSG_MOVE_FEATHER_FALL_ACK(c) => c.tokio_write_encrypted_client(w, e).await,
            Self::CMSG_MOVE_WATER_WALK_ACK(c) => c.tokio_write_encrypted_client(w, e).await,
            Self::CMSG_MOVE_NOT_ACTIVE_MOVER(c) => c.tokio_write_encrypted_client(w, e).await,
            Self::CMSG_BATTLEFIELD_STATUS => CMSG_BATTLEFIELD_STATUS{}.tokio_write_encrypted_client(w, e).await,
            Self::CMSG_BATTLEFIELD_PORT(c) => c.tokio_write_encrypted_client(w, e).await,
            Self::MSG_INSPECT_HONOR_STATS(c) => c.tokio_write_encrypted_client(w, e).await,
            Self::CMSG_BATTLEMASTER_HELLO(c) => c.tokio_write_encrypted_client(w, e).await,
            Self::CMSG_FORCE_WALK_SPEED_CHANGE_ACK(c) => c.tokio_write_encrypted_client(w, e).await,
            Self::CMSG_FORCE_SWIM_BACK_SPEED_CHANGE_ACK(c) => c.tokio_write_encrypted_client(w, e).await,
            Self::CMSG_FORCE_TURN_RATE_CHANGE_ACK(c) => c.tokio_write_encrypted_client(w, e).await,
            Self::MSG_PVP_LOG_DATA => MSG_PVP_LOG_DATA_Client{}.tokio_write_encrypted_client(w, e).await,
            Self::CMSG_LEAVE_BATTLEFIELD(c) => c.tokio_write_encrypted_client(w, e).await,
            Self::CMSG_AREA_SPIRIT_HEALER_QUERY(c) => c.tokio_write_encrypted_client(w, e).await,
            Self::CMSG_AREA_SPIRIT_HEALER_QUEUE(c) => c.tokio_write_encrypted_client(w, e).await,
            Self::CMSG_WARDEN_DATA(c) => c.tokio_write_encrypted_client(w, e).await,
            Self::MSG_BATTLEGROUND_PLAYER_POSITIONS => MSG_BATTLEGROUND_PLAYER_POSITIONS_Client{}.tokio_write_encrypted_client(w, e).await,
            Self::CMSG_PET_STOP_ATTACK(c) => c.tokio_write_encrypted_client(w, e).await,
            Self::CMSG_BATTLEMASTER_JOIN(c) => c.tokio_write_encrypted_client(w, e).await,
            Self::CMSG_PET_UNLEARN(c) => c.tokio_write_encrypted_client(w, e).await,
            Self::CMSG_PET_SPELL_AUTOCAST(c) => c.tokio_write_encrypted_client(w, e).await,
            Self::CMSG_GUILD_INFO_TEXT(c) => c.tokio_write_encrypted_client(w, e).await,
            Self::CMSG_ACTIVATETAXIEXPRESS(c) => c.tokio_write_encrypted_client(w, e).await,
            Self::CMSG_SET_FACTION_INACTIVE(c) => c.tokio_write_encrypted_client(w, e).await,
            Self::CMSG_SET_WATCHED_FACTION(c) => c.tokio_write_encrypted_client(w, e).await,
            Self::CMSG_RESET_INSTANCES => CMSG_RESET_INSTANCES{}.tokio_write_encrypted_client(w, e).await,
            Self::MSG_RAID_TARGET_UPDATE(c) => c.tokio_write_encrypted_client(w, e).await,
            Self::MSG_RAID_READY_CHECK(c) => c.tokio_write_encrypted_client(w, e).await,
            Self::CMSG_GMSURVEY_SUBMIT(c) => c.tokio_write_encrypted_client(w, e).await,
        }
    }

    #[cfg(feature = "tokio")]
    pub async fn tokio_write_unencrypted_client<W: tokio::io::AsyncWriteExt + Unpin + Send>(&self, mut w: W) -> Result<(), std::io::Error> {
        match self {
            Self::MSG_MOVE_WORLDPORT_ACK => MSG_MOVE_WORLDPORT_ACK{}.tokio_write_unencrypted_client(w).await,
            Self::MSG_PETITION_DECLINE(c) => c.tokio_write_unencrypted_client(w).await,
            Self::MSG_TABARDVENDOR_ACTIVATE(c) => c.tokio_write_unencrypted_client(w).await,
            Self::MSG_QUEST_PUSH_RESULT(c) => c.tokio_write_unencrypted_client(w).await,
            Self::MSG_MOVE_WATER_WALK(c) => c.tokio_write_unencrypted_client(w).await,
            Self::MSG_PETITION_RENAME(c) => c.tokio_write_unencrypted_client(w).await,
            Self::CMSG_BOOTME => CMSG_BOOTME{}.tokio_write_unencrypted_client(w).await,
            Self::CMSG_DBLOOKUP(c) => c.tokio_write_unencrypted_client(w).await,
            Self::CMSG_WORLD_TELEPORT(c) => c.tokio_write_unencrypted_client(w).await,
            Self::CMSG_TELEPORT_TO_UNIT(c) => c.tokio_write_unencrypted_client(w).await,
            Self::CMSG_CHAR_CREATE(c) => c.tokio_write_unencrypted_client(w).await,
            Self::CMSG_CHAR_ENUM => CMSG_CHAR_ENUM{}.tokio_write_unencrypted_client(w).await,
            Self::CMSG_CHAR_DELETE(c) => c.tokio_write_unencrypted_client(w).await,
            Self::CMSG_PLAYER_LOGIN(c) => c.tokio_write_unencrypted_client(w).await,
            Self::CMSG_PLAYER_LOGOUT => CMSG_PLAYER_LOGOUT{}.tokio_write_unencrypted_client(w).await,
            Self::CMSG_LOGOUT_REQUEST => CMSG_LOGOUT_REQUEST{}.tokio_write_unencrypted_client(w).await,
            Self::CMSG_LOGOUT_CANCEL => CMSG_LOGOUT_CANCEL{}.tokio_write_unencrypted_client(w).await,
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
            Self::CMSG_FRIEND_LIST => CMSG_FRIEND_LIST{}.tokio_write_unencrypted_client(w).await,
            Self::CMSG_ADD_FRIEND(c) => c.tokio_write_unencrypted_client(w).await,
            Self::CMSG_DEL_FRIEND(c) => c.tokio_write_unencrypted_client(w).await,
            Self::CMSG_ADD_IGNORE(c) => c.tokio_write_unencrypted_client(w).await,
            Self::CMSG_DEL_IGNORE(c) => c.tokio_write_unencrypted_client(w).await,
            Self::CMSG_GROUP_INVITE(c) => c.tokio_write_unencrypted_client(w).await,
            Self::CMSG_GROUP_ACCEPT => CMSG_GROUP_ACCEPT{}.tokio_write_unencrypted_client(w).await,
            Self::CMSG_GROUP_DECLINE => CMSG_GROUP_DECLINE{}.tokio_write_unencrypted_client(w).await,
            Self::CMSG_GROUP_UNINVITE(c) => c.tokio_write_unencrypted_client(w).await,
            Self::CMSG_GROUP_UNINVITE_GUID(c) => c.tokio_write_unencrypted_client(w).await,
            Self::CMSG_GROUP_SET_LEADER(c) => c.tokio_write_unencrypted_client(w).await,
            Self::CMSG_LOOT_METHOD(c) => c.tokio_write_unencrypted_client(w).await,
            Self::CMSG_GROUP_DISBAND => CMSG_GROUP_DISBAND{}.tokio_write_unencrypted_client(w).await,
            Self::CMSG_GUILD_CREATE(c) => c.tokio_write_unencrypted_client(w).await,
            Self::CMSG_GUILD_INVITE(c) => c.tokio_write_unencrypted_client(w).await,
            Self::CMSG_GUILD_ACCEPT => CMSG_GUILD_ACCEPT{}.tokio_write_unencrypted_client(w).await,
            Self::CMSG_GUILD_DECLINE => CMSG_GUILD_DECLINE{}.tokio_write_unencrypted_client(w).await,
            Self::CMSG_GUILD_INFO => CMSG_GUILD_INFO{}.tokio_write_unencrypted_client(w).await,
            Self::CMSG_GUILD_ROSTER => CMSG_GUILD_ROSTER{}.tokio_write_unencrypted_client(w).await,
            Self::CMSG_GUILD_PROMOTE(c) => c.tokio_write_unencrypted_client(w).await,
            Self::CMSG_GUILD_DEMOTE(c) => c.tokio_write_unencrypted_client(w).await,
            Self::CMSG_GUILD_LEAVE => CMSG_GUILD_LEAVE{}.tokio_write_unencrypted_client(w).await,
            Self::CMSG_GUILD_REMOVE(c) => c.tokio_write_unencrypted_client(w).await,
            Self::CMSG_GUILD_DISBAND => CMSG_GUILD_DISBAND{}.tokio_write_unencrypted_client(w).await,
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
            Self::MSG_MOVE_TELEPORT_ACK(c) => c.tokio_write_unencrypted_client(w).await,
            Self::MSG_MOVE_FALL_LAND(c) => c.tokio_write_unencrypted_client(w).await,
            Self::MSG_MOVE_START_SWIM(c) => c.tokio_write_unencrypted_client(w).await,
            Self::MSG_MOVE_STOP_SWIM(c) => c.tokio_write_unencrypted_client(w).await,
            Self::MSG_MOVE_SET_FACING(c) => c.tokio_write_unencrypted_client(w).await,
            Self::MSG_MOVE_SET_PITCH(c) => c.tokio_write_unencrypted_client(w).await,
            Self::CMSG_MOVE_SET_RAW_POSITION(c) => c.tokio_write_unencrypted_client(w).await,
            Self::CMSG_FORCE_RUN_SPEED_CHANGE_ACK(c) => c.tokio_write_unencrypted_client(w).await,
            Self::CMSG_FORCE_RUN_BACK_SPEED_CHANGE_ACK(c) => c.tokio_write_unencrypted_client(w).await,
            Self::CMSG_FORCE_SWIM_SPEED_CHANGE_ACK(c) => c.tokio_write_unencrypted_client(w).await,
            Self::CMSG_FORCE_MOVE_ROOT_ACK(c) => c.tokio_write_unencrypted_client(w).await,
            Self::CMSG_FORCE_MOVE_UNROOT_ACK(c) => c.tokio_write_unencrypted_client(w).await,
            Self::MSG_MOVE_HEARTBEAT(c) => c.tokio_write_unencrypted_client(w).await,
            Self::CMSG_MOVE_KNOCK_BACK_ACK(c) => c.tokio_write_unencrypted_client(w).await,
            Self::CMSG_MOVE_HOVER_ACK(c) => c.tokio_write_unencrypted_client(w).await,
            Self::CMSG_NEXT_CINEMATIC_CAMERA => CMSG_NEXT_CINEMATIC_CAMERA{}.tokio_write_unencrypted_client(w).await,
            Self::CMSG_COMPLETE_CINEMATIC => CMSG_COMPLETE_CINEMATIC{}.tokio_write_unencrypted_client(w).await,
            Self::CMSG_TUTORIAL_FLAG(c) => c.tokio_write_unencrypted_client(w).await,
            Self::CMSG_TUTORIAL_CLEAR => CMSG_TUTORIAL_CLEAR{}.tokio_write_unencrypted_client(w).await,
            Self::CMSG_TUTORIAL_RESET => CMSG_TUTORIAL_RESET{}.tokio_write_unencrypted_client(w).await,
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
            Self::CMSG_BEGIN_TRADE => CMSG_BEGIN_TRADE{}.tokio_write_unencrypted_client(w).await,
            Self::CMSG_BUSY_TRADE => CMSG_BUSY_TRADE{}.tokio_write_unencrypted_client(w).await,
            Self::CMSG_IGNORE_TRADE => CMSG_IGNORE_TRADE{}.tokio_write_unencrypted_client(w).await,
            Self::CMSG_ACCEPT_TRADE(c) => c.tokio_write_unencrypted_client(w).await,
            Self::CMSG_UNACCEPT_TRADE => CMSG_UNACCEPT_TRADE{}.tokio_write_unencrypted_client(w).await,
            Self::CMSG_CANCEL_TRADE => CMSG_CANCEL_TRADE{}.tokio_write_unencrypted_client(w).await,
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
            Self::CMSG_SET_TARGET_OBSOLETE(c) => c.tokio_write_unencrypted_client(w).await,
            Self::CMSG_ATTACKSWING(c) => c.tokio_write_unencrypted_client(w).await,
            Self::CMSG_ATTACKSTOP => CMSG_ATTACKSTOP{}.tokio_write_unencrypted_client(w).await,
            Self::CMSG_REPOP_REQUEST => CMSG_REPOP_REQUEST{}.tokio_write_unencrypted_client(w).await,
            Self::CMSG_RESURRECT_RESPONSE(c) => c.tokio_write_unencrypted_client(w).await,
            Self::CMSG_LOOT(c) => c.tokio_write_unencrypted_client(w).await,
            Self::CMSG_LOOT_MONEY => CMSG_LOOT_MONEY{}.tokio_write_unencrypted_client(w).await,
            Self::CMSG_LOOT_RELEASE(c) => c.tokio_write_unencrypted_client(w).await,
            Self::CMSG_DUEL_ACCEPTED(c) => c.tokio_write_unencrypted_client(w).await,
            Self::CMSG_DUEL_CANCELLED(c) => c.tokio_write_unencrypted_client(w).await,
            Self::CMSG_MOUNTSPECIAL_ANIM => CMSG_MOUNTSPECIAL_ANIM{}.tokio_write_unencrypted_client(w).await,
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
            Self::CMSG_QUESTGIVER_QUEST_AUTOLAUNCH => CMSG_QUESTGIVER_QUEST_AUTOLAUNCH{}.tokio_write_unencrypted_client(w).await,
            Self::CMSG_QUESTGIVER_ACCEPT_QUEST(c) => c.tokio_write_unencrypted_client(w).await,
            Self::CMSG_QUESTGIVER_COMPLETE_QUEST(c) => c.tokio_write_unencrypted_client(w).await,
            Self::CMSG_QUESTGIVER_REQUEST_REWARD(c) => c.tokio_write_unencrypted_client(w).await,
            Self::CMSG_QUESTGIVER_CHOOSE_REWARD(c) => c.tokio_write_unencrypted_client(w).await,
            Self::CMSG_QUESTGIVER_CANCEL => CMSG_QUESTGIVER_CANCEL{}.tokio_write_unencrypted_client(w).await,
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
            Self::CMSG_PLAYED_TIME => CMSG_PLAYED_TIME{}.tokio_write_unencrypted_client(w).await,
            Self::CMSG_QUERY_TIME => CMSG_QUERY_TIME{}.tokio_write_unencrypted_client(w).await,
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
            Self::MSG_LOOKING_FOR_GROUP => MSG_LOOKING_FOR_GROUP_Client{}.tokio_write_unencrypted_client(w).await,
            Self::CMSG_UNLEARN_SKILL(c) => c.tokio_write_unencrypted_client(w).await,
            Self::CMSG_GMTICKET_CREATE(c) => c.tokio_write_unencrypted_client(w).await,
            Self::CMSG_GMTICKET_UPDATETEXT(c) => c.tokio_write_unencrypted_client(w).await,
            Self::CMSG_REQUEST_ACCOUNT_DATA(c) => c.tokio_write_unencrypted_client(w).await,
            Self::CMSG_UPDATE_ACCOUNT_DATA(c) => c.tokio_write_unencrypted_client(w).await,
            Self::CMSG_GMTICKET_GETTICKET => CMSG_GMTICKET_GETTICKET{}.tokio_write_unencrypted_client(w).await,
            Self::MSG_CORPSE_QUERY => MSG_CORPSE_QUERY_Client{}.tokio_write_unencrypted_client(w).await,
            Self::CMSG_GMTICKET_DELETETICKET => CMSG_GMTICKET_DELETETICKET{}.tokio_write_unencrypted_client(w).await,
            Self::CMSG_GMTICKET_SYSTEMSTATUS => CMSG_GMTICKET_SYSTEMSTATUS{}.tokio_write_unencrypted_client(w).await,
            Self::CMSG_SPIRIT_HEALER_ACTIVATE(c) => c.tokio_write_unencrypted_client(w).await,
            Self::CMSG_CHAT_IGNORED(c) => c.tokio_write_unencrypted_client(w).await,
            Self::CMSG_GUILD_RANK(c) => c.tokio_write_unencrypted_client(w).await,
            Self::CMSG_GUILD_ADD_RANK(c) => c.tokio_write_unencrypted_client(w).await,
            Self::CMSG_GUILD_DEL_RANK => CMSG_GUILD_DEL_RANK{}.tokio_write_unencrypted_client(w).await,
            Self::CMSG_GUILD_SET_PUBLIC_NOTE(c) => c.tokio_write_unencrypted_client(w).await,
            Self::CMSG_GUILD_SET_OFFICER_NOTE(c) => c.tokio_write_unencrypted_client(w).await,
            Self::CMSG_SEND_MAIL(c) => c.tokio_write_unencrypted_client(w).await,
            Self::CMSG_GET_MAIL_LIST(c) => c.tokio_write_unencrypted_client(w).await,
            Self::CMSG_BATTLEFIELD_LIST(c) => c.tokio_write_unencrypted_client(w).await,
            Self::CMSG_BATTLEFIELD_JOIN(c) => c.tokio_write_unencrypted_client(w).await,
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
            Self::CMSG_CANCEL_AUTO_REPEAT_SPELL => CMSG_CANCEL_AUTO_REPEAT_SPELL{}.tokio_write_unencrypted_client(w).await,
            Self::MSG_LIST_STABLED_PETS(c) => c.tokio_write_unencrypted_client(w).await,
            Self::CMSG_STABLE_PET(c) => c.tokio_write_unencrypted_client(w).await,
            Self::CMSG_UNSTABLE_PET(c) => c.tokio_write_unencrypted_client(w).await,
            Self::CMSG_BUY_STABLE_SLOT(c) => c.tokio_write_unencrypted_client(w).await,
            Self::CMSG_STABLE_SWAP_PET(c) => c.tokio_write_unencrypted_client(w).await,
            Self::CMSG_REQUEST_PET_INFO => CMSG_REQUEST_PET_INFO{}.tokio_write_unencrypted_client(w).await,
            Self::CMSG_FAR_SIGHT(c) => c.tokio_write_unencrypted_client(w).await,
            Self::CMSG_GROUP_CHANGE_SUB_GROUP(c) => c.tokio_write_unencrypted_client(w).await,
            Self::CMSG_REQUEST_PARTY_MEMBER_STATS(c) => c.tokio_write_unencrypted_client(w).await,
            Self::CMSG_GROUP_SWAP_SUB_GROUP(c) => c.tokio_write_unencrypted_client(w).await,
            Self::CMSG_AUTOSTORE_BANK_ITEM(c) => c.tokio_write_unencrypted_client(w).await,
            Self::CMSG_AUTOBANK_ITEM(c) => c.tokio_write_unencrypted_client(w).await,
            Self::MSG_QUERY_NEXT_MAIL_TIME => MSG_QUERY_NEXT_MAIL_TIME_Client{}.tokio_write_unencrypted_client(w).await,
            Self::CMSG_GROUP_RAID_CONVERT => CMSG_GROUP_RAID_CONVERT{}.tokio_write_unencrypted_client(w).await,
            Self::CMSG_GROUP_ASSISTANT_LEADER(c) => c.tokio_write_unencrypted_client(w).await,
            Self::CMSG_BUYBACK_ITEM(c) => c.tokio_write_unencrypted_client(w).await,
            Self::CMSG_MEETINGSTONE_JOIN(c) => c.tokio_write_unencrypted_client(w).await,
            Self::CMSG_MEETINGSTONE_LEAVE => CMSG_MEETINGSTONE_LEAVE{}.tokio_write_unencrypted_client(w).await,
            Self::CMSG_MEETINGSTONE_INFO => CMSG_MEETINGSTONE_INFO{}.tokio_write_unencrypted_client(w).await,
            Self::CMSG_CANCEL_GROWTH_AURA => CMSG_CANCEL_GROWTH_AURA{}.tokio_write_unencrypted_client(w).await,
            Self::CMSG_LOOT_ROLL(c) => c.tokio_write_unencrypted_client(w).await,
            Self::CMSG_LOOT_MASTER_GIVE(c) => c.tokio_write_unencrypted_client(w).await,
            Self::CMSG_REPAIR_ITEM(c) => c.tokio_write_unencrypted_client(w).await,
            Self::MSG_TALENT_WIPE_CONFIRM(c) => c.tokio_write_unencrypted_client(w).await,
            Self::CMSG_SUMMON_RESPONSE(c) => c.tokio_write_unencrypted_client(w).await,
            Self::CMSG_SELF_RES => CMSG_SELF_RES{}.tokio_write_unencrypted_client(w).await,
            Self::CMSG_TOGGLE_HELM => CMSG_TOGGLE_HELM{}.tokio_write_unencrypted_client(w).await,
            Self::CMSG_TOGGLE_CLOAK => CMSG_TOGGLE_CLOAK{}.tokio_write_unencrypted_client(w).await,
            Self::CMSG_SET_ACTIONBAR_TOGGLES(c) => c.tokio_write_unencrypted_client(w).await,
            Self::CMSG_ITEM_NAME_QUERY(c) => c.tokio_write_unencrypted_client(w).await,
            Self::CMSG_CHAR_RENAME(c) => c.tokio_write_unencrypted_client(w).await,
            Self::CMSG_MOVE_SPLINE_DONE(c) => c.tokio_write_unencrypted_client(w).await,
            Self::CMSG_MOVE_FALL_RESET(c) => c.tokio_write_unencrypted_client(w).await,
            Self::CMSG_REQUEST_RAID_INFO => CMSG_REQUEST_RAID_INFO{}.tokio_write_unencrypted_client(w).await,
            Self::CMSG_MOVE_TIME_SKIPPED(c) => c.tokio_write_unencrypted_client(w).await,
            Self::CMSG_MOVE_FEATHER_FALL_ACK(c) => c.tokio_write_unencrypted_client(w).await,
            Self::CMSG_MOVE_WATER_WALK_ACK(c) => c.tokio_write_unencrypted_client(w).await,
            Self::CMSG_MOVE_NOT_ACTIVE_MOVER(c) => c.tokio_write_unencrypted_client(w).await,
            Self::CMSG_BATTLEFIELD_STATUS => CMSG_BATTLEFIELD_STATUS{}.tokio_write_unencrypted_client(w).await,
            Self::CMSG_BATTLEFIELD_PORT(c) => c.tokio_write_unencrypted_client(w).await,
            Self::MSG_INSPECT_HONOR_STATS(c) => c.tokio_write_unencrypted_client(w).await,
            Self::CMSG_BATTLEMASTER_HELLO(c) => c.tokio_write_unencrypted_client(w).await,
            Self::CMSG_FORCE_WALK_SPEED_CHANGE_ACK(c) => c.tokio_write_unencrypted_client(w).await,
            Self::CMSG_FORCE_SWIM_BACK_SPEED_CHANGE_ACK(c) => c.tokio_write_unencrypted_client(w).await,
            Self::CMSG_FORCE_TURN_RATE_CHANGE_ACK(c) => c.tokio_write_unencrypted_client(w).await,
            Self::MSG_PVP_LOG_DATA => MSG_PVP_LOG_DATA_Client{}.tokio_write_unencrypted_client(w).await,
            Self::CMSG_LEAVE_BATTLEFIELD(c) => c.tokio_write_unencrypted_client(w).await,
            Self::CMSG_AREA_SPIRIT_HEALER_QUERY(c) => c.tokio_write_unencrypted_client(w).await,
            Self::CMSG_AREA_SPIRIT_HEALER_QUEUE(c) => c.tokio_write_unencrypted_client(w).await,
            Self::CMSG_WARDEN_DATA(c) => c.tokio_write_unencrypted_client(w).await,
            Self::MSG_BATTLEGROUND_PLAYER_POSITIONS => MSG_BATTLEGROUND_PLAYER_POSITIONS_Client{}.tokio_write_unencrypted_client(w).await,
            Self::CMSG_PET_STOP_ATTACK(c) => c.tokio_write_unencrypted_client(w).await,
            Self::CMSG_BATTLEMASTER_JOIN(c) => c.tokio_write_unencrypted_client(w).await,
            Self::CMSG_PET_UNLEARN(c) => c.tokio_write_unencrypted_client(w).await,
            Self::CMSG_PET_SPELL_AUTOCAST(c) => c.tokio_write_unencrypted_client(w).await,
            Self::CMSG_GUILD_INFO_TEXT(c) => c.tokio_write_unencrypted_client(w).await,
            Self::CMSG_ACTIVATETAXIEXPRESS(c) => c.tokio_write_unencrypted_client(w).await,
            Self::CMSG_SET_FACTION_INACTIVE(c) => c.tokio_write_unencrypted_client(w).await,
            Self::CMSG_SET_WATCHED_FACTION(c) => c.tokio_write_unencrypted_client(w).await,
            Self::CMSG_RESET_INSTANCES => CMSG_RESET_INSTANCES{}.tokio_write_unencrypted_client(w).await,
            Self::MSG_RAID_TARGET_UPDATE(c) => c.tokio_write_unencrypted_client(w).await,
            Self::MSG_RAID_READY_CHECK(c) => c.tokio_write_unencrypted_client(w).await,
            Self::CMSG_GMSURVEY_SUBMIT(c) => c.tokio_write_unencrypted_client(w).await,
        }
    }

    #[cfg(all(feature = "async-std", feature = "encryption"))]
    pub async fn astd_write_encrypted_client<W: async_std::io::WriteExt + Unpin + Send>(&self, mut w: W, e: &mut EncrypterHalf) -> Result<(), std::io::Error> {
        match self {
            Self::MSG_MOVE_WORLDPORT_ACK => MSG_MOVE_WORLDPORT_ACK{}.astd_write_encrypted_client(w, e).await,
            Self::MSG_PETITION_DECLINE(c) => c.astd_write_encrypted_client(w, e).await,
            Self::MSG_TABARDVENDOR_ACTIVATE(c) => c.astd_write_encrypted_client(w, e).await,
            Self::MSG_QUEST_PUSH_RESULT(c) => c.astd_write_encrypted_client(w, e).await,
            Self::MSG_MOVE_WATER_WALK(c) => c.astd_write_encrypted_client(w, e).await,
            Self::MSG_PETITION_RENAME(c) => c.astd_write_encrypted_client(w, e).await,
            Self::CMSG_BOOTME => CMSG_BOOTME{}.astd_write_encrypted_client(w, e).await,
            Self::CMSG_DBLOOKUP(c) => c.astd_write_encrypted_client(w, e).await,
            Self::CMSG_WORLD_TELEPORT(c) => c.astd_write_encrypted_client(w, e).await,
            Self::CMSG_TELEPORT_TO_UNIT(c) => c.astd_write_encrypted_client(w, e).await,
            Self::CMSG_CHAR_CREATE(c) => c.astd_write_encrypted_client(w, e).await,
            Self::CMSG_CHAR_ENUM => CMSG_CHAR_ENUM{}.astd_write_encrypted_client(w, e).await,
            Self::CMSG_CHAR_DELETE(c) => c.astd_write_encrypted_client(w, e).await,
            Self::CMSG_PLAYER_LOGIN(c) => c.astd_write_encrypted_client(w, e).await,
            Self::CMSG_PLAYER_LOGOUT => CMSG_PLAYER_LOGOUT{}.astd_write_encrypted_client(w, e).await,
            Self::CMSG_LOGOUT_REQUEST => CMSG_LOGOUT_REQUEST{}.astd_write_encrypted_client(w, e).await,
            Self::CMSG_LOGOUT_CANCEL => CMSG_LOGOUT_CANCEL{}.astd_write_encrypted_client(w, e).await,
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
            Self::CMSG_FRIEND_LIST => CMSG_FRIEND_LIST{}.astd_write_encrypted_client(w, e).await,
            Self::CMSG_ADD_FRIEND(c) => c.astd_write_encrypted_client(w, e).await,
            Self::CMSG_DEL_FRIEND(c) => c.astd_write_encrypted_client(w, e).await,
            Self::CMSG_ADD_IGNORE(c) => c.astd_write_encrypted_client(w, e).await,
            Self::CMSG_DEL_IGNORE(c) => c.astd_write_encrypted_client(w, e).await,
            Self::CMSG_GROUP_INVITE(c) => c.astd_write_encrypted_client(w, e).await,
            Self::CMSG_GROUP_ACCEPT => CMSG_GROUP_ACCEPT{}.astd_write_encrypted_client(w, e).await,
            Self::CMSG_GROUP_DECLINE => CMSG_GROUP_DECLINE{}.astd_write_encrypted_client(w, e).await,
            Self::CMSG_GROUP_UNINVITE(c) => c.astd_write_encrypted_client(w, e).await,
            Self::CMSG_GROUP_UNINVITE_GUID(c) => c.astd_write_encrypted_client(w, e).await,
            Self::CMSG_GROUP_SET_LEADER(c) => c.astd_write_encrypted_client(w, e).await,
            Self::CMSG_LOOT_METHOD(c) => c.astd_write_encrypted_client(w, e).await,
            Self::CMSG_GROUP_DISBAND => CMSG_GROUP_DISBAND{}.astd_write_encrypted_client(w, e).await,
            Self::CMSG_GUILD_CREATE(c) => c.astd_write_encrypted_client(w, e).await,
            Self::CMSG_GUILD_INVITE(c) => c.astd_write_encrypted_client(w, e).await,
            Self::CMSG_GUILD_ACCEPT => CMSG_GUILD_ACCEPT{}.astd_write_encrypted_client(w, e).await,
            Self::CMSG_GUILD_DECLINE => CMSG_GUILD_DECLINE{}.astd_write_encrypted_client(w, e).await,
            Self::CMSG_GUILD_INFO => CMSG_GUILD_INFO{}.astd_write_encrypted_client(w, e).await,
            Self::CMSG_GUILD_ROSTER => CMSG_GUILD_ROSTER{}.astd_write_encrypted_client(w, e).await,
            Self::CMSG_GUILD_PROMOTE(c) => c.astd_write_encrypted_client(w, e).await,
            Self::CMSG_GUILD_DEMOTE(c) => c.astd_write_encrypted_client(w, e).await,
            Self::CMSG_GUILD_LEAVE => CMSG_GUILD_LEAVE{}.astd_write_encrypted_client(w, e).await,
            Self::CMSG_GUILD_REMOVE(c) => c.astd_write_encrypted_client(w, e).await,
            Self::CMSG_GUILD_DISBAND => CMSG_GUILD_DISBAND{}.astd_write_encrypted_client(w, e).await,
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
            Self::MSG_MOVE_TELEPORT_ACK(c) => c.astd_write_encrypted_client(w, e).await,
            Self::MSG_MOVE_FALL_LAND(c) => c.astd_write_encrypted_client(w, e).await,
            Self::MSG_MOVE_START_SWIM(c) => c.astd_write_encrypted_client(w, e).await,
            Self::MSG_MOVE_STOP_SWIM(c) => c.astd_write_encrypted_client(w, e).await,
            Self::MSG_MOVE_SET_FACING(c) => c.astd_write_encrypted_client(w, e).await,
            Self::MSG_MOVE_SET_PITCH(c) => c.astd_write_encrypted_client(w, e).await,
            Self::CMSG_MOVE_SET_RAW_POSITION(c) => c.astd_write_encrypted_client(w, e).await,
            Self::CMSG_FORCE_RUN_SPEED_CHANGE_ACK(c) => c.astd_write_encrypted_client(w, e).await,
            Self::CMSG_FORCE_RUN_BACK_SPEED_CHANGE_ACK(c) => c.astd_write_encrypted_client(w, e).await,
            Self::CMSG_FORCE_SWIM_SPEED_CHANGE_ACK(c) => c.astd_write_encrypted_client(w, e).await,
            Self::CMSG_FORCE_MOVE_ROOT_ACK(c) => c.astd_write_encrypted_client(w, e).await,
            Self::CMSG_FORCE_MOVE_UNROOT_ACK(c) => c.astd_write_encrypted_client(w, e).await,
            Self::MSG_MOVE_HEARTBEAT(c) => c.astd_write_encrypted_client(w, e).await,
            Self::CMSG_MOVE_KNOCK_BACK_ACK(c) => c.astd_write_encrypted_client(w, e).await,
            Self::CMSG_MOVE_HOVER_ACK(c) => c.astd_write_encrypted_client(w, e).await,
            Self::CMSG_NEXT_CINEMATIC_CAMERA => CMSG_NEXT_CINEMATIC_CAMERA{}.astd_write_encrypted_client(w, e).await,
            Self::CMSG_COMPLETE_CINEMATIC => CMSG_COMPLETE_CINEMATIC{}.astd_write_encrypted_client(w, e).await,
            Self::CMSG_TUTORIAL_FLAG(c) => c.astd_write_encrypted_client(w, e).await,
            Self::CMSG_TUTORIAL_CLEAR => CMSG_TUTORIAL_CLEAR{}.astd_write_encrypted_client(w, e).await,
            Self::CMSG_TUTORIAL_RESET => CMSG_TUTORIAL_RESET{}.astd_write_encrypted_client(w, e).await,
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
            Self::CMSG_BEGIN_TRADE => CMSG_BEGIN_TRADE{}.astd_write_encrypted_client(w, e).await,
            Self::CMSG_BUSY_TRADE => CMSG_BUSY_TRADE{}.astd_write_encrypted_client(w, e).await,
            Self::CMSG_IGNORE_TRADE => CMSG_IGNORE_TRADE{}.astd_write_encrypted_client(w, e).await,
            Self::CMSG_ACCEPT_TRADE(c) => c.astd_write_encrypted_client(w, e).await,
            Self::CMSG_UNACCEPT_TRADE => CMSG_UNACCEPT_TRADE{}.astd_write_encrypted_client(w, e).await,
            Self::CMSG_CANCEL_TRADE => CMSG_CANCEL_TRADE{}.astd_write_encrypted_client(w, e).await,
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
            Self::CMSG_SET_TARGET_OBSOLETE(c) => c.astd_write_encrypted_client(w, e).await,
            Self::CMSG_ATTACKSWING(c) => c.astd_write_encrypted_client(w, e).await,
            Self::CMSG_ATTACKSTOP => CMSG_ATTACKSTOP{}.astd_write_encrypted_client(w, e).await,
            Self::CMSG_REPOP_REQUEST => CMSG_REPOP_REQUEST{}.astd_write_encrypted_client(w, e).await,
            Self::CMSG_RESURRECT_RESPONSE(c) => c.astd_write_encrypted_client(w, e).await,
            Self::CMSG_LOOT(c) => c.astd_write_encrypted_client(w, e).await,
            Self::CMSG_LOOT_MONEY => CMSG_LOOT_MONEY{}.astd_write_encrypted_client(w, e).await,
            Self::CMSG_LOOT_RELEASE(c) => c.astd_write_encrypted_client(w, e).await,
            Self::CMSG_DUEL_ACCEPTED(c) => c.astd_write_encrypted_client(w, e).await,
            Self::CMSG_DUEL_CANCELLED(c) => c.astd_write_encrypted_client(w, e).await,
            Self::CMSG_MOUNTSPECIAL_ANIM => CMSG_MOUNTSPECIAL_ANIM{}.astd_write_encrypted_client(w, e).await,
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
            Self::CMSG_QUESTGIVER_QUEST_AUTOLAUNCH => CMSG_QUESTGIVER_QUEST_AUTOLAUNCH{}.astd_write_encrypted_client(w, e).await,
            Self::CMSG_QUESTGIVER_ACCEPT_QUEST(c) => c.astd_write_encrypted_client(w, e).await,
            Self::CMSG_QUESTGIVER_COMPLETE_QUEST(c) => c.astd_write_encrypted_client(w, e).await,
            Self::CMSG_QUESTGIVER_REQUEST_REWARD(c) => c.astd_write_encrypted_client(w, e).await,
            Self::CMSG_QUESTGIVER_CHOOSE_REWARD(c) => c.astd_write_encrypted_client(w, e).await,
            Self::CMSG_QUESTGIVER_CANCEL => CMSG_QUESTGIVER_CANCEL{}.astd_write_encrypted_client(w, e).await,
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
            Self::CMSG_PLAYED_TIME => CMSG_PLAYED_TIME{}.astd_write_encrypted_client(w, e).await,
            Self::CMSG_QUERY_TIME => CMSG_QUERY_TIME{}.astd_write_encrypted_client(w, e).await,
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
            Self::MSG_LOOKING_FOR_GROUP => MSG_LOOKING_FOR_GROUP_Client{}.astd_write_encrypted_client(w, e).await,
            Self::CMSG_UNLEARN_SKILL(c) => c.astd_write_encrypted_client(w, e).await,
            Self::CMSG_GMTICKET_CREATE(c) => c.astd_write_encrypted_client(w, e).await,
            Self::CMSG_GMTICKET_UPDATETEXT(c) => c.astd_write_encrypted_client(w, e).await,
            Self::CMSG_REQUEST_ACCOUNT_DATA(c) => c.astd_write_encrypted_client(w, e).await,
            Self::CMSG_UPDATE_ACCOUNT_DATA(c) => c.astd_write_encrypted_client(w, e).await,
            Self::CMSG_GMTICKET_GETTICKET => CMSG_GMTICKET_GETTICKET{}.astd_write_encrypted_client(w, e).await,
            Self::MSG_CORPSE_QUERY => MSG_CORPSE_QUERY_Client{}.astd_write_encrypted_client(w, e).await,
            Self::CMSG_GMTICKET_DELETETICKET => CMSG_GMTICKET_DELETETICKET{}.astd_write_encrypted_client(w, e).await,
            Self::CMSG_GMTICKET_SYSTEMSTATUS => CMSG_GMTICKET_SYSTEMSTATUS{}.astd_write_encrypted_client(w, e).await,
            Self::CMSG_SPIRIT_HEALER_ACTIVATE(c) => c.astd_write_encrypted_client(w, e).await,
            Self::CMSG_CHAT_IGNORED(c) => c.astd_write_encrypted_client(w, e).await,
            Self::CMSG_GUILD_RANK(c) => c.astd_write_encrypted_client(w, e).await,
            Self::CMSG_GUILD_ADD_RANK(c) => c.astd_write_encrypted_client(w, e).await,
            Self::CMSG_GUILD_DEL_RANK => CMSG_GUILD_DEL_RANK{}.astd_write_encrypted_client(w, e).await,
            Self::CMSG_GUILD_SET_PUBLIC_NOTE(c) => c.astd_write_encrypted_client(w, e).await,
            Self::CMSG_GUILD_SET_OFFICER_NOTE(c) => c.astd_write_encrypted_client(w, e).await,
            Self::CMSG_SEND_MAIL(c) => c.astd_write_encrypted_client(w, e).await,
            Self::CMSG_GET_MAIL_LIST(c) => c.astd_write_encrypted_client(w, e).await,
            Self::CMSG_BATTLEFIELD_LIST(c) => c.astd_write_encrypted_client(w, e).await,
            Self::CMSG_BATTLEFIELD_JOIN(c) => c.astd_write_encrypted_client(w, e).await,
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
            Self::CMSG_CANCEL_AUTO_REPEAT_SPELL => CMSG_CANCEL_AUTO_REPEAT_SPELL{}.astd_write_encrypted_client(w, e).await,
            Self::MSG_LIST_STABLED_PETS(c) => c.astd_write_encrypted_client(w, e).await,
            Self::CMSG_STABLE_PET(c) => c.astd_write_encrypted_client(w, e).await,
            Self::CMSG_UNSTABLE_PET(c) => c.astd_write_encrypted_client(w, e).await,
            Self::CMSG_BUY_STABLE_SLOT(c) => c.astd_write_encrypted_client(w, e).await,
            Self::CMSG_STABLE_SWAP_PET(c) => c.astd_write_encrypted_client(w, e).await,
            Self::CMSG_REQUEST_PET_INFO => CMSG_REQUEST_PET_INFO{}.astd_write_encrypted_client(w, e).await,
            Self::CMSG_FAR_SIGHT(c) => c.astd_write_encrypted_client(w, e).await,
            Self::CMSG_GROUP_CHANGE_SUB_GROUP(c) => c.astd_write_encrypted_client(w, e).await,
            Self::CMSG_REQUEST_PARTY_MEMBER_STATS(c) => c.astd_write_encrypted_client(w, e).await,
            Self::CMSG_GROUP_SWAP_SUB_GROUP(c) => c.astd_write_encrypted_client(w, e).await,
            Self::CMSG_AUTOSTORE_BANK_ITEM(c) => c.astd_write_encrypted_client(w, e).await,
            Self::CMSG_AUTOBANK_ITEM(c) => c.astd_write_encrypted_client(w, e).await,
            Self::MSG_QUERY_NEXT_MAIL_TIME => MSG_QUERY_NEXT_MAIL_TIME_Client{}.astd_write_encrypted_client(w, e).await,
            Self::CMSG_GROUP_RAID_CONVERT => CMSG_GROUP_RAID_CONVERT{}.astd_write_encrypted_client(w, e).await,
            Self::CMSG_GROUP_ASSISTANT_LEADER(c) => c.astd_write_encrypted_client(w, e).await,
            Self::CMSG_BUYBACK_ITEM(c) => c.astd_write_encrypted_client(w, e).await,
            Self::CMSG_MEETINGSTONE_JOIN(c) => c.astd_write_encrypted_client(w, e).await,
            Self::CMSG_MEETINGSTONE_LEAVE => CMSG_MEETINGSTONE_LEAVE{}.astd_write_encrypted_client(w, e).await,
            Self::CMSG_MEETINGSTONE_INFO => CMSG_MEETINGSTONE_INFO{}.astd_write_encrypted_client(w, e).await,
            Self::CMSG_CANCEL_GROWTH_AURA => CMSG_CANCEL_GROWTH_AURA{}.astd_write_encrypted_client(w, e).await,
            Self::CMSG_LOOT_ROLL(c) => c.astd_write_encrypted_client(w, e).await,
            Self::CMSG_LOOT_MASTER_GIVE(c) => c.astd_write_encrypted_client(w, e).await,
            Self::CMSG_REPAIR_ITEM(c) => c.astd_write_encrypted_client(w, e).await,
            Self::MSG_TALENT_WIPE_CONFIRM(c) => c.astd_write_encrypted_client(w, e).await,
            Self::CMSG_SUMMON_RESPONSE(c) => c.astd_write_encrypted_client(w, e).await,
            Self::CMSG_SELF_RES => CMSG_SELF_RES{}.astd_write_encrypted_client(w, e).await,
            Self::CMSG_TOGGLE_HELM => CMSG_TOGGLE_HELM{}.astd_write_encrypted_client(w, e).await,
            Self::CMSG_TOGGLE_CLOAK => CMSG_TOGGLE_CLOAK{}.astd_write_encrypted_client(w, e).await,
            Self::CMSG_SET_ACTIONBAR_TOGGLES(c) => c.astd_write_encrypted_client(w, e).await,
            Self::CMSG_ITEM_NAME_QUERY(c) => c.astd_write_encrypted_client(w, e).await,
            Self::CMSG_CHAR_RENAME(c) => c.astd_write_encrypted_client(w, e).await,
            Self::CMSG_MOVE_SPLINE_DONE(c) => c.astd_write_encrypted_client(w, e).await,
            Self::CMSG_MOVE_FALL_RESET(c) => c.astd_write_encrypted_client(w, e).await,
            Self::CMSG_REQUEST_RAID_INFO => CMSG_REQUEST_RAID_INFO{}.astd_write_encrypted_client(w, e).await,
            Self::CMSG_MOVE_TIME_SKIPPED(c) => c.astd_write_encrypted_client(w, e).await,
            Self::CMSG_MOVE_FEATHER_FALL_ACK(c) => c.astd_write_encrypted_client(w, e).await,
            Self::CMSG_MOVE_WATER_WALK_ACK(c) => c.astd_write_encrypted_client(w, e).await,
            Self::CMSG_MOVE_NOT_ACTIVE_MOVER(c) => c.astd_write_encrypted_client(w, e).await,
            Self::CMSG_BATTLEFIELD_STATUS => CMSG_BATTLEFIELD_STATUS{}.astd_write_encrypted_client(w, e).await,
            Self::CMSG_BATTLEFIELD_PORT(c) => c.astd_write_encrypted_client(w, e).await,
            Self::MSG_INSPECT_HONOR_STATS(c) => c.astd_write_encrypted_client(w, e).await,
            Self::CMSG_BATTLEMASTER_HELLO(c) => c.astd_write_encrypted_client(w, e).await,
            Self::CMSG_FORCE_WALK_SPEED_CHANGE_ACK(c) => c.astd_write_encrypted_client(w, e).await,
            Self::CMSG_FORCE_SWIM_BACK_SPEED_CHANGE_ACK(c) => c.astd_write_encrypted_client(w, e).await,
            Self::CMSG_FORCE_TURN_RATE_CHANGE_ACK(c) => c.astd_write_encrypted_client(w, e).await,
            Self::MSG_PVP_LOG_DATA => MSG_PVP_LOG_DATA_Client{}.astd_write_encrypted_client(w, e).await,
            Self::CMSG_LEAVE_BATTLEFIELD(c) => c.astd_write_encrypted_client(w, e).await,
            Self::CMSG_AREA_SPIRIT_HEALER_QUERY(c) => c.astd_write_encrypted_client(w, e).await,
            Self::CMSG_AREA_SPIRIT_HEALER_QUEUE(c) => c.astd_write_encrypted_client(w, e).await,
            Self::CMSG_WARDEN_DATA(c) => c.astd_write_encrypted_client(w, e).await,
            Self::MSG_BATTLEGROUND_PLAYER_POSITIONS => MSG_BATTLEGROUND_PLAYER_POSITIONS_Client{}.astd_write_encrypted_client(w, e).await,
            Self::CMSG_PET_STOP_ATTACK(c) => c.astd_write_encrypted_client(w, e).await,
            Self::CMSG_BATTLEMASTER_JOIN(c) => c.astd_write_encrypted_client(w, e).await,
            Self::CMSG_PET_UNLEARN(c) => c.astd_write_encrypted_client(w, e).await,
            Self::CMSG_PET_SPELL_AUTOCAST(c) => c.astd_write_encrypted_client(w, e).await,
            Self::CMSG_GUILD_INFO_TEXT(c) => c.astd_write_encrypted_client(w, e).await,
            Self::CMSG_ACTIVATETAXIEXPRESS(c) => c.astd_write_encrypted_client(w, e).await,
            Self::CMSG_SET_FACTION_INACTIVE(c) => c.astd_write_encrypted_client(w, e).await,
            Self::CMSG_SET_WATCHED_FACTION(c) => c.astd_write_encrypted_client(w, e).await,
            Self::CMSG_RESET_INSTANCES => CMSG_RESET_INSTANCES{}.astd_write_encrypted_client(w, e).await,
            Self::MSG_RAID_TARGET_UPDATE(c) => c.astd_write_encrypted_client(w, e).await,
            Self::MSG_RAID_READY_CHECK(c) => c.astd_write_encrypted_client(w, e).await,
            Self::CMSG_GMSURVEY_SUBMIT(c) => c.astd_write_encrypted_client(w, e).await,
        }
    }

    #[cfg(feature = "async-std")]
    pub async fn astd_write_unencrypted_client<W: async_std::io::WriteExt + Unpin + Send>(&self, mut w: W) -> Result<(), std::io::Error> {
        match self {
            Self::MSG_MOVE_WORLDPORT_ACK => MSG_MOVE_WORLDPORT_ACK{}.astd_write_unencrypted_client(w).await,
            Self::MSG_PETITION_DECLINE(c) => c.astd_write_unencrypted_client(w).await,
            Self::MSG_TABARDVENDOR_ACTIVATE(c) => c.astd_write_unencrypted_client(w).await,
            Self::MSG_QUEST_PUSH_RESULT(c) => c.astd_write_unencrypted_client(w).await,
            Self::MSG_MOVE_WATER_WALK(c) => c.astd_write_unencrypted_client(w).await,
            Self::MSG_PETITION_RENAME(c) => c.astd_write_unencrypted_client(w).await,
            Self::CMSG_BOOTME => CMSG_BOOTME{}.astd_write_unencrypted_client(w).await,
            Self::CMSG_DBLOOKUP(c) => c.astd_write_unencrypted_client(w).await,
            Self::CMSG_WORLD_TELEPORT(c) => c.astd_write_unencrypted_client(w).await,
            Self::CMSG_TELEPORT_TO_UNIT(c) => c.astd_write_unencrypted_client(w).await,
            Self::CMSG_CHAR_CREATE(c) => c.astd_write_unencrypted_client(w).await,
            Self::CMSG_CHAR_ENUM => CMSG_CHAR_ENUM{}.astd_write_unencrypted_client(w).await,
            Self::CMSG_CHAR_DELETE(c) => c.astd_write_unencrypted_client(w).await,
            Self::CMSG_PLAYER_LOGIN(c) => c.astd_write_unencrypted_client(w).await,
            Self::CMSG_PLAYER_LOGOUT => CMSG_PLAYER_LOGOUT{}.astd_write_unencrypted_client(w).await,
            Self::CMSG_LOGOUT_REQUEST => CMSG_LOGOUT_REQUEST{}.astd_write_unencrypted_client(w).await,
            Self::CMSG_LOGOUT_CANCEL => CMSG_LOGOUT_CANCEL{}.astd_write_unencrypted_client(w).await,
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
            Self::CMSG_FRIEND_LIST => CMSG_FRIEND_LIST{}.astd_write_unencrypted_client(w).await,
            Self::CMSG_ADD_FRIEND(c) => c.astd_write_unencrypted_client(w).await,
            Self::CMSG_DEL_FRIEND(c) => c.astd_write_unencrypted_client(w).await,
            Self::CMSG_ADD_IGNORE(c) => c.astd_write_unencrypted_client(w).await,
            Self::CMSG_DEL_IGNORE(c) => c.astd_write_unencrypted_client(w).await,
            Self::CMSG_GROUP_INVITE(c) => c.astd_write_unencrypted_client(w).await,
            Self::CMSG_GROUP_ACCEPT => CMSG_GROUP_ACCEPT{}.astd_write_unencrypted_client(w).await,
            Self::CMSG_GROUP_DECLINE => CMSG_GROUP_DECLINE{}.astd_write_unencrypted_client(w).await,
            Self::CMSG_GROUP_UNINVITE(c) => c.astd_write_unencrypted_client(w).await,
            Self::CMSG_GROUP_UNINVITE_GUID(c) => c.astd_write_unencrypted_client(w).await,
            Self::CMSG_GROUP_SET_LEADER(c) => c.astd_write_unencrypted_client(w).await,
            Self::CMSG_LOOT_METHOD(c) => c.astd_write_unencrypted_client(w).await,
            Self::CMSG_GROUP_DISBAND => CMSG_GROUP_DISBAND{}.astd_write_unencrypted_client(w).await,
            Self::CMSG_GUILD_CREATE(c) => c.astd_write_unencrypted_client(w).await,
            Self::CMSG_GUILD_INVITE(c) => c.astd_write_unencrypted_client(w).await,
            Self::CMSG_GUILD_ACCEPT => CMSG_GUILD_ACCEPT{}.astd_write_unencrypted_client(w).await,
            Self::CMSG_GUILD_DECLINE => CMSG_GUILD_DECLINE{}.astd_write_unencrypted_client(w).await,
            Self::CMSG_GUILD_INFO => CMSG_GUILD_INFO{}.astd_write_unencrypted_client(w).await,
            Self::CMSG_GUILD_ROSTER => CMSG_GUILD_ROSTER{}.astd_write_unencrypted_client(w).await,
            Self::CMSG_GUILD_PROMOTE(c) => c.astd_write_unencrypted_client(w).await,
            Self::CMSG_GUILD_DEMOTE(c) => c.astd_write_unencrypted_client(w).await,
            Self::CMSG_GUILD_LEAVE => CMSG_GUILD_LEAVE{}.astd_write_unencrypted_client(w).await,
            Self::CMSG_GUILD_REMOVE(c) => c.astd_write_unencrypted_client(w).await,
            Self::CMSG_GUILD_DISBAND => CMSG_GUILD_DISBAND{}.astd_write_unencrypted_client(w).await,
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
            Self::MSG_MOVE_TELEPORT_ACK(c) => c.astd_write_unencrypted_client(w).await,
            Self::MSG_MOVE_FALL_LAND(c) => c.astd_write_unencrypted_client(w).await,
            Self::MSG_MOVE_START_SWIM(c) => c.astd_write_unencrypted_client(w).await,
            Self::MSG_MOVE_STOP_SWIM(c) => c.astd_write_unencrypted_client(w).await,
            Self::MSG_MOVE_SET_FACING(c) => c.astd_write_unencrypted_client(w).await,
            Self::MSG_MOVE_SET_PITCH(c) => c.astd_write_unencrypted_client(w).await,
            Self::CMSG_MOVE_SET_RAW_POSITION(c) => c.astd_write_unencrypted_client(w).await,
            Self::CMSG_FORCE_RUN_SPEED_CHANGE_ACK(c) => c.astd_write_unencrypted_client(w).await,
            Self::CMSG_FORCE_RUN_BACK_SPEED_CHANGE_ACK(c) => c.astd_write_unencrypted_client(w).await,
            Self::CMSG_FORCE_SWIM_SPEED_CHANGE_ACK(c) => c.astd_write_unencrypted_client(w).await,
            Self::CMSG_FORCE_MOVE_ROOT_ACK(c) => c.astd_write_unencrypted_client(w).await,
            Self::CMSG_FORCE_MOVE_UNROOT_ACK(c) => c.astd_write_unencrypted_client(w).await,
            Self::MSG_MOVE_HEARTBEAT(c) => c.astd_write_unencrypted_client(w).await,
            Self::CMSG_MOVE_KNOCK_BACK_ACK(c) => c.astd_write_unencrypted_client(w).await,
            Self::CMSG_MOVE_HOVER_ACK(c) => c.astd_write_unencrypted_client(w).await,
            Self::CMSG_NEXT_CINEMATIC_CAMERA => CMSG_NEXT_CINEMATIC_CAMERA{}.astd_write_unencrypted_client(w).await,
            Self::CMSG_COMPLETE_CINEMATIC => CMSG_COMPLETE_CINEMATIC{}.astd_write_unencrypted_client(w).await,
            Self::CMSG_TUTORIAL_FLAG(c) => c.astd_write_unencrypted_client(w).await,
            Self::CMSG_TUTORIAL_CLEAR => CMSG_TUTORIAL_CLEAR{}.astd_write_unencrypted_client(w).await,
            Self::CMSG_TUTORIAL_RESET => CMSG_TUTORIAL_RESET{}.astd_write_unencrypted_client(w).await,
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
            Self::CMSG_BEGIN_TRADE => CMSG_BEGIN_TRADE{}.astd_write_unencrypted_client(w).await,
            Self::CMSG_BUSY_TRADE => CMSG_BUSY_TRADE{}.astd_write_unencrypted_client(w).await,
            Self::CMSG_IGNORE_TRADE => CMSG_IGNORE_TRADE{}.astd_write_unencrypted_client(w).await,
            Self::CMSG_ACCEPT_TRADE(c) => c.astd_write_unencrypted_client(w).await,
            Self::CMSG_UNACCEPT_TRADE => CMSG_UNACCEPT_TRADE{}.astd_write_unencrypted_client(w).await,
            Self::CMSG_CANCEL_TRADE => CMSG_CANCEL_TRADE{}.astd_write_unencrypted_client(w).await,
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
            Self::CMSG_SET_TARGET_OBSOLETE(c) => c.astd_write_unencrypted_client(w).await,
            Self::CMSG_ATTACKSWING(c) => c.astd_write_unencrypted_client(w).await,
            Self::CMSG_ATTACKSTOP => CMSG_ATTACKSTOP{}.astd_write_unencrypted_client(w).await,
            Self::CMSG_REPOP_REQUEST => CMSG_REPOP_REQUEST{}.astd_write_unencrypted_client(w).await,
            Self::CMSG_RESURRECT_RESPONSE(c) => c.astd_write_unencrypted_client(w).await,
            Self::CMSG_LOOT(c) => c.astd_write_unencrypted_client(w).await,
            Self::CMSG_LOOT_MONEY => CMSG_LOOT_MONEY{}.astd_write_unencrypted_client(w).await,
            Self::CMSG_LOOT_RELEASE(c) => c.astd_write_unencrypted_client(w).await,
            Self::CMSG_DUEL_ACCEPTED(c) => c.astd_write_unencrypted_client(w).await,
            Self::CMSG_DUEL_CANCELLED(c) => c.astd_write_unencrypted_client(w).await,
            Self::CMSG_MOUNTSPECIAL_ANIM => CMSG_MOUNTSPECIAL_ANIM{}.astd_write_unencrypted_client(w).await,
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
            Self::CMSG_QUESTGIVER_QUEST_AUTOLAUNCH => CMSG_QUESTGIVER_QUEST_AUTOLAUNCH{}.astd_write_unencrypted_client(w).await,
            Self::CMSG_QUESTGIVER_ACCEPT_QUEST(c) => c.astd_write_unencrypted_client(w).await,
            Self::CMSG_QUESTGIVER_COMPLETE_QUEST(c) => c.astd_write_unencrypted_client(w).await,
            Self::CMSG_QUESTGIVER_REQUEST_REWARD(c) => c.astd_write_unencrypted_client(w).await,
            Self::CMSG_QUESTGIVER_CHOOSE_REWARD(c) => c.astd_write_unencrypted_client(w).await,
            Self::CMSG_QUESTGIVER_CANCEL => CMSG_QUESTGIVER_CANCEL{}.astd_write_unencrypted_client(w).await,
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
            Self::CMSG_PLAYED_TIME => CMSG_PLAYED_TIME{}.astd_write_unencrypted_client(w).await,
            Self::CMSG_QUERY_TIME => CMSG_QUERY_TIME{}.astd_write_unencrypted_client(w).await,
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
            Self::MSG_LOOKING_FOR_GROUP => MSG_LOOKING_FOR_GROUP_Client{}.astd_write_unencrypted_client(w).await,
            Self::CMSG_UNLEARN_SKILL(c) => c.astd_write_unencrypted_client(w).await,
            Self::CMSG_GMTICKET_CREATE(c) => c.astd_write_unencrypted_client(w).await,
            Self::CMSG_GMTICKET_UPDATETEXT(c) => c.astd_write_unencrypted_client(w).await,
            Self::CMSG_REQUEST_ACCOUNT_DATA(c) => c.astd_write_unencrypted_client(w).await,
            Self::CMSG_UPDATE_ACCOUNT_DATA(c) => c.astd_write_unencrypted_client(w).await,
            Self::CMSG_GMTICKET_GETTICKET => CMSG_GMTICKET_GETTICKET{}.astd_write_unencrypted_client(w).await,
            Self::MSG_CORPSE_QUERY => MSG_CORPSE_QUERY_Client{}.astd_write_unencrypted_client(w).await,
            Self::CMSG_GMTICKET_DELETETICKET => CMSG_GMTICKET_DELETETICKET{}.astd_write_unencrypted_client(w).await,
            Self::CMSG_GMTICKET_SYSTEMSTATUS => CMSG_GMTICKET_SYSTEMSTATUS{}.astd_write_unencrypted_client(w).await,
            Self::CMSG_SPIRIT_HEALER_ACTIVATE(c) => c.astd_write_unencrypted_client(w).await,
            Self::CMSG_CHAT_IGNORED(c) => c.astd_write_unencrypted_client(w).await,
            Self::CMSG_GUILD_RANK(c) => c.astd_write_unencrypted_client(w).await,
            Self::CMSG_GUILD_ADD_RANK(c) => c.astd_write_unencrypted_client(w).await,
            Self::CMSG_GUILD_DEL_RANK => CMSG_GUILD_DEL_RANK{}.astd_write_unencrypted_client(w).await,
            Self::CMSG_GUILD_SET_PUBLIC_NOTE(c) => c.astd_write_unencrypted_client(w).await,
            Self::CMSG_GUILD_SET_OFFICER_NOTE(c) => c.astd_write_unencrypted_client(w).await,
            Self::CMSG_SEND_MAIL(c) => c.astd_write_unencrypted_client(w).await,
            Self::CMSG_GET_MAIL_LIST(c) => c.astd_write_unencrypted_client(w).await,
            Self::CMSG_BATTLEFIELD_LIST(c) => c.astd_write_unencrypted_client(w).await,
            Self::CMSG_BATTLEFIELD_JOIN(c) => c.astd_write_unencrypted_client(w).await,
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
            Self::CMSG_CANCEL_AUTO_REPEAT_SPELL => CMSG_CANCEL_AUTO_REPEAT_SPELL{}.astd_write_unencrypted_client(w).await,
            Self::MSG_LIST_STABLED_PETS(c) => c.astd_write_unencrypted_client(w).await,
            Self::CMSG_STABLE_PET(c) => c.astd_write_unencrypted_client(w).await,
            Self::CMSG_UNSTABLE_PET(c) => c.astd_write_unencrypted_client(w).await,
            Self::CMSG_BUY_STABLE_SLOT(c) => c.astd_write_unencrypted_client(w).await,
            Self::CMSG_STABLE_SWAP_PET(c) => c.astd_write_unencrypted_client(w).await,
            Self::CMSG_REQUEST_PET_INFO => CMSG_REQUEST_PET_INFO{}.astd_write_unencrypted_client(w).await,
            Self::CMSG_FAR_SIGHT(c) => c.astd_write_unencrypted_client(w).await,
            Self::CMSG_GROUP_CHANGE_SUB_GROUP(c) => c.astd_write_unencrypted_client(w).await,
            Self::CMSG_REQUEST_PARTY_MEMBER_STATS(c) => c.astd_write_unencrypted_client(w).await,
            Self::CMSG_GROUP_SWAP_SUB_GROUP(c) => c.astd_write_unencrypted_client(w).await,
            Self::CMSG_AUTOSTORE_BANK_ITEM(c) => c.astd_write_unencrypted_client(w).await,
            Self::CMSG_AUTOBANK_ITEM(c) => c.astd_write_unencrypted_client(w).await,
            Self::MSG_QUERY_NEXT_MAIL_TIME => MSG_QUERY_NEXT_MAIL_TIME_Client{}.astd_write_unencrypted_client(w).await,
            Self::CMSG_GROUP_RAID_CONVERT => CMSG_GROUP_RAID_CONVERT{}.astd_write_unencrypted_client(w).await,
            Self::CMSG_GROUP_ASSISTANT_LEADER(c) => c.astd_write_unencrypted_client(w).await,
            Self::CMSG_BUYBACK_ITEM(c) => c.astd_write_unencrypted_client(w).await,
            Self::CMSG_MEETINGSTONE_JOIN(c) => c.astd_write_unencrypted_client(w).await,
            Self::CMSG_MEETINGSTONE_LEAVE => CMSG_MEETINGSTONE_LEAVE{}.astd_write_unencrypted_client(w).await,
            Self::CMSG_MEETINGSTONE_INFO => CMSG_MEETINGSTONE_INFO{}.astd_write_unencrypted_client(w).await,
            Self::CMSG_CANCEL_GROWTH_AURA => CMSG_CANCEL_GROWTH_AURA{}.astd_write_unencrypted_client(w).await,
            Self::CMSG_LOOT_ROLL(c) => c.astd_write_unencrypted_client(w).await,
            Self::CMSG_LOOT_MASTER_GIVE(c) => c.astd_write_unencrypted_client(w).await,
            Self::CMSG_REPAIR_ITEM(c) => c.astd_write_unencrypted_client(w).await,
            Self::MSG_TALENT_WIPE_CONFIRM(c) => c.astd_write_unencrypted_client(w).await,
            Self::CMSG_SUMMON_RESPONSE(c) => c.astd_write_unencrypted_client(w).await,
            Self::CMSG_SELF_RES => CMSG_SELF_RES{}.astd_write_unencrypted_client(w).await,
            Self::CMSG_TOGGLE_HELM => CMSG_TOGGLE_HELM{}.astd_write_unencrypted_client(w).await,
            Self::CMSG_TOGGLE_CLOAK => CMSG_TOGGLE_CLOAK{}.astd_write_unencrypted_client(w).await,
            Self::CMSG_SET_ACTIONBAR_TOGGLES(c) => c.astd_write_unencrypted_client(w).await,
            Self::CMSG_ITEM_NAME_QUERY(c) => c.astd_write_unencrypted_client(w).await,
            Self::CMSG_CHAR_RENAME(c) => c.astd_write_unencrypted_client(w).await,
            Self::CMSG_MOVE_SPLINE_DONE(c) => c.astd_write_unencrypted_client(w).await,
            Self::CMSG_MOVE_FALL_RESET(c) => c.astd_write_unencrypted_client(w).await,
            Self::CMSG_REQUEST_RAID_INFO => CMSG_REQUEST_RAID_INFO{}.astd_write_unencrypted_client(w).await,
            Self::CMSG_MOVE_TIME_SKIPPED(c) => c.astd_write_unencrypted_client(w).await,
            Self::CMSG_MOVE_FEATHER_FALL_ACK(c) => c.astd_write_unencrypted_client(w).await,
            Self::CMSG_MOVE_WATER_WALK_ACK(c) => c.astd_write_unencrypted_client(w).await,
            Self::CMSG_MOVE_NOT_ACTIVE_MOVER(c) => c.astd_write_unencrypted_client(w).await,
            Self::CMSG_BATTLEFIELD_STATUS => CMSG_BATTLEFIELD_STATUS{}.astd_write_unencrypted_client(w).await,
            Self::CMSG_BATTLEFIELD_PORT(c) => c.astd_write_unencrypted_client(w).await,
            Self::MSG_INSPECT_HONOR_STATS(c) => c.astd_write_unencrypted_client(w).await,
            Self::CMSG_BATTLEMASTER_HELLO(c) => c.astd_write_unencrypted_client(w).await,
            Self::CMSG_FORCE_WALK_SPEED_CHANGE_ACK(c) => c.astd_write_unencrypted_client(w).await,
            Self::CMSG_FORCE_SWIM_BACK_SPEED_CHANGE_ACK(c) => c.astd_write_unencrypted_client(w).await,
            Self::CMSG_FORCE_TURN_RATE_CHANGE_ACK(c) => c.astd_write_unencrypted_client(w).await,
            Self::MSG_PVP_LOG_DATA => MSG_PVP_LOG_DATA_Client{}.astd_write_unencrypted_client(w).await,
            Self::CMSG_LEAVE_BATTLEFIELD(c) => c.astd_write_unencrypted_client(w).await,
            Self::CMSG_AREA_SPIRIT_HEALER_QUERY(c) => c.astd_write_unencrypted_client(w).await,
            Self::CMSG_AREA_SPIRIT_HEALER_QUEUE(c) => c.astd_write_unencrypted_client(w).await,
            Self::CMSG_WARDEN_DATA(c) => c.astd_write_unencrypted_client(w).await,
            Self::MSG_BATTLEGROUND_PLAYER_POSITIONS => MSG_BATTLEGROUND_PLAYER_POSITIONS_Client{}.astd_write_unencrypted_client(w).await,
            Self::CMSG_PET_STOP_ATTACK(c) => c.astd_write_unencrypted_client(w).await,
            Self::CMSG_BATTLEMASTER_JOIN(c) => c.astd_write_unencrypted_client(w).await,
            Self::CMSG_PET_UNLEARN(c) => c.astd_write_unencrypted_client(w).await,
            Self::CMSG_PET_SPELL_AUTOCAST(c) => c.astd_write_unencrypted_client(w).await,
            Self::CMSG_GUILD_INFO_TEXT(c) => c.astd_write_unencrypted_client(w).await,
            Self::CMSG_ACTIVATETAXIEXPRESS(c) => c.astd_write_unencrypted_client(w).await,
            Self::CMSG_SET_FACTION_INACTIVE(c) => c.astd_write_unencrypted_client(w).await,
            Self::CMSG_SET_WATCHED_FACTION(c) => c.astd_write_unencrypted_client(w).await,
            Self::CMSG_RESET_INSTANCES => CMSG_RESET_INSTANCES{}.astd_write_unencrypted_client(w).await,
            Self::MSG_RAID_TARGET_UPDATE(c) => c.astd_write_unencrypted_client(w).await,
            Self::MSG_RAID_READY_CHECK(c) => c.astd_write_unencrypted_client(w).await,
            Self::CMSG_GMSURVEY_SUBMIT(c) => c.astd_write_unencrypted_client(w).await,
        }
    }

    pub fn movement_info(&self) -> Option<&MovementInfo> {
        match self {
            Self::MSG_MOVE_START_FORWARD(c) => Some(&c.info),
            Self::MSG_MOVE_START_BACKWARD(c) => Some(&c.info),
            Self::MSG_MOVE_STOP(c) => Some(&c.info),
            Self::MSG_MOVE_START_STRAFE_LEFT(c) => Some(&c.info),
            Self::MSG_MOVE_START_STRAFE_RIGHT(c) => Some(&c.info),
            Self::MSG_MOVE_STOP_STRAFE(c) => Some(&c.info),
            Self::MSG_MOVE_JUMP(c) => Some(&c.info),
            Self::MSG_MOVE_START_TURN_LEFT(c) => Some(&c.info),
            Self::MSG_MOVE_START_TURN_RIGHT(c) => Some(&c.info),
            Self::MSG_MOVE_STOP_TURN(c) => Some(&c.info),
            Self::MSG_MOVE_START_PITCH_UP(c) => Some(&c.info),
            Self::MSG_MOVE_START_PITCH_DOWN(c) => Some(&c.info),
            Self::MSG_MOVE_STOP_PITCH(c) => Some(&c.info),
            Self::MSG_MOVE_SET_RUN_MODE(c) => Some(&c.info),
            Self::MSG_MOVE_SET_WALK_MODE(c) => Some(&c.info),
            Self::MSG_MOVE_FALL_LAND(c) => Some(&c.info),
            Self::MSG_MOVE_START_SWIM(c) => Some(&c.info),
            Self::MSG_MOVE_STOP_SWIM(c) => Some(&c.info),
            Self::MSG_MOVE_SET_FACING(c) => Some(&c.info),
            Self::MSG_MOVE_SET_PITCH(c) => Some(&c.info),
            Self::MSG_MOVE_HEARTBEAT(c) => Some(&c.info),
            Self::CMSG_MOVE_FALL_RESET(c) => Some(&c.info),
            _ => None,
        }
    }

}

impl std::fmt::Display for ClientOpcodeMessage {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            ClientOpcodeMessage::MSG_MOVE_WORLDPORT_ACK => "MSG_MOVE_WORLDPORT_ACK",
            ClientOpcodeMessage::MSG_PETITION_DECLINE(_) => "MSG_PETITION_DECLINE",
            ClientOpcodeMessage::MSG_TABARDVENDOR_ACTIVATE(_) => "MSG_TABARDVENDOR_ACTIVATE",
            ClientOpcodeMessage::MSG_QUEST_PUSH_RESULT(_) => "MSG_QUEST_PUSH_RESULT",
            ClientOpcodeMessage::MSG_MOVE_WATER_WALK(_) => "MSG_MOVE_WATER_WALK",
            ClientOpcodeMessage::MSG_PETITION_RENAME(_) => "MSG_PETITION_RENAME",
            ClientOpcodeMessage::CMSG_BOOTME => "CMSG_BOOTME",
            ClientOpcodeMessage::CMSG_DBLOOKUP(_) => "CMSG_DBLOOKUP",
            ClientOpcodeMessage::CMSG_WORLD_TELEPORT(_) => "CMSG_WORLD_TELEPORT",
            ClientOpcodeMessage::CMSG_TELEPORT_TO_UNIT(_) => "CMSG_TELEPORT_TO_UNIT",
            ClientOpcodeMessage::CMSG_CHAR_CREATE(_) => "CMSG_CHAR_CREATE",
            ClientOpcodeMessage::CMSG_CHAR_ENUM => "CMSG_CHAR_ENUM",
            ClientOpcodeMessage::CMSG_CHAR_DELETE(_) => "CMSG_CHAR_DELETE",
            ClientOpcodeMessage::CMSG_PLAYER_LOGIN(_) => "CMSG_PLAYER_LOGIN",
            ClientOpcodeMessage::CMSG_PLAYER_LOGOUT => "CMSG_PLAYER_LOGOUT",
            ClientOpcodeMessage::CMSG_LOGOUT_REQUEST => "CMSG_LOGOUT_REQUEST",
            ClientOpcodeMessage::CMSG_LOGOUT_CANCEL => "CMSG_LOGOUT_CANCEL",
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
            ClientOpcodeMessage::CMSG_FRIEND_LIST => "CMSG_FRIEND_LIST",
            ClientOpcodeMessage::CMSG_ADD_FRIEND(_) => "CMSG_ADD_FRIEND",
            ClientOpcodeMessage::CMSG_DEL_FRIEND(_) => "CMSG_DEL_FRIEND",
            ClientOpcodeMessage::CMSG_ADD_IGNORE(_) => "CMSG_ADD_IGNORE",
            ClientOpcodeMessage::CMSG_DEL_IGNORE(_) => "CMSG_DEL_IGNORE",
            ClientOpcodeMessage::CMSG_GROUP_INVITE(_) => "CMSG_GROUP_INVITE",
            ClientOpcodeMessage::CMSG_GROUP_ACCEPT => "CMSG_GROUP_ACCEPT",
            ClientOpcodeMessage::CMSG_GROUP_DECLINE => "CMSG_GROUP_DECLINE",
            ClientOpcodeMessage::CMSG_GROUP_UNINVITE(_) => "CMSG_GROUP_UNINVITE",
            ClientOpcodeMessage::CMSG_GROUP_UNINVITE_GUID(_) => "CMSG_GROUP_UNINVITE_GUID",
            ClientOpcodeMessage::CMSG_GROUP_SET_LEADER(_) => "CMSG_GROUP_SET_LEADER",
            ClientOpcodeMessage::CMSG_LOOT_METHOD(_) => "CMSG_LOOT_METHOD",
            ClientOpcodeMessage::CMSG_GROUP_DISBAND => "CMSG_GROUP_DISBAND",
            ClientOpcodeMessage::CMSG_GUILD_CREATE(_) => "CMSG_GUILD_CREATE",
            ClientOpcodeMessage::CMSG_GUILD_INVITE(_) => "CMSG_GUILD_INVITE",
            ClientOpcodeMessage::CMSG_GUILD_ACCEPT => "CMSG_GUILD_ACCEPT",
            ClientOpcodeMessage::CMSG_GUILD_DECLINE => "CMSG_GUILD_DECLINE",
            ClientOpcodeMessage::CMSG_GUILD_INFO => "CMSG_GUILD_INFO",
            ClientOpcodeMessage::CMSG_GUILD_ROSTER => "CMSG_GUILD_ROSTER",
            ClientOpcodeMessage::CMSG_GUILD_PROMOTE(_) => "CMSG_GUILD_PROMOTE",
            ClientOpcodeMessage::CMSG_GUILD_DEMOTE(_) => "CMSG_GUILD_DEMOTE",
            ClientOpcodeMessage::CMSG_GUILD_LEAVE => "CMSG_GUILD_LEAVE",
            ClientOpcodeMessage::CMSG_GUILD_REMOVE(_) => "CMSG_GUILD_REMOVE",
            ClientOpcodeMessage::CMSG_GUILD_DISBAND => "CMSG_GUILD_DISBAND",
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
            ClientOpcodeMessage::MSG_MOVE_START_FORWARD(_) => "MSG_MOVE_START_FORWARD_Client",
            ClientOpcodeMessage::MSG_MOVE_START_BACKWARD(_) => "MSG_MOVE_START_BACKWARD_Client",
            ClientOpcodeMessage::MSG_MOVE_STOP(_) => "MSG_MOVE_STOP_Client",
            ClientOpcodeMessage::MSG_MOVE_START_STRAFE_LEFT(_) => "MSG_MOVE_START_STRAFE_LEFT_Client",
            ClientOpcodeMessage::MSG_MOVE_START_STRAFE_RIGHT(_) => "MSG_MOVE_START_STRAFE_RIGHT_Client",
            ClientOpcodeMessage::MSG_MOVE_STOP_STRAFE(_) => "MSG_MOVE_STOP_STRAFE_Client",
            ClientOpcodeMessage::MSG_MOVE_JUMP(_) => "MSG_MOVE_JUMP_Client",
            ClientOpcodeMessage::MSG_MOVE_START_TURN_LEFT(_) => "MSG_MOVE_START_TURN_LEFT_Client",
            ClientOpcodeMessage::MSG_MOVE_START_TURN_RIGHT(_) => "MSG_MOVE_START_TURN_RIGHT_Client",
            ClientOpcodeMessage::MSG_MOVE_STOP_TURN(_) => "MSG_MOVE_STOP_TURN_Client",
            ClientOpcodeMessage::MSG_MOVE_START_PITCH_UP(_) => "MSG_MOVE_START_PITCH_UP_Client",
            ClientOpcodeMessage::MSG_MOVE_START_PITCH_DOWN(_) => "MSG_MOVE_START_PITCH_DOWN_Client",
            ClientOpcodeMessage::MSG_MOVE_STOP_PITCH(_) => "MSG_MOVE_STOP_PITCH_Client",
            ClientOpcodeMessage::MSG_MOVE_SET_RUN_MODE(_) => "MSG_MOVE_SET_RUN_MODE_Client",
            ClientOpcodeMessage::MSG_MOVE_SET_WALK_MODE(_) => "MSG_MOVE_SET_WALK_MODE_Client",
            ClientOpcodeMessage::MSG_MOVE_TELEPORT_ACK(_) => "MSG_MOVE_TELEPORT_ACK_Client",
            ClientOpcodeMessage::MSG_MOVE_FALL_LAND(_) => "MSG_MOVE_FALL_LAND_Client",
            ClientOpcodeMessage::MSG_MOVE_START_SWIM(_) => "MSG_MOVE_START_SWIM_Client",
            ClientOpcodeMessage::MSG_MOVE_STOP_SWIM(_) => "MSG_MOVE_STOP_SWIM_Client",
            ClientOpcodeMessage::MSG_MOVE_SET_FACING(_) => "MSG_MOVE_SET_FACING_Client",
            ClientOpcodeMessage::MSG_MOVE_SET_PITCH(_) => "MSG_MOVE_SET_PITCH_Client",
            ClientOpcodeMessage::CMSG_MOVE_SET_RAW_POSITION(_) => "CMSG_MOVE_SET_RAW_POSITION",
            ClientOpcodeMessage::CMSG_FORCE_RUN_SPEED_CHANGE_ACK(_) => "CMSG_FORCE_RUN_SPEED_CHANGE_ACK",
            ClientOpcodeMessage::CMSG_FORCE_RUN_BACK_SPEED_CHANGE_ACK(_) => "CMSG_FORCE_RUN_BACK_SPEED_CHANGE_ACK",
            ClientOpcodeMessage::CMSG_FORCE_SWIM_SPEED_CHANGE_ACK(_) => "CMSG_FORCE_SWIM_SPEED_CHANGE_ACK",
            ClientOpcodeMessage::CMSG_FORCE_MOVE_ROOT_ACK(_) => "CMSG_FORCE_MOVE_ROOT_ACK",
            ClientOpcodeMessage::CMSG_FORCE_MOVE_UNROOT_ACK(_) => "CMSG_FORCE_MOVE_UNROOT_ACK",
            ClientOpcodeMessage::MSG_MOVE_HEARTBEAT(_) => "MSG_MOVE_HEARTBEAT_Client",
            ClientOpcodeMessage::CMSG_MOVE_KNOCK_BACK_ACK(_) => "CMSG_MOVE_KNOCK_BACK_ACK",
            ClientOpcodeMessage::CMSG_MOVE_HOVER_ACK(_) => "CMSG_MOVE_HOVER_ACK",
            ClientOpcodeMessage::CMSG_NEXT_CINEMATIC_CAMERA => "CMSG_NEXT_CINEMATIC_CAMERA",
            ClientOpcodeMessage::CMSG_COMPLETE_CINEMATIC => "CMSG_COMPLETE_CINEMATIC",
            ClientOpcodeMessage::CMSG_TUTORIAL_FLAG(_) => "CMSG_TUTORIAL_FLAG",
            ClientOpcodeMessage::CMSG_TUTORIAL_CLEAR => "CMSG_TUTORIAL_CLEAR",
            ClientOpcodeMessage::CMSG_TUTORIAL_RESET => "CMSG_TUTORIAL_RESET",
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
            ClientOpcodeMessage::CMSG_BEGIN_TRADE => "CMSG_BEGIN_TRADE",
            ClientOpcodeMessage::CMSG_BUSY_TRADE => "CMSG_BUSY_TRADE",
            ClientOpcodeMessage::CMSG_IGNORE_TRADE => "CMSG_IGNORE_TRADE",
            ClientOpcodeMessage::CMSG_ACCEPT_TRADE(_) => "CMSG_ACCEPT_TRADE",
            ClientOpcodeMessage::CMSG_UNACCEPT_TRADE => "CMSG_UNACCEPT_TRADE",
            ClientOpcodeMessage::CMSG_CANCEL_TRADE => "CMSG_CANCEL_TRADE",
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
            ClientOpcodeMessage::CMSG_SET_TARGET_OBSOLETE(_) => "CMSG_SET_TARGET_OBSOLETE",
            ClientOpcodeMessage::CMSG_ATTACKSWING(_) => "CMSG_ATTACKSWING",
            ClientOpcodeMessage::CMSG_ATTACKSTOP => "CMSG_ATTACKSTOP",
            ClientOpcodeMessage::CMSG_REPOP_REQUEST => "CMSG_REPOP_REQUEST",
            ClientOpcodeMessage::CMSG_RESURRECT_RESPONSE(_) => "CMSG_RESURRECT_RESPONSE",
            ClientOpcodeMessage::CMSG_LOOT(_) => "CMSG_LOOT",
            ClientOpcodeMessage::CMSG_LOOT_MONEY => "CMSG_LOOT_MONEY",
            ClientOpcodeMessage::CMSG_LOOT_RELEASE(_) => "CMSG_LOOT_RELEASE",
            ClientOpcodeMessage::CMSG_DUEL_ACCEPTED(_) => "CMSG_DUEL_ACCEPTED",
            ClientOpcodeMessage::CMSG_DUEL_CANCELLED(_) => "CMSG_DUEL_CANCELLED",
            ClientOpcodeMessage::CMSG_MOUNTSPECIAL_ANIM => "CMSG_MOUNTSPECIAL_ANIM",
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
            ClientOpcodeMessage::CMSG_QUESTGIVER_QUEST_AUTOLAUNCH => "CMSG_QUESTGIVER_QUEST_AUTOLAUNCH",
            ClientOpcodeMessage::CMSG_QUESTGIVER_ACCEPT_QUEST(_) => "CMSG_QUESTGIVER_ACCEPT_QUEST",
            ClientOpcodeMessage::CMSG_QUESTGIVER_COMPLETE_QUEST(_) => "CMSG_QUESTGIVER_COMPLETE_QUEST",
            ClientOpcodeMessage::CMSG_QUESTGIVER_REQUEST_REWARD(_) => "CMSG_QUESTGIVER_REQUEST_REWARD",
            ClientOpcodeMessage::CMSG_QUESTGIVER_CHOOSE_REWARD(_) => "CMSG_QUESTGIVER_CHOOSE_REWARD",
            ClientOpcodeMessage::CMSG_QUESTGIVER_CANCEL => "CMSG_QUESTGIVER_CANCEL",
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
            ClientOpcodeMessage::CMSG_PLAYED_TIME => "CMSG_PLAYED_TIME",
            ClientOpcodeMessage::CMSG_QUERY_TIME => "CMSG_QUERY_TIME",
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
            ClientOpcodeMessage::MSG_LOOKING_FOR_GROUP => "MSG_LOOKING_FOR_GROUP_Client",
            ClientOpcodeMessage::CMSG_UNLEARN_SKILL(_) => "CMSG_UNLEARN_SKILL",
            ClientOpcodeMessage::CMSG_GMTICKET_CREATE(_) => "CMSG_GMTICKET_CREATE",
            ClientOpcodeMessage::CMSG_GMTICKET_UPDATETEXT(_) => "CMSG_GMTICKET_UPDATETEXT",
            ClientOpcodeMessage::CMSG_REQUEST_ACCOUNT_DATA(_) => "CMSG_REQUEST_ACCOUNT_DATA",
            ClientOpcodeMessage::CMSG_UPDATE_ACCOUNT_DATA(_) => "CMSG_UPDATE_ACCOUNT_DATA",
            ClientOpcodeMessage::CMSG_GMTICKET_GETTICKET => "CMSG_GMTICKET_GETTICKET",
            ClientOpcodeMessage::MSG_CORPSE_QUERY => "MSG_CORPSE_QUERY_Client",
            ClientOpcodeMessage::CMSG_GMTICKET_DELETETICKET => "CMSG_GMTICKET_DELETETICKET",
            ClientOpcodeMessage::CMSG_GMTICKET_SYSTEMSTATUS => "CMSG_GMTICKET_SYSTEMSTATUS",
            ClientOpcodeMessage::CMSG_SPIRIT_HEALER_ACTIVATE(_) => "CMSG_SPIRIT_HEALER_ACTIVATE",
            ClientOpcodeMessage::CMSG_CHAT_IGNORED(_) => "CMSG_CHAT_IGNORED",
            ClientOpcodeMessage::CMSG_GUILD_RANK(_) => "CMSG_GUILD_RANK",
            ClientOpcodeMessage::CMSG_GUILD_ADD_RANK(_) => "CMSG_GUILD_ADD_RANK",
            ClientOpcodeMessage::CMSG_GUILD_DEL_RANK => "CMSG_GUILD_DEL_RANK",
            ClientOpcodeMessage::CMSG_GUILD_SET_PUBLIC_NOTE(_) => "CMSG_GUILD_SET_PUBLIC_NOTE",
            ClientOpcodeMessage::CMSG_GUILD_SET_OFFICER_NOTE(_) => "CMSG_GUILD_SET_OFFICER_NOTE",
            ClientOpcodeMessage::CMSG_SEND_MAIL(_) => "CMSG_SEND_MAIL",
            ClientOpcodeMessage::CMSG_GET_MAIL_LIST(_) => "CMSG_GET_MAIL_LIST",
            ClientOpcodeMessage::CMSG_BATTLEFIELD_LIST(_) => "CMSG_BATTLEFIELD_LIST",
            ClientOpcodeMessage::CMSG_BATTLEFIELD_JOIN(_) => "CMSG_BATTLEFIELD_JOIN",
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
            ClientOpcodeMessage::CMSG_CANCEL_AUTO_REPEAT_SPELL => "CMSG_CANCEL_AUTO_REPEAT_SPELL",
            ClientOpcodeMessage::MSG_LIST_STABLED_PETS(_) => "MSG_LIST_STABLED_PETS_Client",
            ClientOpcodeMessage::CMSG_STABLE_PET(_) => "CMSG_STABLE_PET",
            ClientOpcodeMessage::CMSG_UNSTABLE_PET(_) => "CMSG_UNSTABLE_PET",
            ClientOpcodeMessage::CMSG_BUY_STABLE_SLOT(_) => "CMSG_BUY_STABLE_SLOT",
            ClientOpcodeMessage::CMSG_STABLE_SWAP_PET(_) => "CMSG_STABLE_SWAP_PET",
            ClientOpcodeMessage::CMSG_REQUEST_PET_INFO => "CMSG_REQUEST_PET_INFO",
            ClientOpcodeMessage::CMSG_FAR_SIGHT(_) => "CMSG_FAR_SIGHT",
            ClientOpcodeMessage::CMSG_GROUP_CHANGE_SUB_GROUP(_) => "CMSG_GROUP_CHANGE_SUB_GROUP",
            ClientOpcodeMessage::CMSG_REQUEST_PARTY_MEMBER_STATS(_) => "CMSG_REQUEST_PARTY_MEMBER_STATS",
            ClientOpcodeMessage::CMSG_GROUP_SWAP_SUB_GROUP(_) => "CMSG_GROUP_SWAP_SUB_GROUP",
            ClientOpcodeMessage::CMSG_AUTOSTORE_BANK_ITEM(_) => "CMSG_AUTOSTORE_BANK_ITEM",
            ClientOpcodeMessage::CMSG_AUTOBANK_ITEM(_) => "CMSG_AUTOBANK_ITEM",
            ClientOpcodeMessage::MSG_QUERY_NEXT_MAIL_TIME => "MSG_QUERY_NEXT_MAIL_TIME_Client",
            ClientOpcodeMessage::CMSG_GROUP_RAID_CONVERT => "CMSG_GROUP_RAID_CONVERT",
            ClientOpcodeMessage::CMSG_GROUP_ASSISTANT_LEADER(_) => "CMSG_GROUP_ASSISTANT_LEADER",
            ClientOpcodeMessage::CMSG_BUYBACK_ITEM(_) => "CMSG_BUYBACK_ITEM",
            ClientOpcodeMessage::CMSG_MEETINGSTONE_JOIN(_) => "CMSG_MEETINGSTONE_JOIN",
            ClientOpcodeMessage::CMSG_MEETINGSTONE_LEAVE => "CMSG_MEETINGSTONE_LEAVE",
            ClientOpcodeMessage::CMSG_MEETINGSTONE_INFO => "CMSG_MEETINGSTONE_INFO",
            ClientOpcodeMessage::CMSG_CANCEL_GROWTH_AURA => "CMSG_CANCEL_GROWTH_AURA",
            ClientOpcodeMessage::CMSG_LOOT_ROLL(_) => "CMSG_LOOT_ROLL",
            ClientOpcodeMessage::CMSG_LOOT_MASTER_GIVE(_) => "CMSG_LOOT_MASTER_GIVE",
            ClientOpcodeMessage::CMSG_REPAIR_ITEM(_) => "CMSG_REPAIR_ITEM",
            ClientOpcodeMessage::MSG_TALENT_WIPE_CONFIRM(_) => "MSG_TALENT_WIPE_CONFIRM_Client",
            ClientOpcodeMessage::CMSG_SUMMON_RESPONSE(_) => "CMSG_SUMMON_RESPONSE",
            ClientOpcodeMessage::CMSG_SELF_RES => "CMSG_SELF_RES",
            ClientOpcodeMessage::CMSG_TOGGLE_HELM => "CMSG_TOGGLE_HELM",
            ClientOpcodeMessage::CMSG_TOGGLE_CLOAK => "CMSG_TOGGLE_CLOAK",
            ClientOpcodeMessage::CMSG_SET_ACTIONBAR_TOGGLES(_) => "CMSG_SET_ACTIONBAR_TOGGLES",
            ClientOpcodeMessage::CMSG_ITEM_NAME_QUERY(_) => "CMSG_ITEM_NAME_QUERY",
            ClientOpcodeMessage::CMSG_CHAR_RENAME(_) => "CMSG_CHAR_RENAME",
            ClientOpcodeMessage::CMSG_MOVE_SPLINE_DONE(_) => "CMSG_MOVE_SPLINE_DONE",
            ClientOpcodeMessage::CMSG_MOVE_FALL_RESET(_) => "CMSG_MOVE_FALL_RESET",
            ClientOpcodeMessage::CMSG_REQUEST_RAID_INFO => "CMSG_REQUEST_RAID_INFO",
            ClientOpcodeMessage::CMSG_MOVE_TIME_SKIPPED(_) => "CMSG_MOVE_TIME_SKIPPED",
            ClientOpcodeMessage::CMSG_MOVE_FEATHER_FALL_ACK(_) => "CMSG_MOVE_FEATHER_FALL_ACK",
            ClientOpcodeMessage::CMSG_MOVE_WATER_WALK_ACK(_) => "CMSG_MOVE_WATER_WALK_ACK",
            ClientOpcodeMessage::CMSG_MOVE_NOT_ACTIVE_MOVER(_) => "CMSG_MOVE_NOT_ACTIVE_MOVER",
            ClientOpcodeMessage::CMSG_BATTLEFIELD_STATUS => "CMSG_BATTLEFIELD_STATUS",
            ClientOpcodeMessage::CMSG_BATTLEFIELD_PORT(_) => "CMSG_BATTLEFIELD_PORT",
            ClientOpcodeMessage::MSG_INSPECT_HONOR_STATS(_) => "MSG_INSPECT_HONOR_STATS_Client",
            ClientOpcodeMessage::CMSG_BATTLEMASTER_HELLO(_) => "CMSG_BATTLEMASTER_HELLO",
            ClientOpcodeMessage::CMSG_FORCE_WALK_SPEED_CHANGE_ACK(_) => "CMSG_FORCE_WALK_SPEED_CHANGE_ACK",
            ClientOpcodeMessage::CMSG_FORCE_SWIM_BACK_SPEED_CHANGE_ACK(_) => "CMSG_FORCE_SWIM_BACK_SPEED_CHANGE_ACK",
            ClientOpcodeMessage::CMSG_FORCE_TURN_RATE_CHANGE_ACK(_) => "CMSG_FORCE_TURN_RATE_CHANGE_ACK",
            ClientOpcodeMessage::MSG_PVP_LOG_DATA => "MSG_PVP_LOG_DATA_Client",
            ClientOpcodeMessage::CMSG_LEAVE_BATTLEFIELD(_) => "CMSG_LEAVE_BATTLEFIELD",
            ClientOpcodeMessage::CMSG_AREA_SPIRIT_HEALER_QUERY(_) => "CMSG_AREA_SPIRIT_HEALER_QUERY",
            ClientOpcodeMessage::CMSG_AREA_SPIRIT_HEALER_QUEUE(_) => "CMSG_AREA_SPIRIT_HEALER_QUEUE",
            ClientOpcodeMessage::CMSG_WARDEN_DATA(_) => "CMSG_WARDEN_DATA",
            ClientOpcodeMessage::MSG_BATTLEGROUND_PLAYER_POSITIONS => "MSG_BATTLEGROUND_PLAYER_POSITIONS_Client",
            ClientOpcodeMessage::CMSG_PET_STOP_ATTACK(_) => "CMSG_PET_STOP_ATTACK",
            ClientOpcodeMessage::CMSG_BATTLEMASTER_JOIN(_) => "CMSG_BATTLEMASTER_JOIN",
            ClientOpcodeMessage::CMSG_PET_UNLEARN(_) => "CMSG_PET_UNLEARN",
            ClientOpcodeMessage::CMSG_PET_SPELL_AUTOCAST(_) => "CMSG_PET_SPELL_AUTOCAST",
            ClientOpcodeMessage::CMSG_GUILD_INFO_TEXT(_) => "CMSG_GUILD_INFO_TEXT",
            ClientOpcodeMessage::CMSG_ACTIVATETAXIEXPRESS(_) => "CMSG_ACTIVATETAXIEXPRESS",
            ClientOpcodeMessage::CMSG_SET_FACTION_INACTIVE(_) => "CMSG_SET_FACTION_INACTIVE",
            ClientOpcodeMessage::CMSG_SET_WATCHED_FACTION(_) => "CMSG_SET_WATCHED_FACTION",
            ClientOpcodeMessage::CMSG_RESET_INSTANCES => "CMSG_RESET_INSTANCES",
            ClientOpcodeMessage::MSG_RAID_TARGET_UPDATE(_) => "MSG_RAID_TARGET_UPDATE_Client",
            ClientOpcodeMessage::MSG_RAID_READY_CHECK(_) => "MSG_RAID_READY_CHECK_Client",
            ClientOpcodeMessage::CMSG_GMSURVEY_SUBMIT(_) => "CMSG_GMSURVEY_SUBMIT",
        })
    }
}

impl From<MSG_MOVE_WORLDPORT_ACK> for ClientOpcodeMessage {
    fn from(_: MSG_MOVE_WORLDPORT_ACK) -> Self {
        Self::MSG_MOVE_WORLDPORT_ACK
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

impl From<MSG_MOVE_WATER_WALK> for ClientOpcodeMessage {
    fn from(c: MSG_MOVE_WATER_WALK) -> Self {
        Self::MSG_MOVE_WATER_WALK(c)
    }
}

impl From<MSG_PETITION_RENAME> for ClientOpcodeMessage {
    fn from(c: MSG_PETITION_RENAME) -> Self {
        Self::MSG_PETITION_RENAME(c)
    }
}

impl From<CMSG_BOOTME> for ClientOpcodeMessage {
    fn from(_: CMSG_BOOTME) -> Self {
        Self::CMSG_BOOTME
    }
}

impl From<CMSG_DBLOOKUP> for ClientOpcodeMessage {
    fn from(c: CMSG_DBLOOKUP) -> Self {
        Self::CMSG_DBLOOKUP(c)
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
    fn from(_: CMSG_CHAR_ENUM) -> Self {
        Self::CMSG_CHAR_ENUM
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
    fn from(_: CMSG_PLAYER_LOGOUT) -> Self {
        Self::CMSG_PLAYER_LOGOUT
    }
}

impl From<CMSG_LOGOUT_REQUEST> for ClientOpcodeMessage {
    fn from(_: CMSG_LOGOUT_REQUEST) -> Self {
        Self::CMSG_LOGOUT_REQUEST
    }
}

impl From<CMSG_LOGOUT_CANCEL> for ClientOpcodeMessage {
    fn from(_: CMSG_LOGOUT_CANCEL) -> Self {
        Self::CMSG_LOGOUT_CANCEL
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

impl From<CMSG_FRIEND_LIST> for ClientOpcodeMessage {
    fn from(_: CMSG_FRIEND_LIST) -> Self {
        Self::CMSG_FRIEND_LIST
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
    fn from(_: CMSG_GROUP_ACCEPT) -> Self {
        Self::CMSG_GROUP_ACCEPT
    }
}

impl From<CMSG_GROUP_DECLINE> for ClientOpcodeMessage {
    fn from(_: CMSG_GROUP_DECLINE) -> Self {
        Self::CMSG_GROUP_DECLINE
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
    fn from(_: CMSG_GROUP_DISBAND) -> Self {
        Self::CMSG_GROUP_DISBAND
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
    fn from(_: CMSG_GUILD_ACCEPT) -> Self {
        Self::CMSG_GUILD_ACCEPT
    }
}

impl From<CMSG_GUILD_DECLINE> for ClientOpcodeMessage {
    fn from(_: CMSG_GUILD_DECLINE) -> Self {
        Self::CMSG_GUILD_DECLINE
    }
}

impl From<CMSG_GUILD_INFO> for ClientOpcodeMessage {
    fn from(_: CMSG_GUILD_INFO) -> Self {
        Self::CMSG_GUILD_INFO
    }
}

impl From<CMSG_GUILD_ROSTER> for ClientOpcodeMessage {
    fn from(_: CMSG_GUILD_ROSTER) -> Self {
        Self::CMSG_GUILD_ROSTER
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
    fn from(_: CMSG_GUILD_LEAVE) -> Self {
        Self::CMSG_GUILD_LEAVE
    }
}

impl From<CMSG_GUILD_REMOVE> for ClientOpcodeMessage {
    fn from(c: CMSG_GUILD_REMOVE) -> Self {
        Self::CMSG_GUILD_REMOVE(c)
    }
}

impl From<CMSG_GUILD_DISBAND> for ClientOpcodeMessage {
    fn from(_: CMSG_GUILD_DISBAND) -> Self {
        Self::CMSG_GUILD_DISBAND
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

impl From<MSG_MOVE_START_FORWARD_Client> for ClientOpcodeMessage {
    fn from(c: MSG_MOVE_START_FORWARD_Client) -> Self {
        Self::MSG_MOVE_START_FORWARD(c)
    }
}

impl From<MSG_MOVE_START_BACKWARD_Client> for ClientOpcodeMessage {
    fn from(c: MSG_MOVE_START_BACKWARD_Client) -> Self {
        Self::MSG_MOVE_START_BACKWARD(c)
    }
}

impl From<MSG_MOVE_STOP_Client> for ClientOpcodeMessage {
    fn from(c: MSG_MOVE_STOP_Client) -> Self {
        Self::MSG_MOVE_STOP(c)
    }
}

impl From<MSG_MOVE_START_STRAFE_LEFT_Client> for ClientOpcodeMessage {
    fn from(c: MSG_MOVE_START_STRAFE_LEFT_Client) -> Self {
        Self::MSG_MOVE_START_STRAFE_LEFT(c)
    }
}

impl From<MSG_MOVE_START_STRAFE_RIGHT_Client> for ClientOpcodeMessage {
    fn from(c: MSG_MOVE_START_STRAFE_RIGHT_Client) -> Self {
        Self::MSG_MOVE_START_STRAFE_RIGHT(c)
    }
}

impl From<MSG_MOVE_STOP_STRAFE_Client> for ClientOpcodeMessage {
    fn from(c: MSG_MOVE_STOP_STRAFE_Client) -> Self {
        Self::MSG_MOVE_STOP_STRAFE(c)
    }
}

impl From<MSG_MOVE_JUMP_Client> for ClientOpcodeMessage {
    fn from(c: MSG_MOVE_JUMP_Client) -> Self {
        Self::MSG_MOVE_JUMP(c)
    }
}

impl From<MSG_MOVE_START_TURN_LEFT_Client> for ClientOpcodeMessage {
    fn from(c: MSG_MOVE_START_TURN_LEFT_Client) -> Self {
        Self::MSG_MOVE_START_TURN_LEFT(c)
    }
}

impl From<MSG_MOVE_START_TURN_RIGHT_Client> for ClientOpcodeMessage {
    fn from(c: MSG_MOVE_START_TURN_RIGHT_Client) -> Self {
        Self::MSG_MOVE_START_TURN_RIGHT(c)
    }
}

impl From<MSG_MOVE_STOP_TURN_Client> for ClientOpcodeMessage {
    fn from(c: MSG_MOVE_STOP_TURN_Client) -> Self {
        Self::MSG_MOVE_STOP_TURN(c)
    }
}

impl From<MSG_MOVE_START_PITCH_UP_Client> for ClientOpcodeMessage {
    fn from(c: MSG_MOVE_START_PITCH_UP_Client) -> Self {
        Self::MSG_MOVE_START_PITCH_UP(c)
    }
}

impl From<MSG_MOVE_START_PITCH_DOWN_Client> for ClientOpcodeMessage {
    fn from(c: MSG_MOVE_START_PITCH_DOWN_Client) -> Self {
        Self::MSG_MOVE_START_PITCH_DOWN(c)
    }
}

impl From<MSG_MOVE_STOP_PITCH_Client> for ClientOpcodeMessage {
    fn from(c: MSG_MOVE_STOP_PITCH_Client) -> Self {
        Self::MSG_MOVE_STOP_PITCH(c)
    }
}

impl From<MSG_MOVE_SET_RUN_MODE_Client> for ClientOpcodeMessage {
    fn from(c: MSG_MOVE_SET_RUN_MODE_Client) -> Self {
        Self::MSG_MOVE_SET_RUN_MODE(c)
    }
}

impl From<MSG_MOVE_SET_WALK_MODE_Client> for ClientOpcodeMessage {
    fn from(c: MSG_MOVE_SET_WALK_MODE_Client) -> Self {
        Self::MSG_MOVE_SET_WALK_MODE(c)
    }
}

impl From<MSG_MOVE_TELEPORT_ACK_Client> for ClientOpcodeMessage {
    fn from(c: MSG_MOVE_TELEPORT_ACK_Client) -> Self {
        Self::MSG_MOVE_TELEPORT_ACK(c)
    }
}

impl From<MSG_MOVE_FALL_LAND_Client> for ClientOpcodeMessage {
    fn from(c: MSG_MOVE_FALL_LAND_Client) -> Self {
        Self::MSG_MOVE_FALL_LAND(c)
    }
}

impl From<MSG_MOVE_START_SWIM_Client> for ClientOpcodeMessage {
    fn from(c: MSG_MOVE_START_SWIM_Client) -> Self {
        Self::MSG_MOVE_START_SWIM(c)
    }
}

impl From<MSG_MOVE_STOP_SWIM_Client> for ClientOpcodeMessage {
    fn from(c: MSG_MOVE_STOP_SWIM_Client) -> Self {
        Self::MSG_MOVE_STOP_SWIM(c)
    }
}

impl From<MSG_MOVE_SET_FACING_Client> for ClientOpcodeMessage {
    fn from(c: MSG_MOVE_SET_FACING_Client) -> Self {
        Self::MSG_MOVE_SET_FACING(c)
    }
}

impl From<MSG_MOVE_SET_PITCH_Client> for ClientOpcodeMessage {
    fn from(c: MSG_MOVE_SET_PITCH_Client) -> Self {
        Self::MSG_MOVE_SET_PITCH(c)
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

impl From<MSG_MOVE_HEARTBEAT_Client> for ClientOpcodeMessage {
    fn from(c: MSG_MOVE_HEARTBEAT_Client) -> Self {
        Self::MSG_MOVE_HEARTBEAT(c)
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
    fn from(_: CMSG_NEXT_CINEMATIC_CAMERA) -> Self {
        Self::CMSG_NEXT_CINEMATIC_CAMERA
    }
}

impl From<CMSG_COMPLETE_CINEMATIC> for ClientOpcodeMessage {
    fn from(_: CMSG_COMPLETE_CINEMATIC) -> Self {
        Self::CMSG_COMPLETE_CINEMATIC
    }
}

impl From<CMSG_TUTORIAL_FLAG> for ClientOpcodeMessage {
    fn from(c: CMSG_TUTORIAL_FLAG) -> Self {
        Self::CMSG_TUTORIAL_FLAG(c)
    }
}

impl From<CMSG_TUTORIAL_CLEAR> for ClientOpcodeMessage {
    fn from(_: CMSG_TUTORIAL_CLEAR) -> Self {
        Self::CMSG_TUTORIAL_CLEAR
    }
}

impl From<CMSG_TUTORIAL_RESET> for ClientOpcodeMessage {
    fn from(_: CMSG_TUTORIAL_RESET) -> Self {
        Self::CMSG_TUTORIAL_RESET
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
    fn from(_: CMSG_BEGIN_TRADE) -> Self {
        Self::CMSG_BEGIN_TRADE
    }
}

impl From<CMSG_BUSY_TRADE> for ClientOpcodeMessage {
    fn from(_: CMSG_BUSY_TRADE) -> Self {
        Self::CMSG_BUSY_TRADE
    }
}

impl From<CMSG_IGNORE_TRADE> for ClientOpcodeMessage {
    fn from(_: CMSG_IGNORE_TRADE) -> Self {
        Self::CMSG_IGNORE_TRADE
    }
}

impl From<CMSG_ACCEPT_TRADE> for ClientOpcodeMessage {
    fn from(c: CMSG_ACCEPT_TRADE) -> Self {
        Self::CMSG_ACCEPT_TRADE(c)
    }
}

impl From<CMSG_UNACCEPT_TRADE> for ClientOpcodeMessage {
    fn from(_: CMSG_UNACCEPT_TRADE) -> Self {
        Self::CMSG_UNACCEPT_TRADE
    }
}

impl From<CMSG_CANCEL_TRADE> for ClientOpcodeMessage {
    fn from(_: CMSG_CANCEL_TRADE) -> Self {
        Self::CMSG_CANCEL_TRADE
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
    fn from(_: CMSG_ATTACKSTOP) -> Self {
        Self::CMSG_ATTACKSTOP
    }
}

impl From<CMSG_REPOP_REQUEST> for ClientOpcodeMessage {
    fn from(_: CMSG_REPOP_REQUEST) -> Self {
        Self::CMSG_REPOP_REQUEST
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
    fn from(_: CMSG_LOOT_MONEY) -> Self {
        Self::CMSG_LOOT_MONEY
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
    fn from(_: CMSG_MOUNTSPECIAL_ANIM) -> Self {
        Self::CMSG_MOUNTSPECIAL_ANIM
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
    fn from(_: CMSG_QUESTGIVER_QUEST_AUTOLAUNCH) -> Self {
        Self::CMSG_QUESTGIVER_QUEST_AUTOLAUNCH
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
    fn from(_: CMSG_QUESTGIVER_CANCEL) -> Self {
        Self::CMSG_QUESTGIVER_CANCEL
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
    fn from(_: CMSG_PLAYED_TIME) -> Self {
        Self::CMSG_PLAYED_TIME
    }
}

impl From<CMSG_QUERY_TIME> for ClientOpcodeMessage {
    fn from(_: CMSG_QUERY_TIME) -> Self {
        Self::CMSG_QUERY_TIME
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

impl From<MSG_LOOKING_FOR_GROUP_Client> for ClientOpcodeMessage {
    fn from(_: MSG_LOOKING_FOR_GROUP_Client) -> Self {
        Self::MSG_LOOKING_FOR_GROUP
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
    fn from(_: CMSG_GMTICKET_GETTICKET) -> Self {
        Self::CMSG_GMTICKET_GETTICKET
    }
}

impl From<MSG_CORPSE_QUERY_Client> for ClientOpcodeMessage {
    fn from(_: MSG_CORPSE_QUERY_Client) -> Self {
        Self::MSG_CORPSE_QUERY
    }
}

impl From<CMSG_GMTICKET_DELETETICKET> for ClientOpcodeMessage {
    fn from(_: CMSG_GMTICKET_DELETETICKET) -> Self {
        Self::CMSG_GMTICKET_DELETETICKET
    }
}

impl From<CMSG_GMTICKET_SYSTEMSTATUS> for ClientOpcodeMessage {
    fn from(_: CMSG_GMTICKET_SYSTEMSTATUS) -> Self {
        Self::CMSG_GMTICKET_SYSTEMSTATUS
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
    fn from(_: CMSG_GUILD_DEL_RANK) -> Self {
        Self::CMSG_GUILD_DEL_RANK
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

impl From<CMSG_BATTLEFIELD_JOIN> for ClientOpcodeMessage {
    fn from(c: CMSG_BATTLEFIELD_JOIN) -> Self {
        Self::CMSG_BATTLEFIELD_JOIN(c)
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
    fn from(_: CMSG_CANCEL_AUTO_REPEAT_SPELL) -> Self {
        Self::CMSG_CANCEL_AUTO_REPEAT_SPELL
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
    fn from(_: CMSG_REQUEST_PET_INFO) -> Self {
        Self::CMSG_REQUEST_PET_INFO
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
    fn from(_: MSG_QUERY_NEXT_MAIL_TIME_Client) -> Self {
        Self::MSG_QUERY_NEXT_MAIL_TIME
    }
}

impl From<CMSG_GROUP_RAID_CONVERT> for ClientOpcodeMessage {
    fn from(_: CMSG_GROUP_RAID_CONVERT) -> Self {
        Self::CMSG_GROUP_RAID_CONVERT
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

impl From<CMSG_MEETINGSTONE_JOIN> for ClientOpcodeMessage {
    fn from(c: CMSG_MEETINGSTONE_JOIN) -> Self {
        Self::CMSG_MEETINGSTONE_JOIN(c)
    }
}

impl From<CMSG_MEETINGSTONE_LEAVE> for ClientOpcodeMessage {
    fn from(_: CMSG_MEETINGSTONE_LEAVE) -> Self {
        Self::CMSG_MEETINGSTONE_LEAVE
    }
}

impl From<CMSG_MEETINGSTONE_INFO> for ClientOpcodeMessage {
    fn from(_: CMSG_MEETINGSTONE_INFO) -> Self {
        Self::CMSG_MEETINGSTONE_INFO
    }
}

impl From<CMSG_CANCEL_GROWTH_AURA> for ClientOpcodeMessage {
    fn from(_: CMSG_CANCEL_GROWTH_AURA) -> Self {
        Self::CMSG_CANCEL_GROWTH_AURA
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

impl From<MSG_TALENT_WIPE_CONFIRM_Client> for ClientOpcodeMessage {
    fn from(c: MSG_TALENT_WIPE_CONFIRM_Client) -> Self {
        Self::MSG_TALENT_WIPE_CONFIRM(c)
    }
}

impl From<CMSG_SUMMON_RESPONSE> for ClientOpcodeMessage {
    fn from(c: CMSG_SUMMON_RESPONSE) -> Self {
        Self::CMSG_SUMMON_RESPONSE(c)
    }
}

impl From<CMSG_SELF_RES> for ClientOpcodeMessage {
    fn from(_: CMSG_SELF_RES) -> Self {
        Self::CMSG_SELF_RES
    }
}

impl From<CMSG_TOGGLE_HELM> for ClientOpcodeMessage {
    fn from(_: CMSG_TOGGLE_HELM) -> Self {
        Self::CMSG_TOGGLE_HELM
    }
}

impl From<CMSG_TOGGLE_CLOAK> for ClientOpcodeMessage {
    fn from(_: CMSG_TOGGLE_CLOAK) -> Self {
        Self::CMSG_TOGGLE_CLOAK
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
    fn from(_: CMSG_REQUEST_RAID_INFO) -> Self {
        Self::CMSG_REQUEST_RAID_INFO
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
    fn from(_: CMSG_BATTLEFIELD_STATUS) -> Self {
        Self::CMSG_BATTLEFIELD_STATUS
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
    fn from(_: MSG_PVP_LOG_DATA_Client) -> Self {
        Self::MSG_PVP_LOG_DATA
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

impl From<CMSG_WARDEN_DATA> for ClientOpcodeMessage {
    fn from(c: CMSG_WARDEN_DATA) -> Self {
        Self::CMSG_WARDEN_DATA(c)
    }
}

impl From<MSG_BATTLEGROUND_PLAYER_POSITIONS_Client> for ClientOpcodeMessage {
    fn from(_: MSG_BATTLEGROUND_PLAYER_POSITIONS_Client) -> Self {
        Self::MSG_BATTLEGROUND_PLAYER_POSITIONS
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
    fn from(_: CMSG_RESET_INSTANCES) -> Self {
        Self::CMSG_RESET_INSTANCES
    }
}

impl From<MSG_RAID_TARGET_UPDATE_Client> for ClientOpcodeMessage {
    fn from(c: MSG_RAID_TARGET_UPDATE_Client) -> Self {
        Self::MSG_RAID_TARGET_UPDATE(c)
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

use crate::vanilla::SMSG_CHAR_CREATE;
use crate::vanilla::SMSG_CHAR_ENUM;
use crate::vanilla::SMSG_CHAR_DELETE;
use crate::vanilla::SMSG_NEW_WORLD;
use crate::vanilla::SMSG_TRANSFER_PENDING;
use crate::vanilla::SMSG_TRANSFER_ABORTED;
use crate::vanilla::SMSG_CHARACTER_LOGIN_FAILED;
use crate::vanilla::SMSG_LOGIN_SETTIMESPEED;
use crate::vanilla::SMSG_LOGOUT_RESPONSE;
use crate::vanilla::SMSG_LOGOUT_COMPLETE;
use crate::vanilla::SMSG_LOGOUT_CANCEL_ACK;
use crate::vanilla::SMSG_NAME_QUERY_RESPONSE;
use crate::vanilla::SMSG_PET_NAME_QUERY_RESPONSE;
use crate::vanilla::SMSG_GUILD_QUERY_RESPONSE;
use crate::vanilla::SMSG_ITEM_QUERY_SINGLE_RESPONSE;
use crate::vanilla::SMSG_PAGE_TEXT_QUERY_RESPONSE;
use crate::vanilla::SMSG_QUEST_QUERY_RESPONSE;
use crate::vanilla::SMSG_GAMEOBJECT_QUERY_RESPONSE;
use crate::vanilla::SMSG_CREATURE_QUERY_RESPONSE;
use crate::vanilla::SMSG_WHO;
use crate::vanilla::SMSG_WHOIS;
use crate::vanilla::SMSG_FRIEND_LIST;
use crate::vanilla::SMSG_FRIEND_STATUS;
use crate::vanilla::SMSG_IGNORE_LIST;
use crate::vanilla::SMSG_GROUP_INVITE;
use crate::vanilla::SMSG_GROUP_DECLINE;
use crate::vanilla::SMSG_GROUP_UNINVITE;
use crate::vanilla::SMSG_GROUP_SET_LEADER;
use crate::vanilla::SMSG_GROUP_DESTROYED;
use crate::vanilla::SMSG_GROUP_LIST;
use crate::vanilla::SMSG_PARTY_MEMBER_STATS;
use crate::vanilla::SMSG_PARTY_COMMAND_RESULT;
use crate::vanilla::SMSG_GUILD_INVITE;
use crate::vanilla::SMSG_GUILD_INFO;
use crate::vanilla::SMSG_GUILD_ROSTER;
use crate::vanilla::SMSG_GUILD_EVENT;
use crate::vanilla::SMSG_GUILD_COMMAND_RESULT;
use crate::vanilla::SMSG_MESSAGECHAT;
use crate::vanilla::SMSG_CHANNEL_NOTIFY;
use crate::vanilla::SMSG_CHANNEL_LIST;
use crate::vanilla::SMSG_UPDATE_OBJECT;
use crate::vanilla::SMSG_DESTROY_OBJECT;
use crate::vanilla::SMSG_READ_ITEM_OK;
use crate::vanilla::SMSG_READ_ITEM_FAILED;
use crate::vanilla::SMSG_ITEM_COOLDOWN;
use crate::vanilla::SMSG_GAMEOBJECT_CUSTOM_ANIM;
use crate::vanilla::MSG_MOVE_START_FORWARD_Server;
use crate::vanilla::MSG_MOVE_START_BACKWARD_Server;
use crate::vanilla::MSG_MOVE_STOP_Server;
use crate::vanilla::MSG_MOVE_START_STRAFE_LEFT_Server;
use crate::vanilla::MSG_MOVE_START_STRAFE_RIGHT_Server;
use crate::vanilla::MSG_MOVE_STOP_STRAFE_Server;
use crate::vanilla::MSG_MOVE_JUMP_Server;
use crate::vanilla::MSG_MOVE_START_TURN_LEFT_Server;
use crate::vanilla::MSG_MOVE_START_TURN_RIGHT_Server;
use crate::vanilla::MSG_MOVE_STOP_TURN_Server;
use crate::vanilla::MSG_MOVE_START_PITCH_UP_Server;
use crate::vanilla::MSG_MOVE_START_PITCH_DOWN_Server;
use crate::vanilla::MSG_MOVE_STOP_PITCH_Server;
use crate::vanilla::MSG_MOVE_SET_RUN_MODE_Server;
use crate::vanilla::MSG_MOVE_SET_WALK_MODE_Server;
use crate::vanilla::MSG_MOVE_TELEPORT_ACK_Server;
use crate::vanilla::MSG_MOVE_FALL_LAND_Server;
use crate::vanilla::MSG_MOVE_START_SWIM_Server;
use crate::vanilla::MSG_MOVE_STOP_SWIM_Server;
use crate::vanilla::MSG_MOVE_SET_FACING_Server;
use crate::vanilla::MSG_MOVE_SET_PITCH_Server;
use crate::vanilla::SMSG_MONSTER_MOVE;
use crate::vanilla::SMSG_MOVE_WATER_WALK;
use crate::vanilla::SMSG_MOVE_LAND_WALK;
use crate::vanilla::SMSG_FORCE_RUN_SPEED_CHANGE;
use crate::vanilla::SMSG_FORCE_RUN_BACK_SPEED_CHANGE;
use crate::vanilla::SMSG_FORCE_SWIM_SPEED_CHANGE;
use crate::vanilla::SMSG_FORCE_MOVE_ROOT;
use crate::vanilla::SMSG_FORCE_MOVE_UNROOT;
use crate::vanilla::MSG_MOVE_HEARTBEAT_Server;
use crate::vanilla::SMSG_MOVE_KNOCK_BACK;
use crate::vanilla::SMSG_MOVE_FEATHER_FALL;
use crate::vanilla::SMSG_MOVE_NORMAL_FALL;
use crate::vanilla::SMSG_MOVE_SET_HOVER;
use crate::vanilla::SMSG_MOVE_UNSET_HOVER;
use crate::vanilla::SMSG_TRIGGER_CINEMATIC;
use crate::vanilla::SMSG_TUTORIAL_FLAGS;
use crate::vanilla::SMSG_EMOTE;
use crate::vanilla::SMSG_TEXT_EMOTE;
use crate::vanilla::SMSG_INVENTORY_CHANGE_FAILURE;
use crate::vanilla::SMSG_OPEN_CONTAINER;
use crate::vanilla::SMSG_INSPECT;
use crate::vanilla::SMSG_TRADE_STATUS;
use crate::vanilla::SMSG_TRADE_STATUS_EXTENDED;
use crate::vanilla::SMSG_INITIALIZE_FACTIONS;
use crate::vanilla::SMSG_SET_FACTION_VISIBLE;
use crate::vanilla::SMSG_SET_FACTION_STANDING;
use crate::vanilla::SMSG_SET_PROFICIENCY;
use crate::vanilla::SMSG_ACTION_BUTTONS;
use crate::vanilla::SMSG_INITIAL_SPELLS;
use crate::vanilla::SMSG_LEARNED_SPELL;
use crate::vanilla::SMSG_SUPERCEDED_SPELL;
use crate::vanilla::SMSG_CAST_RESULT;
use crate::vanilla::SMSG_SPELL_START;
use crate::vanilla::SMSG_SPELL_GO;
use crate::vanilla::SMSG_SPELL_FAILURE;
use crate::vanilla::SMSG_SPELL_COOLDOWN;
use crate::vanilla::SMSG_COOLDOWN_EVENT;
use crate::vanilla::SMSG_UPDATE_AURA_DURATION;
use crate::vanilla::SMSG_PET_CAST_FAILED;
use crate::vanilla::MSG_CHANNEL_START_Server;
use crate::vanilla::MSG_CHANNEL_UPDATE_Server;
use crate::vanilla::SMSG_AI_REACTION;
use crate::vanilla::SMSG_ATTACKSTART;
use crate::vanilla::SMSG_ATTACKSTOP;
use crate::vanilla::SMSG_ATTACKSWING_NOTINRANGE;
use crate::vanilla::SMSG_ATTACKSWING_BADFACING;
use crate::vanilla::SMSG_ATTACKSWING_NOTSTANDING;
use crate::vanilla::SMSG_ATTACKSWING_DEADTARGET;
use crate::vanilla::SMSG_ATTACKSWING_CANT_ATTACK;
use crate::vanilla::SMSG_ATTACKERSTATEUPDATE;
use crate::vanilla::SMSG_CANCEL_COMBAT;
use crate::vanilla::SMSG_SPELLHEALLOG;
use crate::vanilla::SMSG_SPELLENERGIZELOG;
use crate::vanilla::SMSG_BINDPOINTUPDATE;
use crate::vanilla::SMSG_PLAYERBOUND;
use crate::vanilla::SMSG_CLIENT_CONTROL_UPDATE;
use crate::vanilla::SMSG_RESURRECT_REQUEST;
use crate::vanilla::SMSG_LOOT_RESPONSE;
use crate::vanilla::SMSG_LOOT_RELEASE_RESPONSE;
use crate::vanilla::SMSG_LOOT_REMOVED;
use crate::vanilla::SMSG_LOOT_MONEY_NOTIFY;
use crate::vanilla::SMSG_LOOT_CLEAR_MONEY;
use crate::vanilla::SMSG_ITEM_PUSH_RESULT;
use crate::vanilla::SMSG_DUEL_REQUESTED;
use crate::vanilla::SMSG_DUEL_OUTOFBOUNDS;
use crate::vanilla::SMSG_DUEL_INBOUNDS;
use crate::vanilla::SMSG_DUEL_COMPLETE;
use crate::vanilla::SMSG_DUEL_WINNER;
use crate::vanilla::SMSG_MOUNTRESULT;
use crate::vanilla::SMSG_DISMOUNTRESULT;
use crate::vanilla::SMSG_MOUNTSPECIAL_ANIM;
use crate::vanilla::SMSG_PET_TAME_FAILURE;
use crate::vanilla::SMSG_PET_NAME_INVALID;
use crate::vanilla::SMSG_PET_SPELLS;
use crate::vanilla::SMSG_PET_MODE;
use crate::vanilla::SMSG_GOSSIP_MESSAGE;
use crate::vanilla::SMSG_GOSSIP_COMPLETE;
use crate::vanilla::SMSG_NPC_TEXT_UPDATE;
use crate::vanilla::SMSG_QUESTGIVER_STATUS;
use crate::vanilla::SMSG_QUESTGIVER_QUEST_LIST;
use crate::vanilla::SMSG_QUESTGIVER_QUEST_DETAILS;
use crate::vanilla::SMSG_QUESTGIVER_REQUEST_ITEMS;
use crate::vanilla::SMSG_QUESTGIVER_OFFER_REWARD;
use crate::vanilla::SMSG_QUESTGIVER_QUEST_INVALID;
use crate::vanilla::SMSG_QUESTGIVER_QUEST_COMPLETE;
use crate::vanilla::SMSG_QUESTGIVER_QUEST_FAILED;
use crate::vanilla::SMSG_QUESTLOG_FULL;
use crate::vanilla::SMSG_QUESTUPDATE_FAILED;
use crate::vanilla::SMSG_QUESTUPDATE_FAILEDTIMER;
use crate::vanilla::SMSG_QUESTUPDATE_COMPLETE;
use crate::vanilla::SMSG_QUESTUPDATE_ADD_KILL;
use crate::vanilla::SMSG_QUESTUPDATE_ADD_ITEM;
use crate::vanilla::SMSG_QUEST_CONFIRM_ACCEPT;
use crate::vanilla::SMSG_LIST_INVENTORY;
use crate::vanilla::SMSG_SELL_ITEM;
use crate::vanilla::SMSG_BUY_ITEM;
use crate::vanilla::SMSG_BUY_FAILED;
use crate::vanilla::SMSG_SHOWTAXINODES;
use crate::vanilla::SMSG_TAXINODE_STATUS;
use crate::vanilla::SMSG_ACTIVATETAXIREPLY;
use crate::vanilla::SMSG_NEW_TAXI_PATH;
use crate::vanilla::SMSG_TRAINER_LIST;
use crate::vanilla::SMSG_TRAINER_BUY_SUCCEEDED;
use crate::vanilla::SMSG_TRAINER_BUY_FAILED;
use crate::vanilla::SMSG_SHOW_BANK;
use crate::vanilla::SMSG_BUY_BANK_SLOT_RESULT;
use crate::vanilla::SMSG_PETITION_SHOWLIST;
use crate::vanilla::SMSG_PETITION_SHOW_SIGNATURES;
use crate::vanilla::SMSG_PETITION_SIGN_RESULTS;
use crate::vanilla::SMSG_TURN_IN_PETITION_RESULTS;
use crate::vanilla::SMSG_PETITION_QUERY_RESPONSE;
use crate::vanilla::SMSG_FISH_NOT_HOOKED;
use crate::vanilla::SMSG_FISH_ESCAPED;
use crate::vanilla::SMSG_NOTIFICATION;
use crate::vanilla::SMSG_PLAYED_TIME;
use crate::vanilla::SMSG_QUERY_TIME_RESPONSE;
use crate::vanilla::SMSG_LOG_XPGAIN;
use crate::vanilla::SMSG_LEVELUP_INFO;
use crate::vanilla::MSG_MINIMAP_PING_Server;
use crate::vanilla::SMSG_RESISTLOG;
use crate::vanilla::SMSG_ENCHANTMENTLOG;
use crate::vanilla::SMSG_START_MIRROR_TIMER;
use crate::vanilla::SMSG_PAUSE_MIRROR_TIMER;
use crate::vanilla::SMSG_STOP_MIRROR_TIMER;
use crate::vanilla::SMSG_PONG;
use crate::vanilla::SMSG_CLEAR_COOLDOWN;
use crate::vanilla::SMSG_GAMEOBJECT_PAGETEXT;
use crate::vanilla::SMSG_SPELL_DELAYED;
use crate::vanilla::SMSG_ITEM_TIME_UPDATE;
use crate::vanilla::SMSG_ITEM_ENCHANT_TIME_UPDATE;
use crate::vanilla::SMSG_AUTH_CHALLENGE;
use crate::vanilla::SMSG_AUTH_RESPONSE;
use crate::vanilla::MSG_SAVE_GUILD_EMBLEM_Server;
use crate::vanilla::SMSG_PLAY_SPELL_VISUAL;
use crate::vanilla::SMSG_PARTYKILLLOG;
use crate::vanilla::SMSG_COMPRESSED_UPDATE_OBJECT;
use crate::vanilla::SMSG_PLAY_SPELL_IMPACT;
use crate::vanilla::SMSG_EXPLORATION_EXPERIENCE;
use crate::vanilla::MSG_RANDOM_ROLL_Server;
use crate::vanilla::SMSG_ENVIRONMENTAL_DAMAGE_LOG;
use crate::vanilla::MSG_LOOKING_FOR_GROUP_Server;
use crate::vanilla::SMSG_REMOVED_SPELL;
use crate::vanilla::SMSG_GMTICKET_CREATE;
use crate::vanilla::SMSG_GMTICKET_UPDATETEXT;
use crate::vanilla::SMSG_ACCOUNT_DATA_TIMES;
use crate::vanilla::SMSG_GMTICKET_GETTICKET;
use crate::vanilla::SMSG_GAMEOBJECT_SPAWN_ANIM;
use crate::vanilla::SMSG_GAMEOBJECT_DESPAWN_ANIM;
use crate::vanilla::MSG_CORPSE_QUERY_Server;
use crate::vanilla::SMSG_GMTICKET_DELETETICKET;
use crate::vanilla::SMSG_CHAT_WRONG_FACTION;
use crate::vanilla::SMSG_GMTICKET_SYSTEMSTATUS;
use crate::vanilla::SMSG_SET_REST_START;
use crate::vanilla::SMSG_SPIRIT_HEALER_CONFIRM;
use crate::vanilla::SMSG_GOSSIP_POI;
use crate::vanilla::SMSG_LOGIN_VERIFY_WORLD;
use crate::vanilla::SMSG_SEND_MAIL_RESULT;
use crate::vanilla::SMSG_MAIL_LIST_RESULT;
use crate::vanilla::SMSG_BATTLEFIELD_LIST;
use crate::vanilla::SMSG_ITEM_TEXT_QUERY_RESPONSE;
use crate::vanilla::SMSG_SPELLLOGMISS;
use crate::vanilla::SMSG_SPELLLOGEXECUTE;
use crate::vanilla::SMSG_PERIODICAURALOG;
use crate::vanilla::SMSG_SPELLDAMAGESHIELD;
use crate::vanilla::SMSG_SPELLNONMELEEDAMAGELOG;
use crate::vanilla::SMSG_ZONE_UNDER_ATTACK;
use crate::vanilla::MSG_AUCTION_HELLO_Server;
use crate::vanilla::SMSG_AUCTION_COMMAND_RESULT;
use crate::vanilla::SMSG_AUCTION_LIST_RESULT;
use crate::vanilla::SMSG_AUCTION_OWNER_LIST_RESULT;
use crate::vanilla::SMSG_AUCTION_BIDDER_NOTIFICATION;
use crate::vanilla::SMSG_AUCTION_OWNER_NOTIFICATION;
use crate::vanilla::SMSG_PROCRESIST;
use crate::vanilla::SMSG_DISPEL_FAILED;
use crate::vanilla::SMSG_SPELLORDAMAGE_IMMUNE;
use crate::vanilla::SMSG_AUCTION_BIDDER_LIST_RESULT;
use crate::vanilla::SMSG_SET_FLAT_SPELL_MODIFIER;
use crate::vanilla::SMSG_SET_PCT_SPELL_MODIFIER;
use crate::vanilla::SMSG_CORPSE_RECLAIM_DELAY;
use crate::vanilla::MSG_LIST_STABLED_PETS_Server;
use crate::vanilla::SMSG_STABLE_RESULT;
use crate::vanilla::SMSG_PLAY_MUSIC;
use crate::vanilla::SMSG_PLAY_OBJECT_SOUND;
use crate::vanilla::SMSG_SPELLDISPELLOG;
use crate::vanilla::MSG_QUERY_NEXT_MAIL_TIME_Server;
use crate::vanilla::SMSG_RECEIVED_MAIL;
use crate::vanilla::SMSG_RAID_GROUP_ONLY;
use crate::vanilla::SMSG_PVP_CREDIT;
use crate::vanilla::SMSG_AUCTION_REMOVED_NOTIFICATION;
use crate::vanilla::SMSG_SERVER_MESSAGE;
use crate::vanilla::SMSG_MEETINGSTONE_SETQUEUE;
use crate::vanilla::SMSG_MEETINGSTONE_COMPLETE;
use crate::vanilla::SMSG_MEETINGSTONE_IN_PROGRESS;
use crate::vanilla::SMSG_MEETINGSTONE_MEMBER_ADDED;
use crate::vanilla::SMSG_CANCEL_AUTO_REPEAT;
use crate::vanilla::SMSG_STANDSTATE_UPDATE;
use crate::vanilla::SMSG_LOOT_ALL_PASSED;
use crate::vanilla::SMSG_LOOT_ROLL_WON;
use crate::vanilla::SMSG_LOOT_START_ROLL;
use crate::vanilla::SMSG_LOOT_ROLL;
use crate::vanilla::SMSG_LOOT_MASTER_LIST;
use crate::vanilla::SMSG_SET_FORCED_REACTIONS;
use crate::vanilla::SMSG_SPELL_FAILED_OTHER;
use crate::vanilla::SMSG_GAMEOBJECT_RESET_STATE;
use crate::vanilla::SMSG_CHAT_PLAYER_NOT_FOUND;
use crate::vanilla::MSG_TALENT_WIPE_CONFIRM_Server;
use crate::vanilla::SMSG_SUMMON_REQUEST;
use crate::vanilla::SMSG_MONSTER_MOVE_TRANSPORT;
use crate::vanilla::SMSG_PET_BROKEN;
use crate::vanilla::MSG_MOVE_FEATHER_FALL_Server;
use crate::vanilla::SMSG_FEIGN_DEATH_RESISTED;
use crate::vanilla::SMSG_DUEL_COUNTDOWN;
use crate::vanilla::SMSG_AREA_TRIGGER_MESSAGE;
use crate::vanilla::SMSG_MEETINGSTONE_JOINFAILED;
use crate::vanilla::SMSG_PLAYER_SKINNED;
use crate::vanilla::SMSG_DURABILITY_DAMAGE_DEATH;
use crate::vanilla::SMSG_INIT_WORLD_STATES;
use crate::vanilla::SMSG_UPDATE_WORLD_STATE;
use crate::vanilla::SMSG_ITEM_NAME_QUERY_RESPONSE;
use crate::vanilla::SMSG_PET_ACTION_FEEDBACK;
use crate::vanilla::SMSG_CHAR_RENAME;
use crate::vanilla::SMSG_INSTANCE_SAVE_CREATED;
use crate::vanilla::SMSG_RAID_INSTANCE_INFO;
use crate::vanilla::SMSG_PLAY_SOUND;
use crate::vanilla::SMSG_BATTLEFIELD_STATUS;
use crate::vanilla::MSG_INSPECT_HONOR_STATS_Server;
use crate::vanilla::SMSG_FORCE_WALK_SPEED_CHANGE;
use crate::vanilla::SMSG_FORCE_SWIM_BACK_SPEED_CHANGE;
use crate::vanilla::SMSG_FORCE_TURN_RATE_CHANGE;
use crate::vanilla::MSG_PVP_LOG_DATA_Server;
use crate::vanilla::SMSG_AREA_SPIRIT_HEALER_TIME;
use crate::vanilla::SMSG_WARDEN_DATA;
use crate::vanilla::SMSG_GROUP_JOINED_BATTLEGROUND;
use crate::vanilla::MSG_BATTLEGROUND_PLAYER_POSITIONS_Server;
use crate::vanilla::SMSG_BINDER_CONFIRM;
use crate::vanilla::SMSG_BATTLEGROUND_PLAYER_JOINED;
use crate::vanilla::SMSG_BATTLEGROUND_PLAYER_LEFT;
use crate::vanilla::SMSG_ADDON_INFO;
use crate::vanilla::SMSG_PET_UNLEARN_CONFIRM;
use crate::vanilla::SMSG_PARTY_MEMBER_STATS_FULL;
use crate::vanilla::SMSG_WEATHER;
use crate::vanilla::SMSG_RAID_INSTANCE_MESSAGE;
use crate::vanilla::SMSG_COMPRESSED_MOVES;
use crate::vanilla::SMSG_CHAT_RESTRICTED;
use crate::vanilla::SMSG_SPLINE_SET_RUN_SPEED;
use crate::vanilla::SMSG_SPLINE_SET_RUN_BACK_SPEED;
use crate::vanilla::SMSG_SPLINE_SET_SWIM_SPEED;
use crate::vanilla::SMSG_SPLINE_SET_WALK_SPEED;
use crate::vanilla::SMSG_SPLINE_SET_SWIM_BACK_SPEED;
use crate::vanilla::SMSG_SPLINE_SET_TURN_RATE;
use crate::vanilla::SMSG_SPLINE_MOVE_UNROOT;
use crate::vanilla::SMSG_SPLINE_MOVE_FEATHER_FALL;
use crate::vanilla::SMSG_SPLINE_MOVE_NORMAL_FALL;
use crate::vanilla::SMSG_SPLINE_MOVE_SET_HOVER;
use crate::vanilla::SMSG_SPLINE_MOVE_UNSET_HOVER;
use crate::vanilla::SMSG_SPLINE_MOVE_WATER_WALK;
use crate::vanilla::SMSG_SPLINE_MOVE_LAND_WALK;
use crate::vanilla::SMSG_SPLINE_MOVE_START_SWIM;
use crate::vanilla::SMSG_SPLINE_MOVE_STOP_SWIM;
use crate::vanilla::SMSG_SPLINE_MOVE_SET_RUN_MODE;
use crate::vanilla::SMSG_SPLINE_MOVE_SET_WALK_MODE;
use crate::vanilla::MSG_MOVE_TIME_SKIPPED_Server;
use crate::vanilla::SMSG_SPLINE_MOVE_ROOT;
use crate::vanilla::SMSG_INVALIDATE_PLAYER;
use crate::vanilla::SMSG_INSTANCE_RESET;
use crate::vanilla::SMSG_INSTANCE_RESET_FAILED;
use crate::vanilla::SMSG_UPDATE_LAST_INSTANCE;
use crate::vanilla::MSG_RAID_TARGET_UPDATE_Server;
use crate::vanilla::MSG_RAID_READY_CHECK_Server;
use crate::vanilla::SMSG_PET_ACTION_SOUND;
use crate::vanilla::SMSG_PET_DISMISS_SOUND;
use crate::vanilla::SMSG_GM_TICKET_STATUS_UPDATE;
use crate::vanilla::SMSG_UPDATE_INSTANCE_OWNERSHIP;
use crate::vanilla::SMSG_SPELLINSTAKILLLOG;
use crate::vanilla::SMSG_SPELL_UPDATE_CHAIN_TARGETS;
use crate::vanilla::SMSG_EXPECTED_SPAM_RECORDS;
use crate::vanilla::SMSG_DEFENSE_MESSAGE;

#[derive(Debug, Clone, PartialEq)]
pub enum ServerOpcodeMessage {
    MSG_MOVE_WORLDPORT_ACK,
    MSG_PETITION_DECLINE(MSG_PETITION_DECLINE),
    MSG_TABARDVENDOR_ACTIVATE(MSG_TABARDVENDOR_ACTIVATE),
    MSG_QUEST_PUSH_RESULT(MSG_QUEST_PUSH_RESULT),
    MSG_MOVE_WATER_WALK(MSG_MOVE_WATER_WALK),
    MSG_PETITION_RENAME(MSG_PETITION_RENAME),
    SMSG_CHAR_CREATE(SMSG_CHAR_CREATE),
    SMSG_CHAR_ENUM(SMSG_CHAR_ENUM),
    SMSG_CHAR_DELETE(SMSG_CHAR_DELETE),
    SMSG_NEW_WORLD(SMSG_NEW_WORLD),
    SMSG_TRANSFER_PENDING(SMSG_TRANSFER_PENDING),
    SMSG_TRANSFER_ABORTED(SMSG_TRANSFER_ABORTED),
    SMSG_CHARACTER_LOGIN_FAILED(SMSG_CHARACTER_LOGIN_FAILED),
    SMSG_LOGIN_SETTIMESPEED(SMSG_LOGIN_SETTIMESPEED),
    SMSG_LOGOUT_RESPONSE(SMSG_LOGOUT_RESPONSE),
    SMSG_LOGOUT_COMPLETE,
    SMSG_LOGOUT_CANCEL_ACK,
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
    SMSG_FRIEND_LIST(SMSG_FRIEND_LIST),
    SMSG_FRIEND_STATUS(SMSG_FRIEND_STATUS),
    SMSG_IGNORE_LIST(SMSG_IGNORE_LIST),
    SMSG_GROUP_INVITE(SMSG_GROUP_INVITE),
    SMSG_GROUP_DECLINE(SMSG_GROUP_DECLINE),
    SMSG_GROUP_UNINVITE,
    SMSG_GROUP_SET_LEADER(SMSG_GROUP_SET_LEADER),
    SMSG_GROUP_DESTROYED,
    SMSG_GROUP_LIST(SMSG_GROUP_LIST),
    SMSG_PARTY_MEMBER_STATS(SMSG_PARTY_MEMBER_STATS),
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
    MSG_MOVE_START_FORWARD(MSG_MOVE_START_FORWARD_Server),
    MSG_MOVE_START_BACKWARD(MSG_MOVE_START_BACKWARD_Server),
    MSG_MOVE_STOP(MSG_MOVE_STOP_Server),
    MSG_MOVE_START_STRAFE_LEFT(MSG_MOVE_START_STRAFE_LEFT_Server),
    MSG_MOVE_START_STRAFE_RIGHT(MSG_MOVE_START_STRAFE_RIGHT_Server),
    MSG_MOVE_STOP_STRAFE(MSG_MOVE_STOP_STRAFE_Server),
    MSG_MOVE_JUMP(MSG_MOVE_JUMP_Server),
    MSG_MOVE_START_TURN_LEFT(MSG_MOVE_START_TURN_LEFT_Server),
    MSG_MOVE_START_TURN_RIGHT(MSG_MOVE_START_TURN_RIGHT_Server),
    MSG_MOVE_STOP_TURN(MSG_MOVE_STOP_TURN_Server),
    MSG_MOVE_START_PITCH_UP(MSG_MOVE_START_PITCH_UP_Server),
    MSG_MOVE_START_PITCH_DOWN(MSG_MOVE_START_PITCH_DOWN_Server),
    MSG_MOVE_STOP_PITCH(MSG_MOVE_STOP_PITCH_Server),
    MSG_MOVE_SET_RUN_MODE(MSG_MOVE_SET_RUN_MODE_Server),
    MSG_MOVE_SET_WALK_MODE(MSG_MOVE_SET_WALK_MODE_Server),
    MSG_MOVE_TELEPORT_ACK(MSG_MOVE_TELEPORT_ACK_Server),
    MSG_MOVE_FALL_LAND(MSG_MOVE_FALL_LAND_Server),
    MSG_MOVE_START_SWIM(MSG_MOVE_START_SWIM_Server),
    MSG_MOVE_STOP_SWIM(MSG_MOVE_STOP_SWIM_Server),
    MSG_MOVE_SET_FACING(MSG_MOVE_SET_FACING_Server),
    MSG_MOVE_SET_PITCH(MSG_MOVE_SET_PITCH_Server),
    SMSG_MONSTER_MOVE(SMSG_MONSTER_MOVE),
    SMSG_MOVE_WATER_WALK(SMSG_MOVE_WATER_WALK),
    SMSG_MOVE_LAND_WALK(SMSG_MOVE_LAND_WALK),
    SMSG_FORCE_RUN_SPEED_CHANGE(SMSG_FORCE_RUN_SPEED_CHANGE),
    SMSG_FORCE_RUN_BACK_SPEED_CHANGE(SMSG_FORCE_RUN_BACK_SPEED_CHANGE),
    SMSG_FORCE_SWIM_SPEED_CHANGE(SMSG_FORCE_SWIM_SPEED_CHANGE),
    SMSG_FORCE_MOVE_ROOT(SMSG_FORCE_MOVE_ROOT),
    SMSG_FORCE_MOVE_UNROOT(SMSG_FORCE_MOVE_UNROOT),
    MSG_MOVE_HEARTBEAT(MSG_MOVE_HEARTBEAT_Server),
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
    SMSG_OPEN_CONTAINER(SMSG_OPEN_CONTAINER),
    SMSG_INSPECT(SMSG_INSPECT),
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
    SMSG_CAST_RESULT(SMSG_CAST_RESULT),
    SMSG_SPELL_START(SMSG_SPELL_START),
    SMSG_SPELL_GO(SMSG_SPELL_GO),
    SMSG_SPELL_FAILURE(SMSG_SPELL_FAILURE),
    SMSG_SPELL_COOLDOWN(SMSG_SPELL_COOLDOWN),
    SMSG_COOLDOWN_EVENT(SMSG_COOLDOWN_EVENT),
    SMSG_UPDATE_AURA_DURATION(SMSG_UPDATE_AURA_DURATION),
    SMSG_PET_CAST_FAILED(SMSG_PET_CAST_FAILED),
    MSG_CHANNEL_START(MSG_CHANNEL_START_Server),
    MSG_CHANNEL_UPDATE(MSG_CHANNEL_UPDATE_Server),
    SMSG_AI_REACTION(SMSG_AI_REACTION),
    SMSG_ATTACKSTART(SMSG_ATTACKSTART),
    SMSG_ATTACKSTOP(SMSG_ATTACKSTOP),
    SMSG_ATTACKSWING_NOTINRANGE,
    SMSG_ATTACKSWING_BADFACING,
    SMSG_ATTACKSWING_NOTSTANDING,
    SMSG_ATTACKSWING_DEADTARGET,
    SMSG_ATTACKSWING_CANT_ATTACK,
    SMSG_ATTACKERSTATEUPDATE(SMSG_ATTACKERSTATEUPDATE),
    SMSG_CANCEL_COMBAT,
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
    SMSG_LOOT_CLEAR_MONEY,
    SMSG_ITEM_PUSH_RESULT(SMSG_ITEM_PUSH_RESULT),
    SMSG_DUEL_REQUESTED(SMSG_DUEL_REQUESTED),
    SMSG_DUEL_OUTOFBOUNDS,
    SMSG_DUEL_INBOUNDS,
    SMSG_DUEL_COMPLETE(SMSG_DUEL_COMPLETE),
    SMSG_DUEL_WINNER(SMSG_DUEL_WINNER),
    SMSG_MOUNTRESULT(SMSG_MOUNTRESULT),
    SMSG_DISMOUNTRESULT(SMSG_DISMOUNTRESULT),
    SMSG_MOUNTSPECIAL_ANIM(SMSG_MOUNTSPECIAL_ANIM),
    SMSG_PET_TAME_FAILURE(SMSG_PET_TAME_FAILURE),
    SMSG_PET_NAME_INVALID,
    SMSG_PET_SPELLS(SMSG_PET_SPELLS),
    SMSG_PET_MODE(SMSG_PET_MODE),
    SMSG_GOSSIP_MESSAGE(SMSG_GOSSIP_MESSAGE),
    SMSG_GOSSIP_COMPLETE,
    SMSG_NPC_TEXT_UPDATE(SMSG_NPC_TEXT_UPDATE),
    SMSG_QUESTGIVER_STATUS(SMSG_QUESTGIVER_STATUS),
    SMSG_QUESTGIVER_QUEST_LIST(SMSG_QUESTGIVER_QUEST_LIST),
    SMSG_QUESTGIVER_QUEST_DETAILS(SMSG_QUESTGIVER_QUEST_DETAILS),
    SMSG_QUESTGIVER_REQUEST_ITEMS(SMSG_QUESTGIVER_REQUEST_ITEMS),
    SMSG_QUESTGIVER_OFFER_REWARD(SMSG_QUESTGIVER_OFFER_REWARD),
    SMSG_QUESTGIVER_QUEST_INVALID(SMSG_QUESTGIVER_QUEST_INVALID),
    SMSG_QUESTGIVER_QUEST_COMPLETE(SMSG_QUESTGIVER_QUEST_COMPLETE),
    SMSG_QUESTGIVER_QUEST_FAILED(SMSG_QUESTGIVER_QUEST_FAILED),
    SMSG_QUESTLOG_FULL,
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
    SMSG_NEW_TAXI_PATH,
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
    SMSG_FISH_NOT_HOOKED,
    SMSG_FISH_ESCAPED,
    SMSG_NOTIFICATION(SMSG_NOTIFICATION),
    SMSG_PLAYED_TIME(SMSG_PLAYED_TIME),
    SMSG_QUERY_TIME_RESPONSE(SMSG_QUERY_TIME_RESPONSE),
    SMSG_LOG_XPGAIN(SMSG_LOG_XPGAIN),
    SMSG_LEVELUP_INFO(SMSG_LEVELUP_INFO),
    MSG_MINIMAP_PING(MSG_MINIMAP_PING_Server),
    SMSG_RESISTLOG(SMSG_RESISTLOG),
    SMSG_ENCHANTMENTLOG(SMSG_ENCHANTMENTLOG),
    SMSG_START_MIRROR_TIMER(SMSG_START_MIRROR_TIMER),
    SMSG_PAUSE_MIRROR_TIMER(SMSG_PAUSE_MIRROR_TIMER),
    SMSG_STOP_MIRROR_TIMER(SMSG_STOP_MIRROR_TIMER),
    SMSG_PONG(SMSG_PONG),
    SMSG_CLEAR_COOLDOWN(SMSG_CLEAR_COOLDOWN),
    SMSG_GAMEOBJECT_PAGETEXT(SMSG_GAMEOBJECT_PAGETEXT),
    SMSG_SPELL_DELAYED(SMSG_SPELL_DELAYED),
    SMSG_ITEM_TIME_UPDATE(SMSG_ITEM_TIME_UPDATE),
    SMSG_ITEM_ENCHANT_TIME_UPDATE(SMSG_ITEM_ENCHANT_TIME_UPDATE),
    SMSG_AUTH_CHALLENGE(SMSG_AUTH_CHALLENGE),
    SMSG_AUTH_RESPONSE(SMSG_AUTH_RESPONSE),
    MSG_SAVE_GUILD_EMBLEM(MSG_SAVE_GUILD_EMBLEM_Server),
    SMSG_PLAY_SPELL_VISUAL(SMSG_PLAY_SPELL_VISUAL),
    SMSG_PARTYKILLLOG(SMSG_PARTYKILLLOG),
    SMSG_COMPRESSED_UPDATE_OBJECT(SMSG_COMPRESSED_UPDATE_OBJECT),
    SMSG_PLAY_SPELL_IMPACT(SMSG_PLAY_SPELL_IMPACT),
    SMSG_EXPLORATION_EXPERIENCE(SMSG_EXPLORATION_EXPERIENCE),
    MSG_RANDOM_ROLL(MSG_RANDOM_ROLL_Server),
    SMSG_ENVIRONMENTAL_DAMAGE_LOG(SMSG_ENVIRONMENTAL_DAMAGE_LOG),
    MSG_LOOKING_FOR_GROUP(MSG_LOOKING_FOR_GROUP_Server),
    SMSG_REMOVED_SPELL(SMSG_REMOVED_SPELL),
    SMSG_GMTICKET_CREATE(SMSG_GMTICKET_CREATE),
    SMSG_GMTICKET_UPDATETEXT(SMSG_GMTICKET_UPDATETEXT),
    SMSG_ACCOUNT_DATA_TIMES(SMSG_ACCOUNT_DATA_TIMES),
    SMSG_GMTICKET_GETTICKET(SMSG_GMTICKET_GETTICKET),
    SMSG_GAMEOBJECT_SPAWN_ANIM(SMSG_GAMEOBJECT_SPAWN_ANIM),
    SMSG_GAMEOBJECT_DESPAWN_ANIM(SMSG_GAMEOBJECT_DESPAWN_ANIM),
    MSG_CORPSE_QUERY(MSG_CORPSE_QUERY_Server),
    SMSG_GMTICKET_DELETETICKET(SMSG_GMTICKET_DELETETICKET),
    SMSG_CHAT_WRONG_FACTION,
    SMSG_GMTICKET_SYSTEMSTATUS(SMSG_GMTICKET_SYSTEMSTATUS),
    SMSG_SET_REST_START(SMSG_SET_REST_START),
    SMSG_SPIRIT_HEALER_CONFIRM(SMSG_SPIRIT_HEALER_CONFIRM),
    SMSG_GOSSIP_POI(SMSG_GOSSIP_POI),
    SMSG_LOGIN_VERIFY_WORLD(SMSG_LOGIN_VERIFY_WORLD),
    SMSG_SEND_MAIL_RESULT(SMSG_SEND_MAIL_RESULT),
    SMSG_MAIL_LIST_RESULT(SMSG_MAIL_LIST_RESULT),
    SMSG_BATTLEFIELD_LIST(SMSG_BATTLEFIELD_LIST),
    SMSG_ITEM_TEXT_QUERY_RESPONSE(SMSG_ITEM_TEXT_QUERY_RESPONSE),
    SMSG_SPELLLOGMISS(SMSG_SPELLLOGMISS),
    SMSG_SPELLLOGEXECUTE(SMSG_SPELLLOGEXECUTE),
    SMSG_PERIODICAURALOG(SMSG_PERIODICAURALOG),
    SMSG_SPELLDAMAGESHIELD(SMSG_SPELLDAMAGESHIELD),
    SMSG_SPELLNONMELEEDAMAGELOG(SMSG_SPELLNONMELEEDAMAGELOG),
    SMSG_ZONE_UNDER_ATTACK(SMSG_ZONE_UNDER_ATTACK),
    MSG_AUCTION_HELLO(MSG_AUCTION_HELLO_Server),
    SMSG_AUCTION_COMMAND_RESULT(SMSG_AUCTION_COMMAND_RESULT),
    SMSG_AUCTION_LIST_RESULT(SMSG_AUCTION_LIST_RESULT),
    SMSG_AUCTION_OWNER_LIST_RESULT(SMSG_AUCTION_OWNER_LIST_RESULT),
    SMSG_AUCTION_BIDDER_NOTIFICATION(SMSG_AUCTION_BIDDER_NOTIFICATION),
    SMSG_AUCTION_OWNER_NOTIFICATION(SMSG_AUCTION_OWNER_NOTIFICATION),
    SMSG_PROCRESIST(SMSG_PROCRESIST),
    SMSG_DISPEL_FAILED(SMSG_DISPEL_FAILED),
    SMSG_SPELLORDAMAGE_IMMUNE(SMSG_SPELLORDAMAGE_IMMUNE),
    SMSG_AUCTION_BIDDER_LIST_RESULT(SMSG_AUCTION_BIDDER_LIST_RESULT),
    SMSG_SET_FLAT_SPELL_MODIFIER(SMSG_SET_FLAT_SPELL_MODIFIER),
    SMSG_SET_PCT_SPELL_MODIFIER(SMSG_SET_PCT_SPELL_MODIFIER),
    SMSG_CORPSE_RECLAIM_DELAY(SMSG_CORPSE_RECLAIM_DELAY),
    MSG_LIST_STABLED_PETS(MSG_LIST_STABLED_PETS_Server),
    SMSG_STABLE_RESULT(SMSG_STABLE_RESULT),
    SMSG_PLAY_MUSIC(SMSG_PLAY_MUSIC),
    SMSG_PLAY_OBJECT_SOUND(SMSG_PLAY_OBJECT_SOUND),
    SMSG_SPELLDISPELLOG(SMSG_SPELLDISPELLOG),
    MSG_QUERY_NEXT_MAIL_TIME(MSG_QUERY_NEXT_MAIL_TIME_Server),
    SMSG_RECEIVED_MAIL(SMSG_RECEIVED_MAIL),
    SMSG_RAID_GROUP_ONLY(SMSG_RAID_GROUP_ONLY),
    SMSG_PVP_CREDIT(SMSG_PVP_CREDIT),
    SMSG_AUCTION_REMOVED_NOTIFICATION(SMSG_AUCTION_REMOVED_NOTIFICATION),
    SMSG_SERVER_MESSAGE(SMSG_SERVER_MESSAGE),
    SMSG_MEETINGSTONE_SETQUEUE(SMSG_MEETINGSTONE_SETQUEUE),
    SMSG_MEETINGSTONE_COMPLETE,
    SMSG_MEETINGSTONE_IN_PROGRESS,
    SMSG_MEETINGSTONE_MEMBER_ADDED(SMSG_MEETINGSTONE_MEMBER_ADDED),
    SMSG_CANCEL_AUTO_REPEAT,
    SMSG_STANDSTATE_UPDATE(SMSG_STANDSTATE_UPDATE),
    SMSG_LOOT_ALL_PASSED(SMSG_LOOT_ALL_PASSED),
    SMSG_LOOT_ROLL_WON(SMSG_LOOT_ROLL_WON),
    SMSG_LOOT_START_ROLL(SMSG_LOOT_START_ROLL),
    SMSG_LOOT_ROLL(SMSG_LOOT_ROLL),
    SMSG_LOOT_MASTER_LIST(SMSG_LOOT_MASTER_LIST),
    SMSG_SET_FORCED_REACTIONS(SMSG_SET_FORCED_REACTIONS),
    SMSG_SPELL_FAILED_OTHER(SMSG_SPELL_FAILED_OTHER),
    SMSG_GAMEOBJECT_RESET_STATE(SMSG_GAMEOBJECT_RESET_STATE),
    SMSG_CHAT_PLAYER_NOT_FOUND(SMSG_CHAT_PLAYER_NOT_FOUND),
    MSG_TALENT_WIPE_CONFIRM(MSG_TALENT_WIPE_CONFIRM_Server),
    SMSG_SUMMON_REQUEST(SMSG_SUMMON_REQUEST),
    SMSG_MONSTER_MOVE_TRANSPORT(SMSG_MONSTER_MOVE_TRANSPORT),
    SMSG_PET_BROKEN,
    MSG_MOVE_FEATHER_FALL(MSG_MOVE_FEATHER_FALL_Server),
    SMSG_FEIGN_DEATH_RESISTED,
    SMSG_DUEL_COUNTDOWN(SMSG_DUEL_COUNTDOWN),
    SMSG_AREA_TRIGGER_MESSAGE(SMSG_AREA_TRIGGER_MESSAGE),
    SMSG_MEETINGSTONE_JOINFAILED(SMSG_MEETINGSTONE_JOINFAILED),
    SMSG_PLAYER_SKINNED(SMSG_PLAYER_SKINNED),
    SMSG_DURABILITY_DAMAGE_DEATH,
    SMSG_INIT_WORLD_STATES(SMSG_INIT_WORLD_STATES),
    SMSG_UPDATE_WORLD_STATE(SMSG_UPDATE_WORLD_STATE),
    SMSG_ITEM_NAME_QUERY_RESPONSE(SMSG_ITEM_NAME_QUERY_RESPONSE),
    SMSG_PET_ACTION_FEEDBACK(SMSG_PET_ACTION_FEEDBACK),
    SMSG_CHAR_RENAME(SMSG_CHAR_RENAME),
    SMSG_INSTANCE_SAVE_CREATED(SMSG_INSTANCE_SAVE_CREATED),
    SMSG_RAID_INSTANCE_INFO(SMSG_RAID_INSTANCE_INFO),
    SMSG_PLAY_SOUND(SMSG_PLAY_SOUND),
    SMSG_BATTLEFIELD_STATUS(SMSG_BATTLEFIELD_STATUS),
    MSG_INSPECT_HONOR_STATS(MSG_INSPECT_HONOR_STATS_Server),
    SMSG_FORCE_WALK_SPEED_CHANGE(SMSG_FORCE_WALK_SPEED_CHANGE),
    SMSG_FORCE_SWIM_BACK_SPEED_CHANGE(SMSG_FORCE_SWIM_BACK_SPEED_CHANGE),
    SMSG_FORCE_TURN_RATE_CHANGE(SMSG_FORCE_TURN_RATE_CHANGE),
    MSG_PVP_LOG_DATA(MSG_PVP_LOG_DATA_Server),
    SMSG_AREA_SPIRIT_HEALER_TIME(SMSG_AREA_SPIRIT_HEALER_TIME),
    SMSG_WARDEN_DATA(SMSG_WARDEN_DATA),
    SMSG_GROUP_JOINED_BATTLEGROUND(SMSG_GROUP_JOINED_BATTLEGROUND),
    MSG_BATTLEGROUND_PLAYER_POSITIONS(MSG_BATTLEGROUND_PLAYER_POSITIONS_Server),
    SMSG_BINDER_CONFIRM(SMSG_BINDER_CONFIRM),
    SMSG_BATTLEGROUND_PLAYER_JOINED(SMSG_BATTLEGROUND_PLAYER_JOINED),
    SMSG_BATTLEGROUND_PLAYER_LEFT(SMSG_BATTLEGROUND_PLAYER_LEFT),
    SMSG_ADDON_INFO(SMSG_ADDON_INFO),
    SMSG_PET_UNLEARN_CONFIRM(SMSG_PET_UNLEARN_CONFIRM),
    SMSG_PARTY_MEMBER_STATS_FULL(SMSG_PARTY_MEMBER_STATS_FULL),
    SMSG_WEATHER(SMSG_WEATHER),
    SMSG_RAID_INSTANCE_MESSAGE(SMSG_RAID_INSTANCE_MESSAGE),
    SMSG_COMPRESSED_MOVES(SMSG_COMPRESSED_MOVES),
    SMSG_CHAT_RESTRICTED,
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
    MSG_MOVE_TIME_SKIPPED(MSG_MOVE_TIME_SKIPPED_Server),
    SMSG_SPLINE_MOVE_ROOT(SMSG_SPLINE_MOVE_ROOT),
    SMSG_INVALIDATE_PLAYER(SMSG_INVALIDATE_PLAYER),
    SMSG_INSTANCE_RESET(SMSG_INSTANCE_RESET),
    SMSG_INSTANCE_RESET_FAILED(SMSG_INSTANCE_RESET_FAILED),
    SMSG_UPDATE_LAST_INSTANCE(SMSG_UPDATE_LAST_INSTANCE),
    MSG_RAID_TARGET_UPDATE(MSG_RAID_TARGET_UPDATE_Server),
    MSG_RAID_READY_CHECK(MSG_RAID_READY_CHECK_Server),
    SMSG_PET_ACTION_SOUND(SMSG_PET_ACTION_SOUND),
    SMSG_PET_DISMISS_SOUND(SMSG_PET_DISMISS_SOUND),
    SMSG_GM_TICKET_STATUS_UPDATE(SMSG_GM_TICKET_STATUS_UPDATE),
    SMSG_UPDATE_INSTANCE_OWNERSHIP(SMSG_UPDATE_INSTANCE_OWNERSHIP),
    SMSG_SPELLINSTAKILLLOG(SMSG_SPELLINSTAKILLLOG),
    SMSG_SPELL_UPDATE_CHAIN_TARGETS(SMSG_SPELL_UPDATE_CHAIN_TARGETS),
    SMSG_EXPECTED_SPAM_RECORDS(SMSG_EXPECTED_SPAM_RECORDS),
    SMSG_DEFENSE_MESSAGE(SMSG_DEFENSE_MESSAGE),
}

impl ServerOpcodeMessage {
    fn read_opcodes(opcode: u16, body_size: u32, mut r: &[u8]) -> std::result::Result<Self, crate::errors::ExpectedOpcodeError> {
        match opcode {
            0x00DC => crate::util::assert_empty(body_size, opcode).map(|_| Self::MSG_MOVE_WORLDPORT_ACK),
            0x01C2 => Ok(Self::MSG_PETITION_DECLINE(<MSG_PETITION_DECLINE as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x01C2, size: body_size, io, } } else { a } })?)),
            0x01F2 => Ok(Self::MSG_TABARDVENDOR_ACTIVATE(<MSG_TABARDVENDOR_ACTIVATE as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x01F2, size: body_size, io, } } else { a } })?)),
            0x0276 => Ok(Self::MSG_QUEST_PUSH_RESULT(<MSG_QUEST_PUSH_RESULT as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x0276, size: body_size, io, } } else { a } })?)),
            0x02B1 => Ok(Self::MSG_MOVE_WATER_WALK(<MSG_MOVE_WATER_WALK as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x02B1, size: body_size, io, } } else { a } })?)),
            0x02C1 => Ok(Self::MSG_PETITION_RENAME(<MSG_PETITION_RENAME as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x02C1, size: body_size, io, } } else { a } })?)),
            0x003A => Ok(Self::SMSG_CHAR_CREATE(<SMSG_CHAR_CREATE as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x003A, size: body_size, io, } } else { a } })?)),
            0x003B => Ok(Self::SMSG_CHAR_ENUM(<SMSG_CHAR_ENUM as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x003B, size: body_size, io, } } else { a } })?)),
            0x003C => Ok(Self::SMSG_CHAR_DELETE(<SMSG_CHAR_DELETE as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x003C, size: body_size, io, } } else { a } })?)),
            0x003E => Ok(Self::SMSG_NEW_WORLD(<SMSG_NEW_WORLD as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x003E, size: body_size, io, } } else { a } })?)),
            0x003F => Ok(Self::SMSG_TRANSFER_PENDING(<SMSG_TRANSFER_PENDING as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x003F, size: body_size, io, } } else { a } })?)),
            0x0040 => Ok(Self::SMSG_TRANSFER_ABORTED(<SMSG_TRANSFER_ABORTED as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x0040, size: body_size, io, } } else { a } })?)),
            0x0041 => Ok(Self::SMSG_CHARACTER_LOGIN_FAILED(<SMSG_CHARACTER_LOGIN_FAILED as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x0041, size: body_size, io, } } else { a } })?)),
            0x0042 => Ok(Self::SMSG_LOGIN_SETTIMESPEED(<SMSG_LOGIN_SETTIMESPEED as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x0042, size: body_size, io, } } else { a } })?)),
            0x004C => Ok(Self::SMSG_LOGOUT_RESPONSE(<SMSG_LOGOUT_RESPONSE as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x004C, size: body_size, io, } } else { a } })?)),
            0x004D => crate::util::assert_empty(body_size, opcode).map(|_| Self::SMSG_LOGOUT_COMPLETE),
            0x004F => crate::util::assert_empty(body_size, opcode).map(|_| Self::SMSG_LOGOUT_CANCEL_ACK),
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
            0x0067 => Ok(Self::SMSG_FRIEND_LIST(<SMSG_FRIEND_LIST as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x0067, size: body_size, io, } } else { a } })?)),
            0x0068 => Ok(Self::SMSG_FRIEND_STATUS(<SMSG_FRIEND_STATUS as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x0068, size: body_size, io, } } else { a } })?)),
            0x006B => Ok(Self::SMSG_IGNORE_LIST(<SMSG_IGNORE_LIST as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x006B, size: body_size, io, } } else { a } })?)),
            0x006F => Ok(Self::SMSG_GROUP_INVITE(<SMSG_GROUP_INVITE as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x006F, size: body_size, io, } } else { a } })?)),
            0x0074 => Ok(Self::SMSG_GROUP_DECLINE(<SMSG_GROUP_DECLINE as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x0074, size: body_size, io, } } else { a } })?)),
            0x0077 => crate::util::assert_empty(body_size, opcode).map(|_| Self::SMSG_GROUP_UNINVITE),
            0x0079 => Ok(Self::SMSG_GROUP_SET_LEADER(<SMSG_GROUP_SET_LEADER as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x0079, size: body_size, io, } } else { a } })?)),
            0x007C => crate::util::assert_empty(body_size, opcode).map(|_| Self::SMSG_GROUP_DESTROYED),
            0x007D => Ok(Self::SMSG_GROUP_LIST(<SMSG_GROUP_LIST as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x007D, size: body_size, io, } } else { a } })?)),
            0x007E => Ok(Self::SMSG_PARTY_MEMBER_STATS(<SMSG_PARTY_MEMBER_STATS as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x007E, size: body_size, io, } } else { a } })?)),
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
            0x00B5 => Ok(Self::MSG_MOVE_START_FORWARD(<MSG_MOVE_START_FORWARD_Server as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x00B5, size: body_size, io, } } else { a } })?)),
            0x00B6 => Ok(Self::MSG_MOVE_START_BACKWARD(<MSG_MOVE_START_BACKWARD_Server as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x00B6, size: body_size, io, } } else { a } })?)),
            0x00B7 => Ok(Self::MSG_MOVE_STOP(<MSG_MOVE_STOP_Server as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x00B7, size: body_size, io, } } else { a } })?)),
            0x00B8 => Ok(Self::MSG_MOVE_START_STRAFE_LEFT(<MSG_MOVE_START_STRAFE_LEFT_Server as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x00B8, size: body_size, io, } } else { a } })?)),
            0x00B9 => Ok(Self::MSG_MOVE_START_STRAFE_RIGHT(<MSG_MOVE_START_STRAFE_RIGHT_Server as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x00B9, size: body_size, io, } } else { a } })?)),
            0x00BA => Ok(Self::MSG_MOVE_STOP_STRAFE(<MSG_MOVE_STOP_STRAFE_Server as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x00BA, size: body_size, io, } } else { a } })?)),
            0x00BB => Ok(Self::MSG_MOVE_JUMP(<MSG_MOVE_JUMP_Server as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x00BB, size: body_size, io, } } else { a } })?)),
            0x00BC => Ok(Self::MSG_MOVE_START_TURN_LEFT(<MSG_MOVE_START_TURN_LEFT_Server as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x00BC, size: body_size, io, } } else { a } })?)),
            0x00BD => Ok(Self::MSG_MOVE_START_TURN_RIGHT(<MSG_MOVE_START_TURN_RIGHT_Server as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x00BD, size: body_size, io, } } else { a } })?)),
            0x00BE => Ok(Self::MSG_MOVE_STOP_TURN(<MSG_MOVE_STOP_TURN_Server as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x00BE, size: body_size, io, } } else { a } })?)),
            0x00BF => Ok(Self::MSG_MOVE_START_PITCH_UP(<MSG_MOVE_START_PITCH_UP_Server as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x00BF, size: body_size, io, } } else { a } })?)),
            0x00C0 => Ok(Self::MSG_MOVE_START_PITCH_DOWN(<MSG_MOVE_START_PITCH_DOWN_Server as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x00C0, size: body_size, io, } } else { a } })?)),
            0x00C1 => Ok(Self::MSG_MOVE_STOP_PITCH(<MSG_MOVE_STOP_PITCH_Server as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x00C1, size: body_size, io, } } else { a } })?)),
            0x00C2 => Ok(Self::MSG_MOVE_SET_RUN_MODE(<MSG_MOVE_SET_RUN_MODE_Server as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x00C2, size: body_size, io, } } else { a } })?)),
            0x00C3 => Ok(Self::MSG_MOVE_SET_WALK_MODE(<MSG_MOVE_SET_WALK_MODE_Server as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x00C3, size: body_size, io, } } else { a } })?)),
            0x00C7 => Ok(Self::MSG_MOVE_TELEPORT_ACK(<MSG_MOVE_TELEPORT_ACK_Server as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x00C7, size: body_size, io, } } else { a } })?)),
            0x00C9 => Ok(Self::MSG_MOVE_FALL_LAND(<MSG_MOVE_FALL_LAND_Server as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x00C9, size: body_size, io, } } else { a } })?)),
            0x00CA => Ok(Self::MSG_MOVE_START_SWIM(<MSG_MOVE_START_SWIM_Server as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x00CA, size: body_size, io, } } else { a } })?)),
            0x00CB => Ok(Self::MSG_MOVE_STOP_SWIM(<MSG_MOVE_STOP_SWIM_Server as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x00CB, size: body_size, io, } } else { a } })?)),
            0x00DA => Ok(Self::MSG_MOVE_SET_FACING(<MSG_MOVE_SET_FACING_Server as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x00DA, size: body_size, io, } } else { a } })?)),
            0x00DB => Ok(Self::MSG_MOVE_SET_PITCH(<MSG_MOVE_SET_PITCH_Server as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x00DB, size: body_size, io, } } else { a } })?)),
            0x00DD => Ok(Self::SMSG_MONSTER_MOVE(<SMSG_MONSTER_MOVE as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x00DD, size: body_size, io, } } else { a } })?)),
            0x00DE => Ok(Self::SMSG_MOVE_WATER_WALK(<SMSG_MOVE_WATER_WALK as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x00DE, size: body_size, io, } } else { a } })?)),
            0x00DF => Ok(Self::SMSG_MOVE_LAND_WALK(<SMSG_MOVE_LAND_WALK as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x00DF, size: body_size, io, } } else { a } })?)),
            0x00E2 => Ok(Self::SMSG_FORCE_RUN_SPEED_CHANGE(<SMSG_FORCE_RUN_SPEED_CHANGE as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x00E2, size: body_size, io, } } else { a } })?)),
            0x00E4 => Ok(Self::SMSG_FORCE_RUN_BACK_SPEED_CHANGE(<SMSG_FORCE_RUN_BACK_SPEED_CHANGE as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x00E4, size: body_size, io, } } else { a } })?)),
            0x00E6 => Ok(Self::SMSG_FORCE_SWIM_SPEED_CHANGE(<SMSG_FORCE_SWIM_SPEED_CHANGE as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x00E6, size: body_size, io, } } else { a } })?)),
            0x00E8 => Ok(Self::SMSG_FORCE_MOVE_ROOT(<SMSG_FORCE_MOVE_ROOT as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x00E8, size: body_size, io, } } else { a } })?)),
            0x00EA => Ok(Self::SMSG_FORCE_MOVE_UNROOT(<SMSG_FORCE_MOVE_UNROOT as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x00EA, size: body_size, io, } } else { a } })?)),
            0x00EE => Ok(Self::MSG_MOVE_HEARTBEAT(<MSG_MOVE_HEARTBEAT_Server as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x00EE, size: body_size, io, } } else { a } })?)),
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
            0x0113 => Ok(Self::SMSG_OPEN_CONTAINER(<SMSG_OPEN_CONTAINER as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x0113, size: body_size, io, } } else { a } })?)),
            0x0115 => Ok(Self::SMSG_INSPECT(<SMSG_INSPECT as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x0115, size: body_size, io, } } else { a } })?)),
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
            0x0130 => Ok(Self::SMSG_CAST_RESULT(<SMSG_CAST_RESULT as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x0130, size: body_size, io, } } else { a } })?)),
            0x0131 => Ok(Self::SMSG_SPELL_START(<SMSG_SPELL_START as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x0131, size: body_size, io, } } else { a } })?)),
            0x0132 => Ok(Self::SMSG_SPELL_GO(<SMSG_SPELL_GO as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x0132, size: body_size, io, } } else { a } })?)),
            0x0133 => Ok(Self::SMSG_SPELL_FAILURE(<SMSG_SPELL_FAILURE as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x0133, size: body_size, io, } } else { a } })?)),
            0x0134 => Ok(Self::SMSG_SPELL_COOLDOWN(<SMSG_SPELL_COOLDOWN as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x0134, size: body_size, io, } } else { a } })?)),
            0x0135 => Ok(Self::SMSG_COOLDOWN_EVENT(<SMSG_COOLDOWN_EVENT as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x0135, size: body_size, io, } } else { a } })?)),
            0x0137 => Ok(Self::SMSG_UPDATE_AURA_DURATION(<SMSG_UPDATE_AURA_DURATION as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x0137, size: body_size, io, } } else { a } })?)),
            0x0138 => Ok(Self::SMSG_PET_CAST_FAILED(<SMSG_PET_CAST_FAILED as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x0138, size: body_size, io, } } else { a } })?)),
            0x0139 => Ok(Self::MSG_CHANNEL_START(<MSG_CHANNEL_START_Server as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x0139, size: body_size, io, } } else { a } })?)),
            0x013A => Ok(Self::MSG_CHANNEL_UPDATE(<MSG_CHANNEL_UPDATE_Server as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x013A, size: body_size, io, } } else { a } })?)),
            0x013C => Ok(Self::SMSG_AI_REACTION(<SMSG_AI_REACTION as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x013C, size: body_size, io, } } else { a } })?)),
            0x0143 => Ok(Self::SMSG_ATTACKSTART(<SMSG_ATTACKSTART as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x0143, size: body_size, io, } } else { a } })?)),
            0x0144 => Ok(Self::SMSG_ATTACKSTOP(<SMSG_ATTACKSTOP as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x0144, size: body_size, io, } } else { a } })?)),
            0x0145 => crate::util::assert_empty(body_size, opcode).map(|_| Self::SMSG_ATTACKSWING_NOTINRANGE),
            0x0146 => crate::util::assert_empty(body_size, opcode).map(|_| Self::SMSG_ATTACKSWING_BADFACING),
            0x0147 => crate::util::assert_empty(body_size, opcode).map(|_| Self::SMSG_ATTACKSWING_NOTSTANDING),
            0x0148 => crate::util::assert_empty(body_size, opcode).map(|_| Self::SMSG_ATTACKSWING_DEADTARGET),
            0x0149 => crate::util::assert_empty(body_size, opcode).map(|_| Self::SMSG_ATTACKSWING_CANT_ATTACK),
            0x014A => Ok(Self::SMSG_ATTACKERSTATEUPDATE(<SMSG_ATTACKERSTATEUPDATE as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x014A, size: body_size, io, } } else { a } })?)),
            0x014E => crate::util::assert_empty(body_size, opcode).map(|_| Self::SMSG_CANCEL_COMBAT),
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
            0x0165 => crate::util::assert_empty(body_size, opcode).map(|_| Self::SMSG_LOOT_CLEAR_MONEY),
            0x0166 => Ok(Self::SMSG_ITEM_PUSH_RESULT(<SMSG_ITEM_PUSH_RESULT as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x0166, size: body_size, io, } } else { a } })?)),
            0x0167 => Ok(Self::SMSG_DUEL_REQUESTED(<SMSG_DUEL_REQUESTED as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x0167, size: body_size, io, } } else { a } })?)),
            0x0168 => crate::util::assert_empty(body_size, opcode).map(|_| Self::SMSG_DUEL_OUTOFBOUNDS),
            0x0169 => crate::util::assert_empty(body_size, opcode).map(|_| Self::SMSG_DUEL_INBOUNDS),
            0x016A => Ok(Self::SMSG_DUEL_COMPLETE(<SMSG_DUEL_COMPLETE as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x016A, size: body_size, io, } } else { a } })?)),
            0x016B => Ok(Self::SMSG_DUEL_WINNER(<SMSG_DUEL_WINNER as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x016B, size: body_size, io, } } else { a } })?)),
            0x016E => Ok(Self::SMSG_MOUNTRESULT(<SMSG_MOUNTRESULT as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x016E, size: body_size, io, } } else { a } })?)),
            0x016F => Ok(Self::SMSG_DISMOUNTRESULT(<SMSG_DISMOUNTRESULT as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x016F, size: body_size, io, } } else { a } })?)),
            0x0172 => Ok(Self::SMSG_MOUNTSPECIAL_ANIM(<SMSG_MOUNTSPECIAL_ANIM as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x0172, size: body_size, io, } } else { a } })?)),
            0x0173 => Ok(Self::SMSG_PET_TAME_FAILURE(<SMSG_PET_TAME_FAILURE as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x0173, size: body_size, io, } } else { a } })?)),
            0x0178 => crate::util::assert_empty(body_size, opcode).map(|_| Self::SMSG_PET_NAME_INVALID),
            0x0179 => Ok(Self::SMSG_PET_SPELLS(<SMSG_PET_SPELLS as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x0179, size: body_size, io, } } else { a } })?)),
            0x017A => Ok(Self::SMSG_PET_MODE(<SMSG_PET_MODE as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x017A, size: body_size, io, } } else { a } })?)),
            0x017D => Ok(Self::SMSG_GOSSIP_MESSAGE(<SMSG_GOSSIP_MESSAGE as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x017D, size: body_size, io, } } else { a } })?)),
            0x017E => crate::util::assert_empty(body_size, opcode).map(|_| Self::SMSG_GOSSIP_COMPLETE),
            0x0180 => Ok(Self::SMSG_NPC_TEXT_UPDATE(<SMSG_NPC_TEXT_UPDATE as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x0180, size: body_size, io, } } else { a } })?)),
            0x0183 => Ok(Self::SMSG_QUESTGIVER_STATUS(<SMSG_QUESTGIVER_STATUS as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x0183, size: body_size, io, } } else { a } })?)),
            0x0185 => Ok(Self::SMSG_QUESTGIVER_QUEST_LIST(<SMSG_QUESTGIVER_QUEST_LIST as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x0185, size: body_size, io, } } else { a } })?)),
            0x0188 => Ok(Self::SMSG_QUESTGIVER_QUEST_DETAILS(<SMSG_QUESTGIVER_QUEST_DETAILS as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x0188, size: body_size, io, } } else { a } })?)),
            0x018B => Ok(Self::SMSG_QUESTGIVER_REQUEST_ITEMS(<SMSG_QUESTGIVER_REQUEST_ITEMS as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x018B, size: body_size, io, } } else { a } })?)),
            0x018D => Ok(Self::SMSG_QUESTGIVER_OFFER_REWARD(<SMSG_QUESTGIVER_OFFER_REWARD as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x018D, size: body_size, io, } } else { a } })?)),
            0x018F => Ok(Self::SMSG_QUESTGIVER_QUEST_INVALID(<SMSG_QUESTGIVER_QUEST_INVALID as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x018F, size: body_size, io, } } else { a } })?)),
            0x0191 => Ok(Self::SMSG_QUESTGIVER_QUEST_COMPLETE(<SMSG_QUESTGIVER_QUEST_COMPLETE as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x0191, size: body_size, io, } } else { a } })?)),
            0x0192 => Ok(Self::SMSG_QUESTGIVER_QUEST_FAILED(<SMSG_QUESTGIVER_QUEST_FAILED as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x0192, size: body_size, io, } } else { a } })?)),
            0x0195 => crate::util::assert_empty(body_size, opcode).map(|_| Self::SMSG_QUESTLOG_FULL),
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
            0x01AF => crate::util::assert_empty(body_size, opcode).map(|_| Self::SMSG_NEW_TAXI_PATH),
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
            0x01C8 => crate::util::assert_empty(body_size, opcode).map(|_| Self::SMSG_FISH_NOT_HOOKED),
            0x01C9 => crate::util::assert_empty(body_size, opcode).map(|_| Self::SMSG_FISH_ESCAPED),
            0x01CB => Ok(Self::SMSG_NOTIFICATION(<SMSG_NOTIFICATION as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x01CB, size: body_size, io, } } else { a } })?)),
            0x01CD => Ok(Self::SMSG_PLAYED_TIME(<SMSG_PLAYED_TIME as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x01CD, size: body_size, io, } } else { a } })?)),
            0x01CF => Ok(Self::SMSG_QUERY_TIME_RESPONSE(<SMSG_QUERY_TIME_RESPONSE as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x01CF, size: body_size, io, } } else { a } })?)),
            0x01D0 => Ok(Self::SMSG_LOG_XPGAIN(<SMSG_LOG_XPGAIN as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x01D0, size: body_size, io, } } else { a } })?)),
            0x01D4 => Ok(Self::SMSG_LEVELUP_INFO(<SMSG_LEVELUP_INFO as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x01D4, size: body_size, io, } } else { a } })?)),
            0x01D5 => Ok(Self::MSG_MINIMAP_PING(<MSG_MINIMAP_PING_Server as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x01D5, size: body_size, io, } } else { a } })?)),
            0x01D6 => Ok(Self::SMSG_RESISTLOG(<SMSG_RESISTLOG as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x01D6, size: body_size, io, } } else { a } })?)),
            0x01D7 => Ok(Self::SMSG_ENCHANTMENTLOG(<SMSG_ENCHANTMENTLOG as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x01D7, size: body_size, io, } } else { a } })?)),
            0x01D9 => Ok(Self::SMSG_START_MIRROR_TIMER(<SMSG_START_MIRROR_TIMER as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x01D9, size: body_size, io, } } else { a } })?)),
            0x01DA => Ok(Self::SMSG_PAUSE_MIRROR_TIMER(<SMSG_PAUSE_MIRROR_TIMER as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x01DA, size: body_size, io, } } else { a } })?)),
            0x01DB => Ok(Self::SMSG_STOP_MIRROR_TIMER(<SMSG_STOP_MIRROR_TIMER as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x01DB, size: body_size, io, } } else { a } })?)),
            0x01DD => Ok(Self::SMSG_PONG(<SMSG_PONG as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x01DD, size: body_size, io, } } else { a } })?)),
            0x01DE => Ok(Self::SMSG_CLEAR_COOLDOWN(<SMSG_CLEAR_COOLDOWN as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x01DE, size: body_size, io, } } else { a } })?)),
            0x01DF => Ok(Self::SMSG_GAMEOBJECT_PAGETEXT(<SMSG_GAMEOBJECT_PAGETEXT as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x01DF, size: body_size, io, } } else { a } })?)),
            0x01E2 => Ok(Self::SMSG_SPELL_DELAYED(<SMSG_SPELL_DELAYED as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x01E2, size: body_size, io, } } else { a } })?)),
            0x01EA => Ok(Self::SMSG_ITEM_TIME_UPDATE(<SMSG_ITEM_TIME_UPDATE as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x01EA, size: body_size, io, } } else { a } })?)),
            0x01EB => Ok(Self::SMSG_ITEM_ENCHANT_TIME_UPDATE(<SMSG_ITEM_ENCHANT_TIME_UPDATE as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x01EB, size: body_size, io, } } else { a } })?)),
            0x01EC => Ok(Self::SMSG_AUTH_CHALLENGE(<SMSG_AUTH_CHALLENGE as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x01EC, size: body_size, io, } } else { a } })?)),
            0x01EE => Ok(Self::SMSG_AUTH_RESPONSE(<SMSG_AUTH_RESPONSE as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x01EE, size: body_size, io, } } else { a } })?)),
            0x01F1 => Ok(Self::MSG_SAVE_GUILD_EMBLEM(<MSG_SAVE_GUILD_EMBLEM_Server as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x01F1, size: body_size, io, } } else { a } })?)),
            0x01F3 => Ok(Self::SMSG_PLAY_SPELL_VISUAL(<SMSG_PLAY_SPELL_VISUAL as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x01F3, size: body_size, io, } } else { a } })?)),
            0x01F5 => Ok(Self::SMSG_PARTYKILLLOG(<SMSG_PARTYKILLLOG as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x01F5, size: body_size, io, } } else { a } })?)),
            0x01F6 => Ok(Self::SMSG_COMPRESSED_UPDATE_OBJECT(<SMSG_COMPRESSED_UPDATE_OBJECT as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x01F6, size: body_size, io, } } else { a } })?)),
            0x01F7 => Ok(Self::SMSG_PLAY_SPELL_IMPACT(<SMSG_PLAY_SPELL_IMPACT as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x01F7, size: body_size, io, } } else { a } })?)),
            0x01F8 => Ok(Self::SMSG_EXPLORATION_EXPERIENCE(<SMSG_EXPLORATION_EXPERIENCE as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x01F8, size: body_size, io, } } else { a } })?)),
            0x01FB => Ok(Self::MSG_RANDOM_ROLL(<MSG_RANDOM_ROLL_Server as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x01FB, size: body_size, io, } } else { a } })?)),
            0x01FC => Ok(Self::SMSG_ENVIRONMENTAL_DAMAGE_LOG(<SMSG_ENVIRONMENTAL_DAMAGE_LOG as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x01FC, size: body_size, io, } } else { a } })?)),
            0x01FF => Ok(Self::MSG_LOOKING_FOR_GROUP(<MSG_LOOKING_FOR_GROUP_Server as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x01FF, size: body_size, io, } } else { a } })?)),
            0x0203 => Ok(Self::SMSG_REMOVED_SPELL(<SMSG_REMOVED_SPELL as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x0203, size: body_size, io, } } else { a } })?)),
            0x0206 => Ok(Self::SMSG_GMTICKET_CREATE(<SMSG_GMTICKET_CREATE as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x0206, size: body_size, io, } } else { a } })?)),
            0x0208 => Ok(Self::SMSG_GMTICKET_UPDATETEXT(<SMSG_GMTICKET_UPDATETEXT as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x0208, size: body_size, io, } } else { a } })?)),
            0x0209 => Ok(Self::SMSG_ACCOUNT_DATA_TIMES(<SMSG_ACCOUNT_DATA_TIMES as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x0209, size: body_size, io, } } else { a } })?)),
            0x0212 => Ok(Self::SMSG_GMTICKET_GETTICKET(<SMSG_GMTICKET_GETTICKET as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x0212, size: body_size, io, } } else { a } })?)),
            0x0214 => Ok(Self::SMSG_GAMEOBJECT_SPAWN_ANIM(<SMSG_GAMEOBJECT_SPAWN_ANIM as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x0214, size: body_size, io, } } else { a } })?)),
            0x0215 => Ok(Self::SMSG_GAMEOBJECT_DESPAWN_ANIM(<SMSG_GAMEOBJECT_DESPAWN_ANIM as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x0215, size: body_size, io, } } else { a } })?)),
            0x0216 => Ok(Self::MSG_CORPSE_QUERY(<MSG_CORPSE_QUERY_Server as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x0216, size: body_size, io, } } else { a } })?)),
            0x0218 => Ok(Self::SMSG_GMTICKET_DELETETICKET(<SMSG_GMTICKET_DELETETICKET as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x0218, size: body_size, io, } } else { a } })?)),
            0x0219 => crate::util::assert_empty(body_size, opcode).map(|_| Self::SMSG_CHAT_WRONG_FACTION),
            0x021B => Ok(Self::SMSG_GMTICKET_SYSTEMSTATUS(<SMSG_GMTICKET_SYSTEMSTATUS as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x021B, size: body_size, io, } } else { a } })?)),
            0x021E => Ok(Self::SMSG_SET_REST_START(<SMSG_SET_REST_START as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x021E, size: body_size, io, } } else { a } })?)),
            0x0222 => Ok(Self::SMSG_SPIRIT_HEALER_CONFIRM(<SMSG_SPIRIT_HEALER_CONFIRM as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x0222, size: body_size, io, } } else { a } })?)),
            0x0224 => Ok(Self::SMSG_GOSSIP_POI(<SMSG_GOSSIP_POI as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x0224, size: body_size, io, } } else { a } })?)),
            0x0236 => Ok(Self::SMSG_LOGIN_VERIFY_WORLD(<SMSG_LOGIN_VERIFY_WORLD as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x0236, size: body_size, io, } } else { a } })?)),
            0x0239 => Ok(Self::SMSG_SEND_MAIL_RESULT(<SMSG_SEND_MAIL_RESULT as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x0239, size: body_size, io, } } else { a } })?)),
            0x023B => Ok(Self::SMSG_MAIL_LIST_RESULT(<SMSG_MAIL_LIST_RESULT as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x023B, size: body_size, io, } } else { a } })?)),
            0x023D => Ok(Self::SMSG_BATTLEFIELD_LIST(<SMSG_BATTLEFIELD_LIST as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x023D, size: body_size, io, } } else { a } })?)),
            0x0244 => Ok(Self::SMSG_ITEM_TEXT_QUERY_RESPONSE(<SMSG_ITEM_TEXT_QUERY_RESPONSE as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x0244, size: body_size, io, } } else { a } })?)),
            0x024B => Ok(Self::SMSG_SPELLLOGMISS(<SMSG_SPELLLOGMISS as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x024B, size: body_size, io, } } else { a } })?)),
            0x024C => Ok(Self::SMSG_SPELLLOGEXECUTE(<SMSG_SPELLLOGEXECUTE as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x024C, size: body_size, io, } } else { a } })?)),
            0x024E => Ok(Self::SMSG_PERIODICAURALOG(<SMSG_PERIODICAURALOG as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x024E, size: body_size, io, } } else { a } })?)),
            0x024F => Ok(Self::SMSG_SPELLDAMAGESHIELD(<SMSG_SPELLDAMAGESHIELD as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x024F, size: body_size, io, } } else { a } })?)),
            0x0250 => Ok(Self::SMSG_SPELLNONMELEEDAMAGELOG(<SMSG_SPELLNONMELEEDAMAGELOG as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x0250, size: body_size, io, } } else { a } })?)),
            0x0254 => Ok(Self::SMSG_ZONE_UNDER_ATTACK(<SMSG_ZONE_UNDER_ATTACK as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x0254, size: body_size, io, } } else { a } })?)),
            0x0255 => Ok(Self::MSG_AUCTION_HELLO(<MSG_AUCTION_HELLO_Server as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x0255, size: body_size, io, } } else { a } })?)),
            0x025B => Ok(Self::SMSG_AUCTION_COMMAND_RESULT(<SMSG_AUCTION_COMMAND_RESULT as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x025B, size: body_size, io, } } else { a } })?)),
            0x025C => Ok(Self::SMSG_AUCTION_LIST_RESULT(<SMSG_AUCTION_LIST_RESULT as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x025C, size: body_size, io, } } else { a } })?)),
            0x025D => Ok(Self::SMSG_AUCTION_OWNER_LIST_RESULT(<SMSG_AUCTION_OWNER_LIST_RESULT as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x025D, size: body_size, io, } } else { a } })?)),
            0x025E => Ok(Self::SMSG_AUCTION_BIDDER_NOTIFICATION(<SMSG_AUCTION_BIDDER_NOTIFICATION as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x025E, size: body_size, io, } } else { a } })?)),
            0x025F => Ok(Self::SMSG_AUCTION_OWNER_NOTIFICATION(<SMSG_AUCTION_OWNER_NOTIFICATION as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x025F, size: body_size, io, } } else { a } })?)),
            0x0260 => Ok(Self::SMSG_PROCRESIST(<SMSG_PROCRESIST as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x0260, size: body_size, io, } } else { a } })?)),
            0x0262 => Ok(Self::SMSG_DISPEL_FAILED(<SMSG_DISPEL_FAILED as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x0262, size: body_size, io, } } else { a } })?)),
            0x0263 => Ok(Self::SMSG_SPELLORDAMAGE_IMMUNE(<SMSG_SPELLORDAMAGE_IMMUNE as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x0263, size: body_size, io, } } else { a } })?)),
            0x0265 => Ok(Self::SMSG_AUCTION_BIDDER_LIST_RESULT(<SMSG_AUCTION_BIDDER_LIST_RESULT as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x0265, size: body_size, io, } } else { a } })?)),
            0x0266 => Ok(Self::SMSG_SET_FLAT_SPELL_MODIFIER(<SMSG_SET_FLAT_SPELL_MODIFIER as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x0266, size: body_size, io, } } else { a } })?)),
            0x0267 => Ok(Self::SMSG_SET_PCT_SPELL_MODIFIER(<SMSG_SET_PCT_SPELL_MODIFIER as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x0267, size: body_size, io, } } else { a } })?)),
            0x0269 => Ok(Self::SMSG_CORPSE_RECLAIM_DELAY(<SMSG_CORPSE_RECLAIM_DELAY as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x0269, size: body_size, io, } } else { a } })?)),
            0x026F => Ok(Self::MSG_LIST_STABLED_PETS(<MSG_LIST_STABLED_PETS_Server as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x026F, size: body_size, io, } } else { a } })?)),
            0x0273 => Ok(Self::SMSG_STABLE_RESULT(<SMSG_STABLE_RESULT as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x0273, size: body_size, io, } } else { a } })?)),
            0x0277 => Ok(Self::SMSG_PLAY_MUSIC(<SMSG_PLAY_MUSIC as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x0277, size: body_size, io, } } else { a } })?)),
            0x0278 => Ok(Self::SMSG_PLAY_OBJECT_SOUND(<SMSG_PLAY_OBJECT_SOUND as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x0278, size: body_size, io, } } else { a } })?)),
            0x027B => Ok(Self::SMSG_SPELLDISPELLOG(<SMSG_SPELLDISPELLOG as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x027B, size: body_size, io, } } else { a } })?)),
            0x0284 => Ok(Self::MSG_QUERY_NEXT_MAIL_TIME(<MSG_QUERY_NEXT_MAIL_TIME_Server as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x0284, size: body_size, io, } } else { a } })?)),
            0x0285 => Ok(Self::SMSG_RECEIVED_MAIL(<SMSG_RECEIVED_MAIL as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x0285, size: body_size, io, } } else { a } })?)),
            0x0286 => Ok(Self::SMSG_RAID_GROUP_ONLY(<SMSG_RAID_GROUP_ONLY as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x0286, size: body_size, io, } } else { a } })?)),
            0x028C => Ok(Self::SMSG_PVP_CREDIT(<SMSG_PVP_CREDIT as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x028C, size: body_size, io, } } else { a } })?)),
            0x028D => Ok(Self::SMSG_AUCTION_REMOVED_NOTIFICATION(<SMSG_AUCTION_REMOVED_NOTIFICATION as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x028D, size: body_size, io, } } else { a } })?)),
            0x0291 => Ok(Self::SMSG_SERVER_MESSAGE(<SMSG_SERVER_MESSAGE as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x0291, size: body_size, io, } } else { a } })?)),
            0x0295 => Ok(Self::SMSG_MEETINGSTONE_SETQUEUE(<SMSG_MEETINGSTONE_SETQUEUE as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x0295, size: body_size, io, } } else { a } })?)),
            0x0297 => crate::util::assert_empty(body_size, opcode).map(|_| Self::SMSG_MEETINGSTONE_COMPLETE),
            0x0298 => crate::util::assert_empty(body_size, opcode).map(|_| Self::SMSG_MEETINGSTONE_IN_PROGRESS),
            0x0299 => Ok(Self::SMSG_MEETINGSTONE_MEMBER_ADDED(<SMSG_MEETINGSTONE_MEMBER_ADDED as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x0299, size: body_size, io, } } else { a } })?)),
            0x029C => crate::util::assert_empty(body_size, opcode).map(|_| Self::SMSG_CANCEL_AUTO_REPEAT),
            0x029D => Ok(Self::SMSG_STANDSTATE_UPDATE(<SMSG_STANDSTATE_UPDATE as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x029D, size: body_size, io, } } else { a } })?)),
            0x029E => Ok(Self::SMSG_LOOT_ALL_PASSED(<SMSG_LOOT_ALL_PASSED as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x029E, size: body_size, io, } } else { a } })?)),
            0x029F => Ok(Self::SMSG_LOOT_ROLL_WON(<SMSG_LOOT_ROLL_WON as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x029F, size: body_size, io, } } else { a } })?)),
            0x02A1 => Ok(Self::SMSG_LOOT_START_ROLL(<SMSG_LOOT_START_ROLL as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x02A1, size: body_size, io, } } else { a } })?)),
            0x02A2 => Ok(Self::SMSG_LOOT_ROLL(<SMSG_LOOT_ROLL as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x02A2, size: body_size, io, } } else { a } })?)),
            0x02A4 => Ok(Self::SMSG_LOOT_MASTER_LIST(<SMSG_LOOT_MASTER_LIST as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x02A4, size: body_size, io, } } else { a } })?)),
            0x02A5 => Ok(Self::SMSG_SET_FORCED_REACTIONS(<SMSG_SET_FORCED_REACTIONS as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x02A5, size: body_size, io, } } else { a } })?)),
            0x02A6 => Ok(Self::SMSG_SPELL_FAILED_OTHER(<SMSG_SPELL_FAILED_OTHER as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x02A6, size: body_size, io, } } else { a } })?)),
            0x02A7 => Ok(Self::SMSG_GAMEOBJECT_RESET_STATE(<SMSG_GAMEOBJECT_RESET_STATE as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x02A7, size: body_size, io, } } else { a } })?)),
            0x02A9 => Ok(Self::SMSG_CHAT_PLAYER_NOT_FOUND(<SMSG_CHAT_PLAYER_NOT_FOUND as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x02A9, size: body_size, io, } } else { a } })?)),
            0x02AA => Ok(Self::MSG_TALENT_WIPE_CONFIRM(<MSG_TALENT_WIPE_CONFIRM_Server as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x02AA, size: body_size, io, } } else { a } })?)),
            0x02AB => Ok(Self::SMSG_SUMMON_REQUEST(<SMSG_SUMMON_REQUEST as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x02AB, size: body_size, io, } } else { a } })?)),
            0x02AE => Ok(Self::SMSG_MONSTER_MOVE_TRANSPORT(<SMSG_MONSTER_MOVE_TRANSPORT as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x02AE, size: body_size, io, } } else { a } })?)),
            0x02AF => crate::util::assert_empty(body_size, opcode).map(|_| Self::SMSG_PET_BROKEN),
            0x02B0 => Ok(Self::MSG_MOVE_FEATHER_FALL(<MSG_MOVE_FEATHER_FALL_Server as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x02B0, size: body_size, io, } } else { a } })?)),
            0x02B4 => crate::util::assert_empty(body_size, opcode).map(|_| Self::SMSG_FEIGN_DEATH_RESISTED),
            0x02B7 => Ok(Self::SMSG_DUEL_COUNTDOWN(<SMSG_DUEL_COUNTDOWN as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x02B7, size: body_size, io, } } else { a } })?)),
            0x02B8 => Ok(Self::SMSG_AREA_TRIGGER_MESSAGE(<SMSG_AREA_TRIGGER_MESSAGE as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x02B8, size: body_size, io, } } else { a } })?)),
            0x02BB => Ok(Self::SMSG_MEETINGSTONE_JOINFAILED(<SMSG_MEETINGSTONE_JOINFAILED as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x02BB, size: body_size, io, } } else { a } })?)),
            0x02BC => Ok(Self::SMSG_PLAYER_SKINNED(<SMSG_PLAYER_SKINNED as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x02BC, size: body_size, io, } } else { a } })?)),
            0x02BD => crate::util::assert_empty(body_size, opcode).map(|_| Self::SMSG_DURABILITY_DAMAGE_DEATH),
            0x02C2 => Ok(Self::SMSG_INIT_WORLD_STATES(<SMSG_INIT_WORLD_STATES as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x02C2, size: body_size, io, } } else { a } })?)),
            0x02C3 => Ok(Self::SMSG_UPDATE_WORLD_STATE(<SMSG_UPDATE_WORLD_STATE as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x02C3, size: body_size, io, } } else { a } })?)),
            0x02C5 => Ok(Self::SMSG_ITEM_NAME_QUERY_RESPONSE(<SMSG_ITEM_NAME_QUERY_RESPONSE as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x02C5, size: body_size, io, } } else { a } })?)),
            0x02C6 => Ok(Self::SMSG_PET_ACTION_FEEDBACK(<SMSG_PET_ACTION_FEEDBACK as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x02C6, size: body_size, io, } } else { a } })?)),
            0x02C8 => Ok(Self::SMSG_CHAR_RENAME(<SMSG_CHAR_RENAME as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x02C8, size: body_size, io, } } else { a } })?)),
            0x02CB => Ok(Self::SMSG_INSTANCE_SAVE_CREATED(<SMSG_INSTANCE_SAVE_CREATED as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x02CB, size: body_size, io, } } else { a } })?)),
            0x02CC => Ok(Self::SMSG_RAID_INSTANCE_INFO(<SMSG_RAID_INSTANCE_INFO as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x02CC, size: body_size, io, } } else { a } })?)),
            0x02D2 => Ok(Self::SMSG_PLAY_SOUND(<SMSG_PLAY_SOUND as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x02D2, size: body_size, io, } } else { a } })?)),
            0x02D4 => Ok(Self::SMSG_BATTLEFIELD_STATUS(<SMSG_BATTLEFIELD_STATUS as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x02D4, size: body_size, io, } } else { a } })?)),
            0x02D6 => Ok(Self::MSG_INSPECT_HONOR_STATS(<MSG_INSPECT_HONOR_STATS_Server as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x02D6, size: body_size, io, } } else { a } })?)),
            0x02DA => Ok(Self::SMSG_FORCE_WALK_SPEED_CHANGE(<SMSG_FORCE_WALK_SPEED_CHANGE as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x02DA, size: body_size, io, } } else { a } })?)),
            0x02DC => Ok(Self::SMSG_FORCE_SWIM_BACK_SPEED_CHANGE(<SMSG_FORCE_SWIM_BACK_SPEED_CHANGE as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x02DC, size: body_size, io, } } else { a } })?)),
            0x02DE => Ok(Self::SMSG_FORCE_TURN_RATE_CHANGE(<SMSG_FORCE_TURN_RATE_CHANGE as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x02DE, size: body_size, io, } } else { a } })?)),
            0x02E0 => Ok(Self::MSG_PVP_LOG_DATA(<MSG_PVP_LOG_DATA_Server as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x02E0, size: body_size, io, } } else { a } })?)),
            0x02E4 => Ok(Self::SMSG_AREA_SPIRIT_HEALER_TIME(<SMSG_AREA_SPIRIT_HEALER_TIME as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x02E4, size: body_size, io, } } else { a } })?)),
            0x02E6 => Ok(Self::SMSG_WARDEN_DATA(<SMSG_WARDEN_DATA as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x02E6, size: body_size, io, } } else { a } })?)),
            0x02E8 => Ok(Self::SMSG_GROUP_JOINED_BATTLEGROUND(<SMSG_GROUP_JOINED_BATTLEGROUND as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x02E8, size: body_size, io, } } else { a } })?)),
            0x02E9 => Ok(Self::MSG_BATTLEGROUND_PLAYER_POSITIONS(<MSG_BATTLEGROUND_PLAYER_POSITIONS_Server as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x02E9, size: body_size, io, } } else { a } })?)),
            0x02EB => Ok(Self::SMSG_BINDER_CONFIRM(<SMSG_BINDER_CONFIRM as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x02EB, size: body_size, io, } } else { a } })?)),
            0x02EC => Ok(Self::SMSG_BATTLEGROUND_PLAYER_JOINED(<SMSG_BATTLEGROUND_PLAYER_JOINED as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x02EC, size: body_size, io, } } else { a } })?)),
            0x02ED => Ok(Self::SMSG_BATTLEGROUND_PLAYER_LEFT(<SMSG_BATTLEGROUND_PLAYER_LEFT as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x02ED, size: body_size, io, } } else { a } })?)),
            0x02EF => Ok(Self::SMSG_ADDON_INFO(<SMSG_ADDON_INFO as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x02EF, size: body_size, io, } } else { a } })?)),
            0x02F1 => Ok(Self::SMSG_PET_UNLEARN_CONFIRM(<SMSG_PET_UNLEARN_CONFIRM as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x02F1, size: body_size, io, } } else { a } })?)),
            0x02F2 => Ok(Self::SMSG_PARTY_MEMBER_STATS_FULL(<SMSG_PARTY_MEMBER_STATS_FULL as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x02F2, size: body_size, io, } } else { a } })?)),
            0x02F4 => Ok(Self::SMSG_WEATHER(<SMSG_WEATHER as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x02F4, size: body_size, io, } } else { a } })?)),
            0x02FA => Ok(Self::SMSG_RAID_INSTANCE_MESSAGE(<SMSG_RAID_INSTANCE_MESSAGE as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x02FA, size: body_size, io, } } else { a } })?)),
            0x02FB => Ok(Self::SMSG_COMPRESSED_MOVES(<SMSG_COMPRESSED_MOVES as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x02FB, size: body_size, io, } } else { a } })?)),
            0x02FD => crate::util::assert_empty(body_size, opcode).map(|_| Self::SMSG_CHAT_RESTRICTED),
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
            0x0319 => Ok(Self::MSG_MOVE_TIME_SKIPPED(<MSG_MOVE_TIME_SKIPPED_Server as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x0319, size: body_size, io, } } else { a } })?)),
            0x031A => Ok(Self::SMSG_SPLINE_MOVE_ROOT(<SMSG_SPLINE_MOVE_ROOT as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x031A, size: body_size, io, } } else { a } })?)),
            0x031C => Ok(Self::SMSG_INVALIDATE_PLAYER(<SMSG_INVALIDATE_PLAYER as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x031C, size: body_size, io, } } else { a } })?)),
            0x031E => Ok(Self::SMSG_INSTANCE_RESET(<SMSG_INSTANCE_RESET as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x031E, size: body_size, io, } } else { a } })?)),
            0x031F => Ok(Self::SMSG_INSTANCE_RESET_FAILED(<SMSG_INSTANCE_RESET_FAILED as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x031F, size: body_size, io, } } else { a } })?)),
            0x0320 => Ok(Self::SMSG_UPDATE_LAST_INSTANCE(<SMSG_UPDATE_LAST_INSTANCE as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x0320, size: body_size, io, } } else { a } })?)),
            0x0321 => Ok(Self::MSG_RAID_TARGET_UPDATE(<MSG_RAID_TARGET_UPDATE_Server as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x0321, size: body_size, io, } } else { a } })?)),
            0x0322 => Ok(Self::MSG_RAID_READY_CHECK(<MSG_RAID_READY_CHECK_Server as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x0322, size: body_size, io, } } else { a } })?)),
            0x0324 => Ok(Self::SMSG_PET_ACTION_SOUND(<SMSG_PET_ACTION_SOUND as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x0324, size: body_size, io, } } else { a } })?)),
            0x0325 => Ok(Self::SMSG_PET_DISMISS_SOUND(<SMSG_PET_DISMISS_SOUND as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x0325, size: body_size, io, } } else { a } })?)),
            0x0328 => Ok(Self::SMSG_GM_TICKET_STATUS_UPDATE(<SMSG_GM_TICKET_STATUS_UPDATE as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x0328, size: body_size, io, } } else { a } })?)),
            0x032B => Ok(Self::SMSG_UPDATE_INSTANCE_OWNERSHIP(<SMSG_UPDATE_INSTANCE_OWNERSHIP as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x032B, size: body_size, io, } } else { a } })?)),
            0x032F => Ok(Self::SMSG_SPELLINSTAKILLLOG(<SMSG_SPELLINSTAKILLLOG as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x032F, size: body_size, io, } } else { a } })?)),
            0x0330 => Ok(Self::SMSG_SPELL_UPDATE_CHAIN_TARGETS(<SMSG_SPELL_UPDATE_CHAIN_TARGETS as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x0330, size: body_size, io, } } else { a } })?)),
            0x0332 => Ok(Self::SMSG_EXPECTED_SPAM_RECORDS(<SMSG_EXPECTED_SPAM_RECORDS as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x0332, size: body_size, io, } } else { a } })?)),
            0x033B => Ok(Self::SMSG_DEFENSE_MESSAGE(<SMSG_DEFENSE_MESSAGE as crate::Message>::read_body(&mut r, body_size).map_err(|a| { if let ParseError::Io(io) = a { ParseError::BufferSizeTooSmall { opcode: 0x033B, size: body_size, io, } } else { a } })?)),
            _ => Err(crate::errors::ExpectedOpcodeError::Opcode{ opcode: opcode.into(), name: opcode_to_name(opcode.into()), size: body_size }),
        }
    }

    #[cfg(feature = "sync")]
    pub fn read_unencrypted<R: std::io::Read>(mut r: R) -> std::result::Result<Self, crate::errors::ExpectedOpcodeError> {
        let size = (crate::util::read_u16_be(&mut r)?.saturating_sub(2)) as u32;
        let opcode = crate::util::read_u16_le(&mut r)?;

        let mut buf = vec![0; size as usize];
        r.read_exact(&mut buf)?;
        Self::read_opcodes(opcode, size, &buf)
    }
    #[cfg(all(feature = "sync", feature = "encryption"))]
    pub fn read_encrypted<R: std::io::Read>(mut r: R, d: &mut DecrypterHalf) -> std::result::Result<Self, crate::errors::ExpectedOpcodeError> {
        let mut header = [0_u8; 4];
        r.read_exact(&mut header)?;
        let header = d.decrypt_server_header(header);
        let opcode = header.opcode;
        let body_size = (header.size.saturating_sub(2)) as u32;

        let mut buf = vec![0; body_size as usize];
        r.read_exact(&mut buf)?;
        Self::read_opcodes(opcode, body_size, &buf)
    }

    #[cfg(feature = "tokio")]
    pub async fn tokio_read_unencrypted<R: tokio::io::AsyncReadExt + Unpin + Send>(mut r: R) -> std::result::Result<Self, crate::errors::ExpectedOpcodeError> {
        let size = (crate::util::tokio_read_u16_be(&mut r).await?.saturating_sub(2)) as u32;
        let opcode = crate::util::tokio_read_u16_le(&mut r).await?;

        let mut buf = vec![0; size as usize];
        r.read_exact(&mut buf).await?;
        Self::read_opcodes(opcode, size, &buf)
    }
    #[cfg(all(feature = "tokio", feature = "encryption"))]
    pub async fn tokio_read_encrypted<R: tokio::io::AsyncReadExt + Unpin + Send>(mut r: R, d: &mut DecrypterHalf) -> std::result::Result<Self, crate::errors::ExpectedOpcodeError> {
        let mut header = [0_u8; 4];
        r.read_exact(&mut header).await?;
        let header = d.decrypt_server_header(header);
        let opcode = header.opcode;
        let body_size = (header.size.saturating_sub(2)) as u32;

        let mut buf = vec![0; body_size as usize];
        r.read_exact(&mut buf).await?;
        Self::read_opcodes(opcode, body_size, &buf)
    }

    #[cfg(feature = "async-std")]
    pub async fn astd_read_unencrypted<R: async_std::io::ReadExt + Unpin + Send>(mut r: R) -> std::result::Result<Self, crate::errors::ExpectedOpcodeError> {
        let size = (crate::util::astd_read_u16_be(&mut r).await?.saturating_sub(2)) as u32;
        let opcode = crate::util::astd_read_u16_le(&mut r).await?;

        let mut buf = vec![0; size as usize];
        r.read_exact(&mut buf).await?;
        Self::read_opcodes(opcode, size, &buf)
    }
    #[cfg(all(feature = "async-std", feature = "encryption"))]
    pub async fn astd_read_encrypted<R: async_std::io::ReadExt + Unpin + Send>(mut r: R, d: &mut DecrypterHalf) -> std::result::Result<Self, crate::errors::ExpectedOpcodeError> {
        let mut header = [0_u8; 4];
        r.read_exact(&mut header).await?;
        let header = d.decrypt_server_header(header);
        let opcode = header.opcode;
        let body_size = (header.size.saturating_sub(2)) as u32;

        let mut buf = vec![0; body_size as usize];
        r.read_exact(&mut buf).await?;
        Self::read_opcodes(opcode, body_size, &buf)
    }

    #[cfg(all(feature = "sync", feature = "encryption"))]
    pub fn write_encrypted_server<W: std::io::Write>(&self, mut w: W, e: &mut EncrypterHalf) -> Result<(), std::io::Error> {
        match self {
            Self::MSG_MOVE_WORLDPORT_ACK => MSG_MOVE_WORLDPORT_ACK{}.write_encrypted_server(w, e),
            Self::MSG_PETITION_DECLINE(c) => c.write_encrypted_server(w, e),
            Self::MSG_TABARDVENDOR_ACTIVATE(c) => c.write_encrypted_server(w, e),
            Self::MSG_QUEST_PUSH_RESULT(c) => c.write_encrypted_server(w, e),
            Self::MSG_MOVE_WATER_WALK(c) => c.write_encrypted_server(w, e),
            Self::MSG_PETITION_RENAME(c) => c.write_encrypted_server(w, e),
            Self::SMSG_CHAR_CREATE(c) => c.write_encrypted_server(w, e),
            Self::SMSG_CHAR_ENUM(c) => c.write_encrypted_server(w, e),
            Self::SMSG_CHAR_DELETE(c) => c.write_encrypted_server(w, e),
            Self::SMSG_NEW_WORLD(c) => c.write_encrypted_server(w, e),
            Self::SMSG_TRANSFER_PENDING(c) => c.write_encrypted_server(w, e),
            Self::SMSG_TRANSFER_ABORTED(c) => c.write_encrypted_server(w, e),
            Self::SMSG_CHARACTER_LOGIN_FAILED(c) => c.write_encrypted_server(w, e),
            Self::SMSG_LOGIN_SETTIMESPEED(c) => c.write_encrypted_server(w, e),
            Self::SMSG_LOGOUT_RESPONSE(c) => c.write_encrypted_server(w, e),
            Self::SMSG_LOGOUT_COMPLETE => SMSG_LOGOUT_COMPLETE{}.write_encrypted_server(w, e),
            Self::SMSG_LOGOUT_CANCEL_ACK => SMSG_LOGOUT_CANCEL_ACK{}.write_encrypted_server(w, e),
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
            Self::SMSG_FRIEND_LIST(c) => c.write_encrypted_server(w, e),
            Self::SMSG_FRIEND_STATUS(c) => c.write_encrypted_server(w, e),
            Self::SMSG_IGNORE_LIST(c) => c.write_encrypted_server(w, e),
            Self::SMSG_GROUP_INVITE(c) => c.write_encrypted_server(w, e),
            Self::SMSG_GROUP_DECLINE(c) => c.write_encrypted_server(w, e),
            Self::SMSG_GROUP_UNINVITE => SMSG_GROUP_UNINVITE{}.write_encrypted_server(w, e),
            Self::SMSG_GROUP_SET_LEADER(c) => c.write_encrypted_server(w, e),
            Self::SMSG_GROUP_DESTROYED => SMSG_GROUP_DESTROYED{}.write_encrypted_server(w, e),
            Self::SMSG_GROUP_LIST(c) => c.write_encrypted_server(w, e),
            Self::SMSG_PARTY_MEMBER_STATS(c) => c.write_encrypted_server(w, e),
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
            Self::MSG_MOVE_TELEPORT_ACK(c) => c.write_encrypted_server(w, e),
            Self::MSG_MOVE_FALL_LAND(c) => c.write_encrypted_server(w, e),
            Self::MSG_MOVE_START_SWIM(c) => c.write_encrypted_server(w, e),
            Self::MSG_MOVE_STOP_SWIM(c) => c.write_encrypted_server(w, e),
            Self::MSG_MOVE_SET_FACING(c) => c.write_encrypted_server(w, e),
            Self::MSG_MOVE_SET_PITCH(c) => c.write_encrypted_server(w, e),
            Self::SMSG_MONSTER_MOVE(c) => c.write_encrypted_server(w, e),
            Self::SMSG_MOVE_WATER_WALK(c) => c.write_encrypted_server(w, e),
            Self::SMSG_MOVE_LAND_WALK(c) => c.write_encrypted_server(w, e),
            Self::SMSG_FORCE_RUN_SPEED_CHANGE(c) => c.write_encrypted_server(w, e),
            Self::SMSG_FORCE_RUN_BACK_SPEED_CHANGE(c) => c.write_encrypted_server(w, e),
            Self::SMSG_FORCE_SWIM_SPEED_CHANGE(c) => c.write_encrypted_server(w, e),
            Self::SMSG_FORCE_MOVE_ROOT(c) => c.write_encrypted_server(w, e),
            Self::SMSG_FORCE_MOVE_UNROOT(c) => c.write_encrypted_server(w, e),
            Self::MSG_MOVE_HEARTBEAT(c) => c.write_encrypted_server(w, e),
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
            Self::SMSG_OPEN_CONTAINER(c) => c.write_encrypted_server(w, e),
            Self::SMSG_INSPECT(c) => c.write_encrypted_server(w, e),
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
            Self::SMSG_CAST_RESULT(c) => c.write_encrypted_server(w, e),
            Self::SMSG_SPELL_START(c) => c.write_encrypted_server(w, e),
            Self::SMSG_SPELL_GO(c) => c.write_encrypted_server(w, e),
            Self::SMSG_SPELL_FAILURE(c) => c.write_encrypted_server(w, e),
            Self::SMSG_SPELL_COOLDOWN(c) => c.write_encrypted_server(w, e),
            Self::SMSG_COOLDOWN_EVENT(c) => c.write_encrypted_server(w, e),
            Self::SMSG_UPDATE_AURA_DURATION(c) => c.write_encrypted_server(w, e),
            Self::SMSG_PET_CAST_FAILED(c) => c.write_encrypted_server(w, e),
            Self::MSG_CHANNEL_START(c) => c.write_encrypted_server(w, e),
            Self::MSG_CHANNEL_UPDATE(c) => c.write_encrypted_server(w, e),
            Self::SMSG_AI_REACTION(c) => c.write_encrypted_server(w, e),
            Self::SMSG_ATTACKSTART(c) => c.write_encrypted_server(w, e),
            Self::SMSG_ATTACKSTOP(c) => c.write_encrypted_server(w, e),
            Self::SMSG_ATTACKSWING_NOTINRANGE => SMSG_ATTACKSWING_NOTINRANGE{}.write_encrypted_server(w, e),
            Self::SMSG_ATTACKSWING_BADFACING => SMSG_ATTACKSWING_BADFACING{}.write_encrypted_server(w, e),
            Self::SMSG_ATTACKSWING_NOTSTANDING => SMSG_ATTACKSWING_NOTSTANDING{}.write_encrypted_server(w, e),
            Self::SMSG_ATTACKSWING_DEADTARGET => SMSG_ATTACKSWING_DEADTARGET{}.write_encrypted_server(w, e),
            Self::SMSG_ATTACKSWING_CANT_ATTACK => SMSG_ATTACKSWING_CANT_ATTACK{}.write_encrypted_server(w, e),
            Self::SMSG_ATTACKERSTATEUPDATE(c) => c.write_encrypted_server(w, e),
            Self::SMSG_CANCEL_COMBAT => SMSG_CANCEL_COMBAT{}.write_encrypted_server(w, e),
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
            Self::SMSG_LOOT_CLEAR_MONEY => SMSG_LOOT_CLEAR_MONEY{}.write_encrypted_server(w, e),
            Self::SMSG_ITEM_PUSH_RESULT(c) => c.write_encrypted_server(w, e),
            Self::SMSG_DUEL_REQUESTED(c) => c.write_encrypted_server(w, e),
            Self::SMSG_DUEL_OUTOFBOUNDS => SMSG_DUEL_OUTOFBOUNDS{}.write_encrypted_server(w, e),
            Self::SMSG_DUEL_INBOUNDS => SMSG_DUEL_INBOUNDS{}.write_encrypted_server(w, e),
            Self::SMSG_DUEL_COMPLETE(c) => c.write_encrypted_server(w, e),
            Self::SMSG_DUEL_WINNER(c) => c.write_encrypted_server(w, e),
            Self::SMSG_MOUNTRESULT(c) => c.write_encrypted_server(w, e),
            Self::SMSG_DISMOUNTRESULT(c) => c.write_encrypted_server(w, e),
            Self::SMSG_MOUNTSPECIAL_ANIM(c) => c.write_encrypted_server(w, e),
            Self::SMSG_PET_TAME_FAILURE(c) => c.write_encrypted_server(w, e),
            Self::SMSG_PET_NAME_INVALID => SMSG_PET_NAME_INVALID{}.write_encrypted_server(w, e),
            Self::SMSG_PET_SPELLS(c) => c.write_encrypted_server(w, e),
            Self::SMSG_PET_MODE(c) => c.write_encrypted_server(w, e),
            Self::SMSG_GOSSIP_MESSAGE(c) => c.write_encrypted_server(w, e),
            Self::SMSG_GOSSIP_COMPLETE => SMSG_GOSSIP_COMPLETE{}.write_encrypted_server(w, e),
            Self::SMSG_NPC_TEXT_UPDATE(c) => c.write_encrypted_server(w, e),
            Self::SMSG_QUESTGIVER_STATUS(c) => c.write_encrypted_server(w, e),
            Self::SMSG_QUESTGIVER_QUEST_LIST(c) => c.write_encrypted_server(w, e),
            Self::SMSG_QUESTGIVER_QUEST_DETAILS(c) => c.write_encrypted_server(w, e),
            Self::SMSG_QUESTGIVER_REQUEST_ITEMS(c) => c.write_encrypted_server(w, e),
            Self::SMSG_QUESTGIVER_OFFER_REWARD(c) => c.write_encrypted_server(w, e),
            Self::SMSG_QUESTGIVER_QUEST_INVALID(c) => c.write_encrypted_server(w, e),
            Self::SMSG_QUESTGIVER_QUEST_COMPLETE(c) => c.write_encrypted_server(w, e),
            Self::SMSG_QUESTGIVER_QUEST_FAILED(c) => c.write_encrypted_server(w, e),
            Self::SMSG_QUESTLOG_FULL => SMSG_QUESTLOG_FULL{}.write_encrypted_server(w, e),
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
            Self::SMSG_NEW_TAXI_PATH => SMSG_NEW_TAXI_PATH{}.write_encrypted_server(w, e),
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
            Self::SMSG_FISH_NOT_HOOKED => SMSG_FISH_NOT_HOOKED{}.write_encrypted_server(w, e),
            Self::SMSG_FISH_ESCAPED => SMSG_FISH_ESCAPED{}.write_encrypted_server(w, e),
            Self::SMSG_NOTIFICATION(c) => c.write_encrypted_server(w, e),
            Self::SMSG_PLAYED_TIME(c) => c.write_encrypted_server(w, e),
            Self::SMSG_QUERY_TIME_RESPONSE(c) => c.write_encrypted_server(w, e),
            Self::SMSG_LOG_XPGAIN(c) => c.write_encrypted_server(w, e),
            Self::SMSG_LEVELUP_INFO(c) => c.write_encrypted_server(w, e),
            Self::MSG_MINIMAP_PING(c) => c.write_encrypted_server(w, e),
            Self::SMSG_RESISTLOG(c) => c.write_encrypted_server(w, e),
            Self::SMSG_ENCHANTMENTLOG(c) => c.write_encrypted_server(w, e),
            Self::SMSG_START_MIRROR_TIMER(c) => c.write_encrypted_server(w, e),
            Self::SMSG_PAUSE_MIRROR_TIMER(c) => c.write_encrypted_server(w, e),
            Self::SMSG_STOP_MIRROR_TIMER(c) => c.write_encrypted_server(w, e),
            Self::SMSG_PONG(c) => c.write_encrypted_server(w, e),
            Self::SMSG_CLEAR_COOLDOWN(c) => c.write_encrypted_server(w, e),
            Self::SMSG_GAMEOBJECT_PAGETEXT(c) => c.write_encrypted_server(w, e),
            Self::SMSG_SPELL_DELAYED(c) => c.write_encrypted_server(w, e),
            Self::SMSG_ITEM_TIME_UPDATE(c) => c.write_encrypted_server(w, e),
            Self::SMSG_ITEM_ENCHANT_TIME_UPDATE(c) => c.write_encrypted_server(w, e),
            Self::SMSG_AUTH_CHALLENGE(c) => c.write_encrypted_server(w, e),
            Self::SMSG_AUTH_RESPONSE(c) => c.write_encrypted_server(w, e),
            Self::MSG_SAVE_GUILD_EMBLEM(c) => c.write_encrypted_server(w, e),
            Self::SMSG_PLAY_SPELL_VISUAL(c) => c.write_encrypted_server(w, e),
            Self::SMSG_PARTYKILLLOG(c) => c.write_encrypted_server(w, e),
            Self::SMSG_COMPRESSED_UPDATE_OBJECT(c) => c.write_encrypted_server(w, e),
            Self::SMSG_PLAY_SPELL_IMPACT(c) => c.write_encrypted_server(w, e),
            Self::SMSG_EXPLORATION_EXPERIENCE(c) => c.write_encrypted_server(w, e),
            Self::MSG_RANDOM_ROLL(c) => c.write_encrypted_server(w, e),
            Self::SMSG_ENVIRONMENTAL_DAMAGE_LOG(c) => c.write_encrypted_server(w, e),
            Self::MSG_LOOKING_FOR_GROUP(c) => c.write_encrypted_server(w, e),
            Self::SMSG_REMOVED_SPELL(c) => c.write_encrypted_server(w, e),
            Self::SMSG_GMTICKET_CREATE(c) => c.write_encrypted_server(w, e),
            Self::SMSG_GMTICKET_UPDATETEXT(c) => c.write_encrypted_server(w, e),
            Self::SMSG_ACCOUNT_DATA_TIMES(c) => c.write_encrypted_server(w, e),
            Self::SMSG_GMTICKET_GETTICKET(c) => c.write_encrypted_server(w, e),
            Self::SMSG_GAMEOBJECT_SPAWN_ANIM(c) => c.write_encrypted_server(w, e),
            Self::SMSG_GAMEOBJECT_DESPAWN_ANIM(c) => c.write_encrypted_server(w, e),
            Self::MSG_CORPSE_QUERY(c) => c.write_encrypted_server(w, e),
            Self::SMSG_GMTICKET_DELETETICKET(c) => c.write_encrypted_server(w, e),
            Self::SMSG_CHAT_WRONG_FACTION => SMSG_CHAT_WRONG_FACTION{}.write_encrypted_server(w, e),
            Self::SMSG_GMTICKET_SYSTEMSTATUS(c) => c.write_encrypted_server(w, e),
            Self::SMSG_SET_REST_START(c) => c.write_encrypted_server(w, e),
            Self::SMSG_SPIRIT_HEALER_CONFIRM(c) => c.write_encrypted_server(w, e),
            Self::SMSG_GOSSIP_POI(c) => c.write_encrypted_server(w, e),
            Self::SMSG_LOGIN_VERIFY_WORLD(c) => c.write_encrypted_server(w, e),
            Self::SMSG_SEND_MAIL_RESULT(c) => c.write_encrypted_server(w, e),
            Self::SMSG_MAIL_LIST_RESULT(c) => c.write_encrypted_server(w, e),
            Self::SMSG_BATTLEFIELD_LIST(c) => c.write_encrypted_server(w, e),
            Self::SMSG_ITEM_TEXT_QUERY_RESPONSE(c) => c.write_encrypted_server(w, e),
            Self::SMSG_SPELLLOGMISS(c) => c.write_encrypted_server(w, e),
            Self::SMSG_SPELLLOGEXECUTE(c) => c.write_encrypted_server(w, e),
            Self::SMSG_PERIODICAURALOG(c) => c.write_encrypted_server(w, e),
            Self::SMSG_SPELLDAMAGESHIELD(c) => c.write_encrypted_server(w, e),
            Self::SMSG_SPELLNONMELEEDAMAGELOG(c) => c.write_encrypted_server(w, e),
            Self::SMSG_ZONE_UNDER_ATTACK(c) => c.write_encrypted_server(w, e),
            Self::MSG_AUCTION_HELLO(c) => c.write_encrypted_server(w, e),
            Self::SMSG_AUCTION_COMMAND_RESULT(c) => c.write_encrypted_server(w, e),
            Self::SMSG_AUCTION_LIST_RESULT(c) => c.write_encrypted_server(w, e),
            Self::SMSG_AUCTION_OWNER_LIST_RESULT(c) => c.write_encrypted_server(w, e),
            Self::SMSG_AUCTION_BIDDER_NOTIFICATION(c) => c.write_encrypted_server(w, e),
            Self::SMSG_AUCTION_OWNER_NOTIFICATION(c) => c.write_encrypted_server(w, e),
            Self::SMSG_PROCRESIST(c) => c.write_encrypted_server(w, e),
            Self::SMSG_DISPEL_FAILED(c) => c.write_encrypted_server(w, e),
            Self::SMSG_SPELLORDAMAGE_IMMUNE(c) => c.write_encrypted_server(w, e),
            Self::SMSG_AUCTION_BIDDER_LIST_RESULT(c) => c.write_encrypted_server(w, e),
            Self::SMSG_SET_FLAT_SPELL_MODIFIER(c) => c.write_encrypted_server(w, e),
            Self::SMSG_SET_PCT_SPELL_MODIFIER(c) => c.write_encrypted_server(w, e),
            Self::SMSG_CORPSE_RECLAIM_DELAY(c) => c.write_encrypted_server(w, e),
            Self::MSG_LIST_STABLED_PETS(c) => c.write_encrypted_server(w, e),
            Self::SMSG_STABLE_RESULT(c) => c.write_encrypted_server(w, e),
            Self::SMSG_PLAY_MUSIC(c) => c.write_encrypted_server(w, e),
            Self::SMSG_PLAY_OBJECT_SOUND(c) => c.write_encrypted_server(w, e),
            Self::SMSG_SPELLDISPELLOG(c) => c.write_encrypted_server(w, e),
            Self::MSG_QUERY_NEXT_MAIL_TIME(c) => c.write_encrypted_server(w, e),
            Self::SMSG_RECEIVED_MAIL(c) => c.write_encrypted_server(w, e),
            Self::SMSG_RAID_GROUP_ONLY(c) => c.write_encrypted_server(w, e),
            Self::SMSG_PVP_CREDIT(c) => c.write_encrypted_server(w, e),
            Self::SMSG_AUCTION_REMOVED_NOTIFICATION(c) => c.write_encrypted_server(w, e),
            Self::SMSG_SERVER_MESSAGE(c) => c.write_encrypted_server(w, e),
            Self::SMSG_MEETINGSTONE_SETQUEUE(c) => c.write_encrypted_server(w, e),
            Self::SMSG_MEETINGSTONE_COMPLETE => SMSG_MEETINGSTONE_COMPLETE{}.write_encrypted_server(w, e),
            Self::SMSG_MEETINGSTONE_IN_PROGRESS => SMSG_MEETINGSTONE_IN_PROGRESS{}.write_encrypted_server(w, e),
            Self::SMSG_MEETINGSTONE_MEMBER_ADDED(c) => c.write_encrypted_server(w, e),
            Self::SMSG_CANCEL_AUTO_REPEAT => SMSG_CANCEL_AUTO_REPEAT{}.write_encrypted_server(w, e),
            Self::SMSG_STANDSTATE_UPDATE(c) => c.write_encrypted_server(w, e),
            Self::SMSG_LOOT_ALL_PASSED(c) => c.write_encrypted_server(w, e),
            Self::SMSG_LOOT_ROLL_WON(c) => c.write_encrypted_server(w, e),
            Self::SMSG_LOOT_START_ROLL(c) => c.write_encrypted_server(w, e),
            Self::SMSG_LOOT_ROLL(c) => c.write_encrypted_server(w, e),
            Self::SMSG_LOOT_MASTER_LIST(c) => c.write_encrypted_server(w, e),
            Self::SMSG_SET_FORCED_REACTIONS(c) => c.write_encrypted_server(w, e),
            Self::SMSG_SPELL_FAILED_OTHER(c) => c.write_encrypted_server(w, e),
            Self::SMSG_GAMEOBJECT_RESET_STATE(c) => c.write_encrypted_server(w, e),
            Self::SMSG_CHAT_PLAYER_NOT_FOUND(c) => c.write_encrypted_server(w, e),
            Self::MSG_TALENT_WIPE_CONFIRM(c) => c.write_encrypted_server(w, e),
            Self::SMSG_SUMMON_REQUEST(c) => c.write_encrypted_server(w, e),
            Self::SMSG_MONSTER_MOVE_TRANSPORT(c) => c.write_encrypted_server(w, e),
            Self::SMSG_PET_BROKEN => SMSG_PET_BROKEN{}.write_encrypted_server(w, e),
            Self::MSG_MOVE_FEATHER_FALL(c) => c.write_encrypted_server(w, e),
            Self::SMSG_FEIGN_DEATH_RESISTED => SMSG_FEIGN_DEATH_RESISTED{}.write_encrypted_server(w, e),
            Self::SMSG_DUEL_COUNTDOWN(c) => c.write_encrypted_server(w, e),
            Self::SMSG_AREA_TRIGGER_MESSAGE(c) => c.write_encrypted_server(w, e),
            Self::SMSG_MEETINGSTONE_JOINFAILED(c) => c.write_encrypted_server(w, e),
            Self::SMSG_PLAYER_SKINNED(c) => c.write_encrypted_server(w, e),
            Self::SMSG_DURABILITY_DAMAGE_DEATH => SMSG_DURABILITY_DAMAGE_DEATH{}.write_encrypted_server(w, e),
            Self::SMSG_INIT_WORLD_STATES(c) => c.write_encrypted_server(w, e),
            Self::SMSG_UPDATE_WORLD_STATE(c) => c.write_encrypted_server(w, e),
            Self::SMSG_ITEM_NAME_QUERY_RESPONSE(c) => c.write_encrypted_server(w, e),
            Self::SMSG_PET_ACTION_FEEDBACK(c) => c.write_encrypted_server(w, e),
            Self::SMSG_CHAR_RENAME(c) => c.write_encrypted_server(w, e),
            Self::SMSG_INSTANCE_SAVE_CREATED(c) => c.write_encrypted_server(w, e),
            Self::SMSG_RAID_INSTANCE_INFO(c) => c.write_encrypted_server(w, e),
            Self::SMSG_PLAY_SOUND(c) => c.write_encrypted_server(w, e),
            Self::SMSG_BATTLEFIELD_STATUS(c) => c.write_encrypted_server(w, e),
            Self::MSG_INSPECT_HONOR_STATS(c) => c.write_encrypted_server(w, e),
            Self::SMSG_FORCE_WALK_SPEED_CHANGE(c) => c.write_encrypted_server(w, e),
            Self::SMSG_FORCE_SWIM_BACK_SPEED_CHANGE(c) => c.write_encrypted_server(w, e),
            Self::SMSG_FORCE_TURN_RATE_CHANGE(c) => c.write_encrypted_server(w, e),
            Self::MSG_PVP_LOG_DATA(c) => c.write_encrypted_server(w, e),
            Self::SMSG_AREA_SPIRIT_HEALER_TIME(c) => c.write_encrypted_server(w, e),
            Self::SMSG_WARDEN_DATA(c) => c.write_encrypted_server(w, e),
            Self::SMSG_GROUP_JOINED_BATTLEGROUND(c) => c.write_encrypted_server(w, e),
            Self::MSG_BATTLEGROUND_PLAYER_POSITIONS(c) => c.write_encrypted_server(w, e),
            Self::SMSG_BINDER_CONFIRM(c) => c.write_encrypted_server(w, e),
            Self::SMSG_BATTLEGROUND_PLAYER_JOINED(c) => c.write_encrypted_server(w, e),
            Self::SMSG_BATTLEGROUND_PLAYER_LEFT(c) => c.write_encrypted_server(w, e),
            Self::SMSG_ADDON_INFO(c) => c.write_encrypted_server(w, e),
            Self::SMSG_PET_UNLEARN_CONFIRM(c) => c.write_encrypted_server(w, e),
            Self::SMSG_PARTY_MEMBER_STATS_FULL(c) => c.write_encrypted_server(w, e),
            Self::SMSG_WEATHER(c) => c.write_encrypted_server(w, e),
            Self::SMSG_RAID_INSTANCE_MESSAGE(c) => c.write_encrypted_server(w, e),
            Self::SMSG_COMPRESSED_MOVES(c) => c.write_encrypted_server(w, e),
            Self::SMSG_CHAT_RESTRICTED => SMSG_CHAT_RESTRICTED{}.write_encrypted_server(w, e),
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
            Self::MSG_MOVE_TIME_SKIPPED(c) => c.write_encrypted_server(w, e),
            Self::SMSG_SPLINE_MOVE_ROOT(c) => c.write_encrypted_server(w, e),
            Self::SMSG_INVALIDATE_PLAYER(c) => c.write_encrypted_server(w, e),
            Self::SMSG_INSTANCE_RESET(c) => c.write_encrypted_server(w, e),
            Self::SMSG_INSTANCE_RESET_FAILED(c) => c.write_encrypted_server(w, e),
            Self::SMSG_UPDATE_LAST_INSTANCE(c) => c.write_encrypted_server(w, e),
            Self::MSG_RAID_TARGET_UPDATE(c) => c.write_encrypted_server(w, e),
            Self::MSG_RAID_READY_CHECK(c) => c.write_encrypted_server(w, e),
            Self::SMSG_PET_ACTION_SOUND(c) => c.write_encrypted_server(w, e),
            Self::SMSG_PET_DISMISS_SOUND(c) => c.write_encrypted_server(w, e),
            Self::SMSG_GM_TICKET_STATUS_UPDATE(c) => c.write_encrypted_server(w, e),
            Self::SMSG_UPDATE_INSTANCE_OWNERSHIP(c) => c.write_encrypted_server(w, e),
            Self::SMSG_SPELLINSTAKILLLOG(c) => c.write_encrypted_server(w, e),
            Self::SMSG_SPELL_UPDATE_CHAIN_TARGETS(c) => c.write_encrypted_server(w, e),
            Self::SMSG_EXPECTED_SPAM_RECORDS(c) => c.write_encrypted_server(w, e),
            Self::SMSG_DEFENSE_MESSAGE(c) => c.write_encrypted_server(w, e),
        }
    }

    #[cfg(feature = "sync")]
    pub fn write_unencrypted_server<W: std::io::Write>(&self, mut w: W) -> Result<(), std::io::Error> {
        match self {
            Self::MSG_MOVE_WORLDPORT_ACK => MSG_MOVE_WORLDPORT_ACK{}.write_unencrypted_server(w),
            Self::MSG_PETITION_DECLINE(c) => c.write_unencrypted_server(w),
            Self::MSG_TABARDVENDOR_ACTIVATE(c) => c.write_unencrypted_server(w),
            Self::MSG_QUEST_PUSH_RESULT(c) => c.write_unencrypted_server(w),
            Self::MSG_MOVE_WATER_WALK(c) => c.write_unencrypted_server(w),
            Self::MSG_PETITION_RENAME(c) => c.write_unencrypted_server(w),
            Self::SMSG_CHAR_CREATE(c) => c.write_unencrypted_server(w),
            Self::SMSG_CHAR_ENUM(c) => c.write_unencrypted_server(w),
            Self::SMSG_CHAR_DELETE(c) => c.write_unencrypted_server(w),
            Self::SMSG_NEW_WORLD(c) => c.write_unencrypted_server(w),
            Self::SMSG_TRANSFER_PENDING(c) => c.write_unencrypted_server(w),
            Self::SMSG_TRANSFER_ABORTED(c) => c.write_unencrypted_server(w),
            Self::SMSG_CHARACTER_LOGIN_FAILED(c) => c.write_unencrypted_server(w),
            Self::SMSG_LOGIN_SETTIMESPEED(c) => c.write_unencrypted_server(w),
            Self::SMSG_LOGOUT_RESPONSE(c) => c.write_unencrypted_server(w),
            Self::SMSG_LOGOUT_COMPLETE => SMSG_LOGOUT_COMPLETE{}.write_unencrypted_server(w),
            Self::SMSG_LOGOUT_CANCEL_ACK => SMSG_LOGOUT_CANCEL_ACK{}.write_unencrypted_server(w),
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
            Self::SMSG_FRIEND_LIST(c) => c.write_unencrypted_server(w),
            Self::SMSG_FRIEND_STATUS(c) => c.write_unencrypted_server(w),
            Self::SMSG_IGNORE_LIST(c) => c.write_unencrypted_server(w),
            Self::SMSG_GROUP_INVITE(c) => c.write_unencrypted_server(w),
            Self::SMSG_GROUP_DECLINE(c) => c.write_unencrypted_server(w),
            Self::SMSG_GROUP_UNINVITE => SMSG_GROUP_UNINVITE{}.write_unencrypted_server(w),
            Self::SMSG_GROUP_SET_LEADER(c) => c.write_unencrypted_server(w),
            Self::SMSG_GROUP_DESTROYED => SMSG_GROUP_DESTROYED{}.write_unencrypted_server(w),
            Self::SMSG_GROUP_LIST(c) => c.write_unencrypted_server(w),
            Self::SMSG_PARTY_MEMBER_STATS(c) => c.write_unencrypted_server(w),
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
            Self::MSG_MOVE_TELEPORT_ACK(c) => c.write_unencrypted_server(w),
            Self::MSG_MOVE_FALL_LAND(c) => c.write_unencrypted_server(w),
            Self::MSG_MOVE_START_SWIM(c) => c.write_unencrypted_server(w),
            Self::MSG_MOVE_STOP_SWIM(c) => c.write_unencrypted_server(w),
            Self::MSG_MOVE_SET_FACING(c) => c.write_unencrypted_server(w),
            Self::MSG_MOVE_SET_PITCH(c) => c.write_unencrypted_server(w),
            Self::SMSG_MONSTER_MOVE(c) => c.write_unencrypted_server(w),
            Self::SMSG_MOVE_WATER_WALK(c) => c.write_unencrypted_server(w),
            Self::SMSG_MOVE_LAND_WALK(c) => c.write_unencrypted_server(w),
            Self::SMSG_FORCE_RUN_SPEED_CHANGE(c) => c.write_unencrypted_server(w),
            Self::SMSG_FORCE_RUN_BACK_SPEED_CHANGE(c) => c.write_unencrypted_server(w),
            Self::SMSG_FORCE_SWIM_SPEED_CHANGE(c) => c.write_unencrypted_server(w),
            Self::SMSG_FORCE_MOVE_ROOT(c) => c.write_unencrypted_server(w),
            Self::SMSG_FORCE_MOVE_UNROOT(c) => c.write_unencrypted_server(w),
            Self::MSG_MOVE_HEARTBEAT(c) => c.write_unencrypted_server(w),
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
            Self::SMSG_OPEN_CONTAINER(c) => c.write_unencrypted_server(w),
            Self::SMSG_INSPECT(c) => c.write_unencrypted_server(w),
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
            Self::SMSG_CAST_RESULT(c) => c.write_unencrypted_server(w),
            Self::SMSG_SPELL_START(c) => c.write_unencrypted_server(w),
            Self::SMSG_SPELL_GO(c) => c.write_unencrypted_server(w),
            Self::SMSG_SPELL_FAILURE(c) => c.write_unencrypted_server(w),
            Self::SMSG_SPELL_COOLDOWN(c) => c.write_unencrypted_server(w),
            Self::SMSG_COOLDOWN_EVENT(c) => c.write_unencrypted_server(w),
            Self::SMSG_UPDATE_AURA_DURATION(c) => c.write_unencrypted_server(w),
            Self::SMSG_PET_CAST_FAILED(c) => c.write_unencrypted_server(w),
            Self::MSG_CHANNEL_START(c) => c.write_unencrypted_server(w),
            Self::MSG_CHANNEL_UPDATE(c) => c.write_unencrypted_server(w),
            Self::SMSG_AI_REACTION(c) => c.write_unencrypted_server(w),
            Self::SMSG_ATTACKSTART(c) => c.write_unencrypted_server(w),
            Self::SMSG_ATTACKSTOP(c) => c.write_unencrypted_server(w),
            Self::SMSG_ATTACKSWING_NOTINRANGE => SMSG_ATTACKSWING_NOTINRANGE{}.write_unencrypted_server(w),
            Self::SMSG_ATTACKSWING_BADFACING => SMSG_ATTACKSWING_BADFACING{}.write_unencrypted_server(w),
            Self::SMSG_ATTACKSWING_NOTSTANDING => SMSG_ATTACKSWING_NOTSTANDING{}.write_unencrypted_server(w),
            Self::SMSG_ATTACKSWING_DEADTARGET => SMSG_ATTACKSWING_DEADTARGET{}.write_unencrypted_server(w),
            Self::SMSG_ATTACKSWING_CANT_ATTACK => SMSG_ATTACKSWING_CANT_ATTACK{}.write_unencrypted_server(w),
            Self::SMSG_ATTACKERSTATEUPDATE(c) => c.write_unencrypted_server(w),
            Self::SMSG_CANCEL_COMBAT => SMSG_CANCEL_COMBAT{}.write_unencrypted_server(w),
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
            Self::SMSG_LOOT_CLEAR_MONEY => SMSG_LOOT_CLEAR_MONEY{}.write_unencrypted_server(w),
            Self::SMSG_ITEM_PUSH_RESULT(c) => c.write_unencrypted_server(w),
            Self::SMSG_DUEL_REQUESTED(c) => c.write_unencrypted_server(w),
            Self::SMSG_DUEL_OUTOFBOUNDS => SMSG_DUEL_OUTOFBOUNDS{}.write_unencrypted_server(w),
            Self::SMSG_DUEL_INBOUNDS => SMSG_DUEL_INBOUNDS{}.write_unencrypted_server(w),
            Self::SMSG_DUEL_COMPLETE(c) => c.write_unencrypted_server(w),
            Self::SMSG_DUEL_WINNER(c) => c.write_unencrypted_server(w),
            Self::SMSG_MOUNTRESULT(c) => c.write_unencrypted_server(w),
            Self::SMSG_DISMOUNTRESULT(c) => c.write_unencrypted_server(w),
            Self::SMSG_MOUNTSPECIAL_ANIM(c) => c.write_unencrypted_server(w),
            Self::SMSG_PET_TAME_FAILURE(c) => c.write_unencrypted_server(w),
            Self::SMSG_PET_NAME_INVALID => SMSG_PET_NAME_INVALID{}.write_unencrypted_server(w),
            Self::SMSG_PET_SPELLS(c) => c.write_unencrypted_server(w),
            Self::SMSG_PET_MODE(c) => c.write_unencrypted_server(w),
            Self::SMSG_GOSSIP_MESSAGE(c) => c.write_unencrypted_server(w),
            Self::SMSG_GOSSIP_COMPLETE => SMSG_GOSSIP_COMPLETE{}.write_unencrypted_server(w),
            Self::SMSG_NPC_TEXT_UPDATE(c) => c.write_unencrypted_server(w),
            Self::SMSG_QUESTGIVER_STATUS(c) => c.write_unencrypted_server(w),
            Self::SMSG_QUESTGIVER_QUEST_LIST(c) => c.write_unencrypted_server(w),
            Self::SMSG_QUESTGIVER_QUEST_DETAILS(c) => c.write_unencrypted_server(w),
            Self::SMSG_QUESTGIVER_REQUEST_ITEMS(c) => c.write_unencrypted_server(w),
            Self::SMSG_QUESTGIVER_OFFER_REWARD(c) => c.write_unencrypted_server(w),
            Self::SMSG_QUESTGIVER_QUEST_INVALID(c) => c.write_unencrypted_server(w),
            Self::SMSG_QUESTGIVER_QUEST_COMPLETE(c) => c.write_unencrypted_server(w),
            Self::SMSG_QUESTGIVER_QUEST_FAILED(c) => c.write_unencrypted_server(w),
            Self::SMSG_QUESTLOG_FULL => SMSG_QUESTLOG_FULL{}.write_unencrypted_server(w),
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
            Self::SMSG_NEW_TAXI_PATH => SMSG_NEW_TAXI_PATH{}.write_unencrypted_server(w),
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
            Self::SMSG_FISH_NOT_HOOKED => SMSG_FISH_NOT_HOOKED{}.write_unencrypted_server(w),
            Self::SMSG_FISH_ESCAPED => SMSG_FISH_ESCAPED{}.write_unencrypted_server(w),
            Self::SMSG_NOTIFICATION(c) => c.write_unencrypted_server(w),
            Self::SMSG_PLAYED_TIME(c) => c.write_unencrypted_server(w),
            Self::SMSG_QUERY_TIME_RESPONSE(c) => c.write_unencrypted_server(w),
            Self::SMSG_LOG_XPGAIN(c) => c.write_unencrypted_server(w),
            Self::SMSG_LEVELUP_INFO(c) => c.write_unencrypted_server(w),
            Self::MSG_MINIMAP_PING(c) => c.write_unencrypted_server(w),
            Self::SMSG_RESISTLOG(c) => c.write_unencrypted_server(w),
            Self::SMSG_ENCHANTMENTLOG(c) => c.write_unencrypted_server(w),
            Self::SMSG_START_MIRROR_TIMER(c) => c.write_unencrypted_server(w),
            Self::SMSG_PAUSE_MIRROR_TIMER(c) => c.write_unencrypted_server(w),
            Self::SMSG_STOP_MIRROR_TIMER(c) => c.write_unencrypted_server(w),
            Self::SMSG_PONG(c) => c.write_unencrypted_server(w),
            Self::SMSG_CLEAR_COOLDOWN(c) => c.write_unencrypted_server(w),
            Self::SMSG_GAMEOBJECT_PAGETEXT(c) => c.write_unencrypted_server(w),
            Self::SMSG_SPELL_DELAYED(c) => c.write_unencrypted_server(w),
            Self::SMSG_ITEM_TIME_UPDATE(c) => c.write_unencrypted_server(w),
            Self::SMSG_ITEM_ENCHANT_TIME_UPDATE(c) => c.write_unencrypted_server(w),
            Self::SMSG_AUTH_CHALLENGE(c) => c.write_unencrypted_server(w),
            Self::SMSG_AUTH_RESPONSE(c) => c.write_unencrypted_server(w),
            Self::MSG_SAVE_GUILD_EMBLEM(c) => c.write_unencrypted_server(w),
            Self::SMSG_PLAY_SPELL_VISUAL(c) => c.write_unencrypted_server(w),
            Self::SMSG_PARTYKILLLOG(c) => c.write_unencrypted_server(w),
            Self::SMSG_COMPRESSED_UPDATE_OBJECT(c) => c.write_unencrypted_server(w),
            Self::SMSG_PLAY_SPELL_IMPACT(c) => c.write_unencrypted_server(w),
            Self::SMSG_EXPLORATION_EXPERIENCE(c) => c.write_unencrypted_server(w),
            Self::MSG_RANDOM_ROLL(c) => c.write_unencrypted_server(w),
            Self::SMSG_ENVIRONMENTAL_DAMAGE_LOG(c) => c.write_unencrypted_server(w),
            Self::MSG_LOOKING_FOR_GROUP(c) => c.write_unencrypted_server(w),
            Self::SMSG_REMOVED_SPELL(c) => c.write_unencrypted_server(w),
            Self::SMSG_GMTICKET_CREATE(c) => c.write_unencrypted_server(w),
            Self::SMSG_GMTICKET_UPDATETEXT(c) => c.write_unencrypted_server(w),
            Self::SMSG_ACCOUNT_DATA_TIMES(c) => c.write_unencrypted_server(w),
            Self::SMSG_GMTICKET_GETTICKET(c) => c.write_unencrypted_server(w),
            Self::SMSG_GAMEOBJECT_SPAWN_ANIM(c) => c.write_unencrypted_server(w),
            Self::SMSG_GAMEOBJECT_DESPAWN_ANIM(c) => c.write_unencrypted_server(w),
            Self::MSG_CORPSE_QUERY(c) => c.write_unencrypted_server(w),
            Self::SMSG_GMTICKET_DELETETICKET(c) => c.write_unencrypted_server(w),
            Self::SMSG_CHAT_WRONG_FACTION => SMSG_CHAT_WRONG_FACTION{}.write_unencrypted_server(w),
            Self::SMSG_GMTICKET_SYSTEMSTATUS(c) => c.write_unencrypted_server(w),
            Self::SMSG_SET_REST_START(c) => c.write_unencrypted_server(w),
            Self::SMSG_SPIRIT_HEALER_CONFIRM(c) => c.write_unencrypted_server(w),
            Self::SMSG_GOSSIP_POI(c) => c.write_unencrypted_server(w),
            Self::SMSG_LOGIN_VERIFY_WORLD(c) => c.write_unencrypted_server(w),
            Self::SMSG_SEND_MAIL_RESULT(c) => c.write_unencrypted_server(w),
            Self::SMSG_MAIL_LIST_RESULT(c) => c.write_unencrypted_server(w),
            Self::SMSG_BATTLEFIELD_LIST(c) => c.write_unencrypted_server(w),
            Self::SMSG_ITEM_TEXT_QUERY_RESPONSE(c) => c.write_unencrypted_server(w),
            Self::SMSG_SPELLLOGMISS(c) => c.write_unencrypted_server(w),
            Self::SMSG_SPELLLOGEXECUTE(c) => c.write_unencrypted_server(w),
            Self::SMSG_PERIODICAURALOG(c) => c.write_unencrypted_server(w),
            Self::SMSG_SPELLDAMAGESHIELD(c) => c.write_unencrypted_server(w),
            Self::SMSG_SPELLNONMELEEDAMAGELOG(c) => c.write_unencrypted_server(w),
            Self::SMSG_ZONE_UNDER_ATTACK(c) => c.write_unencrypted_server(w),
            Self::MSG_AUCTION_HELLO(c) => c.write_unencrypted_server(w),
            Self::SMSG_AUCTION_COMMAND_RESULT(c) => c.write_unencrypted_server(w),
            Self::SMSG_AUCTION_LIST_RESULT(c) => c.write_unencrypted_server(w),
            Self::SMSG_AUCTION_OWNER_LIST_RESULT(c) => c.write_unencrypted_server(w),
            Self::SMSG_AUCTION_BIDDER_NOTIFICATION(c) => c.write_unencrypted_server(w),
            Self::SMSG_AUCTION_OWNER_NOTIFICATION(c) => c.write_unencrypted_server(w),
            Self::SMSG_PROCRESIST(c) => c.write_unencrypted_server(w),
            Self::SMSG_DISPEL_FAILED(c) => c.write_unencrypted_server(w),
            Self::SMSG_SPELLORDAMAGE_IMMUNE(c) => c.write_unencrypted_server(w),
            Self::SMSG_AUCTION_BIDDER_LIST_RESULT(c) => c.write_unencrypted_server(w),
            Self::SMSG_SET_FLAT_SPELL_MODIFIER(c) => c.write_unencrypted_server(w),
            Self::SMSG_SET_PCT_SPELL_MODIFIER(c) => c.write_unencrypted_server(w),
            Self::SMSG_CORPSE_RECLAIM_DELAY(c) => c.write_unencrypted_server(w),
            Self::MSG_LIST_STABLED_PETS(c) => c.write_unencrypted_server(w),
            Self::SMSG_STABLE_RESULT(c) => c.write_unencrypted_server(w),
            Self::SMSG_PLAY_MUSIC(c) => c.write_unencrypted_server(w),
            Self::SMSG_PLAY_OBJECT_SOUND(c) => c.write_unencrypted_server(w),
            Self::SMSG_SPELLDISPELLOG(c) => c.write_unencrypted_server(w),
            Self::MSG_QUERY_NEXT_MAIL_TIME(c) => c.write_unencrypted_server(w),
            Self::SMSG_RECEIVED_MAIL(c) => c.write_unencrypted_server(w),
            Self::SMSG_RAID_GROUP_ONLY(c) => c.write_unencrypted_server(w),
            Self::SMSG_PVP_CREDIT(c) => c.write_unencrypted_server(w),
            Self::SMSG_AUCTION_REMOVED_NOTIFICATION(c) => c.write_unencrypted_server(w),
            Self::SMSG_SERVER_MESSAGE(c) => c.write_unencrypted_server(w),
            Self::SMSG_MEETINGSTONE_SETQUEUE(c) => c.write_unencrypted_server(w),
            Self::SMSG_MEETINGSTONE_COMPLETE => SMSG_MEETINGSTONE_COMPLETE{}.write_unencrypted_server(w),
            Self::SMSG_MEETINGSTONE_IN_PROGRESS => SMSG_MEETINGSTONE_IN_PROGRESS{}.write_unencrypted_server(w),
            Self::SMSG_MEETINGSTONE_MEMBER_ADDED(c) => c.write_unencrypted_server(w),
            Self::SMSG_CANCEL_AUTO_REPEAT => SMSG_CANCEL_AUTO_REPEAT{}.write_unencrypted_server(w),
            Self::SMSG_STANDSTATE_UPDATE(c) => c.write_unencrypted_server(w),
            Self::SMSG_LOOT_ALL_PASSED(c) => c.write_unencrypted_server(w),
            Self::SMSG_LOOT_ROLL_WON(c) => c.write_unencrypted_server(w),
            Self::SMSG_LOOT_START_ROLL(c) => c.write_unencrypted_server(w),
            Self::SMSG_LOOT_ROLL(c) => c.write_unencrypted_server(w),
            Self::SMSG_LOOT_MASTER_LIST(c) => c.write_unencrypted_server(w),
            Self::SMSG_SET_FORCED_REACTIONS(c) => c.write_unencrypted_server(w),
            Self::SMSG_SPELL_FAILED_OTHER(c) => c.write_unencrypted_server(w),
            Self::SMSG_GAMEOBJECT_RESET_STATE(c) => c.write_unencrypted_server(w),
            Self::SMSG_CHAT_PLAYER_NOT_FOUND(c) => c.write_unencrypted_server(w),
            Self::MSG_TALENT_WIPE_CONFIRM(c) => c.write_unencrypted_server(w),
            Self::SMSG_SUMMON_REQUEST(c) => c.write_unencrypted_server(w),
            Self::SMSG_MONSTER_MOVE_TRANSPORT(c) => c.write_unencrypted_server(w),
            Self::SMSG_PET_BROKEN => SMSG_PET_BROKEN{}.write_unencrypted_server(w),
            Self::MSG_MOVE_FEATHER_FALL(c) => c.write_unencrypted_server(w),
            Self::SMSG_FEIGN_DEATH_RESISTED => SMSG_FEIGN_DEATH_RESISTED{}.write_unencrypted_server(w),
            Self::SMSG_DUEL_COUNTDOWN(c) => c.write_unencrypted_server(w),
            Self::SMSG_AREA_TRIGGER_MESSAGE(c) => c.write_unencrypted_server(w),
            Self::SMSG_MEETINGSTONE_JOINFAILED(c) => c.write_unencrypted_server(w),
            Self::SMSG_PLAYER_SKINNED(c) => c.write_unencrypted_server(w),
            Self::SMSG_DURABILITY_DAMAGE_DEATH => SMSG_DURABILITY_DAMAGE_DEATH{}.write_unencrypted_server(w),
            Self::SMSG_INIT_WORLD_STATES(c) => c.write_unencrypted_server(w),
            Self::SMSG_UPDATE_WORLD_STATE(c) => c.write_unencrypted_server(w),
            Self::SMSG_ITEM_NAME_QUERY_RESPONSE(c) => c.write_unencrypted_server(w),
            Self::SMSG_PET_ACTION_FEEDBACK(c) => c.write_unencrypted_server(w),
            Self::SMSG_CHAR_RENAME(c) => c.write_unencrypted_server(w),
            Self::SMSG_INSTANCE_SAVE_CREATED(c) => c.write_unencrypted_server(w),
            Self::SMSG_RAID_INSTANCE_INFO(c) => c.write_unencrypted_server(w),
            Self::SMSG_PLAY_SOUND(c) => c.write_unencrypted_server(w),
            Self::SMSG_BATTLEFIELD_STATUS(c) => c.write_unencrypted_server(w),
            Self::MSG_INSPECT_HONOR_STATS(c) => c.write_unencrypted_server(w),
            Self::SMSG_FORCE_WALK_SPEED_CHANGE(c) => c.write_unencrypted_server(w),
            Self::SMSG_FORCE_SWIM_BACK_SPEED_CHANGE(c) => c.write_unencrypted_server(w),
            Self::SMSG_FORCE_TURN_RATE_CHANGE(c) => c.write_unencrypted_server(w),
            Self::MSG_PVP_LOG_DATA(c) => c.write_unencrypted_server(w),
            Self::SMSG_AREA_SPIRIT_HEALER_TIME(c) => c.write_unencrypted_server(w),
            Self::SMSG_WARDEN_DATA(c) => c.write_unencrypted_server(w),
            Self::SMSG_GROUP_JOINED_BATTLEGROUND(c) => c.write_unencrypted_server(w),
            Self::MSG_BATTLEGROUND_PLAYER_POSITIONS(c) => c.write_unencrypted_server(w),
            Self::SMSG_BINDER_CONFIRM(c) => c.write_unencrypted_server(w),
            Self::SMSG_BATTLEGROUND_PLAYER_JOINED(c) => c.write_unencrypted_server(w),
            Self::SMSG_BATTLEGROUND_PLAYER_LEFT(c) => c.write_unencrypted_server(w),
            Self::SMSG_ADDON_INFO(c) => c.write_unencrypted_server(w),
            Self::SMSG_PET_UNLEARN_CONFIRM(c) => c.write_unencrypted_server(w),
            Self::SMSG_PARTY_MEMBER_STATS_FULL(c) => c.write_unencrypted_server(w),
            Self::SMSG_WEATHER(c) => c.write_unencrypted_server(w),
            Self::SMSG_RAID_INSTANCE_MESSAGE(c) => c.write_unencrypted_server(w),
            Self::SMSG_COMPRESSED_MOVES(c) => c.write_unencrypted_server(w),
            Self::SMSG_CHAT_RESTRICTED => SMSG_CHAT_RESTRICTED{}.write_unencrypted_server(w),
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
            Self::MSG_MOVE_TIME_SKIPPED(c) => c.write_unencrypted_server(w),
            Self::SMSG_SPLINE_MOVE_ROOT(c) => c.write_unencrypted_server(w),
            Self::SMSG_INVALIDATE_PLAYER(c) => c.write_unencrypted_server(w),
            Self::SMSG_INSTANCE_RESET(c) => c.write_unencrypted_server(w),
            Self::SMSG_INSTANCE_RESET_FAILED(c) => c.write_unencrypted_server(w),
            Self::SMSG_UPDATE_LAST_INSTANCE(c) => c.write_unencrypted_server(w),
            Self::MSG_RAID_TARGET_UPDATE(c) => c.write_unencrypted_server(w),
            Self::MSG_RAID_READY_CHECK(c) => c.write_unencrypted_server(w),
            Self::SMSG_PET_ACTION_SOUND(c) => c.write_unencrypted_server(w),
            Self::SMSG_PET_DISMISS_SOUND(c) => c.write_unencrypted_server(w),
            Self::SMSG_GM_TICKET_STATUS_UPDATE(c) => c.write_unencrypted_server(w),
            Self::SMSG_UPDATE_INSTANCE_OWNERSHIP(c) => c.write_unencrypted_server(w),
            Self::SMSG_SPELLINSTAKILLLOG(c) => c.write_unencrypted_server(w),
            Self::SMSG_SPELL_UPDATE_CHAIN_TARGETS(c) => c.write_unencrypted_server(w),
            Self::SMSG_EXPECTED_SPAM_RECORDS(c) => c.write_unencrypted_server(w),
            Self::SMSG_DEFENSE_MESSAGE(c) => c.write_unencrypted_server(w),
        }
    }

    #[cfg(all(feature = "tokio", feature = "encryption"))]
    pub async fn tokio_write_encrypted_server<W: tokio::io::AsyncWriteExt + Unpin + Send>(&self, mut w: W, e: &mut EncrypterHalf) -> Result<(), std::io::Error> {
        match self {
            Self::MSG_MOVE_WORLDPORT_ACK => MSG_MOVE_WORLDPORT_ACK{}.tokio_write_encrypted_server(w, e).await,
            Self::MSG_PETITION_DECLINE(c) => c.tokio_write_encrypted_server(w, e).await,
            Self::MSG_TABARDVENDOR_ACTIVATE(c) => c.tokio_write_encrypted_server(w, e).await,
            Self::MSG_QUEST_PUSH_RESULT(c) => c.tokio_write_encrypted_server(w, e).await,
            Self::MSG_MOVE_WATER_WALK(c) => c.tokio_write_encrypted_server(w, e).await,
            Self::MSG_PETITION_RENAME(c) => c.tokio_write_encrypted_server(w, e).await,
            Self::SMSG_CHAR_CREATE(c) => c.tokio_write_encrypted_server(w, e).await,
            Self::SMSG_CHAR_ENUM(c) => c.tokio_write_encrypted_server(w, e).await,
            Self::SMSG_CHAR_DELETE(c) => c.tokio_write_encrypted_server(w, e).await,
            Self::SMSG_NEW_WORLD(c) => c.tokio_write_encrypted_server(w, e).await,
            Self::SMSG_TRANSFER_PENDING(c) => c.tokio_write_encrypted_server(w, e).await,
            Self::SMSG_TRANSFER_ABORTED(c) => c.tokio_write_encrypted_server(w, e).await,
            Self::SMSG_CHARACTER_LOGIN_FAILED(c) => c.tokio_write_encrypted_server(w, e).await,
            Self::SMSG_LOGIN_SETTIMESPEED(c) => c.tokio_write_encrypted_server(w, e).await,
            Self::SMSG_LOGOUT_RESPONSE(c) => c.tokio_write_encrypted_server(w, e).await,
            Self::SMSG_LOGOUT_COMPLETE => SMSG_LOGOUT_COMPLETE{}.tokio_write_encrypted_server(w, e).await,
            Self::SMSG_LOGOUT_CANCEL_ACK => SMSG_LOGOUT_CANCEL_ACK{}.tokio_write_encrypted_server(w, e).await,
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
            Self::SMSG_FRIEND_LIST(c) => c.tokio_write_encrypted_server(w, e).await,
            Self::SMSG_FRIEND_STATUS(c) => c.tokio_write_encrypted_server(w, e).await,
            Self::SMSG_IGNORE_LIST(c) => c.tokio_write_encrypted_server(w, e).await,
            Self::SMSG_GROUP_INVITE(c) => c.tokio_write_encrypted_server(w, e).await,
            Self::SMSG_GROUP_DECLINE(c) => c.tokio_write_encrypted_server(w, e).await,
            Self::SMSG_GROUP_UNINVITE => SMSG_GROUP_UNINVITE{}.tokio_write_encrypted_server(w, e).await,
            Self::SMSG_GROUP_SET_LEADER(c) => c.tokio_write_encrypted_server(w, e).await,
            Self::SMSG_GROUP_DESTROYED => SMSG_GROUP_DESTROYED{}.tokio_write_encrypted_server(w, e).await,
            Self::SMSG_GROUP_LIST(c) => c.tokio_write_encrypted_server(w, e).await,
            Self::SMSG_PARTY_MEMBER_STATS(c) => c.tokio_write_encrypted_server(w, e).await,
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
            Self::MSG_MOVE_TELEPORT_ACK(c) => c.tokio_write_encrypted_server(w, e).await,
            Self::MSG_MOVE_FALL_LAND(c) => c.tokio_write_encrypted_server(w, e).await,
            Self::MSG_MOVE_START_SWIM(c) => c.tokio_write_encrypted_server(w, e).await,
            Self::MSG_MOVE_STOP_SWIM(c) => c.tokio_write_encrypted_server(w, e).await,
            Self::MSG_MOVE_SET_FACING(c) => c.tokio_write_encrypted_server(w, e).await,
            Self::MSG_MOVE_SET_PITCH(c) => c.tokio_write_encrypted_server(w, e).await,
            Self::SMSG_MONSTER_MOVE(c) => c.tokio_write_encrypted_server(w, e).await,
            Self::SMSG_MOVE_WATER_WALK(c) => c.tokio_write_encrypted_server(w, e).await,
            Self::SMSG_MOVE_LAND_WALK(c) => c.tokio_write_encrypted_server(w, e).await,
            Self::SMSG_FORCE_RUN_SPEED_CHANGE(c) => c.tokio_write_encrypted_server(w, e).await,
            Self::SMSG_FORCE_RUN_BACK_SPEED_CHANGE(c) => c.tokio_write_encrypted_server(w, e).await,
            Self::SMSG_FORCE_SWIM_SPEED_CHANGE(c) => c.tokio_write_encrypted_server(w, e).await,
            Self::SMSG_FORCE_MOVE_ROOT(c) => c.tokio_write_encrypted_server(w, e).await,
            Self::SMSG_FORCE_MOVE_UNROOT(c) => c.tokio_write_encrypted_server(w, e).await,
            Self::MSG_MOVE_HEARTBEAT(c) => c.tokio_write_encrypted_server(w, e).await,
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
            Self::SMSG_OPEN_CONTAINER(c) => c.tokio_write_encrypted_server(w, e).await,
            Self::SMSG_INSPECT(c) => c.tokio_write_encrypted_server(w, e).await,
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
            Self::SMSG_CAST_RESULT(c) => c.tokio_write_encrypted_server(w, e).await,
            Self::SMSG_SPELL_START(c) => c.tokio_write_encrypted_server(w, e).await,
            Self::SMSG_SPELL_GO(c) => c.tokio_write_encrypted_server(w, e).await,
            Self::SMSG_SPELL_FAILURE(c) => c.tokio_write_encrypted_server(w, e).await,
            Self::SMSG_SPELL_COOLDOWN(c) => c.tokio_write_encrypted_server(w, e).await,
            Self::SMSG_COOLDOWN_EVENT(c) => c.tokio_write_encrypted_server(w, e).await,
            Self::SMSG_UPDATE_AURA_DURATION(c) => c.tokio_write_encrypted_server(w, e).await,
            Self::SMSG_PET_CAST_FAILED(c) => c.tokio_write_encrypted_server(w, e).await,
            Self::MSG_CHANNEL_START(c) => c.tokio_write_encrypted_server(w, e).await,
            Self::MSG_CHANNEL_UPDATE(c) => c.tokio_write_encrypted_server(w, e).await,
            Self::SMSG_AI_REACTION(c) => c.tokio_write_encrypted_server(w, e).await,
            Self::SMSG_ATTACKSTART(c) => c.tokio_write_encrypted_server(w, e).await,
            Self::SMSG_ATTACKSTOP(c) => c.tokio_write_encrypted_server(w, e).await,
            Self::SMSG_ATTACKSWING_NOTINRANGE => SMSG_ATTACKSWING_NOTINRANGE{}.tokio_write_encrypted_server(w, e).await,
            Self::SMSG_ATTACKSWING_BADFACING => SMSG_ATTACKSWING_BADFACING{}.tokio_write_encrypted_server(w, e).await,
            Self::SMSG_ATTACKSWING_NOTSTANDING => SMSG_ATTACKSWING_NOTSTANDING{}.tokio_write_encrypted_server(w, e).await,
            Self::SMSG_ATTACKSWING_DEADTARGET => SMSG_ATTACKSWING_DEADTARGET{}.tokio_write_encrypted_server(w, e).await,
            Self::SMSG_ATTACKSWING_CANT_ATTACK => SMSG_ATTACKSWING_CANT_ATTACK{}.tokio_write_encrypted_server(w, e).await,
            Self::SMSG_ATTACKERSTATEUPDATE(c) => c.tokio_write_encrypted_server(w, e).await,
            Self::SMSG_CANCEL_COMBAT => SMSG_CANCEL_COMBAT{}.tokio_write_encrypted_server(w, e).await,
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
            Self::SMSG_LOOT_CLEAR_MONEY => SMSG_LOOT_CLEAR_MONEY{}.tokio_write_encrypted_server(w, e).await,
            Self::SMSG_ITEM_PUSH_RESULT(c) => c.tokio_write_encrypted_server(w, e).await,
            Self::SMSG_DUEL_REQUESTED(c) => c.tokio_write_encrypted_server(w, e).await,
            Self::SMSG_DUEL_OUTOFBOUNDS => SMSG_DUEL_OUTOFBOUNDS{}.tokio_write_encrypted_server(w, e).await,
            Self::SMSG_DUEL_INBOUNDS => SMSG_DUEL_INBOUNDS{}.tokio_write_encrypted_server(w, e).await,
            Self::SMSG_DUEL_COMPLETE(c) => c.tokio_write_encrypted_server(w, e).await,
            Self::SMSG_DUEL_WINNER(c) => c.tokio_write_encrypted_server(w, e).await,
            Self::SMSG_MOUNTRESULT(c) => c.tokio_write_encrypted_server(w, e).await,
            Self::SMSG_DISMOUNTRESULT(c) => c.tokio_write_encrypted_server(w, e).await,
            Self::SMSG_MOUNTSPECIAL_ANIM(c) => c.tokio_write_encrypted_server(w, e).await,
            Self::SMSG_PET_TAME_FAILURE(c) => c.tokio_write_encrypted_server(w, e).await,
            Self::SMSG_PET_NAME_INVALID => SMSG_PET_NAME_INVALID{}.tokio_write_encrypted_server(w, e).await,
            Self::SMSG_PET_SPELLS(c) => c.tokio_write_encrypted_server(w, e).await,
            Self::SMSG_PET_MODE(c) => c.tokio_write_encrypted_server(w, e).await,
            Self::SMSG_GOSSIP_MESSAGE(c) => c.tokio_write_encrypted_server(w, e).await,
            Self::SMSG_GOSSIP_COMPLETE => SMSG_GOSSIP_COMPLETE{}.tokio_write_encrypted_server(w, e).await,
            Self::SMSG_NPC_TEXT_UPDATE(c) => c.tokio_write_encrypted_server(w, e).await,
            Self::SMSG_QUESTGIVER_STATUS(c) => c.tokio_write_encrypted_server(w, e).await,
            Self::SMSG_QUESTGIVER_QUEST_LIST(c) => c.tokio_write_encrypted_server(w, e).await,
            Self::SMSG_QUESTGIVER_QUEST_DETAILS(c) => c.tokio_write_encrypted_server(w, e).await,
            Self::SMSG_QUESTGIVER_REQUEST_ITEMS(c) => c.tokio_write_encrypted_server(w, e).await,
            Self::SMSG_QUESTGIVER_OFFER_REWARD(c) => c.tokio_write_encrypted_server(w, e).await,
            Self::SMSG_QUESTGIVER_QUEST_INVALID(c) => c.tokio_write_encrypted_server(w, e).await,
            Self::SMSG_QUESTGIVER_QUEST_COMPLETE(c) => c.tokio_write_encrypted_server(w, e).await,
            Self::SMSG_QUESTGIVER_QUEST_FAILED(c) => c.tokio_write_encrypted_server(w, e).await,
            Self::SMSG_QUESTLOG_FULL => SMSG_QUESTLOG_FULL{}.tokio_write_encrypted_server(w, e).await,
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
            Self::SMSG_NEW_TAXI_PATH => SMSG_NEW_TAXI_PATH{}.tokio_write_encrypted_server(w, e).await,
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
            Self::SMSG_FISH_NOT_HOOKED => SMSG_FISH_NOT_HOOKED{}.tokio_write_encrypted_server(w, e).await,
            Self::SMSG_FISH_ESCAPED => SMSG_FISH_ESCAPED{}.tokio_write_encrypted_server(w, e).await,
            Self::SMSG_NOTIFICATION(c) => c.tokio_write_encrypted_server(w, e).await,
            Self::SMSG_PLAYED_TIME(c) => c.tokio_write_encrypted_server(w, e).await,
            Self::SMSG_QUERY_TIME_RESPONSE(c) => c.tokio_write_encrypted_server(w, e).await,
            Self::SMSG_LOG_XPGAIN(c) => c.tokio_write_encrypted_server(w, e).await,
            Self::SMSG_LEVELUP_INFO(c) => c.tokio_write_encrypted_server(w, e).await,
            Self::MSG_MINIMAP_PING(c) => c.tokio_write_encrypted_server(w, e).await,
            Self::SMSG_RESISTLOG(c) => c.tokio_write_encrypted_server(w, e).await,
            Self::SMSG_ENCHANTMENTLOG(c) => c.tokio_write_encrypted_server(w, e).await,
            Self::SMSG_START_MIRROR_TIMER(c) => c.tokio_write_encrypted_server(w, e).await,
            Self::SMSG_PAUSE_MIRROR_TIMER(c) => c.tokio_write_encrypted_server(w, e).await,
            Self::SMSG_STOP_MIRROR_TIMER(c) => c.tokio_write_encrypted_server(w, e).await,
            Self::SMSG_PONG(c) => c.tokio_write_encrypted_server(w, e).await,
            Self::SMSG_CLEAR_COOLDOWN(c) => c.tokio_write_encrypted_server(w, e).await,
            Self::SMSG_GAMEOBJECT_PAGETEXT(c) => c.tokio_write_encrypted_server(w, e).await,
            Self::SMSG_SPELL_DELAYED(c) => c.tokio_write_encrypted_server(w, e).await,
            Self::SMSG_ITEM_TIME_UPDATE(c) => c.tokio_write_encrypted_server(w, e).await,
            Self::SMSG_ITEM_ENCHANT_TIME_UPDATE(c) => c.tokio_write_encrypted_server(w, e).await,
            Self::SMSG_AUTH_CHALLENGE(c) => c.tokio_write_encrypted_server(w, e).await,
            Self::SMSG_AUTH_RESPONSE(c) => c.tokio_write_encrypted_server(w, e).await,
            Self::MSG_SAVE_GUILD_EMBLEM(c) => c.tokio_write_encrypted_server(w, e).await,
            Self::SMSG_PLAY_SPELL_VISUAL(c) => c.tokio_write_encrypted_server(w, e).await,
            Self::SMSG_PARTYKILLLOG(c) => c.tokio_write_encrypted_server(w, e).await,
            Self::SMSG_COMPRESSED_UPDATE_OBJECT(c) => c.tokio_write_encrypted_server(w, e).await,
            Self::SMSG_PLAY_SPELL_IMPACT(c) => c.tokio_write_encrypted_server(w, e).await,
            Self::SMSG_EXPLORATION_EXPERIENCE(c) => c.tokio_write_encrypted_server(w, e).await,
            Self::MSG_RANDOM_ROLL(c) => c.tokio_write_encrypted_server(w, e).await,
            Self::SMSG_ENVIRONMENTAL_DAMAGE_LOG(c) => c.tokio_write_encrypted_server(w, e).await,
            Self::MSG_LOOKING_FOR_GROUP(c) => c.tokio_write_encrypted_server(w, e).await,
            Self::SMSG_REMOVED_SPELL(c) => c.tokio_write_encrypted_server(w, e).await,
            Self::SMSG_GMTICKET_CREATE(c) => c.tokio_write_encrypted_server(w, e).await,
            Self::SMSG_GMTICKET_UPDATETEXT(c) => c.tokio_write_encrypted_server(w, e).await,
            Self::SMSG_ACCOUNT_DATA_TIMES(c) => c.tokio_write_encrypted_server(w, e).await,
            Self::SMSG_GMTICKET_GETTICKET(c) => c.tokio_write_encrypted_server(w, e).await,
            Self::SMSG_GAMEOBJECT_SPAWN_ANIM(c) => c.tokio_write_encrypted_server(w, e).await,
            Self::SMSG_GAMEOBJECT_DESPAWN_ANIM(c) => c.tokio_write_encrypted_server(w, e).await,
            Self::MSG_CORPSE_QUERY(c) => c.tokio_write_encrypted_server(w, e).await,
            Self::SMSG_GMTICKET_DELETETICKET(c) => c.tokio_write_encrypted_server(w, e).await,
            Self::SMSG_CHAT_WRONG_FACTION => SMSG_CHAT_WRONG_FACTION{}.tokio_write_encrypted_server(w, e).await,
            Self::SMSG_GMTICKET_SYSTEMSTATUS(c) => c.tokio_write_encrypted_server(w, e).await,
            Self::SMSG_SET_REST_START(c) => c.tokio_write_encrypted_server(w, e).await,
            Self::SMSG_SPIRIT_HEALER_CONFIRM(c) => c.tokio_write_encrypted_server(w, e).await,
            Self::SMSG_GOSSIP_POI(c) => c.tokio_write_encrypted_server(w, e).await,
            Self::SMSG_LOGIN_VERIFY_WORLD(c) => c.tokio_write_encrypted_server(w, e).await,
            Self::SMSG_SEND_MAIL_RESULT(c) => c.tokio_write_encrypted_server(w, e).await,
            Self::SMSG_MAIL_LIST_RESULT(c) => c.tokio_write_encrypted_server(w, e).await,
            Self::SMSG_BATTLEFIELD_LIST(c) => c.tokio_write_encrypted_server(w, e).await,
            Self::SMSG_ITEM_TEXT_QUERY_RESPONSE(c) => c.tokio_write_encrypted_server(w, e).await,
            Self::SMSG_SPELLLOGMISS(c) => c.tokio_write_encrypted_server(w, e).await,
            Self::SMSG_SPELLLOGEXECUTE(c) => c.tokio_write_encrypted_server(w, e).await,
            Self::SMSG_PERIODICAURALOG(c) => c.tokio_write_encrypted_server(w, e).await,
            Self::SMSG_SPELLDAMAGESHIELD(c) => c.tokio_write_encrypted_server(w, e).await,
            Self::SMSG_SPELLNONMELEEDAMAGELOG(c) => c.tokio_write_encrypted_server(w, e).await,
            Self::SMSG_ZONE_UNDER_ATTACK(c) => c.tokio_write_encrypted_server(w, e).await,
            Self::MSG_AUCTION_HELLO(c) => c.tokio_write_encrypted_server(w, e).await,
            Self::SMSG_AUCTION_COMMAND_RESULT(c) => c.tokio_write_encrypted_server(w, e).await,
            Self::SMSG_AUCTION_LIST_RESULT(c) => c.tokio_write_encrypted_server(w, e).await,
            Self::SMSG_AUCTION_OWNER_LIST_RESULT(c) => c.tokio_write_encrypted_server(w, e).await,
            Self::SMSG_AUCTION_BIDDER_NOTIFICATION(c) => c.tokio_write_encrypted_server(w, e).await,
            Self::SMSG_AUCTION_OWNER_NOTIFICATION(c) => c.tokio_write_encrypted_server(w, e).await,
            Self::SMSG_PROCRESIST(c) => c.tokio_write_encrypted_server(w, e).await,
            Self::SMSG_DISPEL_FAILED(c) => c.tokio_write_encrypted_server(w, e).await,
            Self::SMSG_SPELLORDAMAGE_IMMUNE(c) => c.tokio_write_encrypted_server(w, e).await,
            Self::SMSG_AUCTION_BIDDER_LIST_RESULT(c) => c.tokio_write_encrypted_server(w, e).await,
            Self::SMSG_SET_FLAT_SPELL_MODIFIER(c) => c.tokio_write_encrypted_server(w, e).await,
            Self::SMSG_SET_PCT_SPELL_MODIFIER(c) => c.tokio_write_encrypted_server(w, e).await,
            Self::SMSG_CORPSE_RECLAIM_DELAY(c) => c.tokio_write_encrypted_server(w, e).await,
            Self::MSG_LIST_STABLED_PETS(c) => c.tokio_write_encrypted_server(w, e).await,
            Self::SMSG_STABLE_RESULT(c) => c.tokio_write_encrypted_server(w, e).await,
            Self::SMSG_PLAY_MUSIC(c) => c.tokio_write_encrypted_server(w, e).await,
            Self::SMSG_PLAY_OBJECT_SOUND(c) => c.tokio_write_encrypted_server(w, e).await,
            Self::SMSG_SPELLDISPELLOG(c) => c.tokio_write_encrypted_server(w, e).await,
            Self::MSG_QUERY_NEXT_MAIL_TIME(c) => c.tokio_write_encrypted_server(w, e).await,
            Self::SMSG_RECEIVED_MAIL(c) => c.tokio_write_encrypted_server(w, e).await,
            Self::SMSG_RAID_GROUP_ONLY(c) => c.tokio_write_encrypted_server(w, e).await,
            Self::SMSG_PVP_CREDIT(c) => c.tokio_write_encrypted_server(w, e).await,
            Self::SMSG_AUCTION_REMOVED_NOTIFICATION(c) => c.tokio_write_encrypted_server(w, e).await,
            Self::SMSG_SERVER_MESSAGE(c) => c.tokio_write_encrypted_server(w, e).await,
            Self::SMSG_MEETINGSTONE_SETQUEUE(c) => c.tokio_write_encrypted_server(w, e).await,
            Self::SMSG_MEETINGSTONE_COMPLETE => SMSG_MEETINGSTONE_COMPLETE{}.tokio_write_encrypted_server(w, e).await,
            Self::SMSG_MEETINGSTONE_IN_PROGRESS => SMSG_MEETINGSTONE_IN_PROGRESS{}.tokio_write_encrypted_server(w, e).await,
            Self::SMSG_MEETINGSTONE_MEMBER_ADDED(c) => c.tokio_write_encrypted_server(w, e).await,
            Self::SMSG_CANCEL_AUTO_REPEAT => SMSG_CANCEL_AUTO_REPEAT{}.tokio_write_encrypted_server(w, e).await,
            Self::SMSG_STANDSTATE_UPDATE(c) => c.tokio_write_encrypted_server(w, e).await,
            Self::SMSG_LOOT_ALL_PASSED(c) => c.tokio_write_encrypted_server(w, e).await,
            Self::SMSG_LOOT_ROLL_WON(c) => c.tokio_write_encrypted_server(w, e).await,
            Self::SMSG_LOOT_START_ROLL(c) => c.tokio_write_encrypted_server(w, e).await,
            Self::SMSG_LOOT_ROLL(c) => c.tokio_write_encrypted_server(w, e).await,
            Self::SMSG_LOOT_MASTER_LIST(c) => c.tokio_write_encrypted_server(w, e).await,
            Self::SMSG_SET_FORCED_REACTIONS(c) => c.tokio_write_encrypted_server(w, e).await,
            Self::SMSG_SPELL_FAILED_OTHER(c) => c.tokio_write_encrypted_server(w, e).await,
            Self::SMSG_GAMEOBJECT_RESET_STATE(c) => c.tokio_write_encrypted_server(w, e).await,
            Self::SMSG_CHAT_PLAYER_NOT_FOUND(c) => c.tokio_write_encrypted_server(w, e).await,
            Self::MSG_TALENT_WIPE_CONFIRM(c) => c.tokio_write_encrypted_server(w, e).await,
            Self::SMSG_SUMMON_REQUEST(c) => c.tokio_write_encrypted_server(w, e).await,
            Self::SMSG_MONSTER_MOVE_TRANSPORT(c) => c.tokio_write_encrypted_server(w, e).await,
            Self::SMSG_PET_BROKEN => SMSG_PET_BROKEN{}.tokio_write_encrypted_server(w, e).await,
            Self::MSG_MOVE_FEATHER_FALL(c) => c.tokio_write_encrypted_server(w, e).await,
            Self::SMSG_FEIGN_DEATH_RESISTED => SMSG_FEIGN_DEATH_RESISTED{}.tokio_write_encrypted_server(w, e).await,
            Self::SMSG_DUEL_COUNTDOWN(c) => c.tokio_write_encrypted_server(w, e).await,
            Self::SMSG_AREA_TRIGGER_MESSAGE(c) => c.tokio_write_encrypted_server(w, e).await,
            Self::SMSG_MEETINGSTONE_JOINFAILED(c) => c.tokio_write_encrypted_server(w, e).await,
            Self::SMSG_PLAYER_SKINNED(c) => c.tokio_write_encrypted_server(w, e).await,
            Self::SMSG_DURABILITY_DAMAGE_DEATH => SMSG_DURABILITY_DAMAGE_DEATH{}.tokio_write_encrypted_server(w, e).await,
            Self::SMSG_INIT_WORLD_STATES(c) => c.tokio_write_encrypted_server(w, e).await,
            Self::SMSG_UPDATE_WORLD_STATE(c) => c.tokio_write_encrypted_server(w, e).await,
            Self::SMSG_ITEM_NAME_QUERY_RESPONSE(c) => c.tokio_write_encrypted_server(w, e).await,
            Self::SMSG_PET_ACTION_FEEDBACK(c) => c.tokio_write_encrypted_server(w, e).await,
            Self::SMSG_CHAR_RENAME(c) => c.tokio_write_encrypted_server(w, e).await,
            Self::SMSG_INSTANCE_SAVE_CREATED(c) => c.tokio_write_encrypted_server(w, e).await,
            Self::SMSG_RAID_INSTANCE_INFO(c) => c.tokio_write_encrypted_server(w, e).await,
            Self::SMSG_PLAY_SOUND(c) => c.tokio_write_encrypted_server(w, e).await,
            Self::SMSG_BATTLEFIELD_STATUS(c) => c.tokio_write_encrypted_server(w, e).await,
            Self::MSG_INSPECT_HONOR_STATS(c) => c.tokio_write_encrypted_server(w, e).await,
            Self::SMSG_FORCE_WALK_SPEED_CHANGE(c) => c.tokio_write_encrypted_server(w, e).await,
            Self::SMSG_FORCE_SWIM_BACK_SPEED_CHANGE(c) => c.tokio_write_encrypted_server(w, e).await,
            Self::SMSG_FORCE_TURN_RATE_CHANGE(c) => c.tokio_write_encrypted_server(w, e).await,
            Self::MSG_PVP_LOG_DATA(c) => c.tokio_write_encrypted_server(w, e).await,
            Self::SMSG_AREA_SPIRIT_HEALER_TIME(c) => c.tokio_write_encrypted_server(w, e).await,
            Self::SMSG_WARDEN_DATA(c) => c.tokio_write_encrypted_server(w, e).await,
            Self::SMSG_GROUP_JOINED_BATTLEGROUND(c) => c.tokio_write_encrypted_server(w, e).await,
            Self::MSG_BATTLEGROUND_PLAYER_POSITIONS(c) => c.tokio_write_encrypted_server(w, e).await,
            Self::SMSG_BINDER_CONFIRM(c) => c.tokio_write_encrypted_server(w, e).await,
            Self::SMSG_BATTLEGROUND_PLAYER_JOINED(c) => c.tokio_write_encrypted_server(w, e).await,
            Self::SMSG_BATTLEGROUND_PLAYER_LEFT(c) => c.tokio_write_encrypted_server(w, e).await,
            Self::SMSG_ADDON_INFO(c) => c.tokio_write_encrypted_server(w, e).await,
            Self::SMSG_PET_UNLEARN_CONFIRM(c) => c.tokio_write_encrypted_server(w, e).await,
            Self::SMSG_PARTY_MEMBER_STATS_FULL(c) => c.tokio_write_encrypted_server(w, e).await,
            Self::SMSG_WEATHER(c) => c.tokio_write_encrypted_server(w, e).await,
            Self::SMSG_RAID_INSTANCE_MESSAGE(c) => c.tokio_write_encrypted_server(w, e).await,
            Self::SMSG_COMPRESSED_MOVES(c) => c.tokio_write_encrypted_server(w, e).await,
            Self::SMSG_CHAT_RESTRICTED => SMSG_CHAT_RESTRICTED{}.tokio_write_encrypted_server(w, e).await,
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
            Self::MSG_MOVE_TIME_SKIPPED(c) => c.tokio_write_encrypted_server(w, e).await,
            Self::SMSG_SPLINE_MOVE_ROOT(c) => c.tokio_write_encrypted_server(w, e).await,
            Self::SMSG_INVALIDATE_PLAYER(c) => c.tokio_write_encrypted_server(w, e).await,
            Self::SMSG_INSTANCE_RESET(c) => c.tokio_write_encrypted_server(w, e).await,
            Self::SMSG_INSTANCE_RESET_FAILED(c) => c.tokio_write_encrypted_server(w, e).await,
            Self::SMSG_UPDATE_LAST_INSTANCE(c) => c.tokio_write_encrypted_server(w, e).await,
            Self::MSG_RAID_TARGET_UPDATE(c) => c.tokio_write_encrypted_server(w, e).await,
            Self::MSG_RAID_READY_CHECK(c) => c.tokio_write_encrypted_server(w, e).await,
            Self::SMSG_PET_ACTION_SOUND(c) => c.tokio_write_encrypted_server(w, e).await,
            Self::SMSG_PET_DISMISS_SOUND(c) => c.tokio_write_encrypted_server(w, e).await,
            Self::SMSG_GM_TICKET_STATUS_UPDATE(c) => c.tokio_write_encrypted_server(w, e).await,
            Self::SMSG_UPDATE_INSTANCE_OWNERSHIP(c) => c.tokio_write_encrypted_server(w, e).await,
            Self::SMSG_SPELLINSTAKILLLOG(c) => c.tokio_write_encrypted_server(w, e).await,
            Self::SMSG_SPELL_UPDATE_CHAIN_TARGETS(c) => c.tokio_write_encrypted_server(w, e).await,
            Self::SMSG_EXPECTED_SPAM_RECORDS(c) => c.tokio_write_encrypted_server(w, e).await,
            Self::SMSG_DEFENSE_MESSAGE(c) => c.tokio_write_encrypted_server(w, e).await,
        }
    }

    #[cfg(feature = "tokio")]
    pub async fn tokio_write_unencrypted_server<W: tokio::io::AsyncWriteExt + Unpin + Send>(&self, mut w: W) -> Result<(), std::io::Error> {
        match self {
            Self::MSG_MOVE_WORLDPORT_ACK => MSG_MOVE_WORLDPORT_ACK{}.tokio_write_unencrypted_server(w).await,
            Self::MSG_PETITION_DECLINE(c) => c.tokio_write_unencrypted_server(w).await,
            Self::MSG_TABARDVENDOR_ACTIVATE(c) => c.tokio_write_unencrypted_server(w).await,
            Self::MSG_QUEST_PUSH_RESULT(c) => c.tokio_write_unencrypted_server(w).await,
            Self::MSG_MOVE_WATER_WALK(c) => c.tokio_write_unencrypted_server(w).await,
            Self::MSG_PETITION_RENAME(c) => c.tokio_write_unencrypted_server(w).await,
            Self::SMSG_CHAR_CREATE(c) => c.tokio_write_unencrypted_server(w).await,
            Self::SMSG_CHAR_ENUM(c) => c.tokio_write_unencrypted_server(w).await,
            Self::SMSG_CHAR_DELETE(c) => c.tokio_write_unencrypted_server(w).await,
            Self::SMSG_NEW_WORLD(c) => c.tokio_write_unencrypted_server(w).await,
            Self::SMSG_TRANSFER_PENDING(c) => c.tokio_write_unencrypted_server(w).await,
            Self::SMSG_TRANSFER_ABORTED(c) => c.tokio_write_unencrypted_server(w).await,
            Self::SMSG_CHARACTER_LOGIN_FAILED(c) => c.tokio_write_unencrypted_server(w).await,
            Self::SMSG_LOGIN_SETTIMESPEED(c) => c.tokio_write_unencrypted_server(w).await,
            Self::SMSG_LOGOUT_RESPONSE(c) => c.tokio_write_unencrypted_server(w).await,
            Self::SMSG_LOGOUT_COMPLETE => SMSG_LOGOUT_COMPLETE{}.tokio_write_unencrypted_server(w).await,
            Self::SMSG_LOGOUT_CANCEL_ACK => SMSG_LOGOUT_CANCEL_ACK{}.tokio_write_unencrypted_server(w).await,
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
            Self::SMSG_FRIEND_LIST(c) => c.tokio_write_unencrypted_server(w).await,
            Self::SMSG_FRIEND_STATUS(c) => c.tokio_write_unencrypted_server(w).await,
            Self::SMSG_IGNORE_LIST(c) => c.tokio_write_unencrypted_server(w).await,
            Self::SMSG_GROUP_INVITE(c) => c.tokio_write_unencrypted_server(w).await,
            Self::SMSG_GROUP_DECLINE(c) => c.tokio_write_unencrypted_server(w).await,
            Self::SMSG_GROUP_UNINVITE => SMSG_GROUP_UNINVITE{}.tokio_write_unencrypted_server(w).await,
            Self::SMSG_GROUP_SET_LEADER(c) => c.tokio_write_unencrypted_server(w).await,
            Self::SMSG_GROUP_DESTROYED => SMSG_GROUP_DESTROYED{}.tokio_write_unencrypted_server(w).await,
            Self::SMSG_GROUP_LIST(c) => c.tokio_write_unencrypted_server(w).await,
            Self::SMSG_PARTY_MEMBER_STATS(c) => c.tokio_write_unencrypted_server(w).await,
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
            Self::MSG_MOVE_TELEPORT_ACK(c) => c.tokio_write_unencrypted_server(w).await,
            Self::MSG_MOVE_FALL_LAND(c) => c.tokio_write_unencrypted_server(w).await,
            Self::MSG_MOVE_START_SWIM(c) => c.tokio_write_unencrypted_server(w).await,
            Self::MSG_MOVE_STOP_SWIM(c) => c.tokio_write_unencrypted_server(w).await,
            Self::MSG_MOVE_SET_FACING(c) => c.tokio_write_unencrypted_server(w).await,
            Self::MSG_MOVE_SET_PITCH(c) => c.tokio_write_unencrypted_server(w).await,
            Self::SMSG_MONSTER_MOVE(c) => c.tokio_write_unencrypted_server(w).await,
            Self::SMSG_MOVE_WATER_WALK(c) => c.tokio_write_unencrypted_server(w).await,
            Self::SMSG_MOVE_LAND_WALK(c) => c.tokio_write_unencrypted_server(w).await,
            Self::SMSG_FORCE_RUN_SPEED_CHANGE(c) => c.tokio_write_unencrypted_server(w).await,
            Self::SMSG_FORCE_RUN_BACK_SPEED_CHANGE(c) => c.tokio_write_unencrypted_server(w).await,
            Self::SMSG_FORCE_SWIM_SPEED_CHANGE(c) => c.tokio_write_unencrypted_server(w).await,
            Self::SMSG_FORCE_MOVE_ROOT(c) => c.tokio_write_unencrypted_server(w).await,
            Self::SMSG_FORCE_MOVE_UNROOT(c) => c.tokio_write_unencrypted_server(w).await,
            Self::MSG_MOVE_HEARTBEAT(c) => c.tokio_write_unencrypted_server(w).await,
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
            Self::SMSG_OPEN_CONTAINER(c) => c.tokio_write_unencrypted_server(w).await,
            Self::SMSG_INSPECT(c) => c.tokio_write_unencrypted_server(w).await,
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
            Self::SMSG_CAST_RESULT(c) => c.tokio_write_unencrypted_server(w).await,
            Self::SMSG_SPELL_START(c) => c.tokio_write_unencrypted_server(w).await,
            Self::SMSG_SPELL_GO(c) => c.tokio_write_unencrypted_server(w).await,
            Self::SMSG_SPELL_FAILURE(c) => c.tokio_write_unencrypted_server(w).await,
            Self::SMSG_SPELL_COOLDOWN(c) => c.tokio_write_unencrypted_server(w).await,
            Self::SMSG_COOLDOWN_EVENT(c) => c.tokio_write_unencrypted_server(w).await,
            Self::SMSG_UPDATE_AURA_DURATION(c) => c.tokio_write_unencrypted_server(w).await,
            Self::SMSG_PET_CAST_FAILED(c) => c.tokio_write_unencrypted_server(w).await,
            Self::MSG_CHANNEL_START(c) => c.tokio_write_unencrypted_server(w).await,
            Self::MSG_CHANNEL_UPDATE(c) => c.tokio_write_unencrypted_server(w).await,
            Self::SMSG_AI_REACTION(c) => c.tokio_write_unencrypted_server(w).await,
            Self::SMSG_ATTACKSTART(c) => c.tokio_write_unencrypted_server(w).await,
            Self::SMSG_ATTACKSTOP(c) => c.tokio_write_unencrypted_server(w).await,
            Self::SMSG_ATTACKSWING_NOTINRANGE => SMSG_ATTACKSWING_NOTINRANGE{}.tokio_write_unencrypted_server(w).await,
            Self::SMSG_ATTACKSWING_BADFACING => SMSG_ATTACKSWING_BADFACING{}.tokio_write_unencrypted_server(w).await,
            Self::SMSG_ATTACKSWING_NOTSTANDING => SMSG_ATTACKSWING_NOTSTANDING{}.tokio_write_unencrypted_server(w).await,
            Self::SMSG_ATTACKSWING_DEADTARGET => SMSG_ATTACKSWING_DEADTARGET{}.tokio_write_unencrypted_server(w).await,
            Self::SMSG_ATTACKSWING_CANT_ATTACK => SMSG_ATTACKSWING_CANT_ATTACK{}.tokio_write_unencrypted_server(w).await,
            Self::SMSG_ATTACKERSTATEUPDATE(c) => c.tokio_write_unencrypted_server(w).await,
            Self::SMSG_CANCEL_COMBAT => SMSG_CANCEL_COMBAT{}.tokio_write_unencrypted_server(w).await,
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
            Self::SMSG_LOOT_CLEAR_MONEY => SMSG_LOOT_CLEAR_MONEY{}.tokio_write_unencrypted_server(w).await,
            Self::SMSG_ITEM_PUSH_RESULT(c) => c.tokio_write_unencrypted_server(w).await,
            Self::SMSG_DUEL_REQUESTED(c) => c.tokio_write_unencrypted_server(w).await,
            Self::SMSG_DUEL_OUTOFBOUNDS => SMSG_DUEL_OUTOFBOUNDS{}.tokio_write_unencrypted_server(w).await,
            Self::SMSG_DUEL_INBOUNDS => SMSG_DUEL_INBOUNDS{}.tokio_write_unencrypted_server(w).await,
            Self::SMSG_DUEL_COMPLETE(c) => c.tokio_write_unencrypted_server(w).await,
            Self::SMSG_DUEL_WINNER(c) => c.tokio_write_unencrypted_server(w).await,
            Self::SMSG_MOUNTRESULT(c) => c.tokio_write_unencrypted_server(w).await,
            Self::SMSG_DISMOUNTRESULT(c) => c.tokio_write_unencrypted_server(w).await,
            Self::SMSG_MOUNTSPECIAL_ANIM(c) => c.tokio_write_unencrypted_server(w).await,
            Self::SMSG_PET_TAME_FAILURE(c) => c.tokio_write_unencrypted_server(w).await,
            Self::SMSG_PET_NAME_INVALID => SMSG_PET_NAME_INVALID{}.tokio_write_unencrypted_server(w).await,
            Self::SMSG_PET_SPELLS(c) => c.tokio_write_unencrypted_server(w).await,
            Self::SMSG_PET_MODE(c) => c.tokio_write_unencrypted_server(w).await,
            Self::SMSG_GOSSIP_MESSAGE(c) => c.tokio_write_unencrypted_server(w).await,
            Self::SMSG_GOSSIP_COMPLETE => SMSG_GOSSIP_COMPLETE{}.tokio_write_unencrypted_server(w).await,
            Self::SMSG_NPC_TEXT_UPDATE(c) => c.tokio_write_unencrypted_server(w).await,
            Self::SMSG_QUESTGIVER_STATUS(c) => c.tokio_write_unencrypted_server(w).await,
            Self::SMSG_QUESTGIVER_QUEST_LIST(c) => c.tokio_write_unencrypted_server(w).await,
            Self::SMSG_QUESTGIVER_QUEST_DETAILS(c) => c.tokio_write_unencrypted_server(w).await,
            Self::SMSG_QUESTGIVER_REQUEST_ITEMS(c) => c.tokio_write_unencrypted_server(w).await,
            Self::SMSG_QUESTGIVER_OFFER_REWARD(c) => c.tokio_write_unencrypted_server(w).await,
            Self::SMSG_QUESTGIVER_QUEST_INVALID(c) => c.tokio_write_unencrypted_server(w).await,
            Self::SMSG_QUESTGIVER_QUEST_COMPLETE(c) => c.tokio_write_unencrypted_server(w).await,
            Self::SMSG_QUESTGIVER_QUEST_FAILED(c) => c.tokio_write_unencrypted_server(w).await,
            Self::SMSG_QUESTLOG_FULL => SMSG_QUESTLOG_FULL{}.tokio_write_unencrypted_server(w).await,
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
            Self::SMSG_NEW_TAXI_PATH => SMSG_NEW_TAXI_PATH{}.tokio_write_unencrypted_server(w).await,
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
            Self::SMSG_FISH_NOT_HOOKED => SMSG_FISH_NOT_HOOKED{}.tokio_write_unencrypted_server(w).await,
            Self::SMSG_FISH_ESCAPED => SMSG_FISH_ESCAPED{}.tokio_write_unencrypted_server(w).await,
            Self::SMSG_NOTIFICATION(c) => c.tokio_write_unencrypted_server(w).await,
            Self::SMSG_PLAYED_TIME(c) => c.tokio_write_unencrypted_server(w).await,
            Self::SMSG_QUERY_TIME_RESPONSE(c) => c.tokio_write_unencrypted_server(w).await,
            Self::SMSG_LOG_XPGAIN(c) => c.tokio_write_unencrypted_server(w).await,
            Self::SMSG_LEVELUP_INFO(c) => c.tokio_write_unencrypted_server(w).await,
            Self::MSG_MINIMAP_PING(c) => c.tokio_write_unencrypted_server(w).await,
            Self::SMSG_RESISTLOG(c) => c.tokio_write_unencrypted_server(w).await,
            Self::SMSG_ENCHANTMENTLOG(c) => c.tokio_write_unencrypted_server(w).await,
            Self::SMSG_START_MIRROR_TIMER(c) => c.tokio_write_unencrypted_server(w).await,
            Self::SMSG_PAUSE_MIRROR_TIMER(c) => c.tokio_write_unencrypted_server(w).await,
            Self::SMSG_STOP_MIRROR_TIMER(c) => c.tokio_write_unencrypted_server(w).await,
            Self::SMSG_PONG(c) => c.tokio_write_unencrypted_server(w).await,
            Self::SMSG_CLEAR_COOLDOWN(c) => c.tokio_write_unencrypted_server(w).await,
            Self::SMSG_GAMEOBJECT_PAGETEXT(c) => c.tokio_write_unencrypted_server(w).await,
            Self::SMSG_SPELL_DELAYED(c) => c.tokio_write_unencrypted_server(w).await,
            Self::SMSG_ITEM_TIME_UPDATE(c) => c.tokio_write_unencrypted_server(w).await,
            Self::SMSG_ITEM_ENCHANT_TIME_UPDATE(c) => c.tokio_write_unencrypted_server(w).await,
            Self::SMSG_AUTH_CHALLENGE(c) => c.tokio_write_unencrypted_server(w).await,
            Self::SMSG_AUTH_RESPONSE(c) => c.tokio_write_unencrypted_server(w).await,
            Self::MSG_SAVE_GUILD_EMBLEM(c) => c.tokio_write_unencrypted_server(w).await,
            Self::SMSG_PLAY_SPELL_VISUAL(c) => c.tokio_write_unencrypted_server(w).await,
            Self::SMSG_PARTYKILLLOG(c) => c.tokio_write_unencrypted_server(w).await,
            Self::SMSG_COMPRESSED_UPDATE_OBJECT(c) => c.tokio_write_unencrypted_server(w).await,
            Self::SMSG_PLAY_SPELL_IMPACT(c) => c.tokio_write_unencrypted_server(w).await,
            Self::SMSG_EXPLORATION_EXPERIENCE(c) => c.tokio_write_unencrypted_server(w).await,
            Self::MSG_RANDOM_ROLL(c) => c.tokio_write_unencrypted_server(w).await,
            Self::SMSG_ENVIRONMENTAL_DAMAGE_LOG(c) => c.tokio_write_unencrypted_server(w).await,
            Self::MSG_LOOKING_FOR_GROUP(c) => c.tokio_write_unencrypted_server(w).await,
            Self::SMSG_REMOVED_SPELL(c) => c.tokio_write_unencrypted_server(w).await,
            Self::SMSG_GMTICKET_CREATE(c) => c.tokio_write_unencrypted_server(w).await,
            Self::SMSG_GMTICKET_UPDATETEXT(c) => c.tokio_write_unencrypted_server(w).await,
            Self::SMSG_ACCOUNT_DATA_TIMES(c) => c.tokio_write_unencrypted_server(w).await,
            Self::SMSG_GMTICKET_GETTICKET(c) => c.tokio_write_unencrypted_server(w).await,
            Self::SMSG_GAMEOBJECT_SPAWN_ANIM(c) => c.tokio_write_unencrypted_server(w).await,
            Self::SMSG_GAMEOBJECT_DESPAWN_ANIM(c) => c.tokio_write_unencrypted_server(w).await,
            Self::MSG_CORPSE_QUERY(c) => c.tokio_write_unencrypted_server(w).await,
            Self::SMSG_GMTICKET_DELETETICKET(c) => c.tokio_write_unencrypted_server(w).await,
            Self::SMSG_CHAT_WRONG_FACTION => SMSG_CHAT_WRONG_FACTION{}.tokio_write_unencrypted_server(w).await,
            Self::SMSG_GMTICKET_SYSTEMSTATUS(c) => c.tokio_write_unencrypted_server(w).await,
            Self::SMSG_SET_REST_START(c) => c.tokio_write_unencrypted_server(w).await,
            Self::SMSG_SPIRIT_HEALER_CONFIRM(c) => c.tokio_write_unencrypted_server(w).await,
            Self::SMSG_GOSSIP_POI(c) => c.tokio_write_unencrypted_server(w).await,
            Self::SMSG_LOGIN_VERIFY_WORLD(c) => c.tokio_write_unencrypted_server(w).await,
            Self::SMSG_SEND_MAIL_RESULT(c) => c.tokio_write_unencrypted_server(w).await,
            Self::SMSG_MAIL_LIST_RESULT(c) => c.tokio_write_unencrypted_server(w).await,
            Self::SMSG_BATTLEFIELD_LIST(c) => c.tokio_write_unencrypted_server(w).await,
            Self::SMSG_ITEM_TEXT_QUERY_RESPONSE(c) => c.tokio_write_unencrypted_server(w).await,
            Self::SMSG_SPELLLOGMISS(c) => c.tokio_write_unencrypted_server(w).await,
            Self::SMSG_SPELLLOGEXECUTE(c) => c.tokio_write_unencrypted_server(w).await,
            Self::SMSG_PERIODICAURALOG(c) => c.tokio_write_unencrypted_server(w).await,
            Self::SMSG_SPELLDAMAGESHIELD(c) => c.tokio_write_unencrypted_server(w).await,
            Self::SMSG_SPELLNONMELEEDAMAGELOG(c) => c.tokio_write_unencrypted_server(w).await,
            Self::SMSG_ZONE_UNDER_ATTACK(c) => c.tokio_write_unencrypted_server(w).await,
            Self::MSG_AUCTION_HELLO(c) => c.tokio_write_unencrypted_server(w).await,
            Self::SMSG_AUCTION_COMMAND_RESULT(c) => c.tokio_write_unencrypted_server(w).await,
            Self::SMSG_AUCTION_LIST_RESULT(c) => c.tokio_write_unencrypted_server(w).await,
            Self::SMSG_AUCTION_OWNER_LIST_RESULT(c) => c.tokio_write_unencrypted_server(w).await,
            Self::SMSG_AUCTION_BIDDER_NOTIFICATION(c) => c.tokio_write_unencrypted_server(w).await,
            Self::SMSG_AUCTION_OWNER_NOTIFICATION(c) => c.tokio_write_unencrypted_server(w).await,
            Self::SMSG_PROCRESIST(c) => c.tokio_write_unencrypted_server(w).await,
            Self::SMSG_DISPEL_FAILED(c) => c.tokio_write_unencrypted_server(w).await,
            Self::SMSG_SPELLORDAMAGE_IMMUNE(c) => c.tokio_write_unencrypted_server(w).await,
            Self::SMSG_AUCTION_BIDDER_LIST_RESULT(c) => c.tokio_write_unencrypted_server(w).await,
            Self::SMSG_SET_FLAT_SPELL_MODIFIER(c) => c.tokio_write_unencrypted_server(w).await,
            Self::SMSG_SET_PCT_SPELL_MODIFIER(c) => c.tokio_write_unencrypted_server(w).await,
            Self::SMSG_CORPSE_RECLAIM_DELAY(c) => c.tokio_write_unencrypted_server(w).await,
            Self::MSG_LIST_STABLED_PETS(c) => c.tokio_write_unencrypted_server(w).await,
            Self::SMSG_STABLE_RESULT(c) => c.tokio_write_unencrypted_server(w).await,
            Self::SMSG_PLAY_MUSIC(c) => c.tokio_write_unencrypted_server(w).await,
            Self::SMSG_PLAY_OBJECT_SOUND(c) => c.tokio_write_unencrypted_server(w).await,
            Self::SMSG_SPELLDISPELLOG(c) => c.tokio_write_unencrypted_server(w).await,
            Self::MSG_QUERY_NEXT_MAIL_TIME(c) => c.tokio_write_unencrypted_server(w).await,
            Self::SMSG_RECEIVED_MAIL(c) => c.tokio_write_unencrypted_server(w).await,
            Self::SMSG_RAID_GROUP_ONLY(c) => c.tokio_write_unencrypted_server(w).await,
            Self::SMSG_PVP_CREDIT(c) => c.tokio_write_unencrypted_server(w).await,
            Self::SMSG_AUCTION_REMOVED_NOTIFICATION(c) => c.tokio_write_unencrypted_server(w).await,
            Self::SMSG_SERVER_MESSAGE(c) => c.tokio_write_unencrypted_server(w).await,
            Self::SMSG_MEETINGSTONE_SETQUEUE(c) => c.tokio_write_unencrypted_server(w).await,
            Self::SMSG_MEETINGSTONE_COMPLETE => SMSG_MEETINGSTONE_COMPLETE{}.tokio_write_unencrypted_server(w).await,
            Self::SMSG_MEETINGSTONE_IN_PROGRESS => SMSG_MEETINGSTONE_IN_PROGRESS{}.tokio_write_unencrypted_server(w).await,
            Self::SMSG_MEETINGSTONE_MEMBER_ADDED(c) => c.tokio_write_unencrypted_server(w).await,
            Self::SMSG_CANCEL_AUTO_REPEAT => SMSG_CANCEL_AUTO_REPEAT{}.tokio_write_unencrypted_server(w).await,
            Self::SMSG_STANDSTATE_UPDATE(c) => c.tokio_write_unencrypted_server(w).await,
            Self::SMSG_LOOT_ALL_PASSED(c) => c.tokio_write_unencrypted_server(w).await,
            Self::SMSG_LOOT_ROLL_WON(c) => c.tokio_write_unencrypted_server(w).await,
            Self::SMSG_LOOT_START_ROLL(c) => c.tokio_write_unencrypted_server(w).await,
            Self::SMSG_LOOT_ROLL(c) => c.tokio_write_unencrypted_server(w).await,
            Self::SMSG_LOOT_MASTER_LIST(c) => c.tokio_write_unencrypted_server(w).await,
            Self::SMSG_SET_FORCED_REACTIONS(c) => c.tokio_write_unencrypted_server(w).await,
            Self::SMSG_SPELL_FAILED_OTHER(c) => c.tokio_write_unencrypted_server(w).await,
            Self::SMSG_GAMEOBJECT_RESET_STATE(c) => c.tokio_write_unencrypted_server(w).await,
            Self::SMSG_CHAT_PLAYER_NOT_FOUND(c) => c.tokio_write_unencrypted_server(w).await,
            Self::MSG_TALENT_WIPE_CONFIRM(c) => c.tokio_write_unencrypted_server(w).await,
            Self::SMSG_SUMMON_REQUEST(c) => c.tokio_write_unencrypted_server(w).await,
            Self::SMSG_MONSTER_MOVE_TRANSPORT(c) => c.tokio_write_unencrypted_server(w).await,
            Self::SMSG_PET_BROKEN => SMSG_PET_BROKEN{}.tokio_write_unencrypted_server(w).await,
            Self::MSG_MOVE_FEATHER_FALL(c) => c.tokio_write_unencrypted_server(w).await,
            Self::SMSG_FEIGN_DEATH_RESISTED => SMSG_FEIGN_DEATH_RESISTED{}.tokio_write_unencrypted_server(w).await,
            Self::SMSG_DUEL_COUNTDOWN(c) => c.tokio_write_unencrypted_server(w).await,
            Self::SMSG_AREA_TRIGGER_MESSAGE(c) => c.tokio_write_unencrypted_server(w).await,
            Self::SMSG_MEETINGSTONE_JOINFAILED(c) => c.tokio_write_unencrypted_server(w).await,
            Self::SMSG_PLAYER_SKINNED(c) => c.tokio_write_unencrypted_server(w).await,
            Self::SMSG_DURABILITY_DAMAGE_DEATH => SMSG_DURABILITY_DAMAGE_DEATH{}.tokio_write_unencrypted_server(w).await,
            Self::SMSG_INIT_WORLD_STATES(c) => c.tokio_write_unencrypted_server(w).await,
            Self::SMSG_UPDATE_WORLD_STATE(c) => c.tokio_write_unencrypted_server(w).await,
            Self::SMSG_ITEM_NAME_QUERY_RESPONSE(c) => c.tokio_write_unencrypted_server(w).await,
            Self::SMSG_PET_ACTION_FEEDBACK(c) => c.tokio_write_unencrypted_server(w).await,
            Self::SMSG_CHAR_RENAME(c) => c.tokio_write_unencrypted_server(w).await,
            Self::SMSG_INSTANCE_SAVE_CREATED(c) => c.tokio_write_unencrypted_server(w).await,
            Self::SMSG_RAID_INSTANCE_INFO(c) => c.tokio_write_unencrypted_server(w).await,
            Self::SMSG_PLAY_SOUND(c) => c.tokio_write_unencrypted_server(w).await,
            Self::SMSG_BATTLEFIELD_STATUS(c) => c.tokio_write_unencrypted_server(w).await,
            Self::MSG_INSPECT_HONOR_STATS(c) => c.tokio_write_unencrypted_server(w).await,
            Self::SMSG_FORCE_WALK_SPEED_CHANGE(c) => c.tokio_write_unencrypted_server(w).await,
            Self::SMSG_FORCE_SWIM_BACK_SPEED_CHANGE(c) => c.tokio_write_unencrypted_server(w).await,
            Self::SMSG_FORCE_TURN_RATE_CHANGE(c) => c.tokio_write_unencrypted_server(w).await,
            Self::MSG_PVP_LOG_DATA(c) => c.tokio_write_unencrypted_server(w).await,
            Self::SMSG_AREA_SPIRIT_HEALER_TIME(c) => c.tokio_write_unencrypted_server(w).await,
            Self::SMSG_WARDEN_DATA(c) => c.tokio_write_unencrypted_server(w).await,
            Self::SMSG_GROUP_JOINED_BATTLEGROUND(c) => c.tokio_write_unencrypted_server(w).await,
            Self::MSG_BATTLEGROUND_PLAYER_POSITIONS(c) => c.tokio_write_unencrypted_server(w).await,
            Self::SMSG_BINDER_CONFIRM(c) => c.tokio_write_unencrypted_server(w).await,
            Self::SMSG_BATTLEGROUND_PLAYER_JOINED(c) => c.tokio_write_unencrypted_server(w).await,
            Self::SMSG_BATTLEGROUND_PLAYER_LEFT(c) => c.tokio_write_unencrypted_server(w).await,
            Self::SMSG_ADDON_INFO(c) => c.tokio_write_unencrypted_server(w).await,
            Self::SMSG_PET_UNLEARN_CONFIRM(c) => c.tokio_write_unencrypted_server(w).await,
            Self::SMSG_PARTY_MEMBER_STATS_FULL(c) => c.tokio_write_unencrypted_server(w).await,
            Self::SMSG_WEATHER(c) => c.tokio_write_unencrypted_server(w).await,
            Self::SMSG_RAID_INSTANCE_MESSAGE(c) => c.tokio_write_unencrypted_server(w).await,
            Self::SMSG_COMPRESSED_MOVES(c) => c.tokio_write_unencrypted_server(w).await,
            Self::SMSG_CHAT_RESTRICTED => SMSG_CHAT_RESTRICTED{}.tokio_write_unencrypted_server(w).await,
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
            Self::MSG_MOVE_TIME_SKIPPED(c) => c.tokio_write_unencrypted_server(w).await,
            Self::SMSG_SPLINE_MOVE_ROOT(c) => c.tokio_write_unencrypted_server(w).await,
            Self::SMSG_INVALIDATE_PLAYER(c) => c.tokio_write_unencrypted_server(w).await,
            Self::SMSG_INSTANCE_RESET(c) => c.tokio_write_unencrypted_server(w).await,
            Self::SMSG_INSTANCE_RESET_FAILED(c) => c.tokio_write_unencrypted_server(w).await,
            Self::SMSG_UPDATE_LAST_INSTANCE(c) => c.tokio_write_unencrypted_server(w).await,
            Self::MSG_RAID_TARGET_UPDATE(c) => c.tokio_write_unencrypted_server(w).await,
            Self::MSG_RAID_READY_CHECK(c) => c.tokio_write_unencrypted_server(w).await,
            Self::SMSG_PET_ACTION_SOUND(c) => c.tokio_write_unencrypted_server(w).await,
            Self::SMSG_PET_DISMISS_SOUND(c) => c.tokio_write_unencrypted_server(w).await,
            Self::SMSG_GM_TICKET_STATUS_UPDATE(c) => c.tokio_write_unencrypted_server(w).await,
            Self::SMSG_UPDATE_INSTANCE_OWNERSHIP(c) => c.tokio_write_unencrypted_server(w).await,
            Self::SMSG_SPELLINSTAKILLLOG(c) => c.tokio_write_unencrypted_server(w).await,
            Self::SMSG_SPELL_UPDATE_CHAIN_TARGETS(c) => c.tokio_write_unencrypted_server(w).await,
            Self::SMSG_EXPECTED_SPAM_RECORDS(c) => c.tokio_write_unencrypted_server(w).await,
            Self::SMSG_DEFENSE_MESSAGE(c) => c.tokio_write_unencrypted_server(w).await,
        }
    }

    #[cfg(all(feature = "async-std", feature = "encryption"))]
    pub async fn astd_write_encrypted_server<W: async_std::io::WriteExt + Unpin + Send>(&self, mut w: W, e: &mut EncrypterHalf) -> Result<(), std::io::Error> {
        match self {
            Self::MSG_MOVE_WORLDPORT_ACK => MSG_MOVE_WORLDPORT_ACK{}.astd_write_encrypted_server(w, e).await,
            Self::MSG_PETITION_DECLINE(c) => c.astd_write_encrypted_server(w, e).await,
            Self::MSG_TABARDVENDOR_ACTIVATE(c) => c.astd_write_encrypted_server(w, e).await,
            Self::MSG_QUEST_PUSH_RESULT(c) => c.astd_write_encrypted_server(w, e).await,
            Self::MSG_MOVE_WATER_WALK(c) => c.astd_write_encrypted_server(w, e).await,
            Self::MSG_PETITION_RENAME(c) => c.astd_write_encrypted_server(w, e).await,
            Self::SMSG_CHAR_CREATE(c) => c.astd_write_encrypted_server(w, e).await,
            Self::SMSG_CHAR_ENUM(c) => c.astd_write_encrypted_server(w, e).await,
            Self::SMSG_CHAR_DELETE(c) => c.astd_write_encrypted_server(w, e).await,
            Self::SMSG_NEW_WORLD(c) => c.astd_write_encrypted_server(w, e).await,
            Self::SMSG_TRANSFER_PENDING(c) => c.astd_write_encrypted_server(w, e).await,
            Self::SMSG_TRANSFER_ABORTED(c) => c.astd_write_encrypted_server(w, e).await,
            Self::SMSG_CHARACTER_LOGIN_FAILED(c) => c.astd_write_encrypted_server(w, e).await,
            Self::SMSG_LOGIN_SETTIMESPEED(c) => c.astd_write_encrypted_server(w, e).await,
            Self::SMSG_LOGOUT_RESPONSE(c) => c.astd_write_encrypted_server(w, e).await,
            Self::SMSG_LOGOUT_COMPLETE => SMSG_LOGOUT_COMPLETE{}.astd_write_encrypted_server(w, e).await,
            Self::SMSG_LOGOUT_CANCEL_ACK => SMSG_LOGOUT_CANCEL_ACK{}.astd_write_encrypted_server(w, e).await,
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
            Self::SMSG_FRIEND_LIST(c) => c.astd_write_encrypted_server(w, e).await,
            Self::SMSG_FRIEND_STATUS(c) => c.astd_write_encrypted_server(w, e).await,
            Self::SMSG_IGNORE_LIST(c) => c.astd_write_encrypted_server(w, e).await,
            Self::SMSG_GROUP_INVITE(c) => c.astd_write_encrypted_server(w, e).await,
            Self::SMSG_GROUP_DECLINE(c) => c.astd_write_encrypted_server(w, e).await,
            Self::SMSG_GROUP_UNINVITE => SMSG_GROUP_UNINVITE{}.astd_write_encrypted_server(w, e).await,
            Self::SMSG_GROUP_SET_LEADER(c) => c.astd_write_encrypted_server(w, e).await,
            Self::SMSG_GROUP_DESTROYED => SMSG_GROUP_DESTROYED{}.astd_write_encrypted_server(w, e).await,
            Self::SMSG_GROUP_LIST(c) => c.astd_write_encrypted_server(w, e).await,
            Self::SMSG_PARTY_MEMBER_STATS(c) => c.astd_write_encrypted_server(w, e).await,
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
            Self::MSG_MOVE_TELEPORT_ACK(c) => c.astd_write_encrypted_server(w, e).await,
            Self::MSG_MOVE_FALL_LAND(c) => c.astd_write_encrypted_server(w, e).await,
            Self::MSG_MOVE_START_SWIM(c) => c.astd_write_encrypted_server(w, e).await,
            Self::MSG_MOVE_STOP_SWIM(c) => c.astd_write_encrypted_server(w, e).await,
            Self::MSG_MOVE_SET_FACING(c) => c.astd_write_encrypted_server(w, e).await,
            Self::MSG_MOVE_SET_PITCH(c) => c.astd_write_encrypted_server(w, e).await,
            Self::SMSG_MONSTER_MOVE(c) => c.astd_write_encrypted_server(w, e).await,
            Self::SMSG_MOVE_WATER_WALK(c) => c.astd_write_encrypted_server(w, e).await,
            Self::SMSG_MOVE_LAND_WALK(c) => c.astd_write_encrypted_server(w, e).await,
            Self::SMSG_FORCE_RUN_SPEED_CHANGE(c) => c.astd_write_encrypted_server(w, e).await,
            Self::SMSG_FORCE_RUN_BACK_SPEED_CHANGE(c) => c.astd_write_encrypted_server(w, e).await,
            Self::SMSG_FORCE_SWIM_SPEED_CHANGE(c) => c.astd_write_encrypted_server(w, e).await,
            Self::SMSG_FORCE_MOVE_ROOT(c) => c.astd_write_encrypted_server(w, e).await,
            Self::SMSG_FORCE_MOVE_UNROOT(c) => c.astd_write_encrypted_server(w, e).await,
            Self::MSG_MOVE_HEARTBEAT(c) => c.astd_write_encrypted_server(w, e).await,
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
            Self::SMSG_OPEN_CONTAINER(c) => c.astd_write_encrypted_server(w, e).await,
            Self::SMSG_INSPECT(c) => c.astd_write_encrypted_server(w, e).await,
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
            Self::SMSG_CAST_RESULT(c) => c.astd_write_encrypted_server(w, e).await,
            Self::SMSG_SPELL_START(c) => c.astd_write_encrypted_server(w, e).await,
            Self::SMSG_SPELL_GO(c) => c.astd_write_encrypted_server(w, e).await,
            Self::SMSG_SPELL_FAILURE(c) => c.astd_write_encrypted_server(w, e).await,
            Self::SMSG_SPELL_COOLDOWN(c) => c.astd_write_encrypted_server(w, e).await,
            Self::SMSG_COOLDOWN_EVENT(c) => c.astd_write_encrypted_server(w, e).await,
            Self::SMSG_UPDATE_AURA_DURATION(c) => c.astd_write_encrypted_server(w, e).await,
            Self::SMSG_PET_CAST_FAILED(c) => c.astd_write_encrypted_server(w, e).await,
            Self::MSG_CHANNEL_START(c) => c.astd_write_encrypted_server(w, e).await,
            Self::MSG_CHANNEL_UPDATE(c) => c.astd_write_encrypted_server(w, e).await,
            Self::SMSG_AI_REACTION(c) => c.astd_write_encrypted_server(w, e).await,
            Self::SMSG_ATTACKSTART(c) => c.astd_write_encrypted_server(w, e).await,
            Self::SMSG_ATTACKSTOP(c) => c.astd_write_encrypted_server(w, e).await,
            Self::SMSG_ATTACKSWING_NOTINRANGE => SMSG_ATTACKSWING_NOTINRANGE{}.astd_write_encrypted_server(w, e).await,
            Self::SMSG_ATTACKSWING_BADFACING => SMSG_ATTACKSWING_BADFACING{}.astd_write_encrypted_server(w, e).await,
            Self::SMSG_ATTACKSWING_NOTSTANDING => SMSG_ATTACKSWING_NOTSTANDING{}.astd_write_encrypted_server(w, e).await,
            Self::SMSG_ATTACKSWING_DEADTARGET => SMSG_ATTACKSWING_DEADTARGET{}.astd_write_encrypted_server(w, e).await,
            Self::SMSG_ATTACKSWING_CANT_ATTACK => SMSG_ATTACKSWING_CANT_ATTACK{}.astd_write_encrypted_server(w, e).await,
            Self::SMSG_ATTACKERSTATEUPDATE(c) => c.astd_write_encrypted_server(w, e).await,
            Self::SMSG_CANCEL_COMBAT => SMSG_CANCEL_COMBAT{}.astd_write_encrypted_server(w, e).await,
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
            Self::SMSG_LOOT_CLEAR_MONEY => SMSG_LOOT_CLEAR_MONEY{}.astd_write_encrypted_server(w, e).await,
            Self::SMSG_ITEM_PUSH_RESULT(c) => c.astd_write_encrypted_server(w, e).await,
            Self::SMSG_DUEL_REQUESTED(c) => c.astd_write_encrypted_server(w, e).await,
            Self::SMSG_DUEL_OUTOFBOUNDS => SMSG_DUEL_OUTOFBOUNDS{}.astd_write_encrypted_server(w, e).await,
            Self::SMSG_DUEL_INBOUNDS => SMSG_DUEL_INBOUNDS{}.astd_write_encrypted_server(w, e).await,
            Self::SMSG_DUEL_COMPLETE(c) => c.astd_write_encrypted_server(w, e).await,
            Self::SMSG_DUEL_WINNER(c) => c.astd_write_encrypted_server(w, e).await,
            Self::SMSG_MOUNTRESULT(c) => c.astd_write_encrypted_server(w, e).await,
            Self::SMSG_DISMOUNTRESULT(c) => c.astd_write_encrypted_server(w, e).await,
            Self::SMSG_MOUNTSPECIAL_ANIM(c) => c.astd_write_encrypted_server(w, e).await,
            Self::SMSG_PET_TAME_FAILURE(c) => c.astd_write_encrypted_server(w, e).await,
            Self::SMSG_PET_NAME_INVALID => SMSG_PET_NAME_INVALID{}.astd_write_encrypted_server(w, e).await,
            Self::SMSG_PET_SPELLS(c) => c.astd_write_encrypted_server(w, e).await,
            Self::SMSG_PET_MODE(c) => c.astd_write_encrypted_server(w, e).await,
            Self::SMSG_GOSSIP_MESSAGE(c) => c.astd_write_encrypted_server(w, e).await,
            Self::SMSG_GOSSIP_COMPLETE => SMSG_GOSSIP_COMPLETE{}.astd_write_encrypted_server(w, e).await,
            Self::SMSG_NPC_TEXT_UPDATE(c) => c.astd_write_encrypted_server(w, e).await,
            Self::SMSG_QUESTGIVER_STATUS(c) => c.astd_write_encrypted_server(w, e).await,
            Self::SMSG_QUESTGIVER_QUEST_LIST(c) => c.astd_write_encrypted_server(w, e).await,
            Self::SMSG_QUESTGIVER_QUEST_DETAILS(c) => c.astd_write_encrypted_server(w, e).await,
            Self::SMSG_QUESTGIVER_REQUEST_ITEMS(c) => c.astd_write_encrypted_server(w, e).await,
            Self::SMSG_QUESTGIVER_OFFER_REWARD(c) => c.astd_write_encrypted_server(w, e).await,
            Self::SMSG_QUESTGIVER_QUEST_INVALID(c) => c.astd_write_encrypted_server(w, e).await,
            Self::SMSG_QUESTGIVER_QUEST_COMPLETE(c) => c.astd_write_encrypted_server(w, e).await,
            Self::SMSG_QUESTGIVER_QUEST_FAILED(c) => c.astd_write_encrypted_server(w, e).await,
            Self::SMSG_QUESTLOG_FULL => SMSG_QUESTLOG_FULL{}.astd_write_encrypted_server(w, e).await,
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
            Self::SMSG_NEW_TAXI_PATH => SMSG_NEW_TAXI_PATH{}.astd_write_encrypted_server(w, e).await,
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
            Self::SMSG_FISH_NOT_HOOKED => SMSG_FISH_NOT_HOOKED{}.astd_write_encrypted_server(w, e).await,
            Self::SMSG_FISH_ESCAPED => SMSG_FISH_ESCAPED{}.astd_write_encrypted_server(w, e).await,
            Self::SMSG_NOTIFICATION(c) => c.astd_write_encrypted_server(w, e).await,
            Self::SMSG_PLAYED_TIME(c) => c.astd_write_encrypted_server(w, e).await,
            Self::SMSG_QUERY_TIME_RESPONSE(c) => c.astd_write_encrypted_server(w, e).await,
            Self::SMSG_LOG_XPGAIN(c) => c.astd_write_encrypted_server(w, e).await,
            Self::SMSG_LEVELUP_INFO(c) => c.astd_write_encrypted_server(w, e).await,
            Self::MSG_MINIMAP_PING(c) => c.astd_write_encrypted_server(w, e).await,
            Self::SMSG_RESISTLOG(c) => c.astd_write_encrypted_server(w, e).await,
            Self::SMSG_ENCHANTMENTLOG(c) => c.astd_write_encrypted_server(w, e).await,
            Self::SMSG_START_MIRROR_TIMER(c) => c.astd_write_encrypted_server(w, e).await,
            Self::SMSG_PAUSE_MIRROR_TIMER(c) => c.astd_write_encrypted_server(w, e).await,
            Self::SMSG_STOP_MIRROR_TIMER(c) => c.astd_write_encrypted_server(w, e).await,
            Self::SMSG_PONG(c) => c.astd_write_encrypted_server(w, e).await,
            Self::SMSG_CLEAR_COOLDOWN(c) => c.astd_write_encrypted_server(w, e).await,
            Self::SMSG_GAMEOBJECT_PAGETEXT(c) => c.astd_write_encrypted_server(w, e).await,
            Self::SMSG_SPELL_DELAYED(c) => c.astd_write_encrypted_server(w, e).await,
            Self::SMSG_ITEM_TIME_UPDATE(c) => c.astd_write_encrypted_server(w, e).await,
            Self::SMSG_ITEM_ENCHANT_TIME_UPDATE(c) => c.astd_write_encrypted_server(w, e).await,
            Self::SMSG_AUTH_CHALLENGE(c) => c.astd_write_encrypted_server(w, e).await,
            Self::SMSG_AUTH_RESPONSE(c) => c.astd_write_encrypted_server(w, e).await,
            Self::MSG_SAVE_GUILD_EMBLEM(c) => c.astd_write_encrypted_server(w, e).await,
            Self::SMSG_PLAY_SPELL_VISUAL(c) => c.astd_write_encrypted_server(w, e).await,
            Self::SMSG_PARTYKILLLOG(c) => c.astd_write_encrypted_server(w, e).await,
            Self::SMSG_COMPRESSED_UPDATE_OBJECT(c) => c.astd_write_encrypted_server(w, e).await,
            Self::SMSG_PLAY_SPELL_IMPACT(c) => c.astd_write_encrypted_server(w, e).await,
            Self::SMSG_EXPLORATION_EXPERIENCE(c) => c.astd_write_encrypted_server(w, e).await,
            Self::MSG_RANDOM_ROLL(c) => c.astd_write_encrypted_server(w, e).await,
            Self::SMSG_ENVIRONMENTAL_DAMAGE_LOG(c) => c.astd_write_encrypted_server(w, e).await,
            Self::MSG_LOOKING_FOR_GROUP(c) => c.astd_write_encrypted_server(w, e).await,
            Self::SMSG_REMOVED_SPELL(c) => c.astd_write_encrypted_server(w, e).await,
            Self::SMSG_GMTICKET_CREATE(c) => c.astd_write_encrypted_server(w, e).await,
            Self::SMSG_GMTICKET_UPDATETEXT(c) => c.astd_write_encrypted_server(w, e).await,
            Self::SMSG_ACCOUNT_DATA_TIMES(c) => c.astd_write_encrypted_server(w, e).await,
            Self::SMSG_GMTICKET_GETTICKET(c) => c.astd_write_encrypted_server(w, e).await,
            Self::SMSG_GAMEOBJECT_SPAWN_ANIM(c) => c.astd_write_encrypted_server(w, e).await,
            Self::SMSG_GAMEOBJECT_DESPAWN_ANIM(c) => c.astd_write_encrypted_server(w, e).await,
            Self::MSG_CORPSE_QUERY(c) => c.astd_write_encrypted_server(w, e).await,
            Self::SMSG_GMTICKET_DELETETICKET(c) => c.astd_write_encrypted_server(w, e).await,
            Self::SMSG_CHAT_WRONG_FACTION => SMSG_CHAT_WRONG_FACTION{}.astd_write_encrypted_server(w, e).await,
            Self::SMSG_GMTICKET_SYSTEMSTATUS(c) => c.astd_write_encrypted_server(w, e).await,
            Self::SMSG_SET_REST_START(c) => c.astd_write_encrypted_server(w, e).await,
            Self::SMSG_SPIRIT_HEALER_CONFIRM(c) => c.astd_write_encrypted_server(w, e).await,
            Self::SMSG_GOSSIP_POI(c) => c.astd_write_encrypted_server(w, e).await,
            Self::SMSG_LOGIN_VERIFY_WORLD(c) => c.astd_write_encrypted_server(w, e).await,
            Self::SMSG_SEND_MAIL_RESULT(c) => c.astd_write_encrypted_server(w, e).await,
            Self::SMSG_MAIL_LIST_RESULT(c) => c.astd_write_encrypted_server(w, e).await,
            Self::SMSG_BATTLEFIELD_LIST(c) => c.astd_write_encrypted_server(w, e).await,
            Self::SMSG_ITEM_TEXT_QUERY_RESPONSE(c) => c.astd_write_encrypted_server(w, e).await,
            Self::SMSG_SPELLLOGMISS(c) => c.astd_write_encrypted_server(w, e).await,
            Self::SMSG_SPELLLOGEXECUTE(c) => c.astd_write_encrypted_server(w, e).await,
            Self::SMSG_PERIODICAURALOG(c) => c.astd_write_encrypted_server(w, e).await,
            Self::SMSG_SPELLDAMAGESHIELD(c) => c.astd_write_encrypted_server(w, e).await,
            Self::SMSG_SPELLNONMELEEDAMAGELOG(c) => c.astd_write_encrypted_server(w, e).await,
            Self::SMSG_ZONE_UNDER_ATTACK(c) => c.astd_write_encrypted_server(w, e).await,
            Self::MSG_AUCTION_HELLO(c) => c.astd_write_encrypted_server(w, e).await,
            Self::SMSG_AUCTION_COMMAND_RESULT(c) => c.astd_write_encrypted_server(w, e).await,
            Self::SMSG_AUCTION_LIST_RESULT(c) => c.astd_write_encrypted_server(w, e).await,
            Self::SMSG_AUCTION_OWNER_LIST_RESULT(c) => c.astd_write_encrypted_server(w, e).await,
            Self::SMSG_AUCTION_BIDDER_NOTIFICATION(c) => c.astd_write_encrypted_server(w, e).await,
            Self::SMSG_AUCTION_OWNER_NOTIFICATION(c) => c.astd_write_encrypted_server(w, e).await,
            Self::SMSG_PROCRESIST(c) => c.astd_write_encrypted_server(w, e).await,
            Self::SMSG_DISPEL_FAILED(c) => c.astd_write_encrypted_server(w, e).await,
            Self::SMSG_SPELLORDAMAGE_IMMUNE(c) => c.astd_write_encrypted_server(w, e).await,
            Self::SMSG_AUCTION_BIDDER_LIST_RESULT(c) => c.astd_write_encrypted_server(w, e).await,
            Self::SMSG_SET_FLAT_SPELL_MODIFIER(c) => c.astd_write_encrypted_server(w, e).await,
            Self::SMSG_SET_PCT_SPELL_MODIFIER(c) => c.astd_write_encrypted_server(w, e).await,
            Self::SMSG_CORPSE_RECLAIM_DELAY(c) => c.astd_write_encrypted_server(w, e).await,
            Self::MSG_LIST_STABLED_PETS(c) => c.astd_write_encrypted_server(w, e).await,
            Self::SMSG_STABLE_RESULT(c) => c.astd_write_encrypted_server(w, e).await,
            Self::SMSG_PLAY_MUSIC(c) => c.astd_write_encrypted_server(w, e).await,
            Self::SMSG_PLAY_OBJECT_SOUND(c) => c.astd_write_encrypted_server(w, e).await,
            Self::SMSG_SPELLDISPELLOG(c) => c.astd_write_encrypted_server(w, e).await,
            Self::MSG_QUERY_NEXT_MAIL_TIME(c) => c.astd_write_encrypted_server(w, e).await,
            Self::SMSG_RECEIVED_MAIL(c) => c.astd_write_encrypted_server(w, e).await,
            Self::SMSG_RAID_GROUP_ONLY(c) => c.astd_write_encrypted_server(w, e).await,
            Self::SMSG_PVP_CREDIT(c) => c.astd_write_encrypted_server(w, e).await,
            Self::SMSG_AUCTION_REMOVED_NOTIFICATION(c) => c.astd_write_encrypted_server(w, e).await,
            Self::SMSG_SERVER_MESSAGE(c) => c.astd_write_encrypted_server(w, e).await,
            Self::SMSG_MEETINGSTONE_SETQUEUE(c) => c.astd_write_encrypted_server(w, e).await,
            Self::SMSG_MEETINGSTONE_COMPLETE => SMSG_MEETINGSTONE_COMPLETE{}.astd_write_encrypted_server(w, e).await,
            Self::SMSG_MEETINGSTONE_IN_PROGRESS => SMSG_MEETINGSTONE_IN_PROGRESS{}.astd_write_encrypted_server(w, e).await,
            Self::SMSG_MEETINGSTONE_MEMBER_ADDED(c) => c.astd_write_encrypted_server(w, e).await,
            Self::SMSG_CANCEL_AUTO_REPEAT => SMSG_CANCEL_AUTO_REPEAT{}.astd_write_encrypted_server(w, e).await,
            Self::SMSG_STANDSTATE_UPDATE(c) => c.astd_write_encrypted_server(w, e).await,
            Self::SMSG_LOOT_ALL_PASSED(c) => c.astd_write_encrypted_server(w, e).await,
            Self::SMSG_LOOT_ROLL_WON(c) => c.astd_write_encrypted_server(w, e).await,
            Self::SMSG_LOOT_START_ROLL(c) => c.astd_write_encrypted_server(w, e).await,
            Self::SMSG_LOOT_ROLL(c) => c.astd_write_encrypted_server(w, e).await,
            Self::SMSG_LOOT_MASTER_LIST(c) => c.astd_write_encrypted_server(w, e).await,
            Self::SMSG_SET_FORCED_REACTIONS(c) => c.astd_write_encrypted_server(w, e).await,
            Self::SMSG_SPELL_FAILED_OTHER(c) => c.astd_write_encrypted_server(w, e).await,
            Self::SMSG_GAMEOBJECT_RESET_STATE(c) => c.astd_write_encrypted_server(w, e).await,
            Self::SMSG_CHAT_PLAYER_NOT_FOUND(c) => c.astd_write_encrypted_server(w, e).await,
            Self::MSG_TALENT_WIPE_CONFIRM(c) => c.astd_write_encrypted_server(w, e).await,
            Self::SMSG_SUMMON_REQUEST(c) => c.astd_write_encrypted_server(w, e).await,
            Self::SMSG_MONSTER_MOVE_TRANSPORT(c) => c.astd_write_encrypted_server(w, e).await,
            Self::SMSG_PET_BROKEN => SMSG_PET_BROKEN{}.astd_write_encrypted_server(w, e).await,
            Self::MSG_MOVE_FEATHER_FALL(c) => c.astd_write_encrypted_server(w, e).await,
            Self::SMSG_FEIGN_DEATH_RESISTED => SMSG_FEIGN_DEATH_RESISTED{}.astd_write_encrypted_server(w, e).await,
            Self::SMSG_DUEL_COUNTDOWN(c) => c.astd_write_encrypted_server(w, e).await,
            Self::SMSG_AREA_TRIGGER_MESSAGE(c) => c.astd_write_encrypted_server(w, e).await,
            Self::SMSG_MEETINGSTONE_JOINFAILED(c) => c.astd_write_encrypted_server(w, e).await,
            Self::SMSG_PLAYER_SKINNED(c) => c.astd_write_encrypted_server(w, e).await,
            Self::SMSG_DURABILITY_DAMAGE_DEATH => SMSG_DURABILITY_DAMAGE_DEATH{}.astd_write_encrypted_server(w, e).await,
            Self::SMSG_INIT_WORLD_STATES(c) => c.astd_write_encrypted_server(w, e).await,
            Self::SMSG_UPDATE_WORLD_STATE(c) => c.astd_write_encrypted_server(w, e).await,
            Self::SMSG_ITEM_NAME_QUERY_RESPONSE(c) => c.astd_write_encrypted_server(w, e).await,
            Self::SMSG_PET_ACTION_FEEDBACK(c) => c.astd_write_encrypted_server(w, e).await,
            Self::SMSG_CHAR_RENAME(c) => c.astd_write_encrypted_server(w, e).await,
            Self::SMSG_INSTANCE_SAVE_CREATED(c) => c.astd_write_encrypted_server(w, e).await,
            Self::SMSG_RAID_INSTANCE_INFO(c) => c.astd_write_encrypted_server(w, e).await,
            Self::SMSG_PLAY_SOUND(c) => c.astd_write_encrypted_server(w, e).await,
            Self::SMSG_BATTLEFIELD_STATUS(c) => c.astd_write_encrypted_server(w, e).await,
            Self::MSG_INSPECT_HONOR_STATS(c) => c.astd_write_encrypted_server(w, e).await,
            Self::SMSG_FORCE_WALK_SPEED_CHANGE(c) => c.astd_write_encrypted_server(w, e).await,
            Self::SMSG_FORCE_SWIM_BACK_SPEED_CHANGE(c) => c.astd_write_encrypted_server(w, e).await,
            Self::SMSG_FORCE_TURN_RATE_CHANGE(c) => c.astd_write_encrypted_server(w, e).await,
            Self::MSG_PVP_LOG_DATA(c) => c.astd_write_encrypted_server(w, e).await,
            Self::SMSG_AREA_SPIRIT_HEALER_TIME(c) => c.astd_write_encrypted_server(w, e).await,
            Self::SMSG_WARDEN_DATA(c) => c.astd_write_encrypted_server(w, e).await,
            Self::SMSG_GROUP_JOINED_BATTLEGROUND(c) => c.astd_write_encrypted_server(w, e).await,
            Self::MSG_BATTLEGROUND_PLAYER_POSITIONS(c) => c.astd_write_encrypted_server(w, e).await,
            Self::SMSG_BINDER_CONFIRM(c) => c.astd_write_encrypted_server(w, e).await,
            Self::SMSG_BATTLEGROUND_PLAYER_JOINED(c) => c.astd_write_encrypted_server(w, e).await,
            Self::SMSG_BATTLEGROUND_PLAYER_LEFT(c) => c.astd_write_encrypted_server(w, e).await,
            Self::SMSG_ADDON_INFO(c) => c.astd_write_encrypted_server(w, e).await,
            Self::SMSG_PET_UNLEARN_CONFIRM(c) => c.astd_write_encrypted_server(w, e).await,
            Self::SMSG_PARTY_MEMBER_STATS_FULL(c) => c.astd_write_encrypted_server(w, e).await,
            Self::SMSG_WEATHER(c) => c.astd_write_encrypted_server(w, e).await,
            Self::SMSG_RAID_INSTANCE_MESSAGE(c) => c.astd_write_encrypted_server(w, e).await,
            Self::SMSG_COMPRESSED_MOVES(c) => c.astd_write_encrypted_server(w, e).await,
            Self::SMSG_CHAT_RESTRICTED => SMSG_CHAT_RESTRICTED{}.astd_write_encrypted_server(w, e).await,
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
            Self::MSG_MOVE_TIME_SKIPPED(c) => c.astd_write_encrypted_server(w, e).await,
            Self::SMSG_SPLINE_MOVE_ROOT(c) => c.astd_write_encrypted_server(w, e).await,
            Self::SMSG_INVALIDATE_PLAYER(c) => c.astd_write_encrypted_server(w, e).await,
            Self::SMSG_INSTANCE_RESET(c) => c.astd_write_encrypted_server(w, e).await,
            Self::SMSG_INSTANCE_RESET_FAILED(c) => c.astd_write_encrypted_server(w, e).await,
            Self::SMSG_UPDATE_LAST_INSTANCE(c) => c.astd_write_encrypted_server(w, e).await,
            Self::MSG_RAID_TARGET_UPDATE(c) => c.astd_write_encrypted_server(w, e).await,
            Self::MSG_RAID_READY_CHECK(c) => c.astd_write_encrypted_server(w, e).await,
            Self::SMSG_PET_ACTION_SOUND(c) => c.astd_write_encrypted_server(w, e).await,
            Self::SMSG_PET_DISMISS_SOUND(c) => c.astd_write_encrypted_server(w, e).await,
            Self::SMSG_GM_TICKET_STATUS_UPDATE(c) => c.astd_write_encrypted_server(w, e).await,
            Self::SMSG_UPDATE_INSTANCE_OWNERSHIP(c) => c.astd_write_encrypted_server(w, e).await,
            Self::SMSG_SPELLINSTAKILLLOG(c) => c.astd_write_encrypted_server(w, e).await,
            Self::SMSG_SPELL_UPDATE_CHAIN_TARGETS(c) => c.astd_write_encrypted_server(w, e).await,
            Self::SMSG_EXPECTED_SPAM_RECORDS(c) => c.astd_write_encrypted_server(w, e).await,
            Self::SMSG_DEFENSE_MESSAGE(c) => c.astd_write_encrypted_server(w, e).await,
        }
    }

    #[cfg(feature = "async-std")]
    pub async fn astd_write_unencrypted_server<W: async_std::io::WriteExt + Unpin + Send>(&self, mut w: W) -> Result<(), std::io::Error> {
        match self {
            Self::MSG_MOVE_WORLDPORT_ACK => MSG_MOVE_WORLDPORT_ACK{}.astd_write_unencrypted_server(w).await,
            Self::MSG_PETITION_DECLINE(c) => c.astd_write_unencrypted_server(w).await,
            Self::MSG_TABARDVENDOR_ACTIVATE(c) => c.astd_write_unencrypted_server(w).await,
            Self::MSG_QUEST_PUSH_RESULT(c) => c.astd_write_unencrypted_server(w).await,
            Self::MSG_MOVE_WATER_WALK(c) => c.astd_write_unencrypted_server(w).await,
            Self::MSG_PETITION_RENAME(c) => c.astd_write_unencrypted_server(w).await,
            Self::SMSG_CHAR_CREATE(c) => c.astd_write_unencrypted_server(w).await,
            Self::SMSG_CHAR_ENUM(c) => c.astd_write_unencrypted_server(w).await,
            Self::SMSG_CHAR_DELETE(c) => c.astd_write_unencrypted_server(w).await,
            Self::SMSG_NEW_WORLD(c) => c.astd_write_unencrypted_server(w).await,
            Self::SMSG_TRANSFER_PENDING(c) => c.astd_write_unencrypted_server(w).await,
            Self::SMSG_TRANSFER_ABORTED(c) => c.astd_write_unencrypted_server(w).await,
            Self::SMSG_CHARACTER_LOGIN_FAILED(c) => c.astd_write_unencrypted_server(w).await,
            Self::SMSG_LOGIN_SETTIMESPEED(c) => c.astd_write_unencrypted_server(w).await,
            Self::SMSG_LOGOUT_RESPONSE(c) => c.astd_write_unencrypted_server(w).await,
            Self::SMSG_LOGOUT_COMPLETE => SMSG_LOGOUT_COMPLETE{}.astd_write_unencrypted_server(w).await,
            Self::SMSG_LOGOUT_CANCEL_ACK => SMSG_LOGOUT_CANCEL_ACK{}.astd_write_unencrypted_server(w).await,
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
            Self::SMSG_FRIEND_LIST(c) => c.astd_write_unencrypted_server(w).await,
            Self::SMSG_FRIEND_STATUS(c) => c.astd_write_unencrypted_server(w).await,
            Self::SMSG_IGNORE_LIST(c) => c.astd_write_unencrypted_server(w).await,
            Self::SMSG_GROUP_INVITE(c) => c.astd_write_unencrypted_server(w).await,
            Self::SMSG_GROUP_DECLINE(c) => c.astd_write_unencrypted_server(w).await,
            Self::SMSG_GROUP_UNINVITE => SMSG_GROUP_UNINVITE{}.astd_write_unencrypted_server(w).await,
            Self::SMSG_GROUP_SET_LEADER(c) => c.astd_write_unencrypted_server(w).await,
            Self::SMSG_GROUP_DESTROYED => SMSG_GROUP_DESTROYED{}.astd_write_unencrypted_server(w).await,
            Self::SMSG_GROUP_LIST(c) => c.astd_write_unencrypted_server(w).await,
            Self::SMSG_PARTY_MEMBER_STATS(c) => c.astd_write_unencrypted_server(w).await,
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
            Self::MSG_MOVE_TELEPORT_ACK(c) => c.astd_write_unencrypted_server(w).await,
            Self::MSG_MOVE_FALL_LAND(c) => c.astd_write_unencrypted_server(w).await,
            Self::MSG_MOVE_START_SWIM(c) => c.astd_write_unencrypted_server(w).await,
            Self::MSG_MOVE_STOP_SWIM(c) => c.astd_write_unencrypted_server(w).await,
            Self::MSG_MOVE_SET_FACING(c) => c.astd_write_unencrypted_server(w).await,
            Self::MSG_MOVE_SET_PITCH(c) => c.astd_write_unencrypted_server(w).await,
            Self::SMSG_MONSTER_MOVE(c) => c.astd_write_unencrypted_server(w).await,
            Self::SMSG_MOVE_WATER_WALK(c) => c.astd_write_unencrypted_server(w).await,
            Self::SMSG_MOVE_LAND_WALK(c) => c.astd_write_unencrypted_server(w).await,
            Self::SMSG_FORCE_RUN_SPEED_CHANGE(c) => c.astd_write_unencrypted_server(w).await,
            Self::SMSG_FORCE_RUN_BACK_SPEED_CHANGE(c) => c.astd_write_unencrypted_server(w).await,
            Self::SMSG_FORCE_SWIM_SPEED_CHANGE(c) => c.astd_write_unencrypted_server(w).await,
            Self::SMSG_FORCE_MOVE_ROOT(c) => c.astd_write_unencrypted_server(w).await,
            Self::SMSG_FORCE_MOVE_UNROOT(c) => c.astd_write_unencrypted_server(w).await,
            Self::MSG_MOVE_HEARTBEAT(c) => c.astd_write_unencrypted_server(w).await,
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
            Self::SMSG_OPEN_CONTAINER(c) => c.astd_write_unencrypted_server(w).await,
            Self::SMSG_INSPECT(c) => c.astd_write_unencrypted_server(w).await,
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
            Self::SMSG_CAST_RESULT(c) => c.astd_write_unencrypted_server(w).await,
            Self::SMSG_SPELL_START(c) => c.astd_write_unencrypted_server(w).await,
            Self::SMSG_SPELL_GO(c) => c.astd_write_unencrypted_server(w).await,
            Self::SMSG_SPELL_FAILURE(c) => c.astd_write_unencrypted_server(w).await,
            Self::SMSG_SPELL_COOLDOWN(c) => c.astd_write_unencrypted_server(w).await,
            Self::SMSG_COOLDOWN_EVENT(c) => c.astd_write_unencrypted_server(w).await,
            Self::SMSG_UPDATE_AURA_DURATION(c) => c.astd_write_unencrypted_server(w).await,
            Self::SMSG_PET_CAST_FAILED(c) => c.astd_write_unencrypted_server(w).await,
            Self::MSG_CHANNEL_START(c) => c.astd_write_unencrypted_server(w).await,
            Self::MSG_CHANNEL_UPDATE(c) => c.astd_write_unencrypted_server(w).await,
            Self::SMSG_AI_REACTION(c) => c.astd_write_unencrypted_server(w).await,
            Self::SMSG_ATTACKSTART(c) => c.astd_write_unencrypted_server(w).await,
            Self::SMSG_ATTACKSTOP(c) => c.astd_write_unencrypted_server(w).await,
            Self::SMSG_ATTACKSWING_NOTINRANGE => SMSG_ATTACKSWING_NOTINRANGE{}.astd_write_unencrypted_server(w).await,
            Self::SMSG_ATTACKSWING_BADFACING => SMSG_ATTACKSWING_BADFACING{}.astd_write_unencrypted_server(w).await,
            Self::SMSG_ATTACKSWING_NOTSTANDING => SMSG_ATTACKSWING_NOTSTANDING{}.astd_write_unencrypted_server(w).await,
            Self::SMSG_ATTACKSWING_DEADTARGET => SMSG_ATTACKSWING_DEADTARGET{}.astd_write_unencrypted_server(w).await,
            Self::SMSG_ATTACKSWING_CANT_ATTACK => SMSG_ATTACKSWING_CANT_ATTACK{}.astd_write_unencrypted_server(w).await,
            Self::SMSG_ATTACKERSTATEUPDATE(c) => c.astd_write_unencrypted_server(w).await,
            Self::SMSG_CANCEL_COMBAT => SMSG_CANCEL_COMBAT{}.astd_write_unencrypted_server(w).await,
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
            Self::SMSG_LOOT_CLEAR_MONEY => SMSG_LOOT_CLEAR_MONEY{}.astd_write_unencrypted_server(w).await,
            Self::SMSG_ITEM_PUSH_RESULT(c) => c.astd_write_unencrypted_server(w).await,
            Self::SMSG_DUEL_REQUESTED(c) => c.astd_write_unencrypted_server(w).await,
            Self::SMSG_DUEL_OUTOFBOUNDS => SMSG_DUEL_OUTOFBOUNDS{}.astd_write_unencrypted_server(w).await,
            Self::SMSG_DUEL_INBOUNDS => SMSG_DUEL_INBOUNDS{}.astd_write_unencrypted_server(w).await,
            Self::SMSG_DUEL_COMPLETE(c) => c.astd_write_unencrypted_server(w).await,
            Self::SMSG_DUEL_WINNER(c) => c.astd_write_unencrypted_server(w).await,
            Self::SMSG_MOUNTRESULT(c) => c.astd_write_unencrypted_server(w).await,
            Self::SMSG_DISMOUNTRESULT(c) => c.astd_write_unencrypted_server(w).await,
            Self::SMSG_MOUNTSPECIAL_ANIM(c) => c.astd_write_unencrypted_server(w).await,
            Self::SMSG_PET_TAME_FAILURE(c) => c.astd_write_unencrypted_server(w).await,
            Self::SMSG_PET_NAME_INVALID => SMSG_PET_NAME_INVALID{}.astd_write_unencrypted_server(w).await,
            Self::SMSG_PET_SPELLS(c) => c.astd_write_unencrypted_server(w).await,
            Self::SMSG_PET_MODE(c) => c.astd_write_unencrypted_server(w).await,
            Self::SMSG_GOSSIP_MESSAGE(c) => c.astd_write_unencrypted_server(w).await,
            Self::SMSG_GOSSIP_COMPLETE => SMSG_GOSSIP_COMPLETE{}.astd_write_unencrypted_server(w).await,
            Self::SMSG_NPC_TEXT_UPDATE(c) => c.astd_write_unencrypted_server(w).await,
            Self::SMSG_QUESTGIVER_STATUS(c) => c.astd_write_unencrypted_server(w).await,
            Self::SMSG_QUESTGIVER_QUEST_LIST(c) => c.astd_write_unencrypted_server(w).await,
            Self::SMSG_QUESTGIVER_QUEST_DETAILS(c) => c.astd_write_unencrypted_server(w).await,
            Self::SMSG_QUESTGIVER_REQUEST_ITEMS(c) => c.astd_write_unencrypted_server(w).await,
            Self::SMSG_QUESTGIVER_OFFER_REWARD(c) => c.astd_write_unencrypted_server(w).await,
            Self::SMSG_QUESTGIVER_QUEST_INVALID(c) => c.astd_write_unencrypted_server(w).await,
            Self::SMSG_QUESTGIVER_QUEST_COMPLETE(c) => c.astd_write_unencrypted_server(w).await,
            Self::SMSG_QUESTGIVER_QUEST_FAILED(c) => c.astd_write_unencrypted_server(w).await,
            Self::SMSG_QUESTLOG_FULL => SMSG_QUESTLOG_FULL{}.astd_write_unencrypted_server(w).await,
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
            Self::SMSG_NEW_TAXI_PATH => SMSG_NEW_TAXI_PATH{}.astd_write_unencrypted_server(w).await,
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
            Self::SMSG_FISH_NOT_HOOKED => SMSG_FISH_NOT_HOOKED{}.astd_write_unencrypted_server(w).await,
            Self::SMSG_FISH_ESCAPED => SMSG_FISH_ESCAPED{}.astd_write_unencrypted_server(w).await,
            Self::SMSG_NOTIFICATION(c) => c.astd_write_unencrypted_server(w).await,
            Self::SMSG_PLAYED_TIME(c) => c.astd_write_unencrypted_server(w).await,
            Self::SMSG_QUERY_TIME_RESPONSE(c) => c.astd_write_unencrypted_server(w).await,
            Self::SMSG_LOG_XPGAIN(c) => c.astd_write_unencrypted_server(w).await,
            Self::SMSG_LEVELUP_INFO(c) => c.astd_write_unencrypted_server(w).await,
            Self::MSG_MINIMAP_PING(c) => c.astd_write_unencrypted_server(w).await,
            Self::SMSG_RESISTLOG(c) => c.astd_write_unencrypted_server(w).await,
            Self::SMSG_ENCHANTMENTLOG(c) => c.astd_write_unencrypted_server(w).await,
            Self::SMSG_START_MIRROR_TIMER(c) => c.astd_write_unencrypted_server(w).await,
            Self::SMSG_PAUSE_MIRROR_TIMER(c) => c.astd_write_unencrypted_server(w).await,
            Self::SMSG_STOP_MIRROR_TIMER(c) => c.astd_write_unencrypted_server(w).await,
            Self::SMSG_PONG(c) => c.astd_write_unencrypted_server(w).await,
            Self::SMSG_CLEAR_COOLDOWN(c) => c.astd_write_unencrypted_server(w).await,
            Self::SMSG_GAMEOBJECT_PAGETEXT(c) => c.astd_write_unencrypted_server(w).await,
            Self::SMSG_SPELL_DELAYED(c) => c.astd_write_unencrypted_server(w).await,
            Self::SMSG_ITEM_TIME_UPDATE(c) => c.astd_write_unencrypted_server(w).await,
            Self::SMSG_ITEM_ENCHANT_TIME_UPDATE(c) => c.astd_write_unencrypted_server(w).await,
            Self::SMSG_AUTH_CHALLENGE(c) => c.astd_write_unencrypted_server(w).await,
            Self::SMSG_AUTH_RESPONSE(c) => c.astd_write_unencrypted_server(w).await,
            Self::MSG_SAVE_GUILD_EMBLEM(c) => c.astd_write_unencrypted_server(w).await,
            Self::SMSG_PLAY_SPELL_VISUAL(c) => c.astd_write_unencrypted_server(w).await,
            Self::SMSG_PARTYKILLLOG(c) => c.astd_write_unencrypted_server(w).await,
            Self::SMSG_COMPRESSED_UPDATE_OBJECT(c) => c.astd_write_unencrypted_server(w).await,
            Self::SMSG_PLAY_SPELL_IMPACT(c) => c.astd_write_unencrypted_server(w).await,
            Self::SMSG_EXPLORATION_EXPERIENCE(c) => c.astd_write_unencrypted_server(w).await,
            Self::MSG_RANDOM_ROLL(c) => c.astd_write_unencrypted_server(w).await,
            Self::SMSG_ENVIRONMENTAL_DAMAGE_LOG(c) => c.astd_write_unencrypted_server(w).await,
            Self::MSG_LOOKING_FOR_GROUP(c) => c.astd_write_unencrypted_server(w).await,
            Self::SMSG_REMOVED_SPELL(c) => c.astd_write_unencrypted_server(w).await,
            Self::SMSG_GMTICKET_CREATE(c) => c.astd_write_unencrypted_server(w).await,
            Self::SMSG_GMTICKET_UPDATETEXT(c) => c.astd_write_unencrypted_server(w).await,
            Self::SMSG_ACCOUNT_DATA_TIMES(c) => c.astd_write_unencrypted_server(w).await,
            Self::SMSG_GMTICKET_GETTICKET(c) => c.astd_write_unencrypted_server(w).await,
            Self::SMSG_GAMEOBJECT_SPAWN_ANIM(c) => c.astd_write_unencrypted_server(w).await,
            Self::SMSG_GAMEOBJECT_DESPAWN_ANIM(c) => c.astd_write_unencrypted_server(w).await,
            Self::MSG_CORPSE_QUERY(c) => c.astd_write_unencrypted_server(w).await,
            Self::SMSG_GMTICKET_DELETETICKET(c) => c.astd_write_unencrypted_server(w).await,
            Self::SMSG_CHAT_WRONG_FACTION => SMSG_CHAT_WRONG_FACTION{}.astd_write_unencrypted_server(w).await,
            Self::SMSG_GMTICKET_SYSTEMSTATUS(c) => c.astd_write_unencrypted_server(w).await,
            Self::SMSG_SET_REST_START(c) => c.astd_write_unencrypted_server(w).await,
            Self::SMSG_SPIRIT_HEALER_CONFIRM(c) => c.astd_write_unencrypted_server(w).await,
            Self::SMSG_GOSSIP_POI(c) => c.astd_write_unencrypted_server(w).await,
            Self::SMSG_LOGIN_VERIFY_WORLD(c) => c.astd_write_unencrypted_server(w).await,
            Self::SMSG_SEND_MAIL_RESULT(c) => c.astd_write_unencrypted_server(w).await,
            Self::SMSG_MAIL_LIST_RESULT(c) => c.astd_write_unencrypted_server(w).await,
            Self::SMSG_BATTLEFIELD_LIST(c) => c.astd_write_unencrypted_server(w).await,
            Self::SMSG_ITEM_TEXT_QUERY_RESPONSE(c) => c.astd_write_unencrypted_server(w).await,
            Self::SMSG_SPELLLOGMISS(c) => c.astd_write_unencrypted_server(w).await,
            Self::SMSG_SPELLLOGEXECUTE(c) => c.astd_write_unencrypted_server(w).await,
            Self::SMSG_PERIODICAURALOG(c) => c.astd_write_unencrypted_server(w).await,
            Self::SMSG_SPELLDAMAGESHIELD(c) => c.astd_write_unencrypted_server(w).await,
            Self::SMSG_SPELLNONMELEEDAMAGELOG(c) => c.astd_write_unencrypted_server(w).await,
            Self::SMSG_ZONE_UNDER_ATTACK(c) => c.astd_write_unencrypted_server(w).await,
            Self::MSG_AUCTION_HELLO(c) => c.astd_write_unencrypted_server(w).await,
            Self::SMSG_AUCTION_COMMAND_RESULT(c) => c.astd_write_unencrypted_server(w).await,
            Self::SMSG_AUCTION_LIST_RESULT(c) => c.astd_write_unencrypted_server(w).await,
            Self::SMSG_AUCTION_OWNER_LIST_RESULT(c) => c.astd_write_unencrypted_server(w).await,
            Self::SMSG_AUCTION_BIDDER_NOTIFICATION(c) => c.astd_write_unencrypted_server(w).await,
            Self::SMSG_AUCTION_OWNER_NOTIFICATION(c) => c.astd_write_unencrypted_server(w).await,
            Self::SMSG_PROCRESIST(c) => c.astd_write_unencrypted_server(w).await,
            Self::SMSG_DISPEL_FAILED(c) => c.astd_write_unencrypted_server(w).await,
            Self::SMSG_SPELLORDAMAGE_IMMUNE(c) => c.astd_write_unencrypted_server(w).await,
            Self::SMSG_AUCTION_BIDDER_LIST_RESULT(c) => c.astd_write_unencrypted_server(w).await,
            Self::SMSG_SET_FLAT_SPELL_MODIFIER(c) => c.astd_write_unencrypted_server(w).await,
            Self::SMSG_SET_PCT_SPELL_MODIFIER(c) => c.astd_write_unencrypted_server(w).await,
            Self::SMSG_CORPSE_RECLAIM_DELAY(c) => c.astd_write_unencrypted_server(w).await,
            Self::MSG_LIST_STABLED_PETS(c) => c.astd_write_unencrypted_server(w).await,
            Self::SMSG_STABLE_RESULT(c) => c.astd_write_unencrypted_server(w).await,
            Self::SMSG_PLAY_MUSIC(c) => c.astd_write_unencrypted_server(w).await,
            Self::SMSG_PLAY_OBJECT_SOUND(c) => c.astd_write_unencrypted_server(w).await,
            Self::SMSG_SPELLDISPELLOG(c) => c.astd_write_unencrypted_server(w).await,
            Self::MSG_QUERY_NEXT_MAIL_TIME(c) => c.astd_write_unencrypted_server(w).await,
            Self::SMSG_RECEIVED_MAIL(c) => c.astd_write_unencrypted_server(w).await,
            Self::SMSG_RAID_GROUP_ONLY(c) => c.astd_write_unencrypted_server(w).await,
            Self::SMSG_PVP_CREDIT(c) => c.astd_write_unencrypted_server(w).await,
            Self::SMSG_AUCTION_REMOVED_NOTIFICATION(c) => c.astd_write_unencrypted_server(w).await,
            Self::SMSG_SERVER_MESSAGE(c) => c.astd_write_unencrypted_server(w).await,
            Self::SMSG_MEETINGSTONE_SETQUEUE(c) => c.astd_write_unencrypted_server(w).await,
            Self::SMSG_MEETINGSTONE_COMPLETE => SMSG_MEETINGSTONE_COMPLETE{}.astd_write_unencrypted_server(w).await,
            Self::SMSG_MEETINGSTONE_IN_PROGRESS => SMSG_MEETINGSTONE_IN_PROGRESS{}.astd_write_unencrypted_server(w).await,
            Self::SMSG_MEETINGSTONE_MEMBER_ADDED(c) => c.astd_write_unencrypted_server(w).await,
            Self::SMSG_CANCEL_AUTO_REPEAT => SMSG_CANCEL_AUTO_REPEAT{}.astd_write_unencrypted_server(w).await,
            Self::SMSG_STANDSTATE_UPDATE(c) => c.astd_write_unencrypted_server(w).await,
            Self::SMSG_LOOT_ALL_PASSED(c) => c.astd_write_unencrypted_server(w).await,
            Self::SMSG_LOOT_ROLL_WON(c) => c.astd_write_unencrypted_server(w).await,
            Self::SMSG_LOOT_START_ROLL(c) => c.astd_write_unencrypted_server(w).await,
            Self::SMSG_LOOT_ROLL(c) => c.astd_write_unencrypted_server(w).await,
            Self::SMSG_LOOT_MASTER_LIST(c) => c.astd_write_unencrypted_server(w).await,
            Self::SMSG_SET_FORCED_REACTIONS(c) => c.astd_write_unencrypted_server(w).await,
            Self::SMSG_SPELL_FAILED_OTHER(c) => c.astd_write_unencrypted_server(w).await,
            Self::SMSG_GAMEOBJECT_RESET_STATE(c) => c.astd_write_unencrypted_server(w).await,
            Self::SMSG_CHAT_PLAYER_NOT_FOUND(c) => c.astd_write_unencrypted_server(w).await,
            Self::MSG_TALENT_WIPE_CONFIRM(c) => c.astd_write_unencrypted_server(w).await,
            Self::SMSG_SUMMON_REQUEST(c) => c.astd_write_unencrypted_server(w).await,
            Self::SMSG_MONSTER_MOVE_TRANSPORT(c) => c.astd_write_unencrypted_server(w).await,
            Self::SMSG_PET_BROKEN => SMSG_PET_BROKEN{}.astd_write_unencrypted_server(w).await,
            Self::MSG_MOVE_FEATHER_FALL(c) => c.astd_write_unencrypted_server(w).await,
            Self::SMSG_FEIGN_DEATH_RESISTED => SMSG_FEIGN_DEATH_RESISTED{}.astd_write_unencrypted_server(w).await,
            Self::SMSG_DUEL_COUNTDOWN(c) => c.astd_write_unencrypted_server(w).await,
            Self::SMSG_AREA_TRIGGER_MESSAGE(c) => c.astd_write_unencrypted_server(w).await,
            Self::SMSG_MEETINGSTONE_JOINFAILED(c) => c.astd_write_unencrypted_server(w).await,
            Self::SMSG_PLAYER_SKINNED(c) => c.astd_write_unencrypted_server(w).await,
            Self::SMSG_DURABILITY_DAMAGE_DEATH => SMSG_DURABILITY_DAMAGE_DEATH{}.astd_write_unencrypted_server(w).await,
            Self::SMSG_INIT_WORLD_STATES(c) => c.astd_write_unencrypted_server(w).await,
            Self::SMSG_UPDATE_WORLD_STATE(c) => c.astd_write_unencrypted_server(w).await,
            Self::SMSG_ITEM_NAME_QUERY_RESPONSE(c) => c.astd_write_unencrypted_server(w).await,
            Self::SMSG_PET_ACTION_FEEDBACK(c) => c.astd_write_unencrypted_server(w).await,
            Self::SMSG_CHAR_RENAME(c) => c.astd_write_unencrypted_server(w).await,
            Self::SMSG_INSTANCE_SAVE_CREATED(c) => c.astd_write_unencrypted_server(w).await,
            Self::SMSG_RAID_INSTANCE_INFO(c) => c.astd_write_unencrypted_server(w).await,
            Self::SMSG_PLAY_SOUND(c) => c.astd_write_unencrypted_server(w).await,
            Self::SMSG_BATTLEFIELD_STATUS(c) => c.astd_write_unencrypted_server(w).await,
            Self::MSG_INSPECT_HONOR_STATS(c) => c.astd_write_unencrypted_server(w).await,
            Self::SMSG_FORCE_WALK_SPEED_CHANGE(c) => c.astd_write_unencrypted_server(w).await,
            Self::SMSG_FORCE_SWIM_BACK_SPEED_CHANGE(c) => c.astd_write_unencrypted_server(w).await,
            Self::SMSG_FORCE_TURN_RATE_CHANGE(c) => c.astd_write_unencrypted_server(w).await,
            Self::MSG_PVP_LOG_DATA(c) => c.astd_write_unencrypted_server(w).await,
            Self::SMSG_AREA_SPIRIT_HEALER_TIME(c) => c.astd_write_unencrypted_server(w).await,
            Self::SMSG_WARDEN_DATA(c) => c.astd_write_unencrypted_server(w).await,
            Self::SMSG_GROUP_JOINED_BATTLEGROUND(c) => c.astd_write_unencrypted_server(w).await,
            Self::MSG_BATTLEGROUND_PLAYER_POSITIONS(c) => c.astd_write_unencrypted_server(w).await,
            Self::SMSG_BINDER_CONFIRM(c) => c.astd_write_unencrypted_server(w).await,
            Self::SMSG_BATTLEGROUND_PLAYER_JOINED(c) => c.astd_write_unencrypted_server(w).await,
            Self::SMSG_BATTLEGROUND_PLAYER_LEFT(c) => c.astd_write_unencrypted_server(w).await,
            Self::SMSG_ADDON_INFO(c) => c.astd_write_unencrypted_server(w).await,
            Self::SMSG_PET_UNLEARN_CONFIRM(c) => c.astd_write_unencrypted_server(w).await,
            Self::SMSG_PARTY_MEMBER_STATS_FULL(c) => c.astd_write_unencrypted_server(w).await,
            Self::SMSG_WEATHER(c) => c.astd_write_unencrypted_server(w).await,
            Self::SMSG_RAID_INSTANCE_MESSAGE(c) => c.astd_write_unencrypted_server(w).await,
            Self::SMSG_COMPRESSED_MOVES(c) => c.astd_write_unencrypted_server(w).await,
            Self::SMSG_CHAT_RESTRICTED => SMSG_CHAT_RESTRICTED{}.astd_write_unencrypted_server(w).await,
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
            Self::MSG_MOVE_TIME_SKIPPED(c) => c.astd_write_unencrypted_server(w).await,
            Self::SMSG_SPLINE_MOVE_ROOT(c) => c.astd_write_unencrypted_server(w).await,
            Self::SMSG_INVALIDATE_PLAYER(c) => c.astd_write_unencrypted_server(w).await,
            Self::SMSG_INSTANCE_RESET(c) => c.astd_write_unencrypted_server(w).await,
            Self::SMSG_INSTANCE_RESET_FAILED(c) => c.astd_write_unencrypted_server(w).await,
            Self::SMSG_UPDATE_LAST_INSTANCE(c) => c.astd_write_unencrypted_server(w).await,
            Self::MSG_RAID_TARGET_UPDATE(c) => c.astd_write_unencrypted_server(w).await,
            Self::MSG_RAID_READY_CHECK(c) => c.astd_write_unencrypted_server(w).await,
            Self::SMSG_PET_ACTION_SOUND(c) => c.astd_write_unencrypted_server(w).await,
            Self::SMSG_PET_DISMISS_SOUND(c) => c.astd_write_unencrypted_server(w).await,
            Self::SMSG_GM_TICKET_STATUS_UPDATE(c) => c.astd_write_unencrypted_server(w).await,
            Self::SMSG_UPDATE_INSTANCE_OWNERSHIP(c) => c.astd_write_unencrypted_server(w).await,
            Self::SMSG_SPELLINSTAKILLLOG(c) => c.astd_write_unencrypted_server(w).await,
            Self::SMSG_SPELL_UPDATE_CHAIN_TARGETS(c) => c.astd_write_unencrypted_server(w).await,
            Self::SMSG_EXPECTED_SPAM_RECORDS(c) => c.astd_write_unencrypted_server(w).await,
            Self::SMSG_DEFENSE_MESSAGE(c) => c.astd_write_unencrypted_server(w).await,
        }
    }

}

impl std::fmt::Display for ServerOpcodeMessage {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            ServerOpcodeMessage::MSG_MOVE_WORLDPORT_ACK => "MSG_MOVE_WORLDPORT_ACK",
            ServerOpcodeMessage::MSG_PETITION_DECLINE(_) => "MSG_PETITION_DECLINE",
            ServerOpcodeMessage::MSG_TABARDVENDOR_ACTIVATE(_) => "MSG_TABARDVENDOR_ACTIVATE",
            ServerOpcodeMessage::MSG_QUEST_PUSH_RESULT(_) => "MSG_QUEST_PUSH_RESULT",
            ServerOpcodeMessage::MSG_MOVE_WATER_WALK(_) => "MSG_MOVE_WATER_WALK",
            ServerOpcodeMessage::MSG_PETITION_RENAME(_) => "MSG_PETITION_RENAME",
            ServerOpcodeMessage::SMSG_CHAR_CREATE(_) => "SMSG_CHAR_CREATE",
            ServerOpcodeMessage::SMSG_CHAR_ENUM(_) => "SMSG_CHAR_ENUM",
            ServerOpcodeMessage::SMSG_CHAR_DELETE(_) => "SMSG_CHAR_DELETE",
            ServerOpcodeMessage::SMSG_NEW_WORLD(_) => "SMSG_NEW_WORLD",
            ServerOpcodeMessage::SMSG_TRANSFER_PENDING(_) => "SMSG_TRANSFER_PENDING",
            ServerOpcodeMessage::SMSG_TRANSFER_ABORTED(_) => "SMSG_TRANSFER_ABORTED",
            ServerOpcodeMessage::SMSG_CHARACTER_LOGIN_FAILED(_) => "SMSG_CHARACTER_LOGIN_FAILED",
            ServerOpcodeMessage::SMSG_LOGIN_SETTIMESPEED(_) => "SMSG_LOGIN_SETTIMESPEED",
            ServerOpcodeMessage::SMSG_LOGOUT_RESPONSE(_) => "SMSG_LOGOUT_RESPONSE",
            ServerOpcodeMessage::SMSG_LOGOUT_COMPLETE => "SMSG_LOGOUT_COMPLETE",
            ServerOpcodeMessage::SMSG_LOGOUT_CANCEL_ACK => "SMSG_LOGOUT_CANCEL_ACK",
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
            ServerOpcodeMessage::SMSG_FRIEND_LIST(_) => "SMSG_FRIEND_LIST",
            ServerOpcodeMessage::SMSG_FRIEND_STATUS(_) => "SMSG_FRIEND_STATUS",
            ServerOpcodeMessage::SMSG_IGNORE_LIST(_) => "SMSG_IGNORE_LIST",
            ServerOpcodeMessage::SMSG_GROUP_INVITE(_) => "SMSG_GROUP_INVITE",
            ServerOpcodeMessage::SMSG_GROUP_DECLINE(_) => "SMSG_GROUP_DECLINE",
            ServerOpcodeMessage::SMSG_GROUP_UNINVITE => "SMSG_GROUP_UNINVITE",
            ServerOpcodeMessage::SMSG_GROUP_SET_LEADER(_) => "SMSG_GROUP_SET_LEADER",
            ServerOpcodeMessage::SMSG_GROUP_DESTROYED => "SMSG_GROUP_DESTROYED",
            ServerOpcodeMessage::SMSG_GROUP_LIST(_) => "SMSG_GROUP_LIST",
            ServerOpcodeMessage::SMSG_PARTY_MEMBER_STATS(_) => "SMSG_PARTY_MEMBER_STATS",
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
            ServerOpcodeMessage::MSG_MOVE_START_FORWARD(_) => "MSG_MOVE_START_FORWARD_Server",
            ServerOpcodeMessage::MSG_MOVE_START_BACKWARD(_) => "MSG_MOVE_START_BACKWARD_Server",
            ServerOpcodeMessage::MSG_MOVE_STOP(_) => "MSG_MOVE_STOP_Server",
            ServerOpcodeMessage::MSG_MOVE_START_STRAFE_LEFT(_) => "MSG_MOVE_START_STRAFE_LEFT_Server",
            ServerOpcodeMessage::MSG_MOVE_START_STRAFE_RIGHT(_) => "MSG_MOVE_START_STRAFE_RIGHT_Server",
            ServerOpcodeMessage::MSG_MOVE_STOP_STRAFE(_) => "MSG_MOVE_STOP_STRAFE_Server",
            ServerOpcodeMessage::MSG_MOVE_JUMP(_) => "MSG_MOVE_JUMP_Server",
            ServerOpcodeMessage::MSG_MOVE_START_TURN_LEFT(_) => "MSG_MOVE_START_TURN_LEFT_Server",
            ServerOpcodeMessage::MSG_MOVE_START_TURN_RIGHT(_) => "MSG_MOVE_START_TURN_RIGHT_Server",
            ServerOpcodeMessage::MSG_MOVE_STOP_TURN(_) => "MSG_MOVE_STOP_TURN_Server",
            ServerOpcodeMessage::MSG_MOVE_START_PITCH_UP(_) => "MSG_MOVE_START_PITCH_UP_Server",
            ServerOpcodeMessage::MSG_MOVE_START_PITCH_DOWN(_) => "MSG_MOVE_START_PITCH_DOWN_Server",
            ServerOpcodeMessage::MSG_MOVE_STOP_PITCH(_) => "MSG_MOVE_STOP_PITCH_Server",
            ServerOpcodeMessage::MSG_MOVE_SET_RUN_MODE(_) => "MSG_MOVE_SET_RUN_MODE_Server",
            ServerOpcodeMessage::MSG_MOVE_SET_WALK_MODE(_) => "MSG_MOVE_SET_WALK_MODE_Server",
            ServerOpcodeMessage::MSG_MOVE_TELEPORT_ACK(_) => "MSG_MOVE_TELEPORT_ACK_Server",
            ServerOpcodeMessage::MSG_MOVE_FALL_LAND(_) => "MSG_MOVE_FALL_LAND_Server",
            ServerOpcodeMessage::MSG_MOVE_START_SWIM(_) => "MSG_MOVE_START_SWIM_Server",
            ServerOpcodeMessage::MSG_MOVE_STOP_SWIM(_) => "MSG_MOVE_STOP_SWIM_Server",
            ServerOpcodeMessage::MSG_MOVE_SET_FACING(_) => "MSG_MOVE_SET_FACING_Server",
            ServerOpcodeMessage::MSG_MOVE_SET_PITCH(_) => "MSG_MOVE_SET_PITCH_Server",
            ServerOpcodeMessage::SMSG_MONSTER_MOVE(_) => "SMSG_MONSTER_MOVE",
            ServerOpcodeMessage::SMSG_MOVE_WATER_WALK(_) => "SMSG_MOVE_WATER_WALK",
            ServerOpcodeMessage::SMSG_MOVE_LAND_WALK(_) => "SMSG_MOVE_LAND_WALK",
            ServerOpcodeMessage::SMSG_FORCE_RUN_SPEED_CHANGE(_) => "SMSG_FORCE_RUN_SPEED_CHANGE",
            ServerOpcodeMessage::SMSG_FORCE_RUN_BACK_SPEED_CHANGE(_) => "SMSG_FORCE_RUN_BACK_SPEED_CHANGE",
            ServerOpcodeMessage::SMSG_FORCE_SWIM_SPEED_CHANGE(_) => "SMSG_FORCE_SWIM_SPEED_CHANGE",
            ServerOpcodeMessage::SMSG_FORCE_MOVE_ROOT(_) => "SMSG_FORCE_MOVE_ROOT",
            ServerOpcodeMessage::SMSG_FORCE_MOVE_UNROOT(_) => "SMSG_FORCE_MOVE_UNROOT",
            ServerOpcodeMessage::MSG_MOVE_HEARTBEAT(_) => "MSG_MOVE_HEARTBEAT_Server",
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
            ServerOpcodeMessage::SMSG_OPEN_CONTAINER(_) => "SMSG_OPEN_CONTAINER",
            ServerOpcodeMessage::SMSG_INSPECT(_) => "SMSG_INSPECT",
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
            ServerOpcodeMessage::SMSG_CAST_RESULT(_) => "SMSG_CAST_RESULT",
            ServerOpcodeMessage::SMSG_SPELL_START(_) => "SMSG_SPELL_START",
            ServerOpcodeMessage::SMSG_SPELL_GO(_) => "SMSG_SPELL_GO",
            ServerOpcodeMessage::SMSG_SPELL_FAILURE(_) => "SMSG_SPELL_FAILURE",
            ServerOpcodeMessage::SMSG_SPELL_COOLDOWN(_) => "SMSG_SPELL_COOLDOWN",
            ServerOpcodeMessage::SMSG_COOLDOWN_EVENT(_) => "SMSG_COOLDOWN_EVENT",
            ServerOpcodeMessage::SMSG_UPDATE_AURA_DURATION(_) => "SMSG_UPDATE_AURA_DURATION",
            ServerOpcodeMessage::SMSG_PET_CAST_FAILED(_) => "SMSG_PET_CAST_FAILED",
            ServerOpcodeMessage::MSG_CHANNEL_START(_) => "MSG_CHANNEL_START_Server",
            ServerOpcodeMessage::MSG_CHANNEL_UPDATE(_) => "MSG_CHANNEL_UPDATE_Server",
            ServerOpcodeMessage::SMSG_AI_REACTION(_) => "SMSG_AI_REACTION",
            ServerOpcodeMessage::SMSG_ATTACKSTART(_) => "SMSG_ATTACKSTART",
            ServerOpcodeMessage::SMSG_ATTACKSTOP(_) => "SMSG_ATTACKSTOP",
            ServerOpcodeMessage::SMSG_ATTACKSWING_NOTINRANGE => "SMSG_ATTACKSWING_NOTINRANGE",
            ServerOpcodeMessage::SMSG_ATTACKSWING_BADFACING => "SMSG_ATTACKSWING_BADFACING",
            ServerOpcodeMessage::SMSG_ATTACKSWING_NOTSTANDING => "SMSG_ATTACKSWING_NOTSTANDING",
            ServerOpcodeMessage::SMSG_ATTACKSWING_DEADTARGET => "SMSG_ATTACKSWING_DEADTARGET",
            ServerOpcodeMessage::SMSG_ATTACKSWING_CANT_ATTACK => "SMSG_ATTACKSWING_CANT_ATTACK",
            ServerOpcodeMessage::SMSG_ATTACKERSTATEUPDATE(_) => "SMSG_ATTACKERSTATEUPDATE",
            ServerOpcodeMessage::SMSG_CANCEL_COMBAT => "SMSG_CANCEL_COMBAT",
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
            ServerOpcodeMessage::SMSG_LOOT_CLEAR_MONEY => "SMSG_LOOT_CLEAR_MONEY",
            ServerOpcodeMessage::SMSG_ITEM_PUSH_RESULT(_) => "SMSG_ITEM_PUSH_RESULT",
            ServerOpcodeMessage::SMSG_DUEL_REQUESTED(_) => "SMSG_DUEL_REQUESTED",
            ServerOpcodeMessage::SMSG_DUEL_OUTOFBOUNDS => "SMSG_DUEL_OUTOFBOUNDS",
            ServerOpcodeMessage::SMSG_DUEL_INBOUNDS => "SMSG_DUEL_INBOUNDS",
            ServerOpcodeMessage::SMSG_DUEL_COMPLETE(_) => "SMSG_DUEL_COMPLETE",
            ServerOpcodeMessage::SMSG_DUEL_WINNER(_) => "SMSG_DUEL_WINNER",
            ServerOpcodeMessage::SMSG_MOUNTRESULT(_) => "SMSG_MOUNTRESULT",
            ServerOpcodeMessage::SMSG_DISMOUNTRESULT(_) => "SMSG_DISMOUNTRESULT",
            ServerOpcodeMessage::SMSG_MOUNTSPECIAL_ANIM(_) => "SMSG_MOUNTSPECIAL_ANIM",
            ServerOpcodeMessage::SMSG_PET_TAME_FAILURE(_) => "SMSG_PET_TAME_FAILURE",
            ServerOpcodeMessage::SMSG_PET_NAME_INVALID => "SMSG_PET_NAME_INVALID",
            ServerOpcodeMessage::SMSG_PET_SPELLS(_) => "SMSG_PET_SPELLS",
            ServerOpcodeMessage::SMSG_PET_MODE(_) => "SMSG_PET_MODE",
            ServerOpcodeMessage::SMSG_GOSSIP_MESSAGE(_) => "SMSG_GOSSIP_MESSAGE",
            ServerOpcodeMessage::SMSG_GOSSIP_COMPLETE => "SMSG_GOSSIP_COMPLETE",
            ServerOpcodeMessage::SMSG_NPC_TEXT_UPDATE(_) => "SMSG_NPC_TEXT_UPDATE",
            ServerOpcodeMessage::SMSG_QUESTGIVER_STATUS(_) => "SMSG_QUESTGIVER_STATUS",
            ServerOpcodeMessage::SMSG_QUESTGIVER_QUEST_LIST(_) => "SMSG_QUESTGIVER_QUEST_LIST",
            ServerOpcodeMessage::SMSG_QUESTGIVER_QUEST_DETAILS(_) => "SMSG_QUESTGIVER_QUEST_DETAILS",
            ServerOpcodeMessage::SMSG_QUESTGIVER_REQUEST_ITEMS(_) => "SMSG_QUESTGIVER_REQUEST_ITEMS",
            ServerOpcodeMessage::SMSG_QUESTGIVER_OFFER_REWARD(_) => "SMSG_QUESTGIVER_OFFER_REWARD",
            ServerOpcodeMessage::SMSG_QUESTGIVER_QUEST_INVALID(_) => "SMSG_QUESTGIVER_QUEST_INVALID",
            ServerOpcodeMessage::SMSG_QUESTGIVER_QUEST_COMPLETE(_) => "SMSG_QUESTGIVER_QUEST_COMPLETE",
            ServerOpcodeMessage::SMSG_QUESTGIVER_QUEST_FAILED(_) => "SMSG_QUESTGIVER_QUEST_FAILED",
            ServerOpcodeMessage::SMSG_QUESTLOG_FULL => "SMSG_QUESTLOG_FULL",
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
            ServerOpcodeMessage::SMSG_NEW_TAXI_PATH => "SMSG_NEW_TAXI_PATH",
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
            ServerOpcodeMessage::SMSG_FISH_NOT_HOOKED => "SMSG_FISH_NOT_HOOKED",
            ServerOpcodeMessage::SMSG_FISH_ESCAPED => "SMSG_FISH_ESCAPED",
            ServerOpcodeMessage::SMSG_NOTIFICATION(_) => "SMSG_NOTIFICATION",
            ServerOpcodeMessage::SMSG_PLAYED_TIME(_) => "SMSG_PLAYED_TIME",
            ServerOpcodeMessage::SMSG_QUERY_TIME_RESPONSE(_) => "SMSG_QUERY_TIME_RESPONSE",
            ServerOpcodeMessage::SMSG_LOG_XPGAIN(_) => "SMSG_LOG_XPGAIN",
            ServerOpcodeMessage::SMSG_LEVELUP_INFO(_) => "SMSG_LEVELUP_INFO",
            ServerOpcodeMessage::MSG_MINIMAP_PING(_) => "MSG_MINIMAP_PING_Server",
            ServerOpcodeMessage::SMSG_RESISTLOG(_) => "SMSG_RESISTLOG",
            ServerOpcodeMessage::SMSG_ENCHANTMENTLOG(_) => "SMSG_ENCHANTMENTLOG",
            ServerOpcodeMessage::SMSG_START_MIRROR_TIMER(_) => "SMSG_START_MIRROR_TIMER",
            ServerOpcodeMessage::SMSG_PAUSE_MIRROR_TIMER(_) => "SMSG_PAUSE_MIRROR_TIMER",
            ServerOpcodeMessage::SMSG_STOP_MIRROR_TIMER(_) => "SMSG_STOP_MIRROR_TIMER",
            ServerOpcodeMessage::SMSG_PONG(_) => "SMSG_PONG",
            ServerOpcodeMessage::SMSG_CLEAR_COOLDOWN(_) => "SMSG_CLEAR_COOLDOWN",
            ServerOpcodeMessage::SMSG_GAMEOBJECT_PAGETEXT(_) => "SMSG_GAMEOBJECT_PAGETEXT",
            ServerOpcodeMessage::SMSG_SPELL_DELAYED(_) => "SMSG_SPELL_DELAYED",
            ServerOpcodeMessage::SMSG_ITEM_TIME_UPDATE(_) => "SMSG_ITEM_TIME_UPDATE",
            ServerOpcodeMessage::SMSG_ITEM_ENCHANT_TIME_UPDATE(_) => "SMSG_ITEM_ENCHANT_TIME_UPDATE",
            ServerOpcodeMessage::SMSG_AUTH_CHALLENGE(_) => "SMSG_AUTH_CHALLENGE",
            ServerOpcodeMessage::SMSG_AUTH_RESPONSE(_) => "SMSG_AUTH_RESPONSE",
            ServerOpcodeMessage::MSG_SAVE_GUILD_EMBLEM(_) => "MSG_SAVE_GUILD_EMBLEM_Server",
            ServerOpcodeMessage::SMSG_PLAY_SPELL_VISUAL(_) => "SMSG_PLAY_SPELL_VISUAL",
            ServerOpcodeMessage::SMSG_PARTYKILLLOG(_) => "SMSG_PARTYKILLLOG",
            ServerOpcodeMessage::SMSG_COMPRESSED_UPDATE_OBJECT(_) => "SMSG_COMPRESSED_UPDATE_OBJECT",
            ServerOpcodeMessage::SMSG_PLAY_SPELL_IMPACT(_) => "SMSG_PLAY_SPELL_IMPACT",
            ServerOpcodeMessage::SMSG_EXPLORATION_EXPERIENCE(_) => "SMSG_EXPLORATION_EXPERIENCE",
            ServerOpcodeMessage::MSG_RANDOM_ROLL(_) => "MSG_RANDOM_ROLL_Server",
            ServerOpcodeMessage::SMSG_ENVIRONMENTAL_DAMAGE_LOG(_) => "SMSG_ENVIRONMENTAL_DAMAGE_LOG",
            ServerOpcodeMessage::MSG_LOOKING_FOR_GROUP(_) => "MSG_LOOKING_FOR_GROUP_Server",
            ServerOpcodeMessage::SMSG_REMOVED_SPELL(_) => "SMSG_REMOVED_SPELL",
            ServerOpcodeMessage::SMSG_GMTICKET_CREATE(_) => "SMSG_GMTICKET_CREATE",
            ServerOpcodeMessage::SMSG_GMTICKET_UPDATETEXT(_) => "SMSG_GMTICKET_UPDATETEXT",
            ServerOpcodeMessage::SMSG_ACCOUNT_DATA_TIMES(_) => "SMSG_ACCOUNT_DATA_TIMES",
            ServerOpcodeMessage::SMSG_GMTICKET_GETTICKET(_) => "SMSG_GMTICKET_GETTICKET",
            ServerOpcodeMessage::SMSG_GAMEOBJECT_SPAWN_ANIM(_) => "SMSG_GAMEOBJECT_SPAWN_ANIM",
            ServerOpcodeMessage::SMSG_GAMEOBJECT_DESPAWN_ANIM(_) => "SMSG_GAMEOBJECT_DESPAWN_ANIM",
            ServerOpcodeMessage::MSG_CORPSE_QUERY(_) => "MSG_CORPSE_QUERY_Server",
            ServerOpcodeMessage::SMSG_GMTICKET_DELETETICKET(_) => "SMSG_GMTICKET_DELETETICKET",
            ServerOpcodeMessage::SMSG_CHAT_WRONG_FACTION => "SMSG_CHAT_WRONG_FACTION",
            ServerOpcodeMessage::SMSG_GMTICKET_SYSTEMSTATUS(_) => "SMSG_GMTICKET_SYSTEMSTATUS",
            ServerOpcodeMessage::SMSG_SET_REST_START(_) => "SMSG_SET_REST_START",
            ServerOpcodeMessage::SMSG_SPIRIT_HEALER_CONFIRM(_) => "SMSG_SPIRIT_HEALER_CONFIRM",
            ServerOpcodeMessage::SMSG_GOSSIP_POI(_) => "SMSG_GOSSIP_POI",
            ServerOpcodeMessage::SMSG_LOGIN_VERIFY_WORLD(_) => "SMSG_LOGIN_VERIFY_WORLD",
            ServerOpcodeMessage::SMSG_SEND_MAIL_RESULT(_) => "SMSG_SEND_MAIL_RESULT",
            ServerOpcodeMessage::SMSG_MAIL_LIST_RESULT(_) => "SMSG_MAIL_LIST_RESULT",
            ServerOpcodeMessage::SMSG_BATTLEFIELD_LIST(_) => "SMSG_BATTLEFIELD_LIST",
            ServerOpcodeMessage::SMSG_ITEM_TEXT_QUERY_RESPONSE(_) => "SMSG_ITEM_TEXT_QUERY_RESPONSE",
            ServerOpcodeMessage::SMSG_SPELLLOGMISS(_) => "SMSG_SPELLLOGMISS",
            ServerOpcodeMessage::SMSG_SPELLLOGEXECUTE(_) => "SMSG_SPELLLOGEXECUTE",
            ServerOpcodeMessage::SMSG_PERIODICAURALOG(_) => "SMSG_PERIODICAURALOG",
            ServerOpcodeMessage::SMSG_SPELLDAMAGESHIELD(_) => "SMSG_SPELLDAMAGESHIELD",
            ServerOpcodeMessage::SMSG_SPELLNONMELEEDAMAGELOG(_) => "SMSG_SPELLNONMELEEDAMAGELOG",
            ServerOpcodeMessage::SMSG_ZONE_UNDER_ATTACK(_) => "SMSG_ZONE_UNDER_ATTACK",
            ServerOpcodeMessage::MSG_AUCTION_HELLO(_) => "MSG_AUCTION_HELLO_Server",
            ServerOpcodeMessage::SMSG_AUCTION_COMMAND_RESULT(_) => "SMSG_AUCTION_COMMAND_RESULT",
            ServerOpcodeMessage::SMSG_AUCTION_LIST_RESULT(_) => "SMSG_AUCTION_LIST_RESULT",
            ServerOpcodeMessage::SMSG_AUCTION_OWNER_LIST_RESULT(_) => "SMSG_AUCTION_OWNER_LIST_RESULT",
            ServerOpcodeMessage::SMSG_AUCTION_BIDDER_NOTIFICATION(_) => "SMSG_AUCTION_BIDDER_NOTIFICATION",
            ServerOpcodeMessage::SMSG_AUCTION_OWNER_NOTIFICATION(_) => "SMSG_AUCTION_OWNER_NOTIFICATION",
            ServerOpcodeMessage::SMSG_PROCRESIST(_) => "SMSG_PROCRESIST",
            ServerOpcodeMessage::SMSG_DISPEL_FAILED(_) => "SMSG_DISPEL_FAILED",
            ServerOpcodeMessage::SMSG_SPELLORDAMAGE_IMMUNE(_) => "SMSG_SPELLORDAMAGE_IMMUNE",
            ServerOpcodeMessage::SMSG_AUCTION_BIDDER_LIST_RESULT(_) => "SMSG_AUCTION_BIDDER_LIST_RESULT",
            ServerOpcodeMessage::SMSG_SET_FLAT_SPELL_MODIFIER(_) => "SMSG_SET_FLAT_SPELL_MODIFIER",
            ServerOpcodeMessage::SMSG_SET_PCT_SPELL_MODIFIER(_) => "SMSG_SET_PCT_SPELL_MODIFIER",
            ServerOpcodeMessage::SMSG_CORPSE_RECLAIM_DELAY(_) => "SMSG_CORPSE_RECLAIM_DELAY",
            ServerOpcodeMessage::MSG_LIST_STABLED_PETS(_) => "MSG_LIST_STABLED_PETS_Server",
            ServerOpcodeMessage::SMSG_STABLE_RESULT(_) => "SMSG_STABLE_RESULT",
            ServerOpcodeMessage::SMSG_PLAY_MUSIC(_) => "SMSG_PLAY_MUSIC",
            ServerOpcodeMessage::SMSG_PLAY_OBJECT_SOUND(_) => "SMSG_PLAY_OBJECT_SOUND",
            ServerOpcodeMessage::SMSG_SPELLDISPELLOG(_) => "SMSG_SPELLDISPELLOG",
            ServerOpcodeMessage::MSG_QUERY_NEXT_MAIL_TIME(_) => "MSG_QUERY_NEXT_MAIL_TIME_Server",
            ServerOpcodeMessage::SMSG_RECEIVED_MAIL(_) => "SMSG_RECEIVED_MAIL",
            ServerOpcodeMessage::SMSG_RAID_GROUP_ONLY(_) => "SMSG_RAID_GROUP_ONLY",
            ServerOpcodeMessage::SMSG_PVP_CREDIT(_) => "SMSG_PVP_CREDIT",
            ServerOpcodeMessage::SMSG_AUCTION_REMOVED_NOTIFICATION(_) => "SMSG_AUCTION_REMOVED_NOTIFICATION",
            ServerOpcodeMessage::SMSG_SERVER_MESSAGE(_) => "SMSG_SERVER_MESSAGE",
            ServerOpcodeMessage::SMSG_MEETINGSTONE_SETQUEUE(_) => "SMSG_MEETINGSTONE_SETQUEUE",
            ServerOpcodeMessage::SMSG_MEETINGSTONE_COMPLETE => "SMSG_MEETINGSTONE_COMPLETE",
            ServerOpcodeMessage::SMSG_MEETINGSTONE_IN_PROGRESS => "SMSG_MEETINGSTONE_IN_PROGRESS",
            ServerOpcodeMessage::SMSG_MEETINGSTONE_MEMBER_ADDED(_) => "SMSG_MEETINGSTONE_MEMBER_ADDED",
            ServerOpcodeMessage::SMSG_CANCEL_AUTO_REPEAT => "SMSG_CANCEL_AUTO_REPEAT",
            ServerOpcodeMessage::SMSG_STANDSTATE_UPDATE(_) => "SMSG_STANDSTATE_UPDATE",
            ServerOpcodeMessage::SMSG_LOOT_ALL_PASSED(_) => "SMSG_LOOT_ALL_PASSED",
            ServerOpcodeMessage::SMSG_LOOT_ROLL_WON(_) => "SMSG_LOOT_ROLL_WON",
            ServerOpcodeMessage::SMSG_LOOT_START_ROLL(_) => "SMSG_LOOT_START_ROLL",
            ServerOpcodeMessage::SMSG_LOOT_ROLL(_) => "SMSG_LOOT_ROLL",
            ServerOpcodeMessage::SMSG_LOOT_MASTER_LIST(_) => "SMSG_LOOT_MASTER_LIST",
            ServerOpcodeMessage::SMSG_SET_FORCED_REACTIONS(_) => "SMSG_SET_FORCED_REACTIONS",
            ServerOpcodeMessage::SMSG_SPELL_FAILED_OTHER(_) => "SMSG_SPELL_FAILED_OTHER",
            ServerOpcodeMessage::SMSG_GAMEOBJECT_RESET_STATE(_) => "SMSG_GAMEOBJECT_RESET_STATE",
            ServerOpcodeMessage::SMSG_CHAT_PLAYER_NOT_FOUND(_) => "SMSG_CHAT_PLAYER_NOT_FOUND",
            ServerOpcodeMessage::MSG_TALENT_WIPE_CONFIRM(_) => "MSG_TALENT_WIPE_CONFIRM_Server",
            ServerOpcodeMessage::SMSG_SUMMON_REQUEST(_) => "SMSG_SUMMON_REQUEST",
            ServerOpcodeMessage::SMSG_MONSTER_MOVE_TRANSPORT(_) => "SMSG_MONSTER_MOVE_TRANSPORT",
            ServerOpcodeMessage::SMSG_PET_BROKEN => "SMSG_PET_BROKEN",
            ServerOpcodeMessage::MSG_MOVE_FEATHER_FALL(_) => "MSG_MOVE_FEATHER_FALL_Server",
            ServerOpcodeMessage::SMSG_FEIGN_DEATH_RESISTED => "SMSG_FEIGN_DEATH_RESISTED",
            ServerOpcodeMessage::SMSG_DUEL_COUNTDOWN(_) => "SMSG_DUEL_COUNTDOWN",
            ServerOpcodeMessage::SMSG_AREA_TRIGGER_MESSAGE(_) => "SMSG_AREA_TRIGGER_MESSAGE",
            ServerOpcodeMessage::SMSG_MEETINGSTONE_JOINFAILED(_) => "SMSG_MEETINGSTONE_JOINFAILED",
            ServerOpcodeMessage::SMSG_PLAYER_SKINNED(_) => "SMSG_PLAYER_SKINNED",
            ServerOpcodeMessage::SMSG_DURABILITY_DAMAGE_DEATH => "SMSG_DURABILITY_DAMAGE_DEATH",
            ServerOpcodeMessage::SMSG_INIT_WORLD_STATES(_) => "SMSG_INIT_WORLD_STATES",
            ServerOpcodeMessage::SMSG_UPDATE_WORLD_STATE(_) => "SMSG_UPDATE_WORLD_STATE",
            ServerOpcodeMessage::SMSG_ITEM_NAME_QUERY_RESPONSE(_) => "SMSG_ITEM_NAME_QUERY_RESPONSE",
            ServerOpcodeMessage::SMSG_PET_ACTION_FEEDBACK(_) => "SMSG_PET_ACTION_FEEDBACK",
            ServerOpcodeMessage::SMSG_CHAR_RENAME(_) => "SMSG_CHAR_RENAME",
            ServerOpcodeMessage::SMSG_INSTANCE_SAVE_CREATED(_) => "SMSG_INSTANCE_SAVE_CREATED",
            ServerOpcodeMessage::SMSG_RAID_INSTANCE_INFO(_) => "SMSG_RAID_INSTANCE_INFO",
            ServerOpcodeMessage::SMSG_PLAY_SOUND(_) => "SMSG_PLAY_SOUND",
            ServerOpcodeMessage::SMSG_BATTLEFIELD_STATUS(_) => "SMSG_BATTLEFIELD_STATUS",
            ServerOpcodeMessage::MSG_INSPECT_HONOR_STATS(_) => "MSG_INSPECT_HONOR_STATS_Server",
            ServerOpcodeMessage::SMSG_FORCE_WALK_SPEED_CHANGE(_) => "SMSG_FORCE_WALK_SPEED_CHANGE",
            ServerOpcodeMessage::SMSG_FORCE_SWIM_BACK_SPEED_CHANGE(_) => "SMSG_FORCE_SWIM_BACK_SPEED_CHANGE",
            ServerOpcodeMessage::SMSG_FORCE_TURN_RATE_CHANGE(_) => "SMSG_FORCE_TURN_RATE_CHANGE",
            ServerOpcodeMessage::MSG_PVP_LOG_DATA(_) => "MSG_PVP_LOG_DATA_Server",
            ServerOpcodeMessage::SMSG_AREA_SPIRIT_HEALER_TIME(_) => "SMSG_AREA_SPIRIT_HEALER_TIME",
            ServerOpcodeMessage::SMSG_WARDEN_DATA(_) => "SMSG_WARDEN_DATA",
            ServerOpcodeMessage::SMSG_GROUP_JOINED_BATTLEGROUND(_) => "SMSG_GROUP_JOINED_BATTLEGROUND",
            ServerOpcodeMessage::MSG_BATTLEGROUND_PLAYER_POSITIONS(_) => "MSG_BATTLEGROUND_PLAYER_POSITIONS_Server",
            ServerOpcodeMessage::SMSG_BINDER_CONFIRM(_) => "SMSG_BINDER_CONFIRM",
            ServerOpcodeMessage::SMSG_BATTLEGROUND_PLAYER_JOINED(_) => "SMSG_BATTLEGROUND_PLAYER_JOINED",
            ServerOpcodeMessage::SMSG_BATTLEGROUND_PLAYER_LEFT(_) => "SMSG_BATTLEGROUND_PLAYER_LEFT",
            ServerOpcodeMessage::SMSG_ADDON_INFO(_) => "SMSG_ADDON_INFO",
            ServerOpcodeMessage::SMSG_PET_UNLEARN_CONFIRM(_) => "SMSG_PET_UNLEARN_CONFIRM",
            ServerOpcodeMessage::SMSG_PARTY_MEMBER_STATS_FULL(_) => "SMSG_PARTY_MEMBER_STATS_FULL",
            ServerOpcodeMessage::SMSG_WEATHER(_) => "SMSG_WEATHER",
            ServerOpcodeMessage::SMSG_RAID_INSTANCE_MESSAGE(_) => "SMSG_RAID_INSTANCE_MESSAGE",
            ServerOpcodeMessage::SMSG_COMPRESSED_MOVES(_) => "SMSG_COMPRESSED_MOVES",
            ServerOpcodeMessage::SMSG_CHAT_RESTRICTED => "SMSG_CHAT_RESTRICTED",
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
            ServerOpcodeMessage::MSG_MOVE_TIME_SKIPPED(_) => "MSG_MOVE_TIME_SKIPPED_Server",
            ServerOpcodeMessage::SMSG_SPLINE_MOVE_ROOT(_) => "SMSG_SPLINE_MOVE_ROOT",
            ServerOpcodeMessage::SMSG_INVALIDATE_PLAYER(_) => "SMSG_INVALIDATE_PLAYER",
            ServerOpcodeMessage::SMSG_INSTANCE_RESET(_) => "SMSG_INSTANCE_RESET",
            ServerOpcodeMessage::SMSG_INSTANCE_RESET_FAILED(_) => "SMSG_INSTANCE_RESET_FAILED",
            ServerOpcodeMessage::SMSG_UPDATE_LAST_INSTANCE(_) => "SMSG_UPDATE_LAST_INSTANCE",
            ServerOpcodeMessage::MSG_RAID_TARGET_UPDATE(_) => "MSG_RAID_TARGET_UPDATE_Server",
            ServerOpcodeMessage::MSG_RAID_READY_CHECK(_) => "MSG_RAID_READY_CHECK_Server",
            ServerOpcodeMessage::SMSG_PET_ACTION_SOUND(_) => "SMSG_PET_ACTION_SOUND",
            ServerOpcodeMessage::SMSG_PET_DISMISS_SOUND(_) => "SMSG_PET_DISMISS_SOUND",
            ServerOpcodeMessage::SMSG_GM_TICKET_STATUS_UPDATE(_) => "SMSG_GM_TICKET_STATUS_UPDATE",
            ServerOpcodeMessage::SMSG_UPDATE_INSTANCE_OWNERSHIP(_) => "SMSG_UPDATE_INSTANCE_OWNERSHIP",
            ServerOpcodeMessage::SMSG_SPELLINSTAKILLLOG(_) => "SMSG_SPELLINSTAKILLLOG",
            ServerOpcodeMessage::SMSG_SPELL_UPDATE_CHAIN_TARGETS(_) => "SMSG_SPELL_UPDATE_CHAIN_TARGETS",
            ServerOpcodeMessage::SMSG_EXPECTED_SPAM_RECORDS(_) => "SMSG_EXPECTED_SPAM_RECORDS",
            ServerOpcodeMessage::SMSG_DEFENSE_MESSAGE(_) => "SMSG_DEFENSE_MESSAGE",
        })
    }
}

impl From<MSG_MOVE_WORLDPORT_ACK> for ServerOpcodeMessage {
    fn from(_: MSG_MOVE_WORLDPORT_ACK) -> Self {
        Self::MSG_MOVE_WORLDPORT_ACK
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

impl From<MSG_MOVE_WATER_WALK> for ServerOpcodeMessage {
    fn from(c: MSG_MOVE_WATER_WALK) -> Self {
        Self::MSG_MOVE_WATER_WALK(c)
    }
}

impl From<MSG_PETITION_RENAME> for ServerOpcodeMessage {
    fn from(c: MSG_PETITION_RENAME) -> Self {
        Self::MSG_PETITION_RENAME(c)
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
    fn from(_: SMSG_LOGOUT_COMPLETE) -> Self {
        Self::SMSG_LOGOUT_COMPLETE
    }
}

impl From<SMSG_LOGOUT_CANCEL_ACK> for ServerOpcodeMessage {
    fn from(_: SMSG_LOGOUT_CANCEL_ACK) -> Self {
        Self::SMSG_LOGOUT_CANCEL_ACK
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

impl From<SMSG_FRIEND_LIST> for ServerOpcodeMessage {
    fn from(c: SMSG_FRIEND_LIST) -> Self {
        Self::SMSG_FRIEND_LIST(c)
    }
}

impl From<SMSG_FRIEND_STATUS> for ServerOpcodeMessage {
    fn from(c: SMSG_FRIEND_STATUS) -> Self {
        Self::SMSG_FRIEND_STATUS(c)
    }
}

impl From<SMSG_IGNORE_LIST> for ServerOpcodeMessage {
    fn from(c: SMSG_IGNORE_LIST) -> Self {
        Self::SMSG_IGNORE_LIST(c)
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
    fn from(_: SMSG_GROUP_UNINVITE) -> Self {
        Self::SMSG_GROUP_UNINVITE
    }
}

impl From<SMSG_GROUP_SET_LEADER> for ServerOpcodeMessage {
    fn from(c: SMSG_GROUP_SET_LEADER) -> Self {
        Self::SMSG_GROUP_SET_LEADER(c)
    }
}

impl From<SMSG_GROUP_DESTROYED> for ServerOpcodeMessage {
    fn from(_: SMSG_GROUP_DESTROYED) -> Self {
        Self::SMSG_GROUP_DESTROYED
    }
}

impl From<SMSG_GROUP_LIST> for ServerOpcodeMessage {
    fn from(c: SMSG_GROUP_LIST) -> Self {
        Self::SMSG_GROUP_LIST(c)
    }
}

impl From<SMSG_PARTY_MEMBER_STATS> for ServerOpcodeMessage {
    fn from(c: SMSG_PARTY_MEMBER_STATS) -> Self {
        Self::SMSG_PARTY_MEMBER_STATS(c)
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

impl From<MSG_MOVE_START_FORWARD_Server> for ServerOpcodeMessage {
    fn from(c: MSG_MOVE_START_FORWARD_Server) -> Self {
        Self::MSG_MOVE_START_FORWARD(c)
    }
}

impl From<MSG_MOVE_START_BACKWARD_Server> for ServerOpcodeMessage {
    fn from(c: MSG_MOVE_START_BACKWARD_Server) -> Self {
        Self::MSG_MOVE_START_BACKWARD(c)
    }
}

impl From<MSG_MOVE_STOP_Server> for ServerOpcodeMessage {
    fn from(c: MSG_MOVE_STOP_Server) -> Self {
        Self::MSG_MOVE_STOP(c)
    }
}

impl From<MSG_MOVE_START_STRAFE_LEFT_Server> for ServerOpcodeMessage {
    fn from(c: MSG_MOVE_START_STRAFE_LEFT_Server) -> Self {
        Self::MSG_MOVE_START_STRAFE_LEFT(c)
    }
}

impl From<MSG_MOVE_START_STRAFE_RIGHT_Server> for ServerOpcodeMessage {
    fn from(c: MSG_MOVE_START_STRAFE_RIGHT_Server) -> Self {
        Self::MSG_MOVE_START_STRAFE_RIGHT(c)
    }
}

impl From<MSG_MOVE_STOP_STRAFE_Server> for ServerOpcodeMessage {
    fn from(c: MSG_MOVE_STOP_STRAFE_Server) -> Self {
        Self::MSG_MOVE_STOP_STRAFE(c)
    }
}

impl From<MSG_MOVE_JUMP_Server> for ServerOpcodeMessage {
    fn from(c: MSG_MOVE_JUMP_Server) -> Self {
        Self::MSG_MOVE_JUMP(c)
    }
}

impl From<MSG_MOVE_START_TURN_LEFT_Server> for ServerOpcodeMessage {
    fn from(c: MSG_MOVE_START_TURN_LEFT_Server) -> Self {
        Self::MSG_MOVE_START_TURN_LEFT(c)
    }
}

impl From<MSG_MOVE_START_TURN_RIGHT_Server> for ServerOpcodeMessage {
    fn from(c: MSG_MOVE_START_TURN_RIGHT_Server) -> Self {
        Self::MSG_MOVE_START_TURN_RIGHT(c)
    }
}

impl From<MSG_MOVE_STOP_TURN_Server> for ServerOpcodeMessage {
    fn from(c: MSG_MOVE_STOP_TURN_Server) -> Self {
        Self::MSG_MOVE_STOP_TURN(c)
    }
}

impl From<MSG_MOVE_START_PITCH_UP_Server> for ServerOpcodeMessage {
    fn from(c: MSG_MOVE_START_PITCH_UP_Server) -> Self {
        Self::MSG_MOVE_START_PITCH_UP(c)
    }
}

impl From<MSG_MOVE_START_PITCH_DOWN_Server> for ServerOpcodeMessage {
    fn from(c: MSG_MOVE_START_PITCH_DOWN_Server) -> Self {
        Self::MSG_MOVE_START_PITCH_DOWN(c)
    }
}

impl From<MSG_MOVE_STOP_PITCH_Server> for ServerOpcodeMessage {
    fn from(c: MSG_MOVE_STOP_PITCH_Server) -> Self {
        Self::MSG_MOVE_STOP_PITCH(c)
    }
}

impl From<MSG_MOVE_SET_RUN_MODE_Server> for ServerOpcodeMessage {
    fn from(c: MSG_MOVE_SET_RUN_MODE_Server) -> Self {
        Self::MSG_MOVE_SET_RUN_MODE(c)
    }
}

impl From<MSG_MOVE_SET_WALK_MODE_Server> for ServerOpcodeMessage {
    fn from(c: MSG_MOVE_SET_WALK_MODE_Server) -> Self {
        Self::MSG_MOVE_SET_WALK_MODE(c)
    }
}

impl From<MSG_MOVE_TELEPORT_ACK_Server> for ServerOpcodeMessage {
    fn from(c: MSG_MOVE_TELEPORT_ACK_Server) -> Self {
        Self::MSG_MOVE_TELEPORT_ACK(c)
    }
}

impl From<MSG_MOVE_FALL_LAND_Server> for ServerOpcodeMessage {
    fn from(c: MSG_MOVE_FALL_LAND_Server) -> Self {
        Self::MSG_MOVE_FALL_LAND(c)
    }
}

impl From<MSG_MOVE_START_SWIM_Server> for ServerOpcodeMessage {
    fn from(c: MSG_MOVE_START_SWIM_Server) -> Self {
        Self::MSG_MOVE_START_SWIM(c)
    }
}

impl From<MSG_MOVE_STOP_SWIM_Server> for ServerOpcodeMessage {
    fn from(c: MSG_MOVE_STOP_SWIM_Server) -> Self {
        Self::MSG_MOVE_STOP_SWIM(c)
    }
}

impl From<MSG_MOVE_SET_FACING_Server> for ServerOpcodeMessage {
    fn from(c: MSG_MOVE_SET_FACING_Server) -> Self {
        Self::MSG_MOVE_SET_FACING(c)
    }
}

impl From<MSG_MOVE_SET_PITCH_Server> for ServerOpcodeMessage {
    fn from(c: MSG_MOVE_SET_PITCH_Server) -> Self {
        Self::MSG_MOVE_SET_PITCH(c)
    }
}

impl From<SMSG_MONSTER_MOVE> for ServerOpcodeMessage {
    fn from(c: SMSG_MONSTER_MOVE) -> Self {
        Self::SMSG_MONSTER_MOVE(c)
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

impl From<MSG_MOVE_HEARTBEAT_Server> for ServerOpcodeMessage {
    fn from(c: MSG_MOVE_HEARTBEAT_Server) -> Self {
        Self::MSG_MOVE_HEARTBEAT(c)
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

impl From<SMSG_OPEN_CONTAINER> for ServerOpcodeMessage {
    fn from(c: SMSG_OPEN_CONTAINER) -> Self {
        Self::SMSG_OPEN_CONTAINER(c)
    }
}

impl From<SMSG_INSPECT> for ServerOpcodeMessage {
    fn from(c: SMSG_INSPECT) -> Self {
        Self::SMSG_INSPECT(c)
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

impl From<SMSG_CAST_RESULT> for ServerOpcodeMessage {
    fn from(c: SMSG_CAST_RESULT) -> Self {
        Self::SMSG_CAST_RESULT(c)
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

impl From<SMSG_UPDATE_AURA_DURATION> for ServerOpcodeMessage {
    fn from(c: SMSG_UPDATE_AURA_DURATION) -> Self {
        Self::SMSG_UPDATE_AURA_DURATION(c)
    }
}

impl From<SMSG_PET_CAST_FAILED> for ServerOpcodeMessage {
    fn from(c: SMSG_PET_CAST_FAILED) -> Self {
        Self::SMSG_PET_CAST_FAILED(c)
    }
}

impl From<MSG_CHANNEL_START_Server> for ServerOpcodeMessage {
    fn from(c: MSG_CHANNEL_START_Server) -> Self {
        Self::MSG_CHANNEL_START(c)
    }
}

impl From<MSG_CHANNEL_UPDATE_Server> for ServerOpcodeMessage {
    fn from(c: MSG_CHANNEL_UPDATE_Server) -> Self {
        Self::MSG_CHANNEL_UPDATE(c)
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
    fn from(_: SMSG_ATTACKSWING_NOTINRANGE) -> Self {
        Self::SMSG_ATTACKSWING_NOTINRANGE
    }
}

impl From<SMSG_ATTACKSWING_BADFACING> for ServerOpcodeMessage {
    fn from(_: SMSG_ATTACKSWING_BADFACING) -> Self {
        Self::SMSG_ATTACKSWING_BADFACING
    }
}

impl From<SMSG_ATTACKSWING_NOTSTANDING> for ServerOpcodeMessage {
    fn from(_: SMSG_ATTACKSWING_NOTSTANDING) -> Self {
        Self::SMSG_ATTACKSWING_NOTSTANDING
    }
}

impl From<SMSG_ATTACKSWING_DEADTARGET> for ServerOpcodeMessage {
    fn from(_: SMSG_ATTACKSWING_DEADTARGET) -> Self {
        Self::SMSG_ATTACKSWING_DEADTARGET
    }
}

impl From<SMSG_ATTACKSWING_CANT_ATTACK> for ServerOpcodeMessage {
    fn from(_: SMSG_ATTACKSWING_CANT_ATTACK) -> Self {
        Self::SMSG_ATTACKSWING_CANT_ATTACK
    }
}

impl From<SMSG_ATTACKERSTATEUPDATE> for ServerOpcodeMessage {
    fn from(c: SMSG_ATTACKERSTATEUPDATE) -> Self {
        Self::SMSG_ATTACKERSTATEUPDATE(c)
    }
}

impl From<SMSG_CANCEL_COMBAT> for ServerOpcodeMessage {
    fn from(_: SMSG_CANCEL_COMBAT) -> Self {
        Self::SMSG_CANCEL_COMBAT
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
    fn from(_: SMSG_LOOT_CLEAR_MONEY) -> Self {
        Self::SMSG_LOOT_CLEAR_MONEY
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
    fn from(_: SMSG_DUEL_OUTOFBOUNDS) -> Self {
        Self::SMSG_DUEL_OUTOFBOUNDS
    }
}

impl From<SMSG_DUEL_INBOUNDS> for ServerOpcodeMessage {
    fn from(_: SMSG_DUEL_INBOUNDS) -> Self {
        Self::SMSG_DUEL_INBOUNDS
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

impl From<SMSG_DISMOUNTRESULT> for ServerOpcodeMessage {
    fn from(c: SMSG_DISMOUNTRESULT) -> Self {
        Self::SMSG_DISMOUNTRESULT(c)
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
    fn from(_: SMSG_PET_NAME_INVALID) -> Self {
        Self::SMSG_PET_NAME_INVALID
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
    fn from(_: SMSG_GOSSIP_COMPLETE) -> Self {
        Self::SMSG_GOSSIP_COMPLETE
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
    fn from(_: SMSG_QUESTLOG_FULL) -> Self {
        Self::SMSG_QUESTLOG_FULL
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
    fn from(_: SMSG_NEW_TAXI_PATH) -> Self {
        Self::SMSG_NEW_TAXI_PATH
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
    fn from(_: SMSG_FISH_NOT_HOOKED) -> Self {
        Self::SMSG_FISH_NOT_HOOKED
    }
}

impl From<SMSG_FISH_ESCAPED> for ServerOpcodeMessage {
    fn from(_: SMSG_FISH_ESCAPED) -> Self {
        Self::SMSG_FISH_ESCAPED
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

impl From<SMSG_RESISTLOG> for ServerOpcodeMessage {
    fn from(c: SMSG_RESISTLOG) -> Self {
        Self::SMSG_RESISTLOG(c)
    }
}

impl From<SMSG_ENCHANTMENTLOG> for ServerOpcodeMessage {
    fn from(c: SMSG_ENCHANTMENTLOG) -> Self {
        Self::SMSG_ENCHANTMENTLOG(c)
    }
}

impl From<SMSG_START_MIRROR_TIMER> for ServerOpcodeMessage {
    fn from(c: SMSG_START_MIRROR_TIMER) -> Self {
        Self::SMSG_START_MIRROR_TIMER(c)
    }
}

impl From<SMSG_PAUSE_MIRROR_TIMER> for ServerOpcodeMessage {
    fn from(c: SMSG_PAUSE_MIRROR_TIMER) -> Self {
        Self::SMSG_PAUSE_MIRROR_TIMER(c)
    }
}

impl From<SMSG_STOP_MIRROR_TIMER> for ServerOpcodeMessage {
    fn from(c: SMSG_STOP_MIRROR_TIMER) -> Self {
        Self::SMSG_STOP_MIRROR_TIMER(c)
    }
}

impl From<SMSG_PONG> for ServerOpcodeMessage {
    fn from(c: SMSG_PONG) -> Self {
        Self::SMSG_PONG(c)
    }
}

impl From<SMSG_CLEAR_COOLDOWN> for ServerOpcodeMessage {
    fn from(c: SMSG_CLEAR_COOLDOWN) -> Self {
        Self::SMSG_CLEAR_COOLDOWN(c)
    }
}

impl From<SMSG_GAMEOBJECT_PAGETEXT> for ServerOpcodeMessage {
    fn from(c: SMSG_GAMEOBJECT_PAGETEXT) -> Self {
        Self::SMSG_GAMEOBJECT_PAGETEXT(c)
    }
}

impl From<SMSG_SPELL_DELAYED> for ServerOpcodeMessage {
    fn from(c: SMSG_SPELL_DELAYED) -> Self {
        Self::SMSG_SPELL_DELAYED(c)
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

impl From<SMSG_PLAY_SPELL_VISUAL> for ServerOpcodeMessage {
    fn from(c: SMSG_PLAY_SPELL_VISUAL) -> Self {
        Self::SMSG_PLAY_SPELL_VISUAL(c)
    }
}

impl From<SMSG_PARTYKILLLOG> for ServerOpcodeMessage {
    fn from(c: SMSG_PARTYKILLLOG) -> Self {
        Self::SMSG_PARTYKILLLOG(c)
    }
}

impl From<SMSG_COMPRESSED_UPDATE_OBJECT> for ServerOpcodeMessage {
    fn from(c: SMSG_COMPRESSED_UPDATE_OBJECT) -> Self {
        Self::SMSG_COMPRESSED_UPDATE_OBJECT(c)
    }
}

impl From<SMSG_PLAY_SPELL_IMPACT> for ServerOpcodeMessage {
    fn from(c: SMSG_PLAY_SPELL_IMPACT) -> Self {
        Self::SMSG_PLAY_SPELL_IMPACT(c)
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

impl From<MSG_LOOKING_FOR_GROUP_Server> for ServerOpcodeMessage {
    fn from(c: MSG_LOOKING_FOR_GROUP_Server) -> Self {
        Self::MSG_LOOKING_FOR_GROUP(c)
    }
}

impl From<SMSG_REMOVED_SPELL> for ServerOpcodeMessage {
    fn from(c: SMSG_REMOVED_SPELL) -> Self {
        Self::SMSG_REMOVED_SPELL(c)
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

impl From<SMSG_GMTICKET_GETTICKET> for ServerOpcodeMessage {
    fn from(c: SMSG_GMTICKET_GETTICKET) -> Self {
        Self::SMSG_GMTICKET_GETTICKET(c)
    }
}

impl From<SMSG_GAMEOBJECT_SPAWN_ANIM> for ServerOpcodeMessage {
    fn from(c: SMSG_GAMEOBJECT_SPAWN_ANIM) -> Self {
        Self::SMSG_GAMEOBJECT_SPAWN_ANIM(c)
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
    fn from(_: SMSG_CHAT_WRONG_FACTION) -> Self {
        Self::SMSG_CHAT_WRONG_FACTION
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

impl From<SMSG_SPIRIT_HEALER_CONFIRM> for ServerOpcodeMessage {
    fn from(c: SMSG_SPIRIT_HEALER_CONFIRM) -> Self {
        Self::SMSG_SPIRIT_HEALER_CONFIRM(c)
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

impl From<SMSG_SEND_MAIL_RESULT> for ServerOpcodeMessage {
    fn from(c: SMSG_SEND_MAIL_RESULT) -> Self {
        Self::SMSG_SEND_MAIL_RESULT(c)
    }
}

impl From<SMSG_MAIL_LIST_RESULT> for ServerOpcodeMessage {
    fn from(c: SMSG_MAIL_LIST_RESULT) -> Self {
        Self::SMSG_MAIL_LIST_RESULT(c)
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

impl From<SMSG_SPELLLOGMISS> for ServerOpcodeMessage {
    fn from(c: SMSG_SPELLLOGMISS) -> Self {
        Self::SMSG_SPELLLOGMISS(c)
    }
}

impl From<SMSG_SPELLLOGEXECUTE> for ServerOpcodeMessage {
    fn from(c: SMSG_SPELLLOGEXECUTE) -> Self {
        Self::SMSG_SPELLLOGEXECUTE(c)
    }
}

impl From<SMSG_PERIODICAURALOG> for ServerOpcodeMessage {
    fn from(c: SMSG_PERIODICAURALOG) -> Self {
        Self::SMSG_PERIODICAURALOG(c)
    }
}

impl From<SMSG_SPELLDAMAGESHIELD> for ServerOpcodeMessage {
    fn from(c: SMSG_SPELLDAMAGESHIELD) -> Self {
        Self::SMSG_SPELLDAMAGESHIELD(c)
    }
}

impl From<SMSG_SPELLNONMELEEDAMAGELOG> for ServerOpcodeMessage {
    fn from(c: SMSG_SPELLNONMELEEDAMAGELOG) -> Self {
        Self::SMSG_SPELLNONMELEEDAMAGELOG(c)
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

impl From<SMSG_AUCTION_COMMAND_RESULT> for ServerOpcodeMessage {
    fn from(c: SMSG_AUCTION_COMMAND_RESULT) -> Self {
        Self::SMSG_AUCTION_COMMAND_RESULT(c)
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

impl From<SMSG_DISPEL_FAILED> for ServerOpcodeMessage {
    fn from(c: SMSG_DISPEL_FAILED) -> Self {
        Self::SMSG_DISPEL_FAILED(c)
    }
}

impl From<SMSG_SPELLORDAMAGE_IMMUNE> for ServerOpcodeMessage {
    fn from(c: SMSG_SPELLORDAMAGE_IMMUNE) -> Self {
        Self::SMSG_SPELLORDAMAGE_IMMUNE(c)
    }
}

impl From<SMSG_AUCTION_BIDDER_LIST_RESULT> for ServerOpcodeMessage {
    fn from(c: SMSG_AUCTION_BIDDER_LIST_RESULT) -> Self {
        Self::SMSG_AUCTION_BIDDER_LIST_RESULT(c)
    }
}

impl From<SMSG_SET_FLAT_SPELL_MODIFIER> for ServerOpcodeMessage {
    fn from(c: SMSG_SET_FLAT_SPELL_MODIFIER) -> Self {
        Self::SMSG_SET_FLAT_SPELL_MODIFIER(c)
    }
}

impl From<SMSG_SET_PCT_SPELL_MODIFIER> for ServerOpcodeMessage {
    fn from(c: SMSG_SET_PCT_SPELL_MODIFIER) -> Self {
        Self::SMSG_SET_PCT_SPELL_MODIFIER(c)
    }
}

impl From<SMSG_CORPSE_RECLAIM_DELAY> for ServerOpcodeMessage {
    fn from(c: SMSG_CORPSE_RECLAIM_DELAY) -> Self {
        Self::SMSG_CORPSE_RECLAIM_DELAY(c)
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

impl From<SMSG_PLAY_MUSIC> for ServerOpcodeMessage {
    fn from(c: SMSG_PLAY_MUSIC) -> Self {
        Self::SMSG_PLAY_MUSIC(c)
    }
}

impl From<SMSG_PLAY_OBJECT_SOUND> for ServerOpcodeMessage {
    fn from(c: SMSG_PLAY_OBJECT_SOUND) -> Self {
        Self::SMSG_PLAY_OBJECT_SOUND(c)
    }
}

impl From<SMSG_SPELLDISPELLOG> for ServerOpcodeMessage {
    fn from(c: SMSG_SPELLDISPELLOG) -> Self {
        Self::SMSG_SPELLDISPELLOG(c)
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

impl From<SMSG_RAID_GROUP_ONLY> for ServerOpcodeMessage {
    fn from(c: SMSG_RAID_GROUP_ONLY) -> Self {
        Self::SMSG_RAID_GROUP_ONLY(c)
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

impl From<SMSG_MEETINGSTONE_SETQUEUE> for ServerOpcodeMessage {
    fn from(c: SMSG_MEETINGSTONE_SETQUEUE) -> Self {
        Self::SMSG_MEETINGSTONE_SETQUEUE(c)
    }
}

impl From<SMSG_MEETINGSTONE_COMPLETE> for ServerOpcodeMessage {
    fn from(_: SMSG_MEETINGSTONE_COMPLETE) -> Self {
        Self::SMSG_MEETINGSTONE_COMPLETE
    }
}

impl From<SMSG_MEETINGSTONE_IN_PROGRESS> for ServerOpcodeMessage {
    fn from(_: SMSG_MEETINGSTONE_IN_PROGRESS) -> Self {
        Self::SMSG_MEETINGSTONE_IN_PROGRESS
    }
}

impl From<SMSG_MEETINGSTONE_MEMBER_ADDED> for ServerOpcodeMessage {
    fn from(c: SMSG_MEETINGSTONE_MEMBER_ADDED) -> Self {
        Self::SMSG_MEETINGSTONE_MEMBER_ADDED(c)
    }
}

impl From<SMSG_CANCEL_AUTO_REPEAT> for ServerOpcodeMessage {
    fn from(_: SMSG_CANCEL_AUTO_REPEAT) -> Self {
        Self::SMSG_CANCEL_AUTO_REPEAT
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

impl From<SMSG_SPELL_FAILED_OTHER> for ServerOpcodeMessage {
    fn from(c: SMSG_SPELL_FAILED_OTHER) -> Self {
        Self::SMSG_SPELL_FAILED_OTHER(c)
    }
}

impl From<SMSG_GAMEOBJECT_RESET_STATE> for ServerOpcodeMessage {
    fn from(c: SMSG_GAMEOBJECT_RESET_STATE) -> Self {
        Self::SMSG_GAMEOBJECT_RESET_STATE(c)
    }
}

impl From<SMSG_CHAT_PLAYER_NOT_FOUND> for ServerOpcodeMessage {
    fn from(c: SMSG_CHAT_PLAYER_NOT_FOUND) -> Self {
        Self::SMSG_CHAT_PLAYER_NOT_FOUND(c)
    }
}

impl From<MSG_TALENT_WIPE_CONFIRM_Server> for ServerOpcodeMessage {
    fn from(c: MSG_TALENT_WIPE_CONFIRM_Server) -> Self {
        Self::MSG_TALENT_WIPE_CONFIRM(c)
    }
}

impl From<SMSG_SUMMON_REQUEST> for ServerOpcodeMessage {
    fn from(c: SMSG_SUMMON_REQUEST) -> Self {
        Self::SMSG_SUMMON_REQUEST(c)
    }
}

impl From<SMSG_MONSTER_MOVE_TRANSPORT> for ServerOpcodeMessage {
    fn from(c: SMSG_MONSTER_MOVE_TRANSPORT) -> Self {
        Self::SMSG_MONSTER_MOVE_TRANSPORT(c)
    }
}

impl From<SMSG_PET_BROKEN> for ServerOpcodeMessage {
    fn from(_: SMSG_PET_BROKEN) -> Self {
        Self::SMSG_PET_BROKEN
    }
}

impl From<MSG_MOVE_FEATHER_FALL_Server> for ServerOpcodeMessage {
    fn from(c: MSG_MOVE_FEATHER_FALL_Server) -> Self {
        Self::MSG_MOVE_FEATHER_FALL(c)
    }
}

impl From<SMSG_FEIGN_DEATH_RESISTED> for ServerOpcodeMessage {
    fn from(_: SMSG_FEIGN_DEATH_RESISTED) -> Self {
        Self::SMSG_FEIGN_DEATH_RESISTED
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

impl From<SMSG_MEETINGSTONE_JOINFAILED> for ServerOpcodeMessage {
    fn from(c: SMSG_MEETINGSTONE_JOINFAILED) -> Self {
        Self::SMSG_MEETINGSTONE_JOINFAILED(c)
    }
}

impl From<SMSG_PLAYER_SKINNED> for ServerOpcodeMessage {
    fn from(c: SMSG_PLAYER_SKINNED) -> Self {
        Self::SMSG_PLAYER_SKINNED(c)
    }
}

impl From<SMSG_DURABILITY_DAMAGE_DEATH> for ServerOpcodeMessage {
    fn from(_: SMSG_DURABILITY_DAMAGE_DEATH) -> Self {
        Self::SMSG_DURABILITY_DAMAGE_DEATH
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

impl From<SMSG_INSTANCE_SAVE_CREATED> for ServerOpcodeMessage {
    fn from(c: SMSG_INSTANCE_SAVE_CREATED) -> Self {
        Self::SMSG_INSTANCE_SAVE_CREATED(c)
    }
}

impl From<SMSG_RAID_INSTANCE_INFO> for ServerOpcodeMessage {
    fn from(c: SMSG_RAID_INSTANCE_INFO) -> Self {
        Self::SMSG_RAID_INSTANCE_INFO(c)
    }
}

impl From<SMSG_PLAY_SOUND> for ServerOpcodeMessage {
    fn from(c: SMSG_PLAY_SOUND) -> Self {
        Self::SMSG_PLAY_SOUND(c)
    }
}

impl From<SMSG_BATTLEFIELD_STATUS> for ServerOpcodeMessage {
    fn from(c: SMSG_BATTLEFIELD_STATUS) -> Self {
        Self::SMSG_BATTLEFIELD_STATUS(c)
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

impl From<MSG_PVP_LOG_DATA_Server> for ServerOpcodeMessage {
    fn from(c: MSG_PVP_LOG_DATA_Server) -> Self {
        Self::MSG_PVP_LOG_DATA(c)
    }
}

impl From<SMSG_AREA_SPIRIT_HEALER_TIME> for ServerOpcodeMessage {
    fn from(c: SMSG_AREA_SPIRIT_HEALER_TIME) -> Self {
        Self::SMSG_AREA_SPIRIT_HEALER_TIME(c)
    }
}

impl From<SMSG_WARDEN_DATA> for ServerOpcodeMessage {
    fn from(c: SMSG_WARDEN_DATA) -> Self {
        Self::SMSG_WARDEN_DATA(c)
    }
}

impl From<SMSG_GROUP_JOINED_BATTLEGROUND> for ServerOpcodeMessage {
    fn from(c: SMSG_GROUP_JOINED_BATTLEGROUND) -> Self {
        Self::SMSG_GROUP_JOINED_BATTLEGROUND(c)
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

impl From<SMSG_PARTY_MEMBER_STATS_FULL> for ServerOpcodeMessage {
    fn from(c: SMSG_PARTY_MEMBER_STATS_FULL) -> Self {
        Self::SMSG_PARTY_MEMBER_STATS_FULL(c)
    }
}

impl From<SMSG_WEATHER> for ServerOpcodeMessage {
    fn from(c: SMSG_WEATHER) -> Self {
        Self::SMSG_WEATHER(c)
    }
}

impl From<SMSG_RAID_INSTANCE_MESSAGE> for ServerOpcodeMessage {
    fn from(c: SMSG_RAID_INSTANCE_MESSAGE) -> Self {
        Self::SMSG_RAID_INSTANCE_MESSAGE(c)
    }
}

impl From<SMSG_COMPRESSED_MOVES> for ServerOpcodeMessage {
    fn from(c: SMSG_COMPRESSED_MOVES) -> Self {
        Self::SMSG_COMPRESSED_MOVES(c)
    }
}

impl From<SMSG_CHAT_RESTRICTED> for ServerOpcodeMessage {
    fn from(_: SMSG_CHAT_RESTRICTED) -> Self {
        Self::SMSG_CHAT_RESTRICTED
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

impl From<MSG_MOVE_TIME_SKIPPED_Server> for ServerOpcodeMessage {
    fn from(c: MSG_MOVE_TIME_SKIPPED_Server) -> Self {
        Self::MSG_MOVE_TIME_SKIPPED(c)
    }
}

impl From<SMSG_SPLINE_MOVE_ROOT> for ServerOpcodeMessage {
    fn from(c: SMSG_SPLINE_MOVE_ROOT) -> Self {
        Self::SMSG_SPLINE_MOVE_ROOT(c)
    }
}

impl From<SMSG_INVALIDATE_PLAYER> for ServerOpcodeMessage {
    fn from(c: SMSG_INVALIDATE_PLAYER) -> Self {
        Self::SMSG_INVALIDATE_PLAYER(c)
    }
}

impl From<SMSG_INSTANCE_RESET> for ServerOpcodeMessage {
    fn from(c: SMSG_INSTANCE_RESET) -> Self {
        Self::SMSG_INSTANCE_RESET(c)
    }
}

impl From<SMSG_INSTANCE_RESET_FAILED> for ServerOpcodeMessage {
    fn from(c: SMSG_INSTANCE_RESET_FAILED) -> Self {
        Self::SMSG_INSTANCE_RESET_FAILED(c)
    }
}

impl From<SMSG_UPDATE_LAST_INSTANCE> for ServerOpcodeMessage {
    fn from(c: SMSG_UPDATE_LAST_INSTANCE) -> Self {
        Self::SMSG_UPDATE_LAST_INSTANCE(c)
    }
}

impl From<MSG_RAID_TARGET_UPDATE_Server> for ServerOpcodeMessage {
    fn from(c: MSG_RAID_TARGET_UPDATE_Server) -> Self {
        Self::MSG_RAID_TARGET_UPDATE(c)
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

impl From<SMSG_UPDATE_INSTANCE_OWNERSHIP> for ServerOpcodeMessage {
    fn from(c: SMSG_UPDATE_INSTANCE_OWNERSHIP) -> Self {
        Self::SMSG_UPDATE_INSTANCE_OWNERSHIP(c)
    }
}

impl From<SMSG_SPELLINSTAKILLLOG> for ServerOpcodeMessage {
    fn from(c: SMSG_SPELLINSTAKILLLOG) -> Self {
        Self::SMSG_SPELLINSTAKILLLOG(c)
    }
}

impl From<SMSG_SPELL_UPDATE_CHAIN_TARGETS> for ServerOpcodeMessage {
    fn from(c: SMSG_SPELL_UPDATE_CHAIN_TARGETS) -> Self {
        Self::SMSG_SPELL_UPDATE_CHAIN_TARGETS(c)
    }
}

impl From<SMSG_EXPECTED_SPAM_RECORDS> for ServerOpcodeMessage {
    fn from(c: SMSG_EXPECTED_SPAM_RECORDS) -> Self {
        Self::SMSG_EXPECTED_SPAM_RECORDS(c)
    }
}

impl From<SMSG_DEFENSE_MESSAGE> for ServerOpcodeMessage {
    fn from(c: SMSG_DEFENSE_MESSAGE) -> Self {
        Self::SMSG_DEFENSE_MESSAGE(c)
    }
}

