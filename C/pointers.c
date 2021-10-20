#include<stdio.h>

int main ()
{
    int  a = 1025;
    int *p;
    p = &a;

    //pointer arithmetic
    printf ("Address p is %d\n", p);
    printf ("value at address p is %d\n", *p);
    printf ("size of the integer is %d bytes\n", sizeof(int));
    printf ("Address p+1 is %d\n", p+1);
    printf ("value at address p+1 is %d\n", *(p+1));

    printf ("\nAddress = %d, value = %d\n",p,*p);
    printf ("\nAddress = %d, value = %d\n",p+1,*(p+1));

    char *p0;
    p0 = (char*)p; //typecasting

    printf ("\nSize of the char is %d bytes\n", sizeof(char));
    printf ("Address = %d, value = %d\n",p0,*p0);
    printf ("Address = %d, value = %d\n",p0+1,*(p0+1));
}