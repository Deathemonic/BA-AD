#ifndef BaadSharedGameFile_H
#define BaadSharedGameFile_H

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"


#include "BaadSharedGameFile.d.h"






DiplomatStringView BaadSharedGameFile_path(const BaadSharedGameFile* self);

DiplomatStringView BaadSharedGameFile_hash(const BaadSharedGameFile* self);

DiplomatStringView BaadSharedGameFile_size(const BaadSharedGameFile* self);

void BaadSharedGameFile_destroy(BaadSharedGameFile* self);





#endif // BaadSharedGameFile_H
