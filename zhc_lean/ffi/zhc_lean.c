/* Lean 4 shim over the ZHC C API (`zhc_c_api/include/zhc.h`).
 *
 * Every `zhc_lean_*` function below is the target of an `@[extern]` declaration in
 * the `Zhc/Ffi/` modules. It unpacks Lean arguments, calls the `zhc_*` function of the same name,
 * and packs the result. A non-`ZHC_OK` status becomes an `IO.userError`.
 *
 * Handles are Lean external objects. Their finalizer calls the matching `zhc_*_free`, so the
 * Lean garbage collector owns their lifetime.
 *
 * Handle arguments are declared `@&` (borrowed) on the Lean side, so they are never
 * consumed here. Plain-struct arguments are borrowed too.
 */

#include <lean/lean.h>
#include <stdio.h>
#include <string.h>

#include "zhc.h"

/* ------------------------------------------------------------------------------------------- */
/* Results                                                                                     */
/* ------------------------------------------------------------------------------------------- */

static const char *status_name(zhc_status s) {
    switch (s) {
    case ZHC_OK: return "ok";
    case ZHC_ERR_PANIC: return "panic (message on stderr)";
    case ZHC_ERR_NULL_POINTER: return "null pointer";
    case ZHC_ERR_INVALID_ARGUMENT: return "invalid argument";
    case ZHC_ERR_IO: return "io error";
    }
    return "unknown status";
}

static lean_obj_res fail(const char *fn, zhc_status s) {
    char buf[256];
    snprintf(buf, sizeof buf, "%s: %s", fn, status_name(s));
    return lean_io_result_mk_error(lean_mk_io_user_error(lean_mk_string(buf)));
}

static lean_obj_res ok_unit(void) {
    return lean_io_result_mk_ok(lean_box(0));
}

/* ------------------------------------------------------------------------------------------- */
/* External classes                                                                            */
/* ------------------------------------------------------------------------------------------- */

static void noop_foreach(void *data, b_lean_obj_arg f) {
    (void)data;
    (void)f;
}

static void finalize_builder(void *p) { zhc_builder_free((zhc_builder *)p); }
static void finalize_ciphertext_block(void *p) { zhc_ciphertext_block_free((zhc_ciphertext_block *)p); }
static void finalize_plaintext_block(void *p) { zhc_plaintext_block_free((zhc_plaintext_block *)p); }
static void finalize_bool_ciphertext(void *p) { zhc_bool_ciphertext_free((zhc_bool_ciphertext *)p); }
static void finalize_integer_ciphertext(void *p) { zhc_integer_ciphertext_free((zhc_integer_ciphertext *)p); }
static void finalize_integer_plaintext(void *p) { zhc_integer_plaintext_free((zhc_integer_plaintext *)p); }
static void finalize_lut1(void *p) { zhc_lut1_free((zhc_lut1 *)p); }
static void finalize_lut2(void *p) { zhc_lut2_free((zhc_lut2 *)p); }
static void finalize_lut4(void *p) { zhc_lut4_free((zhc_lut4 *)p); }
static void finalize_lut8(void *p) { zhc_lut8_free((zhc_lut8 *)p); }
static void finalize_file_handle(void *p) { zhc_file_handle_free((zhc_file_handle *)p); }
static void finalize_perfetto_trace(void *p) { zhc_perfetto_trace_free((zhc_perfetto_trace *)p); }
static void finalize_pipeline(void *p) { zhc_pipeline_free((zhc_pipeline *)p); }
static void finalize_hpu_metrics(void *p) { zhc_hpu_metrics_free((zhc_hpu_metrics *)p); }
static void finalize_multi_hpu_metrics(void *p) { zhc_multi_hpu_metrics_free((zhc_multi_hpu_metrics *)p); }
static void finalize_pbs_metrics(void *p) { zhc_pbs_metrics_free((zhc_pbs_metrics *)p); }

static lean_external_class *g_builder_class;
static lean_external_class *g_ciphertext_block_class;
static lean_external_class *g_plaintext_block_class;
static lean_external_class *g_bool_ciphertext_class;
static lean_external_class *g_integer_ciphertext_class;
static lean_external_class *g_integer_plaintext_class;
static lean_external_class *g_lut1_class;
static lean_external_class *g_lut2_class;
static lean_external_class *g_lut4_class;
static lean_external_class *g_lut8_class;
static lean_external_class *g_file_handle_class;
static lean_external_class *g_perfetto_trace_class;
static lean_external_class *g_pipeline_class;
static lean_external_class *g_hpu_metrics_class;
static lean_external_class *g_multi_hpu_metrics_class;
static lean_external_class *g_pbs_metrics_class;

static lean_external_class *class_of(lean_external_class **slot, lean_external_finalize_proc fin) {
    if (*slot == NULL) *slot = lean_register_external_class(fin, noop_foreach);
    return *slot;
}

static lean_obj_res box_builder(zhc_builder *p) { return lean_alloc_external(class_of(&g_builder_class, finalize_builder), p); }
static lean_obj_res box_ciphertext_block(zhc_ciphertext_block *p) { return lean_alloc_external(class_of(&g_ciphertext_block_class, finalize_ciphertext_block), p); }
static lean_obj_res box_plaintext_block(zhc_plaintext_block *p) { return lean_alloc_external(class_of(&g_plaintext_block_class, finalize_plaintext_block), p); }
static lean_obj_res box_bool_ciphertext(zhc_bool_ciphertext *p) { return lean_alloc_external(class_of(&g_bool_ciphertext_class, finalize_bool_ciphertext), p); }
static lean_obj_res box_integer_ciphertext(zhc_integer_ciphertext *p) { return lean_alloc_external(class_of(&g_integer_ciphertext_class, finalize_integer_ciphertext), p); }
static lean_obj_res box_integer_plaintext(zhc_integer_plaintext *p) { return lean_alloc_external(class_of(&g_integer_plaintext_class, finalize_integer_plaintext), p); }
static lean_obj_res box_lut1(zhc_lut1 *p) { return lean_alloc_external(class_of(&g_lut1_class, finalize_lut1), p); }
static lean_obj_res box_lut2(zhc_lut2 *p) { return lean_alloc_external(class_of(&g_lut2_class, finalize_lut2), p); }
static lean_obj_res box_lut4(zhc_lut4 *p) { return lean_alloc_external(class_of(&g_lut4_class, finalize_lut4), p); }
static lean_obj_res box_lut8(zhc_lut8 *p) { return lean_alloc_external(class_of(&g_lut8_class, finalize_lut8), p); }
static lean_obj_res box_file_handle(zhc_file_handle *p) { return lean_alloc_external(class_of(&g_file_handle_class, finalize_file_handle), p); }
static lean_obj_res box_perfetto_trace(zhc_perfetto_trace *p) { return lean_alloc_external(class_of(&g_perfetto_trace_class, finalize_perfetto_trace), p); }
static lean_obj_res box_pipeline(zhc_pipeline *p) { return lean_alloc_external(class_of(&g_pipeline_class, finalize_pipeline), p); }
static lean_obj_res box_hpu_metrics(zhc_hpu_metrics *p) { return lean_alloc_external(class_of(&g_hpu_metrics_class, finalize_hpu_metrics), p); }
static lean_obj_res box_multi_hpu_metrics(zhc_multi_hpu_metrics *p) { return lean_alloc_external(class_of(&g_multi_hpu_metrics_class, finalize_multi_hpu_metrics), p); }
static lean_obj_res box_pbs_metrics(zhc_pbs_metrics *p) { return lean_alloc_external(class_of(&g_pbs_metrics_class, finalize_pbs_metrics), p); }

static zhc_builder *builder_of(b_lean_obj_arg o) { return (zhc_builder *)lean_get_external_data(o); }
static zhc_ciphertext_block *ct_of(b_lean_obj_arg o) { return (zhc_ciphertext_block *)lean_get_external_data(o); }
static zhc_plaintext_block *pt_of(b_lean_obj_arg o) { return (zhc_plaintext_block *)lean_get_external_data(o); }
static zhc_bool_ciphertext *bool_of(b_lean_obj_arg o) { return (zhc_bool_ciphertext *)lean_get_external_data(o); }
static zhc_integer_ciphertext *int_ct_of(b_lean_obj_arg o) { return (zhc_integer_ciphertext *)lean_get_external_data(o); }
static zhc_integer_plaintext *int_pt_of(b_lean_obj_arg o) { return (zhc_integer_plaintext *)lean_get_external_data(o); }
static zhc_lut1 *lut1_of(b_lean_obj_arg o) { return (zhc_lut1 *)lean_get_external_data(o); }
static zhc_lut2 *lut2_of(b_lean_obj_arg o) { return (zhc_lut2 *)lean_get_external_data(o); }
static zhc_lut4 *lut4_of(b_lean_obj_arg o) { return (zhc_lut4 *)lean_get_external_data(o); }
static zhc_lut8 *lut8_of(b_lean_obj_arg o) { return (zhc_lut8 *)lean_get_external_data(o); }
static zhc_file_handle *file_of(b_lean_obj_arg o) { return (zhc_file_handle *)lean_get_external_data(o); }
static zhc_perfetto_trace *trace_of(b_lean_obj_arg o) { return (zhc_perfetto_trace *)lean_get_external_data(o); }
static zhc_pipeline *pipeline_of(b_lean_obj_arg o) { return (zhc_pipeline *)lean_get_external_data(o); }
static zhc_hpu_metrics *hpu_metrics_of(b_lean_obj_arg o) { return (zhc_hpu_metrics *)lean_get_external_data(o); }
static zhc_multi_hpu_metrics *multi_hpu_metrics_of(b_lean_obj_arg o) { return (zhc_multi_hpu_metrics *)lean_get_external_data(o); }
static zhc_pbs_metrics *pbs_metrics_of(b_lean_obj_arg o) { return (zhc_pbs_metrics *)lean_get_external_data(o); }

