#include <bits/stdc++.h>

#define MAX_SIZE 40
#define mp make_pair

using namespace std;

int n, f;
vector<pair<bool,bool>> sign;
vector<int> num;
map<pair<int, int>, bool> memo;

bool depth_first_search(int idx, int curr_sum) {
    if (idx == n) {
        if (curr_sum == f)
            return true;
        return false;
    }

    if (memo.count({curr_sum, idx}) > 0)
        return memo[{curr_sum, idx}];

    bool left, right;
    left = depth_first_search(idx+1, curr_sum+num[idx]);
    right = depth_first_search(idx+1, curr_sum-num[idx]);

    bool pos, neg;
    tie(pos, neg) = sign[idx];

    if (not pos)
        sign[idx].first = left;
    if (not neg)
        sign[idx].second = right;

    memo[{curr_sum, idx}] = (right or left);
    return (right or left);
}

int main () {
    int temp;

    while(true) {
        cin >> n >> f;

        if (n == 0 && f == 0)
            break;

        for (int i=0; i<n; i++) {
            cin >> temp;
            num.push_back(temp);
            sign.push_back({false, false});
        }

        bool found = depth_first_search(0, 0);

        if (not found) {
            cout << "*";
        } else {
            for (int i = 0; i < n; i++) {
                bool pos, neg;
                tie(pos, neg) = sign[i];

                if (pos and not neg)
                    cout << '+';
                else if (neg and not pos)
                    cout << '-';
                else if (pos and neg)
                    cout << '?';
            }
        }
        cout << endl;
        num.clear();
        sign.clear();
        memo.clear();
    }

    return 0;
}
