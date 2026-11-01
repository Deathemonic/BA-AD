#ifndef BaadSharedDownloadMedia_H
#define BaadSharedDownloadMedia_H

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"

#include "BaadSharedHashKind.d.h"

#include "BaadSharedDownloadMedia.d.h"






DiplomatStringView BaadSharedDownloadMedia_url(const BaadSharedDownloadMedia* self);

DiplomatStringView BaadSharedDownloadMedia_path(const BaadSharedDownloadMedia* self);

BaadSharedHashKind BaadSharedDownloadMedia_hash_kind(const BaadSharedDownloadMedia* self);

void BaadSharedDownloadMedia_hash(const BaadSharedDownloadMedia* self, DiplomatWrite* write);

int64_t BaadSharedDownloadMedia_size(const BaadSharedDownloadMedia* self);

void BaadSharedDownloadMedia_destroy(BaadSharedDownloadMedia* self);





#endif // BaadSharedDownloadMedia_H
