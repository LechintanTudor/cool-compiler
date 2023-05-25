use cool_driver::Package;
use cool_parser::ParseError;
use std::fmt;

pub fn print_parse_error(error: &ParseError, package: &Package) {
    let (file, position) = package
        .source_map
        .get_file_and_position_from_offset(error.found.span.start);

    println!(
        "Parse error in file \"{}\" at line {}, column {}.",
        file.path.display(),
        position.line,
        position.column,
    );

    println!(" -> found: \"{}\".", error.found.kind);
    println!(" -> expected: {}.", ListDisplayer(error.expected));
}

#[derive(Clone, Copy, Debug)]
pub struct ListDisplayer<'a, T>(pub &'a [T]);

impl<T> fmt::Display for ListDisplayer<'_, T>
where
    T: fmt::Display,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.0 {
            [] => Ok(()),
            [elem] => write!(f, "\"{}\"", elem),
            [first, middle @ .., last] => {
                write!(f, "\"{first}\"")?;

                for elem in middle {
                    write!(f, ", \"{elem}\"")?;
                }

                write!(f, " or \"{last}\"")
            }
        }
    }
}
