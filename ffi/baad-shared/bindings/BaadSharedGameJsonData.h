#ifndef BaadSharedGameJsonData_H
#define BaadSharedGameJsonData_H

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"

#include "BaadSharedGameFile.d.h"

#include "BaadSharedGameJsonData.d.h"






DiplomatStringView BaadSharedGameJsonData_source(const BaadSharedGameJsonData* self);

size_t BaadSharedGameJsonData_file_len(const BaadSharedGameJsonData* self);

const BaadSharedGameFile* BaadSharedGameJsonData_file_at(const BaadSharedGameJsonData* self, size_t index);

void BaadSharedGameJsonData_destroy(BaadSharedGameJsonData* self);





#endif // BaadSharedGameJsonData_H
