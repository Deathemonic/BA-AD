#ifndef BaadSharedMediaCatalogCN_H
#define BaadSharedMediaCatalogCN_H

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"

#include "BaadError.d.h"
#include "BaadSharedMediaCN.d.h"

#include "BaadSharedMediaCatalogCN.d.h"






typedef struct BaadSharedMediaCatalogCN_parse_json_result {union {BaadSharedMediaCatalogCN* ok; BaadError* err;}; bool is_ok;} BaadSharedMediaCatalogCN_parse_json_result;
BaadSharedMediaCatalogCN_parse_json_result BaadSharedMediaCatalogCN_parse_json(DiplomatStringView json);

size_t BaadSharedMediaCatalogCN_table_len(const BaadSharedMediaCatalogCN* self);

const BaadSharedMediaCN* BaadSharedMediaCatalogCN_table_at(const BaadSharedMediaCatalogCN* self, size_t index);

void BaadSharedMediaCatalogCN_destroy(BaadSharedMediaCatalogCN* self);





#endif // BaadSharedMediaCatalogCN_H
