use std::path::posix::Path;
use std::{os, io, str};
use std::run::{Process, ProcessOptions};


pub struct ShellContext {
    cwd: Path,
}

impl ShellContext {
    fn new() -> ~ShellContext {
        ~ShellContext {
            cwd: os::getcwd(),
        }
    }

    fn cd(&mut self, newPath: Path) {
        self.cwd = newPath
    }
}

fn read_eval_print(ctx: &mut ShellContext) {
    let mut stdin = io::stdin();
    let mut stdout = io::stdout();
    let mut running = true;
    while running {
        stdout.write_str(format!("[{}] $ ", ctx.cwd.display()));
        stdout.flush();
        let mut acc: ~[u8] = ~[];
        loop {
            match stdin.read_byte() {
                Some(data) => {
                    if data == '\n' as u8 {
                        break;
                    }
                    acc.push(data);
                },
                None => break,
            }
        }
        running = evaluate(ctx, acc);
    }
}

fn evaluate(ctx: &mut ShellContext, input: &[u8]) -> bool {
    let as_str = str::from_utf8(input);
    let mut arguments: ~[&str] = as_str.split(' ').collect();
    match arguments[0] {
        &"exit" => {
            return false;
        }
        &"cd"   => {
            change_directory(ctx, arguments);
            return true;
        }
        &"pwd"  => {
            println!("{}", ctx.cwd.display());
            return true;
        }
        &""     => {
            return true;
        },
        _       => {
            program_exec(ctx, arguments);
            return true;
        }
    }
}

fn change_directory(ctx: &mut ShellContext, arguments: &[&str]) {
    if arguments.len() == 1 {
        println("Error: no path given");
        return;
    }
    let path = ctx.cwd.join(&Path::new(arguments[1]));
    if !path.exists() {
        println("Error: path does not exist");
        return;
    }
    if !path.is_dir() {
        println!("Error: {} not a directory", arguments[1]);
        return;
    }
    // path exists and is a directory
    // is it absolute or relative?
        if path.is_relative() {
            // if it's relative, we need to make it absolute
            let base_path = Path::new("/");
            let absolute_path = match path.path_relative_from(&base_path) {
                Some(relative_path) => path.join(&relative_path),
                None => {
                    println("Error: failed to find a path from root to chosen dir");
                    return;
                }
            };
            ctx.cd(absolute_path);
        } else {
            // otherwise, it's absolute
            ctx.cd(path);
        }
}

fn program_exec(ctx: &ShellContext, arguments: &mut [&str]) {
    // if this was a real shell, i'd maintain a "path"
    // environment variable that the shell will use
    // to search for executables. I'm gonna cheat
    // and use /usr/bin/which.
    let title: ~str = arguments[0].to_owned();
    let mut _arguments = arguments.slice(1, arguments.len());
    let owned_arguments: ~[~str] = _arguments.iter().map(|x| x.to_owned()).collect();
    let options = ProcessOptions::new();
    let mut subproc = match Process::new("/usr/bin/which", &[title], options) {
        Some(data) => data,
        None => fail!("Process call to /usr/bin/which failed"),
    };
    let mut output = subproc.finish_with_output().output;
    output.pop();
    let prog_name = match output {
        []    => {
            println!("Program not found");
            return;
        },
        _     => str::from_utf8(output),
    };
    let mut real_options = ProcessOptions::new();
    real_options.dir = Some(&ctx.cwd);
    let mut real_proc = match Process::new(prog_name, 
                                           owned_arguments,
                                           real_options) {
        Some(data) => data,
        None => fail!("Process call to {} failed", prog_name),
    };
    let mut real_output = real_proc.finish_with_output().output;
    real_output.pop();
    println(str::from_utf8(real_output));
}

fn main() {
    let mut ctx = ShellContext::new();
    read_eval_print(ctx);
}

