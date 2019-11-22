#include <stdio.h>
#include <string.h>
#include <unistd.h>
#include <sys/stat.h>
#include <fcntl.h>

#include "crypto.h"
#include "common.h"

#define LEDGER_FILE "ledger.bin"
#define PERMISSIONS (S_IRUSR | S_IWUSR)

int main(int argc, char **argv) {
    struct cipher_params params;
    int fd = open(LEDGER_FILE, O_RDONLY, PERMISSIONS);

    unsigned char fd_key_hash[16];
    read(fd, fd_key_hash, 16);
    memcpy(params.key_hash, fd_key_hash, 16);

    unsigned char key[16];
    unsigned char *key_hash;
    memset(key, 0, 16);
    for (int i = 0; i < 256; i++) {
        key[0] = (unsigned char) i;
        for (int j = 0; j < 256; j++) {
            key[1] = (unsigned char) j;
            key_hash = md5_hash(key, 2);
            if (memcmp(key_hash, fd_key_hash, 16) == 0) {
                printf("Key is ");
                for (int k = 0; k < 16; k++) {
                    printf("%x", key[k]);
                }
                printf(".\n");
                goto outside_of_loop;
            }
            free(key_hash);
        }
        printf("Did i=%2x\n", i);
    }
    outside_of_loop:
    memcpy(params.key, key, 16);

    struct stat st;
    stat(LEDGER_FILE, &st);
    
    unsigned char fd_ctext_hash[16];
    read(fd, fd_ctext_hash, 16);
    read(fd, params.iv, 16);

    int ctext_len = st.st_size - 48;
    params.msg = malloc(ctext_len);
    params.len = ctext_len;
    read(fd, params.msg, ctext_len);

    unsigned char *ptext;
    int ptext_len = aes128_decrypt(&params, &ptext);

    printf("----- BEGIN LEDGER -----\n");
    for (int i = 0; i < ptext_len; i++) {
        printf("%c", ptext[i]);
    }
    printf("----- END LEDGER -----\n");

    free(params.msg);
    close(fd);
}