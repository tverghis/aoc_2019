#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#define OPCODE_ADD 1
#define OPCODE_MULT 2
#define OPCODE_HALT 99
#define GRAV_ASSIST_OUTPUT 19690720

unsigned long num_instrs(char *input) {
	size_t i;
	unsigned long count = 0;

	for (i = 0; i < strlen(input); ++ i) {
		if (input[i] == ',') ++ count;
	}

	return ++ count;
}

void run_instructions(unsigned long *instr_arr, size_t instr_arr_len) {
	size_t i;
	for (i = 0; i < instr_arr_len; i += 4) {
		switch (instr_arr[i]) {
			case OPCODE_ADD:
				instr_arr[instr_arr[i + 3]] = instr_arr[instr_arr[i + 1]] + instr_arr[instr_arr[i + 2]];
				break;

			case OPCODE_MULT:
				instr_arr[instr_arr[i + 3]] = instr_arr[instr_arr[i + 1]] * instr_arr[instr_arr[i + 2]];
				break;

			case OPCODE_HALT:
				return;

			default:
				printf("Unrecognized opcode: %lu", instr_arr[i]);
				exit(-1);
		}
	}
}

unsigned long part_1(unsigned long *instr_arr, size_t instr_arr_len, size_t instr_arr_size) {
	unsigned long temp_instr_arr [instr_arr_len];
	memcpy(temp_instr_arr, instr_arr, instr_arr_size);

	temp_instr_arr[1] = 12; temp_instr_arr[2] = 2;

	run_instructions(temp_instr_arr, instr_arr_size);
	return temp_instr_arr[0];
}

unsigned long part_2(unsigned long *instr_arr, size_t instr_arr_len, size_t instr_arr_size) {
	unsigned int noun, verb;
	
	for (noun = 0; noun <= 99; ++ noun) {
		for (verb = 0; verb <= 99; ++ verb) {
			unsigned long temp_instr_arr [instr_arr_size];
			memcpy(temp_instr_arr, instr_arr, instr_arr_size);

			temp_instr_arr[1] = noun;
			temp_instr_arr[2] = verb;

			run_instructions(temp_instr_arr, instr_arr_size);

			if (GRAV_ASSIST_OUTPUT == temp_instr_arr[0]) {
				return 100 * noun + verb;
			}
		}
	}

	exit(-1);
}

int main(void) {
	FILE *fptr;

	if ((fptr = fopen("input.txt", "r")) == NULL) {
		printf("Cannot open file");
		exit(-1);
	}

	char *input_line = NULL;
	size_t len = 0;
	if (getline(&input_line, &len, fptr) == -1) {
		printf("Could not read input");
		exit(-1);
	}

	fclose(fptr);

	unsigned long instr_arr [num_instrs(input_line)];
	size_t instr_count = 0;

	char *token = NULL;
	token = strtok(input_line, ",");
	while (token != NULL) {
		instr_arr[instr_count] = strtoul(token, NULL, 10);
		token = strtok(NULL, ",");
		++ instr_count;
	}

	printf("Part 1: %lu\n", part_1(instr_arr, instr_count, sizeof(instr_arr)));
	printf("Part 2: %lu\n", part_2(instr_arr, instr_count, sizeof(instr_arr)));

	return 0;
}

