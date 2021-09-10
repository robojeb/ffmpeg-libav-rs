#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

include!(concat!(env!("OUT_DIR"), "/err_bindings.rs"));

pub const fn av_err(err: i32) -> i32 {
    -err.abs()
}

const fn ffaverr(a: char, b: char, c: char, d: char) -> i32 {
    -mk_tag(a, b, c, d)
}

const fn mk_tag(a: char, b: char, c: char, d: char) -> i32 {
    let a = a as i32;
    let b = b as i32;
    let c = c as i32;
    let d = d as i32;

    a | (b << 8) | (c << 16) | (d << 24)
}

// For some reason these don't get pulled in automatically
// so bring these in from https://ffmpeg.org/doxygen/trunk/error_8h_source.html
pub const AVERROR_EOF: i32 = ffaverr('E', 'O', 'F', ' ');
pub const AVERROR_BUG: i32 = ffaverr('B', 'U', 'G', '!');
pub const AVERROR_BUFFER_TOO_SMALL: i32 = ffaverr('B', 'U', 'F', 'S');
pub const AVERROR_DECODER_NOT_FOUND: i32 = ffaverr(0xF8 as char, 'D', 'E', 'C');
pub const AVERROR_DEMUXER_NOT_FOUND: i32 = ffaverr(0xF8 as char, 'D', 'E', 'M');
pub const AVERROR_ENCODER_NOT_FOUND: i32 = ffaverr(0xF8 as char, 'E', 'N', 'C');
pub const AVERROR_EXIT: i32 = ffaverr('E', 'X', 'I', 'T');
pub const AVERROR_EXTERNAL: i32 = ffaverr('E', 'X', 'T', ' ');
pub const AVERROR_FILTER_NOT_FOUND: i32 = ffaverr(0xF8 as char, 'F', 'I', 'L');
pub const AVERROR_INVALIDDATA: i32 = ffaverr('I', 'N', 'D', 'A');
pub const AVERROR_MUXER_NOT_FOUND: i32 = ffaverr(0xF8 as char, 'M', 'U', 'X');
pub const AVERROR_OPTION_NOT_FOUND: i32 = ffaverr(0xF8 as char, 'O', 'P', 'T');
pub const AVERROR_PATCHWELCOME: i32 = ffaverr('P', 'A', 'W', 'E');
pub const AVERROR_PROTOCOL_NOT_FOUND: i32 = ffaverr(0xF8 as char, 'P', 'R', 'O');
pub const AVERROR_BSF_NOT_FOUND: i32 = ffaverr(0xF8 as char, 'B', 'S', 'F');
pub const AVERROR_STREAM_NOT_FOUND: i32 = ffaverr(0xF8 as char, 'S', 'T', 'R');
