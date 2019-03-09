use std::thread;
use std::time::Duration;
use std::collections::HashMap;

struct Cacher<T, U, R> 
    where T: Fn(U) -> R,
        U: std::cmp::Eq + std::hash::Hash + Copy, 
        R: Copy
{
    calculation: T,
    value: HashMap<U, R>,
}

impl<T, U ,R> Cacher<T, U, R> 
    where T: Fn(U) -> R, 
        U: std::cmp::Eq + std::hash::Hash + Copy, 
        R: Copy
{
    fn new(calculation: T) -> Cacher<T, U, R> {
        Cacher {
            calculation,
            value: HashMap::new(),
        }
    }

    fn value(&mut self, arg: U) -> R {
        match self.value.get(&arg) {
            Some(x) => {
                *x
            },
            None => {
                let x = (self.calculation)(arg);
                self.value.insert(arg, x);
                x
            },  
        }
    }
}

fn generate_workout(intensety: u32, random_number: u32) {
    let mut expensive_result = Cacher::new(|num| {
        println!("calculating slowly...");
        thread::sleep(Duration::from_secs(2));
        num
    });
    if intensety < 25 {
        println!(
            "Today, do {} pushups!",
            expensive_result.value(intensety)
        );
        println!(
            "Next, do {} situps!",
            expensive_result.value(intensety)
        );
    } else {
        if random_number == 3 {
            println!("Take a break today.");
        } else {
            println!(
                "Today run for {} minutes!",
                expensive_result.value(intensety)
            );
        }
    }
}

fn main() {
    let simulated_user_specified_value = 10;
    let simulated_random_number = 7;

    generate_workout(
        simulated_user_specified_value,
        simulated_random_number
    );
}


#[test]
fn call_with_different_values() {
    let mut c = Cacher::new(|a| a);

    let v1 = c.value(1);
    let v2 = c.value(2);

    assert_eq!(v1, 1);
    assert_eq!(v2, 2);
    assert_eq!(v1, 1);
}

#[test]
fn call_with_different_type() {
    let mut str_c = Cacher::new(|a| a);

    let v1 = str_c.value("hello");
    assert_eq!(v1, "hello");
}
