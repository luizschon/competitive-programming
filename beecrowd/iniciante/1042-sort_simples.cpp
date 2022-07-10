// vim: noai:ts=4:sw=4
#include <bits/stdc++.h>

#define pb push_back
#define N 3

typedef long long ll;
typedef unsigned long long ull;

using namespace std;

int main () {
    iostream::sync_with_stdio(false);
    cin.tie(NULL);
    cout.tie(NULL);

    int v, n = N;
    vector<int> arr, ordered;

    while (n--) {
        cin >> v;
        arr.pb(v);
    }
    ordered = arr;

    sort(ordered.begin(), ordered.end());

    for (int i=0; i<N; i++)
        cout << ordered[i] << endl;
    cout << endl;
    for (int i=0; i<N; i++)
        cout << arr[i] << endl;

    return 0;
}
