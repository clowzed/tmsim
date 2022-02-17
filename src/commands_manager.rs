use crate::command;

#[derive(Debug)]
pub struct CommandsManager
{
    commands: std::vec::Vec<command::Command>
}

impl std::fmt::Display for CommandsManager
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result
    {
        for command in self.commands.iter()
        {
            write!(f, "{}", command)?
        }
        Ok(())
    }
}

impl CommandsManager
{
    pub fn new() -> CommandsManager
    {
        CommandsManager{ commands: std::vec::Vec::new()}
    }

    //? Checks if all left parts 
    //? of commands are different
    pub fn commands_are_determinated(&self) -> bool
    {
        for first_command in self.commands.iter()
        {
            'second_loop: for second_command in self.commands.iter()
            {
                if first_command == second_command {continue 'second_loop}
                
                if first_command.state == second_command.state &&
                   first_command.reading_char == second_command.reading_char
                {
                    return false;
                }
            }
        }
        true
    }

    pub fn find_command(&self, number: usize, symbol: char) -> Option<command::Command>
    {
        for command in self.commands.iter()
        {
            if command.state == number && command.reading_char == symbol
            {
                return Some(*command);
            }
        }
        None
    }

    pub fn add_command(&mut self, command: command::Command)
    {
        self.commands.push(command);
    }

    pub fn used_symbols(&self) -> Vec<char>
    {
        let mut res = vec![];
        for command in self.commands.iter()
        {
            res.push(command.place_char);
            res.push(command.place_char);
        }
        res.sort_unstable();
        res.dedup();
        res
    }
}