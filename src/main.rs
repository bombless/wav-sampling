fn main() {
    use hound::{WavSpec, WavReader, WavWriter};
    use std::env::args;


    let arg = args().skip(1).next().unwrap();

    println!("arg {arg}");

    let reader = WavReader::open(arg).unwrap();


    let WavSpec {sample_rate: input_rate, bits_per_sample,  ..} = reader.spec();

    println!("{:?}", reader.spec());

    println!("input sample rate {input_rate}, bps {bits_per_sample}");

    assert_eq!(16, bits_per_sample);



    let samples = reader.into_samples();


    let mut input = Vec::new();

    for x in samples {
        let x: i16 = x.unwrap();
        input.push(x);
    }

    let sound_length = input.len() as f32 / input_rate as f32;

    println!("length {sound_length}");

    let output_rate = 16000;
    let output_length = (sound_length * output_rate as f32) as usize;
    let output_spec = WavSpec {
        channels: 1,
        sample_rate: output_rate,
        bits_per_sample: 16,
        sample_format: hound::SampleFormat::Int,
    };
    let mut writer = WavWriter::create("output.wav", output_spec).unwrap();
    for i in 0 .. output_length {
        let t = i as f32 / output_rate as f32;


        let offset =  (t / input_rate as f32) as usize;

        let sample = input[offset];

        if i % 100000 == 0 {
            println!("sampling {} and {}/{}: {sample}", i / output_rate as usize, i % output_rate as usize, output_rate);
        }



        writer.write_sample(sample).unwrap();
    }
}
