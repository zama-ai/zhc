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

    zhc_ciphertext_block_spec got;
    CHECK(zhc_builder_spec(b, &got));
    printf("spec: carry=%u message=%u\n", got.carry, got.message);

    CHECK(zhc_builder_push_comment(b, "smoke"));
    zhc_integer_ciphertext *in = NULL, *out = NULL;
    zhc_plaintext_block *one = NULL;
    CHECK(zhc_builder_integer_ciphertext_input(b, 8, &in));
    CHECK(zhc_builder_integer_ciphertext_declare(b, 8, &out));
    CHECK(zhc_builder_block_let_plaintext(b, 1, &one));
    for (uint8_t i = 0; i < 4; i++) {
        zhc_ciphertext_block *blk = NULL, *dbl = NULL, *inc = NULL;
        zhc_integer_ciphertext *next = NULL;
        CHECK(zhc_builder_integer_ciphertext_get_block(b, in, i, &blk));
        CHECK(zhc_builder_block_add_with(b, blk, blk, ZHC_FLAVOR_WRAPPING, &dbl));
        CHECK(zhc_builder_block_add_plaintext(b, dbl, one, &inc));
        CHECK(zhc_builder_integer_ciphertext_store_block(b, out, i, inc, &next));
        zhc_integer_ciphertext_free(out);
        out = next;
        zhc_ciphertext_block_free(blk);
        zhc_ciphertext_block_free(dbl);
        zhc_ciphertext_block_free(inc);
    }
    CHECK(zhc_builder_integer_ciphertext_output(b, out));
    CHECK(zhc_builder_pop_comment(b));

    /* Error paths. */
    zhc_plaintext_block *bad = NULL;
    zhc_status s = zhc_builder_block_let_plaintext(b, 255, &bad);
    printf("panic path: status=%d out=%p\n", (int)s, (void *)bad);
    if (s != ZHC_ERR_PANIC || bad != NULL) return 1;

    s = zhc_builder_block_let_plaintext(NULL, 1, &bad);
    printf("null builder: status=%d\n", (int)s);
    if (s != ZHC_ERR_NULL_POINTER) return 1;

    s = zhc_builder_block_let_plaintext(b, 1, NULL);
    printf("null out: status=%d\n", (int)s);
    if (s != ZHC_ERR_NULL_POINTER) return 1;

    /* The builder is still usable after a caught panic. */
    zhc_ciphertext_block *zero = NULL;
    CHECK(zhc_builder_block_let_ciphertext(b, 0, &zero));
    zhc_ciphertext_block_free(zero);

    zhc_plaintext_block_free(one);
    zhc_integer_ciphertext_free(in);
    zhc_integer_ciphertext_free(out);
    zhc_builder_free(b);
    printf("ok\n");
    return 0;
}
