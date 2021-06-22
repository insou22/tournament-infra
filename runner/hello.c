#include <stdio.h>
#include <stdlib.h>
#include <unistd.h>
#include <pwd.h>

int main(void) {
	uid_t uid = geteuid();
	struct passwd *pw = getpwuid(uid);
	printf("i am %s  ,  ", pw->pw_name);

	char name[20] = { 0 };
	scanf("%s", name);

	printf("hello, %s!\n", name);
}
