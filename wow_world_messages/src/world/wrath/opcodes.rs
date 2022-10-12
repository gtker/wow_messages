use crate::wrath::{ServerMessage, ClientMessage};
use wow_srp::wrath_header::{ClientEncrypterHalf, ClientDecrypterHalf, ServerEncrypterHalf, ServerDecrypterHalf};

#[cfg(feature = "tokio")]
use tokio::io::{AsyncReadExt, AsyncWriteExt};
#[cfg(feature = "async-std")]
use async_std::io::{ReadExt, WriteExt};
use crate::world::wrath::MovementInfo;
use crate::world::wrath::MSG_SET_DUNGEON_DIFFICULTY;
use crate::world::wrath::CMSG_CHAR_CREATE;
use crate::world::wrath::CMSG_CHAR_ENUM;
use crate::world::wrath::CMSG_CHAR_DELETE;
use crate::world::wrath::CMSG_PLAYER_LOGIN;
use crate::world::wrath::CMSG_PLAYER_LOGOUT;
use crate::world::wrath::CMSG_LOGOUT_REQUEST;
use crate::world::wrath::CMSG_LOGOUT_CANCEL;
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
use crate::world::wrath::MSG_MOVE_START_FORWARD_Client;
use crate::world::wrath::MSG_MOVE_START_BACKWARD_Client;
use crate::world::wrath::MSG_MOVE_STOP_Client;
use crate::world::wrath::MSG_MOVE_START_STRAFE_LEFT_Client;
use crate::world::wrath::MSG_MOVE_START_STRAFE_RIGHT_Client;
use crate::world::wrath::MSG_MOVE_STOP_STRAFE_Client;
use crate::world::wrath::MSG_MOVE_JUMP_Client;
use crate::world::wrath::MSG_MOVE_START_TURN_LEFT_Client;
use crate::world::wrath::MSG_MOVE_START_TURN_RIGHT_Client;
use crate::world::wrath::MSG_MOVE_STOP_TURN_Client;
use crate::world::wrath::MSG_MOVE_START_PITCH_UP_Client;
use crate::world::wrath::MSG_MOVE_START_PITCH_DOWN_Client;
use crate::world::wrath::MSG_MOVE_STOP_PITCH_Client;
use crate::world::wrath::MSG_MOVE_SET_RUN_MODE_Client;
use crate::world::wrath::MSG_MOVE_SET_WALK_MODE_Client;
use crate::world::wrath::MSG_MOVE_FALL_LAND_Client;
use crate::world::wrath::MSG_MOVE_START_SWIM_Client;
use crate::world::wrath::MSG_MOVE_STOP_SWIM_Client;
use crate::world::wrath::MSG_MOVE_SET_FACING_Client;
use crate::world::wrath::MSG_MOVE_SET_PITCH_Client;
use crate::world::wrath::CMSG_FORCE_RUN_SPEED_CHANGE_ACK;
use crate::world::wrath::CMSG_FORCE_RUN_BACK_SPEED_CHANGE_ACK;
use crate::world::wrath::CMSG_FORCE_SWIM_SPEED_CHANGE_ACK;
use crate::world::wrath::CMSG_FORCE_MOVE_ROOT_ACK;
use crate::world::wrath::CMSG_FORCE_MOVE_UNROOT_ACK;
use crate::world::wrath::MSG_MOVE_HEARTBEAT_Client;
use crate::world::wrath::CMSG_NEXT_CINEMATIC_CAMERA;
use crate::world::wrath::CMSG_COMPLETE_CINEMATIC;
use crate::world::wrath::CMSG_TUTORIAL_FLAG;
use crate::world::wrath::CMSG_TUTORIAL_CLEAR;
use crate::world::wrath::CMSG_TUTORIAL_RESET;
use crate::world::wrath::CMSG_EMOTE;
use crate::world::wrath::CMSG_TEXT_EMOTE;
use crate::world::wrath::CMSG_SET_ACTION_BUTTON;
use crate::world::wrath::CMSG_CANCEL_CHANNELLING;
use crate::world::wrath::CMSG_SET_SELECTION;
use crate::world::wrath::CMSG_SET_TARGET_OBSOLETE;
use crate::world::wrath::CMSG_ATTACKSWING;
use crate::world::wrath::CMSG_ATTACKSTOP;
use crate::world::wrath::CMSG_REPOP_REQUEST;
use crate::world::wrath::CMSG_ACTIVATETAXI;
use crate::world::wrath::CMSG_PING;
use crate::world::wrath::CMSG_SETSHEATHED;
use crate::world::wrath::CMSG_AUTH_SESSION;
use crate::world::wrath::CMSG_ZONEUPDATE;
use crate::world::wrath::CMSG_REQUEST_ACCOUNT_DATA;
use crate::world::wrath::CMSG_UPDATE_ACCOUNT_DATA;
use crate::world::wrath::CMSG_CHAT_IGNORED;
use crate::world::wrath::CMSG_BATTLEFIELD_LIST;
use crate::world::wrath::MSG_AUCTION_HELLO_Client;
use crate::world::wrath::CMSG_AUCTION_SELL_ITEM;
use crate::world::wrath::CMSG_AUCTION_REMOVE_ITEM;
use crate::world::wrath::CMSG_AUCTION_LIST_ITEMS;
use crate::world::wrath::CMSG_AUCTION_LIST_OWNER_ITEMS;
use crate::world::wrath::CMSG_AUCTION_PLACE_BID;
use crate::world::wrath::CMSG_AUCTION_LIST_BIDDER_ITEMS;
use crate::world::wrath::CMSG_SET_ACTIVE_MOVER;
use crate::world::wrath::CMSG_SET_ACTIONBAR_TOGGLES;
use crate::world::wrath::CMSG_CHAR_RENAME;
use crate::world::wrath::CMSG_MOVE_FALL_RESET;
use crate::world::wrath::CMSG_FORCE_WALK_SPEED_CHANGE_ACK;
use crate::world::wrath::CMSG_FORCE_SWIM_BACK_SPEED_CHANGE_ACK;
use crate::world::wrath::CMSG_FORCE_TURN_RATE_CHANGE_ACK;
use crate::world::wrath::CMSG_ACTIVATETAXIEXPRESS;
use crate::world::wrath::CMSG_MOVE_SET_FLY;
use crate::world::wrath::MSG_MOVE_START_ASCEND_Client;
use crate::world::wrath::MSG_MOVE_STOP_ASCEND_Client;
use crate::world::wrath::CMSG_REALM_SPLIT;
use crate::world::wrath::CMSG_MOVE_CHNG_TRANSPORT;
use crate::world::wrath::MSG_MOVE_START_DESCEND_Client;
use crate::world::wrath::CMSG_SET_ACTIVE_VOICE_CHANNEL;
use crate::world::wrath::CMSG_READY_FOR_ACCOUNT_DATA_TIMES;

#[derive(Debug, Clone, PartialEq)]
pub enum ClientOpcodeMessage {
    MSG_SET_DUNGEON_DIFFICULTY(MSG_SET_DUNGEON_DIFFICULTY),
    CMSG_CHAR_CREATE(CMSG_CHAR_CREATE),
    CMSG_CHAR_ENUM(CMSG_CHAR_ENUM),
    CMSG_CHAR_DELETE(CMSG_CHAR_DELETE),
    CMSG_PLAYER_LOGIN(CMSG_PLAYER_LOGIN),
    CMSG_PLAYER_LOGOUT(CMSG_PLAYER_LOGOUT),
    CMSG_LOGOUT_REQUEST(CMSG_LOGOUT_REQUEST),
    CMSG_LOGOUT_CANCEL(CMSG_LOGOUT_CANCEL),
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
    MSG_MOVE_FALL_LAND(MSG_MOVE_FALL_LAND_Client),
    MSG_MOVE_START_SWIM(MSG_MOVE_START_SWIM_Client),
    MSG_MOVE_STOP_SWIM(MSG_MOVE_STOP_SWIM_Client),
    MSG_MOVE_SET_FACING(MSG_MOVE_SET_FACING_Client),
    MSG_MOVE_SET_PITCH(MSG_MOVE_SET_PITCH_Client),
    CMSG_FORCE_RUN_SPEED_CHANGE_ACK(CMSG_FORCE_RUN_SPEED_CHANGE_ACK),
    CMSG_FORCE_RUN_BACK_SPEED_CHANGE_ACK(CMSG_FORCE_RUN_BACK_SPEED_CHANGE_ACK),
    CMSG_FORCE_SWIM_SPEED_CHANGE_ACK(CMSG_FORCE_SWIM_SPEED_CHANGE_ACK),
    CMSG_FORCE_MOVE_ROOT_ACK(CMSG_FORCE_MOVE_ROOT_ACK),
    CMSG_FORCE_MOVE_UNROOT_ACK(CMSG_FORCE_MOVE_UNROOT_ACK),
    MSG_MOVE_HEARTBEAT(MSG_MOVE_HEARTBEAT_Client),
    CMSG_NEXT_CINEMATIC_CAMERA(CMSG_NEXT_CINEMATIC_CAMERA),
    CMSG_COMPLETE_CINEMATIC(CMSG_COMPLETE_CINEMATIC),
    CMSG_TUTORIAL_FLAG(CMSG_TUTORIAL_FLAG),
    CMSG_TUTORIAL_CLEAR(CMSG_TUTORIAL_CLEAR),
    CMSG_TUTORIAL_RESET(CMSG_TUTORIAL_RESET),
    CMSG_EMOTE(CMSG_EMOTE),
    CMSG_TEXT_EMOTE(CMSG_TEXT_EMOTE),
    CMSG_SET_ACTION_BUTTON(CMSG_SET_ACTION_BUTTON),
    CMSG_CANCEL_CHANNELLING(CMSG_CANCEL_CHANNELLING),
    CMSG_SET_SELECTION(CMSG_SET_SELECTION),
    CMSG_SET_TARGET_OBSOLETE(CMSG_SET_TARGET_OBSOLETE),
    CMSG_ATTACKSWING(CMSG_ATTACKSWING),
    CMSG_ATTACKSTOP(CMSG_ATTACKSTOP),
    CMSG_REPOP_REQUEST(CMSG_REPOP_REQUEST),
    CMSG_ACTIVATETAXI(CMSG_ACTIVATETAXI),
    CMSG_PING(CMSG_PING),
    CMSG_SETSHEATHED(CMSG_SETSHEATHED),
    CMSG_AUTH_SESSION(CMSG_AUTH_SESSION),
    CMSG_ZONEUPDATE(CMSG_ZONEUPDATE),
    CMSG_REQUEST_ACCOUNT_DATA(CMSG_REQUEST_ACCOUNT_DATA),
    CMSG_UPDATE_ACCOUNT_DATA(CMSG_UPDATE_ACCOUNT_DATA),
    CMSG_CHAT_IGNORED(CMSG_CHAT_IGNORED),
    CMSG_BATTLEFIELD_LIST(CMSG_BATTLEFIELD_LIST),
    MSG_AUCTION_HELLO(MSG_AUCTION_HELLO_Client),
    CMSG_AUCTION_SELL_ITEM(CMSG_AUCTION_SELL_ITEM),
    CMSG_AUCTION_REMOVE_ITEM(CMSG_AUCTION_REMOVE_ITEM),
    CMSG_AUCTION_LIST_ITEMS(CMSG_AUCTION_LIST_ITEMS),
    CMSG_AUCTION_LIST_OWNER_ITEMS(CMSG_AUCTION_LIST_OWNER_ITEMS),
    CMSG_AUCTION_PLACE_BID(CMSG_AUCTION_PLACE_BID),
    CMSG_AUCTION_LIST_BIDDER_ITEMS(CMSG_AUCTION_LIST_BIDDER_ITEMS),
    CMSG_SET_ACTIVE_MOVER(CMSG_SET_ACTIVE_MOVER),
    CMSG_SET_ACTIONBAR_TOGGLES(CMSG_SET_ACTIONBAR_TOGGLES),
    CMSG_CHAR_RENAME(CMSG_CHAR_RENAME),
    CMSG_MOVE_FALL_RESET(CMSG_MOVE_FALL_RESET),
    CMSG_FORCE_WALK_SPEED_CHANGE_ACK(CMSG_FORCE_WALK_SPEED_CHANGE_ACK),
    CMSG_FORCE_SWIM_BACK_SPEED_CHANGE_ACK(CMSG_FORCE_SWIM_BACK_SPEED_CHANGE_ACK),
    CMSG_FORCE_TURN_RATE_CHANGE_ACK(CMSG_FORCE_TURN_RATE_CHANGE_ACK),
    CMSG_ACTIVATETAXIEXPRESS(CMSG_ACTIVATETAXIEXPRESS),
    CMSG_MOVE_SET_FLY(CMSG_MOVE_SET_FLY),
    MSG_MOVE_START_ASCEND(MSG_MOVE_START_ASCEND_Client),
    MSG_MOVE_STOP_ASCEND(MSG_MOVE_STOP_ASCEND_Client),
    CMSG_REALM_SPLIT(CMSG_REALM_SPLIT),
    CMSG_MOVE_CHNG_TRANSPORT(CMSG_MOVE_CHNG_TRANSPORT),
    MSG_MOVE_START_DESCEND(MSG_MOVE_START_DESCEND_Client),
    CMSG_SET_ACTIVE_VOICE_CHANNEL(CMSG_SET_ACTIVE_VOICE_CHANNEL),
    CMSG_READY_FOR_ACCOUNT_DATA_TIMES(CMSG_READY_FOR_ACCOUNT_DATA_TIMES),
}

