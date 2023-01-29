mod day_01;
mod day_02;
mod day_03;

use std::collections::HashSet;
use day_01::day_01;
use day_02::day_02;
use day_03::day_03;

 fn main() {
    let mut days:HashSet<i32> = HashSet::new();
    days.insert(1);
    days.insert(2);
    days.insert(3);

    if days.contains(&1) {
        day_01();
    }
    if days.contains(&2) {
        day_02();
    }
    if days.contains(&3) {
        day_03();
    }
}
