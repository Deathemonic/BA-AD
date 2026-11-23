#ifndef BaadUtils_H
#define BaadUtils_H

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"

#include "../../baad-shared/bindings/BaadError.d.h"
#include "BaadUtilsBytes.d.h"
#include "BaadUtilsJson.d.h"
#include "BaadUtilsLogLevel.d.h"
#include "BaadUtilsLoggingConfig.d.h"
#include "BaadUtilsProgressDisplay.d.h"
#include "BaadUtilsProxy.d.h"
#include "BaadUtilsResponse.d.h"
#include "BaadUtilsStyle.d.h"
#include "BaadUtilsTerminalSize.d.h"

#include "BaadUtils.d.h"





typedef struct DiplomatCallback_BaadUtils_filename_matches_matches {
    const void* data;
    bool (*run_callback)(const void*, DiplomatStringView );
    void (*destructor)(const void*);
} DiplomatCallback_BaadUtils_filename_matches_matches;
typedef struct DiplomatCallback_BaadUtils_json_update_updater {
    const void* data;
    void (*run_callback)(const void*, BaadUtilsJson* );
    void (*destructor)(const void*);
} DiplomatCallback_BaadUtils_json_update_updater;
typedef struct DiplomatCallback_BaadUtils_run_job {
    const void* data;
    bool (*run_callback)(const void*);
    void (*destructor)(const void*);
} DiplomatCallback_BaadUtils_run_job;
typedef struct DiplomatCallback_BaadUtils_run_async_job {
    const void* data;
    bool (*run_callback)(const void*);
    void (*destructor)(const void*);
} DiplomatCallback_BaadUtils_run_async_job;

bool BaadUtils_contains_url(DiplomatStringView value);

typedef struct BaadUtils_format_urls_result {union { BaadError* err;}; bool is_ok;} BaadUtils_format_urls_result;
BaadUtils_format_urls_result BaadUtils_format_urls(DiplomatStringView value, const BaadUtilsStyle* text_style, const BaadUtilsStyle* url_style, DiplomatWrite* write);

typedef struct BaadUtils_init_logging_result {union { BaadError* err;}; bool is_ok;} BaadUtils_init_logging_result;
BaadUtils_init_logging_result BaadUtils_init_logging(BaadUtilsLoggingConfig c);

void BaadUtils_flush_logs(void);

typedef struct BaadUtils_set_app_name_result {union { BaadError* err;}; bool is_ok;} BaadUtils_set_app_name_result;
BaadUtils_set_app_name_result BaadUtils_set_app_name(DiplomatStringView name);

typedef struct BaadUtils_set_data_dir_result {union { BaadError* err;}; bool is_ok;} BaadUtils_set_data_dir_result;
BaadUtils_set_data_dir_result BaadUtils_set_data_dir(DiplomatStringView path);

typedef struct BaadUtils_data_dir_result {union { BaadError* err;}; bool is_ok;} BaadUtils_data_dir_result;
BaadUtils_data_dir_result BaadUtils_data_dir(DiplomatWrite* write);

typedef struct BaadUtils_get_data_path_result {union { BaadError* err;}; bool is_ok;} BaadUtils_get_data_path_result;
BaadUtils_get_data_path_result BaadUtils_get_data_path(DiplomatStringView filename, DiplomatWrite* write);

DiplomatStringView BaadUtils_filename_or(DiplomatStringView path);

typedef struct BaadUtils_load_file_result {union {BaadUtilsBytes* ok; BaadError* err;}; bool is_ok;} BaadUtils_load_file_result;
BaadUtils_load_file_result BaadUtils_load_file(DiplomatStringView path);

typedef struct BaadUtils_save_file_result {union { BaadError* err;}; bool is_ok;} BaadUtils_save_file_result;
BaadUtils_save_file_result BaadUtils_save_file(DiplomatStringView path, DiplomatU8View bytes);

typedef struct BaadUtils_json_save_result {union { BaadError* err;}; bool is_ok;} BaadUtils_json_save_result;
BaadUtils_json_save_result BaadUtils_json_save(DiplomatStringView path, DiplomatStringView json);

typedef struct BaadUtils_get_output_dir_result {union { BaadError* err;}; bool is_ok;} BaadUtils_get_output_dir_result;
BaadUtils_get_output_dir_result BaadUtils_get_output_dir(DiplomatStringView path, DiplomatWrite* write);

