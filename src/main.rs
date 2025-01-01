mod types;
mod serializers;

use std::{env, fs};
use std::ops::BitAnd;
use std::os::raw::c_int;
use std::ptr::null;
use std::slice::from_raw_parts_mut;

use dvdread_rs::{dvd_time_t, ifoOpen, ifoRead_TXTDT_MGI, ifo_handle_t, DVDDiscID, DVDOpen};
use clap::{ValueEnum, Parser};

use crate::serializers::human_readable::HumanReadableSerializer;
use crate::types::{PlaybackTime, Title};

#[derive(ValueEnum, Clone, Debug)]
enum OutputFormat {
    HumanReadable,
    JSON,
}

/// argument parser
#[derive(Parser, Debug)]
#[command(version)]
struct Args {

    /// Path to a dvd block device, image archive, or directory (mounted or archived).
    path: String,

    #[clap(short='a')]
    audio_streams: bool,

    #[clap(short='d')]
    cells: bool,

    #[clap(short='n')]
    angles: bool,

    #[clap(short='c')]
    chapters: bool,

    #[clap(short='s')]
    subpictures: bool,

    #[clap(short='P')]
    palette: bool,
    
    #[clap(short='v')]
    video: bool,
    
    #[clap(short='x')]
    all_information: bool,
    
    #[clap(short='q')]
    quiet: bool,
    
    #[clap(short='f', default_value = "human-readable")]
    format: OutputFormat,

}

