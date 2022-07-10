// vim: noai:ts=4:sw=4
#include <bits/stdc++.h>
#include <cstddef>

#define pb push_back
#define mp make_pair

typedef long long ll;
typedef unsigned long long ull;

using namespace std;

int main () {
    iostream::sync_with_stdio(false);
    cin.tie(NULL);
    cout.tie(NULL);

    int choice, q;
    double total = 0, prices[5] = {4.0, 4.5, 5.0, 2.0, 1.5};
    
    cin >> choice >> q;
    total += prices[choice-1]*q; 

    cout << fixed << setprecision(2);
    cout << "Total: R$ " << total << endl;

    return 0;
}
