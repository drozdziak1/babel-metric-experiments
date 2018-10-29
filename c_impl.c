#include <stdio.h>
#include <math.h>
#include <string.h>

int main(int argc, char *argv[])
{
	int price = 60;
	int quality = 60;

	double metric = log2((double)price) + log2((double)quality);

	printf("price:%d\nquality:%d\nmetric: %lf", price,quality, metric);

	return 0;
}
