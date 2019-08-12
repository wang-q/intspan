use intspan;
use intspan::IntSpan;
use std::env;
use std::time::Instant;

fn run_test() {
    let mut intspan = IntSpan::new();
    intspan.add_pair(1, 9);
    intspan.add_pair(20, 39);

    println!("{}", intspan);
    println!("is_empty {}", intspan.is_empty());
    println!("edge_size {}", intspan.edge_size());
    println!("ranges {:?}", intspan.ranges());
    println!("cardinality {}", intspan.cardinality());

    for n in vec![-5, 29, 40] {
        println!("val {} is contained {}", n, intspan.contains(n));
    }

    intspan.add_range(&vec![60, 70, 80, 90]);
    println!("{}", intspan);

    intspan.add_range(&vec![68, 75]);
    println!("{}", intspan);

    intspan.add_n(99);
    println!("{}", intspan);

    intspan.add_vec(&vec![77, 79]);
    println!("{}", intspan);

    intspan.invert();
    println!("{}", intspan);

    intspan.invert();
    println!("{}", intspan);

    intspan.remove_pair(66, 71);
    println!("{}", intspan);

    intspan.remove_n(85);
    println!("{}", intspan);

    intspan.remove_vec(&vec![87, 88]);
    println!("{}", intspan);

    intspan.add_runlist("-30--10");
    println!("{}", intspan);

    intspan.remove_runlist("62-78".to_string());
    println!("{}", intspan);

    let mut other = IntSpan::new();
    other.add_runlist(&"-15-5".to_string());
    println!("{}", other);

    intspan.merge(&other);
    println!("{}", intspan);

    other.clear();
    println!("{}", other);
    other.add_runlist("-20--5");
    println!("{}", other);
    intspan.subtract(&other);
    println!("{}", intspan);

    //    -30--21,-4-9,20-39,60-61,79-84,86,89-90,99'
}

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
        let vec2 = vec![100, 1000000];

        for _i in 1..50001 {
            let mut set = IntSpan::new();

            if step >= 2 {
                set.add_range(&vec1);
            }
            if step >= 3 {
                set.add_range(&vec2);
            }
            if step >= 4 {
                set.to_string();
            }
            if step >= 5 {
                for j in 1..201 {
                    set.add_pair(j, j);
                }
            }
            if step >= 6 {
                for j in 1..201 {
                    set.add_pair(j * 5, j * 10);
                }
            }
        }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    println!("{:?}", args);

    if args.len() <= 1 {
        eprintln!("{} <test|benchmark|file>", &args[0]);
    } else if &args[1] == "test" {
        run_test();
    } else if &args[1] == "benchmark" {
        run_benchmark();
    } else {
        eprintln!("Unrecognized command {}", &args[1]);
    }
}
