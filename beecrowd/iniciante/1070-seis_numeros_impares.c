#include <stdio.h>

#define N_ODD 6

int main () {
    int num, n_odd = N_ODD;
    scanf("%d", &num);

    // is even
    if (num % 2 == 0)
        num++;
    
    while (n_odd--) {
        printf("%d\n", num);
        num += 2;
    }

    return 0;
}
