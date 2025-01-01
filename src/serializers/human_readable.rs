use std::fmt::{Display, Formatter, Write};
use crate::types::DvdInfo;

pub struct HumanReadableSerializer {
    dvd_info: DvdInfo,
}

impl HumanReadableSerializer {
    pub fn new(dvd_info: DvdInfo) -> HumanReadableSerializer {HumanReadableSerializer {dvd_info}}
}

impl Display for HumanReadableSerializer {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let indent = "";
        f.write_str(format!("{indent}Disc Title: {}\n\n", self.dvd_info.disc_info.disc_title).as_str())?;
        for (i, title) in self.dvd_info.titles.iter().enumerate() {
            f.write_str(
                format!(
                    "{indent}Title: {}, Length: {:02}:{:02}:{:02}.{:03}\n",
                    i,
                    title.general.playback_time.hour,
                    title.general.playback_time.minute,
                    title.general.playback_time.second,
                    title.general.playback_time.microsecond,
                ).as_str()
            )?;
            let indent = format!("{indent}  ");
            f.write_str(
                format!(
                    "{indent}Chapters: {:02}, Cells {:02}, Audio Streams: {:02}, Subpictures: {:02}\n",
                    title.chapter_count,
                    title.cell_count,
                    title.audio_stream_count,
                    title.subtitle_count,
                ).as_str()
            )?;
            if title.parameter.format != "" {
                panic!()
            }
            
            
            
        }

        f.write_char('\n')
    }
}