use {
  rand::random,
  std::fs
};

/*
 * Read a file and return a vector of ones and zeros as u8
 */
fn read_file(file: &str) -> Vec<u8> { 
  let input = fs::read_to_string(file).unwrap(); // read the file as a string
  let mut bytes = Vec::new();
  for byte in input.as_bytes() {
    bytes.push(*byte ^ 0x30); // convert '0' and '1' (ASCII) to 0 and 1 (u8)
  }
  bytes
}

/*
 * Write a vector of ones and zeros as u8 to a file
 */
fn write_bin_file(file: &str, bytes: &Vec<u8>) {
  let mut output = String::new(); // create an empty string
  for byte in bytes {
    output.push((*byte | 0x30) as char); // convert 0 and 1 (u8) to '0' and '1' (ASCII) and add it to the string
  }
  fs::write(file, output).unwrap(); // write the string to the file
}

/*
 * Write a string to a file
 */
fn write_text_file(file: &str, ascii: &String) {
  fs::write(file, ascii).unwrap();
}

/*
 * Detection of errors in the binary encoding
 * This function returns the index of the error (0 if no error)
 */
fn hamming7(c1: u8, c2: u8, c3: u8, c4: u8, c5: u8, c6: u8, c7: u8) -> usize {
  let p1 = c1 ^ c2 ^ c3; // first control bit
  let p2 = c1 ^ c2 ^ c4; // second control bit
  let p3 = c2 ^ c3 ^ c4; // third control bit
  match (c5 != p1, c6 != p2, c7 != p3) { // check control bits
    (true,  true,  false) => { 1 }, // error in c1
    (true,  true,  true)  => { 2 }, //    "     c2
    (true,  false, true)  => { 3 }, //    "     c3
    (false, true,  true)  => { 4 }, //    "     c4
    _                     => { 0 }  // no error
  }
}

/*
 * Correction of errors in the binary encoding
 */
fn correct_errors(input: Vec<u8>) -> Vec<u8> {
  let mut corrected = Vec::new();
  let mut i:usize = 0;
  while i + 7 < input.len() { // while there are 7 bits to read
    // get the index of the error 
    let c = hamming7(input[i], input[i+1], input[i+2], input[i+3], input[i+4], input[i+5], input[i+6]);
    for j in 0..7 {
      corrected.push(
        if j + 1 != c { // if the bit is not the error
          input[i+j]
        } else { // if the bit is the error, correct it
          input[i+j] ^ 0x01
        });
    }
    i += 7; // move to the next 7 bits
  }
  corrected
}

/*
 * Reduction of the binary encoding (removal of the control bits)
 */
fn reduce(input: Vec<u8>) -> Vec<u8> {
  let mut reduced = Vec::new();
  let mut i:usize = 0;
  while i + 7 <= input.len() {  // while there are 7 bits to read
    for j in 0..4 {             // read the first 4 bits
      reduced.push(input[i+j]); // and add them to the reduced vector 
    }
    i += 7; // move to the next 7 bits
  }
  reduced
}

/*
 * Grouping of bits by 8 to form bytes
 * 8 vectors of 1 bit -> 1 vector of 8 bits
 */
fn group_bytes(bytes: &Vec<u8>) -> Vec<u8> {
  let mut grouped = Vec::new();
  let mut i:usize = 0;
  let mut b:u8 = 0;
  for byte in bytes {
    b = b << 1 | byte; // add the bit to the byte
    i += 1;
    if i == 8 {        // if the byte is complete
      grouped.push(b); // add it to the grouped vector
      b = 0;
      i = 0;
    }
  }
  grouped
}

/*
 * Convert a vector of bits (0 and 1) to bytes (u8) and then to ASCII characters
 */
fn convert_to_ascii(input: Vec<u8>) -> String {
  let s = String::from_iter(group_bytes(&input).iter().map(|v| { *v as char }));
  s
}

/*
 * Decryption of the letter (Vigenère cipher with key "python")
 */
fn decrypt(input: String) -> String {
  let key = "python";
  let diff: Vec<u8> = key.chars().map(|c| { c as u8 }).collect(); // convert the key to a vector of bytes
  let mut decrypted = String::new();
  let mut i: usize = 0;
  for c in input.chars() { // for each character in the input
    // if the character is a letter (adapt the case if necessary)
    let b = if c >= 'a' && c <= 'z' {
      'a' as u8
    } else if c >= 'A' && c <= 'Z' {
      'A' as u8
    } else {
      0
    };
    if b != 0 { // if the character is a letter (decrypt it)
      let l = c as u8 - b;
      let k = diff[i] - 'a' as u8;
      let d = (l + 26 - k) % 26 + b;
      decrypted.push(d as char);
      i = (i + 1) % key.len();
    } else { // if the character is not a letter (keep it as is)
      decrypted.push(c);
    }
  }
  decrypted
}

/*
 * Convert a string to a vector of bits (0 and 1)
 */
fn convert_to_bin(input: String) -> Vec<u8> {
  let mut bin = Vec::new();
  for c in input.as_bytes() {       // for each byte in the input
    for i in 0..8 {                 // for each bit in the byte
      bin.push((c >> (7 - i)) & 1); // add the bit to the bin vector
    }
  }
  bin
}

/*
 * Encryption of the letter (Vigenère cipher with a random key)
 */
fn encrypt(input: String) -> (Vec<u8>, Vec<u8>) {
  let bin = convert_to_bin(input); // convert the input to a vector of bits
  let mut encrypted = Vec::new();
  let mut token = Vec::new();
  for b in bin {                   // for each bit in the input
    let k = random::<u8>() & 0x01; // generate a random bit
    encrypted.push(b ^ k);         // add the encrypted bit to the encrypted vector
    token.push(k);                 // add the token bit to the token vector
  }
  (encrypted, token) // return the encrypted vector and the token vector
}

