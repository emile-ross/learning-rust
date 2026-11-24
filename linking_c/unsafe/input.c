#include <stdlib.h>
#include <string.h>
#include <stdio.h>

#define MAX_SIZE 128

char *get_input(void)
{
	char *input_buffer = malloc(MAX_SIZE);

	if (!(fgets(input_buffer, MAX_SIZE - 1, stdin)))
	{
		fprintf(stderr, "Failed to get user input\n");
		exit(EXIT_FAILURE);
	}

	size_t index = strcspn(input_buffer, "\n\t");
	printf("\'%c\'\n", input_buffer[index]);

	return input_buffer;
}
