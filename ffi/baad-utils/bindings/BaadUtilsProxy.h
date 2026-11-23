#ifndef BaadUtilsProxy_H
#define BaadUtilsProxy_H

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"

#include "../../baad-shared/bindings/BaadError.d.h"
#include "BaadUtilsResponse.d.h"

#include "BaadUtilsProxy.d.h"






typedef struct BaadUtilsProxy_fetch_response_result {union {BaadUtilsResponse* ok; BaadError* err;}; bool is_ok;} BaadUtilsProxy_fetch_response_result;
BaadUtilsProxy_fetch_response_result BaadUtilsProxy_fetch_response(const BaadUtilsProxy* self, DiplomatStringView url);

void BaadUtilsProxy_destroy(BaadUtilsProxy* self);





#endif // BaadUtilsProxy_H
