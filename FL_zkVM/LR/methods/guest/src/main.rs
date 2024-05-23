#![no_main]
// If you want to try std support, also update the guest Cargo.toml file

use ndarray::Array2;
use std::io;

fn build_data(width: usize, height: usize) -> Array2<f64> {
    let mut arr = Array2::<f64>::zeros((width, height));

    for i in 1..height {
        arr[[0, i]] = (i as f64) * 2.0;
        arr[[1, i]] = arr[[0, i]] * 2.0; // Getting the double
    }
    arr
}



use risc0_zkvm::guest::env;

risc0_zkvm::guest::entry!(main);

pub fn main() {
    let lenght_data = 12;
    let data = build_data(2, lenght_data);

    let epochs = 15;
    let lr = 0.01;

    let mut m = 0.1;
    let mut b = 0.1;

    for _epoch in 0..epochs {
        for i in 0..lenght_data {
            let x = data[[0, i]];
            let y_target = data[[1, i]];

            let y_hat = m * x + b;

            let error = y_hat - y_target;

            m = m - (lr * error.round() * x);
            b = b - (lr * error.round());
        }
    }

}
