# Projet Codage et Cryptographie

## Introduction

L'objectif de ce projet était de un programme mettant en oeuvre des notions de codage et de cryptographie vues en cours. Nous devions reproduire les traitements réalisés par David Albert Huffman sur une lettre codée en binaire lui ayant été envoyée par Richard Hamming afin de la déchiffrer pour enfin la envoyer à son destinataire final après l'avoir chiffrée à nouveau puis compressée.

Le choix du langage de programmation était libre, nous avons choisi de réaliser ce projet en Rust. Ce langage, n'étant pas enseigné en cours, nous a permis d'allier cet exercice à un apprentissage personnel.

## Etape 1 : Détecter et corriger les erreurs

La lettre codée en binaire a été envoyée par Richard Hamming, mais des erreurs de transmission se sont produites. Afin de les corriger, nous devions connaître le code de Hamming utilisé pour coder la lettre.

Nous savons que le codage a rendu la lettre plus longue d'un facteur 1,75. Ainsi, pour `x` bits de données, il y a `y = 0.75 * x` bits de contrôle. Un bit ne pouvant être divisé, les options possibles pour un nombre minimal de bits de données sont `x = 4` et ses multiples. 

Pour notre première hypothèse (segmentation la plus petite possible), nous avons donc `x = 4` et `y = 3` bits de contrôle, ce qui nous donne des mots de Hamming de 7 bits. Pour confirmer cette hypothèse, nous avons vérifié que la longueur de la lettre codée en binaire était bien un multiple de 7, or `47712 = 6816 * 7`.

Les mots de Hamming sont des mots de `n` bits désignant un alphabet limité de  moins de `2^n` symboles. Or, dans notre cas, nous savons que la lettre transmise contient deux erreurs (une au début et une à la fin). Pour vérifier que notre segmentation est correcte, nous avons donc divisé la lettre en mots de 7 bits et compté le nombre d'itérations de chaque occurrence avec la commande suivante :

```bash
cat doc/lettre.txt | grep -o -E '.{7}' | sort | uniq -c | sort -n
```

Ce qui nous a donné le résultat suivant :

```txt
      1 0011101
      1 1100111
     57 1111111
     75 1101010
     79 1011000
    105 1110100
    142 1010011
    155 0101100
    175 1001101
    200 0001011
    212 0100111
    213 1100001
    303 1000110
    329 0011110
    688 0000000
    982 0010101
   1359 0111001
   1740 0110010
```

Nous avons donc bien deux occurrences de 7 bits qui ne sont pas répétées (`0011101` et `1100111`), ce qui confirme notre hypothèse. Nous avons donc décidé de continuer avec cette segmentation.

Afin de pouvoir correctement corriger ces erreurs, nous devons connaitre comment les bits de contrôle sont calculés. Pour cela, nous avons utilisé un cas récurrent vu en cours : le code de Hamming (7,4,3). Dans ce cas, les bits de contrôle sont calculés de la manière suivante, pour sept bits de données `c1 c2 c3 c4 c5 c6 c7` :

- `c1`, `c2`, `c3` et `c4` sont les bits de données ;
- `c5` est le bit de parité pour les bits `c1`, `c2` et `c3` ;
- `c6` est le bit de parité pour les bits `c1`, `c2` et `c4` ;
- `c7` est le bit de parité pour les bits `c2`, `c3` et `c4`.

Nous avons donc implémenté une fonction en Rust permettant de détecter ces erreurs. Cette fonction prend en paramètre les sept bits de données et retourne un entier correspondant à l'erreur détectée. Si aucune erreur n'est détectée, la fonction retourne 0. Voici le code de cette fonction :

```rust
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
```

Si une erreur est détectée, il nous suffira juste d'inverser le bit erroné pour la corriger. 

```rust
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
```

Le résultat de cette étape apparaîtra dans le fichier `--1-corrected.txt`.

## Etape 2 : Supprimer les bits de contrôle

Une fois les erreurs corrigées, nous devons supprimer les bits de contrôle pour retrouver uniquement les bits de données. Pour cela, nous avons implémenté une fonction en Rust qui prend en paramètre un vecteur de bits et qui copie ces bits dans un nouveau vecteur en supprimant les bits de contrôle. Voici le code de cette fonction :

```rust
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
```

Le résultat de cette étape apparaîtra dans le fichier `--2-reduced.txt`.

## Etape 3 : Traduire les bits en caractères alphanumériques

Une fois les bits de données récupérés, nous devons les traduire en caractères alphanumériques. Voici le code de la fonction permettant de réaliser cette traduction :

```rust
fn convert_to_ascii(input: Vec<u8>) -> String {
  let s = String::from_iter(group_bytes(&input).iter().map(|v| { *v as char }));
  s
}
```

Chaque élément du vecteur `input` représentant un bit, nous devons regrouper ces bits par 8 pour former des octets. Pour cela, nous avons implémenté la fonction `group_bytes` :

