pub mod days;
pub mod solution;
pub mod utils;

use solution::{print_table, Solution};

fn main() {
    let solutions = solution::all_solutions();
    let mut all_results = Vec::new();
    for Solution { input, func, .. } in solutions {
        let results = (func)(input);
        all_results.push(results);
    }

    print_table(all_results);
}
