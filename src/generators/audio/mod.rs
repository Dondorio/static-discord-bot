use hound;

pub async fn generate_white_noise(fname: &str, len: f32, amplitude: f32, rate: f32) {
    let spec = hound::WavSpec {
        channels: 1,
        sample_rate: (44100.0 * rate) as u32,
        bits_per_sample: 16,
        sample_format: hound::SampleFormat::Int,
    };
    let mut writer = hound::WavWriter::create(fname, spec).unwrap();
    // TODO parallelize this
    for _t in (0..spec.sample_rate * len as u32).map(|x| x as f32 / spec.sample_rate as f32) {
        let sample: f32 = rand::random_range(-1.0..1.0);
        let amp = i16::MAX as f32 * 0.5 * amplitude;

        writer.write_sample((sample * amp) as i16).unwrap();
    }
}
