use std::collections::HashMap;
use std::ffi::OsStr;
use std::path::{Path, PathBuf};

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Chapter {
    pub id: i32,
    pub time_base: String,
    pub start: i32,
    pub start_time: String,
    pub end: i32,
    pub end_time: String,
    pub tags: HashMap<String, String>,
}

impl Chapter {
    pub fn title(&self) -> Option<&OsStr> {
        if let Some(v) = self.tags.get("title") {
            if v == "" {
                return None;
            }
            return Some(&OsStr::new(v));
        } else {
            return None;
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct FFProbeOutput {
    pub chapters: Vec<Chapter>,
    //pub max_chapter_num: i32,
}

pub struct Opts<'a> {
    pub use_title_as_name: bool,
    pub use_title_in_meta: bool,
    pub max_chapter_num: usize,
    pub output_extension: Option<&'a OsStr>,
}

impl Opts<'_> {
    pub fn new(max_chapter_num: usize) -> Self {
        return Opts {
            use_title_as_name: true,
            use_title_in_meta: true,
            max_chapter_num,
            output_extension: None,
        };
    }

    pub fn set_use_title_as_name(&mut self, val: bool) -> &Self {
        self.use_title_as_name = val;
        return self;
    }
    pub fn set_use_title_in_meta(&mut self, val: bool) -> &Self {
        self.use_title_in_meta = val;
        return self;
    }
    pub fn chapter_id_formatted_width(&mut self) -> usize {
        return (self.max_chapter_num as f32).log10().ceil() as usize;
    }
}

#[derive(Debug)]
pub struct WorkItem<'a> {
    pub input_file: &'a Path,
    pub output_file: PathBuf,
    pub chapter: &'a Chapter,
}
