#ifndef BaadSharedMediaCN_H
#define BaadSharedMediaCN_H

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"

#include "BaadSharedChinaMediaType.d.h"

#include "BaadSharedMediaCN.d.h"






DiplomatStringView BaadSharedMediaCN_path(const BaadSharedMediaCN* self);

DiplomatStringView BaadSharedMediaCN_hash(const BaadSharedMediaCN* self);

BaadSharedChinaMediaType BaadSharedMediaCN_media_type(const BaadSharedMediaCN* self);

int64_t BaadSharedMediaCN_size(const BaadSharedMediaCN* self);

void BaadSharedMediaCN_destroy(BaadSharedMediaCN* self);





#endif // BaadSharedMediaCN_H
