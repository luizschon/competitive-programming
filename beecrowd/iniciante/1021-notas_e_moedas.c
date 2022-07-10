#include <stdio.h>

int main ()
{
    double valor;
    int n100, n50, n20, n10, n5, n2, m100, m50, m25, m10, m5, m1;
    int valor_int, valor_deci;
    scanf("%lf", &valor);
    
    valor_int = (int)valor;
    valor_deci = (int)(valor*100)%100;
    
    n100 = valor_int/100;
    valor_int %= 100;
    n50 = valor_int/50;
    valor_int %= 50;
    n20 = valor_int/20;
    valor_int %= 20;
    n10 = valor_int/10;
    valor_int %= 10;
    n5 = valor_int/5;
    valor_int %= 5;
    n2 = valor_int/2;
    valor_int %= 2;
    m100 = valor_int;
    m50 = valor_deci/50;
    valor_deci %= 50;
    m25 = valor_deci/25;
    valor_deci %= 25;
    m10 = valor_deci/10;
    valor_deci %= 10;
    m5 = valor_deci/5;
    valor_deci %= 5;
    m1 = valor_deci/1;

    printf("NOTAS:\n");
    printf("%d nota(s) de R$ 100.00\n", n100);
    printf("%d nota(s) de R$ 50.00\n", n50);
    printf("%d nota(s) de R$ 20.00\n", n20);
    printf("%d nota(s) de R$ 10.00\n", n10);
    printf("%d nota(s) de R$ 5.00\n", n5);
    printf("%d nota(s) de R$ 2.00\n", n2);
    printf("MOEDAS:\n");
    printf("%d moeda(s) de R$ 1.00\n", m100);
    printf("%d moeda(s) de R$ 0.50\n", m50);
    printf("%d moeda(s) de R$ 0.25\n", m25);
    printf("%d moeda(s) de R$ 0.10\n", m10);
    printf("%d moeda(s) de R$ 0.05\n", m5);
    printf("%d moeda(s) de R$ 0.01\n", m1);

    return 0;
}