/*
 * Statistics on the values of a vector
 * For each value, the number of occurrences is counted
 */
fn stats(data: &Vec<u8>) -> Vec<(u8, u128)> { // (value, number of occurrences)
  let mut stats = Vec::new();
  for i in 0..256 {           // for each possible value
    stats.push((i as u8, 0)); // initialize the number of occurrences to 0
  }
  for i in data {              // for each value in the data
    stats[*i as usize].1 += 1; // increment the number of occurrences for this value
  }
  stats.sort_by(|a, b| b.1.cmp(&a.1)); // sort the values by decreasing number of occurrences
  stats
}

/*
 * Get the index of the node with the smallest frequency and different from the ignored node
 * If ignore is (false, _), no node is ignored
 */
fn get_min(nodes: &Vec<(bool, u8, u128, usize, usize)>, ignore: (bool, usize)) -> isize {
  let mut min = -1;  // -1 means no node found
  for j in 0..nodes.len() { // for each node
    if nodes[j].0 {         // if the node is available
      if (ignore.0 && j != ignore.1) || (!ignore.0) {        // if the node is not ignored
        if min == -1 || nodes[j].2 < nodes[min as usize].2 { // if the node has a smaller frequency than the current minimum
          min = j as isize;                                  // update the minimum
        }
      }
    }
  }
  min
}

/*
 * Get the indexes of the two nodes with the smallest frequency
 */
fn get_mins(nodes: &Vec<(bool, u8, u128, usize, usize)>) -> (isize, isize) {
  let mut mins = (-1, -1);
  mins.0 = get_min(&nodes, (false, 0));
  mins.1 = get_min(&nodes, (true, mins.0 as usize));
  mins
}

/*
 * Recursive function to encode the nodes of the Huffman tree
 * The encoding is done by prefixing the encoding of the parent node with '0' or '1'
 * The nodes are stored in a vector of tuples (availability, value, frequency, left child index, right child index)
 */
fn encode_node(nodes: &Vec<(bool, u8, u128, usize, usize)>, i: usize, prefix: String) -> Vec<(u8, String)> {
  let mut encoding = Vec::new();
  if nodes[i].3 == 0 && nodes[i].4 == 0 { // Leaf node case (no child)
    encoding.push((nodes[i].1, prefix));
  } else {                                // Internal node case (two children)
    let mut left = prefix.clone();
    let mut right = prefix.clone();
    left.push('0');
    right.push('1');
    encoding.append(&mut encode_node(nodes, nodes[i].3, left)); // encode the left child
    encoding.append(&mut encode_node(nodes, nodes[i].4, right)); // encode the right child
  }
  encoding
}

/*
 * Huffman encoding of the values of a vector
 * (value, frequency) -> (value, encoding)
 */
fn huffman(stats: &Vec<(u8, u128)>) -> Vec<(u8, String)> {
  let mut nodes = Vec::new();
  for (value, frequency) in stats { // for each value and its frequency, create a node
    let value = *value;             //  (availability, value, frequency, left child index, right child index)
    let frequency = *frequency;
    nodes.push((true, value, frequency, 0, 0));
  }
  loop {
    let (min1, min2) = get_mins(&nodes); // get the indexes of the two nodes with the smallest frequency
    if min1 == -1 || min2 == -1 { // if there is only one node left, stop, it is the root
      break;
    } else {
      let min1 = min1 as usize;
      let min2 = min2 as usize;
      let sum_values = nodes[min1].2 + nodes[min2].2; // create a new node with the sum of the frequencies of the two nodes
      nodes.push((true, 0, sum_values, min1, min2));  // and add it to the nodes
      nodes[min1].0 = false;                          // mark the two nodes as unavailable
      nodes[min2].0 = false;
    }
  }
  // convert to (value, encoding)
  let root = get_mins(&nodes).0 as usize; // the root is the last node added
  let r = encode_node(&nodes, root, String::new()); // encode the nodes of the tree
  r
}

/*
 * Convert the encoding to a string (for debugging)
 * (value, frequency) join (value, encoding) on value --> (value, frequency, encoding)
 */
fn encoding_to_string(stats: &Vec<(u8, u128)>, encoding: &Vec<(u8, String)>) -> String {
  let mut s = String::new();
  for (v1, f) in stats {
    for (v2, e) in encoding {
      if v1 == v2 {
        // binary value, decimal value, encoding, frequency
        s.push_str(&format!("{:08b} \t ({:03}): \t {} \t {}\n", v1, v2, e, f));
      }
    }
  }
  s
}

/*
 * Compression of a vector of bytes
 * The compression is done by replacing each byte with its encoding
 */
fn compress(input: Vec<u8>) -> (String, String) {
  let data = group_bytes(&input); // group the bytes by 8
  let stats = stats(&data);       // get statistics on the values of the data (value, frequency)
  let huffman = huffman(&stats);  // get the Huffman encoding of the values (value, encoding)
  let mut compressed = String::new();
  for byte in data {                    // for each byte in the data
    for (value, encoding) in &huffman { // for each value and its encoding
      if byte == *value {               // if the byte is the value
        compressed.push_str(encoding);  // add the encoding to the compressed string
        break;
      }
    }
  }
  // return the compressed string and the encoding string
  (compressed, encoding_to_string(&stats, &huffman))
}

/*
 * Main function
 * Read a file, correct errors, reduce the encoding, convert to ASCII, decrypt, encrypt, compress
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
