#[derive(Debug)]
pub enum Action {
    Add,
    Delete,
    EditExpiration,
    EditName,
    MoveDown,
    MoveLeft,
    MoveRight,
    MoveUp,
    None,
    PromptAccept,
    PromptCancel,
    PromptDelete(Amount),
    PromptInput(char),
    PromptNavigate(Direction),
    Quit,
    Tick,
    ToggleFinish,
}

#[derive(Debug)]
pub enum Amount {
    PreviousCharacter,
    CurrentCharacter,
    Word,
}

#[derive(Debug)]
pub enum Direction {
    Backwards,
    BackwardsWord,
    End,
    Forwards,
    ForwardsWord,
    Start,
}
