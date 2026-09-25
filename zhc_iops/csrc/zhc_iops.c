#include <lean/lean.h>
#include <pthread.h>
#include <stdlib.h>
#include <string.h>

#include "zhc.h"

void lean_initialize_runtime_module(void);
void lean_initialize_thread(void);
lean_obj_res lean_io_error_to_string(lean_obj_arg err);

lean_obj_res initialize_zhc__lean_Zhc_Export(uint8_t builtin);
lean_obj_res zhc_lean_export_add_ripple_carry(uint16_t int_size);
lean_obj_res zhc_lean_export_add_hillis_steele(uint16_t int_size);
lean_obj_res zhc_lean_export_add_tree(uint16_t int_size);

static pthread_once_t g_once = PTHREAD_ONCE_INIT;
static char *g_init_error;
static _Thread_local int g_thread_ready;

static char *error_message(b_lean_obj_arg res) {
    lean_object *err = lean_io_result_get_error(res);
    lean_inc(err);
    lean_object *str = lean_io_error_to_string(err);
    char *msg = strdup(lean_string_cstr(str));
    lean_dec(str);
    return msg;
}

static void init(void) {
    lean_initialize_runtime_module();
    lean_object *res = initialize_zhc__lean_Zhc_Export(1);
    if (!lean_io_result_is_ok(res)) g_init_error = error_message(res);
    lean_dec_ref(res);
    lean_io_mark_end_initialization();
    g_thread_ready = 1;
}

static char *prepare(void) {
    pthread_once(&g_once, init);
    if (g_init_error != NULL) return strdup(g_init_error);
    if (!g_thread_ready) {
        lean_initialize_thread();
        g_thread_ready = 1;
    }
    return NULL;
}

static char *take_builder(lean_obj_arg res, zhc_builder **out) {
    if (!lean_io_result_is_ok(res)) {
        char *msg = error_message(res);
        lean_dec_ref(res);
        return msg;
    }
    lean_object *builder = lean_io_result_get_value(res);
    lean_inc(builder);
    lean_dec_ref(res);
    if (!lean_is_exclusive(builder)) {
        lean_dec(builder);
        return strdup("the emitted builder is still shared on the Lean side");
    }
    *out = (zhc_builder *)lean_get_external_data(builder);
    lean_to_external(builder)->m_data = NULL;
    lean_dec(builder);
    return NULL;
}

void zhc_iops_string_free(char *str) { free(str); }

static char *emit_int(lean_obj_res (*emit)(uint16_t), uint16_t int_size, zhc_builder **out) {
    char *err = prepare();
    if (err != NULL) return err;
    return take_builder(emit(int_size), out);
}

char *zhc_iops_add_ripple_carry(uint16_t int_size, zhc_builder **out) {
    return emit_int(zhc_lean_export_add_ripple_carry, int_size, out);
}

char *zhc_iops_add_hillis_steele(uint16_t int_size, zhc_builder **out) {
    return emit_int(zhc_lean_export_add_hillis_steele, int_size, out);
}

char *zhc_iops_add_tree(uint16_t int_size, zhc_builder **out) {
    return emit_int(zhc_lean_export_add_tree, int_size, out);
}
