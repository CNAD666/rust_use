pub fn main() {
    let data = Sample { d: 11111, ..Sample::default() };
    println!("{:?}", data);
}

#[derive(Debug)]
struct Sample {
    a: u32,
    b: u32,
    c: u32,
    d: u32,
    e: u32,
    f: u32,
}

impl Default for Sample {
    fn default() -> Self {
        Sample { a: 1, b: 2, c: 3, d: 4, e: 5, f: 6 }
    }
}
