mod day_01;
mod day_02;

use std::collections::HashSet;
use day_01::day_01;
use day_02::day_02;

 fn main() {
    let mut days:HashSet<i32> = HashSet::new();
    // days.insert(1);
    days.insert(2);

    if days.contains(&1) {
        day_01();
    }
    if days.contains(&2) {
        day_02();
    }
}
