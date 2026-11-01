#ifndef BaadSharedBundleFileCN_H
#define BaadSharedBundleFileCN_H

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"


#include "BaadSharedBundleFileCN.d.h"






DiplomatStringView BaadSharedBundleFileCN_name(const BaadSharedBundleFileCN* self);

int64_t BaadSharedBundleFileCN_size(const BaadSharedBundleFileCN* self);

bool BaadSharedBundleFileCN_is_prologue(const BaadSharedBundleFileCN* self);

DiplomatStringView BaadSharedBundleFileCN_crc(const BaadSharedBundleFileCN* self);

bool BaadSharedBundleFileCN_is_split_download(const BaadSharedBundleFileCN* self);

void BaadSharedBundleFileCN_destroy(BaadSharedBundleFileCN* self);





#endif // BaadSharedBundleFileCN_H
