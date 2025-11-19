#![allow(non_snake_case)]
#![allow(non_camel_case_types)]
use log::{info, warn, error, debug, trace};
use std::ptr;
use catppuccin_egui::Theme;
use egui::{ComboBox, Context, Grid, Id, Modal};
use crate::gui;
use ini::Ini;
use crate::config::cfgWrite;

pub(crate) struct settings{
    pub(crate) enabled: bool,
    pub(crate) isSaved: bool,
    pub(crate) isSavedEnabled: bool,
    pub(crate) configBackup: Ini,
    pub(crate) themeLabel: String,
}

impl gui{
    pub(crate) fn guiSettingsModal(&mut self, ctx: &Context){
        let modal = Modal::new(Id::new("Settings"))
            .show(ctx, |ui|{
                ui.set_width(500.0);
                ui.heading("Settings");
                ui.separator();

                Grid::new("settingsGrid")
                    .num_columns(2)
                    .spacing([40.0, 4.0])
                    .striped(false)
                    .show(ui, |ui|{
                        ui.label("Theme");
                        ComboBox::from_label("")
                            .selected_text(self.settings.themeLabel.clone())
                            .show_ui(ui, |ui|{
                                if ui.selectable_label(ptr::eq(&self.theme, &catppuccin_egui::FRAPPE), "Frappe").clicked(){
                                    self.theme = catppuccin_egui::FRAPPE;
                                    self.settings.themeLabel = "Frappe".to_string();
                                    self.settings.isSaved = false;
                                }
                                if ui.selectable_label(ptr::eq(&self.theme, &catppuccin_egui::MACCHIATO), "Macchiato").clicked(){
                                    self.theme = catppuccin_egui::MACCHIATO;
                                    self.settings.themeLabel = "Macchiato".to_string();
                                    self.settings.isSaved = false;
                                }
                                if ui.selectable_label(ptr::eq(&self.theme, &catppuccin_egui::MOCHA), "Mocha").clicked(){
                                    self.theme = catppuccin_egui::MOCHA;
                                    self.settings.themeLabel = "Mocha".to_string();
                                    self.settings.isSaved = false;
                                }
                                if ui.selectable_label(ptr::eq(&self.theme, &catppuccin_egui::LATTE), "Latte").clicked(){
                                    self.theme = catppuccin_egui::LATTE;
                                    self.settings.themeLabel = "Latte".to_string();
                                    self.settings.isSaved = false;
                                }
                            });
                        ui.end_row();

                        ui.label("Enable Console");
                        //todo: detect a click
                        ui.checkbox(&mut self.console.enabled, "");
                        ui.end_row();

                        ui.vertical_centered(|ui|{
                            ui.set_width(100.0);
                            ui.horizontal(|ui|{
                                if(!self.settings.isSaved){
                                    if (ui.button("Save")).clicked(){
                                        saveSettings(self);
                                        ui.close();
                                    }
                                }else{
                                    ui.add_enabled_ui(false, |ui|{
                                        if (ui.button("Save")).clicked(){}
                                    });
                                }
                                if(ui.button("Close")).clicked(){
                                    if(self.settings.isSaved){
                                        self.settings.enabled = false;
                                        ui.close();
                                    }else{
                                        //keeps the save warning active across frames
                                        self.settings.isSavedEnabled = true;
                                    }
                                }
                            });
                        });

                        if(self.settings.isSavedEnabled){
                            let modalSub = Modal::new(Id::new("UnsavedWarning"))
                                .show(ctx, |ui|{
                                    ui.set_width(500.0);
                                    ui.heading("Warning!");
                                    ui.separator();
                                    ui.label("You have unsaved changes!");
                                    ui.add_space(32.0);
                                    ui.vertical_centered(|ui| {
                                        ui.set_width(100.0);
                                        ui.horizontal(|ui| {
                                            if ui.button("Save and Close").clicked(){
                                                saveSettings(self);
                                                self.settings.isSavedEnabled = false;
                                                self.settings.enabled = false;
                                                ui.close();
                                            }
                                            if ui.button("Close without Saving").clicked(){
                                                self.config = self.settings.configBackup.clone();
                                                self.settings.isSavedEnabled = false;
                                                self.settings.isSaved = true;
                                                self.settings.enabled = false;
                                                ui.close();
                                            }
                                            if ui.button("Cancel").clicked(){
                                                self.settings.isSavedEnabled = false;
                                                ui.close();
                                            }
                                        })
                                    });
                                });
                        }
                    });
            });
    }
}

pub(crate) fn saveSettings(gui: &mut gui){
    let settings: Vec<(&str, &str, String)> = vec![
        ("gui", "theme", getThemeStr(gui.theme)),
        ("gui", "consoleEnabled", gui.console.enabled.to_string()),
    ];

    for(section, key, value) in settings{
        cfgWrite(&mut gui.config, section, key, value.parse().unwrap());
    }
    gui.settings.isSaved = true;
}

pub(crate) fn getThemeStr(theme: Theme) -> String{
    match theme{
        catppuccin_egui::FRAPPE => "Frappe".to_string(),
        catppuccin_egui::MACCHIATO => "Macchiato".to_string(),
        catppuccin_egui::MOCHA => "Mocha".to_string(),
        catppuccin_egui::LATTE => "Latte".to_string(),
        _ => {
            "Frappe".to_string()
        }
    }
}