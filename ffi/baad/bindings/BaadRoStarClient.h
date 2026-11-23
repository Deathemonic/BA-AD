#ifndef BaadRoStarClient_H
#define BaadRoStarClient_H

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"

#include "../../baad-shared/bindings/BaadError.d.h"
#include "../../baad-shared/bindings/BaadSharedChinaState.d.h"

#include "BaadRoStarClient.d.h"






typedef struct BaadRoStarClient_get_state_result {union {BaadSharedChinaState* ok; BaadError* err;}; bool is_ok;} BaadRoStarClient_get_state_result;
BaadRoStarClient_get_state_result BaadRoStarClient_get_state(DiplomatStringView version);

void BaadRoStarClient_destroy(BaadRoStarClient* self);





#endif // BaadRoStarClient_H
