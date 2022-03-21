use clap::{Arg, Command};
use mupdf::Document;

fn main() -> std::result::Result<(), Box<dyn std::error::Error>> {
    let matches = Command::new("pdfwc")
        .arg(
            Arg::new("file")
                .short('f')
                .long("file")
                .required(true)
                .takes_value(true)
                .help("pdf file name"),
        )
        .get_matches();
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));
    let filename = matches.value_of("file").unwrap().to_string();
    log::trace!("filename:{}", filename);
    let doc = Document::open(filename.as_str())?;
    let mut count = 0u32;
    let iter = doc.pages()?;
    for p in iter {
        let p = p?;
        let v = p.to_text()?;
        for x in v.chars() {
            let cp = x as u32;
            if cp >= 0x4e00 && cp <= 0x9fff {
                count += 1;
            } else if cp >= 0x2b740 && cp <= 0x2b91f {
                count += 1;
            }
        }
    }
    println!("{}", count);
    Ok(())
}
