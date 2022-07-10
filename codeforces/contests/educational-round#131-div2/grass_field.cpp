// https://codeforces.com/contest/1701/problem/A

#include <bits/stdc++.h>
#include <cstdio>

using namespace std;

int main () {
	int t, grass_c=0, temp1, temp2;
	scanf("%d", &t);

	while (t--) {
		grass_c = 0;
		for (int i=0; i<2; i++) {
			scanf("%d %d", &temp1, &temp2);
			grass_c += temp1 + temp2;
		}
		if (grass_c == 0)
			printf("0\n");
		else if (grass_c < 4)
			printf("1\n");
		else 
			printf("2\n");
	}

	return 0;
}
