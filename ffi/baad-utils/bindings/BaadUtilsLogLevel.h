#ifndef BaadUtilsLogLevel_H
#define BaadUtilsLogLevel_H

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"

#include "BaadUtilsStyle.d.h"

#include "BaadUtilsLogLevel.d.h"






size_t BaadUtilsLogLevel_visual_length(BaadUtilsLogLevel self, bool success);

size_t BaadUtilsLogLevel_index(BaadUtilsLogLevel self);

BaadUtilsStyle* BaadUtilsLogLevel_style(BaadUtilsLogLevel self);

BaadUtilsStyle* BaadUtilsLogLevel_value_style(BaadUtilsLogLevel self);





#endif // BaadUtilsLogLevel_H
