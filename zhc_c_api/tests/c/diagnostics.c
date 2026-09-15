#include "zhc.h"
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <sys/stat.h>

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
    char dest[1024];
    snprintf(dest, sizeof dest, "%s/draw.html", argv[1]);

    zhc_ciphertext_block_spec spec = {2, 2};
    zhc_builder *b = NULL;
    CHECK(zhc_builder_new(spec, &b));

    zhc_integer_ciphertext *in = NULL;
    zhc_ciphertext_block *blk = NULL, *sum = NULL;
    CHECK(zhc_builder_integer_ciphertext_input(b, 4, &in));
    CHECK(zhc_builder_integer_ciphertext_get_block(b, in, 0, &blk));
    CHECK(zhc_builder_block_add(b, blk, blk, &sum));
    CHECK(zhc_builder_integer_ciphertext_output(b, in));

    char *dump = NULL, *noise = NULL, *debug = NULL, *block_debug = NULL;
    CHECK(zhc_builder_dump(b, &dump));
    CHECK(zhc_builder_dump_noise(b, &noise));
    CHECK(zhc_builder_debug(b, &debug));
    CHECK(zhc_ciphertext_block_debug(sum, &block_debug));
    if (strstr(dump, "Sig:") == NULL || strstr(dump, "add_ct") == NULL) {
        fprintf(stderr, "unexpected builder dump:\n%s\n", dump);
        return 1;
    }
    if (strstr(noise, "%") == NULL) {
        fprintf(stderr, "unexpected noise dump:\n%s\n", noise);
        return 1;
    }
    if (strlen(debug) == 0 || strlen(block_debug) == 0) return 1;
    printf("dump: %zu bytes, noise: %zu bytes, debug: %zu bytes, block: %s\n", strlen(dump),
           strlen(noise), strlen(debug), block_debug);
    zhc_string_free(dump);
    zhc_string_free(noise);
    zhc_string_free(debug);
    zhc_string_free(block_debug);
    zhc_string_free(NULL);
    CHECK(zhc_builder_check_noise(b));

    zhc_file_handle *f = NULL;
    CHECK(zhc_builder_draw(b, ZHC_IR_KIND_ORIGINAL, &f));
    remove(dest);
    CHECK(zhc_file_handle_move_to(f, dest));
    struct stat st;
    if (stat(dest, &st) != 0 || st.st_size == 0) {
        fprintf(stderr, "drawing not found at %s\n", dest);
        return 1;
    }
    printf("drawing: %lld bytes\n", (long long)st.st_size);
    if (zhc_file_handle_move_to(f, "/nonexistent_dir/x.html") != ZHC_ERR_IO) return 1;

    zhc_file_handle_free(f);
    zhc_ciphertext_block_free(blk);
    zhc_ciphertext_block_free(sum);
    zhc_integer_ciphertext_free(in);
    zhc_builder_free(b);
    printf("ok\n");
    return 0;
}
