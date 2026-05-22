use std::sync::{Arc, Mutex};
use eframe::egui;

use crate::models::{Protocol, ScanConfig, ScanResults};
use crate::scanner::start_scan;

// ─── Controle de Abas ────────────────────────────────────────────────────────

#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub enum ActiveTab {
    #[default]
    Overview, // Tabela consolidada com portas ativas e serviços estimados
    Detailed, // Detalhes categorizados (Abertas, Fechadas, Filtradas, etc.)
    Help,     // Informações e ajuda sobre o utilitário
}

pub struct ArchScanApp {
    ip: String,
    ports: String,
    protocol: Protocol,
    timeout_secs: u64,
    results: Arc<Mutex<ScanResults>>,
    active_tab: ActiveTab,
}

impl Default for ArchScanApp {
    fn default() -> Self {
        Self {
            ip: "127.0.0.1".to_string(),
            ports: "0-1024".to_string(),
            protocol: Protocol::Tcp,
            timeout_secs: 2,
            results: Arc::new(Mutex::new(ScanResults::default())),
            active_tab: ActiveTab::Overview,
        }
    }
}

impl ArchScanApp {
    pub fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        let mut style = (*_cc.egui_ctx.style()).clone();

        use eframe::egui::{FontFamily, FontId, TextStyle};
        
        // Define tamanhos de fontes maiores e mais consistentes
        style.text_styles = [
            (TextStyle::Heading, FontId::new(24.0, FontFamily::Proportional)),
            (TextStyle::Name("Heading2".into()), FontId::new(20.0, FontFamily::Proportional)),
            (TextStyle::Name("Context".into()), FontId::new(18.0, FontFamily::Proportional)),
            (TextStyle::Body, FontId::new(15.0, FontFamily::Proportional)),
            (TextStyle::Monospace, FontId::new(15.0, FontFamily::Monospace)),
            (TextStyle::Button, FontId::new(16.0, FontFamily::Proportional)),
            (TextStyle::Small, FontId::new(13.0, FontFamily::Proportional)),
        ].into();

        // Configuração do Tema Dark Premium
        style.visuals.dark_mode = true;
        style.visuals.override_text_color = Some(egui::Color32::from_rgb(220, 225, 235));
        
        // Cantos arredondados consistentes (estilo web moderno)
        style.visuals.window_rounding = 12.0.into();
        style.visuals.widgets.noninteractive.rounding = 8.0.into();
        style.visuals.widgets.inactive.rounding = 6.0.into();
        style.visuals.widgets.hovered.rounding = 6.0.into();
        style.visuals.widgets.active.rounding = 6.0.into();
        
        // Cores suaves de fundo e inputs
        style.visuals.extreme_bg_color = egui::Color32::from_rgb(12, 15, 22);
        style.visuals.widgets.noninteractive.bg_fill = egui::Color32::from_rgb(23, 28, 41);
        style.visuals.widgets.inactive.bg_fill = egui::Color32::from_rgb(32, 38, 54);
        style.visuals.widgets.hovered.bg_fill = egui::Color32::from_rgb(42, 50, 71);
        style.visuals.widgets.active.bg_fill = egui::Color32::from_rgb(52, 62, 88);

        _cc.egui_ctx.set_style(style);

        Self::default()
    }
}

// ─── Auxiliar de Mapeamento de Serviços ──────────────────────────────────────

fn get_service_name(port: u16) -> &'static str {
    match port {
        21 => "FTP (Transferência de Arquivos)",
        22 => "SSH (Acesso Seguro Remoto)",
        23 => "Telnet (Console Remoto)",
        25 => "SMTP (Envio de E-mail)",
        53 => "DNS (Resolução de Nomes)",
        67 | 68 => "DHCP (Configuração de Rede)",
        80 => "HTTP (Web Não Criptografado)",
        110 => "POP3 (Recebimento de E-mail)",
        123 => "NTP (Sincronismo de Tempo)",
        137 | 138 | 139 => "NetBIOS (Rede Windows)",
        143 => "IMAP (Sincronismo de E-mail)",
        161 | 162 => "SNMP (Gerenciamento de Rede)",
        389 => "LDAP (Diretório)",
        443 => "HTTPS (Web Seguro/SSL)",
        445 => "SMB (Compartilhamento Microsoft)",
        500 => "ISAKMP (Negociação VPN)",
        587 => "SMTP Submission (E-mail)",
        636 => "LDAPS (Diretório Seguro)",
        993 => "IMAPS (E-mail Criptografado)",
        995 => "POP3S (E-mail Criptografado)",
        1433 => "MSSQL Server",
        1521 => "Oracle Database",
        3306 => "MySQL (Banco de Dados)",
        3389 => "RDP (Área de Trabalho Remota)",
        5432 => "PostgreSQL (Banco de Dados)",
        8080 => "HTTP Alt / Proxy",
        8443 => "HTTPS Alt",
        9200 => "Elasticsearch",
        27017 => "MongoDB",
        _ => "Serviço Desconhecido",
    }
}

