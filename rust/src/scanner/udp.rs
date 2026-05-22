/// Scanner UDP assíncrono usando tokio.
///
/// Envia payloads específicos para serviços conhecidos (DNS, NTP, SNMP) e um
/// payload genérico "archscan" para as demais portas — idêntico ao original Python.
///
/// Estado determinado pela resposta (ou ausência dela):
/// - `Open`         → resposta recebida dentro do timeout
/// - `OpenFiltered` → timeout (sem resposta, mas sem ICMP "unreachable" — sem raw socket)
///
/// Nota: detectar portas verdadeiramente `Closed` via ICMP "port unreachable"
/// requer raw sockets e privilégios de root. Nessa implementação, timeouts UDP
/// são classificados como `OpenFiltered` (comportamento idêntico ao Nmap padrão).

use std::time::Duration;
use tokio::net::UdpSocket;

use crate::models::PortState;

// ─── Payloads (idênticos ao scanner.py original) ─────────────────────────────

/// Payload genérico "archscan" em ASCII
const PAYLOAD_ARCHSCAN: &[u8] = b"archscan";

/// Query DNS para www.google.com (tipo A)
const PAYLOAD_DNS: &[u8] = &[
    0x24, 0x1a, 0x01, 0x00, 0x00, 0x01, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x03, 0x77, 0x77, 0x77,
    0x06, 0x67, 0x6f, 0x6f, 0x67, 0x6c, 0x65, 0x03,
    0x63, 0x6f, 0x6d, 0x00, 0x00, 0x01, 0x00, 0x01,
];

/// SNMP GetRequest (community: public)
const PAYLOAD_SNMP: &[u8] = &[
    0x30, 0x2c, 0x02, 0x01, 0x00, 0x04, 0x07, 0x70,
    0x75, 0x62, 0x6c, 0x69, 0x63, 0xa0, 0x1e, 0x02,
    0x01, 0x01, 0x02, 0x01, 0x00, 0x02, 0x01, 0x00,
    0x30, 0x13, 0x30, 0x11, 0x06, 0x0d, 0x2b, 0x06,
    0x01, 0x04, 0x01, 0x94, 0x78, 0x01, 0x02, 0x07,
    0x03, 0x02, 0x00, 0x05, 0x00,
];

/// NTP modo‑3 (client request)
const PAYLOAD_NTP: &[u8] = &[
    0xe3, 0x00, 0x04, 0xfa, 0x00, 0x01, 0x00, 0x00,
    0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0xc5, 0x4f, 0x23, 0x4b, 0x71, 0xb1, 0x52, 0xf3,
];

// ─── Função principal ─────────────────────────────────────────────────────────

pub async fn scan_udp(ip: String, port: u16, timeout: Duration) -> PortState {
    let payload = match port {
        53  => PAYLOAD_DNS,
        123 => PAYLOAD_NTP,
        161 => PAYLOAD_SNMP,
        _   => PAYLOAD_ARCHSCAN,
    };

    // Bind em qualquer porta local disponível
    let socket = match UdpSocket::bind("0.0.0.0:0").await {
        Ok(s) => s,
        Err(_) => return PortState::OpenFiltered,
    };

    let target = format!("{}:{}", ip, port);

    if socket.send_to(payload, &target).await.is_err() {
        return PortState::OpenFiltered;
    }

    let mut buf = [0u8; 1024];
    match tokio::time::timeout(timeout, socket.recv_from(&mut buf)).await {
        Ok(Ok(_)) => PortState::Open,
        // Timeout ou erro → sem informação suficiente sem raw sockets
        _ => PortState::OpenFiltered,
    }
}
