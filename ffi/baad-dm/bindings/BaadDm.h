#ifndef BaadDm_H
#define BaadDm_H

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"

#include "BaadDmDownloaderConfig.d.h"
#include "BaadDmDownloads.d.h"
#include "BaadDmHashType.d.h"
#include "BaadDmSummaries.d.h"
#include "BaadDmZipIndex.d.h"
#include "../../baad-shared/bindings/BaadError.d.h"
#include "../../baad-utils/bindings/BaadUtilsBytes.d.h"

#include "BaadDm.d.h"






typedef struct BaadDm_detect_hash_type_result {union {BaadDmHashType ok; }; bool is_ok;} BaadDm_detect_hash_type_result;
BaadDm_detect_hash_type_result BaadDm_detect_hash_type(DiplomatStringView hash);

typedef struct BaadDm_verify_hash_result {union {bool ok; BaadError* err;}; bool is_ok;} BaadDm_verify_hash_result;
BaadDm_verify_hash_result BaadDm_verify_hash(DiplomatStringView path, DiplomatStringView hash);

void BaadDm_range_header(uint64_t start, OptionU64 end, DiplomatWrite* write);

typedef struct BaadDm_resolve_url_result {union { BaadError* err;}; bool is_ok;} BaadDm_resolve_url_result;
BaadDm_resolve_url_result BaadDm_resolve_url(DiplomatStringView url, DiplomatWrite* write);

typedef struct BaadDm_download_result {union {BaadDmSummaries* ok; BaadError* err;}; bool is_ok;} BaadDm_download_result;
BaadDm_download_result BaadDm_download(BaadDmDownloaderConfig config, DiplomatStringView directory, DiplomatStringView proxy, BaadDmDownloads* items);

typedef struct BaadDm_zip_extract_file_result {union {BaadUtilsBytes* ok; BaadError* err;}; bool is_ok;} BaadDm_zip_extract_file_result;
BaadDm_zip_extract_file_result BaadDm_zip_extract_file(DiplomatStringView url, DiplomatStringView target);

typedef struct BaadDm_zip_index_result {union {BaadDmZipIndex* ok; BaadError* err;}; bool is_ok;} BaadDm_zip_index_result;
BaadDm_zip_index_result BaadDm_zip_index(DiplomatStringView url);

void BaadDm_destroy(BaadDm* self);





#endif // BaadDm_H
