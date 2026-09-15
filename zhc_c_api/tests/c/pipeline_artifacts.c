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

static long long file_size(const char *path) {
    struct stat st;
    return stat(path, &st) == 0 ? (long long)st.st_size : -1;
}

/* argv[1] is a writable scratch directory. */
int main(int argc, char **argv) {
    if (argc < 2) {
        fprintf(stderr, "usage: %s <scratch-dir>\n", argv[0]);
        return 2;
    }
    char asm_path[1024], slack_path[1024], state_path[1024];
    snprintf(asm_path, sizeof asm_path, "%s/hpu.asm", argv[1]);
    snprintf(slack_path, sizeof slack_path, "%s/slack.html", argv[1]);
    snprintf(state_path, sizeof state_path, "%s/state.html", argv[1]);

    /* A small circuit with one bootstrapping so the HPU flow has real work to do. */
    zhc_ciphertext_block_spec spec = {2, 2};
    zhc_builder *b = NULL;
    CHECK(zhc_builder_new(spec, &b));
    zhc_integer_ciphertext *in = NULL, *out = NULL, *next = NULL;
    zhc_ciphertext_block *blk = NULL, *sum = NULL, *clean = NULL;
    uint16_t msg_only[16];
    for (int i = 0; i < 16; i++) msg_only[i] = (uint16_t)(i & 0x3);
    zhc_lut1 *lut = NULL;
    CHECK(zhc_lut1_new("msg_only", spec, msg_only, 16, &lut));
    CHECK(zhc_builder_integer_ciphertext_input(b, 4, &in));
    CHECK(zhc_builder_integer_ciphertext_declare(b, 4, &out));
    for (uint8_t i = 0; i < 2; i++) {
        CHECK(zhc_builder_integer_ciphertext_get_block(b, in, i, &blk));
        CHECK(zhc_builder_block_add(b, blk, blk, &sum));
        CHECK(zhc_builder_block_lookup(b, sum, lut, &clean));
        CHECK(zhc_builder_integer_ciphertext_store_block(b, out, i, clean, &next));
        zhc_integer_ciphertext_free(out);
        out = next;
        zhc_ciphertext_block_free(blk);
        zhc_ciphertext_block_free(sum);
        zhc_ciphertext_block_free(clean);
    }
    CHECK(zhc_builder_integer_ciphertext_output(b, out));

    zhc_hpu_config hpu;
    CHECK(zhc_hpu_config_default(&hpu));
    zhc_pipeline *p = NULL;
    CHECK(zhc_pipeline_new(&p));
    CHECK(zhc_pipeline_set_builder(p, b));
    CHECK(zhc_pipeline_set_hpu_config(p, hpu));

    zhc_ciphertext_block_spec got;
    CHECK(zhc_pipeline_get_ciphertext_block_spec(p, &got));
    if (got.carry != 2 || got.message != 2) return 1;
    zhc_hpu_config hpu_back;
    CHECK(zhc_pipeline_get_hpu_config(p, &hpu_back));
    if (hpu_back.freq != hpu.freq || hpu_back.heap_size != hpu.heap_size) return 1;

    uint64_t fp = 0;
    CHECK(zhc_pipeline_get_fingerprint(p, &fp));
    printf("fingerprint: %llu\n", (unsigned long long)fp);

    zhc_hpu_metrics *m = NULL;
    CHECK(zhc_pipeline_get_hpu_metrics(p, &m));
    double latency = 0.0, lower_bound = 0.0;
    size_t batches = 0, filled = 0, total = 0;
    CHECK(zhc_hpu_metrics_latency(m, &latency));
    CHECK(zhc_hpu_metrics_lower_bound(m, &lower_bound));
    CHECK(zhc_hpu_metrics_batch_count(m, &batches));
    CHECK(zhc_hpu_metrics_slots_filled(m, &filled));
    CHECK(zhc_hpu_metrics_slots_total(m, &total));
    printf("latency=%.2f lower_bound=%.2f batches=%zu slots=%zu/%zu\n", latency, lower_bound,
           batches, filled, total);
    if (!(latency > 0.0) || batches == 0) return 1;

    zhc_pbs_metrics *pbs = NULL;
    CHECK(zhc_pipeline_get_pbs_metrics(p, &pbs));
    size_t pbs_count = 0;
    CHECK(zhc_pbs_metrics_count(pbs, &pbs_count));
    if (pbs_count != 2) {
        fprintf(stderr, "expected 2 bootstrappings, got %zu\n", pbs_count);
        return 1;
    }

    char *m_dump = NULL, *m_debug = NULL, *pbs_dump = NULL, *p_debug = NULL;
    CHECK(zhc_hpu_metrics_dump(m, &m_dump));
    CHECK(zhc_hpu_metrics_debug(m, &m_debug));
    CHECK(zhc_pbs_metrics_dump(pbs, &pbs_dump));
    CHECK(zhc_pipeline_debug(p, &p_debug));
    if (strstr(m_dump, "HPU Metrics") == NULL || strstr(m_debug, "latency") == NULL ||
        strlen(pbs_dump) == 0 || strstr(p_debug, "Pipeline") == NULL) {
        fprintf(stderr, "unexpected dumps:\n%s\n%s\n%s\n%s\n", m_dump, m_debug, pbs_dump, p_debug);
        return 1;
    }
    printf("metrics dump: %zu bytes, pbs dump: %zu bytes\n", strlen(m_dump), strlen(pbs_dump));
    zhc_string_free(m_dump);
    zhc_string_free(m_debug);
    zhc_string_free(pbs_dump);
    zhc_string_free(p_debug);
    zhc_hpu_metrics_free(m);
    zhc_pbs_metrics_free(pbs);

    zhc_file_handle *asm_file = NULL, *slack = NULL, *state = NULL;
    zhc_perfetto_trace *trace = NULL;
    CHECK(zhc_pipeline_get_hpu_assembly(p, &asm_file));
    CHECK(zhc_pipeline_get_slack_drawing(p, &slack));
    CHECK(zhc_pipeline_draw_state(p, &state));
    CHECK(zhc_pipeline_get_hpu_trace(p, &trace));
    CHECK(zhc_file_handle_move_to(asm_file, asm_path));
    CHECK(zhc_file_handle_move_to(slack, slack_path));
    CHECK(zhc_file_handle_move_to(state, state_path));
    printf("assembly=%lld slack=%lld state=%lld bytes\n", file_size(asm_path),
           file_size(slack_path), file_size(state_path));
    if (file_size(asm_path) <= 0 || file_size(slack_path) <= 0 || file_size(state_path) <= 0)
        return 1;

    /* Wrong flow: the multi-HPU getters panic when only a single-HPU config is set. */
    zhc_multi_hpu_metrics *mm = NULL;
    zhc_status s = zhc_pipeline_get_multi_hpu_metrics(p, &mm);
    printf("multi metrics on single-hpu pipeline: status=%d\n", (int)s);
    if (s != ZHC_ERR_PANIC || mm != NULL) return 1;

    zhc_perfetto_trace_free(trace);
    zhc_file_handle_free(asm_file);
    zhc_file_handle_free(slack);
    zhc_file_handle_free(state);
    zhc_pipeline_free(p);
    zhc_lut1_free(lut);
    zhc_integer_ciphertext_free(in);
    zhc_integer_ciphertext_free(out);
    zhc_builder_free(b);
    printf("ok\n");
    return 0;
}
