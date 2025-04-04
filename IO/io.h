#ifndef IO_H
#define IO_H

// function providing ability write to ports
extern void outp(unsigned short port, unsigned short data);
// function providing ability read from ports
extern unsigned short inp(unsigned short port, unsigned short *data);
// function to print on screen char
#define CL_BLACK    0 // colors of text
#define CL_BLUE     1
#define CL_GREEN    2
#define CL_CYAN     3
#define CL_RED      4
#define CL_MAGENTA  5
#define CL_BROWN    6
#define CL_LGREY    7
#define CL_DGREY    8
#define CL_LBLUE    9
#define CL_LGREEN   10
#define CL_LCYAN    11
#define CL_LRED     12
#define CL_LMAGENTA 13
#define CL_LBROWN   14
#define CL_WHITE    15
extern void print_c(char *symb, unsigned short fg, unsigned short bg);
extern void print_s(char *buf, unsigned short fg, unsigned short bg, unsigned short length); 
//function to move cursor on sreen
extern void move_cursor(unsigned int row, unsigned int column);

#endif
