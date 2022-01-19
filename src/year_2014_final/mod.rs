use proconio::input;
use proconio::source::auto::AutoSource;
use std::fs::File;
use std::io::BufReader;

#[derive(PartialEq, Debug)]
struct Data {
    pub n: usize,
    pub m: usize,
    pub t: usize,
    pub c: usize,
    pub s: usize,
    pub junctions: Vec<(f64, f64)>,
    pub streets: Vec<(usize, usize, u8, usize, usize)>,
}

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
    let data = Data {n,m,t,c,s,junctions,streets};
    println!("n: {}, m: {}, t: {}, c: {}, s: {}, junctions: {}, streets: {}", data.n, data.m, data.t, data.c, data.s, data.junctions.len(), data.streets.len());
}