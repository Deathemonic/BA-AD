#ifndef BaadSharedTableBundle_H
#define BaadSharedTableBundle_H

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"


#include "BaadSharedTableBundle.d.h"






DiplomatStringView BaadSharedTableBundle_name(const BaadSharedTableBundle* self);

int64_t BaadSharedTableBundle_size(const BaadSharedTableBundle* self);

int64_t BaadSharedTableBundle_crc(const BaadSharedTableBundle* self);

bool BaadSharedTableBundle_is_inbuild(const BaadSharedTableBundle* self);

bool BaadSharedTableBundle_is_changed(const BaadSharedTableBundle* self);

bool BaadSharedTableBundle_is_prologue(const BaadSharedTableBundle* self);

bool BaadSharedTableBundle_is_split_download(const BaadSharedTableBundle* self);

size_t BaadSharedTableBundle_includes_len(const BaadSharedTableBundle* self);

typedef struct BaadSharedTableBundle_includes_at_result {union {DiplomatStringView ok; }; bool is_ok;} BaadSharedTableBundle_includes_at_result;
BaadSharedTableBundle_includes_at_result BaadSharedTableBundle_includes_at(const BaadSharedTableBundle* self, size_t index);

void BaadSharedTableBundle_destroy(BaadSharedTableBundle* self);





#endif // BaadSharedTableBundle_H
