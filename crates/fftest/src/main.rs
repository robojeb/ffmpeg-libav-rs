use std::time::Duration;

use ffav::{
    error::Error,
    raw::{
        codec::Codec,
        filter::{
            audio::{ABufferSink, ABufferSource, ACrosssfade, AFormat, ATrim},
            FilterGraph,
        },
        format::Format,
    },
    tags::Audio,
    util::{channels::ChannelLayout, sampling::SampleFormat},
};

fn main() {
    let input = Format::open_input(std::env::args().nth(1).unwrap()).unwrap();
    let input2 = Format::open_input(std::env::args().nth(2).unwrap()).unwrap();

    let best_stream = input.get_best_stream::<Audio>().unwrap();
    let codec = Codec::open_decode(&best_stream).unwrap();

    let best_stream = input2.get_best_stream::<Audio>().unwrap();
    let codec2 = Codec::open_decode(&best_stream).unwrap();

    let mut fg = FilterGraph::new().unwrap();

    let src = ABufferSource::from_decoded_stream("src", codec.out_stream_config());
    let src2 = ABufferSource::from_decoded_stream("src2", codec2.out_stream_config());

    let seq = ACrosssfade::no_overlap("cross", Duration::from_secs(0));

    let afmt = AFormat::new(
        "format",
        Some(32000),
        Some(SampleFormat::PackedI16),
        Some(ChannelLayout::LAYOUT_STEREO),
    );

    let trim1 = ATrim::new_with_duration("trim1", Duration::from_secs(0), Duration::from_secs(10));
    let trim2 = ATrim::new_with_duration("trim1", Duration::from_secs(0), Duration::from_secs(10));

    let src = fg.add_input(src).unwrap();
    let trim1 = fg.add(trim1).unwrap();
    let src2 = fg.add_input(src2).unwrap();
    let trim2 = fg.add(trim2).unwrap();
    let cross = fg.add(seq).unwrap();
    let afmt = fg.add(afmt).unwrap();
    let sink = fg.add_output(ABufferSink::new("out")).unwrap();

    fg.connect(&src, 0, &trim1, 0).unwrap();
    fg.connect(&src2, 0, &trim2, 0).unwrap();

    fg.connect(&trim1, 0, &cross, 0).unwrap();
    fg.connect(&trim2, 0, &cross, 1).unwrap();

    fg.connect(&cross, 0, &afmt, 0).unwrap();
    fg.connect(&afmt, 0, &sink, 0).unwrap();

    let mut fg = fg.configure().unwrap();

    println!("{}", fg);

    let mut seq_pos = 0usize;
    let mut in_seqs = [(input, codec, src), (input2, codec2, src2)];

    loop {
        match fg.get_output(&sink) {
            Err(Error::SubmitMoreInput) => {
                let item = &mut in_seqs[seq_pos];
                let mut pkt = match item.0.get_next_packet() {
                    Err(Error::EoF) => {
                        seq_pos += 1;
                        if seq_pos < in_seqs.len() {
                            continue;
                        } else {
                            break;
                        }
                    }
                    Ok(p) => p,
                    Err(e) => panic!("Could not get packet {}", e),
                };

                match item.1.submit_packet(&mut pkt) {
                    Ok(()) => {}
                    Err(Error::PacketFromInvalidStream) => continue,
                    Err(e) => panic!("{}", e),
                }

                fg.submit_input(&item.2, &mut item.1.get_next_frame().unwrap())
                    .unwrap();
            }
            Err(Error::EoF) => break,
            Err(e) => panic!("{}", e),
            Ok(frame) => {
                println!("Frame: {:?}", frame.get_pts());
            }
        }
    }
}