void BaadUtils_progress_started(DiplomatStringView id, DiplomatStringView label, bool count_unit, uint64_t total);

void BaadUtils_progress_advance(DiplomatStringView id, uint64_t current, uint64_t total);

void BaadUtils_progress_completed(DiplomatStringView id);

void BaadUtils_progress_failed(DiplomatStringView id, DiplomatStringView reason);

void BaadUtils_log(BaadUtilsLogLevel level, bool success, DiplomatStringView message);

void BaadUtils_log_with_field(BaadUtilsLogLevel level, bool success, DiplomatStringView message, DiplomatStringView name, DiplomatStringView value);

typedef struct BaadUtils_log_with_fields_result {union { BaadError* err;}; bool is_ok;} BaadUtils_log_with_fields_result;
BaadUtils_log_with_fields_result BaadUtils_log_with_fields(BaadUtilsLogLevel level, bool success, DiplomatStringView message, DiplomatStringsView names, DiplomatStringsView values);

void BaadUtils_format_bytes(uint64_t value, DiplomatWrite* write);

bool BaadUtils_terminal_is_terminal(void);

BaadUtilsTerminalSize BaadUtils_terminal_size(void);

typedef struct BaadUtils_create_parent_dir_result {union { BaadError* err;}; bool is_ok;} BaadUtils_create_parent_dir_result;
BaadUtils_create_parent_dir_result BaadUtils_create_parent_dir(DiplomatStringView path);

typedef struct BaadUtils_is_dir_empty_result {union {bool ok; BaadError* err;}; bool is_ok;} BaadUtils_is_dir_empty_result;
BaadUtils_is_dir_empty_result BaadUtils_is_dir_empty(DiplomatStringView path);

typedef struct BaadUtils_clear_all_result {union { BaadError* err;}; bool is_ok;} BaadUtils_clear_all_result;
BaadUtils_clear_all_result BaadUtils_clear_all(DiplomatStringView path);

typedef struct BaadUtils_json_load_result {union { BaadError* err;}; bool is_ok;} BaadUtils_json_load_result;
BaadUtils_json_load_result BaadUtils_json_load(DiplomatStringView path, DiplomatWrite* write);

typedef struct BaadUtils_fetch_version_result {union { BaadError* err;}; bool is_ok;} BaadUtils_fetch_version_result;
BaadUtils_fetch_version_result BaadUtils_fetch_version(DiplomatStringView path, DiplomatWrite* write);

bool BaadUtils_filename_matches(DiplomatStringView path, DiplomatCallback_BaadUtils_filename_matches_matches matches_cb_wrap);

typedef struct BaadUtils_json_update_result {union { BaadError* err;}; bool is_ok;} BaadUtils_json_update_result;
BaadUtils_json_update_result BaadUtils_json_update(DiplomatStringView path, DiplomatCallback_BaadUtils_json_update_updater updater_cb_wrap);

typedef struct BaadUtils_create_proxy_result {union {BaadUtilsProxy* ok; BaadError* err;}; bool is_ok;} BaadUtils_create_proxy_result;
BaadUtils_create_proxy_result BaadUtils_create_proxy(DiplomatStringView url);

typedef struct BaadUtils_fetch_response_result {union {BaadUtilsResponse* ok; BaadError* err;}; bool is_ok;} BaadUtils_fetch_response_result;
BaadUtils_fetch_response_result BaadUtils_fetch_response(DiplomatStringView url);

void BaadUtils_run(DiplomatCallback_BaadUtils_run_job job_cb_wrap);

typedef struct BaadUtils_run_async_result {union { BaadError* err;}; bool is_ok;} BaadUtils_run_async_result;
BaadUtils_run_async_result BaadUtils_run_async(DiplomatCallback_BaadUtils_run_async_job job_cb_wrap);

void BaadUtils_log_recoverable_error(DiplomatStringView message, DiplomatStringView recovery_action);

typedef struct BaadUtils_init_logging_with_model_result {union { BaadError* err;}; bool is_ok;} BaadUtils_init_logging_with_model_result;
BaadUtils_init_logging_with_model_result BaadUtils_init_logging_with_model(BaadUtilsLoggingConfig config, BaadUtilsProgressDisplay* display);

void BaadUtils_destroy(BaadUtils* self);





#endif // BaadUtils_H
