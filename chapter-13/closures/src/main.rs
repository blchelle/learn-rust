use std::thread;
use std::time::Duration;

struct Cacher<T>
where
    T: Fn(u32) -> u32,
{
    calculation: T,
    value: Option<u32>,
}

impl<T> Cacher<T>
where
    T: Fn(u32) -> u32,
{
    fn new(calculation: T) -> Cacher<T> {
        Cacher {
            calculation,
            value: None,
        }
    }

    fn value(&mut self, arg: u32) -> u32 {
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

fn expensive_calculation(intensity: u32) -> u32 {
    println!("===== Expensive Calculation =====");

    println!("Slowly calculating...");
    thread::sleep(Duration::from_secs(2));
    intensity
}

fn generate_workout(intensity: u32, random_number: u32) {
    println!("===== Generate Workout =====");

    let mut expensive_closure = Cacher::new(|num| {
        println!("===== Expensive Calculation =====");
        println!("Slowly calculating...");
        thread::sleep(Duration::from_secs(2));
        num
    });

    if intensity < 25 {
        println!(
            "Today, do {} pushups!\n",
            expensive_closure.value(intensity)
        );
        println!("Next, do {} situps!\n", expensive_closure.value(intensity));
    } else {
        if random_number == 3 {
            println!("Take a break today! Remember to stay hydrated!");
        } else {
            println!(
                "Today, run for {} minutes!\n",
                expensive_closure.value(intensity)
            );
        }
    }
}

fn closure_env_scope() {
    println!("===== Closure Environment Scope =====");

    let x = 7;
    let is_equal = move |z| z == x;
    let are_they_equal = is_equal(5);

    println!("Are they equal: {}", are_they_equal);
}

fn main() {
    let simulated_user_specified_value = 10;
    let simulated_random_number = 7;

    generate_workout(simulated_user_specified_value, simulated_random_number);
    println!();

    closure_env_scope();
    println!();
}
