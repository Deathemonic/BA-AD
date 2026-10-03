#ifndef BaadResourceDownloader_H
#define BaadResourceDownloader_H

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"

#include "BaadDownloaderOptions.d.h"
#include "../../baad-shared/bindings/BaadError.d.h"
#include "BaadResourceFilter.d.h"
#include "../../baad-shared/bindings/BaadSharedDownloads.d.h"

#include "BaadResourceDownloader.d.h"






uint8_t BaadResourceDownloader_category(bool assets, bool tables, bool media);

typedef struct BaadResourceDownloader_download_result {union { BaadError* err;}; bool is_ok;} BaadResourceDownloader_download_result;
BaadResourceDownloader_download_result BaadResourceDownloader_download(BaadDownloaderOptions options, DiplomatStringView output_dir, DiplomatStringView proxy, BaadSharedDownloads* downloads, const BaadResourceFilter* filter);

typedef struct BaadResourceDownloader_download_file_result {union { BaadError* err;}; bool is_ok;} BaadResourceDownloader_download_file_result;
BaadResourceDownloader_download_file_result BaadResourceDownloader_download_file(DiplomatStringView url, DiplomatStringView output_path, DiplomatStringView hash, uint32_t retries);

void BaadResourceDownloader_destroy(BaadResourceDownloader* self);





#endif // BaadResourceDownloader_H
