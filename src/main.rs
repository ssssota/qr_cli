use image::Luma;
use qrcode::{
    render::{svg, unicode},
    QrCode,
};
use std::{fs, io::BufRead, path::PathBuf};
use structopt::StructOpt;

/// QR code generator
#[derive(StructOpt, Debug)]
#[structopt(name = "qr")]
struct Opt {
    /// Input text
    /// If not specified, stdin will be used.
    #[structopt(short, long)]
    input: Option<String>,

    /// Output file
    /// Accepted file formats are png, jpg, svg.
    #[structopt(short, long, parse(from_os_str))]
    output: Option<PathBuf>,
}

/// Read lines from stdin until EOF
fn read_lines() -> String {
    std::io::stdin()
        .lock()
        .lines()
        .flatten()
        .collect::<Vec<String>>()
        .join("\n")
}

fn main() {
    let opt = Opt::from_args();
    let input = opt.input.unwrap_or_else(read_lines);
    let qr = QrCode::new(input).unwrap();

    match opt.output {
        None => println!("{}", qr.render::<unicode::Dense1x2>().build()),
        Some(output) => match output.extension().map(|s| s.to_str()).flatten() {
            None => panic!("Cannot detect file type"),
            Some("png" | "jpg" | "jpeg") => qr.render::<Luma<u8>>().build().save(output).unwrap(),
            Some("svg") => fs::write(output, qr.render::<svg::Color>().build()).unwrap(),
            _ => println!("Invalid file type. (png/jpg/svg is supported)"),
        },
    }
}
