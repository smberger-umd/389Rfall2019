# Writeup 8 - Binaries II

Name: Simon Berger
Section: 0101

I pledge on my honor that I have not given or received any unauthorized assistance on this assignment or examination.

Digital acknowledgement: Simon Berger

## Assignment Writeup

### Part 1 (100 pts)
Answer the following questions regarding the server executable (for which source is provided).

1. How is the per-session administrator password generated? Are there any inherent weaknesses in this implementation?
    * It seeds `rand` with the current second, and then loops through the constant length of the password, calling rand for each character. The modulus and addition transform the random byte to make the number be a character in ascii, from a space to a z, not including the z. (The z was probably meant to be included, but isn't, so off by one error.)
    * If `rand` did generate a number uniformly throughout its output space, then the modulus would make that transformed output non-uniform, making some passwords more likely than others.
    * This isn't the big problem though. If I know what *second* the program is seeded with, I can calculate the precise password, just by passing in that number into `srand` and running the password generation function again, in my own code. This second is exactly when I connect, so it's very easy to almost immediately get the password. `rand` and `srand` are pseudo-random generators and are not cryptographically secure for password generation. 

2. Describe two vulnerabilities in this program. Provide specific line numbers and classifications of the vulnerability. Explain potential ramifications as well as ways to avoid these vulnerabilities when writing code.

    * On line 46, `output` get passed directly into `printf`, a **Format String Vulnerability**. `output` comes from a modified `input`, which comes directly from stdin. This allows me to have `printf` print any formatting that I want, which gives me some manipulation of the program's memory, primarily allowing me to read it.

    * On line 68, there is a `gets`, which gives a **Buffer Overflow Vulnerability**. This means I can output code and other stuff into the buffer, and likely can get arbitrary code execution.

3. What is the flag?
    * CMSC389R-{expl017-2-w1n}

4. Describe the process you followed to obtain the flag: vulnerabilities exploited, your inputs to the server in a human-readable format, etc. If you create any helper code please include it.

    * Trying to use a printf vulnerability would be a pain, so I just used the `srand` exploit to generate a password list to try. If when I connected to the server and that generated the list at the same time, the first one was the password the server picked. This program to generate it is in `writeup/print_pass.c`.
    * So now I can run the three given commands (or a subset, like `ls /`). The plan is to use `gets` to overflow the accepted commands buffer.
    * The input buffer is 33 bytes long, so with `cat /flag; ` being 11 bytes, we can repeat it for a total of 3 times, to fill our input buffer. Then, we can repeat the construction again, so that the whitelist will contain our command. The command is `cat /flag; cat /flag; cat /flag; cat /flag; cat /flag; cat /flag; `. Note that the space after the third semicolon gets turned into a nul byte, so the last space in the entered text can actually be omitted, and the server will still accept and run the command.
    * And boom, the flag printed out three times.
