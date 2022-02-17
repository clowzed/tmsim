use crate::moves::MoveKind;

#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Debug, Clone, PartialEq, Copy)]
pub struct Command
{
    pub state        : usize,
    pub next_state   : usize,
    pub reading_char : char,
    pub place_char   : char, 
    pub next_move    : MoveKind,
}

impl std::fmt::Display for Command
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result
    {
        write!(f, "q{}({}) -> q{}({}){}", self.state, self.reading_char, self.next_state, self.place_char, self.next_move)
    }
}