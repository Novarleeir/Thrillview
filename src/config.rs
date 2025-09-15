#![allow(non_snake_case)]
#![allow(non_camel_case_types)]
extern crate ini;
use log::{info, warn, error, debug, trace};
use std::{fs::{OpenOptions}, io::{ErrorKind}};
use ini::Ini;

/// Check to see if the ini file exists on disk
fn cfgExists(){
    match OpenOptions::new().read(true).open("../config.ini"){
        Ok(file) => {
            drop(file);
        },
        Err(e) => {
            if e.kind() == ErrorKind::NotFound{
                error!("config.ini not found! Creating new ini using the default template...");
                cfgCreate();
                error!("Saved default config.ini to disk!");
            }else{
                error!("Failed to open config.ini file! Reason: {:?}", e);
            }
        }
    }
}

/// Edit value of loaded config in memory
pub fn cfgWrite(config: &mut Ini, section: &str, key: &str, value: String){
    config.with_section(Some(section))
        .set(key, value);
    cfgSave(&config);
}

/// Fetch value from loaded config in memory
pub fn cfgRead(config: &Ini, section: &str, key: &str) -> String{
    //todo: add check to see if section/key actually exists
    let section = config.section(Some(section)).unwrap();
    return section.get(key).unwrap().to_string();
}

/// Load config into memory from disk
pub fn cfgLoad() -> Ini{
    cfgExists();
    return Ini::load_from_file("../config.ini").unwrap();
}

/// Save config to disk
pub fn cfgSave(config: &Ini){
    cfgExists();
    config.write_to_file("../config.ini").unwrap();
}

/// Create new config ini on disk
fn cfgCreate(){
    let mut config = Ini::new();
    config.with_section(None::<String>)
        .set("encoding", "utf-8");
    config.with_section(Some("thrillview"))
        .set("version", env!("CARGO_PKG_VERSION"))
        .set("debugLevel", "debug");
    config.with_section(Some("gui"))
        .set("theme", "Frappe")
        .set("uiScale", "1.2")
        .set("rootWidth", "1000")
        .set("rootHeight", "650")
        .set("rootWidthMin", "960")
        .set("rootHeightMin", "450")
        .set("consoleEnabled", "true")
        .set("consoleWidth", "1100")
        .set("consoleHeight", "500")
        .set("consoleWidthMin", "1000")
        .set("consoleHeightMin", "166");
    config.write_to_file("../config.ini").unwrap();
}