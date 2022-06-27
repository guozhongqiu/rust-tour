#include <stdio.h>
#include <stdint.h>
#include <stdlib.h>
#include <errno.h>

#include <sys/types.h>
#include <sys/stat.h>
#include <fcntl.h>

#include <unistd.h>

#include "readarg.h"

#define READ_BUF_SZ    4096
#define PATH_BASE    "/dev/shm/readarg"

typedef int32_t (*fill_cb)(uint32_t size);

extern void register_fill_cb(read_args_t *arg, fill_cb cb);

int readarg_init(read_args_t *ra, uint32_t count, uint64_t offset)
{
        int ret = 0;
        if (ra) {
                ra->count = count;
                ra->offset = offset;
                ra->realcount = 0;
                ra->buf = calloc(1, count);
                if (NULL == ra->buf) {
                        ret = errno;
                        return ret;
                }
        }
        return ret;
}

int dump_file(const char *path, uint8_t *value, int size, int flag)
{
        int ret, fd;

        fd = open(path, O_WRONLY | flag, 0644);
        if (fd < 0) {
                //unlink(path);
                ret = errno;
                goto err_ret;
        }

        if (value) {
                ret = pwrite(fd, value, size, 0);
                if (ret < 0) {
                        ret = -ret;
                        goto err_ret;
                }
        }

        close(fd);

        return 0;
err_ret:
        return ret;
}

int main()
{
        int ret = 0;
        read_args_t ra;

        ret = readarg_init(&ra, READ_BUF_SZ, 0);
        if (ret) {
                printf("init failed %d\n", ret);
                goto out;
        }

        printf("realcount before %u\n", ra.realcount);

        //todo
        register_fill_cb(&ra, fill_buf);

        printf("realcount after %u\n", ra.realcount);
        if (ra.realcount <= 0) {
                printf("no data\n");
                goto out;
        }

        ret = dump_file(PATH_BASE, ra.buf, ra.realcount, O_CREAT|O_TRUNC);
        if (ret) {
                printf("dump failed %d\n", ret);
                goto out;
        }
out:
        return ret;
}