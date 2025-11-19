use log::{info, warn, error, debug, trace};
use std::path::PathBuf;
use egui::{CentralPanel, Context};
use egui_file_dialog::{FileDialog, information_panel::InformationPanel};
use crate::gui;

pub(crate) struct fileExplorer{
    pub(crate) enabled: bool,
    pub(crate) guiWidth: f32,
    pub(crate) guiHeight: f32,
    pub(crate) fileDialog: FileDialog,
    pub(crate) informationPanel: InformationPanel, //details panel
    pub(crate) selectedFiles: Vec<PathBuf>,
}

impl gui{
    pub(crate) fn guiFileExplorer(&mut self, ctx: &Context){
        ctx.show_viewport_immediate(
            egui::ViewportId::from_hash_of("guiFileExplorer"),
            egui::ViewportBuilder::default()
                .with_title("File Explorer")
                .with_inner_size([
                    self.fileExplorer.guiWidth,
                    self.fileExplorer.guiHeight
                ])
                .with_min_inner_size([
                    1000.0,
                    166.0
                ]),
            |ctx, _class|{
                CentralPanel::default().show(ctx, |ui|{ egui_logger::logger_ui().show(ui); });
                if ctx.input(|i| i.viewport().close_requested()){ self.fileExplorer.enabled = false; }
            },
        );
    }
}
    