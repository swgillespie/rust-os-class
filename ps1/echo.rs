use std::os;

fn main() {
    let args: ~[~str] = os::args();
    for argument in args.slice(1, args.len()).iter() {
        // argument is a owned pointer to a string
        print(*argument + " ");
    }
}