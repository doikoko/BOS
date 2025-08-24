// heap allocated on addresses 0x400_000 - 0x600_000
#include <stdint.h>

#define NULL (void*)0

#define HEAP_FIRST_ADDR 0x400000 + 0x6650
#define HEAP_LAST_ADDR 0x600000
#define HEAP_SIZE 0x200000 - HEAP_TABLE_SIZE

#define HEAP_TABLE_SIZE 0x6650
#define HEAP_TABLE_ADDR 0x400000
typedef struct HeapTable{
    uint8_t data[HEAP_TABLE_SIZE]
} HeapTable;

void memset(long data, long* start_ptr, int bytes){
    for(int i = 0; i < bytes / 8; i++){
        *start_ptr = data;
        start_ptr++;
    }
}

#define HEAP_PAGE_SIZE 10
#define MAX_ONE 1 << 7
void* malloc(HeapTable *heap_table, uint32_t bytes){
    long addr = HEAP_FIRST_ADDR;
    uint8_t one = 1, count = 0, sequence_len = bytes / HEAP_PAGE_SIZE;

    // if input - 25 sequence_len will contain 2
    if ((bytes % HEAP_PAGE_SIZE) != 0){
        sequence_len += HEAP_PAGE_SIZE;
        if (sequence_len > UINT8_MAX) return NULL;
    }

    // iterate each byte
    for(uint8_t *ptr = (uint8_t*)heap_table; (long)ptr < HEAP_TABLE_SIZE; ptr++){
        // iterate each bit
        for (one = 1; ;one <<= 1, addr += HEAP_PAGE_SIZE){
            if ((*ptr & one) == 1){
                count++;
                if (count == sequence_len) return (void*)addr;
            } 
            else count = 0;
            if (one == MAX_ONE) break;
        }
    }
}
int free(HeapTable *heap_table, void* ptr, uint32_t bytes){
    if ((long)ptr > HEAP_LAST_ADDR && (long)ptr < HEAP_FIRST_ADDR) return 1;
    
    // each byte menegementing 80 bytes in heap (1bit - 10)
    uint32_t
        first_byte = ((long)ptr - HEAP_FIRST_ADDR) / (HEAP_PAGE_SIZE * 8),
        first_bit = (((long)ptr - HEAP_FIRST_ADDR) % (HEAP_PAGE_SIZE * 8)) / 10,
        sequence_len = bytes / HEAP_PAGE_SIZE;

    if((bytes % HEAP_PAGE_SIZE) != 0){
        sequence_len += HEAP_PAGE_SIZE;
        if (sequence_len > UINT8_MAX) return NULL;
    }

    uint8_t one = 1;
    uint8_t *table_ptr = (uint8_t)HEAP_TABLE_ADDR + first_byte;
    
    // set up 1 touchin byte (byte can be |0|0|1|1|1|1..)
    for(one <<= (first_bit - 1); sequence_len > 0 ; one <<= 1, sequence_len--){
        if((*table_ptr & one) == 0) return 1;
        *table_ptr ^= one;
        if (one == MAX_ONE) break;
    }
    if (one != MAX_ONE) return 0;
 //   if ()
 //   for(uint32_t i = 0; i < bytes; i++, ptr++){
 //       for(uint8_t i = 0; i )
 //   }
}
HeapTable *heap_init(){
    HeapTable *heap_table = (HeapTable*)HEAP_TABLE_ADDR;
   
    memset(0, (long*)heap_table, HEAP_TABLE_SIZE + HEAP_SIZE);
}