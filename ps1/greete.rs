use std::os;

fn main() {
    let args: ~[~str] = os::args();
    if args.len() != 2 {
        fail!("Tell me in one word how you feel!");
    }
    match args[1] {
        ~"good" => println("I am so glad you are feeling good!"),
        ~"bad"  => println("I hope you feel better!"),
        ~"iffy" => println("I'm sorry to hear that you are iffy!"),
        _      => println("I'm not sure what that feeling is but I hope it's good!")
    }
}