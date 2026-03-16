#![no_main]
ziskos::entrypoint!(main);

use fibonacci_lib::FibResult;
use ziskos::io;

fn main() {
    let n: u32 = io::read();

    let mut a: u64 = 0;
    let mut b: u64 = 1;
    for _ in 0..n {
        let temp = b;
        b = a + b;
        a = temp;
    }

    io::commit(&FibResult { n, value: a });
}
