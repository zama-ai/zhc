/* C ABI for ZHC.
 *
 * Every function binds a method of the same name in the Rust crates. See the Rust
 * documentation of the bound method for semantics.
 *
 * Status and errors
 * - Every function returns a zhc_status and writes its result, if any, to the trailing `out`
 *   argument. On failure, `out` is left untouched.
 * - ZHC_ERR_PANIC means the Rust method panicked. The message is printed on stderr. After a
 *   panic from a builder or pipeline getter, discard the object: its state may be partial. A
 *   panic from a pipeline setter leaves the pipeline unchanged.
 * - Panics are caught. Aborts are not: a stack overflow or out-of-memory still ends the process.
 *
 * Ownership
 * - Every handle written to an `out` pointer is owned by the caller. Release it with the
 *   matching `*_free` function, and only that one. Never pass it to free().
 * - `*_free` accepts NULL. Freeing a handle twice is undefined behaviour.
 * - Pointer arguments are borrowed for the duration of the call. Strings are copied. Tables
 *   are read. The caller keeps ownership.
 * - Strings written by `*_debug`, `*_dump` and `zhc_builder_dump_noise` are owned by the
 *   caller. Release them with zhc_string_free, and only that one.
 * - Strings are null-terminated. Invalid UTF-8 is replaced, not rejected.
 *
 * Builder and values
 * - A zhc_ciphertext_block, zhc_plaintext_block, zhc_bool_ciphertext, zhc_integer_ciphertext
 *   or zhc_integer_plaintext is a reference to a value of one builder's circuit. Freeing it
 *   does not change the circuit. It has no meaning for another builder, and none after its
 *   builder is freed; the library does not check either.
 * - A zhc_lut* is copied into the circuit by each lookup call. It may be freed right after.
 *   Its spec must be the builder's spec.
 * - The `out` array of lookup2/lookup4/lookup8 must have exactly 2/4/8 slots. On failure,
 *   nothing is written to it.
 *
 * Pipeline
 * - zhc_pipeline_set_builder takes a snapshot. Later changes to the builder do not affect the
 *   pipeline. The builder may be freed right after.
 * - The three target configs are mutually exclusive. Setting a second one panics.
 * - Getters compile on demand and cache. They take a non-const pipeline for this reason.
 *   Calling a getter of another flow than the configured one panics.
 * - Files and traces written by getters are copies of paths. The pipeline keeps its own.
 *   The files live in the system temporary directory and are never deleted by the library.
 *
 * Threads
 * - No function is thread-safe. Never use one handle from two threads at the same time, and
 *   never move a zhc_builder or one of its values to another thread: the builder shares
 *   state with its values through non-atomic reference counting.
 */

#ifndef ZHC_H
#define ZHC_H

#include <stdbool.h>
#include <stddef.h>
#include <stdint.h>

