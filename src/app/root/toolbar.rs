#![allow(non_snake_case)]
#![allow(non_camel_case_types)]
use log::{info, warn, error, debug, trace};
use crate::gui;
use egui::{CentralPanel, Context, MenuBar, PopupCloseBehavior, TopBottomPanel};
use egui::containers::menu::{MenuButton, MenuConfig};
use egui_modal::Icon;
use crate::parse::parseZAP::zapDO;

impl gui{
    pub(crate) fn guiToolbar(&mut self, ctx: &Context){
        TopBottomPanel::top("toolbar")
            .resizable(false)
            .show(ctx, |ui|{
                MenuBar::new().ui(ui, |ui|{
                    MenuButton::new("Menu").config(MenuConfig::default().close_behavior(PopupCloseBehavior::CloseOnClickOutside)).ui(ui, |ui|{
                        if ui.button("Browse...").clicked(){
                            ui.close();
                        }
                        if ui.button("Error Test!").clicked(){
                            ui.close();
                        }
                        ui.separator();
                        if ui.button("Open Console").clicked(){
                            self.console.enabled = !self.console.enabled;
                            ui.close();
                        }
                        if ui.button("Settings...").clicked(){
                            self.settings.configBackup = self.config.clone();
                            self.settings.enabled = !self.settings.enabled;
                            ui.close();
                        }
                        if ui.button("About...").clicked(){
                            self.about.enabled = !self.about.enabled;
                            ui.close();
                        }
                        if ui.button("ZAP...").clicked(){
                            zapDO();
                            ui.close();
                        }
                        ui.separator();
                        if ui.button("Quit").clicked(){ ctx.send_viewport_cmd(egui::ViewportCommand::Close); }
                    });
                });
            });
    }
}