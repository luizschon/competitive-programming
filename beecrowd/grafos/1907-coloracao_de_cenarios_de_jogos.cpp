// vim: noai:ts=4:sw=4
/* 
 * Luiz Carlos Schonarth Junior
 * Univerdidade de Brasília - UnB - 2022
 */
#include <bits/stdc++.h>

#define pb push_back
#define mp make_pair
#define MAX 1026

typedef long long ll;
typedef unsigned long long ull;
typedef unsigned int u;
typedef std::pair<int, int> pii;
typedef std::vector<int> vi;
typedef std::pair<ll, ll> pll;
typedef std::vector<ll> vll;

using namespace std;

char canvas[MAX][MAX];
bool visited[MAX][MAX];

void fill(int x, int y) {
    int curr_x, curr_y;
    queue<pii> connected;
    connected.push({x,y});

    while (not connected.empty()) {
        tie(curr_x, curr_y) = connected.front();

        if (canvas[curr_x][curr_y] == '.' and not visited[curr_x][curr_y]) {
            visited[curr_x][curr_y] = true;
            connected.push({curr_x+1, curr_y});
            connected.push({curr_x, curr_y+1});
            connected.push({curr_x-1, curr_y});
            connected.push({curr_x, curr_y-1});
        }
        connected.pop();
    }
}

int main () {
    iostream::sync_with_stdio(false);
    cin.tie(NULL); cout.tie(NULL);

    int n, m, n_clicks=0;
    cin >> n >> m;

    for (int i = 1; i <= n; i++)
        for (int j = 1; j <= m; j++)
            visited[i][j] = false;

    for (int i = 1; i <= n; i++)
        for (int j = 1; j <= m; j++)
            cin >> canvas[i][j];

    for (int i = 1; i <= n; i++) {
        for (int j = 1; j <= m; j++) {
            if (canvas[i][j] == '.' && not visited[i][j]) {
                n_clicks++;
                fill(i, j);
            }
        }
    }

    cout << n_clicks << endl;

    return 0;
}
