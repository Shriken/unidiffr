use chrono;
use chrono::DateTime;
use chrono::offset::TimeZone;
use chrono::offset::fixed::FixedOffset;

use scan_rules::scanner::NonSpace;
use scan_rules::scanner::Everything;

use std::error;
use error::Error;


pub struct Unidiff {
    header: DiffHeader,
    chunks: Vec<DiffChunk>,
}


#[derive(Debug, PartialEq)]
pub struct DiffHeader {
    from_file: String,
    from_file_mod_datetime: chrono::DateTime<chrono::FixedOffset>,

    to_file: String,
    to_file_mod_datetime: chrono::DateTime<chrono::FixedOffset>,
}

impl DiffHeader {
    pub fn from(from_line: String, to_line: String)
        -> Result<DiffHeader, Box<error::Error>>
    {
        let_scan!(&from_line; (
            "--- ",
            let from_file: NonSpace,
            " ",
            let from_datetime_string: Everything
        ));
        let_scan!(&to_line; (
            "+++ ",
            let to_file: NonSpace,
            " ",
            let to_datetime_string: Everything
        ));

        let from_file_mod_datetime = DateTime::parse_from_str(
            from_datetime_string.as_ref(),
            "%Y-%m-%d %H:%M:%S%.f %z"
        );
        let to_file_mod_datetime = DateTime::parse_from_str(
            to_datetime_string.as_ref(),
            "%Y-%m-%d %H:%M:%S%.f %z"
        );

        Ok(DiffHeader {
            from_file: from_file.to_string(),
            to_file:   to_file.to_string(),
            from_file_mod_datetime: from_file_mod_datetime?,
            to_file_mod_datetime:   to_file_mod_datetime?,
        })
    }
}

#[test]
fn diff_header_test_parse() {
    let from_file = "a/foo/bar/baz";
    let to_file   = "a/foo/baz";
    let from_datetime_str = "1981-03-14 01:23:45.010101 +0800";
    let to_datetime_str   = "2030-03-14 01:23:45.010101 +0600";

    let from_datetime = FixedOffset::east(8 * 3600)
        .ymd(1981, 3, 14)
        .and_hms_nano(01, 23, 45, 010101000);
    let to_datetime   = FixedOffset::east(6 * 3600)
        .ymd(2030, 3, 14)
        .and_hms_nano(01, 23, 45, 010101000);

    let header = DiffHeader {
        from_file: from_file.to_string(),
        to_file:   to_file.to_string(),
        from_file_mod_datetime: from_datetime,
        to_file_mod_datetime:   to_datetime,
    };

    let parsed_header = DiffHeader::from(
        format!("--- {} {}", from_file, from_datetime_str),
        format!("+++ {} {}", to_file, to_datetime_str),
    ).unwrap();

    assert_eq!(header, parsed_header);
}


#[derive(Debug, PartialEq)]
pub struct DiffChunk {
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
            "@@ -",
            let pre_start_line:  i64,
            ",",
            let pre_num_lines:   i64,
            " +",
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

        // Return.
        Ok(DiffChunk {
            pre_start_line:  pre_start_line,
            pre_num_lines:   pre_num_lines,
            post_start_line: post_start_line,
            post_num_lines:  post_num_lines,

            lines: chunk_lines,
        })
    }

}

#[test]
fn diff_chunk_test_parse() {
    let chunk_lines = &[
        "@@ -1,2 +3,4 @@",
        " a",
        "-b",
        "+c",
        "-d",
        " e",
    ];

    let diff_chunk = DiffChunk {
        pre_start_line:  1,
        pre_num_lines:   2,
        post_start_line: 3,
        post_num_lines:  4,
        lines: vec![
            (LineAction::Keep,   "a".to_string()),
            (LineAction::Remove, "b".to_string()),
            (LineAction::Add,    "c".to_string()),
            (LineAction::Remove, "d".to_string()),
            (LineAction::Keep,   "e".to_string()),
        ],
    };

    let from_chunk_res = DiffChunk::from(
        &chunk_lines[..].iter()
            .map(|s| s.to_string())
            .collect::<Vec<String>>()
    );
    assert!(from_chunk_res.is_ok());
    let from_chunk = from_chunk_res.unwrap();
    assert_eq!(diff_chunk, from_chunk);
}


#[derive(Debug, PartialEq)]
enum LineAction { Add, Remove, Keep, }
