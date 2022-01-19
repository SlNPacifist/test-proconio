use std::fs::File;
use std::io::BufReader;
use proconio::{input, derive_readable};
use proconio::source::auto::AutoSource;

#[derive(Debug, PartialEq)]
struct Order {
    row: usize,
    column: usize,
    product_types: Vec<usize>,
}

#[derive(Debug, PartialEq)]
struct Warehouse {
    row: usize,
    column: usize,
    product_amount: Vec<usize>,
}

#[derive(Debug, PartialEq)]
struct Data {
    rows: usize,
    columns: usize,
    drones: usize,
    deadline: usize,
    maximum_drone_load: usize,
    product_types: usize,
    weights: Vec<usize>,
    warehouses: Vec<Warehouse>,
    orders: Vec<Order>,
}

pub fn solve() {
    let file = AutoSource::new(BufReader::new(File::open("./src/year_2016_qual/qualification_round_2016.in/busy_day.in").unwrap()));
    input! {
        from file,
        rows: usize,
        columns: usize,
        d: usize,
        deadline: usize,
        maximum_drone_load: usize,
        p: usize,
        weights: [usize; p],
        w: usize,
        warehouses: [(usize, usize, [usize; p]); w],
        c: usize,
        orders: [(usize, usize, [usize]); c],
    }

    let data = Data {
        rows,
        columns,
        drones: d,
        deadline,
        maximum_drone_load,
        product_types: p,
        weights,
        warehouses: warehouses.into_iter().map(|(row, column, product_amount)| Warehouse {
            row,
            column,
            product_amount,
        }).collect(),
        orders: orders.into_iter().map(|(row, column, product_types)| Order {
            row,
            column,
            product_types,
        }).collect()
    };
    println!("{:?}", data);
}