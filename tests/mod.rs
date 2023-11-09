use std::{io::Write, time::Instant};

// this will generate random data of 10MB to be written to the file
fn generate_data() -> Vec<u8> {
    let mut data = Vec::with_capacity(10 * 1024 * 1024);
    for _ in 0..10 * 1024 * 1024 {
        data.push(rand::random::<u8>());
    }
    data
}

fn write_buf_writer(mut data: &[u8], dst: &str, size: usize) {
    let file = std::fs::File::create(dst).unwrap();
    let mut writer = std::io::BufWriter::with_capacity(size, file);
    let start: Instant = Instant::now();
    writer.write_all(data).unwrap();
    // std::io::copy(&mut data, &mut writer).unwrap();
    let duration = start.elapsed();
    println!(
        "Time elapsed for {}kb copy: {:?} - speed: {}",
        size / 1024,
        duration.as_millis(),
        size as u128 / duration.as_millis()
    );
}

#[test]
fn bufwriter_test() {
    let dst = "./somefile.txt";
    let data = generate_data();

    // warm up
    write_buf_writer(&data, dst, 8 * 1024);

    // 8kb
    write_buf_writer(&data, dst, 8 * 1024);
    // 16kb
    write_buf_writer(&data, dst, 16 * 1024);
    // 24kb
    write_buf_writer(&data, dst, 24 * 1024);
    // 32kb
    write_buf_writer(&data, dst, 32 * 1024);
    // 40kb
    write_buf_writer(&data, dst, 40 * 1024);
    // 48kb
    write_buf_writer(&data, dst, 48 * 1024);
    // 56kb
    write_buf_writer(&data, dst, 56 * 1024);
    // 64kb
    write_buf_writer(&data, dst, 64 * 1024);
}
