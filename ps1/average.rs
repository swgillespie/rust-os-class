use std::os;

fn main() {
    let mut args: ~[~str] = os::args();
    if args.len() == 1 {
        fail!("Usage: average [values]");
    }
    let mut aggregator = 0f64;
    let mut total = 0f64;
    args.remove(0);
    for argument in args.iter() {
        match from_str::<f64>(*argument) {
            Some(number) => { 
                aggregator += number; 
                total += 1f64; 
            }, 
            None => println("not a number: " + *argument)
        }
    }
    println("Average: " + (aggregator / total).to_str());
}