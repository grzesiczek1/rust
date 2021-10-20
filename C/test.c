#include <stdio.h>

int main (void)
{
    int i = 5;
    int j = sizeof(i++);
    int k = sizeof(i);
    printf("%d\n", j);
    printf("%d\n", k);

    return 0;
}