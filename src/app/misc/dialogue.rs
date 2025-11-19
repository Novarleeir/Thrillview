#![allow(non_snake_case)]
#![allow(non_camel_case_types)]
use log::{info, warn, error, debug, trace};
use egui::Context;
use crate::gui;
use egui_modal::{DialogBuilder, Icon, Modal as eguiModal};

pub struct dialogue{
    pub enabled: bool,
    pub message: String,
}

impl gui{
    pub fn guiDialogue(&mut self, ctx: &Context, message: &str) -> eguiModal{
        let modal = eguiModal::new(ctx, "errorModal");
        return modal;
    }
}