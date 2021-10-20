use rand::thread_rng;
use rand::Rng;
use std::collections::HashMap;

// Given a list of integers, use a vector and return the mean (the average value), median (when sorted, the value in the middle position), and mode (the value that occurs most often; a hash map will be helpful here) of the list.

fn get_random_list(len: usize) -> Vec<usize> {
    let mut rng = thread_rng();
    let mut list = Vec::new();
    while list.len() < len {
        let x: usize = rng.gen_range(0..10);
        list.push(x);
    }

    list
}

fn get_mean(list: &Vec<usize>) -> f32 {
    let mut sum = 0;
    let len = list.len();

    for n in list {
        sum += n
    }

    sum as f32 / len as f32
}

fn get_median(list: &Vec<usize>) -> f32 {
    let mut sorted_list = list.clone();
    sorted_list.sort();

    let midpoint = sorted_list.len() / 2;
    (match sorted_list.len() % 2 {
        0 => (sorted_list[midpoint] + sorted_list[midpoint + 1]) / 2,
        1 => sorted_list[midpoint],
        _ => 0,
    }) as f32
}

fn get_mode(list: &Vec<usize>) -> usize {
    let mut map = HashMap::new();

    for n in list {
        let count = map.get(n).unwrap_or(&0) + 1;
        map.insert(n, count);
    }

    println!("{:?}", map);

    let (mut n, mut count) = (0, 0);
    for (&k, v) in map {
        if v > count {
            count = v;
            n = k;
        }
    }
    
    n
}

fn main() {
    let list = get_random_list(20);
    println!("{:?}", list);

    let mean = get_mean(&list);
    println!("mean: {:.3}", mean);

    let median = get_median(&list);
    println!("median: {:.3}", median);

    let mode = get_mode(&list);
    println!("median: {:.3}", mode);
}
