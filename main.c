#include "chan.h"

#include <stdio.h>

int main() 
{
    int32_t i;
    ChannelWrapper *cw = channel_wrapper_create();
    
    for (i = 0; i < 100; i++)
        channel_wrapper_send(cw, i);

    channel_wrapper_free(cw);
    return 0;
}
