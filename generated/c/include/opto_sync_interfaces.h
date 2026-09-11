#ifndef OPTO_SYNC_INTERFACES_H
#define OPTO_SYNC_INTERFACES_H

#include <stdbool.h>
#include <stddef.h>
#include <stdint.h>

typedef struct {
  const char *data;
  size_t length;
} opto_sync_string_view;

typedef struct {
  const uint8_t *data;
  size_t length;
} opto_sync_json_view;

typedef enum {
  OPTO_SYNC_OPERATION_UPSERT = 0,
  OPTO_SYNC_OPERATION_DELETE = 1
} opto_sync_operation;

typedef enum {
  OPTO_SYNC_STATUS_PENDING = 0,
  OPTO_SYNC_STATUS_CONFIRMED = 1
} opto_sync_local_protocol_status;

typedef enum {
  OPTO_SYNC_CONNECTIVITY_UNKNOWN = 0,
  OPTO_SYNC_CONNECTIVITY_OFFLINE = 1,
  OPTO_SYNC_CONNECTIVITY_LINK = 2,
  OPTO_SYNC_CONNECTIVITY_INTERNET = 3
} opto_sync_connectivity_state;

typedef struct {
  opto_sync_string_view table;
  opto_sync_string_view record_id;
  bool has_operation;
  opto_sync_operation operation;
  bool has_base_revision;
  opto_sync_string_view base_revision;
  opto_sync_json_view payload;
} opto_sync_ingest_record;

typedef struct {
  uint8_t format_version;
  bool has_source;
  opto_sync_string_view source;
  const opto_sync_ingest_record *records;
  size_t records_length;
} opto_sync_ingest_envelope;

typedef struct {
  opto_sync_string_view mutation_id;
  opto_sync_operation operation;
  opto_sync_string_view table;
  opto_sync_string_view record_id;
  bool has_payload;
  opto_sync_json_view payload;
  bool has_base_revision;
  opto_sync_string_view base_revision;
  bool resurrect;
  opto_sync_local_protocol_status status;
} opto_sync_protocol_mutation;

typedef struct {
  uint8_t protocol_version;
  opto_sync_string_view client_id;
  const opto_sync_protocol_mutation *mutations;
  size_t mutations_length;
} opto_sync_push_request;

typedef struct {
  opto_sync_connectivity_state state;
  uint64_t changed_at;
  bool has_verified_at;
  uint64_t verified_at;
} opto_sync_connectivity_snapshot;

#endif