impl ClientOpcodeMessage {
    fn read_opcodes(opcode: u32, body_size: u32, mut r: &[u8]) -> std::result::Result<Self, crate::errors::ExpectedOpcodeError> {
        match opcode {
            0x0329 => Ok(Self::MSG_SET_DUNGEON_DIFFICULTY(<MSG_SET_DUNGEON_DIFFICULTY as crate::Message>::read_body(&mut r, body_size)?)),
            0x0036 => Ok(Self::CMSG_CHAR_CREATE(<CMSG_CHAR_CREATE as crate::Message>::read_body(&mut r, body_size)?)),
            0x0037 => Ok(Self::CMSG_CHAR_ENUM(<CMSG_CHAR_ENUM as crate::Message>::read_body(&mut r, body_size)?)),
            0x0038 => Ok(Self::CMSG_CHAR_DELETE(<CMSG_CHAR_DELETE as crate::Message>::read_body(&mut r, body_size)?)),
            0x003D => Ok(Self::CMSG_PLAYER_LOGIN(<CMSG_PLAYER_LOGIN as crate::Message>::read_body(&mut r, body_size)?)),
            0x004A => Ok(Self::CMSG_PLAYER_LOGOUT(<CMSG_PLAYER_LOGOUT as crate::Message>::read_body(&mut r, body_size)?)),
            0x004B => Ok(Self::CMSG_LOGOUT_REQUEST(<CMSG_LOGOUT_REQUEST as crate::Message>::read_body(&mut r, body_size)?)),
            0x004E => Ok(Self::CMSG_LOGOUT_CANCEL(<CMSG_LOGOUT_CANCEL as crate::Message>::read_body(&mut r, body_size)?)),
            0x0095 => Ok(Self::CMSG_MESSAGECHAT(<CMSG_MESSAGECHAT as crate::Message>::read_body(&mut r, body_size)?)),
            0x0097 => Ok(Self::CMSG_JOIN_CHANNEL(<CMSG_JOIN_CHANNEL as crate::Message>::read_body(&mut r, body_size)?)),
            0x0098 => Ok(Self::CMSG_LEAVE_CHANNEL(<CMSG_LEAVE_CHANNEL as crate::Message>::read_body(&mut r, body_size)?)),
            0x009A => Ok(Self::CMSG_CHANNEL_LIST(<CMSG_CHANNEL_LIST as crate::Message>::read_body(&mut r, body_size)?)),
            0x009C => Ok(Self::CMSG_CHANNEL_PASSWORD(<CMSG_CHANNEL_PASSWORD as crate::Message>::read_body(&mut r, body_size)?)),
            0x009D => Ok(Self::CMSG_CHANNEL_SET_OWNER(<CMSG_CHANNEL_SET_OWNER as crate::Message>::read_body(&mut r, body_size)?)),
            0x009E => Ok(Self::CMSG_CHANNEL_OWNER(<CMSG_CHANNEL_OWNER as crate::Message>::read_body(&mut r, body_size)?)),
            0x009F => Ok(Self::CMSG_CHANNEL_MODERATOR(<CMSG_CHANNEL_MODERATOR as crate::Message>::read_body(&mut r, body_size)?)),
            0x00A0 => Ok(Self::CMSG_CHANNEL_UNMODERATOR(<CMSG_CHANNEL_UNMODERATOR as crate::Message>::read_body(&mut r, body_size)?)),
            0x00A1 => Ok(Self::CMSG_CHANNEL_MUTE(<CMSG_CHANNEL_MUTE as crate::Message>::read_body(&mut r, body_size)?)),
            0x00A2 => Ok(Self::CMSG_CHANNEL_UNMUTE(<CMSG_CHANNEL_UNMUTE as crate::Message>::read_body(&mut r, body_size)?)),
            0x00A3 => Ok(Self::CMSG_CHANNEL_INVITE(<CMSG_CHANNEL_INVITE as crate::Message>::read_body(&mut r, body_size)?)),
            0x00A4 => Ok(Self::CMSG_CHANNEL_KICK(<CMSG_CHANNEL_KICK as crate::Message>::read_body(&mut r, body_size)?)),
            0x00A5 => Ok(Self::CMSG_CHANNEL_BAN(<CMSG_CHANNEL_BAN as crate::Message>::read_body(&mut r, body_size)?)),
            0x00A6 => Ok(Self::CMSG_CHANNEL_UNBAN(<CMSG_CHANNEL_UNBAN as crate::Message>::read_body(&mut r, body_size)?)),
            0x00A7 => Ok(Self::CMSG_CHANNEL_ANNOUNCEMENTS(<CMSG_CHANNEL_ANNOUNCEMENTS as crate::Message>::read_body(&mut r, body_size)?)),
            0x00A8 => Ok(Self::CMSG_CHANNEL_MODERATE(<CMSG_CHANNEL_MODERATE as crate::Message>::read_body(&mut r, body_size)?)),
            0x00B5 => Ok(Self::MSG_MOVE_START_FORWARD(<MSG_MOVE_START_FORWARD_Client as crate::Message>::read_body(&mut r, body_size)?)),
            0x00B6 => Ok(Self::MSG_MOVE_START_BACKWARD(<MSG_MOVE_START_BACKWARD_Client as crate::Message>::read_body(&mut r, body_size)?)),
            0x00B7 => Ok(Self::MSG_MOVE_STOP(<MSG_MOVE_STOP_Client as crate::Message>::read_body(&mut r, body_size)?)),
            0x00B8 => Ok(Self::MSG_MOVE_START_STRAFE_LEFT(<MSG_MOVE_START_STRAFE_LEFT_Client as crate::Message>::read_body(&mut r, body_size)?)),
            0x00B9 => Ok(Self::MSG_MOVE_START_STRAFE_RIGHT(<MSG_MOVE_START_STRAFE_RIGHT_Client as crate::Message>::read_body(&mut r, body_size)?)),
            0x00BA => Ok(Self::MSG_MOVE_STOP_STRAFE(<MSG_MOVE_STOP_STRAFE_Client as crate::Message>::read_body(&mut r, body_size)?)),
            0x00BB => Ok(Self::MSG_MOVE_JUMP(<MSG_MOVE_JUMP_Client as crate::Message>::read_body(&mut r, body_size)?)),
            0x00BC => Ok(Self::MSG_MOVE_START_TURN_LEFT(<MSG_MOVE_START_TURN_LEFT_Client as crate::Message>::read_body(&mut r, body_size)?)),
            0x00BD => Ok(Self::MSG_MOVE_START_TURN_RIGHT(<MSG_MOVE_START_TURN_RIGHT_Client as crate::Message>::read_body(&mut r, body_size)?)),
            0x00BE => Ok(Self::MSG_MOVE_STOP_TURN(<MSG_MOVE_STOP_TURN_Client as crate::Message>::read_body(&mut r, body_size)?)),
            0x00BF => Ok(Self::MSG_MOVE_START_PITCH_UP(<MSG_MOVE_START_PITCH_UP_Client as crate::Message>::read_body(&mut r, body_size)?)),
            0x00C0 => Ok(Self::MSG_MOVE_START_PITCH_DOWN(<MSG_MOVE_START_PITCH_DOWN_Client as crate::Message>::read_body(&mut r, body_size)?)),
            0x00C1 => Ok(Self::MSG_MOVE_STOP_PITCH(<MSG_MOVE_STOP_PITCH_Client as crate::Message>::read_body(&mut r, body_size)?)),
            0x00C2 => Ok(Self::MSG_MOVE_SET_RUN_MODE(<MSG_MOVE_SET_RUN_MODE_Client as crate::Message>::read_body(&mut r, body_size)?)),
            0x00C3 => Ok(Self::MSG_MOVE_SET_WALK_MODE(<MSG_MOVE_SET_WALK_MODE_Client as crate::Message>::read_body(&mut r, body_size)?)),
            0x00C9 => Ok(Self::MSG_MOVE_FALL_LAND(<MSG_MOVE_FALL_LAND_Client as crate::Message>::read_body(&mut r, body_size)?)),
            0x00CA => Ok(Self::MSG_MOVE_START_SWIM(<MSG_MOVE_START_SWIM_Client as crate::Message>::read_body(&mut r, body_size)?)),
            0x00CB => Ok(Self::MSG_MOVE_STOP_SWIM(<MSG_MOVE_STOP_SWIM_Client as crate::Message>::read_body(&mut r, body_size)?)),
            0x00DA => Ok(Self::MSG_MOVE_SET_FACING(<MSG_MOVE_SET_FACING_Client as crate::Message>::read_body(&mut r, body_size)?)),
            0x00DB => Ok(Self::MSG_MOVE_SET_PITCH(<MSG_MOVE_SET_PITCH_Client as crate::Message>::read_body(&mut r, body_size)?)),
            0x00E3 => Ok(Self::CMSG_FORCE_RUN_SPEED_CHANGE_ACK(<CMSG_FORCE_RUN_SPEED_CHANGE_ACK as crate::Message>::read_body(&mut r, body_size)?)),
            0x00E5 => Ok(Self::CMSG_FORCE_RUN_BACK_SPEED_CHANGE_ACK(<CMSG_FORCE_RUN_BACK_SPEED_CHANGE_ACK as crate::Message>::read_body(&mut r, body_size)?)),
            0x00E7 => Ok(Self::CMSG_FORCE_SWIM_SPEED_CHANGE_ACK(<CMSG_FORCE_SWIM_SPEED_CHANGE_ACK as crate::Message>::read_body(&mut r, body_size)?)),
            0x00E9 => Ok(Self::CMSG_FORCE_MOVE_ROOT_ACK(<CMSG_FORCE_MOVE_ROOT_ACK as crate::Message>::read_body(&mut r, body_size)?)),
            0x00EB => Ok(Self::CMSG_FORCE_MOVE_UNROOT_ACK(<CMSG_FORCE_MOVE_UNROOT_ACK as crate::Message>::read_body(&mut r, body_size)?)),
            0x00EE => Ok(Self::MSG_MOVE_HEARTBEAT(<MSG_MOVE_HEARTBEAT_Client as crate::Message>::read_body(&mut r, body_size)?)),
            0x00FB => Ok(Self::CMSG_NEXT_CINEMATIC_CAMERA(<CMSG_NEXT_CINEMATIC_CAMERA as crate::Message>::read_body(&mut r, body_size)?)),
            0x00FC => Ok(Self::CMSG_COMPLETE_CINEMATIC(<CMSG_COMPLETE_CINEMATIC as crate::Message>::read_body(&mut r, body_size)?)),
            0x00FE => Ok(Self::CMSG_TUTORIAL_FLAG(<CMSG_TUTORIAL_FLAG as crate::Message>::read_body(&mut r, body_size)?)),
            0x00FF => Ok(Self::CMSG_TUTORIAL_CLEAR(<CMSG_TUTORIAL_CLEAR as crate::Message>::read_body(&mut r, body_size)?)),
            0x0100 => Ok(Self::CMSG_TUTORIAL_RESET(<CMSG_TUTORIAL_RESET as crate::Message>::read_body(&mut r, body_size)?)),
            0x0102 => Ok(Self::CMSG_EMOTE(<CMSG_EMOTE as crate::Message>::read_body(&mut r, body_size)?)),
            0x0104 => Ok(Self::CMSG_TEXT_EMOTE(<CMSG_TEXT_EMOTE as crate::Message>::read_body(&mut r, body_size)?)),
            0x0128 => Ok(Self::CMSG_SET_ACTION_BUTTON(<CMSG_SET_ACTION_BUTTON as crate::Message>::read_body(&mut r, body_size)?)),
            0x013B => Ok(Self::CMSG_CANCEL_CHANNELLING(<CMSG_CANCEL_CHANNELLING as crate::Message>::read_body(&mut r, body_size)?)),
            0x013D => Ok(Self::CMSG_SET_SELECTION(<CMSG_SET_SELECTION as crate::Message>::read_body(&mut r, body_size)?)),
            0x013E => Ok(Self::CMSG_SET_TARGET_OBSOLETE(<CMSG_SET_TARGET_OBSOLETE as crate::Message>::read_body(&mut r, body_size)?)),
            0x0141 => Ok(Self::CMSG_ATTACKSWING(<CMSG_ATTACKSWING as crate::Message>::read_body(&mut r, body_size)?)),
            0x0142 => Ok(Self::CMSG_ATTACKSTOP(<CMSG_ATTACKSTOP as crate::Message>::read_body(&mut r, body_size)?)),
            0x015A => Ok(Self::CMSG_REPOP_REQUEST(<CMSG_REPOP_REQUEST as crate::Message>::read_body(&mut r, body_size)?)),
            0x01AD => Ok(Self::CMSG_ACTIVATETAXI(<CMSG_ACTIVATETAXI as crate::Message>::read_body(&mut r, body_size)?)),
            0x01DC => Ok(Self::CMSG_PING(<CMSG_PING as crate::Message>::read_body(&mut r, body_size)?)),
            0x01E0 => Ok(Self::CMSG_SETSHEATHED(<CMSG_SETSHEATHED as crate::Message>::read_body(&mut r, body_size)?)),
            0x01ED => Ok(Self::CMSG_AUTH_SESSION(<CMSG_AUTH_SESSION as crate::Message>::read_body(&mut r, body_size)?)),
            0x01F4 => Ok(Self::CMSG_ZONEUPDATE(<CMSG_ZONEUPDATE as crate::Message>::read_body(&mut r, body_size)?)),
            0x020A => Ok(Self::CMSG_REQUEST_ACCOUNT_DATA(<CMSG_REQUEST_ACCOUNT_DATA as crate::Message>::read_body(&mut r, body_size)?)),
            0x020B => Ok(Self::CMSG_UPDATE_ACCOUNT_DATA(<CMSG_UPDATE_ACCOUNT_DATA as crate::Message>::read_body(&mut r, body_size)?)),
            0x0225 => Ok(Self::CMSG_CHAT_IGNORED(<CMSG_CHAT_IGNORED as crate::Message>::read_body(&mut r, body_size)?)),
            0x023C => Ok(Self::CMSG_BATTLEFIELD_LIST(<CMSG_BATTLEFIELD_LIST as crate::Message>::read_body(&mut r, body_size)?)),
            0x0255 => Ok(Self::MSG_AUCTION_HELLO(<MSG_AUCTION_HELLO_Client as crate::Message>::read_body(&mut r, body_size)?)),
            0x0256 => Ok(Self::CMSG_AUCTION_SELL_ITEM(<CMSG_AUCTION_SELL_ITEM as crate::Message>::read_body(&mut r, body_size)?)),
            0x0257 => Ok(Self::CMSG_AUCTION_REMOVE_ITEM(<CMSG_AUCTION_REMOVE_ITEM as crate::Message>::read_body(&mut r, body_size)?)),
            0x0258 => Ok(Self::CMSG_AUCTION_LIST_ITEMS(<CMSG_AUCTION_LIST_ITEMS as crate::Message>::read_body(&mut r, body_size)?)),
            0x0259 => Ok(Self::CMSG_AUCTION_LIST_OWNER_ITEMS(<CMSG_AUCTION_LIST_OWNER_ITEMS as crate::Message>::read_body(&mut r, body_size)?)),
            0x025A => Ok(Self::CMSG_AUCTION_PLACE_BID(<CMSG_AUCTION_PLACE_BID as crate::Message>::read_body(&mut r, body_size)?)),
            0x0264 => Ok(Self::CMSG_AUCTION_LIST_BIDDER_ITEMS(<CMSG_AUCTION_LIST_BIDDER_ITEMS as crate::Message>::read_body(&mut r, body_size)?)),
            0x026A => Ok(Self::CMSG_SET_ACTIVE_MOVER(<CMSG_SET_ACTIVE_MOVER as crate::Message>::read_body(&mut r, body_size)?)),
            0x02BF => Ok(Self::CMSG_SET_ACTIONBAR_TOGGLES(<CMSG_SET_ACTIONBAR_TOGGLES as crate::Message>::read_body(&mut r, body_size)?)),
            0x02C7 => Ok(Self::CMSG_CHAR_RENAME(<CMSG_CHAR_RENAME as crate::Message>::read_body(&mut r, body_size)?)),
            0x02CA => Ok(Self::CMSG_MOVE_FALL_RESET(<CMSG_MOVE_FALL_RESET as crate::Message>::read_body(&mut r, body_size)?)),
            0x02DB => Ok(Self::CMSG_FORCE_WALK_SPEED_CHANGE_ACK(<CMSG_FORCE_WALK_SPEED_CHANGE_ACK as crate::Message>::read_body(&mut r, body_size)?)),
            0x02DD => Ok(Self::CMSG_FORCE_SWIM_BACK_SPEED_CHANGE_ACK(<CMSG_FORCE_SWIM_BACK_SPEED_CHANGE_ACK as crate::Message>::read_body(&mut r, body_size)?)),
            0x02DF => Ok(Self::CMSG_FORCE_TURN_RATE_CHANGE_ACK(<CMSG_FORCE_TURN_RATE_CHANGE_ACK as crate::Message>::read_body(&mut r, body_size)?)),
            0x0312 => Ok(Self::CMSG_ACTIVATETAXIEXPRESS(<CMSG_ACTIVATETAXIEXPRESS as crate::Message>::read_body(&mut r, body_size)?)),
            0x0346 => Ok(Self::CMSG_MOVE_SET_FLY(<CMSG_MOVE_SET_FLY as crate::Message>::read_body(&mut r, body_size)?)),
            0x0359 => Ok(Self::MSG_MOVE_START_ASCEND(<MSG_MOVE_START_ASCEND_Client as crate::Message>::read_body(&mut r, body_size)?)),
            0x035A => Ok(Self::MSG_MOVE_STOP_ASCEND(<MSG_MOVE_STOP_ASCEND_Client as crate::Message>::read_body(&mut r, body_size)?)),
            0x038C => Ok(Self::CMSG_REALM_SPLIT(<CMSG_REALM_SPLIT as crate::Message>::read_body(&mut r, body_size)?)),
            0x038D => Ok(Self::CMSG_MOVE_CHNG_TRANSPORT(<CMSG_MOVE_CHNG_TRANSPORT as crate::Message>::read_body(&mut r, body_size)?)),
            0x03A7 => Ok(Self::MSG_MOVE_START_DESCEND(<MSG_MOVE_START_DESCEND_Client as crate::Message>::read_body(&mut r, body_size)?)),
            0x03D3 => Ok(Self::CMSG_SET_ACTIVE_VOICE_CHANNEL(<CMSG_SET_ACTIVE_VOICE_CHANNEL as crate::Message>::read_body(&mut r, body_size)?)),
            0x04FF => Ok(Self::CMSG_READY_FOR_ACCOUNT_DATA_TIMES(<CMSG_READY_FOR_ACCOUNT_DATA_TIMES as crate::Message>::read_body(&mut r, body_size)?)),
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
    #[cfg(feature = "sync")]
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
    #[cfg(feature = "tokio")]
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
    #[cfg(feature = "async-std")]
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

    #[cfg(feature = "sync")]
    pub fn write_encrypted_client<W: std::io::Write>(&self, w: &mut W, e: &mut ClientEncrypterHalf) -> Result<(), std::io::Error> {
        match self {
            Self::MSG_SET_DUNGEON_DIFFICULTY(c) => c.write_encrypted_client(w, e),
            Self::CMSG_CHAR_CREATE(c) => c.write_encrypted_client(w, e),
            Self::CMSG_CHAR_ENUM(c) => c.write_encrypted_client(w, e),
            Self::CMSG_CHAR_DELETE(c) => c.write_encrypted_client(w, e),
            Self::CMSG_PLAYER_LOGIN(c) => c.write_encrypted_client(w, e),
            Self::CMSG_PLAYER_LOGOUT(c) => c.write_encrypted_client(w, e),
            Self::CMSG_LOGOUT_REQUEST(c) => c.write_encrypted_client(w, e),
            Self::CMSG_LOGOUT_CANCEL(c) => c.write_encrypted_client(w, e),
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
            Self::CMSG_FORCE_RUN_SPEED_CHANGE_ACK(c) => c.write_encrypted_client(w, e),
            Self::CMSG_FORCE_RUN_BACK_SPEED_CHANGE_ACK(c) => c.write_encrypted_client(w, e),
            Self::CMSG_FORCE_SWIM_SPEED_CHANGE_ACK(c) => c.write_encrypted_client(w, e),
            Self::CMSG_FORCE_MOVE_ROOT_ACK(c) => c.write_encrypted_client(w, e),
            Self::CMSG_FORCE_MOVE_UNROOT_ACK(c) => c.write_encrypted_client(w, e),
            Self::MSG_MOVE_HEARTBEAT(c) => c.write_encrypted_client(w, e),
            Self::CMSG_NEXT_CINEMATIC_CAMERA(c) => c.write_encrypted_client(w, e),
            Self::CMSG_COMPLETE_CINEMATIC(c) => c.write_encrypted_client(w, e),
            Self::CMSG_TUTORIAL_FLAG(c) => c.write_encrypted_client(w, e),
            Self::CMSG_TUTORIAL_CLEAR(c) => c.write_encrypted_client(w, e),
            Self::CMSG_TUTORIAL_RESET(c) => c.write_encrypted_client(w, e),
            Self::CMSG_EMOTE(c) => c.write_encrypted_client(w, e),
            Self::CMSG_TEXT_EMOTE(c) => c.write_encrypted_client(w, e),
            Self::CMSG_SET_ACTION_BUTTON(c) => c.write_encrypted_client(w, e),
            Self::CMSG_CANCEL_CHANNELLING(c) => c.write_encrypted_client(w, e),
            Self::CMSG_SET_SELECTION(c) => c.write_encrypted_client(w, e),
            Self::CMSG_SET_TARGET_OBSOLETE(c) => c.write_encrypted_client(w, e),
            Self::CMSG_ATTACKSWING(c) => c.write_encrypted_client(w, e),
            Self::CMSG_ATTACKSTOP(c) => c.write_encrypted_client(w, e),
            Self::CMSG_REPOP_REQUEST(c) => c.write_encrypted_client(w, e),
            Self::CMSG_ACTIVATETAXI(c) => c.write_encrypted_client(w, e),
            Self::CMSG_PING(c) => c.write_encrypted_client(w, e),
            Self::CMSG_SETSHEATHED(c) => c.write_encrypted_client(w, e),
            Self::CMSG_AUTH_SESSION(c) => c.write_encrypted_client(w, e),
            Self::CMSG_ZONEUPDATE(c) => c.write_encrypted_client(w, e),
            Self::CMSG_REQUEST_ACCOUNT_DATA(c) => c.write_encrypted_client(w, e),
            Self::CMSG_UPDATE_ACCOUNT_DATA(c) => c.write_encrypted_client(w, e),
            Self::CMSG_CHAT_IGNORED(c) => c.write_encrypted_client(w, e),
            Self::CMSG_BATTLEFIELD_LIST(c) => c.write_encrypted_client(w, e),
            Self::MSG_AUCTION_HELLO(c) => c.write_encrypted_client(w, e),
            Self::CMSG_AUCTION_SELL_ITEM(c) => c.write_encrypted_client(w, e),
            Self::CMSG_AUCTION_REMOVE_ITEM(c) => c.write_encrypted_client(w, e),
            Self::CMSG_AUCTION_LIST_ITEMS(c) => c.write_encrypted_client(w, e),
            Self::CMSG_AUCTION_LIST_OWNER_ITEMS(c) => c.write_encrypted_client(w, e),
            Self::CMSG_AUCTION_PLACE_BID(c) => c.write_encrypted_client(w, e),
            Self::CMSG_AUCTION_LIST_BIDDER_ITEMS(c) => c.write_encrypted_client(w, e),
            Self::CMSG_SET_ACTIVE_MOVER(c) => c.write_encrypted_client(w, e),
            Self::CMSG_SET_ACTIONBAR_TOGGLES(c) => c.write_encrypted_client(w, e),
            Self::CMSG_CHAR_RENAME(c) => c.write_encrypted_client(w, e),
            Self::CMSG_MOVE_FALL_RESET(c) => c.write_encrypted_client(w, e),
            Self::CMSG_FORCE_WALK_SPEED_CHANGE_ACK(c) => c.write_encrypted_client(w, e),
            Self::CMSG_FORCE_SWIM_BACK_SPEED_CHANGE_ACK(c) => c.write_encrypted_client(w, e),
            Self::CMSG_FORCE_TURN_RATE_CHANGE_ACK(c) => c.write_encrypted_client(w, e),
            Self::CMSG_ACTIVATETAXIEXPRESS(c) => c.write_encrypted_client(w, e),
            Self::CMSG_MOVE_SET_FLY(c) => c.write_encrypted_client(w, e),
            Self::MSG_MOVE_START_ASCEND(c) => c.write_encrypted_client(w, e),
            Self::MSG_MOVE_STOP_ASCEND(c) => c.write_encrypted_client(w, e),
            Self::CMSG_REALM_SPLIT(c) => c.write_encrypted_client(w, e),
            Self::CMSG_MOVE_CHNG_TRANSPORT(c) => c.write_encrypted_client(w, e),
            Self::MSG_MOVE_START_DESCEND(c) => c.write_encrypted_client(w, e),
            Self::CMSG_SET_ACTIVE_VOICE_CHANNEL(c) => c.write_encrypted_client(w, e),
            Self::CMSG_READY_FOR_ACCOUNT_DATA_TIMES(c) => c.write_encrypted_client(w, e),
        }
    }

    #[cfg(feature = "sync")]
    pub fn write_unencrypted_client<W: std::io::Write>(&self, w: &mut W) -> Result<(), std::io::Error> {
        match self {
            Self::MSG_SET_DUNGEON_DIFFICULTY(c) => c.write_unencrypted_client(w),
            Self::CMSG_CHAR_CREATE(c) => c.write_unencrypted_client(w),
            Self::CMSG_CHAR_ENUM(c) => c.write_unencrypted_client(w),
            Self::CMSG_CHAR_DELETE(c) => c.write_unencrypted_client(w),
            Self::CMSG_PLAYER_LOGIN(c) => c.write_unencrypted_client(w),
            Self::CMSG_PLAYER_LOGOUT(c) => c.write_unencrypted_client(w),
            Self::CMSG_LOGOUT_REQUEST(c) => c.write_unencrypted_client(w),
            Self::CMSG_LOGOUT_CANCEL(c) => c.write_unencrypted_client(w),
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
            Self::CMSG_FORCE_RUN_SPEED_CHANGE_ACK(c) => c.write_unencrypted_client(w),
            Self::CMSG_FORCE_RUN_BACK_SPEED_CHANGE_ACK(c) => c.write_unencrypted_client(w),
            Self::CMSG_FORCE_SWIM_SPEED_CHANGE_ACK(c) => c.write_unencrypted_client(w),
            Self::CMSG_FORCE_MOVE_ROOT_ACK(c) => c.write_unencrypted_client(w),
            Self::CMSG_FORCE_MOVE_UNROOT_ACK(c) => c.write_unencrypted_client(w),
            Self::MSG_MOVE_HEARTBEAT(c) => c.write_unencrypted_client(w),
            Self::CMSG_NEXT_CINEMATIC_CAMERA(c) => c.write_unencrypted_client(w),
            Self::CMSG_COMPLETE_CINEMATIC(c) => c.write_unencrypted_client(w),
            Self::CMSG_TUTORIAL_FLAG(c) => c.write_unencrypted_client(w),
            Self::CMSG_TUTORIAL_CLEAR(c) => c.write_unencrypted_client(w),
            Self::CMSG_TUTORIAL_RESET(c) => c.write_unencrypted_client(w),
            Self::CMSG_EMOTE(c) => c.write_unencrypted_client(w),
            Self::CMSG_TEXT_EMOTE(c) => c.write_unencrypted_client(w),
            Self::CMSG_SET_ACTION_BUTTON(c) => c.write_unencrypted_client(w),
            Self::CMSG_CANCEL_CHANNELLING(c) => c.write_unencrypted_client(w),
            Self::CMSG_SET_SELECTION(c) => c.write_unencrypted_client(w),
            Self::CMSG_SET_TARGET_OBSOLETE(c) => c.write_unencrypted_client(w),
            Self::CMSG_ATTACKSWING(c) => c.write_unencrypted_client(w),
            Self::CMSG_ATTACKSTOP(c) => c.write_unencrypted_client(w),
            Self::CMSG_REPOP_REQUEST(c) => c.write_unencrypted_client(w),
            Self::CMSG_ACTIVATETAXI(c) => c.write_unencrypted_client(w),
            Self::CMSG_PING(c) => c.write_unencrypted_client(w),
            Self::CMSG_SETSHEATHED(c) => c.write_unencrypted_client(w),
            Self::CMSG_AUTH_SESSION(c) => c.write_unencrypted_client(w),
            Self::CMSG_ZONEUPDATE(c) => c.write_unencrypted_client(w),
            Self::CMSG_REQUEST_ACCOUNT_DATA(c) => c.write_unencrypted_client(w),
            Self::CMSG_UPDATE_ACCOUNT_DATA(c) => c.write_unencrypted_client(w),
            Self::CMSG_CHAT_IGNORED(c) => c.write_unencrypted_client(w),
            Self::CMSG_BATTLEFIELD_LIST(c) => c.write_unencrypted_client(w),
            Self::MSG_AUCTION_HELLO(c) => c.write_unencrypted_client(w),
            Self::CMSG_AUCTION_SELL_ITEM(c) => c.write_unencrypted_client(w),
            Self::CMSG_AUCTION_REMOVE_ITEM(c) => c.write_unencrypted_client(w),
            Self::CMSG_AUCTION_LIST_ITEMS(c) => c.write_unencrypted_client(w),
            Self::CMSG_AUCTION_LIST_OWNER_ITEMS(c) => c.write_unencrypted_client(w),
            Self::CMSG_AUCTION_PLACE_BID(c) => c.write_unencrypted_client(w),
            Self::CMSG_AUCTION_LIST_BIDDER_ITEMS(c) => c.write_unencrypted_client(w),
            Self::CMSG_SET_ACTIVE_MOVER(c) => c.write_unencrypted_client(w),
            Self::CMSG_SET_ACTIONBAR_TOGGLES(c) => c.write_unencrypted_client(w),
            Self::CMSG_CHAR_RENAME(c) => c.write_unencrypted_client(w),
            Self::CMSG_MOVE_FALL_RESET(c) => c.write_unencrypted_client(w),
            Self::CMSG_FORCE_WALK_SPEED_CHANGE_ACK(c) => c.write_unencrypted_client(w),
            Self::CMSG_FORCE_SWIM_BACK_SPEED_CHANGE_ACK(c) => c.write_unencrypted_client(w),
            Self::CMSG_FORCE_TURN_RATE_CHANGE_ACK(c) => c.write_unencrypted_client(w),
            Self::CMSG_ACTIVATETAXIEXPRESS(c) => c.write_unencrypted_client(w),
            Self::CMSG_MOVE_SET_FLY(c) => c.write_unencrypted_client(w),
            Self::MSG_MOVE_START_ASCEND(c) => c.write_unencrypted_client(w),
            Self::MSG_MOVE_STOP_ASCEND(c) => c.write_unencrypted_client(w),
            Self::CMSG_REALM_SPLIT(c) => c.write_unencrypted_client(w),
            Self::CMSG_MOVE_CHNG_TRANSPORT(c) => c.write_unencrypted_client(w),
            Self::MSG_MOVE_START_DESCEND(c) => c.write_unencrypted_client(w),
            Self::CMSG_SET_ACTIVE_VOICE_CHANNEL(c) => c.write_unencrypted_client(w),
            Self::CMSG_READY_FOR_ACCOUNT_DATA_TIMES(c) => c.write_unencrypted_client(w),
        }
    }

    #[cfg(feature = "tokio")]
    pub async fn tokio_write_encrypted_client<W: tokio::io::AsyncWriteExt + Unpin + Send>(&self, w: &mut W, e: &mut ClientEncrypterHalf) -> Result<(), std::io::Error> {
        match self {
            Self::MSG_SET_DUNGEON_DIFFICULTY(c) => c.tokio_write_encrypted_client(w, e).await,
            Self::CMSG_CHAR_CREATE(c) => c.tokio_write_encrypted_client(w, e).await,
            Self::CMSG_CHAR_ENUM(c) => c.tokio_write_encrypted_client(w, e).await,
            Self::CMSG_CHAR_DELETE(c) => c.tokio_write_encrypted_client(w, e).await,
            Self::CMSG_PLAYER_LOGIN(c) => c.tokio_write_encrypted_client(w, e).await,
            Self::CMSG_PLAYER_LOGOUT(c) => c.tokio_write_encrypted_client(w, e).await,
            Self::CMSG_LOGOUT_REQUEST(c) => c.tokio_write_encrypted_client(w, e).await,
            Self::CMSG_LOGOUT_CANCEL(c) => c.tokio_write_encrypted_client(w, e).await,
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
            Self::CMSG_FORCE_RUN_SPEED_CHANGE_ACK(c) => c.tokio_write_encrypted_client(w, e).await,
            Self::CMSG_FORCE_RUN_BACK_SPEED_CHANGE_ACK(c) => c.tokio_write_encrypted_client(w, e).await,
            Self::CMSG_FORCE_SWIM_SPEED_CHANGE_ACK(c) => c.tokio_write_encrypted_client(w, e).await,
            Self::CMSG_FORCE_MOVE_ROOT_ACK(c) => c.tokio_write_encrypted_client(w, e).await,
            Self::CMSG_FORCE_MOVE_UNROOT_ACK(c) => c.tokio_write_encrypted_client(w, e).await,
            Self::MSG_MOVE_HEARTBEAT(c) => c.tokio_write_encrypted_client(w, e).await,
            Self::CMSG_NEXT_CINEMATIC_CAMERA(c) => c.tokio_write_encrypted_client(w, e).await,
            Self::CMSG_COMPLETE_CINEMATIC(c) => c.tokio_write_encrypted_client(w, e).await,
            Self::CMSG_TUTORIAL_FLAG(c) => c.tokio_write_encrypted_client(w, e).await,
            Self::CMSG_TUTORIAL_CLEAR(c) => c.tokio_write_encrypted_client(w, e).await,
            Self::CMSG_TUTORIAL_RESET(c) => c.tokio_write_encrypted_client(w, e).await,
            Self::CMSG_EMOTE(c) => c.tokio_write_encrypted_client(w, e).await,
            Self::CMSG_TEXT_EMOTE(c) => c.tokio_write_encrypted_client(w, e).await,
            Self::CMSG_SET_ACTION_BUTTON(c) => c.tokio_write_encrypted_client(w, e).await,
            Self::CMSG_CANCEL_CHANNELLING(c) => c.tokio_write_encrypted_client(w, e).await,
            Self::CMSG_SET_SELECTION(c) => c.tokio_write_encrypted_client(w, e).await,
            Self::CMSG_SET_TARGET_OBSOLETE(c) => c.tokio_write_encrypted_client(w, e).await,
            Self::CMSG_ATTACKSWING(c) => c.tokio_write_encrypted_client(w, e).await,
            Self::CMSG_ATTACKSTOP(c) => c.tokio_write_encrypted_client(w, e).await,
            Self::CMSG_REPOP_REQUEST(c) => c.tokio_write_encrypted_client(w, e).await,
            Self::CMSG_ACTIVATETAXI(c) => c.tokio_write_encrypted_client(w, e).await,
            Self::CMSG_PING(c) => c.tokio_write_encrypted_client(w, e).await,
            Self::CMSG_SETSHEATHED(c) => c.tokio_write_encrypted_client(w, e).await,
            Self::CMSG_AUTH_SESSION(c) => c.tokio_write_encrypted_client(w, e).await,
            Self::CMSG_ZONEUPDATE(c) => c.tokio_write_encrypted_client(w, e).await,
            Self::CMSG_REQUEST_ACCOUNT_DATA(c) => c.tokio_write_encrypted_client(w, e).await,
            Self::CMSG_UPDATE_ACCOUNT_DATA(c) => c.tokio_write_encrypted_client(w, e).await,
            Self::CMSG_CHAT_IGNORED(c) => c.tokio_write_encrypted_client(w, e).await,
            Self::CMSG_BATTLEFIELD_LIST(c) => c.tokio_write_encrypted_client(w, e).await,
            Self::MSG_AUCTION_HELLO(c) => c.tokio_write_encrypted_client(w, e).await,
            Self::CMSG_AUCTION_SELL_ITEM(c) => c.tokio_write_encrypted_client(w, e).await,
            Self::CMSG_AUCTION_REMOVE_ITEM(c) => c.tokio_write_encrypted_client(w, e).await,
            Self::CMSG_AUCTION_LIST_ITEMS(c) => c.tokio_write_encrypted_client(w, e).await,
            Self::CMSG_AUCTION_LIST_OWNER_ITEMS(c) => c.tokio_write_encrypted_client(w, e).await,
            Self::CMSG_AUCTION_PLACE_BID(c) => c.tokio_write_encrypted_client(w, e).await,
            Self::CMSG_AUCTION_LIST_BIDDER_ITEMS(c) => c.tokio_write_encrypted_client(w, e).await,
            Self::CMSG_SET_ACTIVE_MOVER(c) => c.tokio_write_encrypted_client(w, e).await,
            Self::CMSG_SET_ACTIONBAR_TOGGLES(c) => c.tokio_write_encrypted_client(w, e).await,
            Self::CMSG_CHAR_RENAME(c) => c.tokio_write_encrypted_client(w, e).await,
            Self::CMSG_MOVE_FALL_RESET(c) => c.tokio_write_encrypted_client(w, e).await,
            Self::CMSG_FORCE_WALK_SPEED_CHANGE_ACK(c) => c.tokio_write_encrypted_client(w, e).await,
            Self::CMSG_FORCE_SWIM_BACK_SPEED_CHANGE_ACK(c) => c.tokio_write_encrypted_client(w, e).await,
            Self::CMSG_FORCE_TURN_RATE_CHANGE_ACK(c) => c.tokio_write_encrypted_client(w, e).await,
            Self::CMSG_ACTIVATETAXIEXPRESS(c) => c.tokio_write_encrypted_client(w, e).await,
            Self::CMSG_MOVE_SET_FLY(c) => c.tokio_write_encrypted_client(w, e).await,
            Self::MSG_MOVE_START_ASCEND(c) => c.tokio_write_encrypted_client(w, e).await,
            Self::MSG_MOVE_STOP_ASCEND(c) => c.tokio_write_encrypted_client(w, e).await,
            Self::CMSG_REALM_SPLIT(c) => c.tokio_write_encrypted_client(w, e).await,
            Self::CMSG_MOVE_CHNG_TRANSPORT(c) => c.tokio_write_encrypted_client(w, e).await,
            Self::MSG_MOVE_START_DESCEND(c) => c.tokio_write_encrypted_client(w, e).await,
            Self::CMSG_SET_ACTIVE_VOICE_CHANNEL(c) => c.tokio_write_encrypted_client(w, e).await,
            Self::CMSG_READY_FOR_ACCOUNT_DATA_TIMES(c) => c.tokio_write_encrypted_client(w, e).await,
        }
    }

    #[cfg(feature = "tokio")]
    pub async fn tokio_write_unencrypted_client<W: tokio::io::AsyncWriteExt + Unpin + Send>(&self, w: &mut W) -> Result<(), std::io::Error> {
        match self {
            Self::MSG_SET_DUNGEON_DIFFICULTY(c) => c.tokio_write_unencrypted_client(w).await,
            Self::CMSG_CHAR_CREATE(c) => c.tokio_write_unencrypted_client(w).await,
            Self::CMSG_CHAR_ENUM(c) => c.tokio_write_unencrypted_client(w).await,
            Self::CMSG_CHAR_DELETE(c) => c.tokio_write_unencrypted_client(w).await,
            Self::CMSG_PLAYER_LOGIN(c) => c.tokio_write_unencrypted_client(w).await,
            Self::CMSG_PLAYER_LOGOUT(c) => c.tokio_write_unencrypted_client(w).await,
            Self::CMSG_LOGOUT_REQUEST(c) => c.tokio_write_unencrypted_client(w).await,
            Self::CMSG_LOGOUT_CANCEL(c) => c.tokio_write_unencrypted_client(w).await,
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
            Self::CMSG_FORCE_RUN_SPEED_CHANGE_ACK(c) => c.tokio_write_unencrypted_client(w).await,
            Self::CMSG_FORCE_RUN_BACK_SPEED_CHANGE_ACK(c) => c.tokio_write_unencrypted_client(w).await,
            Self::CMSG_FORCE_SWIM_SPEED_CHANGE_ACK(c) => c.tokio_write_unencrypted_client(w).await,
            Self::CMSG_FORCE_MOVE_ROOT_ACK(c) => c.tokio_write_unencrypted_client(w).await,
            Self::CMSG_FORCE_MOVE_UNROOT_ACK(c) => c.tokio_write_unencrypted_client(w).await,
            Self::MSG_MOVE_HEARTBEAT(c) => c.tokio_write_unencrypted_client(w).await,
            Self::CMSG_NEXT_CINEMATIC_CAMERA(c) => c.tokio_write_unencrypted_client(w).await,
            Self::CMSG_COMPLETE_CINEMATIC(c) => c.tokio_write_unencrypted_client(w).await,
            Self::CMSG_TUTORIAL_FLAG(c) => c.tokio_write_unencrypted_client(w).await,
            Self::CMSG_TUTORIAL_CLEAR(c) => c.tokio_write_unencrypted_client(w).await,
            Self::CMSG_TUTORIAL_RESET(c) => c.tokio_write_unencrypted_client(w).await,
            Self::CMSG_EMOTE(c) => c.tokio_write_unencrypted_client(w).await,
            Self::CMSG_TEXT_EMOTE(c) => c.tokio_write_unencrypted_client(w).await,
            Self::CMSG_SET_ACTION_BUTTON(c) => c.tokio_write_unencrypted_client(w).await,
            Self::CMSG_CANCEL_CHANNELLING(c) => c.tokio_write_unencrypted_client(w).await,
            Self::CMSG_SET_SELECTION(c) => c.tokio_write_unencrypted_client(w).await,
            Self::CMSG_SET_TARGET_OBSOLETE(c) => c.tokio_write_unencrypted_client(w).await,
            Self::CMSG_ATTACKSWING(c) => c.tokio_write_unencrypted_client(w).await,
            Self::CMSG_ATTACKSTOP(c) => c.tokio_write_unencrypted_client(w).await,
            Self::CMSG_REPOP_REQUEST(c) => c.tokio_write_unencrypted_client(w).await,
            Self::CMSG_ACTIVATETAXI(c) => c.tokio_write_unencrypted_client(w).await,
            Self::CMSG_PING(c) => c.tokio_write_unencrypted_client(w).await,
            Self::CMSG_SETSHEATHED(c) => c.tokio_write_unencrypted_client(w).await,
            Self::CMSG_AUTH_SESSION(c) => c.tokio_write_unencrypted_client(w).await,
            Self::CMSG_ZONEUPDATE(c) => c.tokio_write_unencrypted_client(w).await,
            Self::CMSG_REQUEST_ACCOUNT_DATA(c) => c.tokio_write_unencrypted_client(w).await,
            Self::CMSG_UPDATE_ACCOUNT_DATA(c) => c.tokio_write_unencrypted_client(w).await,
            Self::CMSG_CHAT_IGNORED(c) => c.tokio_write_unencrypted_client(w).await,
            Self::CMSG_BATTLEFIELD_LIST(c) => c.tokio_write_unencrypted_client(w).await,
            Self::MSG_AUCTION_HELLO(c) => c.tokio_write_unencrypted_client(w).await,
            Self::CMSG_AUCTION_SELL_ITEM(c) => c.tokio_write_unencrypted_client(w).await,
            Self::CMSG_AUCTION_REMOVE_ITEM(c) => c.tokio_write_unencrypted_client(w).await,
            Self::CMSG_AUCTION_LIST_ITEMS(c) => c.tokio_write_unencrypted_client(w).await,
            Self::CMSG_AUCTION_LIST_OWNER_ITEMS(c) => c.tokio_write_unencrypted_client(w).await,
            Self::CMSG_AUCTION_PLACE_BID(c) => c.tokio_write_unencrypted_client(w).await,
            Self::CMSG_AUCTION_LIST_BIDDER_ITEMS(c) => c.tokio_write_unencrypted_client(w).await,
            Self::CMSG_SET_ACTIVE_MOVER(c) => c.tokio_write_unencrypted_client(w).await,
            Self::CMSG_SET_ACTIONBAR_TOGGLES(c) => c.tokio_write_unencrypted_client(w).await,
            Self::CMSG_CHAR_RENAME(c) => c.tokio_write_unencrypted_client(w).await,
            Self::CMSG_MOVE_FALL_RESET(c) => c.tokio_write_unencrypted_client(w).await,
            Self::CMSG_FORCE_WALK_SPEED_CHANGE_ACK(c) => c.tokio_write_unencrypted_client(w).await,
            Self::CMSG_FORCE_SWIM_BACK_SPEED_CHANGE_ACK(c) => c.tokio_write_unencrypted_client(w).await,
            Self::CMSG_FORCE_TURN_RATE_CHANGE_ACK(c) => c.tokio_write_unencrypted_client(w).await,
            Self::CMSG_ACTIVATETAXIEXPRESS(c) => c.tokio_write_unencrypted_client(w).await,
            Self::CMSG_MOVE_SET_FLY(c) => c.tokio_write_unencrypted_client(w).await,
            Self::MSG_MOVE_START_ASCEND(c) => c.tokio_write_unencrypted_client(w).await,
            Self::MSG_MOVE_STOP_ASCEND(c) => c.tokio_write_unencrypted_client(w).await,
            Self::CMSG_REALM_SPLIT(c) => c.tokio_write_unencrypted_client(w).await,
            Self::CMSG_MOVE_CHNG_TRANSPORT(c) => c.tokio_write_unencrypted_client(w).await,
            Self::MSG_MOVE_START_DESCEND(c) => c.tokio_write_unencrypted_client(w).await,
            Self::CMSG_SET_ACTIVE_VOICE_CHANNEL(c) => c.tokio_write_unencrypted_client(w).await,
            Self::CMSG_READY_FOR_ACCOUNT_DATA_TIMES(c) => c.tokio_write_unencrypted_client(w).await,
        }
    }

    #[cfg(feature = "async-std")]
    pub async fn astd_write_encrypted_client<W: async_std::io::WriteExt + Unpin + Send>(&self, w: &mut W, e: &mut ClientEncrypterHalf) -> Result<(), std::io::Error> {
        match self {
            Self::MSG_SET_DUNGEON_DIFFICULTY(c) => c.astd_write_encrypted_client(w, e).await,
            Self::CMSG_CHAR_CREATE(c) => c.astd_write_encrypted_client(w, e).await,
            Self::CMSG_CHAR_ENUM(c) => c.astd_write_encrypted_client(w, e).await,
            Self::CMSG_CHAR_DELETE(c) => c.astd_write_encrypted_client(w, e).await,
            Self::CMSG_PLAYER_LOGIN(c) => c.astd_write_encrypted_client(w, e).await,
            Self::CMSG_PLAYER_LOGOUT(c) => c.astd_write_encrypted_client(w, e).await,
            Self::CMSG_LOGOUT_REQUEST(c) => c.astd_write_encrypted_client(w, e).await,
            Self::CMSG_LOGOUT_CANCEL(c) => c.astd_write_encrypted_client(w, e).await,
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
            Self::CMSG_FORCE_RUN_SPEED_CHANGE_ACK(c) => c.astd_write_encrypted_client(w, e).await,
            Self::CMSG_FORCE_RUN_BACK_SPEED_CHANGE_ACK(c) => c.astd_write_encrypted_client(w, e).await,
            Self::CMSG_FORCE_SWIM_SPEED_CHANGE_ACK(c) => c.astd_write_encrypted_client(w, e).await,
            Self::CMSG_FORCE_MOVE_ROOT_ACK(c) => c.astd_write_encrypted_client(w, e).await,
            Self::CMSG_FORCE_MOVE_UNROOT_ACK(c) => c.astd_write_encrypted_client(w, e).await,
            Self::MSG_MOVE_HEARTBEAT(c) => c.astd_write_encrypted_client(w, e).await,
            Self::CMSG_NEXT_CINEMATIC_CAMERA(c) => c.astd_write_encrypted_client(w, e).await,
            Self::CMSG_COMPLETE_CINEMATIC(c) => c.astd_write_encrypted_client(w, e).await,
            Self::CMSG_TUTORIAL_FLAG(c) => c.astd_write_encrypted_client(w, e).await,
            Self::CMSG_TUTORIAL_CLEAR(c) => c.astd_write_encrypted_client(w, e).await,
            Self::CMSG_TUTORIAL_RESET(c) => c.astd_write_encrypted_client(w, e).await,
            Self::CMSG_EMOTE(c) => c.astd_write_encrypted_client(w, e).await,
            Self::CMSG_TEXT_EMOTE(c) => c.astd_write_encrypted_client(w, e).await,
            Self::CMSG_SET_ACTION_BUTTON(c) => c.astd_write_encrypted_client(w, e).await,
            Self::CMSG_CANCEL_CHANNELLING(c) => c.astd_write_encrypted_client(w, e).await,
            Self::CMSG_SET_SELECTION(c) => c.astd_write_encrypted_client(w, e).await,
            Self::CMSG_SET_TARGET_OBSOLETE(c) => c.astd_write_encrypted_client(w, e).await,
            Self::CMSG_ATTACKSWING(c) => c.astd_write_encrypted_client(w, e).await,
            Self::CMSG_ATTACKSTOP(c) => c.astd_write_encrypted_client(w, e).await,
            Self::CMSG_REPOP_REQUEST(c) => c.astd_write_encrypted_client(w, e).await,
            Self::CMSG_ACTIVATETAXI(c) => c.astd_write_encrypted_client(w, e).await,
            Self::CMSG_PING(c) => c.astd_write_encrypted_client(w, e).await,
            Self::CMSG_SETSHEATHED(c) => c.astd_write_encrypted_client(w, e).await,
            Self::CMSG_AUTH_SESSION(c) => c.astd_write_encrypted_client(w, e).await,
            Self::CMSG_ZONEUPDATE(c) => c.astd_write_encrypted_client(w, e).await,
            Self::CMSG_REQUEST_ACCOUNT_DATA(c) => c.astd_write_encrypted_client(w, e).await,
            Self::CMSG_UPDATE_ACCOUNT_DATA(c) => c.astd_write_encrypted_client(w, e).await,
            Self::CMSG_CHAT_IGNORED(c) => c.astd_write_encrypted_client(w, e).await,
            Self::CMSG_BATTLEFIELD_LIST(c) => c.astd_write_encrypted_client(w, e).await,
            Self::MSG_AUCTION_HELLO(c) => c.astd_write_encrypted_client(w, e).await,
            Self::CMSG_AUCTION_SELL_ITEM(c) => c.astd_write_encrypted_client(w, e).await,
            Self::CMSG_AUCTION_REMOVE_ITEM(c) => c.astd_write_encrypted_client(w, e).await,
            Self::CMSG_AUCTION_LIST_ITEMS(c) => c.astd_write_encrypted_client(w, e).await,
            Self::CMSG_AUCTION_LIST_OWNER_ITEMS(c) => c.astd_write_encrypted_client(w, e).await,
            Self::CMSG_AUCTION_PLACE_BID(c) => c.astd_write_encrypted_client(w, e).await,
            Self::CMSG_AUCTION_LIST_BIDDER_ITEMS(c) => c.astd_write_encrypted_client(w, e).await,
            Self::CMSG_SET_ACTIVE_MOVER(c) => c.astd_write_encrypted_client(w, e).await,
            Self::CMSG_SET_ACTIONBAR_TOGGLES(c) => c.astd_write_encrypted_client(w, e).await,
            Self::CMSG_CHAR_RENAME(c) => c.astd_write_encrypted_client(w, e).await,
            Self::CMSG_MOVE_FALL_RESET(c) => c.astd_write_encrypted_client(w, e).await,
            Self::CMSG_FORCE_WALK_SPEED_CHANGE_ACK(c) => c.astd_write_encrypted_client(w, e).await,
            Self::CMSG_FORCE_SWIM_BACK_SPEED_CHANGE_ACK(c) => c.astd_write_encrypted_client(w, e).await,
            Self::CMSG_FORCE_TURN_RATE_CHANGE_ACK(c) => c.astd_write_encrypted_client(w, e).await,
            Self::CMSG_ACTIVATETAXIEXPRESS(c) => c.astd_write_encrypted_client(w, e).await,
            Self::CMSG_MOVE_SET_FLY(c) => c.astd_write_encrypted_client(w, e).await,
            Self::MSG_MOVE_START_ASCEND(c) => c.astd_write_encrypted_client(w, e).await,
            Self::MSG_MOVE_STOP_ASCEND(c) => c.astd_write_encrypted_client(w, e).await,
            Self::CMSG_REALM_SPLIT(c) => c.astd_write_encrypted_client(w, e).await,
            Self::CMSG_MOVE_CHNG_TRANSPORT(c) => c.astd_write_encrypted_client(w, e).await,
            Self::MSG_MOVE_START_DESCEND(c) => c.astd_write_encrypted_client(w, e).await,
            Self::CMSG_SET_ACTIVE_VOICE_CHANNEL(c) => c.astd_write_encrypted_client(w, e).await,
            Self::CMSG_READY_FOR_ACCOUNT_DATA_TIMES(c) => c.astd_write_encrypted_client(w, e).await,
        }
    }

    #[cfg(feature = "async-std")]
    pub async fn astd_write_unencrypted_client<W: async_std::io::WriteExt + Unpin + Send>(&self, w: &mut W) -> Result<(), std::io::Error> {
        match self {
            Self::MSG_SET_DUNGEON_DIFFICULTY(c) => c.astd_write_unencrypted_client(w).await,
            Self::CMSG_CHAR_CREATE(c) => c.astd_write_unencrypted_client(w).await,
            Self::CMSG_CHAR_ENUM(c) => c.astd_write_unencrypted_client(w).await,
            Self::CMSG_CHAR_DELETE(c) => c.astd_write_unencrypted_client(w).await,
            Self::CMSG_PLAYER_LOGIN(c) => c.astd_write_unencrypted_client(w).await,
            Self::CMSG_PLAYER_LOGOUT(c) => c.astd_write_unencrypted_client(w).await,
            Self::CMSG_LOGOUT_REQUEST(c) => c.astd_write_unencrypted_client(w).await,
            Self::CMSG_LOGOUT_CANCEL(c) => c.astd_write_unencrypted_client(w).await,
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
            Self::CMSG_FORCE_RUN_SPEED_CHANGE_ACK(c) => c.astd_write_unencrypted_client(w).await,
            Self::CMSG_FORCE_RUN_BACK_SPEED_CHANGE_ACK(c) => c.astd_write_unencrypted_client(w).await,
            Self::CMSG_FORCE_SWIM_SPEED_CHANGE_ACK(c) => c.astd_write_unencrypted_client(w).await,
            Self::CMSG_FORCE_MOVE_ROOT_ACK(c) => c.astd_write_unencrypted_client(w).await,
            Self::CMSG_FORCE_MOVE_UNROOT_ACK(c) => c.astd_write_unencrypted_client(w).await,
            Self::MSG_MOVE_HEARTBEAT(c) => c.astd_write_unencrypted_client(w).await,
            Self::CMSG_NEXT_CINEMATIC_CAMERA(c) => c.astd_write_unencrypted_client(w).await,
            Self::CMSG_COMPLETE_CINEMATIC(c) => c.astd_write_unencrypted_client(w).await,
            Self::CMSG_TUTORIAL_FLAG(c) => c.astd_write_unencrypted_client(w).await,
            Self::CMSG_TUTORIAL_CLEAR(c) => c.astd_write_unencrypted_client(w).await,
            Self::CMSG_TUTORIAL_RESET(c) => c.astd_write_unencrypted_client(w).await,
            Self::CMSG_EMOTE(c) => c.astd_write_unencrypted_client(w).await,
            Self::CMSG_TEXT_EMOTE(c) => c.astd_write_unencrypted_client(w).await,
            Self::CMSG_SET_ACTION_BUTTON(c) => c.astd_write_unencrypted_client(w).await,
            Self::CMSG_CANCEL_CHANNELLING(c) => c.astd_write_unencrypted_client(w).await,
            Self::CMSG_SET_SELECTION(c) => c.astd_write_unencrypted_client(w).await,
            Self::CMSG_SET_TARGET_OBSOLETE(c) => c.astd_write_unencrypted_client(w).await,
            Self::CMSG_ATTACKSWING(c) => c.astd_write_unencrypted_client(w).await,
            Self::CMSG_ATTACKSTOP(c) => c.astd_write_unencrypted_client(w).await,
            Self::CMSG_REPOP_REQUEST(c) => c.astd_write_unencrypted_client(w).await,
            Self::CMSG_ACTIVATETAXI(c) => c.astd_write_unencrypted_client(w).await,
            Self::CMSG_PING(c) => c.astd_write_unencrypted_client(w).await,
            Self::CMSG_SETSHEATHED(c) => c.astd_write_unencrypted_client(w).await,
            Self::CMSG_AUTH_SESSION(c) => c.astd_write_unencrypted_client(w).await,
            Self::CMSG_ZONEUPDATE(c) => c.astd_write_unencrypted_client(w).await,
            Self::CMSG_REQUEST_ACCOUNT_DATA(c) => c.astd_write_unencrypted_client(w).await,
            Self::CMSG_UPDATE_ACCOUNT_DATA(c) => c.astd_write_unencrypted_client(w).await,
            Self::CMSG_CHAT_IGNORED(c) => c.astd_write_unencrypted_client(w).await,
            Self::CMSG_BATTLEFIELD_LIST(c) => c.astd_write_unencrypted_client(w).await,
            Self::MSG_AUCTION_HELLO(c) => c.astd_write_unencrypted_client(w).await,
            Self::CMSG_AUCTION_SELL_ITEM(c) => c.astd_write_unencrypted_client(w).await,
            Self::CMSG_AUCTION_REMOVE_ITEM(c) => c.astd_write_unencrypted_client(w).await,
            Self::CMSG_AUCTION_LIST_ITEMS(c) => c.astd_write_unencrypted_client(w).await,
            Self::CMSG_AUCTION_LIST_OWNER_ITEMS(c) => c.astd_write_unencrypted_client(w).await,
            Self::CMSG_AUCTION_PLACE_BID(c) => c.astd_write_unencrypted_client(w).await,
            Self::CMSG_AUCTION_LIST_BIDDER_ITEMS(c) => c.astd_write_unencrypted_client(w).await,
            Self::CMSG_SET_ACTIVE_MOVER(c) => c.astd_write_unencrypted_client(w).await,
            Self::CMSG_SET_ACTIONBAR_TOGGLES(c) => c.astd_write_unencrypted_client(w).await,
            Self::CMSG_CHAR_RENAME(c) => c.astd_write_unencrypted_client(w).await,
            Self::CMSG_MOVE_FALL_RESET(c) => c.astd_write_unencrypted_client(w).await,
            Self::CMSG_FORCE_WALK_SPEED_CHANGE_ACK(c) => c.astd_write_unencrypted_client(w).await,
            Self::CMSG_FORCE_SWIM_BACK_SPEED_CHANGE_ACK(c) => c.astd_write_unencrypted_client(w).await,
            Self::CMSG_FORCE_TURN_RATE_CHANGE_ACK(c) => c.astd_write_unencrypted_client(w).await,
            Self::CMSG_ACTIVATETAXIEXPRESS(c) => c.astd_write_unencrypted_client(w).await,
            Self::CMSG_MOVE_SET_FLY(c) => c.astd_write_unencrypted_client(w).await,
            Self::MSG_MOVE_START_ASCEND(c) => c.astd_write_unencrypted_client(w).await,
            Self::MSG_MOVE_STOP_ASCEND(c) => c.astd_write_unencrypted_client(w).await,
            Self::CMSG_REALM_SPLIT(c) => c.astd_write_unencrypted_client(w).await,
            Self::CMSG_MOVE_CHNG_TRANSPORT(c) => c.astd_write_unencrypted_client(w).await,
            Self::MSG_MOVE_START_DESCEND(c) => c.astd_write_unencrypted_client(w).await,
            Self::CMSG_SET_ACTIVE_VOICE_CHANNEL(c) => c.astd_write_unencrypted_client(w).await,
            Self::CMSG_READY_FOR_ACCOUNT_DATA_TIMES(c) => c.astd_write_unencrypted_client(w).await,
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
            Self::CMSG_MOVE_SET_FLY(c) => Some(&c.info),
            Self::MSG_MOVE_START_ASCEND(c) => Some(&c.info),
            Self::MSG_MOVE_STOP_ASCEND(c) => Some(&c.info),
            Self::CMSG_MOVE_CHNG_TRANSPORT(c) => Some(&c.info),
            Self::MSG_MOVE_START_DESCEND(c) => Some(&c.info),
            _ => None,
        }
    }

}

impl std::fmt::Display for ClientOpcodeMessage {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            ClientOpcodeMessage::MSG_SET_DUNGEON_DIFFICULTY(_) => "MSG_SET_DUNGEON_DIFFICULTY",
            ClientOpcodeMessage::CMSG_CHAR_CREATE(_) => "CMSG_CHAR_CREATE",
            ClientOpcodeMessage::CMSG_CHAR_ENUM(_) => "CMSG_CHAR_ENUM",
            ClientOpcodeMessage::CMSG_CHAR_DELETE(_) => "CMSG_CHAR_DELETE",
            ClientOpcodeMessage::CMSG_PLAYER_LOGIN(_) => "CMSG_PLAYER_LOGIN",
            ClientOpcodeMessage::CMSG_PLAYER_LOGOUT(_) => "CMSG_PLAYER_LOGOUT",
            ClientOpcodeMessage::CMSG_LOGOUT_REQUEST(_) => "CMSG_LOGOUT_REQUEST",
            ClientOpcodeMessage::CMSG_LOGOUT_CANCEL(_) => "CMSG_LOGOUT_CANCEL",
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
            ClientOpcodeMessage::MSG_MOVE_FALL_LAND(_) => "MSG_MOVE_FALL_LAND_Client",
            ClientOpcodeMessage::MSG_MOVE_START_SWIM(_) => "MSG_MOVE_START_SWIM_Client",
            ClientOpcodeMessage::MSG_MOVE_STOP_SWIM(_) => "MSG_MOVE_STOP_SWIM_Client",
            ClientOpcodeMessage::MSG_MOVE_SET_FACING(_) => "MSG_MOVE_SET_FACING_Client",
            ClientOpcodeMessage::MSG_MOVE_SET_PITCH(_) => "MSG_MOVE_SET_PITCH_Client",
            ClientOpcodeMessage::CMSG_FORCE_RUN_SPEED_CHANGE_ACK(_) => "CMSG_FORCE_RUN_SPEED_CHANGE_ACK",
            ClientOpcodeMessage::CMSG_FORCE_RUN_BACK_SPEED_CHANGE_ACK(_) => "CMSG_FORCE_RUN_BACK_SPEED_CHANGE_ACK",
            ClientOpcodeMessage::CMSG_FORCE_SWIM_SPEED_CHANGE_ACK(_) => "CMSG_FORCE_SWIM_SPEED_CHANGE_ACK",
            ClientOpcodeMessage::CMSG_FORCE_MOVE_ROOT_ACK(_) => "CMSG_FORCE_MOVE_ROOT_ACK",
            ClientOpcodeMessage::CMSG_FORCE_MOVE_UNROOT_ACK(_) => "CMSG_FORCE_MOVE_UNROOT_ACK",
            ClientOpcodeMessage::MSG_MOVE_HEARTBEAT(_) => "MSG_MOVE_HEARTBEAT_Client",
            ClientOpcodeMessage::CMSG_NEXT_CINEMATIC_CAMERA(_) => "CMSG_NEXT_CINEMATIC_CAMERA",
            ClientOpcodeMessage::CMSG_COMPLETE_CINEMATIC(_) => "CMSG_COMPLETE_CINEMATIC",
            ClientOpcodeMessage::CMSG_TUTORIAL_FLAG(_) => "CMSG_TUTORIAL_FLAG",
            ClientOpcodeMessage::CMSG_TUTORIAL_CLEAR(_) => "CMSG_TUTORIAL_CLEAR",
            ClientOpcodeMessage::CMSG_TUTORIAL_RESET(_) => "CMSG_TUTORIAL_RESET",
            ClientOpcodeMessage::CMSG_EMOTE(_) => "CMSG_EMOTE",
            ClientOpcodeMessage::CMSG_TEXT_EMOTE(_) => "CMSG_TEXT_EMOTE",
            ClientOpcodeMessage::CMSG_SET_ACTION_BUTTON(_) => "CMSG_SET_ACTION_BUTTON",
            ClientOpcodeMessage::CMSG_CANCEL_CHANNELLING(_) => "CMSG_CANCEL_CHANNELLING",
            ClientOpcodeMessage::CMSG_SET_SELECTION(_) => "CMSG_SET_SELECTION",
            ClientOpcodeMessage::CMSG_SET_TARGET_OBSOLETE(_) => "CMSG_SET_TARGET_OBSOLETE",
            ClientOpcodeMessage::CMSG_ATTACKSWING(_) => "CMSG_ATTACKSWING",
            ClientOpcodeMessage::CMSG_ATTACKSTOP(_) => "CMSG_ATTACKSTOP",
            ClientOpcodeMessage::CMSG_REPOP_REQUEST(_) => "CMSG_REPOP_REQUEST",
            ClientOpcodeMessage::CMSG_ACTIVATETAXI(_) => "CMSG_ACTIVATETAXI",
            ClientOpcodeMessage::CMSG_PING(_) => "CMSG_PING",
            ClientOpcodeMessage::CMSG_SETSHEATHED(_) => "CMSG_SETSHEATHED",
            ClientOpcodeMessage::CMSG_AUTH_SESSION(_) => "CMSG_AUTH_SESSION",
            ClientOpcodeMessage::CMSG_ZONEUPDATE(_) => "CMSG_ZONEUPDATE",
            ClientOpcodeMessage::CMSG_REQUEST_ACCOUNT_DATA(_) => "CMSG_REQUEST_ACCOUNT_DATA",
            ClientOpcodeMessage::CMSG_UPDATE_ACCOUNT_DATA(_) => "CMSG_UPDATE_ACCOUNT_DATA",
            ClientOpcodeMessage::CMSG_CHAT_IGNORED(_) => "CMSG_CHAT_IGNORED",
            ClientOpcodeMessage::CMSG_BATTLEFIELD_LIST(_) => "CMSG_BATTLEFIELD_LIST",
            ClientOpcodeMessage::MSG_AUCTION_HELLO(_) => "MSG_AUCTION_HELLO_Client",
            ClientOpcodeMessage::CMSG_AUCTION_SELL_ITEM(_) => "CMSG_AUCTION_SELL_ITEM",
            ClientOpcodeMessage::CMSG_AUCTION_REMOVE_ITEM(_) => "CMSG_AUCTION_REMOVE_ITEM",
            ClientOpcodeMessage::CMSG_AUCTION_LIST_ITEMS(_) => "CMSG_AUCTION_LIST_ITEMS",
            ClientOpcodeMessage::CMSG_AUCTION_LIST_OWNER_ITEMS(_) => "CMSG_AUCTION_LIST_OWNER_ITEMS",
            ClientOpcodeMessage::CMSG_AUCTION_PLACE_BID(_) => "CMSG_AUCTION_PLACE_BID",
            ClientOpcodeMessage::CMSG_AUCTION_LIST_BIDDER_ITEMS(_) => "CMSG_AUCTION_LIST_BIDDER_ITEMS",
            ClientOpcodeMessage::CMSG_SET_ACTIVE_MOVER(_) => "CMSG_SET_ACTIVE_MOVER",
            ClientOpcodeMessage::CMSG_SET_ACTIONBAR_TOGGLES(_) => "CMSG_SET_ACTIONBAR_TOGGLES",
            ClientOpcodeMessage::CMSG_CHAR_RENAME(_) => "CMSG_CHAR_RENAME",
            ClientOpcodeMessage::CMSG_MOVE_FALL_RESET(_) => "CMSG_MOVE_FALL_RESET",
            ClientOpcodeMessage::CMSG_FORCE_WALK_SPEED_CHANGE_ACK(_) => "CMSG_FORCE_WALK_SPEED_CHANGE_ACK",
            ClientOpcodeMessage::CMSG_FORCE_SWIM_BACK_SPEED_CHANGE_ACK(_) => "CMSG_FORCE_SWIM_BACK_SPEED_CHANGE_ACK",
            ClientOpcodeMessage::CMSG_FORCE_TURN_RATE_CHANGE_ACK(_) => "CMSG_FORCE_TURN_RATE_CHANGE_ACK",
            ClientOpcodeMessage::CMSG_ACTIVATETAXIEXPRESS(_) => "CMSG_ACTIVATETAXIEXPRESS",
            ClientOpcodeMessage::CMSG_MOVE_SET_FLY(_) => "CMSG_MOVE_SET_FLY",
            ClientOpcodeMessage::MSG_MOVE_START_ASCEND(_) => "MSG_MOVE_START_ASCEND_Client",
            ClientOpcodeMessage::MSG_MOVE_STOP_ASCEND(_) => "MSG_MOVE_STOP_ASCEND_Client",
            ClientOpcodeMessage::CMSG_REALM_SPLIT(_) => "CMSG_REALM_SPLIT",
            ClientOpcodeMessage::CMSG_MOVE_CHNG_TRANSPORT(_) => "CMSG_MOVE_CHNG_TRANSPORT",
            ClientOpcodeMessage::MSG_MOVE_START_DESCEND(_) => "MSG_MOVE_START_DESCEND_Client",
            ClientOpcodeMessage::CMSG_SET_ACTIVE_VOICE_CHANNEL(_) => "CMSG_SET_ACTIVE_VOICE_CHANNEL",
            ClientOpcodeMessage::CMSG_READY_FOR_ACCOUNT_DATA_TIMES(_) => "CMSG_READY_FOR_ACCOUNT_DATA_TIMES",
        })
    }
}

