#include <stdio.h>

int main ()
{
    double r;
    scanf("%lf", &r);

    printf("VOLUME = %.3lf\n", (4./3.)*3.14159*r*r*r);

    return 0;
}
