extern crate chrono;
#[macro_use] extern crate scan_rules;

mod diff;
mod error;

pub use diff::Unidiff;


#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
    }
}
