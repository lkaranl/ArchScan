/// Tipos e estruturas compartilhados entre a GUI e o scanner.

// ─── Protocolo ───────────────────────────────────────────────────────────────

#[derive(Debug, Clone, PartialEq, Default)]
pub enum Protocol {
    #[default]
    Tcp,
    Udp,
}

impl std::fmt::Display for Protocol {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Protocol::Tcp => write!(f, "TCP"),
            Protocol::Udp => write!(f, "UDP"),
        }
    }
}

// ─── Estado de uma porta ─────────────────────────────────────────────────────

#[derive(Debug, Clone, PartialEq)]
pub enum PortState {
    Open,
    Closed,
    Filtered,
    OpenFiltered,
}

// ─── Configuração de varredura ────────────────────────────────────────────────

#[derive(Debug, Clone)]
pub struct ScanConfig {
    pub ip: String,
    /// Range ("0-1024"), lista ("22,80,443") ou porta única ("80")
    pub ports: String,
    pub protocol: Protocol,
    pub timeout_secs: u64,
}

// ─── Resultados ───────────────────────────────────────────────────────────────

#[derive(Debug, Clone, Default)]
pub struct ScanResults {
    pub open: Vec<u16>,
    pub closed: Vec<u16>,
    pub filtered: Vec<u16>,
    pub open_filtered: Vec<u16>,
    /// `true` enquanto a varredura estiver em andamento
    pub is_running: bool,
    /// Progresso de 0.0 a 1.0
    pub progress: f32,
    /// Segundos decorridos desde o início da varredura
    pub elapsed_secs: f64,
    /// Mensagem de erro, caso haja algum
    pub error: Option<String>,
    /// Total de portas a escanear (para cálculo de progresso)
    pub total_ports: usize,
}

impl ScanResults {
    pub fn push(&mut self, port: u16, state: PortState) {
        match state {
            PortState::Open         => self.open.push(port),
            PortState::Closed       => self.closed.push(port),
            PortState::Filtered     => self.filtered.push(port),
            PortState::OpenFiltered => self.open_filtered.push(port),
        }
    }

    pub fn clear(&mut self) {
        self.open.clear();
        self.closed.clear();
        self.filtered.clear();
        self.open_filtered.clear();
        self.error        = None;
        self.progress     = 0.0;
        self.elapsed_secs = 0.0;
        self.total_ports  = 0;
    }

    pub fn scanned_so_far(&self) -> usize {
        self.open.len() + self.closed.len() + self.filtered.len() + self.open_filtered.len()
    }
}
