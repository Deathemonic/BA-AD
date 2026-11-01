#ifndef BaadDmDownloads_H
#define BaadDmDownloads_H

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"

#include "../../baad-shared/bindings/BaadError.d.h"

#include "BaadDmDownloads.d.h"






BaadDmDownloads* BaadDmDownloads_new(void);

typedef struct BaadDmDownloads_push_result {union { BaadError* err;}; bool is_ok;} BaadDmDownloads_push_result;
BaadDmDownloads_push_result BaadDmDownloads_push(BaadDmDownloads* self, DiplomatStringView url, DiplomatStringView filename, DiplomatStringView hash, DiplomatStringView target_file, OptionU64 size);

bool BaadDmDownloads_is_empty(const BaadDmDownloads* self);

size_t BaadDmDownloads_len(const BaadDmDownloads* self);

void BaadDmDownloads_destroy(BaadDmDownloads* self);





#endif // BaadDmDownloads_H
