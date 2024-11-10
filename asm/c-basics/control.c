#include <stdio.h>

int main(int argc, char *argv[])
{
	int a = 0;
	for (int i = 0; i < 5; ++i) 
	{
		a = a + 2;
	}
	printf("a is %d\n", a);

	return 0;
}
