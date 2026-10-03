#ifndef BaadSharedBundleFile_H
#define BaadSharedBundleFile_H

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"


#include "BaadSharedBundleFile.d.h"






DiplomatStringView BaadSharedBundleFile_name(const BaadSharedBundleFile* self);

int64_t BaadSharedBundleFile_size(const BaadSharedBundleFile* self);

bool BaadSharedBundleFile_is_prologue(const BaadSharedBundleFile* self);

int64_t BaadSharedBundleFile_crc(const BaadSharedBundleFile* self);

bool BaadSharedBundleFile_is_split_download(const BaadSharedBundleFile* self);

uint64_t BaadSharedBundleFile_file_hash(const BaadSharedBundleFile* self);

DiplomatStringView BaadSharedBundleFile_signature(const BaadSharedBundleFile* self);

void BaadSharedBundleFile_destroy(BaadSharedBundleFile* self);





#endif // BaadSharedBundleFile_H
