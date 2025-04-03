#ifndef IO_H
#define IO_H

extern void outp(unsigned short port, unsigned short data);
extern void print(char *buf, int length, int fg, int bg);
extern void move_cursor(int row, int column);

#endif
