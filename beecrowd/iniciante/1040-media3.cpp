// vim: noai:ts=4:sw=4
#include <bits/stdc++.h>

#define pb push_back
#define mp make_pair
#define N 4

typedef long long ll;
typedef unsigned long long ull;

using namespace std;

int main () {
    iostream::sync_with_stdio(false);
    cin.tie(NULL);
    cout.tie(NULL);

    int weight[N] = {2, 3, 4, 1};
    float grade, avg = 0;
    
    for (int i=0; i<N; i++) {
        cin >> grade;
        avg += weight[i]*grade;
    }
    avg /= 10;

    cout << fixed << setprecision(1);
    cout << "Media: " << avg << endl;

    if (avg >= 7.0)
        cout << "Aluno aprovado.\n";
    else if (avg < 5.0)
        cout << "Aluno reprovado.\n";
    else {
        cout << "Aluno em exame.\n";
        cin >> grade;
        cout << "Nota do exame: " << grade << endl;
        avg = (avg+grade)/2;
        cout << ((avg > 5.0) ? "Aluno aprovado.\n" : "Aluno reprovado.\n");
        cout << "Media final: " << avg << endl;
    }
    return 0;
}
