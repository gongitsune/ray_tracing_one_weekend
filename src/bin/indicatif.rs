use std::{thread::sleep, time::Duration};

use indicatif::ParallelProgressIterator;
use rayon::prelude::{IntoParallelIterator, ParallelIterator};

fn main() {
    let v: Vec<_> = (0..100).collect();
    let v2: Vec<_> = v
        .into_par_iter()
        .progress_count(100)
        .map(|i| {
            sleep(Duration::from_millis(100));
            i + 1
        })
        .collect();
    assert_eq!(v2[0], 1);
}
