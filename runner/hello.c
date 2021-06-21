#include <stdio.h>

int main(void) {
	printf("hello, ");

	int ch;
	while ((ch = getchar()) != EOF) {
		putchar(ch);
	}

	printf("!\n");
}
