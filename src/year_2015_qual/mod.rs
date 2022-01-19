use std::fs::File;
use std::io::BufReader;
use proconio::source::auto::AutoSource;
use proconio::{input, derive_readable};

#[derive_readable]
#[derive(Debug, PartialEq)]
struct Coord {
    r: usize,
    s: usize,
}

#[derive_readable]
#[derive(Debug, PartialEq)]
struct Server {
    z: usize,
    c: usize,
}

#[derive(Debug, PartialEq)]
struct Data {
    r: usize,
    s: usize,
    p: usize,
    unavailable: Vec<Coord>,
    servers: Vec<Server>,
}

pub fn solve() {
    let file = AutoSource::new(BufReader::new(File::open("./src/year_2015_qual/dc.in").unwrap()));
    input! {
        from file,
        r: usize,
        s: usize,
        u: usize,
        p: usize,
        m: usize,
        unavailable: [Coord; u],
        servers: [Server; m],
    };

    let data = Data {r,s,p,unavailable,servers};
    println!("{:?}", data);
}