use rand::Rng; // Import the Rng trait
extern crate rand;

fn main() {
    let mut rng = rand::thread_rng(); // Create a mutable reference to a new random number generator


    // Generate a random number between 0 and 10
    let random_numone:i32  = rng.gen_range(0, 11);
    let random_numtwo:i32 = rng.gen_range(0, 11);

    //declaring
    let mut random_string1 = "one";
    let mut random_string2 = "two";

    // declare num two rust or acean
    if random_numone % 2 == 0 {
        // The number is even
        println!("acean  your number is  {}", random_numone);
         random_string1 = "acean";
    } else {
        // The number is odd
        println!("rust  your number is  {}", random_numone);
        random_string1 = "rust";
    }


    // declare num two rust or acean
    if random_numtwo % 2 == 0 {
        // The number is even
        println!("acean  your number is  {}", random_numtwo);
         random_string2 = "acean";
    } else {
        // The number is odd
        println!("rust  your number is  {}", random_numtwo);
         random_string2 = "rust";
    }

    let rustacean = format!("{}{}", random_string1, random_string2);

    println!(" you got a {}", rustacean);

    if rustacean == "rustacean" {
        println!("wow, a rustacean! your numbers were  {}  {}", random_numone, random_numtwo);
    } else {
        println!(
            "no rustacean for you, your numbers were {}  {}",
            random_numone, random_numtwo
        )
    }
}