/* Result helpers: box a fresh result into an `IO` result, or report the failure.
 *
 * Each helper takes the ADDRESS of the caller's `out` variable, never its value. The call sites
 * read as `ret_x("name", zhc_x(..., &out), &out)`, and C leaves the order in which those two
 * arguments are evaluated unspecified: passing `out` by value would let the compiler read it
 * before the `zhc_x` call that fills it, yielding a null handle or a null string. Taking the
 * address defers the read to the helper body, which runs after every argument is evaluated. */
static lean_obj_res ret_ct(const char *fn, zhc_status s, zhc_ciphertext_block **p) {
    return s == ZHC_OK ? lean_io_result_mk_ok(box_ciphertext_block(*p)) : fail(fn, s);
}
static lean_obj_res ret_pt(const char *fn, zhc_status s, zhc_plaintext_block **p) {
    return s == ZHC_OK ? lean_io_result_mk_ok(box_plaintext_block(*p)) : fail(fn, s);
}
static lean_obj_res ret_bool(const char *fn, zhc_status s, zhc_bool_ciphertext **p) {
    return s == ZHC_OK ? lean_io_result_mk_ok(box_bool_ciphertext(*p)) : fail(fn, s);
}
static lean_obj_res ret_int_ct(const char *fn, zhc_status s, zhc_integer_ciphertext **p) {
    return s == ZHC_OK ? lean_io_result_mk_ok(box_integer_ciphertext(*p)) : fail(fn, s);
}
static lean_obj_res ret_int_pt(const char *fn, zhc_status s, zhc_integer_plaintext **p) {
    return s == ZHC_OK ? lean_io_result_mk_ok(box_integer_plaintext(*p)) : fail(fn, s);
}
static lean_obj_res ret_file(const char *fn, zhc_status s, zhc_file_handle **p) {
    return s == ZHC_OK ? lean_io_result_mk_ok(box_file_handle(*p)) : fail(fn, s);
}
static lean_obj_res ret_trace(const char *fn, zhc_status s, zhc_perfetto_trace **p) {
    return s == ZHC_OK ? lean_io_result_mk_ok(box_perfetto_trace(*p)) : fail(fn, s);
}
static lean_obj_res ret_unit(const char *fn, zhc_status s) {
    return s == ZHC_OK ? ok_unit() : fail(fn, s);
}

/* Copies a string result into a Lean `String` and releases the C string. */
static lean_obj_res ret_string(const char *fn, zhc_status s, char **str) {
    if (s != ZHC_OK) return fail(fn, s);
    lean_object *out = lean_mk_string(*str);
    zhc_string_free(*str);
    return lean_io_result_mk_ok(out);
}

static lean_obj_res ret_f64(const char *fn, zhc_status s, const double *v) {
    return s == ZHC_OK ? lean_io_result_mk_ok(lean_box_float(*v)) : fail(fn, s);
}

static lean_obj_res ret_usize(const char *fn, zhc_status s, const size_t *v) {
    return s == ZHC_OK ? lean_io_result_mk_ok(lean_box_usize(*v)) : fail(fn, s);
}

static lean_obj_res ret_u16(const char *fn, zhc_status s, const uint16_t *v) {
    return s == ZHC_OK ? lean_io_result_mk_ok(lean_box(*v)) : fail(fn, s);
}

/* ------------------------------------------------------------------------------------------- */
/* Debug and dump                                                                              */
/* ------------------------------------------------------------------------------------------- */

LEAN_EXPORT lean_obj_res zhc_lean_builder_debug(b_lean_obj_arg h, lean_obj_arg w) {
    (void)w;
    char *out = NULL;
    return ret_string("zhc_builder_debug", zhc_builder_debug(builder_of(h), &out), &out);
}
LEAN_EXPORT lean_obj_res zhc_lean_builder_dump(b_lean_obj_arg h, lean_obj_arg w) {
    (void)w;
    char *out = NULL;
    return ret_string("zhc_builder_dump", zhc_builder_dump(builder_of(h), &out), &out);
}
LEAN_EXPORT lean_obj_res zhc_lean_ciphertext_block_debug(b_lean_obj_arg h, lean_obj_arg w) {
    (void)w;
    char *out = NULL;
    return ret_string("zhc_ciphertext_block_debug", zhc_ciphertext_block_debug(ct_of(h), &out), &out);
}
LEAN_EXPORT lean_obj_res zhc_lean_plaintext_block_debug(b_lean_obj_arg h, lean_obj_arg w) {
    (void)w;
    char *out = NULL;
    return ret_string("zhc_plaintext_block_debug", zhc_plaintext_block_debug(pt_of(h), &out), &out);
}
LEAN_EXPORT lean_obj_res zhc_lean_bool_ciphertext_debug(b_lean_obj_arg h, lean_obj_arg w) {
    (void)w;
    char *out = NULL;
    return ret_string("zhc_bool_ciphertext_debug", zhc_bool_ciphertext_debug(bool_of(h), &out), &out);
}
LEAN_EXPORT lean_obj_res zhc_lean_integer_ciphertext_debug(b_lean_obj_arg h, lean_obj_arg w) {
    (void)w;
    char *out = NULL;
    return ret_string("zhc_integer_ciphertext_debug", zhc_integer_ciphertext_debug(int_ct_of(h), &out), &out);
}
LEAN_EXPORT lean_obj_res zhc_lean_integer_plaintext_debug(b_lean_obj_arg h, lean_obj_arg w) {
    (void)w;
    char *out = NULL;
    return ret_string("zhc_integer_plaintext_debug", zhc_integer_plaintext_debug(int_pt_of(h), &out), &out);
}
LEAN_EXPORT lean_obj_res zhc_lean_lut1_debug(b_lean_obj_arg h, lean_obj_arg w) {
    (void)w;
    char *out = NULL;
    return ret_string("zhc_lut1_debug", zhc_lut1_debug(lut1_of(h), &out), &out);
}
LEAN_EXPORT lean_obj_res zhc_lean_lut1_dump(b_lean_obj_arg h, lean_obj_arg w) {
    (void)w;
    char *out = NULL;
    return ret_string("zhc_lut1_dump", zhc_lut1_dump(lut1_of(h), &out), &out);
}
LEAN_EXPORT lean_obj_res zhc_lean_lut2_debug(b_lean_obj_arg h, lean_obj_arg w) {
    (void)w;
    char *out = NULL;
    return ret_string("zhc_lut2_debug", zhc_lut2_debug(lut2_of(h), &out), &out);
}
LEAN_EXPORT lean_obj_res zhc_lean_lut2_dump(b_lean_obj_arg h, lean_obj_arg w) {
    (void)w;
    char *out = NULL;
    return ret_string("zhc_lut2_dump", zhc_lut2_dump(lut2_of(h), &out), &out);
}
LEAN_EXPORT lean_obj_res zhc_lean_lut4_debug(b_lean_obj_arg h, lean_obj_arg w) {
    (void)w;
    char *out = NULL;
    return ret_string("zhc_lut4_debug", zhc_lut4_debug(lut4_of(h), &out), &out);
}
LEAN_EXPORT lean_obj_res zhc_lean_lut4_dump(b_lean_obj_arg h, lean_obj_arg w) {
    (void)w;
    char *out = NULL;
    return ret_string("zhc_lut4_dump", zhc_lut4_dump(lut4_of(h), &out), &out);
}
LEAN_EXPORT lean_obj_res zhc_lean_lut8_debug(b_lean_obj_arg h, lean_obj_arg w) {
    (void)w;
    char *out = NULL;
    return ret_string("zhc_lut8_debug", zhc_lut8_debug(lut8_of(h), &out), &out);
}
LEAN_EXPORT lean_obj_res zhc_lean_lut8_dump(b_lean_obj_arg h, lean_obj_arg w) {
    (void)w;
    char *out = NULL;
    return ret_string("zhc_lut8_dump", zhc_lut8_dump(lut8_of(h), &out), &out);
}
LEAN_EXPORT lean_obj_res zhc_lean_file_handle_debug(b_lean_obj_arg h, lean_obj_arg w) {
    (void)w;
    char *out = NULL;
    return ret_string("zhc_file_handle_debug", zhc_file_handle_debug(file_of(h), &out), &out);
}
LEAN_EXPORT lean_obj_res zhc_lean_perfetto_trace_debug(b_lean_obj_arg h, lean_obj_arg w) {
    (void)w;
    char *out = NULL;
    return ret_string("zhc_perfetto_trace_debug", zhc_perfetto_trace_debug(trace_of(h), &out), &out);
}
LEAN_EXPORT lean_obj_res zhc_lean_pipeline_debug(b_lean_obj_arg h, lean_obj_arg w) {
    (void)w;
    char *out = NULL;
    return ret_string("zhc_pipeline_debug", zhc_pipeline_debug(pipeline_of(h), &out), &out);
}
LEAN_EXPORT lean_obj_res zhc_lean_hpu_metrics_debug(b_lean_obj_arg h, lean_obj_arg w) {
    (void)w;
    char *out = NULL;
    return ret_string("zhc_hpu_metrics_debug", zhc_hpu_metrics_debug(hpu_metrics_of(h), &out), &out);
}
LEAN_EXPORT lean_obj_res zhc_lean_hpu_metrics_dump(b_lean_obj_arg h, lean_obj_arg w) {
    (void)w;
    char *out = NULL;
    return ret_string("zhc_hpu_metrics_dump", zhc_hpu_metrics_dump(hpu_metrics_of(h), &out), &out);
}
LEAN_EXPORT lean_obj_res zhc_lean_multi_hpu_metrics_debug(b_lean_obj_arg h, lean_obj_arg w) {
    (void)w;
    char *out = NULL;
    return ret_string("zhc_multi_hpu_metrics_debug", zhc_multi_hpu_metrics_debug(multi_hpu_metrics_of(h), &out), &out);
}
LEAN_EXPORT lean_obj_res zhc_lean_multi_hpu_metrics_dump(b_lean_obj_arg h, lean_obj_arg w) {
    (void)w;
    char *out = NULL;
    return ret_string("zhc_multi_hpu_metrics_dump", zhc_multi_hpu_metrics_dump(multi_hpu_metrics_of(h), &out), &out);
}
LEAN_EXPORT lean_obj_res zhc_lean_pbs_metrics_debug(b_lean_obj_arg h, lean_obj_arg w) {
    (void)w;
    char *out = NULL;
    return ret_string("zhc_pbs_metrics_debug", zhc_pbs_metrics_debug(pbs_metrics_of(h), &out), &out);
}
LEAN_EXPORT lean_obj_res zhc_lean_pbs_metrics_dump(b_lean_obj_arg h, lean_obj_arg w) {
    (void)w;
    char *out = NULL;
    return ret_string("zhc_pbs_metrics_dump", zhc_pbs_metrics_dump(pbs_metrics_of(h), &out), &out);
}

