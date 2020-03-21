#include <stdio.h>

#define max_comp(a, b) (a > b ? a : b)

int N;
double c[22][22];
int fin[1<<22];
double logs[1<<22];

double process(int current_bond, int missions) {
    if (current_bond == N) return 1.0;
    if (fin[missions]) return logs[missions];
    fin[missions] = 1;
    double result = 0.0;
    int i;
    for (i = 0; i < N; ++i) {
        if ((missions & (1 << i)) == 0) {
            double d = c[current_bond][i] * process(current_bond+1, missions | (1<<i));
            result = max_comp(result, d);
        }
    }
    logs[missions] = result;
    return result;
}

int main() {
    scanf("%d", &N);
    int i, j;
    for (i = 0; i < N; ++i) {
        for (j = 0; j < N; ++j) {
            int percent;
            scanf("%d", &percent);
            c[i][j] = percent / 100.0;
        }
    }
    double result = process(0, 0);
    printf("%.6f\n", result * 100);
    return 0;
}
