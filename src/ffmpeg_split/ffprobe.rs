use std::collections::HashMap;
use std::path::Path;
use std::process::Command;
use std::str;

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

#[derive(Serialize, Deserialize, Debug)]
pub struct FFProbeOutput {
    pub chapters: Vec<Chapter>,
}

pub fn read_chapters(path: &Path) -> serde_json::Result<FFProbeOutput> {
    let cmd = Command::new("ffprobe")
        .arg("-i")
        .arg(path)
        .args(["-v", "error", "-print_format", "json", "-show_chapters"])
        .output()
        .expect("omg failed");

    let s = match str::from_utf8(&cmd.stdout) {
        Ok(v) => v,
        Err(e) => panic!("Invalid UTF-8 sequence: {}", e),
    };

    return serde_json::from_str(&s);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn read_chapters_expect_some() {
        let parent = Path::new(file!()).parent().unwrap();
        let res = read_chapters(&parent.join("testdata/beep.m4a")).unwrap();
        assert_eq!(res.chapters.len(), 3);
        assert_eq!(res.chapters[0].id, 0);
        assert_eq!(
            res.chapters[0].tags["title"],
            "It All Started With a Simple BEEP"
        );
        assert_eq!(res.chapters[1].id, 1);
        assert_eq!(res.chapters[1].tags["title"], "All You Can BEEP Buffee");
        assert_eq!(res.chapters[2].id, 2);
        assert_eq!(res.chapters[2].tags["title"], "The Final Beep");
    }

    #[test]
    fn read_chapters_expect_none() {
        let parent = Path::new(file!()).parent().unwrap();
        let res = read_chapters(&parent.join("testdata/beep-nochap.m4a")).unwrap();
        assert_eq!(res.chapters.len(), 0);
    }
}
