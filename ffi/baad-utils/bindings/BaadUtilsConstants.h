#ifndef BaadUtilsConstants_H
#define BaadUtilsConstants_H

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"

#include "BaadUtilsStyle.d.h"

#include "BaadUtilsConstants.d.h"






size_t BaadUtilsConstants_level_prefixes_len(void);

typedef struct BaadUtilsConstants_level_prefixes_at_result {union {DiplomatStringView ok; }; bool is_ok;} BaadUtilsConstants_level_prefixes_at_result;
BaadUtilsConstants_level_prefixes_at_result BaadUtilsConstants_level_prefixes_at(size_t index);

DiplomatStringView BaadUtilsConstants_success_prefix(void);

DiplomatStringView BaadUtilsConstants_cause_prefix(void);

BaadUtilsStyle* BaadUtilsConstants_timestamp_style(void);

BaadUtilsStyle* BaadUtilsConstants_error_style(void);

BaadUtilsStyle* BaadUtilsConstants_warn_style(void);

BaadUtilsStyle* BaadUtilsConstants_info_style(void);

BaadUtilsStyle* BaadUtilsConstants_debug_style(void);

BaadUtilsStyle* BaadUtilsConstants_trace_style(void);

BaadUtilsStyle* BaadUtilsConstants_success_style(void);

BaadUtilsStyle* BaadUtilsConstants_cause_style(void);

BaadUtilsStyle* BaadUtilsConstants_error_value_style(void);

BaadUtilsStyle* BaadUtilsConstants_warn_value_style(void);

BaadUtilsStyle* BaadUtilsConstants_info_value_style(void);

BaadUtilsStyle* BaadUtilsConstants_debug_value_style(void);

BaadUtilsStyle* BaadUtilsConstants_trace_value_style(void);

BaadUtilsStyle* BaadUtilsConstants_success_value_style(void);

BaadUtilsStyle* BaadUtilsConstants_cause_value_style(void);

void BaadUtilsConstants_destroy(BaadUtilsConstants* self);





#endif // BaadUtilsConstants_H
