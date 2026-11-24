#include <stdlib.h>
#include <string.h>
#include <stdio.h>

#define MAX_SIZE 128

void *smalloc(size_t size);

char *get_input(void)
{
	char *input_buffer = smalloc(MAX_SIZE);

	if (!(fgets(input_buffer, MAX_SIZE - 1, stdin)))
	{
		fprintf(stderr, "Failed to get user input\n");
		exit(EXIT_FAILURE);
	}

	size_t index = strcspn(input_buffer, "\n\t");
	input_buffer[index] = '\0';

	return input_buffer;
}