unsafe fn unsafe_main() {

    let args = Args::parse();

    println!("{:?}", args);

    let dvd_title = String::from_utf8([40; 32].to_vec()).unwrap();

    // something about wh
    let mut max_length = 0;
    let mut max_track = 0;


    // println!("Hello, world!");
    // Prints each argument on a separate line
    // for argument in env::args() {
    //     println!("{argument}");
    // }

    // println!("args: {:?}", env::args());

    let filename = args.path.clone();

    // let metadata = fs::metadata(&filename);

    // println!("metadata: {:?}", metadata);

    let reader = DVDOpen(filename.as_str().as_ptr() as *const i8) ;

    let ifo_zero = ifoOpen(reader, 0);
    let ifo_zero_deref = { ifo_zero.read() };


    // { println!("ifo_zero: {:?}", ifo_zero.read().vts_atrt.read()); }

    let mut ifo_list: Vec<*mut ifo_handle_t> = Vec::new();
    // ifo_list.push({ ifoOpen(reader, 0) });
    // println!("vtss: {:?}", { ifo_zero.read().vts_atrt.read().nr_of_vtss } as usize);
    //
    // println!("ifo_list size: {:?}", ifo_list.capacity());
    // println!("ifo_list[0]: {:?}", ifo_list[0]);

    for i in 0..(ifo_zero.read().vts_atrt.read().nr_of_vtss as usize + 1) {
        ifo_list.push(ifoOpen(reader, i as c_int));

        // println!("ifo_list[{}]: {:?}", i, {ifo_list[i].read()});
        if ifo_list[i] == 0 as *mut ifo_handle_t && true {
            panic!("ifo_list[{}] is empty", i);
        }
    }

    // println!("ifo_list: {:?}", ifo_list[0]);

    let num_titles = &{ ifo_zero.read().tt_srpt.read().nr_of_srpts };

    let has_title = get_title_name(filename.clone(), &dvd_title);

    let mut dvd_info = types::DvdInfo::default();

    dvd_info.disc_info.device = filename.clone();
    dvd_info.disc_info.disc_title = if has_title { dvd_title } else { "unknown".to_string() };
    dvd_info.disc_info.vmg_id = ifo_zero.read().vmgi_mat.read().vmg_identifier.iter().map(|x| *x as u8 as char).collect();
    dvd_info.disc_info.provider_id = ifo_zero.read().vmgi_mat.read().provider_identifier.iter().map(|x| *x as u8 as char).collect();

    // println!("disc_info: {:?}", dvd_info.disc_info );

    dvd_info.title_count = *num_titles as usize;

    for _ in 0..*num_titles {
        dvd_info.titles.push(Title::default());
    }

    let disc_id = &mut [0u8; 16];
    let result = { DVDDiscID(reader, disc_id as *mut u8) };

    for (j, title) in dvd_info.titles.iter_mut().enumerate() {
        if true {  // this will be set for a command line argument later
            // println!("{j}, {title:?}");
            let ifo_title = std::slice::from_raw_parts_mut(ifo_zero.read().tt_srpt.read().title, *num_titles as usize + 1)[j];

            let ifo_title_set_number = ifo_title.title_set_nr;
            let ifo = ifo_list[ifo_title_set_number as usize].read();
            let mut vtsi_mat = ifo_list[ifo_title_set_number as usize].read().vtsi_mat.read();
            // println!("{ifo:?}");
            // println!("{:?}", vtsi_mat);

            title.enabled = true;

            let vts_pgcit = { ifo.vts_pgcit.read() };
            let video_attr = vtsi_mat.vts_video_attr;
            let vts_ttn = ifo_title.vts_ttn;
            let vmgi_mat = ifo_zero_deref.vmgi_mat.read();
            let title_set_number = ifo_title.title_set_nr;

            let program_chain_number = from_raw_parts_mut(ifo.vts_ptt_srpt.read().title, *num_titles as usize)[vts_ttn as usize - 1].ptt.read().pgcn as usize;
            let pgc = from_raw_parts_mut(vts_pgcit.pgci_srp, program_chain_number)[program_chain_number - 1].pgc.read();
            // println!("{:?}", pgc);

            title.general.vts_id = String::from_utf8(from_raw_parts_mut(vtsi_mat.vts_identifier.as_mut_ptr() as *mut u8, 12).to_vec()).unwrap();
            title.chapter_count_reported = ifo_title.nr_of_ptts as usize;


            if pgc.cell_playback as *const _ == null() || pgc.program_map as *const _ == null() {
            } else {
                let pgc_playback_time = dvd_time_to_milliseconds(pgc.playback_time).unwrap();

                title.general.length = pgc_playback_time as f64/1000.0;
                title.general.playback_time = PlaybackTime::from_dvd_time(pgc.playback_time).unwrap();
                title.chapter_count = pgc.nr_of_cells as usize;
                title.audio_stream_count_reported = vtsi_mat.nr_of_vts_audio_streams as usize;
                for k in 0..title.audio_stream_count_reported {
                    if pgc.subp_control[k].bitand(0x8000) != 0 {
                        title.audio_stream_count += 1;
                    }
                }

                if pgc_playback_time > max_length {
                    max_length = pgc_playback_time;
                    max_track = j + 1;
                }
            }

            // println!("{title:?}");
            // println!();

        }
    }


    println!("finished loop");

    println!(
        "{}", 
        match args.format { 
            OutputFormat::HumanReadable => {HumanReadableSerializer::new(dvd_info)} 
            OutputFormat::JSON => {todo!()} 
        },
    );

}



fn dvd_time_to_milliseconds(pgc_time: dvd_time_t) -> Result<i64, String> {
    let frame_u = pgc_time.frame_u;
    let fps: f64 = match frame_u.bitand(0xc0) >> 6 {
        0 => -1.0,
        1 => 25.0,
        2 => -1.0,
        3 => 29.97,
        _ => {return Err("Invalid fps".to_string())}
    };

    let hour = pgc_time.hour;
    let minute = pgc_time.minute;
    let second = pgc_time.second;

    let mut ms: i64 = 0;
    ms = ms + ((((hour.bitand(0xf0) >> 3)*5) as i64) + (hour.bitand(0x0f) as i64))*3_600_000;
    ms = ms + ((((minute.bitand(0xf0) >> 3)*5) as i64) + (minute.bitand(0x0f) as i64))*60_000;
    ms = ms + ((((second.bitand(0xf0) >> 3)*5) as i64) + (second.bitand(0x0f) as i64))*1_000;

    if fps > 0.0 {
        ms = ms + (((((second.bitand(0x30) >> 3)*5) as f64) + (second.bitand(0x0f) as f64))*1_000.0/fps) as i64;
    }

    Ok(ms)

}

fn main() {
    unsafe { unsafe_main() }

}

///
fn get_title_name(dvd_device: String, dvd_title: &String) -> bool {
    false
}
