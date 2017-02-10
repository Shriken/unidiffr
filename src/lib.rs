#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
    }
}

struct PatchHeader {
    pre_start: usize,
    pre_count: usize,

    post_start: usize,
    post_count: usize,
}

fn parse_header(header: String) -> PatchHeader {
}
