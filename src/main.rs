use intspan;
use intspan::IntSpan;

fn main() {
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

//    intspan.add_n(9);
//    intspan.add_vec(&vec![12, 16, 15, 15, 20]);
//    println!("{}", intspan);
//    println!("{:?}", intspan.ranges());
//
//    intspan.add_runlist(&"-14-2,14-18".to_string());
//    println!("{}", intspan);
}
