use ffav::{
    error::Error,
    raw::{
        codec::Codec,
        filter::{
            audio::{ABufferSink, ABufferSource, AFormat},
            FilterGraph,
        },
        format::Format,
    },
    tags::Audio,
    util::{channels::ChannelLayout, sampling::SampleFormat},
};

fn main() {
    let mut input = Format::open_input(std::env::args().nth(1).unwrap()).unwrap();

    let best_stream = input.get_best_stream::<Audio>().unwrap();

    let mut codec = Codec::open_decode(&best_stream).unwrap();

    let mut fg = FilterGraph::new().unwrap();

    let src = ABufferSource::from_decoded_stream("src", codec.out_stream_config());

    let afmt = AFormat::new(
        "format",
        Some(44100),
        Some(SampleFormat::PackedI16),
        Some(ChannelLayout::LAYOUT_STEREO),
    );

    let src = fg.add_input(src).unwrap();
    let afmt = fg.add(afmt).unwrap();
    let sink = fg.add_output(ABufferSink::new("out")).unwrap();

    fg.connect(&src, 0, &afmt, 0).unwrap();
    fg.connect(&afmt, 0, &sink, 0).unwrap();
    //fg.connect(&src, 0, &sink, 0);

    let mut fg = fg.configure().unwrap();

    loop {
        match fg.get_output(&sink) {
            Err(Error::SubmitMoreInput) => {
                let mut pkt = match input.get_next_packet() {
                    Err(Error::EoF) => break,
                    Ok(p) => p,
                    Err(e) => panic!("Could not get packet {}", e),
                };

                codec.submit_packet(&mut pkt).unwrap();

                fg.submit_input(&src, &mut codec.get_next_frame().unwrap())
                    .unwrap();
            }
            Err(e) => panic!("{}", e),
            Ok(_frame) => {
                println!("Frame")
            }
        }
    }
}
