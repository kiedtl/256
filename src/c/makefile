NAME	= 256
WARNING	= -Wall -Wextra -pedantic -Wmissing-prototypes \
	  -Wold-style-definition -Werror -Wno-unused-parameter

CC	= gcc
CFLAGS	= -std=c99 $(WARNING) -O3 -fno-stack-protector
LDFLAGS	= -fuse-ld=gold

SRC	= 256.c
OBJ	= $(SRC:.c=.o)

all: $(NAME)

clean:
	@echo "RM    $(NAME) $(OBJ)"
	@rm -f $(NAME) $(OBJ)

.c.o:
	@echo "CC    $@"
	@$(CC) $(CFLAGS) -c $<

$(NAME): $(OBJ)
	@echo "LD    $(NAME)"
	@$(CC) -o $@ $^ $(CFLAGS) $(LDFLAGS)

.PHONY: all clean
