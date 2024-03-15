use std::{
  fs,
  io
};

fn read_file(file: &str) -> Vec<u8> {
  let input = fs::read_to_string(file).unwrap();
  let mut bytes = Vec::new();
  for byte in input.as_bytes() {
    bytes.push(*byte ^ 0x30);
  }
  bytes
}

fn write_bin_file(file: &str, bytes: &Vec<u8>) {
  let mut output = String::new();
  for byte in bytes {
    output.push((*byte | 0x30) as char);
  }
  fs::write(file, output).unwrap();
}

fn write_text_file(file: &str, ascii: &String) {
  fs::write(file, ascii).unwrap();
}

fn hamming7(c1: u8, c2: u8, c3: u8, c4: u8, c5: u8, c6: u8, c7: u8) -> usize {
  let p1 = c1 ^ c2 ^ c3;
  let p2 = c1 ^ c2 ^ c4;
  let p3 = c2 ^ c3 ^ c4;
  let e1 = c5 != p1;
  let e2 = c6 != p2;
  let e3 = c7 != p3;
  match (e1, e2, e3) {
    (true, true, false) => {
      1
    },
    (true, true, true) => {
      2
    },
    (true, false, true) => {
      3
    },
    (false, true, true) => {
      4
    },
    _ => {
      0
    }
  }
}

fn correct_errors(input: Vec<u8>) -> Vec<u8> {
  let mut corrected = Vec::new();
  let mut i:usize = 0;
  loop {
    if i + 7 > input.len(){
      break;
    }
    let c = hamming7(input[i], input[i+1], input[i+2], input[i+3], input[i+4], input[i+5], input[i+6]);
    for j in 0..7 {
      if j + 1 != c {
        corrected.push(input[i+j]);
      } else {
        corrected.push(if input[i+j] == 0 { 1 } else { 0 });
      }
    }
    i += 7;
  }
  corrected
}

fn reduce(input: Vec<u8>) -> Vec<u8> {
  let mut reduced = Vec::new();
  let mut i:usize = 0;
  loop {
    if i + 7 > input.len(){
      break;
    }
    for j in 0..4 {
      reduced.push(input[i+j]);
    }
    i += 7;
  }
  reduced
}

fn group_bytes(bytes: &Vec<u8>) -> Vec<u8> {
  let mut grouped = Vec::new();
  let mut i:usize = 0;
  let mut b:u8 = 0;
  for byte in bytes {
    b = b << 1;
    b = b | byte;
    i += 1;
    if i == 8 {
      grouped.push(b);
      b = 0;
      i = 0;
    }
  }
  grouped
}

fn convert_to_ascii(input: Vec<u8>) -> String {
  let g = group_bytes(&input);
  println!("{:?}", g);
  let s = String::from_iter(g.iter().map(|v| { *v as char }));
  s
}

fn decrypt(input: String) -> String {
  input
}


fn main() {
  let input = read_file("doc/lettre.txt");
  let corrected = correct_errors(input);
  write_bin_file("target/1-corrected.txt", &corrected);
  let reduced = reduce(corrected);
  write_bin_file("target/2-reduced.txt", &reduced);
  let ascii = convert_to_ascii(reduced);
  write_text_file("target/3-ascii.txt", &ascii);
  let decrypted = decrypt(ascii);
  write_text_file("target/4-decrypted.txt", &decrypted);
}
