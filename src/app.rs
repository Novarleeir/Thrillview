#![allow(non_snake_case)]
#![allow(non_camel_case_types)]
use log::{info, warn, error, debug, trace};
use std::{path::PathBuf, ptr};
use crate::config::{cfgLoad, cfgRead, cfgWrite};
use egui::{CentralPanel, ComboBox, Grid, MenuBar, PopupCloseBehavior, TopBottomPanel, containers::menu::{MenuButton, MenuConfig}, Context, Modal, Id};
use eframe::{App, CreationContext};
use catppuccin_egui::Theme;
use egui_file_dialog::{FileDialog, information_panel::InformationPanel};
use ini::Ini;

/******************
    Variables
******************/
struct console{
    enabled: bool,
    guiWidth: f32,
    guiHeight: f32,
}

struct fileExplorer{
    enabled: bool,
    guiWidth: f32,
    guiHeight: f32,
    fileDialog: FileDialog,
    informationPanel: InformationPanel, //details panel
    selectedFiles: Vec<PathBuf>,
}

struct about{
    enabled: bool,
}

struct settings{
    enabled: bool,
    isSaved: bool,
    isSavedEnabled: bool,
    configBackup: Ini,
}

pub struct error{
    enabled: bool,
    message: String,
}

struct modal{
    about: about,
    settings: settings,
    error: error,
}

struct guiChildVars{
    console: console,
    fileExplorer: fileExplorer,
    modal: modal,
}

pub struct gui{
    config: Ini,
    theme: Theme,
    themeLabel: String,
    guiChild: guiChildVars,
}

/// Called before the first frame is rendered
impl gui{
    pub fn new(cc: &CreationContext<'_>, config: Ini) -> Self{
        cc.egui_ctx.set_zoom_factor(cfgRead(&config, "gui", "uiScale").parse().unwrap());
        Self{
            config: config.clone(),
            theme: getTheme(),
            themeLabel: getThemeStr(getTheme()),
            guiChild: guiChildVars{
                console: console{
                    enabled: cfgRead(&config, "gui", "consoleEnabled").parse().unwrap(),
                    guiWidth: cfgRead(&config, "gui", "consoleWidth").parse().unwrap(),
                    guiHeight: cfgRead(&config, "gui", "consoleHeight").parse().unwrap(),
                },
                fileExplorer: fileExplorer{
                    enabled: false,
                    guiWidth: 1530.0,
                    guiHeight: 830.0,
                    fileDialog: FileDialog::default()
                        .as_modal(false)
                        .show_hidden_option(true)
                        .show_pinned_folders(true)
                        .default_file_filter("Thrillville archive")
                        .add_file_filter_extensions("Thrillville archive", vec!["zap", "ovl"]),
                    informationPanel: InformationPanel::default()
                        //todo: Is this actually not needed?
                        .add_file_preview("csv", |ui, item|{
                            if let Some(mut content) = item.content(){
                                egui::ScrollArea::vertical()
                                    .max_height(ui.available_height())
                                    .show(ui, |ui|{
                                        ui.add(egui::TextEdit::multiline(&mut content).code_editor());
                                    });
                            }
                        }),
                    selectedFiles: vec![],
                },
                modal: modal{
                    about: about{
                        enabled: false,
                    },
                    settings: settings{
                        enabled: false,
                        isSaved: true,
                        isSavedEnabled: false,
                        configBackup: Default::default(),
                    },
                    error: error{
                        enabled: false,
                        message: "NULL".to_string(),
                    },
                },
            },
        }
    }
}

/******************
    Update GUI
******************/
impl App for gui{
    /// Called each time the UI needs repainting
    fn update(&mut self, ctx: &Context, _frame: &mut eframe::Frame){
        //put your widgets into a `SidePanel`, `TopBottomPanel`, `CentralPanel`, `Window` or `Area`. https://emilk.github.io/egui
        catppuccin_egui::set_theme(ctx, self.theme);

        //- Display GUI
        guiToolbar(&ctx, self);
        guiCentralPanel(&ctx, &mut self.guiChild);
        if(self.guiChild.console.enabled){ guiConsole(&ctx, &mut self.guiChild.console); }
        if(self.guiChild.fileExplorer.enabled){ guiFileExplorer(&ctx, &mut self.guiChild.fileExplorer, &self.config); }
        if(self.guiChild.modal.about.enabled){ guiAboutModal(&ctx, &mut self.guiChild.modal); }
        if(self.guiChild.modal.settings.enabled){ guiSettingsModal(&ctx, self); }
        if(self.guiChild.modal.error.enabled){ guiErrorModal(&ctx, &mut self.guiChild.modal); }
    }
}