// ─── Implementação do App ────────────────────────────────────────────────────

impl eframe::App for ArchScanApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let is_running = {
            let res = self.results.lock().unwrap();
            res.is_running
        };
        if is_running {
            ctx.request_repaint();
        }

        // Definição da Paleta de Cores
        let color_bg_main = egui::Color32::from_rgb(16, 19, 27);
        let color_bg_side = egui::Color32::from_rgb(22, 27, 39);
        let color_border = egui::Color32::from_rgb(34, 42, 59);
        let color_accent = egui::Color32::from_rgb(0, 229, 255); // Ciano
        let color_open = egui::Color32::from_rgb(46, 213, 115); // Verde
        let color_closed = egui::Color32::from_rgb(116, 125, 140); // Cinza
        let color_filtered = egui::Color32::from_rgb(255, 165, 2); // Amarelo
        let color_open_filtered = egui::Color32::from_rgb(255, 107, 129); // Coral

        // ─── Painel Lateral Esquerdo (Sidebar) ─────────────────────────────────
        egui::SidePanel::left("sidebar_panel")
            .resizable(true)
            .min_width(240.0)
            .default_width(260.0)
            .frame(
                egui::Frame::none()
                    .fill(color_bg_side)
                    .stroke(egui::Stroke::new(1.0, color_border))
                    .inner_margin(16.0),
            )
            .show(ctx, |ui| {
                // Título do App
                ui.vertical_centered(|ui| {
                    ui.add_space(10.0);
                    ui.horizontal(|ui| {
                        ui.add_space(15.0); // ajuste de alinhamento
                        ui.heading(
                            egui::RichText::new("ArchScan 🛡️")
                                .strong()
                                .color(color_accent),
                        );
                    });
                    ui.add_space(4.0);
                    ui.label(egui::RichText::new("Scanner de Rede em Rust").weak());
                    ui.add_space(20.0);
                });

                ui.separator();
                ui.add_space(15.0);

                // Formulário de Configuração
                ui.vertical(|ui| {
                    ui.label(egui::RichText::new("IP / Host do Alvo").strong());
                    ui.add_space(4.0);
                    ui.add(
                        egui::TextEdit::singleline(&mut self.ip)
                            .desired_width(ui.available_width())
                            .margin(egui::Margin::symmetric(8.0, 8.0)),
                    );
                    ui.add_space(16.0);

                    ui.label(egui::RichText::new("Faixa de Portas").strong());
                    ui.add_space(4.0);
                    ui.add(
                        egui::TextEdit::singleline(&mut self.ports)
                            .desired_width(ui.available_width())
                            .margin(egui::Margin::symmetric(8.0, 8.0)),
                    );
                    ui.add_space(16.0);

                    ui.label(egui::RichText::new("Timeout (segundos)").strong());
                    ui.add_space(4.0);
                    ui.horizontal(|ui| {
                        ui.add(egui::DragValue::new(&mut self.timeout_secs).range(1..=30));
                        ui.weak("seg");
                    });
                    ui.add_space(16.0);

                    ui.label(egui::RichText::new("Protocolo").strong());
                    ui.add_space(4.0);
                    ui.horizontal(|ui| {
                        ui.selectable_value(&mut self.protocol, Protocol::Tcp, "TCP");
                        ui.selectable_value(&mut self.protocol, Protocol::Udp, "UDP");
                    });
                });

                // Botões de Ação no Rodapé da Sidebar
                ui.with_layout(egui::Layout::bottom_up(egui::Align::Center), |ui| {
                    ui.add_space(8.0);
                    
                    ui.horizontal(|ui| {
                        // Botão Limpar
                        if ui.button(egui::RichText::new("Limpar").strong()).clicked() {
                            let mut res = self.results.lock().unwrap();
                            res.clear();
                        }

                        // Botão Scanear
                        let scan_text = if is_running { "Rodando..." } else { "Scanear" };
                        let scan_btn = ui.add_enabled(
                            !is_running,
                            egui::Button::new(
                                egui::RichText::new(scan_text)
                                    .color(egui::Color32::WHITE)
                                    .strong(),
                            )
                            .fill(color_open)
                            .min_size(egui::Vec2::new(100.0, 30.0)),
                        );

                        if scan_btn.clicked() {
                            let mut res = self.results.lock().unwrap();
                            res.clear();
                            res.is_running = true;

                            let config = ScanConfig {
                                ip: self.ip.clone(),
                                ports: self.ports.clone(),
                                protocol: self.protocol.clone(),
                                timeout_secs: self.timeout_secs,
                            };

                            start_scan(config, self.results.clone());
                        }
                    });
                    
                    ui.add_space(15.0);
                    ui.separator();
                });
            });

        // ─── Painel Central (Main Panel) ──────────────────────────────────────
        egui::CentralPanel::default()
            .frame(
                egui::Frame::none()
                    .fill(color_bg_main)
                    .inner_margin(egui::Margin::same(16.0)),
            )
            .show(ctx, |ui| {
                
                // Variáveis dos resultados compartilhados
                let (progress, scanned, total, elapsed, err_msg, open_list, closed_list, filtered_list, opfil_list) = {
                    let res = self.results.lock().unwrap();
                    (
                        res.progress,
                        res.scanned_so_far(),
                        res.total_ports,
                        res.elapsed_secs,
                        res.error.clone(),
                        res.open.clone(),
                        res.closed.clone(),
                        res.filtered.clone(),
                        res.open_filtered.clone(),
                    )
                };

                // ─── Banner de Status Superior ──────────────────────────────────
                if is_running || progress > 0.0 {
                    egui::Frame::none()
                        .fill(egui::Color32::from_rgb(23, 28, 41))
                        .rounding(8.0)
                        .stroke(egui::Stroke::new(1.0, color_border))
                        .inner_margin(12.0)
                        .show(ui, |ui| {
                            ui.horizontal(|ui| {
                                let speed = if elapsed > 0.0 { scanned as f64 / elapsed } else { 0.0 };
                                
                                ui.vertical(|ui| {
                                    ui.horizontal(|ui| {
                                        ui.colored_label(
                                            if is_running { color_accent } else { color_open },
                                            egui::RichText::new(if is_running { "⚡ Varredura em Andamento" } else { "✅ Varredura Concluída" }).strong()
                                        );
                                        ui.weak(format!("• Alvo: {}", self.ip));
                                    });
                                    ui.add_space(6.0);
                                    
                                    ui.horizontal(|ui| {
                                        let progress_bar = egui::ProgressBar::new(progress)
                                            .show_percentage()
                                            .animate(is_running)
                                            .fill(color_accent);
                                        ui.add_sized([ui.available_width() - 250.0, 18.0], progress_bar);

                                        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                                            ui.weak(format!("{}/{} • {:.1}s • {:.0} p/s", scanned, total, elapsed, speed));
                                        });
                                    });
                                });
                            });
                        });
                    ui.add_space(12.0);
                }

                if let Some(err) = err_msg {
                    egui::Frame::none()
                        .fill(egui::Color32::from_rgb(46, 20, 27))
                        .rounding(6.0)
                        .inner_margin(8.0)
                        .show(ui, |ui| {
                            ui.colored_label(egui::Color32::from_rgb(255, 107, 129), format!("⚠️ Erro: {}", err));
                        });
                    ui.add_space(10.0);
                }

                // ─── Painel de KPIs (Estatísticas Rápidas em Destaque) ─────────────
                ui.columns(4, |cols| {
                    // Card Abertas
                    render_kpi_card(&mut cols[0], "Abertas", open_list.len(), color_open, color_border);
                    // Card Filtradas
                    render_kpi_card(&mut cols[1], "Filtradas", filtered_list.len(), color_filtered, color_border);
                    // Card Ab/Filtradas
                    render_kpi_card(&mut cols[2], "Ab/Filtrada", opfil_list.len(), color_open_filtered, color_border);
                    // Card Fechadas
                    render_kpi_card(&mut cols[3], "Fechadas", closed_list.len(), color_closed, color_border);
                });

                ui.add_space(16.0);

                // ─── Seletor de Abas (Tabs) ──────────────────────────────────────
                ui.horizontal(|ui| {
                    ui.selectable_value(&mut self.active_tab, ActiveTab::Overview, "📋 Visão Geral (Portas Ativas)");
                    ui.selectable_value(&mut self.active_tab, ActiveTab::Detailed, "🔍 Detalhes do Scanner");
                    ui.selectable_value(&mut self.active_tab, ActiveTab::Help, "ℹ️ Informações & Ajuda");
                });
                ui.add_space(6.0);
                ui.separator();
                ui.add_space(10.0);

                // ─── Conteúdo das Abas ──────────────────────────────────────────
                match self.active_tab {
                    ActiveTab::Overview => {
                        // Filtra portas ativas (qualquer uma que não esteja explicitamente fechada)
                        let mut active_ports = Vec::new();
                        for p in &open_list { active_ports.push((*p, "Aberta", color_open)); }
                        for p in &filtered_list { active_ports.push((*p, "Filtrada", color_filtered)); }
                        for p in &opfil_list { active_ports.push((*p, "Aberta/Filtrada", color_open_filtered)); }
                        active_ports.sort_by_key(|k| k.0);

                        if active_ports.is_empty() {
                            ui.vertical_centered(|ui| {
                                ui.add_space(50.0);
                                ui.heading(egui::RichText::new("Nenhuma porta ativa identificada").weak());
                                ui.weak("Configure o IP alvo na barra lateral e clique em 'Scanear' para iniciar.");
                                ui.add_space(50.0);
                            });
                        } else {
                            egui::ScrollArea::vertical()
                                .id_salt("overview_scroll")
                                .max_height(240.0)
                                .show(ui, |ui| {
                                    // Cabeçalho da Tabela
                                    egui::Grid::new("overview_grid")
                                        .striped(true)
                                        .num_columns(4)
                                        .spacing([40.0, 12.0])
                                        .min_col_width(120.0)
                                        .show(ui, |ui| {
                                            ui.label(egui::RichText::new("PORTA").strong().color(egui::Color32::from_rgb(150, 160, 180)));
                                            ui.label(egui::RichText::new("ESTADO").strong().color(egui::Color32::from_rgb(150, 160, 180)));
                                            ui.label(egui::RichText::new("PROTOCOLO").strong().color(egui::Color32::from_rgb(150, 160, 180)));
                                            ui.label(egui::RichText::new("SERVIÇO ESTIMADO").strong().color(egui::Color32::from_rgb(150, 160, 180)));
                                            ui.end_row();

                                            for (port, status, color) in active_ports {
                                                ui.label(egui::RichText::new(format!("{}", port)).strong().color(color_accent));
                                                
                                                ui.horizontal(|ui| {
                                                    ui.colored_label(color, "•");
                                                    ui.colored_label(color, status);
                                                });

                                                ui.label(format!("{}", self.protocol));
                                                ui.label(get_service_name(port));
                                                ui.end_row();
                                            }
                                        });
                                });
                        }
                    }
                    ActiveTab::Detailed => {
                        // Listagem em 4 colunas roláveis
                        ui.columns(4, |cols| {
                            // Abertas
                            render_detailed_column(&mut cols[0], "Abertas", &open_list, color_open, color_border);
                            // Filtradas
                            render_detailed_column(&mut cols[1], "Filtradas", &filtered_list, color_filtered, color_border);
                            // Ab/Filtradas
                            render_detailed_column(&mut cols[2], "Ab/Filtrada", &opfil_list, color_open_filtered, color_border);
                            // Fechadas
                            render_detailed_column(&mut cols[3], "Fechadas", &closed_list, color_closed, color_border);
                        });
                    }
                    ActiveTab::Help => {
                        egui::ScrollArea::vertical().show(ui, |ui| {
                            ui.vertical(|ui| {
                                ui.heading("Sobre os Estados de Portas");
                                ui.add_space(4.0);
                                ui.label("• Abertas: O serviço está ativamente aceitando conexões na porta correspondente.");
                                ui.label("• Fechadas: A porta está acessível mas não há nenhuma aplicação ouvindo nela.");
                                ui.label("• Filtradas: Um firewall, filtro ou obstáculo na rede está impedindo que as requisições cheguem até a porta.");
                                ui.label("• Aberta/Filtrada: O scanner UDP não recebeu nenhuma resposta da porta e ela pode estar aberta ou filtrada por um firewall.");
                                
                                ui.add_space(14.0);
                                ui.heading("Notas sobre Scans UDP");
                                ui.add_space(4.0);
                                ui.label("O escaneamento UDP envia payloads estruturados específicos de serviços comumente expostos (como DNS, NTP e SNMP) para obter respostas mais precisas.");
                                ui.weak("Lembre-se de rodar com permissões adequadas em redes corporativas.");
                            });
                        });
                    }
                }

                // ─── Créditos do Rodapé ──────────────────────────────────────────
                ui.with_layout(egui::Layout::bottom_up(egui::Align::Center), |ui| {
                    ui.add_space(4.0);
                    ui.weak(egui::RichText::new("Desenvolvido por Karan Luciano & Douglas • Reescrito com egui & tokio em Rust").size(9.0));
                });
            });
    }
}

