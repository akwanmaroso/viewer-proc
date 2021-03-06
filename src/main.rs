use std::io::Result;
use viewer_proc::{args::Args, read, stats, write};

fn main() -> Result<()> {
    let args = Args::parse();
    let mut total_bytes = 0;
    loop {
        let buffer = match read::read(&args.infile) {
            Ok(x) if x.is_empty() => break,
            Ok(x) => x,
            Err(_) => break,
        };
        stats::stats(args.silent, buffer.len(), &mut total_bytes, false);
        if !write::write(&args.outfile, &buffer)? {
            break;
        }
    }

    stats::stats(args.silent, 0, &mut total_bytes, true);

    Ok(())
}
