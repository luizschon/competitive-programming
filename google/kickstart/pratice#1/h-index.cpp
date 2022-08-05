#include <bits/stdc++.h>

using namespace std;

int h_index (int n, vector<int> c) {
    int sum = 0, cur;
    int lookup[n+1];
    memset(lookup, 0, (n+1)*sizeof(int));

    for (int i = 0; i < n; i++) {
        cur = c[i];
        if (cur > n) {
            lookup[n]++;
        } else {
            lookup[cur]++;
        }
    }

    for (int i = n; i >= 0; i--) {
        if (lookup[i]) {
            sum += lookup[i];
        }
        if (sum >= i) {
            return i;
        }
    }
    return 0;
}

int main () {
    cin.sync_with_stdio(false);
    cin.tie(NULL); cout.tie(NULL);

    int t, n, cit;
    vector<int> c;
    cin >> t;
    
    for (int i = 1; i <= t; i++) {
        cin >> n;
        cout << "Case #" << i << ":";

        for (int j = 1; j <= n; j++) {
            cin >> cit;
            c.push_back(cit);
            cout << " " << h_index(j, c);
        }
        cout << endl;
        c.clear();
    }
    
    return 0;
}

