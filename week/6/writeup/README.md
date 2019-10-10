# Writeup 6 - Binaries I

Name: Simon Berger
Section: 0101

I pledge on my honor that I have not given or received any unauthorized assistance on this assignment or examination.

Digital acknowledgement: Simon Berger

## Assignment Writeup

### Part 1 (50 pts)

CMSC389R-{di5a55_0r_d13}

### Part 2 (50 pts)

First I just ran the crackme as is, just to see what would happen, and it told me to disassemble it. Before doing so, I tried running it give some arguments, and it told me that its input what some multi-word quoted thing. Which I tried a few things, to no avail.

So I then opened Binary Ninja, where I quickly realized that main called 3 check functions, `check1`, `check2`, and `check3` to output the flag if I plug in the correct things, whatever they were. The program, in C, was written to get the output from each check, then do an if after each one, to give a hint depending on a pass or fail. Then, at the end, if each passed, it would do some hidden work to output the flag (hidden so we couldn't just search the binary for the flag).

`check1` was very simple. It just does a strcmp against a built in string, `"Oh God"`. It's all in one string, so I had to quote it on the command line, like crackme recommended. At last, it told me the next hint, "I wish you cared more about the environment".

So I went to `check2`, and saw that it checked FOOBAR for what looked to be `"seye my "`. When I put that in, it said I almost had it, so I thought I was done with `check2`.

I went on to `check3` and saw a switch statement, which checked the value of a file, `sesame`, byte by byte. The values it checked against were in hexadecimal, so I didn't bother converting them to ascii, and just did a straight echo. `echo -e "\x20\x74\x68\x65\x79\x20\x62\x75\x72\x6e" > sesame`

However, nothing happened, but I was *sure* I was done. So I went and searched through the binary where `You've almost got this!` was. Turned out, I had still failed `check2`! I ended up having to go through it, instruction by instruction, to see what a loop at the end of `check2` did, which I clearly wasn't passing, but needed to. Some deciphering let me see it was a for loop, that checked the input string against `"seye my "` backwards. A quick facepalm and a reversal later, lead to the final command, `FOOBAR=" my eyes" ./crackme "Oh God"`, which then outputted the final flag.