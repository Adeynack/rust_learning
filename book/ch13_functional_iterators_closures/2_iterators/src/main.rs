fn main() {
    iterator_use_with_for();
    iterator_use_with_next();
    iterator_consumed_by_sum();
    iterator_adapted_by_map_and_consumed_by_collect();
    filters_by_size();
    implement_our_own_iterator();
    using_other_iterator_trait_methods();
}

fn iterator_use_with_for() {
    let v1 = vec![1, 2, 3];
    let v1_iter = v1.iter();
    for val in v1_iter {
        println!("Got: {}", val);
    }
}

fn iterator_use_with_next() {
    let v1 = vec![1, 2, 3];

    let mut v1_iter = v1.iter();

    assert_eq!(v1_iter.next(), Some(&1));
    assert_eq!(v1_iter.next(), Some(&2));
    assert_eq!(v1_iter.next(), Some(&3));
    assert_eq!(v1_iter.next(), None);
    assert_eq!(v1_iter.next(), None);
    assert_eq!(v1_iter.next(), None);
    assert_eq!(v1_iter.next(), None);
}

fn iterator_consumed_by_sum() {
    let v1 = vec![1, 2, 3];
    let v1_iter = v1.iter();
    let total: i32 = v1_iter.sum();
    assert_eq!(total, 6);
}

fn iterator_adapted_by_map_and_consumed_by_collect() {
    let v1 = vec![1, 2, 3];
    let v1_mapped = v1.iter().map(|x| x + 1);
    let v1_collected: Vec<_> = v1_mapped.collect();
    assert_eq!(v1_collected, vec![2, 3, 4]);
}

#[derive(PartialEq, Debug)]
struct Shoe {
    size: u32,
    style: String,
}

fn shoes_in_my_size(shows: Vec<Shoe>, show_size: u32) -> Vec<Shoe> {
    shows.into_iter() // moves the values out of the original iterated collection
        .filter(|s| s.size == show_size)
        .collect()
}

fn filters_by_size() {
    let shoes = vec![
        Shoe { size: 10, style: String::from("sneaker") },
        Shoe { size: 13, style: String::from("sandal") },
        Shoe { size: 10, style: String::from("boot") },
    ];

    let in_my_size = shoes_in_my_size(shoes, 10);

    assert_eq!(
        in_my_size,
        vec![
            Shoe { size: 10, style: String::from("sneaker") },
            Shoe { size: 10, style: String::from("boot") },
        ]
    );
}

struct Counter {
    max: u32,
    count: u32,
}

impl Counter {
    fn new(max: u32) -> Counter {
        Counter {
            max,
            count: 0,
        }
    }
}

impl Iterator for Counter {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        self.count += 1;

        if self.count <= self.max {
            Some(self.count)
        } else {
            None
        }
    }
}

fn implement_our_own_iterator() {
    let mut counter = Counter::new(3);

    assert_eq!(counter.next(), Some(1));
    assert_eq!(counter.next(), Some(2));
    assert_eq!(counter.next(), Some(3));
    assert_eq!(counter.next(), None);
    assert_eq!(counter.next(), None);
    assert_eq!(counter.next(), None);
    assert_eq!(counter.next(), None);
}

fn using_other_iterator_trait_methods() {
    let sum: u32 = Counter::new(5)
        .zip(Counter::new(5).skip(1))
        .map(|(a, b)| a * b)
        .filter(|x| x % 3 == 0)
        .sum();
    assert_eq!(18, sum);
}
