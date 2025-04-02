#ifndef KERNEL_H
#define KERNEL_H

extern void outp(unsigned short port, unsigned short data);
extern void print(char *buf, int length, int fg, int bg);

#endif
