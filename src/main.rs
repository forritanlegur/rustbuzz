use rand::Rng; // importing the Rng trait for number gen
extern crate rand;

fn main() {
    let mut rng = rand::thread_rng(); // create a mutable reference to a new random number generator to get odd or even number


    // generate a random number between 0 and 10.. you can change this if you want a bigger or smaller range
    let random_numone:i32  = rng.gen_range(0, 11);
    let random_numtwo:i32 = rng.gen_range(0, 11);

    //declaring
    let mut random_string1 = "one";
    let mut random_string2 = "two";

    // declare num one rust or acean
    if random_numone % 2 == 0 {
        // the number is even
        println!("acean  your number is  {}", random_numone);
         random_string1 = "acean";
    } else {
        // the number is odd
        println!("rust  your number is  {}", random_numone);
        random_string1 = "rust";
    }


    // declare num two rust or acean
    if random_numtwo % 2 == 0 {
        // the number is even
        println!("acean  your number is  {}", random_numtwo);
         random_string2 = "acean";
    } else {
        // the number is odd
        println!("rust  your number is  {}", random_numtwo);
         random_string2 = "rust";
    }

    //making string from the random numbers
    let rustacean = format!("{}{}", random_string1, random_string2);

    println!(" you got a {}", rustacean);

    //checking if the string is rustacean 
    if rustacean == "rustacean" {
        println!("wow, a rustacean! your numbers were  {}  {}", random_numone, random_numtwo);
    } else {
        println!(
            "no rustacean for you, your numbers were {}  {}",
            random_numone, random_numtwo
        )
    }
}
