#ifndef BaadUtilsProgressView_H
#define BaadUtilsProgressView_H

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"

#include "../../baad-shared/bindings/BaadError.d.h"
#include "../../baad-shared/bindings/BaadSharedProgressEvent.d.h"
#include "BaadUtilsProgressDisplay.d.h"

#include "BaadUtilsProgressView.d.h"






typedef struct BaadUtilsProgressView_new_result {union {BaadUtilsProgressView* ok; BaadError* err;}; bool is_ok;} BaadUtilsProgressView_new_result;
BaadUtilsProgressView_new_result BaadUtilsProgressView_new(BaadUtilsProgressDisplay* display, uint64_t interval_ms);

typedef struct BaadUtilsProgressView_handle_event_result {union { BaadError* err;}; bool is_ok;} BaadUtilsProgressView_handle_event_result;
BaadUtilsProgressView_handle_event_result BaadUtilsProgressView_handle_event(const BaadUtilsProgressView* self, const BaadSharedProgressEvent* event);

typedef struct BaadUtilsProgressView_message_result {union { BaadError* err;}; bool is_ok;} BaadUtilsProgressView_message_result;
BaadUtilsProgressView_message_result BaadUtilsProgressView_message(const BaadUtilsProgressView* self, DiplomatStringView message);

typedef struct BaadUtilsProgressView_clear_result {union { BaadError* err;}; bool is_ok;} BaadUtilsProgressView_clear_result;
BaadUtilsProgressView_clear_result BaadUtilsProgressView_clear(const BaadUtilsProgressView* self);

typedef struct BaadUtilsProgressView_finish_result {union {BaadUtilsProgressDisplay* ok; BaadError* err;}; bool is_ok;} BaadUtilsProgressView_finish_result;
BaadUtilsProgressView_finish_result BaadUtilsProgressView_finish(BaadUtilsProgressView* self);

typedef struct BaadUtilsProgressView_write_result {union {size_t ok; BaadError* err;}; bool is_ok;} BaadUtilsProgressView_write_result;
BaadUtilsProgressView_write_result BaadUtilsProgressView_write(const BaadUtilsProgressView* self, DiplomatU8View bytes);

void BaadUtilsProgressView_destroy(BaadUtilsProgressView* self);





#endif // BaadUtilsProgressView_H
