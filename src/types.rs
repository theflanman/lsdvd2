use std::ops::BitAnd;
use dvdread_rs::dvd_time_t;

#[derive(Debug, Default)]
pub struct PlaybackTime {
    pub hour: usize,
    pub minute: usize,
    pub second: usize,
    pub microsecond: usize,
}

impl PlaybackTime {
    pub fn from_dvd_time(dvd_time: dvd_time_t) -> Result<PlaybackTime, String> {
        let frame_u = dvd_time.frame_u;
        let fps: f64 = match frame_u.bitand(0xc0) >> 6 {
            0 => -1.0,
            1 => 25.0,
            2 => -1.0,
            3 => 29.97,
            _ => {return Err("Invalid fps".to_string())}
        };


        let hour = dvd_time.hour;
        let minute = dvd_time.minute;
        let second = dvd_time.second;


        let mut hour = (((hour.bitand(0xf0) >> 3)*5) + (hour.bitand(0x0f))) as usize;
        let mut minute = (((minute.bitand(0xf0) >> 3)*5) + (minute.bitand(0x0f))) as usize;
        let mut second = (((second.bitand(0xf0) >> 3)*5) + (second.bitand(0x0f))) as usize;
        let mut microsecond = (((((second.bitand(0x30) >> 3)*5) as f64) + (second.bitand(0x0f) as f64))*1_000.0/fps) as usize;
        
        while microsecond >= 1000 {
            microsecond = microsecond - 1000;
            second = second + 1;
        }
        
        while second >= 60 {
            second = second - 60;
            minute = minute + 1;
        }
        
        while minute >= 60 {
            minute = minute - 60;
            hour = hour + 1;
        }
        
        Ok(
            PlaybackTime {
                hour,
                minute,
                second,
                microsecond,
            }
        )
    }
}

#[derive(Debug, Default)]
pub struct DiscInfo {
    pub device: String,
    pub disc_title: String,
    pub vmg_id: String,
    pub provider_id: String,
}

#[derive(Debug, Default)]
pub struct TitleGeneralInfo {
    pub length: f64,
    pub playback_time: PlaybackTime,
    pub vts_id: String,
}

#[derive(Debug, Default)]
pub struct Parameter {
    pub vts: i64,
    pub ttn: i64,
    pub fps: f64,
    pub format: String,
    pub aspect: String,
    pub width: String,
    pub height: String,
    pub df: String,
}

#[derive(Debug, Default)]
pub struct AudioStream {
    pub language_code: String,
    pub language: String,
    pub format: String,
    pub frequency: String,
    pub quantization: String,
    pub channels: usize,
    pub ap_mode: usize,
    pub content: String,
    pub stream_id: usize,
}

#[derive(Debug, Default)]
pub struct Chapter {
    pub length: f64,
    pub playback_time: PlaybackTime,
    pub start_cell: usize,
}

#[derive(Debug, Default)]
pub struct DvdCell {
    pub length: f64,
    pub playback_time: PlaybackTime,
    pub first_sector: usize,
    pub last_sector: usize,
}

#[derive(Debug, Default)]
pub struct Subtitle {
    pub language_code: String,
    pub language: String,
    pub content: String,
    pub stream_id: usize,
}

#[derive(Debug, Default)]
pub struct Title {
    pub general: TitleGeneralInfo,
    pub parameter: Parameter,

    pub enabled: bool,

    pub angle_count: usize,
    pub audio_stream_count_reported: usize,
    pub audio_stream_count: usize,
    pub audio_streams: Vec<AudioStream>,
    pub chapter_count_reported: usize,
    pub chapter_count: usize,
    pub chapters: Vec<Chapter>,
    pub cell_count: usize,
    pub cells: Vec<DvdCell>,
    pub subtitle_count_reported: usize,
    pub subtitle_count: usize,
    pub subtitles: Vec<Subtitle>,
    pub palette: Vec<i64>,
}

#[derive(Debug, Default)]
pub struct DvdInfo {

    pub disc_info: DiscInfo,
    pub title_count: usize,
    pub titles: Vec<Title>,
    pub longest_track: usize,
    pub dvd_disc_id: u128,

}