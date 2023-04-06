use reader::{Reader, ReaderBranch};

mod grammar;
mod reader;
#[cfg(test)]
mod tests;

type Result<T> = std::result::Result<T, Error>;

#[derive(Debug)]
enum Error {
    Todo,
    Eof,
}