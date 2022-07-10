#include <stdio.h>

int main ()
{
    int valor, tmp, notas100, notas50, notas20, notas10, notas5, notas2, notas1;
    scanf("%d", &valor);
    
    tmp = valor;
    notas100 = tmp/100;
    tmp %= 100;
    notas50 = tmp/50;
    tmp %= 50;
    notas20 = tmp/20;
    tmp %= 20;
    notas10 = tmp/10;
    tmp %= 10;
    notas5 = tmp/5;
    tmp %= 5;
    notas2 = tmp/2;
    tmp %= 2;
    notas1 = tmp;

    printf("%d\n", valor);
    printf("%d nota(s) de R$ 100,00\n", notas100);
    printf("%d nota(s) de R$ 50,00\n", notas50);
    printf("%d nota(s) de R$ 20,00\n", notas20);
    printf("%d nota(s) de R$ 10,00\n", notas10);
    printf("%d nota(s) de R$ 5,00\n", notas5);
    printf("%d nota(s) de R$ 2,00\n", notas2);
    printf("%d nota(s) de R$ 1,00\n", notas1); 

    return 0;
}
