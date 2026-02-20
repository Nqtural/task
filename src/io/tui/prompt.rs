#[derive(Default)]
pub enum Prompt {
    Confirm(&'static str),
    #[default]
    None,
    Text(Vec<char>),
}
