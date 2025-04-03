#ifndef IO_H
#define IO_H

// function providing ability to use a ports
extern void outp(unsigned short port, unsigned short data);

// function to print on screen char
#define CL_BLACK    0
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
extern void print(char symb, unsigned int fg, unsigned int bg);

//function to move cursor on sreen
extern void move_cursor(unsigned int row, unsigned int column);

// function to configure speed in bouds of serial port (default is like in macro)
// default speed is 115200hz, divisor is a velue that speed will divide
// if divisor = 2, speed will be (115200 / 2)bouds
#define SERIAL_PORT 0x3F8
extern void configure_serial_port_baud_rate(unsigned int serial_port, unsigned int divisor);


#endif
