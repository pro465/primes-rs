use std::env;
use std::time::Instant;

use primes::Prime;

fn main() {
    let num: usize = env::args()
        .nth(1)
        .expect("expeected atleast 1 number as argument")
        .parse()
        .expect("expected number");

    let second_arg = env::args().nth(2);
    let second_arg = match second_arg {
        Some(x) if x == "true" => true,
        _ => false,
    };

    if second_arg {
        let mut sum = 0;
        for _ in 0..10 {
            let timer = Instant::now();
            let mut num_times = 0;
            while (Instant::now() - timer).as_secs_f32() < 5. {
                let mut primes = Prime::new(num);
                primes.seive();
                num_times += 1;
            }
            println!("{}", num_times);
            sum += num_times;
        }

        println!("avg: {}", sum / 10);
    } else {
        let mut primes = Prime::new(num);
        let mut num_primes = 0;
        primes.seive();
        for i in (0..=num - 2).filter(|x| primes.get(*x)) {
            print!("{}\n", i + 2);
            num_primes += 1
        }

        println!("{} primes found", num_primes)
    }
}
