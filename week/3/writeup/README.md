# Writeup 3 - OPSEC & Social Engineering

Name: Simon Berger
Section: 0101

I pledge on my honor that I have not given or received any unauthorized assistance on this assignment or examination.

Digital acknowledgement: Simon Berger

## Assignment Writeup

### Part 1 (40 pts)
Me: Hello, is this Eric Norma? This is \[your bank\] calling. (Assuming I can figure out his bank using OSINT.)
Eric: Yes, this is Eric Norman. What do you need?
Me: Unfortunately, we've had a bit of a technical error with our databases here at the bank.
Eric: Oh crap, really?
Me: Yes. Thankfully, though, your money is safe and sound, but bits of our databases were corrupted.
Eric: Yikes.
Me: Most of our user accounts are fine, but some accounts lost some details like their email and password information, while others lost password reset information and their phone numbers. A great deal of your account's info was lost, but we do have your phone number and picture on file. While you could come in to set everything back up, it would probably be easier for us both if we just went over things online.
Eric: Yeah I am kinda busy...
Me: If we can't do this now, we can book a time for me to call you back later.
Eric: No, I can do this now. It'll be quick, right?
Me: Yeah, we should be able to finish this quickly. Again, I'm sorry that you have to deal with this sort of thing.
Eric: No, no! I'm sure you've been calling people for days; you must be so busy. Let me help you get this out of the way as fast as we can.
Me: Thanks. Alright, I've got a site you can visit so you can take a picture using your webcam to confirm you're really Eric Norman. \[Bank\] policy, you know? You'll also need to take a picture of your license, passport, or birth certificate, then you can reenter all your old information. Sound good?
Eric: Yeah. What's the site.
Me: Alright, enter in \[custom website with similar url of bank to pwn this noob; with an ending like he might get in an email to "log in" automatically, since his password wouldn't work since it was corrupted\].

Eventually I'll walk him through confirming it's really him, then for security questions, the site I made will have him enter his mother's maiden name, the city he was born in, and the name of his first pet. One of the things missing would be his ATM pin number, that he'd need to enter, and the site would detect his browser. I could also have a lot of other public information already inputted, that I could have him double check.

If he's suspicious of the EV lock not showing, and instead being a normal lock, I can say our engineers set up the site separately, since they were trying to get things done as fast as possible. If necessary, I can scare him off by starting say how it would be insecure copying this super important private key, yadda yadda. If he's confused why I called him, thinking it's a scam, I could say that if he'd prefer, we can send the mail that banks always send (since calls are scams), but we wanted to call now to save time, effort, and paper. And the planet. He's free to go to his bank, but we're trying to keep the database corruption on the down low, since it's bad PR, you know, so most people in the bank don't know about it. And that I'd really rather get this done now, and make it seem like I've had to do several other people. If he's suspicious about having just used ATM or the bank website, feign surprise and excitement, and ask if I can have him check some stuff, since IT thought all the information might be deleted, but if he was able to use ATM/bank website, it's probably cached somewhere, and if we act quick, we might be able to save other people from data corruption. Or we could just enter his information now, because we probably won't be able to recover the cached data, and I don't want to call back in a few days since he's on the phone now.

### Part 2 (60 pts)

#### 3 Vulnerabilies
1. Leaked Password
2. Running unneccessary services on public machine.
3. Services on a public machine don't follow [principle of least privilege](ttps://searchsecurity.techtarget.com/definition/principle-of-least-privilege-POLP).

#### Suggestions for Each Vulnerability
1. Use [Have I been pwned?](https://haveibeenpwned.com/) to automatically get emails on if your password was leaked, for any site you have an account with. This way, you might get warning that you should change passwords, so that a hacker couldn't hack into it. In addition, you should use a password manager like [KeePass](https://keepass.info/) with cloud storage, or [LastPass](https://www.lastpass.com/). This way, every password for every site you use would all be different, so even if hackers got access to one password on one site, they wouldn't be able to use that to get into any other account, because all the passwords would be randomly generated and all different. Also, since each password would be randomly generated, it would be very difficult for a hacker to brute force through the password. This is all ignoring how nice it is to have a list of sites you have accounts on, and never having to remember a password again, excepting a rare few (for logging onto your computer/phone, or other passwords you have to enter multiple times per day).
2. Don't have extra services that someone can exploit that aren't necessary. You want to [reduce your attack surface](https://www.securitymagazine.com/articles/89283-ways-to-reduce-your-attack-surface). So you shouldn't leave some random, insecure service running on 1337. It would probably be best to run each service, like www, each entirely on their own virtual machines, so one service being compromised wouldn't affect others. Thus, if you want to run some 1337 service, you can, but do it on a separate VM with nothing else important on the system, in case it gets hacked. That, or see if you can find a more secure alternative to that service.
3. Everything on a system should only be able to access the minimum amount of stuff that it needs to work, which is the [principle of least privilege](https://searchsecurity.techtarget.com/definition/principle-of-least-privilege-POLP). You should configure Apache properly, so that you can't browse through folders if they don't have an index.html, to get a file listing inside that directory. You should make sure you can't even login with a common password on your account to get in, but instead only log in to change stuff using a secure private key. You shouldn't be using wordpress, but rather a static site generator, so that no server-side scripts need to run, adding to the attack surface. You should make sure public services have the minimum permissions possible, so that even if ssh or apache were hacked, they'd only be able to access their designated folders, and apache, for example, wouldn't even be able to write to its www files.

