#define NULL (void*)0

#define HEAP_FIRST_ADDR 0x400000 + 0x6650
#define HEAP_LAST_ADDR 0x600000
#define HEAP_SIZE 0x200000 - HEAP_TABLE_SIZE

#define HEAP_TABLE_SIZE 0x6650
#define HEAP_TABLE_ADDR 0x400000

typedef unsigned char uint8_t;
typedef unsigned short uint16_t;
typedef unsigned int uint32_t;
typedef unsigned long uint64_t;

#define UINT8_MAX 0xFF

typedef struct{
    uint8_t head_table[HEAP_TABLE_SIZE];
    uint8_t heap_memory[HEAP_SIZE];
} Heap;

#define HEAP_PAGE_SIZE 0x10
#define MAX_ONE 1 << 7
void* _malloc(Heap *heap_table, uint32_t len){
    long addr = HEAP_FIRST_ADDR;
    uint8_t one = 1, count = 0, sequence_len = len / HEAP_PAGE_SIZE;

    // if input - 25 sequence_len will contain 2
    if ((len % HEAP_PAGE_SIZE) != 0)
        sequence_len += 1;

    // iterate each byte
    for(uint8_t *ptr = (uint8_t*)heap_table; (long)ptr < HEAP_TABLE_SIZE; ptr++){
        // iterate each bit
        for (one = 1; ;one <<= 1, addr += HEAP_PAGE_SIZE){
            if ((*ptr & one) == 0){
                count++;
                if (count == sequence_len){
                    for(; count > 0; count--, one <<= 1){
                        // set bit
                        *ptr | one;
                        if(one == 0)
                            ptr--;
                    }
                    return (void*)addr;
                }
            } 
            else count = 0;
            if (one == MAX_ONE) break;
        }
    }
    return NULL;
}

uint8_t _free(Heap *heap_table, void* ptr, uint32_t len){
    if ((long)ptr > HEAP_LAST_ADDR && (long)ptr < HEAP_FIRST_ADDR) return 1;
    
    // each byte menegementing 80 bytes in heap (1bit - 10)
    uint32_t
        first_byte = ((long)ptr - HEAP_FIRST_ADDR) / (HEAP_PAGE_SIZE * 8),
        first_bit = (((long)ptr - HEAP_FIRST_ADDR) % (HEAP_PAGE_SIZE * 8)) / 10,
        sequence_len = len / HEAP_PAGE_SIZE;

    if((len % HEAP_PAGE_SIZE) != 0){
        sequence_len += 1;
        if (sequence_len > UINT8_MAX) return 1;
    }

    uint8_t one = 1;
    uint8_t *table_ptr = (uint8_t*)(HEAP_TABLE_ADDR + first_byte);
    
    // set up 1 touchin byte (byte can be |0|0|1|1|1|1..)
    for(one <<= (first_bit - 1); sequence_len > 0 ; one <<= 1, sequence_len--){
        if((*table_ptr & one) == 0) return 1;
        *table_ptr ^= one;
        if (one == MAX_ONE) break;
    }
    if (one != MAX_ONE) return 0;
    
    for(uint8_t *table_ptr = first_byte++; sequence_len > 0; sequence_len--, table_ptr++){
        for(one = 1; ;one <<= 1){
            if((*table_ptr & one) != 0)
                return 1;

            *table_ptr ^= one;
            if(one == MAX_ONE)
                break;
        }
    }
    return 0;
}