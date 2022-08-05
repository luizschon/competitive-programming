// BSEARCH1 - Binary search (https://www.spoj.com/problems/BSEARCH1/)
#include <stdio.h>

#define MAX_SIZE 100000

int bsearch(int * arr, int beg, int end, int el) {
    if (beg > end) {
        return -1;
    }

    int mid = (beg+end)/2;

    if (el == arr[mid]) {
        if(mid == 0 || el != arr[mid-1]) {
            return mid;
        } 
        return bsearch(arr, beg, mid-1, el);
    }

    if (el < arr[mid]) {
        return bsearch(arr, beg, mid-1, el);
    }
    return bsearch(arr, mid+1, end, el);
    
}

int main() {
    int N, Q, el;
    int ordered_arr[MAX_SIZE];
    scanf("%i %i", &N, &Q);

    for(int i=0; i<N; i++) 
        scanf("%d", &ordered_arr[i]);

    while(Q--) {
        scanf("%d", &el);
        printf("%d\n", bsearch(ordered_arr, 0, N-1, el));
    }

    return 0;
}
