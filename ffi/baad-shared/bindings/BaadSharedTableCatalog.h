#ifndef BaadSharedTableCatalog_H
#define BaadSharedTableCatalog_H

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"

#include "BaadError.d.h"
#include "BaadSharedStrings.d.h"
#include "BaadSharedTableBundle.d.h"
#include "BaadSharedTablePatchPack.d.h"

#include "BaadSharedTableCatalog.d.h"






typedef struct BaadSharedTableCatalog_parse_json_result {union {BaadSharedTableCatalog* ok; BaadError* err;}; bool is_ok;} BaadSharedTableCatalog_parse_json_result;
BaadSharedTableCatalog_parse_json_result BaadSharedTableCatalog_parse_json(DiplomatStringView json);

BaadSharedStrings* BaadSharedTableCatalog_table_keys(const BaadSharedTableCatalog* self);

const BaadSharedTableBundle* BaadSharedTableCatalog_table_get(const BaadSharedTableCatalog* self, DiplomatStringView key);

BaadSharedStrings* BaadSharedTableCatalog_table_pack_keys(const BaadSharedTableCatalog* self);

const BaadSharedTablePatchPack* BaadSharedTableCatalog_table_pack_get(const BaadSharedTableCatalog* self, DiplomatStringView key);

void BaadSharedTableCatalog_destroy(BaadSharedTableCatalog* self);





#endif // BaadSharedTableCatalog_H
