// https://codeforces.com/contest/1701/problem/D

#include <bits/stdc++.h>

using namespace std;

int main () {
	int t, n, temp;
	scanf("%d", &t);

	while (t--) {
		scanf("%d", &n);
		for (int i=0; i<n; i++) {
			scanf("%d", &temp);
			if (temp == 0)
				printf("1 ");
			else {
				printf("%d ", (int)ceil((i+1)/temp));
			}
		}
		printf("\n");
	}
	return 0;
}
