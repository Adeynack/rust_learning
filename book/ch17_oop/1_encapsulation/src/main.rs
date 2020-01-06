use std::f64::NAN;

//  public struct
#[derive(Debug)]
pub struct AveragedCollection {
    // private fields
    list: Vec<i32>,
    average: f64,
}

impl AveragedCollection {
    pub fn new() -> AveragedCollection {
        AveragedCollection {
            list: vec![],
            average: NAN
        }
    }

    pub fn add(&mut self, value: i32) {
        self.list.push(value);
        self.update_average();
    }

    pub fn remove(&mut self) -> Option<i32> {
        match self.list.pop() {
            None => None,
            Some(value) => {
                self.update_average();
                Some(value)
            },
        }
    }

    pub fn len(&self) -> usize {
        self.list.len()
    }

    pub fn average(&self) -> f64 {
        self.average
    }

    fn update_average(&mut self) {
        let total: i32 = self.list.iter().sum();
        self.average = total as f64 / self.list.len() as f64;
    }
}

fn main() {
    let mut c = AveragedCollection::new();
    println!("after creation: {:?}", c);

    c.add(4);
    println!("after adding 4: {:?}", c);

    c.add(4);
    println!("after adding 4: {:?}", c);

    c.add(5);
    println!("after adding 5: {:?}", c);

    c.add(6);
    println!("after adding 6: {:?}", c);

    for _ in 0..(c.len() + 2) {
        let removed = c.remove();
        println!("after removing {:?}: {:?}", removed, c);
    }
}
