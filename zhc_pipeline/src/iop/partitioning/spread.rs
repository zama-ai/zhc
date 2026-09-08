use zhc_ir::{
    AnnIR, AnnOpRef, IR, OpMap,
    partitioning::{PartitionId, PartitionPlacement},
};
use zhc_langs::ioplang::IopLang;
use zhc_utils::{data_visulization::Histogram, svec};

use crate::SpreadPolicy;

pub fn spread(
    ir: &IR<IopLang>,
    opmap: OpMap<PartitionPlacement>,
    policy: SpreadPolicy,
) -> OpMap<PartitionPlacement> {
    match policy {
        SpreadPolicy::FavorLatency => spread_for_latency(ir, opmap),
        SpreadPolicy::FavorBandwidth => spread_for_bandwidth(ir, opmap),
    }
}

fn spread_for_latency(
    ir: &IR<IopLang>,
    opmap: OpMap<PartitionPlacement>,
) -> OpMap<PartitionPlacement> {
    let default_pid = opmap
        .iter()
        .flat_map(|(_, placement)| placement.iter_pids())
        .min()
        .unwrap_or(PartitionId(0));
    let ann = AnnIR::new(ir, opmap, ir.filled_valmap(()));
    ann.backward_dataflow_analysis(
        |a: AnnOpRef<
            '_,
            '_,
            IopLang,
            zhc_ir::Analysing<PartitionPlacement>,
            zhc_ir::Analysing<()>,
        >,
         old| {
            let placement = match old.get_annotation() {
                PartitionPlacement::Irrelevant => a
                    .get_users_iter()
                    .flat_map(|user| user.get_annotation().as_ref().unwrap_analyzed().iter_pids())
                    .collect(),
                PartitionPlacement::Exclusive(pid) => PartitionPlacement::Exclusive(*pid),
                _ => unreachable!(),
            };
            (placement, svec![(); a.get_return_arity()])
        },
    )
    .forward_dataflow_analysis(
        |a: AnnOpRef<
            '_,
            '_,
            IopLang,
            zhc_ir::Analysing<PartitionPlacement>,
            zhc_ir::Analysing<_>,
        >,
         old| {
            let placement: PartitionPlacement = match old.get_annotation() {
                PartitionPlacement::Irrelevant => {
                    let mut histogram = Histogram::empty();
                    for pid in a.get_args_iter().flat_map(|arg| {
                        arg.get_origin()
                            .opref
                            .get_annotation()
                            .as_ref()
                            .unwrap_analyzed()
                            .iter_pids()
                    }) {
                        histogram.count(&pid);
                    }

                    histogram
                        .iter_classes()
                        .max_by_key(|(pid, count)| (**count, **pid))
                        .map_or(PartitionPlacement::Irrelevant, |(pid, _)| {
                            PartitionPlacement::Exclusive(*pid)
                        })
                }
                placement => placement.clone(),
            };
            (placement, svec![(); a.get_return_arity()])
        },
    )
    .backward_dataflow_analysis(
        |a: AnnOpRef<
            '_,
            '_,
            IopLang,
            zhc_ir::Analysing<PartitionPlacement>,
            zhc_ir::Analysing<()>,
        >,
         old| {
            let placement = match old.get_annotation() {
                PartitionPlacement::Irrelevant => a
                    .get_users_iter()
                    .flat_map(|user| user.get_annotation().as_ref().unwrap_analyzed().iter_pids())
                    .collect(),
                placement => placement.clone(),
            };
            (placement, svec![(); a.get_return_arity()])
        },
    )
    .forward_dataflow_analysis(
        |a: AnnOpRef<
            '_,
            '_,
            IopLang,
            zhc_ir::Analysing<PartitionPlacement>,
            zhc_ir::Analysing<_>,
        >,
         old| {
            let placement = match old.get_annotation() {
                PartitionPlacement::Irrelevant => a
                    .get_args_iter()
                    .flat_map(|arg| {
                        arg.get_origin()
                            .opref
                            .get_annotation()
                            .as_ref()
                            .unwrap_analyzed()
                            .iter_pids()
                    })
                    .next()
                    .map_or(PartitionPlacement::Exclusive(default_pid), |pid| {
                        PartitionPlacement::Exclusive(pid)
                    }),
                placement => placement.clone(),
            };
            (placement, svec![(); a.get_return_arity()])
        },
    )
    .into_opmap()
}

fn spread_for_bandwidth(
    ir: &IR<IopLang>,
    opmap: OpMap<PartitionPlacement>,
) -> OpMap<PartitionPlacement> {
    let default_pid = opmap
        .iter()
        .flat_map(|(_, placement)| placement.iter_pids())
        .min()
        .unwrap_or(PartitionId(0));
    let ann = AnnIR::new(ir, opmap, ir.filled_valmap(()));
    ann.forward_dataflow_analysis(
        |a: AnnOpRef<
            '_,
            '_,
            IopLang,
            zhc_ir::Analysing<PartitionPlacement>,
            zhc_ir::Analysing<_>,
        >,
         old| {
            let placement: PartitionPlacement = match old.get_annotation() {
                PartitionPlacement::Irrelevant => {
                    let mut histogram = Histogram::empty();
                    for pid in a.get_args_iter().flat_map(|arg| {
                        arg.get_origin()
                            .opref
                            .get_annotation()
                            .as_ref()
                            .unwrap_analyzed()
                            .iter_pids()
                    }) {
                        histogram.count(&pid);
                    }

                    histogram
                        .iter_classes()
                        .max_by_key(|(pid, count)| (**count, **pid))
                        .map_or(PartitionPlacement::Irrelevant, |(pid, _)| {
                            PartitionPlacement::Exclusive(*pid)
                        })
                }
                placement => placement.clone(),
            };
            (placement, svec![(); a.get_return_arity()])
        },
    )
    .backward_dataflow_analysis(
        |a: AnnOpRef<
            '_,
            '_,
            IopLang,
            zhc_ir::Analysing<PartitionPlacement>,
            zhc_ir::Analysing<()>,
        >,
         old| {
            let placement = match old.get_annotation() {
                PartitionPlacement::Irrelevant => a
                    .get_users_iter()
                    .flat_map(|user| user.get_annotation().as_ref().unwrap_analyzed().iter_pids())
                    .collect(),
                PartitionPlacement::Exclusive(pid) => PartitionPlacement::Exclusive(*pid),
                _ => unreachable!(),
            };
            (placement, svec![(); a.get_return_arity()])
        },
    )
    .forward_dataflow_analysis(
        |a: AnnOpRef<
            '_,
            '_,
            IopLang,
            zhc_ir::Analysing<PartitionPlacement>,
            zhc_ir::Analysing<_>,
        >,
         old| {
            let placement = match old.get_annotation() {
                PartitionPlacement::Irrelevant => a
                    .get_args_iter()
                    .flat_map(|arg| {
                        arg.get_origin()
                            .opref
                            .get_annotation()
                            .as_ref()
                            .unwrap_analyzed()
                            .iter_pids()
                    })
                    .next()
                    .map_or(PartitionPlacement::Exclusive(default_pid), |pid| {
                        PartitionPlacement::Exclusive(pid)
                    }),
                placement => placement.clone(),
            };
            (placement, svec![(); a.get_return_arity()])
        },
    )
    .into_opmap()
}
