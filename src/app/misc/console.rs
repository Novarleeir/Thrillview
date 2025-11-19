#![allow(non_snake_case)]
#![allow(non_camel_case_types)]
use log::{info, warn, error, debug, trace};
use egui::{CentralPanel, Context};
use crate::gui;

pub(crate) struct console{
    pub(crate) enabled: bool,
    pub(crate) guiWidth: f32,
    pub(crate) guiHeight: f32,
}

impl gui{
    pub(crate) fn guiConsole(&mut self, ctx: &Context){
        ctx.show_viewport_immediate(
            egui::ViewportId::from_hash_of("guiConsole"),
            egui::ViewportBuilder::default()
                .with_title("Console Log")
                .with_inner_size([self.console.guiWidth, self.console.guiHeight])
                .with_min_inner_size([1000.0, 166.0]),
            |ctx, _class|{
                CentralPanel::default().show(ctx, |ui|{ egui_logger::logger_ui().show(ui); });
                if ctx.input(|i| i.viewport().close_requested()){ self.console.enabled = false; }
            },
        );
    }
}