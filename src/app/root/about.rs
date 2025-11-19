use log::{info, warn, error, debug, trace};
use egui::{Context, Id, Modal};
use crate::gui;

pub(crate) struct about{
    pub(crate) enabled: bool,
}

impl gui{
    pub(crate) fn guiAboutModal(&mut self, ctx: &Context){
        let modal = Modal::new(Id::new("About"))
            .show(ctx, |ui|{
                ui.set_width(200.0);
                ui.heading("About Thrillview");

                ui.add_space(32.0);
                if ui.button("Close").clicked(){
                    self.about.enabled = false;
                    ui.close();
                }
            });
    }
}