// https://codeforces.com/contest/1701/problem/B

#include <bits/stdc++.h>

using namespace std;

int main () {
	int t, n, count = 1;
	scanf("%d", &t);

	while (t--) {
		scanf("%d", &n);
		printf("2\n");
		for (int i=1; i<=n; i++) {
			if (i % 2 == 0)
				continue;
			for (int j=i; j<=n; j*=2) {
				printf("%d ", j);
			}
		}
		printf("\n");
	}

	return 0;
}
