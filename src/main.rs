/*
 * Copyright (C) 2015 Alek Ratzloff
 * 
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU Affero General Public License as published by
 * the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 * 
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU Affero General Public License for more details.
 * 
 * You should have received a copy of the GNU Affero General Public License
 * along with this program.  If not, see <http://www.gnu.org/licenses/>.
 */

extern crate getopts;
extern crate char_iter;

mod trtable;

use std::{env, thread};
use std::sync::mpsc::{channel, Receiver};
use getopts::Options;
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
    println!("{}", &built);
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

  opts.optopt("j", "threads", "number of threads to use. Default is 1.", "NUM_THREADS");
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
  let num_threads = match matches.opt_str("j") {
    Some(n_jobs) => n_jobs.parse::<usize>().unwrap(),
    None => 1
  };

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
  }
  // all combinations 
  if let Some(combos) = matches.opt_str("c") {
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

    // threads that are currently spawned
    let mut threads: Vec<(thread::JoinHandle<()>, Receiver<i32>)> = Vec::new();

    for n in counts {
      for letter in &all_chars {
        // get whether we have any thread slots open; if not, then
        // block until a thread finishes.
        while threads.len() == num_threads {
          let mut offset = 0;
          for index in 0 .. threads.len() {
            let remove = threads[index - offset].1
              .try_recv()
              .is_ok();

            if remove {
              threads.remove(index - offset);
              offset += 1;
            }
          }
          thread::sleep_ms(1);
        }

        // set up a new channel
        let (tx, rx) = channel();
        // create an intermediate var for the all_chars so we don't have to copy it fifty million times
        let all_chars_imdt = all_chars.clone();
        let l = *letter;
        let child = thread::spawn(move || {
          // shift ownership to the thread
          let all_chars_dup = all_chars_imdt;
          let mut new_built = String::new();
          
          new_built.push(l);
          all_combos(&all_chars_dup, n - 1, new_built);
          drop(tx.send(0));
        });
        threads.push((child, rx));
      }
    }

    // let the rest of the threads finish up
    for (th, _) in threads {
      drop(th.join());
    }
  }
}
