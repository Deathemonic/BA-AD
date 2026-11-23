#ifndef BaadSharedTableBundleCN_H
#define BaadSharedTableBundleCN_H

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"


#include "BaadSharedTableBundleCN.d.h"






DiplomatStringView BaadSharedTableBundleCN_name(const BaadSharedTableBundleCN* self);

int64_t BaadSharedTableBundleCN_size(const BaadSharedTableBundleCN* self);

DiplomatStringView BaadSharedTableBundleCN_crc(const BaadSharedTableBundleCN* self);

bool BaadSharedTableBundleCN_is_inbuild(const BaadSharedTableBundleCN* self);

bool BaadSharedTableBundleCN_is_changed(const BaadSharedTableBundleCN* self);

bool BaadSharedTableBundleCN_is_prologue(const BaadSharedTableBundleCN* self);

bool BaadSharedTableBundleCN_is_split_download(const BaadSharedTableBundleCN* self);

bool BaadSharedTableBundleCN_has_includes(const BaadSharedTableBundleCN* self);

size_t BaadSharedTableBundleCN_includes_len(const BaadSharedTableBundleCN* self);

typedef struct BaadSharedTableBundleCN_includes_at_result {union {DiplomatStringView ok; }; bool is_ok;} BaadSharedTableBundleCN_includes_at_result;
BaadSharedTableBundleCN_includes_at_result BaadSharedTableBundleCN_includes_at(const BaadSharedTableBundleCN* self, size_t index);

void BaadSharedTableBundleCN_destroy(BaadSharedTableBundleCN* self);





#endif // BaadSharedTableBundleCN_H
