#![allow(non_snake_case)]
#![allow(non_camel_case_types)]
mod extractor;
mod app;
mod config;
mod parse;
mod parseOVL;
mod parseZAP;
mod viewer;
mod viewerModel;
mod viewerSfx;
mod viewerImage;
mod viewerText;
mod viewerVideo;
mod appDirectory;
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

pub enum ArchiveType{
    ZAP,
    OVL,
}

pub enum FileType{
    MODEL, //mdl
    SOUND, //sfx, music
    IMAGE, //textures, gui elements, etc.
    VIDEO, //fmvs (might not be needed)
    TEXT, //string + localization files, lua
}

pub struct File{
    //path?
    //name
    //size
    //format
}

pub struct Zap{
    path: PathBuf,
}

pub struct Ovl{
    path: PathBuf,
}
