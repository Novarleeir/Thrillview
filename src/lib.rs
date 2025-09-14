#![allow(non_snake_case)]
#![allow(non_camel_case_types)]
pub mod parse;
pub mod extractor;
pub mod app;
mod config;

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

pub enum ArchiveType {
    ZAP,
    OVL,
}

pub enum FileType{
    MODEL,
    SOUND,
    TEXTURE,
    VIDEO,
    TEXT,
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

/*pub enum EntryType{
    /*Model*/
    Model, //.mdl //mesh data
    ModelAnim, //.modelanim [animation data]

    /*Materials & Textures*/
    Sprite, //.spr [sprite for UI?]
    Texture, //.tex [textures for model]
    TextureRegion, // .texreg [???]
    GUITexturedScalable, //.txs [vector image?]
    BmpTbl, //.btbl [???]
    Flic, //.flic [animated image]

    /*Audio*/
    Audio, //.aud [audio data]
    SoundEffectDesc, //.sfx [parameters for making runtime sfx? or extra metadata for aud file?]

    /*Strings & Text*/
    Text, //.txt [strings]
    FontAuthored, //.font [font]

    /*Misc*/
    MissionDesc, //.mis [???]
    MissionPeepGroupDesc, //.mpg [???]
    SavedTrackRide, //.trh [stores coaster design? or coaster metadata?]
}*/


