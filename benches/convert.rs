#![feature(test)]
extern crate test;

extern crate xvii;

use test::Bencher;
use xvii::Roman;

#[bench]
fn convert_mmmdcccxciii(b: &mut Bencher) {
    let value = "mmmmdcccxciiii";
    b.iter(|| test::black_box(value.parse::<Roman>()));
}

#[bench]
fn format(b: &mut Bencher) {
    let value = 4894;
    b.iter(|| test::black_box(Roman::from_unchecked(value).to_string()));
}
