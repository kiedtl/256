#include <stdio.h>
#include "256.h"
#include "types.h"

int
main(int argc, char **argv)
{
	usize n;
	if (argc < 2)
		n = 256;
	else
		n = to_int(argv[1]);

	for (usize i = 0; i <= n; ++i) {
		printf("%i%c[4D%c[4C%c[48;5;%im%c[K%c[m\n",
			i, e, e, e, i, e, e);
	}

	return 0;
}

usize
to_int(char *str)
{
	usize buf = 0;

	while (*str) {
		if (*str >= '0' && *str <= '9')
			buf = (buf * 10) + (*str - '0');
		else
			break;
		++str;
	}

	return buf;
}