```rust
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
```

Une fois les octets formés, nous pouvons convertir chaque octet (`u8`) en caractère alphanumérique simplement en castant chaque élément du vecteur en `char` et en les regroupant dans une chaîne de caractères.

Le résultat de cette étape apparaîtra dans le fichier `--3-ascii.txt`.

## Etape 4 : Déchiffrer la lettre

Une fois la lettre traduite en caractères alphanumériques, nous obtenons le message suivant :

```txt
Ac 02 fhff 1952, p Knyfnn Fbsz, N fsb ks qgmba, Qutp Tsoa Iskpbt, [...]
```

Pour déchiffrer cette lettre, nous devons connaître le chiffrement ainsi la clé de chiffrement utilisés. Le sujet nous indique que la lettre a été chiffrée avec un chiffrement polyalphabétique du XVIe siècle. Une recherche sur internet nous a rapidement orienté vers le chiffrement de Vigenère. Or, ce chiffrement s'appliquant sur des lettres, nous supposerons que chaque caractère différent d'une lettre sera ignoré par le chiffrement (ce qui laisse du sens aux chiffres, aux espaces et à la ponctuation dans le message chiffré).

Afin de pouvoir déchiffrer la lettre, nous avons implémenté une fonction en Rust permettant de déchiffrer, à partir d'une clé définie, un message chiffré avec le chiffrement de Vigenère. Voici le code de cette fonction :

```rust
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
```

Réaliser une attaque par force brute serait fastidieuse, nous allons donc essayer d'utiliser une attaque par dictionnaire. De plus, afin de réduire le domaine des clés à tester, nous utiliserons des indices laissés dans le sujet, ainsi que la sémantique supposée du message.

