use structopt::StructOpt;

// Command line arguments
#[derive(Debug, StructOpt)]
#[structopt(name = "ascii", about = "A CLI example of the siege library.")]
struct Opt {
    /// Input file
    #[structopt(parse(from_os_str))]
    input: std::path::PathBuf,

    /// Output width
    #[structopt(short, long, default_value = "10")]
    width: u32,

    /// Output height
    #[structopt(short, long, default_value = "10")]
    height: u32,
}

fn main() {
    let opt = Opt::from_args();

    // Parse the ASCII file
    let generator = siege::Generator::from_ascii_file(opt.input).unwrap();

    // Generate a skeleton using WFC
    let engine = generator
        .generate_skeleton(opt.width, opt.height, 100, &mut rand::thread_rng())
        .unwrap();

    // Print it
    println!("{}", engine.to_ascii());
}
