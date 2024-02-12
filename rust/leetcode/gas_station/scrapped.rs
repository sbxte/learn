const LEN: usize = 5;
const OFFSET: usize = 4; // MUST NOT BE MORE THAN LEN

fn main() {
    assert!(OFFSET < LEN);

    let gas = [1, 2, 3, 4, 5];
    // let cost = [3, 4, 5, 1, 2];

    assert_eq!(gas.len(), LEN);
    // assert_eq!(cost.len(), LEN);

    // Delta per station
    let mut delta = [0; LEN];
    for i in 0..delta.len() {
        // delta[i] = gas[i] - cost[i];
        delta[i] = gas[i] - gas[(i + OFFSET) % LEN];
    }

    let mut totals = vec![vec![0; LEN + 1]; LEN];

    for t in 1..delta.len() {
        for d in 0..delta.len() {
            if d >= t {
                totals[t][d] += totals[t-1][d] + delta[(d + t - 1) % delta.len()];
            }
        }
    }
    for t in 1..delta.len() {
        for d in 0..delta.len() {
            if d < t {
                totals[t][d] += totals[t-1][d] + delta[(d + t - 1) % delta.len()];
            }
        }
    }
    // O(n^2) run time. HORRIBLE
    // how to get less AAAAAAAAAAAAAAAAA

    println!("Totals: {:#?}", totals);
}
