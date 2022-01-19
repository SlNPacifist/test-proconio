use proconio::input;
use proconio::source::auto::AutoSource;
use std::fs::File;
use std::io::BufReader;

pub fn solve() {
    input! {
        from AutoSource::new(BufReader::new(File::open("src/year_2014_final/paris_54000.txt").unwrap())),
        n: usize,
        m: usize,
        t: usize,
        c: usize,
        s: usize,
        junctions: [(f64, f64); n],
        streets: [(usize, usize, u8, usize, usize); m],
    };

    println!("n: {}, m: {}, t: {}, c: {}, s: {}, junctions: {}, streets: {}", n, m, t, c, s, junctions.len(), streets.len());
}