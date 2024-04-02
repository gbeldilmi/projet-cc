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
fn stats(data: &Vec<u8>) -> Vec<(u8, u128)> {
  let mut stats = Vec::new();
  for i in 0..256 {
    stats.push((i as u8, 0));
  }
  for i in data {
    stats[*i as usize].1 += 1;
  }
  stats.sort_by(|a, b| b.1.cmp(&a.1));
  stats
}

fn get_min(nodes: &Vec<(bool, u8, u128, usize, usize)>, ignore: (bool, u128, usize)) -> isize { // get the index of the node with the smallest frequency and different from the ignore node
  let mut min = -1; // -1 means no node found
  for j in 0..nodes.len() {
    if nodes[j].0 {
      if (ignore.0 && nodes[j].2 >= ignore.1 && j != ignore.2) || (!ignore.0) {
        if min == -1 || nodes[j].2 < nodes[min as usize].2 {
          min = j as isize;
        }
      }
    }
  }
  min
}

fn get_mins(nodes: &Vec<(bool, u8, u128, usize, usize)>) -> (isize, isize) { // get the index of the two nodes with the smallest frequency
  let mut mins = (-1, -1);
  mins.0 = get_min(&nodes, (false, 0, 0));
  mins.1 = get_min(&nodes, (true, nodes[mins.0 as usize].2, mins.0 as usize));
  mins
}

fn encode_node(nodes: &Vec<(bool, u8, u128, usize, usize)>, i: usize, prefix: String) -> Vec<(u8, String)> {
  let mut encoding = Vec::new();
  if nodes[i].3 == 0 && nodes[i].4 == 0 { // leaf node
    encoding.push((nodes[i].1, prefix));
  } else { // internal node
    let mut left = prefix.clone();
    left.push('0');
    let mut right = prefix.clone();
    right.push('1');
    encoding.append(&mut encode_node(nodes, nodes[i].3, left));
    encoding.append(&mut encode_node(nodes, nodes[i].4, right));
  }
  encoding
}

fn huffman(stats: Vec<(u8, u128)>) -> Vec<(u8, String)> { // (value, frequency) -> (value, encoding)
  let mut nodes = Vec::new();
  let max_nodes = 2 * stats.len() - 1;
  for (value, frequency) in stats {
    nodes.push((true, value, frequency, 0, 0));
  }
  loop {
    let (min1, min2) = get_mins(&nodes);
    if min1 == -1 || min2 == -1 {
      break;
    } else {
      let min1 = min1 as usize;
      let min2 = min2 as usize;
      let sum_values = nodes[min1].2 + nodes[min2].2;
      nodes.push((true, 0, sum_values, min1, min2));
      nodes[min1].0 = false;
      nodes[min2].0 = false;
    }
  }
  // convert to (value, encoding)
  let root = get_mins(&nodes).0 as usize;
  let r = encode_node(&nodes, root, String::new());
  r
}

fn encoding_to_string(encoding: &Vec<(u8, String)>) -> String {
  let mut s = String::new();
  for (value, encoding) in encoding {
    s.push_str(&format!("{:08b} ({}): {}\n", *value, *value, encoding));
  }
  s
}

fn compress(input: Vec<u8>) -> (String, String) {
  let data = group_bytes(&input);
  let stats = stats(&data); // (value, frequency)
  let huffman = huffman(stats); // (value, encoding)
  let mut compressed = String::new();
  for byte in data {
    for (value, encoding) in &huffman {
      if byte == *value {
        compressed.push_str(encoding);
        break;
      }
    }
  }
  (compressed, encoding_to_string(&huffman))
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
  write_text_file("--6-compressed-message.txt", &compressed_message.0);
  write_text_file("--6-compressed-message-encoding.txt", &compressed_message.1);
  write_text_file("--6-compressed-token.txt", &compressed_token.0);
  write_text_file("--6-compressed-token-encoding.txt", &compressed_token.1);
}
