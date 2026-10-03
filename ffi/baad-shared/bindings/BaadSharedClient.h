#ifndef BaadSharedClient_H
#define BaadSharedClient_H

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"

#include "BaadError.d.h"

#include "BaadSharedClient.d.h"






typedef struct BaadSharedClient_init_result {union { BaadError* err;}; bool is_ok;} BaadSharedClient_init_result;
BaadSharedClient_init_result BaadSharedClient_init(DiplomatStringView proxy, DiplomatStringView user_agent, bool no_proxy);

void BaadSharedClient_destroy(BaadSharedClient* self);





#endif // BaadSharedClient_H
