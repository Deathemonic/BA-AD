#ifndef BaadSharedBundleCatalogCN_H
#define BaadSharedBundleCatalogCN_H

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"

#include "BaadError.d.h"
#include "BaadSharedBundleFileCN.d.h"

#include "BaadSharedBundleCatalogCN.d.h"






typedef struct BaadSharedBundleCatalogCN_parse_json_result {union {BaadSharedBundleCatalogCN* ok; BaadError* err;}; bool is_ok;} BaadSharedBundleCatalogCN_parse_json_result;
BaadSharedBundleCatalogCN_parse_json_result BaadSharedBundleCatalogCN_parse_json(DiplomatStringView json);

size_t BaadSharedBundleCatalogCN_bundle_files_len(const BaadSharedBundleCatalogCN* self);

const BaadSharedBundleFileCN* BaadSharedBundleCatalogCN_bundle_files_at(const BaadSharedBundleCatalogCN* self, size_t index);

void BaadSharedBundleCatalogCN_destroy(BaadSharedBundleCatalogCN* self);





#endif // BaadSharedBundleCatalogCN_H