#ifdef __cplusplus
extern "C" {
#endif

/* ------------------------------------------------------------------------------------------- */
/* Status                                                                                      */
/* ------------------------------------------------------------------------------------------- */

typedef enum zhc_status {
    ZHC_OK = 0,
    ZHC_ERR_PANIC = 1,
    ZHC_ERR_NULL_POINTER = 2,
    ZHC_ERR_INVALID_ARGUMENT = 3,
    ZHC_ERR_IO = 4,
} zhc_status;

/* ------------------------------------------------------------------------------------------- */
/* Plain data                                                                                  */
/* ------------------------------------------------------------------------------------------- */

typedef struct zhc_ciphertext_block_spec {
    uint8_t carry;
    uint8_t message;
} zhc_ciphertext_block_spec;

typedef enum zhc_flavor {
    ZHC_FLAVOR_PROTECT,
    ZHC_FLAVOR_TEMPER,
    ZHC_FLAVOR_WRAPPING,
} zhc_flavor;

typedef struct zhc_lookup_check {
    bool allow_input_padding;
    bool allow_index_bits;
    bool allow_output_padding;
} zhc_lookup_check;

typedef enum zhc_ir_kind {
    ZHC_IR_KIND_ORIGINAL,
    ZHC_IR_KIND_OPTIMIZED,
} zhc_ir_kind;

typedef struct zhc_hpu_config {
    size_t freq;
    size_t isc_depth;
    size_t isc_query_period;
    size_t mem_fifo_capacity;
    size_t mem_read_latency;
    size_t mem_write_latency;
    size_t alu_fifo_capacity;
    size_t alu_read_latency;
    size_t alu_write_latency;
    size_t pbs_fifo_capacity;
    size_t pbs_memory_capacity;
    size_t pbs_min_batch_size;
    size_t pbs_max_batch_size;
    size_t pbs_timeout;
    size_t pbs_load_unload_latency;
    size_t pbs_processing_latency_a;
    size_t pbs_processing_latency_b;
    size_t pbs_processing_latency_m;
    size_t regf_size;
    size_t heap_size;
} zhc_hpu_config;

typedef struct zhc_multi_hpu_config {
    zhc_hpu_config hpu_config;
    uint8_t n_hpus;
} zhc_multi_hpu_config;

typedef struct zhc_vm_config {
    size_t lwe_dim;
    size_t bsk_polynomial_size;
    size_t bsk_glwe_dim;
    size_t bsk_dec_levels;
    size_t bsk_dec_base_log;
    size_t ksk_dec_levels;
    size_t ksk_dec_base_log;
    size_t delta;
    size_t carry_size;
    size_t message_size;
    size_t regf_size;
} zhc_vm_config;

zhc_status zhc_hpu_config_default(zhc_hpu_config *out);
zhc_status zhc_multi_hpu_config_default(zhc_multi_hpu_config *out);


/* ------------------------------------------------------------------------------------------- */
/* Opaque handles                                                                              */
/* ------------------------------------------------------------------------------------------- */

typedef struct zhc_builder zhc_builder;
typedef struct zhc_ciphertext_block zhc_ciphertext_block;
typedef struct zhc_plaintext_block zhc_plaintext_block;
typedef struct zhc_bool_ciphertext zhc_bool_ciphertext;
typedef struct zhc_integer_ciphertext zhc_integer_ciphertext;
typedef struct zhc_integer_plaintext zhc_integer_plaintext;
typedef struct zhc_lut1 zhc_lut1;
typedef struct zhc_lut2 zhc_lut2;
typedef struct zhc_lut4 zhc_lut4;
typedef struct zhc_lut8 zhc_lut8;
typedef struct zhc_file_handle zhc_file_handle;
typedef struct zhc_perfetto_trace zhc_perfetto_trace;
typedef struct zhc_pipeline zhc_pipeline;
typedef struct zhc_hpu_metrics zhc_hpu_metrics;
typedef struct zhc_multi_hpu_metrics zhc_multi_hpu_metrics;
typedef struct zhc_pbs_metrics zhc_pbs_metrics;

void zhc_ciphertext_block_free(zhc_ciphertext_block *ptr);
void zhc_plaintext_block_free(zhc_plaintext_block *ptr);
void zhc_bool_ciphertext_free(zhc_bool_ciphertext *ptr);
void zhc_integer_ciphertext_free(zhc_integer_ciphertext *ptr);
void zhc_integer_plaintext_free(zhc_integer_plaintext *ptr);
void zhc_lut1_free(zhc_lut1 *ptr);
void zhc_lut2_free(zhc_lut2 *ptr);
void zhc_lut4_free(zhc_lut4 *ptr);
void zhc_lut8_free(zhc_lut8 *ptr);
void zhc_file_handle_free(zhc_file_handle *ptr);
void zhc_perfetto_trace_free(zhc_perfetto_trace *ptr);
void zhc_hpu_metrics_free(zhc_hpu_metrics *ptr);
void zhc_multi_hpu_metrics_free(zhc_multi_hpu_metrics *ptr);
void zhc_pbs_metrics_free(zhc_pbs_metrics *ptr);

/* Strings written by `*_debug`, `*_dump` and `zhc_builder_dump_noise` are owned by the caller. */
void zhc_string_free(char *ptr);

/* ------------------------------------------------------------------------------------------- */
/* Debug and dump                                                                              */
/* ------------------------------------------------------------------------------------------- */
/* `*_debug` binds Rust's `{:?}`. `*_dump` binds `Dumpable::dump_to_string`, for the types      */
/* that implement it.                                                                          */

zhc_status zhc_builder_debug(const zhc_builder *ptr, char **out);
zhc_status zhc_builder_dump(const zhc_builder *ptr, char **out);
zhc_status zhc_ciphertext_block_debug(const zhc_ciphertext_block *ptr, char **out);
zhc_status zhc_plaintext_block_debug(const zhc_plaintext_block *ptr, char **out);
zhc_status zhc_bool_ciphertext_debug(const zhc_bool_ciphertext *ptr, char **out);
zhc_status zhc_integer_ciphertext_debug(const zhc_integer_ciphertext *ptr, char **out);
zhc_status zhc_integer_plaintext_debug(const zhc_integer_plaintext *ptr, char **out);
zhc_status zhc_lut1_debug(const zhc_lut1 *ptr, char **out);
zhc_status zhc_lut1_dump(const zhc_lut1 *ptr, char **out);
zhc_status zhc_lut2_debug(const zhc_lut2 *ptr, char **out);
zhc_status zhc_lut2_dump(const zhc_lut2 *ptr, char **out);
zhc_status zhc_lut4_debug(const zhc_lut4 *ptr, char **out);
zhc_status zhc_lut4_dump(const zhc_lut4 *ptr, char **out);
zhc_status zhc_lut8_debug(const zhc_lut8 *ptr, char **out);
zhc_status zhc_lut8_dump(const zhc_lut8 *ptr, char **out);
zhc_status zhc_file_handle_debug(const zhc_file_handle *ptr, char **out);
zhc_status zhc_perfetto_trace_debug(const zhc_perfetto_trace *ptr, char **out);
zhc_status zhc_pipeline_debug(const zhc_pipeline *ptr, char **out);
zhc_status zhc_hpu_metrics_debug(const zhc_hpu_metrics *ptr, char **out);
zhc_status zhc_hpu_metrics_dump(const zhc_hpu_metrics *ptr, char **out);
zhc_status zhc_multi_hpu_metrics_debug(const zhc_multi_hpu_metrics *ptr, char **out);
zhc_status zhc_multi_hpu_metrics_dump(const zhc_multi_hpu_metrics *ptr, char **out);
zhc_status zhc_pbs_metrics_debug(const zhc_pbs_metrics *ptr, char **out);
zhc_status zhc_pbs_metrics_dump(const zhc_pbs_metrics *ptr, char **out);

/* ------------------------------------------------------------------------------------------- */
/* Metrics fields                                                                              */
/* ------------------------------------------------------------------------------------------- */
/* Timing values are microseconds. The histograms are only visible through `*_dump`.           */

zhc_status zhc_hpu_metrics_latency(const zhc_hpu_metrics *m, double *out);
zhc_status zhc_hpu_metrics_lower_bound(const zhc_hpu_metrics *m, double *out);
zhc_status zhc_hpu_metrics_batching_overhead(const zhc_hpu_metrics *m, double *out);
zhc_status zhc_hpu_metrics_starvation(const zhc_hpu_metrics *m, double *out);
zhc_status zhc_hpu_metrics_batch_count(const zhc_hpu_metrics *m, size_t *out);
zhc_status zhc_hpu_metrics_slots_filled(const zhc_hpu_metrics *m, size_t *out);
zhc_status zhc_hpu_metrics_slots_total(const zhc_hpu_metrics *m, size_t *out);
zhc_status zhc_hpu_metrics_timeout_launches(const zhc_hpu_metrics *m, uint16_t *out);
zhc_status zhc_multi_hpu_metrics_latency(const zhc_multi_hpu_metrics *m, double *out);
zhc_status zhc_pbs_metrics_count(const zhc_pbs_metrics *m, size_t *out);
zhc_status zhc_pbs_metrics_critical_length(const zhc_pbs_metrics *m, size_t *out);

/* ------------------------------------------------------------------------------------------- */
/* File handles                                                                                */
/* ------------------------------------------------------------------------------------------- */

zhc_status zhc_file_handle_open(const zhc_file_handle *handle);
zhc_status zhc_file_handle_move_to(zhc_file_handle *handle, const char *path);

/* ------------------------------------------------------------------------------------------- */
/* Perfetto traces                                                                             */
/* ------------------------------------------------------------------------------------------- */

zhc_status zhc_perfetto_trace_open(const zhc_perfetto_trace *trace);

/* ------------------------------------------------------------------------------------------- */
/* Lookup tables                                                                               */
/* ------------------------------------------------------------------------------------------- */
/* `table_k[i]` is the output bits (padding | carry | message) for input data value `i`.        */
/* An entry may set the padding bit, as the builtin padding tables do; it is then read back by  */
/* zhc_builder_block_padding_lookup or _wrapping_lookup. Entries must fit the complete width.   */
/* `len` is the number of entries of each table: 2^(carry + message - k) for a 2^k-output LUT. */

zhc_status zhc_lut1_new(const char *name, zhc_ciphertext_block_spec spec, const uint16_t *table_1,
                        size_t len, zhc_lut1 **out);
zhc_status zhc_lut2_new(const char *name, zhc_ciphertext_block_spec spec, const uint16_t *table_1,
                        const uint16_t *table_2, size_t len, zhc_lut2 **out);
zhc_status zhc_lut4_new(const char *name, zhc_ciphertext_block_spec spec, const uint16_t *table_1,
                        const uint16_t *table_2, const uint16_t *table_3, const uint16_t *table_4,
                        size_t len, zhc_lut4 **out);
zhc_status zhc_lut8_new(const char *name, zhc_ciphertext_block_spec spec, const uint16_t *table_1,
                        const uint16_t *table_2, const uint16_t *table_3, const uint16_t *table_4,
                        const uint16_t *table_5, const uint16_t *table_6, const uint16_t *table_7,
                        const uint16_t *table_8, size_t len, zhc_lut8 **out);

/* ------------------------------------------------------------------------------------------- */
/* Builder: construction                                                                       */
/* ------------------------------------------------------------------------------------------- */

zhc_status zhc_builder_new(zhc_ciphertext_block_spec spec, zhc_builder **out);
void zhc_builder_free(zhc_builder *builder);
zhc_status zhc_builder_spec(const zhc_builder *builder, zhc_ciphertext_block_spec *out);

/* ------------------------------------------------------------------------------------------- */
/* Builder: diagnostics                                                                        */
/* ------------------------------------------------------------------------------------------- */

zhc_status zhc_builder_draw(const zhc_builder *builder, zhc_ir_kind kind, zhc_file_handle **out);
zhc_status zhc_builder_dump_noise(const zhc_builder *builder, char **out);
zhc_status zhc_builder_check_noise(const zhc_builder *builder);

/* ------------------------------------------------------------------------------------------- */
/* Builder: comments                                                                           */
/* ------------------------------------------------------------------------------------------- */

zhc_status zhc_builder_push_comment(const zhc_builder *builder, const char *comment);
zhc_status zhc_builder_pop_comment(const zhc_builder *builder);

/* ------------------------------------------------------------------------------------------- */
/* Builder: inputs, outputs, declarations                                                      */
/* ------------------------------------------------------------------------------------------- */

zhc_status zhc_builder_integer_ciphertext_input(const zhc_builder *builder, uint16_t int_size,
                                                zhc_integer_ciphertext **out);
zhc_status zhc_builder_integer_ciphertext_declare(const zhc_builder *builder, uint16_t int_size,
                                                  zhc_integer_ciphertext **out);
zhc_status zhc_builder_integer_ciphertext_store_block(const zhc_builder *builder,
                                                      const zhc_integer_ciphertext *ct,
                                                      uint8_t index,
                                                      const zhc_ciphertext_block *block,
                                                      zhc_integer_ciphertext **out);
zhc_status zhc_builder_integer_ciphertext_get_block(const zhc_builder *builder,
                                                    const zhc_integer_ciphertext *ct,
                                                    uint8_t index, zhc_ciphertext_block **out);
zhc_status zhc_builder_integer_ciphertext_inspect(const zhc_builder *builder,
                                                  const zhc_integer_ciphertext *src,
                                                  zhc_integer_ciphertext **out);
zhc_status zhc_builder_integer_ciphertext_output(const zhc_builder *builder,
                                                 const zhc_integer_ciphertext *ct);

zhc_status zhc_builder_integer_plaintext_input(const zhc_builder *builder, uint16_t int_size,
                                               zhc_integer_plaintext **out);
zhc_status zhc_builder_integer_plaintext_get_block(const zhc_builder *builder,
                                                   const zhc_integer_plaintext *pt,
                                                   uint8_t index, zhc_plaintext_block **out);

zhc_status zhc_builder_bool_ciphertext_input(const zhc_builder *builder,
                                             zhc_bool_ciphertext **out);
zhc_status zhc_builder_bool_ciphertext_from_block(const zhc_builder *builder,
                                                  const zhc_ciphertext_block *block,
                                                  zhc_bool_ciphertext **out);
zhc_status zhc_builder_bool_ciphertext_get_block(const zhc_builder *builder,
                                                 const zhc_bool_ciphertext *value,
                                                 zhc_ciphertext_block **out);
zhc_status zhc_builder_bool_ciphertext_output(const zhc_builder *builder,
                                              const zhc_bool_ciphertext *value);

/* ------------------------------------------------------------------------------------------- */
/* Builder: block constants and inspection                                                     */
/* ------------------------------------------------------------------------------------------- */

zhc_status zhc_builder_block_let_plaintext(const zhc_builder *builder, uint8_t value,
                                           zhc_plaintext_block **out);
zhc_status zhc_builder_block_let_ciphertext(const zhc_builder *builder, uint8_t value,
                                            zhc_ciphertext_block **out);
zhc_status zhc_builder_block_inspect(const zhc_builder *builder, const zhc_ciphertext_block *src,
                                     zhc_ciphertext_block **out);

/* ------------------------------------------------------------------------------------------- */
/* Builder: block add                                                                          */
/* ------------------------------------------------------------------------------------------- */

zhc_status zhc_builder_block_add_with(const zhc_builder *builder,
                                      const zhc_ciphertext_block *src_a,
                                      const zhc_ciphertext_block *src_b, zhc_flavor flavor,
                                      zhc_ciphertext_block **out);
zhc_status zhc_builder_block_add(const zhc_builder *builder, const zhc_ciphertext_block *src_a,
                                 const zhc_ciphertext_block *src_b, zhc_ciphertext_block **out);
zhc_status zhc_builder_block_temper_add(const zhc_builder *builder,
                                        const zhc_ciphertext_block *src_a,
                                        const zhc_ciphertext_block *src_b,
                                        zhc_ciphertext_block **out);
zhc_status zhc_builder_block_wrapping_add(const zhc_builder *builder,
                                          const zhc_ciphertext_block *src_a,
                                          const zhc_ciphertext_block *src_b,
                                          zhc_ciphertext_block **out);

/* ------------------------------------------------------------------------------------------- */
/* Builder: block sub                                                                          */
/* ------------------------------------------------------------------------------------------- */

zhc_status zhc_builder_block_sub_with(const zhc_builder *builder,
                                      const zhc_ciphertext_block *src_a,
                                      const zhc_ciphertext_block *src_b, zhc_flavor flavor,
                                      zhc_ciphertext_block **out);
zhc_status zhc_builder_block_sub(const zhc_builder *builder, const zhc_ciphertext_block *src_a,
                                 const zhc_ciphertext_block *src_b, zhc_ciphertext_block **out);
zhc_status zhc_builder_block_temper_sub(const zhc_builder *builder,
                                        const zhc_ciphertext_block *src_a,
                                        const zhc_ciphertext_block *src_b,
                                        zhc_ciphertext_block **out);
zhc_status zhc_builder_block_wrapping_sub(const zhc_builder *builder,
                                          const zhc_ciphertext_block *src_a,
                                          const zhc_ciphertext_block *src_b,
                                          zhc_ciphertext_block **out);

/* ------------------------------------------------------------------------------------------- */
/* Builder: block shl                                                                          */
/* ------------------------------------------------------------------------------------------- */

zhc_status zhc_builder_block_shl_with(const zhc_builder *builder, const zhc_ciphertext_block *src,
                                      uint8_t amount, zhc_flavor flavor,
                                      zhc_ciphertext_block **out);
zhc_status zhc_builder_block_shl(const zhc_builder *builder, const zhc_ciphertext_block *src,
                                 uint8_t amount, zhc_ciphertext_block **out);
zhc_status zhc_builder_block_temper_shl(const zhc_builder *builder,
                                        const zhc_ciphertext_block *src, uint8_t amount,
                                        zhc_ciphertext_block **out);
zhc_status zhc_builder_block_wrapping_shl(const zhc_builder *builder,
                                          const zhc_ciphertext_block *src, uint8_t amount,
                                          zhc_ciphertext_block **out);

/* ------------------------------------------------------------------------------------------- */
/* Builder: block add plaintext                                                                */
/* ------------------------------------------------------------------------------------------- */

zhc_status zhc_builder_block_add_plaintext_with(const zhc_builder *builder,
                                                const zhc_ciphertext_block *src_a,
                                                const zhc_plaintext_block *src_b,
                                                zhc_flavor flavor, zhc_ciphertext_block **out);
zhc_status zhc_builder_block_add_plaintext(const zhc_builder *builder,
                                           const zhc_ciphertext_block *src_a,
                                           const zhc_plaintext_block *src_b,
                                           zhc_ciphertext_block **out);
zhc_status zhc_builder_block_temper_add_plaintext(const zhc_builder *builder,
                                                  const zhc_ciphertext_block *src_a,
                                                  const zhc_plaintext_block *src_b,
                                                  zhc_ciphertext_block **out);
zhc_status zhc_builder_block_wrapping_add_plaintext(const zhc_builder *builder,
                                                    const zhc_ciphertext_block *src_a,
                                                    const zhc_plaintext_block *src_b,
                                                    zhc_ciphertext_block **out);

/* ------------------------------------------------------------------------------------------- */
/* Builder: block sub plaintext                                                                */
/* ------------------------------------------------------------------------------------------- */

zhc_status zhc_builder_block_sub_plaintext_with(const zhc_builder *builder,
                                                const zhc_ciphertext_block *src_a,
                                                const zhc_plaintext_block *src_b,
                                                zhc_flavor flavor, zhc_ciphertext_block **out);
zhc_status zhc_builder_block_sub_plaintext(const zhc_builder *builder,
                                           const zhc_ciphertext_block *src_a,
                                           const zhc_plaintext_block *src_b,
                                           zhc_ciphertext_block **out);
zhc_status zhc_builder_block_temper_sub_plaintext(const zhc_builder *builder,
                                                  const zhc_ciphertext_block *src_a,
                                                  const zhc_plaintext_block *src_b,
                                                  zhc_ciphertext_block **out);
zhc_status zhc_builder_block_wrapping_sub_plaintext(const zhc_builder *builder,
                                                    const zhc_ciphertext_block *src_a,
                                                    const zhc_plaintext_block *src_b,
                                                    zhc_ciphertext_block **out);

/* ------------------------------------------------------------------------------------------- */
/* Builder: block plaintext sub                                                                */
/* ------------------------------------------------------------------------------------------- */

zhc_status zhc_builder_block_plaintext_sub_with(const zhc_builder *builder,
                                                const zhc_plaintext_block *src_a,
                                                const zhc_ciphertext_block *src_b,
                                                zhc_flavor flavor, zhc_ciphertext_block **out);
zhc_status zhc_builder_block_plaintext_sub(const zhc_builder *builder,
                                           const zhc_plaintext_block *src_a,
                                           const zhc_ciphertext_block *src_b,
                                           zhc_ciphertext_block **out);
zhc_status zhc_builder_block_temper_plaintext_sub(const zhc_builder *builder,
                                                  const zhc_plaintext_block *src_a,
                                                  const zhc_ciphertext_block *src_b,
                                                  zhc_ciphertext_block **out);
zhc_status zhc_builder_block_wrapping_plaintext_sub(const zhc_builder *builder,
                                                    const zhc_plaintext_block *src_a,
                                                    const zhc_ciphertext_block *src_b,
                                                    zhc_ciphertext_block **out);

/* ------------------------------------------------------------------------------------------- */
/* Builder: block mul plaintext                                                                */
/* ------------------------------------------------------------------------------------------- */

zhc_status zhc_builder_block_mul_plaintext_with(const zhc_builder *builder,
                                                const zhc_ciphertext_block *src_a,
                                                const zhc_plaintext_block *src_b,
                                                zhc_flavor flavor, zhc_ciphertext_block **out);
zhc_status zhc_builder_block_mul_plaintext(const zhc_builder *builder,
                                           const zhc_ciphertext_block *src_a,
                                           const zhc_plaintext_block *src_b,
                                           zhc_ciphertext_block **out);
zhc_status zhc_builder_block_temper_mul_plaintext(const zhc_builder *builder,
                                                  const zhc_ciphertext_block *src_a,
                                                  const zhc_plaintext_block *src_b,
                                                  zhc_ciphertext_block **out);
zhc_status zhc_builder_block_wrapping_mul_plaintext(const zhc_builder *builder,
                                                    const zhc_ciphertext_block *src_a,
                                                    const zhc_plaintext_block *src_b,
                                                    zhc_ciphertext_block **out);

/* ------------------------------------------------------------------------------------------- */
/* Builder: block mac                                                                          */
/* ------------------------------------------------------------------------------------------- */

zhc_status zhc_builder_block_mac_with(const zhc_builder *builder,
                                      const zhc_ciphertext_block *src_a,
                                      const zhc_ciphertext_block *src_b, uint8_t mul,
                                      zhc_flavor flavor, zhc_ciphertext_block **out);
zhc_status zhc_builder_block_mac(const zhc_builder *builder, const zhc_ciphertext_block *src_a,
                                 const zhc_ciphertext_block *src_b, uint8_t mul,
                                 zhc_ciphertext_block **out);
zhc_status zhc_builder_block_temper_mac(const zhc_builder *builder,
                                        const zhc_ciphertext_block *src_a,
                                        const zhc_ciphertext_block *src_b, uint8_t mul,
                                        zhc_ciphertext_block **out);
zhc_status zhc_builder_block_wrapping_mac(const zhc_builder *builder,
                                          const zhc_ciphertext_block *src_a,
                                          const zhc_ciphertext_block *src_b, uint8_t mul,
                                          zhc_ciphertext_block **out);

/* ------------------------------------------------------------------------------------------- */
/* Builder: block pack                                                                         */
/* ------------------------------------------------------------------------------------------- */

zhc_status zhc_builder_block_pack_with(const zhc_builder *builder,
                                       const zhc_ciphertext_block *src_a,
                                       const zhc_ciphertext_block *src_b, zhc_flavor flavor,
                                       zhc_ciphertext_block **out);
zhc_status zhc_builder_block_pack(const zhc_builder *builder, const zhc_ciphertext_block *src_a,
                                  const zhc_ciphertext_block *src_b, zhc_ciphertext_block **out);
zhc_status zhc_builder_block_temper_pack(const zhc_builder *builder,
                                         const zhc_ciphertext_block *src_a,
                                         const zhc_ciphertext_block *src_b,
                                         zhc_ciphertext_block **out);
zhc_status zhc_builder_block_wrapping_pack(const zhc_builder *builder,
                                           const zhc_ciphertext_block *src_a,
                                           const zhc_ciphertext_block *src_b,
                                           zhc_ciphertext_block **out);

/* ------------------------------------------------------------------------------------------- */
/* Builder: block lookups                                                                      */
/* ------------------------------------------------------------------------------------------- */

zhc_status zhc_builder_block_lookup_with(const zhc_builder *builder,
                                         const zhc_ciphertext_block *src, const zhc_lut1 *lut,
                                         zhc_lookup_check check, zhc_ciphertext_block **out);
zhc_status zhc_builder_block_lookup(const zhc_builder *builder, const zhc_ciphertext_block *src,
                                    const zhc_lut1 *lut, zhc_ciphertext_block **out);
zhc_status zhc_builder_block_padding_lookup(const zhc_builder *builder,
                                            const zhc_ciphertext_block *src, const zhc_lut1 *lut,
                                            zhc_ciphertext_block **out);
zhc_status zhc_builder_block_wrapping_lookup(const zhc_builder *builder,
                                             const zhc_ciphertext_block *src,
                                             const zhc_lut1 *lut, zhc_ciphertext_block **out);
zhc_status zhc_builder_block_pack_then_lookup(const zhc_builder *builder,
                                              const zhc_ciphertext_block *src_a,
                                              const zhc_ciphertext_block *src_b,
                                              const zhc_lut1 *lut, zhc_ciphertext_block **out);
zhc_status zhc_builder_block_mac_then_lookup(const zhc_builder *builder,
                                             const zhc_ciphertext_block *src_a,
                                             const zhc_ciphertext_block *src_b, uint8_t mul,
                                             const zhc_lut1 *lut, zhc_ciphertext_block **out);

/* `out` must point to an array of 2 pointers. */
zhc_status zhc_builder_block_lookup2_with(const zhc_builder *builder,
                                          const zhc_ciphertext_block *src, const zhc_lut2 *lut,
                                          zhc_lookup_check check, zhc_ciphertext_block **out);
zhc_status zhc_builder_block_lookup2(const zhc_builder *builder, const zhc_ciphertext_block *src,
                                     const zhc_lut2 *lut, zhc_ciphertext_block **out);

/* `out` must point to an array of 4 pointers. */
zhc_status zhc_builder_block_lookup4_with(const zhc_builder *builder,
                                          const zhc_ciphertext_block *src, const zhc_lut4 *lut,
                                          zhc_lookup_check check, zhc_ciphertext_block **out);
zhc_status zhc_builder_block_lookup4(const zhc_builder *builder, const zhc_ciphertext_block *src,
                                     const zhc_lut4 *lut, zhc_ciphertext_block **out);

/* `out` must point to an array of 8 pointers. */
zhc_status zhc_builder_block_lookup8_with(const zhc_builder *builder,
                                          const zhc_ciphertext_block *src, const zhc_lut8 *lut,
                                          zhc_lookup_check check, zhc_ciphertext_block **out);
zhc_status zhc_builder_block_lookup8(const zhc_builder *builder, const zhc_ciphertext_block *src,
                                     const zhc_lut8 *lut, zhc_ciphertext_block **out);

/* ------------------------------------------------------------------------------------------- */
/* Pipeline: construction                                                                      */
/* ------------------------------------------------------------------------------------------- */

zhc_status zhc_pipeline_new(zhc_pipeline **out);
void zhc_pipeline_free(zhc_pipeline *pipeline);

/* ------------------------------------------------------------------------------------------- */
/* Pipeline: inputs                                                                            */
/* ------------------------------------------------------------------------------------------- */

zhc_status zhc_pipeline_set_builder(zhc_pipeline *pipeline, const zhc_builder *builder);
zhc_status zhc_pipeline_set_ciphertext_block_spec(zhc_pipeline *pipeline,
                                                  zhc_ciphertext_block_spec spec);
zhc_status zhc_pipeline_set_hpu_config(zhc_pipeline *pipeline, zhc_hpu_config config);
zhc_status zhc_pipeline_set_multi_hpu_config(zhc_pipeline *pipeline,
                                             zhc_multi_hpu_config config);
zhc_status zhc_pipeline_set_vm_config(zhc_pipeline *pipeline, zhc_vm_config config);
zhc_status zhc_pipeline_set_legacy_hpu_scheduler(zhc_pipeline *pipeline);
zhc_status zhc_pipeline_set_trace_hpu_events(zhc_pipeline *pipeline);

/* ------------------------------------------------------------------------------------------- */
/* Pipeline: artifacts                                                                         */
/* ------------------------------------------------------------------------------------------- */
/* Requesting an artifact compiles what it depends on. Files and traces are copies of the      */
/* pipeline's handles; the pipeline keeps its own.                                             */

zhc_status zhc_pipeline_get_ciphertext_block_spec(zhc_pipeline *pipeline,
                                                  zhc_ciphertext_block_spec *out);
zhc_status zhc_pipeline_get_hpu_config(zhc_pipeline *pipeline, zhc_hpu_config *out);
zhc_status zhc_pipeline_get_multi_hpu_config(zhc_pipeline *pipeline, zhc_multi_hpu_config *out);
zhc_status zhc_pipeline_get_vm_config(zhc_pipeline *pipeline, zhc_vm_config *out);

zhc_status zhc_pipeline_get_fingerprint(zhc_pipeline *pipeline, uint64_t *out);
zhc_status zhc_pipeline_get_hpu_metrics(zhc_pipeline *pipeline, zhc_hpu_metrics **out);
zhc_status zhc_pipeline_get_multi_hpu_metrics(zhc_pipeline *pipeline,
                                              zhc_multi_hpu_metrics **out);
zhc_status zhc_pipeline_get_pbs_metrics(zhc_pipeline *pipeline, zhc_pbs_metrics **out);

zhc_status zhc_pipeline_draw_state(zhc_pipeline *pipeline, zhc_file_handle **out);
zhc_status zhc_pipeline_get_slack_drawing(zhc_pipeline *pipeline, zhc_file_handle **out);
zhc_status zhc_pipeline_get_hpu_assembly(zhc_pipeline *pipeline, zhc_file_handle **out);
zhc_status zhc_pipeline_get_hpu_trace(zhc_pipeline *pipeline, zhc_perfetto_trace **out);
zhc_status zhc_pipeline_get_multi_hpu_trace(zhc_pipeline *pipeline, zhc_perfetto_trace **out);

#ifdef __cplusplus
}
#endif

#endif /* ZHC_H */
