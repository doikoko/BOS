#define NULL (void*)0

#define HEAP_FIRST_ADDR 0x400000
#define HEAP_MEMORY_FIRST_ADDR 0x400000 + HEAP_TABLE_SIZE
#define HEAP_LAST_ADDR 0x600000
#define HEAP_MEMORY_SIZE HEAP_LAST_ADDR - HEAP_FIRST_ADDR - HEAP_TABLE_SIZE

#define HEAP_TABLE_SIZE 0x4000
#define HEAP_TABLE_FIRST_ADDR HEAP_FIRST_ADDR

typedef unsigned char uint8_t;
typedef unsigned short uint16_t;
typedef unsigned int uint32_t;
typedef unsigned long uint64_t;

#define UINT8_MAX 0xFF

#define HEAP_PAGE_SIZE 0x10
#define MAX_ONE 1 << 7
void* _malloc(uint32_t len){
    uint64_t addr = HEAP_MEMORY_FIRST_ADDR;
    uint8_t one = 1, count = 0, sequence_len = len / HEAP_PAGE_SIZE;

    // if input - 25 sequence_len will contain 2
    if ((len % HEAP_PAGE_SIZE) != 0)
        sequence_len += 1;

    // iterate each byte
    for(
        uint8_t *ptr = (uint8_t*)HEAP_TABLE_FIRST_ADDR; 
        (uint64_t)ptr < addr;
        ptr++
    ){
        // iterate each bit
        for (one = 1; ;one <<= 1, addr += HEAP_PAGE_SIZE){
            if ((*ptr & one) == 0){
                count++;
                if (count == sequence_len){
                    uint64_t new_addr = addr - (HEAP_PAGE_SIZE * (count - 1));
                    for(; count > 0; count--, one >>= 1){
                        // set bit
                        *ptr |= one;
                        if(one == 0)
                            ptr--;
                    }
                    return (void*)new_addr;
                }
            } 
            else count = 0;
            if (one == MAX_ONE) break;
        }
    }
    return NULL;
}

uint8_t _free(void* ptr, uint32_t len){
    if ((uint64_t)ptr > HEAP_LAST_ADDR && (uint64_t)ptr + len < HEAP_MEMORY_FIRST_ADDR) return 1;
    uint32_t
        first_byte = ((uint64_t)ptr - HEAP_MEMORY_FIRST_ADDR) / (HEAP_PAGE_SIZE * 8),
        first_bit = (((uint64_t)ptr - HEAP_MEMORY_FIRST_ADDR) % (HEAP_PAGE_SIZE * 8)) / HEAP_PAGE_SIZE,
        bit_count = (((uint64_t)ptr + len) - HEAP_MEMORY_FIRST_ADDR) / HEAP_PAGE_SIZE;

    if ((bit_count % HEAP_PAGE_SIZE) != 0) bit_count++;

    uint8_t one = 1;
    uint8_t *table_ptr = (uint8_t*)(HEAP_TABLE_FIRST_ADDR + (uint64_t)first_byte);
    
    // set up 1 byte (byte can be |0|0|1|1|1|1..)
    for(one <<= first_bit;; one <<= 1, bit_count--){
        if((*table_ptr & one) == 0) return 0;
        *table_ptr ^= one;
        if (one == MAX_ONE) {
            bit_count--;
            break;
        }
        if (bit_count == 0) return 0;
    }
    for(table_ptr++; bit_count > 0; table_ptr++){
        for(one = 1;; one <<= 1){
            *table_ptr ^= one;
            if(--bit_count == 0) return 0;
            if(one == MAX_ONE) break;
        }
    }
    return 0;
}