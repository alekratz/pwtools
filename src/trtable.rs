extern crate yaml;

use std::collections::HashMap;
use std::fs::File;
use self::yaml::constructor::YamlStandardData;

fn unwrap_yaml_str(yaml_str: YamlStandardData) -> Option<String> {
  match yaml_str {
    YamlStandardData::YamlString(s) => Some(s),
    _ => None,
  }
}

pub struct TrTable {
  table: HashMap<char, Vec<String>>,
}

impl TrTable {
  /// Creates an empty TrTable
  fn new() -> TrTable {
    TrTable {
      table: HashMap::new()
    }
  }

  pub fn contains_key(&self, letter: char) -> bool {
    self.table.contains_key(&letter)
  }

  pub fn get(&self, letter: char) -> Option<&Vec<String>> {
    self.table.get(&letter)
  }

  pub fn load(fname: &str) -> Result<TrTable, &'static str> {
    // open and load the file
    let mut fp = File::open(&fname)
      .ok()
      .expect("trtab file could not be found");
    let yaml_data = yaml::parse_io_utf8(&mut fp)
      .ok()
      .expect("could not parse trtab YAML");

    if yaml_data.is_empty() {
      return Err("YAML file did not contain any data")
    }

    // create a new TrTable that will be returned
    let mut trtab = TrTable::new();
    for yaml_datum in yaml_data {
      // get all items in the list
      let translations = match yaml_datum {
        YamlStandardData::YamlMapping(v) => v,
        _ => return Err("reached unexpected YAML data"),
      };

      for pair in translations {
        let key = unwrap_yaml_str(pair.0)
          .expect("Couldn't unwrap YAML key");
        let value = unwrap_yaml_str(pair.1)
          .expect("Couldn't unwrap YAML value");
        // verify that it's a single letter
        if key.len() != 1 {
          panic!("expected a single letter, but instead got '{}'", key);
        }
        // set the letter
        let letter: char = key
          .chars()
          .nth(0)
          .unwrap();
        // get all the words as a Vec<String>
        let mut words: Vec<String> = value.split(" ")
          .map(|s| s.to_string())
          .collect();

        // useful for debugging
        //println!("{} : {}", letter, words.connect(" - "));
        if trtab.table.contains_key(&letter) {
          // append the words to the thing
          trtab.table.get_mut(&letter)
            .unwrap()
            .extend(words);
        } else {
          trtab.table.insert(letter, words);
        }

      }
    }

    Ok(trtab)
  }
}