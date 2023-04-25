use std::fs::File;
use std::io::Write;
use std::path::Path;

use clap::builder::TypedValueParser as _;
use clap::Parser;
use silk_rs::decode_silk;

mod wav;
use wav::{WavHeader, WavResult};

mod error;
use error::LocatedError;

#[derive(Parser)]
#[command(author, version, about, long_about = None, arg_required_else_help(true))]
struct Cli {
    #[arg(
        short, long,
        default_value_t=16000,
        value_parser = clap::builder::PossibleValuesParser::new(["8000", "16000", "441000"])
            .map(|s| s.parse::<u32>().unwrap()),
    )]
    sample_rate: u32,

    input_file: Option<String>,
}

fn main() -> WavResult {
    let cli = Cli::parse();
    real_main(cli)
}

fn real_main(cli: Cli) -> WavResult {
    let input = cli.input_file.unwrap();
    let sample_rate: u32 = cli.sample_rate;
    let file_path = Path::new(&input);
    let out_file_path = file_path.with_extension("wav");

    let input = std::fs::read(file_path).map_err(loc!())?;
    let output = decode_silk(input, sample_rate as i32).map_err(loc!())?;

    let wav_header = WavHeader::new(1, sample_rate, output.len() as u32);

    let mut file = File::create(&out_file_path)?;
    wav_header.write(&file)?;
    file.write(&output).map_err(loc!())?;
    println!("Convert to wav {:?} successfully!", out_file_path);
    Ok(())
}
