CC = gcc
CFLAGS = -Wall -Wextra #-Werror
TARGET = tracker

SRCS = $(wildcard src/*.c)
TST_SRCS = $(filter-out src/main.c, $(wildcard src/*.c)) $(wildcard tst/*.c)

OBJS = $(SRCS:.c=.o)
TST_OBJS = $(TST_SRCS:.c=.o)

%.o: %.c
	$(CC) $(CFLAGS) -c $< -o $@

all: $(TARGET)

$(TARGET): $(OBJS)
	$(CC) $(CFLAGS) -o $@ $^

run: $(TARGET)
	./$(TARGET) -v 3

test: $(TST_OBJS)
	$(CC) $(CFLAGS) -o $@ $^
	./$@

clean:
	rm -f $(TARGET) test src/*.o tst/*.o log/*.log

help:
	grep '^[a-zA-Z0-9_\-]*:' Makefile | sed 's/://'

