# Writeup 9 - Forensics II

Name: Simon Berger
Section: 0101

I pledge on my honor that I have not given or received any unauthorized assistance on this assignment or examination.

Digital acknowledgement: Simon Berger


## Assignment details

### Part 1 (45 Pts)

1. 159.203.113.181
2. Netcat to find which services the server is serving/what ports are open on the server.
3. 142.93.136.81 on DigitalOcean from the Netherlands.
4. The server is sending FTP data over port 20, after they connect to port 21 to control the server over FTP.
5. They stole a `findme.jpeg` file, an image file. I don't recognize it; it looks like a few people standing on/by a statue of a giant hand buried in the sand at the beach.
6. They left behind `greetz.fpff`.
7. They could have secured the server using a password, by using something like SFTP, which works over SSH, instead of leaving an open FTP service.

### Part 2 (55 Pts)

Here's fpffparse's output, when called on greetz.fpff.

```
This file was authored on 2019-03-27 04:15:05 UTC by fl1nch.
Section 1 is ASCII of "Hey you, keep looking :)".
Section 2 is A Coord of x: 52.336035, y: 4.880673.
Section 3 is a PNG. Saving as 3.png.
Section 4 is ASCII of "}R983CSMC_perg_tndid_u0y_yllufep0h{-R983CSMC".
Section 5 is ASCII of "Q01TQzM4OVIte2hleV9oM3lfeTBVX3lvdV9JX2RvbnRfbGlrZV95b3VyX2Jhc2U2NF9lbmNvZGluZ30=".
```

Here are the 3 flags:

CMSC389R-{w3lc0me_b@ck_fr0m_spr1ng_br3ak}
CMSC389R-{h0pefully_y0u_didnt_grep_CMSC389R}
CMSC389R-{hey_h3y_y0U_you_I_dont_like_your_base64_encoding}
