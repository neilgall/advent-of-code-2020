#include <stdio.h>
#include <string.h>
#include <stdlib.h>
#include <ctype.h>

struct input {
	int *data;
	size_t size;
};

struct input read_file(const char *path) {
	size_t size = 100;
	int *data = realloc(NULL, size*sizeof(int));
	size_t pos = 0;
	FILE *f = fopen(path, "rt");
	if (f == NULL) {
		perror("can't open file");
		abort();
	}
	int value = 0;
	int digits = 0;
	int c;
	do {
		c = fgetc(f);
		if (isdigit(c)) {
			value = value * 10 + (c - '0');
			digits += 1;
		} else if (digits > 0) {
			if (pos == size) {
				size *= 2;
				data = realloc(data, size*sizeof(int));
			}
			data[pos++] = value;
			value = 0;
			digits = 0;
		}
	}
	while (c != EOF);

	struct input rsp = { .data = data, size = pos };
	return rsp;
}

int part1(const struct input *input) {
	const int *data = input->data;
	for (int x = 0; x < input->size; ++x) {
		for (int y = 0; y < input-> size; ++y) {
			if (data[x] + data[y] == 2020)
				return data[x] * data[y];
		}
	}
}

int part2(const struct input *input) {
	const int *data = input->data;
	for (int x = 0; x < input->size; ++x) {
		for (int y = 0; y < input-> size; ++y) {
			if (y == x)
				continue;
			for (int z = 0; z < input->size; ++z) {
				if (z == y || z == x)
					continue;
				if (data[x] + data[y] + data[z] == 2020)
					return data[x] * data[y] * data[z];
			}
		}
	}
}

int main() {
	struct input input = read_file("../input.txt");

	printf("part1: %d\n", part1(&input));
	printf("part2: %d\n", part2(&input));

	free(input.data);
	return 0;
}
