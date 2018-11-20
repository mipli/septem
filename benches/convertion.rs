#![feature(test)] 
#[cfg(test)]
mod tests {
    extern crate test;

    use self::test::{black_box, Bencher};
    use septem::Roman;

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
