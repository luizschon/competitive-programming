#include <stdio.h>

double fracao_continua_raiz (int N)
{
    return (N>1 ? 2+1/fracao_continua_raiz(N-1) : 2); 
}

int main ()
{
    int N;
    scanf("%d",&N);
    if (N==0) printf("%.10lf\n", 1.);
    else printf("%.10lf\n",1.+1./fracao_continua_raiz(N));
    
    return 0;
}
