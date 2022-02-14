use std::thread;
use std::time::Duration;

/* Below is simulated into a closure within the generate workout fun
fn simulated_expensive_calculation(intensity: u32) -> u32 {
    println!("calculating slowly...");
    thread::sleep(Duration::from_secs(2));
    intensity
}
*/

fn generate_workout(intensity: u32, random_number: u32){
    let expensive_result = |num| {
        println!("calculating slowly...");
        thread::sleep(Duration::from_secs(2));
        num
    };
    if intensity < 25 {
        println!(
            "Today do {} pushups",
            expensive_result(intensity)        
        );
        println!(
            "next do {} situps",
            expensive_result(intensity)
        );
    } else {
        if random_number == 3 {
            println!("Take a break today! Hydrate.");
        } else { 
        println!("run for {} minutes",
        expensive_result(intensity)
        );
        }
    };

}
fn main() {
    let simulated_user_specified_value = 10;
    let simulated_random_number = 7;
    generate_workout(
        simulated_user_specified_value,
        simulated_random_number
    );
}
