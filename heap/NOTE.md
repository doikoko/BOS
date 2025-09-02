my heap following next stucture:

0x400_000 - 0x406_650 - 
```C
    struct HeapTable{
        uint_8t data[0x6650]
    }
```
in data field containing bytes status of pages
1 - busy
0 - not

each page is 16 bytes
in each byte of page field containing info about 8 pages