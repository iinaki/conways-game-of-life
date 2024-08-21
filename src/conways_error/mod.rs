/// Enum representing possible errors that can occur while running the game
#[derive(Debug)]
pub enum ConwaysError {
    FailedToCreateCell(String),
}