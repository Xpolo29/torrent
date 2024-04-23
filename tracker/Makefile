CC = gcc
CFLAGS = -Wall -Wextra -g#-Werror
RCFLAGS = -O3
TARGET = tracker

SRCS = $(wildcard src/*.c)
TST_SRCS = $(filter-out src/main.c, $(wildcard src/*.c)) $(wildcard tst/*.c)

OBJS = $(SRCS:.c=.o)
ROBJS = $(SRCS:.c=.or)
TST_OBJS = $(TST_SRCS:.c=.o)

%.o: %.c
	$(CC) $(CFLAGS) -c $< -o $@

%.or: %.c
	$(CC) $(RCFLAGS) -c $< -o $@

all: $(TARGET)

$(TARGET): $(OBJS)
	$(CC) $(CFLAGS) -o $@ $^

run: $(TARGET)
	./$(TARGET) -v 2 -m 1

test: $(TST_OBJS)
	$(CC) $(CFLAGS) -o $@ $^

check: test
	./test

clean:
	rm -f $(TARGET) test src/*.o* tst/*.o log/*.log

help:
	grep '^[a-zA-Z0-9_\-]*:' Makefile | sed 's/://'

release: $(ROBJS) 
	$(CC) $(RCFLAGS) -o $(TARGET) $^
