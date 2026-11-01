#ifndef BaadSharedGlobalCatalogData_H
#define BaadSharedGlobalCatalogData_H

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"

#include "BaadError.d.h"
#include "BaadSharedCategory.d.h"
#include "BaadSharedCategoryMapping.d.h"
#include "BaadSharedResource.d.h"

#include "BaadSharedGlobalCatalogData.d.h"






typedef struct BaadSharedGlobalCatalogData_parse_json_result {union {BaadSharedGlobalCatalogData* ok; BaadError* err;}; bool is_ok;} BaadSharedGlobalCatalogData_parse_json_result;
BaadSharedGlobalCatalogData_parse_json_result BaadSharedGlobalCatalogData_parse_json(DiplomatStringView json);

BaadSharedGlobalCatalogData* BaadSharedGlobalCatalogData_new(void);

void BaadSharedGlobalCatalogData_push_resource(BaadSharedGlobalCatalogData* self, DiplomatStringView group, DiplomatStringView path, int64_t size, DiplomatStringView hash);

int32_t BaadSharedGlobalCatalogData_id(const BaadSharedGlobalCatalogData* self);

DiplomatStringView BaadSharedGlobalCatalogData_market_game_id(const BaadSharedGlobalCatalogData* self);

DiplomatI32View BaadSharedGlobalCatalogData_build_id(const BaadSharedGlobalCatalogData* self);

int32_t BaadSharedGlobalCatalogData_patch_version(const BaadSharedGlobalCatalogData* self);

DiplomatStringView BaadSharedGlobalCatalogData_name(const BaadSharedGlobalCatalogData* self);

DiplomatStringView BaadSharedGlobalCatalogData_patch_state(const BaadSharedGlobalCatalogData* self);

bool BaadSharedGlobalCatalogData_security_checked(const BaadSharedGlobalCatalogData* self);

bool BaadSharedGlobalCatalogData_multi_language(const BaadSharedGlobalCatalogData* self);

bool BaadSharedGlobalCatalogData_multi_texture_encode(const BaadSharedGlobalCatalogData* self);

bool BaadSharedGlobalCatalogData_multi_texture_quality(const BaadSharedGlobalCatalogData* self);

DiplomatStringView BaadSharedGlobalCatalogData_description(const BaadSharedGlobalCatalogData* self);

DiplomatStringView BaadSharedGlobalCatalogData_register(const BaadSharedGlobalCatalogData* self);

DiplomatStringView BaadSharedGlobalCatalogData_register_date(const BaadSharedGlobalCatalogData* self);

DiplomatStringView BaadSharedGlobalCatalogData_updater(const BaadSharedGlobalCatalogData* self);

DiplomatStringView BaadSharedGlobalCatalogData_update_date(const BaadSharedGlobalCatalogData* self);

bool BaadSharedGlobalCatalogData_compress(const BaadSharedGlobalCatalogData* self);

int64_t BaadSharedGlobalCatalogData_size(const BaadSharedGlobalCatalogData* self);

int32_t BaadSharedGlobalCatalogData_count(const BaadSharedGlobalCatalogData* self);

bool BaadSharedGlobalCatalogData_use_multi_resource(const BaadSharedGlobalCatalogData* self);

const BaadSharedCategory* BaadSharedGlobalCatalogData_category(const BaadSharedGlobalCatalogData* self);

size_t BaadSharedGlobalCatalogData_category_mapping_len(const BaadSharedGlobalCatalogData* self);

const BaadSharedCategoryMapping* BaadSharedGlobalCatalogData_category_mapping_at(const BaadSharedGlobalCatalogData* self, size_t index);

size_t BaadSharedGlobalCatalogData_resources_len(const BaadSharedGlobalCatalogData* self);

const BaadSharedResource* BaadSharedGlobalCatalogData_resources_at(const BaadSharedGlobalCatalogData* self, size_t index);

void BaadSharedGlobalCatalogData_destroy(BaadSharedGlobalCatalogData* self);





#endif // BaadSharedGlobalCatalogData_H
