# Writeup 2 - OSINT

Name: Simon Berger
Section: 0101

I pledge on my honor that I have not given or received any unauthorized assistance on this assignment or examination.

Digital acknowledgement: Simon Berger

## Assignment Writeup

### Part 1 (45 pts)

1. What is `ejnorman84`'s real name?  
    * It's Eric Norman, and I found this out by going to his instagram.

2. Where does `ejnorman84` work? What is the URL to their website?
    * He works at Wattsamp Energy. The url is http://wattsamp.net/. This link to this website is in his instagram profile.


3. List all personal information (including social media accounts, contacts, etc) you can find about `ejnorman84`. For each, briefly detail how you discovered them.
    * [Instagram](https://www.instagram.com/ejnorman84/). I double checked to see if it existed just by typing the URL.
    * [Reddit](https://www.reddit.com/user/ejnorman84). I double checked just by typing in the URL.
    * UMDCSEC Hackers found his info and posted a little thing on [pastebin](https://pastebin.com/4yJRgkFm). This was found using DuckDuckGo "ejnorman84".
    * Eric Norman is registered on WHOIS at 1300 Adabel Dr, El Paso, TX  79835. His phone is 2026562837 and his email is ejnorman84@gmail.com. The phone number is registered using Google Voice.

4. List any ( >= 1 ) IP addresses associated with the website. For each, detail the location of the server, any history in DNS, and how you discovered this information.
    * 157.230.179.99 can be found by doing a DNS lookup on wattsamp.net. It was registered on 2019-09-04 using Google as a registrar, and expires a year from then. 

5. List any hidden files or directories you found on this website.  
    * Couldn't find anything, unfortunately.

6. What ports are open on the website? What services are running behind these ports? How did you discover this?
    * 22/tcp is ssh
    * 80/tcp is http
    * 1337/tcp is "waste"
    * 2000/tcp is cisco-sccp
    * This was all found using nmap.

7. Which operating system is running on the server that is hosting the website? How did you discover this?
    * Going to http://wattsamp.net/views/ shows that they didn't set up their apache permissions properly, so you can see what's in that folder. It shows some extra info about Apache, which says it's running Ubuntu.

8. **BONUS:** Did you find any other flags on your OSINT mission? Note: the standard flag format for bonus flags is `*CMSC389R-{}`. (Up to 9 pts!)
   * Found `*CMSC389R-{html_h@x0r_lulz}` in the html of http://wattsamp.net/.

### Part 2 (75 pts)

*Please use this space to detail your approach and solutions for part 2. Don't forget to upload your completed source code to this /writeup directory as well!*

I started with the stub file, which I assume you guys are running with Python 2, which is what `python` normally goes to in Kali unfortunately. It should support Python 3, though, however it is a very quick and dirty script.

Instead of going with threads, I was just going to read in as much as I needed to, in order to have the OS wait for me, instead of sleeping in an imprecise way. Also, trying to thread would've been possible, but I didn't think the complication was really *that* necessary.

I'm assuming the password brute force list is already unzipped. So I just read that in and split on each newline to have a list. I also coded in a mechanism to show how far I was in the file, so I would be able to continue at whatever point if I stopped the program.

For the actual testing part, I solved the captcha using eval, and just using string split to grab that part quickly, since I'm not as used to Python. This might not be the best idea to use in the real world, but for something quick and dirty like this? It's fine. Then, I sent the username and then the password I was trying, and seeing if I got in or not.

Unfortunately, the server crapped out and wouldn't respond, so I couldn't try and hack in before the deadline. This is being submitted now, but I'm going to come back tomorrow to complete this fully, but just some late points off. Were I able to access it on time, though, I'd probably have `cd`'d around, and tried using `find` to see if I could find anything related to a flag. And also look at suspicious files. But alas, I couldn't complete this on time... stupid server.
