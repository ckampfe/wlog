use anyhow::Result;
use log::{error, info};
use std::fs::OpenOptions;
use std::path::PathBuf;
use std::process::Command;
use std::time::Duration;
use structopt::StructOpt;

const WLOG_VERSION: &str = env!("WLOG_VERSION");

#[derive(Clone, Debug, StructOpt)]
#[structopt(name = "wlog", version = crate::WLOG_VERSION)]
struct Options {
    /// Time in ms between two ticks
    #[structopt(short = "n", long = "interval", default_value = "2")]
    seconds: u64,
    /// Suppress terminal output. Has no effect on file output
    #[structopt(short, long)]
    quiet: bool,
    /// Log output to a file
    #[structopt(short = "f", long = "file")]
    output: Option<PathBuf>,
    /// The command to repeat
    #[structopt()]
    command: String,
}

fn main() -> Result<()> {
    let options: Options = Options::from_args();
    let tick_rate = Duration::from_secs(options.seconds);
    let mut log_options: Vec<Box<dyn simplelog::SharedLogger>> = vec![];

    if !options.quiet {
        log_options.push(simplelog::TermLogger::new(
            simplelog::LevelFilter::Info,
            simplelog::Config::default(),
            simplelog::TerminalMode::Mixed,
        ));
    }

    if let Some(log_file) = &options.output {
        let file = OpenOptions::new().append(true).open(log_file)?;

        log_options.push(simplelog::WriteLogger::new(
            simplelog::LevelFilter::Info,
            simplelog::Config::default(),
            file,
        ))
    }

    simplelog::CombinedLogger::init(log_options).expect("Could not initialize logger");

    let mut app = App::new(options)?;

    loop {
        app.execute_command()?;
        std::thread::sleep(tick_rate);
    }
}

#[derive(Clone, Debug)]
struct App {
    command_name: String,
    args: Vec<String>,
    command_text: String,
}

impl App {
    fn new(options: crate::Options) -> Result<Self> {
        let command_text = options.command.clone();
        let split = shell_words::split(&options.command)?;
        let command_name = split[0].clone();

        Ok(App {
            command_name,
            args: (&split[1..]).to_vec(),
            command_text,
        })
    }

    fn execute_command(&mut self) -> Result<()> {
        let mut command = Command::new(&self.command_name);
        let process = command.args(&self.args).output()?;

        if process.status.success() {
            info!("{}", String::from_utf8(process.stdout).unwrap());
        } else {
            error!("{}", String::from_utf8(process.stderr).unwrap());
        }

        Ok(())
    }
}
