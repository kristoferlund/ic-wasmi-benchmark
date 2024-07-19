#[no_mangle]
pub extern "C" fn run() -> i64 {
    let mut sum: i64 = 0;
    for i in 0..1_000_000 {
        sum += (i % 100) * (i % 100);
    }
    sum
}
