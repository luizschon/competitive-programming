#include <stdio.h>

int main () {
    int init_i = 1, init_j = 7, j_it = 3, final_i = 9;

    for (int i=init_i; i<=final_i; i+=2, init_j+=2)
        for (int j=init_j; j>init_j-j_it; j--)
            printf("I=%d J=%d\n", i, j);

    return 0;
}
