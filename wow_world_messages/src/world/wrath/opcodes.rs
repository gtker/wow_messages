use crate::wrath::{ServerMessage, ClientMessage};
use wow_srp::wrath_header::{ClientEncrypterHalf, ClientDecrypterHalf, ServerEncrypterHalf, ServerDecrypterHalf};

#[cfg(feature = "tokio")]
use tokio::io::{AsyncReadExt, AsyncWriteExt};
#[cfg(feature = "async-std")]
use async_std::io::{ReadExt, WriteExt};
use crate::world::wrath::MovementInfo;
use crate::world::wrath::CMSG_CHAR_CREATE;
use crate::world::wrath::CMSG_CHAR_ENUM;
use crate::world::wrath::CMSG_CHAR_DELETE;
use crate::world::wrath::CMSG_PLAYER_LOGIN;
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
use crate::world::wrath::CMSG_ACTIVATETAXI;
use crate::world::wrath::CMSG_PING;
use crate::world::wrath::CMSG_AUTH_SESSION;
use crate::world::wrath::CMSG_REQUEST_ACCOUNT_DATA;
use crate::world::wrath::CMSG_UPDATE_ACCOUNT_DATA;
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
    CMSG_CHAR_CREATE(CMSG_CHAR_CREATE),
    CMSG_CHAR_ENUM(CMSG_CHAR_ENUM),
    CMSG_CHAR_DELETE(CMSG_CHAR_DELETE),
    CMSG_PLAYER_LOGIN(CMSG_PLAYER_LOGIN),
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
    CMSG_ACTIVATETAXI(CMSG_ACTIVATETAXI),
    CMSG_PING(CMSG_PING),
    CMSG_AUTH_SESSION(CMSG_AUTH_SESSION),
    CMSG_REQUEST_ACCOUNT_DATA(CMSG_REQUEST_ACCOUNT_DATA),
    CMSG_UPDATE_ACCOUNT_DATA(CMSG_UPDATE_ACCOUNT_DATA),
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
            0x0036 => Ok(Self::CMSG_CHAR_CREATE(<CMSG_CHAR_CREATE as crate::Message>::read_body(&mut r, body_size)?)),
            0x0037 => Ok(Self::CMSG_CHAR_ENUM(<CMSG_CHAR_ENUM as crate::Message>::read_body(&mut r, body_size)?)),
            0x0038 => Ok(Self::CMSG_CHAR_DELETE(<CMSG_CHAR_DELETE as crate::Message>::read_body(&mut r, body_size)?)),
            0x003D => Ok(Self::CMSG_PLAYER_LOGIN(<CMSG_PLAYER_LOGIN as crate::Message>::read_body(&mut r, body_size)?)),
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
            0x01AD => Ok(Self::CMSG_ACTIVATETAXI(<CMSG_ACTIVATETAXI as crate::Message>::read_body(&mut r, body_size)?)),
            0x01DC => Ok(Self::CMSG_PING(<CMSG_PING as crate::Message>::read_body(&mut r, body_size)?)),
            0x01ED => Ok(Self::CMSG_AUTH_SESSION(<CMSG_AUTH_SESSION as crate::Message>::read_body(&mut r, body_size)?)),
            0x020A => Ok(Self::CMSG_REQUEST_ACCOUNT_DATA(<CMSG_REQUEST_ACCOUNT_DATA as crate::Message>::read_body(&mut r, body_size)?)),
            0x020B => Ok(Self::CMSG_UPDATE_ACCOUNT_DATA(<CMSG_UPDATE_ACCOUNT_DATA as crate::Message>::read_body(&mut r, body_size)?)),
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
            Self::CMSG_CHAR_CREATE(c) => c.write_encrypted_client(w, e),
            Self::CMSG_CHAR_ENUM(c) => c.write_encrypted_client(w, e),
            Self::CMSG_CHAR_DELETE(c) => c.write_encrypted_client(w, e),
            Self::CMSG_PLAYER_LOGIN(c) => c.write_encrypted_client(w, e),
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
            Self::CMSG_ACTIVATETAXI(c) => c.write_encrypted_client(w, e),
            Self::CMSG_PING(c) => c.write_encrypted_client(w, e),
            Self::CMSG_AUTH_SESSION(c) => c.write_encrypted_client(w, e),
            Self::CMSG_REQUEST_ACCOUNT_DATA(c) => c.write_encrypted_client(w, e),
            Self::CMSG_UPDATE_ACCOUNT_DATA(c) => c.write_encrypted_client(w, e),
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
            Self::CMSG_CHAR_CREATE(c) => c.write_unencrypted_client(w),
            Self::CMSG_CHAR_ENUM(c) => c.write_unencrypted_client(w),
            Self::CMSG_CHAR_DELETE(c) => c.write_unencrypted_client(w),
            Self::CMSG_PLAYER_LOGIN(c) => c.write_unencrypted_client(w),
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
            Self::CMSG_ACTIVATETAXI(c) => c.write_unencrypted_client(w),
            Self::CMSG_PING(c) => c.write_unencrypted_client(w),
            Self::CMSG_AUTH_SESSION(c) => c.write_unencrypted_client(w),
            Self::CMSG_REQUEST_ACCOUNT_DATA(c) => c.write_unencrypted_client(w),
            Self::CMSG_UPDATE_ACCOUNT_DATA(c) => c.write_unencrypted_client(w),
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
            Self::CMSG_CHAR_CREATE(c) => c.tokio_write_encrypted_client(w, e).await,
            Self::CMSG_CHAR_ENUM(c) => c.tokio_write_encrypted_client(w, e).await,
            Self::CMSG_CHAR_DELETE(c) => c.tokio_write_encrypted_client(w, e).await,
            Self::CMSG_PLAYER_LOGIN(c) => c.tokio_write_encrypted_client(w, e).await,
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
            Self::CMSG_ACTIVATETAXI(c) => c.tokio_write_encrypted_client(w, e).await,
            Self::CMSG_PING(c) => c.tokio_write_encrypted_client(w, e).await,
            Self::CMSG_AUTH_SESSION(c) => c.tokio_write_encrypted_client(w, e).await,
            Self::CMSG_REQUEST_ACCOUNT_DATA(c) => c.tokio_write_encrypted_client(w, e).await,
            Self::CMSG_UPDATE_ACCOUNT_DATA(c) => c.tokio_write_encrypted_client(w, e).await,
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
            Self::CMSG_CHAR_CREATE(c) => c.tokio_write_unencrypted_client(w).await,
            Self::CMSG_CHAR_ENUM(c) => c.tokio_write_unencrypted_client(w).await,
            Self::CMSG_CHAR_DELETE(c) => c.tokio_write_unencrypted_client(w).await,
            Self::CMSG_PLAYER_LOGIN(c) => c.tokio_write_unencrypted_client(w).await,
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
            Self::CMSG_ACTIVATETAXI(c) => c.tokio_write_unencrypted_client(w).await,
            Self::CMSG_PING(c) => c.tokio_write_unencrypted_client(w).await,
            Self::CMSG_AUTH_SESSION(c) => c.tokio_write_unencrypted_client(w).await,
            Self::CMSG_REQUEST_ACCOUNT_DATA(c) => c.tokio_write_unencrypted_client(w).await,
            Self::CMSG_UPDATE_ACCOUNT_DATA(c) => c.tokio_write_unencrypted_client(w).await,
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
            Self::CMSG_CHAR_CREATE(c) => c.astd_write_encrypted_client(w, e).await,
            Self::CMSG_CHAR_ENUM(c) => c.astd_write_encrypted_client(w, e).await,
            Self::CMSG_CHAR_DELETE(c) => c.astd_write_encrypted_client(w, e).await,
            Self::CMSG_PLAYER_LOGIN(c) => c.astd_write_encrypted_client(w, e).await,
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
            Self::CMSG_ACTIVATETAXI(c) => c.astd_write_encrypted_client(w, e).await,
            Self::CMSG_PING(c) => c.astd_write_encrypted_client(w, e).await,
            Self::CMSG_AUTH_SESSION(c) => c.astd_write_encrypted_client(w, e).await,
            Self::CMSG_REQUEST_ACCOUNT_DATA(c) => c.astd_write_encrypted_client(w, e).await,
            Self::CMSG_UPDATE_ACCOUNT_DATA(c) => c.astd_write_encrypted_client(w, e).await,
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
            Self::CMSG_CHAR_CREATE(c) => c.astd_write_unencrypted_client(w).await,
            Self::CMSG_CHAR_ENUM(c) => c.astd_write_unencrypted_client(w).await,
            Self::CMSG_CHAR_DELETE(c) => c.astd_write_unencrypted_client(w).await,
            Self::CMSG_PLAYER_LOGIN(c) => c.astd_write_unencrypted_client(w).await,
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
            Self::CMSG_ACTIVATETAXI(c) => c.astd_write_unencrypted_client(w).await,
            Self::CMSG_PING(c) => c.astd_write_unencrypted_client(w).await,
            Self::CMSG_AUTH_SESSION(c) => c.astd_write_unencrypted_client(w).await,
            Self::CMSG_REQUEST_ACCOUNT_DATA(c) => c.astd_write_unencrypted_client(w).await,
            Self::CMSG_UPDATE_ACCOUNT_DATA(c) => c.astd_write_unencrypted_client(w).await,
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
            ClientOpcodeMessage::CMSG_CHAR_CREATE(_) => "CMSG_CHAR_CREATE",
            ClientOpcodeMessage::CMSG_CHAR_ENUM(_) => "CMSG_CHAR_ENUM",
            ClientOpcodeMessage::CMSG_CHAR_DELETE(_) => "CMSG_CHAR_DELETE",
            ClientOpcodeMessage::CMSG_PLAYER_LOGIN(_) => "CMSG_PLAYER_LOGIN",
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
            ClientOpcodeMessage::CMSG_ACTIVATETAXI(_) => "CMSG_ACTIVATETAXI",
            ClientOpcodeMessage::CMSG_PING(_) => "CMSG_PING",
            ClientOpcodeMessage::CMSG_AUTH_SESSION(_) => "CMSG_AUTH_SESSION",
            ClientOpcodeMessage::CMSG_REQUEST_ACCOUNT_DATA(_) => "CMSG_REQUEST_ACCOUNT_DATA",
            ClientOpcodeMessage::CMSG_UPDATE_ACCOUNT_DATA(_) => "CMSG_UPDATE_ACCOUNT_DATA",
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

