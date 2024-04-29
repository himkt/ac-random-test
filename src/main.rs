use clap::{Arg, Command as ClapCommand};
use std::{
    fs::File,
    process::{Command, Stdio},
    time::Instant,
};


const OKBLUE: &str = "\x1b[94m";
const OKGREEN: &str = "\x1b[92m";
const WARN: &str = "\x1b[1;33m";
const FAIL: &str = "\x1b[91m";
const ENDC: &str = "\x1b[0m";

fn run(command: &[String]) -> Result<(String, f64), Box<dyn std::error::Error>> {
    let start = Instant::now();
    let file = File::open("in.txt")?;
    let output = Command::new(&command[0])
        .args(&command[1..])
        .stdin(Stdio::from(file))
        .stderr(Stdio::null())
        .output()?;
    let duration = start.elapsed().as_secs_f64() * 1000.0;

    if !output.status.success() {
        let error_message = format!(
            "Run: {}fail{} command '{}' failed with exit status: {}",
            FAIL,
            ENDC,
            command.join(" "),
            output.status
        );
        println!("{}", error_message);
        std::process::exit(1);
    }

    let output_str = String::from_utf8(output.stdout)?;
    Ok((output_str.trim().to_string(), duration))
}

fn run_and_decode(command: &[String]) -> Result<(String, f64), Box<dyn std::error::Error>> {
    let start = Instant::now();
    let file = File::open("in.txt")?;
    let output = Command::new(&command[0])
        .args(&command[1..])
        .stdin(Stdio::from(file))
        .stderr(Stdio::null())
        .output()?;
    let duration = start.elapsed().as_secs_f64() * 1000.0;

    if !output.status.success() {
        let error_message = format!(
            "Run: {}fail{} command '{}' failed with exit status: {}",
            FAIL,
            ENDC,
            command.join(" "),
            output.status
        );
        println!("{}", error_message);
        std::process::exit(1);
    }

    let output_str = String::from_utf8(output.stdout)?;
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
    format!("{}{:.2}{}", color, elapsed, ENDC)
}

fn verify(args: &clap::ArgMatches) -> Result<(), Box<dyn std::error::Error>> {
    let name = args.get_one::<String>("name").unwrap();
    let gen_input_cmd = args.get_one::<String>("gen-input-cmd").unwrap();
    let run_cmd_template = args.get_one::<String>("run-cmd").unwrap();
    let max_ms = *args.get_one::<f64>("max-ms").unwrap();
    let without_lazy = args.get_flag("without-lazy");

    let gen_cmd: Vec<String> = gen_input_cmd.split_whitespace().map(String::from).collect();
    let output_gen = run(&gen_cmd);
    if let Err(ret) = output_gen {
        let error_message = format!(
            "Run: {}fail{} command '{}' failed with exit status: {}",
            FAIL,
            ENDC,
            ret,
            1,
        );
        println!("{}", error_message);
        std::process::exit(1);
    }

    let run_cmd: Vec<String> = run_cmd_template.replace("{argv0}", name).split_whitespace().map(String::from).collect();

    let result = run_and_decode(&run_cmd);
    if let Err(ret) = result {
        let error_message = format!(
            "Run: {}fail{} command '{}' failed with exit status: {}",
            FAIL,
            ENDC,
            ret,
            1,
        );
        println!("{}", error_message);
        std::process::exit(1);
    }
    let (output, elapsed) = result?;

    if elapsed >= max_ms {
        println!("Run: {}fail{} (took {}ms>={:.2}ms)", FAIL, ENDC, elapsed_with_color(elapsed), max_ms);
        std::process::exit(1);
    }

    if !without_lazy {
        let run_lazy_cmd: Vec<String> = run_cmd_template.replace("{argv0}", &format!("{}_lazy", name)).split_whitespace().map(String::from).collect();
        let (lazy_output, _) = run_and_decode(&run_lazy_cmd)?;

        if lazy_output == output {
            println!("Run: {}passed{} ({}ms)", OKGREEN, ENDC, elapsed_with_color(elapsed));
        } else {
            println!("Run: {}failed{} (found the edge case and stored in in.txt. [output: expected={}, got={}])", FAIL, ENDC, OKGREEN.to_owned() + &lazy_output + ENDC, FAIL.to_owned() + &output + ENDC);
            std::process::exit(1);
        }
    } else {
        println!("Run: {}passed{} ({}ms)", OKGREEN, ENDC, elapsed_with_color(elapsed));
    }

    Ok(())
}

fn main() {
    let app = ClapCommand::new("Test Runner")
        .arg(Arg::new("name")
            .required(true)
            .help("Name to be passed to `--run-cmd`."))
        .arg(Arg::new("gen-input-cmd")
            .long("gen-input-cmd")  // コマンドラインオプションとして指定
            .default_value("gen.py")
            .help("Command or executable file that creates `in.txt`."))
        .arg(Arg::new("run-cmd")
            .long("run-cmd")  // コマンドラインオプションとして指定
            .default_value("cargo run --release --bin {argv0}")
            .help("Command to run a program."))
        .arg(Arg::new("without-lazy")
            .long("without-lazy")
            .action(clap::ArgAction::SetTrue)
            .help("If this option is specified, skip the lazy version."))
        .arg(Arg::new("max-ms")
            .long("max-ms")  // この行を追加
            .default_value("inf")
            .value_parser(clap::value_parser!(f64))
            .help("Maximum allowed milliseconds for a run."));

    let matches = app.get_matches();

    loop {
        let _ = verify(&matches);
    }
}
