#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Debug, Clone, PartialEq, Copy)]
pub enum MoveKind
{
    Left,
    Right,
    Stop,
}

impl std::fmt::Display for MoveKind
{
    fn fmt(&self, f:&mut std::fmt::Formatter) -> std::fmt::Result
    {
        let stringed = match self
        {
            MoveKind::Left => "L",
            MoveKind::Right=> "R",
            MoveKind::Stop => "S",
        };
        write!(f, "{}", stringed)
    }
}
