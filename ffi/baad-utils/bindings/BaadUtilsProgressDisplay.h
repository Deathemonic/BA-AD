#ifndef BaadUtilsProgressDisplay_H
#define BaadUtilsProgressDisplay_H

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"

#include "../../baad-shared/bindings/BaadError.d.h"
#include "../../baad-shared/bindings/BaadSharedProgressEvent.d.h"

#include "BaadUtilsProgressDisplay.d.h"






BaadUtilsProgressDisplay* BaadUtilsProgressDisplay_new(void);

typedef struct BaadUtilsProgressDisplay_handle_event_result {union { BaadError* err;}; bool is_ok;} BaadUtilsProgressDisplay_handle_event_result;
BaadUtilsProgressDisplay_handle_event_result BaadUtilsProgressDisplay_handle_event(BaadUtilsProgressDisplay* self, const BaadSharedProgressEvent* event);

typedef struct BaadUtilsProgressDisplay_render_result {union { BaadError* err;}; bool is_ok;} BaadUtilsProgressDisplay_render_result;
BaadUtilsProgressDisplay_render_result BaadUtilsProgressDisplay_render(BaadUtilsProgressDisplay* self, size_t width, size_t height, DiplomatWrite* write);

void BaadUtilsProgressDisplay_destroy(BaadUtilsProgressDisplay* self);





#endif // BaadUtilsProgressDisplay_H
