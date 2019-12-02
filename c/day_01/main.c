#include <stdio.h>
#include <stdlib.h>

long mass_to_fuel(long mass) {
	return mass / 3 - 2;
}

long fuel_for_fuel(long fuel_mass) {
	if (fuel_mass <= 0) {
		return 0;
	}

	return fuel_mass + fuel_for_fuel(mass_to_fuel(fuel_mass));
}

long part_1(FILE* fptr) {
	char* line = NULL;
	size_t len = 0;
	long sum = 0;

	while (getline(&line, &len, fptr) != -1) {
		long module_mass = strtol(line, NULL, 10);
		sum += mass_to_fuel(module_mass);
	}

	return sum;
}

long part_2(FILE* fptr) {
	char* line = NULL;
	size_t len = 0;
	long sum = 0;

	while (getline(&line, &len, fptr) != -1) {
		long module_mass = strtol(line, NULL, 10);
		sum += fuel_for_fuel(mass_to_fuel(module_mass));
	}

	return sum;
}

int main(void) {
	FILE* fptr;

	if ((fptr = fopen("input.txt", "r")) == NULL) {
		printf("Cannot open file");
		return 1;
	}

	printf("Part 1: %ld\n", part_1(fptr));
	rewind(fptr);
	printf("Part 2: %ld\n", part_2(fptr));

	return 0;
}
