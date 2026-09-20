#include <lean/lean.h>
#include <pthread.h>
#include <stdlib.h>
#include <string.h>

void lean_initialize_runtime_module(void);
void lean_initialize_thread(void);
lean_obj_res lean_io_error_to_string(lean_obj_arg err);
lean_obj_res initialize_zhc__lean_Zhc_Ffi_R2l_Export(uint8_t builtin);

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
    lean_set_exit_on_panic(true);
    lean_object *res = initialize_zhc__lean_Zhc_Ffi_R2l_Export(1);
    if (!lean_io_result_is_ok(res)) g_init_error = error_message(res);
    lean_dec_ref(res);
    lean_io_mark_end_initialization();
    g_thread_ready = 1;
}

char *zhc_iops_prepare(void) {
    pthread_once(&g_once, init);
    if (g_init_error != NULL) return strdup(g_init_error);
    if (!g_thread_ready) {
        lean_initialize_thread();
        g_thread_ready = 1;
    }
    return NULL;
}

char *zhc_iops_result(lean_obj_arg res) {
    char *msg = lean_io_result_is_ok(res) ? NULL : error_message(res);
    lean_dec_ref(res);
    return msg;
}

void zhc_iops_string_free(char *str) {
    free(str);
}
