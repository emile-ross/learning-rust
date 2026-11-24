#include <stdlib.h>
#include <inttypes.h>
#include <stdio.h>


void *smalloc(size_t size)
{
	if (!(size))
	{
		fprintf(stderr, "allocating buffer of size 0 is useless\n");
		exit(EXIT_FAILURE);
	}

	void *buffer = malloc(size);
	if (!(buffer))
	{
		fprintf(stderr, "malloc() failed to allocate memory of size %" PRIu64 "\n", size);
		exit(EXIT_FAILURE);
	}
	return buffer;
}