/* ------------------------------------------------------------------------------------------- */
/* Metrics fields                                                                              */
/* ------------------------------------------------------------------------------------------- */

LEAN_EXPORT lean_obj_res zhc_lean_hpu_metrics_latency(b_lean_obj_arg h, lean_obj_arg w) {
    (void)w;
    double v = 0.0;
    return ret_f64("zhc_hpu_metrics_latency", zhc_hpu_metrics_latency(hpu_metrics_of(h), &v), &v);
}
LEAN_EXPORT lean_obj_res zhc_lean_hpu_metrics_lower_bound(b_lean_obj_arg h, lean_obj_arg w) {
    (void)w;
    double v = 0.0;
    return ret_f64("zhc_hpu_metrics_lower_bound", zhc_hpu_metrics_lower_bound(hpu_metrics_of(h), &v), &v);
}
LEAN_EXPORT lean_obj_res zhc_lean_hpu_metrics_batching_overhead(b_lean_obj_arg h, lean_obj_arg w) {
    (void)w;
    double v = 0.0;
    return ret_f64("zhc_hpu_metrics_batching_overhead", zhc_hpu_metrics_batching_overhead(hpu_metrics_of(h), &v), &v);
}
LEAN_EXPORT lean_obj_res zhc_lean_hpu_metrics_starvation(b_lean_obj_arg h, lean_obj_arg w) {
    (void)w;
    double v = 0.0;
    return ret_f64("zhc_hpu_metrics_starvation", zhc_hpu_metrics_starvation(hpu_metrics_of(h), &v), &v);
}
LEAN_EXPORT lean_obj_res zhc_lean_hpu_metrics_batch_count(b_lean_obj_arg h, lean_obj_arg w) {
    (void)w;
    size_t v = 0;
    return ret_usize("zhc_hpu_metrics_batch_count", zhc_hpu_metrics_batch_count(hpu_metrics_of(h), &v), &v);
}
LEAN_EXPORT lean_obj_res zhc_lean_hpu_metrics_slots_filled(b_lean_obj_arg h, lean_obj_arg w) {
    (void)w;
    size_t v = 0;
    return ret_usize("zhc_hpu_metrics_slots_filled", zhc_hpu_metrics_slots_filled(hpu_metrics_of(h), &v), &v);
}
LEAN_EXPORT lean_obj_res zhc_lean_hpu_metrics_slots_total(b_lean_obj_arg h, lean_obj_arg w) {
    (void)w;
    size_t v = 0;
    return ret_usize("zhc_hpu_metrics_slots_total", zhc_hpu_metrics_slots_total(hpu_metrics_of(h), &v), &v);
}
LEAN_EXPORT lean_obj_res zhc_lean_hpu_metrics_timeout_launches(b_lean_obj_arg h, lean_obj_arg w) {
    (void)w;
    uint16_t v = 0;
    return ret_u16("zhc_hpu_metrics_timeout_launches", zhc_hpu_metrics_timeout_launches(hpu_metrics_of(h), &v), &v);
}
LEAN_EXPORT lean_obj_res zhc_lean_multi_hpu_metrics_latency(b_lean_obj_arg h, lean_obj_arg w) {
    (void)w;
    double v = 0.0;
    return ret_f64("zhc_multi_hpu_metrics_latency", zhc_multi_hpu_metrics_latency(multi_hpu_metrics_of(h), &v), &v);
}
LEAN_EXPORT lean_obj_res zhc_lean_pbs_metrics_count(b_lean_obj_arg h, lean_obj_arg w) {
    (void)w;
    size_t v = 0;
    return ret_usize("zhc_pbs_metrics_count", zhc_pbs_metrics_count(pbs_metrics_of(h), &v), &v);
}
LEAN_EXPORT lean_obj_res zhc_lean_pbs_metrics_critical_length(b_lean_obj_arg h, lean_obj_arg w) {
    (void)w;
    size_t v = 0;
    return ret_usize("zhc_pbs_metrics_critical_length", zhc_pbs_metrics_critical_length(pbs_metrics_of(h), &v), &v);
}

/* `n` fresh block handles as a Lean `Array CiphertextBlock`. */
static lean_obj_res ret_cts(const char *fn, zhc_status s, zhc_ciphertext_block **blocks, size_t n) {
    if (s != ZHC_OK) return fail(fn, s);
    lean_object *arr = lean_alloc_array(n, n);
    for (size_t i = 0; i < n; i++) lean_array_set_core(arr, i, box_ciphertext_block(blocks[i]));
    return lean_io_result_mk_ok(arr);
}

/* ------------------------------------------------------------------------------------------- */
/* Plain structures                                                                            */
/* ------------------------------------------------------------------------------------------- */
/* Lean lays out scalar fields after the object fields: `USize` fields first, then the others  */
/* by decreasing size. The Lean structures in `Zhc/Ffi/Types.lean` declare their fields in     */
/* that order, so declaration order is memory order.                                           */

/* `structure BlockSpec where carry message : UInt8` */
static zhc_ciphertext_block_spec spec_of(b_lean_obj_arg o) {
    zhc_ciphertext_block_spec s;
    s.carry = lean_ctor_get_uint8(o, 0);
    s.message = lean_ctor_get_uint8(o, 1);
    return s;
}

/* `structure LookupCheck where allowInputPadding allowIndexBits allowOutputPadding : Bool` */
static zhc_lookup_check check_of(b_lean_obj_arg o) {
    zhc_lookup_check c;
    c.allow_input_padding = lean_ctor_get_uint8(o, 0);
    c.allow_index_bits = lean_ctor_get_uint8(o, 1);
    c.allow_output_padding = lean_ctor_get_uint8(o, 2);
    return c;
}

static lean_obj_res box_spec(zhc_ciphertext_block_spec s) {
    lean_object *o = lean_alloc_ctor(0, 0, 2);
    lean_ctor_set_uint8(o, 0, s.carry);
    lean_ctor_set_uint8(o, 1, s.message);
    return o;
}

/* `structure HpuConfig`: 20 `USize` fields, in the order of `zhc_hpu_config`. */
enum { HPU_CONFIG_FIELDS = 20 };

static zhc_hpu_config hpu_config_of(b_lean_obj_arg o) {
    zhc_hpu_config c;
    c.freq = lean_ctor_get_usize(o, 0);
    c.isc_depth = lean_ctor_get_usize(o, 1);
    c.isc_query_period = lean_ctor_get_usize(o, 2);
    c.mem_fifo_capacity = lean_ctor_get_usize(o, 3);
    c.mem_read_latency = lean_ctor_get_usize(o, 4);
    c.mem_write_latency = lean_ctor_get_usize(o, 5);
    c.alu_fifo_capacity = lean_ctor_get_usize(o, 6);
    c.alu_read_latency = lean_ctor_get_usize(o, 7);
    c.alu_write_latency = lean_ctor_get_usize(o, 8);
    c.pbs_fifo_capacity = lean_ctor_get_usize(o, 9);
    c.pbs_memory_capacity = lean_ctor_get_usize(o, 10);
    c.pbs_min_batch_size = lean_ctor_get_usize(o, 11);
    c.pbs_max_batch_size = lean_ctor_get_usize(o, 12);
    c.pbs_timeout = lean_ctor_get_usize(o, 13);
    c.pbs_load_unload_latency = lean_ctor_get_usize(o, 14);
    c.pbs_processing_latency_a = lean_ctor_get_usize(o, 15);
    c.pbs_processing_latency_b = lean_ctor_get_usize(o, 16);
    c.pbs_processing_latency_m = lean_ctor_get_usize(o, 17);
    c.regf_size = lean_ctor_get_usize(o, 18);
    c.heap_size = lean_ctor_get_usize(o, 19);
    return c;
}

static lean_obj_res box_hpu_config(zhc_hpu_config c) {
    lean_object *o = lean_alloc_ctor(0, 0, HPU_CONFIG_FIELDS * sizeof(size_t));
    lean_ctor_set_usize(o, 0, c.freq);
    lean_ctor_set_usize(o, 1, c.isc_depth);
    lean_ctor_set_usize(o, 2, c.isc_query_period);
    lean_ctor_set_usize(o, 3, c.mem_fifo_capacity);
    lean_ctor_set_usize(o, 4, c.mem_read_latency);
    lean_ctor_set_usize(o, 5, c.mem_write_latency);
    lean_ctor_set_usize(o, 6, c.alu_fifo_capacity);
    lean_ctor_set_usize(o, 7, c.alu_read_latency);
    lean_ctor_set_usize(o, 8, c.alu_write_latency);
    lean_ctor_set_usize(o, 9, c.pbs_fifo_capacity);
    lean_ctor_set_usize(o, 10, c.pbs_memory_capacity);
    lean_ctor_set_usize(o, 11, c.pbs_min_batch_size);
    lean_ctor_set_usize(o, 12, c.pbs_max_batch_size);
    lean_ctor_set_usize(o, 13, c.pbs_timeout);
    lean_ctor_set_usize(o, 14, c.pbs_load_unload_latency);
    lean_ctor_set_usize(o, 15, c.pbs_processing_latency_a);
    lean_ctor_set_usize(o, 16, c.pbs_processing_latency_b);
    lean_ctor_set_usize(o, 17, c.pbs_processing_latency_m);
    lean_ctor_set_usize(o, 18, c.regf_size);
    lean_ctor_set_usize(o, 19, c.heap_size);
    return o;
}

