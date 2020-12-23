#include <assert.h>
#include <malloc.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

typedef unsigned long cup_id;

struct cup {
	cup_id id;
	struct cup *next;
};

struct cups {
	struct cup *current;
	struct cup *by_id;
	size_t length;
};

void print_cups(const struct cups *cups) {
	const struct cup *cup = &cups->by_id[1];
	do {
		if (cup == cups->current)
			printf("(%lu) ", cup->id);
		else 
			printf("%lu ", cup->id);
		cup = cup->next;
	} while (cup != &cups->by_id[1]);
	printf("\n");
}

const struct cup *first_after_1(const struct cups *cups) {
	return cups->by_id[1].next;
}

cup_id prev(cup_id id, size_t length) {
	return id == 1 ? length : id - 1;
}

void apply_move(struct cups *cups) {
	struct cup *curr = cups->current;
	struct cup *next1 = curr->next;
	struct cup *next2 = next1->next;
	struct cup *next3 = next2->next;
	struct cup *next4 = next3->next;

	cup_id dest_id = prev(cups->current->id, cups->length);
	struct cup *dest;
	do {
		dest = &cups->by_id[dest_id];
		dest_id = prev(dest_id, cups->length);
	}
	while (dest == next1 || dest == next2 || dest == next3);

	struct cup *dest_next1 = dest->next;
	struct cup *dest_next2 = dest_next1->next;
	struct cup *dest_next3 = dest_next2->next;

	// remove next1..next3 from ring
	curr->next = next4;

	// reinsert after dest
	dest->next = next1;
	next3->next = dest_next1;

	cups->current = cups->current->next;
}

void apply_n_moves(struct cups *cups, size_t count) {
	for (size_t i = 0; i < count; ++i) {
		apply_move(cups);
	}
}

struct cups *make_cups(const char *init, size_t length, cup_id current) {
	struct cups *cups = (struct cups *)malloc(sizeof(struct cups));
	cups->current = NULL;
	cups->length = length;
	cups->by_id = (struct cup *)calloc(length + 1, sizeof(struct cup));

	struct cup *first = NULL;
	struct cup **next_p = &first;
	size_t count = 0;
	
	while (*init) {
		cup_id id = (*init++) - '0';
		struct cup *cup = &cups->by_id[id];
		cup->id = id;
		*next_p = cup;
		next_p = &cup->next;
		count++;
	}

	while (++count <= length) {
		struct cup *cup = &cups->by_id[count];
		cup->id = count;
		*next_p = cup;
		next_p = &cup->next;
	}

	*next_p = first;
	cups->current = &cups->by_id[current];

	return cups;
}

void assert_cups(const char *tag, const struct cups *cups, const char *expect) {
	for (const struct cup *cup = first_after_1(cups); *expect; cup = cup->next, expect++) {
		cup_id expected_cup = *expect - '0';
		if (cup->id != expected_cup) {
			printf("%s: expected %u got %u\n", tag, expected_cup, cup->id);
		}
	}
}

void test_10_moves() {
	struct cups *cups = make_cups("389125467", 9, 3);
	apply_n_moves(cups, 10);
	assert_cups("test 10 moves", cups, "92658374");
}


void test_100_moves() {
	struct cups *cups = make_cups("389125467", 9, 3);
	apply_n_moves(cups, 100);
	assert_cups("test 100 moves", cups, "67384529");
}

void test_10_million_moves() {
	struct cups *cups = make_cups("389125467", 1000000, 3);
	apply_n_moves(cups, 10000000);
	const struct cup *cup = first_after_1(cups);
	cup_id prod = cup->id * cup->next->id;
	if (prod != 149245887792) {
		printf("test 10 million moves: expected 149245887792 got %lu * %lu = %lu\n", cup->id, cup->next->id, prod);
	}
}

void run_tests() {
	test_10_moves();
	test_100_moves();
	test_10_million_moves();
}

cup_id part2(struct cups *cups) {
	return 0;
}

int main(int argc, char **argv) {
	if (argc > 1 && strcmp(argv[1], "test") == 0) {
		run_tests();
	} else {
		struct cups* cups = make_cups("523764819", 1000000, 5);
		apply_n_moves(cups, 10000000);
		const struct cup *cup = first_after_1(cups);
		printf("part 2: %lu\n", cup->id * cup->next->id);
	}
}
