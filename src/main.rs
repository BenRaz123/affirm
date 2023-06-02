use clap::Parser;

#[derive(Parser)]
#[command(name = "affirm")]
#[command(author = "Ben R. <Ben.Raz2008@gmail.com>")]
#[command(version = "1.0")]
#[command(about = "repeats what you say", long_about = None)]
struct Args {
    #[arg(
        help = "If no input is supplied, 'y' will be printed, just like the original yes command"
    )]
    input: Option<String>,
    #[arg(help = "(milliseconds) the amount of time between printing the input")]
    #[arg(short, long)]
    timing: Option<u64>,
}

fn sleep_for(milli_seconds: &u64) -> () {
    std::thread::sleep(std::time::Duration::from_millis(*milli_seconds));
}

fn main() {
    let arguments = Args::parse();

    let input: String = match arguments.input {
        Some(input) => input,
        None => "y".to_owned(),
    };

    let delay: u64 = match arguments.timing {
        Some(delay) => delay,
        None => 0,
    };

    loop {
        println!("{}", input);
        sleep_for(&delay);
    }
}
