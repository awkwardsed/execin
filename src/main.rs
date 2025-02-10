use std::env;
use std::thread;
use std::time::Duration;
use std::process::Command;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 3 {
        println!("Usage: {} <command> <duration_in_seconds>", args[0]);
        return;
    }

    let command = &args[1];
    let duration = match args[2].parse::<u64>() {
        Ok(n) => n,
        Err(_) => {
            println!("Please type a number!");
            return;
        }
    };

    loop {
         // on comment this part if you are in linux and comment the one under this
        /*
        let output = Command::new("sh")
            .args(&["-c", command])
            .output()
            .expect("Failed to run command");
        */

        let output = Command::new("cmd")
            .args(&["/C", command])
            .output()
            .expect("Failed to run command");

        let stdout = String::from_utf8_lossy(&output.stdout);
        println!("{}", stdout);

        thread::sleep(Duration::from_secs(duration));
    }
}
