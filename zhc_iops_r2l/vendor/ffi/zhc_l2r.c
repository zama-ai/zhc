#include <lean/lean.h>
#include <stdio.h>

#include "zhc.h"

static const char *status_name(zhc_status s) {
    switch (s) {
    case ZHC_OK: return "ok";
    case ZHC_ERR_NULL_POINTER: return "null pointer";
    case ZHC_ERR_INVALID_ARGUMENT: return "invalid argument";
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

static void noop_foreach(void *data, b_lean_obj_arg f) {
    (void)data;
    (void)f;
}

static void finalize_spec_block(void *p) { zhc_spec_block_free((zhc_spec_block *)p); }
static void finalize_spec_integer(void *p) { zhc_spec_integer_free((zhc_spec_integer *)p); }
static void finalize_builder(void *p) { zhc_builder_free((zhc_builder *)p); }
static void finalize_ciphertext_block(void *p) { zhc_ciphertext_block_free((zhc_ciphertext_block *)p); }
static void finalize_plaintext_block(void *p) { zhc_plaintext_block_free((zhc_plaintext_block *)p); }
static void finalize_ciphertext_bool(void *p) { zhc_ciphertext_bool_free((zhc_ciphertext_bool *)p); }
static void finalize_ciphertext_integer(void *p) { zhc_ciphertext_integer_free((zhc_ciphertext_integer *)p); }
static void finalize_plaintext_integer(void *p) { zhc_plaintext_integer_free((zhc_plaintext_integer *)p); }
static void finalize_lut1(void *p) { zhc_lut1_free((zhc_lut1 *)p); }
static void finalize_lut2(void *p) { zhc_lut2_free((zhc_lut2 *)p); }
static void finalize_lut4(void *p) { zhc_lut4_free((zhc_lut4 *)p); }
static void finalize_lut8(void *p) { zhc_lut8_free((zhc_lut8 *)p); }

static lean_external_class *g_spec_block_class;
static lean_external_class *g_spec_integer_class;
static lean_external_class *g_builder_class;
static lean_external_class *g_ciphertext_block_class;
static lean_external_class *g_plaintext_block_class;
static lean_external_class *g_ciphertext_bool_class;
static lean_external_class *g_ciphertext_integer_class;
static lean_external_class *g_plaintext_integer_class;
static lean_external_class *g_lut1_class;
static lean_external_class *g_lut2_class;
static lean_external_class *g_lut4_class;
static lean_external_class *g_lut8_class;

static lean_external_class *class_of(lean_external_class **slot, lean_external_finalize_proc fin) {
    if (*slot == NULL) *slot = lean_register_external_class(fin, noop_foreach);
    return *slot;
}

static lean_obj_res box_spec_block(zhc_spec_block *p) { return lean_alloc_external(class_of(&g_spec_block_class, finalize_spec_block), p); }
static lean_obj_res box_spec_integer(zhc_spec_integer *p) { return lean_alloc_external(class_of(&g_spec_integer_class, finalize_spec_integer), p); }
static lean_obj_res box_builder(zhc_builder *p) { return lean_alloc_external(class_of(&g_builder_class, finalize_builder), p); }
static lean_obj_res box_ciphertext_block(zhc_ciphertext_block *p) { return lean_alloc_external(class_of(&g_ciphertext_block_class, finalize_ciphertext_block), p); }
static lean_obj_res box_plaintext_block(zhc_plaintext_block *p) { return lean_alloc_external(class_of(&g_plaintext_block_class, finalize_plaintext_block), p); }
static lean_obj_res box_ciphertext_bool(zhc_ciphertext_bool *p) { return lean_alloc_external(class_of(&g_ciphertext_bool_class, finalize_ciphertext_bool), p); }
static lean_obj_res box_ciphertext_integer(zhc_ciphertext_integer *p) { return lean_alloc_external(class_of(&g_ciphertext_integer_class, finalize_ciphertext_integer), p); }
static lean_obj_res box_plaintext_integer(zhc_plaintext_integer *p) { return lean_alloc_external(class_of(&g_plaintext_integer_class, finalize_plaintext_integer), p); }
static lean_obj_res box_lut1(zhc_lut1 *p) { return lean_alloc_external(class_of(&g_lut1_class, finalize_lut1), p); }
static lean_obj_res box_lut2(zhc_lut2 *p) { return lean_alloc_external(class_of(&g_lut2_class, finalize_lut2), p); }
static lean_obj_res box_lut4(zhc_lut4 *p) { return lean_alloc_external(class_of(&g_lut4_class, finalize_lut4), p); }
static lean_obj_res box_lut8(zhc_lut8 *p) { return lean_alloc_external(class_of(&g_lut8_class, finalize_lut8), p); }

LEAN_EXPORT lean_obj_res zhc_lean_box_builder(zhc_builder *p) { return box_builder(p); }
LEAN_EXPORT lean_obj_res zhc_lean_box_ciphertext_integer(zhc_ciphertext_integer *p) { return box_ciphertext_integer(p); }
LEAN_EXPORT lean_obj_res zhc_lean_box_plaintext_integer(zhc_plaintext_integer *p) { return box_plaintext_integer(p); }
LEAN_EXPORT lean_obj_res zhc_lean_box_ciphertext_bool(zhc_ciphertext_bool *p) { return box_ciphertext_bool(p); }

static zhc_spec_block *spec_block_of(b_lean_obj_arg o) { return (zhc_spec_block *)lean_get_external_data(o); }
static zhc_spec_integer *spec_integer_of(b_lean_obj_arg o) { return (zhc_spec_integer *)lean_get_external_data(o); }
static zhc_builder *builder_of(b_lean_obj_arg o) { return (zhc_builder *)lean_get_external_data(o); }
static zhc_ciphertext_block *ct_of(b_lean_obj_arg o) { return (zhc_ciphertext_block *)lean_get_external_data(o); }
static zhc_plaintext_block *pt_of(b_lean_obj_arg o) { return (zhc_plaintext_block *)lean_get_external_data(o); }
static zhc_ciphertext_bool *bool_of(b_lean_obj_arg o) { return (zhc_ciphertext_bool *)lean_get_external_data(o); }
static zhc_ciphertext_integer *int_ct_of(b_lean_obj_arg o) { return (zhc_ciphertext_integer *)lean_get_external_data(o); }
static zhc_plaintext_integer *int_pt_of(b_lean_obj_arg o) { return (zhc_plaintext_integer *)lean_get_external_data(o); }
static zhc_lut1 *lut1_of(b_lean_obj_arg o) { return (zhc_lut1 *)lean_get_external_data(o); }
static zhc_lut2 *lut2_of(b_lean_obj_arg o) { return (zhc_lut2 *)lean_get_external_data(o); }
static zhc_lut4 *lut4_of(b_lean_obj_arg o) { return (zhc_lut4 *)lean_get_external_data(o); }
static zhc_lut8 *lut8_of(b_lean_obj_arg o) { return (zhc_lut8 *)lean_get_external_data(o); }

static void *take_data(b_lean_obj_arg o) {
    if (!lean_is_exclusive(o)) return NULL;
    void *p = lean_get_external_data(o);
    lean_to_external(o)->m_data = NULL;
    return p;
}

static lean_obj_res ret_spec_block(const char *fn, zhc_status s, zhc_spec_block **p) {
    return s == ZHC_OK ? lean_io_result_mk_ok(box_spec_block(*p)) : fail(fn, s);
}
static lean_obj_res ret_spec_integer(const char *fn, zhc_status s, zhc_spec_integer **p) {
    return s == ZHC_OK ? lean_io_result_mk_ok(box_spec_integer(*p)) : fail(fn, s);
}
static lean_obj_res ret_ct(const char *fn, zhc_status s, zhc_ciphertext_block **p) {
    return s == ZHC_OK ? lean_io_result_mk_ok(box_ciphertext_block(*p)) : fail(fn, s);
}
static lean_obj_res ret_pt(const char *fn, zhc_status s, zhc_plaintext_block **p) {
    return s == ZHC_OK ? lean_io_result_mk_ok(box_plaintext_block(*p)) : fail(fn, s);
}
static lean_obj_res ret_bool(const char *fn, zhc_status s, zhc_ciphertext_bool **p) {
    return s == ZHC_OK ? lean_io_result_mk_ok(box_ciphertext_bool(*p)) : fail(fn, s);
}
static lean_obj_res ret_int_ct(const char *fn, zhc_status s, zhc_ciphertext_integer **p) {
    return s == ZHC_OK ? lean_io_result_mk_ok(box_ciphertext_integer(*p)) : fail(fn, s);
}
static lean_obj_res ret_int_pt(const char *fn, zhc_status s, zhc_plaintext_integer **p) {
    return s == ZHC_OK ? lean_io_result_mk_ok(box_plaintext_integer(*p)) : fail(fn, s);
}
static lean_obj_res ret_unit(const char *fn, zhc_status s) {
    return s == ZHC_OK ? ok_unit() : fail(fn, s);
}

static lean_obj_res ret_string(const char *fn, zhc_status s, char **str) {
    if (s != ZHC_OK) return fail(fn, s);
    lean_object *out = lean_mk_string(*str);
    zhc_string_free(*str);
    return lean_io_result_mk_ok(out);
}

static lean_obj_res ret_cts(const char *fn, zhc_status s, zhc_ciphertext_block **blocks, size_t n) {
    if (s != ZHC_OK) return fail(fn, s);
    lean_object *arr = lean_alloc_array(n, n);
    for (size_t i = 0; i < n; i++) lean_array_set_core(arr, i, box_ciphertext_block(blocks[i]));
    return lean_io_result_mk_ok(arr);
}

LEAN_EXPORT lean_obj_res zhc_lean_spec_block_new(uint8_t carry, uint8_t message) {
    zhc_spec_block *out = NULL;
    return ret_spec_block("zhc_spec_block_new", zhc_spec_block_new(carry, message, &out), &out);
}

LEAN_EXPORT lean_obj_res zhc_lean_spec_block_carry(b_lean_obj_arg spec) {
    uint8_t out = 0;
    zhc_status s = zhc_spec_block_carry(spec_block_of(spec), &out);
    return s == ZHC_OK ? lean_io_result_mk_ok(lean_box(out)) : fail("zhc_spec_block_carry", s);
}

LEAN_EXPORT lean_obj_res zhc_lean_spec_block_message(b_lean_obj_arg spec) {
    uint8_t out = 0;
    zhc_status s = zhc_spec_block_message(spec_block_of(spec), &out);
    return s == ZHC_OK ? lean_io_result_mk_ok(lean_box(out)) : fail("zhc_spec_block_message", s);
}

LEAN_EXPORT lean_obj_res zhc_lean_spec_integer_int_size(b_lean_obj_arg spec) {
    uint16_t out = 0;
    zhc_status s = zhc_spec_integer_int_size(spec_integer_of(spec), &out);
    return s == ZHC_OK ? lean_io_result_mk_ok(lean_box(out)) : fail("zhc_spec_integer_int_size", s);
}

LEAN_EXPORT lean_obj_res zhc_lean_spec_integer_block(b_lean_obj_arg spec) {
    zhc_spec_block *out = NULL;
    return ret_spec_block("zhc_spec_integer_block", zhc_spec_integer_block(spec_integer_of(spec), &out), &out);
}

static zhc_status table_len(b_lean_obj_arg spec, unsigned k, size_t *len) {
    uint8_t carry = 0, message = 0;
    zhc_status s = zhc_spec_block_carry(spec_block_of(spec), &carry);
    if (s != ZHC_OK) return s;
    s = zhc_spec_block_message(spec_block_of(spec), &message);
    if (s != ZHC_OK) return s;
    unsigned data_size = (unsigned)carry + message;
    if (data_size < k || data_size >= 16) return ZHC_ERR_INVALID_ARGUMENT;
    *len = (size_t)1 << (data_size - k);
    return ZHC_OK;
}

static bool table_of(b_lean_obj_arg arr, uint16_t *buf, size_t len) {
    if (lean_array_size(arr) != len) return false;
    for (size_t i = 0; i < len; i++) buf[i] = (uint16_t)lean_unbox(lean_array_get_core(arr, i));
    return true;
}

LEAN_EXPORT lean_obj_res zhc_lean_lut1_new(b_lean_obj_arg name, b_lean_obj_arg spec, b_lean_obj_arg t1) {
    size_t len = 0;
    zhc_status s = table_len(spec, 0, &len);
    if (s != ZHC_OK) return fail("zhc_lut1_new", s);
    uint16_t b1[len];
    if (!table_of(t1, b1, len))
        return fail("zhc_lut1_new", ZHC_ERR_INVALID_ARGUMENT);
    zhc_lut1 *out = NULL;
    s = zhc_lut1_new(lean_string_cstr(name), spec_block_of(spec), b1, len, &out);
    return s == ZHC_OK ? lean_io_result_mk_ok(box_lut1(out)) : fail("zhc_lut1_new", s);
}

LEAN_EXPORT lean_obj_res zhc_lean_lut2_new(b_lean_obj_arg name, b_lean_obj_arg spec, b_lean_obj_arg t1, b_lean_obj_arg t2) {
    size_t len = 0;
    zhc_status s = table_len(spec, 1, &len);
    if (s != ZHC_OK) return fail("zhc_lut2_new", s);
    uint16_t b1[len], b2[len];
    if (!table_of(t1, b1, len) || !table_of(t2, b2, len))
        return fail("zhc_lut2_new", ZHC_ERR_INVALID_ARGUMENT);
    zhc_lut2 *out = NULL;
    s = zhc_lut2_new(lean_string_cstr(name), spec_block_of(spec), b1, b2, len, &out);
    return s == ZHC_OK ? lean_io_result_mk_ok(box_lut2(out)) : fail("zhc_lut2_new", s);
}

LEAN_EXPORT lean_obj_res zhc_lean_lut4_new(b_lean_obj_arg name, b_lean_obj_arg spec, b_lean_obj_arg t1, b_lean_obj_arg t2, b_lean_obj_arg t3, b_lean_obj_arg t4) {
    size_t len = 0;
    zhc_status s = table_len(spec, 2, &len);
    if (s != ZHC_OK) return fail("zhc_lut4_new", s);
    uint16_t b1[len], b2[len], b3[len], b4[len];
    if (!table_of(t1, b1, len) || !table_of(t2, b2, len) || !table_of(t3, b3, len) || !table_of(t4, b4, len))
        return fail("zhc_lut4_new", ZHC_ERR_INVALID_ARGUMENT);
    zhc_lut4 *out = NULL;
    s = zhc_lut4_new(lean_string_cstr(name), spec_block_of(spec), b1, b2, b3, b4, len, &out);
    return s == ZHC_OK ? lean_io_result_mk_ok(box_lut4(out)) : fail("zhc_lut4_new", s);
}

LEAN_EXPORT lean_obj_res zhc_lean_lut8_new(b_lean_obj_arg name, b_lean_obj_arg spec, b_lean_obj_arg t1, b_lean_obj_arg t2, b_lean_obj_arg t3, b_lean_obj_arg t4, b_lean_obj_arg t5, b_lean_obj_arg t6, b_lean_obj_arg t7, b_lean_obj_arg t8) {
    size_t len = 0;
    zhc_status s = table_len(spec, 3, &len);
    if (s != ZHC_OK) return fail("zhc_lut8_new", s);
    uint16_t b1[len], b2[len], b3[len], b4[len], b5[len], b6[len], b7[len], b8[len];
    if (!table_of(t1, b1, len) || !table_of(t2, b2, len) || !table_of(t3, b3, len) || !table_of(t4, b4, len) ||
        !table_of(t5, b5, len) || !table_of(t6, b6, len) || !table_of(t7, b7, len) || !table_of(t8, b8, len))
        return fail("zhc_lut8_new", ZHC_ERR_INVALID_ARGUMENT);
    zhc_lut8 *out = NULL;
    s = zhc_lut8_new(lean_string_cstr(name), spec_block_of(spec), b1, b2, b3, b4, b5, b6, b7, b8, len, &out);
    return s == ZHC_OK ? lean_io_result_mk_ok(box_lut8(out)) : fail("zhc_lut8_new", s);
}

LEAN_EXPORT lean_obj_res zhc_lean_builder_new(b_lean_obj_arg spec) {
    zhc_builder *out = NULL;
    zhc_status s = zhc_builder_new(spec_block_of(spec), &out);
    return s == ZHC_OK ? lean_io_result_mk_ok(box_builder(out)) : fail("zhc_builder_new", s);
}

LEAN_EXPORT lean_obj_res zhc_lean_builder_spec(b_lean_obj_arg b) {
    zhc_spec_block *out = NULL;
    return ret_spec_block("zhc_builder_spec", zhc_builder_spec(builder_of(b), &out), &out);
}

LEAN_EXPORT lean_obj_res zhc_lean_builder_dump(b_lean_obj_arg h) {
    char *out = NULL;
    return ret_string("zhc_builder_dump", zhc_builder_dump(builder_of(h), &out), &out);
}

LEAN_EXPORT lean_obj_res zhc_lean_builder_ciphertext_integer_input(b_lean_obj_arg b, uint16_t int_size) {
    zhc_ciphertext_integer *out = NULL;
    return ret_int_ct("zhc_builder_ciphertext_integer_input", zhc_builder_ciphertext_integer_input(builder_of(b), int_size, &out), &out);
}

LEAN_EXPORT lean_obj_res zhc_lean_builder_ciphertext_integer_declare(b_lean_obj_arg b, uint16_t int_size) {
    zhc_ciphertext_integer *out = NULL;
    return ret_int_ct("zhc_builder_ciphertext_integer_declare", zhc_builder_ciphertext_integer_declare(builder_of(b), int_size, &out), &out);
}

LEAN_EXPORT lean_obj_res zhc_lean_builder_ciphertext_integer_store_block(b_lean_obj_arg b, b_lean_obj_arg ct, uint8_t index, b_lean_obj_arg block) {
    zhc_ciphertext_integer *out = NULL;
    return ret_int_ct("zhc_builder_ciphertext_integer_store_block", zhc_builder_ciphertext_integer_store_block(builder_of(b), int_ct_of(ct), index, ct_of(block), &out), &out);
}

LEAN_EXPORT lean_obj_res zhc_lean_builder_ciphertext_integer_get_block(b_lean_obj_arg b, b_lean_obj_arg ct, uint8_t index) {
    zhc_ciphertext_block *out = NULL;
    return ret_ct("zhc_builder_ciphertext_integer_get_block", zhc_builder_ciphertext_integer_get_block(builder_of(b), int_ct_of(ct), index, &out), &out);
}

LEAN_EXPORT lean_obj_res zhc_lean_builder_ciphertext_integer_output(b_lean_obj_arg b, b_lean_obj_arg ct) {
    return ret_unit("zhc_builder_ciphertext_integer_output", zhc_builder_ciphertext_integer_output(builder_of(b), int_ct_of(ct)));
}

LEAN_EXPORT lean_obj_res zhc_lean_builder_plaintext_integer_input(b_lean_obj_arg b, uint16_t int_size) {
    zhc_plaintext_integer *out = NULL;
    return ret_int_pt("zhc_builder_plaintext_integer_input", zhc_builder_plaintext_integer_input(builder_of(b), int_size, &out), &out);
}

LEAN_EXPORT lean_obj_res zhc_lean_builder_plaintext_integer_get_block(b_lean_obj_arg b, b_lean_obj_arg pt, uint8_t index) {
    zhc_plaintext_block *out = NULL;
    return ret_pt("zhc_builder_plaintext_integer_get_block", zhc_builder_plaintext_integer_get_block(builder_of(b), int_pt_of(pt), index, &out), &out);
}

LEAN_EXPORT lean_obj_res zhc_lean_builder_ciphertext_bool_input(b_lean_obj_arg b) {
    zhc_ciphertext_bool *out = NULL;
    return ret_bool("zhc_builder_ciphertext_bool_input", zhc_builder_ciphertext_bool_input(builder_of(b), &out), &out);
}

LEAN_EXPORT lean_obj_res zhc_lean_builder_ciphertext_bool_store_block(b_lean_obj_arg b, b_lean_obj_arg block) {
    zhc_ciphertext_bool *out = NULL;
    return ret_bool("zhc_builder_ciphertext_bool_store_block", zhc_builder_ciphertext_bool_store_block(builder_of(b), ct_of(block), &out), &out);
}

LEAN_EXPORT lean_obj_res zhc_lean_builder_ciphertext_bool_get_block(b_lean_obj_arg b, b_lean_obj_arg value) {
    zhc_ciphertext_block *out = NULL;
    return ret_ct("zhc_builder_ciphertext_bool_get_block", zhc_builder_ciphertext_bool_get_block(builder_of(b), bool_of(value), &out), &out);
}

LEAN_EXPORT lean_obj_res zhc_lean_builder_ciphertext_bool_output(b_lean_obj_arg b, b_lean_obj_arg value) {
    return ret_unit("zhc_builder_ciphertext_bool_output", zhc_builder_ciphertext_bool_output(builder_of(b), bool_of(value)));
}

LEAN_EXPORT lean_obj_res zhc_lean_builder_block_let_plaintext(b_lean_obj_arg b, uint8_t value) {
    zhc_plaintext_block *out = NULL;
    return ret_pt("zhc_builder_block_let_plaintext", zhc_builder_block_let_plaintext(builder_of(b), value, &out), &out);
}

LEAN_EXPORT lean_obj_res zhc_lean_builder_block_let_ciphertext(b_lean_obj_arg b, uint8_t value) {
    zhc_ciphertext_block *out = NULL;
    return ret_ct("zhc_builder_block_let_ciphertext", zhc_builder_block_let_ciphertext(builder_of(b), value, &out), &out);
}

LEAN_EXPORT lean_obj_res zhc_lean_builder_block_add(b_lean_obj_arg b, b_lean_obj_arg a, b_lean_obj_arg c) {
    zhc_ciphertext_block *out = NULL;
    return ret_ct("zhc_builder_block_add", zhc_builder_block_add(builder_of(b), ct_of(a), ct_of(c), &out), &out);
}

LEAN_EXPORT lean_obj_res zhc_lean_builder_block_sub(b_lean_obj_arg b, b_lean_obj_arg a, b_lean_obj_arg c) {
    zhc_ciphertext_block *out = NULL;
    return ret_ct("zhc_builder_block_sub", zhc_builder_block_sub(builder_of(b), ct_of(a), ct_of(c), &out), &out);
}

LEAN_EXPORT lean_obj_res zhc_lean_builder_block_shl(b_lean_obj_arg b, b_lean_obj_arg a, uint8_t amount) {
    zhc_ciphertext_block *out = NULL;
    return ret_ct("zhc_builder_block_shl", zhc_builder_block_shl(builder_of(b), ct_of(a), amount, &out), &out);
}

LEAN_EXPORT lean_obj_res zhc_lean_builder_block_mac(b_lean_obj_arg b, b_lean_obj_arg a, b_lean_obj_arg c, uint8_t mul) {
    zhc_ciphertext_block *out = NULL;
    return ret_ct("zhc_builder_block_mac", zhc_builder_block_mac(builder_of(b), ct_of(a), ct_of(c), mul, &out), &out);
}

LEAN_EXPORT lean_obj_res zhc_lean_builder_block_add_plaintext(b_lean_obj_arg b, b_lean_obj_arg a, b_lean_obj_arg c) {
    zhc_ciphertext_block *out = NULL;
    return ret_ct("zhc_builder_block_add_plaintext", zhc_builder_block_add_plaintext(builder_of(b), ct_of(a), pt_of(c), &out), &out);
}

LEAN_EXPORT lean_obj_res zhc_lean_builder_block_sub_plaintext(b_lean_obj_arg b, b_lean_obj_arg a, b_lean_obj_arg c) {
    zhc_ciphertext_block *out = NULL;
    return ret_ct("zhc_builder_block_sub_plaintext", zhc_builder_block_sub_plaintext(builder_of(b), ct_of(a), pt_of(c), &out), &out);
}

LEAN_EXPORT lean_obj_res zhc_lean_builder_block_plaintext_sub(b_lean_obj_arg b, b_lean_obj_arg a, b_lean_obj_arg c) {
    zhc_ciphertext_block *out = NULL;
    return ret_ct("zhc_builder_block_plaintext_sub", zhc_builder_block_plaintext_sub(builder_of(b), pt_of(a), ct_of(c), &out), &out);
}

LEAN_EXPORT lean_obj_res zhc_lean_builder_block_mul_plaintext(b_lean_obj_arg b, b_lean_obj_arg a, b_lean_obj_arg c) {
    zhc_ciphertext_block *out = NULL;
    return ret_ct("zhc_builder_block_mul_plaintext", zhc_builder_block_mul_plaintext(builder_of(b), ct_of(a), pt_of(c), &out), &out);
}

LEAN_EXPORT lean_obj_res zhc_lean_builder_block_lookup(b_lean_obj_arg b, b_lean_obj_arg src, b_lean_obj_arg lut) {
    zhc_ciphertext_block *out = NULL;
    return ret_ct("zhc_builder_block_lookup", zhc_builder_block_lookup(builder_of(b), ct_of(src), lut1_of(lut), &out), &out);
}

LEAN_EXPORT lean_obj_res zhc_lean_builder_block_lookup2(b_lean_obj_arg b, b_lean_obj_arg src, b_lean_obj_arg lut) {
    zhc_ciphertext_block *out[2] = {NULL, NULL};
    return ret_cts("zhc_builder_block_lookup2", zhc_builder_block_lookup2(builder_of(b), ct_of(src), lut2_of(lut), out), out, 2);
}

LEAN_EXPORT lean_obj_res zhc_lean_builder_block_lookup4(b_lean_obj_arg b, b_lean_obj_arg src, b_lean_obj_arg lut) {
    zhc_ciphertext_block *out[4] = {NULL, NULL, NULL, NULL};
    return ret_cts("zhc_builder_block_lookup4", zhc_builder_block_lookup4(builder_of(b), ct_of(src), lut4_of(lut), out), out, 4);
}

LEAN_EXPORT lean_obj_res zhc_lean_builder_block_lookup8(b_lean_obj_arg b, b_lean_obj_arg src, b_lean_obj_arg lut) {
    zhc_ciphertext_block *out[8] = {NULL, NULL, NULL, NULL, NULL, NULL, NULL, NULL};
    return ret_cts("zhc_builder_block_lookup8", zhc_builder_block_lookup8(builder_of(b), ct_of(src), lut8_of(lut), out), out, 8);
}

LEAN_EXPORT lean_obj_res zhc_lean_ciphertext_integer_spec(b_lean_obj_arg ct) {
    zhc_spec_integer *out = NULL;
    return ret_spec_integer("zhc_ciphertext_integer_spec", zhc_ciphertext_integer_spec(int_ct_of(ct), &out), &out);
}

LEAN_EXPORT lean_obj_res zhc_lean_ciphertext_integer_unbox(b_lean_obj_arg ct, size_t out) {
    void *p = take_data(ct);
    if (p != NULL) {
        *(void **)out = p;
        return ok_unit();
    }
    return ret_unit("zhc_ciphertext_integer_clone", zhc_ciphertext_integer_clone(int_ct_of(ct), (zhc_ciphertext_integer **)out));
}

LEAN_EXPORT lean_obj_res zhc_lean_ciphertext_bool_spec(b_lean_obj_arg value) {
    zhc_spec_block *out = NULL;
    return ret_spec_block("zhc_ciphertext_bool_spec", zhc_ciphertext_bool_spec(bool_of(value), &out), &out);
}

LEAN_EXPORT lean_obj_res zhc_lean_ciphertext_bool_unbox(b_lean_obj_arg value, size_t out) {
    void *p = take_data(value);
    if (p != NULL) {
        *(void **)out = p;
        return ok_unit();
    }
    return ret_unit("zhc_ciphertext_bool_clone", zhc_ciphertext_bool_clone(bool_of(value), (zhc_ciphertext_bool **)out));
}

LEAN_EXPORT lean_obj_res zhc_lean_plaintext_integer_spec(b_lean_obj_arg pt) {
    zhc_spec_integer *out = NULL;
    return ret_spec_integer("zhc_plaintext_integer_spec", zhc_plaintext_integer_spec(int_pt_of(pt), &out), &out);
}

LEAN_EXPORT lean_obj_res zhc_lean_builder_interpret(b_lean_obj_arg b, b_lean_obj_arg inputs, size_t outputs_len) {
    size_t inputs_len = lean_array_size(inputs);
    uint16_t in[inputs_len + 1], out[outputs_len + 1];
    for (size_t i = 0; i < inputs_len; i++) in[i] = (uint16_t)lean_unbox(lean_array_get_core(inputs, i));
    zhc_status s = zhc_builder_interpret(builder_of(b), in, inputs_len, out, outputs_len);
    if (s != ZHC_OK) return fail("zhc_builder_interpret", s);
    lean_object *arr = lean_alloc_array(outputs_len, outputs_len);
    for (size_t i = 0; i < outputs_len; i++) lean_array_set_core(arr, i, lean_box(out[i]));
    return lean_io_result_mk_ok(arr);
}