/* `structure MultiHpuConfig where hpuConfig : HpuConfig; nHpus : UInt8` */
static zhc_multi_hpu_config multi_hpu_config_of(b_lean_obj_arg o) {
    zhc_multi_hpu_config c;
    c.hpu_config = hpu_config_of(lean_ctor_get(o, 0));
    c.n_hpus = lean_ctor_get_uint8(o, sizeof(void *));
    return c;
}

static lean_obj_res box_multi_hpu_config(zhc_multi_hpu_config c) {
    lean_object *o = lean_alloc_ctor(0, 1, 1);
    lean_ctor_set(o, 0, box_hpu_config(c.hpu_config));
    lean_ctor_set_uint8(o, sizeof(void *), c.n_hpus);
    return o;
}

/* `structure VmConfig`: 11 `USize` fields, in the order of `zhc_vm_config`. */
enum { VM_CONFIG_FIELDS = 11 };

static zhc_vm_config vm_config_of(b_lean_obj_arg o) {
    zhc_vm_config c;
    c.lwe_dim = lean_ctor_get_usize(o, 0);
    c.bsk_polynomial_size = lean_ctor_get_usize(o, 1);
    c.bsk_glwe_dim = lean_ctor_get_usize(o, 2);
    c.bsk_dec_levels = lean_ctor_get_usize(o, 3);
    c.bsk_dec_base_log = lean_ctor_get_usize(o, 4);
    c.ksk_dec_levels = lean_ctor_get_usize(o, 5);
    c.ksk_dec_base_log = lean_ctor_get_usize(o, 6);
    c.delta = lean_ctor_get_usize(o, 7);
    c.carry_size = lean_ctor_get_usize(o, 8);
    c.message_size = lean_ctor_get_usize(o, 9);
    c.regf_size = lean_ctor_get_usize(o, 10);
    return c;
}

static lean_obj_res box_vm_config(zhc_vm_config c) {
    lean_object *o = lean_alloc_ctor(0, 0, VM_CONFIG_FIELDS * sizeof(size_t));
    lean_ctor_set_usize(o, 0, c.lwe_dim);
    lean_ctor_set_usize(o, 1, c.bsk_polynomial_size);
    lean_ctor_set_usize(o, 2, c.bsk_glwe_dim);
    lean_ctor_set_usize(o, 3, c.bsk_dec_levels);
    lean_ctor_set_usize(o, 4, c.bsk_dec_base_log);
    lean_ctor_set_usize(o, 5, c.ksk_dec_levels);
    lean_ctor_set_usize(o, 6, c.ksk_dec_base_log);
    lean_ctor_set_usize(o, 7, c.delta);
    lean_ctor_set_usize(o, 8, c.carry_size);
    lean_ctor_set_usize(o, 9, c.message_size);
    lean_ctor_set_usize(o, 10, c.regf_size);
    return o;
}

/* Copies a Lean `Array UInt16` into a C buffer of at most `cap` entries. Returns the length. */
static size_t table_of(b_lean_obj_arg arr, uint16_t *buf, size_t cap) {
    size_t n = lean_array_size(arr);
    if (n > cap) n = cap;
    for (size_t i = 0; i < n; i++) buf[i] = (uint16_t)lean_unbox(lean_array_get_core(arr, i));
    return n;
}

/* Largest table a LUT constructor accepts: 2^(carry + message) with both at most 8 bits. */
enum { TABLE_CAP = 1u << 16 };

/* ------------------------------------------------------------------------------------------- */
/* Configs                                                                                     */
/* ------------------------------------------------------------------------------------------- */

LEAN_EXPORT lean_obj_res zhc_lean_hpu_config_default(lean_obj_arg w) {
    (void)w;
    zhc_hpu_config c;
    zhc_status s = zhc_hpu_config_default(&c);
    return s == ZHC_OK ? lean_io_result_mk_ok(box_hpu_config(c)) : fail("zhc_hpu_config_default", s);
}

LEAN_EXPORT lean_obj_res zhc_lean_multi_hpu_config_default(lean_obj_arg w) {
    (void)w;
    zhc_multi_hpu_config c;
    zhc_status s = zhc_multi_hpu_config_default(&c);
    return s == ZHC_OK ? lean_io_result_mk_ok(box_multi_hpu_config(c)) : fail("zhc_multi_hpu_config_default", s);
}

/* ------------------------------------------------------------------------------------------- */
/* File handles and traces                                                                     */
/* ------------------------------------------------------------------------------------------- */

LEAN_EXPORT lean_obj_res zhc_lean_file_handle_open(b_lean_obj_arg h, lean_obj_arg w) {
    (void)w;
    return ret_unit("zhc_file_handle_open", zhc_file_handle_open(file_of(h)));
}

LEAN_EXPORT lean_obj_res zhc_lean_file_handle_move_to(b_lean_obj_arg h, b_lean_obj_arg path, lean_obj_arg w) {
    (void)w;
    return ret_unit("zhc_file_handle_move_to", zhc_file_handle_move_to(file_of(h), lean_string_cstr(path)));
}

LEAN_EXPORT lean_obj_res zhc_lean_perfetto_trace_open(b_lean_obj_arg t, lean_obj_arg w) {
    (void)w;
    return ret_unit("zhc_perfetto_trace_open", zhc_perfetto_trace_open(trace_of(t)));
}

/* ------------------------------------------------------------------------------------------- */
/* Lookup tables                                                                               */
/* ------------------------------------------------------------------------------------------- */

LEAN_EXPORT lean_obj_res zhc_lean_lut1_new(b_lean_obj_arg name, b_lean_obj_arg spec, b_lean_obj_arg t1, lean_obj_arg w) {
    (void)w;
    static uint16_t b1[TABLE_CAP];
    size_t len = table_of(t1, b1, TABLE_CAP);
    zhc_lut1 *out = NULL;
    zhc_status s = zhc_lut1_new(lean_string_cstr(name), spec_of(spec), b1, len, &out);
    return s == ZHC_OK ? lean_io_result_mk_ok(box_lut1(out)) : fail("zhc_lut1_new", s);
}

LEAN_EXPORT lean_obj_res zhc_lean_lut2_new(b_lean_obj_arg name, b_lean_obj_arg spec, b_lean_obj_arg t1, b_lean_obj_arg t2, lean_obj_arg w) {
    (void)w;
    static uint16_t b1[TABLE_CAP], b2[TABLE_CAP];
    size_t len = table_of(t1, b1, TABLE_CAP);
    if (table_of(t2, b2, TABLE_CAP) != len) return fail("zhc_lut2_new", ZHC_ERR_INVALID_ARGUMENT);
    zhc_lut2 *out = NULL;
    zhc_status s = zhc_lut2_new(lean_string_cstr(name), spec_of(spec), b1, b2, len, &out);
    return s == ZHC_OK ? lean_io_result_mk_ok(box_lut2(out)) : fail("zhc_lut2_new", s);
}

LEAN_EXPORT lean_obj_res zhc_lean_lut4_new(b_lean_obj_arg name, b_lean_obj_arg spec, b_lean_obj_arg t1, b_lean_obj_arg t2, b_lean_obj_arg t3, b_lean_obj_arg t4, lean_obj_arg w) {
    (void)w;
    static uint16_t b1[TABLE_CAP], b2[TABLE_CAP], b3[TABLE_CAP], b4[TABLE_CAP];
    size_t len = table_of(t1, b1, TABLE_CAP);
    if (table_of(t2, b2, TABLE_CAP) != len || table_of(t3, b3, TABLE_CAP) != len || table_of(t4, b4, TABLE_CAP) != len)
        return fail("zhc_lut4_new", ZHC_ERR_INVALID_ARGUMENT);
    zhc_lut4 *out = NULL;
    zhc_status s = zhc_lut4_new(lean_string_cstr(name), spec_of(spec), b1, b2, b3, b4, len, &out);
    return s == ZHC_OK ? lean_io_result_mk_ok(box_lut4(out)) : fail("zhc_lut4_new", s);
}

LEAN_EXPORT lean_obj_res zhc_lean_lut8_new(b_lean_obj_arg name, b_lean_obj_arg spec, b_lean_obj_arg t1, b_lean_obj_arg t2, b_lean_obj_arg t3, b_lean_obj_arg t4, b_lean_obj_arg t5, b_lean_obj_arg t6, b_lean_obj_arg t7, b_lean_obj_arg t8, lean_obj_arg w) {
    (void)w;
    static uint16_t b1[TABLE_CAP], b2[TABLE_CAP], b3[TABLE_CAP], b4[TABLE_CAP], b5[TABLE_CAP], b6[TABLE_CAP], b7[TABLE_CAP], b8[TABLE_CAP];
    size_t len = table_of(t1, b1, TABLE_CAP);
    if (table_of(t2, b2, TABLE_CAP) != len || table_of(t3, b3, TABLE_CAP) != len || table_of(t4, b4, TABLE_CAP) != len ||
        table_of(t5, b5, TABLE_CAP) != len || table_of(t6, b6, TABLE_CAP) != len || table_of(t7, b7, TABLE_CAP) != len ||
        table_of(t8, b8, TABLE_CAP) != len)
        return fail("zhc_lut8_new", ZHC_ERR_INVALID_ARGUMENT);
    zhc_lut8 *out = NULL;
    zhc_status s = zhc_lut8_new(lean_string_cstr(name), spec_of(spec), b1, b2, b3, b4, b5, b6, b7, b8, len, &out);
    return s == ZHC_OK ? lean_io_result_mk_ok(box_lut8(out)) : fail("zhc_lut8_new", s);
}

/* ------------------------------------------------------------------------------------------- */
/* Builder: construction, diagnostics, comments                                                */
/* ------------------------------------------------------------------------------------------- */

LEAN_EXPORT lean_obj_res zhc_lean_builder_new(b_lean_obj_arg spec, lean_obj_arg w) {
    (void)w;
    zhc_builder *out = NULL;
    zhc_status s = zhc_builder_new(spec_of(spec), &out);
    return s == ZHC_OK ? lean_io_result_mk_ok(box_builder(out)) : fail("zhc_builder_new", s);
}

