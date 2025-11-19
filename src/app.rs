#![allow(non_snake_case)]
#![allow(non_camel_case_types)]

mod viewer;
mod root;
mod misc;
use crate::app::root::about::about;
use crate::app::misc::fileExplorer::fileExplorer;
use crate::app::misc::console::console;
use crate::app::misc::dialogue::dialogue;
use crate::app::root::settings::{getThemeStr, settings};
use log::{info, warn, error, debug, trace};
use crate::config::{cfgLoad, cfgRead, cfgWrite};
use ini::Ini;
use egui::{Context};
use eframe::{App};
use egui_file_dialog::{FileDialog, information_panel::InformationPanel};
use catppuccin_egui::Theme;

/******************
    Variables
******************/

pub struct gui{
    config: Ini,
    theme: Theme,
    console: console,
    fileExplorer: fileExplorer,
    about: about,
    settings: settings,
    dialogue: dialogue,
}

/// Called before the first frame is rendered
impl gui{
    pub fn new(ctx: &Context, config: Ini) -> Self{
        ctx.set_zoom_factor(cfgRead(&config, "gui", "uiScale").parse().unwrap());
        Self{
            config: config.clone(),
            theme: getTheme(),
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
            about: about{
                enabled: false,
            },
            settings: settings{
                enabled: false,
                isSaved: true,
                isSavedEnabled: false,
                configBackup: Default::default(),
                themeLabel: getThemeStr(getTheme()),
            },
            dialogue: dialogue{
                enabled: false,
                message: "".to_string(),
            }
        }
    }
}

/******************
    Update GUI
******************/
impl App for gui{
    /// Called each time the UI needs repainting
    fn update(&mut self, ctx: &Context, _frame: &mut eframe::Frame){
        //https://emilk.github.io/egui
        catppuccin_egui::set_theme(ctx, self.theme);

        //- Display GUI
        self.guiToolbar(&ctx);
        self.guiCentralPanel(&ctx);
        if(self.console.enabled){ self.guiConsole(&ctx); }
        if(self.fileExplorer.enabled){ self.guiFileExplorer(&ctx); }
        if(self.about.enabled){ self.guiAboutModal(&ctx); }
        if(self.settings.enabled){ self.guiSettingsModal(&ctx); }
    }
}

/******************
    Misc Utility
******************/

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