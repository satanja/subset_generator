use subset_generator::SubsetGenerator;

fn main() {
    let set = vec![3, 34, 4, 12, 5, 2];
    let target = 9;

    let sg = SubsetGenerator::new(&set, false);
    let mut found = false;
    for subset in sg.into_iter() {
        let sum = subset.into_iter().fold(0, |acc, i| acc + *i);
        if sum == target {
            found = true;
            break;
        }
    }

    println!("{:}", found);
}