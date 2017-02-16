#[macro_use] extern crate clap;
extern crate unidiffr;

use std::fs::File;
use std::io;
use std::io::BufRead;
use std::io::BufReader;
use std::process;

fn main() {
    let matches = clap_app!(unidiffr =>
        (version: "1.0")
        (author: "Zane Sterling <me@zanesterling.io>")
        (about: "Parses unified diff format")
        (@arg FILE: "blah")
        (@arg json: -j --json "Sets the output format to JSON")
    ).get_matches();

    // Get output format.
    let mut output_format = "raw";
    if matches.is_present("json") { output_format = "json"; }
    let output_format = output_format; // re-immut it

    let lines = get_lines(matches.value_of("FILE"));
    let str_lines = lines.iter().map(|s| s.as_ref()).collect::<Vec<_>>();
    let diff = unidiffr::Unidiff::from(&str_lines);

    match output_format {
        "raw" => println!("{:#?}", diff),

        _ => panic!("output format {} not handled", output_format),
    }
}

fn get_lines(filename: Option<&str>) -> Vec<String> {
    let stdin = io::stdin();
    let input: Box<Iterator<Item=String>> =
        if filename.is_some() {
            let file = File::open(filename.unwrap())
                .unwrap_or_else(|err| {
                    println!("error: {}", err);
                    process::exit(-1);
                });
            Box::new(BufReader::new(file)
                .lines()
                .filter_map(|res| res.ok())
            )
        } else {
            Box::new(stdin
                .lock()
                .lines()
                .filter_map(|res| res.ok())
            )
        };

    input.collect::<Vec<_>>()
}
