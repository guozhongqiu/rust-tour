#include <stdio.h>
#include <string.h>

#include "readarg.h"

int32_t fill_buf(read_args_t *ra, uint32_t size)
{
    int32_t ret = 0;

    ra->realcount = size;

    return ret;
}