LEAN_EXPORT lean_obj_res zhc_lean_builder_spec(b_lean_obj_arg b, lean_obj_arg w) {
    (void)w;
    zhc_ciphertext_block_spec out;
    zhc_status s = zhc_builder_spec(builder_of(b), &out);
    return s == ZHC_OK ? lean_io_result_mk_ok(box_spec(out)) : fail("zhc_builder_spec", s);
}

LEAN_EXPORT lean_obj_res zhc_lean_builder_draw(b_lean_obj_arg b, uint8_t kind, lean_obj_arg w) {
    (void)w;
    zhc_file_handle *out = NULL;
    return ret_file("zhc_builder_draw", zhc_builder_draw(builder_of(b), (zhc_ir_kind)kind, &out), &out);
}

LEAN_EXPORT lean_obj_res zhc_lean_builder_dump_noise(b_lean_obj_arg b, lean_obj_arg w) {
    (void)w;
    char *out = NULL;
    return ret_string("zhc_builder_dump_noise", zhc_builder_dump_noise(builder_of(b), &out), &out);
}

LEAN_EXPORT lean_obj_res zhc_lean_builder_check_noise(b_lean_obj_arg b, lean_obj_arg w) {
    (void)w;
    return ret_unit("zhc_builder_check_noise", zhc_builder_check_noise(builder_of(b)));
}

LEAN_EXPORT lean_obj_res zhc_lean_builder_push_comment(b_lean_obj_arg b, b_lean_obj_arg comment, lean_obj_arg w) {
    (void)w;
    return ret_unit("zhc_builder_push_comment", zhc_builder_push_comment(builder_of(b), lean_string_cstr(comment)));
}

LEAN_EXPORT lean_obj_res zhc_lean_builder_pop_comment(b_lean_obj_arg b, lean_obj_arg w) {
    (void)w;
    return ret_unit("zhc_builder_pop_comment", zhc_builder_pop_comment(builder_of(b)));
}

/* ------------------------------------------------------------------------------------------- */
/* Builder: inputs, outputs, declarations                                                      */
/* ------------------------------------------------------------------------------------------- */

LEAN_EXPORT lean_obj_res zhc_lean_builder_integer_ciphertext_input(b_lean_obj_arg b, uint16_t int_size, lean_obj_arg w) {
    (void)w;
    zhc_integer_ciphertext *out = NULL;
    return ret_int_ct("zhc_builder_integer_ciphertext_input", zhc_builder_integer_ciphertext_input(builder_of(b), int_size, &out), &out);
}

LEAN_EXPORT lean_obj_res zhc_lean_builder_integer_ciphertext_declare(b_lean_obj_arg b, uint16_t int_size, lean_obj_arg w) {
    (void)w;
    zhc_integer_ciphertext *out = NULL;
    return ret_int_ct("zhc_builder_integer_ciphertext_declare", zhc_builder_integer_ciphertext_declare(builder_of(b), int_size, &out), &out);
}

LEAN_EXPORT lean_obj_res zhc_lean_builder_integer_ciphertext_store_block(b_lean_obj_arg b, b_lean_obj_arg ct, uint8_t index, b_lean_obj_arg block, lean_obj_arg w) {
    (void)w;
    zhc_integer_ciphertext *out = NULL;
    return ret_int_ct("zhc_builder_integer_ciphertext_store_block", zhc_builder_integer_ciphertext_store_block(builder_of(b), int_ct_of(ct), index, ct_of(block), &out), &out);
}

LEAN_EXPORT lean_obj_res zhc_lean_builder_integer_ciphertext_get_block(b_lean_obj_arg b, b_lean_obj_arg ct, uint8_t index, lean_obj_arg w) {
    (void)w;
    zhc_ciphertext_block *out = NULL;
    return ret_ct("zhc_builder_integer_ciphertext_get_block", zhc_builder_integer_ciphertext_get_block(builder_of(b), int_ct_of(ct), index, &out), &out);
}

LEAN_EXPORT lean_obj_res zhc_lean_builder_integer_ciphertext_inspect(b_lean_obj_arg b, b_lean_obj_arg src, lean_obj_arg w) {
    (void)w;
    zhc_integer_ciphertext *out = NULL;
    return ret_int_ct("zhc_builder_integer_ciphertext_inspect", zhc_builder_integer_ciphertext_inspect(builder_of(b), int_ct_of(src), &out), &out);
}

LEAN_EXPORT lean_obj_res zhc_lean_builder_integer_ciphertext_output(b_lean_obj_arg b, b_lean_obj_arg ct, lean_obj_arg w) {
    (void)w;
    return ret_unit("zhc_builder_integer_ciphertext_output", zhc_builder_integer_ciphertext_output(builder_of(b), int_ct_of(ct)));
}

LEAN_EXPORT lean_obj_res zhc_lean_builder_integer_plaintext_input(b_lean_obj_arg b, uint16_t int_size, lean_obj_arg w) {
    (void)w;
    zhc_integer_plaintext *out = NULL;
    return ret_int_pt("zhc_builder_integer_plaintext_input", zhc_builder_integer_plaintext_input(builder_of(b), int_size, &out), &out);
}

LEAN_EXPORT lean_obj_res zhc_lean_builder_integer_plaintext_get_block(b_lean_obj_arg b, b_lean_obj_arg pt, uint8_t index, lean_obj_arg w) {
    (void)w;
    zhc_plaintext_block *out = NULL;
    return ret_pt("zhc_builder_integer_plaintext_get_block", zhc_builder_integer_plaintext_get_block(builder_of(b), int_pt_of(pt), index, &out), &out);
}

LEAN_EXPORT lean_obj_res zhc_lean_builder_bool_ciphertext_input(b_lean_obj_arg b, lean_obj_arg w) {
    (void)w;
    zhc_bool_ciphertext *out = NULL;
    return ret_bool("zhc_builder_bool_ciphertext_input", zhc_builder_bool_ciphertext_input(builder_of(b), &out), &out);
}

LEAN_EXPORT lean_obj_res zhc_lean_builder_bool_ciphertext_from_block(b_lean_obj_arg b, b_lean_obj_arg block, lean_obj_arg w) {
    (void)w;
    zhc_bool_ciphertext *out = NULL;
    return ret_bool("zhc_builder_bool_ciphertext_from_block", zhc_builder_bool_ciphertext_from_block(builder_of(b), ct_of(block), &out), &out);
}

LEAN_EXPORT lean_obj_res zhc_lean_builder_bool_ciphertext_get_block(b_lean_obj_arg b, b_lean_obj_arg value, lean_obj_arg w) {
    (void)w;
    zhc_ciphertext_block *out = NULL;
    return ret_ct("zhc_builder_bool_ciphertext_get_block", zhc_builder_bool_ciphertext_get_block(builder_of(b), bool_of(value), &out), &out);
}

LEAN_EXPORT lean_obj_res zhc_lean_builder_bool_ciphertext_output(b_lean_obj_arg b, b_lean_obj_arg value, lean_obj_arg w) {
    (void)w;
    return ret_unit("zhc_builder_bool_ciphertext_output", zhc_builder_bool_ciphertext_output(builder_of(b), bool_of(value)));
}

/* ------------------------------------------------------------------------------------------- */
/* Builder: block constants and inspection                                                     */
/* ------------------------------------------------------------------------------------------- */

LEAN_EXPORT lean_obj_res zhc_lean_builder_block_let_plaintext(b_lean_obj_arg b, uint8_t value, lean_obj_arg w) {
    (void)w;
    zhc_plaintext_block *out = NULL;
    return ret_pt("zhc_builder_block_let_plaintext", zhc_builder_block_let_plaintext(builder_of(b), value, &out), &out);
}

LEAN_EXPORT lean_obj_res zhc_lean_builder_block_let_ciphertext(b_lean_obj_arg b, uint8_t value, lean_obj_arg w) {
    (void)w;
    zhc_ciphertext_block *out = NULL;
    return ret_ct("zhc_builder_block_let_ciphertext", zhc_builder_block_let_ciphertext(builder_of(b), value, &out), &out);
}

LEAN_EXPORT lean_obj_res zhc_lean_builder_block_inspect(b_lean_obj_arg b, b_lean_obj_arg src, lean_obj_arg w) {
    (void)w;
    zhc_ciphertext_block *out = NULL;
    return ret_ct("zhc_builder_block_inspect", zhc_builder_block_inspect(builder_of(b), ct_of(src), &out), &out);
}

/* ------------------------------------------------------------------------------------------- */
/* Builder: block arithmetic                                                                   */
/* ------------------------------------------------------------------------------------------- */

LEAN_EXPORT lean_obj_res zhc_lean_builder_block_add_with(b_lean_obj_arg b, b_lean_obj_arg a, b_lean_obj_arg c, uint8_t flavor, lean_obj_arg w) {
    (void)w;
    zhc_ciphertext_block *out = NULL;
    return ret_ct("zhc_builder_block_add_with", zhc_builder_block_add_with(builder_of(b), ct_of(a), ct_of(c), (zhc_flavor)flavor, &out), &out);
}
LEAN_EXPORT lean_obj_res zhc_lean_builder_block_add(b_lean_obj_arg b, b_lean_obj_arg a, b_lean_obj_arg c, lean_obj_arg w) {
    (void)w;
    zhc_ciphertext_block *out = NULL;
    return ret_ct("zhc_builder_block_add", zhc_builder_block_add(builder_of(b), ct_of(a), ct_of(c), &out), &out);
}
LEAN_EXPORT lean_obj_res zhc_lean_builder_block_temper_add(b_lean_obj_arg b, b_lean_obj_arg a, b_lean_obj_arg c, lean_obj_arg w) {
    (void)w;
    zhc_ciphertext_block *out = NULL;
    return ret_ct("zhc_builder_block_temper_add", zhc_builder_block_temper_add(builder_of(b), ct_of(a), ct_of(c), &out), &out);
}
LEAN_EXPORT lean_obj_res zhc_lean_builder_block_wrapping_add(b_lean_obj_arg b, b_lean_obj_arg a, b_lean_obj_arg c, lean_obj_arg w) {
    (void)w;
    zhc_ciphertext_block *out = NULL;
    return ret_ct("zhc_builder_block_wrapping_add", zhc_builder_block_wrapping_add(builder_of(b), ct_of(a), ct_of(c), &out), &out);
}

