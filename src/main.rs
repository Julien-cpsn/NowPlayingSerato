use std::fmt::{Debug, Display, Formatter, write};
use std::fs;
use std::fs::{File};
use std::io::Read;
use std::path::{PathBuf};
use std::thread::sleep;
use std::time::{Duration, SystemTime};
use substring::Substring;

struct Track {
    raw_data: String,
    title: String,
    artist: String,
    style: String
}

impl Debug for Track {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Track")
            .field("title", &self.title)
            .field("artist", &self.artist)
            .field("style", &self.style)
            .finish()
    }
}

impl Display for Track {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} — {} — {}", &self.title, &self.artist, &self.style)
    }
}

macro_rules! attempt {
   (@recurse ($a:expr) { } catch ($e:ident) $b:block) => {
      if let Err ($e) = $a $b
   };
   (@recurse ($a:expr) { $e:expr; $($tail:tt)* } $($handler:tt)*) => {
      attempt!{@recurse ($a.and_then (|_| $e)) { $($tail)* } $($handler)*}
   };
   ({ $e:expr; $($tail:tt)* } $($handler:tt)*) => {
      attempt!{@recurse ($e) { $($tail)* } $($handler)* }
   };
}

const SONG_TITLE_START_BYTES: &str = "\x00\x00\x00\x00\x06";
const SONG_ARTIST_START_BYTES: &str = "\x00\x00\x00\x00\x07";
const SONG_STYLE_START_BYTES: &str = "\x00\x00\x00\x00\t";
const SONG_END_BYTES: &str = "\x00\x00\x00\x00\x0f";
const BYTES_TO_SKIP: usize = 10;

fn main() {
    let _serato_ = std::env::args().nth(1).expect("No _serato_ path given");

    let paths = fs::read_dir(_serato_ + "\\History\\Sessions\\").expect("Cannot open folder");

    let mut max_date = SystemTime::UNIX_EPOCH;
    let mut path_with_date = PathBuf::new();

    for entry in paths {
        let path = entry.unwrap().path();
        let metadata = path.metadata().unwrap();
        let created = metadata.created().unwrap();

        if created > max_date {
            max_date = created;
            path_with_date = path;
        }
    }

    println!("File to read: {}", path_with_date.display());

    loop {
        let mut tracks: Vec<Track>= vec![];

        let mut file = File::open(path_with_date.to_owned()).expect("Cannot open file");

        let mut content: Vec<u8> = vec![];
        file.read_to_end(&mut content).expect("Cannot read file");

        let stringed_content = latin1_to_string(content.as_slice());
        let raw_tracks = stringed_content.split("oent").collect::<Vec<&str>>();

        let mut song_title_start: Option<usize>;
        let mut song_artist_start: Option<usize>;
        let mut song_style_start: Option<usize>;
        let mut song_end: Option<usize>;

        for raw_track in raw_tracks {
            song_title_start = raw_track.find(SONG_TITLE_START_BYTES);
            song_artist_start = raw_track.find(SONG_ARTIST_START_BYTES);
            song_style_start = raw_track.find(SONG_STYLE_START_BYTES);
            song_end = raw_track.find(SONG_END_BYTES);

            if song_title_start.is_none() || song_artist_start.is_none() || song_style_start.is_none() || song_end.is_none() {
                continue
            }

            //println!("{}, {}, {}, {}", song_title_start.unwrap(), song_artist_start.unwrap(), song_style_start.unwrap(), song_end.unwrap());

            let track = get_track(
                raw_track,
                song_title_start.unwrap(),
                song_artist_start.unwrap(),
                song_style_start.unwrap(),
                song_end.unwrap()
            );

            tracks.push(track);

            /*
            let mut track: Track = Track {
                raw_data: String::from(raw_track),
                title: raw_track[song_title_start.unwrap() + BYTES_TO_SKIP..song_artist_start.unwrap()].replace("\x00", ""),
                artist: raw_track[song_artist_start.unwrap() + BYTES_TO_SKIP..song_style_start.unwrap()].replace("\x00", ""),
                style: raw_track[song_style_start.unwrap() + BYTES_TO_SKIP..song_end.unwrap()].replace("\x00", ""),
            };*/

        }


        if let Some(last_track) = tracks.last() {
            print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
            println!("{}", last_track);
        };

        //println!("{:?}", tracks);

        sleep(Duration::from_secs(2));
    }
}

fn latin1_to_string(s: &[u8]) -> String {
    s.iter().map(|&c| c as char).collect()
}

fn get_track(raw_track: &str, song_title_start: usize, song_artist_start: usize, song_style_start: usize, song_end: usize) -> Track {
    let title: String;
    let artist: String;
    let style: String;

    if song_title_start + BYTES_TO_SKIP < song_artist_start {
        title = raw_track[song_title_start + BYTES_TO_SKIP..song_artist_start].replace("\x00", "");
    }
    else {
        title = String::new();
    }

    if song_artist_start + BYTES_TO_SKIP < song_style_start {
        artist = raw_track[song_artist_start + BYTES_TO_SKIP..song_style_start].replace("\x00", "");
    }
    else {
        artist = String::new();
    }


    if song_style_start + BYTES_TO_SKIP <song_end {
        style = raw_track[song_style_start + BYTES_TO_SKIP..song_end].replace("\x00", "");
    }
    else {
        style = String::new();
    }

    Track {
        raw_data: String::from(raw_track),
        title,
        artist,
        style,
    }
}