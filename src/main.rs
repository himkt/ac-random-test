use clap::{Parser, ArgAction};
use std::{
    fs::File,
    io::{Error,ErrorKind,Result},
    process::{Command, Stdio},
    time::Instant,
};

#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// Name to be passed to `--run-cmd`
    name: String,

    /// Command or executable file that creates `in.txt`
    #[clap(long, default_value = "./gen.py")]
    gen_input_cmd: String,

    /// Command to run a program
    #[clap(long, default_value = "cargo run --release --bin {argv0}")]
    run_cmd: String,

    /// Command to run a lazy program
    #[clap(long)]
    run_lazy_cmd: Option<String>,

    /// If this option is specified, skip the lazy version
    #[clap(long, action = ArgAction::SetTrue)]
    without_lazy: bool,

    /// Maximum allowed milliseconds for a run
    #[clap(long, default_value_t = f64::INFINITY, value_parser = clap::value_parser!(f64))]
    max_ms: f64,
}

const OKBLUE: &str = "\x1b[94m";
const OKGREEN: &str = "\x1b[92m";
const WARN: &str = "\x1b[1;33m";
const FAIL: &str = "\x1b[91m";
const ENDC: &str = "\x1b[0m";

fn run(command: &[String], input_path: &str) -> Result<(String, f64)> {
    let start = Instant::now();
    let file = File::open(input_path)?;
    let output = Command::new(&command[0])
        .args(&command[1..])
        .stdin(Stdio::from(file))
        .stderr(Stdio::null())
        .output()?;

    if !output.status.success() {
        return Err(Error::new(
            ErrorKind::Other,
            format!(
                "Run: {}failed{} command '{}' failed with exit status: {}",
                FAIL,
                ENDC,
                command.join(" "),
                output.status
            ),
        ));
    }

    let duration = start.elapsed().as_secs_f64() * 1000.0;
    let output_str =
        String::from_utf8(output.stdout).map_err(|e| Error::new(ErrorKind::InvalidData, e))?;
    Ok((output_str.trim().to_string(), duration))
}

fn elapsed_with_color(elapsed: f64) -> String {
    let color = if elapsed >= 2000.0 {
        FAIL
    } else if elapsed >= 1000.0 {
        WARN
    } else {
        OKBLUE
    };
    format!("{}{:.2} ms{}", color, elapsed, ENDC)
}

fn verify(args: &Args) -> Result<()> {
    let name = &args.name;
    let gen_input_cmd = &args.gen_input_cmd;
    let run_cmd_template = &args.run_cmd;
    let max_ms = args.max_ms;
    let without_lazy = args.without_lazy;

    let gen_cmd: Vec<String> = gen_input_cmd.split_whitespace().map(String::from).collect();
    let run_cmd: Vec<String> = run_cmd_template
        .replace("{argv0}", name)
        .split_whitespace()
        .map(String::from)
        .collect();

    run(&gen_cmd, "in.txt")?;
    let (output, elapsed) = run(&run_cmd, "in.txt")?;

    if elapsed >= max_ms {
        println!(
            "Run: {}failed{} (took {}>={:.2} ms)",
            FAIL,
            ENDC,
            elapsed_with_color(elapsed),
            max_ms
        );
        std::process::exit(1);
    }

    if !without_lazy {
        let run_lazy_cmd: Vec<String> = if let Some(run_lazy_cmd_template) = &args.run_lazy_cmd {
            run_lazy_cmd_template
                .replace("{argv0}", &name.to_string())
                .split_whitespace()
                .map(String::from)
                .collect()
        }
        else {
            run_cmd_template
                .replace("{argv0}", &format!("{}_lazy", name))
                .split_whitespace()
                .map(String::from)
                .collect()
        };
        let (lazy_output, _) = run(&run_lazy_cmd, "in.txt")?;

        if lazy_output != output {
            println!("Run: {}failed{} (found the edge case and stored in in.txt. [output: expected={}, got={}])", FAIL, ENDC, OKGREEN.to_owned() + &lazy_output + ENDC, FAIL.to_owned() + &output + ENDC);
            std::process::exit(1);
        }
    }

    println!(
        "Run: {}passed{} ({})",
        OKGREEN,
        ENDC,
        elapsed_with_color(elapsed)
    );
    Ok(())
}

fn main() {
    let args = Args::parse();

    loop {
        if let Err(e) = verify(&args) {
            eprintln!("Error: {}", e);
            break;
        }
    }
}
