#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // ocultar console no Windows em release

pub mod models;
pub mod scanner;
pub mod app;

use eframe::egui;
use app::ArchScanApp;

fn main() -> eframe::Result {
    // Configurações da janela principal
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([1000.0, 700.0])
            .with_resizable(true),
        ..Default::default()
    };

    eframe::run_native(
        "ArchScan - Scanner de Portas TCP/UDP",
        options,
        Box::new(|cc| Ok(Box::new(ArchScanApp::new(cc)))),
    )
}
