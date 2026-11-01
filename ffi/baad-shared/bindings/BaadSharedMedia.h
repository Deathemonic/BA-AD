#ifndef BaadSharedMedia_H
#define BaadSharedMedia_H

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"


#include "BaadSharedMedia.d.h"






DiplomatStringView BaadSharedMedia_path(const BaadSharedMedia* self);

DiplomatStringView BaadSharedMedia_file_name(const BaadSharedMedia* self);

int64_t BaadSharedMedia_bytes(const BaadSharedMedia* self);

int64_t BaadSharedMedia_crc(const BaadSharedMedia* self);

bool BaadSharedMedia_is_prologue(const BaadSharedMedia* self);

bool BaadSharedMedia_is_split_download(const BaadSharedMedia* self);

int32_t BaadSharedMedia_media_type(const BaadSharedMedia* self);

void BaadSharedMedia_destroy(BaadSharedMedia* self);





#endif // BaadSharedMedia_H
