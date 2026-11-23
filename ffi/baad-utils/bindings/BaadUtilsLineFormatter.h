#ifndef BaadUtilsLineFormatter_H
#define BaadUtilsLineFormatter_H

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"

#include "../../baad-shared/bindings/BaadError.d.h"
#include "BaadUtilsAlignedLine.d.h"
#include "BaadUtilsLogLevel.d.h"

#include "BaadUtilsLineFormatter.d.h"






BaadUtilsLineFormatter* BaadUtilsLineFormatter_new(void);

void BaadUtilsLineFormatter_set_timestamps(BaadUtilsLineFormatter* self, bool enabled);

bool BaadUtilsLineFormatter_includes_timestamps(const BaadUtilsLineFormatter* self);

typedef struct BaadUtilsLineFormatter_write_timestamp_result {union { BaadError* err;}; bool is_ok;} BaadUtilsLineFormatter_write_timestamp_result;
BaadUtilsLineFormatter_write_timestamp_result BaadUtilsLineFormatter_write_timestamp(const BaadUtilsLineFormatter* self, DiplomatWrite* write);

typedef struct BaadUtilsLineFormatter_write_level_prefix_result {union { BaadError* err;}; bool is_ok;} BaadUtilsLineFormatter_write_level_prefix_result;
BaadUtilsLineFormatter_write_level_prefix_result BaadUtilsLineFormatter_write_level_prefix(const BaadUtilsLineFormatter* self, BaadUtilsLogLevel level, bool success, DiplomatWrite* write);

typedef struct BaadUtilsLineFormatter_write_simple_message_result {union { BaadError* err;}; bool is_ok;} BaadUtilsLineFormatter_write_simple_message_result;
BaadUtilsLineFormatter_write_simple_message_result BaadUtilsLineFormatter_write_simple_message(const BaadUtilsLineFormatter* self, BaadUtilsLogLevel level, bool success, DiplomatStringView message, DiplomatWrite* write);

typedef struct BaadUtilsLineFormatter_write_line_result {union { BaadError* err;}; bool is_ok;} BaadUtilsLineFormatter_write_line_result;
BaadUtilsLineFormatter_write_line_result BaadUtilsLineFormatter_write_line(const BaadUtilsLineFormatter* self, BaadUtilsLogLevel level, bool success, DiplomatStringView message, DiplomatStringsView names, DiplomatStringsView values, DiplomatWrite* write);

typedef struct BaadUtilsLineFormatter_write_line_aligned_result {union { BaadError* err;}; bool is_ok;} BaadUtilsLineFormatter_write_line_aligned_result;
BaadUtilsLineFormatter_write_line_aligned_result BaadUtilsLineFormatter_write_line_aligned(const BaadUtilsLineFormatter* self, BaadUtilsAlignedLine line, DiplomatWrite* write);

void BaadUtilsLineFormatter_destroy(BaadUtilsLineFormatter* self);





#endif // BaadUtilsLineFormatter_H
