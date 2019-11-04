#include <stdio.h>
#include <stdlib.h>
#include <stdint.h>
#include <string.h>
#include <time.h>

#define PASS_SIZE 16

int main(void) {
    /* password for admin to provide to dump flag */
    char *password;
	password = calloc(1, PASS_SIZE+1);

    /* seed random with time so that we can password */
	int t = time(0);
	
	for (int i = 0; i < 60; i++) {
		srand(t - i);
		
		for (int i = 0; i < PASS_SIZE; i++) {
			password[i] = rand() % ('z'-' ') + ' ';
		}

		printf("%.2d - %s\n", i, password);
	}

    return 0;
}
