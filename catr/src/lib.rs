use clap::{App, Arg};
use std::error::Error;
use std::fs::File;
use std::io::{self, BufRead, BufReader};

type MyResult<T> = Result<T, Box<dyn Error>>;

#[derive(Debug)]
pub struct Config {
	files: Vec<String>,
	number_lines: bool,
	number_nonblank_lines: bool,
}

pub fn get_args() -> MyResult<Config> {
	let matches = App::new("catr")
		.version("0.1.0")
		.author("Cornelius Toole <cornelius.toole@gmail.com>")
		.about("Rust cat")
		.arg(
			Arg::with_name("file")
				.value_name("FILE")
				.help("Input file(s)")
				.required(true)
				.min_values(1),
		)
		.arg(
			Arg::with_name("number")
				.help("Number lines")
				.takes_value(false)
				.long("number")
				.short("n")
				.conflicts_with("number_nonblank"),
		)
		.arg(
			Arg::with_name("number_nonblank")
				.help("Number non-blank lines")
				.takes_value(false)
				.long("number-nonblank")
				.short("b"),
		)
		.get_matches();

	let files = matches.values_of_lossy("file").unwrap();

	Ok(Config {
		files: files,
		number_lines: matches.is_present("number"),
		number_nonblank_lines: matches.is_present("number_nonblank"),
	})
}

pub fn run(config: Config) -> MyResult<()> {
	for filename in config.files {
		let mut line_num = 1;
		let file: MyResult<Box<dyn BufRead>> = match filename.as_str() {
			"-" => Ok(Box::new(BufReader::new(io::stdin()))),
			_ => match File::open(&filename) {
				Ok(file) => Ok(Box::new(BufReader::new(file))),
				Err(e) => Err(From::from(format!("{}: {}", &filename, e))),
			},
		};

		if let Err(e) = file {
			eprintln!("{}", e);
			continue;
		}

		let mut _file = file.unwrap();

		let mut contents = String::new();
		let mut num_bytes = _file.read_line(&mut contents)?;
		loop {
			if num_bytes == 0 {
				break;
			}
			let is_blank = contents.trim().is_empty();
			print!(
				"{}{}",
				if config.number_lines || config.number_nonblank_lines && !is_blank {
					format!("{:width$}\t", line_num, width = 6)
				} else {
					"".to_string()
				},
				contents
			);
			contents.clear();
			num_bytes = _file.read_line(&mut contents)?;
			if !config.number_nonblank_lines || !is_blank {
				line_num += 1;
			}
		}
	}
	Ok(())
}
