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

		let file = file.unwrap();
		let lines = io::BufReader::new(file).lines();
		let mut last_num = 0;

		for (line_num, line) in lines.enumerate() {
			let line = line?;
			if config.number_lines {
				println!("{:6}\t{}", line_num + 1, line);
			} else if config.number_nonblank_lines {
				if line.len() > 0 {
					last_num += 1;
					println!("{:6}\t{}", last_num, line);
				} else {
					println!("");
				}
			} else {
				println!("{}", line);
			}
		}
	}
	Ok(())
}
