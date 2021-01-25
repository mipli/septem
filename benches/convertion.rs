#![feature(test)]
#[cfg(test)]
mod benches {
    extern crate test;

    use septem::Roman;
    use test::{black_box, Bencher};

    #[bench]
    fn convert_mmmdcccxciii(b: &mut Bencher) {
        let value = "mmmmdcccxciiii";
        b.iter(|| black_box(value.parse::<Roman>()));
    }

    #[bench]
    fn format(b: &mut Bencher) {
        let value = 4894u32;
        b.iter(|| black_box(Roman::from_unchecked(value).to_string()));
    }
}
