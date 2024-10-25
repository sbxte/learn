fn gen_log_facs(n: usize) -> Vec<f64> {
    let mut facs = Vec::with_capacity(n + 1);
    let mut digits = 0.0;
    facs.push(0.0); // zero factorial is 1
    for i in 1..=n {
        digits += (i as f64).log10();
        facs.push(digits);
    }
    facs
}

fn log_choose(log_fac: &[f64], n: usize, k: usize) -> f64 {
    if k > n {
        panic!("k > n");
    }
    if k == n {
        return 0.0;
    }
    log_fac[n] - log_fac[k] - log_fac[n - k]
}

/// inclusive low high
fn bsearch_lbound<F, N>(low: usize, high: usize, value: F, min: N) -> usize
where
    F: Fn(usize) -> N,
    N: PartialOrd,
{
    let mut l = low;
    let mut h = high;
    let mut mid = (l + h) >> 1;
    while l < mid {
        if value(mid) > min {
            h = mid;
        } else {
            l = mid;
        }
        mid = (l + h) >> 1;
    }
    mid
}

fn main() {
    // Generate log facs
    let max_n_thresh = 100;
    let min_digit_thresh = 6.0;
    let log_fac = gen_log_facs(max_n_thresh);

    println!("{:?}", log_fac);

    let mut total_values = 0;
    // Iterate choose n for some k
    for n in 1..=max_n_thresh {
        // k being the mid of n maximizes the choose function
        let max_d_i = n >> 1;
        let max_d = log_choose(&log_fac, n, max_d_i);
        if max_d < min_digit_thresh {
            continue;
        }
        println!("{}", n);
        println!("{}: {}", max_d_i, max_d);

        // Binary search the k lower bound
        // + 1 to get the inclusive lower bound because the bsearch
        // finds the index where its less than the threshold (outside the bound)
        let min_d_i =
            bsearch_lbound(1, max_d_i, |i| log_choose(&log_fac, n, i), min_digit_thresh) + 1;
        println!("{}: {}", min_d_i, log_choose(&log_fac, n, min_d_i));

        // Choose n k configurations that fit in the min digit threshold
        let half_values = max_d_i - min_d_i + 1;
        let values = if n % 2 == 0 {
            (half_values << 1) - 1
        } else {
            half_values << 1
        };
        total_values += values;
        println!("{}, {}", values, total_values);
    }

    println!("Finale: {}", total_values);
}
