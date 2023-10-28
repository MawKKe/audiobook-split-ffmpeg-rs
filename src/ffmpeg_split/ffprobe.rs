use std::ffi::OsStr;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::str;

use crate::ffmpeg_split::model::{Chapter, FFProbeOutput, Opts, WorkItem};

pub fn make_workitem<'a>(input_file: &'a Path, output_dir: &'a Path, opts: &Opts, chap: &'a Chapter) -> WorkItem<'a> {
    return WorkItem {
        input_file,
        output_file: compute_outfile(input_file, output_dir, opts, chap),
        chapter: chap,
    };
}

impl WorkItem<'_> {
    pub fn to_cmd(&self, opts: &Opts) -> std::process::Command {
        let mut cmd = std::process::Command::new("ffmpeg");
        cmd.arg("-nostdin");
        cmd.arg("-i").arg(self.input_file);
        cmd.args(["-v", "error", "-map_chapters", "-1", "-vn", "-c", "copy"]);
        cmd.arg("-ss").arg(&self.chapter.start_time);
        cmd.arg("-to").arg(&self.chapter.end_time);
        cmd.arg("-n");
        cmd.args([
            "-metadata",
            &format!("track={}/{}", self.chapter.id + 1, opts.max_chapter_num + 1),
        ]);
        if let Some(title) = self.chapter.title() {
            if opts.use_title_in_meta {
                cmd.args(["-metadata", &format!("title={}", &title.to_str().unwrap())]);
            }
        }
        cmd.arg(&self.output_file);
        return cmd;
    }
}

pub fn read_chapters(path: &Path, debug: bool) -> serde_json::Result<FFProbeOutput> {
    let cmd = Command::new("ffprobe")
        .arg("-i")
        .arg(path)
        .args(["-v", "error", "-print_format", "json", "-show_chapters"])
        .stderr(std::process::Stdio::piped())
        .output()
        .expect("omg failed");

    let s = match str::from_utf8(&cmd.stdout) {
        Ok(v) => v,
        Err(e) => panic!("Invalid UTF-8 sequence: {}", e),
    };

    if debug {
        println!("Chapters JSON:\n-----\n{}\n-----\n", s);
    }

    return serde_json::from_str(&s);
}

fn compute_outfile(input: &Path, out_dir: &Path, opts: &Opts, chap: &Chapter) -> PathBuf {
    let stem = input.file_stem().unwrap();
    let ext = input.extension().unwrap();

    let name = if opts.use_title_as_name {
        let title = chap.title().unwrap_or(&OsStr::new(""));
        if title == "" {
            println!(
                "warning: chapter {} has no title or title is empty, using input filename instead",
                chap.id
            );
            stem
        } else {
            title
        }
    } else {
        stem
    };

    let width = opts.max_chapter_num as usize;

    let new_name = format!(
        "{id:0width$} - {name}.{ext}",
        id = chap.id + 1,
        name = name.to_str().unwrap(),
        ext = opts.output_extension.unwrap_or(ext).to_str().unwrap(),
        width = width
    );
    return out_dir.join(new_name);
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use super::*;

    #[test]
    fn read_chapters_expect_some() {
        let parent = Path::new(file!()).parent().unwrap();
        let res = read_chapters(&parent.join("testdata/beep.m4a"), false).unwrap();
        assert_eq!(res.chapters.len(), 3);
        assert_eq!(res.chapters[0].id, 0);
        assert_eq!(res.chapters[0].tags["title"], "It All Started With a Simple BEEP");
        assert_eq!(res.chapters[1].id, 1);
        assert_eq!(res.chapters[1].tags["title"], "All You Can BEEP Buffee");
        assert_eq!(res.chapters[2].id, 2);
        assert_eq!(res.chapters[2].tags["title"], "The Final Beep");
    }

    #[test]
    fn read_chapters_expect_none() {
        let parent = Path::new(file!()).parent().unwrap();
        let res = read_chapters(&parent.join("testdata/beep-nochap.m4a"), false).unwrap();
        assert_eq!(res.chapters.len(), 0);
    }

    #[test]
    fn test_compute_outfile() {
        let input = PathBuf::from("in/path/baz.m4a");
        let out_dir = PathBuf::from("out/path");
        let opts = Opts::new(2);

        // some of the fields are not really needed...
        let chapter = Chapter {
            id: 0,
            time_base: "0".to_string(),
            start: 0,
            start_time: "0".to_string(),
            end: 1,
            end_time: "0".to_string(),
            tags: HashMap::from([("title".to_string(), "My Fancy Title".to_string())]),
        };

        let output = compute_outfile(&input, &out_dir, &opts, &chapter);

        assert_eq!(output, PathBuf::from("out/path/01 - My Fancy Title.m4a"));
    }
}
