use crate::{ServerMessage, ClientMessage};
use wow_srp::header_crypto::{Decrypter, Encrypter};

#[cfg(feature = "tokio")]
use tokio::io::{AsyncReadExt, AsyncWriteExt};
#[cfg(feature = "async-std")]
use async_std::io::{ReadExt, WriteExt};
use crate::world::version_1_12::MSG_MOVE_START_FORWARD;
use crate::world::version_1_12::MSG_MOVE_START_BACKWARD;
use crate::world::version_1_12::MSG_MOVE_STOP;
use crate::world::version_1_12::MSG_MOVE_START_STRAFE_LEFT;
use crate::world::version_1_12::MSG_MOVE_START_STRAFE_RIGHT;
use crate::world::version_1_12::MSG_MOVE_STOP_STRAFE;
use crate::world::version_1_12::MSG_MOVE_JUMP;
use crate::world::version_1_12::MSG_MOVE_START_TURN_LEFT;
use crate::world::version_1_12::MSG_MOVE_START_TURN_RIGHT;
use crate::world::version_1_12::MSG_MOVE_STOP_TURN;
use crate::world::version_1_12::MSG_MOVE_START_PITCH_UP;
use crate::world::version_1_12::MSG_MOVE_START_PITCH_DOWN;
use crate::world::version_1_12::MSG_MOVE_STOP_PITCH;
use crate::world::version_1_12::MSG_MOVE_SET_RUN_MODE;
use crate::world::version_1_12::MSG_MOVE_SET_WALK_MODE;
use crate::world::version_1_12::MSG_MOVE_TELEPORT_ACK;
use crate::world::version_1_12::MSG_MOVE_FALL_LAND;
use crate::world::version_1_12::MSG_MOVE_START_SWIM;
use crate::world::version_1_12::MSG_MOVE_STOP_SWIM;
use crate::world::version_1_12::MSG_MOVE_SET_FACING;
use crate::world::version_1_12::MSG_MOVE_SET_PITCH;
use crate::world::version_1_12::MSG_MOVE_WORLDPORT_ACK;
use crate::world::version_1_12::MSG_MOVE_HEARTBEAT;
use crate::world::version_1_12::MSG_PETITION_DECLINE;
use crate::world::version_1_12::MSG_TABARDVENDOR_ACTIVATE;
use crate::world::version_1_12::MSG_QUEST_PUSH_RESULT;
use crate::world::version_1_12::MSG_PETITION_RENAME;
use crate::world::version_1_12::CMSG_WORLD_TELEPORT;
use crate::world::version_1_12::CMSG_CHAR_CREATE;
use crate::world::version_1_2::CMSG_CHAR_ENUM;
use crate::world::version_1_12::CMSG_CHAR_DELETE;
use crate::world::version_1_12::CMSG_PLAYER_LOGIN;
use crate::world::version_1_12::CMSG_PLAYER_LOGOUT;
use crate::world::version_1_12::CMSG_LOGOUT_REQUEST;
use crate::world::version_1_12::CMSG_LOGOUT_CANCEL;
use crate::world::version_1_12::CMSG_NAME_QUERY;
use crate::world::version_1_12::CMSG_PET_NAME_QUERY;
use crate::world::version_1_12::CMSG_GUILD_QUERY;
use crate::world::version_1_12::CMSG_ITEM_QUERY_SINGLE;
use crate::world::version_1_12::CMSG_PAGE_TEXT_QUERY;
use crate::world::version_1_12::CMSG_QUEST_QUERY;
use crate::world::version_1_12::CMSG_GAMEOBJECT_QUERY;
use crate::world::version_1_12::CMSG_CREATURE_QUERY;
use crate::world::version_1_12::CMSG_WHO;
use crate::world::version_1_12::CMSG_WHOIS;
use crate::world::version_1_12::CMSG_FRIEND_LIST;
use crate::world::version_1_12::CMSG_ADD_FRIEND;
use crate::world::version_1_12::CMSG_DEL_FRIEND;
use crate::world::version_1_12::CMSG_ADD_IGNORE;
use crate::world::version_1_12::CMSG_DEL_IGNORE;
use crate::world::version_1_12::CMSG_GROUP_INVITE;
use crate::world::version_1_12::CMSG_GROUP_ACCEPT;
use crate::world::version_1_12::CMSG_GROUP_DECLINE;
use crate::world::version_1_12::CMSG_GROUP_UNINVITE;
use crate::world::version_1_12::CMSG_GROUP_UNINVITE_GUID;
use crate::world::version_1_12::CMSG_GROUP_SET_LEADER;
use crate::world::version_1_12::CMSG_LOOT_METHOD;
use crate::world::version_1_12::CMSG_GROUP_DISBAND;
use crate::world::version_1_12::CMSG_GUILD_CREATE;
use crate::world::version_1_12::CMSG_GUILD_INVITE;
use crate::world::version_1_12::CMSG_GUILD_ACCEPT;
use crate::world::version_1_12::CMSG_GUILD_DECLINE;
use crate::world::version_1_12::CMSG_GUILD_INFO;
use crate::world::version_1_12::CMSG_GUILD_ROSTER;
use crate::world::version_1_12::CMSG_GUILD_PROMOTE;
use crate::world::version_1_12::CMSG_GUILD_DEMOTE;
use crate::world::version_1_12::CMSG_GUILD_LEAVE;
use crate::world::version_1_12::CMSG_GUILD_REMOVE;
use crate::world::version_1_12::CMSG_GUILD_DISBAND;
use crate::world::version_1_12::CMSG_GUILD_LEADER;
use crate::world::version_1_12::CMSG_GUILD_MOTD;
use crate::world::version_1_12::CMSG_MESSAGECHAT;
use crate::world::version_1_12::CMSG_JOIN_CHANNEL;
use crate::world::version_1_12::CMSG_LEAVE_CHANNEL;
use crate::world::version_1_12::CMSG_CHANNEL_LIST;
use crate::world::version_1_12::CMSG_CHANNEL_PASSWORD;
use crate::world::version_1_12::CMSG_CHANNEL_SET_OWNER;
use crate::world::version_1_12::CMSG_CHANNEL_OWNER;
use crate::world::version_1_12::CMSG_CHANNEL_MODERATOR;
use crate::world::version_1_12::CMSG_CHANNEL_UNMODERATOR;
use crate::world::version_1_12::CMSG_CHANNEL_MUTE;
use crate::world::version_1_12::CMSG_CHANNEL_UNMUTE;
use crate::world::version_1_12::CMSG_CHANNEL_INVITE;
use crate::world::version_1_12::CMSG_CHANNEL_KICK;
use crate::world::version_1_12::CMSG_CHANNEL_BAN;
use crate::world::version_1_12::CMSG_CHANNEL_UNBAN;
use crate::world::version_1_12::CMSG_CHANNEL_ANNOUNCEMENTS;
use crate::world::version_1_12::CMSG_CHANNEL_MODERATE;
use crate::world::version_1_12::CMSG_USE_ITEM;
use crate::world::version_1_12::CMSG_OPEN_ITEM;
use crate::world::version_1_12::CMSG_READ_ITEM;
use crate::world::version_1_12::CMSG_GAMEOBJ_USE;
use crate::world::version_1_12::CMSG_AREATRIGGER;
use crate::world::version_1_12::CMSG_MOVE_SET_RAW_POSITION;
use crate::world::version_1_12::CMSG_FORCE_RUN_SPEED_CHANGE_ACK;
use crate::world::version_1_12::CMSG_FORCE_RUN_BACK_SPEED_CHANGE_ACK;
use crate::world::version_1_12::CMSG_FORCE_SWIM_SPEED_CHANGE_ACK;
use crate::world::version_1_12::CMSG_FORCE_MOVE_ROOT_ACK;
use crate::world::version_1_12::CMSG_FORCE_MOVE_UNROOT_ACK;
use crate::world::version_1_12::CMSG_MOVE_KNOCK_BACK_ACK;
use crate::world::version_1_12::CMSG_MOVE_HOVER_ACK;
use crate::world::version_1_12::CMSG_NEXT_CINEMATIC_CAMERA;
use crate::world::version_1_12::CMSG_COMPLETE_CINEMATIC;
use crate::world::version_1_12::CMSG_TUTORIAL_FLAG;
use crate::world::version_1_12::CMSG_TUTORIAL_CLEAR;
use crate::world::version_1_12::CMSG_TUTORIAL_RESET;
use crate::world::version_1_12::CMSG_STANDSTATECHANGE;
use crate::world::version_1_12::CMSG_EMOTE;
use crate::world::version_1_12::CMSG_TEXT_EMOTE;
use crate::world::version_1_12::CMSG_AUTOSTORE_LOOT_ITEM;
use crate::world::version_1_12::CMSG_AUTOEQUIP_ITEM;
use crate::world::version_1_12::CMSG_AUTOSTORE_BAG_ITEM;
use crate::world::version_1_12::CMSG_SWAP_ITEM;
use crate::world::version_1_12::CMSG_SWAP_INV_ITEM;
use crate::world::version_1_12::CMSG_SPLIT_ITEM;
use crate::world::version_1_12::CMSG_AUTOEQUIP_ITEM_SLOT;
use crate::world::version_1_12::CMSG_DESTROYITEM;
use crate::world::version_1_12::CMSG_INSPECT;
use crate::world::version_1_12::CMSG_INITIATE_TRADE;
use crate::world::version_1_12::CMSG_BEGIN_TRADE;
use crate::world::version_1_12::CMSG_BUSY_TRADE;
use crate::world::version_1_12::CMSG_IGNORE_TRADE;
use crate::world::version_1_12::CMSG_ACCEPT_TRADE;
use crate::world::version_1_12::CMSG_UNACCEPT_TRADE;
use crate::world::version_1_12::CMSG_CANCEL_TRADE;
use crate::world::version_1_12::CMSG_SET_TRADE_ITEM;
use crate::world::version_1_12::CMSG_CLEAR_TRADE_ITEM;
use crate::world::version_1_12::CMSG_SET_TRADE_GOLD;
use crate::world::version_1_12::CMSG_SET_FACTION_ATWAR;
use crate::world::version_1_12::CMSG_SET_ACTION_BUTTON;
use crate::world::version_1_12::CMSG_CAST_SPELL;
use crate::world::version_1_12::CMSG_CANCEL_CAST;
use crate::world::version_1_12::CMSG_CANCEL_AURA;
use crate::world::version_1_12::CMSG_CANCEL_CHANNELLING;
use crate::world::version_1_12::CMSG_SET_SELECTION;
use crate::world::version_1_12::CMSG_SET_TARGET_OBSOLETE;
use crate::world::version_1_12::CMSG_ATTACKSWING;
use crate::world::version_1_12::CMSG_ATTACKSTOP;
use crate::world::version_1_12::CMSG_REPOP_REQUEST;
use crate::world::version_1_12::CMSG_RESURRECT_RESPONSE;
use crate::world::version_1_12::CMSG_LOOT;
use crate::world::version_1_12::CMSG_LOOT_MONEY;
use crate::world::version_1_12::CMSG_LOOT_RELEASE;
use crate::world::version_1_12::CMSG_DUEL_ACCEPTED;
use crate::world::version_1_12::CMSG_DUEL_CANCELLED;
use crate::world::version_1_12::CMSG_MOUNTSPECIAL_ANIM;
use crate::world::version_1_12::CMSG_PET_SET_ACTION;
use crate::world::version_1_12::CMSG_PET_ACTION;
use crate::world::version_1_12::CMSG_PET_ABANDON;
use crate::world::version_1_12::CMSG_PET_RENAME;
use crate::world::version_1_12::CMSG_GOSSIP_HELLO;
use crate::world::version_1_12::CMSG_GOSSIP_SELECT_OPTION;
use crate::world::version_1_12::CMSG_NPC_TEXT_QUERY;
use crate::world::version_1_12::CMSG_QUESTGIVER_STATUS_QUERY;
use crate::world::version_1_12::CMSG_QUESTGIVER_HELLO;
use crate::world::version_1_12::CMSG_QUESTGIVER_QUERY_QUEST;
use crate::world::version_1_12::CMSG_QUESTGIVER_QUEST_AUTOLAUNCH;
use crate::world::version_1_12::CMSG_QUESTGIVER_ACCEPT_QUEST;
use crate::world::version_1_12::CMSG_QUESTGIVER_COMPLETE_QUEST;
use crate::world::version_1_12::CMSG_QUESTGIVER_REQUEST_REWARD;
use crate::world::version_1_12::CMSG_QUESTGIVER_CHOOSE_REWARD;
use crate::world::version_1_12::CMSG_QUESTGIVER_CANCEL;
use crate::world::version_1_12::CMSG_QUESTLOG_SWAP_QUEST;
use crate::world::version_1_12::CMSG_QUESTLOG_REMOVE_QUEST;
use crate::world::version_1_12::CMSG_QUEST_CONFIRM_ACCEPT;
use crate::world::version_1_12::CMSG_PUSHQUESTTOPARTY;
use crate::world::version_1_12::CMSG_LIST_INVENTORY;
use crate::world::version_1_12::CMSG_SELL_ITEM;
use crate::world::version_1_12::CMSG_BUY_ITEM;
use crate::world::version_1_12::CMSG_BUY_ITEM_IN_SLOT;
use crate::world::version_1_12::CMSG_TAXINODE_STATUS_QUERY;
use crate::world::version_1_12::CMSG_TAXIQUERYAVAILABLENODES;
use crate::world::version_1_12::CMSG_ACTIVATETAXI;
use crate::world::version_1_12::CMSG_TRAINER_LIST;
use crate::world::version_1_12::CMSG_TRAINER_BUY_SPELL;
use crate::world::version_1_12::CMSG_BINDER_ACTIVATE;
use crate::world::version_1_12::CMSG_BANKER_ACTIVATE;
use crate::world::version_1_12::CMSG_BUY_BANK_SLOT;
use crate::world::version_1_12::CMSG_PETITION_SHOWLIST;
use crate::world::version_1_12::CMSG_PETITION_BUY;
use crate::world::version_1_12::CMSG_PETITION_SHOW_SIGNATURES;
use crate::world::version_1_12::CMSG_PETITION_SIGN;
use crate::world::version_1_12::CMSG_OFFER_PETITION;
use crate::world::version_1_12::CMSG_TURN_IN_PETITION;
use crate::world::version_1_12::CMSG_PETITION_QUERY;
use crate::world::version_1_12::CMSG_BUG;
use crate::world::version_1_12::CMSG_PLAYED_TIME;
use crate::world::version_1_12::CMSG_QUERY_TIME;
use crate::world::version_1_12::CMSG_RECLAIM_CORPSE;
use crate::world::version_1_12::CMSG_WRAP_ITEM;
use crate::world::version_1_12::MSG_MINIMAP_PING_Client;
use crate::world::version_1_12::CMSG_PING;
use crate::world::version_1_12::CMSG_SETSHEATHED;
use crate::world::version_1_12::CMSG_AUTH_SESSION;
use crate::world::version_1_12::CMSG_PET_CAST_SPELL;
use crate::world::version_1_12::MSG_SAVE_GUILD_EMBLEM_Client;
use crate::world::version_1_12::CMSG_ZONEUPDATE;
use crate::world::version_1_12::MSG_RANDOM_ROLL_Client;
use crate::world::version_1_12::MSG_LOOKING_FOR_GROUP_Client;
use crate::world::version_1_12::CMSG_UNLEARN_SKILL;
use crate::world::version_1_12::CMSG_GMTICKET_CREATE;
use crate::world::version_1_12::CMSG_GMTICKET_UPDATETEXT;
use crate::world::version_1_12::CMSG_REQUEST_ACCOUNT_DATA;
use crate::world::version_1_12::CMSG_GMTICKET_GETTICKET;
use crate::world::version_1_12::MSG_CORPSE_QUERY_Client;
use crate::world::version_1_12::CMSG_GMTICKET_DELETETICKET;
use crate::world::version_1_12::CMSG_GMTICKET_SYSTEMSTATUS;
use crate::world::version_1_12::CMSG_SPIRIT_HEALER_ACTIVATE;
use crate::world::version_1_12::CMSG_CHAT_IGNORED;
use crate::world::version_1_12::CMSG_GUILD_RANK;
use crate::world::version_1_12::CMSG_GUILD_ADD_RANK;
use crate::world::version_1_12::CMSG_GUILD_DEL_RANK;
use crate::world::version_1_12::CMSG_GUILD_SET_PUBLIC_NOTE;
use crate::world::version_1_12::CMSG_GUILD_SET_OFFICER_NOTE;
use crate::world::version_1_12::CMSG_SEND_MAIL;
use crate::world::version_1_12::CMSG_GET_MAIL_LIST;
use crate::world::version_1_12::CMSG_BATTLEFIELD_LIST;
use crate::world::version_1_12::CMSG_BATTLEFIELD_JOIN;
use crate::world::version_1_12::CMSG_ITEM_TEXT_QUERY;
use crate::world::version_1_12::CMSG_MAIL_TAKE_MONEY;
use crate::world::version_1_12::CMSG_MAIL_TAKE_ITEM;
use crate::world::version_1_12::CMSG_MAIL_MARK_AS_READ;
use crate::world::version_1_12::CMSG_MAIL_RETURN_TO_SENDER;
use crate::world::version_1_12::CMSG_MAIL_DELETE;
use crate::world::version_1_12::CMSG_MAIL_CREATE_TEXT_ITEM;
use crate::world::version_1_12::CMSG_LEARN_TALENT;
use crate::world::version_1_12::CMSG_TOGGLE_PVP;
use crate::world::version_1_12::MSG_AUCTION_HELLO_Client;
use crate::world::version_1_12::CMSG_AUCTION_SELL_ITEM;
use crate::world::version_1_12::CMSG_AUCTION_REMOVE_ITEM;
use crate::world::version_1_12::CMSG_AUCTION_LIST_ITEMS;
use crate::world::version_1_12::CMSG_AUCTION_LIST_OWNER_ITEMS;
use crate::world::version_1_12::CMSG_AUCTION_PLACE_BID;
use crate::world::version_1_12::CMSG_AUCTION_LIST_BIDDER_ITEMS;
use crate::world::version_1_12::CMSG_SET_AMMO;
use crate::world::version_1_12::CMSG_SET_ACTIVE_MOVER;
use crate::world::version_1_12::CMSG_PET_CANCEL_AURA;
use crate::world::version_1_12::CMSG_CANCEL_AUTO_REPEAT_SPELL;
use crate::world::version_1_12::MSG_LIST_STABLED_PETS_Client;
use crate::world::version_1_12::CMSG_STABLE_PET;
use crate::world::version_1_12::CMSG_UNSTABLE_PET;
use crate::world::version_1_12::CMSG_BUY_STABLE_SLOT;
use crate::world::version_1_12::CMSG_STABLE_SWAP_PET;
use crate::world::version_1_12::CMSG_REQUEST_PET_INFO;
use crate::world::version_1_12::CMSG_FAR_SIGHT;
use crate::world::version_1_12::CMSG_GROUP_CHANGE_SUB_GROUP;
use crate::world::version_1_12::CMSG_REQUEST_PARTY_MEMBER_STATS;
use crate::world::version_1_12::CMSG_GROUP_SWAP_SUB_GROUP;
use crate::world::version_1_12::CMSG_AUTOSTORE_BANK_ITEM;
use crate::world::version_1_12::CMSG_AUTOBANK_ITEM;
use crate::world::version_1_12::MSG_QUERY_NEXT_MAIL_TIME_Client;
use crate::world::version_1_12::CMSG_GROUP_RAID_CONVERT;
use crate::world::version_1_12::CMSG_GROUP_ASSISTANT_LEADER;
use crate::world::version_1_12::CMSG_BUYBACK_ITEM;
use crate::world::version_1_12::CMSG_MEETINGSTONE_JOIN;
use crate::world::version_1_12::CMSG_MEETINGSTONE_LEAVE;
use crate::world::version_1_12::CMSG_MEETINGSTONE_INFO;
use crate::world::version_1_12::CMSG_CANCEL_GROWTH_AURA;
use crate::world::version_1_12::CMSG_LOOT_ROLL;
use crate::world::version_1_12::CMSG_LOOT_MASTER_GIVE;
use crate::world::version_1_12::CMSG_REPAIR_ITEM;
use crate::world::version_1_12::MSG_TALENT_WIPE_CONFIRM_Client;
use crate::world::version_1_12::CMSG_SUMMON_RESPONSE;
use crate::world::version_1_12::CMSG_SELF_RES;
use crate::world::version_1_12::CMSG_TOGGLE_HELM;
use crate::world::version_1_12::CMSG_TOGGLE_CLOAK;
use crate::world::version_1_12::CMSG_SET_ACTIONBAR_TOGGLES;
use crate::world::version_1_12::CMSG_ITEM_NAME_QUERY;
use crate::world::version_1_12::CMSG_CHAR_RENAME;
use crate::world::version_1_12::CMSG_MOVE_SPLINE_DONE;
use crate::world::version_1_12::CMSG_MOVE_FALL_RESET;
use crate::world::version_1_12::CMSG_REQUEST_RAID_INFO;
use crate::world::version_1_12::CMSG_MOVE_TIME_SKIPPED;
use crate::world::version_1_12::CMSG_MOVE_FEATHER_FALL_ACK;
use crate::world::version_1_12::CMSG_MOVE_WATER_WALK_ACK;
use crate::world::version_1_12::CMSG_MOVE_NOT_ACTIVE_MOVER;
use crate::world::version_1_12::CMSG_BATTLEFIELD_STATUS;
use crate::world::version_1_12::CMSG_BATTLEFIELD_PORT;
use crate::world::version_1_12::MSG_INSPECT_HONOR_STATS_Client;
use crate::world::version_1_12::CMSG_BATTLEMASTER_HELLO;
use crate::world::version_1_12::CMSG_FORCE_WALK_SPEED_CHANGE_ACK;
use crate::world::version_1_12::CMSG_FORCE_SWIM_BACK_SPEED_CHANGE_ACK;
use crate::world::version_1_12::CMSG_FORCE_TURN_RATE_CHANGE_ACK;
use crate::world::version_1_12::MSG_PVP_LOG_DATA_Client;
use crate::world::version_1_12::CMSG_LEAVE_BATTLEFIELD;
use crate::world::version_1_12::CMSG_AREA_SPIRIT_HEALER_QUERY;
use crate::world::version_1_12::CMSG_AREA_SPIRIT_HEALER_QUEUE;
use crate::world::version_1_12::MSG_BATTLEGROUND_PLAYER_POSITIONS_Client;
use crate::world::version_1_12::CMSG_PET_STOP_ATTACK;
use crate::world::version_1_12::CMSG_BATTLEMASTER_JOIN;
use crate::world::version_1_12::CMSG_PET_UNLEARN;
use crate::world::version_1_12::CMSG_PET_SPELL_AUTOCAST;
use crate::world::version_1_12::CMSG_GUILD_INFO_TEXT;
use crate::world::version_1_12::CMSG_ACTIVATETAXIEXPRESS;
use crate::world::version_1_12::CMSG_SET_FACTION_INACTIVE;
use crate::world::version_1_12::CMSG_SET_WATCHED_FACTION;
use crate::world::version_1_12::CMSG_RESET_INSTANCES;
use crate::world::version_1_12::MSG_RAID_TARGET_UPDATE_Client;
use crate::world::version_1_12::MSG_RAID_READY_CHECK_Client;
use crate::world::version_1_12::CMSG_GMSURVEY_SUBMIT;

