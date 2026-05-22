/// Scanner TCP assíncrono usando tokio.
///
/// O estado é determinado pelo resultado da tentativa de conexão TCP:
/// - `Open`     → conexão estabelecida com sucesso (SYN‑ACK recebido)
/// - `Closed`   → conexão recusada (RST recebido)
/// - `Filtered` → timeout ou outro erro de rede (sem resposta)

use std::io::ErrorKind;
use std::time::Duration;
use tokio::net::TcpStream;

use crate::models::PortState;

pub async fn scan_tcp(ip: String, port: u16, timeout: Duration) -> PortState {
    let addr = format!("{}:{}", ip, port);

    match tokio::time::timeout(timeout, TcpStream::connect(&addr)).await {
        // Conexão bem‑sucedida → porta aberta
        Ok(Ok(_stream)) => PortState::Open,

        // Erro de conexão → classificar pelo tipo de erro
        Ok(Err(e)) => match e.kind() {
            ErrorKind::ConnectionRefused => PortState::Closed,
            ErrorKind::ConnectionReset  => PortState::Closed,
            _                           => PortState::Filtered,
        },

        // Timeout → porta filtrada (sem resposta)
        Err(_elapsed) => PortState::Filtered,
    }
}
