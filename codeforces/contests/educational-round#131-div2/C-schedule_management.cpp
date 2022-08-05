// https://codeforces.com/contest/1701/problem/C

#include <bits/stdc++.h>

using namespace std;

int main () {
	int t, n, m, temp, pos, time=0;
	vector<int> tasks;
	scanf("%d", &t);

	while (t--) {
		scanf("%d %d", &n, &m);
		for (int i=0; i<m; i++) {
			scanf("%d", &temp);
			tasks.push_back(temp);
		}

		while (tasks.begin() != tasks.end()) {
			for (int i=1; i<=n; i++) {
				pos = find(tasks.begin(), tasks.end(), i) - tasks.begin();
				if (pos >= tasks.size()) {
					tasks.erase(tasks.end());
					time += 2;
				}
				else {
					tasks.erase(tasks.begin() + pos);
					time += 1;
				}
			}
		}
		printf("%d\n", time);
	}

	return 0;
}
