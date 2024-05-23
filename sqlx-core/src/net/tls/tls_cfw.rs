use crate::io::ReadBuf;
use crate::net::tls::util::StdSocket;
use crate::net::tls::TlsConfig;
use crate::net::Socket;
use crate::Error;

pub async fn handshake<S: Socket>(
    socket: S,
    config: TlsConfig<'_>,
) -> crate::Result<worker::Socket> {
    todo!()
}
