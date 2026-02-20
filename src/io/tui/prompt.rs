#[derive(Default)]
pub enum Prompt {
    Confirm(&'static str),
    Info(&'static str),
    #[default]
    None,
    Text((&'static str, Vec<char>)),
}
