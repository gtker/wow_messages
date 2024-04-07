use crate::all::{CMD_AUTH_LOGON_CHALLENGE_Client, CMD_AUTH_RECONNECT_CHALLENGE_Client};

impl From<CMD_AUTH_RECONNECT_CHALLENGE_Client> for CMD_AUTH_LOGON_CHALLENGE_Client {
    fn from(c: CMD_AUTH_RECONNECT_CHALLENGE_Client) -> Self {
        Self {
            protocol_version: c.protocol_version,
            version: c.version,
            platform: c.platform,
            os: c.os,
            locale: c.locale,
            utc_timezone_offset: c.utc_timezone_offset,
            client_ip_address: c.client_ip_address,
            account_name: c.account_name,
        }
    }
}

impl From<CMD_AUTH_LOGON_CHALLENGE_Client> for CMD_AUTH_RECONNECT_CHALLENGE_Client {
    fn from(c: CMD_AUTH_LOGON_CHALLENGE_Client) -> Self {
        Self {
            protocol_version: c.protocol_version,
            version: c.version,
            platform: c.platform,
            os: c.os,
            locale: c.locale,
            utc_timezone_offset: c.utc_timezone_offset,
            client_ip_address: c.client_ip_address,
            account_name: c.account_name,
        }
    }
}
