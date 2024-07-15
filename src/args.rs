use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about)]
pub struct Args {
    #[arg(short, long)]
    pub log_file: String,
}
