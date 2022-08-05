// vim: noai:ts=4:sw=4
#include <bits/stdc++.h>

#define pb push_back
#define mp make_pair

typedef long long ll;
typedef unsigned long long ull;
typedef unsigned int u;

using namespace std;

int main () {
    iostream::sync_with_stdio(false);
    cin.tie(NULL); cout.tie(NULL);

    int k, n, m, x, y;
    

    while (cin >> k && k != 0) {
        cin >> n >> m;
        while (k--) {
            cin >> x >> y;
            if (x == n || y == m)
                cout << "divisa";
            else if (x > n && y > m)
                cout << "NE";
            else if (x < n && y > m)
                cout << "NO";
            else if (x < n && y < m)
                cout << "SO";
            else if (x > n && y < m)
                cout << "SE";
            cout << endl;
        }
    }

    return 0;
}