// ─── Helpers de Renderização de Componentes ───────────────────────────────────

/// KPI Card para os totais de estatísticas no topo da tela
fn render_kpi_card(ui: &mut egui::Ui, title: &str, count: usize, color: egui::Color32, border_color: egui::Color32) {
    egui::Frame::none()
        .fill(egui::Color32::from_rgb(23, 28, 41))
        .rounding(8.0)
        .stroke(egui::Stroke::new(1.0, border_color))
        .inner_margin(egui::Margin::symmetric(16.0, 12.0))
        .show(ui, |ui| {
            ui.vertical_centered(|ui| {
                ui.horizontal(|ui| {
                    ui.colored_label(color, "•");
                    ui.label(egui::RichText::new(title).weak().size(14.0));
                });
                ui.add_space(6.0);
                ui.label(
                    egui::RichText::new(format!("{}", count))
                        .strong()
                        .size(32.0)
                        .color(color),
                );
            });
        });
}

/// Coluna de logs detalhada na Aba Detailed
fn render_detailed_column(ui: &mut egui::Ui, title: &str, ports: &[u16], color: egui::Color32, border_color: egui::Color32) {
    egui::Frame::none()
        .fill(egui::Color32::from_rgb(23, 28, 41))
        .rounding(8.0)
        .stroke(egui::Stroke::new(1.0, border_color))
        .inner_margin(12.0)
        .show(ui, |ui| {
            ui.vertical(|ui| {
                ui.colored_label(color, egui::RichText::new(title).strong().size(15.0));
                ui.add_space(4.0);
                ui.separator();
                ui.add_space(4.0);

                if ports.is_empty() {
                    ui.vertical_centered(|ui| {
                        ui.add_space(30.0);
                        ui.weak("Vazia");
                        ui.add_space(30.0);
                    });
                } else {
                    egui::ScrollArea::vertical()
                        .id_salt(format!("{}_det_scroll", title))
                        .max_height(200.0)
                        .show(ui, |ui| {
                            for port in ports {
                                render_port_badge(ui, *port, color);
                            }
                        });
                }
            });
        });
}

/// Badge visual arredondado de porta individual
fn render_port_badge(ui: &mut egui::Ui, port: u16, color: egui::Color32) {
    ui.add_space(4.0);
    egui::Frame::none()
        .fill(egui::Color32::from_rgb(32, 38, 54))
        .rounding(6.0)
        .inner_margin(egui::Margin::symmetric(10.0, 6.0))
        .show(ui, |ui| {
            ui.horizontal(|ui| {
                ui.colored_label(color, "•");
                ui.label(
                    egui::RichText::new(format!("Porta {}", port))
                        .strong(),
                );
            });
        });
}
