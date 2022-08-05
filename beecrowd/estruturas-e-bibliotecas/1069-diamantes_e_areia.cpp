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

    int n, d_counter; cin >> n;
    string s;
    stack<char> diamonds;

    while (n--) {
        d_counter = 0;
        cin >> s;
        for (auto &c : s) {
            if (!diamonds.empty()) {
                if (diamonds.top() == '<' && c == '>') {
                    diamonds.pop();
                    d_counter++;
                    continue;
                }
            }
            if (c != '.')
                diamonds.push(c);
        }
        cout << d_counter << endl;

        while (!diamonds.empty())
            diamonds.pop();
    }

    return 0;
}
