use std::fs;

// cat concatenates files
pub fn cat(f1: &str, f2: &str) -> String {
  let content1 = fs::read_to_string(f1).expect("file should have been read");
  let content2 = fs::read_to_string(f2).expect("file should have been read");

  content1 + &"\n\n" + &content2
}