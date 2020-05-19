use structopt::StructOpt;

fn main() {
    let Opt {
        subcommand,
        debug: _,
    } = Opt::from_args();
    match subcommand {
        SubCommand::Load {
            dry_run: d,
            repository: r,
        } => println!("Loading: {}, dry_run is {}", r, d),
    }
}

#[derive(StructOpt, Debug)]
#[structopt(name = "shellis")]
struct Opt {
    #[structopt(short, long)]
    debug: bool,

    #[structopt(subcommand)]
    pub subcommand: SubCommand,
}

#[derive(StructOpt, Debug)]
#[structopt(name = "shellis")]
enum SubCommand {
    Load {
        #[structopt(short, long)]
        dry_run: bool,
        #[structopt(default_value = "http://localhost:8080/")]
        repository: String,
    },
}
