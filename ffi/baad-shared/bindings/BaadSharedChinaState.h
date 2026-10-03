#ifndef BaadSharedChinaState_H
#define BaadSharedChinaState_H

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"


#include "BaadSharedChinaState.d.h"






size_t BaadSharedChinaState_addressables_catalog_url_roots_len(const BaadSharedChinaState* self);

typedef struct BaadSharedChinaState_addressables_catalog_url_roots_at_result {union {DiplomatStringView ok; }; bool is_ok;} BaadSharedChinaState_addressables_catalog_url_roots_at_result;
BaadSharedChinaState_addressables_catalog_url_roots_at_result BaadSharedChinaState_addressables_catalog_url_roots_at(const BaadSharedChinaState* self, size_t index);

DiplomatStringView BaadSharedChinaState_resource_version(const BaadSharedChinaState* self);

DiplomatStringView BaadSharedChinaState_table_version(const BaadSharedChinaState* self);

DiplomatStringView BaadSharedChinaState_media_version(const BaadSharedChinaState* self);

DiplomatStringView BaadSharedChinaState_patch_version(const BaadSharedChinaState* self);

int32_t BaadSharedChinaState_state(const BaadSharedChinaState* self);

bool BaadSharedChinaState_is_open_pre_download(const BaadSharedChinaState* self);

void BaadSharedChinaState_destroy(BaadSharedChinaState* self);





#endif // BaadSharedChinaState_H
