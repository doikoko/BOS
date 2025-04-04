/* It's a kernel of BOS, in this 
*   i will use functions from other 
*   directories like IO
*/
#include "../IO/io.h"
#include "../ports/ports.h"

int main(void){
    configure_serial_port_baud_rate(SERIAL_PORT, 2); // 115200 / 2 bouds
    set_serial_port_settings(SERIAL_PORT, CONFIG_PORT_SETTING);
    set_serial_port_FIFO_buffers(SERIAL_PORT, FIFO_ENABLE);
    set_serial_port_modem(SERIAL_PORT, MODEM_READY_STATUS); 
}
