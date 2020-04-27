use structopt::StructOpt;

const SVG_SCALE: f32 = 10.0;

// Command line arguments
#[derive(Debug, StructOpt)]
#[structopt(name = "svg", about = "A CLI example of the siege library.")]
struct Opt {
    /// Input file
    #[structopt(parse(from_os_str))]
    input: std::path::PathBuf,

    /// Output file
    #[structopt(parse(from_os_str))]
    output: std::path::PathBuf,

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

    // Write the SVG string to a file
    std::fs::write(&opt.output, engine.to_svg(SVG_SCALE)).expect("Unable to write file");
    println!("Written SVG output to file {:?}", opt.output);
}
