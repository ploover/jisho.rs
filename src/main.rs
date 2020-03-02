use std::env;
use serde_json::Value;

fn main() {

  // let results: usize = 1;

  let args: Vec<String> = env::args().collect();
  let word = &args[1];
  let results: usize = args[2].parse().unwrap();

  println!("Finding definition for {}", word);

  let json_response = get_json(word);

  for i in 0..results {
    let j_word = &json_response["data"][i]["japanese"][0]["word"];
    let j_reading = &json_response["data"][i]["japanese"][0]["reading"];
    let e_word = &json_response["data"][i]["senses"][0]["english_definitions"][0];

    println!("{} ({}) - {}", j_word.as_str().unwrap(), j_reading.as_str().unwrap(), e_word.as_str().unwrap());

  }




}

fn get_json(word: &str) -> Value {
  let jisho = ureq::get("https://jisho.org/api/v1/search/words")
    .query("keyword", word)
    .call();

  return jisho.into_json().unwrap();
}
