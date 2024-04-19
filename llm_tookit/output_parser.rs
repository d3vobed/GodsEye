use std::fs::{File, OpenOptions};
use std::io::{BufWriter, Read, Write};
use std::path::PathBuf;
use std::process::Command;
use std::{env, io};

fn is_raw_output(filename: &str) -> bool {
    filename.ends_with(".rawoutput")
}

fn parse_args() -> Result<CliArgs, pico::CliError> {
    let mut args = pico::Cli::new("llm_prompt_gen");
    args.add_argument(
        pico::Arg::from("-r")
            .long("--llm-response-path")
            .required(true)
            .help("A file containing the response from LLM."),
    );
    args.add_argument(
        pico::Arg::from("-o")
            .long("--output-path")
            .required(true)
            .help("A directory to save the parsed output."),
    );

    args.parse()
}

struct CliArgs {
    llm_response_path: PathBuf,
    output_path: PathBuf,
}

fn parse_code(response_path: &PathBuf) -> Result<String, io::Error> {
    let mut file = File::open(response_path)?;
    let mut content = String::new();
    file.read_to_string(&mut content)?;

    let solution = content.split("</solution>").next().unwrap();
    let parsed_code = solution
        .replace("<code>", "")
        .replace("</code>", "")
        .lines()
        .filter(|line| !line.trim().is_empty() && !line.trim().starts_with("```"))
        .collect::<Vec<_>>()
        .join("\n");

    Ok(parsed_code)
}

fn save_output(content: &str, output_path: &PathBuf) -> Result<(), io::Error> {
    let mut file = OpenOptions::new().create(true).write(true).open(output_path)?;
    file.write_all(content.as_bytes())?;
    Ok(())
}

fn main() -> Result<(), i32> {
    match parse_args() {
        Ok(args) => {
            let content = parse_code(&args.llm_response_path)?;
            save_output(&content, &args.output_path)?;
            Ok(())
        }
        Err(err) => {
            eprintln!("Error parsing arguments: {}", err);
            Err(1)
        }
    }
}
