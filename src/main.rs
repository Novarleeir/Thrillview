#![allow(non_snake_case)]
#![allow(non_camel_case_types)]
use log::{info, warn, error, debug, trace, LevelFilter};
use crate::config::{cfgLoad, cfgRead};
use ini::Ini;
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

/*todo master list:
   - Make the settings actually revert for things like the uiScale/theme when close without saving is selected. Likely just make a revertSettings function that sets all the values
   - Parser for zaps and ovls
   - Way to extract individual files, or the entire contents of a zap/ovl
   - The isSaved flag doesn't get cleared for some reason
   - File explorer to open files
   - Zap file system view
   - Dedicated viewers for previewing things like 3D models and textures (should be created on a new thread for performance)
*/

/*todo app:
   - app.rs | main gui handler
   - appDirectory.rs | for viewing the zap file system and files within the ovls. https://crates.io/crates/egui_ltreeview
*/

/*todo viewer (for viewing files):
   - viewer.rs | root viewport that calls the correct viewer function for the specific file type. Should be made to be instantiated so that multiple preview windows may be opened at once
   - viewerModel.rs | viewing 3D models. Use this?: https://crates.io/crates/egui-gizmo
   - viewerSfx.rs | listen to sound files and music. View waveform? Volume controls?
   - viewerImage.rs | viewing textures and other various 2D elements
   - viewerVideo.rs | https://crates.io/crates/door_player
*/

/*Other crates that could prove to be helpful:
https://crates.io/crates/egui_transition_animation
https://crates.io/crates/egui-modal-spinner
https://crates.io/crates/egui_dock
https://crates.io/crates/egui_thumbhash
https://crates.io/crates/irox-egui-extras (for extractor ui/modal)
https://crates.io/crates/egui_listview
https://github.com/saturn77/egui_mobius_template
*/