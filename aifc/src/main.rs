use structopt::StructOpt;

#[derive(StructOpt)]
struct CompilerArgs {
    #[structopt(
        long = "output",
        short = "o",
        default_value = "a.json",
        help = "output file"
    )]
    output: String,
    #[structopt(long = "minimize", short = "m")]
    minimize: bool,
    #[structopt(name = "INPUT_FILE")]
    file_name: String,
}

fn main() {
    let args = CompilerArgs::from_args();
    let input_file = std::fs::read_to_string(args.file_name).expect("Could not read input file");
    let json_logic = match aifc::compile_logic(&input_file, args.minimize) {
        Ok(logic) => logic,
        Err(e) => { 
            println!("{}", e);
            return;
        },
    };

     std::fs::write(args.output, json_logic)
            .expect("Couldn't write output file");
}