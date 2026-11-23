#ifndef BaadSharedGameBaseConfig_H
#define BaadSharedGameBaseConfig_H

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"


#include "BaadSharedGameBaseConfig.d.h"






DiplomatStringView BaadSharedGameBaseConfig_game_lowest_version(const BaadSharedGameBaseConfig* self);

DiplomatStringView BaadSharedGameBaseConfig_game_latest_version(const BaadSharedGameBaseConfig* self);

DiplomatStringView BaadSharedGameBaseConfig_game_latest_file_path(const BaadSharedGameBaseConfig* self);

DiplomatStringView BaadSharedGameBaseConfig_game_start_exe_name(const BaadSharedGameBaseConfig* self);

DiplomatStringView BaadSharedGameBaseConfig_game_file_size(const BaadSharedGameBaseConfig* self);

DiplomatStringView BaadSharedGameBaseConfig_game_file_size_type(const BaadSharedGameBaseConfig* self);

DiplomatStringView BaadSharedGameBaseConfig_crc64(const BaadSharedGameBaseConfig* self);

int64_t BaadSharedGameBaseConfig_size(const BaadSharedGameBaseConfig* self);

DiplomatStringView BaadSharedGameBaseConfig_file_url(const BaadSharedGameBaseConfig* self);

DiplomatStringView BaadSharedGameBaseConfig_decompression_size(const BaadSharedGameBaseConfig* self);

int32_t BaadSharedGameBaseConfig_config_id(const BaadSharedGameBaseConfig* self);

size_t BaadSharedGameBaseConfig_game_start_params_len(const BaadSharedGameBaseConfig* self);

typedef struct BaadSharedGameBaseConfig_game_start_params_at_result {union {DiplomatStringView ok; }; bool is_ok;} BaadSharedGameBaseConfig_game_start_params_at_result;
BaadSharedGameBaseConfig_game_start_params_at_result BaadSharedGameBaseConfig_game_start_params_at(const BaadSharedGameBaseConfig* self, size_t index);

void BaadSharedGameBaseConfig_destroy(BaadSharedGameBaseConfig* self);





#endif // BaadSharedGameBaseConfig_H
