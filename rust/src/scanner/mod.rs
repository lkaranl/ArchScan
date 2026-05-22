pub mod ports;
pub mod tcp;
pub mod udp;

use std::sync::{
    atomic::{AtomicUsize, Ordering},
    Arc, Mutex,
};
use std::time::{Duration, Instant};

use futures::future::join_all;
use tokio::sync::Semaphore;

use crate::models::{Protocol, ScanConfig, ScanResults};
use ports::parse_ports;
use tcp::scan_tcp;
use udp::scan_udp;

/// Número máximo de tarefas assíncronas simultâneas.
/// Valor alto → varreduras rápidas; valor baixo → menor pressão na rede/alvo.
const MAX_CONCURRENT: usize = 500;

/// Inicia a varredura em uma thread separada do sistema operacional
/// (para não bloquear a thread da GUI do egui).
///
/// A thread cria seu próprio runtime tokio e escreve os resultados no
/// `Arc<Mutex<ScanResults>>` compartilhado com a GUI.
pub fn start_scan(config: ScanConfig, results: Arc<Mutex<ScanResults>>) {
    std::thread::spawn(move || {
        let rt = tokio::runtime::Runtime::new().expect("Falha ao criar runtime tokio");
        rt.block_on(run_scan(config, results));
    });
}

async fn run_scan(config: ScanConfig, results: Arc<Mutex<ScanResults>>) {
    // Parsear portas — erros são reportados na GUI
    let ports = match parse_ports(&config.ports) {
        Ok(p) => p,
        Err(e) => {
            let mut r = results.lock().unwrap();
            r.error = Some(e);
            r.is_running = false;
            return;
        }
    };

    let total = ports.len();
    {
        let mut r = results.lock().unwrap();
        r.total_ports = total;
    }

    let timeout   = Duration::from_secs(config.timeout_secs);
    let semaphore = Arc::new(Semaphore::new(MAX_CONCURRENT));
    let completed = Arc::new(AtomicUsize::new(0));
    let start     = Instant::now();

    let mut handles = Vec::with_capacity(total);

    for port in ports {
        let sem       = semaphore.clone();
        let res_ref   = results.clone();
        let done_ref  = completed.clone();
        let ip        = config.ip.clone();
        let protocol  = config.protocol.clone();
        let start_ref = start;

        handles.push(tokio::spawn(async move {
            // Controla o número máximo de tasks concorrentes
            let _permit = sem.acquire().await.unwrap();

            let state = match protocol {
                Protocol::Tcp => scan_tcp(ip, port, timeout).await,
                Protocol::Udp => scan_udp(ip, port, timeout).await,
            };

            let done = done_ref.fetch_add(1, Ordering::Relaxed) + 1;

            let mut r = res_ref.lock().unwrap();
            r.push(port, state);
            r.progress     = done as f32 / total as f32;
            r.elapsed_secs = start_ref.elapsed().as_secs_f64();
        }));
    }

    join_all(handles).await;

    // Finalizar
    let mut r = results.lock().unwrap();
    // Ordenar resultados para exibição consistente
    r.open.sort_unstable();
    r.closed.sort_unstable();
    r.filtered.sort_unstable();
    r.open_filtered.sort_unstable();
    r.is_running   = false;
    r.progress     = 1.0;
    r.elapsed_secs = start.elapsed().as_secs_f64();
}
