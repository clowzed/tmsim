use structopt::StructOpt;
mod command;
mod commands_manager;
mod moves;
mod state;
mod tmachine;

#[derive(StructOpt, Debug)]
#[structopt(name = "Turing machine simulator", about = "Runs files with commands and start position.")]
struct Options
{
    #[structopt(parse(from_os_str))]
    source: std::path::PathBuf,
}


#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Debug)]
struct TMachineConfiguration
{
    commands: std::vec::Vec<crate::command::Command>,
    alphabet: String,
    tape: String,
}

fn main() 
{
    let options = Options::from_args();
    let mut tmachine = tmachine::TMachine::new();

    if !options.source.exists()
    {
        eprintln!("Failed to start! Configuration file provided does not exists!");
        std::process::exit(1);
    }

    let opened_source = match std::fs::File::open(&options.source)
    {
        Ok(descriptor) => descriptor,
        Err(e) => 
        {
            eprintln!("Failed to open provded conf file! Reason: {}", e);
            std::process::exit(2);
        }
    };

    let reader = std::io::BufReader::new(opened_source);


    let conf:TMachineConfiguration = match serde_json::from_reader(reader)
    {
        Ok(conf) => conf,
        Err(e) => 
        {
            eprintln!("fialed to parse conf file! Reason: {}", e);
            std::process::exit(3);
        }
    };


    tmachine.set_tape(conf.tape);
    tmachine.set_alphabet(conf.alphabet);
    tmachine.set_commands(conf.commands);
    tmachine.check();
    tmachine.run();
}