#[derive(Debug)]
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
    MSG_MOVE_TELEPORT_ACK(MSG_MOVE_TELEPORT_ACK),
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
    CMSG_WORLD_TELEPORT(CMSG_WORLD_TELEPORT),
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
    CMSG_FRIEND_LIST(CMSG_FRIEND_LIST),
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
    CMSG_SET_TARGET_OBSOLETE(CMSG_SET_TARGET_OBSOLETE),
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
    MSG_LOOKING_FOR_GROUP(MSG_LOOKING_FOR_GROUP_Client),
    CMSG_UNLEARN_SKILL(CMSG_UNLEARN_SKILL),
    CMSG_GMTICKET_CREATE(CMSG_GMTICKET_CREATE),
    CMSG_GMTICKET_UPDATETEXT(CMSG_GMTICKET_UPDATETEXT),
    CMSG_REQUEST_ACCOUNT_DATA(CMSG_REQUEST_ACCOUNT_DATA),
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
    CMSG_MEETINGSTONE_JOIN(CMSG_MEETINGSTONE_JOIN),
    CMSG_MEETINGSTONE_LEAVE(CMSG_MEETINGSTONE_LEAVE),
    CMSG_MEETINGSTONE_INFO(CMSG_MEETINGSTONE_INFO),
    CMSG_CANCEL_GROWTH_AURA(CMSG_CANCEL_GROWTH_AURA),
    CMSG_LOOT_ROLL(CMSG_LOOT_ROLL),
    CMSG_LOOT_MASTER_GIVE(CMSG_LOOT_MASTER_GIVE),
    CMSG_REPAIR_ITEM(CMSG_REPAIR_ITEM),
    MSG_TALENT_WIPE_CONFIRM(MSG_TALENT_WIPE_CONFIRM_Client),
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
    MSG_RAID_TARGET_UPDATE(MSG_RAID_TARGET_UPDATE_Client),
    MSG_RAID_READY_CHECK(MSG_RAID_READY_CHECK_Client),
    CMSG_GMSURVEY_SUBMIT(CMSG_GMSURVEY_SUBMIT),
}

