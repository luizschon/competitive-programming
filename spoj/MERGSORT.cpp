// vim: noai:ts=4:sw=4
#include <bits/stdc++.h>

#define pb push_back
#define mp make_pair

typedef long long ll;
typedef unsigned long long ull;
typedef unsigned int u;
typedef std::vector<int> vi;
typedef std::list<int> li;

using namespace std;

li merge_sort(li vector);
li merge(li left, li right);

li merge_sort(li list) {
    int size = list.size();

    if (size <= 1)
        return list;

    li left, right;

    int i=0;
    for (auto it=list.begin(); it!=list.end(); it++, i++) {
        if (i < size/2)
            left.pb(*it);
        else 
            right.pb(*it);
    }

    left = merge_sort(left);
    right = merge_sort(right);

    return merge(left, right);
}

li merge(li left, li right) {
    li res;

    while (!left.empty() && !right.empty()) {
        if (left.front() < right.front()) {
            res.pb(left.front());
            left.pop_front();
        } else {
            res.pb(right.front());
            right.pop_front();
        }
    }

    while (!left.empty()) {
        res.pb(left.front());
        left.pop_front();
    }
    while (!right.empty()) {
        res.pb(right.front());
        right.pop_front();
    }

    return res;
}

int main () {
    iostream::sync_with_stdio(false);
    cin.tie(NULL); cout.tie(NULL);

    int el;
    li vector;

    while (cin >> el)
        vector.pb(el);

    li ordered = merge_sort(vector);

    for (auto it=ordered.begin(); it!=ordered.end(); it++) {
        cout << *it << " ";
    }
    cout << endl;

    return 0;
}