LEAN_EXPORT lean_obj_res zhc_lean_builder_block_sub_with(b_lean_obj_arg b, b_lean_obj_arg a, b_lean_obj_arg c, uint8_t flavor, lean_obj_arg w) {
    (void)w;
    zhc_ciphertext_block *out = NULL;
    return ret_ct("zhc_builder_block_sub_with", zhc_builder_block_sub_with(builder_of(b), ct_of(a), ct_of(c), (zhc_flavor)flavor, &out), &out);
}
LEAN_EXPORT lean_obj_res zhc_lean_builder_block_sub(b_lean_obj_arg b, b_lean_obj_arg a, b_lean_obj_arg c, lean_obj_arg w) {
    (void)w;
    zhc_ciphertext_block *out = NULL;
    return ret_ct("zhc_builder_block_sub", zhc_builder_block_sub(builder_of(b), ct_of(a), ct_of(c), &out), &out);
}
LEAN_EXPORT lean_obj_res zhc_lean_builder_block_temper_sub(b_lean_obj_arg b, b_lean_obj_arg a, b_lean_obj_arg c, lean_obj_arg w) {
    (void)w;
    zhc_ciphertext_block *out = NULL;
    return ret_ct("zhc_builder_block_temper_sub", zhc_builder_block_temper_sub(builder_of(b), ct_of(a), ct_of(c), &out), &out);
}
LEAN_EXPORT lean_obj_res zhc_lean_builder_block_wrapping_sub(b_lean_obj_arg b, b_lean_obj_arg a, b_lean_obj_arg c, lean_obj_arg w) {
    (void)w;
    zhc_ciphertext_block *out = NULL;
    return ret_ct("zhc_builder_block_wrapping_sub", zhc_builder_block_wrapping_sub(builder_of(b), ct_of(a), ct_of(c), &out), &out);
}

LEAN_EXPORT lean_obj_res zhc_lean_builder_block_shl_with(b_lean_obj_arg b, b_lean_obj_arg a, uint8_t amount, uint8_t flavor, lean_obj_arg w) {
    (void)w;
    zhc_ciphertext_block *out = NULL;
    return ret_ct("zhc_builder_block_shl_with", zhc_builder_block_shl_with(builder_of(b), ct_of(a), amount, (zhc_flavor)flavor, &out), &out);
}
LEAN_EXPORT lean_obj_res zhc_lean_builder_block_shl(b_lean_obj_arg b, b_lean_obj_arg a, uint8_t amount, lean_obj_arg w) {
    (void)w;
    zhc_ciphertext_block *out = NULL;
    return ret_ct("zhc_builder_block_shl", zhc_builder_block_shl(builder_of(b), ct_of(a), amount, &out), &out);
}
LEAN_EXPORT lean_obj_res zhc_lean_builder_block_temper_shl(b_lean_obj_arg b, b_lean_obj_arg a, uint8_t amount, lean_obj_arg w) {
    (void)w;
    zhc_ciphertext_block *out = NULL;
    return ret_ct("zhc_builder_block_temper_shl", zhc_builder_block_temper_shl(builder_of(b), ct_of(a), amount, &out), &out);
}
LEAN_EXPORT lean_obj_res zhc_lean_builder_block_wrapping_shl(b_lean_obj_arg b, b_lean_obj_arg a, uint8_t amount, lean_obj_arg w) {
    (void)w;
    zhc_ciphertext_block *out = NULL;
    return ret_ct("zhc_builder_block_wrapping_shl", zhc_builder_block_wrapping_shl(builder_of(b), ct_of(a), amount, &out), &out);
}

LEAN_EXPORT lean_obj_res zhc_lean_builder_block_add_plaintext_with(b_lean_obj_arg b, b_lean_obj_arg a, b_lean_obj_arg c, uint8_t flavor, lean_obj_arg w) {
    (void)w;
    zhc_ciphertext_block *out = NULL;
    return ret_ct("zhc_builder_block_add_plaintext_with", zhc_builder_block_add_plaintext_with(builder_of(b), ct_of(a), pt_of(c), (zhc_flavor)flavor, &out), &out);
}
LEAN_EXPORT lean_obj_res zhc_lean_builder_block_add_plaintext(b_lean_obj_arg b, b_lean_obj_arg a, b_lean_obj_arg c, lean_obj_arg w) {
    (void)w;
    zhc_ciphertext_block *out = NULL;
    return ret_ct("zhc_builder_block_add_plaintext", zhc_builder_block_add_plaintext(builder_of(b), ct_of(a), pt_of(c), &out), &out);
}
LEAN_EXPORT lean_obj_res zhc_lean_builder_block_temper_add_plaintext(b_lean_obj_arg b, b_lean_obj_arg a, b_lean_obj_arg c, lean_obj_arg w) {
    (void)w;
    zhc_ciphertext_block *out = NULL;
    return ret_ct("zhc_builder_block_temper_add_plaintext", zhc_builder_block_temper_add_plaintext(builder_of(b), ct_of(a), pt_of(c), &out), &out);
}
LEAN_EXPORT lean_obj_res zhc_lean_builder_block_wrapping_add_plaintext(b_lean_obj_arg b, b_lean_obj_arg a, b_lean_obj_arg c, lean_obj_arg w) {
    (void)w;
    zhc_ciphertext_block *out = NULL;
    return ret_ct("zhc_builder_block_wrapping_add_plaintext", zhc_builder_block_wrapping_add_plaintext(builder_of(b), ct_of(a), pt_of(c), &out), &out);
}

LEAN_EXPORT lean_obj_res zhc_lean_builder_block_sub_plaintext_with(b_lean_obj_arg b, b_lean_obj_arg a, b_lean_obj_arg c, uint8_t flavor, lean_obj_arg w) {
    (void)w;
    zhc_ciphertext_block *out = NULL;
    return ret_ct("zhc_builder_block_sub_plaintext_with", zhc_builder_block_sub_plaintext_with(builder_of(b), ct_of(a), pt_of(c), (zhc_flavor)flavor, &out), &out);
}
LEAN_EXPORT lean_obj_res zhc_lean_builder_block_sub_plaintext(b_lean_obj_arg b, b_lean_obj_arg a, b_lean_obj_arg c, lean_obj_arg w) {
    (void)w;
    zhc_ciphertext_block *out = NULL;
    return ret_ct("zhc_builder_block_sub_plaintext", zhc_builder_block_sub_plaintext(builder_of(b), ct_of(a), pt_of(c), &out), &out);
}
LEAN_EXPORT lean_obj_res zhc_lean_builder_block_temper_sub_plaintext(b_lean_obj_arg b, b_lean_obj_arg a, b_lean_obj_arg c, lean_obj_arg w) {
    (void)w;
    zhc_ciphertext_block *out = NULL;
    return ret_ct("zhc_builder_block_temper_sub_plaintext", zhc_builder_block_temper_sub_plaintext(builder_of(b), ct_of(a), pt_of(c), &out), &out);
}
LEAN_EXPORT lean_obj_res zhc_lean_builder_block_wrapping_sub_plaintext(b_lean_obj_arg b, b_lean_obj_arg a, b_lean_obj_arg c, lean_obj_arg w) {
    (void)w;
    zhc_ciphertext_block *out = NULL;
    return ret_ct("zhc_builder_block_wrapping_sub_plaintext", zhc_builder_block_wrapping_sub_plaintext(builder_of(b), ct_of(a), pt_of(c), &out), &out);
}

LEAN_EXPORT lean_obj_res zhc_lean_builder_block_plaintext_sub_with(b_lean_obj_arg b, b_lean_obj_arg a, b_lean_obj_arg c, uint8_t flavor, lean_obj_arg w) {
    (void)w;
    zhc_ciphertext_block *out = NULL;
    return ret_ct("zhc_builder_block_plaintext_sub_with", zhc_builder_block_plaintext_sub_with(builder_of(b), pt_of(a), ct_of(c), (zhc_flavor)flavor, &out), &out);
}
LEAN_EXPORT lean_obj_res zhc_lean_builder_block_plaintext_sub(b_lean_obj_arg b, b_lean_obj_arg a, b_lean_obj_arg c, lean_obj_arg w) {
    (void)w;
    zhc_ciphertext_block *out = NULL;
    return ret_ct("zhc_builder_block_plaintext_sub", zhc_builder_block_plaintext_sub(builder_of(b), pt_of(a), ct_of(c), &out), &out);
}
LEAN_EXPORT lean_obj_res zhc_lean_builder_block_temper_plaintext_sub(b_lean_obj_arg b, b_lean_obj_arg a, b_lean_obj_arg c, lean_obj_arg w) {
    (void)w;
    zhc_ciphertext_block *out = NULL;
    return ret_ct("zhc_builder_block_temper_plaintext_sub", zhc_builder_block_temper_plaintext_sub(builder_of(b), pt_of(a), ct_of(c), &out), &out);
}
LEAN_EXPORT lean_obj_res zhc_lean_builder_block_wrapping_plaintext_sub(b_lean_obj_arg b, b_lean_obj_arg a, b_lean_obj_arg c, lean_obj_arg w) {
    (void)w;
    zhc_ciphertext_block *out = NULL;
    return ret_ct("zhc_builder_block_wrapping_plaintext_sub", zhc_builder_block_wrapping_plaintext_sub(builder_of(b), pt_of(a), ct_of(c), &out), &out);
}

