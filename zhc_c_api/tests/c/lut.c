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

#define EXPECT(call, want)                                                                 \
    do {                                                                                   \
        zhc_status s_ = (call);                                                            \
        if (s_ != (want)) {                                                                \
            fprintf(stderr, "%s: got %d want %d\n", #call, (int)s_, (int)(want));          \
            exit(1);                                                                       \
        }                                                                                  \
    } while (0)

int main(void) {
    /* 2 carry bits, 2 message bits: data size is 4, data space has 16 values. */
    zhc_ciphertext_block_spec spec = {2, 2};

    /* Lut1: keep the message bits only. */
    uint16_t msg_only[16];
    for (int i = 0; i < 16; i++) msg_only[i] = (uint16_t)(i & 0x3);
    zhc_lut1 *lut1 = NULL;
    CHECK(zhc_lut1_new("msg_only", spec, msg_only, 16, &lut1));

    /* Lut2: two sub-tables of 8 entries each. */
    uint16_t plus1[8], minus1[8];
    for (int i = 0; i < 8; i++) {
        plus1[i] = (uint16_t)((i + 1) & 0x3);
        minus1[i] = (uint16_t)((i - 1) & 0x3);
    }
    zhc_lut2 *lut2 = NULL;
    CHECK(zhc_lut2_new("plus_minus", spec, plus1, minus1, 8, &lut2));

    /* Lut4 and Lut8: identity on the reduced input space. */
    uint16_t id4[4] = {0, 1, 2, 3};
    uint16_t id8[2] = {0, 1};
    zhc_lut4 *lut4 = NULL;
    zhc_lut8 *lut8 = NULL;
    CHECK(zhc_lut4_new("id4", spec, id4, id4, id4, id4, 4, &lut4));
    CHECK(zhc_lut8_new("id8", spec, id8, id8, id8, id8, id8, id8, id8, id8, 2, &lut8));

    /* Rejections. */
    zhc_lut1 *bad = NULL;
    EXPECT(zhc_lut1_new("bad_len", spec, msg_only, 8, &bad), ZHC_ERR_INVALID_ARGUMENT);
    /* Entries may set the padding bit (16), as the builtin padding tables do, but must still
       fit the complete width: at this spec that is 5 bits, so 31 is accepted and 32 is not. */
    uint16_t padding_entry[16] = {16, 31};
    zhc_lut1 *padded = NULL;
    CHECK(zhc_lut1_new("padding_entry", spec, padding_entry, 16, &padded));
    zhc_lut1_free(padded);
    uint16_t too_big[16] = {32};
    EXPECT(zhc_lut1_new("bad_val", spec, too_big, 16, &bad), ZHC_ERR_INVALID_ARGUMENT);
    EXPECT(zhc_lut1_new("null_tbl", spec, NULL, 16, &bad), ZHC_ERR_NULL_POINTER);
    EXPECT(zhc_lut1_new(NULL, spec, msg_only, 16, &bad), ZHC_ERR_NULL_POINTER);
    if (bad != NULL) return 1;

    /* Use the LUTs in a circuit. */
    zhc_builder *b = NULL;
    CHECK(zhc_builder_new(spec, &b));
    zhc_integer_ciphertext *in = NULL;
    zhc_ciphertext_block *blk = NULL, *r1 = NULL, *r2[2] = {NULL, NULL}, *r4[4], *r8[8];
    CHECK(zhc_builder_integer_ciphertext_input(b, 8, &in));
    CHECK(zhc_builder_integer_ciphertext_get_block(b, in, 0, &blk));
    CHECK(zhc_builder_block_lookup(b, blk, lut1, &r1));
    CHECK(zhc_builder_block_lookup2(b, blk, lut2, r2));
    CHECK(zhc_builder_block_lookup4(b, blk, lut4, r4));
    CHECK(zhc_builder_block_lookup8(b, blk, lut8, r8));
    for (int i = 0; i < 2; i++) if (r2[i] == NULL) return 1;
    for (int i = 0; i < 4; i++) if (r4[i] == NULL) return 1;
    for (int i = 0; i < 8; i++) if (r8[i] == NULL) return 1;

    zhc_ciphertext_block_free(r1);
    for (int i = 0; i < 2; i++) zhc_ciphertext_block_free(r2[i]);
    for (int i = 0; i < 4; i++) zhc_ciphertext_block_free(r4[i]);
    for (int i = 0; i < 8; i++) zhc_ciphertext_block_free(r8[i]);
    zhc_ciphertext_block_free(blk);
    zhc_integer_ciphertext_free(in);
    zhc_builder_free(b);
    zhc_lut1_free(lut1);
    zhc_lut2_free(lut2);
    zhc_lut4_free(lut4);
    zhc_lut8_free(lut8);
    printf("ok\n");
    return 0;
}
