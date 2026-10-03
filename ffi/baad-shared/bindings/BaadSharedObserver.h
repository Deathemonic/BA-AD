#ifndef BaadSharedObserver_H
#define BaadSharedObserver_H

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"

#include "BaadSharedProgressEvent.d.h"

#include "BaadSharedObserver.d.h"





typedef struct DiplomatCallback_BaadSharedObserver_set_callback {
    const void* data;
    void (*run_callback)(const void*, BaadSharedProgressEvent* );
    void (*destructor)(const void*);
} DiplomatCallback_BaadSharedObserver_set_callback;

void BaadSharedObserver_set(DiplomatCallback_BaadSharedObserver_set_callback callback_cb_wrap);

void BaadSharedObserver_clear(void);

void BaadSharedObserver_destroy(BaadSharedObserver* self);





#endif // BaadSharedObserver_H
