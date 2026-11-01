#ifndef BaadSharedPlatform_H
#define BaadSharedPlatform_H

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"

#include "BaadError.d.h"

#include "BaadSharedPlatform.d.h"






DiplomatStringView BaadSharedPlatform_patch_pack(BaadSharedPlatform self);

DiplomatStringView BaadSharedPlatform_media_path(BaadSharedPlatform self);

DiplomatStringView BaadSharedPlatform_display_name(BaadSharedPlatform self);

typedef struct BaadSharedPlatform_ensure_supported_result {union { BaadError* err;}; bool is_ok;} BaadSharedPlatform_ensure_supported_result;
BaadSharedPlatform_ensure_supported_result BaadSharedPlatform_ensure_supported(BaadSharedPlatform self);





#endif // BaadSharedPlatform_H
