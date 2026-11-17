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

    zhc_ciphertext_integer *in = NULL, *out = NULL;
    CHECK(zhc_builder_ciphertext_integer_input(b, 8, &in));
    CHECK(zhc_builder_ciphertext_integer_declare(b, 8, &out));
    for (uint8_t i = 0; i < 4; i++) {
        zhc_ciphertext_block *blk = NULL, *dbl = NULL;
        zhc_ciphertext_integer *next = NULL;
        CHECK(zhc_builder_ciphertext_integer_get_block(b, in, i, &blk));
        CHECK(zhc_builder_block_add(b, blk, blk, &dbl));
        CHECK(zhc_builder_ciphertext_integer_store_block(b, out, i, dbl, &next));
        zhc_ciphertext_integer_free(out);
        out = next;
        zhc_ciphertext_block_free(blk);
        zhc_ciphertext_block_free(dbl);
    }
    CHECK(zhc_builder_ciphertext_integer_output(b, out));
    zhc_ciphertext_bool *flag = NULL;
    CHECK(zhc_builder_ciphertext_bool_input(b, &flag));
    CHECK(zhc_builder_ciphertext_bool_output(b, flag));

    uint16_t inputs[5] = {3, 1, 0, 2, 1};
    uint16_t outputs[5] = {0};
    CHECK(zhc_builder_interpret(b, inputs, 5, outputs, 5));
    printf("outputs: %u %u %u %u %u\n", outputs[0], outputs[1], outputs[2], outputs[3], outputs[4]);
    if (outputs[0] != 6 || outputs[1] != 2 || outputs[2] != 0 || outputs[3] != 4 || outputs[4] != 1)
        return 1;

    zhc_ciphertext_integer_free(in);
    zhc_ciphertext_integer_free(out);
    zhc_ciphertext_bool_free(flag);
    zhc_builder_free(b);
    printf("ok\n");
    return 0;
}
