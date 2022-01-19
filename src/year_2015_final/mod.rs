use std::fs::File;
use std::io::BufReader;
use proconio::{input, derive_readable};
use proconio::source::auto::AutoSource;

#[derive_readable]
#[derive(Debug, PartialEq)]
struct Coord {
    r: usize,
    c: usize,
}

#[derive_readable]
#[derive(Debug, PartialEq)]
struct Movement {
    r: isize,
    c: isize,
}

#[derive(Debug, PartialEq)]
struct Data {
    rows: usize,
    columns: usize,
    altitudes: usize,
    coverage_radius: usize,
    balloons: usize,
    turns: usize,
    start: Coord,
    target: Vec<Coord>,
    winds: Vec<Vec<Vec<Movement>>>,
}

pub fn solve() {
    let file = AutoSource::new(BufReader::new(File::open("./src/year_2015_final/hashcode_2015_final_round.in").unwrap()));
    input! {
        from file,
        r: usize,
        c: usize,
        a: usize,
        l: usize,
        v: usize,
        b: usize,
        t: usize,
        start: Coord,
        target: [Coord; l],
        winds: [[[Movement; c]; r]; a],
    };

    let data = Data { rows: r, columns: c, altitudes: a, coverage_radius: v, balloons: b, turns: t, start, target, winds };
    println!("{:?}", data);
}