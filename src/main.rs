use {
  rand::random,
  std::fs
};

/*
 * Utilities functions (read and write files)
 */ 
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

/*
 * Detection and correction of errors
 */
fn hamming7(c1: u8, c2: u8, c3: u8, c4: u8, c5: u8, c6: u8, c7: u8) -> usize {
  let p1 = c1 ^ c2 ^ c3;
  let p2 = c1 ^ c2 ^ c4;
  let p3 = c2 ^ c3 ^ c4;
  match (c5 != p1, c6 != p2, c7 != p3) {
    (true,  true,  false) => { 1 },
    (true,  true,  true)  => { 2 },
    (true,  false, true)  => { 3 },
    (false, true,  true)  => { 4 },
    _                     => { 0 }
  }
}

fn correct_errors(input: Vec<u8>) -> Vec<u8> {
  let mut corrected = Vec::new();
  let mut i:usize = 0;
  while i + 7 < input.len() {
    let c = hamming7(input[i], input[i+1], input[i+2], input[i+3], input[i+4], input[i+5], input[i+6]);
    for j in 0..7 {
      corrected.push(
        if j + 1 != c {
          input[i+j]
        } else {
          input[i+j] ^ 0x01
        });
    }
    i += 7;
  }
  corrected
}

/*
 * Reduction of the binary encoding (removal of the control bits)
 */
fn reduce(input: Vec<u8>) -> Vec<u8> {
  let mut reduced = Vec::new();
  let mut i:usize = 0;
  while i + 7 <= input.len() {
    for j in 0..4 {
      reduced.push(input[i+j]);
    }
    i += 7;
  }
  reduced
}

/*
 * Conversion to ASCII characters
 */
fn group_bytes(bytes: &Vec<u8>) -> Vec<u8> {
  let mut grouped = Vec::new();
  let mut i:usize = 0;
  let mut b:u8 = 0;
  for byte in bytes {
    b = b << 1 | byte;
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
  let s = String::from_iter(group_bytes(&input).iter().map(|v| { *v as char }));
  s
}

/*
 * Decryption of the letter (Vigenère cipher with key "python")
 */
fn decrypt(input: String) -> String {
  let key = "python";
  let diff: Vec<u8> = key.chars().map(|c| { c as u8 }).collect(); 
  let mut decrypted = String::new();
  let mut i: usize = 0;
  for c in input.chars() {
    let b = if c >= 'a' && c <= 'z' {
      'a' as u8
    } else if c >= 'A' && c <= 'Z' {
      'A' as u8
    } else {
      0
    };
    if b != 0 {
      let l = c as u8 - b;
      let k = diff[i] - 'a' as u8;
      let d = (l + 26 - k) % 26 + b;
      decrypted.push(d as char);
      i = (i + 1) % key.len();
    } else {
      decrypted.push(c);
    }
  }
  decrypted
}

/*
 * Encryption of the letter (Vigenère cipher with a random key)
 */
fn convert_to_bin(input: String) -> Vec<u8> {
  let mut bin = Vec::new();
  for c in input.as_bytes() {
    for i in 0..8 {
      bin.push((c >> (7 - i)) & 1);
    }
  }
  bin
}

fn encrypt(input: String) -> (Vec<u8>, Vec<u8>) {
  let bin = convert_to_bin(input);
  let mut encrypted = Vec::new();
  let mut token = Vec::new();
  for b in bin {
    let k = random::<u8>() & 0x01;
    encrypted.push(b ^ k);
    token.push(k);
  }
  (encrypted, token)
}

/*
 * Compress data with the optimal binary encoding
 */
fn optimal_encoding(data: Vec<u8>) -> () {
  let mut stats = Vec::new();
  for i in 0..256 {
    stats.push((i, 0));
  }
  for i in data {
    stats[i as usize].1 += 1;
  }
  stats.sort_by(|a, b| b.1.cmp(&a.1));
  println!("{:?}", stats);
  ////////////////////////////////////////////////////////////////////////
}

fn compress(input: Vec<u8>) -> Vec<u8> {
  let data = group_bytes(&input);
  let mut compressed = Vec::new();
  optimal_encoding(data);
  ////////////////////////////////////////////////////////////////////////
  ////////////////////////////////////////////////////////////////////////

  compressed
}

/*
 * Main function
 */
fn main() {
  let input = read_file("doc/lettre.txt");
  let corrected = correct_errors(input);
  write_bin_file("--1-corrected.txt", &corrected);
  let reduced = reduce(corrected);
  write_bin_file("--2-reduced.txt", &reduced);
  let ascii = convert_to_ascii(reduced);
  write_text_file("--3-ascii.txt", &ascii);
  let decrypted = decrypt(ascii);
  write_text_file("--4-decrypted.txt", &decrypted);
  let (encrypted, token) = encrypt(decrypted);
  write_bin_file("--5-encrypted.txt", &encrypted);
  write_bin_file("--5-token.txt", &token);
  let compressed_message = compress(encrypted);
  let compressed_token = compress(token);
  write_bin_file("--6-compressed-message.txt", &compressed_message);
  write_bin_file("--6-compressed-token.txt", &compressed_token);
}
