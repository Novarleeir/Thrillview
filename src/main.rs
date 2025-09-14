#![allow(non_snake_case)]
#![allow(non_camel_case_types)]
use log::{info, warn, error, debug, trace, LevelFilter};
use crate::config::{cfgLoad, cfgRead};
use ini::Ini;
mod app;
mod parse;
mod config;
mod extractor;

fn main(){
    let config = cfgLoad();
    egui_logger::builder().max_level(LevelFilter::Debug).init().unwrap();
    thrillview(&config).expect("Unable to call thrillview()!");
}

fn thrillview(config: &Ini) -> eframe::Result{
    warn!("Starting up Thrillview...");
    warn!("Thrillview version: {}", env!("CARGO_PKG_VERSION"));
    let viewportOptions = eframe::NativeOptions{
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([
                cfgRead(&config, "gui", "rootWidth").parse().unwrap(),
                cfgRead(&config, "gui", "rootHeight").parse().unwrap(),
            ])
            .with_min_inner_size([
                cfgRead(&config, "gui", "rootWidthMin").parse().unwrap(),
                cfgRead(&config, "gui", "rootHeightMin").parse().unwrap()
            ])
            //.with_icon( eframe::icon_data::from_png_bytes(&include_bytes!("gui/assets/icon.png")[..]).unwrap() )
        ,
        ..Default::default()
    };

    eframe::run_native(
        &("Thrillview ".to_owned() + env!("CARGO_PKG_VERSION")),
        viewportOptions,
        Box::new(|ctx|{
            Ok(Box::new(thrillview::gui::new(ctx, config.clone())))}),
    )
}


/*todo list:
   - Make the settings actually revert for things like the uiScale/theme when close without saving is selected. Likely just make a revertSettings function that sets all the values
   - Parser
   - Encoder
   - The isSaved flag doesn't get cleared for some reason
   - file explorer to open files
   - file list
*/