#include <stdio.h>

int fatorial (int N)
{
    return (N>1 ? N*fatorial(N-1) : 1); 
}

int main ()
{
    int N;
    scanf("%d",&N);
    printf("%d\n",fatorial(N));
    
    return 0;
}
