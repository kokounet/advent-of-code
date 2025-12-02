#[macro_export]
macro_rules! time {
    ($expr:expr) => {{
        let start = std::time::Instant::now();
        let res = { $expr };
        let dur = std::time::Instant::now().duration_since(start);
        println!("{}: {:?}", std::stringify!($expr), dur);
        res
    }};
}
