// use ffav::{
//     error::Error,
//     raw::{
//         codec::Codec,
//         filter::{
//             video::{VBufferSink, VBufferSource, VFormat},
//             FilterGraph,
//         },
//         format::Format,
//     },
//     util::{
//         color::{Endian, PixelFormat},
//         marker::Video,
//     },
// };

fn main() {
    // let mut input = Format::open_input(std::env::args().nth(1).unwrap()).unwrap();
    // let best_stream = input.get_best_stream::<Video>().unwrap();
    // let config = best_stream.config();
    // println!("{:?} {:?}", config.pixel_format(), config.color_primary());

    // let mut codec = Codec::open_decode(&best_stream).unwrap();

    // let mut fg = FilterGraph::new().unwrap();

    // let src = VBufferSource::from_decoded_stream("src", codec.out_stream_config());
    // let format = VFormat::new("fmt", Some(PixelFormat::RGB48(Endian::Little)));
    // let sink = VBufferSink::new("sink");

    // let src = fg.add_input(src).unwrap();
    // let sink = fg.add_output(sink).unwrap();
    // let format = fg.add(format).unwrap();

    // fg.connect(&src, 0, &format, 0).unwrap();
    // fg.connect(&format, 0, &sink, 0).unwrap();

    // let mut fg = fg.configure().unwrap();

    // loop {
    //     match fg.get_output(&sink) {
    //         Err(Error::SubmitMoreInput) => {
    //             let mut pkt = match input.get_next_packet() {
    //                 Err(Error::EoF) => break,
    //                 Ok(p) => p,
    //                 Err(e) => panic!("Could not get packet {}", e),
    //             };

    //             match codec.submit_packet(&mut pkt) {
    //                 Ok(()) => {}
    //                 // Filter out packets that come from streams we don't care about
    //                 Err(Error::PacketFromInvalidStream) => continue,
    //                 Err(e) => panic!("{}", e),
    //             }

    //             let mut frame = codec.get_next_frame().unwrap();
    //             let plane = frame.plane(0);

    //             for pix in plane.chunks_exact(plane.linesize) {
    //                 println!("Input: {:?}", pix);
    //             }

    //             fg.submit_input(&src, &mut frame).unwrap();
    //         }
    //         Err(Error::EoF) => break,
    //         Err(e) => panic!("{}", e),
    //         Ok(frame) => {
    //             let plane = frame.plane(0);
    //             for pix in plane
    //                 .chunks_exact(plane.linesize)
    //                 .flat_map(|line| line.chunks_exact(2))
    //             {
    //                 println!("Output: {:?}", pix);
    //             }
    //         }
    //     }
    // }
}
