#ifndef BaadSharedTableCatalogCN_H
#define BaadSharedTableCatalogCN_H

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"

#include "BaadError.d.h"
#include "BaadSharedStrings.d.h"
#include "BaadSharedTableBundleCN.d.h"

#include "BaadSharedTableCatalogCN.d.h"






typedef struct BaadSharedTableCatalogCN_parse_json_result {union {BaadSharedTableCatalogCN* ok; BaadError* err;}; bool is_ok;} BaadSharedTableCatalogCN_parse_json_result;
BaadSharedTableCatalogCN_parse_json_result BaadSharedTableCatalogCN_parse_json(DiplomatStringView json);

BaadSharedStrings* BaadSharedTableCatalogCN_table_keys(const BaadSharedTableCatalogCN* self);

const BaadSharedTableBundleCN* BaadSharedTableCatalogCN_table_get(const BaadSharedTableCatalogCN* self, DiplomatStringView key);

void BaadSharedTableCatalogCN_destroy(BaadSharedTableCatalogCN* self);





#endif // BaadSharedTableCatalogCN_H
