use structopt::StructOpt;

fn main() {
    let opt = Opt::from_args();
    println!("{:?}", opt)
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
