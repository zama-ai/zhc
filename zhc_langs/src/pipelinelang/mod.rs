//! Pipeline meta-dialect for the ZHC compiler IR.
//!
//! Unlike the other dialects in this crate, which model FHE computations,
//! this dialect models the compilation flow itself: values are compilation
//! artifacts (source circuit, dialect IRs, target configurations, output
//! streams, metrics) and instructions are the compilation steps that
//! produce them. An `IR<PipelineLang>` program is thus a dataflow graph of
//! the whole compilation pipeline, which downstream crates can evaluate
//! step by step or render for inspection.
//!
//! The whole dialect is generated from the single `pipeline` definition
//! below: each `let` introduces an artifact kind of [`PipelineTypeSystem`],
//! each call introduces a step of [`PipelineInstructionSet`], and each
//! labeled block marks the [`PipelineAffinity`] of the steps it contains.
//! The resulting graph is available through [`PipelineLang::ir`], and the
//! value identifiers of every artifact through [`PipelineLang::val_ids`].

use zhc_langs_macro::gen_pipeline_lang;

#[gen_pipeline_lang]
pub fn pipeline() {
    'commons: {
        let unchecked_ioplang = input_unchecked_ioplang();
        let partitions = input_partitions();
        let prototype = input_prototype();
        let ciphertext_block_spec = input_ciphertext_block_spec();
        let slack_drawing = draw_slack(unchecked_ioplang);
        let pbs_metrics = compute_pbs_metrics(unchecked_ioplang);
        let ioplang = check_ioplang(unchecked_ioplang, ciphertext_block_spec);
        let fingerprint = compute_fingerprint(ioplang);
        let lut_registry = ioplang_to_lut_registry(ioplang);
    }
    'hpu: {
        let hpu_lut_relocation = input_hpu_lut_relocation();
        let hpu_config = input_hpu_config();
        let hpulang_translated = ioplang_to_hpulang(ioplang);
        let hpulang_scheduled = schedule_hpulang(hpulang_translated, hpu_config);
        let doplang = allocate_doplang(hpulang_scheduled, hpu_config, lut_registry);
        let hpu_stream = generate_hpu_stream(doplang, hpu_lut_relocation);
        let hpu_trace = trace_hpu_execution(doplang, hpu_config);
        let hpu_metrics = compute_hpu_metrics(doplang, hpulang_scheduled);
        let hpu_assembly = generate_hpu_assembly(doplang, lut_registry, prototype);
    }
    'multi_hpu: {
        let multi_hpu_lut_relocation = input_multi_hpu_lut_relocation();
        let multi_hpu_config = input_multi_hpu_config();
        let (multi_hpulang_translated, multi_hpu_localities) =
            ioplang_to_multi_hpu(ioplang, partitions);
        let multi_hpulang_scheduled = schedule_multi_hpulang(
            multi_hpulang_translated,
            multi_hpu_localities,
            multi_hpu_config,
        );
        let multi_doplang =
            allocate_multi_doplang(multi_hpulang_scheduled, multi_hpu_config, lut_registry);
        let multi_hpu_metrics = compute_multi_hpu_metrics(multi_doplang, multi_hpu_config);
        let multi_hpu_trace = trace_multi_hpu_execution(multi_doplang, multi_hpu_config);
        let multi_hpu_stream = generate_multi_hpu_stream(multi_doplang, multi_hpu_lut_relocation);
        let multi_hpu_assembly =
            generate_multi_hpu_assembly(multi_doplang, lut_registry, prototype);
    }
    'vm: {
        let vm_config = input_vm_config();
        let topology = input_topology();
        let vmlang = ioplang_to_vmlang(ioplang);
        let vm_execution_plan =
            generate_vm_execution_plan(vmlang, vm_config, topology, lut_registry);
    }
}