impl ClientOpcodeMessage {
    fn read_opcodes(opcode: u32, body_size: u32, mut r: &[u8]) -> std::result::Result<Self, crate::errors::ExpectedOpcodeError> {
        match opcode {
            0x00B5 => Ok(Self::MSG_MOVE_START_FORWARD(<MSG_MOVE_START_FORWARD as ClientMessage>::read_body(&mut r, body_size)?)),
            0x00B6 => Ok(Self::MSG_MOVE_START_BACKWARD(<MSG_MOVE_START_BACKWARD as ClientMessage>::read_body(&mut r, body_size)?)),
            0x00B7 => Ok(Self::MSG_MOVE_STOP(<MSG_MOVE_STOP as ClientMessage>::read_body(&mut r, body_size)?)),
            0x00B8 => Ok(Self::MSG_MOVE_START_STRAFE_LEFT(<MSG_MOVE_START_STRAFE_LEFT as ClientMessage>::read_body(&mut r, body_size)?)),
            0x00B9 => Ok(Self::MSG_MOVE_START_STRAFE_RIGHT(<MSG_MOVE_START_STRAFE_RIGHT as ClientMessage>::read_body(&mut r, body_size)?)),
            0x00BA => Ok(Self::MSG_MOVE_STOP_STRAFE(<MSG_MOVE_STOP_STRAFE as ClientMessage>::read_body(&mut r, body_size)?)),
            0x00BB => Ok(Self::MSG_MOVE_JUMP(<MSG_MOVE_JUMP as ClientMessage>::read_body(&mut r, body_size)?)),
            0x00BC => Ok(Self::MSG_MOVE_START_TURN_LEFT(<MSG_MOVE_START_TURN_LEFT as ClientMessage>::read_body(&mut r, body_size)?)),
            0x00BD => Ok(Self::MSG_MOVE_START_TURN_RIGHT(<MSG_MOVE_START_TURN_RIGHT as ClientMessage>::read_body(&mut r, body_size)?)),
            0x00BE => Ok(Self::MSG_MOVE_STOP_TURN(<MSG_MOVE_STOP_TURN as ClientMessage>::read_body(&mut r, body_size)?)),
            0x00BF => Ok(Self::MSG_MOVE_START_PITCH_UP(<MSG_MOVE_START_PITCH_UP as ClientMessage>::read_body(&mut r, body_size)?)),
            0x00C0 => Ok(Self::MSG_MOVE_START_PITCH_DOWN(<MSG_MOVE_START_PITCH_DOWN as ClientMessage>::read_body(&mut r, body_size)?)),
            0x00C1 => Ok(Self::MSG_MOVE_STOP_PITCH(<MSG_MOVE_STOP_PITCH as ClientMessage>::read_body(&mut r, body_size)?)),
            0x00C2 => Ok(Self::MSG_MOVE_SET_RUN_MODE(<MSG_MOVE_SET_RUN_MODE as ClientMessage>::read_body(&mut r, body_size)?)),
            0x00C3 => Ok(Self::MSG_MOVE_SET_WALK_MODE(<MSG_MOVE_SET_WALK_MODE as ClientMessage>::read_body(&mut r, body_size)?)),
            0x00C7 => Ok(Self::MSG_MOVE_TELEPORT_ACK(<MSG_MOVE_TELEPORT_ACK as ClientMessage>::read_body(&mut r, body_size)?)),
            0x00C9 => Ok(Self::MSG_MOVE_FALL_LAND(<MSG_MOVE_FALL_LAND as ClientMessage>::read_body(&mut r, body_size)?)),
            0x00CA => Ok(Self::MSG_MOVE_START_SWIM(<MSG_MOVE_START_SWIM as ClientMessage>::read_body(&mut r, body_size)?)),
            0x00CB => Ok(Self::MSG_MOVE_STOP_SWIM(<MSG_MOVE_STOP_SWIM as ClientMessage>::read_body(&mut r, body_size)?)),
            0x00DA => Ok(Self::MSG_MOVE_SET_FACING(<MSG_MOVE_SET_FACING as ClientMessage>::read_body(&mut r, body_size)?)),
            0x00DB => Ok(Self::MSG_MOVE_SET_PITCH(<MSG_MOVE_SET_PITCH as ClientMessage>::read_body(&mut r, body_size)?)),
            0x00DC => Ok(Self::MSG_MOVE_WORLDPORT_ACK(<MSG_MOVE_WORLDPORT_ACK as ClientMessage>::read_body(&mut r, body_size)?)),
            0x00EE => Ok(Self::MSG_MOVE_HEARTBEAT(<MSG_MOVE_HEARTBEAT as ClientMessage>::read_body(&mut r, body_size)?)),
            0x01C2 => Ok(Self::MSG_PETITION_DECLINE(<MSG_PETITION_DECLINE as ClientMessage>::read_body(&mut r, body_size)?)),
            0x01F2 => Ok(Self::MSG_TABARDVENDOR_ACTIVATE(<MSG_TABARDVENDOR_ACTIVATE as ClientMessage>::read_body(&mut r, body_size)?)),
            0x0276 => Ok(Self::MSG_QUEST_PUSH_RESULT(<MSG_QUEST_PUSH_RESULT as ClientMessage>::read_body(&mut r, body_size)?)),
            0x02C1 => Ok(Self::MSG_PETITION_RENAME(<MSG_PETITION_RENAME as ClientMessage>::read_body(&mut r, body_size)?)),
            0x0008 => Ok(Self::CMSG_WORLD_TELEPORT(<CMSG_WORLD_TELEPORT as ClientMessage>::read_body(&mut r, body_size)?)),
            0x0036 => Ok(Self::CMSG_CHAR_CREATE(<CMSG_CHAR_CREATE as ClientMessage>::read_body(&mut r, body_size)?)),
            0x0037 => Ok(Self::CMSG_CHAR_ENUM(<CMSG_CHAR_ENUM as ClientMessage>::read_body(&mut r, body_size)?)),
            0x0038 => Ok(Self::CMSG_CHAR_DELETE(<CMSG_CHAR_DELETE as ClientMessage>::read_body(&mut r, body_size)?)),
            0x003D => Ok(Self::CMSG_PLAYER_LOGIN(<CMSG_PLAYER_LOGIN as ClientMessage>::read_body(&mut r, body_size)?)),
            0x004A => Ok(Self::CMSG_PLAYER_LOGOUT(<CMSG_PLAYER_LOGOUT as ClientMessage>::read_body(&mut r, body_size)?)),
            0x004B => Ok(Self::CMSG_LOGOUT_REQUEST(<CMSG_LOGOUT_REQUEST as ClientMessage>::read_body(&mut r, body_size)?)),
            0x004E => Ok(Self::CMSG_LOGOUT_CANCEL(<CMSG_LOGOUT_CANCEL as ClientMessage>::read_body(&mut r, body_size)?)),
            0x0050 => Ok(Self::CMSG_NAME_QUERY(<CMSG_NAME_QUERY as ClientMessage>::read_body(&mut r, body_size)?)),
            0x0052 => Ok(Self::CMSG_PET_NAME_QUERY(<CMSG_PET_NAME_QUERY as ClientMessage>::read_body(&mut r, body_size)?)),
            0x0054 => Ok(Self::CMSG_GUILD_QUERY(<CMSG_GUILD_QUERY as ClientMessage>::read_body(&mut r, body_size)?)),
            0x0056 => Ok(Self::CMSG_ITEM_QUERY_SINGLE(<CMSG_ITEM_QUERY_SINGLE as ClientMessage>::read_body(&mut r, body_size)?)),
            0x005A => Ok(Self::CMSG_PAGE_TEXT_QUERY(<CMSG_PAGE_TEXT_QUERY as ClientMessage>::read_body(&mut r, body_size)?)),
            0x005C => Ok(Self::CMSG_QUEST_QUERY(<CMSG_QUEST_QUERY as ClientMessage>::read_body(&mut r, body_size)?)),
            0x005E => Ok(Self::CMSG_GAMEOBJECT_QUERY(<CMSG_GAMEOBJECT_QUERY as ClientMessage>::read_body(&mut r, body_size)?)),
            0x0060 => Ok(Self::CMSG_CREATURE_QUERY(<CMSG_CREATURE_QUERY as ClientMessage>::read_body(&mut r, body_size)?)),
            0x0062 => Ok(Self::CMSG_WHO(<CMSG_WHO as ClientMessage>::read_body(&mut r, body_size)?)),
            0x0064 => Ok(Self::CMSG_WHOIS(<CMSG_WHOIS as ClientMessage>::read_body(&mut r, body_size)?)),
            0x0066 => Ok(Self::CMSG_FRIEND_LIST(<CMSG_FRIEND_LIST as ClientMessage>::read_body(&mut r, body_size)?)),
            0x0069 => Ok(Self::CMSG_ADD_FRIEND(<CMSG_ADD_FRIEND as ClientMessage>::read_body(&mut r, body_size)?)),
            0x006A => Ok(Self::CMSG_DEL_FRIEND(<CMSG_DEL_FRIEND as ClientMessage>::read_body(&mut r, body_size)?)),
            0x006C => Ok(Self::CMSG_ADD_IGNORE(<CMSG_ADD_IGNORE as ClientMessage>::read_body(&mut r, body_size)?)),
            0x006D => Ok(Self::CMSG_DEL_IGNORE(<CMSG_DEL_IGNORE as ClientMessage>::read_body(&mut r, body_size)?)),
            0x006E => Ok(Self::CMSG_GROUP_INVITE(<CMSG_GROUP_INVITE as ClientMessage>::read_body(&mut r, body_size)?)),
            0x0072 => Ok(Self::CMSG_GROUP_ACCEPT(<CMSG_GROUP_ACCEPT as ClientMessage>::read_body(&mut r, body_size)?)),
            0x0073 => Ok(Self::CMSG_GROUP_DECLINE(<CMSG_GROUP_DECLINE as ClientMessage>::read_body(&mut r, body_size)?)),
            0x0075 => Ok(Self::CMSG_GROUP_UNINVITE(<CMSG_GROUP_UNINVITE as ClientMessage>::read_body(&mut r, body_size)?)),
            0x0076 => Ok(Self::CMSG_GROUP_UNINVITE_GUID(<CMSG_GROUP_UNINVITE_GUID as ClientMessage>::read_body(&mut r, body_size)?)),
            0x0078 => Ok(Self::CMSG_GROUP_SET_LEADER(<CMSG_GROUP_SET_LEADER as ClientMessage>::read_body(&mut r, body_size)?)),
            0x007A => Ok(Self::CMSG_LOOT_METHOD(<CMSG_LOOT_METHOD as ClientMessage>::read_body(&mut r, body_size)?)),
            0x007B => Ok(Self::CMSG_GROUP_DISBAND(<CMSG_GROUP_DISBAND as ClientMessage>::read_body(&mut r, body_size)?)),
            0x0081 => Ok(Self::CMSG_GUILD_CREATE(<CMSG_GUILD_CREATE as ClientMessage>::read_body(&mut r, body_size)?)),
            0x0082 => Ok(Self::CMSG_GUILD_INVITE(<CMSG_GUILD_INVITE as ClientMessage>::read_body(&mut r, body_size)?)),
            0x0084 => Ok(Self::CMSG_GUILD_ACCEPT(<CMSG_GUILD_ACCEPT as ClientMessage>::read_body(&mut r, body_size)?)),
            0x0085 => Ok(Self::CMSG_GUILD_DECLINE(<CMSG_GUILD_DECLINE as ClientMessage>::read_body(&mut r, body_size)?)),
            0x0087 => Ok(Self::CMSG_GUILD_INFO(<CMSG_GUILD_INFO as ClientMessage>::read_body(&mut r, body_size)?)),
            0x0089 => Ok(Self::CMSG_GUILD_ROSTER(<CMSG_GUILD_ROSTER as ClientMessage>::read_body(&mut r, body_size)?)),
            0x008B => Ok(Self::CMSG_GUILD_PROMOTE(<CMSG_GUILD_PROMOTE as ClientMessage>::read_body(&mut r, body_size)?)),
            0x008C => Ok(Self::CMSG_GUILD_DEMOTE(<CMSG_GUILD_DEMOTE as ClientMessage>::read_body(&mut r, body_size)?)),
            0x008D => Ok(Self::CMSG_GUILD_LEAVE(<CMSG_GUILD_LEAVE as ClientMessage>::read_body(&mut r, body_size)?)),
            0x008E => Ok(Self::CMSG_GUILD_REMOVE(<CMSG_GUILD_REMOVE as ClientMessage>::read_body(&mut r, body_size)?)),
            0x008F => Ok(Self::CMSG_GUILD_DISBAND(<CMSG_GUILD_DISBAND as ClientMessage>::read_body(&mut r, body_size)?)),
            0x0090 => Ok(Self::CMSG_GUILD_LEADER(<CMSG_GUILD_LEADER as ClientMessage>::read_body(&mut r, body_size)?)),
            0x0091 => Ok(Self::CMSG_GUILD_MOTD(<CMSG_GUILD_MOTD as ClientMessage>::read_body(&mut r, body_size)?)),
            0x0095 => Ok(Self::CMSG_MESSAGECHAT(<CMSG_MESSAGECHAT as ClientMessage>::read_body(&mut r, body_size)?)),
            0x0097 => Ok(Self::CMSG_JOIN_CHANNEL(<CMSG_JOIN_CHANNEL as ClientMessage>::read_body(&mut r, body_size)?)),
            0x0098 => Ok(Self::CMSG_LEAVE_CHANNEL(<CMSG_LEAVE_CHANNEL as ClientMessage>::read_body(&mut r, body_size)?)),
            0x009A => Ok(Self::CMSG_CHANNEL_LIST(<CMSG_CHANNEL_LIST as ClientMessage>::read_body(&mut r, body_size)?)),
            0x009C => Ok(Self::CMSG_CHANNEL_PASSWORD(<CMSG_CHANNEL_PASSWORD as ClientMessage>::read_body(&mut r, body_size)?)),
            0x009D => Ok(Self::CMSG_CHANNEL_SET_OWNER(<CMSG_CHANNEL_SET_OWNER as ClientMessage>::read_body(&mut r, body_size)?)),
            0x009E => Ok(Self::CMSG_CHANNEL_OWNER(<CMSG_CHANNEL_OWNER as ClientMessage>::read_body(&mut r, body_size)?)),
            0x009F => Ok(Self::CMSG_CHANNEL_MODERATOR(<CMSG_CHANNEL_MODERATOR as ClientMessage>::read_body(&mut r, body_size)?)),
            0x00A0 => Ok(Self::CMSG_CHANNEL_UNMODERATOR(<CMSG_CHANNEL_UNMODERATOR as ClientMessage>::read_body(&mut r, body_size)?)),
            0x00A1 => Ok(Self::CMSG_CHANNEL_MUTE(<CMSG_CHANNEL_MUTE as ClientMessage>::read_body(&mut r, body_size)?)),
            0x00A2 => Ok(Self::CMSG_CHANNEL_UNMUTE(<CMSG_CHANNEL_UNMUTE as ClientMessage>::read_body(&mut r, body_size)?)),
            0x00A3 => Ok(Self::CMSG_CHANNEL_INVITE(<CMSG_CHANNEL_INVITE as ClientMessage>::read_body(&mut r, body_size)?)),
            0x00A4 => Ok(Self::CMSG_CHANNEL_KICK(<CMSG_CHANNEL_KICK as ClientMessage>::read_body(&mut r, body_size)?)),
            0x00A5 => Ok(Self::CMSG_CHANNEL_BAN(<CMSG_CHANNEL_BAN as ClientMessage>::read_body(&mut r, body_size)?)),
            0x00A6 => Ok(Self::CMSG_CHANNEL_UNBAN(<CMSG_CHANNEL_UNBAN as ClientMessage>::read_body(&mut r, body_size)?)),
            0x00A7 => Ok(Self::CMSG_CHANNEL_ANNOUNCEMENTS(<CMSG_CHANNEL_ANNOUNCEMENTS as ClientMessage>::read_body(&mut r, body_size)?)),
            0x00A8 => Ok(Self::CMSG_CHANNEL_MODERATE(<CMSG_CHANNEL_MODERATE as ClientMessage>::read_body(&mut r, body_size)?)),
            0x00AB => Ok(Self::CMSG_USE_ITEM(<CMSG_USE_ITEM as ClientMessage>::read_body(&mut r, body_size)?)),
            0x00AC => Ok(Self::CMSG_OPEN_ITEM(<CMSG_OPEN_ITEM as ClientMessage>::read_body(&mut r, body_size)?)),
            0x00AD => Ok(Self::CMSG_READ_ITEM(<CMSG_READ_ITEM as ClientMessage>::read_body(&mut r, body_size)?)),
            0x00B1 => Ok(Self::CMSG_GAMEOBJ_USE(<CMSG_GAMEOBJ_USE as ClientMessage>::read_body(&mut r, body_size)?)),
            0x00B4 => Ok(Self::CMSG_AREATRIGGER(<CMSG_AREATRIGGER as ClientMessage>::read_body(&mut r, body_size)?)),
            0x00E1 => Ok(Self::CMSG_MOVE_SET_RAW_POSITION(<CMSG_MOVE_SET_RAW_POSITION as ClientMessage>::read_body(&mut r, body_size)?)),
            0x00E3 => Ok(Self::CMSG_FORCE_RUN_SPEED_CHANGE_ACK(<CMSG_FORCE_RUN_SPEED_CHANGE_ACK as ClientMessage>::read_body(&mut r, body_size)?)),
            0x00E5 => Ok(Self::CMSG_FORCE_RUN_BACK_SPEED_CHANGE_ACK(<CMSG_FORCE_RUN_BACK_SPEED_CHANGE_ACK as ClientMessage>::read_body(&mut r, body_size)?)),
            0x00E7 => Ok(Self::CMSG_FORCE_SWIM_SPEED_CHANGE_ACK(<CMSG_FORCE_SWIM_SPEED_CHANGE_ACK as ClientMessage>::read_body(&mut r, body_size)?)),
            0x00E9 => Ok(Self::CMSG_FORCE_MOVE_ROOT_ACK(<CMSG_FORCE_MOVE_ROOT_ACK as ClientMessage>::read_body(&mut r, body_size)?)),
            0x00EB => Ok(Self::CMSG_FORCE_MOVE_UNROOT_ACK(<CMSG_FORCE_MOVE_UNROOT_ACK as ClientMessage>::read_body(&mut r, body_size)?)),
            0x00F0 => Ok(Self::CMSG_MOVE_KNOCK_BACK_ACK(<CMSG_MOVE_KNOCK_BACK_ACK as ClientMessage>::read_body(&mut r, body_size)?)),
            0x00F6 => Ok(Self::CMSG_MOVE_HOVER_ACK(<CMSG_MOVE_HOVER_ACK as ClientMessage>::read_body(&mut r, body_size)?)),
            0x00FB => Ok(Self::CMSG_NEXT_CINEMATIC_CAMERA(<CMSG_NEXT_CINEMATIC_CAMERA as ClientMessage>::read_body(&mut r, body_size)?)),
            0x00FC => Ok(Self::CMSG_COMPLETE_CINEMATIC(<CMSG_COMPLETE_CINEMATIC as ClientMessage>::read_body(&mut r, body_size)?)),
            0x00FE => Ok(Self::CMSG_TUTORIAL_FLAG(<CMSG_TUTORIAL_FLAG as ClientMessage>::read_body(&mut r, body_size)?)),
            0x00FF => Ok(Self::CMSG_TUTORIAL_CLEAR(<CMSG_TUTORIAL_CLEAR as ClientMessage>::read_body(&mut r, body_size)?)),
            0x0100 => Ok(Self::CMSG_TUTORIAL_RESET(<CMSG_TUTORIAL_RESET as ClientMessage>::read_body(&mut r, body_size)?)),
            0x0101 => Ok(Self::CMSG_STANDSTATECHANGE(<CMSG_STANDSTATECHANGE as ClientMessage>::read_body(&mut r, body_size)?)),
            0x0102 => Ok(Self::CMSG_EMOTE(<CMSG_EMOTE as ClientMessage>::read_body(&mut r, body_size)?)),
            0x0104 => Ok(Self::CMSG_TEXT_EMOTE(<CMSG_TEXT_EMOTE as ClientMessage>::read_body(&mut r, body_size)?)),
            0x0108 => Ok(Self::CMSG_AUTOSTORE_LOOT_ITEM(<CMSG_AUTOSTORE_LOOT_ITEM as ClientMessage>::read_body(&mut r, body_size)?)),
            0x010A => Ok(Self::CMSG_AUTOEQUIP_ITEM(<CMSG_AUTOEQUIP_ITEM as ClientMessage>::read_body(&mut r, body_size)?)),
            0x010B => Ok(Self::CMSG_AUTOSTORE_BAG_ITEM(<CMSG_AUTOSTORE_BAG_ITEM as ClientMessage>::read_body(&mut r, body_size)?)),
            0x010C => Ok(Self::CMSG_SWAP_ITEM(<CMSG_SWAP_ITEM as ClientMessage>::read_body(&mut r, body_size)?)),
            0x010D => Ok(Self::CMSG_SWAP_INV_ITEM(<CMSG_SWAP_INV_ITEM as ClientMessage>::read_body(&mut r, body_size)?)),
            0x010E => Ok(Self::CMSG_SPLIT_ITEM(<CMSG_SPLIT_ITEM as ClientMessage>::read_body(&mut r, body_size)?)),
            0x010F => Ok(Self::CMSG_AUTOEQUIP_ITEM_SLOT(<CMSG_AUTOEQUIP_ITEM_SLOT as ClientMessage>::read_body(&mut r, body_size)?)),
            0x0111 => Ok(Self::CMSG_DESTROYITEM(<CMSG_DESTROYITEM as ClientMessage>::read_body(&mut r, body_size)?)),
            0x0114 => Ok(Self::CMSG_INSPECT(<CMSG_INSPECT as ClientMessage>::read_body(&mut r, body_size)?)),
            0x0116 => Ok(Self::CMSG_INITIATE_TRADE(<CMSG_INITIATE_TRADE as ClientMessage>::read_body(&mut r, body_size)?)),
            0x0117 => Ok(Self::CMSG_BEGIN_TRADE(<CMSG_BEGIN_TRADE as ClientMessage>::read_body(&mut r, body_size)?)),
            0x0118 => Ok(Self::CMSG_BUSY_TRADE(<CMSG_BUSY_TRADE as ClientMessage>::read_body(&mut r, body_size)?)),
            0x0119 => Ok(Self::CMSG_IGNORE_TRADE(<CMSG_IGNORE_TRADE as ClientMessage>::read_body(&mut r, body_size)?)),
            0x011A => Ok(Self::CMSG_ACCEPT_TRADE(<CMSG_ACCEPT_TRADE as ClientMessage>::read_body(&mut r, body_size)?)),
            0x011B => Ok(Self::CMSG_UNACCEPT_TRADE(<CMSG_UNACCEPT_TRADE as ClientMessage>::read_body(&mut r, body_size)?)),
            0x011C => Ok(Self::CMSG_CANCEL_TRADE(<CMSG_CANCEL_TRADE as ClientMessage>::read_body(&mut r, body_size)?)),
            0x011D => Ok(Self::CMSG_SET_TRADE_ITEM(<CMSG_SET_TRADE_ITEM as ClientMessage>::read_body(&mut r, body_size)?)),
            0x011E => Ok(Self::CMSG_CLEAR_TRADE_ITEM(<CMSG_CLEAR_TRADE_ITEM as ClientMessage>::read_body(&mut r, body_size)?)),
            0x011F => Ok(Self::CMSG_SET_TRADE_GOLD(<CMSG_SET_TRADE_GOLD as ClientMessage>::read_body(&mut r, body_size)?)),
            0x0125 => Ok(Self::CMSG_SET_FACTION_ATWAR(<CMSG_SET_FACTION_ATWAR as ClientMessage>::read_body(&mut r, body_size)?)),
            0x0128 => Ok(Self::CMSG_SET_ACTION_BUTTON(<CMSG_SET_ACTION_BUTTON as ClientMessage>::read_body(&mut r, body_size)?)),
            0x012E => Ok(Self::CMSG_CAST_SPELL(<CMSG_CAST_SPELL as ClientMessage>::read_body(&mut r, body_size)?)),
            0x012F => Ok(Self::CMSG_CANCEL_CAST(<CMSG_CANCEL_CAST as ClientMessage>::read_body(&mut r, body_size)?)),
            0x0136 => Ok(Self::CMSG_CANCEL_AURA(<CMSG_CANCEL_AURA as ClientMessage>::read_body(&mut r, body_size)?)),
            0x013B => Ok(Self::CMSG_CANCEL_CHANNELLING(<CMSG_CANCEL_CHANNELLING as ClientMessage>::read_body(&mut r, body_size)?)),
            0x013D => Ok(Self::CMSG_SET_SELECTION(<CMSG_SET_SELECTION as ClientMessage>::read_body(&mut r, body_size)?)),
            0x013E => Ok(Self::CMSG_SET_TARGET_OBSOLETE(<CMSG_SET_TARGET_OBSOLETE as ClientMessage>::read_body(&mut r, body_size)?)),
            0x0141 => Ok(Self::CMSG_ATTACKSWING(<CMSG_ATTACKSWING as ClientMessage>::read_body(&mut r, body_size)?)),
            0x0142 => Ok(Self::CMSG_ATTACKSTOP(<CMSG_ATTACKSTOP as ClientMessage>::read_body(&mut r, body_size)?)),
            0x015A => Ok(Self::CMSG_REPOP_REQUEST(<CMSG_REPOP_REQUEST as ClientMessage>::read_body(&mut r, body_size)?)),
            0x015C => Ok(Self::CMSG_RESURRECT_RESPONSE(<CMSG_RESURRECT_RESPONSE as ClientMessage>::read_body(&mut r, body_size)?)),
            0x015D => Ok(Self::CMSG_LOOT(<CMSG_LOOT as ClientMessage>::read_body(&mut r, body_size)?)),
            0x015E => Ok(Self::CMSG_LOOT_MONEY(<CMSG_LOOT_MONEY as ClientMessage>::read_body(&mut r, body_size)?)),
            0x015F => Ok(Self::CMSG_LOOT_RELEASE(<CMSG_LOOT_RELEASE as ClientMessage>::read_body(&mut r, body_size)?)),
            0x016C => Ok(Self::CMSG_DUEL_ACCEPTED(<CMSG_DUEL_ACCEPTED as ClientMessage>::read_body(&mut r, body_size)?)),
            0x016D => Ok(Self::CMSG_DUEL_CANCELLED(<CMSG_DUEL_CANCELLED as ClientMessage>::read_body(&mut r, body_size)?)),
            0x0171 => Ok(Self::CMSG_MOUNTSPECIAL_ANIM(<CMSG_MOUNTSPECIAL_ANIM as ClientMessage>::read_body(&mut r, body_size)?)),
            0x0174 => Ok(Self::CMSG_PET_SET_ACTION(<CMSG_PET_SET_ACTION as ClientMessage>::read_body(&mut r, body_size)?)),
            0x0175 => Ok(Self::CMSG_PET_ACTION(<CMSG_PET_ACTION as ClientMessage>::read_body(&mut r, body_size)?)),
            0x0176 => Ok(Self::CMSG_PET_ABANDON(<CMSG_PET_ABANDON as ClientMessage>::read_body(&mut r, body_size)?)),
            0x0177 => Ok(Self::CMSG_PET_RENAME(<CMSG_PET_RENAME as ClientMessage>::read_body(&mut r, body_size)?)),
            0x017B => Ok(Self::CMSG_GOSSIP_HELLO(<CMSG_GOSSIP_HELLO as ClientMessage>::read_body(&mut r, body_size)?)),
            0x017C => Ok(Self::CMSG_GOSSIP_SELECT_OPTION(<CMSG_GOSSIP_SELECT_OPTION as ClientMessage>::read_body(&mut r, body_size)?)),
            0x017F => Ok(Self::CMSG_NPC_TEXT_QUERY(<CMSG_NPC_TEXT_QUERY as ClientMessage>::read_body(&mut r, body_size)?)),
            0x0182 => Ok(Self::CMSG_QUESTGIVER_STATUS_QUERY(<CMSG_QUESTGIVER_STATUS_QUERY as ClientMessage>::read_body(&mut r, body_size)?)),
            0x0184 => Ok(Self::CMSG_QUESTGIVER_HELLO(<CMSG_QUESTGIVER_HELLO as ClientMessage>::read_body(&mut r, body_size)?)),
            0x0186 => Ok(Self::CMSG_QUESTGIVER_QUERY_QUEST(<CMSG_QUESTGIVER_QUERY_QUEST as ClientMessage>::read_body(&mut r, body_size)?)),
            0x0187 => Ok(Self::CMSG_QUESTGIVER_QUEST_AUTOLAUNCH(<CMSG_QUESTGIVER_QUEST_AUTOLAUNCH as ClientMessage>::read_body(&mut r, body_size)?)),
            0x0189 => Ok(Self::CMSG_QUESTGIVER_ACCEPT_QUEST(<CMSG_QUESTGIVER_ACCEPT_QUEST as ClientMessage>::read_body(&mut r, body_size)?)),
            0x018A => Ok(Self::CMSG_QUESTGIVER_COMPLETE_QUEST(<CMSG_QUESTGIVER_COMPLETE_QUEST as ClientMessage>::read_body(&mut r, body_size)?)),
            0x018C => Ok(Self::CMSG_QUESTGIVER_REQUEST_REWARD(<CMSG_QUESTGIVER_REQUEST_REWARD as ClientMessage>::read_body(&mut r, body_size)?)),
            0x018E => Ok(Self::CMSG_QUESTGIVER_CHOOSE_REWARD(<CMSG_QUESTGIVER_CHOOSE_REWARD as ClientMessage>::read_body(&mut r, body_size)?)),
            0x0190 => Ok(Self::CMSG_QUESTGIVER_CANCEL(<CMSG_QUESTGIVER_CANCEL as ClientMessage>::read_body(&mut r, body_size)?)),
            0x0193 => Ok(Self::CMSG_QUESTLOG_SWAP_QUEST(<CMSG_QUESTLOG_SWAP_QUEST as ClientMessage>::read_body(&mut r, body_size)?)),
            0x0194 => Ok(Self::CMSG_QUESTLOG_REMOVE_QUEST(<CMSG_QUESTLOG_REMOVE_QUEST as ClientMessage>::read_body(&mut r, body_size)?)),
            0x019B => Ok(Self::CMSG_QUEST_CONFIRM_ACCEPT(<CMSG_QUEST_CONFIRM_ACCEPT as ClientMessage>::read_body(&mut r, body_size)?)),
            0x019D => Ok(Self::CMSG_PUSHQUESTTOPARTY(<CMSG_PUSHQUESTTOPARTY as ClientMessage>::read_body(&mut r, body_size)?)),
            0x019E => Ok(Self::CMSG_LIST_INVENTORY(<CMSG_LIST_INVENTORY as ClientMessage>::read_body(&mut r, body_size)?)),
            0x01A0 => Ok(Self::CMSG_SELL_ITEM(<CMSG_SELL_ITEM as ClientMessage>::read_body(&mut r, body_size)?)),
            0x01A2 => Ok(Self::CMSG_BUY_ITEM(<CMSG_BUY_ITEM as ClientMessage>::read_body(&mut r, body_size)?)),
            0x01A3 => Ok(Self::CMSG_BUY_ITEM_IN_SLOT(<CMSG_BUY_ITEM_IN_SLOT as ClientMessage>::read_body(&mut r, body_size)?)),
            0x01AA => Ok(Self::CMSG_TAXINODE_STATUS_QUERY(<CMSG_TAXINODE_STATUS_QUERY as ClientMessage>::read_body(&mut r, body_size)?)),
            0x01AC => Ok(Self::CMSG_TAXIQUERYAVAILABLENODES(<CMSG_TAXIQUERYAVAILABLENODES as ClientMessage>::read_body(&mut r, body_size)?)),
            0x01AD => Ok(Self::CMSG_ACTIVATETAXI(<CMSG_ACTIVATETAXI as ClientMessage>::read_body(&mut r, body_size)?)),
            0x01B0 => Ok(Self::CMSG_TRAINER_LIST(<CMSG_TRAINER_LIST as ClientMessage>::read_body(&mut r, body_size)?)),
            0x01B2 => Ok(Self::CMSG_TRAINER_BUY_SPELL(<CMSG_TRAINER_BUY_SPELL as ClientMessage>::read_body(&mut r, body_size)?)),
            0x01B5 => Ok(Self::CMSG_BINDER_ACTIVATE(<CMSG_BINDER_ACTIVATE as ClientMessage>::read_body(&mut r, body_size)?)),
            0x01B7 => Ok(Self::CMSG_BANKER_ACTIVATE(<CMSG_BANKER_ACTIVATE as ClientMessage>::read_body(&mut r, body_size)?)),
            0x01B9 => Ok(Self::CMSG_BUY_BANK_SLOT(<CMSG_BUY_BANK_SLOT as ClientMessage>::read_body(&mut r, body_size)?)),
            0x01BB => Ok(Self::CMSG_PETITION_SHOWLIST(<CMSG_PETITION_SHOWLIST as ClientMessage>::read_body(&mut r, body_size)?)),
            0x01BD => Ok(Self::CMSG_PETITION_BUY(<CMSG_PETITION_BUY as ClientMessage>::read_body(&mut r, body_size)?)),
            0x01BE => Ok(Self::CMSG_PETITION_SHOW_SIGNATURES(<CMSG_PETITION_SHOW_SIGNATURES as ClientMessage>::read_body(&mut r, body_size)?)),
            0x01C0 => Ok(Self::CMSG_PETITION_SIGN(<CMSG_PETITION_SIGN as ClientMessage>::read_body(&mut r, body_size)?)),
            0x01C3 => Ok(Self::CMSG_OFFER_PETITION(<CMSG_OFFER_PETITION as ClientMessage>::read_body(&mut r, body_size)?)),
            0x01C4 => Ok(Self::CMSG_TURN_IN_PETITION(<CMSG_TURN_IN_PETITION as ClientMessage>::read_body(&mut r, body_size)?)),
            0x01C6 => Ok(Self::CMSG_PETITION_QUERY(<CMSG_PETITION_QUERY as ClientMessage>::read_body(&mut r, body_size)?)),
            0x01CA => Ok(Self::CMSG_BUG(<CMSG_BUG as ClientMessage>::read_body(&mut r, body_size)?)),
            0x01CC => Ok(Self::CMSG_PLAYED_TIME(<CMSG_PLAYED_TIME as ClientMessage>::read_body(&mut r, body_size)?)),
            0x01CE => Ok(Self::CMSG_QUERY_TIME(<CMSG_QUERY_TIME as ClientMessage>::read_body(&mut r, body_size)?)),
            0x01D2 => Ok(Self::CMSG_RECLAIM_CORPSE(<CMSG_RECLAIM_CORPSE as ClientMessage>::read_body(&mut r, body_size)?)),
            0x01D3 => Ok(Self::CMSG_WRAP_ITEM(<CMSG_WRAP_ITEM as ClientMessage>::read_body(&mut r, body_size)?)),
            0x01D5 => Ok(Self::MSG_MINIMAP_PING(<MSG_MINIMAP_PING_Client as ClientMessage>::read_body(&mut r, body_size)?)),
            0x01DC => Ok(Self::CMSG_PING(<CMSG_PING as ClientMessage>::read_body(&mut r, body_size)?)),
            0x01E0 => Ok(Self::CMSG_SETSHEATHED(<CMSG_SETSHEATHED as ClientMessage>::read_body(&mut r, body_size)?)),
            0x01ED => Ok(Self::CMSG_AUTH_SESSION(<CMSG_AUTH_SESSION as ClientMessage>::read_body(&mut r, body_size)?)),
            0x01F0 => Ok(Self::CMSG_PET_CAST_SPELL(<CMSG_PET_CAST_SPELL as ClientMessage>::read_body(&mut r, body_size)?)),
            0x01F1 => Ok(Self::MSG_SAVE_GUILD_EMBLEM(<MSG_SAVE_GUILD_EMBLEM_Client as ClientMessage>::read_body(&mut r, body_size)?)),
            0x01F4 => Ok(Self::CMSG_ZONEUPDATE(<CMSG_ZONEUPDATE as ClientMessage>::read_body(&mut r, body_size)?)),
            0x01FB => Ok(Self::MSG_RANDOM_ROLL(<MSG_RANDOM_ROLL_Client as ClientMessage>::read_body(&mut r, body_size)?)),
            0x01FF => Ok(Self::MSG_LOOKING_FOR_GROUP(<MSG_LOOKING_FOR_GROUP_Client as ClientMessage>::read_body(&mut r, body_size)?)),
            0x0202 => Ok(Self::CMSG_UNLEARN_SKILL(<CMSG_UNLEARN_SKILL as ClientMessage>::read_body(&mut r, body_size)?)),
            0x0205 => Ok(Self::CMSG_GMTICKET_CREATE(<CMSG_GMTICKET_CREATE as ClientMessage>::read_body(&mut r, body_size)?)),
            0x0207 => Ok(Self::CMSG_GMTICKET_UPDATETEXT(<CMSG_GMTICKET_UPDATETEXT as ClientMessage>::read_body(&mut r, body_size)?)),
            0x020A => Ok(Self::CMSG_REQUEST_ACCOUNT_DATA(<CMSG_REQUEST_ACCOUNT_DATA as ClientMessage>::read_body(&mut r, body_size)?)),
            0x0211 => Ok(Self::CMSG_GMTICKET_GETTICKET(<CMSG_GMTICKET_GETTICKET as ClientMessage>::read_body(&mut r, body_size)?)),
            0x0216 => Ok(Self::MSG_CORPSE_QUERY(<MSG_CORPSE_QUERY_Client as ClientMessage>::read_body(&mut r, body_size)?)),
            0x0217 => Ok(Self::CMSG_GMTICKET_DELETETICKET(<CMSG_GMTICKET_DELETETICKET as ClientMessage>::read_body(&mut r, body_size)?)),
            0x021A => Ok(Self::CMSG_GMTICKET_SYSTEMSTATUS(<CMSG_GMTICKET_SYSTEMSTATUS as ClientMessage>::read_body(&mut r, body_size)?)),
            0x021C => Ok(Self::CMSG_SPIRIT_HEALER_ACTIVATE(<CMSG_SPIRIT_HEALER_ACTIVATE as ClientMessage>::read_body(&mut r, body_size)?)),
            0x0225 => Ok(Self::CMSG_CHAT_IGNORED(<CMSG_CHAT_IGNORED as ClientMessage>::read_body(&mut r, body_size)?)),
            0x0231 => Ok(Self::CMSG_GUILD_RANK(<CMSG_GUILD_RANK as ClientMessage>::read_body(&mut r, body_size)?)),
            0x0232 => Ok(Self::CMSG_GUILD_ADD_RANK(<CMSG_GUILD_ADD_RANK as ClientMessage>::read_body(&mut r, body_size)?)),
            0x0233 => Ok(Self::CMSG_GUILD_DEL_RANK(<CMSG_GUILD_DEL_RANK as ClientMessage>::read_body(&mut r, body_size)?)),
            0x0234 => Ok(Self::CMSG_GUILD_SET_PUBLIC_NOTE(<CMSG_GUILD_SET_PUBLIC_NOTE as ClientMessage>::read_body(&mut r, body_size)?)),
            0x0235 => Ok(Self::CMSG_GUILD_SET_OFFICER_NOTE(<CMSG_GUILD_SET_OFFICER_NOTE as ClientMessage>::read_body(&mut r, body_size)?)),
            0x0238 => Ok(Self::CMSG_SEND_MAIL(<CMSG_SEND_MAIL as ClientMessage>::read_body(&mut r, body_size)?)),
            0x023A => Ok(Self::CMSG_GET_MAIL_LIST(<CMSG_GET_MAIL_LIST as ClientMessage>::read_body(&mut r, body_size)?)),
            0x023C => Ok(Self::CMSG_BATTLEFIELD_LIST(<CMSG_BATTLEFIELD_LIST as ClientMessage>::read_body(&mut r, body_size)?)),
            0x023E => Ok(Self::CMSG_BATTLEFIELD_JOIN(<CMSG_BATTLEFIELD_JOIN as ClientMessage>::read_body(&mut r, body_size)?)),
            0x0243 => Ok(Self::CMSG_ITEM_TEXT_QUERY(<CMSG_ITEM_TEXT_QUERY as ClientMessage>::read_body(&mut r, body_size)?)),
            0x0245 => Ok(Self::CMSG_MAIL_TAKE_MONEY(<CMSG_MAIL_TAKE_MONEY as ClientMessage>::read_body(&mut r, body_size)?)),
            0x0246 => Ok(Self::CMSG_MAIL_TAKE_ITEM(<CMSG_MAIL_TAKE_ITEM as ClientMessage>::read_body(&mut r, body_size)?)),
            0x0247 => Ok(Self::CMSG_MAIL_MARK_AS_READ(<CMSG_MAIL_MARK_AS_READ as ClientMessage>::read_body(&mut r, body_size)?)),
            0x0248 => Ok(Self::CMSG_MAIL_RETURN_TO_SENDER(<CMSG_MAIL_RETURN_TO_SENDER as ClientMessage>::read_body(&mut r, body_size)?)),
            0x0249 => Ok(Self::CMSG_MAIL_DELETE(<CMSG_MAIL_DELETE as ClientMessage>::read_body(&mut r, body_size)?)),
            0x024A => Ok(Self::CMSG_MAIL_CREATE_TEXT_ITEM(<CMSG_MAIL_CREATE_TEXT_ITEM as ClientMessage>::read_body(&mut r, body_size)?)),
            0x0251 => Ok(Self::CMSG_LEARN_TALENT(<CMSG_LEARN_TALENT as ClientMessage>::read_body(&mut r, body_size)?)),
            0x0253 => Ok(Self::CMSG_TOGGLE_PVP(<CMSG_TOGGLE_PVP as ClientMessage>::read_body(&mut r, body_size)?)),
            0x0255 => Ok(Self::MSG_AUCTION_HELLO(<MSG_AUCTION_HELLO_Client as ClientMessage>::read_body(&mut r, body_size)?)),
            0x0256 => Ok(Self::CMSG_AUCTION_SELL_ITEM(<CMSG_AUCTION_SELL_ITEM as ClientMessage>::read_body(&mut r, body_size)?)),
            0x0257 => Ok(Self::CMSG_AUCTION_REMOVE_ITEM(<CMSG_AUCTION_REMOVE_ITEM as ClientMessage>::read_body(&mut r, body_size)?)),
            0x0258 => Ok(Self::CMSG_AUCTION_LIST_ITEMS(<CMSG_AUCTION_LIST_ITEMS as ClientMessage>::read_body(&mut r, body_size)?)),
            0x0259 => Ok(Self::CMSG_AUCTION_LIST_OWNER_ITEMS(<CMSG_AUCTION_LIST_OWNER_ITEMS as ClientMessage>::read_body(&mut r, body_size)?)),
            0x025A => Ok(Self::CMSG_AUCTION_PLACE_BID(<CMSG_AUCTION_PLACE_BID as ClientMessage>::read_body(&mut r, body_size)?)),
            0x0264 => Ok(Self::CMSG_AUCTION_LIST_BIDDER_ITEMS(<CMSG_AUCTION_LIST_BIDDER_ITEMS as ClientMessage>::read_body(&mut r, body_size)?)),
            0x0268 => Ok(Self::CMSG_SET_AMMO(<CMSG_SET_AMMO as ClientMessage>::read_body(&mut r, body_size)?)),
            0x026A => Ok(Self::CMSG_SET_ACTIVE_MOVER(<CMSG_SET_ACTIVE_MOVER as ClientMessage>::read_body(&mut r, body_size)?)),
            0x026B => Ok(Self::CMSG_PET_CANCEL_AURA(<CMSG_PET_CANCEL_AURA as ClientMessage>::read_body(&mut r, body_size)?)),
            0x026D => Ok(Self::CMSG_CANCEL_AUTO_REPEAT_SPELL(<CMSG_CANCEL_AUTO_REPEAT_SPELL as ClientMessage>::read_body(&mut r, body_size)?)),
            0x026F => Ok(Self::MSG_LIST_STABLED_PETS(<MSG_LIST_STABLED_PETS_Client as ClientMessage>::read_body(&mut r, body_size)?)),
            0x0270 => Ok(Self::CMSG_STABLE_PET(<CMSG_STABLE_PET as ClientMessage>::read_body(&mut r, body_size)?)),
            0x0271 => Ok(Self::CMSG_UNSTABLE_PET(<CMSG_UNSTABLE_PET as ClientMessage>::read_body(&mut r, body_size)?)),
            0x0272 => Ok(Self::CMSG_BUY_STABLE_SLOT(<CMSG_BUY_STABLE_SLOT as ClientMessage>::read_body(&mut r, body_size)?)),
            0x0275 => Ok(Self::CMSG_STABLE_SWAP_PET(<CMSG_STABLE_SWAP_PET as ClientMessage>::read_body(&mut r, body_size)?)),
            0x0279 => Ok(Self::CMSG_REQUEST_PET_INFO(<CMSG_REQUEST_PET_INFO as ClientMessage>::read_body(&mut r, body_size)?)),
            0x027A => Ok(Self::CMSG_FAR_SIGHT(<CMSG_FAR_SIGHT as ClientMessage>::read_body(&mut r, body_size)?)),
            0x027E => Ok(Self::CMSG_GROUP_CHANGE_SUB_GROUP(<CMSG_GROUP_CHANGE_SUB_GROUP as ClientMessage>::read_body(&mut r, body_size)?)),
            0x027F => Ok(Self::CMSG_REQUEST_PARTY_MEMBER_STATS(<CMSG_REQUEST_PARTY_MEMBER_STATS as ClientMessage>::read_body(&mut r, body_size)?)),
            0x0280 => Ok(Self::CMSG_GROUP_SWAP_SUB_GROUP(<CMSG_GROUP_SWAP_SUB_GROUP as ClientMessage>::read_body(&mut r, body_size)?)),
            0x0282 => Ok(Self::CMSG_AUTOSTORE_BANK_ITEM(<CMSG_AUTOSTORE_BANK_ITEM as ClientMessage>::read_body(&mut r, body_size)?)),
            0x0283 => Ok(Self::CMSG_AUTOBANK_ITEM(<CMSG_AUTOBANK_ITEM as ClientMessage>::read_body(&mut r, body_size)?)),
            0x0284 => Ok(Self::MSG_QUERY_NEXT_MAIL_TIME(<MSG_QUERY_NEXT_MAIL_TIME_Client as ClientMessage>::read_body(&mut r, body_size)?)),
            0x028E => Ok(Self::CMSG_GROUP_RAID_CONVERT(<CMSG_GROUP_RAID_CONVERT as ClientMessage>::read_body(&mut r, body_size)?)),
            0x028F => Ok(Self::CMSG_GROUP_ASSISTANT_LEADER(<CMSG_GROUP_ASSISTANT_LEADER as ClientMessage>::read_body(&mut r, body_size)?)),
            0x0290 => Ok(Self::CMSG_BUYBACK_ITEM(<CMSG_BUYBACK_ITEM as ClientMessage>::read_body(&mut r, body_size)?)),
            0x0292 => Ok(Self::CMSG_MEETINGSTONE_JOIN(<CMSG_MEETINGSTONE_JOIN as ClientMessage>::read_body(&mut r, body_size)?)),
            0x0293 => Ok(Self::CMSG_MEETINGSTONE_LEAVE(<CMSG_MEETINGSTONE_LEAVE as ClientMessage>::read_body(&mut r, body_size)?)),
            0x0296 => Ok(Self::CMSG_MEETINGSTONE_INFO(<CMSG_MEETINGSTONE_INFO as ClientMessage>::read_body(&mut r, body_size)?)),
            0x029B => Ok(Self::CMSG_CANCEL_GROWTH_AURA(<CMSG_CANCEL_GROWTH_AURA as ClientMessage>::read_body(&mut r, body_size)?)),
            0x02A0 => Ok(Self::CMSG_LOOT_ROLL(<CMSG_LOOT_ROLL as ClientMessage>::read_body(&mut r, body_size)?)),
            0x02A3 => Ok(Self::CMSG_LOOT_MASTER_GIVE(<CMSG_LOOT_MASTER_GIVE as ClientMessage>::read_body(&mut r, body_size)?)),
            0x02A8 => Ok(Self::CMSG_REPAIR_ITEM(<CMSG_REPAIR_ITEM as ClientMessage>::read_body(&mut r, body_size)?)),
            0x02AA => Ok(Self::MSG_TALENT_WIPE_CONFIRM(<MSG_TALENT_WIPE_CONFIRM_Client as ClientMessage>::read_body(&mut r, body_size)?)),
            0x02AC => Ok(Self::CMSG_SUMMON_RESPONSE(<CMSG_SUMMON_RESPONSE as ClientMessage>::read_body(&mut r, body_size)?)),
            0x02B3 => Ok(Self::CMSG_SELF_RES(<CMSG_SELF_RES as ClientMessage>::read_body(&mut r, body_size)?)),
            0x02B9 => Ok(Self::CMSG_TOGGLE_HELM(<CMSG_TOGGLE_HELM as ClientMessage>::read_body(&mut r, body_size)?)),
            0x02BA => Ok(Self::CMSG_TOGGLE_CLOAK(<CMSG_TOGGLE_CLOAK as ClientMessage>::read_body(&mut r, body_size)?)),
            0x02BF => Ok(Self::CMSG_SET_ACTIONBAR_TOGGLES(<CMSG_SET_ACTIONBAR_TOGGLES as ClientMessage>::read_body(&mut r, body_size)?)),
            0x02C4 => Ok(Self::CMSG_ITEM_NAME_QUERY(<CMSG_ITEM_NAME_QUERY as ClientMessage>::read_body(&mut r, body_size)?)),
            0x02C7 => Ok(Self::CMSG_CHAR_RENAME(<CMSG_CHAR_RENAME as ClientMessage>::read_body(&mut r, body_size)?)),
            0x02C9 => Ok(Self::CMSG_MOVE_SPLINE_DONE(<CMSG_MOVE_SPLINE_DONE as ClientMessage>::read_body(&mut r, body_size)?)),
            0x02CA => Ok(Self::CMSG_MOVE_FALL_RESET(<CMSG_MOVE_FALL_RESET as ClientMessage>::read_body(&mut r, body_size)?)),
            0x02CD => Ok(Self::CMSG_REQUEST_RAID_INFO(<CMSG_REQUEST_RAID_INFO as ClientMessage>::read_body(&mut r, body_size)?)),
            0x02CE => Ok(Self::CMSG_MOVE_TIME_SKIPPED(<CMSG_MOVE_TIME_SKIPPED as ClientMessage>::read_body(&mut r, body_size)?)),
            0x02CF => Ok(Self::CMSG_MOVE_FEATHER_FALL_ACK(<CMSG_MOVE_FEATHER_FALL_ACK as ClientMessage>::read_body(&mut r, body_size)?)),
            0x02D0 => Ok(Self::CMSG_MOVE_WATER_WALK_ACK(<CMSG_MOVE_WATER_WALK_ACK as ClientMessage>::read_body(&mut r, body_size)?)),
            0x02D1 => Ok(Self::CMSG_MOVE_NOT_ACTIVE_MOVER(<CMSG_MOVE_NOT_ACTIVE_MOVER as ClientMessage>::read_body(&mut r, body_size)?)),
            0x02D3 => Ok(Self::CMSG_BATTLEFIELD_STATUS(<CMSG_BATTLEFIELD_STATUS as ClientMessage>::read_body(&mut r, body_size)?)),
            0x02D5 => Ok(Self::CMSG_BATTLEFIELD_PORT(<CMSG_BATTLEFIELD_PORT as ClientMessage>::read_body(&mut r, body_size)?)),
            0x02D6 => Ok(Self::MSG_INSPECT_HONOR_STATS(<MSG_INSPECT_HONOR_STATS_Client as ClientMessage>::read_body(&mut r, body_size)?)),
            0x02D7 => Ok(Self::CMSG_BATTLEMASTER_HELLO(<CMSG_BATTLEMASTER_HELLO as ClientMessage>::read_body(&mut r, body_size)?)),
            0x02DB => Ok(Self::CMSG_FORCE_WALK_SPEED_CHANGE_ACK(<CMSG_FORCE_WALK_SPEED_CHANGE_ACK as ClientMessage>::read_body(&mut r, body_size)?)),
            0x02DD => Ok(Self::CMSG_FORCE_SWIM_BACK_SPEED_CHANGE_ACK(<CMSG_FORCE_SWIM_BACK_SPEED_CHANGE_ACK as ClientMessage>::read_body(&mut r, body_size)?)),
            0x02DF => Ok(Self::CMSG_FORCE_TURN_RATE_CHANGE_ACK(<CMSG_FORCE_TURN_RATE_CHANGE_ACK as ClientMessage>::read_body(&mut r, body_size)?)),
            0x02E0 => Ok(Self::MSG_PVP_LOG_DATA(<MSG_PVP_LOG_DATA_Client as ClientMessage>::read_body(&mut r, body_size)?)),
            0x02E1 => Ok(Self::CMSG_LEAVE_BATTLEFIELD(<CMSG_LEAVE_BATTLEFIELD as ClientMessage>::read_body(&mut r, body_size)?)),
            0x02E2 => Ok(Self::CMSG_AREA_SPIRIT_HEALER_QUERY(<CMSG_AREA_SPIRIT_HEALER_QUERY as ClientMessage>::read_body(&mut r, body_size)?)),
            0x02E3 => Ok(Self::CMSG_AREA_SPIRIT_HEALER_QUEUE(<CMSG_AREA_SPIRIT_HEALER_QUEUE as ClientMessage>::read_body(&mut r, body_size)?)),
            0x02E9 => Ok(Self::MSG_BATTLEGROUND_PLAYER_POSITIONS(<MSG_BATTLEGROUND_PLAYER_POSITIONS_Client as ClientMessage>::read_body(&mut r, body_size)?)),
            0x02EA => Ok(Self::CMSG_PET_STOP_ATTACK(<CMSG_PET_STOP_ATTACK as ClientMessage>::read_body(&mut r, body_size)?)),
            0x02EE => Ok(Self::CMSG_BATTLEMASTER_JOIN(<CMSG_BATTLEMASTER_JOIN as ClientMessage>::read_body(&mut r, body_size)?)),
            0x02F0 => Ok(Self::CMSG_PET_UNLEARN(<CMSG_PET_UNLEARN as ClientMessage>::read_body(&mut r, body_size)?)),
            0x02F3 => Ok(Self::CMSG_PET_SPELL_AUTOCAST(<CMSG_PET_SPELL_AUTOCAST as ClientMessage>::read_body(&mut r, body_size)?)),
            0x02FC => Ok(Self::CMSG_GUILD_INFO_TEXT(<CMSG_GUILD_INFO_TEXT as ClientMessage>::read_body(&mut r, body_size)?)),
            0x0312 => Ok(Self::CMSG_ACTIVATETAXIEXPRESS(<CMSG_ACTIVATETAXIEXPRESS as ClientMessage>::read_body(&mut r, body_size)?)),
            0x0317 => Ok(Self::CMSG_SET_FACTION_INACTIVE(<CMSG_SET_FACTION_INACTIVE as ClientMessage>::read_body(&mut r, body_size)?)),
            0x0318 => Ok(Self::CMSG_SET_WATCHED_FACTION(<CMSG_SET_WATCHED_FACTION as ClientMessage>::read_body(&mut r, body_size)?)),
            0x031D => Ok(Self::CMSG_RESET_INSTANCES(<CMSG_RESET_INSTANCES as ClientMessage>::read_body(&mut r, body_size)?)),
            0x0321 => Ok(Self::MSG_RAID_TARGET_UPDATE(<MSG_RAID_TARGET_UPDATE_Client as ClientMessage>::read_body(&mut r, body_size)?)),
            0x0322 => Ok(Self::MSG_RAID_READY_CHECK(<MSG_RAID_READY_CHECK_Client as ClientMessage>::read_body(&mut r, body_size)?)),
            0x032A => Ok(Self::CMSG_GMSURVEY_SUBMIT(<CMSG_GMSURVEY_SUBMIT as ClientMessage>::read_body(&mut r, body_size)?)),
            _ => Err(crate::errors::ExpectedOpcodeError::Opcode(opcode as u32)),
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
    #[cfg(feature = "sync")]
    pub fn read_encrypted<R: std::io::Read, D: Decrypter>(r: &mut R, d: &mut D) -> std::result::Result<Self, crate::errors::ExpectedOpcodeError> {
        let mut header = [0_u8; 6];
        r.read_exact(&mut header)?;
        let header = d.decrypt_client_header(header);
        let body_size = (header.size.saturating_sub(4)) as u32;

        let mut buf = vec![0; body_size as usize];
        r.read_exact(&mut buf)?;
        Self::read_opcodes(header.opcode, body_size, &buf)
    }

    #[cfg(feature = "tokio")]
    pub async fn tokio_read_unencrypted<R: tokio::io::AsyncReadExt + Unpin + Send>(r: &mut R) -> std::result::Result<Self, crate::errors::ExpectedOpcodeError> {
        let size = (crate::util::tokio_read_u16_be(r).await?.saturating_sub(4)) as u32;
        let opcode = crate::util::tokio_read_u32_le(r).await?;

        let mut buf = vec![0; size as usize];
        r.read_exact(&mut buf).await?;
        Self::read_opcodes(opcode, size, &buf)
    }
    #[cfg(feature = "tokio")]
    pub async fn tokio_read_encrypted<R: tokio::io::AsyncReadExt + Unpin + Send, D: Decrypter + Send>(r: &mut R, d: &mut D) -> std::result::Result<Self, crate::errors::ExpectedOpcodeError> {
        let mut header = [0_u8; 6];
        r.read_exact(&mut header).await?;
        let header = d.decrypt_client_header(header);
        let body_size = (header.size.saturating_sub(4)) as u32;

        let mut buf = vec![0; body_size as usize];
        r.read_exact(&mut buf).await?;
        Self::read_opcodes(header.opcode, body_size, &buf)
    }

    #[cfg(feature = "async-std")]
    pub async fn astd_read_unencrypted<R: async_std::io::ReadExt + Unpin + Send>(r: &mut R) -> std::result::Result<Self, crate::errors::ExpectedOpcodeError> {
        let size = (crate::util::astd_read_u16_be(r).await?.saturating_sub(4)) as u32;
        let opcode = crate::util::astd_read_u32_le(r).await?;

        let mut buf = vec![0; size as usize];
        r.read_exact(&mut buf).await?;
        Self::read_opcodes(opcode, size, &buf)
    }
    #[cfg(feature = "async-std")]
    pub async fn astd_read_encrypted<R: async_std::io::ReadExt + Unpin + Send, D: Decrypter + Send>(r: &mut R, d: &mut D) -> std::result::Result<Self, crate::errors::ExpectedOpcodeError> {
        let mut header = [0_u8; 6];
        r.read_exact(&mut header).await?;
        let header = d.decrypt_client_header(header);
        let body_size = (header.size.saturating_sub(4)) as u32;

        let mut buf = vec![0; body_size as usize];
        r.read_exact(&mut buf).await?;
        Self::read_opcodes(header.opcode, body_size, &buf)
    }

}

use crate::world::version_1_12::SMSG_CHAR_CREATE;
use crate::world::version_1_12::SMSG_CHAR_ENUM;
use crate::world::version_1_12::SMSG_CHAR_DELETE;
use crate::world::version_1_12::SMSG_NEW_WORLD;
use crate::world::version_1_12::SMSG_TRANSFER_PENDING;
use crate::world::version_1_12::SMSG_TRANSFER_ABORTED;
use crate::world::version_1_12::SMSG_CHARACTER_LOGIN_FAILED;
use crate::world::version_1_12::SMSG_LOGIN_SETTIMESPEED;
use crate::world::version_1_12::SMSG_LOGOUT_RESPONSE;
use crate::world::version_1_12::SMSG_LOGOUT_COMPLETE;
use crate::world::version_1_12::SMSG_LOGOUT_CANCEL_ACK;
use crate::world::version_1_12::SMSG_NAME_QUERY_RESPONSE;
use crate::world::version_1_12::SMSG_PET_NAME_QUERY_RESPONSE;
use crate::world::version_1_12::SMSG_GUILD_QUERY_RESPONSE;
use crate::world::version_1_12::SMSG_ITEM_QUERY_SINGLE_RESPONSE;
use crate::world::version_1_12::SMSG_PAGE_TEXT_QUERY_RESPONSE;
use crate::world::version_1_12::SMSG_QUEST_QUERY_RESPONSE;
use crate::world::version_1_12::SMSG_GAMEOBJECT_QUERY_RESPONSE;
use crate::world::version_1_12::SMSG_CREATURE_QUERY_RESPONSE;
use crate::world::version_1_12::SMSG_WHO;
use crate::world::version_1_12::SMSG_WHOIS;
use crate::world::version_1_12::SMSG_FRIEND_LIST;
use crate::world::version_1_12::SMSG_FRIEND_STATUS;
use crate::world::version_1_12::SMSG_IGNORE_LIST;
use crate::world::version_1_12::SMSG_GROUP_INVITE;
use crate::world::version_1_12::SMSG_GROUP_DECLINE;
use crate::world::version_1_12::SMSG_GROUP_UNINVITE;
use crate::world::version_1_12::SMSG_GROUP_SET_LEADER;
use crate::world::version_1_12::SMSG_GROUP_DESTROYED;
use crate::world::version_1_12::SMSG_GROUP_LIST;
use crate::world::version_1_12::SMSG_PARTY_MEMBER_STATS;
use crate::world::version_1_12::SMSG_PARTY_COMMAND_RESULT;
use crate::world::version_1_12::SMSG_GUILD_INVITE;
use crate::world::version_1_12::SMSG_GUILD_INFO;
use crate::world::version_1_12::SMSG_GUILD_ROSTER;
use crate::world::version_1_12::SMSG_GUILD_EVENT;
use crate::world::version_1_12::SMSG_GUILD_COMMAND_RESULT;
use crate::world::version_1_12::SMSG_MESSAGECHAT;
use crate::world::version_1_12::SMSG_CHANNEL_NOTIFY;
use crate::world::version_1_12::SMSG_CHANNEL_LIST;
use crate::world::version_1_12::SMSG_UPDATE_OBJECT;
use crate::world::version_1_12::SMSG_DESTROY_OBJECT;
use crate::world::version_1_12::SMSG_READ_ITEM_OK;
use crate::world::version_1_12::SMSG_READ_ITEM_FAILED;
use crate::world::version_1_12::SMSG_ITEM_COOLDOWN;
use crate::world::version_1_12::SMSG_GAMEOBJECT_CUSTOM_ANIM;
use crate::world::version_1_12::SMSG_MONSTER_MOVE;
use crate::world::version_1_12::SMSG_MOVE_WATER_WALK;
use crate::world::version_1_12::SMSG_MOVE_LAND_WALK;
use crate::world::version_1_12::SMSG_FORCE_RUN_SPEED_CHANGE;
use crate::world::version_1_12::SMSG_FORCE_RUN_BACK_SPEED_CHANGE;
use crate::world::version_1_12::SMSG_FORCE_SWIM_SPEED_CHANGE;
use crate::world::version_1_12::SMSG_FORCE_MOVE_ROOT;
use crate::world::version_1_12::SMSG_FORCE_MOVE_UNROOT;
use crate::world::version_1_12::SMSG_MOVE_KNOCK_BACK;
use crate::world::version_1_12::SMSG_MOVE_FEATHER_FALL;
use crate::world::version_1_12::SMSG_MOVE_NORMAL_FALL;
use crate::world::version_1_12::SMSG_MOVE_SET_HOVER;
use crate::world::version_1_12::SMSG_MOVE_UNSET_HOVER;
use crate::world::version_1_12::SMSG_TRIGGER_CINEMATIC;
use crate::world::version_1_12::SMSG_TUTORIAL_FLAGS;
use crate::world::version_1_12::SMSG_EMOTE;
use crate::world::version_1_12::SMSG_TEXT_EMOTE;
use crate::world::version_1_12::SMSG_INVENTORY_CHANGE_FAILURE;
use crate::world::version_1_12::SMSG_OPEN_CONTAINER;
use crate::world::version_1_12::SMSG_INSPECT;
use crate::world::version_1_12::SMSG_TRADE_STATUS;
use crate::world::version_1_12::SMSG_TRADE_STATUS_EXTENDED;
use crate::world::version_1_12::SMSG_INITIALIZE_FACTIONS;
use crate::world::version_1_12::SMSG_SET_FACTION_VISIBLE;
use crate::world::version_1_12::SMSG_SET_FACTION_STANDING;
use crate::world::version_1_12::SMSG_SET_PROFICIENCY;
use crate::world::version_1_12::SMSG_ACTION_BUTTONS;
use crate::world::version_1_12::SMSG_INITIAL_SPELLS;
use crate::world::version_1_12::SMSG_LEARNED_SPELL;
use crate::world::version_1_12::SMSG_SUPERCEDED_SPELL;
use crate::world::version_1_12::SMSG_CAST_RESULT;
use crate::world::version_1_12::SMSG_SPELL_START;
use crate::world::version_1_12::SMSG_SPELL_GO;
use crate::world::version_1_12::SMSG_SPELL_FAILURE;
use crate::world::version_1_12::SMSG_SPELL_COOLDOWN;
use crate::world::version_1_12::SMSG_COOLDOWN_EVENT;
use crate::world::version_1_12::SMSG_UPDATE_AURA_DURATION;
use crate::world::version_1_12::SMSG_PET_CAST_FAILED;
use crate::world::version_1_12::SMSG_AI_REACTION;
use crate::world::version_1_12::SMSG_ATTACKSTART;
use crate::world::version_1_12::SMSG_ATTACKSTOP;
use crate::world::version_1_12::SMSG_ATTACKSWING_NOTINRANGE;
use crate::world::version_1_12::SMSG_ATTACKSWING_BADFACING;
use crate::world::version_1_12::SMSG_ATTACKSWING_NOTSTANDING;
use crate::world::version_1_12::SMSG_ATTACKSWING_DEADTARGET;
use crate::world::version_1_12::SMSG_ATTACKSWING_CANT_ATTACK;
use crate::world::version_1_12::SMSG_ATTACKERSTATEUPDATE;
use crate::world::version_1_12::SMSG_CANCEL_COMBAT;
use crate::world::version_1_12::SMSG_SPELLHEALLOG;
use crate::world::version_1_12::SMSG_SPELLENERGIZELOG;
use crate::world::version_1_12::SMSG_BINDPOINTUPDATE;
use crate::world::version_1_12::SMSG_PLAYERBOUND;
use crate::world::version_1_12::SMSG_CLIENT_CONTROL_UPDATE;
use crate::world::version_1_12::SMSG_RESURRECT_REQUEST;
use crate::world::version_1_12::SMSG_LOOT_RESPONSE;
use crate::world::version_1_12::SMSG_LOOT_RELEASE_RESPONSE;
use crate::world::version_1_12::SMSG_LOOT_REMOVED;
use crate::world::version_1_12::SMSG_LOOT_MONEY_NOTIFY;
use crate::world::version_1_12::SMSG_LOOT_CLEAR_MONEY;
use crate::world::version_1_12::SMSG_ITEM_PUSH_RESULT;
use crate::world::version_1_12::SMSG_DUEL_REQUESTED;
use crate::world::version_1_12::SMSG_DUEL_OUTOFBOUNDS;
use crate::world::version_1_12::SMSG_DUEL_INBOUNDS;
use crate::world::version_1_12::SMSG_DUEL_COMPLETE;
use crate::world::version_1_12::SMSG_DUEL_WINNER;
use crate::world::version_1_12::SMSG_MOUNTRESULT;
use crate::world::version_1_12::SMSG_DISMOUNTRESULT;
use crate::world::version_1_12::SMSG_MOUNTSPECIAL_ANIM;
use crate::world::version_1_12::SMSG_PET_TAME_FAILURE;
use crate::world::version_1_12::SMSG_PET_NAME_INVALID;
use crate::world::version_1_12::SMSG_PET_SPELLS;
use crate::world::version_1_12::SMSG_PET_MODE;
use crate::world::version_1_12::SMSG_GOSSIP_MESSAGE;
use crate::world::version_1_12::SMSG_GOSSIP_COMPLETE;
use crate::world::version_1_12::SMSG_NPC_TEXT_UPDATE;
use crate::world::version_1_12::SMSG_QUESTGIVER_STATUS;
use crate::world::version_1_12::SMSG_QUESTGIVER_QUEST_LIST;
use crate::world::version_1_12::SMSG_QUESTGIVER_QUEST_DETAILS;
use crate::world::version_1_12::SMSG_QUESTGIVER_REQUEST_ITEMS;
use crate::world::version_1_12::SMSG_QUESTGIVER_OFFER_REWARD;
use crate::world::version_1_12::SMSG_QUESTGIVER_QUEST_INVALID;
use crate::world::version_1_12::SMSG_QUESTGIVER_QUEST_COMPLETE;
use crate::world::version_1_12::SMSG_QUESTGIVER_QUEST_FAILED;
use crate::world::version_1_12::SMSG_QUESTLOG_FULL;
use crate::world::version_1_12::SMSG_QUESTUPDATE_FAILED;
use crate::world::version_1_12::SMSG_QUESTUPDATE_FAILEDTIMER;
use crate::world::version_1_12::SMSG_QUESTUPDATE_COMPLETE;
use crate::world::version_1_12::SMSG_QUESTUPDATE_ADD_KILL;
use crate::world::version_1_12::SMSG_QUESTUPDATE_ADD_ITEM;
use crate::world::version_1_12::SMSG_QUEST_CONFIRM_ACCEPT;
use crate::world::version_1_12::SMSG_LIST_INVENTORY;
use crate::world::version_1_12::SMSG_SELL_ITEM;
use crate::world::version_1_12::SMSG_BUY_ITEM;
use crate::world::version_1_12::SMSG_BUY_FAILED;
use crate::world::version_1_12::SMSG_SHOWTAXINODES;
use crate::world::version_1_12::SMSG_TAXINODE_STATUS;
use crate::world::version_1_12::SMSG_ACTIVATETAXIREPLY;
use crate::world::version_1_12::SMSG_NEW_TAXI_PATH;
use crate::world::version_1_12::SMSG_TRAINER_LIST;
use crate::world::version_1_12::SMSG_TRAINER_BUY_SUCCEEDED;
use crate::world::version_1_12::SMSG_TRAINER_BUY_FAILED;
use crate::world::version_1_12::SMSG_SHOW_BANK;
use crate::world::version_1_12::SMSG_BUY_BANK_SLOT_RESULT;
use crate::world::version_1_12::SMSG_PETITION_SHOWLIST;
use crate::world::version_1_12::SMSG_PETITION_SHOW_SIGNATURES;
use crate::world::version_1_12::SMSG_PETITION_SIGN_RESULTS;
use crate::world::version_1_12::SMSG_TURN_IN_PETITION_RESULTS;
use crate::world::version_1_12::SMSG_PETITION_QUERY_RESPONSE;
use crate::world::version_1_12::SMSG_FISH_NOT_HOOKED;
use crate::world::version_1_12::SMSG_FISH_ESCAPED;
use crate::world::version_1_12::SMSG_NOTIFICATION;
use crate::world::version_1_12::SMSG_PLAYED_TIME;
use crate::world::version_1_12::SMSG_QUERY_TIME_RESPONSE;
use crate::world::version_1_12::SMSG_LOG_XPGAIN;
use crate::world::version_1_12::SMSG_LEVELUP_INFO;
use crate::world::version_1_12::MSG_MINIMAP_PING_Server;
use crate::world::version_1_12::SMSG_RESISTLOG;
use crate::world::version_1_12::SMSG_ENCHANTMENTLOG;
use crate::world::version_1_12::SMSG_START_MIRROR_TIMER;
use crate::world::version_1_12::SMSG_PAUSE_MIRROR_TIMER;
use crate::world::version_1_12::SMSG_STOP_MIRROR_TIMER;
use crate::world::version_1_12::SMSG_PONG;
use crate::world::version_1_12::SMSG_CLEAR_COOLDOWN;
use crate::world::version_1_12::SMSG_GAMEOBJECT_PAGETEXT;
use crate::world::version_1_12::SMSG_SPELL_DELAYED;
use crate::world::version_1_12::SMSG_ITEM_TIME_UPDATE;
use crate::world::version_1_12::SMSG_ITEM_ENCHANT_TIME_UPDATE;
use crate::world::version_1_2::SMSG_AUTH_CHALLENGE;
use crate::world::version_1_2::SMSG_AUTH_RESPONSE;
use crate::world::version_1_12::MSG_SAVE_GUILD_EMBLEM_Server;
use crate::world::version_1_12::SMSG_PLAY_SPELL_VISUAL;
use crate::world::version_1_12::SMSG_PARTYKILLLOG;
use crate::world::version_1_12::SMSG_PLAY_SPELL_IMPACT;
use crate::world::version_1_12::SMSG_EXPLORATION_EXPERIENCE;
use crate::world::version_1_12::MSG_RANDOM_ROLL_Server;
use crate::world::version_1_12::SMSG_ENVIRONMENTALDAMAGELOG;
use crate::world::version_1_12::MSG_LOOKING_FOR_GROUP_Server;
use crate::world::version_1_12::SMSG_REMOVED_SPELL;
use crate::world::version_1_12::SMSG_GMTICKET_CREATE;
use crate::world::version_1_12::SMSG_GMTICKET_UPDATETEXT;
use crate::world::version_1_12::SMSG_ACCOUNT_DATA_TIMES;
use crate::world::version_1_12::SMSG_GMTICKET_GETTICKET;
use crate::world::version_1_12::SMSG_GAMEOBJECT_SPAWN_ANIM;
use crate::world::version_1_12::SMSG_GAMEOBJECT_DESPAWN_ANIM;
use crate::world::version_1_12::MSG_CORPSE_QUERY_Server;
use crate::world::version_1_12::SMSG_GMTICKET_DELETETICKET;
use crate::world::version_1_12::SMSG_CHAT_WRONG_FACTION;
use crate::world::version_1_12::SMSG_GMTICKET_SYSTEMSTATUS;
use crate::world::version_1_12::SMSG_SET_REST_START;
use crate::world::version_1_12::SMSG_SPIRIT_HEALER_CONFIRM;
use crate::world::version_1_12::SMSG_GOSSIP_POI;
use crate::world::version_1_12::SMSG_LOGIN_VERIFY_WORLD;
use crate::world::version_1_12::SMSG_MAIL_LIST_RESULT;
use crate::world::version_1_12::SMSG_BATTLEFIELD_LIST;
use crate::world::version_1_12::SMSG_ITEM_TEXT_QUERY_RESPONSE;
use crate::world::version_1_12::SMSG_SPELLLOGMISS;
use crate::world::version_1_12::SMSG_SPELLLOGEXECUTE;
use crate::world::version_1_12::SMSG_PERIODICAURALOG;
use crate::world::version_1_12::SMSG_SPELLDAMAGESHIELD;
use crate::world::version_1_12::SMSG_SPELLNONMELEEDAMAGELOG;
use crate::world::version_1_12::SMSG_ZONE_UNDER_ATTACK;
use crate::world::version_1_12::MSG_AUCTION_HELLO_Server;
use crate::world::version_1_12::SMSG_AUCTION_LIST_RESULT;
use crate::world::version_1_12::SMSG_AUCTION_OWNER_LIST_RESULT;
use crate::world::version_1_12::SMSG_AUCTION_BIDDER_NOTIFICATION;
use crate::world::version_1_12::SMSG_AUCTION_OWNER_NOTIFICATION;
use crate::world::version_1_12::SMSG_PROCRESIST;
use crate::world::version_1_12::SMSG_DISPEL_FAILED;
use crate::world::version_1_12::SMSG_SPELLORDAMAGE_IMMUNE;
use crate::world::version_1_12::SMSG_AUCTION_BIDDER_LIST_RESULT;
use crate::world::version_1_12::SMSG_SET_FLAT_SPELL_MODIFIER;
use crate::world::version_1_12::SMSG_SET_PCT_SPELL_MODIFIER;
use crate::world::version_1_12::SMSG_CORPSE_RECLAIM_DELAY;
use crate::world::version_1_12::MSG_LIST_STABLED_PETS_Server;
use crate::world::version_1_12::SMSG_STABLE_RESULT;
use crate::world::version_1_12::SMSG_PLAY_MUSIC;
use crate::world::version_1_12::SMSG_PLAY_OBJECT_SOUND;
use crate::world::version_1_12::SMSG_SPELLDISPELLOG;
use crate::world::version_1_12::MSG_QUERY_NEXT_MAIL_TIME_Server;
use crate::world::version_1_12::SMSG_RECEIVED_MAIL;
use crate::world::version_1_12::SMSG_RAID_GROUP_ONLY;
use crate::world::version_1_12::SMSG_PVP_CREDIT;
use crate::world::version_1_12::SMSG_AUCTION_REMOVED_NOTIFICATION;
use crate::world::version_1_12::SMSG_SERVER_MESSAGE;
use crate::world::version_1_12::SMSG_MEETINGSTONE_SETQUEUE;
use crate::world::version_1_12::SMSG_MEETINGSTONE_COMPLETE;
use crate::world::version_1_12::SMSG_MEETINGSTONE_IN_PROGRESS;
use crate::world::version_1_12::SMSG_MEETINGSTONE_MEMBER_ADDED;
use crate::world::version_1_12::SMSG_CANCEL_AUTO_REPEAT;
use crate::world::version_1_12::SMSG_STANDSTATE_UPDATE;
use crate::world::version_1_12::SMSG_LOOT_ALL_PASSED;
use crate::world::version_1_12::SMSG_LOOT_ROLL_WON;
use crate::world::version_1_12::SMSG_LOOT_START_ROLL;
use crate::world::version_1_12::SMSG_LOOT_ROLL;
use crate::world::version_1_12::SMSG_LOOT_MASTER_LIST;
use crate::world::version_1_12::SMSG_SET_FORCED_REACTIONS;
use crate::world::version_1_12::SMSG_SPELL_FAILED_OTHER;
use crate::world::version_1_12::SMSG_GAMEOBJECT_RESET_STATE;
use crate::world::version_1_12::SMSG_CHAT_PLAYER_NOT_FOUND;
use crate::world::version_1_12::MSG_TALENT_WIPE_CONFIRM_Server;
use crate::world::version_1_12::SMSG_SUMMON_REQUEST;
use crate::world::version_1_12::SMSG_MONSTER_MOVE_TRANSPORT;
use crate::world::version_1_12::SMSG_PET_BROKEN;
use crate::world::version_1_12::SMSG_FEIGN_DEATH_RESISTED;
use crate::world::version_1_12::SMSG_DUEL_COUNTDOWN;
use crate::world::version_1_12::SMSG_AREA_TRIGGER_MESSAGE;
use crate::world::version_1_12::SMSG_MEETINGSTONE_JOINFAILED;
use crate::world::version_1_12::SMSG_PLAYER_SKINNED;
use crate::world::version_1_12::SMSG_DURABILITY_DAMAGE_DEATH;
use crate::world::version_1_12::SMSG_INIT_WORLD_STATES;
use crate::world::version_1_12::SMSG_UPDATE_WORLD_STATE;
use crate::world::version_1_12::SMSG_ITEM_NAME_QUERY_RESPONSE;
use crate::world::version_1_12::SMSG_PET_ACTION_FEEDBACK;
use crate::world::version_1_12::SMSG_CHAR_RENAME;
use crate::world::version_1_12::SMSG_INSTANCE_SAVE_CREATED;
use crate::world::version_1_12::SMSG_RAID_INSTANCE_INFO;
use crate::world::version_1_12::SMSG_PLAY_SOUND;
use crate::world::version_1_12::SMSG_BATTLEFIELD_STATUS;
use crate::world::version_1_12::MSG_INSPECT_HONOR_STATS_Server;
use crate::world::version_1_12::SMSG_FORCE_WALK_SPEED_CHANGE;
use crate::world::version_1_12::SMSG_FORCE_SWIM_BACK_SPEED_CHANGE;
use crate::world::version_1_12::SMSG_FORCE_TURN_RATE_CHANGE;
use crate::world::version_1_12::MSG_PVP_LOG_DATA_Server;
use crate::world::version_1_12::SMSG_AREA_SPIRIT_HEALER_TIME;
use crate::world::version_1_12::SMSG_GROUP_JOINED_BATTLEGROUND;
use crate::world::version_1_12::MSG_BATTLEGROUND_PLAYER_POSITIONS_Server;
use crate::world::version_1_12::SMSG_BINDER_CONFIRM;
use crate::world::version_1_12::SMSG_BATTLEGROUND_PLAYER_JOINED;
use crate::world::version_1_12::SMSG_BATTLEGROUND_PLAYER_LEFT;
use crate::world::version_1_12::SMSG_PET_UNLEARN_CONFIRM;
use crate::world::version_1_12::SMSG_PARTY_MEMBER_STATS_FULL;
use crate::world::version_1_12::SMSG_WEATHER;
use crate::world::version_1_12::SMSG_RAID_INSTANCE_MESSAGE;
use crate::world::version_1_12::SMSG_CHAT_RESTRICTED;
use crate::world::version_1_12::SMSG_SPLINE_SET_RUN_SPEED;
use crate::world::version_1_12::SMSG_SPLINE_SET_RUN_BACK_SPEED;
use crate::world::version_1_12::SMSG_SPLINE_SET_SWIM_SPEED;
use crate::world::version_1_12::SMSG_SPLINE_SET_WALK_SPEED;
use crate::world::version_1_12::SMSG_SPLINE_SET_SWIM_BACK_SPEED;
use crate::world::version_1_12::SMSG_SPLINE_SET_TURN_RATE;
use crate::world::version_1_12::SMSG_SPLINE_MOVE_UNROOT;
use crate::world::version_1_12::SMSG_SPLINE_MOVE_FEATHER_FALL;
use crate::world::version_1_12::SMSG_SPLINE_MOVE_NORMAL_FALL;
use crate::world::version_1_12::SMSG_SPLINE_MOVE_SET_HOVER;
use crate::world::version_1_12::SMSG_SPLINE_MOVE_UNSET_HOVER;
use crate::world::version_1_12::SMSG_SPLINE_MOVE_WATER_WALK;
use crate::world::version_1_12::SMSG_SPLINE_MOVE_LAND_WALK;
use crate::world::version_1_12::SMSG_SPLINE_MOVE_START_SWIM;
use crate::world::version_1_12::SMSG_SPLINE_MOVE_STOP_SWIM;
use crate::world::version_1_12::SMSG_SPLINE_MOVE_SET_RUN_MODE;
use crate::world::version_1_12::SMSG_SPLINE_MOVE_SET_WALK_MODE;
use crate::world::version_1_12::SMSG_SPLINE_MOVE_ROOT;
use crate::world::version_1_12::SMSG_INVALIDATE_PLAYER;
use crate::world::version_1_12::SMSG_INSTANCE_RESET;
use crate::world::version_1_12::SMSG_INSTANCE_RESET_FAILED;
use crate::world::version_1_12::SMSG_UPDATE_LAST_INSTANCE;
use crate::world::version_1_12::MSG_RAID_TARGET_UPDATE_Server;
use crate::world::version_1_12::MSG_RAID_READY_CHECK_Server;
use crate::world::version_1_12::SMSG_PET_ACTION_SOUND;
use crate::world::version_1_12::SMSG_PET_DISMISS_SOUND;
use crate::world::version_1_12::SMSG_GM_TICKET_STATUS_UPDATE;
use crate::world::version_1_12::SMSG_UPDATE_INSTANCE_OWNERSHIP;
use crate::world::version_1_12::SMSG_SPELLINSTAKILLLOG;
use crate::world::version_1_12::SMSG_SPELL_UPDATE_CHAIN_TARGETS;
use crate::world::version_1_12::SMSG_EXPECTED_SPAM_RECORDS;
use crate::world::version_1_12::SMSG_DEFENSE_MESSAGE;

#[derive(Debug)]
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
    MSG_MOVE_TELEPORT_ACK(MSG_MOVE_TELEPORT_ACK),
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
    SMSG_FRIEND_LIST(SMSG_FRIEND_LIST),
    SMSG_FRIEND_STATUS(SMSG_FRIEND_STATUS),
    SMSG_IGNORE_LIST(SMSG_IGNORE_LIST),
    SMSG_GROUP_INVITE(SMSG_GROUP_INVITE),
    SMSG_GROUP_DECLINE(SMSG_GROUP_DECLINE),
    SMSG_GROUP_UNINVITE(SMSG_GROUP_UNINVITE),
    SMSG_GROUP_SET_LEADER(SMSG_GROUP_SET_LEADER),
    SMSG_GROUP_DESTROYED(SMSG_GROUP_DESTROYED),
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
    SMSG_MONSTER_MOVE(SMSG_MONSTER_MOVE),
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
    SMSG_AI_REACTION(SMSG_AI_REACTION),
    SMSG_ATTACKSTART(SMSG_ATTACKSTART),
    SMSG_ATTACKSTOP(SMSG_ATTACKSTOP),
    SMSG_ATTACKSWING_NOTINRANGE(SMSG_ATTACKSWING_NOTINRANGE),
    SMSG_ATTACKSWING_BADFACING(SMSG_ATTACKSWING_BADFACING),
    SMSG_ATTACKSWING_NOTSTANDING(SMSG_ATTACKSWING_NOTSTANDING),
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
    SMSG_DISMOUNTRESULT(SMSG_DISMOUNTRESULT),
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
    SMSG_PLAY_SPELL_IMPACT(SMSG_PLAY_SPELL_IMPACT),
    SMSG_EXPLORATION_EXPERIENCE(SMSG_EXPLORATION_EXPERIENCE),
    MSG_RANDOM_ROLL(MSG_RANDOM_ROLL_Server),
    SMSG_ENVIRONMENTALDAMAGELOG(SMSG_ENVIRONMENTALDAMAGELOG),
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
    SMSG_CHAT_WRONG_FACTION(SMSG_CHAT_WRONG_FACTION),
    SMSG_GMTICKET_SYSTEMSTATUS(SMSG_GMTICKET_SYSTEMSTATUS),
    SMSG_SET_REST_START(SMSG_SET_REST_START),
    SMSG_SPIRIT_HEALER_CONFIRM(SMSG_SPIRIT_HEALER_CONFIRM),
    SMSG_GOSSIP_POI(SMSG_GOSSIP_POI),
    SMSG_LOGIN_VERIFY_WORLD(SMSG_LOGIN_VERIFY_WORLD),
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
    SMSG_MEETINGSTONE_COMPLETE(SMSG_MEETINGSTONE_COMPLETE),
    SMSG_MEETINGSTONE_IN_PROGRESS(SMSG_MEETINGSTONE_IN_PROGRESS),
    SMSG_MEETINGSTONE_MEMBER_ADDED(SMSG_MEETINGSTONE_MEMBER_ADDED),
    SMSG_CANCEL_AUTO_REPEAT(SMSG_CANCEL_AUTO_REPEAT),
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
    SMSG_PET_BROKEN(SMSG_PET_BROKEN),
    SMSG_FEIGN_DEATH_RESISTED(SMSG_FEIGN_DEATH_RESISTED),
    SMSG_DUEL_COUNTDOWN(SMSG_DUEL_COUNTDOWN),
    SMSG_AREA_TRIGGER_MESSAGE(SMSG_AREA_TRIGGER_MESSAGE),
    SMSG_MEETINGSTONE_JOINFAILED(SMSG_MEETINGSTONE_JOINFAILED),
    SMSG_PLAYER_SKINNED(SMSG_PLAYER_SKINNED),
    SMSG_DURABILITY_DAMAGE_DEATH(SMSG_DURABILITY_DAMAGE_DEATH),
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
    SMSG_GROUP_JOINED_BATTLEGROUND(SMSG_GROUP_JOINED_BATTLEGROUND),
    MSG_BATTLEGROUND_PLAYER_POSITIONS(MSG_BATTLEGROUND_PLAYER_POSITIONS_Server),
    SMSG_BINDER_CONFIRM(SMSG_BINDER_CONFIRM),
    SMSG_BATTLEGROUND_PLAYER_JOINED(SMSG_BATTLEGROUND_PLAYER_JOINED),
    SMSG_BATTLEGROUND_PLAYER_LEFT(SMSG_BATTLEGROUND_PLAYER_LEFT),
    SMSG_PET_UNLEARN_CONFIRM(SMSG_PET_UNLEARN_CONFIRM),
    SMSG_PARTY_MEMBER_STATS_FULL(SMSG_PARTY_MEMBER_STATS_FULL),
    SMSG_WEATHER(SMSG_WEATHER),
    SMSG_RAID_INSTANCE_MESSAGE(SMSG_RAID_INSTANCE_MESSAGE),
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
            0x00B5 => Ok(Self::MSG_MOVE_START_FORWARD(<MSG_MOVE_START_FORWARD as ServerMessage>::read_body(&mut r, body_size)?)),
            0x00B6 => Ok(Self::MSG_MOVE_START_BACKWARD(<MSG_MOVE_START_BACKWARD as ServerMessage>::read_body(&mut r, body_size)?)),
            0x00B7 => Ok(Self::MSG_MOVE_STOP(<MSG_MOVE_STOP as ServerMessage>::read_body(&mut r, body_size)?)),
            0x00B8 => Ok(Self::MSG_MOVE_START_STRAFE_LEFT(<MSG_MOVE_START_STRAFE_LEFT as ServerMessage>::read_body(&mut r, body_size)?)),
            0x00B9 => Ok(Self::MSG_MOVE_START_STRAFE_RIGHT(<MSG_MOVE_START_STRAFE_RIGHT as ServerMessage>::read_body(&mut r, body_size)?)),
            0x00BA => Ok(Self::MSG_MOVE_STOP_STRAFE(<MSG_MOVE_STOP_STRAFE as ServerMessage>::read_body(&mut r, body_size)?)),
            0x00BB => Ok(Self::MSG_MOVE_JUMP(<MSG_MOVE_JUMP as ServerMessage>::read_body(&mut r, body_size)?)),
            0x00BC => Ok(Self::MSG_MOVE_START_TURN_LEFT(<MSG_MOVE_START_TURN_LEFT as ServerMessage>::read_body(&mut r, body_size)?)),
            0x00BD => Ok(Self::MSG_MOVE_START_TURN_RIGHT(<MSG_MOVE_START_TURN_RIGHT as ServerMessage>::read_body(&mut r, body_size)?)),
            0x00BE => Ok(Self::MSG_MOVE_STOP_TURN(<MSG_MOVE_STOP_TURN as ServerMessage>::read_body(&mut r, body_size)?)),
            0x00BF => Ok(Self::MSG_MOVE_START_PITCH_UP(<MSG_MOVE_START_PITCH_UP as ServerMessage>::read_body(&mut r, body_size)?)),
            0x00C0 => Ok(Self::MSG_MOVE_START_PITCH_DOWN(<MSG_MOVE_START_PITCH_DOWN as ServerMessage>::read_body(&mut r, body_size)?)),
            0x00C1 => Ok(Self::MSG_MOVE_STOP_PITCH(<MSG_MOVE_STOP_PITCH as ServerMessage>::read_body(&mut r, body_size)?)),
            0x00C2 => Ok(Self::MSG_MOVE_SET_RUN_MODE(<MSG_MOVE_SET_RUN_MODE as ServerMessage>::read_body(&mut r, body_size)?)),
            0x00C3 => Ok(Self::MSG_MOVE_SET_WALK_MODE(<MSG_MOVE_SET_WALK_MODE as ServerMessage>::read_body(&mut r, body_size)?)),
            0x00C7 => Ok(Self::MSG_MOVE_TELEPORT_ACK(<MSG_MOVE_TELEPORT_ACK as ServerMessage>::read_body(&mut r, body_size)?)),
            0x00C9 => Ok(Self::MSG_MOVE_FALL_LAND(<MSG_MOVE_FALL_LAND as ServerMessage>::read_body(&mut r, body_size)?)),
            0x00CA => Ok(Self::MSG_MOVE_START_SWIM(<MSG_MOVE_START_SWIM as ServerMessage>::read_body(&mut r, body_size)?)),
            0x00CB => Ok(Self::MSG_MOVE_STOP_SWIM(<MSG_MOVE_STOP_SWIM as ServerMessage>::read_body(&mut r, body_size)?)),
            0x00DA => Ok(Self::MSG_MOVE_SET_FACING(<MSG_MOVE_SET_FACING as ServerMessage>::read_body(&mut r, body_size)?)),
            0x00DB => Ok(Self::MSG_MOVE_SET_PITCH(<MSG_MOVE_SET_PITCH as ServerMessage>::read_body(&mut r, body_size)?)),
            0x00DC => Ok(Self::MSG_MOVE_WORLDPORT_ACK(<MSG_MOVE_WORLDPORT_ACK as ServerMessage>::read_body(&mut r, body_size)?)),
            0x00EE => Ok(Self::MSG_MOVE_HEARTBEAT(<MSG_MOVE_HEARTBEAT as ServerMessage>::read_body(&mut r, body_size)?)),
            0x01C2 => Ok(Self::MSG_PETITION_DECLINE(<MSG_PETITION_DECLINE as ServerMessage>::read_body(&mut r, body_size)?)),
            0x01F2 => Ok(Self::MSG_TABARDVENDOR_ACTIVATE(<MSG_TABARDVENDOR_ACTIVATE as ServerMessage>::read_body(&mut r, body_size)?)),
            0x0276 => Ok(Self::MSG_QUEST_PUSH_RESULT(<MSG_QUEST_PUSH_RESULT as ServerMessage>::read_body(&mut r, body_size)?)),
            0x02C1 => Ok(Self::MSG_PETITION_RENAME(<MSG_PETITION_RENAME as ServerMessage>::read_body(&mut r, body_size)?)),
            0x003A => Ok(Self::SMSG_CHAR_CREATE(<SMSG_CHAR_CREATE as ServerMessage>::read_body(&mut r, body_size)?)),
            0x003B => Ok(Self::SMSG_CHAR_ENUM(<SMSG_CHAR_ENUM as ServerMessage>::read_body(&mut r, body_size)?)),
            0x003C => Ok(Self::SMSG_CHAR_DELETE(<SMSG_CHAR_DELETE as ServerMessage>::read_body(&mut r, body_size)?)),
            0x003E => Ok(Self::SMSG_NEW_WORLD(<SMSG_NEW_WORLD as ServerMessage>::read_body(&mut r, body_size)?)),
            0x003F => Ok(Self::SMSG_TRANSFER_PENDING(<SMSG_TRANSFER_PENDING as ServerMessage>::read_body(&mut r, body_size)?)),
            0x0040 => Ok(Self::SMSG_TRANSFER_ABORTED(<SMSG_TRANSFER_ABORTED as ServerMessage>::read_body(&mut r, body_size)?)),
            0x0041 => Ok(Self::SMSG_CHARACTER_LOGIN_FAILED(<SMSG_CHARACTER_LOGIN_FAILED as ServerMessage>::read_body(&mut r, body_size)?)),
            0x0042 => Ok(Self::SMSG_LOGIN_SETTIMESPEED(<SMSG_LOGIN_SETTIMESPEED as ServerMessage>::read_body(&mut r, body_size)?)),
            0x004C => Ok(Self::SMSG_LOGOUT_RESPONSE(<SMSG_LOGOUT_RESPONSE as ServerMessage>::read_body(&mut r, body_size)?)),
            0x004D => Ok(Self::SMSG_LOGOUT_COMPLETE(<SMSG_LOGOUT_COMPLETE as ServerMessage>::read_body(&mut r, body_size)?)),
            0x004F => Ok(Self::SMSG_LOGOUT_CANCEL_ACK(<SMSG_LOGOUT_CANCEL_ACK as ServerMessage>::read_body(&mut r, body_size)?)),
            0x0051 => Ok(Self::SMSG_NAME_QUERY_RESPONSE(<SMSG_NAME_QUERY_RESPONSE as ServerMessage>::read_body(&mut r, body_size)?)),
            0x0053 => Ok(Self::SMSG_PET_NAME_QUERY_RESPONSE(<SMSG_PET_NAME_QUERY_RESPONSE as ServerMessage>::read_body(&mut r, body_size)?)),
            0x0055 => Ok(Self::SMSG_GUILD_QUERY_RESPONSE(<SMSG_GUILD_QUERY_RESPONSE as ServerMessage>::read_body(&mut r, body_size)?)),
            0x0058 => Ok(Self::SMSG_ITEM_QUERY_SINGLE_RESPONSE(<SMSG_ITEM_QUERY_SINGLE_RESPONSE as ServerMessage>::read_body(&mut r, body_size)?)),
            0x005B => Ok(Self::SMSG_PAGE_TEXT_QUERY_RESPONSE(<SMSG_PAGE_TEXT_QUERY_RESPONSE as ServerMessage>::read_body(&mut r, body_size)?)),
            0x005D => Ok(Self::SMSG_QUEST_QUERY_RESPONSE(<SMSG_QUEST_QUERY_RESPONSE as ServerMessage>::read_body(&mut r, body_size)?)),
            0x005F => Ok(Self::SMSG_GAMEOBJECT_QUERY_RESPONSE(<SMSG_GAMEOBJECT_QUERY_RESPONSE as ServerMessage>::read_body(&mut r, body_size)?)),
            0x0061 => Ok(Self::SMSG_CREATURE_QUERY_RESPONSE(<SMSG_CREATURE_QUERY_RESPONSE as ServerMessage>::read_body(&mut r, body_size)?)),
            0x0063 => Ok(Self::SMSG_WHO(<SMSG_WHO as ServerMessage>::read_body(&mut r, body_size)?)),
            0x0065 => Ok(Self::SMSG_WHOIS(<SMSG_WHOIS as ServerMessage>::read_body(&mut r, body_size)?)),
            0x0067 => Ok(Self::SMSG_FRIEND_LIST(<SMSG_FRIEND_LIST as ServerMessage>::read_body(&mut r, body_size)?)),
            0x0068 => Ok(Self::SMSG_FRIEND_STATUS(<SMSG_FRIEND_STATUS as ServerMessage>::read_body(&mut r, body_size)?)),
            0x006B => Ok(Self::SMSG_IGNORE_LIST(<SMSG_IGNORE_LIST as ServerMessage>::read_body(&mut r, body_size)?)),
            0x006F => Ok(Self::SMSG_GROUP_INVITE(<SMSG_GROUP_INVITE as ServerMessage>::read_body(&mut r, body_size)?)),
            0x0074 => Ok(Self::SMSG_GROUP_DECLINE(<SMSG_GROUP_DECLINE as ServerMessage>::read_body(&mut r, body_size)?)),
            0x0077 => Ok(Self::SMSG_GROUP_UNINVITE(<SMSG_GROUP_UNINVITE as ServerMessage>::read_body(&mut r, body_size)?)),
            0x0079 => Ok(Self::SMSG_GROUP_SET_LEADER(<SMSG_GROUP_SET_LEADER as ServerMessage>::read_body(&mut r, body_size)?)),
            0x007C => Ok(Self::SMSG_GROUP_DESTROYED(<SMSG_GROUP_DESTROYED as ServerMessage>::read_body(&mut r, body_size)?)),
            0x007D => Ok(Self::SMSG_GROUP_LIST(<SMSG_GROUP_LIST as ServerMessage>::read_body(&mut r, body_size)?)),
            0x007E => Ok(Self::SMSG_PARTY_MEMBER_STATS(<SMSG_PARTY_MEMBER_STATS as ServerMessage>::read_body(&mut r, body_size)?)),
            0x007F => Ok(Self::SMSG_PARTY_COMMAND_RESULT(<SMSG_PARTY_COMMAND_RESULT as ServerMessage>::read_body(&mut r, body_size)?)),
            0x0083 => Ok(Self::SMSG_GUILD_INVITE(<SMSG_GUILD_INVITE as ServerMessage>::read_body(&mut r, body_size)?)),
            0x0088 => Ok(Self::SMSG_GUILD_INFO(<SMSG_GUILD_INFO as ServerMessage>::read_body(&mut r, body_size)?)),
            0x008A => Ok(Self::SMSG_GUILD_ROSTER(<SMSG_GUILD_ROSTER as ServerMessage>::read_body(&mut r, body_size)?)),
            0x0092 => Ok(Self::SMSG_GUILD_EVENT(<SMSG_GUILD_EVENT as ServerMessage>::read_body(&mut r, body_size)?)),
            0x0093 => Ok(Self::SMSG_GUILD_COMMAND_RESULT(<SMSG_GUILD_COMMAND_RESULT as ServerMessage>::read_body(&mut r, body_size)?)),
            0x0096 => Ok(Self::SMSG_MESSAGECHAT(<SMSG_MESSAGECHAT as ServerMessage>::read_body(&mut r, body_size)?)),
            0x0099 => Ok(Self::SMSG_CHANNEL_NOTIFY(<SMSG_CHANNEL_NOTIFY as ServerMessage>::read_body(&mut r, body_size)?)),
            0x009B => Ok(Self::SMSG_CHANNEL_LIST(<SMSG_CHANNEL_LIST as ServerMessage>::read_body(&mut r, body_size)?)),
            0x00A9 => Ok(Self::SMSG_UPDATE_OBJECT(<SMSG_UPDATE_OBJECT as ServerMessage>::read_body(&mut r, body_size)?)),
            0x00AA => Ok(Self::SMSG_DESTROY_OBJECT(<SMSG_DESTROY_OBJECT as ServerMessage>::read_body(&mut r, body_size)?)),
            0x00AE => Ok(Self::SMSG_READ_ITEM_OK(<SMSG_READ_ITEM_OK as ServerMessage>::read_body(&mut r, body_size)?)),
            0x00AF => Ok(Self::SMSG_READ_ITEM_FAILED(<SMSG_READ_ITEM_FAILED as ServerMessage>::read_body(&mut r, body_size)?)),
            0x00B0 => Ok(Self::SMSG_ITEM_COOLDOWN(<SMSG_ITEM_COOLDOWN as ServerMessage>::read_body(&mut r, body_size)?)),
            0x00B3 => Ok(Self::SMSG_GAMEOBJECT_CUSTOM_ANIM(<SMSG_GAMEOBJECT_CUSTOM_ANIM as ServerMessage>::read_body(&mut r, body_size)?)),
            0x00DD => Ok(Self::SMSG_MONSTER_MOVE(<SMSG_MONSTER_MOVE as ServerMessage>::read_body(&mut r, body_size)?)),
            0x00DE => Ok(Self::SMSG_MOVE_WATER_WALK(<SMSG_MOVE_WATER_WALK as ServerMessage>::read_body(&mut r, body_size)?)),
            0x00DF => Ok(Self::SMSG_MOVE_LAND_WALK(<SMSG_MOVE_LAND_WALK as ServerMessage>::read_body(&mut r, body_size)?)),
            0x00E2 => Ok(Self::SMSG_FORCE_RUN_SPEED_CHANGE(<SMSG_FORCE_RUN_SPEED_CHANGE as ServerMessage>::read_body(&mut r, body_size)?)),
            0x00E4 => Ok(Self::SMSG_FORCE_RUN_BACK_SPEED_CHANGE(<SMSG_FORCE_RUN_BACK_SPEED_CHANGE as ServerMessage>::read_body(&mut r, body_size)?)),
            0x00E6 => Ok(Self::SMSG_FORCE_SWIM_SPEED_CHANGE(<SMSG_FORCE_SWIM_SPEED_CHANGE as ServerMessage>::read_body(&mut r, body_size)?)),
            0x00E8 => Ok(Self::SMSG_FORCE_MOVE_ROOT(<SMSG_FORCE_MOVE_ROOT as ServerMessage>::read_body(&mut r, body_size)?)),
            0x00EA => Ok(Self::SMSG_FORCE_MOVE_UNROOT(<SMSG_FORCE_MOVE_UNROOT as ServerMessage>::read_body(&mut r, body_size)?)),
            0x00EF => Ok(Self::SMSG_MOVE_KNOCK_BACK(<SMSG_MOVE_KNOCK_BACK as ServerMessage>::read_body(&mut r, body_size)?)),
            0x00F2 => Ok(Self::SMSG_MOVE_FEATHER_FALL(<SMSG_MOVE_FEATHER_FALL as ServerMessage>::read_body(&mut r, body_size)?)),
            0x00F3 => Ok(Self::SMSG_MOVE_NORMAL_FALL(<SMSG_MOVE_NORMAL_FALL as ServerMessage>::read_body(&mut r, body_size)?)),
            0x00F4 => Ok(Self::SMSG_MOVE_SET_HOVER(<SMSG_MOVE_SET_HOVER as ServerMessage>::read_body(&mut r, body_size)?)),
            0x00F5 => Ok(Self::SMSG_MOVE_UNSET_HOVER(<SMSG_MOVE_UNSET_HOVER as ServerMessage>::read_body(&mut r, body_size)?)),
            0x00FA => Ok(Self::SMSG_TRIGGER_CINEMATIC(<SMSG_TRIGGER_CINEMATIC as ServerMessage>::read_body(&mut r, body_size)?)),
            0x00FD => Ok(Self::SMSG_TUTORIAL_FLAGS(<SMSG_TUTORIAL_FLAGS as ServerMessage>::read_body(&mut r, body_size)?)),
            0x0103 => Ok(Self::SMSG_EMOTE(<SMSG_EMOTE as ServerMessage>::read_body(&mut r, body_size)?)),
            0x0105 => Ok(Self::SMSG_TEXT_EMOTE(<SMSG_TEXT_EMOTE as ServerMessage>::read_body(&mut r, body_size)?)),
            0x0112 => Ok(Self::SMSG_INVENTORY_CHANGE_FAILURE(<SMSG_INVENTORY_CHANGE_FAILURE as ServerMessage>::read_body(&mut r, body_size)?)),
            0x0113 => Ok(Self::SMSG_OPEN_CONTAINER(<SMSG_OPEN_CONTAINER as ServerMessage>::read_body(&mut r, body_size)?)),
            0x0115 => Ok(Self::SMSG_INSPECT(<SMSG_INSPECT as ServerMessage>::read_body(&mut r, body_size)?)),
            0x0120 => Ok(Self::SMSG_TRADE_STATUS(<SMSG_TRADE_STATUS as ServerMessage>::read_body(&mut r, body_size)?)),
            0x0121 => Ok(Self::SMSG_TRADE_STATUS_EXTENDED(<SMSG_TRADE_STATUS_EXTENDED as ServerMessage>::read_body(&mut r, body_size)?)),
            0x0122 => Ok(Self::SMSG_INITIALIZE_FACTIONS(<SMSG_INITIALIZE_FACTIONS as ServerMessage>::read_body(&mut r, body_size)?)),
            0x0123 => Ok(Self::SMSG_SET_FACTION_VISIBLE(<SMSG_SET_FACTION_VISIBLE as ServerMessage>::read_body(&mut r, body_size)?)),
            0x0124 => Ok(Self::SMSG_SET_FACTION_STANDING(<SMSG_SET_FACTION_STANDING as ServerMessage>::read_body(&mut r, body_size)?)),
            0x0127 => Ok(Self::SMSG_SET_PROFICIENCY(<SMSG_SET_PROFICIENCY as ServerMessage>::read_body(&mut r, body_size)?)),
            0x0129 => Ok(Self::SMSG_ACTION_BUTTONS(<SMSG_ACTION_BUTTONS as ServerMessage>::read_body(&mut r, body_size)?)),
            0x012A => Ok(Self::SMSG_INITIAL_SPELLS(<SMSG_INITIAL_SPELLS as ServerMessage>::read_body(&mut r, body_size)?)),
            0x012B => Ok(Self::SMSG_LEARNED_SPELL(<SMSG_LEARNED_SPELL as ServerMessage>::read_body(&mut r, body_size)?)),
            0x012C => Ok(Self::SMSG_SUPERCEDED_SPELL(<SMSG_SUPERCEDED_SPELL as ServerMessage>::read_body(&mut r, body_size)?)),
            0x0130 => Ok(Self::SMSG_CAST_RESULT(<SMSG_CAST_RESULT as ServerMessage>::read_body(&mut r, body_size)?)),
            0x0131 => Ok(Self::SMSG_SPELL_START(<SMSG_SPELL_START as ServerMessage>::read_body(&mut r, body_size)?)),
            0x0132 => Ok(Self::SMSG_SPELL_GO(<SMSG_SPELL_GO as ServerMessage>::read_body(&mut r, body_size)?)),
            0x0133 => Ok(Self::SMSG_SPELL_FAILURE(<SMSG_SPELL_FAILURE as ServerMessage>::read_body(&mut r, body_size)?)),
            0x0134 => Ok(Self::SMSG_SPELL_COOLDOWN(<SMSG_SPELL_COOLDOWN as ServerMessage>::read_body(&mut r, body_size)?)),
            0x0135 => Ok(Self::SMSG_COOLDOWN_EVENT(<SMSG_COOLDOWN_EVENT as ServerMessage>::read_body(&mut r, body_size)?)),
            0x0137 => Ok(Self::SMSG_UPDATE_AURA_DURATION(<SMSG_UPDATE_AURA_DURATION as ServerMessage>::read_body(&mut r, body_size)?)),
            0x0138 => Ok(Self::SMSG_PET_CAST_FAILED(<SMSG_PET_CAST_FAILED as ServerMessage>::read_body(&mut r, body_size)?)),
            0x013C => Ok(Self::SMSG_AI_REACTION(<SMSG_AI_REACTION as ServerMessage>::read_body(&mut r, body_size)?)),
            0x0143 => Ok(Self::SMSG_ATTACKSTART(<SMSG_ATTACKSTART as ServerMessage>::read_body(&mut r, body_size)?)),
            0x0144 => Ok(Self::SMSG_ATTACKSTOP(<SMSG_ATTACKSTOP as ServerMessage>::read_body(&mut r, body_size)?)),
            0x0145 => Ok(Self::SMSG_ATTACKSWING_NOTINRANGE(<SMSG_ATTACKSWING_NOTINRANGE as ServerMessage>::read_body(&mut r, body_size)?)),
            0x0146 => Ok(Self::SMSG_ATTACKSWING_BADFACING(<SMSG_ATTACKSWING_BADFACING as ServerMessage>::read_body(&mut r, body_size)?)),
            0x0147 => Ok(Self::SMSG_ATTACKSWING_NOTSTANDING(<SMSG_ATTACKSWING_NOTSTANDING as ServerMessage>::read_body(&mut r, body_size)?)),
            0x0148 => Ok(Self::SMSG_ATTACKSWING_DEADTARGET(<SMSG_ATTACKSWING_DEADTARGET as ServerMessage>::read_body(&mut r, body_size)?)),
            0x0149 => Ok(Self::SMSG_ATTACKSWING_CANT_ATTACK(<SMSG_ATTACKSWING_CANT_ATTACK as ServerMessage>::read_body(&mut r, body_size)?)),
            0x014A => Ok(Self::SMSG_ATTACKERSTATEUPDATE(<SMSG_ATTACKERSTATEUPDATE as ServerMessage>::read_body(&mut r, body_size)?)),
            0x014E => Ok(Self::SMSG_CANCEL_COMBAT(<SMSG_CANCEL_COMBAT as ServerMessage>::read_body(&mut r, body_size)?)),
            0x0150 => Ok(Self::SMSG_SPELLHEALLOG(<SMSG_SPELLHEALLOG as ServerMessage>::read_body(&mut r, body_size)?)),
            0x0151 => Ok(Self::SMSG_SPELLENERGIZELOG(<SMSG_SPELLENERGIZELOG as ServerMessage>::read_body(&mut r, body_size)?)),
            0x0155 => Ok(Self::SMSG_BINDPOINTUPDATE(<SMSG_BINDPOINTUPDATE as ServerMessage>::read_body(&mut r, body_size)?)),
            0x0158 => Ok(Self::SMSG_PLAYERBOUND(<SMSG_PLAYERBOUND as ServerMessage>::read_body(&mut r, body_size)?)),
            0x0159 => Ok(Self::SMSG_CLIENT_CONTROL_UPDATE(<SMSG_CLIENT_CONTROL_UPDATE as ServerMessage>::read_body(&mut r, body_size)?)),
            0x015B => Ok(Self::SMSG_RESURRECT_REQUEST(<SMSG_RESURRECT_REQUEST as ServerMessage>::read_body(&mut r, body_size)?)),
            0x0160 => Ok(Self::SMSG_LOOT_RESPONSE(<SMSG_LOOT_RESPONSE as ServerMessage>::read_body(&mut r, body_size)?)),
            0x0161 => Ok(Self::SMSG_LOOT_RELEASE_RESPONSE(<SMSG_LOOT_RELEASE_RESPONSE as ServerMessage>::read_body(&mut r, body_size)?)),
            0x0162 => Ok(Self::SMSG_LOOT_REMOVED(<SMSG_LOOT_REMOVED as ServerMessage>::read_body(&mut r, body_size)?)),
            0x0163 => Ok(Self::SMSG_LOOT_MONEY_NOTIFY(<SMSG_LOOT_MONEY_NOTIFY as ServerMessage>::read_body(&mut r, body_size)?)),
            0x0165 => Ok(Self::SMSG_LOOT_CLEAR_MONEY(<SMSG_LOOT_CLEAR_MONEY as ServerMessage>::read_body(&mut r, body_size)?)),
            0x0166 => Ok(Self::SMSG_ITEM_PUSH_RESULT(<SMSG_ITEM_PUSH_RESULT as ServerMessage>::read_body(&mut r, body_size)?)),
            0x0167 => Ok(Self::SMSG_DUEL_REQUESTED(<SMSG_DUEL_REQUESTED as ServerMessage>::read_body(&mut r, body_size)?)),
            0x0168 => Ok(Self::SMSG_DUEL_OUTOFBOUNDS(<SMSG_DUEL_OUTOFBOUNDS as ServerMessage>::read_body(&mut r, body_size)?)),
            0x0169 => Ok(Self::SMSG_DUEL_INBOUNDS(<SMSG_DUEL_INBOUNDS as ServerMessage>::read_body(&mut r, body_size)?)),
            0x016A => Ok(Self::SMSG_DUEL_COMPLETE(<SMSG_DUEL_COMPLETE as ServerMessage>::read_body(&mut r, body_size)?)),
            0x016B => Ok(Self::SMSG_DUEL_WINNER(<SMSG_DUEL_WINNER as ServerMessage>::read_body(&mut r, body_size)?)),
            0x016E => Ok(Self::SMSG_MOUNTRESULT(<SMSG_MOUNTRESULT as ServerMessage>::read_body(&mut r, body_size)?)),
            0x016F => Ok(Self::SMSG_DISMOUNTRESULT(<SMSG_DISMOUNTRESULT as ServerMessage>::read_body(&mut r, body_size)?)),
            0x0172 => Ok(Self::SMSG_MOUNTSPECIAL_ANIM(<SMSG_MOUNTSPECIAL_ANIM as ServerMessage>::read_body(&mut r, body_size)?)),
            0x0173 => Ok(Self::SMSG_PET_TAME_FAILURE(<SMSG_PET_TAME_FAILURE as ServerMessage>::read_body(&mut r, body_size)?)),
            0x0178 => Ok(Self::SMSG_PET_NAME_INVALID(<SMSG_PET_NAME_INVALID as ServerMessage>::read_body(&mut r, body_size)?)),
            0x0179 => Ok(Self::SMSG_PET_SPELLS(<SMSG_PET_SPELLS as ServerMessage>::read_body(&mut r, body_size)?)),
            0x017A => Ok(Self::SMSG_PET_MODE(<SMSG_PET_MODE as ServerMessage>::read_body(&mut r, body_size)?)),
            0x017D => Ok(Self::SMSG_GOSSIP_MESSAGE(<SMSG_GOSSIP_MESSAGE as ServerMessage>::read_body(&mut r, body_size)?)),
            0x017E => Ok(Self::SMSG_GOSSIP_COMPLETE(<SMSG_GOSSIP_COMPLETE as ServerMessage>::read_body(&mut r, body_size)?)),
            0x0180 => Ok(Self::SMSG_NPC_TEXT_UPDATE(<SMSG_NPC_TEXT_UPDATE as ServerMessage>::read_body(&mut r, body_size)?)),
            0x0183 => Ok(Self::SMSG_QUESTGIVER_STATUS(<SMSG_QUESTGIVER_STATUS as ServerMessage>::read_body(&mut r, body_size)?)),
            0x0185 => Ok(Self::SMSG_QUESTGIVER_QUEST_LIST(<SMSG_QUESTGIVER_QUEST_LIST as ServerMessage>::read_body(&mut r, body_size)?)),
            0x0188 => Ok(Self::SMSG_QUESTGIVER_QUEST_DETAILS(<SMSG_QUESTGIVER_QUEST_DETAILS as ServerMessage>::read_body(&mut r, body_size)?)),
            0x018B => Ok(Self::SMSG_QUESTGIVER_REQUEST_ITEMS(<SMSG_QUESTGIVER_REQUEST_ITEMS as ServerMessage>::read_body(&mut r, body_size)?)),
            0x018D => Ok(Self::SMSG_QUESTGIVER_OFFER_REWARD(<SMSG_QUESTGIVER_OFFER_REWARD as ServerMessage>::read_body(&mut r, body_size)?)),
            0x018F => Ok(Self::SMSG_QUESTGIVER_QUEST_INVALID(<SMSG_QUESTGIVER_QUEST_INVALID as ServerMessage>::read_body(&mut r, body_size)?)),
            0x0191 => Ok(Self::SMSG_QUESTGIVER_QUEST_COMPLETE(<SMSG_QUESTGIVER_QUEST_COMPLETE as ServerMessage>::read_body(&mut r, body_size)?)),
            0x0192 => Ok(Self::SMSG_QUESTGIVER_QUEST_FAILED(<SMSG_QUESTGIVER_QUEST_FAILED as ServerMessage>::read_body(&mut r, body_size)?)),
            0x0195 => Ok(Self::SMSG_QUESTLOG_FULL(<SMSG_QUESTLOG_FULL as ServerMessage>::read_body(&mut r, body_size)?)),
            0x0196 => Ok(Self::SMSG_QUESTUPDATE_FAILED(<SMSG_QUESTUPDATE_FAILED as ServerMessage>::read_body(&mut r, body_size)?)),
            0x0197 => Ok(Self::SMSG_QUESTUPDATE_FAILEDTIMER(<SMSG_QUESTUPDATE_FAILEDTIMER as ServerMessage>::read_body(&mut r, body_size)?)),
            0x0198 => Ok(Self::SMSG_QUESTUPDATE_COMPLETE(<SMSG_QUESTUPDATE_COMPLETE as ServerMessage>::read_body(&mut r, body_size)?)),
            0x0199 => Ok(Self::SMSG_QUESTUPDATE_ADD_KILL(<SMSG_QUESTUPDATE_ADD_KILL as ServerMessage>::read_body(&mut r, body_size)?)),
            0x019A => Ok(Self::SMSG_QUESTUPDATE_ADD_ITEM(<SMSG_QUESTUPDATE_ADD_ITEM as ServerMessage>::read_body(&mut r, body_size)?)),
            0x019C => Ok(Self::SMSG_QUEST_CONFIRM_ACCEPT(<SMSG_QUEST_CONFIRM_ACCEPT as ServerMessage>::read_body(&mut r, body_size)?)),
            0x019F => Ok(Self::SMSG_LIST_INVENTORY(<SMSG_LIST_INVENTORY as ServerMessage>::read_body(&mut r, body_size)?)),
            0x01A1 => Ok(Self::SMSG_SELL_ITEM(<SMSG_SELL_ITEM as ServerMessage>::read_body(&mut r, body_size)?)),
            0x01A4 => Ok(Self::SMSG_BUY_ITEM(<SMSG_BUY_ITEM as ServerMessage>::read_body(&mut r, body_size)?)),
            0x01A5 => Ok(Self::SMSG_BUY_FAILED(<SMSG_BUY_FAILED as ServerMessage>::read_body(&mut r, body_size)?)),
            0x01A9 => Ok(Self::SMSG_SHOWTAXINODES(<SMSG_SHOWTAXINODES as ServerMessage>::read_body(&mut r, body_size)?)),
            0x01AB => Ok(Self::SMSG_TAXINODE_STATUS(<SMSG_TAXINODE_STATUS as ServerMessage>::read_body(&mut r, body_size)?)),
            0x01AE => Ok(Self::SMSG_ACTIVATETAXIREPLY(<SMSG_ACTIVATETAXIREPLY as ServerMessage>::read_body(&mut r, body_size)?)),
            0x01AF => Ok(Self::SMSG_NEW_TAXI_PATH(<SMSG_NEW_TAXI_PATH as ServerMessage>::read_body(&mut r, body_size)?)),
            0x01B1 => Ok(Self::SMSG_TRAINER_LIST(<SMSG_TRAINER_LIST as ServerMessage>::read_body(&mut r, body_size)?)),
            0x01B3 => Ok(Self::SMSG_TRAINER_BUY_SUCCEEDED(<SMSG_TRAINER_BUY_SUCCEEDED as ServerMessage>::read_body(&mut r, body_size)?)),
            0x01B4 => Ok(Self::SMSG_TRAINER_BUY_FAILED(<SMSG_TRAINER_BUY_FAILED as ServerMessage>::read_body(&mut r, body_size)?)),
            0x01B8 => Ok(Self::SMSG_SHOW_BANK(<SMSG_SHOW_BANK as ServerMessage>::read_body(&mut r, body_size)?)),
            0x01BA => Ok(Self::SMSG_BUY_BANK_SLOT_RESULT(<SMSG_BUY_BANK_SLOT_RESULT as ServerMessage>::read_body(&mut r, body_size)?)),
            0x01BC => Ok(Self::SMSG_PETITION_SHOWLIST(<SMSG_PETITION_SHOWLIST as ServerMessage>::read_body(&mut r, body_size)?)),
            0x01BF => Ok(Self::SMSG_PETITION_SHOW_SIGNATURES(<SMSG_PETITION_SHOW_SIGNATURES as ServerMessage>::read_body(&mut r, body_size)?)),
            0x01C1 => Ok(Self::SMSG_PETITION_SIGN_RESULTS(<SMSG_PETITION_SIGN_RESULTS as ServerMessage>::read_body(&mut r, body_size)?)),
            0x01C5 => Ok(Self::SMSG_TURN_IN_PETITION_RESULTS(<SMSG_TURN_IN_PETITION_RESULTS as ServerMessage>::read_body(&mut r, body_size)?)),
            0x01C7 => Ok(Self::SMSG_PETITION_QUERY_RESPONSE(<SMSG_PETITION_QUERY_RESPONSE as ServerMessage>::read_body(&mut r, body_size)?)),
            0x01C8 => Ok(Self::SMSG_FISH_NOT_HOOKED(<SMSG_FISH_NOT_HOOKED as ServerMessage>::read_body(&mut r, body_size)?)),
            0x01C9 => Ok(Self::SMSG_FISH_ESCAPED(<SMSG_FISH_ESCAPED as ServerMessage>::read_body(&mut r, body_size)?)),
            0x01CB => Ok(Self::SMSG_NOTIFICATION(<SMSG_NOTIFICATION as ServerMessage>::read_body(&mut r, body_size)?)),
            0x01CD => Ok(Self::SMSG_PLAYED_TIME(<SMSG_PLAYED_TIME as ServerMessage>::read_body(&mut r, body_size)?)),
            0x01CF => Ok(Self::SMSG_QUERY_TIME_RESPONSE(<SMSG_QUERY_TIME_RESPONSE as ServerMessage>::read_body(&mut r, body_size)?)),
            0x01D0 => Ok(Self::SMSG_LOG_XPGAIN(<SMSG_LOG_XPGAIN as ServerMessage>::read_body(&mut r, body_size)?)),
            0x01D4 => Ok(Self::SMSG_LEVELUP_INFO(<SMSG_LEVELUP_INFO as ServerMessage>::read_body(&mut r, body_size)?)),
            0x01D5 => Ok(Self::MSG_MINIMAP_PING(<MSG_MINIMAP_PING_Server as ServerMessage>::read_body(&mut r, body_size)?)),
            0x01D6 => Ok(Self::SMSG_RESISTLOG(<SMSG_RESISTLOG as ServerMessage>::read_body(&mut r, body_size)?)),
            0x01D7 => Ok(Self::SMSG_ENCHANTMENTLOG(<SMSG_ENCHANTMENTLOG as ServerMessage>::read_body(&mut r, body_size)?)),
            0x01D9 => Ok(Self::SMSG_START_MIRROR_TIMER(<SMSG_START_MIRROR_TIMER as ServerMessage>::read_body(&mut r, body_size)?)),
            0x01DA => Ok(Self::SMSG_PAUSE_MIRROR_TIMER(<SMSG_PAUSE_MIRROR_TIMER as ServerMessage>::read_body(&mut r, body_size)?)),
            0x01DB => Ok(Self::SMSG_STOP_MIRROR_TIMER(<SMSG_STOP_MIRROR_TIMER as ServerMessage>::read_body(&mut r, body_size)?)),
            0x01DD => Ok(Self::SMSG_PONG(<SMSG_PONG as ServerMessage>::read_body(&mut r, body_size)?)),
            0x01DE => Ok(Self::SMSG_CLEAR_COOLDOWN(<SMSG_CLEAR_COOLDOWN as ServerMessage>::read_body(&mut r, body_size)?)),
            0x01DF => Ok(Self::SMSG_GAMEOBJECT_PAGETEXT(<SMSG_GAMEOBJECT_PAGETEXT as ServerMessage>::read_body(&mut r, body_size)?)),
            0x01E2 => Ok(Self::SMSG_SPELL_DELAYED(<SMSG_SPELL_DELAYED as ServerMessage>::read_body(&mut r, body_size)?)),
            0x01EA => Ok(Self::SMSG_ITEM_TIME_UPDATE(<SMSG_ITEM_TIME_UPDATE as ServerMessage>::read_body(&mut r, body_size)?)),
            0x01EB => Ok(Self::SMSG_ITEM_ENCHANT_TIME_UPDATE(<SMSG_ITEM_ENCHANT_TIME_UPDATE as ServerMessage>::read_body(&mut r, body_size)?)),
            0x01EC => Ok(Self::SMSG_AUTH_CHALLENGE(<SMSG_AUTH_CHALLENGE as ServerMessage>::read_body(&mut r, body_size)?)),
            0x01EE => Ok(Self::SMSG_AUTH_RESPONSE(<SMSG_AUTH_RESPONSE as ServerMessage>::read_body(&mut r, body_size)?)),
            0x01F1 => Ok(Self::MSG_SAVE_GUILD_EMBLEM(<MSG_SAVE_GUILD_EMBLEM_Server as ServerMessage>::read_body(&mut r, body_size)?)),
            0x01F3 => Ok(Self::SMSG_PLAY_SPELL_VISUAL(<SMSG_PLAY_SPELL_VISUAL as ServerMessage>::read_body(&mut r, body_size)?)),
            0x01F5 => Ok(Self::SMSG_PARTYKILLLOG(<SMSG_PARTYKILLLOG as ServerMessage>::read_body(&mut r, body_size)?)),
            0x01F7 => Ok(Self::SMSG_PLAY_SPELL_IMPACT(<SMSG_PLAY_SPELL_IMPACT as ServerMessage>::read_body(&mut r, body_size)?)),
            0x01F8 => Ok(Self::SMSG_EXPLORATION_EXPERIENCE(<SMSG_EXPLORATION_EXPERIENCE as ServerMessage>::read_body(&mut r, body_size)?)),
            0x01FB => Ok(Self::MSG_RANDOM_ROLL(<MSG_RANDOM_ROLL_Server as ServerMessage>::read_body(&mut r, body_size)?)),
            0x01FC => Ok(Self::SMSG_ENVIRONMENTALDAMAGELOG(<SMSG_ENVIRONMENTALDAMAGELOG as ServerMessage>::read_body(&mut r, body_size)?)),
            0x01FF => Ok(Self::MSG_LOOKING_FOR_GROUP(<MSG_LOOKING_FOR_GROUP_Server as ServerMessage>::read_body(&mut r, body_size)?)),
            0x0203 => Ok(Self::SMSG_REMOVED_SPELL(<SMSG_REMOVED_SPELL as ServerMessage>::read_body(&mut r, body_size)?)),
            0x0206 => Ok(Self::SMSG_GMTICKET_CREATE(<SMSG_GMTICKET_CREATE as ServerMessage>::read_body(&mut r, body_size)?)),
            0x0208 => Ok(Self::SMSG_GMTICKET_UPDATETEXT(<SMSG_GMTICKET_UPDATETEXT as ServerMessage>::read_body(&mut r, body_size)?)),
            0x0209 => Ok(Self::SMSG_ACCOUNT_DATA_TIMES(<SMSG_ACCOUNT_DATA_TIMES as ServerMessage>::read_body(&mut r, body_size)?)),
            0x0212 => Ok(Self::SMSG_GMTICKET_GETTICKET(<SMSG_GMTICKET_GETTICKET as ServerMessage>::read_body(&mut r, body_size)?)),
            0x0214 => Ok(Self::SMSG_GAMEOBJECT_SPAWN_ANIM(<SMSG_GAMEOBJECT_SPAWN_ANIM as ServerMessage>::read_body(&mut r, body_size)?)),
            0x0215 => Ok(Self::SMSG_GAMEOBJECT_DESPAWN_ANIM(<SMSG_GAMEOBJECT_DESPAWN_ANIM as ServerMessage>::read_body(&mut r, body_size)?)),
            0x0216 => Ok(Self::MSG_CORPSE_QUERY(<MSG_CORPSE_QUERY_Server as ServerMessage>::read_body(&mut r, body_size)?)),
            0x0218 => Ok(Self::SMSG_GMTICKET_DELETETICKET(<SMSG_GMTICKET_DELETETICKET as ServerMessage>::read_body(&mut r, body_size)?)),
            0x0219 => Ok(Self::SMSG_CHAT_WRONG_FACTION(<SMSG_CHAT_WRONG_FACTION as ServerMessage>::read_body(&mut r, body_size)?)),
            0x021B => Ok(Self::SMSG_GMTICKET_SYSTEMSTATUS(<SMSG_GMTICKET_SYSTEMSTATUS as ServerMessage>::read_body(&mut r, body_size)?)),
            0x021E => Ok(Self::SMSG_SET_REST_START(<SMSG_SET_REST_START as ServerMessage>::read_body(&mut r, body_size)?)),
            0x0222 => Ok(Self::SMSG_SPIRIT_HEALER_CONFIRM(<SMSG_SPIRIT_HEALER_CONFIRM as ServerMessage>::read_body(&mut r, body_size)?)),
            0x0224 => Ok(Self::SMSG_GOSSIP_POI(<SMSG_GOSSIP_POI as ServerMessage>::read_body(&mut r, body_size)?)),
            0x0236 => Ok(Self::SMSG_LOGIN_VERIFY_WORLD(<SMSG_LOGIN_VERIFY_WORLD as ServerMessage>::read_body(&mut r, body_size)?)),
            0x023B => Ok(Self::SMSG_MAIL_LIST_RESULT(<SMSG_MAIL_LIST_RESULT as ServerMessage>::read_body(&mut r, body_size)?)),
            0x023D => Ok(Self::SMSG_BATTLEFIELD_LIST(<SMSG_BATTLEFIELD_LIST as ServerMessage>::read_body(&mut r, body_size)?)),
            0x0244 => Ok(Self::SMSG_ITEM_TEXT_QUERY_RESPONSE(<SMSG_ITEM_TEXT_QUERY_RESPONSE as ServerMessage>::read_body(&mut r, body_size)?)),
            0x024B => Ok(Self::SMSG_SPELLLOGMISS(<SMSG_SPELLLOGMISS as ServerMessage>::read_body(&mut r, body_size)?)),
            0x024C => Ok(Self::SMSG_SPELLLOGEXECUTE(<SMSG_SPELLLOGEXECUTE as ServerMessage>::read_body(&mut r, body_size)?)),
            0x024E => Ok(Self::SMSG_PERIODICAURALOG(<SMSG_PERIODICAURALOG as ServerMessage>::read_body(&mut r, body_size)?)),
            0x024F => Ok(Self::SMSG_SPELLDAMAGESHIELD(<SMSG_SPELLDAMAGESHIELD as ServerMessage>::read_body(&mut r, body_size)?)),
            0x0250 => Ok(Self::SMSG_SPELLNONMELEEDAMAGELOG(<SMSG_SPELLNONMELEEDAMAGELOG as ServerMessage>::read_body(&mut r, body_size)?)),
            0x0254 => Ok(Self::SMSG_ZONE_UNDER_ATTACK(<SMSG_ZONE_UNDER_ATTACK as ServerMessage>::read_body(&mut r, body_size)?)),
            0x0255 => Ok(Self::MSG_AUCTION_HELLO(<MSG_AUCTION_HELLO_Server as ServerMessage>::read_body(&mut r, body_size)?)),
            0x025C => Ok(Self::SMSG_AUCTION_LIST_RESULT(<SMSG_AUCTION_LIST_RESULT as ServerMessage>::read_body(&mut r, body_size)?)),
            0x025D => Ok(Self::SMSG_AUCTION_OWNER_LIST_RESULT(<SMSG_AUCTION_OWNER_LIST_RESULT as ServerMessage>::read_body(&mut r, body_size)?)),
            0x025E => Ok(Self::SMSG_AUCTION_BIDDER_NOTIFICATION(<SMSG_AUCTION_BIDDER_NOTIFICATION as ServerMessage>::read_body(&mut r, body_size)?)),
            0x025F => Ok(Self::SMSG_AUCTION_OWNER_NOTIFICATION(<SMSG_AUCTION_OWNER_NOTIFICATION as ServerMessage>::read_body(&mut r, body_size)?)),
            0x0260 => Ok(Self::SMSG_PROCRESIST(<SMSG_PROCRESIST as ServerMessage>::read_body(&mut r, body_size)?)),
            0x0262 => Ok(Self::SMSG_DISPEL_FAILED(<SMSG_DISPEL_FAILED as ServerMessage>::read_body(&mut r, body_size)?)),
            0x0263 => Ok(Self::SMSG_SPELLORDAMAGE_IMMUNE(<SMSG_SPELLORDAMAGE_IMMUNE as ServerMessage>::read_body(&mut r, body_size)?)),
            0x0265 => Ok(Self::SMSG_AUCTION_BIDDER_LIST_RESULT(<SMSG_AUCTION_BIDDER_LIST_RESULT as ServerMessage>::read_body(&mut r, body_size)?)),
            0x0266 => Ok(Self::SMSG_SET_FLAT_SPELL_MODIFIER(<SMSG_SET_FLAT_SPELL_MODIFIER as ServerMessage>::read_body(&mut r, body_size)?)),
            0x0267 => Ok(Self::SMSG_SET_PCT_SPELL_MODIFIER(<SMSG_SET_PCT_SPELL_MODIFIER as ServerMessage>::read_body(&mut r, body_size)?)),
            0x0269 => Ok(Self::SMSG_CORPSE_RECLAIM_DELAY(<SMSG_CORPSE_RECLAIM_DELAY as ServerMessage>::read_body(&mut r, body_size)?)),
            0x026F => Ok(Self::MSG_LIST_STABLED_PETS(<MSG_LIST_STABLED_PETS_Server as ServerMessage>::read_body(&mut r, body_size)?)),
            0x0273 => Ok(Self::SMSG_STABLE_RESULT(<SMSG_STABLE_RESULT as ServerMessage>::read_body(&mut r, body_size)?)),
            0x0277 => Ok(Self::SMSG_PLAY_MUSIC(<SMSG_PLAY_MUSIC as ServerMessage>::read_body(&mut r, body_size)?)),
            0x0278 => Ok(Self::SMSG_PLAY_OBJECT_SOUND(<SMSG_PLAY_OBJECT_SOUND as ServerMessage>::read_body(&mut r, body_size)?)),
            0x027B => Ok(Self::SMSG_SPELLDISPELLOG(<SMSG_SPELLDISPELLOG as ServerMessage>::read_body(&mut r, body_size)?)),
            0x0284 => Ok(Self::MSG_QUERY_NEXT_MAIL_TIME(<MSG_QUERY_NEXT_MAIL_TIME_Server as ServerMessage>::read_body(&mut r, body_size)?)),
            0x0285 => Ok(Self::SMSG_RECEIVED_MAIL(<SMSG_RECEIVED_MAIL as ServerMessage>::read_body(&mut r, body_size)?)),
            0x0286 => Ok(Self::SMSG_RAID_GROUP_ONLY(<SMSG_RAID_GROUP_ONLY as ServerMessage>::read_body(&mut r, body_size)?)),
            0x028C => Ok(Self::SMSG_PVP_CREDIT(<SMSG_PVP_CREDIT as ServerMessage>::read_body(&mut r, body_size)?)),
            0x028D => Ok(Self::SMSG_AUCTION_REMOVED_NOTIFICATION(<SMSG_AUCTION_REMOVED_NOTIFICATION as ServerMessage>::read_body(&mut r, body_size)?)),
            0x0291 => Ok(Self::SMSG_SERVER_MESSAGE(<SMSG_SERVER_MESSAGE as ServerMessage>::read_body(&mut r, body_size)?)),
            0x0295 => Ok(Self::SMSG_MEETINGSTONE_SETQUEUE(<SMSG_MEETINGSTONE_SETQUEUE as ServerMessage>::read_body(&mut r, body_size)?)),
            0x0297 => Ok(Self::SMSG_MEETINGSTONE_COMPLETE(<SMSG_MEETINGSTONE_COMPLETE as ServerMessage>::read_body(&mut r, body_size)?)),
            0x0298 => Ok(Self::SMSG_MEETINGSTONE_IN_PROGRESS(<SMSG_MEETINGSTONE_IN_PROGRESS as ServerMessage>::read_body(&mut r, body_size)?)),
            0x0299 => Ok(Self::SMSG_MEETINGSTONE_MEMBER_ADDED(<SMSG_MEETINGSTONE_MEMBER_ADDED as ServerMessage>::read_body(&mut r, body_size)?)),
            0x029C => Ok(Self::SMSG_CANCEL_AUTO_REPEAT(<SMSG_CANCEL_AUTO_REPEAT as ServerMessage>::read_body(&mut r, body_size)?)),
            0x029D => Ok(Self::SMSG_STANDSTATE_UPDATE(<SMSG_STANDSTATE_UPDATE as ServerMessage>::read_body(&mut r, body_size)?)),
            0x029E => Ok(Self::SMSG_LOOT_ALL_PASSED(<SMSG_LOOT_ALL_PASSED as ServerMessage>::read_body(&mut r, body_size)?)),
            0x029F => Ok(Self::SMSG_LOOT_ROLL_WON(<SMSG_LOOT_ROLL_WON as ServerMessage>::read_body(&mut r, body_size)?)),
            0x02A1 => Ok(Self::SMSG_LOOT_START_ROLL(<SMSG_LOOT_START_ROLL as ServerMessage>::read_body(&mut r, body_size)?)),
            0x02A2 => Ok(Self::SMSG_LOOT_ROLL(<SMSG_LOOT_ROLL as ServerMessage>::read_body(&mut r, body_size)?)),
            0x02A4 => Ok(Self::SMSG_LOOT_MASTER_LIST(<SMSG_LOOT_MASTER_LIST as ServerMessage>::read_body(&mut r, body_size)?)),
            0x02A5 => Ok(Self::SMSG_SET_FORCED_REACTIONS(<SMSG_SET_FORCED_REACTIONS as ServerMessage>::read_body(&mut r, body_size)?)),
            0x02A6 => Ok(Self::SMSG_SPELL_FAILED_OTHER(<SMSG_SPELL_FAILED_OTHER as ServerMessage>::read_body(&mut r, body_size)?)),
            0x02A7 => Ok(Self::SMSG_GAMEOBJECT_RESET_STATE(<SMSG_GAMEOBJECT_RESET_STATE as ServerMessage>::read_body(&mut r, body_size)?)),
            0x02A9 => Ok(Self::SMSG_CHAT_PLAYER_NOT_FOUND(<SMSG_CHAT_PLAYER_NOT_FOUND as ServerMessage>::read_body(&mut r, body_size)?)),
            0x02AA => Ok(Self::MSG_TALENT_WIPE_CONFIRM(<MSG_TALENT_WIPE_CONFIRM_Server as ServerMessage>::read_body(&mut r, body_size)?)),
            0x02AB => Ok(Self::SMSG_SUMMON_REQUEST(<SMSG_SUMMON_REQUEST as ServerMessage>::read_body(&mut r, body_size)?)),
            0x02AE => Ok(Self::SMSG_MONSTER_MOVE_TRANSPORT(<SMSG_MONSTER_MOVE_TRANSPORT as ServerMessage>::read_body(&mut r, body_size)?)),
            0x02AF => Ok(Self::SMSG_PET_BROKEN(<SMSG_PET_BROKEN as ServerMessage>::read_body(&mut r, body_size)?)),
            0x02B4 => Ok(Self::SMSG_FEIGN_DEATH_RESISTED(<SMSG_FEIGN_DEATH_RESISTED as ServerMessage>::read_body(&mut r, body_size)?)),
            0x02B7 => Ok(Self::SMSG_DUEL_COUNTDOWN(<SMSG_DUEL_COUNTDOWN as ServerMessage>::read_body(&mut r, body_size)?)),
            0x02B8 => Ok(Self::SMSG_AREA_TRIGGER_MESSAGE(<SMSG_AREA_TRIGGER_MESSAGE as ServerMessage>::read_body(&mut r, body_size)?)),
            0x02BB => Ok(Self::SMSG_MEETINGSTONE_JOINFAILED(<SMSG_MEETINGSTONE_JOINFAILED as ServerMessage>::read_body(&mut r, body_size)?)),
            0x02BC => Ok(Self::SMSG_PLAYER_SKINNED(<SMSG_PLAYER_SKINNED as ServerMessage>::read_body(&mut r, body_size)?)),
            0x02BD => Ok(Self::SMSG_DURABILITY_DAMAGE_DEATH(<SMSG_DURABILITY_DAMAGE_DEATH as ServerMessage>::read_body(&mut r, body_size)?)),
            0x02C2 => Ok(Self::SMSG_INIT_WORLD_STATES(<SMSG_INIT_WORLD_STATES as ServerMessage>::read_body(&mut r, body_size)?)),
            0x02C3 => Ok(Self::SMSG_UPDATE_WORLD_STATE(<SMSG_UPDATE_WORLD_STATE as ServerMessage>::read_body(&mut r, body_size)?)),
            0x02C5 => Ok(Self::SMSG_ITEM_NAME_QUERY_RESPONSE(<SMSG_ITEM_NAME_QUERY_RESPONSE as ServerMessage>::read_body(&mut r, body_size)?)),
            0x02C6 => Ok(Self::SMSG_PET_ACTION_FEEDBACK(<SMSG_PET_ACTION_FEEDBACK as ServerMessage>::read_body(&mut r, body_size)?)),
            0x02C8 => Ok(Self::SMSG_CHAR_RENAME(<SMSG_CHAR_RENAME as ServerMessage>::read_body(&mut r, body_size)?)),
            0x02CB => Ok(Self::SMSG_INSTANCE_SAVE_CREATED(<SMSG_INSTANCE_SAVE_CREATED as ServerMessage>::read_body(&mut r, body_size)?)),
            0x02CC => Ok(Self::SMSG_RAID_INSTANCE_INFO(<SMSG_RAID_INSTANCE_INFO as ServerMessage>::read_body(&mut r, body_size)?)),
            0x02D2 => Ok(Self::SMSG_PLAY_SOUND(<SMSG_PLAY_SOUND as ServerMessage>::read_body(&mut r, body_size)?)),
            0x02D4 => Ok(Self::SMSG_BATTLEFIELD_STATUS(<SMSG_BATTLEFIELD_STATUS as ServerMessage>::read_body(&mut r, body_size)?)),
            0x02D6 => Ok(Self::MSG_INSPECT_HONOR_STATS(<MSG_INSPECT_HONOR_STATS_Server as ServerMessage>::read_body(&mut r, body_size)?)),
            0x02DA => Ok(Self::SMSG_FORCE_WALK_SPEED_CHANGE(<SMSG_FORCE_WALK_SPEED_CHANGE as ServerMessage>::read_body(&mut r, body_size)?)),
            0x02DC => Ok(Self::SMSG_FORCE_SWIM_BACK_SPEED_CHANGE(<SMSG_FORCE_SWIM_BACK_SPEED_CHANGE as ServerMessage>::read_body(&mut r, body_size)?)),
            0x02DE => Ok(Self::SMSG_FORCE_TURN_RATE_CHANGE(<SMSG_FORCE_TURN_RATE_CHANGE as ServerMessage>::read_body(&mut r, body_size)?)),
            0x02E0 => Ok(Self::MSG_PVP_LOG_DATA(<MSG_PVP_LOG_DATA_Server as ServerMessage>::read_body(&mut r, body_size)?)),
            0x02E4 => Ok(Self::SMSG_AREA_SPIRIT_HEALER_TIME(<SMSG_AREA_SPIRIT_HEALER_TIME as ServerMessage>::read_body(&mut r, body_size)?)),
            0x02E8 => Ok(Self::SMSG_GROUP_JOINED_BATTLEGROUND(<SMSG_GROUP_JOINED_BATTLEGROUND as ServerMessage>::read_body(&mut r, body_size)?)),
            0x02E9 => Ok(Self::MSG_BATTLEGROUND_PLAYER_POSITIONS(<MSG_BATTLEGROUND_PLAYER_POSITIONS_Server as ServerMessage>::read_body(&mut r, body_size)?)),
            0x02EB => Ok(Self::SMSG_BINDER_CONFIRM(<SMSG_BINDER_CONFIRM as ServerMessage>::read_body(&mut r, body_size)?)),
            0x02EC => Ok(Self::SMSG_BATTLEGROUND_PLAYER_JOINED(<SMSG_BATTLEGROUND_PLAYER_JOINED as ServerMessage>::read_body(&mut r, body_size)?)),
            0x02ED => Ok(Self::SMSG_BATTLEGROUND_PLAYER_LEFT(<SMSG_BATTLEGROUND_PLAYER_LEFT as ServerMessage>::read_body(&mut r, body_size)?)),
            0x02F1 => Ok(Self::SMSG_PET_UNLEARN_CONFIRM(<SMSG_PET_UNLEARN_CONFIRM as ServerMessage>::read_body(&mut r, body_size)?)),
            0x02F2 => Ok(Self::SMSG_PARTY_MEMBER_STATS_FULL(<SMSG_PARTY_MEMBER_STATS_FULL as ServerMessage>::read_body(&mut r, body_size)?)),
            0x02F4 => Ok(Self::SMSG_WEATHER(<SMSG_WEATHER as ServerMessage>::read_body(&mut r, body_size)?)),
            0x02FA => Ok(Self::SMSG_RAID_INSTANCE_MESSAGE(<SMSG_RAID_INSTANCE_MESSAGE as ServerMessage>::read_body(&mut r, body_size)?)),
            0x02FD => Ok(Self::SMSG_CHAT_RESTRICTED(<SMSG_CHAT_RESTRICTED as ServerMessage>::read_body(&mut r, body_size)?)),
            0x02FE => Ok(Self::SMSG_SPLINE_SET_RUN_SPEED(<SMSG_SPLINE_SET_RUN_SPEED as ServerMessage>::read_body(&mut r, body_size)?)),
            0x02FF => Ok(Self::SMSG_SPLINE_SET_RUN_BACK_SPEED(<SMSG_SPLINE_SET_RUN_BACK_SPEED as ServerMessage>::read_body(&mut r, body_size)?)),
            0x0300 => Ok(Self::SMSG_SPLINE_SET_SWIM_SPEED(<SMSG_SPLINE_SET_SWIM_SPEED as ServerMessage>::read_body(&mut r, body_size)?)),
            0x0301 => Ok(Self::SMSG_SPLINE_SET_WALK_SPEED(<SMSG_SPLINE_SET_WALK_SPEED as ServerMessage>::read_body(&mut r, body_size)?)),
            0x0302 => Ok(Self::SMSG_SPLINE_SET_SWIM_BACK_SPEED(<SMSG_SPLINE_SET_SWIM_BACK_SPEED as ServerMessage>::read_body(&mut r, body_size)?)),
            0x0303 => Ok(Self::SMSG_SPLINE_SET_TURN_RATE(<SMSG_SPLINE_SET_TURN_RATE as ServerMessage>::read_body(&mut r, body_size)?)),
            0x0304 => Ok(Self::SMSG_SPLINE_MOVE_UNROOT(<SMSG_SPLINE_MOVE_UNROOT as ServerMessage>::read_body(&mut r, body_size)?)),
            0x0305 => Ok(Self::SMSG_SPLINE_MOVE_FEATHER_FALL(<SMSG_SPLINE_MOVE_FEATHER_FALL as ServerMessage>::read_body(&mut r, body_size)?)),
            0x0306 => Ok(Self::SMSG_SPLINE_MOVE_NORMAL_FALL(<SMSG_SPLINE_MOVE_NORMAL_FALL as ServerMessage>::read_body(&mut r, body_size)?)),
            0x0307 => Ok(Self::SMSG_SPLINE_MOVE_SET_HOVER(<SMSG_SPLINE_MOVE_SET_HOVER as ServerMessage>::read_body(&mut r, body_size)?)),
            0x0308 => Ok(Self::SMSG_SPLINE_MOVE_UNSET_HOVER(<SMSG_SPLINE_MOVE_UNSET_HOVER as ServerMessage>::read_body(&mut r, body_size)?)),
            0x0309 => Ok(Self::SMSG_SPLINE_MOVE_WATER_WALK(<SMSG_SPLINE_MOVE_WATER_WALK as ServerMessage>::read_body(&mut r, body_size)?)),
            0x030A => Ok(Self::SMSG_SPLINE_MOVE_LAND_WALK(<SMSG_SPLINE_MOVE_LAND_WALK as ServerMessage>::read_body(&mut r, body_size)?)),
            0x030B => Ok(Self::SMSG_SPLINE_MOVE_START_SWIM(<SMSG_SPLINE_MOVE_START_SWIM as ServerMessage>::read_body(&mut r, body_size)?)),
            0x030C => Ok(Self::SMSG_SPLINE_MOVE_STOP_SWIM(<SMSG_SPLINE_MOVE_STOP_SWIM as ServerMessage>::read_body(&mut r, body_size)?)),
            0x030D => Ok(Self::SMSG_SPLINE_MOVE_SET_RUN_MODE(<SMSG_SPLINE_MOVE_SET_RUN_MODE as ServerMessage>::read_body(&mut r, body_size)?)),
            0x030E => Ok(Self::SMSG_SPLINE_MOVE_SET_WALK_MODE(<SMSG_SPLINE_MOVE_SET_WALK_MODE as ServerMessage>::read_body(&mut r, body_size)?)),
            0x031A => Ok(Self::SMSG_SPLINE_MOVE_ROOT(<SMSG_SPLINE_MOVE_ROOT as ServerMessage>::read_body(&mut r, body_size)?)),
            0x031C => Ok(Self::SMSG_INVALIDATE_PLAYER(<SMSG_INVALIDATE_PLAYER as ServerMessage>::read_body(&mut r, body_size)?)),
            0x031E => Ok(Self::SMSG_INSTANCE_RESET(<SMSG_INSTANCE_RESET as ServerMessage>::read_body(&mut r, body_size)?)),
            0x031F => Ok(Self::SMSG_INSTANCE_RESET_FAILED(<SMSG_INSTANCE_RESET_FAILED as ServerMessage>::read_body(&mut r, body_size)?)),
            0x0320 => Ok(Self::SMSG_UPDATE_LAST_INSTANCE(<SMSG_UPDATE_LAST_INSTANCE as ServerMessage>::read_body(&mut r, body_size)?)),
            0x0321 => Ok(Self::MSG_RAID_TARGET_UPDATE(<MSG_RAID_TARGET_UPDATE_Server as ServerMessage>::read_body(&mut r, body_size)?)),
            0x0322 => Ok(Self::MSG_RAID_READY_CHECK(<MSG_RAID_READY_CHECK_Server as ServerMessage>::read_body(&mut r, body_size)?)),
            0x0324 => Ok(Self::SMSG_PET_ACTION_SOUND(<SMSG_PET_ACTION_SOUND as ServerMessage>::read_body(&mut r, body_size)?)),
            0x0325 => Ok(Self::SMSG_PET_DISMISS_SOUND(<SMSG_PET_DISMISS_SOUND as ServerMessage>::read_body(&mut r, body_size)?)),
            0x0328 => Ok(Self::SMSG_GM_TICKET_STATUS_UPDATE(<SMSG_GM_TICKET_STATUS_UPDATE as ServerMessage>::read_body(&mut r, body_size)?)),
            0x032B => Ok(Self::SMSG_UPDATE_INSTANCE_OWNERSHIP(<SMSG_UPDATE_INSTANCE_OWNERSHIP as ServerMessage>::read_body(&mut r, body_size)?)),
            0x032F => Ok(Self::SMSG_SPELLINSTAKILLLOG(<SMSG_SPELLINSTAKILLLOG as ServerMessage>::read_body(&mut r, body_size)?)),
            0x0330 => Ok(Self::SMSG_SPELL_UPDATE_CHAIN_TARGETS(<SMSG_SPELL_UPDATE_CHAIN_TARGETS as ServerMessage>::read_body(&mut r, body_size)?)),
            0x0332 => Ok(Self::SMSG_EXPECTED_SPAM_RECORDS(<SMSG_EXPECTED_SPAM_RECORDS as ServerMessage>::read_body(&mut r, body_size)?)),
            0x033B => Ok(Self::SMSG_DEFENSE_MESSAGE(<SMSG_DEFENSE_MESSAGE as ServerMessage>::read_body(&mut r, body_size)?)),
            _ => Err(crate::errors::ExpectedOpcodeError::Opcode(opcode as u32)),
        }
    }

    #[cfg(feature = "sync")]
    pub fn read_unencrypted<R: std::io::Read>(r: &mut R) -> std::result::Result<Self, crate::errors::ExpectedOpcodeError> {
        let size = (crate::util::read_u16_be(r)?.saturating_sub(2)) as u32;
        let opcode = crate::util::read_u16_le(r)?;

        let mut buf = vec![0; size as usize];
        r.read_exact(&mut buf)?;
        Self::read_opcodes(opcode, size, &buf)
    }
    #[cfg(feature = "sync")]
    pub fn read_encrypted<R: std::io::Read, D: Decrypter>(r: &mut R, d: &mut D) -> std::result::Result<Self, crate::errors::ExpectedOpcodeError> {
        let mut header = [0_u8; 4];
        r.read_exact(&mut header)?;
        let header = d.decrypt_server_header(header);
        let body_size = (header.size.saturating_sub(2)) as u32;

        let mut buf = vec![0; body_size as usize];
        r.read_exact(&mut buf)?;
        Self::read_opcodes(header.opcode, body_size, &buf)
    }

    #[cfg(feature = "tokio")]
    pub async fn tokio_read_unencrypted<R: tokio::io::AsyncReadExt + Unpin + Send>(r: &mut R) -> std::result::Result<Self, crate::errors::ExpectedOpcodeError> {
        let size = (crate::util::tokio_read_u16_be(r).await?.saturating_sub(2)) as u32;
        let opcode = crate::util::tokio_read_u16_le(r).await?;

        let mut buf = vec![0; size as usize];
        r.read_exact(&mut buf).await?;
        Self::read_opcodes(opcode, size, &buf)
    }
    #[cfg(feature = "tokio")]
    pub async fn tokio_read_encrypted<R: tokio::io::AsyncReadExt + Unpin + Send, D: Decrypter + Send>(r: &mut R, d: &mut D) -> std::result::Result<Self, crate::errors::ExpectedOpcodeError> {
        let mut header = [0_u8; 4];
        r.read_exact(&mut header).await?;
        let header = d.decrypt_server_header(header);
        let body_size = (header.size.saturating_sub(2)) as u32;

        let mut buf = vec![0; body_size as usize];
        r.read_exact(&mut buf).await?;
        Self::read_opcodes(header.opcode, body_size, &buf)
    }

    #[cfg(feature = "async-std")]
    pub async fn astd_read_unencrypted<R: async_std::io::ReadExt + Unpin + Send>(r: &mut R) -> std::result::Result<Self, crate::errors::ExpectedOpcodeError> {
        let size = (crate::util::astd_read_u16_be(r).await?.saturating_sub(2)) as u32;
        let opcode = crate::util::astd_read_u16_le(r).await?;

        let mut buf = vec![0; size as usize];
        r.read_exact(&mut buf).await?;
        Self::read_opcodes(opcode, size, &buf)
    }
    #[cfg(feature = "async-std")]
    pub async fn astd_read_encrypted<R: async_std::io::ReadExt + Unpin + Send, D: Decrypter + Send>(r: &mut R, d: &mut D) -> std::result::Result<Self, crate::errors::ExpectedOpcodeError> {
        let mut header = [0_u8; 4];
        r.read_exact(&mut header).await?;
        let header = d.decrypt_server_header(header);
        let body_size = (header.size.saturating_sub(2)) as u32;

        let mut buf = vec![0; body_size as usize];
        r.read_exact(&mut buf).await?;
        Self::read_opcodes(header.opcode, body_size, &buf)
    }

}