LEAN_EXPORT lean_obj_res zhc_lean_builder_block_mul_plaintext_with(b_lean_obj_arg b, b_lean_obj_arg a, b_lean_obj_arg c, uint8_t flavor, lean_obj_arg w) {
    (void)w;
    zhc_ciphertext_block *out = NULL;
    return ret_ct("zhc_builder_block_mul_plaintext_with", zhc_builder_block_mul_plaintext_with(builder_of(b), ct_of(a), pt_of(c), (zhc_flavor)flavor, &out), &out);
}
LEAN_EXPORT lean_obj_res zhc_lean_builder_block_mul_plaintext(b_lean_obj_arg b, b_lean_obj_arg a, b_lean_obj_arg c, lean_obj_arg w) {
    (void)w;
    zhc_ciphertext_block *out = NULL;
    return ret_ct("zhc_builder_block_mul_plaintext", zhc_builder_block_mul_plaintext(builder_of(b), ct_of(a), pt_of(c), &out), &out);
}
LEAN_EXPORT lean_obj_res zhc_lean_builder_block_temper_mul_plaintext(b_lean_obj_arg b, b_lean_obj_arg a, b_lean_obj_arg c, lean_obj_arg w) {
    (void)w;
    zhc_ciphertext_block *out = NULL;
    return ret_ct("zhc_builder_block_temper_mul_plaintext", zhc_builder_block_temper_mul_plaintext(builder_of(b), ct_of(a), pt_of(c), &out), &out);
}
LEAN_EXPORT lean_obj_res zhc_lean_builder_block_wrapping_mul_plaintext(b_lean_obj_arg b, b_lean_obj_arg a, b_lean_obj_arg c, lean_obj_arg w) {
    (void)w;
    zhc_ciphertext_block *out = NULL;
    return ret_ct("zhc_builder_block_wrapping_mul_plaintext", zhc_builder_block_wrapping_mul_plaintext(builder_of(b), ct_of(a), pt_of(c), &out), &out);
}

LEAN_EXPORT lean_obj_res zhc_lean_builder_block_mac_with(b_lean_obj_arg b, b_lean_obj_arg a, b_lean_obj_arg c, uint8_t mul, uint8_t flavor, lean_obj_arg w) {
    (void)w;
    zhc_ciphertext_block *out = NULL;
    return ret_ct("zhc_builder_block_mac_with", zhc_builder_block_mac_with(builder_of(b), ct_of(a), ct_of(c), mul, (zhc_flavor)flavor, &out), &out);
}
LEAN_EXPORT lean_obj_res zhc_lean_builder_block_mac(b_lean_obj_arg b, b_lean_obj_arg a, b_lean_obj_arg c, uint8_t mul, lean_obj_arg w) {
    (void)w;
    zhc_ciphertext_block *out = NULL;
    return ret_ct("zhc_builder_block_mac", zhc_builder_block_mac(builder_of(b), ct_of(a), ct_of(c), mul, &out), &out);
}
LEAN_EXPORT lean_obj_res zhc_lean_builder_block_temper_mac(b_lean_obj_arg b, b_lean_obj_arg a, b_lean_obj_arg c, uint8_t mul, lean_obj_arg w) {
    (void)w;
    zhc_ciphertext_block *out = NULL;
    return ret_ct("zhc_builder_block_temper_mac", zhc_builder_block_temper_mac(builder_of(b), ct_of(a), ct_of(c), mul, &out), &out);
}
LEAN_EXPORT lean_obj_res zhc_lean_builder_block_wrapping_mac(b_lean_obj_arg b, b_lean_obj_arg a, b_lean_obj_arg c, uint8_t mul, lean_obj_arg w) {
    (void)w;
    zhc_ciphertext_block *out = NULL;
    return ret_ct("zhc_builder_block_wrapping_mac", zhc_builder_block_wrapping_mac(builder_of(b), ct_of(a), ct_of(c), mul, &out), &out);
}

LEAN_EXPORT lean_obj_res zhc_lean_builder_block_pack_with(b_lean_obj_arg b, b_lean_obj_arg a, b_lean_obj_arg c, uint8_t flavor, lean_obj_arg w) {
    (void)w;
    zhc_ciphertext_block *out = NULL;
    return ret_ct("zhc_builder_block_pack_with", zhc_builder_block_pack_with(builder_of(b), ct_of(a), ct_of(c), (zhc_flavor)flavor, &out), &out);
}
LEAN_EXPORT lean_obj_res zhc_lean_builder_block_pack(b_lean_obj_arg b, b_lean_obj_arg a, b_lean_obj_arg c, lean_obj_arg w) {
    (void)w;
    zhc_ciphertext_block *out = NULL;
    return ret_ct("zhc_builder_block_pack", zhc_builder_block_pack(builder_of(b), ct_of(a), ct_of(c), &out), &out);
}
LEAN_EXPORT lean_obj_res zhc_lean_builder_block_temper_pack(b_lean_obj_arg b, b_lean_obj_arg a, b_lean_obj_arg c, lean_obj_arg w) {
    (void)w;
    zhc_ciphertext_block *out = NULL;
    return ret_ct("zhc_builder_block_temper_pack", zhc_builder_block_temper_pack(builder_of(b), ct_of(a), ct_of(c), &out), &out);
}
LEAN_EXPORT lean_obj_res zhc_lean_builder_block_wrapping_pack(b_lean_obj_arg b, b_lean_obj_arg a, b_lean_obj_arg c, lean_obj_arg w) {
    (void)w;
    zhc_ciphertext_block *out = NULL;
    return ret_ct("zhc_builder_block_wrapping_pack", zhc_builder_block_wrapping_pack(builder_of(b), ct_of(a), ct_of(c), &out), &out);
}

/* ------------------------------------------------------------------------------------------- */
/* Builder: block lookups                                                                      */
/* ------------------------------------------------------------------------------------------- */

LEAN_EXPORT lean_obj_res zhc_lean_builder_block_lookup_with(b_lean_obj_arg b, b_lean_obj_arg src, b_lean_obj_arg lut, b_lean_obj_arg check, lean_obj_arg w) {
    (void)w;
    zhc_ciphertext_block *out = NULL;
    return ret_ct("zhc_builder_block_lookup_with", zhc_builder_block_lookup_with(builder_of(b), ct_of(src), lut1_of(lut), check_of(check), &out), &out);
}
LEAN_EXPORT lean_obj_res zhc_lean_builder_block_lookup(b_lean_obj_arg b, b_lean_obj_arg src, b_lean_obj_arg lut, lean_obj_arg w) {
    (void)w;
    zhc_ciphertext_block *out = NULL;
    return ret_ct("zhc_builder_block_lookup", zhc_builder_block_lookup(builder_of(b), ct_of(src), lut1_of(lut), &out), &out);
}
LEAN_EXPORT lean_obj_res zhc_lean_builder_block_padding_lookup(b_lean_obj_arg b, b_lean_obj_arg src, b_lean_obj_arg lut, lean_obj_arg w) {
    (void)w;
    zhc_ciphertext_block *out = NULL;
    return ret_ct("zhc_builder_block_padding_lookup", zhc_builder_block_padding_lookup(builder_of(b), ct_of(src), lut1_of(lut), &out), &out);
}
LEAN_EXPORT lean_obj_res zhc_lean_builder_block_wrapping_lookup(b_lean_obj_arg b, b_lean_obj_arg src, b_lean_obj_arg lut, lean_obj_arg w) {
    (void)w;
    zhc_ciphertext_block *out = NULL;
    return ret_ct("zhc_builder_block_wrapping_lookup", zhc_builder_block_wrapping_lookup(builder_of(b), ct_of(src), lut1_of(lut), &out), &out);
}
LEAN_EXPORT lean_obj_res zhc_lean_builder_block_pack_then_lookup(b_lean_obj_arg b, b_lean_obj_arg a, b_lean_obj_arg c, b_lean_obj_arg lut, lean_obj_arg w) {
    (void)w;
    zhc_ciphertext_block *out = NULL;
    return ret_ct("zhc_builder_block_pack_then_lookup", zhc_builder_block_pack_then_lookup(builder_of(b), ct_of(a), ct_of(c), lut1_of(lut), &out), &out);
}
LEAN_EXPORT lean_obj_res zhc_lean_builder_block_mac_then_lookup(b_lean_obj_arg b, b_lean_obj_arg a, b_lean_obj_arg c, uint8_t mul, b_lean_obj_arg lut, lean_obj_arg w) {
    (void)w;
    zhc_ciphertext_block *out = NULL;
    return ret_ct("zhc_builder_block_mac_then_lookup", zhc_builder_block_mac_then_lookup(builder_of(b), ct_of(a), ct_of(c), mul, lut1_of(lut), &out), &out);
}

LEAN_EXPORT lean_obj_res zhc_lean_builder_block_lookup2_with(b_lean_obj_arg b, b_lean_obj_arg src, b_lean_obj_arg lut, b_lean_obj_arg check, lean_obj_arg w) {
    (void)w;
    zhc_ciphertext_block *out[2] = {NULL, NULL};
    return ret_cts("zhc_builder_block_lookup2_with", zhc_builder_block_lookup2_with(builder_of(b), ct_of(src), lut2_of(lut), check_of(check), out), out, 2);
}
LEAN_EXPORT lean_obj_res zhc_lean_builder_block_lookup2(b_lean_obj_arg b, b_lean_obj_arg src, b_lean_obj_arg lut, lean_obj_arg w) {
    (void)w;
    zhc_ciphertext_block *out[2] = {NULL, NULL};
    return ret_cts("zhc_builder_block_lookup2", zhc_builder_block_lookup2(builder_of(b), ct_of(src), lut2_of(lut), out), out, 2);
}
LEAN_EXPORT lean_obj_res zhc_lean_builder_block_lookup4_with(b_lean_obj_arg b, b_lean_obj_arg src, b_lean_obj_arg lut, b_lean_obj_arg check, lean_obj_arg w) {
    (void)w;
    zhc_ciphertext_block *out[4] = {NULL, NULL, NULL, NULL};
    return ret_cts("zhc_builder_block_lookup4_with", zhc_builder_block_lookup4_with(builder_of(b), ct_of(src), lut4_of(lut), check_of(check), out), out, 4);
}
LEAN_EXPORT lean_obj_res zhc_lean_builder_block_lookup4(b_lean_obj_arg b, b_lean_obj_arg src, b_lean_obj_arg lut, lean_obj_arg w) {
    (void)w;
    zhc_ciphertext_block *out[4] = {NULL, NULL, NULL, NULL};
    return ret_cts("zhc_builder_block_lookup4", zhc_builder_block_lookup4(builder_of(b), ct_of(src), lut4_of(lut), out), out, 4);
}
LEAN_EXPORT lean_obj_res zhc_lean_builder_block_lookup8_with(b_lean_obj_arg b, b_lean_obj_arg src, b_lean_obj_arg lut, b_lean_obj_arg check, lean_obj_arg w) {
    (void)w;
    zhc_ciphertext_block *out[8] = {NULL, NULL, NULL, NULL, NULL, NULL, NULL, NULL};
    return ret_cts("zhc_builder_block_lookup8_with", zhc_builder_block_lookup8_with(builder_of(b), ct_of(src), lut8_of(lut), check_of(check), out), out, 8);
}
LEAN_EXPORT lean_obj_res zhc_lean_builder_block_lookup8(b_lean_obj_arg b, b_lean_obj_arg src, b_lean_obj_arg lut, lean_obj_arg w) {
    (void)w;
    zhc_ciphertext_block *out[8] = {NULL, NULL, NULL, NULL, NULL, NULL, NULL, NULL};
    return ret_cts("zhc_builder_block_lookup8", zhc_builder_block_lookup8(builder_of(b), ct_of(src), lut8_of(lut), out), out, 8);
}

