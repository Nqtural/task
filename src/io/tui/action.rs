#[derive(Debug)]
pub enum Action {
    Delete,
    EditExpiration,
    EditName,
    MoveDown,
    MoveLeft,
    MoveRight,
    MoveUp,
    None,
    PromptAccept,
    PromptBackspace,
    PromptCancel,
    PromptInput(char),
    Quit,
    Tick,
    ToggleFinish,
}
