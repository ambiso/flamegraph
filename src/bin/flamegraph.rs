use std::path::PathBuf;

use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(raw(
    setting = "structopt::clap::AppSettings::TrailingVarArg"
))]
struct Opt {
    /// Output file, flamegraph.svg if not present
    #[structopt(
        parse(from_os_str),
        short = "o",
        long = "output"
    )]
    output: Option<PathBuf>,

    /// Sampling frequency
    #[structopt(short = "F", long = "freq")]
    frequency: Option<String>,

    trailing_arguments: Vec<String>,
}

fn workload(opt: &Opt) -> String {
    if opt.trailing_arguments.is_empty() {
        eprintln!("no workload given to generate a flamegraph for!");
        std::process::exit(1);
    }

    opt.trailing_arguments.join(" ")
}

fn main() {
    let mut opt = Opt::from_args();

    let workload = workload(&opt);

    let flamegraph_filename: PathBuf = opt
        .output
        .take()
        .unwrap_or("flamegraph.svg".into());

    flamegraph::generate_flamegraph_by_running_command(
        workload,
        flamegraph_filename,
        opt.frequency,
    );
}
