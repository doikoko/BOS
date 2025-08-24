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

each page is 10 bytes
in each byte of page field containing info about 8 pages

0x6650 because 0x6650 * 8 * 10 = 0x1ff900 (a little less than heap size)
but 0x6650 + 0x1ff900 = 0x205f50