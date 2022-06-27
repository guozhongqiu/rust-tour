#ifndef __READARG_H__
#define __READARG_H__

#include <stdint.h>

typedef struct read_args
{
    uint32_t count;
    uint64_t offset;
    uint8_t *buf;
    uint32_t realcount;
} read_args_t;


int32_t fill_buf(read_args_t *ra, uint32_t size);


#endif /* __READARG_H__ */