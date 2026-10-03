#ifndef BaadYoStarClient_H
#define BaadYoStarClient_H

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"

#include "../../baad-shared/bindings/BaadError.d.h"
#include "BaadResourcesAsset.d.h"
#include "../../baad-shared/bindings/BaadSharedDomain.d.h"
#include "../../baad-shared/bindings/BaadSharedGameBaseConfig.d.h"
#include "../../baad-shared/bindings/BaadSharedGameJsonConfig.d.h"
#include "../../baad-shared/bindings/BaadSharedGameJsonData.d.h"

#include "BaadYoStarClient.d.h"






typedef struct BaadYoStarClient_get_base_config_result {union {BaadSharedGameBaseConfig* ok; BaadError* err;}; bool is_ok;} BaadYoStarClient_get_base_config_result;
BaadYoStarClient_get_base_config_result BaadYoStarClient_get_base_config(void);

typedef struct BaadYoStarClient_get_domain_result {union {BaadSharedDomain* ok; BaadError* err;}; bool is_ok;} BaadYoStarClient_get_domain_result;
BaadYoStarClient_get_domain_result BaadYoStarClient_get_domain(void);

typedef struct BaadYoStarClient_get_json_config_result {union {BaadSharedGameJsonConfig* ok; BaadError* err;}; bool is_ok;} BaadYoStarClient_get_json_config_result;
BaadYoStarClient_get_json_config_result BaadYoStarClient_get_json_config(DiplomatStringView version, DiplomatStringView path);

typedef struct BaadYoStarClient_get_json_data_result {union {BaadSharedGameJsonData* ok; BaadError* err;}; bool is_ok;} BaadYoStarClient_get_json_data_result;
BaadYoStarClient_get_json_data_result BaadYoStarClient_get_json_data(DiplomatStringView url);

typedef struct BaadYoStarClient_get_resources_asset_result {union {BaadResourcesAsset* ok; BaadError* err;}; bool is_ok;} BaadYoStarClient_get_resources_asset_result;
BaadYoStarClient_get_resources_asset_result BaadYoStarClient_get_resources_asset(void);

void BaadYoStarClient_destroy(BaadYoStarClient* self);





#endif // BaadYoStarClient_H
