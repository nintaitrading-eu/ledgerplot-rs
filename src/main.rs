extern crate docopt;

use docopt::Docopt;

const USAGE: &'static str = "
ledgerplot

Usage:
  ledgerplot (-h | --help)
  ledgerplot --version

Options:
  -h --help  Show usage info.
  --version  Show version.
";

fn main()
{
    let args = Docopt::new(USAGE)
        .and_then(|dopt| dopt.parse())
        .unwrap_or_else(|e| e.exit());
    
    print!("TBD");
}
