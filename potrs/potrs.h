// library to work with ports

#ifndef PORTS_H
#define PORTS_H

// function to configure speed in bouds of serial port (default is like in macro)
// default speed is 115200hz, divisor is a velue that speed will divide
// if divisor = 2, speed will be (115200 / 2)bouds
#define SERIAL_PORT 0x3F8
extern void configure_serial_port_baud_rate(unsigned int serial_port, unsigned int divisor);


#define CONFIG_PORT_SETTING 0x03
// 7| 0 |6| 0 |5| 000 |2| 0 |1| 11 |0|
// 0,1: 	8 bit data
// 2: 	number of stop bytes
// 3,4,5:number of parity
// 6:	break control
// 7:	access byte
extern void set_serial_port_settings(unsigned int serial_port, unsigned short settings);

//function to enable FIFO in buffers
#define FIFO_ENABLE 0xC7
extern void  set_serial_port_FIFO_buffers(unsigned short serial_port, unsigned short settings);

//function to set modem ready status
#define MODEM_READY_STATUS 0x03
extern void set_serial_port_modem(unsigned short serial_port, unsigned short settings);

#endif
