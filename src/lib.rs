extern crate chrono;

#[macro_use]
extern crate scan_rules;

use std::fmt;
use std::fmt::Display;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
    }
}

#[derive(Debug)]
struct Error {
    message: String,
}

impl Error {
    fn from(message: &str) -> Error {
        Error { message: message.to_string(), }
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}

struct Unidiff {
    header: DiffHeader,
    chunks: Vec<DiffChunk>,
}

struct DiffHeader {
    from_file: String,
    from_file_mod_datetime: chrono::DateTime<chrono::UTC>,

    to_file: String,
    to_file_mod_datetime: chrono::DateTime<chrono::UTC>,
}

struct DiffChunk {
    pre_start_line:  i64,
    pre_num_lines:   i64,
    post_start_line: i64,
    post_num_lines:  i64,

    lines: Vec<(LineAction, String)>,
}

impl DiffChunk {
    pub fn from(text_lines: &[String]) -> Result<DiffChunk, Error> {
        assert!(text_lines.len() > 0);

        // Parse header line.
        let header_line = &text_lines[0];
        let_scan!(header_line; (
            "@@ ",
            let pre_start_line:  i64,
            ",",
            let pre_num_lines:   i64,
            " ",
            let post_start_line: i64,
            ",",
            let post_num_lines:  i64,
            " @@"
        ));

        // Parse body.
        let chunk_lines = text_lines[1..].iter()
            .map(|line| {
                let (prefix, line_left) = line.split_at(1);
                let action = match prefix {
                    "+" => LineAction::Add,
                    "-" => LineAction::Remove,
                    " " => LineAction::Keep,
                    _ => return Err(Error::from("bad line prefix")),
                };

                Ok((action, line_left.to_string()))
            }).collect::<Result<Vec<_>, _>>()?;

        Ok(DiffChunk {
            pre_start_line:  pre_start_line,
            pre_num_lines:   pre_num_lines,
            post_start_line: post_start_line,
            post_num_lines:  post_num_lines,

            lines: chunk_lines,
        })
    }
}


enum LineAction { Add, Remove, Keep, }
