use std::collections::HashSet;
use subset_generator::SubsetGenerator;

fn main() {
    let universe = 5;
    let families = vec![
        vec![4],
        vec![0, 1, 2],
        vec![1, 3],
        vec![2, 4],
        vec![0, 3, 4],
    ];

    let mut opt = std::usize::MAX;

    let sg = SubsetGenerator::new(&families, false);
    for subset in sg.iter() {

        // Compute the union of all the selected families
        let mut result = HashSet::new();
        for family in subset.iter() {
            for element in *family {
                result.insert(*element);
            }
        }

        // verify whether its a solution, and whether its a better solution
        if result.len() == universe && subset.len() <= opt {
            opt = subset.len();
        }
    }
    println!("{:}", opt);
}
