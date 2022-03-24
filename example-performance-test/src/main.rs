#![feature(test)]

extern crate test;

fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {
    use test::Bencher;

    #[test]
    fn test1() {}

    #[bench]
    fn bench1(b: &mut Bencher) {
        b.iter(|| println!("test"));
    }
}
