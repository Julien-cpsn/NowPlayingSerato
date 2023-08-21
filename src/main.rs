use std::fmt::{Debug, Display, Formatter};
use std::fs;
use std::fs::{File};
use std::io::Read;
use std::path::{PathBuf};
use std::thread::sleep;
use std::time::{Duration, SystemTime};
use colored::Colorize;

struct Track {
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
        write!(f, "{}", format!(" {:70}\n {:70}", &self.title.bold(), &self.artist).on_truecolor(0, 255, 0))
    }
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
        let raw_tracks = stringed_content.split("oent")
            .map(|data| {
                data.replace(|c: char| !c.is_ascii(), "*")
            })
            .collect::<Vec<String>>();

        let mut song_title_start: Option<usize>;
        let mut song_artist_start: Option<usize>;
        let mut song_style_start: Option<usize>;
        let mut song_end: Option<usize>;

        for raw_track in raw_tracks {
            song_title_start = raw_track.find(SONG_TITLE_START_BYTES);
            song_artist_start = raw_track.find(SONG_ARTIST_START_BYTES);
            song_style_start = raw_track.find(SONG_STYLE_START_BYTES);
            song_end = raw_track.find(SONG_END_BYTES);

            if song_title_start.is_none() || song_end.is_none() {
                continue
            }

            let track = get_track(
                raw_track,
                song_title_start.unwrap(),
                song_artist_start,
                song_style_start,
                song_end.unwrap()
            );

            tracks.push(track);
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

fn get_track(raw_track: String, song_title_start: usize, song_artist_start: Option<usize>, song_style_start: Option<usize>, song_end: usize) -> Track {
    let mut title = String::new();
    let mut artist = String::new();
    let mut style = String::new();


    if song_style_start.is_some() {
        if song_title_start + BYTES_TO_SKIP < song_artist_start.unwrap() {
            title = raw_track[song_title_start + BYTES_TO_SKIP..song_artist_start.unwrap()].replace("\x00", "");
        }

        if song_style_start.is_some() {
            if song_artist_start.unwrap() + BYTES_TO_SKIP < song_style_start.unwrap() {
                artist = raw_track[song_artist_start.unwrap() + BYTES_TO_SKIP..song_style_start.unwrap()].replace("\x00", "");
            }
        }
        else {
            if song_artist_start.unwrap() + BYTES_TO_SKIP < song_end {
                artist = raw_track[song_artist_start.unwrap() + BYTES_TO_SKIP..song_end].replace("\x00", "");
            }
        }
    }
    else if song_style_start.is_some() {
        if song_title_start + BYTES_TO_SKIP < song_style_start.unwrap() {
            title = raw_track[song_title_start + BYTES_TO_SKIP..song_style_start.unwrap()].replace("\x00", "");
        }
    }
    else {
        if song_title_start + BYTES_TO_SKIP < song_end {
            title = raw_track[song_title_start + BYTES_TO_SKIP..song_end].replace("\x00", "");
        }
    }

    if song_style_start.is_some() {
        if song_style_start.unwrap() + BYTES_TO_SKIP < song_end {
            style = raw_track[song_style_start.unwrap() + BYTES_TO_SKIP..song_end].replace("\x00", "");
        }
    }

    Track {
        title,
        artist,
        style,
    }
}