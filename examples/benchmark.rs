use intspan;
use intspan::IntSpan;
use std::env;
use std::time::Instant;

fn run_benchmark() {
    for step in 2..7 {
        println!("step {}", step);
        let start = Instant::now();

        test_add_range(step);

        let elapsed = start.elapsed();
        println!(
            "duration: {} s",
            (elapsed.as_nanos() as f64) / 1000.0 / 1000.0 / 1000.0
        );
    }

    fn test_add_range(step: i32) {
        let vec1 = vec![
            1, 30, 32, 149, 153, 155, 159, 247, 250, 250, 253, 464, 516, 518, 520, 523, 582, 585,
            595, 600, 622, 1679,
        ];
        let vec2 = vec![100, 1_000_000];

        for _i in 1..=50000 {
            let mut set = IntSpan::new();

            if step >= 2 {
                set.add_ranges(&vec1);
            }
            if step >= 3 {
                set.add_ranges(&vec2);
            }
            if step >= 4 {
                set.to_string();
            }
            if step >= 5 {
                for j in 1..=200 {
                    set.add_pair(j, j);
                }
            }
            if step >= 6 {
                for j in 1..=200 {
                    set.add_pair(j * 5, j * 10);
                }
            }
        }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    println!("{:?}", args);

    run_benchmark();
}
