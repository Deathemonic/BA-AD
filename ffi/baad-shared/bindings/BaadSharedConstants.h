#ifndef BaadSharedConstants_H
#define BaadSharedConstants_H

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"


#include "BaadSharedConstants.d.h"






DiplomatStringView BaadSharedConstants_global_api_url(void);

DiplomatStringView BaadSharedConstants_global_playstore_url(void);

DiplomatStringView BaadSharedConstants_global_appstore_url(void);

DiplomatStringView BaadSharedConstants_global_android_standard_id(void);

DiplomatStringView BaadSharedConstants_global_android_teen_id(void);

DiplomatStringView BaadSharedConstants_global_ios_standard_id(void);

DiplomatStringView BaadSharedConstants_global_ios_teen_id(void);

DiplomatStringView BaadSharedConstants_playstore_code(void);

DiplomatStringView BaadSharedConstants_appstore_code(void);

DiplomatStringView BaadSharedConstants_yostar_base_url(void);

DiplomatStringView BaadSharedConstants_yostar_game_base_config_path(void);

DiplomatStringView BaadSharedConstants_yostar_game_json_config_path(void);

DiplomatStringView BaadSharedConstants_yostar_domain_path(void);

DiplomatStringView BaadSharedConstants_yostar_game_tag(void);

DiplomatStringView BaadSharedConstants_yostar_signature_data(void);

DiplomatStringView BaadSharedConstants_yostar_version(void);

DiplomatStringView BaadSharedConstants_rostar_version_url(void);

DiplomatStringView BaadSharedConstants_rostar_state_url(void);

DiplomatStringView BaadSharedConstants_rostar_platform_id(void);

DiplomatStringView BaadSharedConstants_rostar_channel_id(void);

DiplomatStringView BaadSharedConstants_platform_name_android(void);

DiplomatStringView BaadSharedConstants_platform_name_ios(void);

DiplomatStringView BaadSharedConstants_platform_name_windows(void);

DiplomatStringView BaadSharedConstants_api_filename(void);

DiplomatU8View BaadSharedConstants_game_config_pattern(void);

DiplomatStringView BaadSharedConstants_global_apk_path(void);

DiplomatStringView BaadSharedConstants_japan_apk_path(void);

DiplomatStringView BaadSharedConstants_config_apk(void);

size_t BaadSharedConstants_libil2cpp_path_len(void);

typedef struct BaadSharedConstants_libil2cpp_path_at_result {union {DiplomatStringView ok; }; bool is_ok;} BaadSharedConstants_libil2cpp_path_at_result;
BaadSharedConstants_libil2cpp_path_at_result BaadSharedConstants_libil2cpp_path_at(size_t index);

DiplomatStringView BaadSharedConstants_libil2cpp_pattern(void);

DiplomatStringView BaadSharedConstants_asset_apk(void);

DiplomatStringView BaadSharedConstants_jp_data_apk(void);

DiplomatStringView BaadSharedConstants_global_data_apk(void);

size_t BaadSharedConstants_data_path_len(void);

typedef struct BaadSharedConstants_data_path_at_result {union {DiplomatStringView ok; }; bool is_ok;} BaadSharedConstants_data_path_at_result;
BaadSharedConstants_data_path_at_result BaadSharedConstants_data_path_at(size_t index);

size_t BaadSharedConstants_metadata_path_len(void);

typedef struct BaadSharedConstants_metadata_path_at_result {union {DiplomatStringView ok; }; bool is_ok;} BaadSharedConstants_metadata_path_at_result;
BaadSharedConstants_metadata_path_at_result BaadSharedConstants_metadata_path_at(size_t index);

DiplomatStringView BaadSharedConstants_data_pattern(void);

DiplomatStringView BaadSharedConstants_metadata_pattern(void);

DiplomatStringView BaadSharedConstants_executable_name(void);

DiplomatStringView BaadSharedConstants_asset_bundles(void);

DiplomatStringView BaadSharedConstants_table_bundles(void);

DiplomatStringView BaadSharedConstants_media_resources(void);

DiplomatStringView BaadSharedConstants_media_resources_windows(void);

DiplomatStringView BaadSharedConstants_patch_pack_android(void);

DiplomatStringView BaadSharedConstants_patch_pack_ios(void);

DiplomatStringView BaadSharedConstants_patch_pack_windows(void);

DiplomatStringView BaadSharedConstants_catalog_prefix(void);

DiplomatStringView BaadSharedConstants_library_version(void);

void BaadSharedConstants_destroy(BaadSharedConstants* self);





#endif // BaadSharedConstants_H
