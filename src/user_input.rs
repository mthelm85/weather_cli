use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(name = "weather")]
pub struct Cli {

    #[structopt(long, required=true, help="The name of the city for which you want to get the weather")]
    pub city: Vec<String>,

    #[structopt(long, required=true, help="The state in which the city is located")]
    pub state: Vec<String>,

    #[structopt(short, long, help="Adds sunrise/sunset times to results")]
    pub daylight: bool

}

pub fn get_user_input() -> Cli {
    let cli = Cli::from_args();
    return cli
}