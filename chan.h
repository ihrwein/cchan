#ifndef CHAN_H_INCLUDED
#define CHAN_H_INCLUDED

#include <sys/types.h>

struct _ChannelWrapper;
typedef struct _ChannelWrapper ChannelWrapper;

int channel_wrapper_send(ChannelWrapper *this, int32_t value);
void channel_wrapper_free(ChannelWrapper *this);
ChannelWrapper* channel_wrapper_create();

#endif
