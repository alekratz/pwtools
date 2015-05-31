extern crate getopts;

mod trtable;

use getopts::Options;
use std::env;
use trtable::TrTable;

/// Permutes over a single term's possible alternate characters
///
fn permute(index: usize, term: &str, built: String, trtab: &TrTable) {
}

fn print_usage(program: &str, opts: Options) {
  let brief = format!("Usage: {} [options]", program);
  print!("{}", opts.usage(&brief));
}

fn main() {
  let args: Vec<String> = env::args().collect();
  let program = args[0].clone();

  let mut opts = Options::new();
  opts.optopt("p", "permute", "permute over a term with similar letters. Use the -t option to specify a table file", 
    "TERM");
  opts.optopt("t", "trtab", "translation table file, in YAML format. Required with the -p option.", "FILENAME");
  opts.optflag("h", "help", "print this menu");
  let matches = match opts.parse(&args[1..]) {
    Ok(m) => { m },
    Err(f) => { panic!(f.to_string()) },
  };

  if matches.opt_present("h") {
    // print help and exit
    print_usage(&program, opts);
    return;
  }

  let table_file = matches.opt_str("t");

  // permutation
  if let Some(permute_str) = matches.opt_str("p") {
    match table_file {
      Some(trtab_fname) => {
        // load the file
        let trtab = TrTable::load(&trtab_fname);
        //permute(0, &permute_str, "".to_string());
      },
      None => println!("You must specify a table file with the permute option")
    }
  }


}
