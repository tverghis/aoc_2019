#include <stdio.h>
#include <stdlib.h>

typedef unsigned int uint;

uint mass_to_fuel(uint mass) {
	return mass / 3 - 2;
}

uint part_1(FILE* fptr) {
	char* line = NULL;
	size_t len = 0;
	uint sum = 0;

	while (getline(&line, &len, fptr) != -1) {
		uint module_mass = strtoul(line, NULL, 10);
		sum += mass_to_fuel(module_mass);
	}

	return sum;
}

int main(void) {
	FILE* fptr;

	if ((fptr = fopen("input.txt", "r")) == NULL) {
		printf("Cannot open file");
		return 1;
	}

	printf("Part 1: %u", part_1(fptr));

	return 0;
}
