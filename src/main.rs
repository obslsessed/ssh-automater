use clap::Parser;
use std::process::Command;
use users::get_current_username;

/// a (bad) thing to go through (local) ip addresses and (try to) ssh into them.
/// basically, imagine if you could run "ssh 192.168.1.*".
#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// port
    #[arg(short, long, default_value_t = 22)]
    port: u32,

    /// user
    #[arg(short, long)]
    user: Option<String>,

    /// 192.168.0.* instead of 192.168.1.* // i don't know what the difference is
    #[arg(short = '0', long, default_value_t = false)]
    zero: bool,
}

fn main() {
    let args = Args::parse();
    match run_ssh(args.port, get_user(args.user), zero_or_nah(args.zero)) {
        Ok(success) => println!("success: {}", success),
        Err(()) => println!("failure: none found"),
    };
}

fn get_user(name: Option<String>) -> String {
    match name {
        Some(name) => return name,
        None => {
            return String::from(
                get_current_username()
                    .unwrap()
                    .to_string_lossy()
                    .to_string(),
            );
        }
    }
}

fn zero_or_nah(zero: bool) -> char {
    match zero {
        false => '1',
        true => '0',
    }
}

fn run_ssh(port: u32, user: String, third: char) -> Result<String, ()> {
    println!("ssh -p {} {}@192.168.{}.*", port, user, third);
    if third == '1' {
        println!("if it stalls, try running with -0")
    } else {
        println!("if it stalls, try running without -0")
    }
    for addr in 2..=254 {
        if Command::new("ssh")
            .arg(format!("-p {}", port))
            .arg(format!("{}@192.168.{}.{}", user, third, addr))
            .status()
            .unwrap()
            .code()
            .unwrap()
            == 0
        {
            return Ok(format!(
                "ssh -p {} {}@192.168.{}.{}",
                port, user, third, addr
            ));
        }
    }
    return Err(());
}
