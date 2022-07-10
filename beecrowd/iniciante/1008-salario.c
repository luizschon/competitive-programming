#include <stdio.h>

int main ()
{
    int num, horas;
    float salario;
    scanf("%d%d%f", &num, &horas, &salario);

    printf("NUMBER = %d\nSALARY = U$ %.2f\n", num, salario*horas);

    return 0;
}