impl From<CMSG_AUTH_SESSION> for ClientOpcodeMessage {
    fn from(c: CMSG_AUTH_SESSION) -> Self {
        Self::CMSG_AUTH_SESSION(c)
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
use crate::world::wrath::SMSG_TUTORIAL_FLAGS;
use crate::world::wrath::SMSG_PONG;
use crate::world::wrath::SMSG_AUTH_CHALLENGE;
use crate::world::wrath::SMSG_AUTH_RESPONSE;
use crate::world::wrath::SMSG_ACCOUNT_DATA_TIMES;
use crate::world::wrath::SMSG_UPDATE_ACCOUNT_DATA;
use crate::world::wrath::SMSG_LOGIN_VERIFY_WORLD;
use crate::world::wrath::SMSG_CHAR_RENAME;
use crate::world::wrath::SMSG_ADDON_INFO;
use crate::world::wrath::MSG_MOVE_START_ASCEND_Server;
use crate::world::wrath::MSG_MOVE_STOP_ASCEND_Server;
use crate::world::wrath::SMSG_REALM_SPLIT;
use crate::world::wrath::SMSG_TIME_SYNC_REQ;
use crate::world::wrath::MSG_MOVE_START_DESCEND_Server;
use crate::world::wrath::SMSG_UPDATE_ACCOUNT_DATA_COMPLETE;
use crate::world::wrath::SMSG_CLIENTCACHE_VERSION;

#[derive(Debug, Clone, PartialEq)]
pub enum ServerOpcodeMessage {
    SMSG_CHAR_CREATE(SMSG_CHAR_CREATE),
    SMSG_CHAR_ENUM(SMSG_CHAR_ENUM),
    SMSG_CHAR_DELETE(SMSG_CHAR_DELETE),
    SMSG_CHARACTER_LOGIN_FAILED(SMSG_CHARACTER_LOGIN_FAILED),
    SMSG_LOGIN_SETTIMESPEED(SMSG_LOGIN_SETTIMESPEED),
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
    SMSG_TUTORIAL_FLAGS(SMSG_TUTORIAL_FLAGS),
    SMSG_PONG(SMSG_PONG),
    SMSG_AUTH_CHALLENGE(SMSG_AUTH_CHALLENGE),
    SMSG_AUTH_RESPONSE(SMSG_AUTH_RESPONSE),
    SMSG_ACCOUNT_DATA_TIMES(SMSG_ACCOUNT_DATA_TIMES),
    SMSG_UPDATE_ACCOUNT_DATA(SMSG_UPDATE_ACCOUNT_DATA),
    SMSG_LOGIN_VERIFY_WORLD(SMSG_LOGIN_VERIFY_WORLD),
    SMSG_CHAR_RENAME(SMSG_CHAR_RENAME),
    SMSG_ADDON_INFO(SMSG_ADDON_INFO),
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
            0x003A => Ok(Self::SMSG_CHAR_CREATE(<SMSG_CHAR_CREATE as crate::Message>::read_body(&mut r, body_size)?)),
            0x003B => Ok(Self::SMSG_CHAR_ENUM(<SMSG_CHAR_ENUM as crate::Message>::read_body(&mut r, body_size)?)),
            0x003C => Ok(Self::SMSG_CHAR_DELETE(<SMSG_CHAR_DELETE as crate::Message>::read_body(&mut r, body_size)?)),
            0x0041 => Ok(Self::SMSG_CHARACTER_LOGIN_FAILED(<SMSG_CHARACTER_LOGIN_FAILED as crate::Message>::read_body(&mut r, body_size)?)),
            0x0042 => Ok(Self::SMSG_LOGIN_SETTIMESPEED(<SMSG_LOGIN_SETTIMESPEED as crate::Message>::read_body(&mut r, body_size)?)),
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
            0x00FD => Ok(Self::SMSG_TUTORIAL_FLAGS(<SMSG_TUTORIAL_FLAGS as crate::Message>::read_body(&mut r, body_size)?)),
            0x01DD => Ok(Self::SMSG_PONG(<SMSG_PONG as crate::Message>::read_body(&mut r, body_size)?)),
            0x01EC => Ok(Self::SMSG_AUTH_CHALLENGE(<SMSG_AUTH_CHALLENGE as crate::Message>::read_body(&mut r, body_size)?)),
            0x01EE => Ok(Self::SMSG_AUTH_RESPONSE(<SMSG_AUTH_RESPONSE as crate::Message>::read_body(&mut r, body_size)?)),
            0x0209 => Ok(Self::SMSG_ACCOUNT_DATA_TIMES(<SMSG_ACCOUNT_DATA_TIMES as crate::Message>::read_body(&mut r, body_size)?)),
            0x020C => Ok(Self::SMSG_UPDATE_ACCOUNT_DATA(<SMSG_UPDATE_ACCOUNT_DATA as crate::Message>::read_body(&mut r, body_size)?)),
            0x0236 => Ok(Self::SMSG_LOGIN_VERIFY_WORLD(<SMSG_LOGIN_VERIFY_WORLD as crate::Message>::read_body(&mut r, body_size)?)),
            0x02C8 => Ok(Self::SMSG_CHAR_RENAME(<SMSG_CHAR_RENAME as crate::Message>::read_body(&mut r, body_size)?)),
            0x02EF => Ok(Self::SMSG_ADDON_INFO(<SMSG_ADDON_INFO as crate::Message>::read_body(&mut r, body_size)?)),
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
            Self::SMSG_CHAR_CREATE(c) => c.write_encrypted_server(w, e),
            Self::SMSG_CHAR_ENUM(c) => c.write_encrypted_server(w, e),
            Self::SMSG_CHAR_DELETE(c) => c.write_encrypted_server(w, e),
            Self::SMSG_CHARACTER_LOGIN_FAILED(c) => c.write_encrypted_server(w, e),
            Self::SMSG_LOGIN_SETTIMESPEED(c) => c.write_encrypted_server(w, e),
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
            Self::SMSG_TUTORIAL_FLAGS(c) => c.write_encrypted_server(w, e),
            Self::SMSG_PONG(c) => c.write_encrypted_server(w, e),
            Self::SMSG_AUTH_CHALLENGE(c) => c.write_encrypted_server(w, e),
            Self::SMSG_AUTH_RESPONSE(c) => c.write_encrypted_server(w, e),
            Self::SMSG_ACCOUNT_DATA_TIMES(c) => c.write_encrypted_server(w, e),
            Self::SMSG_UPDATE_ACCOUNT_DATA(c) => c.write_encrypted_server(w, e),
            Self::SMSG_LOGIN_VERIFY_WORLD(c) => c.write_encrypted_server(w, e),
            Self::SMSG_CHAR_RENAME(c) => c.write_encrypted_server(w, e),
            Self::SMSG_ADDON_INFO(c) => c.write_encrypted_server(w, e),
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
            Self::SMSG_CHAR_CREATE(c) => c.write_unencrypted_server(w),
            Self::SMSG_CHAR_ENUM(c) => c.write_unencrypted_server(w),
            Self::SMSG_CHAR_DELETE(c) => c.write_unencrypted_server(w),
            Self::SMSG_CHARACTER_LOGIN_FAILED(c) => c.write_unencrypted_server(w),
            Self::SMSG_LOGIN_SETTIMESPEED(c) => c.write_unencrypted_server(w),
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
            Self::SMSG_TUTORIAL_FLAGS(c) => c.write_unencrypted_server(w),
            Self::SMSG_PONG(c) => c.write_unencrypted_server(w),
            Self::SMSG_AUTH_CHALLENGE(c) => c.write_unencrypted_server(w),
            Self::SMSG_AUTH_RESPONSE(c) => c.write_unencrypted_server(w),
            Self::SMSG_ACCOUNT_DATA_TIMES(c) => c.write_unencrypted_server(w),
            Self::SMSG_UPDATE_ACCOUNT_DATA(c) => c.write_unencrypted_server(w),
            Self::SMSG_LOGIN_VERIFY_WORLD(c) => c.write_unencrypted_server(w),
            Self::SMSG_CHAR_RENAME(c) => c.write_unencrypted_server(w),
            Self::SMSG_ADDON_INFO(c) => c.write_unencrypted_server(w),
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
            Self::SMSG_CHAR_CREATE(c) => c.tokio_write_encrypted_server(w, e).await,
            Self::SMSG_CHAR_ENUM(c) => c.tokio_write_encrypted_server(w, e).await,
            Self::SMSG_CHAR_DELETE(c) => c.tokio_write_encrypted_server(w, e).await,
            Self::SMSG_CHARACTER_LOGIN_FAILED(c) => c.tokio_write_encrypted_server(w, e).await,
            Self::SMSG_LOGIN_SETTIMESPEED(c) => c.tokio_write_encrypted_server(w, e).await,
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
            Self::SMSG_TUTORIAL_FLAGS(c) => c.tokio_write_encrypted_server(w, e).await,
            Self::SMSG_PONG(c) => c.tokio_write_encrypted_server(w, e).await,
            Self::SMSG_AUTH_CHALLENGE(c) => c.tokio_write_encrypted_server(w, e).await,
            Self::SMSG_AUTH_RESPONSE(c) => c.tokio_write_encrypted_server(w, e).await,
            Self::SMSG_ACCOUNT_DATA_TIMES(c) => c.tokio_write_encrypted_server(w, e).await,
            Self::SMSG_UPDATE_ACCOUNT_DATA(c) => c.tokio_write_encrypted_server(w, e).await,
            Self::SMSG_LOGIN_VERIFY_WORLD(c) => c.tokio_write_encrypted_server(w, e).await,
            Self::SMSG_CHAR_RENAME(c) => c.tokio_write_encrypted_server(w, e).await,
            Self::SMSG_ADDON_INFO(c) => c.tokio_write_encrypted_server(w, e).await,
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
            Self::SMSG_CHAR_CREATE(c) => c.tokio_write_unencrypted_server(w).await,
            Self::SMSG_CHAR_ENUM(c) => c.tokio_write_unencrypted_server(w).await,
            Self::SMSG_CHAR_DELETE(c) => c.tokio_write_unencrypted_server(w).await,
            Self::SMSG_CHARACTER_LOGIN_FAILED(c) => c.tokio_write_unencrypted_server(w).await,
            Self::SMSG_LOGIN_SETTIMESPEED(c) => c.tokio_write_unencrypted_server(w).await,
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
            Self::SMSG_TUTORIAL_FLAGS(c) => c.tokio_write_unencrypted_server(w).await,
            Self::SMSG_PONG(c) => c.tokio_write_unencrypted_server(w).await,
            Self::SMSG_AUTH_CHALLENGE(c) => c.tokio_write_unencrypted_server(w).await,
            Self::SMSG_AUTH_RESPONSE(c) => c.tokio_write_unencrypted_server(w).await,
            Self::SMSG_ACCOUNT_DATA_TIMES(c) => c.tokio_write_unencrypted_server(w).await,
            Self::SMSG_UPDATE_ACCOUNT_DATA(c) => c.tokio_write_unencrypted_server(w).await,
            Self::SMSG_LOGIN_VERIFY_WORLD(c) => c.tokio_write_unencrypted_server(w).await,
            Self::SMSG_CHAR_RENAME(c) => c.tokio_write_unencrypted_server(w).await,
            Self::SMSG_ADDON_INFO(c) => c.tokio_write_unencrypted_server(w).await,
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
            Self::SMSG_CHAR_CREATE(c) => c.astd_write_encrypted_server(w, e).await,
            Self::SMSG_CHAR_ENUM(c) => c.astd_write_encrypted_server(w, e).await,
            Self::SMSG_CHAR_DELETE(c) => c.astd_write_encrypted_server(w, e).await,
            Self::SMSG_CHARACTER_LOGIN_FAILED(c) => c.astd_write_encrypted_server(w, e).await,
            Self::SMSG_LOGIN_SETTIMESPEED(c) => c.astd_write_encrypted_server(w, e).await,
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
            Self::SMSG_TUTORIAL_FLAGS(c) => c.astd_write_encrypted_server(w, e).await,
            Self::SMSG_PONG(c) => c.astd_write_encrypted_server(w, e).await,
            Self::SMSG_AUTH_CHALLENGE(c) => c.astd_write_encrypted_server(w, e).await,
            Self::SMSG_AUTH_RESPONSE(c) => c.astd_write_encrypted_server(w, e).await,
            Self::SMSG_ACCOUNT_DATA_TIMES(c) => c.astd_write_encrypted_server(w, e).await,
            Self::SMSG_UPDATE_ACCOUNT_DATA(c) => c.astd_write_encrypted_server(w, e).await,
            Self::SMSG_LOGIN_VERIFY_WORLD(c) => c.astd_write_encrypted_server(w, e).await,
            Self::SMSG_CHAR_RENAME(c) => c.astd_write_encrypted_server(w, e).await,
            Self::SMSG_ADDON_INFO(c) => c.astd_write_encrypted_server(w, e).await,
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
            Self::SMSG_CHAR_CREATE(c) => c.astd_write_unencrypted_server(w).await,
            Self::SMSG_CHAR_ENUM(c) => c.astd_write_unencrypted_server(w).await,
            Self::SMSG_CHAR_DELETE(c) => c.astd_write_unencrypted_server(w).await,
            Self::SMSG_CHARACTER_LOGIN_FAILED(c) => c.astd_write_unencrypted_server(w).await,
            Self::SMSG_LOGIN_SETTIMESPEED(c) => c.astd_write_unencrypted_server(w).await,
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
            Self::SMSG_TUTORIAL_FLAGS(c) => c.astd_write_unencrypted_server(w).await,
            Self::SMSG_PONG(c) => c.astd_write_unencrypted_server(w).await,
            Self::SMSG_AUTH_CHALLENGE(c) => c.astd_write_unencrypted_server(w).await,
            Self::SMSG_AUTH_RESPONSE(c) => c.astd_write_unencrypted_server(w).await,
            Self::SMSG_ACCOUNT_DATA_TIMES(c) => c.astd_write_unencrypted_server(w).await,
            Self::SMSG_UPDATE_ACCOUNT_DATA(c) => c.astd_write_unencrypted_server(w).await,
            Self::SMSG_LOGIN_VERIFY_WORLD(c) => c.astd_write_unencrypted_server(w).await,
            Self::SMSG_CHAR_RENAME(c) => c.astd_write_unencrypted_server(w).await,
            Self::SMSG_ADDON_INFO(c) => c.astd_write_unencrypted_server(w).await,
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
            ServerOpcodeMessage::SMSG_CHAR_CREATE(_) => "SMSG_CHAR_CREATE",
            ServerOpcodeMessage::SMSG_CHAR_ENUM(_) => "SMSG_CHAR_ENUM",
            ServerOpcodeMessage::SMSG_CHAR_DELETE(_) => "SMSG_CHAR_DELETE",
            ServerOpcodeMessage::SMSG_CHARACTER_LOGIN_FAILED(_) => "SMSG_CHARACTER_LOGIN_FAILED",
            ServerOpcodeMessage::SMSG_LOGIN_SETTIMESPEED(_) => "SMSG_LOGIN_SETTIMESPEED",
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
            ServerOpcodeMessage::SMSG_TUTORIAL_FLAGS(_) => "SMSG_TUTORIAL_FLAGS",
            ServerOpcodeMessage::SMSG_PONG(_) => "SMSG_PONG",
            ServerOpcodeMessage::SMSG_AUTH_CHALLENGE(_) => "SMSG_AUTH_CHALLENGE",
            ServerOpcodeMessage::SMSG_AUTH_RESPONSE(_) => "SMSG_AUTH_RESPONSE",
            ServerOpcodeMessage::SMSG_ACCOUNT_DATA_TIMES(_) => "SMSG_ACCOUNT_DATA_TIMES",
            ServerOpcodeMessage::SMSG_UPDATE_ACCOUNT_DATA(_) => "SMSG_UPDATE_ACCOUNT_DATA",
            ServerOpcodeMessage::SMSG_LOGIN_VERIFY_WORLD(_) => "SMSG_LOGIN_VERIFY_WORLD",
            ServerOpcodeMessage::SMSG_CHAR_RENAME(_) => "SMSG_CHAR_RENAME",
            ServerOpcodeMessage::SMSG_ADDON_INFO(_) => "SMSG_ADDON_INFO",
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

impl From<SMSG_TUTORIAL_FLAGS> for ServerOpcodeMessage {
    fn from(c: SMSG_TUTORIAL_FLAGS) -> Self {
        Self::SMSG_TUTORIAL_FLAGS(c)
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

impl From<SMSG_LOGIN_VERIFY_WORLD> for ServerOpcodeMessage {
    fn from(c: SMSG_LOGIN_VERIFY_WORLD) -> Self {
        Self::SMSG_LOGIN_VERIFY_WORLD(c)
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

