use std::env;
use serde_json::{Value};

fn main() {

  // let mut results: u8 = 1;

  let word: Vec<String> = env::args().collect();
  // println!("{}", word[1].as_str());
  let jisho = ureq::get("https://jisho.org/api/v1/search/words")
    .query("keyword", word[1].as_str())
    .call();

  let json_string = jisho.into_string().unwrap().as_str();
  let json_response: Value = serde_json::from_str(json_string);
    // [location] .as_str().unwrap().replace("\"", "");

  // let j_word = json_response["data"][0]["japanese"][0]["word"].as_str().unwrap().replace("\"", "");
  // let e_word = json_response["data"][0]["senses"][0]["english_definitions"][0].as_str().unwrap().replace("\"", "");


  // println!("{}", json_response["data"][0]["japanese"][0]["word"]);
  // println!("{} ", e_word);

}
