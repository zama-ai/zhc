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
    zhc_spec_block *spec = NULL;
    CHECK(zhc_spec_block_new(2, 2, &spec));
    zhc_builder *b = NULL;
    CHECK(zhc_builder_new(spec, &b));
    zhc_spec_block_free(spec);

    zhc_spec_block *got = NULL;
    uint8_t carry = 0, message = 0;
    CHECK(zhc_builder_spec(b, &got));
    CHECK(zhc_spec_block_carry(got, &carry));
    CHECK(zhc_spec_block_message(got, &message));
    zhc_spec_block_free(got);
    printf("spec: carry=%u message=%u\n", carry, message);

    zhc_ciphertext_integer *in = NULL, *out = NULL;
    zhc_plaintext_block *one = NULL;
    CHECK(zhc_builder_ciphertext_integer_input(b, 8, &in));
    CHECK(zhc_builder_ciphertext_integer_declare(b, 8, &out));
    CHECK(zhc_builder_block_let_plaintext(b, 1, &one));
    for (uint8_t i = 0; i < 4; i++) {
        zhc_ciphertext_block *blk = NULL, *dbl = NULL, *inc = NULL;
        zhc_ciphertext_integer *next = NULL;
        CHECK(zhc_builder_ciphertext_integer_get_block(b, in, i, &blk));
        CHECK(zhc_builder_block_add(b, blk, blk, &dbl));
        CHECK(zhc_builder_block_add_plaintext(b, dbl, one, &inc));
        CHECK(zhc_builder_ciphertext_integer_store_block(b, out, i, inc, &next));
        zhc_ciphertext_integer_free(out);
        out = next;
        zhc_ciphertext_block_free(blk);
        zhc_ciphertext_block_free(dbl);
        zhc_ciphertext_block_free(inc);
    }
    CHECK(zhc_builder_ciphertext_integer_output(b, out));

    /* Error paths. */
    zhc_plaintext_block *bad = NULL;
    zhc_status s = zhc_builder_block_let_plaintext(NULL, 1, &bad);
    printf("null builder: status=%d\n", (int)s);
    if (s != ZHC_ERR_NULL_POINTER) return 1;

    s = zhc_builder_block_let_plaintext(b, 1, NULL);
    printf("null out: status=%d\n", (int)s);
    if (s != ZHC_ERR_NULL_POINTER) return 1;

    zhc_plaintext_block_free(one);
    zhc_ciphertext_integer_free(in);
    zhc_ciphertext_integer_free(out);
    zhc_builder_free(b);
    printf("ok\n");
    return 0;
}
