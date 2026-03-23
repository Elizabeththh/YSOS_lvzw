use std::{fs, io, thread, time::Duration};

pub fn count_down(seconds: u64) {
    for i in (1..=seconds).rev() {
        thread::sleep(Duration::from_secs(1));
        println!("{i} seconds left");
    }
    println!("Countdown finished!");
}
pub fn read_and_print(file_path: &str) -> io::Result<()> {
    let data = fs::read_to_string(file_path)?;
    println!("{}", data);
    Ok(())
}

pub fn file_size(file_path: &str) -> Result<u64, &'static str> {
    let metadata = fs::metadata(file_path).map_err(|_| "File not found!")?;
    let file_size = metadata.len();

    Ok(file_size)
}
