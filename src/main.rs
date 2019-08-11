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

    intspan.remove_pair(66, 71);
    println!("{}", intspan);

    intspan.remove_n(85);
    println!("{}", intspan);

    intspan.remove_vec(&vec![87, 88]);
    println!("{}", intspan);

    intspan.add_runlist(&"-30--10".to_string());
    println!("{}", intspan);

    intspan.remove_runlist(&"62-78".to_string());
    println!("{}", intspan);

    let mut other = IntSpan::new();
    other.add_runlist(&"-15-5".to_string());
    println!("{}", other);

    intspan.merge(&other);
    println!("{}", intspan);

    other.clear();
    println!("{}", other);
    other.add_runlist(&"-20--5".to_string());
    println!("{}", other);
    intspan.subtract(&other);
    println!("{}", intspan);

    //    -30--21,-4-9,20-39,60-61,79-84,86,89-90,99'
}
