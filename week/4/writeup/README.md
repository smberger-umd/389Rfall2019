# Writeup 4 - Pentesting

Name: Simon Berger
Section: 0101

I pledge on my honor that I have not given or received any unauthorized assistance on this assignment or examination.

Digital acknowledgement: Simon Berger

## Assignment Writeup

### Part 1 (45 pts)

#### How I got there

At first, I had no clue how to get it. Just copying shellshock, and many permutations weren't working. So I started smaller. I realized that if I typed `-c 5 1.1.1.1`, it would ping 5 times instead of 2. For some reason, it doesn't go over 5. Similarly, I can type `1.1.1.1 -c 5` or `-c 5 -- 1.1.1.1`. My guess is that they just dump the results into a python `os.system` call, using an f=string or something similar, and don't print anything on an error. Indeed, doing `\`echo 1.1.1.1\`` works, since bash turns it into `1.1.1.1`.

So basically, I have shell execution, but don't have stdout, since the subshell saves the stdout and turns it into an argument. It took a moment to realize it, but all I have to do redirect using `/proc`. So `-c 1 1.1.1.1 $(echo "hi" > /proc/$$/fd/1)`. Interestingly enough, using backticks doesn't work, though.  However, using `$(...)`, it prints regardless of whether ping succeeds or fails. Our final command to use is: `$((...) > /proc/$$/fd/1)` or `$((...) &> /proc/$$/fd/1)` to also capture stderr.

#### Flags and other notables

Some poking around leads to me printing the **entire** bash script that takes my input, calls ping, etc! It's at `/opt/container_startup.sh`. In any case, the flag is at `/home/flag.txt`, and is `CMSC389R-{p1ng_as_a_$erv1c3}`, using `$((ls /home/**) > /proc/$$/fd/1)` then `$((cat /home/**) > /proc/$$/fd/1)`.


#### Suggested Precautions

First, yay for having each request execute in its own docker container! Note, though, that this can be DDOS'd and even just DOS'd really easily, since Docker is *not* lightweight. In all seriousness, I know this is for the students, but still, I gotta say it.

It's *really* bad that the bash script runs as root. Really freaking bad!. Whatever program that does get run really should **not** be root.

Second, the server loop shouldn't be directing towards a bash script. Bash is *way* to easy to accidentally introduce vulnerabilities, like this. Instead, the server loop should execute a separate program, made directly for this, so that types and good APIs would mean you couldn't accidentally mix data and code, like with bash.

Even better would be just replacing the server loop with a separate program, and running the whole server program with really low execution privileges. This way, it's not as heavyweight and susceptible to DDOS attacks, and you wouldn't be doing fancy redirection stuff that can be looked at by other processes. If the bash script and the server loop weren't separate, I wouldn't have been able to intercept stdout in order to even see what I was executing.

I'd recommend writing such a program in Rust, which is great for this for several reasons. It's type system and APIs are top notch, so that it's really hard to accidentally mix data and code, like it can be in scripting languages. It's also compiled, not garbage collected, and has a minimal runtime, so that it's a lot harder to do a DDOS attack, since it can take so many requests at once. It's async/await support is about to be stable, making it even easier to write code that's even *more* efficient.

I would also implement some sort of authentication system and a limiting system, because otherwise the server could be swamped with requests, that it may or may not be able to handle, but more importantly, might flood the network, and not be wanted. Only Wattsamp Energy employees that need to use the Uptime Monitor should be able to access and use it, and they shouldn't be able to overuse it, either.


### Part 2 (55 pts)

My code is in the `wattsampshell` folder, `wattsamp/src/main.rs` specifically, and the instructions for running it are in wattsampshell/README.txt.

After figuring out the hard bit from Part 1, this part was relatively straight forward, even if it took a while.

First off, I knew I would do this in Rust. For Week 2's assignment, I did it in Python, and it was horrible trying to go and program it. I am not a fan of Python, and it's a struggle making a program that isn't broken in some way, since there's no static analysis. No type checker, no good type system, no lifetimes. Rust is a lot nicer, since it's typically only small, easy to find bugs, where there's large flaws in whicher logic I come with.

Second, in Rust, I went with the Beta of 1.39. This is because that's when async and await goes live, so this could quite possibly be the same version that people use in the future. In any case, I knew I'd probably have to use timeouts and other fancy bits, so I went with the async version of the standard library, instead of the sync version. However, it *is* a bit rough at the moment, but everything's about to be stabilized, so it's fine.

Besides that, I grabbed a few libraries. One, was the `async_std` I mentioned a moment ago, to do IO and net code with. Two was a rust readline library, which sadly isn't async, but my usecase wasn't so complex that it was wholly necessary, though I wouldn't want to publish this without having it be properly async. I have the `futures-preview` library, which is necessary to use `async_std` with. Then I have `simple-error` to do stringly typed errors, which is fine, since this is a small program. Finally, I have `path-clean`, so that my cd path behavior actually collapsed any `..`s.

So then I started writing. I wrote the main loop, with the readline library, and did exit and help pretty easily. Then I set up the pull part, which took a lot of fiddling to make the network code just right. After that, I copied that over to shell_call, where it's largely, but not exactly, the same. At the same time as I fiddled with shell_call, I set up shell, which was a copy and modification of the main loop. However, after I had everything done, I realized that I forgot to do the cd in the particular way you're supposed to, so I had to go and add that, where I just used PathBuf to handle cd.

Ultimately the pull call is just a `cat` and saving what the server sends back, while shell is making another shell loop where you split into the cd logic and the shell_call logic. The shell_call logic is very similar to the pull call, except that it can be any command, but it has to be preceded by a cd to the current path, because each command isn't really apart of the same session. In any case, it was definitely some good practice for rust async stuff, even if it took longer than I had wanted.