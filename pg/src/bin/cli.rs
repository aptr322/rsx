use clap::Parser;

#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
struct Cli {
    #[clap(short, long, default_value_t = false)]
    domains: bool,

    #[clap(value_parser)]
    source_dir: String,

    #[clap(value_parser)]
    output_dir: String,

}

fn main() {
    let cli = Cli::parse();

    println!("{} {} {}",&cli.domains, &cli.source_dir, &cli.output_dir);
}
