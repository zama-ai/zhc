/* C ABI for ZHC. */


#ifndef ZHC_H
#define ZHC_H

#include <stdbool.h>
#include <stddef.h>
#include <stdint.h>

typedef enum {
    ZHC_OK = 0,
    ZHC_ERR_NULL_POINTER = 1,
    ZHC_ERR_INVALID_ARGUMENT = 2,
} zhc_status;

typedef struct zhc_builder zhc_builder;

typedef struct zhc_ciphertext_block zhc_ciphertext_block;

typedef struct zhc_ciphertext_bool zhc_ciphertext_bool;

typedef struct zhc_ciphertext_integer zhc_ciphertext_integer;

typedef struct zhc_lut1 zhc_lut1;

typedef struct zhc_lut2 zhc_lut2;

typedef struct zhc_lut4 zhc_lut4;

typedef struct zhc_lut8 zhc_lut8;

typedef struct zhc_plaintext_block zhc_plaintext_block;

typedef struct zhc_plaintext_integer zhc_plaintext_integer;

typedef struct zhc_spec_block zhc_spec_block;

typedef struct zhc_spec_integer zhc_spec_integer;

#ifdef __cplusplus
extern "C" {
#endif // __cplusplus

zhc_status zhc_builder_new(const zhc_spec_block *spec, zhc_builder **out);

void zhc_builder_free(zhc_builder *builder);

zhc_status zhc_builder_spec(const zhc_builder *builder, zhc_spec_block **out);

zhc_status zhc_builder_dump(const zhc_builder *builder, char **out);

zhc_status zhc_builder_ciphertext_integer_input(const zhc_builder *builder,
                                                uint16_t int_size,
                                                zhc_ciphertext_integer **out);

zhc_status zhc_builder_ciphertext_integer_declare(const zhc_builder *builder,
                                                  uint16_t int_size,
                                                  zhc_ciphertext_integer **out);

zhc_status zhc_builder_ciphertext_integer_store_block(const zhc_builder *builder,
                                                      const zhc_ciphertext_integer *ct,
                                                      uint8_t index,
                                                      const zhc_ciphertext_block *block,
                                                      zhc_ciphertext_integer **out);

zhc_status zhc_builder_ciphertext_integer_get_block(const zhc_builder *builder,
                                                    const zhc_ciphertext_integer *ct,
                                                    uint8_t index,
                                                    zhc_ciphertext_block **out);

zhc_status zhc_builder_ciphertext_integer_output(const zhc_builder *builder,
                                                 const zhc_ciphertext_integer *ct);

zhc_status zhc_builder_plaintext_integer_input(const zhc_builder *builder,
                                               uint16_t int_size,
                                               zhc_plaintext_integer **out);

zhc_status zhc_builder_plaintext_integer_get_block(const zhc_builder *builder,
                                                   const zhc_plaintext_integer *pt,
                                                   uint8_t index,
                                                   zhc_plaintext_block **out);

zhc_status zhc_builder_ciphertext_bool_input(const zhc_builder *builder, zhc_ciphertext_bool **out);

zhc_status zhc_builder_ciphertext_bool_store_block(const zhc_builder *builder,
                                                   const zhc_ciphertext_block *block,
                                                   zhc_ciphertext_bool **out);

zhc_status zhc_builder_ciphertext_bool_get_block(const zhc_builder *builder,
                                                 const zhc_ciphertext_bool *value,
                                                 zhc_ciphertext_block **out);

zhc_status zhc_builder_ciphertext_bool_output(const zhc_builder *builder,
                                              const zhc_ciphertext_bool *value);

zhc_status zhc_builder_block_let_plaintext(const zhc_builder *builder,
                                           uint8_t value,
                                           zhc_plaintext_block **out);

zhc_status zhc_builder_block_let_ciphertext(const zhc_builder *builder,
                                            uint8_t value,
                                            zhc_ciphertext_block **out);

zhc_status zhc_builder_block_add(const zhc_builder *builder,
                                 const zhc_ciphertext_block *src_a,
                                 const zhc_ciphertext_block *src_b,
                                 zhc_ciphertext_block **out);

zhc_status zhc_builder_block_sub(const zhc_builder *builder,
                                 const zhc_ciphertext_block *src_a,
                                 const zhc_ciphertext_block *src_b,
                                 zhc_ciphertext_block **out);

zhc_status zhc_builder_block_shl(const zhc_builder *builder,
                                 const zhc_ciphertext_block *src,
                                 uint8_t amount,
                                 zhc_ciphertext_block **out);

zhc_status zhc_builder_block_mac(const zhc_builder *builder,
                                 const zhc_ciphertext_block *src_a,
                                 const zhc_ciphertext_block *src_b,
                                 uint8_t mul,
                                 zhc_ciphertext_block **out);

zhc_status zhc_builder_block_add_plaintext(const zhc_builder *builder,
                                           const zhc_ciphertext_block *src_a,
                                           const zhc_plaintext_block *src_b,
                                           zhc_ciphertext_block **out);

zhc_status zhc_builder_block_sub_plaintext(const zhc_builder *builder,
                                           const zhc_ciphertext_block *src_a,
                                           const zhc_plaintext_block *src_b,
                                           zhc_ciphertext_block **out);

zhc_status zhc_builder_block_plaintext_sub(const zhc_builder *builder,
                                           const zhc_plaintext_block *src_a,
                                           const zhc_ciphertext_block *src_b,
                                           zhc_ciphertext_block **out);

zhc_status zhc_builder_block_mul_plaintext(const zhc_builder *builder,
                                           const zhc_ciphertext_block *src_a,
                                           const zhc_plaintext_block *src_b,
                                           zhc_ciphertext_block **out);

zhc_status zhc_builder_block_lookup(const zhc_builder *builder,
                                    const zhc_ciphertext_block *src,
                                    const zhc_lut1 *lut,
                                    zhc_ciphertext_block **out);

zhc_status zhc_builder_block_lookup2(const zhc_builder *builder,
                                     const zhc_ciphertext_block *src,
                                     const zhc_lut2 *lut,
                                     zhc_ciphertext_block **out);

zhc_status zhc_builder_block_lookup4(const zhc_builder *builder,
                                     const zhc_ciphertext_block *src,
                                     const zhc_lut4 *lut,
                                     zhc_ciphertext_block **out);

zhc_status zhc_builder_block_lookup8(const zhc_builder *builder,
                                     const zhc_ciphertext_block *src,
                                     const zhc_lut8 *lut,
                                     zhc_ciphertext_block **out);

zhc_status zhc_builder_interpret(const zhc_builder *builder,
                                 const uint16_t *inputs,
                                 size_t inputs_len,
                                 uint16_t *outputs,
                                 size_t outputs_len);

void zhc_ciphertext_block_free(zhc_ciphertext_block *ptr);

void zhc_ciphertext_bool_free(zhc_ciphertext_bool *ptr);

zhc_status zhc_ciphertext_bool_clone(const zhc_ciphertext_bool *value, zhc_ciphertext_bool **out);

zhc_status zhc_ciphertext_bool_spec(const zhc_ciphertext_bool *value, zhc_spec_block **out);

void zhc_ciphertext_integer_free(zhc_ciphertext_integer *ptr);

zhc_status zhc_ciphertext_integer_clone(const zhc_ciphertext_integer *ct,
                                        zhc_ciphertext_integer **out);

zhc_status zhc_ciphertext_integer_spec(const zhc_ciphertext_integer *ct, zhc_spec_integer **out);

void zhc_lut1_free(zhc_lut1 *ptr);

zhc_status zhc_lut1_new(const char *name,
                        const zhc_spec_block *spec,
                        const uint16_t *table_1,
                        size_t len,
                        zhc_lut1 **out);

void zhc_lut2_free(zhc_lut2 *ptr);

zhc_status zhc_lut2_new(const char *name,
                        const zhc_spec_block *spec,
                        const uint16_t *table_1,
                        const uint16_t *table_2,
                        size_t len,
                        zhc_lut2 **out);

void zhc_lut4_free(zhc_lut4 *ptr);

zhc_status zhc_lut4_new(const char *name,
                        const zhc_spec_block *spec,
                        const uint16_t *table_1,
                        const uint16_t *table_2,
                        const uint16_t *table_3,
                        const uint16_t *table_4,
                        size_t len,
                        zhc_lut4 **out);

void zhc_lut8_free(zhc_lut8 *ptr);

zhc_status zhc_lut8_new(const char *name,
                        const zhc_spec_block *spec,
                        const uint16_t *table_1,
                        const uint16_t *table_2,
                        const uint16_t *table_3,
                        const uint16_t *table_4,
                        const uint16_t *table_5,
                        const uint16_t *table_6,
                        const uint16_t *table_7,
                        const uint16_t *table_8,
                        size_t len,
                        zhc_lut8 **out);

void zhc_plaintext_block_free(zhc_plaintext_block *ptr);

void zhc_plaintext_integer_free(zhc_plaintext_integer *ptr);

zhc_status zhc_plaintext_integer_spec(const zhc_plaintext_integer *pt, zhc_spec_integer **out);

zhc_status zhc_spec_block_new(uint8_t carry, uint8_t message, zhc_spec_block **out);

void zhc_spec_block_free(zhc_spec_block *ptr);

zhc_status zhc_spec_block_carry(const zhc_spec_block *spec, uint8_t *out);

zhc_status zhc_spec_block_message(const zhc_spec_block *spec, uint8_t *out);

void zhc_spec_integer_free(zhc_spec_integer *ptr);

zhc_status zhc_spec_integer_int_size(const zhc_spec_integer *spec, uint16_t *out);

zhc_status zhc_spec_integer_block(const zhc_spec_integer *spec, zhc_spec_block **out);

void zhc_string_free(char *ptr);

#ifdef __cplusplus
}  // extern "C"
#endif  // __cplusplus

#endif  /* ZHC_H */
