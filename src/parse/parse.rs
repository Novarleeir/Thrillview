#![allow(non_snake_case)]
#![allow(non_camel_case_types)]
use log::{info, warn, error, debug, trace};
use std::{fs::{OpenOptions, File}, path::{PathBuf}, io::{self, Read}};

/// Does the file at the provided path exist?
fn parseExists(path: PathBuf) -> Result<(), String>{
    match OpenOptions::new().read(true).open(path){
        Ok(..) => { Ok(()) },
        Err(e) => { Err(format!("Failed to open file! Reason: {}", e)) },
    }
}

fn parseLoad(path: PathBuf){
    //todo: check exists

}

fn parseEndian(){
    
}