/******************
    GUIS
******************/
fn guiToolbar(ctx: &Context, gui: &mut gui){
    TopBottomPanel::top("toolbar")
        .resizable(false)
        .show(ctx, |ui|{
            MenuBar::new().ui(ui, |ui|{
                MenuButton::new("Menu").config(MenuConfig::default().close_behavior(PopupCloseBehavior::CloseOnClickOutside)).ui(ui, |ui|{
                    if ui.button("Browse...").clicked(){

                    }
                    ui.separator();
                    if ui.button("Open Console").clicked(){
                        gui.guiChild.console.enabled = !gui.guiChild.console.enabled;
                        ui.close();
                    }
                    if ui.button("Settings...").clicked(){
                        gui.guiChild.modal.settings.configBackup = gui.config.clone();
                        gui.guiChild.modal.settings.enabled = !gui.guiChild.modal.settings.enabled;
                        ui.close();
                    }
                    if ui.button("About...").clicked(){
                        gui.guiChild.modal.about.enabled = !gui.guiChild.modal.about.enabled;
                        ui.close();
                    }
                    ui.separator();
                    if ui.button("Quit").clicked(){ ctx.send_viewport_cmd(egui::ViewportCommand::Close); }
                });
            });
        });
}

fn guiCentralPanel(ctx: &Context, guiChild: &mut guiChildVars){
    CentralPanel::default().show(ctx, |ui|{
        //todo: will hold the zap FS directory view
    });
}

fn guiConsole(ctx: &Context, console: &mut console){
    ctx.show_viewport_immediate(
        egui::ViewportId::from_hash_of("guiConsole"),
        egui::ViewportBuilder::default()
            .with_title("Console Log")
            .with_inner_size([console.guiWidth, console.guiHeight])
            .with_min_inner_size([1000.0, 166.0]),
        |ctx, _class|{
            CentralPanel::default().show(ctx, |ui|{ egui_logger::logger_ui().show(ui); });
            if ctx.input(|i| i.viewport().close_requested()){ console.enabled = false; }
        },
    );
}

fn guiFileExplorer(ctx: &Context, fileExplorer: &mut fileExplorer, config: &Ini){
    ctx.show_viewport_immediate(
        egui::ViewportId::from_hash_of("guiFileExplorer"),
        egui::ViewportBuilder::default()
            .with_title("File Explorer")
            .with_inner_size([
                fileExplorer.guiWidth,
                fileExplorer.guiHeight
            ])
            .with_min_inner_size([
                1000.0,
                166.0
            ]),
        |ctx, _class|{
            CentralPanel::default().show(ctx, |ui|{ egui_logger::logger_ui().show(ui); });
            if ctx.input(|i| i.viewport().close_requested()){ fileExplorer.enabled = false; }
        },
    );
}

fn guiAboutModal(ctx: &Context, modal: &mut modal){
    let modal = Modal::new(Id::new("About"))
        .show(ctx, |ui|{
            ui.set_width(200.0);
            ui.heading("About Thrillview");

            ui.add_space(32.0);
            if ui.button("Close").clicked(){
                modal.about.enabled = false;
                ui.close();
            }
        });
}

//todo: make it so that the modal can be triggered from out-of-scope
//note: investigate using the egui modal crate
fn guiErrorModal(ctx: &Context, modal: &mut modal){
    let modal = Modal::new(Id::new("Error"))
        .show(ctx, |ui|{
            ui.set_width(200.0);
            ui.heading("Error:");

            ui.add_space(32.0);
            if ui.button("Close").clicked(){
                modal.error.enabled = false;
                ui.close();
            }
        });
}

