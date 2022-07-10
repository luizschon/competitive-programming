#include <stdio.h>

int main ()
{
    int dias, meses, anos;
    scanf("%d", &dias);

    anos = dias/365;
    dias %= 365;
    meses = dias/30;
    dias %= 30;

    printf("%d ano(s)\n", anos);
    printf("%d mes(es)\n", meses);
    printf("%d dia(s)\n", dias);

    return 0;
}
