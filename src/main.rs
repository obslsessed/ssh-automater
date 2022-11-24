use clap::Parser;
use std::process::Command;
use tokio::time::{sleep, Duration};
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

#[tokio::main]
async fn main() {
    let mut handles = vec![];
    for addr in 2..=254 {
        let handle = tokio::spawn(async move {
            if let Ok(success) = run_ssh(&addr).await {
                println!("success: {}", success)
            };
        });
        handles.push(handle);
        sleep(Duration::from_millis(250)).await;
    }
    for handle in handles {
        handle.await.unwrap();
    }
}

fn zero_or_nah(zero: &bool) -> char {
    match zero {
        false => '1',
        true => '0',
    }
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

async fn run_ssh(addr: &u8) -> Result<String, ()> {
    let args = Args::parse();
    let user = get_user(args.user);
    let third = zero_or_nah(&args.zero);
    if Command::new("ssh")
        .arg(format!("-p {}", args.port))
        .arg(format!("{}@192.168.{}.{}", user, third, addr))
        .status()
        .unwrap()
        .code()
        .unwrap()
        == 0
    {
        return Ok(format!(
            "ssh -p {} {}@192.168.{}.{}",
            args.port, user, third, addr
        ));
    }
    return Err(());
}
