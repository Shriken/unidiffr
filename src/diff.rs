use chrono;

use error::Error;

pub struct Unidiff {
    header: DiffHeader,
    chunks: Vec<DiffChunk>,
}


pub struct DiffHeader {
    from_file: String,
    from_file_mod_datetime: chrono::DateTime<chrono::UTC>,

    to_file: String,
    to_file_mod_datetime: chrono::DateTime<chrono::UTC>,
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