Nous ne connaissons pas la clé de chiffrement utilisée hormis le fait qu'elle soit liée au domaine des serpents. Ne connaissant pas la langue dans laquelle la lettre a été écrite, nous avons décidé de considérer les langues les plus probables : l'anglais (langue maternelle de Claude Shannon,  Richard Hamming et David Albert Huffman qui sont tous trois américains) et le français (langue dans lequel cette unité d'enseignement est dispensée).

En recoupant ces informations, nous pouvons supposer qu'une clé de chiffrement évidente serait `serpent` ou `snake` en anglais. Nous avons donc décidé de tester ces deux clés pour déchiffrer la lettre.

- Avec la clé `serpent`, nous obtenons le message suivant : `Iy 02 osbs 1952, w Sjhqja Mjoi, Y bfi so zrioh, Yqca Pfvi Ebvloa, [...]`
- Avec la clé `snake`, nous obtenons le message suivant : `Ip 02 fxbn 1952, c Kdunan Vxam, N voj xs gcuoa, Gqbc Tiki Vsaljg, [...]`

Ces deux résultats n'ayant pas de sens, ces clées sont donc incorrectes. Nous pourrions continuer à tester des clés de chiffrement correspondant à différentes espèces de serpents en anglais et en français. Cependant, il sera encore pertinant de réduire encore le domaine des clés à tester grâce à la sémantique supposée du message.

Le texte à déchiffrer, étant une lettre, nous pouvons supposer qu'au début d'une lettre, il puisse apparaître : une date, un nom, une adresse, une formule de politesse, etc. 

Or, le message commence par `Ac 02 fhff 1952, [...]`, "02" et "1952" semblant composer une date, nous pouvons supposer que le mot `fhff` désigne un mois. Etant composé de quatre lettres, nous pouvons supposer que ce mois peut être `mars`, `juin` ou `aout` en français ou `june` ou `july` en anglais.

Pour nous aider, nous avons utilisé la table de Vigenère suivante (la première ligne correspond à la lettre du message et la première colonne à la lettre de la clé, la lettre chiffrée est donc l'intersection de la ligne et de la colonne correspondant à la lettre du message et de la clé) :

Clé \ Message | A | B | C | D | E | F | G | H | I | J | K | L | M | N | O | P | Q | R | S | T | U | V | W | X | Y | Z
--- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | ---
A | A | B | C | D | E | F | G | H | I | J | K | L | M | N | O | P | Q | R | S | T | U | V | W | X | Y | Z
B | B | C | D | E | F | G | H | I | J | K | L | M | N | O | P | Q | R | S | T | U | V | W | X | Y | Z | A
C | C | D | E | F | G | H | I | J | K | L | M | N | O | P | Q | R | S | T | U | V | W | X | Y | Z | A | B
D | D | E | F | G | H | I | J | K | L | M | N | O | P | Q | R | S | T | U | V | W | X | Y | Z | A | B | C
E | E | F | G | H | I | J | K | L | M | N | O | P | Q | R | S | T | U | V | W | X | Y | Z | A | B | C | D
F | F | G | H | I | J | K | L | M | N | O | P | Q | R | S | T | U | V | W | X | Y | Z | A | B | C | D | E
G | G | H | I | J | K | L | M | N | O | P | Q | R | S | T | U | V | W | X | Y | Z | A | B | C | D | E | F
H | H | I | J | K | L | M | N | O | P | Q | R | S | T | U | V | W | X | Y | Z | A | B | C | D | E | F | G
I | I | J | K | L | M | N | O | P | Q | R | S | T | U | V | W | X | Y | Z | A | B | C | D | E | F | G | H
J | J | K | L | M | N | O | P | Q | R | S | T | U | V | W | X | Y | Z | A | B | C | D | E | F | G | H | I
K | K | L | M | N | O | P | Q | R | S | T | U | V | W | X | Y | Z | A | B | C | D | E | F | G | H | I | J
L | L | M | N | O | P | Q | R | S | T | U | V | W | X | Y | Z | A | B | C | D | E | F | G | H | I | J | K
M | M | N | O | P | Q | R | S | T | U | V | W | X | Y | Z | A | B | C | D | E | F | G | H | I | J | K | L
N | N | O | P | Q | R | S | T | U | V | W | X | Y | Z | A | B | C | D | E | F | G | H | I | J | K | L | M
O | O | P | Q | R | S | T | U | V | W | X | Y | Z | A | B | C | D | E | F | G | H | I | J | K | L | M | N
P | P | Q | R | S | T | U | V | W | X | Y | Z | A | B | C | D | E | F | G | H | I | J | K | L | M | N | O
Q | Q | R | S | T | U | V | W | X | Y | Z | A | B | C | D | E | F | G | H | I | J | K | L | M | N | O | P
R | R | S | T | U | V | W | X | Y | Z | A | B | C | D | E | F | G | H | I | J | K | L | M | N | O | P | Q
S | S | T | U | V | W | X | Y | Z | A | B | C | D | E | F | G | H | I | J | K | L | M | N | O | P | Q | R
T | T | U | V | W | X | Y | Z | A | B | C | D | E | F | G | H | I | J | K | L | M | N | O | P | Q | R | S
U | U | V | W | X | Y | Z | A | B | C | D | E | F | G | H | I | J | K | L | M | N | O | P | Q | R | S | T
V | V | W | X | Y | Z | A | B | C | D | E | F | G | H | I | J | K | L | M | N | O | P | Q | R | S | T | U
W | W | X | Y | Z | A | B | C | D | E | F | G | H | I | J | K | L | M | N | O | P | Q | R | S | T | U | V
X | X | Y | Z | A | B | C | D | E | F | G | H | I | J | K | L | M | N | O | P | Q | R | S | T | U | V | W
Y | Y | Z | A | B | C | D | E | F | G | H | I | J | K | L | M | N | O | P | Q | R | S | T | U | V | W | X
Z | Z | A | B | C | D | E | F | G | H | I | J | K | L | M | N | O | P | Q | R | S | T | U | V | W | X | Y

En testant les clés `mars`, `juin` et `aout` en français et `june` et `july` en anglais, nous obtenons les résultats suivants concernant les clés possibles :

- Si `fhff` correspond à `mars`, nous obtenons le morceau de clé : `..thon..`
- Si `fhff` correspond à `juin`, nous obtenons le morceau de clé : `..wnns..`
- Si `fhff` correspond à `aout`, nous obtenons le morceau de clé : `..ftlm..`
- Si `fhff` correspond à `june`, nous obtenons le morceau de clé : `..wnsb..`
- Si `fhff` correspond à `july`, nous obtenons le morceau de clé : `..wnuh..`

Nous pouvons donc supposer que le mot `fhff` correspond à `mars` et que la clé de chiffrement et donc le contenu du message sont en français car le morceau de clé obtenu est le plus cohérent parmi les résultats obtenus.

Nous avons donc identifié la langue du message et un morceau de la clé de chiffrement. Nous pouvons donc continuer notre analyse avec le premier mot du message chiffré `Ac` qui pourrait être un article précédant une date. Nous pouvons donc supposer que le mot `Ac` pourrait correspondre à l'article `Le`, or cela nous indiquerait que la clé de chiffrement commence par les caractères `py...`.

En mettant en correspondance les lettres du message chiffré avec les lettres du message déchiffré, nous obtenons le morceau de clé suivant : `python`, qui est bien une espèce de serpent. Nous avons donc essayé de déchiffrer le message avec cette clé et nous obtenons le début de message suivant :

```txt
Le 02 mars 1952, a Murray Hill, A qui de droit, Cher Alan Turing, [...]
```

Nous avons donc réussi à déchiffrer la lettre. Le résultat de cette étape apparaîtra dans le fichier `--4-decrypted.txt`.

## Etape 5 : Chiffrer la lettre

## Etape 6 : Compresser la lettre sa clé de chiffrement
