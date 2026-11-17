#include "zhc.h"
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#define CHECK(call)                                                                        \
    do {                                                                                   \
        zhc_status s_ = (call);                                                            \
        if (s_ != ZHC_OK) {                                                                \
            fprintf(stderr, "%s failed with status %d\n", #call, (int)s_);                 \
            exit(1);                                                                       \
        }                                                                                  \
    } while (0)

/* argv[1] is a writable scratch directory. */
int main(int argc, char **argv) {
    if (argc < 2) {
        fprintf(stderr, "usage: %s <scratch-dir>\n", argv[0]);
        return 2;
    }

    zhc_spec_block *spec = NULL;
    CHECK(zhc_spec_block_new(2, 2, &spec));
    zhc_builder *b = NULL;
    CHECK(zhc_builder_new(spec, &b));
    zhc_spec_block_free(spec);

    zhc_ciphertext_integer *in = NULL;
    zhc_ciphertext_block *blk = NULL, *sum = NULL;
    CHECK(zhc_builder_ciphertext_integer_input(b, 4, &in));
    CHECK(zhc_builder_ciphertext_integer_get_block(b, in, 0, &blk));
    CHECK(zhc_builder_block_add(b, blk, blk, &sum));
    CHECK(zhc_builder_ciphertext_integer_output(b, in));

    char *dump = NULL;
    CHECK(zhc_builder_dump(b, &dump));
    if (strstr(dump, "Sig:") == NULL || strstr(dump, "add_ct") == NULL) {
        fprintf(stderr, "unexpected builder dump:\n%s\n", dump);
        return 1;
    }
    printf("dump: %zu bytes\n", strlen(dump));
    zhc_string_free(dump);
    zhc_string_free(NULL);

    zhc_ciphertext_block_free(blk);
    zhc_ciphertext_block_free(sum);
    zhc_ciphertext_integer_free(in);
    zhc_builder_free(b);
    printf("ok\n");
    return 0;
}
