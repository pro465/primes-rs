use crate::Prime;

#[test]
fn t() {
    let mut prime = Prime::new(100_000_000);
    prime.seive();
    let map = [
        (10, 4),
        (100, 25),
        (1_000, 168),
        (10_000, 1_229),
        (100_000, 9_592),
        (1_000_000, 78_498),
        (10_000_000, 664_579),
        (100_000_000, 5_761_455),
    ];
    let mut n = 0;
    for i in 2..100_000_000 {
        if prime.get(i) {
            n += 1;
        }
        if let Some((_, n_primes)) = map.into_iter().find(|x| x.0 == i + 1) {
            assert_eq!(n, *n_primes);
        }
    }
}
