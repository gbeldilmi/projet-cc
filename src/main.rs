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

fn write_file(file: &str, bytes: &Vec<u8>) {
  let mut output = String::new();
  for byte in bytes {
    output.push((*byte | 0x30) as char);
  }
  fs::write(file, output).unwrap();
}

fn write_ascii_file(file: &str, ascii: &String) {
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
    let mut c = hamming7(input[i], input[i+1], input[i+2], input[i+3], input[i+4], input[i+5], input[i+6]);
    let mut j:usize = 0;
    while j < 7 {
      corrected.push(
        if c == 1 {
          input[i+j] ^ 0x01
        } else {
          input[i+j]
        });
      if c != 0 {
        c -= 1;
      }
      j += 1;
    }
    i += 7;
  }
  corrected
}

fn reduce(input: Vec<u8>) -> Vec<u8> {
  let mut reduced = Vec::new();
  let mut i:usize = 0;
  for byte in input {
    if i % 7 > 4 {
      reduced.push(byte);
    }
    i += 1;
  }
  reduced
}

fn decrypt(input: Vec<u8>) -> Vec<u8> {
  let mut decrypted = Vec::new();
  for byte in input {
    decrypted.push(byte);
  }
  decrypted
}

fn convert_to_ascii(input: Vec<u8>) -> String {
  let mut s = String::new();
  let mut i:usize = 0;
  while i < input.len() {
    let mut b:u8 = 0;
    for j in 0..8 {
      if i + j < input.len() {
        b = b << 1;
        b = b | input[i+j];
      }
    }
    s.push(b as char);
    i += 8;
  }
  s
}


fn main() {
  let input = read_file("doc/lettre.txt");
  let corrected = correct_errors(input);
  write_file("target/-corrected.txt", &corrected);
  let reduced = reduce(corrected);
  write_file("target/-reduced.txt", &reduced);
  let decrypted = decrypt(reduced);
  write_file("target/-decrypted.txt", &decrypted);
  let ascii = convert_to_ascii(decrypted);
  write_ascii_file("target/-ascii.txt", &ascii);
}