/* ------------------------------------------------------------------------------------------- */
/* Pipeline                                                                                    */
/* ------------------------------------------------------------------------------------------- */

LEAN_EXPORT lean_obj_res zhc_lean_pipeline_new(lean_obj_arg w) {
    (void)w;
    zhc_pipeline *out = NULL;
    zhc_status s = zhc_pipeline_new(&out);
    return s == ZHC_OK ? lean_io_result_mk_ok(box_pipeline(out)) : fail("zhc_pipeline_new", s);
}

LEAN_EXPORT lean_obj_res zhc_lean_pipeline_set_builder(b_lean_obj_arg p, b_lean_obj_arg b, lean_obj_arg w) {
    (void)w;
    return ret_unit("zhc_pipeline_set_builder", zhc_pipeline_set_builder(pipeline_of(p), builder_of(b)));
}
LEAN_EXPORT lean_obj_res zhc_lean_pipeline_set_ciphertext_block_spec(b_lean_obj_arg p, b_lean_obj_arg spec, lean_obj_arg w) {
    (void)w;
    return ret_unit("zhc_pipeline_set_ciphertext_block_spec", zhc_pipeline_set_ciphertext_block_spec(pipeline_of(p), spec_of(spec)));
}
LEAN_EXPORT lean_obj_res zhc_lean_pipeline_set_hpu_config(b_lean_obj_arg p, b_lean_obj_arg c, lean_obj_arg w) {
    (void)w;
    return ret_unit("zhc_pipeline_set_hpu_config", zhc_pipeline_set_hpu_config(pipeline_of(p), hpu_config_of(c)));
}
LEAN_EXPORT lean_obj_res zhc_lean_pipeline_set_multi_hpu_config(b_lean_obj_arg p, b_lean_obj_arg c, lean_obj_arg w) {
    (void)w;
    return ret_unit("zhc_pipeline_set_multi_hpu_config", zhc_pipeline_set_multi_hpu_config(pipeline_of(p), multi_hpu_config_of(c)));
}
LEAN_EXPORT lean_obj_res zhc_lean_pipeline_set_vm_config(b_lean_obj_arg p, b_lean_obj_arg c, lean_obj_arg w) {
    (void)w;
    return ret_unit("zhc_pipeline_set_vm_config", zhc_pipeline_set_vm_config(pipeline_of(p), vm_config_of(c)));
}
LEAN_EXPORT lean_obj_res zhc_lean_pipeline_set_legacy_hpu_scheduler(b_lean_obj_arg p, lean_obj_arg w) {
    (void)w;
    return ret_unit("zhc_pipeline_set_legacy_hpu_scheduler", zhc_pipeline_set_legacy_hpu_scheduler(pipeline_of(p)));
}
LEAN_EXPORT lean_obj_res zhc_lean_pipeline_set_trace_hpu_events(b_lean_obj_arg p, lean_obj_arg w) {
    (void)w;
    return ret_unit("zhc_pipeline_set_trace_hpu_events", zhc_pipeline_set_trace_hpu_events(pipeline_of(p)));
}

LEAN_EXPORT lean_obj_res zhc_lean_pipeline_get_ciphertext_block_spec(b_lean_obj_arg p, lean_obj_arg w) {
    (void)w;
    zhc_ciphertext_block_spec out;
    zhc_status s = zhc_pipeline_get_ciphertext_block_spec(pipeline_of(p), &out);
    return s == ZHC_OK ? lean_io_result_mk_ok(box_spec(out)) : fail("zhc_pipeline_get_ciphertext_block_spec", s);
}
LEAN_EXPORT lean_obj_res zhc_lean_pipeline_get_hpu_config(b_lean_obj_arg p, lean_obj_arg w) {
    (void)w;
    zhc_hpu_config out;
    zhc_status s = zhc_pipeline_get_hpu_config(pipeline_of(p), &out);
    return s == ZHC_OK ? lean_io_result_mk_ok(box_hpu_config(out)) : fail("zhc_pipeline_get_hpu_config", s);
}
LEAN_EXPORT lean_obj_res zhc_lean_pipeline_get_multi_hpu_config(b_lean_obj_arg p, lean_obj_arg w) {
    (void)w;
    zhc_multi_hpu_config out;
    zhc_status s = zhc_pipeline_get_multi_hpu_config(pipeline_of(p), &out);
    return s == ZHC_OK ? lean_io_result_mk_ok(box_multi_hpu_config(out)) : fail("zhc_pipeline_get_multi_hpu_config", s);
}
LEAN_EXPORT lean_obj_res zhc_lean_pipeline_get_vm_config(b_lean_obj_arg p, lean_obj_arg w) {
    (void)w;
    zhc_vm_config out;
    zhc_status s = zhc_pipeline_get_vm_config(pipeline_of(p), &out);
    return s == ZHC_OK ? lean_io_result_mk_ok(box_vm_config(out)) : fail("zhc_pipeline_get_vm_config", s);
}
LEAN_EXPORT lean_obj_res zhc_lean_pipeline_get_fingerprint(b_lean_obj_arg p, lean_obj_arg w) {
    (void)w;
    uint64_t out = 0;
    zhc_status s = zhc_pipeline_get_fingerprint(pipeline_of(p), &out);
    return s == ZHC_OK ? lean_io_result_mk_ok(lean_box_uint64(out)) : fail("zhc_pipeline_get_fingerprint", s);
}
LEAN_EXPORT lean_obj_res zhc_lean_pipeline_get_hpu_metrics(b_lean_obj_arg p, lean_obj_arg w) {
    (void)w;
    zhc_hpu_metrics *out = NULL;
    zhc_status s = zhc_pipeline_get_hpu_metrics(pipeline_of(p), &out);
    return s == ZHC_OK ? lean_io_result_mk_ok(box_hpu_metrics(out)) : fail("zhc_pipeline_get_hpu_metrics", s);
}
LEAN_EXPORT lean_obj_res zhc_lean_pipeline_get_multi_hpu_metrics(b_lean_obj_arg p, lean_obj_arg w) {
    (void)w;
    zhc_multi_hpu_metrics *out = NULL;
    zhc_status s = zhc_pipeline_get_multi_hpu_metrics(pipeline_of(p), &out);
    return s == ZHC_OK ? lean_io_result_mk_ok(box_multi_hpu_metrics(out)) : fail("zhc_pipeline_get_multi_hpu_metrics", s);
}
LEAN_EXPORT lean_obj_res zhc_lean_pipeline_get_pbs_metrics(b_lean_obj_arg p, lean_obj_arg w) {
    (void)w;
    zhc_pbs_metrics *out = NULL;
    zhc_status s = zhc_pipeline_get_pbs_metrics(pipeline_of(p), &out);
    return s == ZHC_OK ? lean_io_result_mk_ok(box_pbs_metrics(out)) : fail("zhc_pipeline_get_pbs_metrics", s);
}
LEAN_EXPORT lean_obj_res zhc_lean_pipeline_draw_state(b_lean_obj_arg p, lean_obj_arg w) {
    (void)w;
    zhc_file_handle *out = NULL;
    return ret_file("zhc_pipeline_draw_state", zhc_pipeline_draw_state(pipeline_of(p), &out), &out);
}
LEAN_EXPORT lean_obj_res zhc_lean_pipeline_get_slack_drawing(b_lean_obj_arg p, lean_obj_arg w) {
    (void)w;
    zhc_file_handle *out = NULL;
    return ret_file("zhc_pipeline_get_slack_drawing", zhc_pipeline_get_slack_drawing(pipeline_of(p), &out), &out);
}
LEAN_EXPORT lean_obj_res zhc_lean_pipeline_get_hpu_assembly(b_lean_obj_arg p, lean_obj_arg w) {
    (void)w;
    zhc_file_handle *out = NULL;
    return ret_file("zhc_pipeline_get_hpu_assembly", zhc_pipeline_get_hpu_assembly(pipeline_of(p), &out), &out);
}
LEAN_EXPORT lean_obj_res zhc_lean_pipeline_get_hpu_trace(b_lean_obj_arg p, lean_obj_arg w) {
    (void)w;
    zhc_perfetto_trace *out = NULL;
    return ret_trace("zhc_pipeline_get_hpu_trace", zhc_pipeline_get_hpu_trace(pipeline_of(p), &out), &out);
}
LEAN_EXPORT lean_obj_res zhc_lean_pipeline_get_multi_hpu_trace(b_lean_obj_arg p, lean_obj_arg w) {
    (void)w;
    zhc_perfetto_trace *out = NULL;
    return ret_trace("zhc_pipeline_get_multi_hpu_trace", zhc_pipeline_get_multi_hpu_trace(pipeline_of(p), &out), &out);
}
