#![feature(test)]
extern crate test;

extern crate xvii;

use test::Bencher;
use xvii::Roman;

#[bench]
fn convert_from(b: &mut Bencher) {
    let values: Vec<_> = (100..501)
        .map(|n| Roman::from_unchecked(n).to_string())
        .collect();

    b.iter(|| {
        for value in &values {
            test::black_box(value.parse::<Roman>().ok());
        }
    });
}

#[bench]
fn format(b: &mut Bencher) {
    let values: Vec<_> = (100..501).collect();

    b.iter(|| {
        for &value in &values {
            test::black_box(Roman::from_unchecked(value).to_string());
        }
    })
}
