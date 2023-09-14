use std::f32::consts::PI;

use fundsp::prelude::*;
use hound::{WavReader, WavWriter, WavSpec, SampleFormat};
use rustfft::num_complex::Complex;

fn main() {
}

#[test]
fn fft() {
    let (buffer, aux, spec) = read_wav("H.wav");

    let mut complex_buffer = vec_to_complex_vec(&buffer);

    let mut planer = rustfft::FftPlanner::<f32>::new();
    let fft = planer.plan_fft_forward(aux.len());


    fft.process(&mut complex_buffer);
    
    std::fs::write("data.txt", complex_buffer.iter().map(|s| s.re as u8).collect::<Vec<_>>());

    let cutoff_bin = (100.0 * complex_buffer.len() as f32) as usize;
    for i in cutoff_bin + 1..complex_buffer.len() {
        complex_buffer[i] = Complex::new(0.0, 0.0);
    }

    let ifft = planer.plan_fft_inverse(complex_buffer.len());
    ifft.process(&mut complex_buffer);
    
    write_wav("filtered_fft.wav", &complex_vec_to_vec(&complex_buffer), spec);
}

#[test]
fn descending_frequency() {
    let duration = 2f32;
    let sample_rate = 48000f32;
    let length = duration * sample_rate;

    let mut signal = (0.0..length).collect::<Vec<f32>>();

    let mut ifft_planner = rustfft::FftPlanner::new();
    let ifft = ifft_planner.plan_fft_inverse(signal_len);
    ifft.process(&mut frequency_domain_signal);

    write_wav("descending_frequency.wav", &complex_vec_to_vec(&frequency_domain_signal), WavSpec { channels: 1, sample_rate: 48000, bits_per_sample: 32, sample_format: SampleFormat::Float });
}

#[test]
fn lowpass() {
    let (mut buffer, aux, spec) = read_wav("H.wav");

    let mut butter = ButterLowpass::<f32, f32, U1>::new(1000.0);
    butter.set_sample_rate(spec.sample_rate as f64);

    buffer.chunks_mut(MAX_BUFFER_SIZE).zip(aux.chunks(MAX_BUFFER_SIZE)).for_each(|(out, aux)| {
        butter.process(MAX_BUFFER_SIZE, &[aux], &mut [out]);
    });

    write_wav("filtered_lowpass.wav", &buffer, spec);
}

fn read_wav<P: AsRef<str>>(path: P) -> (Vec<f32>, Vec<f32>, WavSpec) { 
    let mut reader = WavReader::open(path.as_ref()).unwrap();
    let buffer = reader
        .samples::<i16>()
        .map(|s| s.unwrap() as f32)
        .collect::<Vec<_>>();
    let aux = buffer.clone();

    (buffer, aux, reader.spec())
}

fn write_wav<P: AsRef<str>>(path: P, data: &[f32], spec: WavSpec) {
    let mut writer = WavWriter::create(path.as_ref(), spec).unwrap();
    data.iter().for_each(|s| writer.write_sample(*s as i16).unwrap());
    writer.finalize().unwrap();
}

fn vec_to_complex_vec(buffer: &[f32]) -> Vec<Complex<f32>> {
    buffer.iter().map(|s| Complex::new(*s, 0.0)).collect::<Vec<_>>()
}

fn complex_vec_to_vec(buffer: &[Complex<f32>]) -> Vec<f32> {
    buffer.iter().map(|s| s.re / buffer.len() as f32).collect::<Vec<_>>()
}