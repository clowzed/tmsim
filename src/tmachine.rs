use crate::commands_manager;
use crate::state;
use crate::command;
use crate::moves;
use colored::Colorize;


#[derive(Debug)]
pub struct TMachine
{
    cmanager     : commands_manager::CommandsManager,
    tape         : std::vec::Vec<char>,
    current_state: state::State,
    alphabet     : String,
}

impl TMachine
{
    pub fn new() -> TMachine
    {
        TMachine{cmanager: commands_manager::CommandsManager::new(), tape: vec![], current_state: state::State::new(0, 0), alphabet: String::new()}
    }

    pub fn set_tape(&mut self, new_tape: String)
    {
        self.tape = new_tape.chars().collect();
    }

    pub fn current_symbol(&self) -> char
    {
        self.tape[self.current_state.tape_cell_index]
    }

    pub fn execute_command(&mut self, c: command::Command)
    {
        if self.current_state.tape_cell_index == self.tape.len() - 1 && c.next_move == moves::MoveKind::Right
        {
            for _ in 0..self.current_state.tape_cell_index - self.tape.len() + 2
            {
                self.tape.push(' ');
            }

            if self.current_state.tape_cell_index == 0 && c.next_move == moves::MoveKind::Left
            {
                eprintln!("Failed to step left as you are on start of the tape!");
                std::process::exit(9);
            }
        }

        self.tape[self.current_state.tape_cell_index] = c.place_char;
        self.current_state.number = c.next_state;
        match c.next_move
        {   
            moves::MoveKind::Left => self.current_state.tape_cell_index -= 1,
            moves::MoveKind::Right => self.current_state.tape_cell_index += 1,
            moves::MoveKind::Stop => {}
        };
    }

    pub fn run(&mut self)
    {
        while let Some(command) = self.cmanager.find_command(self.current_state.number, self.current_symbol())
        {
            println!("Executing: {}", command);
            self.execute_command(command);
            self.repr();
            
        }
    }

    pub fn set_commands(&mut self, commands: Vec<command::Command>)
    {
        for command in commands
        {
            self.cmanager.add_command(command)
        }
    }

    pub fn repr(&self)
    {
        print!("[");
        for (i, symbol) in self.tape.iter().enumerate()
        {
            if i == self.current_state.tape_cell_index
            {
                print!("{}", format!("{}", symbol).green());
            }
            else 
            {
                print!("{}", symbol);
            }
        }
        println!("]");
    }

    pub fn check(&self)
    {
        if !self.cmanager.commands_are_determinated()
        {
            eprintln!("Commands are not determinated!");
            std::process::exit(8);
        }
        let chars_from_commands = self.cmanager.used_symbols();
        if !chars_from_commands.iter().all(|c| self.alphabet.contains(*c))
        {
            eprintln!("Commands contain symbols which are not in alphabet!");
            std::process::exit(8);
        }
    }

    pub fn set_alphabet(&mut self, alphabet: String)
    {
        let mut alphabet = alphabet;
        alphabet.push('*');
        self.alphabet = alphabet;
    }
}