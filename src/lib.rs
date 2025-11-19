#![allow(non_snake_case)]
#![allow(non_camel_case_types)]
mod app;
mod config;
mod parse;
use std::fs::File;
use std::path::PathBuf;
pub use app::gui;

//todo: will likely have to change
pub enum EntryState{
    Parsed, //all data related to the entry (name, extension, offset, size, etc.) has been found
    Queued, //entry is queued for extraction
    Extracting, //entry extraction has started
    Extracted, //for when the entry is successfully extracted from the ovl and stored in memory
    Saving, //started saving entry to disk
    Done, //saved entry file to disk successfully
    Error, //if any step has failed
}

pub struct Archive{
    path: Option<PathBuf>,
    file: Option<File>,
    data: Option<ArchiveData>,
}

pub enum ArchiveData{
    ZAP(ZAP),
    OVL(OVL),
}

pub struct ZAP{
    numOVL: u32,
    OVLs: Vec<Archive>,
}

pub struct OVL{
    version: Option<OVLVersion>, //temporary optional
    name: Option<String>,
    isCommon: bool,
    data: OVLData,
}

pub enum OVLVersion{
    v1,
    v2,
    v3,
    v4,
    v5,
    v6,
    v7,
}

/// Stores file data extracted from the OVL
struct OVLData{
    pub bytes: Vec<u8>,
}

pub enum EntryType{
    MODEL, //mdl
    SOUND, //sfx, music
    IMAGE, //textures, gui elements, etc.
    VIDEO, //fmvs (might not be needed)
    TEXT, //string + localization files, lua
}
