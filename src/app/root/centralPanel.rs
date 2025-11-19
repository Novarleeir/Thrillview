#![allow(non_snake_case)]
#![allow(non_camel_case_types)]
use log::{info, warn, error, debug, trace};
use crate::gui;
use egui::{CentralPanel, Context};

impl gui{
    pub(crate) fn guiCentralPanel(&mut self, ctx: &Context){
        CentralPanel::default().show(ctx, |ui|{
            //todo: will hold the zap FS directory view
        });
    }
}