extern crate getopts;
extern crate char_iter;

mod trtable;

use getopts::Options;
use std::env;
use trtable::TrTable;

/// Permutes over a single term's possible alternate characters
///
fn permute(index: usize, term: &str, built: String, trtab: &TrTable) {
  // base case
  if index == term.len() {
    println!("{}", built);
    return;
  }

  let c: char = term.chars()
    .nth(index)
    .expect("");
  
  let list = trtab.get(c);
  if list == None {
    // just add on the next string to the built and keep going
    let mut new_built: String = (&built).to_string();
    new_built.push(c);
    permute(index + 1, &term, new_built, &trtab);
  } else {
    // go through all of the possibilities
    let u_list = list.unwrap();
    for append in u_list {
      let new_built: String = (&built).to_string() + append;
      permute(index + 1, &term, new_built, &trtab);
    }
  }
}

fn all_combos(chars: &Vec<char>, count: usize, built: String) {
  // base case
  if count == 0 {
    println!("{}", built);
    return;
  }

  for letter in chars {
    let mut new_built: String = (&built).to_string();
    new_built.push(*letter);
    all_combos(&chars, count - 1, new_built);
  }
}

fn print_usage(program: &str, opts: Options) {
  let brief = format!("Usage: {} [options]", program);
  print!("{}", opts.usage(&brief));
}

fn main() {
  let args: Vec<String> = env::args().collect();
  let program = args[0].clone();

  let mut opts = Options::new();
  opts.optopt("c", "combos", "iterate through all possible printable combinations of N letters. You may specify a comma separated list, e.g. 1,2,4,5", 
    "N");
  opts.optflag("", "no-upper", "specific to -c. If this flag is set, all combinations will omit upper case letters.");
  opts.optflag("", "no-lower", "specific to -c. If this flag is set, all combinations will omit lower case letters.");
  opts.optflag("", "no-numbers", "specific to -c. If this flag is set, all combinations will omit numbers.");
  opts.optflag("", "no-symbols", "specific to -c. If this flag is set, all combinations will omit symbols.");
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
        let trtab = TrTable::load(&trtab_fname)
          .ok()
          .unwrap();
        permute(0, &permute_str, "".to_string(), &trtab);
      },
      None => println!("You must specify a table file with the permute option")
    }
  // all combinations 
  } else if let Some(combos) = matches.opt_str("c") {
    let counts = combos.split(",")
      .map(|n| n.parse::<usize>().unwrap());

    let mut all_chars = Vec::new();
    // upper case letters
    if !matches.opt_present("no-upper") {
      for l in char_iter::new('\x41', '\x5a') {
        all_chars.push(l);
      }
    }
    // lower case letters
    if !matches.opt_present("no-lower") {
      for l in char_iter::new('\x61', '\x7a') {
        all_chars.push(l);
      }
    }
    // numbers
    if !matches.opt_present("no-numbers") {
      for l in char_iter::new('\x30', '\x39') {
        all_chars.push(l);
      }
    }
    // symbols
    if !matches.opt_present("no-symbols") {
      // <space> to /
      for l in char_iter::new('\x20', '\x2F') {
        all_chars.push(l);
      }
      // : to @
      for l in char_iter::new('\x3a', '\x40') {
        all_chars.push(l);
      }
      // [ to `
      for l in char_iter::new('\x5b', '\x60') {
        all_chars.push(l);
      }
      // { to ~
      for l in char_iter::new('\x7b', '\x7e') {
        all_chars.push(l);
      }
    }

    for n in counts {
      all_combos(&all_chars, n, String::new());
    }
  }
}
