use std::net::TcpStream;

mod auth;
mod server;

const USERNAME: &str = "A";
const PASSWORD: &str = USERNAME;

fn main() {
    let (session_key, mut world_server, server_id) = {
        let mut auth_server = TcpStream::connect("127.0.0.1:3724").unwrap();
        let (session_key, realms) = auth::auth(&mut auth_server);
        (
            session_key,
            TcpStream::connect(&realms.realms[0].address).unwrap(),
            realms.realms[0].realm_id,
        )
        // Real client technically waits until successfully connected
        // to the realm to disconnect from auth server but this is easier control flow
    };
    server::server(&mut world_server, session_key, server_id);
}
