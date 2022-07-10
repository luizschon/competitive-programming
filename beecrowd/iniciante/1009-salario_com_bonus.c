#include <stdio.h>

int main ()
{
    char nome[50];
    double sal, mont;
    scanf("%s%lf%lf", nome, &sal, &mont);

    printf("TOTAL = R$ %.2lf\n", mont*0.15+sal);

    return 0;
}