//todo: add a save button that saves the settings to config.ini and clicking close reverts everything back to
fn guiSettingsModal(ctx: &Context, gui: &mut gui){
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
                        .selected_text(gui.themeLabel.clone())
                        .show_ui(ui, |ui|{
                            if ui.selectable_label(ptr::eq(&gui.theme, &catppuccin_egui::FRAPPE), "Frappe").clicked(){
                                gui.theme = catppuccin_egui::FRAPPE;
                                gui.themeLabel = "Frappe".to_string();
                                gui.guiChild.modal.settings.isSaved = false;
                            }
                            if ui.selectable_label(ptr::eq(&gui.theme, &catppuccin_egui::MACCHIATO), "Macchiato").clicked(){
                                gui.theme = catppuccin_egui::MACCHIATO;
                                gui.themeLabel = "Macchiato".to_string();
                                gui.guiChild.modal.settings.isSaved = false;
                            }
                            if ui.selectable_label(ptr::eq(&gui.theme, &catppuccin_egui::MOCHA), "Mocha").clicked(){
                                gui.theme = catppuccin_egui::MOCHA;
                                gui.themeLabel = "Mocha".to_string();
                                gui.guiChild.modal.settings.isSaved = false;
                            }
                            if ui.selectable_label(ptr::eq(&gui.theme, &catppuccin_egui::LATTE), "Latte").clicked(){
                                gui.theme = catppuccin_egui::LATTE;
                                gui.themeLabel = "Latte".to_string();
                                gui.guiChild.modal.settings.isSaved = false;
                            }
                        });
                    ui.end_row();

                    ui.label("Enable Console");
                    //todo: detect a click
                    ui.checkbox(&mut gui.guiChild.console.enabled, "");
                    ui.end_row();

                    ui.vertical_centered(|ui|{
                        ui.set_width(100.0);
                        ui.horizontal(|ui|{
                            if(!gui.guiChild.modal.settings.isSaved){
                                if (ui.button("Save")).clicked(){
                                    saveSettings(gui);
                                    ui.close();
                                }
                            }else{
                                ui.add_enabled_ui(false, |ui|{
                                    if (ui.button("Save")).clicked(){}
                                });
                            }
                            if(ui.button("Close")).clicked(){
                                if(gui.guiChild.modal.settings.isSaved){
                                    gui.guiChild.modal.settings.enabled = false;
                                    ui.close();
                                }else{
                                    //keeps the save warning active across frames
                                    gui.guiChild.modal.settings.isSavedEnabled = true;
                                }
                            }
                        });
                });

                if(gui.guiChild.modal.settings.isSavedEnabled){
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
                                        saveSettings(gui);
                                        gui.guiChild.modal.settings.isSavedEnabled = false;
                                        gui.guiChild.modal.settings.enabled = false;
                                        ui.close();
                                    }
                                    if ui.button("Close without Saving").clicked(){
                                        gui.config = gui.guiChild.modal.settings.configBackup.clone();
                                        gui.guiChild.modal.settings.isSavedEnabled = false;
                                        gui.guiChild.modal.settings.isSaved = true;
                                        gui.guiChild.modal.settings.enabled = false;
                                        ui.close();
                                    }
                                    if ui.button("Cancel").clicked(){
                                        gui.guiChild.modal.settings.isSavedEnabled = false;
                                        ui.close();
                                    }
                                })
                            });
                        });
                }
            });
        });
}

/******************
    Misc Utility
******************/

///Save Settings to config
fn saveSettings(gui: &mut gui){
    let settings: Vec<(&str, &str, String)> = vec![
        ("gui", "theme", getThemeStr(gui.theme)),
        ("gui", "consoleEnabled", gui.guiChild.console.enabled.to_string()),
    ];

    for(section, key, value) in settings{
        cfgWrite(&mut gui.config, section, key, value.parse().unwrap());
    }
    gui.guiChild.modal.settings.isSaved = true;
}

fn getTheme() -> Theme{
    match cfgRead(&cfgLoad(), "gui", "theme").to_lowercase().as_str(){
        "frappe" => { catppuccin_egui::FRAPPE }
        "macchiato" => { catppuccin_egui::MACCHIATO }
        "mocha" => { catppuccin_egui::MOCHA }
        "latte" => { catppuccin_egui::LATTE }
        _ => {
            error!("Invalid theme in config.ini! Defaulting to frappe");
            catppuccin_egui::FRAPPE
        }
    }
}

fn getThemeStr(theme: Theme) -> String{
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