impl From<MSG_SET_DUNGEON_DIFFICULTY> for ClientOpcodeMessage {
    fn from(c: MSG_SET_DUNGEON_DIFFICULTY) -> Self {
        Self::MSG_SET_DUNGEON_DIFFICULTY(c)
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

impl From<CMSG_ACTIVATETAXI> for ClientOpcodeMessage {
    fn from(c: CMSG_ACTIVATETAXI) -> Self {
        Self::CMSG_ACTIVATETAXI(c)
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

impl From<CMSG_ZONEUPDATE> for ClientOpcodeMessage {
    fn from(c: CMSG_ZONEUPDATE) -> Self {
        Self::CMSG_ZONEUPDATE(c)
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

impl From<CMSG_CHAT_IGNORED> for ClientOpcodeMessage {
    fn from(c: CMSG_CHAT_IGNORED) -> Self {
        Self::CMSG_CHAT_IGNORED(c)
    }
}

impl From<CMSG_BATTLEFIELD_LIST> for ClientOpcodeMessage {
    fn from(c: CMSG_BATTLEFIELD_LIST) -> Self {
        Self::CMSG_BATTLEFIELD_LIST(c)
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

impl From<CMSG_SET_ACTIVE_MOVER> for ClientOpcodeMessage {
    fn from(c: CMSG_SET_ACTIVE_MOVER) -> Self {
        Self::CMSG_SET_ACTIVE_MOVER(c)
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

impl From<CMSG_ACTIVATETAXIEXPRESS> for ClientOpcodeMessage {
    fn from(c: CMSG_ACTIVATETAXIEXPRESS) -> Self {
        Self::CMSG_ACTIVATETAXIEXPRESS(c)
    }
}

impl From<CMSG_MOVE_SET_FLY> for ClientOpcodeMessage {
    fn from(c: CMSG_MOVE_SET_FLY) -> Self {
        Self::CMSG_MOVE_SET_FLY(c)
    }
}

impl From<MSG_MOVE_START_ASCEND_Client> for ClientOpcodeMessage {
    fn from(c: MSG_MOVE_START_ASCEND_Client) -> Self {
        Self::MSG_MOVE_START_ASCEND(c)
    }
}

impl From<MSG_MOVE_STOP_ASCEND_Client> for ClientOpcodeMessage {
    fn from(c: MSG_MOVE_STOP_ASCEND_Client) -> Self {
        Self::MSG_MOVE_STOP_ASCEND(c)
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

impl From<MSG_MOVE_START_DESCEND_Client> for ClientOpcodeMessage {
    fn from(c: MSG_MOVE_START_DESCEND_Client) -> Self {
        Self::MSG_MOVE_START_DESCEND(c)
    }
}

impl From<CMSG_SET_ACTIVE_VOICE_CHANNEL> for ClientOpcodeMessage {
    fn from(c: CMSG_SET_ACTIVE_VOICE_CHANNEL) -> Self {
        Self::CMSG_SET_ACTIVE_VOICE_CHANNEL(c)
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
use crate::world::wrath::SMSG_CHARACTER_LOGIN_FAILED;
use crate::world::wrath::SMSG_LOGIN_SETTIMESPEED;
use crate::world::wrath::SMSG_CONTACT_LIST;
use crate::world::wrath::SMSG_CHANNEL_NOTIFY;
use crate::world::wrath::SMSG_CHANNEL_LIST;
use crate::world::wrath::SMSG_UPDATE_OBJECT;
use crate::world::wrath::MSG_MOVE_START_FORWARD_Server;
use crate::world::wrath::MSG_MOVE_START_BACKWARD_Server;
use crate::world::wrath::MSG_MOVE_STOP_Server;
use crate::world::wrath::MSG_MOVE_START_STRAFE_LEFT_Server;
use crate::world::wrath::MSG_MOVE_START_STRAFE_RIGHT_Server;
use crate::world::wrath::MSG_MOVE_STOP_STRAFE_Server;
use crate::world::wrath::MSG_MOVE_JUMP_Server;
use crate::world::wrath::MSG_MOVE_START_TURN_LEFT_Server;
use crate::world::wrath::MSG_MOVE_START_TURN_RIGHT_Server;
use crate::world::wrath::MSG_MOVE_STOP_TURN_Server;
use crate::world::wrath::MSG_MOVE_START_PITCH_UP_Server;
use crate::world::wrath::MSG_MOVE_START_PITCH_DOWN_Server;
use crate::world::wrath::MSG_MOVE_STOP_PITCH_Server;
use crate::world::wrath::MSG_MOVE_SET_RUN_MODE_Server;
use crate::world::wrath::MSG_MOVE_SET_WALK_MODE_Server;
use crate::world::wrath::MSG_MOVE_FALL_LAND_Server;
use crate::world::wrath::MSG_MOVE_START_SWIM_Server;
use crate::world::wrath::MSG_MOVE_STOP_SWIM_Server;
use crate::world::wrath::MSG_MOVE_SET_FACING_Server;
use crate::world::wrath::MSG_MOVE_SET_PITCH_Server;
use crate::world::wrath::MSG_MOVE_HEARTBEAT_Server;
use crate::world::wrath::SMSG_TRIGGER_CINEMATIC;
use crate::world::wrath::SMSG_TUTORIAL_FLAGS;
use crate::world::wrath::SMSG_EMOTE;
use crate::world::wrath::SMSG_TEXT_EMOTE;
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
use crate::world::wrath::SMSG_NOTIFICATION;
use crate::world::wrath::SMSG_PONG;
use crate::world::wrath::SMSG_AUTH_CHALLENGE;
use crate::world::wrath::SMSG_AUTH_RESPONSE;
use crate::world::wrath::SMSG_ENVIRONMENTALDAMAGELOG;
use crate::world::wrath::SMSG_ACCOUNT_DATA_TIMES;
use crate::world::wrath::SMSG_UPDATE_ACCOUNT_DATA;
use crate::world::wrath::SMSG_CHAT_WRONG_FACTION;
use crate::world::wrath::SMSG_LOGIN_VERIFY_WORLD;
use crate::world::wrath::SMSG_ZONE_UNDER_ATTACK;
use crate::world::wrath::MSG_AUCTION_HELLO_Server;
use crate::world::wrath::SMSG_AUCTION_LIST_RESULT;
use crate::world::wrath::SMSG_AUCTION_OWNER_LIST_RESULT;
use crate::world::wrath::SMSG_AUCTION_BIDDER_NOTIFICATION;
use crate::world::wrath::SMSG_AUCTION_OWNER_NOTIFICATION;
use crate::world::wrath::SMSG_PROCRESIST;
use crate::world::wrath::SMSG_AUCTION_BIDDER_LIST_RESULT;
use crate::world::wrath::SMSG_AUCTION_REMOVED_NOTIFICATION;
use crate::world::wrath::SMSG_SERVER_MESSAGE;
use crate::world::wrath::SMSG_CHAT_PLAYER_NOT_FOUND;
use crate::world::wrath::SMSG_DURABILITY_DAMAGE_DEATH;
use crate::world::wrath::SMSG_CHAR_RENAME;
use crate::world::wrath::SMSG_ADDON_INFO;
use crate::world::wrath::SMSG_CHAT_RESTRICTED;
use crate::world::wrath::SMSG_DEFENSE_MESSAGE;
use crate::world::wrath::MSG_MOVE_START_ASCEND_Server;
use crate::world::wrath::MSG_MOVE_STOP_ASCEND_Server;
use crate::world::wrath::SMSG_REALM_SPLIT;
use crate::world::wrath::SMSG_TIME_SYNC_REQ;
use crate::world::wrath::MSG_MOVE_START_DESCEND_Server;
use crate::world::wrath::SMSG_UPDATE_ACCOUNT_DATA_COMPLETE;
use crate::world::wrath::SMSG_CLIENTCACHE_VERSION;

#[derive(Debug, Clone, PartialEq)]
pub enum ServerOpcodeMessage {
    MSG_SET_DUNGEON_DIFFICULTY(MSG_SET_DUNGEON_DIFFICULTY),
    SMSG_CHAR_CREATE(SMSG_CHAR_CREATE),
    SMSG_CHAR_ENUM(SMSG_CHAR_ENUM),
    SMSG_CHAR_DELETE(SMSG_CHAR_DELETE),
    SMSG_CHARACTER_LOGIN_FAILED(SMSG_CHARACTER_LOGIN_FAILED),
    SMSG_LOGIN_SETTIMESPEED(SMSG_LOGIN_SETTIMESPEED),
    SMSG_CONTACT_LIST(SMSG_CONTACT_LIST),
    SMSG_CHANNEL_NOTIFY(SMSG_CHANNEL_NOTIFY),
    SMSG_CHANNEL_LIST(SMSG_CHANNEL_LIST),
    SMSG_UPDATE_OBJECT(SMSG_UPDATE_OBJECT),
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
    MSG_MOVE_FALL_LAND(MSG_MOVE_FALL_LAND_Server),
    MSG_MOVE_START_SWIM(MSG_MOVE_START_SWIM_Server),
    MSG_MOVE_STOP_SWIM(MSG_MOVE_STOP_SWIM_Server),
    MSG_MOVE_SET_FACING(MSG_MOVE_SET_FACING_Server),
    MSG_MOVE_SET_PITCH(MSG_MOVE_SET_PITCH_Server),
    MSG_MOVE_HEARTBEAT(MSG_MOVE_HEARTBEAT_Server),
    SMSG_TRIGGER_CINEMATIC(SMSG_TRIGGER_CINEMATIC),
    SMSG_TUTORIAL_FLAGS(SMSG_TUTORIAL_FLAGS),
    SMSG_EMOTE(SMSG_EMOTE),
    SMSG_TEXT_EMOTE(SMSG_TEXT_EMOTE),
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
    SMSG_NOTIFICATION(SMSG_NOTIFICATION),
    SMSG_PONG(SMSG_PONG),
    SMSG_AUTH_CHALLENGE(SMSG_AUTH_CHALLENGE),
    SMSG_AUTH_RESPONSE(SMSG_AUTH_RESPONSE),
    SMSG_ENVIRONMENTALDAMAGELOG(SMSG_ENVIRONMENTALDAMAGELOG),
    SMSG_ACCOUNT_DATA_TIMES(SMSG_ACCOUNT_DATA_TIMES),
    SMSG_UPDATE_ACCOUNT_DATA(SMSG_UPDATE_ACCOUNT_DATA),
    SMSG_CHAT_WRONG_FACTION(SMSG_CHAT_WRONG_FACTION),
    SMSG_LOGIN_VERIFY_WORLD(SMSG_LOGIN_VERIFY_WORLD),
    SMSG_ZONE_UNDER_ATTACK(SMSG_ZONE_UNDER_ATTACK),
    MSG_AUCTION_HELLO(MSG_AUCTION_HELLO_Server),
    SMSG_AUCTION_LIST_RESULT(SMSG_AUCTION_LIST_RESULT),
    SMSG_AUCTION_OWNER_LIST_RESULT(SMSG_AUCTION_OWNER_LIST_RESULT),
    SMSG_AUCTION_BIDDER_NOTIFICATION(SMSG_AUCTION_BIDDER_NOTIFICATION),
    SMSG_AUCTION_OWNER_NOTIFICATION(SMSG_AUCTION_OWNER_NOTIFICATION),
    SMSG_PROCRESIST(SMSG_PROCRESIST),
    SMSG_AUCTION_BIDDER_LIST_RESULT(SMSG_AUCTION_BIDDER_LIST_RESULT),
    SMSG_AUCTION_REMOVED_NOTIFICATION(SMSG_AUCTION_REMOVED_NOTIFICATION),
    SMSG_SERVER_MESSAGE(SMSG_SERVER_MESSAGE),
    SMSG_CHAT_PLAYER_NOT_FOUND(SMSG_CHAT_PLAYER_NOT_FOUND),
    SMSG_DURABILITY_DAMAGE_DEATH(SMSG_DURABILITY_DAMAGE_DEATH),
    SMSG_CHAR_RENAME(SMSG_CHAR_RENAME),
    SMSG_ADDON_INFO(SMSG_ADDON_INFO),
    SMSG_CHAT_RESTRICTED(SMSG_CHAT_RESTRICTED),
    SMSG_DEFENSE_MESSAGE(SMSG_DEFENSE_MESSAGE),
    MSG_MOVE_START_ASCEND(MSG_MOVE_START_ASCEND_Server),
    MSG_MOVE_STOP_ASCEND(MSG_MOVE_STOP_ASCEND_Server),
    SMSG_REALM_SPLIT(SMSG_REALM_SPLIT),
    SMSG_TIME_SYNC_REQ(SMSG_TIME_SYNC_REQ),
    MSG_MOVE_START_DESCEND(MSG_MOVE_START_DESCEND_Server),
    SMSG_UPDATE_ACCOUNT_DATA_COMPLETE(SMSG_UPDATE_ACCOUNT_DATA_COMPLETE),
    SMSG_CLIENTCACHE_VERSION(SMSG_CLIENTCACHE_VERSION),
}

impl ServerOpcodeMessage {
    fn read_opcodes(opcode: u16, body_size: u32, mut r: &[u8]) -> std::result::Result<Self, crate::errors::ExpectedOpcodeError> {
        match opcode {
            0x0329 => Ok(Self::MSG_SET_DUNGEON_DIFFICULTY(<MSG_SET_DUNGEON_DIFFICULTY as crate::Message>::read_body(&mut r, body_size)?)),
            0x003A => Ok(Self::SMSG_CHAR_CREATE(<SMSG_CHAR_CREATE as crate::Message>::read_body(&mut r, body_size)?)),
            0x003B => Ok(Self::SMSG_CHAR_ENUM(<SMSG_CHAR_ENUM as crate::Message>::read_body(&mut r, body_size)?)),
            0x003C => Ok(Self::SMSG_CHAR_DELETE(<SMSG_CHAR_DELETE as crate::Message>::read_body(&mut r, body_size)?)),
            0x0041 => Ok(Self::SMSG_CHARACTER_LOGIN_FAILED(<SMSG_CHARACTER_LOGIN_FAILED as crate::Message>::read_body(&mut r, body_size)?)),
            0x0042 => Ok(Self::SMSG_LOGIN_SETTIMESPEED(<SMSG_LOGIN_SETTIMESPEED as crate::Message>::read_body(&mut r, body_size)?)),
            0x0067 => Ok(Self::SMSG_CONTACT_LIST(<SMSG_CONTACT_LIST as crate::Message>::read_body(&mut r, body_size)?)),
            0x0099 => Ok(Self::SMSG_CHANNEL_NOTIFY(<SMSG_CHANNEL_NOTIFY as crate::Message>::read_body(&mut r, body_size)?)),
            0x009B => Ok(Self::SMSG_CHANNEL_LIST(<SMSG_CHANNEL_LIST as crate::Message>::read_body(&mut r, body_size)?)),
            0x00A9 => Ok(Self::SMSG_UPDATE_OBJECT(<SMSG_UPDATE_OBJECT as crate::Message>::read_body(&mut r, body_size)?)),
            0x00B5 => Ok(Self::MSG_MOVE_START_FORWARD(<MSG_MOVE_START_FORWARD_Server as crate::Message>::read_body(&mut r, body_size)?)),
            0x00B6 => Ok(Self::MSG_MOVE_START_BACKWARD(<MSG_MOVE_START_BACKWARD_Server as crate::Message>::read_body(&mut r, body_size)?)),
            0x00B7 => Ok(Self::MSG_MOVE_STOP(<MSG_MOVE_STOP_Server as crate::Message>::read_body(&mut r, body_size)?)),
            0x00B8 => Ok(Self::MSG_MOVE_START_STRAFE_LEFT(<MSG_MOVE_START_STRAFE_LEFT_Server as crate::Message>::read_body(&mut r, body_size)?)),
            0x00B9 => Ok(Self::MSG_MOVE_START_STRAFE_RIGHT(<MSG_MOVE_START_STRAFE_RIGHT_Server as crate::Message>::read_body(&mut r, body_size)?)),
            0x00BA => Ok(Self::MSG_MOVE_STOP_STRAFE(<MSG_MOVE_STOP_STRAFE_Server as crate::Message>::read_body(&mut r, body_size)?)),
            0x00BB => Ok(Self::MSG_MOVE_JUMP(<MSG_MOVE_JUMP_Server as crate::Message>::read_body(&mut r, body_size)?)),
            0x00BC => Ok(Self::MSG_MOVE_START_TURN_LEFT(<MSG_MOVE_START_TURN_LEFT_Server as crate::Message>::read_body(&mut r, body_size)?)),
            0x00BD => Ok(Self::MSG_MOVE_START_TURN_RIGHT(<MSG_MOVE_START_TURN_RIGHT_Server as crate::Message>::read_body(&mut r, body_size)?)),
            0x00BE => Ok(Self::MSG_MOVE_STOP_TURN(<MSG_MOVE_STOP_TURN_Server as crate::Message>::read_body(&mut r, body_size)?)),
            0x00BF => Ok(Self::MSG_MOVE_START_PITCH_UP(<MSG_MOVE_START_PITCH_UP_Server as crate::Message>::read_body(&mut r, body_size)?)),
            0x00C0 => Ok(Self::MSG_MOVE_START_PITCH_DOWN(<MSG_MOVE_START_PITCH_DOWN_Server as crate::Message>::read_body(&mut r, body_size)?)),
            0x00C1 => Ok(Self::MSG_MOVE_STOP_PITCH(<MSG_MOVE_STOP_PITCH_Server as crate::Message>::read_body(&mut r, body_size)?)),
            0x00C2 => Ok(Self::MSG_MOVE_SET_RUN_MODE(<MSG_MOVE_SET_RUN_MODE_Server as crate::Message>::read_body(&mut r, body_size)?)),
            0x00C3 => Ok(Self::MSG_MOVE_SET_WALK_MODE(<MSG_MOVE_SET_WALK_MODE_Server as crate::Message>::read_body(&mut r, body_size)?)),
            0x00C9 => Ok(Self::MSG_MOVE_FALL_LAND(<MSG_MOVE_FALL_LAND_Server as crate::Message>::read_body(&mut r, body_size)?)),
            0x00CA => Ok(Self::MSG_MOVE_START_SWIM(<MSG_MOVE_START_SWIM_Server as crate::Message>::read_body(&mut r, body_size)?)),
            0x00CB => Ok(Self::MSG_MOVE_STOP_SWIM(<MSG_MOVE_STOP_SWIM_Server as crate::Message>::read_body(&mut r, body_size)?)),
            0x00DA => Ok(Self::MSG_MOVE_SET_FACING(<MSG_MOVE_SET_FACING_Server as crate::Message>::read_body(&mut r, body_size)?)),
            0x00DB => Ok(Self::MSG_MOVE_SET_PITCH(<MSG_MOVE_SET_PITCH_Server as crate::Message>::read_body(&mut r, body_size)?)),
            0x00EE => Ok(Self::MSG_MOVE_HEARTBEAT(<MSG_MOVE_HEARTBEAT_Server as crate::Message>::read_body(&mut r, body_size)?)),
            0x00FA => Ok(Self::SMSG_TRIGGER_CINEMATIC(<SMSG_TRIGGER_CINEMATIC as crate::Message>::read_body(&mut r, body_size)?)),
            0x00FD => Ok(Self::SMSG_TUTORIAL_FLAGS(<SMSG_TUTORIAL_FLAGS as crate::Message>::read_body(&mut r, body_size)?)),
            0x0103 => Ok(Self::SMSG_EMOTE(<SMSG_EMOTE as crate::Message>::read_body(&mut r, body_size)?)),
            0x0105 => Ok(Self::SMSG_TEXT_EMOTE(<SMSG_TEXT_EMOTE as crate::Message>::read_body(&mut r, body_size)?)),
            0x0129 => Ok(Self::SMSG_ACTION_BUTTONS(<SMSG_ACTION_BUTTONS as crate::Message>::read_body(&mut r, body_size)?)),
            0x013C => Ok(Self::SMSG_AI_REACTION(<SMSG_AI_REACTION as crate::Message>::read_body(&mut r, body_size)?)),
            0x0143 => Ok(Self::SMSG_ATTACKSTART(<SMSG_ATTACKSTART as crate::Message>::read_body(&mut r, body_size)?)),
            0x0144 => Ok(Self::SMSG_ATTACKSTOP(<SMSG_ATTACKSTOP as crate::Message>::read_body(&mut r, body_size)?)),
            0x0145 => Ok(Self::SMSG_ATTACKSWING_NOTINRANGE(<SMSG_ATTACKSWING_NOTINRANGE as crate::Message>::read_body(&mut r, body_size)?)),
            0x0146 => Ok(Self::SMSG_ATTACKSWING_BADFACING(<SMSG_ATTACKSWING_BADFACING as crate::Message>::read_body(&mut r, body_size)?)),
            0x0148 => Ok(Self::SMSG_ATTACKSWING_DEADTARGET(<SMSG_ATTACKSWING_DEADTARGET as crate::Message>::read_body(&mut r, body_size)?)),
            0x0149 => Ok(Self::SMSG_ATTACKSWING_CANT_ATTACK(<SMSG_ATTACKSWING_CANT_ATTACK as crate::Message>::read_body(&mut r, body_size)?)),
            0x014A => Ok(Self::SMSG_ATTACKERSTATEUPDATE(<SMSG_ATTACKERSTATEUPDATE as crate::Message>::read_body(&mut r, body_size)?)),
            0x014E => Ok(Self::SMSG_CANCEL_COMBAT(<SMSG_CANCEL_COMBAT as crate::Message>::read_body(&mut r, body_size)?)),
            0x0155 => Ok(Self::SMSG_BINDPOINTUPDATE(<SMSG_BINDPOINTUPDATE as crate::Message>::read_body(&mut r, body_size)?)),
            0x01CB => Ok(Self::SMSG_NOTIFICATION(<SMSG_NOTIFICATION as crate::Message>::read_body(&mut r, body_size)?)),
            0x01DD => Ok(Self::SMSG_PONG(<SMSG_PONG as crate::Message>::read_body(&mut r, body_size)?)),
            0x01EC => Ok(Self::SMSG_AUTH_CHALLENGE(<SMSG_AUTH_CHALLENGE as crate::Message>::read_body(&mut r, body_size)?)),
            0x01EE => Ok(Self::SMSG_AUTH_RESPONSE(<SMSG_AUTH_RESPONSE as crate::Message>::read_body(&mut r, body_size)?)),
            0x01FC => Ok(Self::SMSG_ENVIRONMENTALDAMAGELOG(<SMSG_ENVIRONMENTALDAMAGELOG as crate::Message>::read_body(&mut r, body_size)?)),
            0x0209 => Ok(Self::SMSG_ACCOUNT_DATA_TIMES(<SMSG_ACCOUNT_DATA_TIMES as crate::Message>::read_body(&mut r, body_size)?)),
            0x020C => Ok(Self::SMSG_UPDATE_ACCOUNT_DATA(<SMSG_UPDATE_ACCOUNT_DATA as crate::Message>::read_body(&mut r, body_size)?)),
            0x0219 => Ok(Self::SMSG_CHAT_WRONG_FACTION(<SMSG_CHAT_WRONG_FACTION as crate::Message>::read_body(&mut r, body_size)?)),
            0x0236 => Ok(Self::SMSG_LOGIN_VERIFY_WORLD(<SMSG_LOGIN_VERIFY_WORLD as crate::Message>::read_body(&mut r, body_size)?)),
            0x0254 => Ok(Self::SMSG_ZONE_UNDER_ATTACK(<SMSG_ZONE_UNDER_ATTACK as crate::Message>::read_body(&mut r, body_size)?)),
            0x0255 => Ok(Self::MSG_AUCTION_HELLO(<MSG_AUCTION_HELLO_Server as crate::Message>::read_body(&mut r, body_size)?)),
            0x025C => Ok(Self::SMSG_AUCTION_LIST_RESULT(<SMSG_AUCTION_LIST_RESULT as crate::Message>::read_body(&mut r, body_size)?)),
            0x025D => Ok(Self::SMSG_AUCTION_OWNER_LIST_RESULT(<SMSG_AUCTION_OWNER_LIST_RESULT as crate::Message>::read_body(&mut r, body_size)?)),
            0x025E => Ok(Self::SMSG_AUCTION_BIDDER_NOTIFICATION(<SMSG_AUCTION_BIDDER_NOTIFICATION as crate::Message>::read_body(&mut r, body_size)?)),
            0x025F => Ok(Self::SMSG_AUCTION_OWNER_NOTIFICATION(<SMSG_AUCTION_OWNER_NOTIFICATION as crate::Message>::read_body(&mut r, body_size)?)),
            0x0260 => Ok(Self::SMSG_PROCRESIST(<SMSG_PROCRESIST as crate::Message>::read_body(&mut r, body_size)?)),
            0x0265 => Ok(Self::SMSG_AUCTION_BIDDER_LIST_RESULT(<SMSG_AUCTION_BIDDER_LIST_RESULT as crate::Message>::read_body(&mut r, body_size)?)),
            0x028D => Ok(Self::SMSG_AUCTION_REMOVED_NOTIFICATION(<SMSG_AUCTION_REMOVED_NOTIFICATION as crate::Message>::read_body(&mut r, body_size)?)),
            0x0291 => Ok(Self::SMSG_SERVER_MESSAGE(<SMSG_SERVER_MESSAGE as crate::Message>::read_body(&mut r, body_size)?)),
            0x02A9 => Ok(Self::SMSG_CHAT_PLAYER_NOT_FOUND(<SMSG_CHAT_PLAYER_NOT_FOUND as crate::Message>::read_body(&mut r, body_size)?)),
            0x02BD => Ok(Self::SMSG_DURABILITY_DAMAGE_DEATH(<SMSG_DURABILITY_DAMAGE_DEATH as crate::Message>::read_body(&mut r, body_size)?)),
            0x02C8 => Ok(Self::SMSG_CHAR_RENAME(<SMSG_CHAR_RENAME as crate::Message>::read_body(&mut r, body_size)?)),
            0x02EF => Ok(Self::SMSG_ADDON_INFO(<SMSG_ADDON_INFO as crate::Message>::read_body(&mut r, body_size)?)),
            0x02FD => Ok(Self::SMSG_CHAT_RESTRICTED(<SMSG_CHAT_RESTRICTED as crate::Message>::read_body(&mut r, body_size)?)),
            0x033A => Ok(Self::SMSG_DEFENSE_MESSAGE(<SMSG_DEFENSE_MESSAGE as crate::Message>::read_body(&mut r, body_size)?)),
            0x0359 => Ok(Self::MSG_MOVE_START_ASCEND(<MSG_MOVE_START_ASCEND_Server as crate::Message>::read_body(&mut r, body_size)?)),
            0x035A => Ok(Self::MSG_MOVE_STOP_ASCEND(<MSG_MOVE_STOP_ASCEND_Server as crate::Message>::read_body(&mut r, body_size)?)),
            0x038B => Ok(Self::SMSG_REALM_SPLIT(<SMSG_REALM_SPLIT as crate::Message>::read_body(&mut r, body_size)?)),
            0x0390 => Ok(Self::SMSG_TIME_SYNC_REQ(<SMSG_TIME_SYNC_REQ as crate::Message>::read_body(&mut r, body_size)?)),
            0x03A7 => Ok(Self::MSG_MOVE_START_DESCEND(<MSG_MOVE_START_DESCEND_Server as crate::Message>::read_body(&mut r, body_size)?)),
            0x0463 => Ok(Self::SMSG_UPDATE_ACCOUNT_DATA_COMPLETE(<SMSG_UPDATE_ACCOUNT_DATA_COMPLETE as crate::Message>::read_body(&mut r, body_size)?)),
            0x04AB => Ok(Self::SMSG_CLIENTCACHE_VERSION(<SMSG_CLIENTCACHE_VERSION as crate::Message>::read_body(&mut r, body_size)?)),
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
    #[cfg(feature = "sync")]
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
    #[cfg(feature = "tokio")]
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
    #[cfg(feature = "async-std")]
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

    #[cfg(feature = "sync")]
    pub fn write_encrypted_server<W: std::io::Write>(&self, w: &mut W, e: &mut ServerEncrypterHalf) -> Result<(), std::io::Error> {
        match self {
            Self::MSG_SET_DUNGEON_DIFFICULTY(c) => c.write_encrypted_server(w, e),
            Self::SMSG_CHAR_CREATE(c) => c.write_encrypted_server(w, e),
            Self::SMSG_CHAR_ENUM(c) => c.write_encrypted_server(w, e),
            Self::SMSG_CHAR_DELETE(c) => c.write_encrypted_server(w, e),
            Self::SMSG_CHARACTER_LOGIN_FAILED(c) => c.write_encrypted_server(w, e),
            Self::SMSG_LOGIN_SETTIMESPEED(c) => c.write_encrypted_server(w, e),
            Self::SMSG_CONTACT_LIST(c) => c.write_encrypted_server(w, e),
            Self::SMSG_CHANNEL_NOTIFY(c) => c.write_encrypted_server(w, e),
            Self::SMSG_CHANNEL_LIST(c) => c.write_encrypted_server(w, e),
            Self::SMSG_UPDATE_OBJECT(c) => c.write_encrypted_server(w, e),
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
            Self::MSG_MOVE_HEARTBEAT(c) => c.write_encrypted_server(w, e),
            Self::SMSG_TRIGGER_CINEMATIC(c) => c.write_encrypted_server(w, e),
            Self::SMSG_TUTORIAL_FLAGS(c) => c.write_encrypted_server(w, e),
            Self::SMSG_EMOTE(c) => c.write_encrypted_server(w, e),
            Self::SMSG_TEXT_EMOTE(c) => c.write_encrypted_server(w, e),
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
            Self::SMSG_NOTIFICATION(c) => c.write_encrypted_server(w, e),
            Self::SMSG_PONG(c) => c.write_encrypted_server(w, e),
            Self::SMSG_AUTH_CHALLENGE(c) => c.write_encrypted_server(w, e),
            Self::SMSG_AUTH_RESPONSE(c) => c.write_encrypted_server(w, e),
            Self::SMSG_ENVIRONMENTALDAMAGELOG(c) => c.write_encrypted_server(w, e),
            Self::SMSG_ACCOUNT_DATA_TIMES(c) => c.write_encrypted_server(w, e),
            Self::SMSG_UPDATE_ACCOUNT_DATA(c) => c.write_encrypted_server(w, e),
            Self::SMSG_CHAT_WRONG_FACTION(c) => c.write_encrypted_server(w, e),
            Self::SMSG_LOGIN_VERIFY_WORLD(c) => c.write_encrypted_server(w, e),
            Self::SMSG_ZONE_UNDER_ATTACK(c) => c.write_encrypted_server(w, e),
            Self::MSG_AUCTION_HELLO(c) => c.write_encrypted_server(w, e),
            Self::SMSG_AUCTION_LIST_RESULT(c) => c.write_encrypted_server(w, e),
            Self::SMSG_AUCTION_OWNER_LIST_RESULT(c) => c.write_encrypted_server(w, e),
            Self::SMSG_AUCTION_BIDDER_NOTIFICATION(c) => c.write_encrypted_server(w, e),
            Self::SMSG_AUCTION_OWNER_NOTIFICATION(c) => c.write_encrypted_server(w, e),
            Self::SMSG_PROCRESIST(c) => c.write_encrypted_server(w, e),
            Self::SMSG_AUCTION_BIDDER_LIST_RESULT(c) => c.write_encrypted_server(w, e),
            Self::SMSG_AUCTION_REMOVED_NOTIFICATION(c) => c.write_encrypted_server(w, e),
            Self::SMSG_SERVER_MESSAGE(c) => c.write_encrypted_server(w, e),
            Self::SMSG_CHAT_PLAYER_NOT_FOUND(c) => c.write_encrypted_server(w, e),
            Self::SMSG_DURABILITY_DAMAGE_DEATH(c) => c.write_encrypted_server(w, e),
            Self::SMSG_CHAR_RENAME(c) => c.write_encrypted_server(w, e),
            Self::SMSG_ADDON_INFO(c) => c.write_encrypted_server(w, e),
            Self::SMSG_CHAT_RESTRICTED(c) => c.write_encrypted_server(w, e),
            Self::SMSG_DEFENSE_MESSAGE(c) => c.write_encrypted_server(w, e),
            Self::MSG_MOVE_START_ASCEND(c) => c.write_encrypted_server(w, e),
            Self::MSG_MOVE_STOP_ASCEND(c) => c.write_encrypted_server(w, e),
            Self::SMSG_REALM_SPLIT(c) => c.write_encrypted_server(w, e),
            Self::SMSG_TIME_SYNC_REQ(c) => c.write_encrypted_server(w, e),
            Self::MSG_MOVE_START_DESCEND(c) => c.write_encrypted_server(w, e),
            Self::SMSG_UPDATE_ACCOUNT_DATA_COMPLETE(c) => c.write_encrypted_server(w, e),
            Self::SMSG_CLIENTCACHE_VERSION(c) => c.write_encrypted_server(w, e),
        }
    }

    #[cfg(feature = "sync")]
    pub fn write_unencrypted_server<W: std::io::Write>(&self, w: &mut W) -> Result<(), std::io::Error> {
        match self {
            Self::MSG_SET_DUNGEON_DIFFICULTY(c) => c.write_unencrypted_server(w),
            Self::SMSG_CHAR_CREATE(c) => c.write_unencrypted_server(w),
            Self::SMSG_CHAR_ENUM(c) => c.write_unencrypted_server(w),
            Self::SMSG_CHAR_DELETE(c) => c.write_unencrypted_server(w),
            Self::SMSG_CHARACTER_LOGIN_FAILED(c) => c.write_unencrypted_server(w),
            Self::SMSG_LOGIN_SETTIMESPEED(c) => c.write_unencrypted_server(w),
            Self::SMSG_CONTACT_LIST(c) => c.write_unencrypted_server(w),
            Self::SMSG_CHANNEL_NOTIFY(c) => c.write_unencrypted_server(w),
            Self::SMSG_CHANNEL_LIST(c) => c.write_unencrypted_server(w),
            Self::SMSG_UPDATE_OBJECT(c) => c.write_unencrypted_server(w),
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
            Self::MSG_MOVE_HEARTBEAT(c) => c.write_unencrypted_server(w),
            Self::SMSG_TRIGGER_CINEMATIC(c) => c.write_unencrypted_server(w),
            Self::SMSG_TUTORIAL_FLAGS(c) => c.write_unencrypted_server(w),
            Self::SMSG_EMOTE(c) => c.write_unencrypted_server(w),
            Self::SMSG_TEXT_EMOTE(c) => c.write_unencrypted_server(w),
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
            Self::SMSG_NOTIFICATION(c) => c.write_unencrypted_server(w),
            Self::SMSG_PONG(c) => c.write_unencrypted_server(w),
            Self::SMSG_AUTH_CHALLENGE(c) => c.write_unencrypted_server(w),
            Self::SMSG_AUTH_RESPONSE(c) => c.write_unencrypted_server(w),
            Self::SMSG_ENVIRONMENTALDAMAGELOG(c) => c.write_unencrypted_server(w),
            Self::SMSG_ACCOUNT_DATA_TIMES(c) => c.write_unencrypted_server(w),
            Self::SMSG_UPDATE_ACCOUNT_DATA(c) => c.write_unencrypted_server(w),
            Self::SMSG_CHAT_WRONG_FACTION(c) => c.write_unencrypted_server(w),
            Self::SMSG_LOGIN_VERIFY_WORLD(c) => c.write_unencrypted_server(w),
            Self::SMSG_ZONE_UNDER_ATTACK(c) => c.write_unencrypted_server(w),
            Self::MSG_AUCTION_HELLO(c) => c.write_unencrypted_server(w),
            Self::SMSG_AUCTION_LIST_RESULT(c) => c.write_unencrypted_server(w),
            Self::SMSG_AUCTION_OWNER_LIST_RESULT(c) => c.write_unencrypted_server(w),
            Self::SMSG_AUCTION_BIDDER_NOTIFICATION(c) => c.write_unencrypted_server(w),
            Self::SMSG_AUCTION_OWNER_NOTIFICATION(c) => c.write_unencrypted_server(w),
            Self::SMSG_PROCRESIST(c) => c.write_unencrypted_server(w),
            Self::SMSG_AUCTION_BIDDER_LIST_RESULT(c) => c.write_unencrypted_server(w),
            Self::SMSG_AUCTION_REMOVED_NOTIFICATION(c) => c.write_unencrypted_server(w),
            Self::SMSG_SERVER_MESSAGE(c) => c.write_unencrypted_server(w),
            Self::SMSG_CHAT_PLAYER_NOT_FOUND(c) => c.write_unencrypted_server(w),
            Self::SMSG_DURABILITY_DAMAGE_DEATH(c) => c.write_unencrypted_server(w),
            Self::SMSG_CHAR_RENAME(c) => c.write_unencrypted_server(w),
            Self::SMSG_ADDON_INFO(c) => c.write_unencrypted_server(w),
            Self::SMSG_CHAT_RESTRICTED(c) => c.write_unencrypted_server(w),
            Self::SMSG_DEFENSE_MESSAGE(c) => c.write_unencrypted_server(w),
            Self::MSG_MOVE_START_ASCEND(c) => c.write_unencrypted_server(w),
            Self::MSG_MOVE_STOP_ASCEND(c) => c.write_unencrypted_server(w),
            Self::SMSG_REALM_SPLIT(c) => c.write_unencrypted_server(w),
            Self::SMSG_TIME_SYNC_REQ(c) => c.write_unencrypted_server(w),
            Self::MSG_MOVE_START_DESCEND(c) => c.write_unencrypted_server(w),
            Self::SMSG_UPDATE_ACCOUNT_DATA_COMPLETE(c) => c.write_unencrypted_server(w),
            Self::SMSG_CLIENTCACHE_VERSION(c) => c.write_unencrypted_server(w),
        }
    }

    #[cfg(feature = "tokio")]
    pub async fn tokio_write_encrypted_server<W: tokio::io::AsyncWriteExt + Unpin + Send>(&self, w: &mut W, e: &mut ServerEncrypterHalf) -> Result<(), std::io::Error> {
        match self {
            Self::MSG_SET_DUNGEON_DIFFICULTY(c) => c.tokio_write_encrypted_server(w, e).await,
            Self::SMSG_CHAR_CREATE(c) => c.tokio_write_encrypted_server(w, e).await,
            Self::SMSG_CHAR_ENUM(c) => c.tokio_write_encrypted_server(w, e).await,
            Self::SMSG_CHAR_DELETE(c) => c.tokio_write_encrypted_server(w, e).await,
            Self::SMSG_CHARACTER_LOGIN_FAILED(c) => c.tokio_write_encrypted_server(w, e).await,
            Self::SMSG_LOGIN_SETTIMESPEED(c) => c.tokio_write_encrypted_server(w, e).await,
            Self::SMSG_CONTACT_LIST(c) => c.tokio_write_encrypted_server(w, e).await,
            Self::SMSG_CHANNEL_NOTIFY(c) => c.tokio_write_encrypted_server(w, e).await,
            Self::SMSG_CHANNEL_LIST(c) => c.tokio_write_encrypted_server(w, e).await,
            Self::SMSG_UPDATE_OBJECT(c) => c.tokio_write_encrypted_server(w, e).await,
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
            Self::MSG_MOVE_HEARTBEAT(c) => c.tokio_write_encrypted_server(w, e).await,
            Self::SMSG_TRIGGER_CINEMATIC(c) => c.tokio_write_encrypted_server(w, e).await,
            Self::SMSG_TUTORIAL_FLAGS(c) => c.tokio_write_encrypted_server(w, e).await,
            Self::SMSG_EMOTE(c) => c.tokio_write_encrypted_server(w, e).await,
            Self::SMSG_TEXT_EMOTE(c) => c.tokio_write_encrypted_server(w, e).await,
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
            Self::SMSG_NOTIFICATION(c) => c.tokio_write_encrypted_server(w, e).await,
            Self::SMSG_PONG(c) => c.tokio_write_encrypted_server(w, e).await,
            Self::SMSG_AUTH_CHALLENGE(c) => c.tokio_write_encrypted_server(w, e).await,
            Self::SMSG_AUTH_RESPONSE(c) => c.tokio_write_encrypted_server(w, e).await,
            Self::SMSG_ENVIRONMENTALDAMAGELOG(c) => c.tokio_write_encrypted_server(w, e).await,
            Self::SMSG_ACCOUNT_DATA_TIMES(c) => c.tokio_write_encrypted_server(w, e).await,
            Self::SMSG_UPDATE_ACCOUNT_DATA(c) => c.tokio_write_encrypted_server(w, e).await,
            Self::SMSG_CHAT_WRONG_FACTION(c) => c.tokio_write_encrypted_server(w, e).await,
            Self::SMSG_LOGIN_VERIFY_WORLD(c) => c.tokio_write_encrypted_server(w, e).await,
            Self::SMSG_ZONE_UNDER_ATTACK(c) => c.tokio_write_encrypted_server(w, e).await,
            Self::MSG_AUCTION_HELLO(c) => c.tokio_write_encrypted_server(w, e).await,
            Self::SMSG_AUCTION_LIST_RESULT(c) => c.tokio_write_encrypted_server(w, e).await,
            Self::SMSG_AUCTION_OWNER_LIST_RESULT(c) => c.tokio_write_encrypted_server(w, e).await,
            Self::SMSG_AUCTION_BIDDER_NOTIFICATION(c) => c.tokio_write_encrypted_server(w, e).await,
            Self::SMSG_AUCTION_OWNER_NOTIFICATION(c) => c.tokio_write_encrypted_server(w, e).await,
            Self::SMSG_PROCRESIST(c) => c.tokio_write_encrypted_server(w, e).await,
            Self::SMSG_AUCTION_BIDDER_LIST_RESULT(c) => c.tokio_write_encrypted_server(w, e).await,
            Self::SMSG_AUCTION_REMOVED_NOTIFICATION(c) => c.tokio_write_encrypted_server(w, e).await,
            Self::SMSG_SERVER_MESSAGE(c) => c.tokio_write_encrypted_server(w, e).await,
            Self::SMSG_CHAT_PLAYER_NOT_FOUND(c) => c.tokio_write_encrypted_server(w, e).await,
            Self::SMSG_DURABILITY_DAMAGE_DEATH(c) => c.tokio_write_encrypted_server(w, e).await,
            Self::SMSG_CHAR_RENAME(c) => c.tokio_write_encrypted_server(w, e).await,
            Self::SMSG_ADDON_INFO(c) => c.tokio_write_encrypted_server(w, e).await,
            Self::SMSG_CHAT_RESTRICTED(c) => c.tokio_write_encrypted_server(w, e).await,
            Self::SMSG_DEFENSE_MESSAGE(c) => c.tokio_write_encrypted_server(w, e).await,
            Self::MSG_MOVE_START_ASCEND(c) => c.tokio_write_encrypted_server(w, e).await,
            Self::MSG_MOVE_STOP_ASCEND(c) => c.tokio_write_encrypted_server(w, e).await,
            Self::SMSG_REALM_SPLIT(c) => c.tokio_write_encrypted_server(w, e).await,
            Self::SMSG_TIME_SYNC_REQ(c) => c.tokio_write_encrypted_server(w, e).await,
            Self::MSG_MOVE_START_DESCEND(c) => c.tokio_write_encrypted_server(w, e).await,
            Self::SMSG_UPDATE_ACCOUNT_DATA_COMPLETE(c) => c.tokio_write_encrypted_server(w, e).await,
            Self::SMSG_CLIENTCACHE_VERSION(c) => c.tokio_write_encrypted_server(w, e).await,
        }
    }

    #[cfg(feature = "tokio")]
    pub async fn tokio_write_unencrypted_server<W: tokio::io::AsyncWriteExt + Unpin + Send>(&self, w: &mut W) -> Result<(), std::io::Error> {
        match self {
            Self::MSG_SET_DUNGEON_DIFFICULTY(c) => c.tokio_write_unencrypted_server(w).await,
            Self::SMSG_CHAR_CREATE(c) => c.tokio_write_unencrypted_server(w).await,
            Self::SMSG_CHAR_ENUM(c) => c.tokio_write_unencrypted_server(w).await,
            Self::SMSG_CHAR_DELETE(c) => c.tokio_write_unencrypted_server(w).await,
            Self::SMSG_CHARACTER_LOGIN_FAILED(c) => c.tokio_write_unencrypted_server(w).await,
            Self::SMSG_LOGIN_SETTIMESPEED(c) => c.tokio_write_unencrypted_server(w).await,
            Self::SMSG_CONTACT_LIST(c) => c.tokio_write_unencrypted_server(w).await,
            Self::SMSG_CHANNEL_NOTIFY(c) => c.tokio_write_unencrypted_server(w).await,
            Self::SMSG_CHANNEL_LIST(c) => c.tokio_write_unencrypted_server(w).await,
            Self::SMSG_UPDATE_OBJECT(c) => c.tokio_write_unencrypted_server(w).await,
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
            Self::MSG_MOVE_HEARTBEAT(c) => c.tokio_write_unencrypted_server(w).await,
            Self::SMSG_TRIGGER_CINEMATIC(c) => c.tokio_write_unencrypted_server(w).await,
            Self::SMSG_TUTORIAL_FLAGS(c) => c.tokio_write_unencrypted_server(w).await,
            Self::SMSG_EMOTE(c) => c.tokio_write_unencrypted_server(w).await,
            Self::SMSG_TEXT_EMOTE(c) => c.tokio_write_unencrypted_server(w).await,
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
            Self::SMSG_NOTIFICATION(c) => c.tokio_write_unencrypted_server(w).await,
            Self::SMSG_PONG(c) => c.tokio_write_unencrypted_server(w).await,
            Self::SMSG_AUTH_CHALLENGE(c) => c.tokio_write_unencrypted_server(w).await,
            Self::SMSG_AUTH_RESPONSE(c) => c.tokio_write_unencrypted_server(w).await,
            Self::SMSG_ENVIRONMENTALDAMAGELOG(c) => c.tokio_write_unencrypted_server(w).await,
            Self::SMSG_ACCOUNT_DATA_TIMES(c) => c.tokio_write_unencrypted_server(w).await,
            Self::SMSG_UPDATE_ACCOUNT_DATA(c) => c.tokio_write_unencrypted_server(w).await,
            Self::SMSG_CHAT_WRONG_FACTION(c) => c.tokio_write_unencrypted_server(w).await,
            Self::SMSG_LOGIN_VERIFY_WORLD(c) => c.tokio_write_unencrypted_server(w).await,
            Self::SMSG_ZONE_UNDER_ATTACK(c) => c.tokio_write_unencrypted_server(w).await,
            Self::MSG_AUCTION_HELLO(c) => c.tokio_write_unencrypted_server(w).await,
            Self::SMSG_AUCTION_LIST_RESULT(c) => c.tokio_write_unencrypted_server(w).await,
            Self::SMSG_AUCTION_OWNER_LIST_RESULT(c) => c.tokio_write_unencrypted_server(w).await,
            Self::SMSG_AUCTION_BIDDER_NOTIFICATION(c) => c.tokio_write_unencrypted_server(w).await,
            Self::SMSG_AUCTION_OWNER_NOTIFICATION(c) => c.tokio_write_unencrypted_server(w).await,
            Self::SMSG_PROCRESIST(c) => c.tokio_write_unencrypted_server(w).await,
            Self::SMSG_AUCTION_BIDDER_LIST_RESULT(c) => c.tokio_write_unencrypted_server(w).await,
            Self::SMSG_AUCTION_REMOVED_NOTIFICATION(c) => c.tokio_write_unencrypted_server(w).await,
            Self::SMSG_SERVER_MESSAGE(c) => c.tokio_write_unencrypted_server(w).await,
            Self::SMSG_CHAT_PLAYER_NOT_FOUND(c) => c.tokio_write_unencrypted_server(w).await,
            Self::SMSG_DURABILITY_DAMAGE_DEATH(c) => c.tokio_write_unencrypted_server(w).await,
            Self::SMSG_CHAR_RENAME(c) => c.tokio_write_unencrypted_server(w).await,
            Self::SMSG_ADDON_INFO(c) => c.tokio_write_unencrypted_server(w).await,
            Self::SMSG_CHAT_RESTRICTED(c) => c.tokio_write_unencrypted_server(w).await,
            Self::SMSG_DEFENSE_MESSAGE(c) => c.tokio_write_unencrypted_server(w).await,
            Self::MSG_MOVE_START_ASCEND(c) => c.tokio_write_unencrypted_server(w).await,
            Self::MSG_MOVE_STOP_ASCEND(c) => c.tokio_write_unencrypted_server(w).await,
            Self::SMSG_REALM_SPLIT(c) => c.tokio_write_unencrypted_server(w).await,
            Self::SMSG_TIME_SYNC_REQ(c) => c.tokio_write_unencrypted_server(w).await,
            Self::MSG_MOVE_START_DESCEND(c) => c.tokio_write_unencrypted_server(w).await,
            Self::SMSG_UPDATE_ACCOUNT_DATA_COMPLETE(c) => c.tokio_write_unencrypted_server(w).await,
            Self::SMSG_CLIENTCACHE_VERSION(c) => c.tokio_write_unencrypted_server(w).await,
        }
    }

    #[cfg(feature = "async-std")]
    pub async fn astd_write_encrypted_server<W: async_std::io::WriteExt + Unpin + Send>(&self, w: &mut W, e: &mut ServerEncrypterHalf) -> Result<(), std::io::Error> {
        match self {
            Self::MSG_SET_DUNGEON_DIFFICULTY(c) => c.astd_write_encrypted_server(w, e).await,
            Self::SMSG_CHAR_CREATE(c) => c.astd_write_encrypted_server(w, e).await,
            Self::SMSG_CHAR_ENUM(c) => c.astd_write_encrypted_server(w, e).await,
            Self::SMSG_CHAR_DELETE(c) => c.astd_write_encrypted_server(w, e).await,
            Self::SMSG_CHARACTER_LOGIN_FAILED(c) => c.astd_write_encrypted_server(w, e).await,
            Self::SMSG_LOGIN_SETTIMESPEED(c) => c.astd_write_encrypted_server(w, e).await,
            Self::SMSG_CONTACT_LIST(c) => c.astd_write_encrypted_server(w, e).await,
            Self::SMSG_CHANNEL_NOTIFY(c) => c.astd_write_encrypted_server(w, e).await,
            Self::SMSG_CHANNEL_LIST(c) => c.astd_write_encrypted_server(w, e).await,
            Self::SMSG_UPDATE_OBJECT(c) => c.astd_write_encrypted_server(w, e).await,
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
            Self::MSG_MOVE_HEARTBEAT(c) => c.astd_write_encrypted_server(w, e).await,
            Self::SMSG_TRIGGER_CINEMATIC(c) => c.astd_write_encrypted_server(w, e).await,
            Self::SMSG_TUTORIAL_FLAGS(c) => c.astd_write_encrypted_server(w, e).await,
            Self::SMSG_EMOTE(c) => c.astd_write_encrypted_server(w, e).await,
            Self::SMSG_TEXT_EMOTE(c) => c.astd_write_encrypted_server(w, e).await,
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
            Self::SMSG_NOTIFICATION(c) => c.astd_write_encrypted_server(w, e).await,
            Self::SMSG_PONG(c) => c.astd_write_encrypted_server(w, e).await,
            Self::SMSG_AUTH_CHALLENGE(c) => c.astd_write_encrypted_server(w, e).await,
            Self::SMSG_AUTH_RESPONSE(c) => c.astd_write_encrypted_server(w, e).await,
            Self::SMSG_ENVIRONMENTALDAMAGELOG(c) => c.astd_write_encrypted_server(w, e).await,
            Self::SMSG_ACCOUNT_DATA_TIMES(c) => c.astd_write_encrypted_server(w, e).await,
            Self::SMSG_UPDATE_ACCOUNT_DATA(c) => c.astd_write_encrypted_server(w, e).await,
            Self::SMSG_CHAT_WRONG_FACTION(c) => c.astd_write_encrypted_server(w, e).await,
            Self::SMSG_LOGIN_VERIFY_WORLD(c) => c.astd_write_encrypted_server(w, e).await,
            Self::SMSG_ZONE_UNDER_ATTACK(c) => c.astd_write_encrypted_server(w, e).await,
            Self::MSG_AUCTION_HELLO(c) => c.astd_write_encrypted_server(w, e).await,
            Self::SMSG_AUCTION_LIST_RESULT(c) => c.astd_write_encrypted_server(w, e).await,
            Self::SMSG_AUCTION_OWNER_LIST_RESULT(c) => c.astd_write_encrypted_server(w, e).await,
            Self::SMSG_AUCTION_BIDDER_NOTIFICATION(c) => c.astd_write_encrypted_server(w, e).await,
            Self::SMSG_AUCTION_OWNER_NOTIFICATION(c) => c.astd_write_encrypted_server(w, e).await,
            Self::SMSG_PROCRESIST(c) => c.astd_write_encrypted_server(w, e).await,
            Self::SMSG_AUCTION_BIDDER_LIST_RESULT(c) => c.astd_write_encrypted_server(w, e).await,
            Self::SMSG_AUCTION_REMOVED_NOTIFICATION(c) => c.astd_write_encrypted_server(w, e).await,
            Self::SMSG_SERVER_MESSAGE(c) => c.astd_write_encrypted_server(w, e).await,
            Self::SMSG_CHAT_PLAYER_NOT_FOUND(c) => c.astd_write_encrypted_server(w, e).await,
            Self::SMSG_DURABILITY_DAMAGE_DEATH(c) => c.astd_write_encrypted_server(w, e).await,
            Self::SMSG_CHAR_RENAME(c) => c.astd_write_encrypted_server(w, e).await,
            Self::SMSG_ADDON_INFO(c) => c.astd_write_encrypted_server(w, e).await,
            Self::SMSG_CHAT_RESTRICTED(c) => c.astd_write_encrypted_server(w, e).await,
            Self::SMSG_DEFENSE_MESSAGE(c) => c.astd_write_encrypted_server(w, e).await,
            Self::MSG_MOVE_START_ASCEND(c) => c.astd_write_encrypted_server(w, e).await,
            Self::MSG_MOVE_STOP_ASCEND(c) => c.astd_write_encrypted_server(w, e).await,
            Self::SMSG_REALM_SPLIT(c) => c.astd_write_encrypted_server(w, e).await,
            Self::SMSG_TIME_SYNC_REQ(c) => c.astd_write_encrypted_server(w, e).await,
            Self::MSG_MOVE_START_DESCEND(c) => c.astd_write_encrypted_server(w, e).await,
            Self::SMSG_UPDATE_ACCOUNT_DATA_COMPLETE(c) => c.astd_write_encrypted_server(w, e).await,
            Self::SMSG_CLIENTCACHE_VERSION(c) => c.astd_write_encrypted_server(w, e).await,
        }
    }

    #[cfg(feature = "async-std")]
    pub async fn astd_write_unencrypted_server<W: async_std::io::WriteExt + Unpin + Send>(&self, w: &mut W) -> Result<(), std::io::Error> {
        match self {
            Self::MSG_SET_DUNGEON_DIFFICULTY(c) => c.astd_write_unencrypted_server(w).await,
            Self::SMSG_CHAR_CREATE(c) => c.astd_write_unencrypted_server(w).await,
            Self::SMSG_CHAR_ENUM(c) => c.astd_write_unencrypted_server(w).await,
            Self::SMSG_CHAR_DELETE(c) => c.astd_write_unencrypted_server(w).await,
            Self::SMSG_CHARACTER_LOGIN_FAILED(c) => c.astd_write_unencrypted_server(w).await,
            Self::SMSG_LOGIN_SETTIMESPEED(c) => c.astd_write_unencrypted_server(w).await,
            Self::SMSG_CONTACT_LIST(c) => c.astd_write_unencrypted_server(w).await,
            Self::SMSG_CHANNEL_NOTIFY(c) => c.astd_write_unencrypted_server(w).await,
            Self::SMSG_CHANNEL_LIST(c) => c.astd_write_unencrypted_server(w).await,
            Self::SMSG_UPDATE_OBJECT(c) => c.astd_write_unencrypted_server(w).await,
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
            Self::MSG_MOVE_HEARTBEAT(c) => c.astd_write_unencrypted_server(w).await,
            Self::SMSG_TRIGGER_CINEMATIC(c) => c.astd_write_unencrypted_server(w).await,
            Self::SMSG_TUTORIAL_FLAGS(c) => c.astd_write_unencrypted_server(w).await,
            Self::SMSG_EMOTE(c) => c.astd_write_unencrypted_server(w).await,
            Self::SMSG_TEXT_EMOTE(c) => c.astd_write_unencrypted_server(w).await,
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
            Self::SMSG_NOTIFICATION(c) => c.astd_write_unencrypted_server(w).await,
            Self::SMSG_PONG(c) => c.astd_write_unencrypted_server(w).await,
            Self::SMSG_AUTH_CHALLENGE(c) => c.astd_write_unencrypted_server(w).await,
            Self::SMSG_AUTH_RESPONSE(c) => c.astd_write_unencrypted_server(w).await,
            Self::SMSG_ENVIRONMENTALDAMAGELOG(c) => c.astd_write_unencrypted_server(w).await,
            Self::SMSG_ACCOUNT_DATA_TIMES(c) => c.astd_write_unencrypted_server(w).await,
            Self::SMSG_UPDATE_ACCOUNT_DATA(c) => c.astd_write_unencrypted_server(w).await,
            Self::SMSG_CHAT_WRONG_FACTION(c) => c.astd_write_unencrypted_server(w).await,
            Self::SMSG_LOGIN_VERIFY_WORLD(c) => c.astd_write_unencrypted_server(w).await,
            Self::SMSG_ZONE_UNDER_ATTACK(c) => c.astd_write_unencrypted_server(w).await,
            Self::MSG_AUCTION_HELLO(c) => c.astd_write_unencrypted_server(w).await,
            Self::SMSG_AUCTION_LIST_RESULT(c) => c.astd_write_unencrypted_server(w).await,
            Self::SMSG_AUCTION_OWNER_LIST_RESULT(c) => c.astd_write_unencrypted_server(w).await,
            Self::SMSG_AUCTION_BIDDER_NOTIFICATION(c) => c.astd_write_unencrypted_server(w).await,
            Self::SMSG_AUCTION_OWNER_NOTIFICATION(c) => c.astd_write_unencrypted_server(w).await,
            Self::SMSG_PROCRESIST(c) => c.astd_write_unencrypted_server(w).await,
            Self::SMSG_AUCTION_BIDDER_LIST_RESULT(c) => c.astd_write_unencrypted_server(w).await,
            Self::SMSG_AUCTION_REMOVED_NOTIFICATION(c) => c.astd_write_unencrypted_server(w).await,
            Self::SMSG_SERVER_MESSAGE(c) => c.astd_write_unencrypted_server(w).await,
            Self::SMSG_CHAT_PLAYER_NOT_FOUND(c) => c.astd_write_unencrypted_server(w).await,
            Self::SMSG_DURABILITY_DAMAGE_DEATH(c) => c.astd_write_unencrypted_server(w).await,
            Self::SMSG_CHAR_RENAME(c) => c.astd_write_unencrypted_server(w).await,
            Self::SMSG_ADDON_INFO(c) => c.astd_write_unencrypted_server(w).await,
            Self::SMSG_CHAT_RESTRICTED(c) => c.astd_write_unencrypted_server(w).await,
            Self::SMSG_DEFENSE_MESSAGE(c) => c.astd_write_unencrypted_server(w).await,
            Self::MSG_MOVE_START_ASCEND(c) => c.astd_write_unencrypted_server(w).await,
            Self::MSG_MOVE_STOP_ASCEND(c) => c.astd_write_unencrypted_server(w).await,
            Self::SMSG_REALM_SPLIT(c) => c.astd_write_unencrypted_server(w).await,
            Self::SMSG_TIME_SYNC_REQ(c) => c.astd_write_unencrypted_server(w).await,
            Self::MSG_MOVE_START_DESCEND(c) => c.astd_write_unencrypted_server(w).await,
            Self::SMSG_UPDATE_ACCOUNT_DATA_COMPLETE(c) => c.astd_write_unencrypted_server(w).await,
            Self::SMSG_CLIENTCACHE_VERSION(c) => c.astd_write_unencrypted_server(w).await,
        }
    }

}

impl std::fmt::Display for ServerOpcodeMessage {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            ServerOpcodeMessage::MSG_SET_DUNGEON_DIFFICULTY(_) => "MSG_SET_DUNGEON_DIFFICULTY",
            ServerOpcodeMessage::SMSG_CHAR_CREATE(_) => "SMSG_CHAR_CREATE",
            ServerOpcodeMessage::SMSG_CHAR_ENUM(_) => "SMSG_CHAR_ENUM",
            ServerOpcodeMessage::SMSG_CHAR_DELETE(_) => "SMSG_CHAR_DELETE",
            ServerOpcodeMessage::SMSG_CHARACTER_LOGIN_FAILED(_) => "SMSG_CHARACTER_LOGIN_FAILED",
            ServerOpcodeMessage::SMSG_LOGIN_SETTIMESPEED(_) => "SMSG_LOGIN_SETTIMESPEED",
            ServerOpcodeMessage::SMSG_CONTACT_LIST(_) => "SMSG_CONTACT_LIST",
            ServerOpcodeMessage::SMSG_CHANNEL_NOTIFY(_) => "SMSG_CHANNEL_NOTIFY",
            ServerOpcodeMessage::SMSG_CHANNEL_LIST(_) => "SMSG_CHANNEL_LIST",
            ServerOpcodeMessage::SMSG_UPDATE_OBJECT(_) => "SMSG_UPDATE_OBJECT",
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
            ServerOpcodeMessage::MSG_MOVE_FALL_LAND(_) => "MSG_MOVE_FALL_LAND_Server",
            ServerOpcodeMessage::MSG_MOVE_START_SWIM(_) => "MSG_MOVE_START_SWIM_Server",
            ServerOpcodeMessage::MSG_MOVE_STOP_SWIM(_) => "MSG_MOVE_STOP_SWIM_Server",
            ServerOpcodeMessage::MSG_MOVE_SET_FACING(_) => "MSG_MOVE_SET_FACING_Server",
            ServerOpcodeMessage::MSG_MOVE_SET_PITCH(_) => "MSG_MOVE_SET_PITCH_Server",
            ServerOpcodeMessage::MSG_MOVE_HEARTBEAT(_) => "MSG_MOVE_HEARTBEAT_Server",
            ServerOpcodeMessage::SMSG_TRIGGER_CINEMATIC(_) => "SMSG_TRIGGER_CINEMATIC",
            ServerOpcodeMessage::SMSG_TUTORIAL_FLAGS(_) => "SMSG_TUTORIAL_FLAGS",
            ServerOpcodeMessage::SMSG_EMOTE(_) => "SMSG_EMOTE",
            ServerOpcodeMessage::SMSG_TEXT_EMOTE(_) => "SMSG_TEXT_EMOTE",
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
            ServerOpcodeMessage::SMSG_NOTIFICATION(_) => "SMSG_NOTIFICATION",
            ServerOpcodeMessage::SMSG_PONG(_) => "SMSG_PONG",
            ServerOpcodeMessage::SMSG_AUTH_CHALLENGE(_) => "SMSG_AUTH_CHALLENGE",
            ServerOpcodeMessage::SMSG_AUTH_RESPONSE(_) => "SMSG_AUTH_RESPONSE",
            ServerOpcodeMessage::SMSG_ENVIRONMENTALDAMAGELOG(_) => "SMSG_ENVIRONMENTALDAMAGELOG",
            ServerOpcodeMessage::SMSG_ACCOUNT_DATA_TIMES(_) => "SMSG_ACCOUNT_DATA_TIMES",
            ServerOpcodeMessage::SMSG_UPDATE_ACCOUNT_DATA(_) => "SMSG_UPDATE_ACCOUNT_DATA",
            ServerOpcodeMessage::SMSG_CHAT_WRONG_FACTION(_) => "SMSG_CHAT_WRONG_FACTION",
            ServerOpcodeMessage::SMSG_LOGIN_VERIFY_WORLD(_) => "SMSG_LOGIN_VERIFY_WORLD",
            ServerOpcodeMessage::SMSG_ZONE_UNDER_ATTACK(_) => "SMSG_ZONE_UNDER_ATTACK",
            ServerOpcodeMessage::MSG_AUCTION_HELLO(_) => "MSG_AUCTION_HELLO_Server",
            ServerOpcodeMessage::SMSG_AUCTION_LIST_RESULT(_) => "SMSG_AUCTION_LIST_RESULT",
            ServerOpcodeMessage::SMSG_AUCTION_OWNER_LIST_RESULT(_) => "SMSG_AUCTION_OWNER_LIST_RESULT",
            ServerOpcodeMessage::SMSG_AUCTION_BIDDER_NOTIFICATION(_) => "SMSG_AUCTION_BIDDER_NOTIFICATION",
            ServerOpcodeMessage::SMSG_AUCTION_OWNER_NOTIFICATION(_) => "SMSG_AUCTION_OWNER_NOTIFICATION",
            ServerOpcodeMessage::SMSG_PROCRESIST(_) => "SMSG_PROCRESIST",
            ServerOpcodeMessage::SMSG_AUCTION_BIDDER_LIST_RESULT(_) => "SMSG_AUCTION_BIDDER_LIST_RESULT",
            ServerOpcodeMessage::SMSG_AUCTION_REMOVED_NOTIFICATION(_) => "SMSG_AUCTION_REMOVED_NOTIFICATION",
            ServerOpcodeMessage::SMSG_SERVER_MESSAGE(_) => "SMSG_SERVER_MESSAGE",
            ServerOpcodeMessage::SMSG_CHAT_PLAYER_NOT_FOUND(_) => "SMSG_CHAT_PLAYER_NOT_FOUND",
            ServerOpcodeMessage::SMSG_DURABILITY_DAMAGE_DEATH(_) => "SMSG_DURABILITY_DAMAGE_DEATH",
            ServerOpcodeMessage::SMSG_CHAR_RENAME(_) => "SMSG_CHAR_RENAME",
            ServerOpcodeMessage::SMSG_ADDON_INFO(_) => "SMSG_ADDON_INFO",
            ServerOpcodeMessage::SMSG_CHAT_RESTRICTED(_) => "SMSG_CHAT_RESTRICTED",
            ServerOpcodeMessage::SMSG_DEFENSE_MESSAGE(_) => "SMSG_DEFENSE_MESSAGE",
            ServerOpcodeMessage::MSG_MOVE_START_ASCEND(_) => "MSG_MOVE_START_ASCEND_Server",
            ServerOpcodeMessage::MSG_MOVE_STOP_ASCEND(_) => "MSG_MOVE_STOP_ASCEND_Server",
            ServerOpcodeMessage::SMSG_REALM_SPLIT(_) => "SMSG_REALM_SPLIT",
            ServerOpcodeMessage::SMSG_TIME_SYNC_REQ(_) => "SMSG_TIME_SYNC_REQ",
            ServerOpcodeMessage::MSG_MOVE_START_DESCEND(_) => "MSG_MOVE_START_DESCEND_Server",
            ServerOpcodeMessage::SMSG_UPDATE_ACCOUNT_DATA_COMPLETE(_) => "SMSG_UPDATE_ACCOUNT_DATA_COMPLETE",
            ServerOpcodeMessage::SMSG_CLIENTCACHE_VERSION(_) => "SMSG_CLIENTCACHE_VERSION",
        })
    }
}

impl From<MSG_SET_DUNGEON_DIFFICULTY> for ServerOpcodeMessage {
    fn from(c: MSG_SET_DUNGEON_DIFFICULTY) -> Self {
        Self::MSG_SET_DUNGEON_DIFFICULTY(c)
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

impl From<SMSG_CONTACT_LIST> for ServerOpcodeMessage {
    fn from(c: SMSG_CONTACT_LIST) -> Self {
        Self::SMSG_CONTACT_LIST(c)
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

impl From<MSG_MOVE_HEARTBEAT_Server> for ServerOpcodeMessage {
    fn from(c: MSG_MOVE_HEARTBEAT_Server) -> Self {
        Self::MSG_MOVE_HEARTBEAT(c)
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

impl From<SMSG_NOTIFICATION> for ServerOpcodeMessage {
    fn from(c: SMSG_NOTIFICATION) -> Self {
        Self::SMSG_NOTIFICATION(c)
    }
}

impl From<SMSG_PONG> for ServerOpcodeMessage {
    fn from(c: SMSG_PONG) -> Self {
        Self::SMSG_PONG(c)
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

impl From<SMSG_ENVIRONMENTALDAMAGELOG> for ServerOpcodeMessage {
    fn from(c: SMSG_ENVIRONMENTALDAMAGELOG) -> Self {
        Self::SMSG_ENVIRONMENTALDAMAGELOG(c)
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

impl From<SMSG_CHAT_WRONG_FACTION> for ServerOpcodeMessage {
    fn from(c: SMSG_CHAT_WRONG_FACTION) -> Self {
        Self::SMSG_CHAT_WRONG_FACTION(c)
    }
}

impl From<SMSG_LOGIN_VERIFY_WORLD> for ServerOpcodeMessage {
    fn from(c: SMSG_LOGIN_VERIFY_WORLD) -> Self {
        Self::SMSG_LOGIN_VERIFY_WORLD(c)
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

impl From<SMSG_CHAT_PLAYER_NOT_FOUND> for ServerOpcodeMessage {
    fn from(c: SMSG_CHAT_PLAYER_NOT_FOUND) -> Self {
        Self::SMSG_CHAT_PLAYER_NOT_FOUND(c)
    }
}

impl From<SMSG_DURABILITY_DAMAGE_DEATH> for ServerOpcodeMessage {
    fn from(c: SMSG_DURABILITY_DAMAGE_DEATH) -> Self {
        Self::SMSG_DURABILITY_DAMAGE_DEATH(c)
    }
}

impl From<SMSG_CHAR_RENAME> for ServerOpcodeMessage {
    fn from(c: SMSG_CHAR_RENAME) -> Self {
        Self::SMSG_CHAR_RENAME(c)
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

impl From<SMSG_DEFENSE_MESSAGE> for ServerOpcodeMessage {
    fn from(c: SMSG_DEFENSE_MESSAGE) -> Self {
        Self::SMSG_DEFENSE_MESSAGE(c)
    }
}

impl From<MSG_MOVE_START_ASCEND_Server> for ServerOpcodeMessage {
    fn from(c: MSG_MOVE_START_ASCEND_Server) -> Self {
        Self::MSG_MOVE_START_ASCEND(c)
    }
}

impl From<MSG_MOVE_STOP_ASCEND_Server> for ServerOpcodeMessage {
    fn from(c: MSG_MOVE_STOP_ASCEND_Server) -> Self {
        Self::MSG_MOVE_STOP_ASCEND(c)
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

impl From<MSG_MOVE_START_DESCEND_Server> for ServerOpcodeMessage {
    fn from(c: MSG_MOVE_START_DESCEND_Server) -> Self {
        Self::MSG_MOVE_START_DESCEND(c)
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

