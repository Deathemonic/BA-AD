#ifndef BaadSharedMediaCatalog_H
#define BaadSharedMediaCatalog_H

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"

#include "BaadError.d.h"
#include "BaadSharedMedia.d.h"
#include "BaadSharedStrings.d.h"

#include "BaadSharedMediaCatalog.d.h"






typedef struct BaadSharedMediaCatalog_parse_json_result {union {BaadSharedMediaCatalog* ok; BaadError* err;}; bool is_ok;} BaadSharedMediaCatalog_parse_json_result;
BaadSharedMediaCatalog_parse_json_result BaadSharedMediaCatalog_parse_json(DiplomatStringView json);

BaadSharedStrings* BaadSharedMediaCatalog_table_keys(const BaadSharedMediaCatalog* self);

const BaadSharedMedia* BaadSharedMediaCatalog_table_get(const BaadSharedMediaCatalog* self, DiplomatStringView key);

void BaadSharedMediaCatalog_destroy(BaadSharedMediaCatalog* self);





#endif // BaadSharedMediaCatalog_H
