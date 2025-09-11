use clap::Parser;

#[derive(Parser, Debug)]
#[command(name = "Connected Agents")]
#[command(author = "Amin Khozaei <amin.khozaei@gmail.com>")]
#[command(version = "0.1")]
#[command(about = "Multi-agent system control tool", long_about = None)]
pub struct Args {
    #[arg(short, long, default_value = "settings.yaml")]
    pub settings: String,
    
    #[arg(short, long)]
    pub workflow: Option<String>,
}