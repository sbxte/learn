use once_cell::sync::Lazy;

static mut PELL_VALUES: Lazy<Vec<u128>> = Lazy::new(|| vec![0, 1]);

fn pell(n: u32) -> u128 {
    unsafe { PELL_VALUES.get(n as usize) }
        // unsafe { PELL_VALUES }
        //     .get(n as usize)
        .map(|v| *v)
        .unwrap_or_else(|| {
            let val: u128 = 2 * pell(n - 1) + pell(n - 2);
            unsafe { PELL_VALUES.push(val) }
            val
        })
}

fn pell_recurse(n: u32) -> u128 {
    if n <= 1 {
        return n as u128;
    }
    pell_recurse(n - 2) + 2 * pell_recurse(n - 1)
}

fn main() {
    for i in 0..10 {
        println!("Pell {}", pell(i),);
    }
}
