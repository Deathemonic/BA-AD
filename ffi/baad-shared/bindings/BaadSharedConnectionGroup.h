#ifndef BaadSharedConnectionGroup_H
#define BaadSharedConnectionGroup_H

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"

#include "BaadSharedOverrideConnectionGroup.d.h"

#include "BaadSharedConnectionGroup.d.h"






DiplomatStringView BaadSharedConnectionGroup_name(const BaadSharedConnectionGroup* self);

DiplomatStringView BaadSharedConnectionGroup_management_data_url(const BaadSharedConnectionGroup* self);

bool BaadSharedConnectionGroup_is_production_addressables(const BaadSharedConnectionGroup* self);

DiplomatStringView BaadSharedConnectionGroup_api_url(const BaadSharedConnectionGroup* self);

DiplomatStringView BaadSharedConnectionGroup_gateway_url(const BaadSharedConnectionGroup* self);

DiplomatStringView BaadSharedConnectionGroup_kibana_log_url(const BaadSharedConnectionGroup* self);

DiplomatStringView BaadSharedConnectionGroup_prohibited_word_black_list_uri(const BaadSharedConnectionGroup* self);

DiplomatStringView BaadSharedConnectionGroup_prohibited_word_white_list_uri(const BaadSharedConnectionGroup* self);

DiplomatStringView BaadSharedConnectionGroup_customer_service_url(const BaadSharedConnectionGroup* self);

size_t BaadSharedConnectionGroup_override_connection_groups_len(const BaadSharedConnectionGroup* self);

const BaadSharedOverrideConnectionGroup* BaadSharedConnectionGroup_override_connection_groups_at(const BaadSharedConnectionGroup* self, size_t index);

DiplomatStringView BaadSharedConnectionGroup_bundle_version(const BaadSharedConnectionGroup* self);

void BaadSharedConnectionGroup_destroy(BaadSharedConnectionGroup* self);





#endif // BaadSharedConnectionGroup_H
