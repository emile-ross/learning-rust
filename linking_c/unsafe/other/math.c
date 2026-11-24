#include <stdio.h>
#include <stdlib.h>
#include <inttypes.h>
#include <stdint.h>

double divide(int64_t num, int64_t div)
{
	if (div == 0)
	{
		fprintf(stderr, "Attempting to divide %" PRId64 " by zero\n", num);
		exit(EXIT_FAILURE);
	}

	return (double)num / (double)div;
}
