#[derive(Debug, Clone, PartialEq)]
pub struct State
{
    pub number: usize,
    pub tape_cell_index: usize,
}

impl State
{
    pub fn new(number:usize, tape_cell_index:usize) -> State
    {
        State{number, tape_cell_index}
    }
}