#ifndef BaadSharedMarketConfig_H
#define BaadSharedMarketConfig_H

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"

#include "BaadError.d.h"
#include "BaadSharedBuildType.d.h"
#include "BaadSharedPlatform.d.h"

#include "BaadSharedMarketConfig.d.h"






typedef struct BaadSharedMarketConfig_for_global_result {union {BaadSharedMarketConfig* ok; BaadError* err;}; bool is_ok;} BaadSharedMarketConfig_for_global_result;
BaadSharedMarketConfig_for_global_result BaadSharedMarketConfig_for_global(BaadSharedPlatform platform, BaadSharedBuildType build_type);

DiplomatStringView BaadSharedMarketConfig_market_game_id(const BaadSharedMarketConfig* self);

DiplomatStringView BaadSharedMarketConfig_market_code(const BaadSharedMarketConfig* self);

void BaadSharedMarketConfig_destroy(BaadSharedMarketConfig* self);





#endif // BaadSharedMarketConfig_H
