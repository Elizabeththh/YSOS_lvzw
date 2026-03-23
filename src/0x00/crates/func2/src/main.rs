fn main() {
    let (size, unit) = humanized_size(784848954785);
    println!("Size : {:.4} {}", size, unit);
}

pub fn humanized_size(size: u64) -> (f64, &'static str) {
    let units = vec!["B", "KiB", "MiB", "GiB", "TiB"];
    let mut float_size = size as f64;
    let mut cnt = 0;

    while float_size > 1024.0 && cnt < units.len() - 1 {
        float_size /= 1024.0;
        cnt += 1;
    }

    (float_size, units[cnt])
}

#[test]
fn test_humanized_size() {
    let byte_size = 1554056;
    let (size, unit) = humanized_size(byte_size);
    assert_eq!("Size : 1.4821 MiB", format!("Size : {:.4} {}", size, unit));
}
