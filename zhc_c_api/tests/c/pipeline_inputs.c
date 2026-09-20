#include "zhc.h"
#include <stdio.h>
#include <stdlib.h>

#define CHECK(call)                                                                        \
    do {                                                                                   \
        zhc_status s_ = (call);                                                            \
        if (s_ != ZHC_OK) {                                                                \
            fprintf(stderr, "%s failed with status %d\n", #call, (int)s_);                 \
            exit(1);                                                                       \
        }                                                                                  \
    } while (0)

int main(void) {
    zhc_ciphertext_block_spec spec = {2, 2};
    zhc_builder *b = NULL;
    CHECK(zhc_builder_new(spec, &b));
    zhc_integer_ciphertext *in = NULL;
    CHECK(zhc_builder_integer_ciphertext_input(b, 8, &in));
    CHECK(zhc_builder_integer_ciphertext_output(b, in));

    zhc_hpu_config hpu;
    CHECK(zhc_hpu_config_default(&hpu));
    printf("default hpu: freq=%zu regf_size=%zu heap_size=%zu\n", hpu.freq, hpu.regf_size,
           hpu.heap_size);
    zhc_multi_hpu_config mhpu;
    CHECK(zhc_multi_hpu_config_default(&mhpu));
    printf("default multi hpu: n_hpus=%u freq=%zu\n", mhpu.n_hpus, mhpu.hpu_config.freq);
    if (mhpu.hpu_config.freq != hpu.freq) return 1;

    zhc_pipeline *p = NULL;
    CHECK(zhc_pipeline_new(&p));
    CHECK(zhc_pipeline_set_builder(p, b));
    CHECK(zhc_pipeline_set_hpu_config(p, hpu));
    CHECK(zhc_pipeline_set_legacy_hpu_scheduler(p));
    CHECK(zhc_pipeline_set_trace_hpu_events(p));

    /* The three target configs are mutually exclusive in Rust; the assert becomes a panic. */
    zhc_status s = zhc_pipeline_set_multi_hpu_config(p, mhpu);
    printf("second target config: status=%d\n", (int)s);
    if (s != ZHC_ERR_PANIC) return 1;

    if (zhc_pipeline_set_builder(NULL, b) != ZHC_ERR_NULL_POINTER) return 1;
    if (zhc_pipeline_set_builder(p, NULL) != ZHC_ERR_NULL_POINTER) return 1;

    zhc_pipeline_free(p);
    zhc_integer_ciphertext_free(in);
    zhc_builder_free(b);
    printf("ok\n");
    return 0;
}
