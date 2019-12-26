use std::thread;
use std::time::Duration;
use std::marker::PhantomData;

fn main() {
    let simulated_user_specified_value = 10;
    let simulated_random_number = 7;

    generate_workout(
        simulated_user_specified_value,
        simulated_random_number,
    );
}

struct Cacher<TFn, TIn, TOut>
    where TFn: Fn(TIn) -> TOut,
          TOut: Copy,
{
    calculation: TFn,
    value: Option<TOut>,
    phantom: PhantomData<TIn>,
}

impl<TFn, TIn, TOut> Cacher<TFn, TIn, TOut>
    where TFn: Fn(TIn) -> TOut,
          TOut: Copy,
{
    fn new(calculation: TFn) -> Cacher<TFn, TIn, TOut> {
        Cacher {
            calculation,
            value: None,
            phantom: PhantomData
        }
    }

    fn value(&mut self, arg: TIn) -> TOut {
        match self.value {
            Some(v) => v,
            None => {
                let v = (self.calculation)(arg);
                self.value = Some(v);
                v
            }
        }
    }
}

fn generate_workout(intensity: u32, random_number: u32) {
    let mut expensive_result = Cacher::new(|num| {
        println!("calculating slowly...");
        thread::sleep(Duration::from_secs(2));
        num
    });
    if intensity < 25 {
        println!("Today, do {} pushups!", expensive_result.value(intensity));
        println!("Next, do {} situps!", expensive_result.value(intensity));
    } else {
        if random_number == 3 {
            println!("Take a break today! Remember to stay hydrated!");
        } else {
            println!("Today, run for {} minutes!", expensive_result.value(intensity));
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn call_with_different_values() {
        let mut c = Cacher::new(|a| a);

        let _v1 = c.value(1);
        let v2 = c.value(2);

        assert_eq!(v2.clone(), 2);
    }
}
