# Writeup 10 - Crypto I

Name: Simon Berger
Section: 0101

I pledge on my honor that I have not given or received any unauthorized assistance on this assignment or examination.

Digital acknowledgement: Simon Berger


## Assignment details

### Part 1 (45 Pts)
#### 1
From 0 to 15, 16 bytes, is the md5 hash of the key (which is a hash) `fd_key_hash` or `params->key_hash`.
From 16 to 31, 16 bytes, is the md5 hash of ciphertext `ctext_hash` or `ctext_hash`.
From 32 to 47 is the IV, which was randomly generated when the ledger was created, which is `params->iv`.
Starting from 48 is an aes128 encrypted ciphertext, which has a variable length.
The plaintext is just text from getline, separated by `\n`s.

#### 2
md5 is used, which was broken a while ago, and has only grown more vulnerable.
aes128 is used, which is fine, if it's inputs as done correctly. I'm not sure if this is just straight application of the key to each block, or whether it's using something like CBC, but if not, then it's more insecure.
openssl is used, which may or may not be on an old vulnerable version, which might cause the above two to be more vulnerable, or for `RAND_bytes` to be vulnerable.

#### 3
We have an md5 hash of the key (which is a hash), an md5 hash of the cipher text, an IV, and a length of the ciphertext, which maps to the length of plaintext, plus padding.

#### 4
The application ensures Confidentiality by encrypting the ledger file using aes128 crypto.
The key is derived from the first two bytes of a md5 hash of the password, with the rest of the bytes zeroed.

#### 5
The application ensures Integrity with an md5 hash of the ciphertext in the header of the file.
However, an attacker can freely modify the ciphertext, and update the ctext hash accordingly, and the user won't realize their text was modified beyond some garbled plaintext.
The hash should be a hmac instead, so that a key can be used so that an attacker can't spoof this, too.

#### 6
The application ensures Authenticity by checking against an md5 hash of the key that's saved in the header.
This is flawed, because the hacker can just always use and save the given hash, to trick the user, since this value is freely saved.

#### 7
The IV is generated using the cryptographically secure `RAND_bytes` openssl function, and saved in the ledger's header, on the first run of the ledger program.
This is a fine place to save it, and it was generated securely.
However, every time the file is saved, it should be generating a different IV to use each time, so that info isn't leaked about the key from having an identical message plus some extra.

### Part 2 (45 Pts)

`CMSC389R-{k3y5p4c3_2_sm411}`

### Part 3 (10 Pts)
I think security stuff that gets widely used, like particular algorithms like DES and AES, should be well known and well tested, along the thinking of Kerckhoffs's principle, but that specific details of particular projects, like a banking application, should be kept under wraps, so that that project's exposed to less risk.
A counter argument is that security through obscurity will also help particular algorithms. Imagine if so many people weren't trying to bruteforce DES? Then it would have taken longer to break it, for example.
