// Lean compiler output
// Module: Zhc.Iops.AddHillisSteele.Impl
// Imports: public import Init public meta import Init public import Zhc.BlockAlgebra.Class public import Zhc.BlockAlgebra.Model.Instance public import Zhc.BlockAlgebra.Tracer.Instance public import Zhc.Iops.AddRippleCarry.Impl
#include <lean/lean.h>
#if defined(__clang__)
#pragma clang diagnostic ignored "-Wunused-parameter"
#pragma clang diagnostic ignored "-Wunused-label"
#elif defined(__GNUC__) && !defined(__CLANG__)
#pragma GCC diagnostic ignored "-Wunused-parameter"
#pragma GCC diagnostic ignored "-Wunused-label"
#pragma GCC diagnostic ignored "-Wunused-but-set-variable"
#endif
#ifdef __cplusplus
extern "C" {
#endif
lean_object* lean_nat_pow(lean_object*, lean_object*);
lean_object* l_List_drop___redArg(lean_object*, lean_object*);
lean_object* l_List_zipWith___at___00List_zip_spec__0___redArg(lean_object*, lean_object*);
lean_object* l_List_zipIdxTR___redArg(lean_object*, lean_object*);
lean_object* l_List_reverse___redArg(lean_object*);
lean_object* lp_zhc__lean_Zhc_Model_Operators_mac(lean_object*, lean_object*, lean_object*, lean_object*);
uint8_t lean_nat_dec_lt(lean_object*, lean_object*);
lean_object* lean_nat_shiftr(lean_object*, lean_object*);
lean_object* lean_nat_mod(lean_object*, lean_object*);
uint8_t lean_nat_dec_eq(lean_object*, lean_object*);
lean_object* lp_zhc__lean_Zhc_Model_CiphertextBlock_ofNat(lean_object*, lean_object*);
lean_object* lp_zhc__lean_Zhc_Model_Lut1_fromFn(lean_object*, lean_object*, lean_object*);
lean_object* lp_zhc__lean_Zhc_Model_ManyLut_output(lean_object*, lean_object*, lean_object*, lean_object*, lean_object*);
lean_object* lean_mk_empty_array_with_capacity(lean_object*);
lean_object* l___private_Init_Data_List_Impl_0__List_takeTR_go___redArg(lean_object*, lean_object*, lean_object*, lean_object*);
lean_object* l_List_appendTR___redArg(lean_object*, lean_object*);
lean_object* lp_zhc__lean_Zhc_BlockAlgebra_Tracer_declareInteger(lean_object*, lean_object*, lean_object*);
lean_object* l_List_finRange(lean_object*);
lean_object* lp_zhc__lean_Zhc_BlockAlgebra_Tracer_storeCtBlock___redArg(lean_object*, lean_object*, lean_object*, lean_object*);
lean_object* lean_nat_add(lean_object*, lean_object*);
lean_object* lean_nat_div(lean_object*, lean_object*);
lean_object* l_Nat_reprFast(lean_object*);
lean_object* lean_string_append(lean_object*, lean_object*);
lean_object* lean_nat_mul(lean_object*, lean_object*);
lean_object* lean_nat_sub(lean_object*, lean_object*);
extern lean_object* lp_zhc__lean_Zhc_Iops_carryMsgLut;
lean_object* lp_zhc__lean_Zhc_BlockAlgebra_Tracer_addCt(lean_object*, lean_object*, lean_object*, lean_object*);
lean_object* lp_zhc__lean_Zhc_BlockAlgebra_Tracer_pbs(lean_object*, lean_object*, lean_object*, lean_object*);
lean_object* lp_zhc__lean_Zhc_BlockAlgebra_Tracer_letPtBlock(lean_object*, lean_object*, lean_object*);
lean_object* lp_zhc__lean_Zhc_BlockAlgebra_Tracer_addPt(lean_object*, lean_object*, lean_object*, lean_object*);
lean_object* lp_zhc__lean_Zhc_Model_Operators_add(lean_object*, lean_object*, lean_object*);
lean_object* lp_zhc__lean_Zhc_Model_CiphertextBlock_doMaskMessage(lean_object*, lean_object*);
lean_object* lp_zhc__lean_Zhc_BlockAlgebra_Tracer_extractCtBlock___redArg(lean_object*, lean_object*, lean_object*);
lean_object* lp_zhc__lean_Zhc_BlockAlgebra_Tracer_letCtBlock(lean_object*, lean_object*, lean_object*);
lean_object* l_Nat_nextPowerOfTwo(lean_object*);
lean_object* l_List_replicateTR___redArg(lean_object*, lean_object*);
lean_object* l_List_mapM_loop___redArg(lean_object*, lean_object*, lean_object*, lean_object*);
lean_object* l_List_lengthTR___redArg(lean_object*);
lean_object* lean_nat_log2(lean_object*);
lean_object* l_List_range(lean_object*);
lean_object* lp_zhc__lean_Zhc_Model_CiphertextInteger_setBlock___redArg(lean_object*, lean_object*, lean_object*);
lean_object* l_List_mapTR_loop___redArg(lean_object*, lean_object*, lean_object*);
lean_object* lp_zhc__lean_Zhc_BlockAlgebra_Tracer_inputCtInteger(lean_object*, lean_object*, lean_object*);
lean_object* lp_zhc__lean_Zhc_BlockAlgebra_Tracer_pbs2(lean_object*, lean_object*, lean_object*, lean_object*);
lean_object* lp_zhc__lean_Zhc_BlockAlgebra_Tracer_packCt(lean_object*, lean_object*, lean_object*, lean_object*, lean_object*);
lean_object* lp_zhc__lean_Zhc_BlockAlgebra_Tracer_outputCtInteger___redArg(lean_object*, lean_object*);
lean_object* lp_zhc__lean_Zhc_Model_CiphertextBlock_digits(lean_object*, lean_object*, lean_object*);
lean_object* l_List_get___redArg(lean_object*, lean_object*);
lean_object* lp_zhc__lean_Zhc_Model_PlaintextBlock_ofNat(lean_object*, lean_object*);
lean_object* lp_zhc__lean_Zhc_Model_Operators_addPt(lean_object*, lean_object*, lean_object*);
lean_object* lp_zhc__lean_Zhc_Model_CiphertextBlock_listToNat(lean_object*, lean_object*);
lean_object* l_List_forIn_x27_loop___redArg(lean_object*, lean_object*, lean_object*, lean_object*);
lean_object* lp_zhc__lean_Zhc_BlockAlgebra_Tracer_emit(lean_object*, lean_object*);
LEAN_EXPORT lean_object* lp_zhc__lean_Zhc_Iops_HillisSteele_computeSize(lean_object*);
LEAN_EXPORT lean_object* lp_zhc__lean_Zhc_Iops_HillisSteele_computeSize___boxed(lean_object*);
LEAN_EXPORT lean_object* lp_zhc__lean_Zhc_Iops_HillisSteele_nbStages(lean_object*);
LEAN_EXPORT lean_object* lp_zhc__lean_Zhc_Iops_HillisSteele_nbStages___boxed(lean_object*);
LEAN_EXPORT lean_object* lp_zhc__lean_Zhc_Iops_HillisSteele_chunk4___redArg(lean_object*);
LEAN_EXPORT lean_object* lp_zhc__lean_Zhc_Iops_HillisSteele_chunk4(lean_object*, lean_object*);
LEAN_EXPORT lean_object* lp_zhc__lean_Zhc_Iops_HillisSteele_unchunk4___redArg(lean_object*);
LEAN_EXPORT lean_object* lp_zhc__lean_Zhc_Iops_HillisSteele_unchunk4(lean_object*, lean_object*);
LEAN_EXPORT lean_object* lp_zhc__lean_Zhc_Iops_HillisSteele_extractPropGroupLut___lam__0(lean_object*, lean_object*, lean_object*, lean_object*, lean_object*);
LEAN_EXPORT lean_object* lp_zhc__lean_Zhc_Iops_HillisSteele_extractPropGroupLut___lam__0___boxed(lean_object*, lean_object*, lean_object*, lean_object*, lean_object*);
static const lean_ctor_object lp_zhc__lean_Zhc_Iops_HillisSteele_extractPropGroupLut___closed__0_value = {.m_header = {.m_rc = 0, .m_cs_sz = sizeof(lean_ctor_object) + sizeof(void*)*2 + 0, .m_other = 2, .m_tag = 0}, .m_objs = {((lean_object*)(((size_t)(2) << 1) | 1)),((lean_object*)(((size_t)(2) << 1) | 1))}};
static const lean_object* lp_zhc__lean_Zhc_Iops_HillisSteele_extractPropGroupLut___closed__0 = (const lean_object*)&lp_zhc__lean_Zhc_Iops_HillisSteele_extractPropGroupLut___closed__0_value;
static const lean_string_object lp_zhc__lean_Zhc_Iops_HillisSteele_extractPropGroupLut___closed__1_value = {.m_header = {.m_rc = 0, .m_cs_sz = 0, .m_other = 0, .m_tag = 249}, .m_size = 17, .m_capacity = 17, .m_length = 16, .m_data = "ExtractPropGroup"};
static const lean_object* lp_zhc__lean_Zhc_Iops_HillisSteele_extractPropGroupLut___closed__1 = (const lean_object*)&lp_zhc__lean_Zhc_Iops_HillisSteele_extractPropGroupLut___closed__1_value;
LEAN_EXPORT lean_object* lp_zhc__lean_Zhc_Iops_HillisSteele_extractPropGroupLut(lean_object*);
LEAN_EXPORT lean_object* lp_zhc__lean_Zhc_Iops_HillisSteele_solvePropGroupFinalLut___lam__0(lean_object*, lean_object*, lean_object*, lean_object*);
LEAN_EXPORT lean_object* lp_zhc__lean_Zhc_Iops_HillisSteele_solvePropGroupFinalLut___lam__0___boxed(lean_object*, lean_object*, lean_object*, lean_object*);
static const lean_string_object lp_zhc__lean_Zhc_Iops_HillisSteele_solvePropGroupFinalLut___closed__0_value = {.m_header = {.m_rc = 0, .m_cs_sz = 0, .m_other = 0, .m_tag = 249}, .m_size = 20, .m_capacity = 20, .m_length = 19, .m_data = "SolvePropGroupFinal"};
static const lean_object* lp_zhc__lean_Zhc_Iops_HillisSteele_solvePropGroupFinalLut___closed__0 = (const lean_object*)&lp_zhc__lean_Zhc_Iops_HillisSteele_solvePropGroupFinalLut___closed__0_value;
LEAN_EXPORT lean_object* lp_zhc__lean_Zhc_Iops_HillisSteele_solvePropGroupFinalLut(lean_object*);
LEAN_EXPORT lean_object* lp_zhc__lean_Zhc_Iops_HillisSteele_reduceCarryPadLut___lam__0(lean_object*, lean_object*);
LEAN_EXPORT lean_object* lp_zhc__lean_Zhc_Iops_HillisSteele_reduceCarryPadLut___lam__0___boxed(lean_object*, lean_object*);
static const lean_closure_object lp_zhc__lean_Zhc_Iops_HillisSteele_reduceCarryPadLut___closed__0_value = {.m_header = {.m_rc = 0, .m_cs_sz = sizeof(lean_closure_object) + sizeof(void*)*1, .m_other = 0, .m_tag = 245}, .m_fun = (void*)lp_zhc__lean_Zhc_Iops_HillisSteele_reduceCarryPadLut___lam__0___boxed, .m_arity = 2, .m_num_fixed = 1, .m_objs = {((lean_object*)&lp_zhc__lean_Zhc_Iops_HillisSteele_extractPropGroupLut___closed__0_value)} };
static const lean_object* lp_zhc__lean_Zhc_Iops_HillisSteele_reduceCarryPadLut___closed__0 = (const lean_object*)&lp_zhc__lean_Zhc_Iops_HillisSteele_reduceCarryPadLut___closed__0_value;
static const lean_string_object lp_zhc__lean_Zhc_Iops_HillisSteele_reduceCarryPadLut___closed__1_value = {.m_header = {.m_rc = 0, .m_cs_sz = 0, .m_other = 0, .m_tag = 249}, .m_size = 15, .m_capacity = 15, .m_length = 14, .m_data = "ReduceCarryPad"};
static const lean_object* lp_zhc__lean_Zhc_Iops_HillisSteele_reduceCarryPadLut___closed__1 = (const lean_object*)&lp_zhc__lean_Zhc_Iops_HillisSteele_reduceCarryPadLut___closed__1_value;
static lean_once_cell_t lp_zhc__lean_Zhc_Iops_HillisSteele_reduceCarryPadLut___closed__2_once = LEAN_ONCE_CELL_INITIALIZER;
static lean_object* lp_zhc__lean_Zhc_Iops_HillisSteele_reduceCarryPadLut___closed__2;
LEAN_EXPORT lean_object* lp_zhc__lean_Zhc_Iops_HillisSteele_reduceCarryPadLut;
LEAN_EXPORT lean_object* lp_zhc__lean_Zhc_Iops_HillisSteele_solvePropLut___lam__0(lean_object*, lean_object*, lean_object*);
LEAN_EXPORT lean_object* lp_zhc__lean_Zhc_Iops_HillisSteele_solvePropLut___lam__0___boxed(lean_object*, lean_object*, lean_object*);
static const lean_closure_object lp_zhc__lean_Zhc_Iops_HillisSteele_solvePropLut___closed__0_value = {.m_header = {.m_rc = 0, .m_cs_sz = sizeof(lean_closure_object) + sizeof(void*)*2, .m_other = 0, .m_tag = 245}, .m_fun = (void*)lp_zhc__lean_Zhc_Iops_HillisSteele_solvePropLut___lam__0___boxed, .m_arity = 3, .m_num_fixed = 2, .m_objs = {((lean_object*)(((size_t)(2) << 1) | 1)),((lean_object*)&lp_zhc__lean_Zhc_Iops_HillisSteele_extractPropGroupLut___closed__0_value)} };
static const lean_object* lp_zhc__lean_Zhc_Iops_HillisSteele_solvePropLut___closed__0 = (const lean_object*)&lp_zhc__lean_Zhc_Iops_HillisSteele_solvePropLut___closed__0_value;
static const lean_string_object lp_zhc__lean_Zhc_Iops_HillisSteele_solvePropLut___closed__1_value = {.m_header = {.m_rc = 0, .m_cs_sz = 0, .m_other = 0, .m_tag = 249}, .m_size = 10, .m_capacity = 10, .m_length = 9, .m_data = "SolveProp"};
static const lean_object* lp_zhc__lean_Zhc_Iops_HillisSteele_solvePropLut___closed__1 = (const lean_object*)&lp_zhc__lean_Zhc_Iops_HillisSteele_solvePropLut___closed__1_value;
static lean_once_cell_t lp_zhc__lean_Zhc_Iops_HillisSteele_solvePropLut___closed__2_once = LEAN_ONCE_CELL_INITIALIZER;
static lean_object* lp_zhc__lean_Zhc_Iops_HillisSteele_solvePropLut___closed__2;
LEAN_EXPORT lean_object* lp_zhc__lean_Zhc_Iops_HillisSteele_solvePropLut;
LEAN_EXPORT lean_object* lp_zhc__lean_Zhc_Iops_HillisSteele_solvePropCarryLut___lam__0(lean_object*, lean_object*, lean_object*);
LEAN_EXPORT lean_object* lp_zhc__lean_Zhc_Iops_HillisSteele_solvePropCarryLut___lam__0___boxed(lean_object*, lean_object*, lean_object*);
static const lean_closure_object lp_zhc__lean_Zhc_Iops_HillisSteele_solvePropCarryLut___closed__0_value = {.m_header = {.m_rc = 0, .m_cs_sz = sizeof(lean_closure_object) + sizeof(void*)*2, .m_other = 0, .m_tag = 245}, .m_fun = (void*)lp_zhc__lean_Zhc_Iops_HillisSteele_solvePropCarryLut___lam__0___boxed, .m_arity = 3, .m_num_fixed = 2, .m_objs = {((lean_object*)(((size_t)(2) << 1) | 1)),((lean_object*)&lp_zhc__lean_Zhc_Iops_HillisSteele_extractPropGroupLut___closed__0_value)} };
static const lean_object* lp_zhc__lean_Zhc_Iops_HillisSteele_solvePropCarryLut___closed__0 = (const lean_object*)&lp_zhc__lean_Zhc_Iops_HillisSteele_solvePropCarryLut___closed__0_value;
static const lean_string_object lp_zhc__lean_Zhc_Iops_HillisSteele_solvePropCarryLut___closed__1_value = {.m_header = {.m_rc = 0, .m_cs_sz = 0, .m_other = 0, .m_tag = 249}, .m_size = 15, .m_capacity = 15, .m_length = 14, .m_data = "SolvePropCarry"};
static const lean_object* lp_zhc__lean_Zhc_Iops_HillisSteele_solvePropCarryLut___closed__1 = (const lean_object*)&lp_zhc__lean_Zhc_Iops_HillisSteele_solvePropCarryLut___closed__1_value;
static lean_once_cell_t lp_zhc__lean_Zhc_Iops_HillisSteele_solvePropCarryLut___closed__2_once = LEAN_ONCE_CELL_INITIALIZER;
static lean_object* lp_zhc__lean_Zhc_Iops_HillisSteele_solvePropCarryLut___closed__2;
LEAN_EXPORT lean_object* lp_zhc__lean_Zhc_Iops_HillisSteele_solvePropCarryLut;
LEAN_EXPORT lean_object* lp_zhc__lean_Zhc_Iops_HillisSteele_msgOnlyLut___lam__0(lean_object*, lean_object*);
LEAN_EXPORT lean_object* lp_zhc__lean_Zhc_Iops_HillisSteele_msgOnlyLut___lam__0___boxed(lean_object*, lean_object*);
static const lean_closure_object lp_zhc__lean_Zhc_Iops_HillisSteele_msgOnlyLut___closed__0_value = {.m_header = {.m_rc = 0, .m_cs_sz = sizeof(lean_closure_object) + sizeof(void*)*1, .m_other = 0, .m_tag = 245}, .m_fun = (void*)lp_zhc__lean_Zhc_Iops_HillisSteele_msgOnlyLut___lam__0___boxed, .m_arity = 2, .m_num_fixed = 1, .m_objs = {((lean_object*)&lp_zhc__lean_Zhc_Iops_HillisSteele_extractPropGroupLut___closed__0_value)} };
static const lean_object* lp_zhc__lean_Zhc_Iops_HillisSteele_msgOnlyLut___closed__0 = (const lean_object*)&lp_zhc__lean_Zhc_Iops_HillisSteele_msgOnlyLut___closed__0_value;
static const lean_string_object lp_zhc__lean_Zhc_Iops_HillisSteele_msgOnlyLut___closed__1_value = {.m_header = {.m_rc = 0, .m_cs_sz = 0, .m_other = 0, .m_tag = 249}, .m_size = 8, .m_capacity = 8, .m_length = 7, .m_data = "MsgOnly"};
static const lean_object* lp_zhc__lean_Zhc_Iops_HillisSteele_msgOnlyLut___closed__1 = (const lean_object*)&lp_zhc__lean_Zhc_Iops_HillisSteele_msgOnlyLut___closed__1_value;
static lean_once_cell_t lp_zhc__lean_Zhc_Iops_HillisSteele_msgOnlyLut___closed__2_once = LEAN_ONCE_CELL_INITIALIZER;
static lean_object* lp_zhc__lean_Zhc_Iops_HillisSteele_msgOnlyLut___closed__2;
LEAN_EXPORT lean_object* lp_zhc__lean_Zhc_Iops_HillisSteele_msgOnlyLut;
LEAN_EXPORT lean_object* lp_zhc__lean_Zhc_Iops_HillisSteele_rawSums___redArg___lam__0(lean_object*, lean_object*, lean_object*);
LEAN_EXPORT lean_object* lp_zhc__lean_Zhc_Iops_HillisSteele_rawSums___redArg___lam__1(lean_object*, lean_object*, lean_object*, lean_object*, lean_object*, lean_object*, lean_object*);
LEAN_EXPORT lean_object* lp_zhc__lean_Zhc_Iops_HillisSteele_rawSums___redArg___lam__2(lean_object*, lean_object*, lean_object*, lean_object*, lean_object*, lean_object*);
LEAN_EXPORT lean_object* lp_zhc__lean_Zhc_Iops_HillisSteele_rawSums___redArg___lam__3(lean_object*, lean_object*, lean_object*, lean_object*);
LEAN_EXPORT lean_object* lp_zhc__lean_Zhc_Iops_HillisSteele_rawSums___redArg___lam__3___boxed(lean_object*, lean_object*, lean_object*, lean_object*);
LEAN_EXPORT lean_object* lp_zhc__lean_Zhc_Iops_HillisSteele_rawSums___redArg___lam__4(lean_object*, lean_object*, lean_object*, lean_object*, lean_object*);
LEAN_EXPORT lean_object* lp_zhc__lean_Zhc_Iops_HillisSteele_rawSums___redArg(lean_object*, lean_object*, lean_object*, lean_object*, lean_object*);
LEAN_EXPORT lean_object* lp_zhc__lean_Zhc_Iops_HillisSteele_rawSums(lean_object*, lean_object*, lean_object*, lean_object*, lean_object*, lean_object*);
LEAN_EXPORT lean_object* lp_zhc__lean_Zhc_Iops_HillisSteele_blockStates___redArg___lam__0(lean_object*, lean_object*);
LEAN_EXPORT lean_object* lp_zhc__lean_Zhc_Iops_HillisSteele_blockStates___redArg___lam__1(lean_object*, lean_object*, lean_object*, lean_object*);
LEAN_EXPORT lean_object* lp_zhc__lean_Zhc_Iops_HillisSteele_blockStates___redArg(lean_object*, lean_object*, lean_object*);
LEAN_EXPORT lean_object* lp_zhc__lean_Zhc_Iops_HillisSteele_blockStates(lean_object*, lean_object*, lean_object*, lean_object*);
LEAN_EXPORT lean_object* lp_zhc__lean_Zhc_Iops_HillisSteele_groupStates___redArg___lam__0(lean_object*, lean_object*, lean_object*, lean_object*, lean_object*);
LEAN_EXPORT lean_object* lp_zhc__lean_Zhc_Iops_HillisSteele_groupStates___redArg___lam__1(lean_object*, lean_object*);
LEAN_EXPORT lean_object* lp_zhc__lean_Zhc_Iops_HillisSteele_groupStates___redArg___lam__2(lean_object*, lean_object*, lean_object*, lean_object*, lean_object*);
LEAN_EXPORT lean_object* lp_zhc__lean_Zhc_Iops_HillisSteele_groupStates___redArg___lam__3(lean_object*, lean_object*, lean_object*, lean_object*, lean_object*);
static lean_once_cell_t lp_zhc__lean_Zhc_Iops_HillisSteele_groupStates___redArg___lam__5___closed__0_once = LEAN_ONCE_CELL_INITIALIZER;
static lean_object* lp_zhc__lean_Zhc_Iops_HillisSteele_groupStates___redArg___lam__5___closed__0;
static lean_once_cell_t lp_zhc__lean_Zhc_Iops_HillisSteele_groupStates___redArg___lam__5___closed__1_once = LEAN_ONCE_CELL_INITIALIZER;
static lean_object* lp_zhc__lean_Zhc_Iops_HillisSteele_groupStates___redArg___lam__5___closed__1;
LEAN_EXPORT lean_object* lp_zhc__lean_Zhc_Iops_HillisSteele_groupStates___redArg___lam__5(lean_object*, lean_object*, lean_object*, lean_object*, lean_object*, lean_object*);
LEAN_EXPORT lean_object* lp_zhc__lean_Zhc_Iops_HillisSteele_groupStates___redArg___lam__5___boxed(lean_object*, lean_object*, lean_object*, lean_object*, lean_object*, lean_object*);
LEAN_EXPORT lean_object* lp_zhc__lean_Zhc_Iops_HillisSteele_groupStates___redArg___lam__4(lean_object*, lean_object*, lean_object*, lean_object*, lean_object*, lean_object*, lean_object*, lean_object*, lean_object*, lean_object*, lean_object*);
LEAN_EXPORT lean_object* lp_zhc__lean_Zhc_Iops_HillisSteele_groupStates___redArg___lam__6(lean_object*, lean_object*, lean_object*, lean_object*, lean_object*, lean_object*, lean_object*, lean_object*, lean_object*, lean_object*, lean_object*);
LEAN_EXPORT lean_object* lp_zhc__lean_Zhc_Iops_HillisSteele_groupStates___redArg___lam__7(lean_object*, lean_object*, lean_object*, lean_object*);
LEAN_EXPORT lean_object* lp_zhc__lean_Zhc_Iops_HillisSteele_groupStates___redArg___lam__8(lean_object*, lean_object*);
LEAN_EXPORT lean_object* lp_zhc__lean_Zhc_Iops_HillisSteele_groupStates___redArg(lean_object*, lean_object*, lean_object*);
LEAN_EXPORT lean_object* lp_zhc__lean_Zhc_Iops_HillisSteele_groupStates(lean_object*, lean_object*, lean_object*, lean_object*);
LEAN_EXPORT lean_object* lp_zhc__lean_Zhc_Iops_HillisSteele_hsStage___redArg___lam__0(uint8_t, lean_object*, lean_object*);
LEAN_EXPORT lean_object* lp_zhc__lean_Zhc_Iops_HillisSteele_hsStage___redArg___lam__0___boxed(lean_object*, lean_object*, lean_object*);
LEAN_EXPORT lean_object* lp_zhc__lean_Zhc_Iops_HillisSteele_hsStage___redArg___lam__1(lean_object*, lean_object*, lean_object*, lean_object*);
LEAN_EXPORT lean_object* lp_zhc__lean_Zhc_Iops_HillisSteele_hsStage___redArg___lam__1___boxed(lean_object*, lean_object*, lean_object*, lean_object*);
static const lean_array_object lp_zhc__lean_Zhc_Iops_HillisSteele_hsStage___redArg___lam__2___closed__0_value = {.m_header = {.m_rc = 0, .m_cs_sz = sizeof(lean_array_object) + sizeof(void*)*0, .m_other = 0, .m_tag = 246}, .m_size = 0, .m_capacity = 0, .m_data = {}};
static const lean_object* lp_zhc__lean_Zhc_Iops_HillisSteele_hsStage___redArg___lam__2___closed__0 = (const lean_object*)&lp_zhc__lean_Zhc_Iops_HillisSteele_hsStage___redArg___lam__2___closed__0_value;
LEAN_EXPORT lean_object* lp_zhc__lean_Zhc_Iops_HillisSteele_hsStage___redArg___lam__2(lean_object*, lean_object*, lean_object*, lean_object*);
LEAN_EXPORT lean_object* lp_zhc__lean_Zhc_Iops_HillisSteele_hsStage___redArg(lean_object*, lean_object*, lean_object*, lean_object*);
LEAN_EXPORT lean_object* lp_zhc__lean_Zhc_Iops_HillisSteele_hsStage___redArg___boxed(lean_object*, lean_object*, lean_object*, lean_object*);
LEAN_EXPORT lean_object* lp_zhc__lean_Zhc_Iops_HillisSteele_hsStage(lean_object*, lean_object*, lean_object*, lean_object*, lean_object*);
LEAN_EXPORT lean_object* lp_zhc__lean_Zhc_Iops_HillisSteele_hsStage___boxed(lean_object*, lean_object*, lean_object*, lean_object*, lean_object*);
LEAN_EXPORT lean_object* lp_zhc__lean_Zhc_Iops_HillisSteele_groupCarries___redArg___lam__0(lean_object*);
LEAN_EXPORT lean_object* lp_zhc__lean_Zhc_Iops_HillisSteele_groupCarries___redArg___lam__0___boxed(lean_object*);
LEAN_EXPORT lean_object* lp_zhc__lean_Zhc_Iops_HillisSteele_groupCarries___redArg___lam__1(lean_object*, lean_object*);
LEAN_EXPORT lean_object* lp_zhc__lean_Zhc_Iops_HillisSteele_groupCarries___redArg___lam__2(lean_object*, lean_object*);
LEAN_EXPORT lean_object* lp_zhc__lean_Zhc_Iops_HillisSteele_groupCarries___redArg___lam__3(lean_object*, lean_object*, lean_object*, lean_object*, lean_object*, lean_object*, lean_object*);
LEAN_EXPORT lean_object* lp_zhc__lean_Zhc_Iops_HillisSteele_groupCarries___redArg___lam__3___boxed(lean_object*, lean_object*, lean_object*, lean_object*, lean_object*, lean_object*, lean_object*);
static const lean_closure_object lp_zhc__lean_Zhc_Iops_HillisSteele_groupCarries___redArg___closed__0_value = {.m_header = {.m_rc = 0, .m_cs_sz = sizeof(lean_closure_object) + sizeof(void*)*0, .m_other = 0, .m_tag = 245}, .m_fun = (void*)lp_zhc__lean_Zhc_Iops_HillisSteele_groupCarries___redArg___lam__0___boxed, .m_arity = 1, .m_num_fixed = 0, .m_objs = {} };
static const lean_object* lp_zhc__lean_Zhc_Iops_HillisSteele_groupCarries___redArg___closed__0 = (const lean_object*)&lp_zhc__lean_Zhc_Iops_HillisSteele_groupCarries___redArg___closed__0_value;
LEAN_EXPORT lean_object* lp_zhc__lean_Zhc_Iops_HillisSteele_groupCarries___redArg(lean_object*, lean_object*, lean_object*);
LEAN_EXPORT lean_object* lp_zhc__lean_Zhc_Iops_HillisSteele_groupCarries(lean_object*, lean_object*, lean_object*, lean_object*);
LEAN_EXPORT lean_object* lp_zhc__lean_Zhc_Iops_HillisSteele_finalResolution___redArg___lam__0(lean_object*);
LEAN_EXPORT lean_object* lp_zhc__lean_Zhc_Iops_HillisSteele_finalResolution___redArg___lam__1(lean_object*, lean_object*, lean_object*, lean_object*, lean_object*);
LEAN_EXPORT lean_object* lp_zhc__lean_Zhc_Iops_HillisSteele_finalResolution___redArg___lam__2(lean_object*, lean_object*, lean_object*, lean_object*, lean_object*, lean_object*, lean_object*, lean_object*);
LEAN_EXPORT lean_object* lp_zhc__lean_Zhc_Iops_HillisSteele_finalResolution___redArg___lam__2___boxed(lean_object*, lean_object*, lean_object*, lean_object*, lean_object*, lean_object*, lean_object*, lean_object*);
LEAN_EXPORT lean_object* lp_zhc__lean_Zhc_Iops_HillisSteele_finalResolution___redArg___lam__3(lean_object*, lean_object*, lean_object*, lean_object*, lean_object*);
LEAN_EXPORT lean_object* lp_zhc__lean_Zhc_Iops_HillisSteele_finalResolution___redArg___lam__4(lean_object*, lean_object*, lean_object*, lean_object*, lean_object*);
LEAN_EXPORT lean_object* lp_zhc__lean_Zhc_Iops_HillisSteele_finalResolution___redArg___lam__4___boxed(lean_object*, lean_object*, lean_object*, lean_object*, lean_object*);
LEAN_EXPORT lean_object* lp_zhc__lean_Zhc_Iops_HillisSteele_finalResolution___redArg___lam__5(lean_object*, lean_object*, lean_object*, lean_object*, lean_object*, lean_object*, lean_object*, lean_object*, lean_object*, lean_object*);
LEAN_EXPORT lean_object* lp_zhc__lean_Zhc_Iops_HillisSteele_finalResolution___redArg___lam__6(lean_object*, lean_object*, lean_object*, lean_object*, lean_object*);
LEAN_EXPORT lean_object* lp_zhc__lean_Zhc_Iops_HillisSteele_finalResolution___redArg___lam__6___boxed(lean_object*, lean_object*, lean_object*, lean_object*, lean_object*);
LEAN_EXPORT lean_object* lp_zhc__lean_Zhc_Iops_HillisSteele_finalResolution___redArg___lam__7(lean_object*, lean_object*, lean_object*, lean_object*, lean_object*, lean_object*, lean_object*, lean_object*, lean_object*, lean_object*);
static lean_once_cell_t lp_zhc__lean_Zhc_Iops_HillisSteele_finalResolution___redArg___lam__8___closed__0_once = LEAN_ONCE_CELL_INITIALIZER;
static lean_object* lp_zhc__lean_Zhc_Iops_HillisSteele_finalResolution___redArg___lam__8___closed__0;
static lean_once_cell_t lp_zhc__lean_Zhc_Iops_HillisSteele_finalResolution___redArg___lam__8___closed__1_once = LEAN_ONCE_CELL_INITIALIZER;
static lean_object* lp_zhc__lean_Zhc_Iops_HillisSteele_finalResolution___redArg___lam__8___closed__1;
LEAN_EXPORT lean_object* lp_zhc__lean_Zhc_Iops_HillisSteele_finalResolution___redArg___lam__8(lean_object*, lean_object*, lean_object*, lean_object*, lean_object*, lean_object*, lean_object*, lean_object*, lean_object*);
LEAN_EXPORT lean_object* lp_zhc__lean_Zhc_Iops_HillisSteele_finalResolution___redArg___lam__9(lean_object*, lean_object*, lean_object*, lean_object*);
LEAN_EXPORT lean_object* lp_zhc__lean_Zhc_Iops_HillisSteele_finalResolution___redArg___lam__10(lean_object*, lean_object*);
static const lean_closure_object lp_zhc__lean_Zhc_Iops_HillisSteele_finalResolution___redArg___closed__0_value = {.m_header = {.m_rc = 0, .m_cs_sz = sizeof(lean_closure_object) + sizeof(void*)*0, .m_other = 0, .m_tag = 245}, .m_fun = (void*)lp_zhc__lean_Zhc_Iops_HillisSteele_finalResolution___redArg___lam__0, .m_arity = 1, .m_num_fixed = 0, .m_objs = {} };
static const lean_object* lp_zhc__lean_Zhc_Iops_HillisSteele_finalResolution___redArg___closed__0 = (const lean_object*)&lp_zhc__lean_Zhc_Iops_HillisSteele_finalResolution___redArg___closed__0_value;
LEAN_EXPORT lean_object* lp_zhc__lean_Zhc_Iops_HillisSteele_finalResolution___redArg(lean_object*, lean_object*, lean_object*, lean_object*);
LEAN_EXPORT lean_object* lp_zhc__lean_Zhc_Iops_HillisSteele_finalResolution(lean_object*, lean_object*, lean_object*, lean_object*, lean_object*);
LEAN_EXPORT lean_object* lp_zhc__lean_Zhc_Iops_HillisSteele_propagate___redArg___lam__0(lean_object*, lean_object*);
LEAN_EXPORT lean_object* lp_zhc__lean_Zhc_Iops_HillisSteele_propagate___redArg___lam__1(lean_object*, lean_object*, lean_object*, lean_object*);
LEAN_EXPORT lean_object* lp_zhc__lean_Zhc_Iops_HillisSteele_propagate___redArg___lam__2(lean_object*, lean_object*, lean_object*, lean_object*, lean_object*, lean_object*, lean_object*);
LEAN_EXPORT lean_object* lp_zhc__lean_Zhc_Iops_HillisSteele_propagate___redArg(lean_object*, lean_object*, lean_object*, lean_object*);
LEAN_EXPORT lean_object* lp_zhc__lean_Zhc_Iops_HillisSteele_propagate(lean_object*, lean_object*, lean_object*, lean_object*, lean_object*);
LEAN_EXPORT lean_object* lp_zhc__lean_Zhc_Iops_HillisSteele_joinBlocks___redArg___lam__0(lean_object*, lean_object*);
LEAN_EXPORT lean_object* lp_zhc__lean_Zhc_Iops_HillisSteele_joinBlocks___redArg___lam__1(lean_object*, lean_object*, lean_object*, lean_object*, lean_object*, lean_object*, lean_object*);
LEAN_EXPORT lean_object* lp_zhc__lean_Zhc_Iops_HillisSteele_joinBlocks___redArg___lam__2(lean_object*, lean_object*);
LEAN_EXPORT lean_object* lp_zhc__lean_Zhc_Iops_HillisSteele_joinBlocks___redArg___lam__3(lean_object*, lean_object*, lean_object*, lean_object*, lean_object*, lean_object*, lean_object*);
LEAN_EXPORT lean_object* lp_zhc__lean_Zhc_Iops_HillisSteele_joinBlocks___redArg(lean_object*, lean_object*, lean_object*, lean_object*);
LEAN_EXPORT lean_object* lp_zhc__lean_Zhc_Iops_HillisSteele_joinBlocks(lean_object*, lean_object*, lean_object*, lean_object*, lean_object*);
LEAN_EXPORT lean_object* lp_zhc__lean_Zhc_Iops_addHillisSteele___redArg___lam__0(lean_object*, lean_object*, lean_object*, lean_object*);
LEAN_EXPORT lean_object* lp_zhc__lean_Zhc_Iops_addHillisSteele___redArg___lam__1(lean_object*, lean_object*, lean_object*, lean_object*, lean_object*, lean_object*);
LEAN_EXPORT lean_object* lp_zhc__lean_Zhc_Iops_addHillisSteele___redArg___lam__2(lean_object*, lean_object*, lean_object*, lean_object*, lean_object*, lean_object*);
LEAN_EXPORT lean_object* lp_zhc__lean_Zhc_Iops_addHillisSteele___redArg___lam__3(lean_object*, lean_object*, lean_object*, lean_object*, lean_object*);
LEAN_EXPORT lean_object* lp_zhc__lean_Zhc_Iops_addHillisSteele___redArg___lam__4(lean_object*, lean_object*, lean_object*, lean_object*, lean_object*);
LEAN_EXPORT lean_object* lp_zhc__lean_Zhc_Iops_addHillisSteele___redArg___lam__5(lean_object*, lean_object*, lean_object*, lean_object*, lean_object*);
LEAN_EXPORT lean_object* lp_zhc__lean_Zhc_Iops_addHillisSteele___redArg(lean_object*, lean_object*, lean_object*, lean_object*, lean_object*);
LEAN_EXPORT lean_object* lp_zhc__lean_Zhc_Iops_addHillisSteele(lean_object*, lean_object*, lean_object*, lean_object*, lean_object*, lean_object*);
static lean_once_cell_t lp_zhc__lean_List_mapM_loop___at___00Zhc_Iops_HillisSteele_finalResolution___at___00Zhc_Iops_addHillisSteele___at___00__private_Zhc_Iops_AddHillisSteele_Impl_0__Zhc_Iops_smokeHs_spec__0_spec__4_spec__12___closed__0_once = LEAN_ONCE_CELL_INITIALIZER;
static lean_object* lp_zhc__lean_List_mapM_loop___at___00Zhc_Iops_HillisSteele_finalResolution___at___00Zhc_Iops_addHillisSteele___at___00__private_Zhc_Iops_AddHillisSteele_Impl_0__Zhc_Iops_smokeHs_spec__0_spec__4_spec__12___closed__0;
static lean_once_cell_t lp_zhc__lean_List_mapM_loop___at___00Zhc_Iops_HillisSteele_finalResolution___at___00Zhc_Iops_addHillisSteele___at___00__private_Zhc_Iops_AddHillisSteele_Impl_0__Zhc_Iops_smokeHs_spec__0_spec__4_spec__12___closed__1_once = LEAN_ONCE_CELL_INITIALIZER;
static lean_object* lp_zhc__lean_List_mapM_loop___at___00Zhc_Iops_HillisSteele_finalResolution___at___00Zhc_Iops_addHillisSteele___at___00__private_Zhc_Iops_AddHillisSteele_Impl_0__Zhc_Iops_smokeHs_spec__0_spec__4_spec__12___closed__1;
static lean_once_cell_t lp_zhc__lean_List_mapM_loop___at___00Zhc_Iops_HillisSteele_finalResolution___at___00Zhc_Iops_addHillisSteele___at___00__private_Zhc_Iops_AddHillisSteele_Impl_0__Zhc_Iops_smokeHs_spec__0_spec__4_spec__12___closed__2_once = LEAN_ONCE_CELL_INITIALIZER;
static lean_object* lp_zhc__lean_List_mapM_loop___at___00Zhc_Iops_HillisSteele_finalResolution___at___00Zhc_Iops_addHillisSteele___at___00__private_Zhc_Iops_AddHillisSteele_Impl_0__Zhc_Iops_smokeHs_spec__0_spec__4_spec__12___closed__2;
LEAN_EXPORT lean_object* lp_zhc__lean_List_mapM_loop___at___00Zhc_Iops_HillisSteele_finalResolution___at___00Zhc_Iops_addHillisSteele___at___00__private_Zhc_Iops_AddHillisSteele_Impl_0__Zhc_Iops_smokeHs_spec__0_spec__4_spec__12(lean_object*, lean_object*);
LEAN_EXPORT lean_object* lp_zhc__lean_List_mapTR_loop___at___00Zhc_Iops_HillisSteele_finalResolution___at___00Zhc_Iops_addHillisSteele___at___00__private_Zhc_Iops_AddHillisSteele_Impl_0__Zhc_Iops_smokeHs_spec__0_spec__4_spec__11(lean_object*, lean_object*);
LEAN_EXPORT lean_object* lp_zhc__lean_Zhc_Iops_HillisSteele_finalResolution___at___00Zhc_Iops_addHillisSteele___at___00__private_Zhc_Iops_AddHillisSteele_Impl_0__Zhc_Iops_smokeHs_spec__0_spec__4(lean_object*, lean_object*);
LEAN_EXPORT lean_object* lp_zhc__lean_List_forIn_x27_loop___at___00Zhc_Iops_HillisSteele_joinBlocks___at___00Zhc_Iops_addHillisSteele___at___00__private_Zhc_Iops_AddHillisSteele_Impl_0__Zhc_Iops_smokeHs_spec__0_spec__6_spec__17___redArg(lean_object*, lean_object*);
LEAN_EXPORT lean_object* lp_zhc__lean_List_forIn_x27_loop___at___00Zhc_Iops_HillisSteele_joinBlocks___at___00Zhc_Iops_addHillisSteele___at___00__private_Zhc_Iops_AddHillisSteele_Impl_0__Zhc_Iops_smokeHs_spec__0_spec__6_spec__17___redArg___boxed(lean_object*, lean_object*);
LEAN_EXPORT lean_object* lp_zhc__lean_Zhc_Iops_HillisSteele_joinBlocks___at___00Zhc_Iops_addHillisSteele___at___00__private_Zhc_Iops_AddHillisSteele_Impl_0__Zhc_Iops_smokeHs_spec__0_spec__6(lean_object*, lean_object*);
LEAN_EXPORT lean_object* lp_zhc__lean_List_mapTR_loop___at___00Zhc_Iops_HillisSteele_groupCarries___at___00Zhc_Iops_addHillisSteele___at___00__private_Zhc_Iops_AddHillisSteele_Impl_0__Zhc_Iops_smokeHs_spec__0_spec__3_spec__7(lean_object*, lean_object*);
LEAN_EXPORT lean_object* lp_zhc__lean_List_mapM_loop___at___00Zhc_Iops_HillisSteele_hsStage___at___00Zhc_Iops_HillisSteele_groupCarries___at___00Zhc_Iops_addHillisSteele___at___00__private_Zhc_Iops_AddHillisSteele_Impl_0__Zhc_Iops_smokeHs_spec__0_spec__3_spec__8_spec__9(lean_object*, lean_object*, lean_object*);
LEAN_EXPORT lean_object* lp_zhc__lean_List_mapM_loop___at___00Zhc_Iops_HillisSteele_hsStage___at___00Zhc_Iops_HillisSteele_groupCarries___at___00Zhc_Iops_addHillisSteele___at___00__private_Zhc_Iops_AddHillisSteele_Impl_0__Zhc_Iops_smokeHs_spec__0_spec__3_spec__8_spec__9___boxed(lean_object*, lean_object*, lean_object*);
LEAN_EXPORT lean_object* lp_zhc__lean_Zhc_Iops_HillisSteele_hsStage___at___00Zhc_Iops_HillisSteele_groupCarries___at___00Zhc_Iops_addHillisSteele___at___00__private_Zhc_Iops_AddHillisSteele_Impl_0__Zhc_Iops_smokeHs_spec__0_spec__3_spec__8(lean_object*, lean_object*);
LEAN_EXPORT lean_object* lp_zhc__lean_Zhc_Iops_HillisSteele_hsStage___at___00Zhc_Iops_HillisSteele_groupCarries___at___00Zhc_Iops_addHillisSteele___at___00__private_Zhc_Iops_AddHillisSteele_Impl_0__Zhc_Iops_smokeHs_spec__0_spec__3_spec__8___boxed(lean_object*, lean_object*);
LEAN_EXPORT lean_object* lp_zhc__lean_List_forIn_x27_loop___at___00Zhc_Iops_HillisSteele_groupCarries___at___00Zhc_Iops_addHillisSteele___at___00__private_Zhc_Iops_AddHillisSteele_Impl_0__Zhc_Iops_smokeHs_spec__0_spec__3_spec__9___redArg(lean_object*, lean_object*);
LEAN_EXPORT lean_object* lp_zhc__lean_List_forIn_x27_loop___at___00Zhc_Iops_HillisSteele_groupCarries___at___00Zhc_Iops_addHillisSteele___at___00__private_Zhc_Iops_AddHillisSteele_Impl_0__Zhc_Iops_smokeHs_spec__0_spec__3_spec__9___redArg___boxed(lean_object*, lean_object*);
LEAN_EXPORT lean_object* lp_zhc__lean_Zhc_Iops_HillisSteele_groupCarries___at___00Zhc_Iops_addHillisSteele___at___00__private_Zhc_Iops_AddHillisSteele_Impl_0__Zhc_Iops_smokeHs_spec__0_spec__3(lean_object*);
LEAN_EXPORT lean_object* lp_zhc__lean_List_mapM_loop___at___00Zhc_Iops_HillisSteele_propagate___at___00Zhc_Iops_addHillisSteele___at___00__private_Zhc_Iops_AddHillisSteele_Impl_0__Zhc_Iops_smokeHs_spec__0_spec__5_spec__14(lean_object*, lean_object*);
LEAN_EXPORT lean_object* lp_zhc__lean_List_mapM_loop___at___00Zhc_Iops_HillisSteele_propagate___at___00Zhc_Iops_addHillisSteele___at___00__private_Zhc_Iops_AddHillisSteele_Impl_0__Zhc_Iops_smokeHs_spec__0_spec__5_spec__15(lean_object*, lean_object*);
static lean_once_cell_t lp_zhc__lean_Zhc_Iops_HillisSteele_propagate___at___00Zhc_Iops_addHillisSteele___at___00__private_Zhc_Iops_AddHillisSteele_Impl_0__Zhc_Iops_smokeHs_spec__0_spec__5___closed__0_once = LEAN_ONCE_CELL_INITIALIZER;
static lean_object* lp_zhc__lean_Zhc_Iops_HillisSteele_propagate___at___00Zhc_Iops_addHillisSteele___at___00__private_Zhc_Iops_AddHillisSteele_Impl_0__Zhc_Iops_smokeHs_spec__0_spec__5___closed__0;
LEAN_EXPORT lean_object* lp_zhc__lean_Zhc_Iops_HillisSteele_propagate___at___00Zhc_Iops_addHillisSteele___at___00__private_Zhc_Iops_AddHillisSteele_Impl_0__Zhc_Iops_smokeHs_spec__0_spec__5(lean_object*, lean_object*);
LEAN_EXPORT lean_object* lp_zhc__lean_List_mapM_loop___at___00Zhc_Iops_HillisSteele_groupStates___at___00Zhc_Iops_addHillisSteele___at___00__private_Zhc_Iops_AddHillisSteele_Impl_0__Zhc_Iops_smokeHs_spec__0_spec__2_spec__5___lam__0(lean_object*, lean_object*, lean_object*, lean_object*);
static lean_once_cell_t lp_zhc__lean_List_mapM_loop___at___00Zhc_Iops_HillisSteele_groupStates___at___00Zhc_Iops_addHillisSteele___at___00__private_Zhc_Iops_AddHillisSteele_Impl_0__Zhc_Iops_smokeHs_spec__0_spec__2_spec__5___closed__0_once = LEAN_ONCE_CELL_INITIALIZER;
static lean_object* lp_zhc__lean_List_mapM_loop___at___00Zhc_Iops_HillisSteele_groupStates___at___00Zhc_Iops_addHillisSteele___at___00__private_Zhc_Iops_AddHillisSteele_Impl_0__Zhc_Iops_smokeHs_spec__0_spec__2_spec__5___closed__0;
LEAN_EXPORT lean_object* lp_zhc__lean_List_mapM_loop___at___00Zhc_Iops_HillisSteele_groupStates___at___00Zhc_Iops_addHillisSteele___at___00__private_Zhc_Iops_AddHillisSteele_Impl_0__Zhc_Iops_smokeHs_spec__0_spec__2_spec__5(lean_object*, lean_object*);
LEAN_EXPORT lean_object* lp_zhc__lean_Zhc_Iops_HillisSteele_groupStates___at___00Zhc_Iops_addHillisSteele___at___00__private_Zhc_Iops_AddHillisSteele_Impl_0__Zhc_Iops_smokeHs_spec__0_spec__2(lean_object*);
LEAN_EXPORT lean_object* lp_zhc__lean_List_mapM_loop___at___00Zhc_Iops_HillisSteele_rawSums___at___00Zhc_Iops_addHillisSteele___at___00__private_Zhc_Iops_AddHillisSteele_Impl_0__Zhc_Iops_smokeHs_spec__0_spec__0_spec__1___redArg(lean_object*, lean_object*, lean_object*, lean_object*);
LEAN_EXPORT lean_object* lp_zhc__lean_List_mapM_loop___at___00Zhc_Iops_HillisSteele_rawSums___at___00Zhc_Iops_addHillisSteele___at___00__private_Zhc_Iops_AddHillisSteele_Impl_0__Zhc_Iops_smokeHs_spec__0_spec__0_spec__1___redArg___boxed(lean_object*, lean_object*, lean_object*, lean_object*);
static lean_once_cell_t lp_zhc__lean_Zhc_Iops_HillisSteele_rawSums___at___00Zhc_Iops_addHillisSteele___at___00__private_Zhc_Iops_AddHillisSteele_Impl_0__Zhc_Iops_smokeHs_spec__0_spec__0___closed__0_once = LEAN_ONCE_CELL_INITIALIZER;
static lean_object* lp_zhc__lean_Zhc_Iops_HillisSteele_rawSums___at___00Zhc_Iops_addHillisSteele___at___00__private_Zhc_Iops_AddHillisSteele_Impl_0__Zhc_Iops_smokeHs_spec__0_spec__0___closed__0;
LEAN_EXPORT lean_object* lp_zhc__lean_Zhc_Iops_HillisSteele_rawSums___at___00Zhc_Iops_addHillisSteele___at___00__private_Zhc_Iops_AddHillisSteele_Impl_0__Zhc_Iops_smokeHs_spec__0_spec__0(lean_object*, lean_object*, lean_object*);
LEAN_EXPORT lean_object* lp_zhc__lean_Zhc_Iops_HillisSteele_rawSums___at___00Zhc_Iops_addHillisSteele___at___00__private_Zhc_Iops_AddHillisSteele_Impl_0__Zhc_Iops_smokeHs_spec__0_spec__0___boxed(lean_object*, lean_object*, lean_object*);
static lean_once_cell_t lp_zhc__lean_List_mapM_loop___at___00Zhc_Iops_HillisSteele_blockStates___at___00Zhc_Iops_addHillisSteele___at___00__private_Zhc_Iops_AddHillisSteele_Impl_0__Zhc_Iops_smokeHs_spec__0_spec__1_spec__3___closed__0_once = LEAN_ONCE_CELL_INITIALIZER;
static lean_object* lp_zhc__lean_List_mapM_loop___at___00Zhc_Iops_HillisSteele_blockStates___at___00Zhc_Iops_addHillisSteele___at___00__private_Zhc_Iops_AddHillisSteele_Impl_0__Zhc_Iops_smokeHs_spec__0_spec__1_spec__3___closed__0;
LEAN_EXPORT lean_object* lp_zhc__lean_List_mapM_loop___at___00Zhc_Iops_HillisSteele_blockStates___at___00Zhc_Iops_addHillisSteele___at___00__private_Zhc_Iops_AddHillisSteele_Impl_0__Zhc_Iops_smokeHs_spec__0_spec__1_spec__3(lean_object*, lean_object*);
LEAN_EXPORT lean_object* lp_zhc__lean_Zhc_Iops_HillisSteele_blockStates___at___00Zhc_Iops_addHillisSteele___at___00__private_Zhc_Iops_AddHillisSteele_Impl_0__Zhc_Iops_smokeHs_spec__0_spec__1(lean_object*);
LEAN_EXPORT lean_object* lp_zhc__lean_Zhc_Iops_addHillisSteele___at___00__private_Zhc_Iops_AddHillisSteele_Impl_0__Zhc_Iops_smokeHs_spec__0(lean_object*, lean_object*, lean_object*);
LEAN_EXPORT lean_object* lp_zhc__lean_Zhc_Iops_addHillisSteele___at___00__private_Zhc_Iops_AddHillisSteele_Impl_0__Zhc_Iops_smokeHs_spec__0___boxed(lean_object*, lean_object*, lean_object*);
LEAN_EXPORT lean_object* lp_zhc__lean___private_Zhc_Iops_AddHillisSteele_Impl_0__Zhc_Iops_smokeHs(lean_object*, lean_object*, lean_object*);
LEAN_EXPORT lean_object* lp_zhc__lean___private_Zhc_Iops_AddHillisSteele_Impl_0__Zhc_Iops_smokeHs___boxed(lean_object*, lean_object*, lean_object*);
LEAN_EXPORT lean_object* lp_zhc__lean_List_mapM_loop___at___00Zhc_Iops_HillisSteele_rawSums___at___00Zhc_Iops_addHillisSteele___at___00__private_Zhc_Iops_AddHillisSteele_Impl_0__Zhc_Iops_smokeHs_spec__0_spec__0_spec__1(lean_object*, lean_object*, lean_object*, lean_object*, lean_object*);
LEAN_EXPORT lean_object* lp_zhc__lean_List_mapM_loop___at___00Zhc_Iops_HillisSteele_rawSums___at___00Zhc_Iops_addHillisSteele___at___00__private_Zhc_Iops_AddHillisSteele_Impl_0__Zhc_Iops_smokeHs_spec__0_spec__0_spec__1___boxed(lean_object*, lean_object*, lean_object*, lean_object*, lean_object*);
LEAN_EXPORT lean_object* lp_zhc__lean_List_forIn_x27_loop___at___00Zhc_Iops_HillisSteele_groupCarries___at___00Zhc_Iops_addHillisSteele___at___00__private_Zhc_Iops_AddHillisSteele_Impl_0__Zhc_Iops_smokeHs_spec__0_spec__3_spec__9(lean_object*, lean_object*, lean_object*, lean_object*);
LEAN_EXPORT lean_object* lp_zhc__lean_List_forIn_x27_loop___at___00Zhc_Iops_HillisSteele_groupCarries___at___00Zhc_Iops_addHillisSteele___at___00__private_Zhc_Iops_AddHillisSteele_Impl_0__Zhc_Iops_smokeHs_spec__0_spec__3_spec__9___boxed(lean_object*, lean_object*, lean_object*, lean_object*);
LEAN_EXPORT lean_object* lp_zhc__lean_List_forIn_x27_loop___at___00Zhc_Iops_HillisSteele_joinBlocks___at___00Zhc_Iops_addHillisSteele___at___00__private_Zhc_Iops_AddHillisSteele_Impl_0__Zhc_Iops_smokeHs_spec__0_spec__6_spec__17(lean_object*, lean_object*, lean_object*, lean_object*, lean_object*);
LEAN_EXPORT lean_object* lp_zhc__lean_List_forIn_x27_loop___at___00Zhc_Iops_HillisSteele_joinBlocks___at___00Zhc_Iops_addHillisSteele___at___00__private_Zhc_Iops_AddHillisSteele_Impl_0__Zhc_Iops_smokeHs_spec__0_spec__6_spec__17___boxed(lean_object*, lean_object*, lean_object*, lean_object*, lean_object*);
LEAN_EXPORT lean_object* lp_zhc__lean_List_mapM_loop___at___00Zhc_Iops_HillisSteele_hsStage___at___00Zhc_Iops_HillisSteele_groupCarries___at___00Zhc_Iops_addHillisSteele___at___00Zhc_Iops_addHillisSteeleCircuit_spec__0_spec__3_spec__7_spec__8(lean_object*, lean_object*, lean_object*, lean_object*);
LEAN_EXPORT lean_object* lp_zhc__lean_List_mapM_loop___at___00Zhc_Iops_HillisSteele_hsStage___at___00Zhc_Iops_HillisSteele_groupCarries___at___00Zhc_Iops_addHillisSteele___at___00Zhc_Iops_addHillisSteeleCircuit_spec__0_spec__3_spec__7_spec__8___boxed(lean_object*, lean_object*, lean_object*, lean_object*, lean_object*);
LEAN_EXPORT lean_object* lp_zhc__lean_Zhc_Iops_HillisSteele_hsStage___at___00Zhc_Iops_HillisSteele_groupCarries___at___00Zhc_Iops_addHillisSteele___at___00Zhc_Iops_addHillisSteeleCircuit_spec__0_spec__3_spec__7(lean_object*, lean_object*, lean_object*);
LEAN_EXPORT lean_object* lp_zhc__lean_Zhc_Iops_HillisSteele_hsStage___at___00Zhc_Iops_HillisSteele_groupCarries___at___00Zhc_Iops_addHillisSteele___at___00Zhc_Iops_addHillisSteeleCircuit_spec__0_spec__3_spec__7___boxed(lean_object*, lean_object*, lean_object*, lean_object*);
LEAN_EXPORT lean_object* lp_zhc__lean_List_forIn_x27_loop___at___00Zhc_Iops_HillisSteele_groupCarries___at___00Zhc_Iops_addHillisSteele___at___00Zhc_Iops_addHillisSteeleCircuit_spec__0_spec__3_spec__8___redArg(lean_object*, lean_object*, lean_object*);
LEAN_EXPORT lean_object* lp_zhc__lean_List_forIn_x27_loop___at___00Zhc_Iops_HillisSteele_groupCarries___at___00Zhc_Iops_addHillisSteele___at___00Zhc_Iops_addHillisSteeleCircuit_spec__0_spec__3_spec__8___redArg___boxed(lean_object*, lean_object*, lean_object*, lean_object*);
LEAN_EXPORT lean_object* lp_zhc__lean_Zhc_Iops_HillisSteele_groupCarries___at___00Zhc_Iops_addHillisSteele___at___00Zhc_Iops_addHillisSteeleCircuit_spec__0_spec__3(lean_object*, lean_object*);
LEAN_EXPORT lean_object* lp_zhc__lean_Zhc_Iops_HillisSteele_groupCarries___at___00Zhc_Iops_addHillisSteele___at___00Zhc_Iops_addHillisSteeleCircuit_spec__0_spec__3___boxed(lean_object*, lean_object*, lean_object*);
LEAN_EXPORT lean_object* lp_zhc__lean_List_mapM_loop___at___00Zhc_Iops_HillisSteele_propagate___at___00Zhc_Iops_addHillisSteele___at___00Zhc_Iops_addHillisSteeleCircuit_spec__0_spec__5_spec__13(lean_object*, lean_object*, lean_object*);
LEAN_EXPORT lean_object* lp_zhc__lean_List_mapM_loop___at___00Zhc_Iops_HillisSteele_propagate___at___00Zhc_Iops_addHillisSteele___at___00Zhc_Iops_addHillisSteeleCircuit_spec__0_spec__5_spec__13___boxed(lean_object*, lean_object*, lean_object*, lean_object*);
LEAN_EXPORT lean_object* lp_zhc__lean_List_mapM_loop___at___00Zhc_Iops_HillisSteele_propagate___at___00Zhc_Iops_addHillisSteele___at___00Zhc_Iops_addHillisSteeleCircuit_spec__0_spec__5_spec__12(lean_object*, lean_object*, lean_object*);
LEAN_EXPORT lean_object* lp_zhc__lean_List_mapM_loop___at___00Zhc_Iops_HillisSteele_propagate___at___00Zhc_Iops_addHillisSteele___at___00Zhc_Iops_addHillisSteeleCircuit_spec__0_spec__5_spec__12___boxed(lean_object*, lean_object*, lean_object*, lean_object*);
LEAN_EXPORT lean_object* lp_zhc__lean_Zhc_Iops_HillisSteele_propagate___at___00Zhc_Iops_addHillisSteele___at___00Zhc_Iops_addHillisSteeleCircuit_spec__0_spec__5(lean_object*, lean_object*, lean_object*);
LEAN_EXPORT lean_object* lp_zhc__lean_Zhc_Iops_HillisSteele_propagate___at___00Zhc_Iops_addHillisSteele___at___00Zhc_Iops_addHillisSteeleCircuit_spec__0_spec__5___boxed(lean_object*, lean_object*, lean_object*, lean_object*);
LEAN_EXPORT lean_object* lp_zhc__lean_List_forIn_x27_loop___at___00Zhc_Iops_HillisSteele_joinBlocks___at___00Zhc_Iops_addHillisSteele___at___00Zhc_Iops_addHillisSteeleCircuit_spec__0_spec__6_spec__15___redArg(lean_object*, lean_object*, lean_object*);
LEAN_EXPORT lean_object* lp_zhc__lean_List_forIn_x27_loop___at___00Zhc_Iops_HillisSteele_joinBlocks___at___00Zhc_Iops_addHillisSteele___at___00Zhc_Iops_addHillisSteeleCircuit_spec__0_spec__6_spec__15___redArg___boxed(lean_object*, lean_object*, lean_object*, lean_object*);
LEAN_EXPORT lean_object* lp_zhc__lean_Zhc_Iops_HillisSteele_joinBlocks___at___00Zhc_Iops_addHillisSteele___at___00Zhc_Iops_addHillisSteeleCircuit_spec__0_spec__6(lean_object*, lean_object*, lean_object*);
LEAN_EXPORT lean_object* lp_zhc__lean_Zhc_Iops_HillisSteele_joinBlocks___at___00Zhc_Iops_addHillisSteele___at___00Zhc_Iops_addHillisSteeleCircuit_spec__0_spec__6___boxed(lean_object*, lean_object*, lean_object*, lean_object*);
LEAN_EXPORT lean_object* lp_zhc__lean_List_mapM_loop___at___00Zhc_Iops_HillisSteele_groupStates___at___00Zhc_Iops_addHillisSteele___at___00Zhc_Iops_addHillisSteeleCircuit_spec__0_spec__2_spec__5___lam__0(lean_object*, lean_object*, lean_object*, lean_object*, lean_object*);
LEAN_EXPORT lean_object* lp_zhc__lean_List_mapM_loop___at___00Zhc_Iops_HillisSteele_groupStates___at___00Zhc_Iops_addHillisSteele___at___00Zhc_Iops_addHillisSteeleCircuit_spec__0_spec__2_spec__5___lam__0___boxed(lean_object*, lean_object*, lean_object*, lean_object*, lean_object*, lean_object*);
LEAN_EXPORT lean_object* lp_zhc__lean_List_mapM_loop___at___00Zhc_Iops_HillisSteele_groupStates___at___00Zhc_Iops_addHillisSteele___at___00Zhc_Iops_addHillisSteeleCircuit_spec__0_spec__2_spec__5(lean_object*, lean_object*, lean_object*);
LEAN_EXPORT lean_object* lp_zhc__lean_List_mapM_loop___at___00Zhc_Iops_HillisSteele_groupStates___at___00Zhc_Iops_addHillisSteele___at___00Zhc_Iops_addHillisSteeleCircuit_spec__0_spec__2_spec__5___boxed(lean_object*, lean_object*, lean_object*, lean_object*);
LEAN_EXPORT lean_object* lp_zhc__lean_Zhc_Iops_HillisSteele_groupStates___at___00Zhc_Iops_addHillisSteele___at___00Zhc_Iops_addHillisSteeleCircuit_spec__0_spec__2(lean_object*, lean_object*);
LEAN_EXPORT lean_object* lp_zhc__lean_Zhc_Iops_HillisSteele_groupStates___at___00Zhc_Iops_addHillisSteele___at___00Zhc_Iops_addHillisSteeleCircuit_spec__0_spec__2___boxed(lean_object*, lean_object*, lean_object*);
LEAN_EXPORT lean_object* lp_zhc__lean_List_mapM_loop___at___00Zhc_Iops_HillisSteele_blockStates___at___00Zhc_Iops_addHillisSteele___at___00Zhc_Iops_addHillisSteeleCircuit_spec__0_spec__1_spec__3(lean_object*, lean_object*, lean_object*);
LEAN_EXPORT lean_object* lp_zhc__lean_List_mapM_loop___at___00Zhc_Iops_HillisSteele_blockStates___at___00Zhc_Iops_addHillisSteele___at___00Zhc_Iops_addHillisSteeleCircuit_spec__0_spec__1_spec__3___boxed(lean_object*, lean_object*, lean_object*, lean_object*);
LEAN_EXPORT lean_object* lp_zhc__lean_Zhc_Iops_HillisSteele_blockStates___at___00Zhc_Iops_addHillisSteele___at___00Zhc_Iops_addHillisSteeleCircuit_spec__0_spec__1(lean_object*, lean_object*);
LEAN_EXPORT lean_object* lp_zhc__lean_Zhc_Iops_HillisSteele_blockStates___at___00Zhc_Iops_addHillisSteele___at___00Zhc_Iops_addHillisSteeleCircuit_spec__0_spec__1___boxed(lean_object*, lean_object*, lean_object*);
LEAN_EXPORT lean_object* lp_zhc__lean_List_mapM_loop___at___00Zhc_Iops_HillisSteele_finalResolution___at___00Zhc_Iops_addHillisSteele___at___00Zhc_Iops_addHillisSteeleCircuit_spec__0_spec__4_spec__10(lean_object*, lean_object*, lean_object*);
LEAN_EXPORT lean_object* lp_zhc__lean_List_mapM_loop___at___00Zhc_Iops_HillisSteele_finalResolution___at___00Zhc_Iops_addHillisSteele___at___00Zhc_Iops_addHillisSteeleCircuit_spec__0_spec__4_spec__10___boxed(lean_object*, lean_object*, lean_object*, lean_object*);
LEAN_EXPORT lean_object* lp_zhc__lean_Zhc_Iops_HillisSteele_finalResolution___at___00Zhc_Iops_addHillisSteele___at___00Zhc_Iops_addHillisSteeleCircuit_spec__0_spec__4(lean_object*, lean_object*, lean_object*);
LEAN_EXPORT lean_object* lp_zhc__lean_Zhc_Iops_HillisSteele_finalResolution___at___00Zhc_Iops_addHillisSteele___at___00Zhc_Iops_addHillisSteeleCircuit_spec__0_spec__4___boxed(lean_object*, lean_object*, lean_object*, lean_object*);
LEAN_EXPORT lean_object* lp_zhc__lean_List_mapM_loop___at___00Zhc_Iops_HillisSteele_rawSums___at___00Zhc_Iops_addHillisSteele___at___00Zhc_Iops_addHillisSteeleCircuit_spec__0_spec__0_spec__1___redArg(lean_object*, lean_object*, lean_object*, lean_object*, lean_object*);
LEAN_EXPORT lean_object* lp_zhc__lean_List_mapM_loop___at___00Zhc_Iops_HillisSteele_rawSums___at___00Zhc_Iops_addHillisSteele___at___00Zhc_Iops_addHillisSteeleCircuit_spec__0_spec__0_spec__1___redArg___boxed(lean_object*, lean_object*, lean_object*, lean_object*, lean_object*, lean_object*);
LEAN_EXPORT lean_object* lp_zhc__lean_Zhc_Iops_HillisSteele_rawSums___at___00Zhc_Iops_addHillisSteele___at___00Zhc_Iops_addHillisSteeleCircuit_spec__0_spec__0(lean_object*, lean_object*, lean_object*, lean_object*);
LEAN_EXPORT lean_object* lp_zhc__lean_Zhc_Iops_HillisSteele_rawSums___at___00Zhc_Iops_addHillisSteele___at___00Zhc_Iops_addHillisSteeleCircuit_spec__0_spec__0___boxed(lean_object*, lean_object*, lean_object*, lean_object*, lean_object*);
LEAN_EXPORT lean_object* lp_zhc__lean_Zhc_Iops_addHillisSteele___at___00Zhc_Iops_addHillisSteeleCircuit_spec__0(lean_object*, lean_object*, lean_object*, lean_object*);
LEAN_EXPORT lean_object* lp_zhc__lean_Zhc_Iops_addHillisSteele___at___00Zhc_Iops_addHillisSteeleCircuit_spec__0___boxed(lean_object*, lean_object*, lean_object*, lean_object*, lean_object*);
LEAN_EXPORT lean_object* lp_zhc__lean_Zhc_Iops_addHillisSteeleCircuit___lam__0(lean_object*, lean_object*, lean_object*);
LEAN_EXPORT lean_object* lp_zhc__lean_Zhc_Iops_addHillisSteeleCircuit___lam__0___boxed(lean_object*, lean_object*, lean_object*, lean_object*);
LEAN_EXPORT lean_object* lp_zhc__lean_Zhc_Iops_addHillisSteeleCircuit(lean_object*);
LEAN_EXPORT lean_object* lp_zhc__lean_Zhc_Iops_addHillisSteeleCircuit___boxed(lean_object*, lean_object*);
LEAN_EXPORT lean_object* lp_zhc__lean_List_mapM_loop___at___00Zhc_Iops_HillisSteele_rawSums___at___00Zhc_Iops_addHillisSteele___at___00Zhc_Iops_addHillisSteeleCircuit_spec__0_spec__0_spec__1(lean_object*, lean_object*, lean_object*, lean_object*, lean_object*, lean_object*);
LEAN_EXPORT lean_object* lp_zhc__lean_List_mapM_loop___at___00Zhc_Iops_HillisSteele_rawSums___at___00Zhc_Iops_addHillisSteele___at___00Zhc_Iops_addHillisSteeleCircuit_spec__0_spec__0_spec__1___boxed(lean_object*, lean_object*, lean_object*, lean_object*, lean_object*, lean_object*, lean_object*);
LEAN_EXPORT lean_object* lp_zhc__lean_List_forIn_x27_loop___at___00Zhc_Iops_HillisSteele_groupCarries___at___00Zhc_Iops_addHillisSteele___at___00Zhc_Iops_addHillisSteeleCircuit_spec__0_spec__3_spec__8(lean_object*, lean_object*, lean_object*, lean_object*, lean_object*);
LEAN_EXPORT lean_object* lp_zhc__lean_List_forIn_x27_loop___at___00Zhc_Iops_HillisSteele_groupCarries___at___00Zhc_Iops_addHillisSteele___at___00Zhc_Iops_addHillisSteeleCircuit_spec__0_spec__3_spec__8___boxed(lean_object*, lean_object*, lean_object*, lean_object*, lean_object*, lean_object*);
LEAN_EXPORT lean_object* lp_zhc__lean_List_forIn_x27_loop___at___00Zhc_Iops_HillisSteele_joinBlocks___at___00Zhc_Iops_addHillisSteele___at___00Zhc_Iops_addHillisSteeleCircuit_spec__0_spec__6_spec__15(lean_object*, lean_object*, lean_object*, lean_object*, lean_object*, lean_object*);
LEAN_EXPORT lean_object* lp_zhc__lean_List_forIn_x27_loop___at___00Zhc_Iops_HillisSteele_joinBlocks___at___00Zhc_Iops_addHillisSteele___at___00Zhc_Iops_addHillisSteeleCircuit_spec__0_spec__6_spec__15___boxed(lean_object*, lean_object*, lean_object*, lean_object*, lean_object*, lean_object*, lean_object*);
LEAN_EXPORT lean_object* lp_zhc__lean_Zhc_Iops_HillisSteele_computeSize(lean_object* v_blockCount_1_){
_start:
{
lean_object* v___x_2_; lean_object* v___x_3_; lean_object* v___x_4_; lean_object* v___x_5_; lean_object* v___x_6_; lean_object* v___x_7_; lean_object* v___x_8_; 
v___x_2_ = lean_unsigned_to_nat(3u);
v___x_3_ = lean_nat_add(v_blockCount_1_, v___x_2_);
v___x_4_ = lean_unsigned_to_nat(4u);
v___x_5_ = lean_unsigned_to_nat(2u);
v___x_6_ = lean_nat_shiftr(v___x_3_, v___x_5_);
lean_dec(v___x_3_);
v___x_7_ = lean_nat_mul(v___x_6_, v___x_4_);
lean_dec(v___x_6_);
v___x_8_ = l_Nat_nextPowerOfTwo(v___x_7_);
lean_dec(v___x_7_);
return v___x_8_;
}
}
LEAN_EXPORT lean_object* lp_zhc__lean_Zhc_Iops_HillisSteele_computeSize___boxed(lean_object* v_blockCount_9_){
_start:
{
lean_object* v_res_10_; 
v_res_10_ = lp_zhc__lean_Zhc_Iops_HillisSteele_computeSize(v_blockCount_9_);
lean_dec(v_blockCount_9_);
return v_res_10_;
}
}
LEAN_EXPORT lean_object* lp_zhc__lean_Zhc_Iops_HillisSteele_nbStages(lean_object* v_nbGroups_11_){
_start:
{
lean_object* v___x_12_; 
v___x_12_ = lean_nat_log2(v_nbGroups_11_);
return v___x_12_;
}
}
LEAN_EXPORT lean_object* lp_zhc__lean_Zhc_Iops_HillisSteele_nbStages___boxed(lean_object* v_nbGroups_13_){
_start:
{
lean_object* v_res_14_; 
v_res_14_ = lp_zhc__lean_Zhc_Iops_HillisSteele_nbStages(v_nbGroups_13_);
lean_dec(v_nbGroups_13_);
return v_res_14_;
}
}
LEAN_EXPORT lean_object* lp_zhc__lean_Zhc_Iops_HillisSteele_chunk4___redArg(lean_object* v_x_15_){
_start:
{
if (lean_obj_tag(v_x_15_) == 1)
{
lean_object* v_tail_16_; 
v_tail_16_ = lean_ctor_get(v_x_15_, 1);
lean_inc(v_tail_16_);
if (lean_obj_tag(v_tail_16_) == 1)
{
lean_object* v_tail_17_; 
v_tail_17_ = lean_ctor_get(v_tail_16_, 1);
lean_inc(v_tail_17_);
if (lean_obj_tag(v_tail_17_) == 1)
{
lean_object* v_tail_18_; 
v_tail_18_ = lean_ctor_get(v_tail_17_, 1);
lean_inc(v_tail_18_);
if (lean_obj_tag(v_tail_18_) == 1)
{
lean_object* v_head_19_; lean_object* v___x_21_; uint8_t v_isShared_22_; uint8_t v_isSharedCheck_54_; 
v_head_19_ = lean_ctor_get(v_x_15_, 0);
v_isSharedCheck_54_ = !lean_is_exclusive(v_x_15_);
if (v_isSharedCheck_54_ == 0)
{
lean_object* v_unused_55_; 
v_unused_55_ = lean_ctor_get(v_x_15_, 1);
lean_dec(v_unused_55_);
v___x_21_ = v_x_15_;
v_isShared_22_ = v_isSharedCheck_54_;
goto v_resetjp_20_;
}
else
{
lean_inc(v_head_19_);
lean_dec(v_x_15_);
v___x_21_ = lean_box(0);
v_isShared_22_ = v_isSharedCheck_54_;
goto v_resetjp_20_;
}
v_resetjp_20_:
{
lean_object* v_head_23_; lean_object* v___x_25_; uint8_t v_isShared_26_; uint8_t v_isSharedCheck_52_; 
v_head_23_ = lean_ctor_get(v_tail_16_, 0);
v_isSharedCheck_52_ = !lean_is_exclusive(v_tail_16_);
if (v_isSharedCheck_52_ == 0)
{
lean_object* v_unused_53_; 
v_unused_53_ = lean_ctor_get(v_tail_16_, 1);
lean_dec(v_unused_53_);
v___x_25_ = v_tail_16_;
v_isShared_26_ = v_isSharedCheck_52_;
goto v_resetjp_24_;
}
else
{
lean_inc(v_head_23_);
lean_dec(v_tail_16_);
v___x_25_ = lean_box(0);
v_isShared_26_ = v_isSharedCheck_52_;
goto v_resetjp_24_;
}
v_resetjp_24_:
{
lean_object* v_head_27_; lean_object* v___x_29_; uint8_t v_isShared_30_; uint8_t v_isSharedCheck_50_; 
v_head_27_ = lean_ctor_get(v_tail_17_, 0);
v_isSharedCheck_50_ = !lean_is_exclusive(v_tail_17_);
if (v_isSharedCheck_50_ == 0)
{
lean_object* v_unused_51_; 
v_unused_51_ = lean_ctor_get(v_tail_17_, 1);
lean_dec(v_unused_51_);
v___x_29_ = v_tail_17_;
v_isShared_30_ = v_isSharedCheck_50_;
goto v_resetjp_28_;
}
else
{
lean_inc(v_head_27_);
lean_dec(v_tail_17_);
v___x_29_ = lean_box(0);
v_isShared_30_ = v_isSharedCheck_50_;
goto v_resetjp_28_;
}
v_resetjp_28_:
{
lean_object* v_head_31_; lean_object* v_tail_32_; lean_object* v___x_34_; uint8_t v_isShared_35_; uint8_t v_isSharedCheck_49_; 
v_head_31_ = lean_ctor_get(v_tail_18_, 0);
v_tail_32_ = lean_ctor_get(v_tail_18_, 1);
v_isSharedCheck_49_ = !lean_is_exclusive(v_tail_18_);
if (v_isSharedCheck_49_ == 0)
{
v___x_34_ = v_tail_18_;
v_isShared_35_ = v_isSharedCheck_49_;
goto v_resetjp_33_;
}
else
{
lean_inc(v_tail_32_);
lean_inc(v_head_31_);
lean_dec(v_tail_18_);
v___x_34_ = lean_box(0);
v_isShared_35_ = v_isSharedCheck_49_;
goto v_resetjp_33_;
}
v_resetjp_33_:
{
lean_object* v___x_37_; 
if (v_isShared_30_ == 0)
{
lean_ctor_set_tag(v___x_29_, 0);
lean_ctor_set(v___x_29_, 1, v_head_31_);
v___x_37_ = v___x_29_;
goto v_reusejp_36_;
}
else
{
lean_object* v_reuseFailAlloc_48_; 
v_reuseFailAlloc_48_ = lean_alloc_ctor(0, 2, 0);
lean_ctor_set(v_reuseFailAlloc_48_, 0, v_head_27_);
lean_ctor_set(v_reuseFailAlloc_48_, 1, v_head_31_);
v___x_37_ = v_reuseFailAlloc_48_;
goto v_reusejp_36_;
}
v_reusejp_36_:
{
lean_object* v___x_39_; 
if (v_isShared_26_ == 0)
{
lean_ctor_set_tag(v___x_25_, 0);
lean_ctor_set(v___x_25_, 1, v___x_37_);
v___x_39_ = v___x_25_;
goto v_reusejp_38_;
}
else
{
lean_object* v_reuseFailAlloc_47_; 
v_reuseFailAlloc_47_ = lean_alloc_ctor(0, 2, 0);
lean_ctor_set(v_reuseFailAlloc_47_, 0, v_head_23_);
lean_ctor_set(v_reuseFailAlloc_47_, 1, v___x_37_);
v___x_39_ = v_reuseFailAlloc_47_;
goto v_reusejp_38_;
}
v_reusejp_38_:
{
lean_object* v___x_41_; 
if (v_isShared_22_ == 0)
{
lean_ctor_set_tag(v___x_21_, 0);
lean_ctor_set(v___x_21_, 1, v___x_39_);
v___x_41_ = v___x_21_;
goto v_reusejp_40_;
}
else
{
lean_object* v_reuseFailAlloc_46_; 
v_reuseFailAlloc_46_ = lean_alloc_ctor(0, 2, 0);
lean_ctor_set(v_reuseFailAlloc_46_, 0, v_head_19_);
lean_ctor_set(v_reuseFailAlloc_46_, 1, v___x_39_);
v___x_41_ = v_reuseFailAlloc_46_;
goto v_reusejp_40_;
}
v_reusejp_40_:
{
lean_object* v___x_42_; lean_object* v___x_44_; 
v___x_42_ = lp_zhc__lean_Zhc_Iops_HillisSteele_chunk4___redArg(v_tail_32_);
if (v_isShared_35_ == 0)
{
lean_ctor_set(v___x_34_, 1, v___x_42_);
lean_ctor_set(v___x_34_, 0, v___x_41_);
v___x_44_ = v___x_34_;
goto v_reusejp_43_;
}
else
{
lean_object* v_reuseFailAlloc_45_; 
v_reuseFailAlloc_45_ = lean_alloc_ctor(1, 2, 0);
lean_ctor_set(v_reuseFailAlloc_45_, 0, v___x_41_);
lean_ctor_set(v_reuseFailAlloc_45_, 1, v___x_42_);
v___x_44_ = v_reuseFailAlloc_45_;
goto v_reusejp_43_;
}
v_reusejp_43_:
{
return v___x_44_;
}
}
}
}
}
}
}
}
}
else
{
lean_object* v___x_56_; 
lean_dec_ref_known(v_tail_17_, 2);
lean_dec(v_tail_18_);
lean_dec_ref_known(v_tail_16_, 2);
lean_dec_ref_known(v_x_15_, 2);
v___x_56_ = lean_box(0);
return v___x_56_;
}
}
else
{
lean_object* v___x_57_; 
lean_dec(v_tail_17_);
lean_dec_ref_known(v_tail_16_, 2);
lean_dec_ref_known(v_x_15_, 2);
v___x_57_ = lean_box(0);
return v___x_57_;
}
}
else
{
lean_object* v___x_58_; 
lean_dec(v_tail_16_);
lean_dec_ref_known(v_x_15_, 2);
v___x_58_ = lean_box(0);
return v___x_58_;
}
}
else
{
lean_object* v___x_59_; 
lean_dec(v_x_15_);
v___x_59_ = lean_box(0);
return v___x_59_;
}
}
}
LEAN_EXPORT lean_object* lp_zhc__lean_Zhc_Iops_HillisSteele_chunk4(lean_object* v_00_u03b1_60_, lean_object* v_x_61_){
_start:
{
lean_object* v___x_62_; 
v___x_62_ = lp_zhc__lean_Zhc_Iops_HillisSteele_chunk4___redArg(v_x_61_);
return v___x_62_;
}
}
LEAN_EXPORT lean_object* lp_zhc__lean_Zhc_Iops_HillisSteele_unchunk4___redArg(lean_object* v_x_63_){
_start:
{
if (lean_obj_tag(v_x_63_) == 0)
{
lean_object* v___x_64_; 
v___x_64_ = lean_box(0);
return v___x_64_;
}
else
{
lean_object* v_head_65_; lean_object* v_snd_66_; lean_object* v_snd_67_; lean_object* v_tail_68_; lean_object* v___x_70_; uint8_t v_isShared_71_; uint8_t v_isSharedCheck_103_; 
v_head_65_ = lean_ctor_get(v_x_63_, 0);
lean_inc(v_head_65_);
v_snd_66_ = lean_ctor_get(v_head_65_, 1);
lean_inc(v_snd_66_);
v_snd_67_ = lean_ctor_get(v_snd_66_, 1);
lean_inc(v_snd_67_);
v_tail_68_ = lean_ctor_get(v_x_63_, 1);
v_isSharedCheck_103_ = !lean_is_exclusive(v_x_63_);
if (v_isSharedCheck_103_ == 0)
{
lean_object* v_unused_104_; 
v_unused_104_ = lean_ctor_get(v_x_63_, 0);
lean_dec(v_unused_104_);
v___x_70_ = v_x_63_;
v_isShared_71_ = v_isSharedCheck_103_;
goto v_resetjp_69_;
}
else
{
lean_inc(v_tail_68_);
lean_dec(v_x_63_);
v___x_70_ = lean_box(0);
v_isShared_71_ = v_isSharedCheck_103_;
goto v_resetjp_69_;
}
v_resetjp_69_:
{
lean_object* v_fst_72_; lean_object* v___x_74_; uint8_t v_isShared_75_; uint8_t v_isSharedCheck_101_; 
v_fst_72_ = lean_ctor_get(v_head_65_, 0);
v_isSharedCheck_101_ = !lean_is_exclusive(v_head_65_);
if (v_isSharedCheck_101_ == 0)
{
lean_object* v_unused_102_; 
v_unused_102_ = lean_ctor_get(v_head_65_, 1);
lean_dec(v_unused_102_);
v___x_74_ = v_head_65_;
v_isShared_75_ = v_isSharedCheck_101_;
goto v_resetjp_73_;
}
else
{
lean_inc(v_fst_72_);
lean_dec(v_head_65_);
v___x_74_ = lean_box(0);
v_isShared_75_ = v_isSharedCheck_101_;
goto v_resetjp_73_;
}
v_resetjp_73_:
{
lean_object* v_fst_76_; lean_object* v___x_78_; uint8_t v_isShared_79_; uint8_t v_isSharedCheck_99_; 
v_fst_76_ = lean_ctor_get(v_snd_66_, 0);
v_isSharedCheck_99_ = !lean_is_exclusive(v_snd_66_);
if (v_isSharedCheck_99_ == 0)
{
lean_object* v_unused_100_; 
v_unused_100_ = lean_ctor_get(v_snd_66_, 1);
lean_dec(v_unused_100_);
v___x_78_ = v_snd_66_;
v_isShared_79_ = v_isSharedCheck_99_;
goto v_resetjp_77_;
}
else
{
lean_inc(v_fst_76_);
lean_dec(v_snd_66_);
v___x_78_ = lean_box(0);
v_isShared_79_ = v_isSharedCheck_99_;
goto v_resetjp_77_;
}
v_resetjp_77_:
{
lean_object* v_fst_80_; lean_object* v_snd_81_; lean_object* v___x_83_; uint8_t v_isShared_84_; uint8_t v_isSharedCheck_98_; 
v_fst_80_ = lean_ctor_get(v_snd_67_, 0);
v_snd_81_ = lean_ctor_get(v_snd_67_, 1);
v_isSharedCheck_98_ = !lean_is_exclusive(v_snd_67_);
if (v_isSharedCheck_98_ == 0)
{
v___x_83_ = v_snd_67_;
v_isShared_84_ = v_isSharedCheck_98_;
goto v_resetjp_82_;
}
else
{
lean_inc(v_snd_81_);
lean_inc(v_fst_80_);
lean_dec(v_snd_67_);
v___x_83_ = lean_box(0);
v_isShared_84_ = v_isSharedCheck_98_;
goto v_resetjp_82_;
}
v_resetjp_82_:
{
lean_object* v___x_85_; lean_object* v___x_87_; 
v___x_85_ = lp_zhc__lean_Zhc_Iops_HillisSteele_unchunk4___redArg(v_tail_68_);
if (v_isShared_71_ == 0)
{
lean_ctor_set(v___x_70_, 1, v___x_85_);
lean_ctor_set(v___x_70_, 0, v_snd_81_);
v___x_87_ = v___x_70_;
goto v_reusejp_86_;
}
else
{
lean_object* v_reuseFailAlloc_97_; 
v_reuseFailAlloc_97_ = lean_alloc_ctor(1, 2, 0);
lean_ctor_set(v_reuseFailAlloc_97_, 0, v_snd_81_);
lean_ctor_set(v_reuseFailAlloc_97_, 1, v___x_85_);
v___x_87_ = v_reuseFailAlloc_97_;
goto v_reusejp_86_;
}
v_reusejp_86_:
{
lean_object* v___x_89_; 
if (v_isShared_84_ == 0)
{
lean_ctor_set_tag(v___x_83_, 1);
lean_ctor_set(v___x_83_, 1, v___x_87_);
v___x_89_ = v___x_83_;
goto v_reusejp_88_;
}
else
{
lean_object* v_reuseFailAlloc_96_; 
v_reuseFailAlloc_96_ = lean_alloc_ctor(1, 2, 0);
lean_ctor_set(v_reuseFailAlloc_96_, 0, v_fst_80_);
lean_ctor_set(v_reuseFailAlloc_96_, 1, v___x_87_);
v___x_89_ = v_reuseFailAlloc_96_;
goto v_reusejp_88_;
}
v_reusejp_88_:
{
lean_object* v___x_91_; 
if (v_isShared_79_ == 0)
{
lean_ctor_set_tag(v___x_78_, 1);
lean_ctor_set(v___x_78_, 1, v___x_89_);
v___x_91_ = v___x_78_;
goto v_reusejp_90_;
}
else
{
lean_object* v_reuseFailAlloc_95_; 
v_reuseFailAlloc_95_ = lean_alloc_ctor(1, 2, 0);
lean_ctor_set(v_reuseFailAlloc_95_, 0, v_fst_76_);
lean_ctor_set(v_reuseFailAlloc_95_, 1, v___x_89_);
v___x_91_ = v_reuseFailAlloc_95_;
goto v_reusejp_90_;
}
v_reusejp_90_:
{
lean_object* v___x_93_; 
if (v_isShared_75_ == 0)
{
lean_ctor_set_tag(v___x_74_, 1);
lean_ctor_set(v___x_74_, 1, v___x_91_);
v___x_93_ = v___x_74_;
goto v_reusejp_92_;
}
else
{
lean_object* v_reuseFailAlloc_94_; 
v_reuseFailAlloc_94_ = lean_alloc_ctor(1, 2, 0);
lean_ctor_set(v_reuseFailAlloc_94_, 0, v_fst_72_);
lean_ctor_set(v_reuseFailAlloc_94_, 1, v___x_91_);
v___x_93_ = v_reuseFailAlloc_94_;
goto v_reusejp_92_;
}
v_reusejp_92_:
{
return v___x_93_;
}
}
}
}
}
}
}
}
}
}
}
LEAN_EXPORT lean_object* lp_zhc__lean_Zhc_Iops_HillisSteele_unchunk4(lean_object* v_00_u03b1_105_, lean_object* v_x_106_){
_start:
{
lean_object* v___x_107_; 
v___x_107_ = lp_zhc__lean_Zhc_Iops_HillisSteele_unchunk4___redArg(v_x_106_);
return v___x_107_;
}
}
LEAN_EXPORT lean_object* lp_zhc__lean_Zhc_Iops_HillisSteele_extractPropGroupLut___lam__0(lean_object* v___x_108_, lean_object* v_p_109_, lean_object* v___x_110_, lean_object* v___x_111_, lean_object* v_b_112_){
_start:
{
lean_object* v___y_114_; lean_object* v___x_118_; lean_object* v___x_119_; lean_object* v___x_120_; uint8_t v___x_121_; 
v___x_118_ = lean_nat_shiftr(v_b_112_, v___x_108_);
v___x_119_ = lean_nat_mod(v___x_118_, v___x_108_);
lean_dec(v___x_118_);
v___x_120_ = lean_unsigned_to_nat(1u);
v___x_121_ = lean_nat_dec_eq(v___x_119_, v___x_120_);
lean_dec(v___x_119_);
if (v___x_121_ == 0)
{
lean_object* v___x_122_; lean_object* v___x_123_; uint8_t v___x_124_; 
v___x_122_ = lean_nat_mod(v_b_112_, v___x_111_);
v___x_123_ = lean_unsigned_to_nat(3u);
v___x_124_ = lean_nat_dec_eq(v___x_122_, v___x_123_);
lean_dec(v___x_122_);
if (v___x_124_ == 0)
{
lean_object* v___x_125_; 
v___x_125_ = lean_unsigned_to_nat(0u);
v___y_114_ = v___x_125_;
goto v___jp_113_;
}
else
{
v___y_114_ = v___x_120_;
goto v___jp_113_;
}
}
else
{
v___y_114_ = v___x_108_;
goto v___jp_113_;
}
v___jp_113_:
{
lean_object* v___x_115_; lean_object* v___x_116_; lean_object* v___x_117_; 
v___x_115_ = lean_nat_pow(v___x_108_, v_p_109_);
v___x_116_ = lean_nat_mul(v___y_114_, v___x_115_);
lean_dec(v___x_115_);
v___x_117_ = lp_zhc__lean_Zhc_Model_CiphertextBlock_ofNat(v___x_110_, v___x_116_);
lean_dec(v___x_116_);
return v___x_117_;
}
}
}
LEAN_EXPORT lean_object* lp_zhc__lean_Zhc_Iops_HillisSteele_extractPropGroupLut___lam__0___boxed(lean_object* v___x_126_, lean_object* v_p_127_, lean_object* v___x_128_, lean_object* v___x_129_, lean_object* v_b_130_){
_start:
{
lean_object* v_res_131_; 
v_res_131_ = lp_zhc__lean_Zhc_Iops_HillisSteele_extractPropGroupLut___lam__0(v___x_126_, v_p_127_, v___x_128_, v___x_129_, v_b_130_);
lean_dec(v_b_130_);
lean_dec(v___x_129_);
lean_dec_ref(v___x_128_);
lean_dec(v_p_127_);
lean_dec(v___x_126_);
return v_res_131_;
}
}
LEAN_EXPORT lean_object* lp_zhc__lean_Zhc_Iops_HillisSteele_extractPropGroupLut(lean_object* v_p_135_){
_start:
{
lean_object* v___x_136_; lean_object* v___x_137_; lean_object* v___x_138_; lean_object* v___x_139_; lean_object* v___f_140_; lean_object* v___x_141_; lean_object* v___x_142_; lean_object* v___x_143_; 
v___x_136_ = lean_unsigned_to_nat(2u);
v___x_137_ = ((lean_object*)(lp_zhc__lean_Zhc_Iops_HillisSteele_extractPropGroupLut___closed__0));
v___x_138_ = ((lean_object*)(lp_zhc__lean_Zhc_Iops_HillisSteele_extractPropGroupLut___closed__1));
v___x_139_ = lean_unsigned_to_nat(4u);
lean_inc(v_p_135_);
v___f_140_ = lean_alloc_closure((void*)(lp_zhc__lean_Zhc_Iops_HillisSteele_extractPropGroupLut___lam__0___boxed), 5, 4);
lean_closure_set(v___f_140_, 0, v___x_136_);
lean_closure_set(v___f_140_, 1, v_p_135_);
lean_closure_set(v___f_140_, 2, v___x_137_);
lean_closure_set(v___f_140_, 3, v___x_139_);
v___x_141_ = l_Nat_reprFast(v_p_135_);
v___x_142_ = lean_string_append(v___x_138_, v___x_141_);
lean_dec_ref(v___x_141_);
v___x_143_ = lp_zhc__lean_Zhc_Model_Lut1_fromFn(v___x_137_, v___x_142_, v___f_140_);
return v___x_143_;
}
}
LEAN_EXPORT lean_object* lp_zhc__lean_Zhc_Iops_HillisSteele_solvePropGroupFinalLut___lam__0(lean_object* v_p_144_, lean_object* v___x_145_, lean_object* v___x_146_, lean_object* v_b_147_){
_start:
{
lean_object* v___x_148_; lean_object* v___x_149_; lean_object* v___x_150_; lean_object* v___x_151_; lean_object* v___x_152_; lean_object* v___x_153_; 
v___x_148_ = lean_unsigned_to_nat(1u);
v___x_149_ = lean_nat_add(v_p_144_, v___x_148_);
v___x_150_ = lean_nat_pow(v___x_145_, v___x_149_);
lean_dec(v___x_149_);
v___x_151_ = lean_nat_div(v_b_147_, v___x_150_);
lean_dec(v___x_150_);
v___x_152_ = lean_nat_mod(v___x_151_, v___x_145_);
lean_dec(v___x_151_);
v___x_153_ = lp_zhc__lean_Zhc_Model_CiphertextBlock_ofNat(v___x_146_, v___x_152_);
lean_dec(v___x_152_);
return v___x_153_;
}
}
LEAN_EXPORT lean_object* lp_zhc__lean_Zhc_Iops_HillisSteele_solvePropGroupFinalLut___lam__0___boxed(lean_object* v_p_154_, lean_object* v___x_155_, lean_object* v___x_156_, lean_object* v_b_157_){
_start:
{
lean_object* v_res_158_; 
v_res_158_ = lp_zhc__lean_Zhc_Iops_HillisSteele_solvePropGroupFinalLut___lam__0(v_p_154_, v___x_155_, v___x_156_, v_b_157_);
lean_dec(v_b_157_);
lean_dec_ref(v___x_156_);
lean_dec(v___x_155_);
lean_dec(v_p_154_);
return v_res_158_;
}
}
LEAN_EXPORT lean_object* lp_zhc__lean_Zhc_Iops_HillisSteele_solvePropGroupFinalLut(lean_object* v_p_160_){
_start:
{
lean_object* v___x_161_; lean_object* v___x_162_; lean_object* v___f_163_; lean_object* v___x_164_; lean_object* v___x_165_; lean_object* v___x_166_; lean_object* v___x_167_; 
v___x_161_ = lean_unsigned_to_nat(2u);
v___x_162_ = ((lean_object*)(lp_zhc__lean_Zhc_Iops_HillisSteele_extractPropGroupLut___closed__0));
lean_inc(v_p_160_);
v___f_163_ = lean_alloc_closure((void*)(lp_zhc__lean_Zhc_Iops_HillisSteele_solvePropGroupFinalLut___lam__0___boxed), 4, 3);
lean_closure_set(v___f_163_, 0, v_p_160_);
lean_closure_set(v___f_163_, 1, v___x_161_);
lean_closure_set(v___f_163_, 2, v___x_162_);
v___x_164_ = ((lean_object*)(lp_zhc__lean_Zhc_Iops_HillisSteele_solvePropGroupFinalLut___closed__0));
v___x_165_ = l_Nat_reprFast(v_p_160_);
v___x_166_ = lean_string_append(v___x_164_, v___x_165_);
lean_dec_ref(v___x_165_);
v___x_167_ = lp_zhc__lean_Zhc_Model_Lut1_fromFn(v___x_162_, v___x_166_, v___f_163_);
return v___x_167_;
}
}
LEAN_EXPORT lean_object* lp_zhc__lean_Zhc_Iops_HillisSteele_reduceCarryPadLut___lam__0(lean_object* v___x_168_, lean_object* v_b_169_){
_start:
{
lean_object* v___x_170_; uint8_t v___x_171_; 
v___x_170_ = lean_unsigned_to_nat(15u);
v___x_171_ = lean_nat_dec_eq(v_b_169_, v___x_170_);
if (v___x_171_ == 0)
{
lean_object* v___x_172_; lean_object* v___x_173_; 
v___x_172_ = lean_unsigned_to_nat(31u);
v___x_173_ = lp_zhc__lean_Zhc_Model_CiphertextBlock_ofNat(v___x_168_, v___x_172_);
return v___x_173_;
}
else
{
lean_object* v___x_174_; lean_object* v___x_175_; 
v___x_174_ = lean_unsigned_to_nat(0u);
v___x_175_ = lp_zhc__lean_Zhc_Model_CiphertextBlock_ofNat(v___x_168_, v___x_174_);
return v___x_175_;
}
}
}
LEAN_EXPORT lean_object* lp_zhc__lean_Zhc_Iops_HillisSteele_reduceCarryPadLut___lam__0___boxed(lean_object* v___x_176_, lean_object* v_b_177_){
_start:
{
lean_object* v_res_178_; 
v_res_178_ = lp_zhc__lean_Zhc_Iops_HillisSteele_reduceCarryPadLut___lam__0(v___x_176_, v_b_177_);
lean_dec(v_b_177_);
lean_dec_ref(v___x_176_);
return v_res_178_;
}
}
static lean_object* _init_lp_zhc__lean_Zhc_Iops_HillisSteele_reduceCarryPadLut___closed__2(void){
_start:
{
lean_object* v___f_182_; lean_object* v___x_183_; lean_object* v___x_184_; lean_object* v___x_185_; 
v___f_182_ = ((lean_object*)(lp_zhc__lean_Zhc_Iops_HillisSteele_reduceCarryPadLut___closed__0));
v___x_183_ = ((lean_object*)(lp_zhc__lean_Zhc_Iops_HillisSteele_reduceCarryPadLut___closed__1));
v___x_184_ = ((lean_object*)(lp_zhc__lean_Zhc_Iops_HillisSteele_extractPropGroupLut___closed__0));
v___x_185_ = lp_zhc__lean_Zhc_Model_Lut1_fromFn(v___x_184_, v___x_183_, v___f_182_);
return v___x_185_;
}
}
static lean_object* _init_lp_zhc__lean_Zhc_Iops_HillisSteele_reduceCarryPadLut(void){
_start:
{
lean_object* v___x_186_; 
v___x_186_ = lean_obj_once(&lp_zhc__lean_Zhc_Iops_HillisSteele_reduceCarryPadLut___closed__2, &lp_zhc__lean_Zhc_Iops_HillisSteele_reduceCarryPadLut___closed__2_once, _init_lp_zhc__lean_Zhc_Iops_HillisSteele_reduceCarryPadLut___closed__2);
return v___x_186_;
}
}
LEAN_EXPORT lean_object* lp_zhc__lean_Zhc_Iops_HillisSteele_solvePropLut___lam__0(lean_object* v___x_187_, lean_object* v___x_188_, lean_object* v_b_189_){
_start:
{
lean_object* v___x_190_; lean_object* v___x_191_; lean_object* v_msb_192_; lean_object* v___x_193_; uint8_t v___x_194_; 
v___x_190_ = lean_unsigned_to_nat(4u);
v___x_191_ = lean_nat_shiftr(v_b_189_, v___x_187_);
v_msb_192_ = lean_nat_mod(v___x_191_, v___x_190_);
lean_dec(v___x_191_);
v___x_193_ = lean_unsigned_to_nat(1u);
v___x_194_ = lean_nat_dec_eq(v_msb_192_, v___x_193_);
if (v___x_194_ == 0)
{
lean_object* v___x_195_; 
v___x_195_ = lp_zhc__lean_Zhc_Model_CiphertextBlock_ofNat(v___x_188_, v_msb_192_);
lean_dec(v_msb_192_);
return v___x_195_;
}
else
{
lean_object* v_lsb_196_; lean_object* v___x_197_; 
lean_dec(v_msb_192_);
v_lsb_196_ = lean_nat_mod(v_b_189_, v___x_190_);
v___x_197_ = lp_zhc__lean_Zhc_Model_CiphertextBlock_ofNat(v___x_188_, v_lsb_196_);
lean_dec(v_lsb_196_);
return v___x_197_;
}
}
}
LEAN_EXPORT lean_object* lp_zhc__lean_Zhc_Iops_HillisSteele_solvePropLut___lam__0___boxed(lean_object* v___x_198_, lean_object* v___x_199_, lean_object* v_b_200_){
_start:
{
lean_object* v_res_201_; 
v_res_201_ = lp_zhc__lean_Zhc_Iops_HillisSteele_solvePropLut___lam__0(v___x_198_, v___x_199_, v_b_200_);
lean_dec(v_b_200_);
lean_dec_ref(v___x_199_);
lean_dec(v___x_198_);
return v_res_201_;
}
}
static lean_object* _init_lp_zhc__lean_Zhc_Iops_HillisSteele_solvePropLut___closed__2(void){
_start:
{
lean_object* v___f_206_; lean_object* v___x_207_; lean_object* v___x_208_; lean_object* v___x_209_; 
v___f_206_ = ((lean_object*)(lp_zhc__lean_Zhc_Iops_HillisSteele_solvePropLut___closed__0));
v___x_207_ = ((lean_object*)(lp_zhc__lean_Zhc_Iops_HillisSteele_solvePropLut___closed__1));
v___x_208_ = ((lean_object*)(lp_zhc__lean_Zhc_Iops_HillisSteele_extractPropGroupLut___closed__0));
v___x_209_ = lp_zhc__lean_Zhc_Model_Lut1_fromFn(v___x_208_, v___x_207_, v___f_206_);
return v___x_209_;
}
}
static lean_object* _init_lp_zhc__lean_Zhc_Iops_HillisSteele_solvePropLut(void){
_start:
{
lean_object* v___x_210_; 
v___x_210_ = lean_obj_once(&lp_zhc__lean_Zhc_Iops_HillisSteele_solvePropLut___closed__2, &lp_zhc__lean_Zhc_Iops_HillisSteele_solvePropLut___closed__2_once, _init_lp_zhc__lean_Zhc_Iops_HillisSteele_solvePropLut___closed__2);
return v___x_210_;
}
}
LEAN_EXPORT lean_object* lp_zhc__lean_Zhc_Iops_HillisSteele_solvePropCarryLut___lam__0(lean_object* v___x_211_, lean_object* v___x_212_, lean_object* v_b_213_){
_start:
{
lean_object* v___x_214_; lean_object* v___x_215_; lean_object* v_msb_216_; lean_object* v___x_217_; uint8_t v___x_218_; 
v___x_214_ = lean_unsigned_to_nat(4u);
v___x_215_ = lean_nat_shiftr(v_b_213_, v___x_211_);
v_msb_216_ = lean_nat_mod(v___x_215_, v___x_214_);
lean_dec(v___x_215_);
v___x_217_ = lean_unsigned_to_nat(1u);
v___x_218_ = lean_nat_dec_eq(v_msb_216_, v___x_217_);
if (v___x_218_ == 0)
{
lean_object* v___x_219_; lean_object* v___x_220_; 
v___x_219_ = lean_nat_shiftr(v_msb_216_, v___x_217_);
lean_dec(v_msb_216_);
v___x_220_ = lp_zhc__lean_Zhc_Model_CiphertextBlock_ofNat(v___x_212_, v___x_219_);
lean_dec(v___x_219_);
return v___x_220_;
}
else
{
lean_object* v_lsb_221_; lean_object* v___x_222_; 
lean_dec(v_msb_216_);
v_lsb_221_ = lean_nat_mod(v_b_213_, v___x_214_);
v___x_222_ = lp_zhc__lean_Zhc_Model_CiphertextBlock_ofNat(v___x_212_, v_lsb_221_);
lean_dec(v_lsb_221_);
return v___x_222_;
}
}
}
LEAN_EXPORT lean_object* lp_zhc__lean_Zhc_Iops_HillisSteele_solvePropCarryLut___lam__0___boxed(lean_object* v___x_223_, lean_object* v___x_224_, lean_object* v_b_225_){
_start:
{
lean_object* v_res_226_; 
v_res_226_ = lp_zhc__lean_Zhc_Iops_HillisSteele_solvePropCarryLut___lam__0(v___x_223_, v___x_224_, v_b_225_);
lean_dec(v_b_225_);
lean_dec_ref(v___x_224_);
lean_dec(v___x_223_);
return v_res_226_;
}
}
static lean_object* _init_lp_zhc__lean_Zhc_Iops_HillisSteele_solvePropCarryLut___closed__2(void){
_start:
{
lean_object* v___f_231_; lean_object* v___x_232_; lean_object* v___x_233_; lean_object* v___x_234_; 
v___f_231_ = ((lean_object*)(lp_zhc__lean_Zhc_Iops_HillisSteele_solvePropCarryLut___closed__0));
v___x_232_ = ((lean_object*)(lp_zhc__lean_Zhc_Iops_HillisSteele_solvePropCarryLut___closed__1));
v___x_233_ = ((lean_object*)(lp_zhc__lean_Zhc_Iops_HillisSteele_extractPropGroupLut___closed__0));
v___x_234_ = lp_zhc__lean_Zhc_Model_Lut1_fromFn(v___x_233_, v___x_232_, v___f_231_);
return v___x_234_;
}
}
static lean_object* _init_lp_zhc__lean_Zhc_Iops_HillisSteele_solvePropCarryLut(void){
_start:
{
lean_object* v___x_235_; 
v___x_235_ = lean_obj_once(&lp_zhc__lean_Zhc_Iops_HillisSteele_solvePropCarryLut___closed__2, &lp_zhc__lean_Zhc_Iops_HillisSteele_solvePropCarryLut___closed__2_once, _init_lp_zhc__lean_Zhc_Iops_HillisSteele_solvePropCarryLut___closed__2);
return v___x_235_;
}
}
LEAN_EXPORT lean_object* lp_zhc__lean_Zhc_Iops_HillisSteele_msgOnlyLut___lam__0(lean_object* v___x_236_, lean_object* v_x_237_){
_start:
{
lean_object* v___x_238_; 
v___x_238_ = lp_zhc__lean_Zhc_Model_CiphertextBlock_doMaskMessage(v___x_236_, v_x_237_);
return v___x_238_;
}
}
LEAN_EXPORT lean_object* lp_zhc__lean_Zhc_Iops_HillisSteele_msgOnlyLut___lam__0___boxed(lean_object* v___x_239_, lean_object* v_x_240_){
_start:
{
lean_object* v_res_241_; 
v_res_241_ = lp_zhc__lean_Zhc_Iops_HillisSteele_msgOnlyLut___lam__0(v___x_239_, v_x_240_);
lean_dec(v_x_240_);
lean_dec_ref(v___x_239_);
return v_res_241_;
}
}
static lean_object* _init_lp_zhc__lean_Zhc_Iops_HillisSteele_msgOnlyLut___closed__2(void){
_start:
{
lean_object* v___f_245_; lean_object* v___x_246_; lean_object* v___x_247_; lean_object* v___x_248_; 
v___f_245_ = ((lean_object*)(lp_zhc__lean_Zhc_Iops_HillisSteele_msgOnlyLut___closed__0));
v___x_246_ = ((lean_object*)(lp_zhc__lean_Zhc_Iops_HillisSteele_msgOnlyLut___closed__1));
v___x_247_ = ((lean_object*)(lp_zhc__lean_Zhc_Iops_HillisSteele_extractPropGroupLut___closed__0));
v___x_248_ = lp_zhc__lean_Zhc_Model_Lut1_fromFn(v___x_247_, v___x_246_, v___f_245_);
return v___x_248_;
}
}
static lean_object* _init_lp_zhc__lean_Zhc_Iops_HillisSteele_msgOnlyLut(void){
_start:
{
lean_object* v___x_249_; 
v___x_249_ = lean_obj_once(&lp_zhc__lean_Zhc_Iops_HillisSteele_msgOnlyLut___closed__2, &lp_zhc__lean_Zhc_Iops_HillisSteele_msgOnlyLut___closed__2_once, _init_lp_zhc__lean_Zhc_Iops_HillisSteele_msgOnlyLut___closed__2);
return v___x_249_;
}
}
LEAN_EXPORT lean_object* lp_zhc__lean_Zhc_Iops_HillisSteele_rawSums___redArg___lam__0(lean_object* v_addCt_250_, lean_object* v_xi_251_, lean_object* v_yi_252_){
_start:
{
lean_object* v___x_253_; 
v___x_253_ = lean_apply_2(v_addCt_250_, v_xi_251_, v_yi_252_);
return v___x_253_;
}
}
LEAN_EXPORT lean_object* lp_zhc__lean_Zhc_Iops_HillisSteele_rawSums___redArg___lam__1(lean_object* v_addCt_254_, lean_object* v_extractCtBlock_255_, lean_object* v_si_256_, lean_object* v_y_257_, lean_object* v_i_258_, lean_object* v_toBind_259_, lean_object* v_xi_260_){
_start:
{
lean_object* v___f_261_; lean_object* v___x_262_; lean_object* v___x_263_; 
v___f_261_ = lean_alloc_closure((void*)(lp_zhc__lean_Zhc_Iops_HillisSteele_rawSums___redArg___lam__0), 3, 2);
lean_closure_set(v___f_261_, 0, v_addCt_254_);
lean_closure_set(v___f_261_, 1, v_xi_260_);
v___x_262_ = lean_apply_3(v_extractCtBlock_255_, v_si_256_, v_y_257_, v_i_258_);
v___x_263_ = lean_apply_4(v_toBind_259_, lean_box(0), lean_box(0), v___x_262_, v___f_261_);
return v___x_263_;
}
}
LEAN_EXPORT lean_object* lp_zhc__lean_Zhc_Iops_HillisSteele_rawSums___redArg___lam__2(lean_object* v_A_264_, lean_object* v_si_265_, lean_object* v_y_266_, lean_object* v_toBind_267_, lean_object* v_x_268_, lean_object* v_i_269_){
_start:
{
lean_object* v_addCt_270_; lean_object* v_extractCtBlock_271_; lean_object* v___f_272_; lean_object* v___x_273_; lean_object* v___x_274_; 
v_addCt_270_ = lean_ctor_get(v_A_264_, 3);
lean_inc(v_addCt_270_);
v_extractCtBlock_271_ = lean_ctor_get(v_A_264_, 11);
lean_inc_n(v_extractCtBlock_271_, 2);
lean_dec_ref(v_A_264_);
lean_inc(v_toBind_267_);
lean_inc(v_i_269_);
lean_inc(v_si_265_);
v___f_272_ = lean_alloc_closure((void*)(lp_zhc__lean_Zhc_Iops_HillisSteele_rawSums___redArg___lam__1), 7, 6);
lean_closure_set(v___f_272_, 0, v_addCt_270_);
lean_closure_set(v___f_272_, 1, v_extractCtBlock_271_);
lean_closure_set(v___f_272_, 2, v_si_265_);
lean_closure_set(v___f_272_, 3, v_y_266_);
lean_closure_set(v___f_272_, 4, v_i_269_);
lean_closure_set(v___f_272_, 5, v_toBind_267_);
v___x_273_ = lean_apply_3(v_extractCtBlock_271_, v_si_265_, v_x_268_, v_i_269_);
v___x_274_ = lean_apply_4(v_toBind_267_, lean_box(0), lean_box(0), v___x_273_, v___f_272_);
return v___x_274_;
}
}
LEAN_EXPORT lean_object* lp_zhc__lean_Zhc_Iops_HillisSteele_rawSums___redArg___lam__3(lean_object* v_si_275_, lean_object* v_sums_276_, lean_object* v_toPure_277_, lean_object* v_zero_278_){
_start:
{
lean_object* v___x_279_; lean_object* v___x_280_; lean_object* v___x_281_; lean_object* v___x_282_; lean_object* v___x_283_; 
v___x_279_ = lp_zhc__lean_Zhc_Iops_HillisSteele_computeSize(v_si_275_);
v___x_280_ = lean_nat_sub(v___x_279_, v_si_275_);
lean_dec(v___x_279_);
v___x_281_ = l_List_replicateTR___redArg(v___x_280_, v_zero_278_);
v___x_282_ = l_List_appendTR___redArg(v_sums_276_, v___x_281_);
v___x_283_ = lean_apply_2(v_toPure_277_, lean_box(0), v___x_282_);
return v___x_283_;
}
}
LEAN_EXPORT lean_object* lp_zhc__lean_Zhc_Iops_HillisSteele_rawSums___redArg___lam__3___boxed(lean_object* v_si_284_, lean_object* v_sums_285_, lean_object* v_toPure_286_, lean_object* v_zero_287_){
_start:
{
lean_object* v_res_288_; 
v_res_288_ = lp_zhc__lean_Zhc_Iops_HillisSteele_rawSums___redArg___lam__3(v_si_284_, v_sums_285_, v_toPure_286_, v_zero_287_);
lean_dec(v_si_284_);
return v_res_288_;
}
}
LEAN_EXPORT lean_object* lp_zhc__lean_Zhc_Iops_HillisSteele_rawSums___redArg___lam__4(lean_object* v_A_289_, lean_object* v_si_290_, lean_object* v_toPure_291_, lean_object* v_toBind_292_, lean_object* v_sums_293_){
_start:
{
lean_object* v_letCtBlock_294_; lean_object* v___f_295_; lean_object* v___x_296_; lean_object* v___x_297_; lean_object* v___x_298_; 
v_letCtBlock_294_ = lean_ctor_get(v_A_289_, 2);
lean_inc(v_letCtBlock_294_);
lean_dec_ref(v_A_289_);
v___f_295_ = lean_alloc_closure((void*)(lp_zhc__lean_Zhc_Iops_HillisSteele_rawSums___redArg___lam__3___boxed), 4, 3);
lean_closure_set(v___f_295_, 0, v_si_290_);
lean_closure_set(v___f_295_, 1, v_sums_293_);
lean_closure_set(v___f_295_, 2, v_toPure_291_);
v___x_296_ = lean_unsigned_to_nat(0u);
v___x_297_ = lean_apply_1(v_letCtBlock_294_, v___x_296_);
v___x_298_ = lean_apply_4(v_toBind_292_, lean_box(0), lean_box(0), v___x_297_, v___f_295_);
return v___x_298_;
}
}
LEAN_EXPORT lean_object* lp_zhc__lean_Zhc_Iops_HillisSteele_rawSums___redArg(lean_object* v_inst_299_, lean_object* v_A_300_, lean_object* v_si_301_, lean_object* v_x_302_, lean_object* v_y_303_){
_start:
{
lean_object* v_toApplicative_304_; lean_object* v_toBind_305_; lean_object* v_toPure_306_; lean_object* v___f_307_; lean_object* v___f_308_; lean_object* v___x_309_; lean_object* v___x_310_; lean_object* v___x_311_; lean_object* v___x_312_; 
v_toApplicative_304_ = lean_ctor_get(v_inst_299_, 0);
v_toBind_305_ = lean_ctor_get(v_inst_299_, 1);
lean_inc_n(v_toBind_305_, 3);
v_toPure_306_ = lean_ctor_get(v_toApplicative_304_, 1);
lean_inc_n(v_si_301_, 2);
lean_inc_ref(v_A_300_);
v___f_307_ = lean_alloc_closure((void*)(lp_zhc__lean_Zhc_Iops_HillisSteele_rawSums___redArg___lam__2), 6, 5);
lean_closure_set(v___f_307_, 0, v_A_300_);
lean_closure_set(v___f_307_, 1, v_si_301_);
lean_closure_set(v___f_307_, 2, v_y_303_);
lean_closure_set(v___f_307_, 3, v_toBind_305_);
lean_closure_set(v___f_307_, 4, v_x_302_);
lean_inc(v_toPure_306_);
v___f_308_ = lean_alloc_closure((void*)(lp_zhc__lean_Zhc_Iops_HillisSteele_rawSums___redArg___lam__4), 5, 4);
lean_closure_set(v___f_308_, 0, v_A_300_);
lean_closure_set(v___f_308_, 1, v_si_301_);
lean_closure_set(v___f_308_, 2, v_toPure_306_);
lean_closure_set(v___f_308_, 3, v_toBind_305_);
v___x_309_ = l_List_finRange(v_si_301_);
v___x_310_ = lean_box(0);
v___x_311_ = l_List_mapM_loop___redArg(v_inst_299_, v___f_307_, v___x_309_, v___x_310_);
v___x_312_ = lean_apply_4(v_toBind_305_, lean_box(0), lean_box(0), v___x_311_, v___f_308_);
return v___x_312_;
}
}
LEAN_EXPORT lean_object* lp_zhc__lean_Zhc_Iops_HillisSteele_rawSums(lean_object* v_M_313_, lean_object* v_inst_314_, lean_object* v_A_315_, lean_object* v_si_316_, lean_object* v_x_317_, lean_object* v_y_318_){
_start:
{
lean_object* v___x_319_; 
v___x_319_ = lp_zhc__lean_Zhc_Iops_HillisSteele_rawSums___redArg(v_inst_314_, v_A_315_, v_si_316_, v_x_317_, v_y_318_);
return v___x_319_;
}
}
LEAN_EXPORT lean_object* lp_zhc__lean_Zhc_Iops_HillisSteele_blockStates___redArg___lam__0(lean_object* v_inst_320_, lean_object* v_____do__lift_321_){
_start:
{
lean_object* v_toApplicative_322_; lean_object* v_toPure_323_; lean_object* v_fst_324_; lean_object* v___x_325_; 
v_toApplicative_322_ = lean_ctor_get(v_inst_320_, 0);
lean_inc_ref(v_toApplicative_322_);
lean_dec_ref(v_inst_320_);
v_toPure_323_ = lean_ctor_get(v_toApplicative_322_, 1);
lean_inc(v_toPure_323_);
lean_dec_ref(v_toApplicative_322_);
v_fst_324_ = lean_ctor_get(v_____do__lift_321_, 0);
lean_inc(v_fst_324_);
lean_dec_ref(v_____do__lift_321_);
v___x_325_ = lean_apply_2(v_toPure_323_, lean_box(0), v_fst_324_);
return v___x_325_;
}
}
LEAN_EXPORT lean_object* lp_zhc__lean_Zhc_Iops_HillisSteele_blockStates___redArg___lam__1(lean_object* v_A_326_, lean_object* v_inst_327_, lean_object* v___f_328_, lean_object* v_x_329_){
_start:
{
lean_object* v_fst_330_; lean_object* v_snd_331_; lean_object* v___x_332_; uint8_t v___x_333_; 
v_fst_330_ = lean_ctor_get(v_x_329_, 0);
lean_inc(v_fst_330_);
v_snd_331_ = lean_ctor_get(v_x_329_, 1);
lean_inc(v_snd_331_);
lean_dec_ref(v_x_329_);
v___x_332_ = lean_unsigned_to_nat(0u);
v___x_333_ = lean_nat_dec_eq(v_snd_331_, v___x_332_);
if (v___x_333_ == 0)
{
lean_object* v___x_334_; uint8_t v___x_335_; 
lean_dec(v___f_328_);
lean_dec_ref(v_inst_327_);
v___x_334_ = lean_unsigned_to_nat(4u);
v___x_335_ = lean_nat_dec_lt(v_snd_331_, v___x_334_);
if (v___x_335_ == 0)
{
lean_object* v_pbs_336_; lean_object* v___x_337_; lean_object* v___x_338_; lean_object* v___x_339_; 
v_pbs_336_ = lean_ctor_get(v_A_326_, 16);
lean_inc(v_pbs_336_);
lean_dec_ref(v_A_326_);
v___x_337_ = lean_nat_mod(v_snd_331_, v___x_334_);
lean_dec(v_snd_331_);
v___x_338_ = lp_zhc__lean_Zhc_Iops_HillisSteele_extractPropGroupLut(v___x_337_);
v___x_339_ = lean_apply_2(v_pbs_336_, v___x_338_, v_fst_330_);
return v___x_339_;
}
else
{
lean_object* v_pbs_340_; lean_object* v___x_341_; lean_object* v___x_342_; lean_object* v___x_343_; lean_object* v___x_344_; lean_object* v___x_345_; 
v_pbs_340_ = lean_ctor_get(v_A_326_, 16);
lean_inc(v_pbs_340_);
lean_dec_ref(v_A_326_);
v___x_341_ = lean_unsigned_to_nat(1u);
v___x_342_ = lean_nat_sub(v_snd_331_, v___x_341_);
lean_dec(v_snd_331_);
v___x_343_ = lean_nat_mod(v___x_342_, v___x_334_);
lean_dec(v___x_342_);
v___x_344_ = lp_zhc__lean_Zhc_Iops_HillisSteele_extractPropGroupLut(v___x_343_);
v___x_345_ = lean_apply_2(v_pbs_340_, v___x_344_, v_fst_330_);
return v___x_345_;
}
}
else
{
lean_object* v_toBind_346_; lean_object* v_pbs2_347_; lean_object* v___x_348_; lean_object* v___x_349_; lean_object* v___x_350_; 
lean_dec(v_snd_331_);
v_toBind_346_ = lean_ctor_get(v_inst_327_, 1);
lean_inc(v_toBind_346_);
lean_dec_ref(v_inst_327_);
v_pbs2_347_ = lean_ctor_get(v_A_326_, 17);
lean_inc(v_pbs2_347_);
lean_dec_ref(v_A_326_);
v___x_348_ = lp_zhc__lean_Zhc_Iops_carryMsgLut;
v___x_349_ = lean_apply_2(v_pbs2_347_, v___x_348_, v_fst_330_);
v___x_350_ = lean_apply_4(v_toBind_346_, lean_box(0), lean_box(0), v___x_349_, v___f_328_);
return v___x_350_;
}
}
}
LEAN_EXPORT lean_object* lp_zhc__lean_Zhc_Iops_HillisSteele_blockStates___redArg(lean_object* v_inst_351_, lean_object* v_A_352_, lean_object* v_sums_353_){
_start:
{
lean_object* v___f_354_; lean_object* v___f_355_; lean_object* v___x_356_; lean_object* v___x_357_; lean_object* v___x_358_; lean_object* v___x_359_; 
lean_inc_ref_n(v_inst_351_, 2);
v___f_354_ = lean_alloc_closure((void*)(lp_zhc__lean_Zhc_Iops_HillisSteele_blockStates___redArg___lam__0), 2, 1);
lean_closure_set(v___f_354_, 0, v_inst_351_);
v___f_355_ = lean_alloc_closure((void*)(lp_zhc__lean_Zhc_Iops_HillisSteele_blockStates___redArg___lam__1), 4, 3);
lean_closure_set(v___f_355_, 0, v_A_352_);
lean_closure_set(v___f_355_, 1, v_inst_351_);
lean_closure_set(v___f_355_, 2, v___f_354_);
v___x_356_ = lean_unsigned_to_nat(0u);
v___x_357_ = l_List_zipIdxTR___redArg(v_sums_353_, v___x_356_);
v___x_358_ = lean_box(0);
v___x_359_ = l_List_mapM_loop___redArg(v_inst_351_, v___f_355_, v___x_357_, v___x_358_);
return v___x_359_;
}
}
LEAN_EXPORT lean_object* lp_zhc__lean_Zhc_Iops_HillisSteele_blockStates(lean_object* v_M_360_, lean_object* v_inst_361_, lean_object* v_A_362_, lean_object* v_sums_363_){
_start:
{
lean_object* v___x_364_; 
v___x_364_ = lp_zhc__lean_Zhc_Iops_HillisSteele_blockStates___redArg(v_inst_361_, v_A_362_, v_sums_363_);
return v___x_364_;
}
}
LEAN_EXPORT lean_object* lp_zhc__lean_Zhc_Iops_HillisSteele_groupStates___redArg___lam__0(lean_object* v_b2_365_, lean_object* v_b1_366_, lean_object* v_fst_367_, lean_object* v_toPure_368_, lean_object* v_b3_369_){
_start:
{
lean_object* v___x_370_; lean_object* v___x_371_; lean_object* v___x_372_; lean_object* v___x_373_; 
v___x_370_ = lean_alloc_ctor(0, 2, 0);
lean_ctor_set(v___x_370_, 0, v_b2_365_);
lean_ctor_set(v___x_370_, 1, v_b3_369_);
v___x_371_ = lean_alloc_ctor(0, 2, 0);
lean_ctor_set(v___x_371_, 0, v_b1_366_);
lean_ctor_set(v___x_371_, 1, v___x_370_);
v___x_372_ = lean_alloc_ctor(0, 2, 0);
lean_ctor_set(v___x_372_, 0, v_fst_367_);
lean_ctor_set(v___x_372_, 1, v___x_371_);
v___x_373_ = lean_apply_2(v_toPure_368_, lean_box(0), v___x_372_);
return v___x_373_;
}
}
LEAN_EXPORT lean_object* lp_zhc__lean_Zhc_Iops_HillisSteele_groupStates___redArg___lam__1(lean_object* v___f_374_, lean_object* v_b3_375_){
_start:
{
lean_object* v___x_376_; 
v___x_376_ = lean_apply_1(v___f_374_, v_b3_375_);
return v___x_376_;
}
}
LEAN_EXPORT lean_object* lp_zhc__lean_Zhc_Iops_HillisSteele_groupStates___redArg___lam__2(lean_object* v_addPt_377_, lean_object* v_r_378_, lean_object* v_toBind_379_, lean_object* v___f_380_, lean_object* v_one_381_){
_start:
{
lean_object* v___x_382_; lean_object* v___x_383_; 
v___x_382_ = lean_apply_2(v_addPt_377_, v_r_378_, v_one_381_);
v___x_383_ = lean_apply_4(v_toBind_379_, lean_box(0), lean_box(0), v___x_382_, v___f_380_);
return v___x_383_;
}
}
LEAN_EXPORT lean_object* lp_zhc__lean_Zhc_Iops_HillisSteele_groupStates___redArg___lam__3(lean_object* v_addPt_384_, lean_object* v_toBind_385_, lean_object* v___f_386_, lean_object* v_letPtBlock_387_, lean_object* v_r_388_){
_start:
{
lean_object* v___f_389_; lean_object* v___x_390_; lean_object* v___x_391_; lean_object* v___x_392_; 
lean_inc(v_toBind_385_);
v___f_389_ = lean_alloc_closure((void*)(lp_zhc__lean_Zhc_Iops_HillisSteele_groupStates___redArg___lam__2), 5, 4);
lean_closure_set(v___f_389_, 0, v_addPt_384_);
lean_closure_set(v___f_389_, 1, v_r_388_);
lean_closure_set(v___f_389_, 2, v_toBind_385_);
lean_closure_set(v___f_389_, 3, v___f_386_);
v___x_390_ = lean_unsigned_to_nat(1u);
v___x_391_ = lean_apply_1(v_letPtBlock_387_, v___x_390_);
v___x_392_ = lean_apply_4(v_toBind_385_, lean_box(0), lean_box(0), v___x_391_, v___f_389_);
return v___x_392_;
}
}
static lean_object* _init_lp_zhc__lean_Zhc_Iops_HillisSteele_groupStates___redArg___lam__5___closed__0(void){
_start:
{
lean_object* v___x_393_; lean_object* v___x_394_; lean_object* v___x_395_; 
v___x_393_ = lean_unsigned_to_nat(3u);
v___x_394_ = lean_unsigned_to_nat(2u);
v___x_395_ = lean_nat_mod(v___x_394_, v___x_393_);
return v___x_395_;
}
}
static lean_object* _init_lp_zhc__lean_Zhc_Iops_HillisSteele_groupStates___redArg___lam__5___closed__1(void){
_start:
{
lean_object* v___x_396_; lean_object* v___x_397_; 
v___x_396_ = lean_obj_once(&lp_zhc__lean_Zhc_Iops_HillisSteele_groupStates___redArg___lam__5___closed__0, &lp_zhc__lean_Zhc_Iops_HillisSteele_groupStates___redArg___lam__5___closed__0_once, _init_lp_zhc__lean_Zhc_Iops_HillisSteele_groupStates___redArg___lam__5___closed__0);
v___x_397_ = lp_zhc__lean_Zhc_Iops_HillisSteele_solvePropGroupFinalLut(v___x_396_);
return v___x_397_;
}
}
LEAN_EXPORT lean_object* lp_zhc__lean_Zhc_Iops_HillisSteele_groupStates___redArg___lam__5(lean_object* v_snd_398_, lean_object* v_pbs_399_, lean_object* v_toBind_400_, lean_object* v___f_401_, lean_object* v___f_402_, lean_object* v_b3_403_){
_start:
{
lean_object* v___x_404_; uint8_t v___x_405_; 
v___x_404_ = lean_unsigned_to_nat(0u);
v___x_405_ = lean_nat_dec_eq(v_snd_398_, v___x_404_);
if (v___x_405_ == 0)
{
lean_object* v___x_406_; lean_object* v___x_407_; lean_object* v___x_408_; 
lean_dec(v___f_402_);
v___x_406_ = lp_zhc__lean_Zhc_Iops_HillisSteele_reduceCarryPadLut;
v___x_407_ = lean_apply_2(v_pbs_399_, v___x_406_, v_b3_403_);
v___x_408_ = lean_apply_4(v_toBind_400_, lean_box(0), lean_box(0), v___x_407_, v___f_401_);
return v___x_408_;
}
else
{
lean_object* v___x_409_; lean_object* v___x_410_; lean_object* v___x_411_; 
lean_dec(v___f_401_);
v___x_409_ = lean_obj_once(&lp_zhc__lean_Zhc_Iops_HillisSteele_groupStates___redArg___lam__5___closed__1, &lp_zhc__lean_Zhc_Iops_HillisSteele_groupStates___redArg___lam__5___closed__1_once, _init_lp_zhc__lean_Zhc_Iops_HillisSteele_groupStates___redArg___lam__5___closed__1);
v___x_410_ = lean_apply_2(v_pbs_399_, v___x_409_, v_b3_403_);
v___x_411_ = lean_apply_4(v_toBind_400_, lean_box(0), lean_box(0), v___x_410_, v___f_402_);
return v___x_411_;
}
}
}
LEAN_EXPORT lean_object* lp_zhc__lean_Zhc_Iops_HillisSteele_groupStates___redArg___lam__5___boxed(lean_object* v_snd_412_, lean_object* v_pbs_413_, lean_object* v_toBind_414_, lean_object* v___f_415_, lean_object* v___f_416_, lean_object* v_b3_417_){
_start:
{
lean_object* v_res_418_; 
v_res_418_ = lp_zhc__lean_Zhc_Iops_HillisSteele_groupStates___redArg___lam__5(v_snd_412_, v_pbs_413_, v_toBind_414_, v___f_415_, v___f_416_, v_b3_417_);
lean_dec(v_snd_412_);
return v_res_418_;
}
}
LEAN_EXPORT lean_object* lp_zhc__lean_Zhc_Iops_HillisSteele_groupStates___redArg___lam__4(lean_object* v_b1_419_, lean_object* v_fst_420_, lean_object* v_toPure_421_, lean_object* v_addPt_422_, lean_object* v_toBind_423_, lean_object* v_letPtBlock_424_, lean_object* v_snd_425_, lean_object* v_pbs_426_, lean_object* v_addCt_427_, lean_object* v_snd_428_, lean_object* v_b2_429_){
_start:
{
lean_object* v___f_430_; lean_object* v___f_431_; lean_object* v___f_432_; lean_object* v___f_433_; lean_object* v___x_434_; lean_object* v___x_435_; 
lean_inc(v_b2_429_);
v___f_430_ = lean_alloc_closure((void*)(lp_zhc__lean_Zhc_Iops_HillisSteele_groupStates___redArg___lam__0), 5, 4);
lean_closure_set(v___f_430_, 0, v_b2_429_);
lean_closure_set(v___f_430_, 1, v_b1_419_);
lean_closure_set(v___f_430_, 2, v_fst_420_);
lean_closure_set(v___f_430_, 3, v_toPure_421_);
v___f_431_ = lean_alloc_closure((void*)(lp_zhc__lean_Zhc_Iops_HillisSteele_groupStates___redArg___lam__1), 2, 1);
lean_closure_set(v___f_431_, 0, v___f_430_);
lean_inc_ref(v___f_431_);
lean_inc_n(v_toBind_423_, 2);
v___f_432_ = lean_alloc_closure((void*)(lp_zhc__lean_Zhc_Iops_HillisSteele_groupStates___redArg___lam__3), 5, 4);
lean_closure_set(v___f_432_, 0, v_addPt_422_);
lean_closure_set(v___f_432_, 1, v_toBind_423_);
lean_closure_set(v___f_432_, 2, v___f_431_);
lean_closure_set(v___f_432_, 3, v_letPtBlock_424_);
v___f_433_ = lean_alloc_closure((void*)(lp_zhc__lean_Zhc_Iops_HillisSteele_groupStates___redArg___lam__5___boxed), 6, 5);
lean_closure_set(v___f_433_, 0, v_snd_425_);
lean_closure_set(v___f_433_, 1, v_pbs_426_);
lean_closure_set(v___f_433_, 2, v_toBind_423_);
lean_closure_set(v___f_433_, 3, v___f_432_);
lean_closure_set(v___f_433_, 4, v___f_431_);
v___x_434_ = lean_apply_2(v_addCt_427_, v_b2_429_, v_snd_428_);
v___x_435_ = lean_apply_4(v_toBind_423_, lean_box(0), lean_box(0), v___x_434_, v___f_433_);
return v___x_435_;
}
}
LEAN_EXPORT lean_object* lp_zhc__lean_Zhc_Iops_HillisSteele_groupStates___redArg___lam__6(lean_object* v_fst_436_, lean_object* v_toPure_437_, lean_object* v_addPt_438_, lean_object* v_toBind_439_, lean_object* v_letPtBlock_440_, lean_object* v_snd_441_, lean_object* v_pbs_442_, lean_object* v_addCt_443_, lean_object* v_snd_444_, lean_object* v_fst_445_, lean_object* v_b1_446_){
_start:
{
lean_object* v___f_447_; lean_object* v___x_448_; lean_object* v___x_449_; 
lean_inc(v_addCt_443_);
lean_inc(v_toBind_439_);
lean_inc(v_b1_446_);
v___f_447_ = lean_alloc_closure((void*)(lp_zhc__lean_Zhc_Iops_HillisSteele_groupStates___redArg___lam__4), 11, 10);
lean_closure_set(v___f_447_, 0, v_b1_446_);
lean_closure_set(v___f_447_, 1, v_fst_436_);
lean_closure_set(v___f_447_, 2, v_toPure_437_);
lean_closure_set(v___f_447_, 3, v_addPt_438_);
lean_closure_set(v___f_447_, 4, v_toBind_439_);
lean_closure_set(v___f_447_, 5, v_letPtBlock_440_);
lean_closure_set(v___f_447_, 6, v_snd_441_);
lean_closure_set(v___f_447_, 7, v_pbs_442_);
lean_closure_set(v___f_447_, 8, v_addCt_443_);
lean_closure_set(v___f_447_, 9, v_snd_444_);
v___x_448_ = lean_apply_2(v_addCt_443_, v_b1_446_, v_fst_445_);
v___x_449_ = lean_apply_4(v_toBind_439_, lean_box(0), lean_box(0), v___x_448_, v___f_447_);
return v___x_449_;
}
}
LEAN_EXPORT lean_object* lp_zhc__lean_Zhc_Iops_HillisSteele_groupStates___redArg___lam__7(lean_object* v_A_450_, lean_object* v_toPure_451_, lean_object* v_toBind_452_, lean_object* v_x_453_){
_start:
{
lean_object* v_fst_454_; lean_object* v_snd_455_; lean_object* v_snd_456_; lean_object* v_snd_457_; lean_object* v_fst_458_; lean_object* v_fst_459_; lean_object* v_fst_460_; lean_object* v_snd_461_; lean_object* v_letPtBlock_462_; lean_object* v_addCt_463_; lean_object* v_addPt_464_; lean_object* v_pbs_465_; lean_object* v___f_466_; lean_object* v___x_467_; lean_object* v___x_468_; 
v_fst_454_ = lean_ctor_get(v_x_453_, 0);
lean_inc(v_fst_454_);
v_snd_455_ = lean_ctor_get(v_fst_454_, 1);
lean_inc(v_snd_455_);
v_snd_456_ = lean_ctor_get(v_snd_455_, 1);
lean_inc(v_snd_456_);
v_snd_457_ = lean_ctor_get(v_x_453_, 1);
lean_inc(v_snd_457_);
lean_dec_ref(v_x_453_);
v_fst_458_ = lean_ctor_get(v_fst_454_, 0);
lean_inc_n(v_fst_458_, 2);
lean_dec(v_fst_454_);
v_fst_459_ = lean_ctor_get(v_snd_455_, 0);
lean_inc(v_fst_459_);
lean_dec(v_snd_455_);
v_fst_460_ = lean_ctor_get(v_snd_456_, 0);
lean_inc(v_fst_460_);
v_snd_461_ = lean_ctor_get(v_snd_456_, 1);
lean_inc(v_snd_461_);
lean_dec(v_snd_456_);
v_letPtBlock_462_ = lean_ctor_get(v_A_450_, 1);
lean_inc(v_letPtBlock_462_);
v_addCt_463_ = lean_ctor_get(v_A_450_, 3);
lean_inc_n(v_addCt_463_, 2);
v_addPt_464_ = lean_ctor_get(v_A_450_, 7);
lean_inc(v_addPt_464_);
v_pbs_465_ = lean_ctor_get(v_A_450_, 16);
lean_inc(v_pbs_465_);
lean_dec_ref(v_A_450_);
lean_inc(v_toBind_452_);
v___f_466_ = lean_alloc_closure((void*)(lp_zhc__lean_Zhc_Iops_HillisSteele_groupStates___redArg___lam__6), 11, 10);
lean_closure_set(v___f_466_, 0, v_fst_458_);
lean_closure_set(v___f_466_, 1, v_toPure_451_);
lean_closure_set(v___f_466_, 2, v_addPt_464_);
lean_closure_set(v___f_466_, 3, v_toBind_452_);
lean_closure_set(v___f_466_, 4, v_letPtBlock_462_);
lean_closure_set(v___f_466_, 5, v_snd_457_);
lean_closure_set(v___f_466_, 6, v_pbs_465_);
lean_closure_set(v___f_466_, 7, v_addCt_463_);
lean_closure_set(v___f_466_, 8, v_snd_461_);
lean_closure_set(v___f_466_, 9, v_fst_460_);
v___x_467_ = lean_apply_2(v_addCt_463_, v_fst_458_, v_fst_459_);
v___x_468_ = lean_apply_4(v_toBind_452_, lean_box(0), lean_box(0), v___x_467_, v___f_466_);
return v___x_468_;
}
}
LEAN_EXPORT lean_object* lp_zhc__lean_Zhc_Iops_HillisSteele_groupStates___redArg___lam__8(lean_object* v_toPure_469_, lean_object* v_groups_470_){
_start:
{
lean_object* v___x_471_; lean_object* v___x_472_; 
v___x_471_ = lp_zhc__lean_Zhc_Iops_HillisSteele_unchunk4___redArg(v_groups_470_);
v___x_472_ = lean_apply_2(v_toPure_469_, lean_box(0), v___x_471_);
return v___x_472_;
}
}
LEAN_EXPORT lean_object* lp_zhc__lean_Zhc_Iops_HillisSteele_groupStates___redArg(lean_object* v_inst_473_, lean_object* v_A_474_, lean_object* v_states_475_){
_start:
{
lean_object* v_toApplicative_476_; lean_object* v_toBind_477_; lean_object* v_toPure_478_; lean_object* v___f_479_; lean_object* v___f_480_; lean_object* v___x_481_; lean_object* v___x_482_; lean_object* v___x_483_; lean_object* v___x_484_; lean_object* v___x_485_; lean_object* v___x_486_; 
v_toApplicative_476_ = lean_ctor_get(v_inst_473_, 0);
v_toBind_477_ = lean_ctor_get(v_inst_473_, 1);
lean_inc_n(v_toBind_477_, 2);
v_toPure_478_ = lean_ctor_get(v_toApplicative_476_, 1);
lean_inc_n(v_toPure_478_, 2);
v___f_479_ = lean_alloc_closure((void*)(lp_zhc__lean_Zhc_Iops_HillisSteele_groupStates___redArg___lam__7), 4, 3);
lean_closure_set(v___f_479_, 0, v_A_474_);
lean_closure_set(v___f_479_, 1, v_toPure_478_);
lean_closure_set(v___f_479_, 2, v_toBind_477_);
v___f_480_ = lean_alloc_closure((void*)(lp_zhc__lean_Zhc_Iops_HillisSteele_groupStates___redArg___lam__8), 2, 1);
lean_closure_set(v___f_480_, 0, v_toPure_478_);
v___x_481_ = lp_zhc__lean_Zhc_Iops_HillisSteele_chunk4___redArg(v_states_475_);
v___x_482_ = lean_unsigned_to_nat(0u);
v___x_483_ = l_List_zipIdxTR___redArg(v___x_481_, v___x_482_);
v___x_484_ = lean_box(0);
v___x_485_ = l_List_mapM_loop___redArg(v_inst_473_, v___f_479_, v___x_483_, v___x_484_);
v___x_486_ = lean_apply_4(v_toBind_477_, lean_box(0), lean_box(0), v___x_485_, v___f_480_);
return v___x_486_;
}
}
LEAN_EXPORT lean_object* lp_zhc__lean_Zhc_Iops_HillisSteele_groupStates(lean_object* v_M_487_, lean_object* v_inst_488_, lean_object* v_A_489_, lean_object* v_states_490_){
_start:
{
lean_object* v___x_491_; 
v___x_491_ = lp_zhc__lean_Zhc_Iops_HillisSteele_groupStates___redArg(v_inst_488_, v_A_489_, v_states_490_);
return v___x_491_;
}
}
LEAN_EXPORT lean_object* lp_zhc__lean_Zhc_Iops_HillisSteele_hsStage___redArg___lam__0(uint8_t v___x_492_, lean_object* v_pbs_493_, lean_object* v_packed_494_){
_start:
{
if (v___x_492_ == 0)
{
lean_object* v___x_495_; lean_object* v___x_496_; 
v___x_495_ = lp_zhc__lean_Zhc_Iops_HillisSteele_solvePropLut;
v___x_496_ = lean_apply_2(v_pbs_493_, v___x_495_, v_packed_494_);
return v___x_496_;
}
else
{
lean_object* v___x_497_; lean_object* v___x_498_; 
v___x_497_ = lp_zhc__lean_Zhc_Iops_HillisSteele_solvePropCarryLut;
v___x_498_ = lean_apply_2(v_pbs_493_, v___x_497_, v_packed_494_);
return v___x_498_;
}
}
}
LEAN_EXPORT lean_object* lp_zhc__lean_Zhc_Iops_HillisSteele_hsStage___redArg___lam__0___boxed(lean_object* v___x_499_, lean_object* v_pbs_500_, lean_object* v_packed_501_){
_start:
{
uint8_t v___x_126__boxed_502_; lean_object* v_res_503_; 
v___x_126__boxed_502_ = lean_unbox(v___x_499_);
v_res_503_ = lp_zhc__lean_Zhc_Iops_HillisSteele_hsStage___redArg___lam__0(v___x_126__boxed_502_, v_pbs_500_, v_packed_501_);
return v_res_503_;
}
}
LEAN_EXPORT lean_object* lp_zhc__lean_Zhc_Iops_HillisSteele_hsStage___redArg___lam__1(lean_object* v_A_504_, lean_object* v_stride_505_, lean_object* v_toBind_506_, lean_object* v_x_507_){
_start:
{
lean_object* v_fst_508_; lean_object* v_snd_509_; lean_object* v_fst_510_; lean_object* v_snd_511_; lean_object* v_packCt_512_; lean_object* v_pbs_513_; lean_object* v___x_514_; lean_object* v___x_515_; uint8_t v___x_516_; lean_object* v___x_517_; lean_object* v___f_518_; lean_object* v___x_519_; 
v_fst_508_ = lean_ctor_get(v_x_507_, 0);
lean_inc(v_fst_508_);
v_snd_509_ = lean_ctor_get(v_x_507_, 1);
lean_inc(v_snd_509_);
lean_dec_ref(v_x_507_);
v_fst_510_ = lean_ctor_get(v_fst_508_, 0);
lean_inc(v_fst_510_);
v_snd_511_ = lean_ctor_get(v_fst_508_, 1);
lean_inc(v_snd_511_);
lean_dec(v_fst_508_);
v_packCt_512_ = lean_ctor_get(v_A_504_, 6);
lean_inc(v_packCt_512_);
v_pbs_513_ = lean_ctor_get(v_A_504_, 16);
lean_inc(v_pbs_513_);
lean_dec_ref(v_A_504_);
v___x_514_ = lean_unsigned_to_nat(4u);
v___x_515_ = lean_apply_3(v_packCt_512_, v_fst_510_, v___x_514_, v_snd_511_);
v___x_516_ = lean_nat_dec_lt(v_snd_509_, v_stride_505_);
lean_dec(v_snd_509_);
v___x_517_ = lean_box(v___x_516_);
v___f_518_ = lean_alloc_closure((void*)(lp_zhc__lean_Zhc_Iops_HillisSteele_hsStage___redArg___lam__0___boxed), 3, 2);
lean_closure_set(v___f_518_, 0, v___x_517_);
lean_closure_set(v___f_518_, 1, v_pbs_513_);
v___x_519_ = lean_apply_4(v_toBind_506_, lean_box(0), lean_box(0), v___x_515_, v___f_518_);
return v___x_519_;
}
}
LEAN_EXPORT lean_object* lp_zhc__lean_Zhc_Iops_HillisSteele_hsStage___redArg___lam__1___boxed(lean_object* v_A_520_, lean_object* v_stride_521_, lean_object* v_toBind_522_, lean_object* v_x_523_){
_start:
{
lean_object* v_res_524_; 
v_res_524_ = lp_zhc__lean_Zhc_Iops_HillisSteele_hsStage___redArg___lam__1(v_A_520_, v_stride_521_, v_toBind_522_, v_x_523_);
lean_dec(v_stride_521_);
return v_res_524_;
}
}
LEAN_EXPORT lean_object* lp_zhc__lean_Zhc_Iops_HillisSteele_hsStage___redArg___lam__2(lean_object* v_carries_527_, lean_object* v_stride_528_, lean_object* v_toPure_529_, lean_object* v_solved_530_){
_start:
{
lean_object* v___x_531_; lean_object* v___x_532_; lean_object* v___x_533_; lean_object* v___x_534_; 
v___x_531_ = ((lean_object*)(lp_zhc__lean_Zhc_Iops_HillisSteele_hsStage___redArg___lam__2___closed__0));
lean_inc(v_carries_527_);
v___x_532_ = l___private_Init_Data_List_Impl_0__List_takeTR_go___redArg(v_carries_527_, v_carries_527_, v_stride_528_, v___x_531_);
lean_dec(v_carries_527_);
v___x_533_ = l_List_appendTR___redArg(v___x_532_, v_solved_530_);
v___x_534_ = lean_apply_2(v_toPure_529_, lean_box(0), v___x_533_);
return v___x_534_;
}
}
LEAN_EXPORT lean_object* lp_zhc__lean_Zhc_Iops_HillisSteele_hsStage___redArg(lean_object* v_inst_535_, lean_object* v_A_536_, lean_object* v_stage_537_, lean_object* v_carries_538_){
_start:
{
lean_object* v_toApplicative_539_; lean_object* v_toBind_540_; lean_object* v_toPure_541_; lean_object* v___x_542_; lean_object* v_stride_543_; lean_object* v___f_544_; lean_object* v___f_545_; lean_object* v___x_546_; lean_object* v___x_547_; lean_object* v___x_548_; lean_object* v___x_549_; lean_object* v___x_550_; lean_object* v___x_551_; lean_object* v___x_552_; 
v_toApplicative_539_ = lean_ctor_get(v_inst_535_, 0);
v_toBind_540_ = lean_ctor_get(v_inst_535_, 1);
lean_inc_n(v_toBind_540_, 2);
v_toPure_541_ = lean_ctor_get(v_toApplicative_539_, 1);
v___x_542_ = lean_unsigned_to_nat(2u);
v_stride_543_ = lean_nat_pow(v___x_542_, v_stage_537_);
lean_inc_n(v_stride_543_, 2);
v___f_544_ = lean_alloc_closure((void*)(lp_zhc__lean_Zhc_Iops_HillisSteele_hsStage___redArg___lam__1___boxed), 4, 3);
lean_closure_set(v___f_544_, 0, v_A_536_);
lean_closure_set(v___f_544_, 1, v_stride_543_);
lean_closure_set(v___f_544_, 2, v_toBind_540_);
lean_inc(v_toPure_541_);
lean_inc(v_carries_538_);
v___f_545_ = lean_alloc_closure((void*)(lp_zhc__lean_Zhc_Iops_HillisSteele_hsStage___redArg___lam__2), 4, 3);
lean_closure_set(v___f_545_, 0, v_carries_538_);
lean_closure_set(v___f_545_, 1, v_stride_543_);
lean_closure_set(v___f_545_, 2, v_toPure_541_);
v___x_546_ = l_List_drop___redArg(v_stride_543_, v_carries_538_);
v___x_547_ = l_List_zipWith___at___00List_zip_spec__0___redArg(v___x_546_, v_carries_538_);
v___x_548_ = lean_unsigned_to_nat(0u);
v___x_549_ = l_List_zipIdxTR___redArg(v___x_547_, v___x_548_);
v___x_550_ = lean_box(0);
v___x_551_ = l_List_mapM_loop___redArg(v_inst_535_, v___f_544_, v___x_549_, v___x_550_);
v___x_552_ = lean_apply_4(v_toBind_540_, lean_box(0), lean_box(0), v___x_551_, v___f_545_);
return v___x_552_;
}
}
LEAN_EXPORT lean_object* lp_zhc__lean_Zhc_Iops_HillisSteele_hsStage___redArg___boxed(lean_object* v_inst_553_, lean_object* v_A_554_, lean_object* v_stage_555_, lean_object* v_carries_556_){
_start:
{
lean_object* v_res_557_; 
v_res_557_ = lp_zhc__lean_Zhc_Iops_HillisSteele_hsStage___redArg(v_inst_553_, v_A_554_, v_stage_555_, v_carries_556_);
lean_dec(v_stage_555_);
return v_res_557_;
}
}
LEAN_EXPORT lean_object* lp_zhc__lean_Zhc_Iops_HillisSteele_hsStage(lean_object* v_M_558_, lean_object* v_inst_559_, lean_object* v_A_560_, lean_object* v_stage_561_, lean_object* v_carries_562_){
_start:
{
lean_object* v___x_563_; 
v___x_563_ = lp_zhc__lean_Zhc_Iops_HillisSteele_hsStage___redArg(v_inst_559_, v_A_560_, v_stage_561_, v_carries_562_);
return v___x_563_;
}
}
LEAN_EXPORT lean_object* lp_zhc__lean_Zhc_Iops_HillisSteele_hsStage___boxed(lean_object* v_M_564_, lean_object* v_inst_565_, lean_object* v_A_566_, lean_object* v_stage_567_, lean_object* v_carries_568_){
_start:
{
lean_object* v_res_569_; 
v_res_569_ = lp_zhc__lean_Zhc_Iops_HillisSteele_hsStage(v_M_564_, v_inst_565_, v_A_566_, v_stage_567_, v_carries_568_);
lean_dec(v_stage_567_);
return v_res_569_;
}
}
LEAN_EXPORT lean_object* lp_zhc__lean_Zhc_Iops_HillisSteele_groupCarries___redArg___lam__0(lean_object* v_x_570_){
_start:
{
lean_object* v_snd_571_; lean_object* v_snd_572_; lean_object* v_snd_573_; 
v_snd_571_ = lean_ctor_get(v_x_570_, 1);
v_snd_572_ = lean_ctor_get(v_snd_571_, 1);
v_snd_573_ = lean_ctor_get(v_snd_572_, 1);
lean_inc(v_snd_573_);
return v_snd_573_;
}
}
LEAN_EXPORT lean_object* lp_zhc__lean_Zhc_Iops_HillisSteele_groupCarries___redArg___lam__0___boxed(lean_object* v_x_574_){
_start:
{
lean_object* v_res_575_; 
v_res_575_ = lp_zhc__lean_Zhc_Iops_HillisSteele_groupCarries___redArg___lam__0(v_x_574_);
lean_dec_ref(v_x_574_);
return v_res_575_;
}
}
LEAN_EXPORT lean_object* lp_zhc__lean_Zhc_Iops_HillisSteele_groupCarries___redArg___lam__1(lean_object* v_toPure_576_, lean_object* v_____s_577_){
_start:
{
lean_object* v___x_578_; 
v___x_578_ = lean_apply_2(v_toPure_576_, lean_box(0), v_____s_577_);
return v___x_578_;
}
}
LEAN_EXPORT lean_object* lp_zhc__lean_Zhc_Iops_HillisSteele_groupCarries___redArg___lam__2(lean_object* v_toPure_579_, lean_object* v_carries_580_){
_start:
{
lean_object* v___x_581_; lean_object* v___x_582_; 
v___x_581_ = lean_alloc_ctor(1, 1, 0);
lean_ctor_set(v___x_581_, 0, v_carries_580_);
v___x_582_ = lean_apply_2(v_toPure_579_, lean_box(0), v___x_581_);
return v___x_582_;
}
}
LEAN_EXPORT lean_object* lp_zhc__lean_Zhc_Iops_HillisSteele_groupCarries___redArg___lam__3(lean_object* v_inst_583_, lean_object* v_A_584_, lean_object* v_toBind_585_, lean_object* v___f_586_, lean_object* v_a_587_, lean_object* v_x_588_, lean_object* v___y_589_){
_start:
{
lean_object* v___x_590_; lean_object* v___x_591_; 
v___x_590_ = lp_zhc__lean_Zhc_Iops_HillisSteele_hsStage___redArg(v_inst_583_, v_A_584_, v_a_587_, v___y_589_);
v___x_591_ = lean_apply_4(v_toBind_585_, lean_box(0), lean_box(0), v___x_590_, v___f_586_);
return v___x_591_;
}
}
LEAN_EXPORT lean_object* lp_zhc__lean_Zhc_Iops_HillisSteele_groupCarries___redArg___lam__3___boxed(lean_object* v_inst_592_, lean_object* v_A_593_, lean_object* v_toBind_594_, lean_object* v___f_595_, lean_object* v_a_596_, lean_object* v_x_597_, lean_object* v___y_598_){
_start:
{
lean_object* v_res_599_; 
v_res_599_ = lp_zhc__lean_Zhc_Iops_HillisSteele_groupCarries___redArg___lam__3(v_inst_592_, v_A_593_, v_toBind_594_, v___f_595_, v_a_596_, v_x_597_, v___y_598_);
lean_dec(v_a_596_);
return v_res_599_;
}
}
LEAN_EXPORT lean_object* lp_zhc__lean_Zhc_Iops_HillisSteele_groupCarries___redArg(lean_object* v_inst_601_, lean_object* v_A_602_, lean_object* v_groups_603_){
_start:
{
lean_object* v_toApplicative_604_; lean_object* v_toBind_605_; lean_object* v_toPure_606_; lean_object* v___f_607_; lean_object* v___f_608_; lean_object* v___f_609_; lean_object* v___f_610_; lean_object* v___x_611_; lean_object* v___x_612_; lean_object* v_carries_613_; lean_object* v___x_614_; lean_object* v___x_615_; lean_object* v___x_616_; lean_object* v___x_617_; lean_object* v___x_618_; 
v_toApplicative_604_ = lean_ctor_get(v_inst_601_, 0);
v_toBind_605_ = lean_ctor_get(v_inst_601_, 1);
lean_inc_n(v_toBind_605_, 2);
v_toPure_606_ = lean_ctor_get(v_toApplicative_604_, 1);
v___f_607_ = ((lean_object*)(lp_zhc__lean_Zhc_Iops_HillisSteele_groupCarries___redArg___closed__0));
lean_inc_n(v_toPure_606_, 2);
v___f_608_ = lean_alloc_closure((void*)(lp_zhc__lean_Zhc_Iops_HillisSteele_groupCarries___redArg___lam__1), 2, 1);
lean_closure_set(v___f_608_, 0, v_toPure_606_);
v___f_609_ = lean_alloc_closure((void*)(lp_zhc__lean_Zhc_Iops_HillisSteele_groupCarries___redArg___lam__2), 2, 1);
lean_closure_set(v___f_609_, 0, v_toPure_606_);
lean_inc_ref(v_inst_601_);
v___f_610_ = lean_alloc_closure((void*)(lp_zhc__lean_Zhc_Iops_HillisSteele_groupCarries___redArg___lam__3___boxed), 7, 4);
lean_closure_set(v___f_610_, 0, v_inst_601_);
lean_closure_set(v___f_610_, 1, v_A_602_);
lean_closure_set(v___f_610_, 2, v_toBind_605_);
lean_closure_set(v___f_610_, 3, v___f_609_);
v___x_611_ = lp_zhc__lean_Zhc_Iops_HillisSteele_chunk4___redArg(v_groups_603_);
v___x_612_ = lean_box(0);
v_carries_613_ = l_List_mapTR_loop___redArg(v___f_607_, v___x_611_, v___x_612_);
v___x_614_ = l_List_lengthTR___redArg(v_carries_613_);
v___x_615_ = lean_nat_log2(v___x_614_);
lean_dec(v___x_614_);
v___x_616_ = l_List_range(v___x_615_);
v___x_617_ = l_List_forIn_x27_loop___redArg(v_inst_601_, v___f_610_, v___x_616_, v_carries_613_);
lean_dec(v___x_616_);
v___x_618_ = lean_apply_4(v_toBind_605_, lean_box(0), lean_box(0), v___x_617_, v___f_608_);
return v___x_618_;
}
}
LEAN_EXPORT lean_object* lp_zhc__lean_Zhc_Iops_HillisSteele_groupCarries(lean_object* v_M_619_, lean_object* v_inst_620_, lean_object* v_A_621_, lean_object* v_groups_622_){
_start:
{
lean_object* v___x_623_; 
v___x_623_ = lp_zhc__lean_Zhc_Iops_HillisSteele_groupCarries___redArg(v_inst_620_, v_A_621_, v_groups_622_);
return v___x_623_;
}
}
LEAN_EXPORT lean_object* lp_zhc__lean_Zhc_Iops_HillisSteele_finalResolution___redArg___lam__0(lean_object* v_val_624_){
_start:
{
lean_object* v___x_625_; 
v___x_625_ = lean_alloc_ctor(1, 1, 0);
lean_ctor_set(v___x_625_, 0, v_val_624_);
return v___x_625_;
}
}
LEAN_EXPORT lean_object* lp_zhc__lean_Zhc_Iops_HillisSteele_finalResolution___redArg___lam__1(lean_object* v_snd_626_, lean_object* v_b1_627_, lean_object* v_fst_628_, lean_object* v_toPure_629_, lean_object* v_b2_630_){
_start:
{
lean_object* v___x_631_; lean_object* v___x_632_; lean_object* v___x_633_; lean_object* v___x_634_; 
v___x_631_ = lean_alloc_ctor(0, 2, 0);
lean_ctor_set(v___x_631_, 0, v_b2_630_);
lean_ctor_set(v___x_631_, 1, v_snd_626_);
v___x_632_ = lean_alloc_ctor(0, 2, 0);
lean_ctor_set(v___x_632_, 0, v_b1_627_);
lean_ctor_set(v___x_632_, 1, v___x_631_);
v___x_633_ = lean_alloc_ctor(0, 2, 0);
lean_ctor_set(v___x_633_, 0, v_fst_628_);
lean_ctor_set(v___x_633_, 1, v___x_632_);
v___x_634_ = lean_apply_2(v_toPure_629_, lean_box(0), v___x_633_);
return v___x_634_;
}
}
LEAN_EXPORT lean_object* lp_zhc__lean_Zhc_Iops_HillisSteele_finalResolution___redArg___lam__2(lean_object* v_snd_635_, lean_object* v_fst_636_, lean_object* v_toPure_637_, lean_object* v___x_638_, lean_object* v_pbs_639_, lean_object* v_fst_640_, lean_object* v_toBind_641_, lean_object* v_b1_642_){
_start:
{
lean_object* v___f_643_; lean_object* v___x_644_; lean_object* v___x_645_; lean_object* v___x_646_; lean_object* v___x_647_; lean_object* v___x_648_; 
v___f_643_ = lean_alloc_closure((void*)(lp_zhc__lean_Zhc_Iops_HillisSteele_finalResolution___redArg___lam__1), 5, 4);
lean_closure_set(v___f_643_, 0, v_snd_635_);
lean_closure_set(v___f_643_, 1, v_b1_642_);
lean_closure_set(v___f_643_, 2, v_fst_636_);
lean_closure_set(v___f_643_, 3, v_toPure_637_);
v___x_644_ = lean_unsigned_to_nat(1u);
v___x_645_ = lean_nat_mod(v___x_644_, v___x_638_);
v___x_646_ = lp_zhc__lean_Zhc_Iops_HillisSteele_solvePropGroupFinalLut(v___x_645_);
v___x_647_ = lean_apply_2(v_pbs_639_, v___x_646_, v_fst_640_);
v___x_648_ = lean_apply_4(v_toBind_641_, lean_box(0), lean_box(0), v___x_647_, v___f_643_);
return v___x_648_;
}
}
LEAN_EXPORT lean_object* lp_zhc__lean_Zhc_Iops_HillisSteele_finalResolution___redArg___lam__2___boxed(lean_object* v_snd_649_, lean_object* v_fst_650_, lean_object* v_toPure_651_, lean_object* v___x_652_, lean_object* v_pbs_653_, lean_object* v_fst_654_, lean_object* v_toBind_655_, lean_object* v_b1_656_){
_start:
{
lean_object* v_res_657_; 
v_res_657_ = lp_zhc__lean_Zhc_Iops_HillisSteele_finalResolution___redArg___lam__2(v_snd_649_, v_fst_650_, v_toPure_651_, v___x_652_, v_pbs_653_, v_fst_654_, v_toBind_655_, v_b1_656_);
lean_dec(v___x_652_);
return v_res_657_;
}
}
LEAN_EXPORT lean_object* lp_zhc__lean_Zhc_Iops_HillisSteele_finalResolution___redArg___lam__3(lean_object* v_snd_658_, lean_object* v_b1_659_, lean_object* v_b0_660_, lean_object* v_toPure_661_, lean_object* v_b2_662_){
_start:
{
lean_object* v___x_663_; lean_object* v___x_664_; lean_object* v___x_665_; lean_object* v___x_666_; 
v___x_663_ = lean_alloc_ctor(0, 2, 0);
lean_ctor_set(v___x_663_, 0, v_b2_662_);
lean_ctor_set(v___x_663_, 1, v_snd_658_);
v___x_664_ = lean_alloc_ctor(0, 2, 0);
lean_ctor_set(v___x_664_, 0, v_b1_659_);
lean_ctor_set(v___x_664_, 1, v___x_663_);
v___x_665_ = lean_alloc_ctor(0, 2, 0);
lean_ctor_set(v___x_665_, 0, v_b0_660_);
lean_ctor_set(v___x_665_, 1, v___x_664_);
v___x_666_ = lean_apply_2(v_toPure_661_, lean_box(0), v___x_665_);
return v___x_666_;
}
}
LEAN_EXPORT lean_object* lp_zhc__lean_Zhc_Iops_HillisSteele_finalResolution___redArg___lam__4(lean_object* v___x_667_, lean_object* v_pbs_668_, lean_object* v_toBind_669_, lean_object* v___f_670_, lean_object* v_____do__lift_671_){
_start:
{
lean_object* v___x_672_; lean_object* v___x_673_; lean_object* v___x_674_; lean_object* v___x_675_; lean_object* v___x_676_; 
v___x_672_ = lean_unsigned_to_nat(2u);
v___x_673_ = lean_nat_mod(v___x_672_, v___x_667_);
v___x_674_ = lp_zhc__lean_Zhc_Iops_HillisSteele_solvePropGroupFinalLut(v___x_673_);
v___x_675_ = lean_apply_2(v_pbs_668_, v___x_674_, v_____do__lift_671_);
v___x_676_ = lean_apply_4(v_toBind_669_, lean_box(0), lean_box(0), v___x_675_, v___f_670_);
return v___x_676_;
}
}
LEAN_EXPORT lean_object* lp_zhc__lean_Zhc_Iops_HillisSteele_finalResolution___redArg___lam__4___boxed(lean_object* v___x_677_, lean_object* v_pbs_678_, lean_object* v_toBind_679_, lean_object* v___f_680_, lean_object* v_____do__lift_681_){
_start:
{
lean_object* v_res_682_; 
v_res_682_ = lp_zhc__lean_Zhc_Iops_HillisSteele_finalResolution___redArg___lam__4(v___x_677_, v_pbs_678_, v_toBind_679_, v___f_680_, v_____do__lift_681_);
lean_dec(v___x_677_);
return v_res_682_;
}
}
LEAN_EXPORT lean_object* lp_zhc__lean_Zhc_Iops_HillisSteele_finalResolution___redArg___lam__5(lean_object* v_snd_683_, lean_object* v_b0_684_, lean_object* v_toPure_685_, lean_object* v___x_686_, lean_object* v_pbs_687_, lean_object* v_toBind_688_, lean_object* v_addCt_689_, lean_object* v_fst_690_, lean_object* v_val_691_, lean_object* v_b1_692_){
_start:
{
lean_object* v___f_693_; lean_object* v___f_694_; lean_object* v___x_695_; lean_object* v___x_696_; 
v___f_693_ = lean_alloc_closure((void*)(lp_zhc__lean_Zhc_Iops_HillisSteele_finalResolution___redArg___lam__3), 5, 4);
lean_closure_set(v___f_693_, 0, v_snd_683_);
lean_closure_set(v___f_693_, 1, v_b1_692_);
lean_closure_set(v___f_693_, 2, v_b0_684_);
lean_closure_set(v___f_693_, 3, v_toPure_685_);
lean_inc(v_toBind_688_);
v___f_694_ = lean_alloc_closure((void*)(lp_zhc__lean_Zhc_Iops_HillisSteele_finalResolution___redArg___lam__4___boxed), 5, 4);
lean_closure_set(v___f_694_, 0, v___x_686_);
lean_closure_set(v___f_694_, 1, v_pbs_687_);
lean_closure_set(v___f_694_, 2, v_toBind_688_);
lean_closure_set(v___f_694_, 3, v___f_693_);
v___x_695_ = lean_apply_2(v_addCt_689_, v_fst_690_, v_val_691_);
v___x_696_ = lean_apply_4(v_toBind_688_, lean_box(0), lean_box(0), v___x_695_, v___f_694_);
return v___x_696_;
}
}
LEAN_EXPORT lean_object* lp_zhc__lean_Zhc_Iops_HillisSteele_finalResolution___redArg___lam__6(lean_object* v___x_697_, lean_object* v_pbs_698_, lean_object* v_toBind_699_, lean_object* v___f_700_, lean_object* v_____do__lift_701_){
_start:
{
lean_object* v___x_702_; lean_object* v___x_703_; lean_object* v___x_704_; lean_object* v___x_705_; lean_object* v___x_706_; 
v___x_702_ = lean_unsigned_to_nat(1u);
v___x_703_ = lean_nat_mod(v___x_702_, v___x_697_);
v___x_704_ = lp_zhc__lean_Zhc_Iops_HillisSteele_solvePropGroupFinalLut(v___x_703_);
v___x_705_ = lean_apply_2(v_pbs_698_, v___x_704_, v_____do__lift_701_);
v___x_706_ = lean_apply_4(v_toBind_699_, lean_box(0), lean_box(0), v___x_705_, v___f_700_);
return v___x_706_;
}
}
LEAN_EXPORT lean_object* lp_zhc__lean_Zhc_Iops_HillisSteele_finalResolution___redArg___lam__6___boxed(lean_object* v___x_707_, lean_object* v_pbs_708_, lean_object* v_toBind_709_, lean_object* v___f_710_, lean_object* v_____do__lift_711_){
_start:
{
lean_object* v_res_712_; 
v_res_712_ = lp_zhc__lean_Zhc_Iops_HillisSteele_finalResolution___redArg___lam__6(v___x_707_, v_pbs_708_, v_toBind_709_, v___f_710_, v_____do__lift_711_);
lean_dec(v___x_707_);
return v_res_712_;
}
}
LEAN_EXPORT lean_object* lp_zhc__lean_Zhc_Iops_HillisSteele_finalResolution___redArg___lam__7(lean_object* v_snd_713_, lean_object* v_toPure_714_, lean_object* v___x_715_, lean_object* v_pbs_716_, lean_object* v_toBind_717_, lean_object* v_addCt_718_, lean_object* v_fst_719_, lean_object* v_val_720_, lean_object* v_fst_721_, lean_object* v_b0_722_){
_start:
{
lean_object* v___f_723_; lean_object* v___f_724_; lean_object* v___x_725_; lean_object* v___x_726_; 
lean_inc(v_val_720_);
lean_inc(v_addCt_718_);
lean_inc_n(v_toBind_717_, 2);
lean_inc(v_pbs_716_);
lean_inc(v___x_715_);
v___f_723_ = lean_alloc_closure((void*)(lp_zhc__lean_Zhc_Iops_HillisSteele_finalResolution___redArg___lam__5), 10, 9);
lean_closure_set(v___f_723_, 0, v_snd_713_);
lean_closure_set(v___f_723_, 1, v_b0_722_);
lean_closure_set(v___f_723_, 2, v_toPure_714_);
lean_closure_set(v___f_723_, 3, v___x_715_);
lean_closure_set(v___f_723_, 4, v_pbs_716_);
lean_closure_set(v___f_723_, 5, v_toBind_717_);
lean_closure_set(v___f_723_, 6, v_addCt_718_);
lean_closure_set(v___f_723_, 7, v_fst_719_);
lean_closure_set(v___f_723_, 8, v_val_720_);
v___f_724_ = lean_alloc_closure((void*)(lp_zhc__lean_Zhc_Iops_HillisSteele_finalResolution___redArg___lam__6___boxed), 5, 4);
lean_closure_set(v___f_724_, 0, v___x_715_);
lean_closure_set(v___f_724_, 1, v_pbs_716_);
lean_closure_set(v___f_724_, 2, v_toBind_717_);
lean_closure_set(v___f_724_, 3, v___f_723_);
v___x_725_ = lean_apply_2(v_addCt_718_, v_fst_721_, v_val_720_);
v___x_726_ = lean_apply_4(v_toBind_717_, lean_box(0), lean_box(0), v___x_725_, v___f_724_);
return v___x_726_;
}
}
static lean_object* _init_lp_zhc__lean_Zhc_Iops_HillisSteele_finalResolution___redArg___lam__8___closed__0(void){
_start:
{
lean_object* v___x_727_; lean_object* v___x_728_; lean_object* v___x_729_; 
v___x_727_ = lean_unsigned_to_nat(3u);
v___x_728_ = lean_unsigned_to_nat(0u);
v___x_729_ = lean_nat_mod(v___x_728_, v___x_727_);
return v___x_729_;
}
}
static lean_object* _init_lp_zhc__lean_Zhc_Iops_HillisSteele_finalResolution___redArg___lam__8___closed__1(void){
_start:
{
lean_object* v___x_730_; lean_object* v___x_731_; 
v___x_730_ = lean_obj_once(&lp_zhc__lean_Zhc_Iops_HillisSteele_finalResolution___redArg___lam__8___closed__0, &lp_zhc__lean_Zhc_Iops_HillisSteele_finalResolution___redArg___lam__8___closed__0_once, _init_lp_zhc__lean_Zhc_Iops_HillisSteele_finalResolution___redArg___lam__8___closed__0);
v___x_731_ = lp_zhc__lean_Zhc_Iops_HillisSteele_solvePropGroupFinalLut(v___x_730_);
return v___x_731_;
}
}
LEAN_EXPORT lean_object* lp_zhc__lean_Zhc_Iops_HillisSteele_finalResolution___redArg___lam__8(lean_object* v_snd_732_, lean_object* v_toPure_733_, lean_object* v_pbs_734_, lean_object* v_toBind_735_, lean_object* v_addCt_736_, lean_object* v_fst_737_, lean_object* v_val_738_, lean_object* v_fst_739_, lean_object* v_____do__lift_740_){
_start:
{
lean_object* v___x_741_; lean_object* v___f_742_; lean_object* v___x_743_; lean_object* v___x_744_; lean_object* v___x_745_; 
v___x_741_ = lean_unsigned_to_nat(3u);
lean_inc(v_toBind_735_);
lean_inc(v_pbs_734_);
v___f_742_ = lean_alloc_closure((void*)(lp_zhc__lean_Zhc_Iops_HillisSteele_finalResolution___redArg___lam__7), 10, 9);
lean_closure_set(v___f_742_, 0, v_snd_732_);
lean_closure_set(v___f_742_, 1, v_toPure_733_);
lean_closure_set(v___f_742_, 2, v___x_741_);
lean_closure_set(v___f_742_, 3, v_pbs_734_);
lean_closure_set(v___f_742_, 4, v_toBind_735_);
lean_closure_set(v___f_742_, 5, v_addCt_736_);
lean_closure_set(v___f_742_, 6, v_fst_737_);
lean_closure_set(v___f_742_, 7, v_val_738_);
lean_closure_set(v___f_742_, 8, v_fst_739_);
v___x_743_ = lean_obj_once(&lp_zhc__lean_Zhc_Iops_HillisSteele_finalResolution___redArg___lam__8___closed__1, &lp_zhc__lean_Zhc_Iops_HillisSteele_finalResolution___redArg___lam__8___closed__1_once, _init_lp_zhc__lean_Zhc_Iops_HillisSteele_finalResolution___redArg___lam__8___closed__1);
v___x_744_ = lean_apply_2(v_pbs_734_, v___x_743_, v_____do__lift_740_);
v___x_745_ = lean_apply_4(v_toBind_735_, lean_box(0), lean_box(0), v___x_744_, v___f_742_);
return v___x_745_;
}
}
LEAN_EXPORT lean_object* lp_zhc__lean_Zhc_Iops_HillisSteele_finalResolution___redArg___lam__9(lean_object* v_A_746_, lean_object* v_toPure_747_, lean_object* v_toBind_748_, lean_object* v_x_749_){
_start:
{
lean_object* v_fst_750_; lean_object* v_fst_751_; lean_object* v_snd_752_; lean_object* v_snd_753_; lean_object* v_snd_754_; 
v_fst_750_ = lean_ctor_get(v_x_749_, 0);
lean_inc(v_fst_750_);
v_fst_751_ = lean_ctor_get(v_fst_750_, 0);
lean_inc(v_fst_751_);
v_snd_752_ = lean_ctor_get(v_fst_751_, 1);
lean_inc(v_snd_752_);
v_snd_753_ = lean_ctor_get(v_snd_752_, 1);
lean_inc(v_snd_753_);
v_snd_754_ = lean_ctor_get(v_x_749_, 1);
lean_inc(v_snd_754_);
lean_dec_ref(v_x_749_);
if (lean_obj_tag(v_snd_754_) == 0)
{
lean_object* v_snd_755_; lean_object* v_fst_756_; lean_object* v_fst_757_; lean_object* v_fst_758_; lean_object* v_pbs_759_; lean_object* v___x_760_; lean_object* v___f_761_; lean_object* v___x_762_; lean_object* v___x_763_; lean_object* v___x_764_; 
v_snd_755_ = lean_ctor_get(v_fst_750_, 1);
lean_inc(v_snd_755_);
lean_dec(v_fst_750_);
v_fst_756_ = lean_ctor_get(v_fst_751_, 0);
lean_inc(v_fst_756_);
lean_dec(v_fst_751_);
v_fst_757_ = lean_ctor_get(v_snd_752_, 0);
lean_inc(v_fst_757_);
lean_dec(v_snd_752_);
v_fst_758_ = lean_ctor_get(v_snd_753_, 0);
lean_inc(v_fst_758_);
lean_dec(v_snd_753_);
v_pbs_759_ = lean_ctor_get(v_A_746_, 16);
lean_inc_n(v_pbs_759_, 2);
lean_dec_ref(v_A_746_);
v___x_760_ = lean_unsigned_to_nat(3u);
lean_inc(v_toBind_748_);
v___f_761_ = lean_alloc_closure((void*)(lp_zhc__lean_Zhc_Iops_HillisSteele_finalResolution___redArg___lam__2___boxed), 8, 7);
lean_closure_set(v___f_761_, 0, v_snd_755_);
lean_closure_set(v___f_761_, 1, v_fst_756_);
lean_closure_set(v___f_761_, 2, v_toPure_747_);
lean_closure_set(v___f_761_, 3, v___x_760_);
lean_closure_set(v___f_761_, 4, v_pbs_759_);
lean_closure_set(v___f_761_, 5, v_fst_758_);
lean_closure_set(v___f_761_, 6, v_toBind_748_);
v___x_762_ = lean_obj_once(&lp_zhc__lean_Zhc_Iops_HillisSteele_finalResolution___redArg___lam__8___closed__1, &lp_zhc__lean_Zhc_Iops_HillisSteele_finalResolution___redArg___lam__8___closed__1_once, _init_lp_zhc__lean_Zhc_Iops_HillisSteele_finalResolution___redArg___lam__8___closed__1);
v___x_763_ = lean_apply_2(v_pbs_759_, v___x_762_, v_fst_757_);
v___x_764_ = lean_apply_4(v_toBind_748_, lean_box(0), lean_box(0), v___x_763_, v___f_761_);
return v___x_764_;
}
else
{
lean_object* v_snd_765_; lean_object* v_fst_766_; lean_object* v_fst_767_; lean_object* v_fst_768_; lean_object* v_val_769_; lean_object* v_addCt_770_; lean_object* v_pbs_771_; lean_object* v___f_772_; lean_object* v___x_773_; lean_object* v___x_774_; 
v_snd_765_ = lean_ctor_get(v_fst_750_, 1);
lean_inc(v_snd_765_);
lean_dec(v_fst_750_);
v_fst_766_ = lean_ctor_get(v_fst_751_, 0);
lean_inc(v_fst_766_);
lean_dec(v_fst_751_);
v_fst_767_ = lean_ctor_get(v_snd_752_, 0);
lean_inc(v_fst_767_);
lean_dec(v_snd_752_);
v_fst_768_ = lean_ctor_get(v_snd_753_, 0);
lean_inc(v_fst_768_);
lean_dec(v_snd_753_);
v_val_769_ = lean_ctor_get(v_snd_754_, 0);
lean_inc_n(v_val_769_, 2);
lean_dec_ref_known(v_snd_754_, 1);
v_addCt_770_ = lean_ctor_get(v_A_746_, 3);
lean_inc_n(v_addCt_770_, 2);
v_pbs_771_ = lean_ctor_get(v_A_746_, 16);
lean_inc(v_pbs_771_);
lean_dec_ref(v_A_746_);
lean_inc(v_toBind_748_);
v___f_772_ = lean_alloc_closure((void*)(lp_zhc__lean_Zhc_Iops_HillisSteele_finalResolution___redArg___lam__8), 9, 8);
lean_closure_set(v___f_772_, 0, v_snd_765_);
lean_closure_set(v___f_772_, 1, v_toPure_747_);
lean_closure_set(v___f_772_, 2, v_pbs_771_);
lean_closure_set(v___f_772_, 3, v_toBind_748_);
lean_closure_set(v___f_772_, 4, v_addCt_770_);
lean_closure_set(v___f_772_, 5, v_fst_768_);
lean_closure_set(v___f_772_, 6, v_val_769_);
lean_closure_set(v___f_772_, 7, v_fst_767_);
v___x_773_ = lean_apply_2(v_addCt_770_, v_fst_766_, v_val_769_);
v___x_774_ = lean_apply_4(v_toBind_748_, lean_box(0), lean_box(0), v___x_773_, v___f_772_);
return v___x_774_;
}
}
}
LEAN_EXPORT lean_object* lp_zhc__lean_Zhc_Iops_HillisSteele_finalResolution___redArg___lam__10(lean_object* v_toPure_775_, lean_object* v_resolved_776_){
_start:
{
lean_object* v___x_777_; lean_object* v___x_778_; 
v___x_777_ = lp_zhc__lean_Zhc_Iops_HillisSteele_unchunk4___redArg(v_resolved_776_);
v___x_778_ = lean_apply_2(v_toPure_775_, lean_box(0), v___x_777_);
return v___x_778_;
}
}
LEAN_EXPORT lean_object* lp_zhc__lean_Zhc_Iops_HillisSteele_finalResolution___redArg(lean_object* v_inst_780_, lean_object* v_A_781_, lean_object* v_groups_782_, lean_object* v_carries_783_){
_start:
{
lean_object* v_toApplicative_784_; lean_object* v_toBind_785_; lean_object* v_toPure_786_; lean_object* v___f_787_; lean_object* v___x_788_; lean_object* v___f_789_; lean_object* v___f_790_; lean_object* v___x_791_; lean_object* v___x_792_; lean_object* v_prevs_793_; lean_object* v___x_794_; lean_object* v___x_795_; lean_object* v___x_796_; lean_object* v___x_797_; lean_object* v___x_798_; 
v_toApplicative_784_ = lean_ctor_get(v_inst_780_, 0);
v_toBind_785_ = lean_ctor_get(v_inst_780_, 1);
lean_inc_n(v_toBind_785_, 2);
v_toPure_786_ = lean_ctor_get(v_toApplicative_784_, 1);
v___f_787_ = ((lean_object*)(lp_zhc__lean_Zhc_Iops_HillisSteele_finalResolution___redArg___closed__0));
v___x_788_ = lean_box(0);
lean_inc_n(v_toPure_786_, 2);
v___f_789_ = lean_alloc_closure((void*)(lp_zhc__lean_Zhc_Iops_HillisSteele_finalResolution___redArg___lam__9), 4, 3);
lean_closure_set(v___f_789_, 0, v_A_781_);
lean_closure_set(v___f_789_, 1, v_toPure_786_);
lean_closure_set(v___f_789_, 2, v_toBind_785_);
v___f_790_ = lean_alloc_closure((void*)(lp_zhc__lean_Zhc_Iops_HillisSteele_finalResolution___redArg___lam__10), 2, 1);
lean_closure_set(v___f_790_, 0, v_toPure_786_);
v___x_791_ = lean_box(0);
lean_inc(v_carries_783_);
v___x_792_ = l_List_mapTR_loop___redArg(v___f_787_, v_carries_783_, v___x_791_);
v_prevs_793_ = lean_alloc_ctor(1, 2, 0);
lean_ctor_set(v_prevs_793_, 0, v___x_788_);
lean_ctor_set(v_prevs_793_, 1, v___x_792_);
v___x_794_ = lp_zhc__lean_Zhc_Iops_HillisSteele_chunk4___redArg(v_groups_782_);
v___x_795_ = l_List_zipWith___at___00List_zip_spec__0___redArg(v___x_794_, v_carries_783_);
v___x_796_ = l_List_zipWith___at___00List_zip_spec__0___redArg(v___x_795_, v_prevs_793_);
v___x_797_ = l_List_mapM_loop___redArg(v_inst_780_, v___f_789_, v___x_796_, v___x_791_);
v___x_798_ = lean_apply_4(v_toBind_785_, lean_box(0), lean_box(0), v___x_797_, v___f_790_);
return v___x_798_;
}
}
LEAN_EXPORT lean_object* lp_zhc__lean_Zhc_Iops_HillisSteele_finalResolution(lean_object* v_M_799_, lean_object* v_inst_800_, lean_object* v_A_801_, lean_object* v_groups_802_, lean_object* v_carries_803_){
_start:
{
lean_object* v___x_804_; 
v___x_804_ = lp_zhc__lean_Zhc_Iops_HillisSteele_finalResolution___redArg(v_inst_800_, v_A_801_, v_groups_802_, v_carries_803_);
return v___x_804_;
}
}
LEAN_EXPORT lean_object* lp_zhc__lean_Zhc_Iops_HillisSteele_propagate___redArg___lam__0(lean_object* v_addCt_805_, lean_object* v_x_806_){
_start:
{
lean_object* v_fst_807_; lean_object* v_snd_808_; lean_object* v___x_809_; 
v_fst_807_ = lean_ctor_get(v_x_806_, 0);
lean_inc(v_fst_807_);
v_snd_808_ = lean_ctor_get(v_x_806_, 1);
lean_inc(v_snd_808_);
lean_dec_ref(v_x_806_);
v___x_809_ = lean_apply_2(v_addCt_805_, v_fst_807_, v_snd_808_);
return v___x_809_;
}
}
LEAN_EXPORT lean_object* lp_zhc__lean_Zhc_Iops_HillisSteele_propagate___redArg___lam__1(lean_object* v_pbs_810_, lean_object* v_snd_811_, lean_object* v_inst_812_, lean_object* v_added_813_){
_start:
{
lean_object* v___x_814_; lean_object* v___x_815_; lean_object* v___x_816_; lean_object* v___x_817_; lean_object* v___x_818_; 
v___x_814_ = lp_zhc__lean_Zhc_Iops_HillisSteele_msgOnlyLut;
v___x_815_ = lean_apply_1(v_pbs_810_, v___x_814_);
v___x_816_ = lean_alloc_ctor(1, 2, 0);
lean_ctor_set(v___x_816_, 0, v_snd_811_);
lean_ctor_set(v___x_816_, 1, v_added_813_);
v___x_817_ = lean_box(0);
v___x_818_ = l_List_mapM_loop___redArg(v_inst_812_, v___x_815_, v___x_816_, v___x_817_);
return v___x_818_;
}
}
LEAN_EXPORT lean_object* lp_zhc__lean_Zhc_Iops_HillisSteele_propagate___redArg___lam__2(lean_object* v_pbs_819_, lean_object* v_inst_820_, lean_object* v_tail_821_, lean_object* v_carries_822_, lean_object* v___f_823_, lean_object* v_toBind_824_, lean_object* v_____do__lift_825_){
_start:
{
lean_object* v_snd_826_; lean_object* v___f_827_; lean_object* v___x_828_; lean_object* v___x_829_; lean_object* v___x_830_; lean_object* v___x_831_; 
v_snd_826_ = lean_ctor_get(v_____do__lift_825_, 1);
lean_inc(v_snd_826_);
lean_dec_ref(v_____do__lift_825_);
lean_inc_ref(v_inst_820_);
v___f_827_ = lean_alloc_closure((void*)(lp_zhc__lean_Zhc_Iops_HillisSteele_propagate___redArg___lam__1), 4, 3);
lean_closure_set(v___f_827_, 0, v_pbs_819_);
lean_closure_set(v___f_827_, 1, v_snd_826_);
lean_closure_set(v___f_827_, 2, v_inst_820_);
v___x_828_ = l_List_zipWith___at___00List_zip_spec__0___redArg(v_tail_821_, v_carries_822_);
v___x_829_ = lean_box(0);
v___x_830_ = l_List_mapM_loop___redArg(v_inst_820_, v___f_823_, v___x_828_, v___x_829_);
v___x_831_ = lean_apply_4(v_toBind_824_, lean_box(0), lean_box(0), v___x_830_, v___f_827_);
return v___x_831_;
}
}
LEAN_EXPORT lean_object* lp_zhc__lean_Zhc_Iops_HillisSteele_propagate___redArg(lean_object* v_inst_832_, lean_object* v_A_833_, lean_object* v_sums_834_, lean_object* v_carries_835_){
_start:
{
if (lean_obj_tag(v_sums_834_) == 0)
{
lean_object* v_toApplicative_836_; lean_object* v_toPure_837_; lean_object* v___x_838_; 
v_toApplicative_836_ = lean_ctor_get(v_inst_832_, 0);
lean_inc_ref(v_toApplicative_836_);
lean_dec(v_carries_835_);
lean_dec_ref(v_A_833_);
lean_dec_ref(v_inst_832_);
v_toPure_837_ = lean_ctor_get(v_toApplicative_836_, 1);
lean_inc(v_toPure_837_);
lean_dec_ref(v_toApplicative_836_);
v___x_838_ = lean_apply_2(v_toPure_837_, lean_box(0), v_sums_834_);
return v___x_838_;
}
else
{
lean_object* v_toBind_839_; lean_object* v_head_840_; lean_object* v_tail_841_; lean_object* v_addCt_842_; lean_object* v_pbs_843_; lean_object* v_pbs2_844_; lean_object* v___f_845_; lean_object* v___f_846_; lean_object* v___x_847_; lean_object* v___x_848_; lean_object* v___x_849_; 
v_toBind_839_ = lean_ctor_get(v_inst_832_, 1);
lean_inc_n(v_toBind_839_, 2);
v_head_840_ = lean_ctor_get(v_sums_834_, 0);
lean_inc(v_head_840_);
v_tail_841_ = lean_ctor_get(v_sums_834_, 1);
lean_inc(v_tail_841_);
lean_dec_ref_known(v_sums_834_, 2);
v_addCt_842_ = lean_ctor_get(v_A_833_, 3);
lean_inc(v_addCt_842_);
v_pbs_843_ = lean_ctor_get(v_A_833_, 16);
lean_inc(v_pbs_843_);
v_pbs2_844_ = lean_ctor_get(v_A_833_, 17);
lean_inc(v_pbs2_844_);
lean_dec_ref(v_A_833_);
v___f_845_ = lean_alloc_closure((void*)(lp_zhc__lean_Zhc_Iops_HillisSteele_propagate___redArg___lam__0), 2, 1);
lean_closure_set(v___f_845_, 0, v_addCt_842_);
v___f_846_ = lean_alloc_closure((void*)(lp_zhc__lean_Zhc_Iops_HillisSteele_propagate___redArg___lam__2), 7, 6);
lean_closure_set(v___f_846_, 0, v_pbs_843_);
lean_closure_set(v___f_846_, 1, v_inst_832_);
lean_closure_set(v___f_846_, 2, v_tail_841_);
lean_closure_set(v___f_846_, 3, v_carries_835_);
lean_closure_set(v___f_846_, 4, v___f_845_);
lean_closure_set(v___f_846_, 5, v_toBind_839_);
v___x_847_ = lp_zhc__lean_Zhc_Iops_carryMsgLut;
v___x_848_ = lean_apply_2(v_pbs2_844_, v___x_847_, v_head_840_);
v___x_849_ = lean_apply_4(v_toBind_839_, lean_box(0), lean_box(0), v___x_848_, v___f_846_);
return v___x_849_;
}
}
}
LEAN_EXPORT lean_object* lp_zhc__lean_Zhc_Iops_HillisSteele_propagate(lean_object* v_M_850_, lean_object* v_inst_851_, lean_object* v_A_852_, lean_object* v_sums_853_, lean_object* v_carries_854_){
_start:
{
lean_object* v___x_855_; 
v___x_855_ = lp_zhc__lean_Zhc_Iops_HillisSteele_propagate___redArg(v_inst_851_, v_A_852_, v_sums_853_, v_carries_854_);
return v___x_855_;
}
}
LEAN_EXPORT lean_object* lp_zhc__lean_Zhc_Iops_HillisSteele_joinBlocks___redArg___lam__0(lean_object* v_toPure_856_, lean_object* v_acc_857_){
_start:
{
lean_object* v___x_858_; lean_object* v___x_859_; 
v___x_858_ = lean_alloc_ctor(1, 1, 0);
lean_ctor_set(v___x_858_, 0, v_acc_857_);
v___x_859_ = lean_apply_2(v_toPure_856_, lean_box(0), v___x_858_);
return v___x_859_;
}
}
LEAN_EXPORT lean_object* lp_zhc__lean_Zhc_Iops_HillisSteele_joinBlocks___redArg___lam__1(lean_object* v_storeCtBlock_860_, lean_object* v_si_861_, lean_object* v_toBind_862_, lean_object* v___f_863_, lean_object* v_a_864_, lean_object* v_x_865_, lean_object* v___y_866_){
_start:
{
lean_object* v_fst_867_; lean_object* v_snd_868_; lean_object* v___x_869_; lean_object* v___x_870_; 
v_fst_867_ = lean_ctor_get(v_a_864_, 0);
lean_inc(v_fst_867_);
v_snd_868_ = lean_ctor_get(v_a_864_, 1);
lean_inc(v_snd_868_);
lean_dec_ref(v_a_864_);
v___x_869_ = lean_apply_4(v_storeCtBlock_860_, v_si_861_, v___y_866_, v_snd_868_, v_fst_867_);
v___x_870_ = lean_apply_4(v_toBind_862_, lean_box(0), lean_box(0), v___x_869_, v___f_863_);
return v___x_870_;
}
}
LEAN_EXPORT lean_object* lp_zhc__lean_Zhc_Iops_HillisSteele_joinBlocks___redArg___lam__2(lean_object* v_toPure_871_, lean_object* v_____s_872_){
_start:
{
lean_object* v___x_873_; 
v___x_873_ = lean_apply_2(v_toPure_871_, lean_box(0), v_____s_872_);
return v___x_873_;
}
}
LEAN_EXPORT lean_object* lp_zhc__lean_Zhc_Iops_HillisSteele_joinBlocks___redArg___lam__3(lean_object* v_si_874_, lean_object* v_blocks_875_, lean_object* v_inst_876_, lean_object* v___f_877_, lean_object* v_toBind_878_, lean_object* v___f_879_, lean_object* v_acc_880_){
_start:
{
lean_object* v___x_881_; lean_object* v___x_882_; lean_object* v___x_883_; lean_object* v___x_884_; 
v___x_881_ = l_List_finRange(v_si_874_);
v___x_882_ = l_List_zipWith___at___00List_zip_spec__0___redArg(v_blocks_875_, v___x_881_);
v___x_883_ = l_List_forIn_x27_loop___redArg(v_inst_876_, v___f_877_, v___x_882_, v_acc_880_);
lean_dec(v___x_882_);
v___x_884_ = lean_apply_4(v_toBind_878_, lean_box(0), lean_box(0), v___x_883_, v___f_879_);
return v___x_884_;
}
}
LEAN_EXPORT lean_object* lp_zhc__lean_Zhc_Iops_HillisSteele_joinBlocks___redArg(lean_object* v_inst_885_, lean_object* v_A_886_, lean_object* v_si_887_, lean_object* v_blocks_888_){
_start:
{
lean_object* v_toApplicative_889_; lean_object* v_toBind_890_; lean_object* v_declareInteger_891_; lean_object* v_storeCtBlock_892_; lean_object* v_toPure_893_; lean_object* v___x_894_; lean_object* v___f_895_; lean_object* v___f_896_; lean_object* v___f_897_; lean_object* v___f_898_; lean_object* v___x_899_; 
v_toApplicative_889_ = lean_ctor_get(v_inst_885_, 0);
v_toBind_890_ = lean_ctor_get(v_inst_885_, 1);
lean_inc_n(v_toBind_890_, 3);
v_declareInteger_891_ = lean_ctor_get(v_A_886_, 0);
lean_inc(v_declareInteger_891_);
v_storeCtBlock_892_ = lean_ctor_get(v_A_886_, 13);
lean_inc(v_storeCtBlock_892_);
lean_dec_ref(v_A_886_);
v_toPure_893_ = lean_ctor_get(v_toApplicative_889_, 1);
lean_inc_n(v_si_887_, 2);
v___x_894_ = lean_apply_1(v_declareInteger_891_, v_si_887_);
lean_inc_n(v_toPure_893_, 2);
v___f_895_ = lean_alloc_closure((void*)(lp_zhc__lean_Zhc_Iops_HillisSteele_joinBlocks___redArg___lam__0), 2, 1);
lean_closure_set(v___f_895_, 0, v_toPure_893_);
v___f_896_ = lean_alloc_closure((void*)(lp_zhc__lean_Zhc_Iops_HillisSteele_joinBlocks___redArg___lam__1), 7, 4);
lean_closure_set(v___f_896_, 0, v_storeCtBlock_892_);
lean_closure_set(v___f_896_, 1, v_si_887_);
lean_closure_set(v___f_896_, 2, v_toBind_890_);
lean_closure_set(v___f_896_, 3, v___f_895_);
v___f_897_ = lean_alloc_closure((void*)(lp_zhc__lean_Zhc_Iops_HillisSteele_joinBlocks___redArg___lam__2), 2, 1);
lean_closure_set(v___f_897_, 0, v_toPure_893_);
v___f_898_ = lean_alloc_closure((void*)(lp_zhc__lean_Zhc_Iops_HillisSteele_joinBlocks___redArg___lam__3), 7, 6);
lean_closure_set(v___f_898_, 0, v_si_887_);
lean_closure_set(v___f_898_, 1, v_blocks_888_);
lean_closure_set(v___f_898_, 2, v_inst_885_);
lean_closure_set(v___f_898_, 3, v___f_896_);
lean_closure_set(v___f_898_, 4, v_toBind_890_);
lean_closure_set(v___f_898_, 5, v___f_897_);
v___x_899_ = lean_apply_4(v_toBind_890_, lean_box(0), lean_box(0), v___x_894_, v___f_898_);
return v___x_899_;
}
}
LEAN_EXPORT lean_object* lp_zhc__lean_Zhc_Iops_HillisSteele_joinBlocks(lean_object* v_M_900_, lean_object* v_inst_901_, lean_object* v_A_902_, lean_object* v_si_903_, lean_object* v_blocks_904_){
_start:
{
lean_object* v___x_905_; 
v___x_905_ = lp_zhc__lean_Zhc_Iops_HillisSteele_joinBlocks___redArg(v_inst_901_, v_A_902_, v_si_903_, v_blocks_904_);
return v___x_905_;
}
}
LEAN_EXPORT lean_object* lp_zhc__lean_Zhc_Iops_addHillisSteele___redArg___lam__0(lean_object* v_inst_906_, lean_object* v_A_907_, lean_object* v_si_908_, lean_object* v_res_909_){
_start:
{
lean_object* v___x_910_; 
v___x_910_ = lp_zhc__lean_Zhc_Iops_HillisSteele_joinBlocks___redArg(v_inst_906_, v_A_907_, v_si_908_, v_res_909_);
return v___x_910_;
}
}
LEAN_EXPORT lean_object* lp_zhc__lean_Zhc_Iops_addHillisSteele___redArg___lam__1(lean_object* v_inst_911_, lean_object* v_A_912_, lean_object* v_sums_913_, lean_object* v_toBind_914_, lean_object* v___f_915_, lean_object* v_blockCarries_916_){
_start:
{
lean_object* v___x_917_; lean_object* v___x_918_; 
v___x_917_ = lp_zhc__lean_Zhc_Iops_HillisSteele_propagate___redArg(v_inst_911_, v_A_912_, v_sums_913_, v_blockCarries_916_);
v___x_918_ = lean_apply_4(v_toBind_914_, lean_box(0), lean_box(0), v___x_917_, v___f_915_);
return v___x_918_;
}
}
LEAN_EXPORT lean_object* lp_zhc__lean_Zhc_Iops_addHillisSteele___redArg___lam__2(lean_object* v_inst_919_, lean_object* v_A_920_, lean_object* v_groups_921_, lean_object* v_toBind_922_, lean_object* v___f_923_, lean_object* v_carries_924_){
_start:
{
lean_object* v___x_925_; lean_object* v___x_926_; 
v___x_925_ = lp_zhc__lean_Zhc_Iops_HillisSteele_finalResolution___redArg(v_inst_919_, v_A_920_, v_groups_921_, v_carries_924_);
v___x_926_ = lean_apply_4(v_toBind_922_, lean_box(0), lean_box(0), v___x_925_, v___f_923_);
return v___x_926_;
}
}
LEAN_EXPORT lean_object* lp_zhc__lean_Zhc_Iops_addHillisSteele___redArg___lam__3(lean_object* v_inst_927_, lean_object* v_A_928_, lean_object* v_toBind_929_, lean_object* v___f_930_, lean_object* v_groups_931_){
_start:
{
lean_object* v___f_932_; lean_object* v___x_933_; lean_object* v___x_934_; 
lean_inc(v_toBind_929_);
lean_inc(v_groups_931_);
lean_inc_ref(v_A_928_);
lean_inc_ref(v_inst_927_);
v___f_932_ = lean_alloc_closure((void*)(lp_zhc__lean_Zhc_Iops_addHillisSteele___redArg___lam__2), 6, 5);
lean_closure_set(v___f_932_, 0, v_inst_927_);
lean_closure_set(v___f_932_, 1, v_A_928_);
lean_closure_set(v___f_932_, 2, v_groups_931_);
lean_closure_set(v___f_932_, 3, v_toBind_929_);
lean_closure_set(v___f_932_, 4, v___f_930_);
v___x_933_ = lp_zhc__lean_Zhc_Iops_HillisSteele_groupCarries___redArg(v_inst_927_, v_A_928_, v_groups_931_);
v___x_934_ = lean_apply_4(v_toBind_929_, lean_box(0), lean_box(0), v___x_933_, v___f_932_);
return v___x_934_;
}
}
LEAN_EXPORT lean_object* lp_zhc__lean_Zhc_Iops_addHillisSteele___redArg___lam__4(lean_object* v_inst_935_, lean_object* v_A_936_, lean_object* v_toBind_937_, lean_object* v___f_938_, lean_object* v_states_939_){
_start:
{
lean_object* v___x_940_; lean_object* v___x_941_; 
v___x_940_ = lp_zhc__lean_Zhc_Iops_HillisSteele_groupStates___redArg(v_inst_935_, v_A_936_, v_states_939_);
v___x_941_ = lean_apply_4(v_toBind_937_, lean_box(0), lean_box(0), v___x_940_, v___f_938_);
return v___x_941_;
}
}
LEAN_EXPORT lean_object* lp_zhc__lean_Zhc_Iops_addHillisSteele___redArg___lam__5(lean_object* v_inst_942_, lean_object* v_A_943_, lean_object* v_toBind_944_, lean_object* v___f_945_, lean_object* v_sums_946_){
_start:
{
lean_object* v___f_947_; lean_object* v___f_948_; lean_object* v___f_949_; lean_object* v___x_950_; lean_object* v___x_951_; 
lean_inc_n(v_toBind_944_, 3);
lean_inc(v_sums_946_);
lean_inc_ref_n(v_A_943_, 3);
lean_inc_ref_n(v_inst_942_, 3);
v___f_947_ = lean_alloc_closure((void*)(lp_zhc__lean_Zhc_Iops_addHillisSteele___redArg___lam__1), 6, 5);
lean_closure_set(v___f_947_, 0, v_inst_942_);
lean_closure_set(v___f_947_, 1, v_A_943_);
lean_closure_set(v___f_947_, 2, v_sums_946_);
lean_closure_set(v___f_947_, 3, v_toBind_944_);
lean_closure_set(v___f_947_, 4, v___f_945_);
v___f_948_ = lean_alloc_closure((void*)(lp_zhc__lean_Zhc_Iops_addHillisSteele___redArg___lam__3), 5, 4);
lean_closure_set(v___f_948_, 0, v_inst_942_);
lean_closure_set(v___f_948_, 1, v_A_943_);
lean_closure_set(v___f_948_, 2, v_toBind_944_);
lean_closure_set(v___f_948_, 3, v___f_947_);
v___f_949_ = lean_alloc_closure((void*)(lp_zhc__lean_Zhc_Iops_addHillisSteele___redArg___lam__4), 5, 4);
lean_closure_set(v___f_949_, 0, v_inst_942_);
lean_closure_set(v___f_949_, 1, v_A_943_);
lean_closure_set(v___f_949_, 2, v_toBind_944_);
lean_closure_set(v___f_949_, 3, v___f_948_);
v___x_950_ = lp_zhc__lean_Zhc_Iops_HillisSteele_blockStates___redArg(v_inst_942_, v_A_943_, v_sums_946_);
v___x_951_ = lean_apply_4(v_toBind_944_, lean_box(0), lean_box(0), v___x_950_, v___f_949_);
return v___x_951_;
}
}
LEAN_EXPORT lean_object* lp_zhc__lean_Zhc_Iops_addHillisSteele___redArg(lean_object* v_inst_952_, lean_object* v_A_953_, lean_object* v_si_954_, lean_object* v_x_955_, lean_object* v_y_956_){
_start:
{
lean_object* v_toBind_957_; lean_object* v___f_958_; lean_object* v___f_959_; lean_object* v___x_960_; lean_object* v___x_961_; 
v_toBind_957_ = lean_ctor_get(v_inst_952_, 1);
lean_inc_n(v_toBind_957_, 2);
lean_inc(v_si_954_);
lean_inc_ref_n(v_A_953_, 2);
lean_inc_ref_n(v_inst_952_, 2);
v___f_958_ = lean_alloc_closure((void*)(lp_zhc__lean_Zhc_Iops_addHillisSteele___redArg___lam__0), 4, 3);
lean_closure_set(v___f_958_, 0, v_inst_952_);
lean_closure_set(v___f_958_, 1, v_A_953_);
lean_closure_set(v___f_958_, 2, v_si_954_);
v___f_959_ = lean_alloc_closure((void*)(lp_zhc__lean_Zhc_Iops_addHillisSteele___redArg___lam__5), 5, 4);
lean_closure_set(v___f_959_, 0, v_inst_952_);
lean_closure_set(v___f_959_, 1, v_A_953_);
lean_closure_set(v___f_959_, 2, v_toBind_957_);
lean_closure_set(v___f_959_, 3, v___f_958_);
v___x_960_ = lp_zhc__lean_Zhc_Iops_HillisSteele_rawSums___redArg(v_inst_952_, v_A_953_, v_si_954_, v_x_955_, v_y_956_);
v___x_961_ = lean_apply_4(v_toBind_957_, lean_box(0), lean_box(0), v___x_960_, v___f_959_);
return v___x_961_;
}
}
LEAN_EXPORT lean_object* lp_zhc__lean_Zhc_Iops_addHillisSteele(lean_object* v_M_962_, lean_object* v_inst_963_, lean_object* v_A_964_, lean_object* v_si_965_, lean_object* v_x_966_, lean_object* v_y_967_){
_start:
{
lean_object* v___x_968_; 
v___x_968_ = lp_zhc__lean_Zhc_Iops_addHillisSteele___redArg(v_inst_963_, v_A_964_, v_si_965_, v_x_966_, v_y_967_);
return v___x_968_;
}
}
static lean_object* _init_lp_zhc__lean_List_mapM_loop___at___00Zhc_Iops_HillisSteele_finalResolution___at___00Zhc_Iops_addHillisSteele___at___00__private_Zhc_Iops_AddHillisSteele_Impl_0__Zhc_Iops_smokeHs_spec__0_spec__4_spec__12___closed__0(void){
_start:
{
lean_object* v___x_969_; lean_object* v___x_970_; lean_object* v___x_971_; 
v___x_969_ = lean_unsigned_to_nat(1u);
v___x_970_ = lean_unsigned_to_nat(0u);
v___x_971_ = lean_nat_mod(v___x_970_, v___x_969_);
return v___x_971_;
}
}
static lean_object* _init_lp_zhc__lean_List_mapM_loop___at___00Zhc_Iops_HillisSteele_finalResolution___at___00Zhc_Iops_addHillisSteele___at___00__private_Zhc_Iops_AddHillisSteele_Impl_0__Zhc_Iops_smokeHs_spec__0_spec__4_spec__12___closed__1(void){
_start:
{
lean_object* v___x_972_; lean_object* v___x_973_; lean_object* v___x_974_; 
v___x_972_ = lean_unsigned_to_nat(3u);
v___x_973_ = lean_unsigned_to_nat(1u);
v___x_974_ = lean_nat_mod(v___x_973_, v___x_972_);
return v___x_974_;
}
}
static lean_object* _init_lp_zhc__lean_List_mapM_loop___at___00Zhc_Iops_HillisSteele_finalResolution___at___00Zhc_Iops_addHillisSteele___at___00__private_Zhc_Iops_AddHillisSteele_Impl_0__Zhc_Iops_smokeHs_spec__0_spec__4_spec__12___closed__2(void){
_start:
{
lean_object* v___x_975_; lean_object* v___x_976_; 
v___x_975_ = lean_obj_once(&lp_zhc__lean_List_mapM_loop___at___00Zhc_Iops_HillisSteele_finalResolution___at___00Zhc_Iops_addHillisSteele___at___00__private_Zhc_Iops_AddHillisSteele_Impl_0__Zhc_Iops_smokeHs_spec__0_spec__4_spec__12___closed__1, &lp_zhc__lean_List_mapM_loop___at___00Zhc_Iops_HillisSteele_finalResolution___at___00Zhc_Iops_addHillisSteele___at___00__private_Zhc_Iops_AddHillisSteele_Impl_0__Zhc_Iops_smokeHs_spec__0_spec__4_spec__12___closed__1_once, _init_lp_zhc__lean_List_mapM_loop___at___00Zhc_Iops_HillisSteele_finalResolution___at___00Zhc_Iops_addHillisSteele___at___00__private_Zhc_Iops_AddHillisSteele_Impl_0__Zhc_Iops_smokeHs_spec__0_spec__4_spec__12___closed__1);
v___x_976_ = lp_zhc__lean_Zhc_Iops_HillisSteele_solvePropGroupFinalLut(v___x_975_);
return v___x_976_;
}
}
LEAN_EXPORT lean_object* lp_zhc__lean_List_mapM_loop___at___00Zhc_Iops_HillisSteele_finalResolution___at___00Zhc_Iops_addHillisSteele___at___00__private_Zhc_Iops_AddHillisSteele_Impl_0__Zhc_Iops_smokeHs_spec__0_spec__4_spec__12(lean_object* v_x_977_, lean_object* v_x_978_){
_start:
{
lean_object* v___x_979_; 
v___x_979_ = ((lean_object*)(lp_zhc__lean_Zhc_Iops_HillisSteele_extractPropGroupLut___closed__0));
if (lean_obj_tag(v_x_977_) == 0)
{
lean_object* v___x_980_; 
v___x_980_ = l_List_reverse___redArg(v_x_978_);
return v___x_980_;
}
else
{
lean_object* v_head_981_; lean_object* v_tail_982_; lean_object* v___x_984_; uint8_t v_isShared_985_; uint8_t v_isSharedCheck_1071_; 
v_head_981_ = lean_ctor_get(v_x_977_, 0);
v_tail_982_ = lean_ctor_get(v_x_977_, 1);
v_isSharedCheck_1071_ = !lean_is_exclusive(v_x_977_);
if (v_isSharedCheck_1071_ == 0)
{
v___x_984_ = v_x_977_;
v_isShared_985_ = v_isSharedCheck_1071_;
goto v_resetjp_983_;
}
else
{
lean_inc(v_tail_982_);
lean_inc(v_head_981_);
lean_dec(v_x_977_);
v___x_984_ = lean_box(0);
v_isShared_985_ = v_isSharedCheck_1071_;
goto v_resetjp_983_;
}
v_resetjp_983_:
{
lean_object* v___y_987_; lean_object* v_fst_992_; lean_object* v_fst_993_; lean_object* v_snd_994_; lean_object* v_snd_995_; lean_object* v_snd_996_; 
v_fst_992_ = lean_ctor_get(v_head_981_, 0);
lean_inc(v_fst_992_);
v_fst_993_ = lean_ctor_get(v_fst_992_, 0);
lean_inc(v_fst_993_);
v_snd_994_ = lean_ctor_get(v_fst_993_, 1);
lean_inc(v_snd_994_);
v_snd_995_ = lean_ctor_get(v_snd_994_, 1);
lean_inc(v_snd_995_);
v_snd_996_ = lean_ctor_get(v_head_981_, 1);
lean_inc(v_snd_996_);
lean_dec(v_head_981_);
if (lean_obj_tag(v_snd_996_) == 0)
{
lean_object* v_snd_997_; lean_object* v_fst_998_; lean_object* v___x_1000_; uint8_t v_isShared_1001_; uint8_t v_isSharedCheck_1029_; 
v_snd_997_ = lean_ctor_get(v_fst_992_, 1);
lean_inc(v_snd_997_);
lean_dec(v_fst_992_);
v_fst_998_ = lean_ctor_get(v_fst_993_, 0);
v_isSharedCheck_1029_ = !lean_is_exclusive(v_fst_993_);
if (v_isSharedCheck_1029_ == 0)
{
lean_object* v_unused_1030_; 
v_unused_1030_ = lean_ctor_get(v_fst_993_, 1);
lean_dec(v_unused_1030_);
v___x_1000_ = v_fst_993_;
v_isShared_1001_ = v_isSharedCheck_1029_;
goto v_resetjp_999_;
}
else
{
lean_inc(v_fst_998_);
lean_dec(v_fst_993_);
v___x_1000_ = lean_box(0);
v_isShared_1001_ = v_isSharedCheck_1029_;
goto v_resetjp_999_;
}
v_resetjp_999_:
{
lean_object* v_fst_1002_; lean_object* v___x_1004_; uint8_t v_isShared_1005_; uint8_t v_isSharedCheck_1027_; 
v_fst_1002_ = lean_ctor_get(v_snd_994_, 0);
v_isSharedCheck_1027_ = !lean_is_exclusive(v_snd_994_);
if (v_isSharedCheck_1027_ == 0)
{
lean_object* v_unused_1028_; 
v_unused_1028_ = lean_ctor_get(v_snd_994_, 1);
lean_dec(v_unused_1028_);
v___x_1004_ = v_snd_994_;
v_isShared_1005_ = v_isSharedCheck_1027_;
goto v_resetjp_1003_;
}
else
{
lean_inc(v_fst_1002_);
lean_dec(v_snd_994_);
v___x_1004_ = lean_box(0);
v_isShared_1005_ = v_isSharedCheck_1027_;
goto v_resetjp_1003_;
}
v_resetjp_1003_:
{
lean_object* v_fst_1006_; lean_object* v___x_1008_; uint8_t v_isShared_1009_; uint8_t v_isSharedCheck_1025_; 
v_fst_1006_ = lean_ctor_get(v_snd_995_, 0);
v_isSharedCheck_1025_ = !lean_is_exclusive(v_snd_995_);
if (v_isSharedCheck_1025_ == 0)
{
lean_object* v_unused_1026_; 
v_unused_1026_ = lean_ctor_get(v_snd_995_, 1);
lean_dec(v_unused_1026_);
v___x_1008_ = v_snd_995_;
v_isShared_1009_ = v_isSharedCheck_1025_;
goto v_resetjp_1007_;
}
else
{
lean_inc(v_fst_1006_);
lean_dec(v_snd_995_);
v___x_1008_ = lean_box(0);
v_isShared_1009_ = v_isSharedCheck_1025_;
goto v_resetjp_1007_;
}
v_resetjp_1007_:
{
lean_object* v___x_1010_; lean_object* v___x_1011_; lean_object* v___x_1012_; lean_object* v___x_1013_; lean_object* v___x_1014_; lean_object* v___x_1015_; lean_object* v___x_1017_; 
v___x_1010_ = lean_unsigned_to_nat(0u);
v___x_1011_ = lean_obj_once(&lp_zhc__lean_Zhc_Iops_HillisSteele_finalResolution___redArg___lam__8___closed__1, &lp_zhc__lean_Zhc_Iops_HillisSteele_finalResolution___redArg___lam__8___closed__1_once, _init_lp_zhc__lean_Zhc_Iops_HillisSteele_finalResolution___redArg___lam__8___closed__1);
v___x_1012_ = lean_obj_once(&lp_zhc__lean_List_mapM_loop___at___00Zhc_Iops_HillisSteele_finalResolution___at___00Zhc_Iops_addHillisSteele___at___00__private_Zhc_Iops_AddHillisSteele_Impl_0__Zhc_Iops_smokeHs_spec__0_spec__4_spec__12___closed__0, &lp_zhc__lean_List_mapM_loop___at___00Zhc_Iops_HillisSteele_finalResolution___at___00Zhc_Iops_addHillisSteele___at___00__private_Zhc_Iops_AddHillisSteele_Impl_0__Zhc_Iops_smokeHs_spec__0_spec__4_spec__12___closed__0_once, _init_lp_zhc__lean_List_mapM_loop___at___00Zhc_Iops_HillisSteele_finalResolution___at___00Zhc_Iops_addHillisSteele___at___00__private_Zhc_Iops_AddHillisSteele_Impl_0__Zhc_Iops_smokeHs_spec__0_spec__4_spec__12___closed__0);
v___x_1013_ = lp_zhc__lean_Zhc_Model_ManyLut_output(v___x_979_, v___x_1010_, v___x_1011_, v_fst_1002_, v___x_1012_);
lean_dec(v_fst_1002_);
v___x_1014_ = lean_obj_once(&lp_zhc__lean_List_mapM_loop___at___00Zhc_Iops_HillisSteele_finalResolution___at___00Zhc_Iops_addHillisSteele___at___00__private_Zhc_Iops_AddHillisSteele_Impl_0__Zhc_Iops_smokeHs_spec__0_spec__4_spec__12___closed__2, &lp_zhc__lean_List_mapM_loop___at___00Zhc_Iops_HillisSteele_finalResolution___at___00Zhc_Iops_addHillisSteele___at___00__private_Zhc_Iops_AddHillisSteele_Impl_0__Zhc_Iops_smokeHs_spec__0_spec__4_spec__12___closed__2_once, _init_lp_zhc__lean_List_mapM_loop___at___00Zhc_Iops_HillisSteele_finalResolution___at___00Zhc_Iops_addHillisSteele___at___00__private_Zhc_Iops_AddHillisSteele_Impl_0__Zhc_Iops_smokeHs_spec__0_spec__4_spec__12___closed__2);
v___x_1015_ = lp_zhc__lean_Zhc_Model_ManyLut_output(v___x_979_, v___x_1010_, v___x_1014_, v_fst_1006_, v___x_1012_);
lean_dec(v_fst_1006_);
if (v_isShared_1009_ == 0)
{
lean_ctor_set(v___x_1008_, 1, v_snd_997_);
lean_ctor_set(v___x_1008_, 0, v___x_1015_);
v___x_1017_ = v___x_1008_;
goto v_reusejp_1016_;
}
else
{
lean_object* v_reuseFailAlloc_1024_; 
v_reuseFailAlloc_1024_ = lean_alloc_ctor(0, 2, 0);
lean_ctor_set(v_reuseFailAlloc_1024_, 0, v___x_1015_);
lean_ctor_set(v_reuseFailAlloc_1024_, 1, v_snd_997_);
v___x_1017_ = v_reuseFailAlloc_1024_;
goto v_reusejp_1016_;
}
v_reusejp_1016_:
{
lean_object* v___x_1019_; 
if (v_isShared_1005_ == 0)
{
lean_ctor_set(v___x_1004_, 1, v___x_1017_);
lean_ctor_set(v___x_1004_, 0, v___x_1013_);
v___x_1019_ = v___x_1004_;
goto v_reusejp_1018_;
}
else
{
lean_object* v_reuseFailAlloc_1023_; 
v_reuseFailAlloc_1023_ = lean_alloc_ctor(0, 2, 0);
lean_ctor_set(v_reuseFailAlloc_1023_, 0, v___x_1013_);
lean_ctor_set(v_reuseFailAlloc_1023_, 1, v___x_1017_);
v___x_1019_ = v_reuseFailAlloc_1023_;
goto v_reusejp_1018_;
}
v_reusejp_1018_:
{
lean_object* v___x_1021_; 
if (v_isShared_1001_ == 0)
{
lean_ctor_set(v___x_1000_, 1, v___x_1019_);
v___x_1021_ = v___x_1000_;
goto v_reusejp_1020_;
}
else
{
lean_object* v_reuseFailAlloc_1022_; 
v_reuseFailAlloc_1022_ = lean_alloc_ctor(0, 2, 0);
lean_ctor_set(v_reuseFailAlloc_1022_, 0, v_fst_998_);
lean_ctor_set(v_reuseFailAlloc_1022_, 1, v___x_1019_);
v___x_1021_ = v_reuseFailAlloc_1022_;
goto v_reusejp_1020_;
}
v_reusejp_1020_:
{
v___y_987_ = v___x_1021_;
goto v___jp_986_;
}
}
}
}
}
}
}
else
{
lean_object* v_snd_1031_; lean_object* v_fst_1032_; lean_object* v___x_1034_; uint8_t v_isShared_1035_; uint8_t v_isSharedCheck_1069_; 
v_snd_1031_ = lean_ctor_get(v_fst_992_, 1);
lean_inc(v_snd_1031_);
lean_dec(v_fst_992_);
v_fst_1032_ = lean_ctor_get(v_fst_993_, 0);
v_isSharedCheck_1069_ = !lean_is_exclusive(v_fst_993_);
if (v_isSharedCheck_1069_ == 0)
{
lean_object* v_unused_1070_; 
v_unused_1070_ = lean_ctor_get(v_fst_993_, 1);
lean_dec(v_unused_1070_);
v___x_1034_ = v_fst_993_;
v_isShared_1035_ = v_isSharedCheck_1069_;
goto v_resetjp_1033_;
}
else
{
lean_inc(v_fst_1032_);
lean_dec(v_fst_993_);
v___x_1034_ = lean_box(0);
v_isShared_1035_ = v_isSharedCheck_1069_;
goto v_resetjp_1033_;
}
v_resetjp_1033_:
{
lean_object* v_fst_1036_; lean_object* v___x_1038_; uint8_t v_isShared_1039_; uint8_t v_isSharedCheck_1067_; 
v_fst_1036_ = lean_ctor_get(v_snd_994_, 0);
v_isSharedCheck_1067_ = !lean_is_exclusive(v_snd_994_);
if (v_isSharedCheck_1067_ == 0)
{
lean_object* v_unused_1068_; 
v_unused_1068_ = lean_ctor_get(v_snd_994_, 1);
lean_dec(v_unused_1068_);
v___x_1038_ = v_snd_994_;
v_isShared_1039_ = v_isSharedCheck_1067_;
goto v_resetjp_1037_;
}
else
{
lean_inc(v_fst_1036_);
lean_dec(v_snd_994_);
v___x_1038_ = lean_box(0);
v_isShared_1039_ = v_isSharedCheck_1067_;
goto v_resetjp_1037_;
}
v_resetjp_1037_:
{
lean_object* v_fst_1040_; lean_object* v___x_1042_; uint8_t v_isShared_1043_; uint8_t v_isSharedCheck_1065_; 
v_fst_1040_ = lean_ctor_get(v_snd_995_, 0);
v_isSharedCheck_1065_ = !lean_is_exclusive(v_snd_995_);
if (v_isSharedCheck_1065_ == 0)
{
lean_object* v_unused_1066_; 
v_unused_1066_ = lean_ctor_get(v_snd_995_, 1);
lean_dec(v_unused_1066_);
v___x_1042_ = v_snd_995_;
v_isShared_1043_ = v_isSharedCheck_1065_;
goto v_resetjp_1041_;
}
else
{
lean_inc(v_fst_1040_);
lean_dec(v_snd_995_);
v___x_1042_ = lean_box(0);
v_isShared_1043_ = v_isSharedCheck_1065_;
goto v_resetjp_1041_;
}
v_resetjp_1041_:
{
lean_object* v_val_1044_; lean_object* v___x_1045_; lean_object* v___x_1046_; lean_object* v___x_1047_; lean_object* v___x_1048_; lean_object* v___x_1049_; lean_object* v___x_1050_; lean_object* v___x_1051_; lean_object* v___x_1052_; lean_object* v___x_1053_; lean_object* v___x_1054_; lean_object* v___x_1055_; lean_object* v___x_1057_; 
v_val_1044_ = lean_ctor_get(v_snd_996_, 0);
lean_inc(v_val_1044_);
lean_dec_ref_known(v_snd_996_, 1);
v___x_1045_ = lp_zhc__lean_Zhc_Model_Operators_add(v___x_979_, v_fst_1032_, v_val_1044_);
lean_dec(v_fst_1032_);
v___x_1046_ = lean_unsigned_to_nat(0u);
v___x_1047_ = lean_obj_once(&lp_zhc__lean_Zhc_Iops_HillisSteele_finalResolution___redArg___lam__8___closed__1, &lp_zhc__lean_Zhc_Iops_HillisSteele_finalResolution___redArg___lam__8___closed__1_once, _init_lp_zhc__lean_Zhc_Iops_HillisSteele_finalResolution___redArg___lam__8___closed__1);
v___x_1048_ = lean_obj_once(&lp_zhc__lean_List_mapM_loop___at___00Zhc_Iops_HillisSteele_finalResolution___at___00Zhc_Iops_addHillisSteele___at___00__private_Zhc_Iops_AddHillisSteele_Impl_0__Zhc_Iops_smokeHs_spec__0_spec__4_spec__12___closed__0, &lp_zhc__lean_List_mapM_loop___at___00Zhc_Iops_HillisSteele_finalResolution___at___00Zhc_Iops_addHillisSteele___at___00__private_Zhc_Iops_AddHillisSteele_Impl_0__Zhc_Iops_smokeHs_spec__0_spec__4_spec__12___closed__0_once, _init_lp_zhc__lean_List_mapM_loop___at___00Zhc_Iops_HillisSteele_finalResolution___at___00Zhc_Iops_addHillisSteele___at___00__private_Zhc_Iops_AddHillisSteele_Impl_0__Zhc_Iops_smokeHs_spec__0_spec__4_spec__12___closed__0);
v___x_1049_ = lp_zhc__lean_Zhc_Model_ManyLut_output(v___x_979_, v___x_1046_, v___x_1047_, v___x_1045_, v___x_1048_);
lean_dec(v___x_1045_);
v___x_1050_ = lp_zhc__lean_Zhc_Model_Operators_add(v___x_979_, v_fst_1036_, v_val_1044_);
lean_dec(v_fst_1036_);
v___x_1051_ = lean_obj_once(&lp_zhc__lean_List_mapM_loop___at___00Zhc_Iops_HillisSteele_finalResolution___at___00Zhc_Iops_addHillisSteele___at___00__private_Zhc_Iops_AddHillisSteele_Impl_0__Zhc_Iops_smokeHs_spec__0_spec__4_spec__12___closed__2, &lp_zhc__lean_List_mapM_loop___at___00Zhc_Iops_HillisSteele_finalResolution___at___00Zhc_Iops_addHillisSteele___at___00__private_Zhc_Iops_AddHillisSteele_Impl_0__Zhc_Iops_smokeHs_spec__0_spec__4_spec__12___closed__2_once, _init_lp_zhc__lean_List_mapM_loop___at___00Zhc_Iops_HillisSteele_finalResolution___at___00Zhc_Iops_addHillisSteele___at___00__private_Zhc_Iops_AddHillisSteele_Impl_0__Zhc_Iops_smokeHs_spec__0_spec__4_spec__12___closed__2);
v___x_1052_ = lp_zhc__lean_Zhc_Model_ManyLut_output(v___x_979_, v___x_1046_, v___x_1051_, v___x_1050_, v___x_1048_);
lean_dec(v___x_1050_);
v___x_1053_ = lp_zhc__lean_Zhc_Model_Operators_add(v___x_979_, v_fst_1040_, v_val_1044_);
lean_dec(v_val_1044_);
lean_dec(v_fst_1040_);
v___x_1054_ = lean_obj_once(&lp_zhc__lean_Zhc_Iops_HillisSteele_groupStates___redArg___lam__5___closed__1, &lp_zhc__lean_Zhc_Iops_HillisSteele_groupStates___redArg___lam__5___closed__1_once, _init_lp_zhc__lean_Zhc_Iops_HillisSteele_groupStates___redArg___lam__5___closed__1);
v___x_1055_ = lp_zhc__lean_Zhc_Model_ManyLut_output(v___x_979_, v___x_1046_, v___x_1054_, v___x_1053_, v___x_1048_);
lean_dec(v___x_1053_);
if (v_isShared_1043_ == 0)
{
lean_ctor_set(v___x_1042_, 1, v_snd_1031_);
lean_ctor_set(v___x_1042_, 0, v___x_1055_);
v___x_1057_ = v___x_1042_;
goto v_reusejp_1056_;
}
else
{
lean_object* v_reuseFailAlloc_1064_; 
v_reuseFailAlloc_1064_ = lean_alloc_ctor(0, 2, 0);
lean_ctor_set(v_reuseFailAlloc_1064_, 0, v___x_1055_);
lean_ctor_set(v_reuseFailAlloc_1064_, 1, v_snd_1031_);
v___x_1057_ = v_reuseFailAlloc_1064_;
goto v_reusejp_1056_;
}
v_reusejp_1056_:
{
lean_object* v___x_1059_; 
if (v_isShared_1039_ == 0)
{
lean_ctor_set(v___x_1038_, 1, v___x_1057_);
lean_ctor_set(v___x_1038_, 0, v___x_1052_);
v___x_1059_ = v___x_1038_;
goto v_reusejp_1058_;
}
else
{
lean_object* v_reuseFailAlloc_1063_; 
v_reuseFailAlloc_1063_ = lean_alloc_ctor(0, 2, 0);
lean_ctor_set(v_reuseFailAlloc_1063_, 0, v___x_1052_);
lean_ctor_set(v_reuseFailAlloc_1063_, 1, v___x_1057_);
v___x_1059_ = v_reuseFailAlloc_1063_;
goto v_reusejp_1058_;
}
v_reusejp_1058_:
{
lean_object* v___x_1061_; 
if (v_isShared_1035_ == 0)
{
lean_ctor_set(v___x_1034_, 1, v___x_1059_);
lean_ctor_set(v___x_1034_, 0, v___x_1049_);
v___x_1061_ = v___x_1034_;
goto v_reusejp_1060_;
}
else
{
lean_object* v_reuseFailAlloc_1062_; 
v_reuseFailAlloc_1062_ = lean_alloc_ctor(0, 2, 0);
lean_ctor_set(v_reuseFailAlloc_1062_, 0, v___x_1049_);
lean_ctor_set(v_reuseFailAlloc_1062_, 1, v___x_1059_);
v___x_1061_ = v_reuseFailAlloc_1062_;
goto v_reusejp_1060_;
}
v_reusejp_1060_:
{
v___y_987_ = v___x_1061_;
goto v___jp_986_;
}
}
}
}
}
}
}
v___jp_986_:
{
lean_object* v___x_989_; 
if (v_isShared_985_ == 0)
{
lean_ctor_set(v___x_984_, 1, v_x_978_);
lean_ctor_set(v___x_984_, 0, v___y_987_);
v___x_989_ = v___x_984_;
goto v_reusejp_988_;
}
else
{
lean_object* v_reuseFailAlloc_991_; 
v_reuseFailAlloc_991_ = lean_alloc_ctor(1, 2, 0);
lean_ctor_set(v_reuseFailAlloc_991_, 0, v___y_987_);
lean_ctor_set(v_reuseFailAlloc_991_, 1, v_x_978_);
v___x_989_ = v_reuseFailAlloc_991_;
goto v_reusejp_988_;
}
v_reusejp_988_:
{
v_x_977_ = v_tail_982_;
v_x_978_ = v___x_989_;
goto _start;
}
}
}
}
}
}
LEAN_EXPORT lean_object* lp_zhc__lean_List_mapTR_loop___at___00Zhc_Iops_HillisSteele_finalResolution___at___00Zhc_Iops_addHillisSteele___at___00__private_Zhc_Iops_AddHillisSteele_Impl_0__Zhc_Iops_smokeHs_spec__0_spec__4_spec__11(lean_object* v_a_1072_, lean_object* v_a_1073_){
_start:
{
if (lean_obj_tag(v_a_1072_) == 0)
{
lean_object* v___x_1074_; 
v___x_1074_ = l_List_reverse___redArg(v_a_1073_);
return v___x_1074_;
}
else
{
lean_object* v_head_1075_; lean_object* v_tail_1076_; lean_object* v___x_1078_; uint8_t v_isShared_1079_; uint8_t v_isSharedCheck_1085_; 
v_head_1075_ = lean_ctor_get(v_a_1072_, 0);
v_tail_1076_ = lean_ctor_get(v_a_1072_, 1);
v_isSharedCheck_1085_ = !lean_is_exclusive(v_a_1072_);
if (v_isSharedCheck_1085_ == 0)
{
v___x_1078_ = v_a_1072_;
v_isShared_1079_ = v_isSharedCheck_1085_;
goto v_resetjp_1077_;
}
else
{
lean_inc(v_tail_1076_);
lean_inc(v_head_1075_);
lean_dec(v_a_1072_);
v___x_1078_ = lean_box(0);
v_isShared_1079_ = v_isSharedCheck_1085_;
goto v_resetjp_1077_;
}
v_resetjp_1077_:
{
lean_object* v___x_1080_; lean_object* v___x_1082_; 
v___x_1080_ = lean_alloc_ctor(1, 1, 0);
lean_ctor_set(v___x_1080_, 0, v_head_1075_);
if (v_isShared_1079_ == 0)
{
lean_ctor_set(v___x_1078_, 1, v_a_1073_);
lean_ctor_set(v___x_1078_, 0, v___x_1080_);
v___x_1082_ = v___x_1078_;
goto v_reusejp_1081_;
}
else
{
lean_object* v_reuseFailAlloc_1084_; 
v_reuseFailAlloc_1084_ = lean_alloc_ctor(1, 2, 0);
lean_ctor_set(v_reuseFailAlloc_1084_, 0, v___x_1080_);
lean_ctor_set(v_reuseFailAlloc_1084_, 1, v_a_1073_);
v___x_1082_ = v_reuseFailAlloc_1084_;
goto v_reusejp_1081_;
}
v_reusejp_1081_:
{
v_a_1072_ = v_tail_1076_;
v_a_1073_ = v___x_1082_;
goto _start;
}
}
}
}
}
LEAN_EXPORT lean_object* lp_zhc__lean_Zhc_Iops_HillisSteele_finalResolution___at___00Zhc_Iops_addHillisSteele___at___00__private_Zhc_Iops_AddHillisSteele_Impl_0__Zhc_Iops_smokeHs_spec__0_spec__4(lean_object* v_groups_1086_, lean_object* v_carries_1087_){
_start:
{
lean_object* v___x_1088_; lean_object* v___x_1089_; lean_object* v___x_1090_; lean_object* v_prevs_1091_; lean_object* v___x_1092_; lean_object* v___x_1093_; lean_object* v___x_1094_; lean_object* v___x_1095_; lean_object* v___x_1096_; 
v___x_1088_ = lean_box(0);
v___x_1089_ = lean_box(0);
lean_inc(v_carries_1087_);
v___x_1090_ = lp_zhc__lean_List_mapTR_loop___at___00Zhc_Iops_HillisSteele_finalResolution___at___00Zhc_Iops_addHillisSteele___at___00__private_Zhc_Iops_AddHillisSteele_Impl_0__Zhc_Iops_smokeHs_spec__0_spec__4_spec__11(v_carries_1087_, v___x_1089_);
v_prevs_1091_ = lean_alloc_ctor(1, 2, 0);
lean_ctor_set(v_prevs_1091_, 0, v___x_1088_);
lean_ctor_set(v_prevs_1091_, 1, v___x_1090_);
v___x_1092_ = lp_zhc__lean_Zhc_Iops_HillisSteele_chunk4___redArg(v_groups_1086_);
v___x_1093_ = l_List_zipWith___at___00List_zip_spec__0___redArg(v___x_1092_, v_carries_1087_);
v___x_1094_ = l_List_zipWith___at___00List_zip_spec__0___redArg(v___x_1093_, v_prevs_1091_);
v___x_1095_ = lp_zhc__lean_List_mapM_loop___at___00Zhc_Iops_HillisSteele_finalResolution___at___00Zhc_Iops_addHillisSteele___at___00__private_Zhc_Iops_AddHillisSteele_Impl_0__Zhc_Iops_smokeHs_spec__0_spec__4_spec__12(v___x_1094_, v___x_1089_);
v___x_1096_ = lp_zhc__lean_Zhc_Iops_HillisSteele_unchunk4___redArg(v___x_1095_);
return v___x_1096_;
}
}
LEAN_EXPORT lean_object* lp_zhc__lean_List_forIn_x27_loop___at___00Zhc_Iops_HillisSteele_joinBlocks___at___00Zhc_Iops_addHillisSteele___at___00__private_Zhc_Iops_AddHillisSteele_Impl_0__Zhc_Iops_smokeHs_spec__0_spec__6_spec__17___redArg(lean_object* v_as_x27_1097_, lean_object* v_b_1098_){
_start:
{
if (lean_obj_tag(v_as_x27_1097_) == 0)
{
return v_b_1098_;
}
else
{
lean_object* v_head_1099_; lean_object* v_tail_1100_; lean_object* v_fst_1101_; lean_object* v_snd_1102_; lean_object* v___x_1103_; 
v_head_1099_ = lean_ctor_get(v_as_x27_1097_, 0);
v_tail_1100_ = lean_ctor_get(v_as_x27_1097_, 1);
v_fst_1101_ = lean_ctor_get(v_head_1099_, 0);
v_snd_1102_ = lean_ctor_get(v_head_1099_, 1);
lean_inc(v_fst_1101_);
lean_inc(v_snd_1102_);
v___x_1103_ = lp_zhc__lean_Zhc_Model_CiphertextInteger_setBlock___redArg(v_b_1098_, v_snd_1102_, v_fst_1101_);
v_as_x27_1097_ = v_tail_1100_;
v_b_1098_ = v___x_1103_;
goto _start;
}
}
}
LEAN_EXPORT lean_object* lp_zhc__lean_List_forIn_x27_loop___at___00Zhc_Iops_HillisSteele_joinBlocks___at___00Zhc_Iops_addHillisSteele___at___00__private_Zhc_Iops_AddHillisSteele_Impl_0__Zhc_Iops_smokeHs_spec__0_spec__6_spec__17___redArg___boxed(lean_object* v_as_x27_1105_, lean_object* v_b_1106_){
_start:
{
lean_object* v_res_1107_; 
v_res_1107_ = lp_zhc__lean_List_forIn_x27_loop___at___00Zhc_Iops_HillisSteele_joinBlocks___at___00Zhc_Iops_addHillisSteele___at___00__private_Zhc_Iops_AddHillisSteele_Impl_0__Zhc_Iops_smokeHs_spec__0_spec__6_spec__17___redArg(v_as_x27_1105_, v_b_1106_);
lean_dec(v_as_x27_1105_);
return v_res_1107_;
}
}
LEAN_EXPORT lean_object* lp_zhc__lean_Zhc_Iops_HillisSteele_joinBlocks___at___00Zhc_Iops_addHillisSteele___at___00__private_Zhc_Iops_AddHillisSteele_Impl_0__Zhc_Iops_smokeHs_spec__0_spec__6(lean_object* v_si_1108_, lean_object* v_blocks_1109_){
_start:
{
lean_object* v___x_1110_; lean_object* v___x_1111_; lean_object* v___x_1112_; lean_object* v___x_1113_; lean_object* v___x_1114_; lean_object* v___x_1115_; 
v___x_1110_ = ((lean_object*)(lp_zhc__lean_Zhc_Iops_HillisSteele_extractPropGroupLut___closed__0));
v___x_1111_ = lean_unsigned_to_nat(0u);
v___x_1112_ = lp_zhc__lean_Zhc_Model_CiphertextBlock_digits(v___x_1110_, v_si_1108_, v___x_1111_);
v___x_1113_ = l_List_finRange(v_si_1108_);
v___x_1114_ = l_List_zipWith___at___00List_zip_spec__0___redArg(v_blocks_1109_, v___x_1113_);
v___x_1115_ = lp_zhc__lean_List_forIn_x27_loop___at___00Zhc_Iops_HillisSteele_joinBlocks___at___00Zhc_Iops_addHillisSteele___at___00__private_Zhc_Iops_AddHillisSteele_Impl_0__Zhc_Iops_smokeHs_spec__0_spec__6_spec__17___redArg(v___x_1114_, v___x_1112_);
lean_dec(v___x_1114_);
return v___x_1115_;
}
}
LEAN_EXPORT lean_object* lp_zhc__lean_List_mapTR_loop___at___00Zhc_Iops_HillisSteele_groupCarries___at___00Zhc_Iops_addHillisSteele___at___00__private_Zhc_Iops_AddHillisSteele_Impl_0__Zhc_Iops_smokeHs_spec__0_spec__3_spec__7(lean_object* v_a_1116_, lean_object* v_a_1117_){
_start:
{
if (lean_obj_tag(v_a_1116_) == 0)
{
lean_object* v___x_1118_; 
v___x_1118_ = l_List_reverse___redArg(v_a_1117_);
return v___x_1118_;
}
else
{
lean_object* v_head_1119_; lean_object* v_snd_1120_; lean_object* v_snd_1121_; lean_object* v_tail_1122_; lean_object* v___x_1124_; uint8_t v_isShared_1125_; uint8_t v_isSharedCheck_1131_; 
v_head_1119_ = lean_ctor_get(v_a_1116_, 0);
v_snd_1120_ = lean_ctor_get(v_head_1119_, 1);
v_snd_1121_ = lean_ctor_get(v_snd_1120_, 1);
lean_inc(v_snd_1121_);
v_tail_1122_ = lean_ctor_get(v_a_1116_, 1);
v_isSharedCheck_1131_ = !lean_is_exclusive(v_a_1116_);
if (v_isSharedCheck_1131_ == 0)
{
lean_object* v_unused_1132_; 
v_unused_1132_ = lean_ctor_get(v_a_1116_, 0);
lean_dec(v_unused_1132_);
v___x_1124_ = v_a_1116_;
v_isShared_1125_ = v_isSharedCheck_1131_;
goto v_resetjp_1123_;
}
else
{
lean_inc(v_tail_1122_);
lean_dec(v_a_1116_);
v___x_1124_ = lean_box(0);
v_isShared_1125_ = v_isSharedCheck_1131_;
goto v_resetjp_1123_;
}
v_resetjp_1123_:
{
lean_object* v_snd_1126_; lean_object* v___x_1128_; 
v_snd_1126_ = lean_ctor_get(v_snd_1121_, 1);
lean_inc(v_snd_1126_);
lean_dec(v_snd_1121_);
if (v_isShared_1125_ == 0)
{
lean_ctor_set(v___x_1124_, 1, v_a_1117_);
lean_ctor_set(v___x_1124_, 0, v_snd_1126_);
v___x_1128_ = v___x_1124_;
goto v_reusejp_1127_;
}
else
{
lean_object* v_reuseFailAlloc_1130_; 
v_reuseFailAlloc_1130_ = lean_alloc_ctor(1, 2, 0);
lean_ctor_set(v_reuseFailAlloc_1130_, 0, v_snd_1126_);
lean_ctor_set(v_reuseFailAlloc_1130_, 1, v_a_1117_);
v___x_1128_ = v_reuseFailAlloc_1130_;
goto v_reusejp_1127_;
}
v_reusejp_1127_:
{
v_a_1116_ = v_tail_1122_;
v_a_1117_ = v___x_1128_;
goto _start;
}
}
}
}
}
LEAN_EXPORT lean_object* lp_zhc__lean_List_mapM_loop___at___00Zhc_Iops_HillisSteele_hsStage___at___00Zhc_Iops_HillisSteele_groupCarries___at___00Zhc_Iops_addHillisSteele___at___00__private_Zhc_Iops_AddHillisSteele_Impl_0__Zhc_Iops_smokeHs_spec__0_spec__3_spec__8_spec__9(lean_object* v_stride_1133_, lean_object* v_x_1134_, lean_object* v_x_1135_){
_start:
{
lean_object* v___x_1136_; 
v___x_1136_ = ((lean_object*)(lp_zhc__lean_Zhc_Iops_HillisSteele_extractPropGroupLut___closed__0));
if (lean_obj_tag(v_x_1134_) == 0)
{
lean_object* v___x_1137_; 
v___x_1137_ = l_List_reverse___redArg(v_x_1135_);
return v___x_1137_;
}
else
{
lean_object* v_head_1138_; lean_object* v_tail_1139_; lean_object* v___x_1141_; uint8_t v_isShared_1142_; uint8_t v_isSharedCheck_1164_; 
v_head_1138_ = lean_ctor_get(v_x_1134_, 0);
v_tail_1139_ = lean_ctor_get(v_x_1134_, 1);
v_isSharedCheck_1164_ = !lean_is_exclusive(v_x_1134_);
if (v_isSharedCheck_1164_ == 0)
{
v___x_1141_ = v_x_1134_;
v_isShared_1142_ = v_isSharedCheck_1164_;
goto v_resetjp_1140_;
}
else
{
lean_inc(v_tail_1139_);
lean_inc(v_head_1138_);
lean_dec(v_x_1134_);
v___x_1141_ = lean_box(0);
v_isShared_1142_ = v_isSharedCheck_1164_;
goto v_resetjp_1140_;
}
v_resetjp_1140_:
{
lean_object* v___y_1144_; lean_object* v_fst_1149_; lean_object* v_snd_1150_; lean_object* v_fst_1151_; lean_object* v_snd_1152_; lean_object* v___x_1153_; lean_object* v___x_1154_; uint8_t v___x_1155_; 
v_fst_1149_ = lean_ctor_get(v_head_1138_, 0);
lean_inc(v_fst_1149_);
v_snd_1150_ = lean_ctor_get(v_head_1138_, 1);
lean_inc(v_snd_1150_);
lean_dec(v_head_1138_);
v_fst_1151_ = lean_ctor_get(v_fst_1149_, 0);
lean_inc(v_fst_1151_);
v_snd_1152_ = lean_ctor_get(v_fst_1149_, 1);
lean_inc(v_snd_1152_);
lean_dec(v_fst_1149_);
v___x_1153_ = lean_unsigned_to_nat(4u);
v___x_1154_ = lp_zhc__lean_Zhc_Model_Operators_mac(v___x_1136_, v_fst_1151_, v___x_1153_, v_snd_1152_);
lean_dec(v_snd_1152_);
lean_dec(v_fst_1151_);
v___x_1155_ = lean_nat_dec_lt(v_snd_1150_, v_stride_1133_);
lean_dec(v_snd_1150_);
if (v___x_1155_ == 0)
{
lean_object* v___x_1156_; lean_object* v___x_1157_; lean_object* v___x_1158_; lean_object* v___x_1159_; 
v___x_1156_ = lp_zhc__lean_Zhc_Iops_HillisSteele_solvePropLut;
v___x_1157_ = lean_unsigned_to_nat(0u);
v___x_1158_ = lean_obj_once(&lp_zhc__lean_List_mapM_loop___at___00Zhc_Iops_HillisSteele_finalResolution___at___00Zhc_Iops_addHillisSteele___at___00__private_Zhc_Iops_AddHillisSteele_Impl_0__Zhc_Iops_smokeHs_spec__0_spec__4_spec__12___closed__0, &lp_zhc__lean_List_mapM_loop___at___00Zhc_Iops_HillisSteele_finalResolution___at___00Zhc_Iops_addHillisSteele___at___00__private_Zhc_Iops_AddHillisSteele_Impl_0__Zhc_Iops_smokeHs_spec__0_spec__4_spec__12___closed__0_once, _init_lp_zhc__lean_List_mapM_loop___at___00Zhc_Iops_HillisSteele_finalResolution___at___00Zhc_Iops_addHillisSteele___at___00__private_Zhc_Iops_AddHillisSteele_Impl_0__Zhc_Iops_smokeHs_spec__0_spec__4_spec__12___closed__0);
v___x_1159_ = lp_zhc__lean_Zhc_Model_ManyLut_output(v___x_1136_, v___x_1157_, v___x_1156_, v___x_1154_, v___x_1158_);
lean_dec(v___x_1154_);
v___y_1144_ = v___x_1159_;
goto v___jp_1143_;
}
else
{
lean_object* v___x_1160_; lean_object* v___x_1161_; lean_object* v___x_1162_; lean_object* v___x_1163_; 
v___x_1160_ = lp_zhc__lean_Zhc_Iops_HillisSteele_solvePropCarryLut;
v___x_1161_ = lean_unsigned_to_nat(0u);
v___x_1162_ = lean_obj_once(&lp_zhc__lean_List_mapM_loop___at___00Zhc_Iops_HillisSteele_finalResolution___at___00Zhc_Iops_addHillisSteele___at___00__private_Zhc_Iops_AddHillisSteele_Impl_0__Zhc_Iops_smokeHs_spec__0_spec__4_spec__12___closed__0, &lp_zhc__lean_List_mapM_loop___at___00Zhc_Iops_HillisSteele_finalResolution___at___00Zhc_Iops_addHillisSteele___at___00__private_Zhc_Iops_AddHillisSteele_Impl_0__Zhc_Iops_smokeHs_spec__0_spec__4_spec__12___closed__0_once, _init_lp_zhc__lean_List_mapM_loop___at___00Zhc_Iops_HillisSteele_finalResolution___at___00Zhc_Iops_addHillisSteele___at___00__private_Zhc_Iops_AddHillisSteele_Impl_0__Zhc_Iops_smokeHs_spec__0_spec__4_spec__12___closed__0);
v___x_1163_ = lp_zhc__lean_Zhc_Model_ManyLut_output(v___x_1136_, v___x_1161_, v___x_1160_, v___x_1154_, v___x_1162_);
lean_dec(v___x_1154_);
v___y_1144_ = v___x_1163_;
goto v___jp_1143_;
}
v___jp_1143_:
{
lean_object* v___x_1146_; 
if (v_isShared_1142_ == 0)
{
lean_ctor_set(v___x_1141_, 1, v_x_1135_);
lean_ctor_set(v___x_1141_, 0, v___y_1144_);
v___x_1146_ = v___x_1141_;
goto v_reusejp_1145_;
}
else
{
lean_object* v_reuseFailAlloc_1148_; 
v_reuseFailAlloc_1148_ = lean_alloc_ctor(1, 2, 0);
lean_ctor_set(v_reuseFailAlloc_1148_, 0, v___y_1144_);
lean_ctor_set(v_reuseFailAlloc_1148_, 1, v_x_1135_);
v___x_1146_ = v_reuseFailAlloc_1148_;
goto v_reusejp_1145_;
}
v_reusejp_1145_:
{
v_x_1134_ = v_tail_1139_;
v_x_1135_ = v___x_1146_;
goto _start;
}
}
}
}
}
}
LEAN_EXPORT lean_object* lp_zhc__lean_List_mapM_loop___at___00Zhc_Iops_HillisSteele_hsStage___at___00Zhc_Iops_HillisSteele_groupCarries___at___00Zhc_Iops_addHillisSteele___at___00__private_Zhc_Iops_AddHillisSteele_Impl_0__Zhc_Iops_smokeHs_spec__0_spec__3_spec__8_spec__9___boxed(lean_object* v_stride_1165_, lean_object* v_x_1166_, lean_object* v_x_1167_){
_start:
{
lean_object* v_res_1168_; 
v_res_1168_ = lp_zhc__lean_List_mapM_loop___at___00Zhc_Iops_HillisSteele_hsStage___at___00Zhc_Iops_HillisSteele_groupCarries___at___00Zhc_Iops_addHillisSteele___at___00__private_Zhc_Iops_AddHillisSteele_Impl_0__Zhc_Iops_smokeHs_spec__0_spec__3_spec__8_spec__9(v_stride_1165_, v_x_1166_, v_x_1167_);
lean_dec(v_stride_1165_);
return v_res_1168_;
}
}
LEAN_EXPORT lean_object* lp_zhc__lean_Zhc_Iops_HillisSteele_hsStage___at___00Zhc_Iops_HillisSteele_groupCarries___at___00Zhc_Iops_addHillisSteele___at___00__private_Zhc_Iops_AddHillisSteele_Impl_0__Zhc_Iops_smokeHs_spec__0_spec__3_spec__8(lean_object* v_stage_1169_, lean_object* v_carries_1170_){
_start:
{
lean_object* v___x_1171_; lean_object* v_stride_1172_; lean_object* v___x_1173_; lean_object* v___x_1174_; lean_object* v___x_1175_; lean_object* v___x_1176_; lean_object* v___x_1177_; lean_object* v___x_1178_; lean_object* v___x_1179_; lean_object* v___x_1180_; lean_object* v___x_1181_; 
v___x_1171_ = lean_unsigned_to_nat(2u);
v_stride_1172_ = lean_nat_pow(v___x_1171_, v_stage_1169_);
lean_inc(v_stride_1172_);
v___x_1173_ = l_List_drop___redArg(v_stride_1172_, v_carries_1170_);
lean_inc_n(v_carries_1170_, 2);
v___x_1174_ = l_List_zipWith___at___00List_zip_spec__0___redArg(v___x_1173_, v_carries_1170_);
v___x_1175_ = lean_unsigned_to_nat(0u);
v___x_1176_ = l_List_zipIdxTR___redArg(v___x_1174_, v___x_1175_);
v___x_1177_ = lean_box(0);
v___x_1178_ = lp_zhc__lean_List_mapM_loop___at___00Zhc_Iops_HillisSteele_hsStage___at___00Zhc_Iops_HillisSteele_groupCarries___at___00Zhc_Iops_addHillisSteele___at___00__private_Zhc_Iops_AddHillisSteele_Impl_0__Zhc_Iops_smokeHs_spec__0_spec__3_spec__8_spec__9(v_stride_1172_, v___x_1176_, v___x_1177_);
v___x_1179_ = ((lean_object*)(lp_zhc__lean_Zhc_Iops_HillisSteele_hsStage___redArg___lam__2___closed__0));
v___x_1180_ = l___private_Init_Data_List_Impl_0__List_takeTR_go___redArg(v_carries_1170_, v_carries_1170_, v_stride_1172_, v___x_1179_);
lean_dec(v_carries_1170_);
v___x_1181_ = l_List_appendTR___redArg(v___x_1180_, v___x_1178_);
return v___x_1181_;
}
}
LEAN_EXPORT lean_object* lp_zhc__lean_Zhc_Iops_HillisSteele_hsStage___at___00Zhc_Iops_HillisSteele_groupCarries___at___00Zhc_Iops_addHillisSteele___at___00__private_Zhc_Iops_AddHillisSteele_Impl_0__Zhc_Iops_smokeHs_spec__0_spec__3_spec__8___boxed(lean_object* v_stage_1182_, lean_object* v_carries_1183_){
_start:
{
lean_object* v_res_1184_; 
v_res_1184_ = lp_zhc__lean_Zhc_Iops_HillisSteele_hsStage___at___00Zhc_Iops_HillisSteele_groupCarries___at___00Zhc_Iops_addHillisSteele___at___00__private_Zhc_Iops_AddHillisSteele_Impl_0__Zhc_Iops_smokeHs_spec__0_spec__3_spec__8(v_stage_1182_, v_carries_1183_);
lean_dec(v_stage_1182_);
return v_res_1184_;
}
}
LEAN_EXPORT lean_object* lp_zhc__lean_List_forIn_x27_loop___at___00Zhc_Iops_HillisSteele_groupCarries___at___00Zhc_Iops_addHillisSteele___at___00__private_Zhc_Iops_AddHillisSteele_Impl_0__Zhc_Iops_smokeHs_spec__0_spec__3_spec__9___redArg(lean_object* v_as_x27_1185_, lean_object* v_b_1186_){
_start:
{
if (lean_obj_tag(v_as_x27_1185_) == 0)
{
return v_b_1186_;
}
else
{
lean_object* v_head_1187_; lean_object* v_tail_1188_; lean_object* v___x_1189_; 
v_head_1187_ = lean_ctor_get(v_as_x27_1185_, 0);
v_tail_1188_ = lean_ctor_get(v_as_x27_1185_, 1);
v___x_1189_ = lp_zhc__lean_Zhc_Iops_HillisSteele_hsStage___at___00Zhc_Iops_HillisSteele_groupCarries___at___00Zhc_Iops_addHillisSteele___at___00__private_Zhc_Iops_AddHillisSteele_Impl_0__Zhc_Iops_smokeHs_spec__0_spec__3_spec__8(v_head_1187_, v_b_1186_);
v_as_x27_1185_ = v_tail_1188_;
v_b_1186_ = v___x_1189_;
goto _start;
}
}
}
LEAN_EXPORT lean_object* lp_zhc__lean_List_forIn_x27_loop___at___00Zhc_Iops_HillisSteele_groupCarries___at___00Zhc_Iops_addHillisSteele___at___00__private_Zhc_Iops_AddHillisSteele_Impl_0__Zhc_Iops_smokeHs_spec__0_spec__3_spec__9___redArg___boxed(lean_object* v_as_x27_1191_, lean_object* v_b_1192_){
_start:
{
lean_object* v_res_1193_; 
v_res_1193_ = lp_zhc__lean_List_forIn_x27_loop___at___00Zhc_Iops_HillisSteele_groupCarries___at___00Zhc_Iops_addHillisSteele___at___00__private_Zhc_Iops_AddHillisSteele_Impl_0__Zhc_Iops_smokeHs_spec__0_spec__3_spec__9___redArg(v_as_x27_1191_, v_b_1192_);
lean_dec(v_as_x27_1191_);
return v_res_1193_;
}
}
LEAN_EXPORT lean_object* lp_zhc__lean_Zhc_Iops_HillisSteele_groupCarries___at___00Zhc_Iops_addHillisSteele___at___00__private_Zhc_Iops_AddHillisSteele_Impl_0__Zhc_Iops_smokeHs_spec__0_spec__3(lean_object* v_groups_1194_){
_start:
{
lean_object* v___x_1195_; lean_object* v___x_1196_; lean_object* v_carries_1197_; lean_object* v___x_1198_; lean_object* v___x_1199_; lean_object* v___x_1200_; lean_object* v___x_1201_; 
v___x_1195_ = lp_zhc__lean_Zhc_Iops_HillisSteele_chunk4___redArg(v_groups_1194_);
v___x_1196_ = lean_box(0);
v_carries_1197_ = lp_zhc__lean_List_mapTR_loop___at___00Zhc_Iops_HillisSteele_groupCarries___at___00Zhc_Iops_addHillisSteele___at___00__private_Zhc_Iops_AddHillisSteele_Impl_0__Zhc_Iops_smokeHs_spec__0_spec__3_spec__7(v___x_1195_, v___x_1196_);
v___x_1198_ = l_List_lengthTR___redArg(v_carries_1197_);
v___x_1199_ = lean_nat_log2(v___x_1198_);
lean_dec(v___x_1198_);
v___x_1200_ = l_List_range(v___x_1199_);
v___x_1201_ = lp_zhc__lean_List_forIn_x27_loop___at___00Zhc_Iops_HillisSteele_groupCarries___at___00Zhc_Iops_addHillisSteele___at___00__private_Zhc_Iops_AddHillisSteele_Impl_0__Zhc_Iops_smokeHs_spec__0_spec__3_spec__9___redArg(v___x_1200_, v_carries_1197_);
lean_dec(v___x_1200_);
return v___x_1201_;
}
}
LEAN_EXPORT lean_object* lp_zhc__lean_List_mapM_loop___at___00Zhc_Iops_HillisSteele_propagate___at___00Zhc_Iops_addHillisSteele___at___00__private_Zhc_Iops_AddHillisSteele_Impl_0__Zhc_Iops_smokeHs_spec__0_spec__5_spec__14(lean_object* v_x_1202_, lean_object* v_x_1203_){
_start:
{
lean_object* v___x_1204_; 
v___x_1204_ = ((lean_object*)(lp_zhc__lean_Zhc_Iops_HillisSteele_extractPropGroupLut___closed__0));
if (lean_obj_tag(v_x_1202_) == 0)
{
lean_object* v___x_1205_; 
v___x_1205_ = l_List_reverse___redArg(v_x_1203_);
return v___x_1205_;
}
else
{
lean_object* v_head_1206_; lean_object* v_tail_1207_; lean_object* v___x_1209_; uint8_t v_isShared_1210_; uint8_t v_isSharedCheck_1218_; 
v_head_1206_ = lean_ctor_get(v_x_1202_, 0);
v_tail_1207_ = lean_ctor_get(v_x_1202_, 1);
v_isSharedCheck_1218_ = !lean_is_exclusive(v_x_1202_);
if (v_isSharedCheck_1218_ == 0)
{
v___x_1209_ = v_x_1202_;
v_isShared_1210_ = v_isSharedCheck_1218_;
goto v_resetjp_1208_;
}
else
{
lean_inc(v_tail_1207_);
lean_inc(v_head_1206_);
lean_dec(v_x_1202_);
v___x_1209_ = lean_box(0);
v_isShared_1210_ = v_isSharedCheck_1218_;
goto v_resetjp_1208_;
}
v_resetjp_1208_:
{
lean_object* v_fst_1211_; lean_object* v_snd_1212_; lean_object* v___x_1213_; lean_object* v___x_1215_; 
v_fst_1211_ = lean_ctor_get(v_head_1206_, 0);
lean_inc(v_fst_1211_);
v_snd_1212_ = lean_ctor_get(v_head_1206_, 1);
lean_inc(v_snd_1212_);
lean_dec(v_head_1206_);
v___x_1213_ = lp_zhc__lean_Zhc_Model_Operators_add(v___x_1204_, v_fst_1211_, v_snd_1212_);
lean_dec(v_snd_1212_);
lean_dec(v_fst_1211_);
if (v_isShared_1210_ == 0)
{
lean_ctor_set(v___x_1209_, 1, v_x_1203_);
lean_ctor_set(v___x_1209_, 0, v___x_1213_);
v___x_1215_ = v___x_1209_;
goto v_reusejp_1214_;
}
else
{
lean_object* v_reuseFailAlloc_1217_; 
v_reuseFailAlloc_1217_ = lean_alloc_ctor(1, 2, 0);
lean_ctor_set(v_reuseFailAlloc_1217_, 0, v___x_1213_);
lean_ctor_set(v_reuseFailAlloc_1217_, 1, v_x_1203_);
v___x_1215_ = v_reuseFailAlloc_1217_;
goto v_reusejp_1214_;
}
v_reusejp_1214_:
{
v_x_1202_ = v_tail_1207_;
v_x_1203_ = v___x_1215_;
goto _start;
}
}
}
}
}
LEAN_EXPORT lean_object* lp_zhc__lean_List_mapM_loop___at___00Zhc_Iops_HillisSteele_propagate___at___00Zhc_Iops_addHillisSteele___at___00__private_Zhc_Iops_AddHillisSteele_Impl_0__Zhc_Iops_smokeHs_spec__0_spec__5_spec__15(lean_object* v_x_1219_, lean_object* v_x_1220_){
_start:
{
lean_object* v___x_1221_; 
v___x_1221_ = ((lean_object*)(lp_zhc__lean_Zhc_Iops_HillisSteele_extractPropGroupLut___closed__0));
if (lean_obj_tag(v_x_1219_) == 0)
{
lean_object* v___x_1222_; 
v___x_1222_ = l_List_reverse___redArg(v_x_1220_);
return v___x_1222_;
}
else
{
lean_object* v_head_1223_; lean_object* v_tail_1224_; lean_object* v___x_1226_; uint8_t v_isShared_1227_; uint8_t v_isSharedCheck_1236_; 
v_head_1223_ = lean_ctor_get(v_x_1219_, 0);
v_tail_1224_ = lean_ctor_get(v_x_1219_, 1);
v_isSharedCheck_1236_ = !lean_is_exclusive(v_x_1219_);
if (v_isSharedCheck_1236_ == 0)
{
v___x_1226_ = v_x_1219_;
v_isShared_1227_ = v_isSharedCheck_1236_;
goto v_resetjp_1225_;
}
else
{
lean_inc(v_tail_1224_);
lean_inc(v_head_1223_);
lean_dec(v_x_1219_);
v___x_1226_ = lean_box(0);
v_isShared_1227_ = v_isSharedCheck_1236_;
goto v_resetjp_1225_;
}
v_resetjp_1225_:
{
lean_object* v___x_1228_; lean_object* v___x_1229_; lean_object* v___x_1230_; lean_object* v___x_1231_; lean_object* v___x_1233_; 
v___x_1228_ = lp_zhc__lean_Zhc_Iops_HillisSteele_msgOnlyLut;
v___x_1229_ = lean_unsigned_to_nat(0u);
v___x_1230_ = lean_obj_once(&lp_zhc__lean_List_mapM_loop___at___00Zhc_Iops_HillisSteele_finalResolution___at___00Zhc_Iops_addHillisSteele___at___00__private_Zhc_Iops_AddHillisSteele_Impl_0__Zhc_Iops_smokeHs_spec__0_spec__4_spec__12___closed__0, &lp_zhc__lean_List_mapM_loop___at___00Zhc_Iops_HillisSteele_finalResolution___at___00Zhc_Iops_addHillisSteele___at___00__private_Zhc_Iops_AddHillisSteele_Impl_0__Zhc_Iops_smokeHs_spec__0_spec__4_spec__12___closed__0_once, _init_lp_zhc__lean_List_mapM_loop___at___00Zhc_Iops_HillisSteele_finalResolution___at___00Zhc_Iops_addHillisSteele___at___00__private_Zhc_Iops_AddHillisSteele_Impl_0__Zhc_Iops_smokeHs_spec__0_spec__4_spec__12___closed__0);
v___x_1231_ = lp_zhc__lean_Zhc_Model_ManyLut_output(v___x_1221_, v___x_1229_, v___x_1228_, v_head_1223_, v___x_1230_);
lean_dec(v_head_1223_);
if (v_isShared_1227_ == 0)
{
lean_ctor_set(v___x_1226_, 1, v_x_1220_);
lean_ctor_set(v___x_1226_, 0, v___x_1231_);
v___x_1233_ = v___x_1226_;
goto v_reusejp_1232_;
}
else
{
lean_object* v_reuseFailAlloc_1235_; 
v_reuseFailAlloc_1235_ = lean_alloc_ctor(1, 2, 0);
lean_ctor_set(v_reuseFailAlloc_1235_, 0, v___x_1231_);
lean_ctor_set(v_reuseFailAlloc_1235_, 1, v_x_1220_);
v___x_1233_ = v_reuseFailAlloc_1235_;
goto v_reusejp_1232_;
}
v_reusejp_1232_:
{
v_x_1219_ = v_tail_1224_;
v_x_1220_ = v___x_1233_;
goto _start;
}
}
}
}
}
static lean_object* _init_lp_zhc__lean_Zhc_Iops_HillisSteele_propagate___at___00Zhc_Iops_addHillisSteele___at___00__private_Zhc_Iops_AddHillisSteele_Impl_0__Zhc_Iops_smokeHs_spec__0_spec__5___closed__0(void){
_start:
{
lean_object* v___x_1237_; lean_object* v___x_1238_; lean_object* v___x_1239_; 
v___x_1237_ = lean_unsigned_to_nat(2u);
v___x_1238_ = lean_unsigned_to_nat(1u);
v___x_1239_ = lean_nat_mod(v___x_1238_, v___x_1237_);
return v___x_1239_;
}
}
LEAN_EXPORT lean_object* lp_zhc__lean_Zhc_Iops_HillisSteele_propagate___at___00Zhc_Iops_addHillisSteele___at___00__private_Zhc_Iops_AddHillisSteele_Impl_0__Zhc_Iops_smokeHs_spec__0_spec__5(lean_object* v_sums_1240_, lean_object* v_carries_1241_){
_start:
{
lean_object* v___x_1242_; 
v___x_1242_ = ((lean_object*)(lp_zhc__lean_Zhc_Iops_HillisSteele_extractPropGroupLut___closed__0));
if (lean_obj_tag(v_sums_1240_) == 0)
{
lean_dec(v_carries_1241_);
return v_sums_1240_;
}
else
{
lean_object* v_head_1243_; lean_object* v_tail_1244_; lean_object* v___x_1246_; uint8_t v_isShared_1247_; uint8_t v_isSharedCheck_1259_; 
v_head_1243_ = lean_ctor_get(v_sums_1240_, 0);
v_tail_1244_ = lean_ctor_get(v_sums_1240_, 1);
v_isSharedCheck_1259_ = !lean_is_exclusive(v_sums_1240_);
if (v_isSharedCheck_1259_ == 0)
{
v___x_1246_ = v_sums_1240_;
v_isShared_1247_ = v_isSharedCheck_1259_;
goto v_resetjp_1245_;
}
else
{
lean_inc(v_tail_1244_);
lean_inc(v_head_1243_);
lean_dec(v_sums_1240_);
v___x_1246_ = lean_box(0);
v_isShared_1247_ = v_isSharedCheck_1259_;
goto v_resetjp_1245_;
}
v_resetjp_1245_:
{
lean_object* v___x_1248_; lean_object* v___x_1249_; lean_object* v___x_1250_; lean_object* v_m0_1251_; lean_object* v___x_1252_; lean_object* v___x_1253_; lean_object* v___x_1254_; lean_object* v___x_1256_; 
v___x_1248_ = lp_zhc__lean_Zhc_Iops_carryMsgLut;
v___x_1249_ = lean_unsigned_to_nat(1u);
v___x_1250_ = lean_obj_once(&lp_zhc__lean_Zhc_Iops_HillisSteele_propagate___at___00Zhc_Iops_addHillisSteele___at___00__private_Zhc_Iops_AddHillisSteele_Impl_0__Zhc_Iops_smokeHs_spec__0_spec__5___closed__0, &lp_zhc__lean_Zhc_Iops_HillisSteele_propagate___at___00Zhc_Iops_addHillisSteele___at___00__private_Zhc_Iops_AddHillisSteele_Impl_0__Zhc_Iops_smokeHs_spec__0_spec__5___closed__0_once, _init_lp_zhc__lean_Zhc_Iops_HillisSteele_propagate___at___00Zhc_Iops_addHillisSteele___at___00__private_Zhc_Iops_AddHillisSteele_Impl_0__Zhc_Iops_smokeHs_spec__0_spec__5___closed__0);
v_m0_1251_ = lp_zhc__lean_Zhc_Model_ManyLut_output(v___x_1242_, v___x_1249_, v___x_1248_, v_head_1243_, v___x_1250_);
lean_dec(v_head_1243_);
v___x_1252_ = l_List_zipWith___at___00List_zip_spec__0___redArg(v_tail_1244_, v_carries_1241_);
v___x_1253_ = lean_box(0);
v___x_1254_ = lp_zhc__lean_List_mapM_loop___at___00Zhc_Iops_HillisSteele_propagate___at___00Zhc_Iops_addHillisSteele___at___00__private_Zhc_Iops_AddHillisSteele_Impl_0__Zhc_Iops_smokeHs_spec__0_spec__5_spec__14(v___x_1252_, v___x_1253_);
if (v_isShared_1247_ == 0)
{
lean_ctor_set(v___x_1246_, 1, v___x_1254_);
lean_ctor_set(v___x_1246_, 0, v_m0_1251_);
v___x_1256_ = v___x_1246_;
goto v_reusejp_1255_;
}
else
{
lean_object* v_reuseFailAlloc_1258_; 
v_reuseFailAlloc_1258_ = lean_alloc_ctor(1, 2, 0);
lean_ctor_set(v_reuseFailAlloc_1258_, 0, v_m0_1251_);
lean_ctor_set(v_reuseFailAlloc_1258_, 1, v___x_1254_);
v___x_1256_ = v_reuseFailAlloc_1258_;
goto v_reusejp_1255_;
}
v_reusejp_1255_:
{
lean_object* v___x_1257_; 
v___x_1257_ = lp_zhc__lean_List_mapM_loop___at___00Zhc_Iops_HillisSteele_propagate___at___00Zhc_Iops_addHillisSteele___at___00__private_Zhc_Iops_AddHillisSteele_Impl_0__Zhc_Iops_smokeHs_spec__0_spec__5_spec__15(v___x_1256_, v___x_1253_);
return v___x_1257_;
}
}
}
}
}
LEAN_EXPORT lean_object* lp_zhc__lean_List_mapM_loop___at___00Zhc_Iops_HillisSteele_groupStates___at___00Zhc_Iops_addHillisSteele___at___00__private_Zhc_Iops_AddHillisSteele_Impl_0__Zhc_Iops_smokeHs_spec__0_spec__2_spec__5___lam__0(lean_object* v___x_1260_, lean_object* v___x_1261_, lean_object* v_fst_1262_, lean_object* v_b3_1263_){
_start:
{
lean_object* v___x_1264_; lean_object* v___x_1265_; lean_object* v___x_1266_; 
v___x_1264_ = lean_alloc_ctor(0, 2, 0);
lean_ctor_set(v___x_1264_, 0, v___x_1260_);
lean_ctor_set(v___x_1264_, 1, v_b3_1263_);
v___x_1265_ = lean_alloc_ctor(0, 2, 0);
lean_ctor_set(v___x_1265_, 0, v___x_1261_);
lean_ctor_set(v___x_1265_, 1, v___x_1264_);
v___x_1266_ = lean_alloc_ctor(0, 2, 0);
lean_ctor_set(v___x_1266_, 0, v_fst_1262_);
lean_ctor_set(v___x_1266_, 1, v___x_1265_);
return v___x_1266_;
}
}
static lean_object* _init_lp_zhc__lean_List_mapM_loop___at___00Zhc_Iops_HillisSteele_groupStates___at___00Zhc_Iops_addHillisSteele___at___00__private_Zhc_Iops_AddHillisSteele_Impl_0__Zhc_Iops_smokeHs_spec__0_spec__2_spec__5___closed__0(void){
_start:
{
lean_object* v___x_1267_; lean_object* v___x_1268_; lean_object* v___x_1269_; 
v___x_1267_ = lean_unsigned_to_nat(1u);
v___x_1268_ = ((lean_object*)(lp_zhc__lean_Zhc_Iops_HillisSteele_extractPropGroupLut___closed__0));
v___x_1269_ = lp_zhc__lean_Zhc_Model_PlaintextBlock_ofNat(v___x_1268_, v___x_1267_);
return v___x_1269_;
}
}
LEAN_EXPORT lean_object* lp_zhc__lean_List_mapM_loop___at___00Zhc_Iops_HillisSteele_groupStates___at___00Zhc_Iops_addHillisSteele___at___00__private_Zhc_Iops_AddHillisSteele_Impl_0__Zhc_Iops_smokeHs_spec__0_spec__2_spec__5(lean_object* v_x_1270_, lean_object* v_x_1271_){
_start:
{
lean_object* v___x_1272_; 
v___x_1272_ = ((lean_object*)(lp_zhc__lean_Zhc_Iops_HillisSteele_extractPropGroupLut___closed__0));
if (lean_obj_tag(v_x_1270_) == 0)
{
lean_object* v___x_1273_; 
v___x_1273_ = l_List_reverse___redArg(v_x_1271_);
return v___x_1273_;
}
else
{
lean_object* v_head_1274_; lean_object* v_tail_1275_; lean_object* v___x_1277_; uint8_t v_isShared_1278_; uint8_t v_isSharedCheck_1308_; 
v_head_1274_ = lean_ctor_get(v_x_1270_, 0);
v_tail_1275_ = lean_ctor_get(v_x_1270_, 1);
v_isSharedCheck_1308_ = !lean_is_exclusive(v_x_1270_);
if (v_isSharedCheck_1308_ == 0)
{
v___x_1277_ = v_x_1270_;
v_isShared_1278_ = v_isSharedCheck_1308_;
goto v_resetjp_1276_;
}
else
{
lean_inc(v_tail_1275_);
lean_inc(v_head_1274_);
lean_dec(v_x_1270_);
v___x_1277_ = lean_box(0);
v_isShared_1278_ = v_isSharedCheck_1308_;
goto v_resetjp_1276_;
}
v_resetjp_1276_:
{
lean_object* v___y_1280_; lean_object* v_fst_1285_; lean_object* v_snd_1286_; lean_object* v_snd_1287_; lean_object* v_snd_1288_; lean_object* v_fst_1289_; lean_object* v_fst_1290_; lean_object* v_fst_1291_; lean_object* v_snd_1292_; lean_object* v___x_1293_; lean_object* v___x_1294_; lean_object* v___x_1295_; lean_object* v___x_1296_; uint8_t v___x_1297_; 
v_fst_1285_ = lean_ctor_get(v_head_1274_, 0);
lean_inc(v_fst_1285_);
v_snd_1286_ = lean_ctor_get(v_fst_1285_, 1);
lean_inc(v_snd_1286_);
v_snd_1287_ = lean_ctor_get(v_snd_1286_, 1);
lean_inc(v_snd_1287_);
v_snd_1288_ = lean_ctor_get(v_head_1274_, 1);
lean_inc(v_snd_1288_);
lean_dec(v_head_1274_);
v_fst_1289_ = lean_ctor_get(v_fst_1285_, 0);
lean_inc(v_fst_1289_);
lean_dec(v_fst_1285_);
v_fst_1290_ = lean_ctor_get(v_snd_1286_, 0);
lean_inc(v_fst_1290_);
lean_dec(v_snd_1286_);
v_fst_1291_ = lean_ctor_get(v_snd_1287_, 0);
lean_inc(v_fst_1291_);
v_snd_1292_ = lean_ctor_get(v_snd_1287_, 1);
lean_inc(v_snd_1292_);
lean_dec(v_snd_1287_);
v___x_1293_ = lp_zhc__lean_Zhc_Model_Operators_add(v___x_1272_, v_fst_1289_, v_fst_1290_);
lean_dec(v_fst_1290_);
v___x_1294_ = lp_zhc__lean_Zhc_Model_Operators_add(v___x_1272_, v___x_1293_, v_fst_1291_);
lean_dec(v_fst_1291_);
v___x_1295_ = lp_zhc__lean_Zhc_Model_Operators_add(v___x_1272_, v___x_1294_, v_snd_1292_);
lean_dec(v_snd_1292_);
v___x_1296_ = lean_unsigned_to_nat(0u);
v___x_1297_ = lean_nat_dec_eq(v_snd_1288_, v___x_1296_);
lean_dec(v_snd_1288_);
if (v___x_1297_ == 0)
{
lean_object* v___x_1298_; lean_object* v___x_1299_; lean_object* v___x_1300_; lean_object* v___x_1301_; lean_object* v___x_1302_; lean_object* v___x_1303_; 
v___x_1298_ = lp_zhc__lean_Zhc_Iops_HillisSteele_reduceCarryPadLut;
v___x_1299_ = lean_obj_once(&lp_zhc__lean_List_mapM_loop___at___00Zhc_Iops_HillisSteele_finalResolution___at___00Zhc_Iops_addHillisSteele___at___00__private_Zhc_Iops_AddHillisSteele_Impl_0__Zhc_Iops_smokeHs_spec__0_spec__4_spec__12___closed__0, &lp_zhc__lean_List_mapM_loop___at___00Zhc_Iops_HillisSteele_finalResolution___at___00Zhc_Iops_addHillisSteele___at___00__private_Zhc_Iops_AddHillisSteele_Impl_0__Zhc_Iops_smokeHs_spec__0_spec__4_spec__12___closed__0_once, _init_lp_zhc__lean_List_mapM_loop___at___00Zhc_Iops_HillisSteele_finalResolution___at___00Zhc_Iops_addHillisSteele___at___00__private_Zhc_Iops_AddHillisSteele_Impl_0__Zhc_Iops_smokeHs_spec__0_spec__4_spec__12___closed__0);
v___x_1300_ = lp_zhc__lean_Zhc_Model_ManyLut_output(v___x_1272_, v___x_1296_, v___x_1298_, v___x_1295_, v___x_1299_);
lean_dec(v___x_1295_);
v___x_1301_ = lean_obj_once(&lp_zhc__lean_List_mapM_loop___at___00Zhc_Iops_HillisSteele_groupStates___at___00Zhc_Iops_addHillisSteele___at___00__private_Zhc_Iops_AddHillisSteele_Impl_0__Zhc_Iops_smokeHs_spec__0_spec__2_spec__5___closed__0, &lp_zhc__lean_List_mapM_loop___at___00Zhc_Iops_HillisSteele_groupStates___at___00Zhc_Iops_addHillisSteele___at___00__private_Zhc_Iops_AddHillisSteele_Impl_0__Zhc_Iops_smokeHs_spec__0_spec__2_spec__5___closed__0_once, _init_lp_zhc__lean_List_mapM_loop___at___00Zhc_Iops_HillisSteele_groupStates___at___00Zhc_Iops_addHillisSteele___at___00__private_Zhc_Iops_AddHillisSteele_Impl_0__Zhc_Iops_smokeHs_spec__0_spec__2_spec__5___closed__0);
v___x_1302_ = lp_zhc__lean_Zhc_Model_Operators_addPt(v___x_1272_, v___x_1300_, v___x_1301_);
lean_dec(v___x_1300_);
v___x_1303_ = lp_zhc__lean_List_mapM_loop___at___00Zhc_Iops_HillisSteele_groupStates___at___00Zhc_Iops_addHillisSteele___at___00__private_Zhc_Iops_AddHillisSteele_Impl_0__Zhc_Iops_smokeHs_spec__0_spec__2_spec__5___lam__0(v___x_1294_, v___x_1293_, v_fst_1289_, v___x_1302_);
v___y_1280_ = v___x_1303_;
goto v___jp_1279_;
}
else
{
lean_object* v___x_1304_; lean_object* v___x_1305_; lean_object* v___x_1306_; lean_object* v___x_1307_; 
v___x_1304_ = lean_obj_once(&lp_zhc__lean_Zhc_Iops_HillisSteele_groupStates___redArg___lam__5___closed__1, &lp_zhc__lean_Zhc_Iops_HillisSteele_groupStates___redArg___lam__5___closed__1_once, _init_lp_zhc__lean_Zhc_Iops_HillisSteele_groupStates___redArg___lam__5___closed__1);
v___x_1305_ = lean_obj_once(&lp_zhc__lean_List_mapM_loop___at___00Zhc_Iops_HillisSteele_finalResolution___at___00Zhc_Iops_addHillisSteele___at___00__private_Zhc_Iops_AddHillisSteele_Impl_0__Zhc_Iops_smokeHs_spec__0_spec__4_spec__12___closed__0, &lp_zhc__lean_List_mapM_loop___at___00Zhc_Iops_HillisSteele_finalResolution___at___00Zhc_Iops_addHillisSteele___at___00__private_Zhc_Iops_AddHillisSteele_Impl_0__Zhc_Iops_smokeHs_spec__0_spec__4_spec__12___closed__0_once, _init_lp_zhc__lean_List_mapM_loop___at___00Zhc_Iops_HillisSteele_finalResolution___at___00Zhc_Iops_addHillisSteele___at___00__private_Zhc_Iops_AddHillisSteele_Impl_0__Zhc_Iops_smokeHs_spec__0_spec__4_spec__12___closed__0);
v___x_1306_ = lp_zhc__lean_Zhc_Model_ManyLut_output(v___x_1272_, v___x_1296_, v___x_1304_, v___x_1295_, v___x_1305_);
lean_dec(v___x_1295_);
v___x_1307_ = lp_zhc__lean_List_mapM_loop___at___00Zhc_Iops_HillisSteele_groupStates___at___00Zhc_Iops_addHillisSteele___at___00__private_Zhc_Iops_AddHillisSteele_Impl_0__Zhc_Iops_smokeHs_spec__0_spec__2_spec__5___lam__0(v___x_1294_, v___x_1293_, v_fst_1289_, v___x_1306_);
v___y_1280_ = v___x_1307_;
goto v___jp_1279_;
}
v___jp_1279_:
{
lean_object* v___x_1282_; 
if (v_isShared_1278_ == 0)
{
lean_ctor_set(v___x_1277_, 1, v_x_1271_);
lean_ctor_set(v___x_1277_, 0, v___y_1280_);
v___x_1282_ = v___x_1277_;
goto v_reusejp_1281_;
}
else
{
lean_object* v_reuseFailAlloc_1284_; 
v_reuseFailAlloc_1284_ = lean_alloc_ctor(1, 2, 0);
lean_ctor_set(v_reuseFailAlloc_1284_, 0, v___y_1280_);
lean_ctor_set(v_reuseFailAlloc_1284_, 1, v_x_1271_);
v___x_1282_ = v_reuseFailAlloc_1284_;
goto v_reusejp_1281_;
}
v_reusejp_1281_:
{
v_x_1270_ = v_tail_1275_;
v_x_1271_ = v___x_1282_;
goto _start;
}
}
}
}
}
}
LEAN_EXPORT lean_object* lp_zhc__lean_Zhc_Iops_HillisSteele_groupStates___at___00Zhc_Iops_addHillisSteele___at___00__private_Zhc_Iops_AddHillisSteele_Impl_0__Zhc_Iops_smokeHs_spec__0_spec__2(lean_object* v_states_1309_){
_start:
{
lean_object* v___x_1310_; lean_object* v___x_1311_; lean_object* v___x_1312_; lean_object* v___x_1313_; lean_object* v___x_1314_; lean_object* v___x_1315_; 
v___x_1310_ = lp_zhc__lean_Zhc_Iops_HillisSteele_chunk4___redArg(v_states_1309_);
v___x_1311_ = lean_unsigned_to_nat(0u);
v___x_1312_ = l_List_zipIdxTR___redArg(v___x_1310_, v___x_1311_);
v___x_1313_ = lean_box(0);
v___x_1314_ = lp_zhc__lean_List_mapM_loop___at___00Zhc_Iops_HillisSteele_groupStates___at___00Zhc_Iops_addHillisSteele___at___00__private_Zhc_Iops_AddHillisSteele_Impl_0__Zhc_Iops_smokeHs_spec__0_spec__2_spec__5(v___x_1312_, v___x_1313_);
v___x_1315_ = lp_zhc__lean_Zhc_Iops_HillisSteele_unchunk4___redArg(v___x_1314_);
return v___x_1315_;
}
}
LEAN_EXPORT lean_object* lp_zhc__lean_List_mapM_loop___at___00Zhc_Iops_HillisSteele_rawSums___at___00Zhc_Iops_addHillisSteele___at___00__private_Zhc_Iops_AddHillisSteele_Impl_0__Zhc_Iops_smokeHs_spec__0_spec__0_spec__1___redArg(lean_object* v_x_1316_, lean_object* v_y_1317_, lean_object* v_x_1318_, lean_object* v_x_1319_){
_start:
{
lean_object* v___x_1320_; 
v___x_1320_ = ((lean_object*)(lp_zhc__lean_Zhc_Iops_HillisSteele_extractPropGroupLut___closed__0));
if (lean_obj_tag(v_x_1318_) == 0)
{
lean_object* v___x_1321_; 
v___x_1321_ = l_List_reverse___redArg(v_x_1319_);
return v___x_1321_;
}
else
{
lean_object* v_head_1322_; lean_object* v_tail_1323_; lean_object* v___x_1325_; uint8_t v_isShared_1326_; uint8_t v_isSharedCheck_1334_; 
v_head_1322_ = lean_ctor_get(v_x_1318_, 0);
v_tail_1323_ = lean_ctor_get(v_x_1318_, 1);
v_isSharedCheck_1334_ = !lean_is_exclusive(v_x_1318_);
if (v_isSharedCheck_1334_ == 0)
{
v___x_1325_ = v_x_1318_;
v_isShared_1326_ = v_isSharedCheck_1334_;
goto v_resetjp_1324_;
}
else
{
lean_inc(v_tail_1323_);
lean_inc(v_head_1322_);
lean_dec(v_x_1318_);
v___x_1325_ = lean_box(0);
v_isShared_1326_ = v_isSharedCheck_1334_;
goto v_resetjp_1324_;
}
v_resetjp_1324_:
{
lean_object* v___x_1327_; lean_object* v___x_1328_; lean_object* v___x_1329_; lean_object* v___x_1331_; 
lean_inc(v_head_1322_);
v___x_1327_ = l_List_get___redArg(v_x_1316_, v_head_1322_);
v___x_1328_ = l_List_get___redArg(v_y_1317_, v_head_1322_);
v___x_1329_ = lp_zhc__lean_Zhc_Model_Operators_add(v___x_1320_, v___x_1327_, v___x_1328_);
lean_dec(v___x_1328_);
lean_dec(v___x_1327_);
if (v_isShared_1326_ == 0)
{
lean_ctor_set(v___x_1325_, 1, v_x_1319_);
lean_ctor_set(v___x_1325_, 0, v___x_1329_);
v___x_1331_ = v___x_1325_;
goto v_reusejp_1330_;
}
else
{
lean_object* v_reuseFailAlloc_1333_; 
v_reuseFailAlloc_1333_ = lean_alloc_ctor(1, 2, 0);
lean_ctor_set(v_reuseFailAlloc_1333_, 0, v___x_1329_);
lean_ctor_set(v_reuseFailAlloc_1333_, 1, v_x_1319_);
v___x_1331_ = v_reuseFailAlloc_1333_;
goto v_reusejp_1330_;
}
v_reusejp_1330_:
{
v_x_1318_ = v_tail_1323_;
v_x_1319_ = v___x_1331_;
goto _start;
}
}
}
}
}
LEAN_EXPORT lean_object* lp_zhc__lean_List_mapM_loop___at___00Zhc_Iops_HillisSteele_rawSums___at___00Zhc_Iops_addHillisSteele___at___00__private_Zhc_Iops_AddHillisSteele_Impl_0__Zhc_Iops_smokeHs_spec__0_spec__0_spec__1___redArg___boxed(lean_object* v_x_1335_, lean_object* v_y_1336_, lean_object* v_x_1337_, lean_object* v_x_1338_){
_start:
{
lean_object* v_res_1339_; 
v_res_1339_ = lp_zhc__lean_List_mapM_loop___at___00Zhc_Iops_HillisSteele_rawSums___at___00Zhc_Iops_addHillisSteele___at___00__private_Zhc_Iops_AddHillisSteele_Impl_0__Zhc_Iops_smokeHs_spec__0_spec__0_spec__1___redArg(v_x_1335_, v_y_1336_, v_x_1337_, v_x_1338_);
lean_dec(v_y_1336_);
lean_dec(v_x_1335_);
return v_res_1339_;
}
}
static lean_object* _init_lp_zhc__lean_Zhc_Iops_HillisSteele_rawSums___at___00Zhc_Iops_addHillisSteele___at___00__private_Zhc_Iops_AddHillisSteele_Impl_0__Zhc_Iops_smokeHs_spec__0_spec__0___closed__0(void){
_start:
{
lean_object* v___x_1340_; lean_object* v___x_1341_; lean_object* v___x_1342_; 
v___x_1340_ = lean_unsigned_to_nat(0u);
v___x_1341_ = ((lean_object*)(lp_zhc__lean_Zhc_Iops_HillisSteele_extractPropGroupLut___closed__0));
v___x_1342_ = lp_zhc__lean_Zhc_Model_CiphertextBlock_ofNat(v___x_1341_, v___x_1340_);
return v___x_1342_;
}
}
LEAN_EXPORT lean_object* lp_zhc__lean_Zhc_Iops_HillisSteele_rawSums___at___00Zhc_Iops_addHillisSteele___at___00__private_Zhc_Iops_AddHillisSteele_Impl_0__Zhc_Iops_smokeHs_spec__0_spec__0(lean_object* v_si_1343_, lean_object* v_x_1344_, lean_object* v_y_1345_){
_start:
{
lean_object* v___x_1346_; lean_object* v___x_1347_; lean_object* v___x_1348_; lean_object* v___x_1349_; lean_object* v___x_1350_; lean_object* v___x_1351_; lean_object* v___x_1352_; lean_object* v___x_1353_; 
lean_inc(v_si_1343_);
v___x_1346_ = l_List_finRange(v_si_1343_);
v___x_1347_ = lean_box(0);
v___x_1348_ = lp_zhc__lean_List_mapM_loop___at___00Zhc_Iops_HillisSteele_rawSums___at___00Zhc_Iops_addHillisSteele___at___00__private_Zhc_Iops_AddHillisSteele_Impl_0__Zhc_Iops_smokeHs_spec__0_spec__0_spec__1___redArg(v_x_1344_, v_y_1345_, v___x_1346_, v___x_1347_);
v___x_1349_ = lean_obj_once(&lp_zhc__lean_Zhc_Iops_HillisSteele_rawSums___at___00Zhc_Iops_addHillisSteele___at___00__private_Zhc_Iops_AddHillisSteele_Impl_0__Zhc_Iops_smokeHs_spec__0_spec__0___closed__0, &lp_zhc__lean_Zhc_Iops_HillisSteele_rawSums___at___00Zhc_Iops_addHillisSteele___at___00__private_Zhc_Iops_AddHillisSteele_Impl_0__Zhc_Iops_smokeHs_spec__0_spec__0___closed__0_once, _init_lp_zhc__lean_Zhc_Iops_HillisSteele_rawSums___at___00Zhc_Iops_addHillisSteele___at___00__private_Zhc_Iops_AddHillisSteele_Impl_0__Zhc_Iops_smokeHs_spec__0_spec__0___closed__0);
v___x_1350_ = lp_zhc__lean_Zhc_Iops_HillisSteele_computeSize(v_si_1343_);
v___x_1351_ = lean_nat_sub(v___x_1350_, v_si_1343_);
lean_dec(v_si_1343_);
lean_dec(v___x_1350_);
v___x_1352_ = l_List_replicateTR___redArg(v___x_1351_, v___x_1349_);
v___x_1353_ = l_List_appendTR___redArg(v___x_1348_, v___x_1352_);
return v___x_1353_;
}
}
LEAN_EXPORT lean_object* lp_zhc__lean_Zhc_Iops_HillisSteele_rawSums___at___00Zhc_Iops_addHillisSteele___at___00__private_Zhc_Iops_AddHillisSteele_Impl_0__Zhc_Iops_smokeHs_spec__0_spec__0___boxed(lean_object* v_si_1354_, lean_object* v_x_1355_, lean_object* v_y_1356_){
_start:
{
lean_object* v_res_1357_; 
v_res_1357_ = lp_zhc__lean_Zhc_Iops_HillisSteele_rawSums___at___00Zhc_Iops_addHillisSteele___at___00__private_Zhc_Iops_AddHillisSteele_Impl_0__Zhc_Iops_smokeHs_spec__0_spec__0(v_si_1354_, v_x_1355_, v_y_1356_);
lean_dec(v_y_1356_);
lean_dec(v_x_1355_);
return v_res_1357_;
}
}
static lean_object* _init_lp_zhc__lean_List_mapM_loop___at___00Zhc_Iops_HillisSteele_blockStates___at___00Zhc_Iops_addHillisSteele___at___00__private_Zhc_Iops_AddHillisSteele_Impl_0__Zhc_Iops_smokeHs_spec__0_spec__1_spec__3___closed__0(void){
_start:
{
lean_object* v___x_1358_; lean_object* v___x_1359_; lean_object* v___x_1360_; 
v___x_1358_ = lean_unsigned_to_nat(2u);
v___x_1359_ = lean_unsigned_to_nat(0u);
v___x_1360_ = lean_nat_mod(v___x_1359_, v___x_1358_);
return v___x_1360_;
}
}
LEAN_EXPORT lean_object* lp_zhc__lean_List_mapM_loop___at___00Zhc_Iops_HillisSteele_blockStates___at___00Zhc_Iops_addHillisSteele___at___00__private_Zhc_Iops_AddHillisSteele_Impl_0__Zhc_Iops_smokeHs_spec__0_spec__1_spec__3(lean_object* v_x_1361_, lean_object* v_x_1362_){
_start:
{
lean_object* v___x_1363_; 
v___x_1363_ = ((lean_object*)(lp_zhc__lean_Zhc_Iops_HillisSteele_extractPropGroupLut___closed__0));
if (lean_obj_tag(v_x_1361_) == 0)
{
lean_object* v___x_1364_; 
v___x_1364_ = l_List_reverse___redArg(v_x_1362_);
return v___x_1364_;
}
else
{
lean_object* v_head_1365_; lean_object* v_tail_1366_; lean_object* v___x_1368_; uint8_t v_isShared_1369_; uint8_t v_isSharedCheck_1396_; 
v_head_1365_ = lean_ctor_get(v_x_1361_, 0);
v_tail_1366_ = lean_ctor_get(v_x_1361_, 1);
v_isSharedCheck_1396_ = !lean_is_exclusive(v_x_1361_);
if (v_isSharedCheck_1396_ == 0)
{
v___x_1368_ = v_x_1361_;
v_isShared_1369_ = v_isSharedCheck_1396_;
goto v_resetjp_1367_;
}
else
{
lean_inc(v_tail_1366_);
lean_inc(v_head_1365_);
lean_dec(v_x_1361_);
v___x_1368_ = lean_box(0);
v_isShared_1369_ = v_isSharedCheck_1396_;
goto v_resetjp_1367_;
}
v_resetjp_1367_:
{
lean_object* v___y_1371_; lean_object* v_fst_1376_; lean_object* v_snd_1377_; lean_object* v___x_1378_; uint8_t v___x_1379_; 
v_fst_1376_ = lean_ctor_get(v_head_1365_, 0);
lean_inc(v_fst_1376_);
v_snd_1377_ = lean_ctor_get(v_head_1365_, 1);
lean_inc(v_snd_1377_);
lean_dec(v_head_1365_);
v___x_1378_ = lean_unsigned_to_nat(0u);
v___x_1379_ = lean_nat_dec_eq(v_snd_1377_, v___x_1378_);
if (v___x_1379_ == 0)
{
lean_object* v___x_1380_; uint8_t v___x_1381_; 
v___x_1380_ = lean_unsigned_to_nat(4u);
v___x_1381_ = lean_nat_dec_lt(v_snd_1377_, v___x_1380_);
if (v___x_1381_ == 0)
{
lean_object* v___x_1382_; lean_object* v___x_1383_; lean_object* v___x_1384_; lean_object* v___x_1385_; 
v___x_1382_ = lean_nat_mod(v_snd_1377_, v___x_1380_);
lean_dec(v_snd_1377_);
v___x_1383_ = lp_zhc__lean_Zhc_Iops_HillisSteele_extractPropGroupLut(v___x_1382_);
v___x_1384_ = lean_obj_once(&lp_zhc__lean_List_mapM_loop___at___00Zhc_Iops_HillisSteele_finalResolution___at___00Zhc_Iops_addHillisSteele___at___00__private_Zhc_Iops_AddHillisSteele_Impl_0__Zhc_Iops_smokeHs_spec__0_spec__4_spec__12___closed__0, &lp_zhc__lean_List_mapM_loop___at___00Zhc_Iops_HillisSteele_finalResolution___at___00Zhc_Iops_addHillisSteele___at___00__private_Zhc_Iops_AddHillisSteele_Impl_0__Zhc_Iops_smokeHs_spec__0_spec__4_spec__12___closed__0_once, _init_lp_zhc__lean_List_mapM_loop___at___00Zhc_Iops_HillisSteele_finalResolution___at___00Zhc_Iops_addHillisSteele___at___00__private_Zhc_Iops_AddHillisSteele_Impl_0__Zhc_Iops_smokeHs_spec__0_spec__4_spec__12___closed__0);
v___x_1385_ = lp_zhc__lean_Zhc_Model_ManyLut_output(v___x_1363_, v___x_1378_, v___x_1383_, v_fst_1376_, v___x_1384_);
lean_dec(v_fst_1376_);
v___y_1371_ = v___x_1385_;
goto v___jp_1370_;
}
else
{
lean_object* v___x_1386_; lean_object* v___x_1387_; lean_object* v___x_1388_; lean_object* v___x_1389_; lean_object* v___x_1390_; lean_object* v___x_1391_; 
v___x_1386_ = lean_unsigned_to_nat(1u);
v___x_1387_ = lean_nat_sub(v_snd_1377_, v___x_1386_);
lean_dec(v_snd_1377_);
v___x_1388_ = lean_nat_mod(v___x_1387_, v___x_1380_);
lean_dec(v___x_1387_);
v___x_1389_ = lp_zhc__lean_Zhc_Iops_HillisSteele_extractPropGroupLut(v___x_1388_);
v___x_1390_ = lean_obj_once(&lp_zhc__lean_List_mapM_loop___at___00Zhc_Iops_HillisSteele_finalResolution___at___00Zhc_Iops_addHillisSteele___at___00__private_Zhc_Iops_AddHillisSteele_Impl_0__Zhc_Iops_smokeHs_spec__0_spec__4_spec__12___closed__0, &lp_zhc__lean_List_mapM_loop___at___00Zhc_Iops_HillisSteele_finalResolution___at___00Zhc_Iops_addHillisSteele___at___00__private_Zhc_Iops_AddHillisSteele_Impl_0__Zhc_Iops_smokeHs_spec__0_spec__4_spec__12___closed__0_once, _init_lp_zhc__lean_List_mapM_loop___at___00Zhc_Iops_HillisSteele_finalResolution___at___00Zhc_Iops_addHillisSteele___at___00__private_Zhc_Iops_AddHillisSteele_Impl_0__Zhc_Iops_smokeHs_spec__0_spec__4_spec__12___closed__0);
v___x_1391_ = lp_zhc__lean_Zhc_Model_ManyLut_output(v___x_1363_, v___x_1378_, v___x_1389_, v_fst_1376_, v___x_1390_);
lean_dec(v_fst_1376_);
v___y_1371_ = v___x_1391_;
goto v___jp_1370_;
}
}
else
{
lean_object* v___x_1392_; lean_object* v___x_1393_; lean_object* v___x_1394_; lean_object* v___x_1395_; 
lean_dec(v_snd_1377_);
v___x_1392_ = lp_zhc__lean_Zhc_Iops_carryMsgLut;
v___x_1393_ = lean_unsigned_to_nat(1u);
v___x_1394_ = lean_obj_once(&lp_zhc__lean_List_mapM_loop___at___00Zhc_Iops_HillisSteele_blockStates___at___00Zhc_Iops_addHillisSteele___at___00__private_Zhc_Iops_AddHillisSteele_Impl_0__Zhc_Iops_smokeHs_spec__0_spec__1_spec__3___closed__0, &lp_zhc__lean_List_mapM_loop___at___00Zhc_Iops_HillisSteele_blockStates___at___00Zhc_Iops_addHillisSteele___at___00__private_Zhc_Iops_AddHillisSteele_Impl_0__Zhc_Iops_smokeHs_spec__0_spec__1_spec__3___closed__0_once, _init_lp_zhc__lean_List_mapM_loop___at___00Zhc_Iops_HillisSteele_blockStates___at___00Zhc_Iops_addHillisSteele___at___00__private_Zhc_Iops_AddHillisSteele_Impl_0__Zhc_Iops_smokeHs_spec__0_spec__1_spec__3___closed__0);
v___x_1395_ = lp_zhc__lean_Zhc_Model_ManyLut_output(v___x_1363_, v___x_1393_, v___x_1392_, v_fst_1376_, v___x_1394_);
lean_dec(v_fst_1376_);
v___y_1371_ = v___x_1395_;
goto v___jp_1370_;
}
v___jp_1370_:
{
lean_object* v___x_1373_; 
if (v_isShared_1369_ == 0)
{
lean_ctor_set(v___x_1368_, 1, v_x_1362_);
lean_ctor_set(v___x_1368_, 0, v___y_1371_);
v___x_1373_ = v___x_1368_;
goto v_reusejp_1372_;
}
else
{
lean_object* v_reuseFailAlloc_1375_; 
v_reuseFailAlloc_1375_ = lean_alloc_ctor(1, 2, 0);
lean_ctor_set(v_reuseFailAlloc_1375_, 0, v___y_1371_);
lean_ctor_set(v_reuseFailAlloc_1375_, 1, v_x_1362_);
v___x_1373_ = v_reuseFailAlloc_1375_;
goto v_reusejp_1372_;
}
v_reusejp_1372_:
{
v_x_1361_ = v_tail_1366_;
v_x_1362_ = v___x_1373_;
goto _start;
}
}
}
}
}
}
LEAN_EXPORT lean_object* lp_zhc__lean_Zhc_Iops_HillisSteele_blockStates___at___00Zhc_Iops_addHillisSteele___at___00__private_Zhc_Iops_AddHillisSteele_Impl_0__Zhc_Iops_smokeHs_spec__0_spec__1(lean_object* v_sums_1397_){
_start:
{
lean_object* v___x_1398_; lean_object* v___x_1399_; lean_object* v___x_1400_; lean_object* v___x_1401_; 
v___x_1398_ = lean_unsigned_to_nat(0u);
v___x_1399_ = l_List_zipIdxTR___redArg(v_sums_1397_, v___x_1398_);
v___x_1400_ = lean_box(0);
v___x_1401_ = lp_zhc__lean_List_mapM_loop___at___00Zhc_Iops_HillisSteele_blockStates___at___00Zhc_Iops_addHillisSteele___at___00__private_Zhc_Iops_AddHillisSteele_Impl_0__Zhc_Iops_smokeHs_spec__0_spec__1_spec__3(v___x_1399_, v___x_1400_);
return v___x_1401_;
}
}
LEAN_EXPORT lean_object* lp_zhc__lean_Zhc_Iops_addHillisSteele___at___00__private_Zhc_Iops_AddHillisSteele_Impl_0__Zhc_Iops_smokeHs_spec__0(lean_object* v_si_1402_, lean_object* v_x_1403_, lean_object* v_y_1404_){
_start:
{
lean_object* v___x_1405_; lean_object* v___x_1406_; lean_object* v___x_1407_; lean_object* v___x_1408_; lean_object* v___x_1409_; lean_object* v___x_1410_; lean_object* v___x_1411_; 
lean_inc(v_si_1402_);
v___x_1405_ = lp_zhc__lean_Zhc_Iops_HillisSteele_rawSums___at___00Zhc_Iops_addHillisSteele___at___00__private_Zhc_Iops_AddHillisSteele_Impl_0__Zhc_Iops_smokeHs_spec__0_spec__0(v_si_1402_, v_x_1403_, v_y_1404_);
lean_inc(v___x_1405_);
v___x_1406_ = lp_zhc__lean_Zhc_Iops_HillisSteele_blockStates___at___00Zhc_Iops_addHillisSteele___at___00__private_Zhc_Iops_AddHillisSteele_Impl_0__Zhc_Iops_smokeHs_spec__0_spec__1(v___x_1405_);
v___x_1407_ = lp_zhc__lean_Zhc_Iops_HillisSteele_groupStates___at___00Zhc_Iops_addHillisSteele___at___00__private_Zhc_Iops_AddHillisSteele_Impl_0__Zhc_Iops_smokeHs_spec__0_spec__2(v___x_1406_);
lean_inc(v___x_1407_);
v___x_1408_ = lp_zhc__lean_Zhc_Iops_HillisSteele_groupCarries___at___00Zhc_Iops_addHillisSteele___at___00__private_Zhc_Iops_AddHillisSteele_Impl_0__Zhc_Iops_smokeHs_spec__0_spec__3(v___x_1407_);
v___x_1409_ = lp_zhc__lean_Zhc_Iops_HillisSteele_finalResolution___at___00Zhc_Iops_addHillisSteele___at___00__private_Zhc_Iops_AddHillisSteele_Impl_0__Zhc_Iops_smokeHs_spec__0_spec__4(v___x_1407_, v___x_1408_);
v___x_1410_ = lp_zhc__lean_Zhc_Iops_HillisSteele_propagate___at___00Zhc_Iops_addHillisSteele___at___00__private_Zhc_Iops_AddHillisSteele_Impl_0__Zhc_Iops_smokeHs_spec__0_spec__5(v___x_1405_, v___x_1409_);
v___x_1411_ = lp_zhc__lean_Zhc_Iops_HillisSteele_joinBlocks___at___00Zhc_Iops_addHillisSteele___at___00__private_Zhc_Iops_AddHillisSteele_Impl_0__Zhc_Iops_smokeHs_spec__0_spec__6(v_si_1402_, v___x_1410_);
return v___x_1411_;
}
}
LEAN_EXPORT lean_object* lp_zhc__lean_Zhc_Iops_addHillisSteele___at___00__private_Zhc_Iops_AddHillisSteele_Impl_0__Zhc_Iops_smokeHs_spec__0___boxed(lean_object* v_si_1412_, lean_object* v_x_1413_, lean_object* v_y_1414_){
_start:
{
lean_object* v_res_1415_; 
v_res_1415_ = lp_zhc__lean_Zhc_Iops_addHillisSteele___at___00__private_Zhc_Iops_AddHillisSteele_Impl_0__Zhc_Iops_smokeHs_spec__0(v_si_1412_, v_x_1413_, v_y_1414_);
lean_dec(v_y_1414_);
lean_dec(v_x_1413_);
return v_res_1415_;
}
}
LEAN_EXPORT lean_object* lp_zhc__lean___private_Zhc_Iops_AddHillisSteele_Impl_0__Zhc_Iops_smokeHs(lean_object* v_n_1416_, lean_object* v_x_1417_, lean_object* v_y_1418_){
_start:
{
lean_object* v___x_1419_; lean_object* v___x_1420_; lean_object* v___x_1421_; lean_object* v___x_1422_; lean_object* v___x_1423_; 
v___x_1419_ = ((lean_object*)(lp_zhc__lean_Zhc_Iops_HillisSteele_extractPropGroupLut___closed__0));
v___x_1420_ = lp_zhc__lean_Zhc_Model_CiphertextBlock_digits(v___x_1419_, v_n_1416_, v_x_1417_);
v___x_1421_ = lp_zhc__lean_Zhc_Model_CiphertextBlock_digits(v___x_1419_, v_n_1416_, v_y_1418_);
v___x_1422_ = lp_zhc__lean_Zhc_Iops_addHillisSteele___at___00__private_Zhc_Iops_AddHillisSteele_Impl_0__Zhc_Iops_smokeHs_spec__0(v_n_1416_, v___x_1420_, v___x_1421_);
lean_dec(v___x_1421_);
lean_dec(v___x_1420_);
v___x_1423_ = lp_zhc__lean_Zhc_Model_CiphertextBlock_listToNat(v___x_1419_, v___x_1422_);
lean_dec(v___x_1422_);
return v___x_1423_;
}
}
LEAN_EXPORT lean_object* lp_zhc__lean___private_Zhc_Iops_AddHillisSteele_Impl_0__Zhc_Iops_smokeHs___boxed(lean_object* v_n_1424_, lean_object* v_x_1425_, lean_object* v_y_1426_){
_start:
{
lean_object* v_res_1427_; 
v_res_1427_ = lp_zhc__lean___private_Zhc_Iops_AddHillisSteele_Impl_0__Zhc_Iops_smokeHs(v_n_1424_, v_x_1425_, v_y_1426_);
lean_dec(v_y_1426_);
lean_dec(v_x_1425_);
return v_res_1427_;
}
}
LEAN_EXPORT lean_object* lp_zhc__lean_List_mapM_loop___at___00Zhc_Iops_HillisSteele_rawSums___at___00Zhc_Iops_addHillisSteele___at___00__private_Zhc_Iops_AddHillisSteele_Impl_0__Zhc_Iops_smokeHs_spec__0_spec__0_spec__1(lean_object* v_si_1428_, lean_object* v_x_1429_, lean_object* v_y_1430_, lean_object* v_x_1431_, lean_object* v_x_1432_){
_start:
{
lean_object* v___x_1433_; 
v___x_1433_ = lp_zhc__lean_List_mapM_loop___at___00Zhc_Iops_HillisSteele_rawSums___at___00Zhc_Iops_addHillisSteele___at___00__private_Zhc_Iops_AddHillisSteele_Impl_0__Zhc_Iops_smokeHs_spec__0_spec__0_spec__1___redArg(v_x_1429_, v_y_1430_, v_x_1431_, v_x_1432_);
return v___x_1433_;
}
}
LEAN_EXPORT lean_object* lp_zhc__lean_List_mapM_loop___at___00Zhc_Iops_HillisSteele_rawSums___at___00Zhc_Iops_addHillisSteele___at___00__private_Zhc_Iops_AddHillisSteele_Impl_0__Zhc_Iops_smokeHs_spec__0_spec__0_spec__1___boxed(lean_object* v_si_1434_, lean_object* v_x_1435_, lean_object* v_y_1436_, lean_object* v_x_1437_, lean_object* v_x_1438_){
_start:
{
lean_object* v_res_1439_; 
v_res_1439_ = lp_zhc__lean_List_mapM_loop___at___00Zhc_Iops_HillisSteele_rawSums___at___00Zhc_Iops_addHillisSteele___at___00__private_Zhc_Iops_AddHillisSteele_Impl_0__Zhc_Iops_smokeHs_spec__0_spec__0_spec__1(v_si_1434_, v_x_1435_, v_y_1436_, v_x_1437_, v_x_1438_);
lean_dec(v_y_1436_);
lean_dec(v_x_1435_);
lean_dec(v_si_1434_);
return v_res_1439_;
}
}
LEAN_EXPORT lean_object* lp_zhc__lean_List_forIn_x27_loop___at___00Zhc_Iops_HillisSteele_groupCarries___at___00Zhc_Iops_addHillisSteele___at___00__private_Zhc_Iops_AddHillisSteele_Impl_0__Zhc_Iops_smokeHs_spec__0_spec__3_spec__9(lean_object* v_as_1440_, lean_object* v_as_x27_1441_, lean_object* v_b_1442_, lean_object* v_a_1443_){
_start:
{
lean_object* v___x_1444_; 
v___x_1444_ = lp_zhc__lean_List_forIn_x27_loop___at___00Zhc_Iops_HillisSteele_groupCarries___at___00Zhc_Iops_addHillisSteele___at___00__private_Zhc_Iops_AddHillisSteele_Impl_0__Zhc_Iops_smokeHs_spec__0_spec__3_spec__9___redArg(v_as_x27_1441_, v_b_1442_);
return v___x_1444_;
}
}
LEAN_EXPORT lean_object* lp_zhc__lean_List_forIn_x27_loop___at___00Zhc_Iops_HillisSteele_groupCarries___at___00Zhc_Iops_addHillisSteele___at___00__private_Zhc_Iops_AddHillisSteele_Impl_0__Zhc_Iops_smokeHs_spec__0_spec__3_spec__9___boxed(lean_object* v_as_1445_, lean_object* v_as_x27_1446_, lean_object* v_b_1447_, lean_object* v_a_1448_){
_start:
{
lean_object* v_res_1449_; 
v_res_1449_ = lp_zhc__lean_List_forIn_x27_loop___at___00Zhc_Iops_HillisSteele_groupCarries___at___00Zhc_Iops_addHillisSteele___at___00__private_Zhc_Iops_AddHillisSteele_Impl_0__Zhc_Iops_smokeHs_spec__0_spec__3_spec__9(v_as_1445_, v_as_x27_1446_, v_b_1447_, v_a_1448_);
lean_dec(v_as_x27_1446_);
lean_dec(v_as_1445_);
return v_res_1449_;
}
}
LEAN_EXPORT lean_object* lp_zhc__lean_List_forIn_x27_loop___at___00Zhc_Iops_HillisSteele_joinBlocks___at___00Zhc_Iops_addHillisSteele___at___00__private_Zhc_Iops_AddHillisSteele_Impl_0__Zhc_Iops_smokeHs_spec__0_spec__6_spec__17(lean_object* v_si_1450_, lean_object* v_as_1451_, lean_object* v_as_x27_1452_, lean_object* v_b_1453_, lean_object* v_a_1454_){
_start:
{
lean_object* v___x_1455_; 
v___x_1455_ = lp_zhc__lean_List_forIn_x27_loop___at___00Zhc_Iops_HillisSteele_joinBlocks___at___00Zhc_Iops_addHillisSteele___at___00__private_Zhc_Iops_AddHillisSteele_Impl_0__Zhc_Iops_smokeHs_spec__0_spec__6_spec__17___redArg(v_as_x27_1452_, v_b_1453_);
return v___x_1455_;
}
}
LEAN_EXPORT lean_object* lp_zhc__lean_List_forIn_x27_loop___at___00Zhc_Iops_HillisSteele_joinBlocks___at___00Zhc_Iops_addHillisSteele___at___00__private_Zhc_Iops_AddHillisSteele_Impl_0__Zhc_Iops_smokeHs_spec__0_spec__6_spec__17___boxed(lean_object* v_si_1456_, lean_object* v_as_1457_, lean_object* v_as_x27_1458_, lean_object* v_b_1459_, lean_object* v_a_1460_){
_start:
{
lean_object* v_res_1461_; 
v_res_1461_ = lp_zhc__lean_List_forIn_x27_loop___at___00Zhc_Iops_HillisSteele_joinBlocks___at___00Zhc_Iops_addHillisSteele___at___00__private_Zhc_Iops_AddHillisSteele_Impl_0__Zhc_Iops_smokeHs_spec__0_spec__6_spec__17(v_si_1456_, v_as_1457_, v_as_x27_1458_, v_b_1459_, v_a_1460_);
lean_dec(v_as_x27_1458_);
lean_dec(v_as_1457_);
lean_dec(v_si_1456_);
return v_res_1461_;
}
}
LEAN_EXPORT lean_object* lp_zhc__lean_List_mapM_loop___at___00Zhc_Iops_HillisSteele_hsStage___at___00Zhc_Iops_HillisSteele_groupCarries___at___00Zhc_Iops_addHillisSteele___at___00Zhc_Iops_addHillisSteeleCircuit_spec__0_spec__3_spec__7_spec__8(lean_object* v_stride_1462_, lean_object* v_x_1463_, lean_object* v_x_1464_, lean_object* v___y_1465_){
_start:
{
lean_object* v___x_1467_; 
v___x_1467_ = ((lean_object*)(lp_zhc__lean_Zhc_Iops_HillisSteele_extractPropGroupLut___closed__0));
if (lean_obj_tag(v_x_1463_) == 0)
{
lean_object* v___x_1468_; lean_object* v___x_1469_; 
v___x_1468_ = l_List_reverse___redArg(v_x_1464_);
v___x_1469_ = lean_alloc_ctor(0, 1, 0);
lean_ctor_set(v___x_1469_, 0, v___x_1468_);
return v___x_1469_;
}
else
{
lean_object* v_head_1470_; lean_object* v_tail_1471_; lean_object* v___x_1473_; uint8_t v_isShared_1474_; uint8_t v_isSharedCheck_1502_; 
v_head_1470_ = lean_ctor_get(v_x_1463_, 0);
v_tail_1471_ = lean_ctor_get(v_x_1463_, 1);
v_isSharedCheck_1502_ = !lean_is_exclusive(v_x_1463_);
if (v_isSharedCheck_1502_ == 0)
{
v___x_1473_ = v_x_1463_;
v_isShared_1474_ = v_isSharedCheck_1502_;
goto v_resetjp_1472_;
}
else
{
lean_inc(v_tail_1471_);
lean_inc(v_head_1470_);
lean_dec(v_x_1463_);
v___x_1473_ = lean_box(0);
v_isShared_1474_ = v_isSharedCheck_1502_;
goto v_resetjp_1472_;
}
v_resetjp_1472_:
{
lean_object* v___y_1476_; lean_object* v_fst_1490_; lean_object* v_snd_1491_; lean_object* v_fst_1492_; lean_object* v_snd_1493_; lean_object* v___x_1494_; lean_object* v___x_1495_; 
v_fst_1490_ = lean_ctor_get(v_head_1470_, 0);
lean_inc(v_fst_1490_);
v_snd_1491_ = lean_ctor_get(v_head_1470_, 1);
lean_inc(v_snd_1491_);
lean_dec(v_head_1470_);
v_fst_1492_ = lean_ctor_get(v_fst_1490_, 0);
lean_inc(v_fst_1492_);
v_snd_1493_ = lean_ctor_get(v_fst_1490_, 1);
lean_inc(v_snd_1493_);
lean_dec(v_fst_1490_);
v___x_1494_ = lean_unsigned_to_nat(4u);
v___x_1495_ = lp_zhc__lean_Zhc_BlockAlgebra_Tracer_packCt(v___x_1467_, v_fst_1492_, v___x_1494_, v_snd_1493_, v___y_1465_);
lean_dec(v_fst_1492_);
if (lean_obj_tag(v___x_1495_) == 0)
{
lean_object* v_a_1496_; uint8_t v___x_1497_; 
v_a_1496_ = lean_ctor_get(v___x_1495_, 0);
lean_inc(v_a_1496_);
lean_dec_ref_known(v___x_1495_, 1);
v___x_1497_ = lean_nat_dec_lt(v_snd_1491_, v_stride_1462_);
lean_dec(v_snd_1491_);
if (v___x_1497_ == 0)
{
lean_object* v___x_1498_; lean_object* v___x_1499_; 
v___x_1498_ = lp_zhc__lean_Zhc_Iops_HillisSteele_solvePropLut;
v___x_1499_ = lp_zhc__lean_Zhc_BlockAlgebra_Tracer_pbs(v___x_1467_, v___x_1498_, v_a_1496_, v___y_1465_);
v___y_1476_ = v___x_1499_;
goto v___jp_1475_;
}
else
{
lean_object* v___x_1500_; lean_object* v___x_1501_; 
v___x_1500_ = lp_zhc__lean_Zhc_Iops_HillisSteele_solvePropCarryLut;
v___x_1501_ = lp_zhc__lean_Zhc_BlockAlgebra_Tracer_pbs(v___x_1467_, v___x_1500_, v_a_1496_, v___y_1465_);
v___y_1476_ = v___x_1501_;
goto v___jp_1475_;
}
}
else
{
lean_dec(v_snd_1491_);
v___y_1476_ = v___x_1495_;
goto v___jp_1475_;
}
v___jp_1475_:
{
if (lean_obj_tag(v___y_1476_) == 0)
{
lean_object* v_a_1477_; lean_object* v___x_1479_; 
v_a_1477_ = lean_ctor_get(v___y_1476_, 0);
lean_inc(v_a_1477_);
lean_dec_ref_known(v___y_1476_, 1);
if (v_isShared_1474_ == 0)
{
lean_ctor_set(v___x_1473_, 1, v_x_1464_);
lean_ctor_set(v___x_1473_, 0, v_a_1477_);
v___x_1479_ = v___x_1473_;
goto v_reusejp_1478_;
}
else
{
lean_object* v_reuseFailAlloc_1481_; 
v_reuseFailAlloc_1481_ = lean_alloc_ctor(1, 2, 0);
lean_ctor_set(v_reuseFailAlloc_1481_, 0, v_a_1477_);
lean_ctor_set(v_reuseFailAlloc_1481_, 1, v_x_1464_);
v___x_1479_ = v_reuseFailAlloc_1481_;
goto v_reusejp_1478_;
}
v_reusejp_1478_:
{
v_x_1463_ = v_tail_1471_;
v_x_1464_ = v___x_1479_;
goto _start;
}
}
else
{
lean_object* v_a_1482_; lean_object* v___x_1484_; uint8_t v_isShared_1485_; uint8_t v_isSharedCheck_1489_; 
lean_del_object(v___x_1473_);
lean_dec(v_tail_1471_);
lean_dec(v_x_1464_);
v_a_1482_ = lean_ctor_get(v___y_1476_, 0);
v_isSharedCheck_1489_ = !lean_is_exclusive(v___y_1476_);
if (v_isSharedCheck_1489_ == 0)
{
v___x_1484_ = v___y_1476_;
v_isShared_1485_ = v_isSharedCheck_1489_;
goto v_resetjp_1483_;
}
else
{
lean_inc(v_a_1482_);
lean_dec(v___y_1476_);
v___x_1484_ = lean_box(0);
v_isShared_1485_ = v_isSharedCheck_1489_;
goto v_resetjp_1483_;
}
v_resetjp_1483_:
{
lean_object* v___x_1487_; 
if (v_isShared_1485_ == 0)
{
v___x_1487_ = v___x_1484_;
goto v_reusejp_1486_;
}
else
{
lean_object* v_reuseFailAlloc_1488_; 
v_reuseFailAlloc_1488_ = lean_alloc_ctor(1, 1, 0);
lean_ctor_set(v_reuseFailAlloc_1488_, 0, v_a_1482_);
v___x_1487_ = v_reuseFailAlloc_1488_;
goto v_reusejp_1486_;
}
v_reusejp_1486_:
{
return v___x_1487_;
}
}
}
}
}
}
}
}
LEAN_EXPORT lean_object* lp_zhc__lean_List_mapM_loop___at___00Zhc_Iops_HillisSteele_hsStage___at___00Zhc_Iops_HillisSteele_groupCarries___at___00Zhc_Iops_addHillisSteele___at___00Zhc_Iops_addHillisSteeleCircuit_spec__0_spec__3_spec__7_spec__8___boxed(lean_object* v_stride_1503_, lean_object* v_x_1504_, lean_object* v_x_1505_, lean_object* v___y_1506_, lean_object* v___y_1507_){
_start:
{
lean_object* v_res_1508_; 
v_res_1508_ = lp_zhc__lean_List_mapM_loop___at___00Zhc_Iops_HillisSteele_hsStage___at___00Zhc_Iops_HillisSteele_groupCarries___at___00Zhc_Iops_addHillisSteele___at___00Zhc_Iops_addHillisSteeleCircuit_spec__0_spec__3_spec__7_spec__8(v_stride_1503_, v_x_1504_, v_x_1505_, v___y_1506_);
lean_dec(v___y_1506_);
lean_dec(v_stride_1503_);
return v_res_1508_;
}
}
LEAN_EXPORT lean_object* lp_zhc__lean_Zhc_Iops_HillisSteele_hsStage___at___00Zhc_Iops_HillisSteele_groupCarries___at___00Zhc_Iops_addHillisSteele___at___00Zhc_Iops_addHillisSteeleCircuit_spec__0_spec__3_spec__7(lean_object* v_stage_1509_, lean_object* v_carries_1510_, lean_object* v___y_1511_){
_start:
{
lean_object* v___x_1513_; lean_object* v_stride_1514_; lean_object* v___x_1515_; lean_object* v___x_1516_; lean_object* v___x_1517_; lean_object* v___x_1518_; lean_object* v___x_1519_; lean_object* v___x_1520_; 
v___x_1513_ = lean_unsigned_to_nat(2u);
v_stride_1514_ = lean_nat_pow(v___x_1513_, v_stage_1509_);
lean_inc(v_stride_1514_);
v___x_1515_ = l_List_drop___redArg(v_stride_1514_, v_carries_1510_);
lean_inc(v_carries_1510_);
v___x_1516_ = l_List_zipWith___at___00List_zip_spec__0___redArg(v___x_1515_, v_carries_1510_);
v___x_1517_ = lean_unsigned_to_nat(0u);
v___x_1518_ = l_List_zipIdxTR___redArg(v___x_1516_, v___x_1517_);
v___x_1519_ = lean_box(0);
v___x_1520_ = lp_zhc__lean_List_mapM_loop___at___00Zhc_Iops_HillisSteele_hsStage___at___00Zhc_Iops_HillisSteele_groupCarries___at___00Zhc_Iops_addHillisSteele___at___00Zhc_Iops_addHillisSteeleCircuit_spec__0_spec__3_spec__7_spec__8(v_stride_1514_, v___x_1518_, v___x_1519_, v___y_1511_);
if (lean_obj_tag(v___x_1520_) == 0)
{
lean_object* v_a_1521_; lean_object* v___x_1523_; uint8_t v_isShared_1524_; uint8_t v_isSharedCheck_1531_; 
v_a_1521_ = lean_ctor_get(v___x_1520_, 0);
v_isSharedCheck_1531_ = !lean_is_exclusive(v___x_1520_);
if (v_isSharedCheck_1531_ == 0)
{
v___x_1523_ = v___x_1520_;
v_isShared_1524_ = v_isSharedCheck_1531_;
goto v_resetjp_1522_;
}
else
{
lean_inc(v_a_1521_);
lean_dec(v___x_1520_);
v___x_1523_ = lean_box(0);
v_isShared_1524_ = v_isSharedCheck_1531_;
goto v_resetjp_1522_;
}
v_resetjp_1522_:
{
lean_object* v___x_1525_; lean_object* v___x_1526_; lean_object* v___x_1527_; lean_object* v___x_1529_; 
v___x_1525_ = ((lean_object*)(lp_zhc__lean_Zhc_Iops_HillisSteele_hsStage___redArg___lam__2___closed__0));
lean_inc(v_carries_1510_);
v___x_1526_ = l___private_Init_Data_List_Impl_0__List_takeTR_go___redArg(v_carries_1510_, v_carries_1510_, v_stride_1514_, v___x_1525_);
lean_dec(v_carries_1510_);
v___x_1527_ = l_List_appendTR___redArg(v___x_1526_, v_a_1521_);
if (v_isShared_1524_ == 0)
{
lean_ctor_set(v___x_1523_, 0, v___x_1527_);
v___x_1529_ = v___x_1523_;
goto v_reusejp_1528_;
}
else
{
lean_object* v_reuseFailAlloc_1530_; 
v_reuseFailAlloc_1530_ = lean_alloc_ctor(0, 1, 0);
lean_ctor_set(v_reuseFailAlloc_1530_, 0, v___x_1527_);
v___x_1529_ = v_reuseFailAlloc_1530_;
goto v_reusejp_1528_;
}
v_reusejp_1528_:
{
return v___x_1529_;
}
}
}
else
{
lean_dec(v_stride_1514_);
lean_dec(v_carries_1510_);
return v___x_1520_;
}
}
}
LEAN_EXPORT lean_object* lp_zhc__lean_Zhc_Iops_HillisSteele_hsStage___at___00Zhc_Iops_HillisSteele_groupCarries___at___00Zhc_Iops_addHillisSteele___at___00Zhc_Iops_addHillisSteeleCircuit_spec__0_spec__3_spec__7___boxed(lean_object* v_stage_1532_, lean_object* v_carries_1533_, lean_object* v___y_1534_, lean_object* v___y_1535_){
_start:
{
lean_object* v_res_1536_; 
v_res_1536_ = lp_zhc__lean_Zhc_Iops_HillisSteele_hsStage___at___00Zhc_Iops_HillisSteele_groupCarries___at___00Zhc_Iops_addHillisSteele___at___00Zhc_Iops_addHillisSteeleCircuit_spec__0_spec__3_spec__7(v_stage_1532_, v_carries_1533_, v___y_1534_);
lean_dec(v___y_1534_);
lean_dec(v_stage_1532_);
return v_res_1536_;
}
}
LEAN_EXPORT lean_object* lp_zhc__lean_List_forIn_x27_loop___at___00Zhc_Iops_HillisSteele_groupCarries___at___00Zhc_Iops_addHillisSteele___at___00Zhc_Iops_addHillisSteeleCircuit_spec__0_spec__3_spec__8___redArg(lean_object* v_as_x27_1537_, lean_object* v_b_1538_, lean_object* v___y_1539_){
_start:
{
if (lean_obj_tag(v_as_x27_1537_) == 0)
{
lean_object* v___x_1541_; 
v___x_1541_ = lean_alloc_ctor(0, 1, 0);
lean_ctor_set(v___x_1541_, 0, v_b_1538_);
return v___x_1541_;
}
else
{
lean_object* v_head_1542_; lean_object* v_tail_1543_; lean_object* v___x_1544_; 
v_head_1542_ = lean_ctor_get(v_as_x27_1537_, 0);
v_tail_1543_ = lean_ctor_get(v_as_x27_1537_, 1);
v___x_1544_ = lp_zhc__lean_Zhc_Iops_HillisSteele_hsStage___at___00Zhc_Iops_HillisSteele_groupCarries___at___00Zhc_Iops_addHillisSteele___at___00Zhc_Iops_addHillisSteeleCircuit_spec__0_spec__3_spec__7(v_head_1542_, v_b_1538_, v___y_1539_);
if (lean_obj_tag(v___x_1544_) == 0)
{
lean_object* v_a_1545_; 
v_a_1545_ = lean_ctor_get(v___x_1544_, 0);
lean_inc(v_a_1545_);
lean_dec_ref_known(v___x_1544_, 1);
v_as_x27_1537_ = v_tail_1543_;
v_b_1538_ = v_a_1545_;
goto _start;
}
else
{
return v___x_1544_;
}
}
}
}
LEAN_EXPORT lean_object* lp_zhc__lean_List_forIn_x27_loop___at___00Zhc_Iops_HillisSteele_groupCarries___at___00Zhc_Iops_addHillisSteele___at___00Zhc_Iops_addHillisSteeleCircuit_spec__0_spec__3_spec__8___redArg___boxed(lean_object* v_as_x27_1547_, lean_object* v_b_1548_, lean_object* v___y_1549_, lean_object* v___y_1550_){
_start:
{
lean_object* v_res_1551_; 
v_res_1551_ = lp_zhc__lean_List_forIn_x27_loop___at___00Zhc_Iops_HillisSteele_groupCarries___at___00Zhc_Iops_addHillisSteele___at___00Zhc_Iops_addHillisSteeleCircuit_spec__0_spec__3_spec__8___redArg(v_as_x27_1547_, v_b_1548_, v___y_1549_);
lean_dec(v___y_1549_);
lean_dec(v_as_x27_1547_);
return v_res_1551_;
}
}
LEAN_EXPORT lean_object* lp_zhc__lean_Zhc_Iops_HillisSteele_groupCarries___at___00Zhc_Iops_addHillisSteele___at___00Zhc_Iops_addHillisSteeleCircuit_spec__0_spec__3(lean_object* v_groups_1552_, lean_object* v___y_1553_){
_start:
{
lean_object* v___x_1555_; lean_object* v___x_1556_; lean_object* v_carries_1557_; lean_object* v___x_1558_; lean_object* v___x_1559_; lean_object* v___x_1560_; lean_object* v___x_1561_; 
v___x_1555_ = lp_zhc__lean_Zhc_Iops_HillisSteele_chunk4___redArg(v_groups_1552_);
v___x_1556_ = lean_box(0);
v_carries_1557_ = lp_zhc__lean_List_mapTR_loop___at___00Zhc_Iops_HillisSteele_groupCarries___at___00Zhc_Iops_addHillisSteele___at___00__private_Zhc_Iops_AddHillisSteele_Impl_0__Zhc_Iops_smokeHs_spec__0_spec__3_spec__7(v___x_1555_, v___x_1556_);
v___x_1558_ = l_List_lengthTR___redArg(v_carries_1557_);
v___x_1559_ = lean_nat_log2(v___x_1558_);
lean_dec(v___x_1558_);
v___x_1560_ = l_List_range(v___x_1559_);
v___x_1561_ = lp_zhc__lean_List_forIn_x27_loop___at___00Zhc_Iops_HillisSteele_groupCarries___at___00Zhc_Iops_addHillisSteele___at___00Zhc_Iops_addHillisSteeleCircuit_spec__0_spec__3_spec__8___redArg(v___x_1560_, v_carries_1557_, v___y_1553_);
lean_dec(v___x_1560_);
return v___x_1561_;
}
}
LEAN_EXPORT lean_object* lp_zhc__lean_Zhc_Iops_HillisSteele_groupCarries___at___00Zhc_Iops_addHillisSteele___at___00Zhc_Iops_addHillisSteeleCircuit_spec__0_spec__3___boxed(lean_object* v_groups_1562_, lean_object* v___y_1563_, lean_object* v___y_1564_){
_start:
{
lean_object* v_res_1565_; 
v_res_1565_ = lp_zhc__lean_Zhc_Iops_HillisSteele_groupCarries___at___00Zhc_Iops_addHillisSteele___at___00Zhc_Iops_addHillisSteeleCircuit_spec__0_spec__3(v_groups_1562_, v___y_1563_);
lean_dec(v___y_1563_);
return v_res_1565_;
}
}
LEAN_EXPORT lean_object* lp_zhc__lean_List_mapM_loop___at___00Zhc_Iops_HillisSteele_propagate___at___00Zhc_Iops_addHillisSteele___at___00Zhc_Iops_addHillisSteeleCircuit_spec__0_spec__5_spec__13(lean_object* v_x_1566_, lean_object* v_x_1567_, lean_object* v___y_1568_){
_start:
{
lean_object* v___x_1570_; 
v___x_1570_ = ((lean_object*)(lp_zhc__lean_Zhc_Iops_HillisSteele_extractPropGroupLut___closed__0));
if (lean_obj_tag(v_x_1566_) == 0)
{
lean_object* v___x_1571_; lean_object* v___x_1572_; 
v___x_1571_ = l_List_reverse___redArg(v_x_1567_);
v___x_1572_ = lean_alloc_ctor(0, 1, 0);
lean_ctor_set(v___x_1572_, 0, v___x_1571_);
return v___x_1572_;
}
else
{
lean_object* v_head_1573_; lean_object* v_tail_1574_; lean_object* v___x_1576_; uint8_t v_isShared_1577_; uint8_t v_isSharedCheck_1593_; 
v_head_1573_ = lean_ctor_get(v_x_1566_, 0);
v_tail_1574_ = lean_ctor_get(v_x_1566_, 1);
v_isSharedCheck_1593_ = !lean_is_exclusive(v_x_1566_);
if (v_isSharedCheck_1593_ == 0)
{
v___x_1576_ = v_x_1566_;
v_isShared_1577_ = v_isSharedCheck_1593_;
goto v_resetjp_1575_;
}
else
{
lean_inc(v_tail_1574_);
lean_inc(v_head_1573_);
lean_dec(v_x_1566_);
v___x_1576_ = lean_box(0);
v_isShared_1577_ = v_isSharedCheck_1593_;
goto v_resetjp_1575_;
}
v_resetjp_1575_:
{
lean_object* v___x_1578_; lean_object* v___x_1579_; 
v___x_1578_ = lp_zhc__lean_Zhc_Iops_HillisSteele_msgOnlyLut;
v___x_1579_ = lp_zhc__lean_Zhc_BlockAlgebra_Tracer_pbs(v___x_1570_, v___x_1578_, v_head_1573_, v___y_1568_);
if (lean_obj_tag(v___x_1579_) == 0)
{
lean_object* v_a_1580_; lean_object* v___x_1582_; 
v_a_1580_ = lean_ctor_get(v___x_1579_, 0);
lean_inc(v_a_1580_);
lean_dec_ref_known(v___x_1579_, 1);
if (v_isShared_1577_ == 0)
{
lean_ctor_set(v___x_1576_, 1, v_x_1567_);
lean_ctor_set(v___x_1576_, 0, v_a_1580_);
v___x_1582_ = v___x_1576_;
goto v_reusejp_1581_;
}
else
{
lean_object* v_reuseFailAlloc_1584_; 
v_reuseFailAlloc_1584_ = lean_alloc_ctor(1, 2, 0);
lean_ctor_set(v_reuseFailAlloc_1584_, 0, v_a_1580_);
lean_ctor_set(v_reuseFailAlloc_1584_, 1, v_x_1567_);
v___x_1582_ = v_reuseFailAlloc_1584_;
goto v_reusejp_1581_;
}
v_reusejp_1581_:
{
v_x_1566_ = v_tail_1574_;
v_x_1567_ = v___x_1582_;
goto _start;
}
}
else
{
lean_object* v_a_1585_; lean_object* v___x_1587_; uint8_t v_isShared_1588_; uint8_t v_isSharedCheck_1592_; 
lean_del_object(v___x_1576_);
lean_dec(v_tail_1574_);
lean_dec(v_x_1567_);
v_a_1585_ = lean_ctor_get(v___x_1579_, 0);
v_isSharedCheck_1592_ = !lean_is_exclusive(v___x_1579_);
if (v_isSharedCheck_1592_ == 0)
{
v___x_1587_ = v___x_1579_;
v_isShared_1588_ = v_isSharedCheck_1592_;
goto v_resetjp_1586_;
}
else
{
lean_inc(v_a_1585_);
lean_dec(v___x_1579_);
v___x_1587_ = lean_box(0);
v_isShared_1588_ = v_isSharedCheck_1592_;
goto v_resetjp_1586_;
}
v_resetjp_1586_:
{
lean_object* v___x_1590_; 
if (v_isShared_1588_ == 0)
{
v___x_1590_ = v___x_1587_;
goto v_reusejp_1589_;
}
else
{
lean_object* v_reuseFailAlloc_1591_; 
v_reuseFailAlloc_1591_ = lean_alloc_ctor(1, 1, 0);
lean_ctor_set(v_reuseFailAlloc_1591_, 0, v_a_1585_);
v___x_1590_ = v_reuseFailAlloc_1591_;
goto v_reusejp_1589_;
}
v_reusejp_1589_:
{
return v___x_1590_;
}
}
}
}
}
}
}
LEAN_EXPORT lean_object* lp_zhc__lean_List_mapM_loop___at___00Zhc_Iops_HillisSteele_propagate___at___00Zhc_Iops_addHillisSteele___at___00Zhc_Iops_addHillisSteeleCircuit_spec__0_spec__5_spec__13___boxed(lean_object* v_x_1594_, lean_object* v_x_1595_, lean_object* v___y_1596_, lean_object* v___y_1597_){
_start:
{
lean_object* v_res_1598_; 
v_res_1598_ = lp_zhc__lean_List_mapM_loop___at___00Zhc_Iops_HillisSteele_propagate___at___00Zhc_Iops_addHillisSteele___at___00Zhc_Iops_addHillisSteeleCircuit_spec__0_spec__5_spec__13(v_x_1594_, v_x_1595_, v___y_1596_);
lean_dec(v___y_1596_);
return v_res_1598_;
}
}
LEAN_EXPORT lean_object* lp_zhc__lean_List_mapM_loop___at___00Zhc_Iops_HillisSteele_propagate___at___00Zhc_Iops_addHillisSteele___at___00Zhc_Iops_addHillisSteeleCircuit_spec__0_spec__5_spec__12(lean_object* v_x_1599_, lean_object* v_x_1600_, lean_object* v___y_1601_){
_start:
{
lean_object* v___x_1603_; 
v___x_1603_ = ((lean_object*)(lp_zhc__lean_Zhc_Iops_HillisSteele_extractPropGroupLut___closed__0));
if (lean_obj_tag(v_x_1599_) == 0)
{
lean_object* v___x_1604_; lean_object* v___x_1605_; 
v___x_1604_ = l_List_reverse___redArg(v_x_1600_);
v___x_1605_ = lean_alloc_ctor(0, 1, 0);
lean_ctor_set(v___x_1605_, 0, v___x_1604_);
return v___x_1605_;
}
else
{
lean_object* v_head_1606_; lean_object* v_tail_1607_; lean_object* v___x_1609_; uint8_t v_isShared_1610_; uint8_t v_isSharedCheck_1627_; 
v_head_1606_ = lean_ctor_get(v_x_1599_, 0);
v_tail_1607_ = lean_ctor_get(v_x_1599_, 1);
v_isSharedCheck_1627_ = !lean_is_exclusive(v_x_1599_);
if (v_isSharedCheck_1627_ == 0)
{
v___x_1609_ = v_x_1599_;
v_isShared_1610_ = v_isSharedCheck_1627_;
goto v_resetjp_1608_;
}
else
{
lean_inc(v_tail_1607_);
lean_inc(v_head_1606_);
lean_dec(v_x_1599_);
v___x_1609_ = lean_box(0);
v_isShared_1610_ = v_isSharedCheck_1627_;
goto v_resetjp_1608_;
}
v_resetjp_1608_:
{
lean_object* v_fst_1611_; lean_object* v_snd_1612_; lean_object* v___x_1613_; 
v_fst_1611_ = lean_ctor_get(v_head_1606_, 0);
lean_inc(v_fst_1611_);
v_snd_1612_ = lean_ctor_get(v_head_1606_, 1);
lean_inc(v_snd_1612_);
lean_dec(v_head_1606_);
v___x_1613_ = lp_zhc__lean_Zhc_BlockAlgebra_Tracer_addCt(v___x_1603_, v_fst_1611_, v_snd_1612_, v___y_1601_);
lean_dec(v_fst_1611_);
if (lean_obj_tag(v___x_1613_) == 0)
{
lean_object* v_a_1614_; lean_object* v___x_1616_; 
v_a_1614_ = lean_ctor_get(v___x_1613_, 0);
lean_inc(v_a_1614_);
lean_dec_ref_known(v___x_1613_, 1);
if (v_isShared_1610_ == 0)
{
lean_ctor_set(v___x_1609_, 1, v_x_1600_);
lean_ctor_set(v___x_1609_, 0, v_a_1614_);
v___x_1616_ = v___x_1609_;
goto v_reusejp_1615_;
}
else
{
lean_object* v_reuseFailAlloc_1618_; 
v_reuseFailAlloc_1618_ = lean_alloc_ctor(1, 2, 0);
lean_ctor_set(v_reuseFailAlloc_1618_, 0, v_a_1614_);
lean_ctor_set(v_reuseFailAlloc_1618_, 1, v_x_1600_);
v___x_1616_ = v_reuseFailAlloc_1618_;
goto v_reusejp_1615_;
}
v_reusejp_1615_:
{
v_x_1599_ = v_tail_1607_;
v_x_1600_ = v___x_1616_;
goto _start;
}
}
else
{
lean_object* v_a_1619_; lean_object* v___x_1621_; uint8_t v_isShared_1622_; uint8_t v_isSharedCheck_1626_; 
lean_del_object(v___x_1609_);
lean_dec(v_tail_1607_);
lean_dec(v_x_1600_);
v_a_1619_ = lean_ctor_get(v___x_1613_, 0);
v_isSharedCheck_1626_ = !lean_is_exclusive(v___x_1613_);
if (v_isSharedCheck_1626_ == 0)
{
v___x_1621_ = v___x_1613_;
v_isShared_1622_ = v_isSharedCheck_1626_;
goto v_resetjp_1620_;
}
else
{
lean_inc(v_a_1619_);
lean_dec(v___x_1613_);
v___x_1621_ = lean_box(0);
v_isShared_1622_ = v_isSharedCheck_1626_;
goto v_resetjp_1620_;
}
v_resetjp_1620_:
{
lean_object* v___x_1624_; 
if (v_isShared_1622_ == 0)
{
v___x_1624_ = v___x_1621_;
goto v_reusejp_1623_;
}
else
{
lean_object* v_reuseFailAlloc_1625_; 
v_reuseFailAlloc_1625_ = lean_alloc_ctor(1, 1, 0);
lean_ctor_set(v_reuseFailAlloc_1625_, 0, v_a_1619_);
v___x_1624_ = v_reuseFailAlloc_1625_;
goto v_reusejp_1623_;
}
v_reusejp_1623_:
{
return v___x_1624_;
}
}
}
}
}
}
}
LEAN_EXPORT lean_object* lp_zhc__lean_List_mapM_loop___at___00Zhc_Iops_HillisSteele_propagate___at___00Zhc_Iops_addHillisSteele___at___00Zhc_Iops_addHillisSteeleCircuit_spec__0_spec__5_spec__12___boxed(lean_object* v_x_1628_, lean_object* v_x_1629_, lean_object* v___y_1630_, lean_object* v___y_1631_){
_start:
{
lean_object* v_res_1632_; 
v_res_1632_ = lp_zhc__lean_List_mapM_loop___at___00Zhc_Iops_HillisSteele_propagate___at___00Zhc_Iops_addHillisSteele___at___00Zhc_Iops_addHillisSteeleCircuit_spec__0_spec__5_spec__12(v_x_1628_, v_x_1629_, v___y_1630_);
lean_dec(v___y_1630_);
return v_res_1632_;
}
}
LEAN_EXPORT lean_object* lp_zhc__lean_Zhc_Iops_HillisSteele_propagate___at___00Zhc_Iops_addHillisSteele___at___00Zhc_Iops_addHillisSteeleCircuit_spec__0_spec__5(lean_object* v_sums_1633_, lean_object* v_carries_1634_, lean_object* v___y_1635_){
_start:
{
lean_object* v___x_1637_; 
v___x_1637_ = ((lean_object*)(lp_zhc__lean_Zhc_Iops_HillisSteele_extractPropGroupLut___closed__0));
if (lean_obj_tag(v_sums_1633_) == 0)
{
lean_object* v___x_1638_; 
lean_dec(v_carries_1634_);
v___x_1638_ = lean_alloc_ctor(0, 1, 0);
lean_ctor_set(v___x_1638_, 0, v_sums_1633_);
return v___x_1638_;
}
else
{
lean_object* v_head_1639_; lean_object* v_tail_1640_; lean_object* v___x_1642_; uint8_t v_isShared_1643_; uint8_t v_isSharedCheck_1664_; 
v_head_1639_ = lean_ctor_get(v_sums_1633_, 0);
v_tail_1640_ = lean_ctor_get(v_sums_1633_, 1);
v_isSharedCheck_1664_ = !lean_is_exclusive(v_sums_1633_);
if (v_isSharedCheck_1664_ == 0)
{
v___x_1642_ = v_sums_1633_;
v_isShared_1643_ = v_isSharedCheck_1664_;
goto v_resetjp_1641_;
}
else
{
lean_inc(v_tail_1640_);
lean_inc(v_head_1639_);
lean_dec(v_sums_1633_);
v___x_1642_ = lean_box(0);
v_isShared_1643_ = v_isSharedCheck_1664_;
goto v_resetjp_1641_;
}
v_resetjp_1641_:
{
lean_object* v___x_1644_; lean_object* v___x_1645_; 
v___x_1644_ = lp_zhc__lean_Zhc_Iops_carryMsgLut;
v___x_1645_ = lp_zhc__lean_Zhc_BlockAlgebra_Tracer_pbs2(v___x_1637_, v___x_1644_, v_head_1639_, v___y_1635_);
if (lean_obj_tag(v___x_1645_) == 0)
{
lean_object* v_a_1646_; lean_object* v_snd_1647_; lean_object* v___x_1648_; lean_object* v___x_1649_; lean_object* v___x_1650_; 
v_a_1646_ = lean_ctor_get(v___x_1645_, 0);
lean_inc(v_a_1646_);
lean_dec_ref_known(v___x_1645_, 1);
v_snd_1647_ = lean_ctor_get(v_a_1646_, 1);
lean_inc(v_snd_1647_);
lean_dec(v_a_1646_);
v___x_1648_ = l_List_zipWith___at___00List_zip_spec__0___redArg(v_tail_1640_, v_carries_1634_);
v___x_1649_ = lean_box(0);
v___x_1650_ = lp_zhc__lean_List_mapM_loop___at___00Zhc_Iops_HillisSteele_propagate___at___00Zhc_Iops_addHillisSteele___at___00Zhc_Iops_addHillisSteeleCircuit_spec__0_spec__5_spec__12(v___x_1648_, v___x_1649_, v___y_1635_);
if (lean_obj_tag(v___x_1650_) == 0)
{
lean_object* v_a_1651_; lean_object* v___x_1653_; 
v_a_1651_ = lean_ctor_get(v___x_1650_, 0);
lean_inc(v_a_1651_);
lean_dec_ref_known(v___x_1650_, 1);
if (v_isShared_1643_ == 0)
{
lean_ctor_set(v___x_1642_, 1, v_a_1651_);
lean_ctor_set(v___x_1642_, 0, v_snd_1647_);
v___x_1653_ = v___x_1642_;
goto v_reusejp_1652_;
}
else
{
lean_object* v_reuseFailAlloc_1655_; 
v_reuseFailAlloc_1655_ = lean_alloc_ctor(1, 2, 0);
lean_ctor_set(v_reuseFailAlloc_1655_, 0, v_snd_1647_);
lean_ctor_set(v_reuseFailAlloc_1655_, 1, v_a_1651_);
v___x_1653_ = v_reuseFailAlloc_1655_;
goto v_reusejp_1652_;
}
v_reusejp_1652_:
{
lean_object* v___x_1654_; 
v___x_1654_ = lp_zhc__lean_List_mapM_loop___at___00Zhc_Iops_HillisSteele_propagate___at___00Zhc_Iops_addHillisSteele___at___00Zhc_Iops_addHillisSteeleCircuit_spec__0_spec__5_spec__13(v___x_1653_, v___x_1649_, v___y_1635_);
return v___x_1654_;
}
}
else
{
lean_dec(v_snd_1647_);
lean_del_object(v___x_1642_);
return v___x_1650_;
}
}
else
{
lean_object* v_a_1656_; lean_object* v___x_1658_; uint8_t v_isShared_1659_; uint8_t v_isSharedCheck_1663_; 
lean_del_object(v___x_1642_);
lean_dec(v_tail_1640_);
lean_dec(v_carries_1634_);
v_a_1656_ = lean_ctor_get(v___x_1645_, 0);
v_isSharedCheck_1663_ = !lean_is_exclusive(v___x_1645_);
if (v_isSharedCheck_1663_ == 0)
{
v___x_1658_ = v___x_1645_;
v_isShared_1659_ = v_isSharedCheck_1663_;
goto v_resetjp_1657_;
}
else
{
lean_inc(v_a_1656_);
lean_dec(v___x_1645_);
v___x_1658_ = lean_box(0);
v_isShared_1659_ = v_isSharedCheck_1663_;
goto v_resetjp_1657_;
}
v_resetjp_1657_:
{
lean_object* v___x_1661_; 
if (v_isShared_1659_ == 0)
{
v___x_1661_ = v___x_1658_;
goto v_reusejp_1660_;
}
else
{
lean_object* v_reuseFailAlloc_1662_; 
v_reuseFailAlloc_1662_ = lean_alloc_ctor(1, 1, 0);
lean_ctor_set(v_reuseFailAlloc_1662_, 0, v_a_1656_);
v___x_1661_ = v_reuseFailAlloc_1662_;
goto v_reusejp_1660_;
}
v_reusejp_1660_:
{
return v___x_1661_;
}
}
}
}
}
}
}
LEAN_EXPORT lean_object* lp_zhc__lean_Zhc_Iops_HillisSteele_propagate___at___00Zhc_Iops_addHillisSteele___at___00Zhc_Iops_addHillisSteeleCircuit_spec__0_spec__5___boxed(lean_object* v_sums_1665_, lean_object* v_carries_1666_, lean_object* v___y_1667_, lean_object* v___y_1668_){
_start:
{
lean_object* v_res_1669_; 
v_res_1669_ = lp_zhc__lean_Zhc_Iops_HillisSteele_propagate___at___00Zhc_Iops_addHillisSteele___at___00Zhc_Iops_addHillisSteeleCircuit_spec__0_spec__5(v_sums_1665_, v_carries_1666_, v___y_1667_);
lean_dec(v___y_1667_);
return v_res_1669_;
}
}
LEAN_EXPORT lean_object* lp_zhc__lean_List_forIn_x27_loop___at___00Zhc_Iops_HillisSteele_joinBlocks___at___00Zhc_Iops_addHillisSteele___at___00Zhc_Iops_addHillisSteeleCircuit_spec__0_spec__6_spec__15___redArg(lean_object* v_as_x27_1670_, lean_object* v_b_1671_, lean_object* v___y_1672_){
_start:
{
if (lean_obj_tag(v_as_x27_1670_) == 0)
{
lean_object* v___x_1674_; 
v___x_1674_ = lean_alloc_ctor(0, 1, 0);
lean_ctor_set(v___x_1674_, 0, v_b_1671_);
return v___x_1674_;
}
else
{
lean_object* v_head_1675_; lean_object* v_tail_1676_; lean_object* v_fst_1677_; lean_object* v_snd_1678_; lean_object* v___x_1679_; 
v_head_1675_ = lean_ctor_get(v_as_x27_1670_, 0);
v_tail_1676_ = lean_ctor_get(v_as_x27_1670_, 1);
v_fst_1677_ = lean_ctor_get(v_head_1675_, 0);
v_snd_1678_ = lean_ctor_get(v_head_1675_, 1);
lean_inc(v_fst_1677_);
lean_inc(v_snd_1678_);
v___x_1679_ = lp_zhc__lean_Zhc_BlockAlgebra_Tracer_storeCtBlock___redArg(v_b_1671_, v_snd_1678_, v_fst_1677_, v___y_1672_);
if (lean_obj_tag(v___x_1679_) == 0)
{
lean_object* v_a_1680_; 
v_a_1680_ = lean_ctor_get(v___x_1679_, 0);
lean_inc(v_a_1680_);
lean_dec_ref_known(v___x_1679_, 1);
v_as_x27_1670_ = v_tail_1676_;
v_b_1671_ = v_a_1680_;
goto _start;
}
else
{
return v___x_1679_;
}
}
}
}
LEAN_EXPORT lean_object* lp_zhc__lean_List_forIn_x27_loop___at___00Zhc_Iops_HillisSteele_joinBlocks___at___00Zhc_Iops_addHillisSteele___at___00Zhc_Iops_addHillisSteeleCircuit_spec__0_spec__6_spec__15___redArg___boxed(lean_object* v_as_x27_1682_, lean_object* v_b_1683_, lean_object* v___y_1684_, lean_object* v___y_1685_){
_start:
{
lean_object* v_res_1686_; 
v_res_1686_ = lp_zhc__lean_List_forIn_x27_loop___at___00Zhc_Iops_HillisSteele_joinBlocks___at___00Zhc_Iops_addHillisSteele___at___00Zhc_Iops_addHillisSteeleCircuit_spec__0_spec__6_spec__15___redArg(v_as_x27_1682_, v_b_1683_, v___y_1684_);
lean_dec(v___y_1684_);
lean_dec(v_as_x27_1682_);
return v_res_1686_;
}
}
LEAN_EXPORT lean_object* lp_zhc__lean_Zhc_Iops_HillisSteele_joinBlocks___at___00Zhc_Iops_addHillisSteele___at___00Zhc_Iops_addHillisSteeleCircuit_spec__0_spec__6(lean_object* v_si_1687_, lean_object* v_blocks_1688_, lean_object* v___y_1689_){
_start:
{
lean_object* v___x_1691_; lean_object* v___x_1692_; 
v___x_1691_ = ((lean_object*)(lp_zhc__lean_Zhc_Iops_HillisSteele_extractPropGroupLut___closed__0));
v___x_1692_ = lp_zhc__lean_Zhc_BlockAlgebra_Tracer_declareInteger(v___x_1691_, v_si_1687_, v___y_1689_);
if (lean_obj_tag(v___x_1692_) == 0)
{
lean_object* v_a_1693_; lean_object* v___x_1694_; lean_object* v___x_1695_; lean_object* v___x_1696_; 
v_a_1693_ = lean_ctor_get(v___x_1692_, 0);
lean_inc(v_a_1693_);
lean_dec_ref_known(v___x_1692_, 1);
v___x_1694_ = l_List_finRange(v_si_1687_);
v___x_1695_ = l_List_zipWith___at___00List_zip_spec__0___redArg(v_blocks_1688_, v___x_1694_);
v___x_1696_ = lp_zhc__lean_List_forIn_x27_loop___at___00Zhc_Iops_HillisSteele_joinBlocks___at___00Zhc_Iops_addHillisSteele___at___00Zhc_Iops_addHillisSteeleCircuit_spec__0_spec__6_spec__15___redArg(v___x_1695_, v_a_1693_, v___y_1689_);
lean_dec(v___x_1695_);
return v___x_1696_;
}
else
{
lean_dec(v_blocks_1688_);
lean_dec(v_si_1687_);
return v___x_1692_;
}
}
}
LEAN_EXPORT lean_object* lp_zhc__lean_Zhc_Iops_HillisSteele_joinBlocks___at___00Zhc_Iops_addHillisSteele___at___00Zhc_Iops_addHillisSteeleCircuit_spec__0_spec__6___boxed(lean_object* v_si_1697_, lean_object* v_blocks_1698_, lean_object* v___y_1699_, lean_object* v___y_1700_){
_start:
{
lean_object* v_res_1701_; 
v_res_1701_ = lp_zhc__lean_Zhc_Iops_HillisSteele_joinBlocks___at___00Zhc_Iops_addHillisSteele___at___00Zhc_Iops_addHillisSteeleCircuit_spec__0_spec__6(v_si_1697_, v_blocks_1698_, v___y_1699_);
lean_dec(v___y_1699_);
return v_res_1701_;
}
}
LEAN_EXPORT lean_object* lp_zhc__lean_List_mapM_loop___at___00Zhc_Iops_HillisSteele_groupStates___at___00Zhc_Iops_addHillisSteele___at___00Zhc_Iops_addHillisSteeleCircuit_spec__0_spec__2_spec__5___lam__0(lean_object* v_a_1702_, lean_object* v_a_1703_, lean_object* v_fst_1704_, lean_object* v_b3_1705_, lean_object* v___y_1706_){
_start:
{
lean_object* v___x_1708_; lean_object* v___x_1709_; lean_object* v___x_1710_; lean_object* v___x_1711_; 
v___x_1708_ = lean_alloc_ctor(0, 2, 0);
lean_ctor_set(v___x_1708_, 0, v_a_1702_);
lean_ctor_set(v___x_1708_, 1, v_b3_1705_);
v___x_1709_ = lean_alloc_ctor(0, 2, 0);
lean_ctor_set(v___x_1709_, 0, v_a_1703_);
lean_ctor_set(v___x_1709_, 1, v___x_1708_);
v___x_1710_ = lean_alloc_ctor(0, 2, 0);
lean_ctor_set(v___x_1710_, 0, v_fst_1704_);
lean_ctor_set(v___x_1710_, 1, v___x_1709_);
v___x_1711_ = lean_alloc_ctor(0, 1, 0);
lean_ctor_set(v___x_1711_, 0, v___x_1710_);
return v___x_1711_;
}
}
LEAN_EXPORT lean_object* lp_zhc__lean_List_mapM_loop___at___00Zhc_Iops_HillisSteele_groupStates___at___00Zhc_Iops_addHillisSteele___at___00Zhc_Iops_addHillisSteeleCircuit_spec__0_spec__2_spec__5___lam__0___boxed(lean_object* v_a_1712_, lean_object* v_a_1713_, lean_object* v_fst_1714_, lean_object* v_b3_1715_, lean_object* v___y_1716_, lean_object* v___y_1717_){
_start:
{
lean_object* v_res_1718_; 
v_res_1718_ = lp_zhc__lean_List_mapM_loop___at___00Zhc_Iops_HillisSteele_groupStates___at___00Zhc_Iops_addHillisSteele___at___00Zhc_Iops_addHillisSteeleCircuit_spec__0_spec__2_spec__5___lam__0(v_a_1712_, v_a_1713_, v_fst_1714_, v_b3_1715_, v___y_1716_);
lean_dec(v___y_1716_);
return v_res_1718_;
}
}
LEAN_EXPORT lean_object* lp_zhc__lean_List_mapM_loop___at___00Zhc_Iops_HillisSteele_groupStates___at___00Zhc_Iops_addHillisSteele___at___00Zhc_Iops_addHillisSteeleCircuit_spec__0_spec__2_spec__5(lean_object* v_x_1719_, lean_object* v_x_1720_, lean_object* v___y_1721_){
_start:
{
lean_object* v___x_1723_; 
v___x_1723_ = ((lean_object*)(lp_zhc__lean_Zhc_Iops_HillisSteele_extractPropGroupLut___closed__0));
if (lean_obj_tag(v_x_1719_) == 0)
{
lean_object* v___x_1724_; lean_object* v___x_1725_; 
v___x_1724_ = l_List_reverse___redArg(v_x_1720_);
v___x_1725_ = lean_alloc_ctor(0, 1, 0);
lean_ctor_set(v___x_1725_, 0, v___x_1724_);
return v___x_1725_;
}
else
{
lean_object* v_head_1726_; lean_object* v_tail_1727_; lean_object* v___x_1729_; uint8_t v_isShared_1730_; uint8_t v_isSharedCheck_1823_; 
v_head_1726_ = lean_ctor_get(v_x_1719_, 0);
v_tail_1727_ = lean_ctor_get(v_x_1719_, 1);
v_isSharedCheck_1823_ = !lean_is_exclusive(v_x_1719_);
if (v_isSharedCheck_1823_ == 0)
{
v___x_1729_ = v_x_1719_;
v_isShared_1730_ = v_isSharedCheck_1823_;
goto v_resetjp_1728_;
}
else
{
lean_inc(v_tail_1727_);
lean_inc(v_head_1726_);
lean_dec(v_x_1719_);
v___x_1729_ = lean_box(0);
v_isShared_1730_ = v_isSharedCheck_1823_;
goto v_resetjp_1728_;
}
v_resetjp_1728_:
{
lean_object* v___y_1732_; lean_object* v_fst_1738_; lean_object* v_snd_1739_; lean_object* v_snd_1740_; lean_object* v_snd_1741_; lean_object* v_fst_1742_; lean_object* v_fst_1743_; lean_object* v_fst_1744_; lean_object* v_snd_1745_; lean_object* v___x_1746_; 
v_fst_1738_ = lean_ctor_get(v_head_1726_, 0);
lean_inc(v_fst_1738_);
v_snd_1739_ = lean_ctor_get(v_fst_1738_, 1);
lean_inc(v_snd_1739_);
v_snd_1740_ = lean_ctor_get(v_snd_1739_, 1);
lean_inc(v_snd_1740_);
v_snd_1741_ = lean_ctor_get(v_head_1726_, 1);
lean_inc(v_snd_1741_);
lean_dec(v_head_1726_);
v_fst_1742_ = lean_ctor_get(v_fst_1738_, 0);
lean_inc(v_fst_1742_);
lean_dec(v_fst_1738_);
v_fst_1743_ = lean_ctor_get(v_snd_1739_, 0);
lean_inc(v_fst_1743_);
lean_dec(v_snd_1739_);
v_fst_1744_ = lean_ctor_get(v_snd_1740_, 0);
lean_inc(v_fst_1744_);
v_snd_1745_ = lean_ctor_get(v_snd_1740_, 1);
lean_inc(v_snd_1745_);
lean_dec(v_snd_1740_);
v___x_1746_ = lp_zhc__lean_Zhc_BlockAlgebra_Tracer_addCt(v___x_1723_, v_fst_1742_, v_fst_1743_, v___y_1721_);
if (lean_obj_tag(v___x_1746_) == 0)
{
lean_object* v_a_1747_; lean_object* v___x_1748_; 
v_a_1747_ = lean_ctor_get(v___x_1746_, 0);
lean_inc(v_a_1747_);
lean_dec_ref_known(v___x_1746_, 1);
v___x_1748_ = lp_zhc__lean_Zhc_BlockAlgebra_Tracer_addCt(v___x_1723_, v_a_1747_, v_fst_1744_, v___y_1721_);
if (lean_obj_tag(v___x_1748_) == 0)
{
lean_object* v_a_1749_; lean_object* v___x_1750_; 
v_a_1749_ = lean_ctor_get(v___x_1748_, 0);
lean_inc(v_a_1749_);
lean_dec_ref_known(v___x_1748_, 1);
v___x_1750_ = lp_zhc__lean_Zhc_BlockAlgebra_Tracer_addCt(v___x_1723_, v_a_1749_, v_snd_1745_, v___y_1721_);
if (lean_obj_tag(v___x_1750_) == 0)
{
lean_object* v_a_1751_; lean_object* v___x_1752_; uint8_t v___x_1753_; 
v_a_1751_ = lean_ctor_get(v___x_1750_, 0);
lean_inc(v_a_1751_);
lean_dec_ref_known(v___x_1750_, 1);
v___x_1752_ = lean_unsigned_to_nat(0u);
v___x_1753_ = lean_nat_dec_eq(v_snd_1741_, v___x_1752_);
lean_dec(v_snd_1741_);
if (v___x_1753_ == 0)
{
lean_object* v___x_1754_; lean_object* v___x_1755_; 
v___x_1754_ = lp_zhc__lean_Zhc_Iops_HillisSteele_reduceCarryPadLut;
v___x_1755_ = lp_zhc__lean_Zhc_BlockAlgebra_Tracer_pbs(v___x_1723_, v___x_1754_, v_a_1751_, v___y_1721_);
if (lean_obj_tag(v___x_1755_) == 0)
{
lean_object* v_a_1756_; lean_object* v___x_1757_; lean_object* v___x_1758_; 
v_a_1756_ = lean_ctor_get(v___x_1755_, 0);
lean_inc(v_a_1756_);
lean_dec_ref_known(v___x_1755_, 1);
v___x_1757_ = lean_unsigned_to_nat(1u);
v___x_1758_ = lp_zhc__lean_Zhc_BlockAlgebra_Tracer_letPtBlock(v___x_1723_, v___x_1757_, v___y_1721_);
if (lean_obj_tag(v___x_1758_) == 0)
{
lean_object* v_a_1759_; lean_object* v___x_1760_; 
v_a_1759_ = lean_ctor_get(v___x_1758_, 0);
lean_inc(v_a_1759_);
lean_dec_ref_known(v___x_1758_, 1);
v___x_1760_ = lp_zhc__lean_Zhc_BlockAlgebra_Tracer_addPt(v___x_1723_, v_a_1756_, v_a_1759_, v___y_1721_);
lean_dec(v_a_1756_);
if (lean_obj_tag(v___x_1760_) == 0)
{
lean_object* v_a_1761_; lean_object* v___x_1762_; 
v_a_1761_ = lean_ctor_get(v___x_1760_, 0);
lean_inc(v_a_1761_);
lean_dec_ref_known(v___x_1760_, 1);
v___x_1762_ = lp_zhc__lean_List_mapM_loop___at___00Zhc_Iops_HillisSteele_groupStates___at___00Zhc_Iops_addHillisSteele___at___00Zhc_Iops_addHillisSteeleCircuit_spec__0_spec__2_spec__5___lam__0(v_a_1749_, v_a_1747_, v_fst_1742_, v_a_1761_, v___y_1721_);
v___y_1732_ = v___x_1762_;
goto v___jp_1731_;
}
else
{
lean_object* v_a_1763_; lean_object* v___x_1765_; uint8_t v_isShared_1766_; uint8_t v_isSharedCheck_1770_; 
lean_dec(v_a_1749_);
lean_dec(v_a_1747_);
lean_dec(v_fst_1742_);
lean_del_object(v___x_1729_);
lean_dec(v_tail_1727_);
lean_dec(v_x_1720_);
v_a_1763_ = lean_ctor_get(v___x_1760_, 0);
v_isSharedCheck_1770_ = !lean_is_exclusive(v___x_1760_);
if (v_isSharedCheck_1770_ == 0)
{
v___x_1765_ = v___x_1760_;
v_isShared_1766_ = v_isSharedCheck_1770_;
goto v_resetjp_1764_;
}
else
{
lean_inc(v_a_1763_);
lean_dec(v___x_1760_);
v___x_1765_ = lean_box(0);
v_isShared_1766_ = v_isSharedCheck_1770_;
goto v_resetjp_1764_;
}
v_resetjp_1764_:
{
lean_object* v___x_1768_; 
if (v_isShared_1766_ == 0)
{
v___x_1768_ = v___x_1765_;
goto v_reusejp_1767_;
}
else
{
lean_object* v_reuseFailAlloc_1769_; 
v_reuseFailAlloc_1769_ = lean_alloc_ctor(1, 1, 0);
lean_ctor_set(v_reuseFailAlloc_1769_, 0, v_a_1763_);
v___x_1768_ = v_reuseFailAlloc_1769_;
goto v_reusejp_1767_;
}
v_reusejp_1767_:
{
return v___x_1768_;
}
}
}
}
else
{
lean_object* v_a_1771_; lean_object* v___x_1773_; uint8_t v_isShared_1774_; uint8_t v_isSharedCheck_1778_; 
lean_dec(v_a_1756_);
lean_dec(v_a_1749_);
lean_dec(v_a_1747_);
lean_dec(v_fst_1742_);
lean_del_object(v___x_1729_);
lean_dec(v_tail_1727_);
lean_dec(v_x_1720_);
v_a_1771_ = lean_ctor_get(v___x_1758_, 0);
v_isSharedCheck_1778_ = !lean_is_exclusive(v___x_1758_);
if (v_isSharedCheck_1778_ == 0)
{
v___x_1773_ = v___x_1758_;
v_isShared_1774_ = v_isSharedCheck_1778_;
goto v_resetjp_1772_;
}
else
{
lean_inc(v_a_1771_);
lean_dec(v___x_1758_);
v___x_1773_ = lean_box(0);
v_isShared_1774_ = v_isSharedCheck_1778_;
goto v_resetjp_1772_;
}
v_resetjp_1772_:
{
lean_object* v___x_1776_; 
if (v_isShared_1774_ == 0)
{
v___x_1776_ = v___x_1773_;
goto v_reusejp_1775_;
}
else
{
lean_object* v_reuseFailAlloc_1777_; 
v_reuseFailAlloc_1777_ = lean_alloc_ctor(1, 1, 0);
lean_ctor_set(v_reuseFailAlloc_1777_, 0, v_a_1771_);
v___x_1776_ = v_reuseFailAlloc_1777_;
goto v_reusejp_1775_;
}
v_reusejp_1775_:
{
return v___x_1776_;
}
}
}
}
else
{
lean_object* v_a_1779_; lean_object* v___x_1781_; uint8_t v_isShared_1782_; uint8_t v_isSharedCheck_1786_; 
lean_dec(v_a_1749_);
lean_dec(v_a_1747_);
lean_dec(v_fst_1742_);
lean_del_object(v___x_1729_);
lean_dec(v_tail_1727_);
lean_dec(v_x_1720_);
v_a_1779_ = lean_ctor_get(v___x_1755_, 0);
v_isSharedCheck_1786_ = !lean_is_exclusive(v___x_1755_);
if (v_isSharedCheck_1786_ == 0)
{
v___x_1781_ = v___x_1755_;
v_isShared_1782_ = v_isSharedCheck_1786_;
goto v_resetjp_1780_;
}
else
{
lean_inc(v_a_1779_);
lean_dec(v___x_1755_);
v___x_1781_ = lean_box(0);
v_isShared_1782_ = v_isSharedCheck_1786_;
goto v_resetjp_1780_;
}
v_resetjp_1780_:
{
lean_object* v___x_1784_; 
if (v_isShared_1782_ == 0)
{
v___x_1784_ = v___x_1781_;
goto v_reusejp_1783_;
}
else
{
lean_object* v_reuseFailAlloc_1785_; 
v_reuseFailAlloc_1785_ = lean_alloc_ctor(1, 1, 0);
lean_ctor_set(v_reuseFailAlloc_1785_, 0, v_a_1779_);
v___x_1784_ = v_reuseFailAlloc_1785_;
goto v_reusejp_1783_;
}
v_reusejp_1783_:
{
return v___x_1784_;
}
}
}
}
else
{
lean_object* v___x_1787_; lean_object* v___x_1788_; 
v___x_1787_ = lean_obj_once(&lp_zhc__lean_Zhc_Iops_HillisSteele_groupStates___redArg___lam__5___closed__1, &lp_zhc__lean_Zhc_Iops_HillisSteele_groupStates___redArg___lam__5___closed__1_once, _init_lp_zhc__lean_Zhc_Iops_HillisSteele_groupStates___redArg___lam__5___closed__1);
v___x_1788_ = lp_zhc__lean_Zhc_BlockAlgebra_Tracer_pbs(v___x_1723_, v___x_1787_, v_a_1751_, v___y_1721_);
if (lean_obj_tag(v___x_1788_) == 0)
{
lean_object* v_a_1789_; lean_object* v___x_1790_; 
v_a_1789_ = lean_ctor_get(v___x_1788_, 0);
lean_inc(v_a_1789_);
lean_dec_ref_known(v___x_1788_, 1);
v___x_1790_ = lp_zhc__lean_List_mapM_loop___at___00Zhc_Iops_HillisSteele_groupStates___at___00Zhc_Iops_addHillisSteele___at___00Zhc_Iops_addHillisSteeleCircuit_spec__0_spec__2_spec__5___lam__0(v_a_1749_, v_a_1747_, v_fst_1742_, v_a_1789_, v___y_1721_);
v___y_1732_ = v___x_1790_;
goto v___jp_1731_;
}
else
{
lean_object* v_a_1791_; lean_object* v___x_1793_; uint8_t v_isShared_1794_; uint8_t v_isSharedCheck_1798_; 
lean_dec(v_a_1749_);
lean_dec(v_a_1747_);
lean_dec(v_fst_1742_);
lean_del_object(v___x_1729_);
lean_dec(v_tail_1727_);
lean_dec(v_x_1720_);
v_a_1791_ = lean_ctor_get(v___x_1788_, 0);
v_isSharedCheck_1798_ = !lean_is_exclusive(v___x_1788_);
if (v_isSharedCheck_1798_ == 0)
{
v___x_1793_ = v___x_1788_;
v_isShared_1794_ = v_isSharedCheck_1798_;
goto v_resetjp_1792_;
}
else
{
lean_inc(v_a_1791_);
lean_dec(v___x_1788_);
v___x_1793_ = lean_box(0);
v_isShared_1794_ = v_isSharedCheck_1798_;
goto v_resetjp_1792_;
}
v_resetjp_1792_:
{
lean_object* v___x_1796_; 
if (v_isShared_1794_ == 0)
{
v___x_1796_ = v___x_1793_;
goto v_reusejp_1795_;
}
else
{
lean_object* v_reuseFailAlloc_1797_; 
v_reuseFailAlloc_1797_ = lean_alloc_ctor(1, 1, 0);
lean_ctor_set(v_reuseFailAlloc_1797_, 0, v_a_1791_);
v___x_1796_ = v_reuseFailAlloc_1797_;
goto v_reusejp_1795_;
}
v_reusejp_1795_:
{
return v___x_1796_;
}
}
}
}
}
else
{
lean_object* v_a_1799_; lean_object* v___x_1801_; uint8_t v_isShared_1802_; uint8_t v_isSharedCheck_1806_; 
lean_dec(v_a_1749_);
lean_dec(v_a_1747_);
lean_dec(v_fst_1742_);
lean_dec(v_snd_1741_);
lean_del_object(v___x_1729_);
lean_dec(v_tail_1727_);
lean_dec(v_x_1720_);
v_a_1799_ = lean_ctor_get(v___x_1750_, 0);
v_isSharedCheck_1806_ = !lean_is_exclusive(v___x_1750_);
if (v_isSharedCheck_1806_ == 0)
{
v___x_1801_ = v___x_1750_;
v_isShared_1802_ = v_isSharedCheck_1806_;
goto v_resetjp_1800_;
}
else
{
lean_inc(v_a_1799_);
lean_dec(v___x_1750_);
v___x_1801_ = lean_box(0);
v_isShared_1802_ = v_isSharedCheck_1806_;
goto v_resetjp_1800_;
}
v_resetjp_1800_:
{
lean_object* v___x_1804_; 
if (v_isShared_1802_ == 0)
{
v___x_1804_ = v___x_1801_;
goto v_reusejp_1803_;
}
else
{
lean_object* v_reuseFailAlloc_1805_; 
v_reuseFailAlloc_1805_ = lean_alloc_ctor(1, 1, 0);
lean_ctor_set(v_reuseFailAlloc_1805_, 0, v_a_1799_);
v___x_1804_ = v_reuseFailAlloc_1805_;
goto v_reusejp_1803_;
}
v_reusejp_1803_:
{
return v___x_1804_;
}
}
}
}
else
{
lean_object* v_a_1807_; lean_object* v___x_1809_; uint8_t v_isShared_1810_; uint8_t v_isSharedCheck_1814_; 
lean_dec(v_a_1747_);
lean_dec(v_snd_1745_);
lean_dec(v_fst_1742_);
lean_dec(v_snd_1741_);
lean_del_object(v___x_1729_);
lean_dec(v_tail_1727_);
lean_dec(v_x_1720_);
v_a_1807_ = lean_ctor_get(v___x_1748_, 0);
v_isSharedCheck_1814_ = !lean_is_exclusive(v___x_1748_);
if (v_isSharedCheck_1814_ == 0)
{
v___x_1809_ = v___x_1748_;
v_isShared_1810_ = v_isSharedCheck_1814_;
goto v_resetjp_1808_;
}
else
{
lean_inc(v_a_1807_);
lean_dec(v___x_1748_);
v___x_1809_ = lean_box(0);
v_isShared_1810_ = v_isSharedCheck_1814_;
goto v_resetjp_1808_;
}
v_resetjp_1808_:
{
lean_object* v___x_1812_; 
if (v_isShared_1810_ == 0)
{
v___x_1812_ = v___x_1809_;
goto v_reusejp_1811_;
}
else
{
lean_object* v_reuseFailAlloc_1813_; 
v_reuseFailAlloc_1813_ = lean_alloc_ctor(1, 1, 0);
lean_ctor_set(v_reuseFailAlloc_1813_, 0, v_a_1807_);
v___x_1812_ = v_reuseFailAlloc_1813_;
goto v_reusejp_1811_;
}
v_reusejp_1811_:
{
return v___x_1812_;
}
}
}
}
else
{
lean_object* v_a_1815_; lean_object* v___x_1817_; uint8_t v_isShared_1818_; uint8_t v_isSharedCheck_1822_; 
lean_dec(v_snd_1745_);
lean_dec(v_fst_1744_);
lean_dec(v_fst_1742_);
lean_dec(v_snd_1741_);
lean_del_object(v___x_1729_);
lean_dec(v_tail_1727_);
lean_dec(v_x_1720_);
v_a_1815_ = lean_ctor_get(v___x_1746_, 0);
v_isSharedCheck_1822_ = !lean_is_exclusive(v___x_1746_);
if (v_isSharedCheck_1822_ == 0)
{
v___x_1817_ = v___x_1746_;
v_isShared_1818_ = v_isSharedCheck_1822_;
goto v_resetjp_1816_;
}
else
{
lean_inc(v_a_1815_);
lean_dec(v___x_1746_);
v___x_1817_ = lean_box(0);
v_isShared_1818_ = v_isSharedCheck_1822_;
goto v_resetjp_1816_;
}
v_resetjp_1816_:
{
lean_object* v___x_1820_; 
if (v_isShared_1818_ == 0)
{
v___x_1820_ = v___x_1817_;
goto v_reusejp_1819_;
}
else
{
lean_object* v_reuseFailAlloc_1821_; 
v_reuseFailAlloc_1821_ = lean_alloc_ctor(1, 1, 0);
lean_ctor_set(v_reuseFailAlloc_1821_, 0, v_a_1815_);
v___x_1820_ = v_reuseFailAlloc_1821_;
goto v_reusejp_1819_;
}
v_reusejp_1819_:
{
return v___x_1820_;
}
}
}
v___jp_1731_:
{
lean_object* v_a_1733_; lean_object* v___x_1735_; 
v_a_1733_ = lean_ctor_get(v___y_1732_, 0);
lean_inc(v_a_1733_);
lean_dec_ref(v___y_1732_);
if (v_isShared_1730_ == 0)
{
lean_ctor_set(v___x_1729_, 1, v_x_1720_);
lean_ctor_set(v___x_1729_, 0, v_a_1733_);
v___x_1735_ = v___x_1729_;
goto v_reusejp_1734_;
}
else
{
lean_object* v_reuseFailAlloc_1737_; 
v_reuseFailAlloc_1737_ = lean_alloc_ctor(1, 2, 0);
lean_ctor_set(v_reuseFailAlloc_1737_, 0, v_a_1733_);
lean_ctor_set(v_reuseFailAlloc_1737_, 1, v_x_1720_);
v___x_1735_ = v_reuseFailAlloc_1737_;
goto v_reusejp_1734_;
}
v_reusejp_1734_:
{
v_x_1719_ = v_tail_1727_;
v_x_1720_ = v___x_1735_;
goto _start;
}
}
}
}
}
}
LEAN_EXPORT lean_object* lp_zhc__lean_List_mapM_loop___at___00Zhc_Iops_HillisSteele_groupStates___at___00Zhc_Iops_addHillisSteele___at___00Zhc_Iops_addHillisSteeleCircuit_spec__0_spec__2_spec__5___boxed(lean_object* v_x_1824_, lean_object* v_x_1825_, lean_object* v___y_1826_, lean_object* v___y_1827_){
_start:
{
lean_object* v_res_1828_; 
v_res_1828_ = lp_zhc__lean_List_mapM_loop___at___00Zhc_Iops_HillisSteele_groupStates___at___00Zhc_Iops_addHillisSteele___at___00Zhc_Iops_addHillisSteeleCircuit_spec__0_spec__2_spec__5(v_x_1824_, v_x_1825_, v___y_1826_);
lean_dec(v___y_1826_);
return v_res_1828_;
}
}
LEAN_EXPORT lean_object* lp_zhc__lean_Zhc_Iops_HillisSteele_groupStates___at___00Zhc_Iops_addHillisSteele___at___00Zhc_Iops_addHillisSteeleCircuit_spec__0_spec__2(lean_object* v_states_1829_, lean_object* v___y_1830_){
_start:
{
lean_object* v___x_1832_; lean_object* v___x_1833_; lean_object* v___x_1834_; lean_object* v___x_1835_; lean_object* v___x_1836_; 
v___x_1832_ = lp_zhc__lean_Zhc_Iops_HillisSteele_chunk4___redArg(v_states_1829_);
v___x_1833_ = lean_unsigned_to_nat(0u);
v___x_1834_ = l_List_zipIdxTR___redArg(v___x_1832_, v___x_1833_);
v___x_1835_ = lean_box(0);
v___x_1836_ = lp_zhc__lean_List_mapM_loop___at___00Zhc_Iops_HillisSteele_groupStates___at___00Zhc_Iops_addHillisSteele___at___00Zhc_Iops_addHillisSteeleCircuit_spec__0_spec__2_spec__5(v___x_1834_, v___x_1835_, v___y_1830_);
if (lean_obj_tag(v___x_1836_) == 0)
{
lean_object* v_a_1837_; lean_object* v___x_1839_; uint8_t v_isShared_1840_; uint8_t v_isSharedCheck_1845_; 
v_a_1837_ = lean_ctor_get(v___x_1836_, 0);
v_isSharedCheck_1845_ = !lean_is_exclusive(v___x_1836_);
if (v_isSharedCheck_1845_ == 0)
{
v___x_1839_ = v___x_1836_;
v_isShared_1840_ = v_isSharedCheck_1845_;
goto v_resetjp_1838_;
}
else
{
lean_inc(v_a_1837_);
lean_dec(v___x_1836_);
v___x_1839_ = lean_box(0);
v_isShared_1840_ = v_isSharedCheck_1845_;
goto v_resetjp_1838_;
}
v_resetjp_1838_:
{
lean_object* v___x_1841_; lean_object* v___x_1843_; 
v___x_1841_ = lp_zhc__lean_Zhc_Iops_HillisSteele_unchunk4___redArg(v_a_1837_);
if (v_isShared_1840_ == 0)
{
lean_ctor_set(v___x_1839_, 0, v___x_1841_);
v___x_1843_ = v___x_1839_;
goto v_reusejp_1842_;
}
else
{
lean_object* v_reuseFailAlloc_1844_; 
v_reuseFailAlloc_1844_ = lean_alloc_ctor(0, 1, 0);
lean_ctor_set(v_reuseFailAlloc_1844_, 0, v___x_1841_);
v___x_1843_ = v_reuseFailAlloc_1844_;
goto v_reusejp_1842_;
}
v_reusejp_1842_:
{
return v___x_1843_;
}
}
}
else
{
lean_object* v_a_1846_; lean_object* v___x_1848_; uint8_t v_isShared_1849_; uint8_t v_isSharedCheck_1853_; 
v_a_1846_ = lean_ctor_get(v___x_1836_, 0);
v_isSharedCheck_1853_ = !lean_is_exclusive(v___x_1836_);
if (v_isSharedCheck_1853_ == 0)
{
v___x_1848_ = v___x_1836_;
v_isShared_1849_ = v_isSharedCheck_1853_;
goto v_resetjp_1847_;
}
else
{
lean_inc(v_a_1846_);
lean_dec(v___x_1836_);
v___x_1848_ = lean_box(0);
v_isShared_1849_ = v_isSharedCheck_1853_;
goto v_resetjp_1847_;
}
v_resetjp_1847_:
{
lean_object* v___x_1851_; 
if (v_isShared_1849_ == 0)
{
v___x_1851_ = v___x_1848_;
goto v_reusejp_1850_;
}
else
{
lean_object* v_reuseFailAlloc_1852_; 
v_reuseFailAlloc_1852_ = lean_alloc_ctor(1, 1, 0);
lean_ctor_set(v_reuseFailAlloc_1852_, 0, v_a_1846_);
v___x_1851_ = v_reuseFailAlloc_1852_;
goto v_reusejp_1850_;
}
v_reusejp_1850_:
{
return v___x_1851_;
}
}
}
}
}
LEAN_EXPORT lean_object* lp_zhc__lean_Zhc_Iops_HillisSteele_groupStates___at___00Zhc_Iops_addHillisSteele___at___00Zhc_Iops_addHillisSteeleCircuit_spec__0_spec__2___boxed(lean_object* v_states_1854_, lean_object* v___y_1855_, lean_object* v___y_1856_){
_start:
{
lean_object* v_res_1857_; 
v_res_1857_ = lp_zhc__lean_Zhc_Iops_HillisSteele_groupStates___at___00Zhc_Iops_addHillisSteele___at___00Zhc_Iops_addHillisSteeleCircuit_spec__0_spec__2(v_states_1854_, v___y_1855_);
lean_dec(v___y_1855_);
return v_res_1857_;
}
}
LEAN_EXPORT lean_object* lp_zhc__lean_List_mapM_loop___at___00Zhc_Iops_HillisSteele_blockStates___at___00Zhc_Iops_addHillisSteele___at___00Zhc_Iops_addHillisSteeleCircuit_spec__0_spec__1_spec__3(lean_object* v_x_1858_, lean_object* v_x_1859_, lean_object* v___y_1860_){
_start:
{
lean_object* v___x_1862_; 
v___x_1862_ = ((lean_object*)(lp_zhc__lean_Zhc_Iops_HillisSteele_extractPropGroupLut___closed__0));
if (lean_obj_tag(v_x_1858_) == 0)
{
lean_object* v___x_1863_; lean_object* v___x_1864_; 
v___x_1863_ = l_List_reverse___redArg(v_x_1859_);
v___x_1864_ = lean_alloc_ctor(0, 1, 0);
lean_ctor_set(v___x_1864_, 0, v___x_1863_);
return v___x_1864_;
}
else
{
lean_object* v_head_1865_; lean_object* v_tail_1866_; lean_object* v___x_1868_; uint8_t v_isShared_1869_; uint8_t v_isSharedCheck_1913_; 
v_head_1865_ = lean_ctor_get(v_x_1858_, 0);
v_tail_1866_ = lean_ctor_get(v_x_1858_, 1);
v_isSharedCheck_1913_ = !lean_is_exclusive(v_x_1858_);
if (v_isSharedCheck_1913_ == 0)
{
v___x_1868_ = v_x_1858_;
v_isShared_1869_ = v_isSharedCheck_1913_;
goto v_resetjp_1867_;
}
else
{
lean_inc(v_tail_1866_);
lean_inc(v_head_1865_);
lean_dec(v_x_1858_);
v___x_1868_ = lean_box(0);
v_isShared_1869_ = v_isSharedCheck_1913_;
goto v_resetjp_1867_;
}
v_resetjp_1867_:
{
lean_object* v_a_1871_; lean_object* v___y_1877_; lean_object* v_fst_1887_; lean_object* v_snd_1888_; lean_object* v___x_1889_; uint8_t v___x_1890_; 
v_fst_1887_ = lean_ctor_get(v_head_1865_, 0);
lean_inc(v_fst_1887_);
v_snd_1888_ = lean_ctor_get(v_head_1865_, 1);
lean_inc(v_snd_1888_);
lean_dec(v_head_1865_);
v___x_1889_ = lean_unsigned_to_nat(0u);
v___x_1890_ = lean_nat_dec_eq(v_snd_1888_, v___x_1889_);
if (v___x_1890_ == 0)
{
lean_object* v___x_1891_; uint8_t v___x_1892_; 
v___x_1891_ = lean_unsigned_to_nat(4u);
v___x_1892_ = lean_nat_dec_lt(v_snd_1888_, v___x_1891_);
if (v___x_1892_ == 0)
{
lean_object* v___x_1893_; lean_object* v___x_1894_; lean_object* v___x_1895_; 
v___x_1893_ = lean_nat_mod(v_snd_1888_, v___x_1891_);
lean_dec(v_snd_1888_);
v___x_1894_ = lp_zhc__lean_Zhc_Iops_HillisSteele_extractPropGroupLut(v___x_1893_);
v___x_1895_ = lp_zhc__lean_Zhc_BlockAlgebra_Tracer_pbs(v___x_1862_, v___x_1894_, v_fst_1887_, v___y_1860_);
v___y_1877_ = v___x_1895_;
goto v___jp_1876_;
}
else
{
lean_object* v___x_1896_; lean_object* v___x_1897_; lean_object* v___x_1898_; lean_object* v___x_1899_; lean_object* v___x_1900_; 
v___x_1896_ = lean_unsigned_to_nat(1u);
v___x_1897_ = lean_nat_sub(v_snd_1888_, v___x_1896_);
lean_dec(v_snd_1888_);
v___x_1898_ = lean_nat_mod(v___x_1897_, v___x_1891_);
lean_dec(v___x_1897_);
v___x_1899_ = lp_zhc__lean_Zhc_Iops_HillisSteele_extractPropGroupLut(v___x_1898_);
v___x_1900_ = lp_zhc__lean_Zhc_BlockAlgebra_Tracer_pbs(v___x_1862_, v___x_1899_, v_fst_1887_, v___y_1860_);
v___y_1877_ = v___x_1900_;
goto v___jp_1876_;
}
}
else
{
lean_object* v___x_1901_; lean_object* v___x_1902_; 
lean_dec(v_snd_1888_);
v___x_1901_ = lp_zhc__lean_Zhc_Iops_carryMsgLut;
v___x_1902_ = lp_zhc__lean_Zhc_BlockAlgebra_Tracer_pbs2(v___x_1862_, v___x_1901_, v_fst_1887_, v___y_1860_);
if (lean_obj_tag(v___x_1902_) == 0)
{
lean_object* v_a_1903_; lean_object* v_fst_1904_; 
v_a_1903_ = lean_ctor_get(v___x_1902_, 0);
lean_inc(v_a_1903_);
lean_dec_ref_known(v___x_1902_, 1);
v_fst_1904_ = lean_ctor_get(v_a_1903_, 0);
lean_inc(v_fst_1904_);
lean_dec(v_a_1903_);
v_a_1871_ = v_fst_1904_;
goto v___jp_1870_;
}
else
{
lean_object* v_a_1905_; lean_object* v___x_1907_; uint8_t v_isShared_1908_; uint8_t v_isSharedCheck_1912_; 
lean_del_object(v___x_1868_);
lean_dec(v_tail_1866_);
lean_dec(v_x_1859_);
v_a_1905_ = lean_ctor_get(v___x_1902_, 0);
v_isSharedCheck_1912_ = !lean_is_exclusive(v___x_1902_);
if (v_isSharedCheck_1912_ == 0)
{
v___x_1907_ = v___x_1902_;
v_isShared_1908_ = v_isSharedCheck_1912_;
goto v_resetjp_1906_;
}
else
{
lean_inc(v_a_1905_);
lean_dec(v___x_1902_);
v___x_1907_ = lean_box(0);
v_isShared_1908_ = v_isSharedCheck_1912_;
goto v_resetjp_1906_;
}
v_resetjp_1906_:
{
lean_object* v___x_1910_; 
if (v_isShared_1908_ == 0)
{
v___x_1910_ = v___x_1907_;
goto v_reusejp_1909_;
}
else
{
lean_object* v_reuseFailAlloc_1911_; 
v_reuseFailAlloc_1911_ = lean_alloc_ctor(1, 1, 0);
lean_ctor_set(v_reuseFailAlloc_1911_, 0, v_a_1905_);
v___x_1910_ = v_reuseFailAlloc_1911_;
goto v_reusejp_1909_;
}
v_reusejp_1909_:
{
return v___x_1910_;
}
}
}
}
v___jp_1870_:
{
lean_object* v___x_1873_; 
if (v_isShared_1869_ == 0)
{
lean_ctor_set(v___x_1868_, 1, v_x_1859_);
lean_ctor_set(v___x_1868_, 0, v_a_1871_);
v___x_1873_ = v___x_1868_;
goto v_reusejp_1872_;
}
else
{
lean_object* v_reuseFailAlloc_1875_; 
v_reuseFailAlloc_1875_ = lean_alloc_ctor(1, 2, 0);
lean_ctor_set(v_reuseFailAlloc_1875_, 0, v_a_1871_);
lean_ctor_set(v_reuseFailAlloc_1875_, 1, v_x_1859_);
v___x_1873_ = v_reuseFailAlloc_1875_;
goto v_reusejp_1872_;
}
v_reusejp_1872_:
{
v_x_1858_ = v_tail_1866_;
v_x_1859_ = v___x_1873_;
goto _start;
}
}
v___jp_1876_:
{
if (lean_obj_tag(v___y_1877_) == 0)
{
lean_object* v_a_1878_; 
v_a_1878_ = lean_ctor_get(v___y_1877_, 0);
lean_inc(v_a_1878_);
lean_dec_ref_known(v___y_1877_, 1);
v_a_1871_ = v_a_1878_;
goto v___jp_1870_;
}
else
{
lean_object* v_a_1879_; lean_object* v___x_1881_; uint8_t v_isShared_1882_; uint8_t v_isSharedCheck_1886_; 
lean_del_object(v___x_1868_);
lean_dec(v_tail_1866_);
lean_dec(v_x_1859_);
v_a_1879_ = lean_ctor_get(v___y_1877_, 0);
v_isSharedCheck_1886_ = !lean_is_exclusive(v___y_1877_);
if (v_isSharedCheck_1886_ == 0)
{
v___x_1881_ = v___y_1877_;
v_isShared_1882_ = v_isSharedCheck_1886_;
goto v_resetjp_1880_;
}
else
{
lean_inc(v_a_1879_);
lean_dec(v___y_1877_);
v___x_1881_ = lean_box(0);
v_isShared_1882_ = v_isSharedCheck_1886_;
goto v_resetjp_1880_;
}
v_resetjp_1880_:
{
lean_object* v___x_1884_; 
if (v_isShared_1882_ == 0)
{
v___x_1884_ = v___x_1881_;
goto v_reusejp_1883_;
}
else
{
lean_object* v_reuseFailAlloc_1885_; 
v_reuseFailAlloc_1885_ = lean_alloc_ctor(1, 1, 0);
lean_ctor_set(v_reuseFailAlloc_1885_, 0, v_a_1879_);
v___x_1884_ = v_reuseFailAlloc_1885_;
goto v_reusejp_1883_;
}
v_reusejp_1883_:
{
return v___x_1884_;
}
}
}
}
}
}
}
}
LEAN_EXPORT lean_object* lp_zhc__lean_List_mapM_loop___at___00Zhc_Iops_HillisSteele_blockStates___at___00Zhc_Iops_addHillisSteele___at___00Zhc_Iops_addHillisSteeleCircuit_spec__0_spec__1_spec__3___boxed(lean_object* v_x_1914_, lean_object* v_x_1915_, lean_object* v___y_1916_, lean_object* v___y_1917_){
_start:
{
lean_object* v_res_1918_; 
v_res_1918_ = lp_zhc__lean_List_mapM_loop___at___00Zhc_Iops_HillisSteele_blockStates___at___00Zhc_Iops_addHillisSteele___at___00Zhc_Iops_addHillisSteeleCircuit_spec__0_spec__1_spec__3(v_x_1914_, v_x_1915_, v___y_1916_);
lean_dec(v___y_1916_);
return v_res_1918_;
}
}
LEAN_EXPORT lean_object* lp_zhc__lean_Zhc_Iops_HillisSteele_blockStates___at___00Zhc_Iops_addHillisSteele___at___00Zhc_Iops_addHillisSteeleCircuit_spec__0_spec__1(lean_object* v_sums_1919_, lean_object* v___y_1920_){
_start:
{
lean_object* v___x_1922_; lean_object* v___x_1923_; lean_object* v___x_1924_; lean_object* v___x_1925_; 
v___x_1922_ = lean_unsigned_to_nat(0u);
v___x_1923_ = l_List_zipIdxTR___redArg(v_sums_1919_, v___x_1922_);
v___x_1924_ = lean_box(0);
v___x_1925_ = lp_zhc__lean_List_mapM_loop___at___00Zhc_Iops_HillisSteele_blockStates___at___00Zhc_Iops_addHillisSteele___at___00Zhc_Iops_addHillisSteeleCircuit_spec__0_spec__1_spec__3(v___x_1923_, v___x_1924_, v___y_1920_);
return v___x_1925_;
}
}
LEAN_EXPORT lean_object* lp_zhc__lean_Zhc_Iops_HillisSteele_blockStates___at___00Zhc_Iops_addHillisSteele___at___00Zhc_Iops_addHillisSteeleCircuit_spec__0_spec__1___boxed(lean_object* v_sums_1926_, lean_object* v___y_1927_, lean_object* v___y_1928_){
_start:
{
lean_object* v_res_1929_; 
v_res_1929_ = lp_zhc__lean_Zhc_Iops_HillisSteele_blockStates___at___00Zhc_Iops_addHillisSteele___at___00Zhc_Iops_addHillisSteeleCircuit_spec__0_spec__1(v_sums_1926_, v___y_1927_);
lean_dec(v___y_1927_);
return v_res_1929_;
}
}
LEAN_EXPORT lean_object* lp_zhc__lean_List_mapM_loop___at___00Zhc_Iops_HillisSteele_finalResolution___at___00Zhc_Iops_addHillisSteele___at___00Zhc_Iops_addHillisSteeleCircuit_spec__0_spec__4_spec__10(lean_object* v_x_1930_, lean_object* v_x_1931_, lean_object* v___y_1932_){
_start:
{
lean_object* v___x_1934_; 
v___x_1934_ = ((lean_object*)(lp_zhc__lean_Zhc_Iops_HillisSteele_extractPropGroupLut___closed__0));
if (lean_obj_tag(v_x_1930_) == 0)
{
lean_object* v___x_1935_; lean_object* v___x_1936_; 
v___x_1935_ = l_List_reverse___redArg(v_x_1931_);
v___x_1936_ = lean_alloc_ctor(0, 1, 0);
lean_ctor_set(v___x_1936_, 0, v___x_1935_);
return v___x_1936_;
}
else
{
lean_object* v_head_1937_; lean_object* v_tail_1938_; lean_object* v___x_1940_; uint8_t v_isShared_1941_; uint8_t v_isSharedCheck_2095_; 
v_head_1937_ = lean_ctor_get(v_x_1930_, 0);
v_tail_1938_ = lean_ctor_get(v_x_1930_, 1);
v_isSharedCheck_2095_ = !lean_is_exclusive(v_x_1930_);
if (v_isSharedCheck_2095_ == 0)
{
v___x_1940_ = v_x_1930_;
v_isShared_1941_ = v_isSharedCheck_2095_;
goto v_resetjp_1939_;
}
else
{
lean_inc(v_tail_1938_);
lean_inc(v_head_1937_);
lean_dec(v_x_1930_);
v___x_1940_ = lean_box(0);
v_isShared_1941_ = v_isSharedCheck_2095_;
goto v_resetjp_1939_;
}
v_resetjp_1939_:
{
lean_object* v_a_1943_; lean_object* v_fst_1948_; lean_object* v_fst_1949_; lean_object* v_snd_1950_; lean_object* v_snd_1951_; lean_object* v_snd_1952_; 
v_fst_1948_ = lean_ctor_get(v_head_1937_, 0);
lean_inc(v_fst_1948_);
v_fst_1949_ = lean_ctor_get(v_fst_1948_, 0);
lean_inc(v_fst_1949_);
v_snd_1950_ = lean_ctor_get(v_fst_1949_, 1);
lean_inc(v_snd_1950_);
v_snd_1951_ = lean_ctor_get(v_snd_1950_, 1);
lean_inc(v_snd_1951_);
v_snd_1952_ = lean_ctor_get(v_head_1937_, 1);
lean_inc(v_snd_1952_);
lean_dec(v_head_1937_);
if (lean_obj_tag(v_snd_1952_) == 0)
{
lean_object* v_snd_1953_; lean_object* v_fst_1954_; lean_object* v___x_1956_; uint8_t v_isShared_1957_; uint8_t v_isSharedCheck_2001_; 
v_snd_1953_ = lean_ctor_get(v_fst_1948_, 1);
lean_inc(v_snd_1953_);
lean_dec(v_fst_1948_);
v_fst_1954_ = lean_ctor_get(v_fst_1949_, 0);
v_isSharedCheck_2001_ = !lean_is_exclusive(v_fst_1949_);
if (v_isSharedCheck_2001_ == 0)
{
lean_object* v_unused_2002_; 
v_unused_2002_ = lean_ctor_get(v_fst_1949_, 1);
lean_dec(v_unused_2002_);
v___x_1956_ = v_fst_1949_;
v_isShared_1957_ = v_isSharedCheck_2001_;
goto v_resetjp_1955_;
}
else
{
lean_inc(v_fst_1954_);
lean_dec(v_fst_1949_);
v___x_1956_ = lean_box(0);
v_isShared_1957_ = v_isSharedCheck_2001_;
goto v_resetjp_1955_;
}
v_resetjp_1955_:
{
lean_object* v_fst_1958_; lean_object* v___x_1960_; uint8_t v_isShared_1961_; uint8_t v_isSharedCheck_1999_; 
v_fst_1958_ = lean_ctor_get(v_snd_1950_, 0);
v_isSharedCheck_1999_ = !lean_is_exclusive(v_snd_1950_);
if (v_isSharedCheck_1999_ == 0)
{
lean_object* v_unused_2000_; 
v_unused_2000_ = lean_ctor_get(v_snd_1950_, 1);
lean_dec(v_unused_2000_);
v___x_1960_ = v_snd_1950_;
v_isShared_1961_ = v_isSharedCheck_1999_;
goto v_resetjp_1959_;
}
else
{
lean_inc(v_fst_1958_);
lean_dec(v_snd_1950_);
v___x_1960_ = lean_box(0);
v_isShared_1961_ = v_isSharedCheck_1999_;
goto v_resetjp_1959_;
}
v_resetjp_1959_:
{
lean_object* v_fst_1962_; lean_object* v___x_1964_; uint8_t v_isShared_1965_; uint8_t v_isSharedCheck_1997_; 
v_fst_1962_ = lean_ctor_get(v_snd_1951_, 0);
v_isSharedCheck_1997_ = !lean_is_exclusive(v_snd_1951_);
if (v_isSharedCheck_1997_ == 0)
{
lean_object* v_unused_1998_; 
v_unused_1998_ = lean_ctor_get(v_snd_1951_, 1);
lean_dec(v_unused_1998_);
v___x_1964_ = v_snd_1951_;
v_isShared_1965_ = v_isSharedCheck_1997_;
goto v_resetjp_1963_;
}
else
{
lean_inc(v_fst_1962_);
lean_dec(v_snd_1951_);
v___x_1964_ = lean_box(0);
v_isShared_1965_ = v_isSharedCheck_1997_;
goto v_resetjp_1963_;
}
v_resetjp_1963_:
{
lean_object* v___x_1966_; lean_object* v___x_1967_; 
v___x_1966_ = lean_obj_once(&lp_zhc__lean_Zhc_Iops_HillisSteele_finalResolution___redArg___lam__8___closed__1, &lp_zhc__lean_Zhc_Iops_HillisSteele_finalResolution___redArg___lam__8___closed__1_once, _init_lp_zhc__lean_Zhc_Iops_HillisSteele_finalResolution___redArg___lam__8___closed__1);
v___x_1967_ = lp_zhc__lean_Zhc_BlockAlgebra_Tracer_pbs(v___x_1934_, v___x_1966_, v_fst_1958_, v___y_1932_);
if (lean_obj_tag(v___x_1967_) == 0)
{
lean_object* v_a_1968_; lean_object* v___x_1969_; lean_object* v___x_1970_; 
v_a_1968_ = lean_ctor_get(v___x_1967_, 0);
lean_inc(v_a_1968_);
lean_dec_ref_known(v___x_1967_, 1);
v___x_1969_ = lean_obj_once(&lp_zhc__lean_List_mapM_loop___at___00Zhc_Iops_HillisSteele_finalResolution___at___00Zhc_Iops_addHillisSteele___at___00__private_Zhc_Iops_AddHillisSteele_Impl_0__Zhc_Iops_smokeHs_spec__0_spec__4_spec__12___closed__2, &lp_zhc__lean_List_mapM_loop___at___00Zhc_Iops_HillisSteele_finalResolution___at___00Zhc_Iops_addHillisSteele___at___00__private_Zhc_Iops_AddHillisSteele_Impl_0__Zhc_Iops_smokeHs_spec__0_spec__4_spec__12___closed__2_once, _init_lp_zhc__lean_List_mapM_loop___at___00Zhc_Iops_HillisSteele_finalResolution___at___00Zhc_Iops_addHillisSteele___at___00__private_Zhc_Iops_AddHillisSteele_Impl_0__Zhc_Iops_smokeHs_spec__0_spec__4_spec__12___closed__2);
v___x_1970_ = lp_zhc__lean_Zhc_BlockAlgebra_Tracer_pbs(v___x_1934_, v___x_1969_, v_fst_1962_, v___y_1932_);
if (lean_obj_tag(v___x_1970_) == 0)
{
lean_object* v_a_1971_; lean_object* v___x_1973_; 
v_a_1971_ = lean_ctor_get(v___x_1970_, 0);
lean_inc(v_a_1971_);
lean_dec_ref_known(v___x_1970_, 1);
if (v_isShared_1965_ == 0)
{
lean_ctor_set(v___x_1964_, 1, v_snd_1953_);
lean_ctor_set(v___x_1964_, 0, v_a_1971_);
v___x_1973_ = v___x_1964_;
goto v_reusejp_1972_;
}
else
{
lean_object* v_reuseFailAlloc_1980_; 
v_reuseFailAlloc_1980_ = lean_alloc_ctor(0, 2, 0);
lean_ctor_set(v_reuseFailAlloc_1980_, 0, v_a_1971_);
lean_ctor_set(v_reuseFailAlloc_1980_, 1, v_snd_1953_);
v___x_1973_ = v_reuseFailAlloc_1980_;
goto v_reusejp_1972_;
}
v_reusejp_1972_:
{
lean_object* v___x_1975_; 
if (v_isShared_1961_ == 0)
{
lean_ctor_set(v___x_1960_, 1, v___x_1973_);
lean_ctor_set(v___x_1960_, 0, v_a_1968_);
v___x_1975_ = v___x_1960_;
goto v_reusejp_1974_;
}
else
{
lean_object* v_reuseFailAlloc_1979_; 
v_reuseFailAlloc_1979_ = lean_alloc_ctor(0, 2, 0);
lean_ctor_set(v_reuseFailAlloc_1979_, 0, v_a_1968_);
lean_ctor_set(v_reuseFailAlloc_1979_, 1, v___x_1973_);
v___x_1975_ = v_reuseFailAlloc_1979_;
goto v_reusejp_1974_;
}
v_reusejp_1974_:
{
lean_object* v___x_1977_; 
if (v_isShared_1957_ == 0)
{
lean_ctor_set(v___x_1956_, 1, v___x_1975_);
v___x_1977_ = v___x_1956_;
goto v_reusejp_1976_;
}
else
{
lean_object* v_reuseFailAlloc_1978_; 
v_reuseFailAlloc_1978_ = lean_alloc_ctor(0, 2, 0);
lean_ctor_set(v_reuseFailAlloc_1978_, 0, v_fst_1954_);
lean_ctor_set(v_reuseFailAlloc_1978_, 1, v___x_1975_);
v___x_1977_ = v_reuseFailAlloc_1978_;
goto v_reusejp_1976_;
}
v_reusejp_1976_:
{
v_a_1943_ = v___x_1977_;
goto v___jp_1942_;
}
}
}
}
else
{
lean_object* v_a_1981_; lean_object* v___x_1983_; uint8_t v_isShared_1984_; uint8_t v_isSharedCheck_1988_; 
lean_dec(v_a_1968_);
lean_del_object(v___x_1964_);
lean_del_object(v___x_1960_);
lean_del_object(v___x_1956_);
lean_dec(v_fst_1954_);
lean_dec(v_snd_1953_);
lean_del_object(v___x_1940_);
lean_dec(v_tail_1938_);
lean_dec(v_x_1931_);
v_a_1981_ = lean_ctor_get(v___x_1970_, 0);
v_isSharedCheck_1988_ = !lean_is_exclusive(v___x_1970_);
if (v_isSharedCheck_1988_ == 0)
{
v___x_1983_ = v___x_1970_;
v_isShared_1984_ = v_isSharedCheck_1988_;
goto v_resetjp_1982_;
}
else
{
lean_inc(v_a_1981_);
lean_dec(v___x_1970_);
v___x_1983_ = lean_box(0);
v_isShared_1984_ = v_isSharedCheck_1988_;
goto v_resetjp_1982_;
}
v_resetjp_1982_:
{
lean_object* v___x_1986_; 
if (v_isShared_1984_ == 0)
{
v___x_1986_ = v___x_1983_;
goto v_reusejp_1985_;
}
else
{
lean_object* v_reuseFailAlloc_1987_; 
v_reuseFailAlloc_1987_ = lean_alloc_ctor(1, 1, 0);
lean_ctor_set(v_reuseFailAlloc_1987_, 0, v_a_1981_);
v___x_1986_ = v_reuseFailAlloc_1987_;
goto v_reusejp_1985_;
}
v_reusejp_1985_:
{
return v___x_1986_;
}
}
}
}
else
{
lean_object* v_a_1989_; lean_object* v___x_1991_; uint8_t v_isShared_1992_; uint8_t v_isSharedCheck_1996_; 
lean_del_object(v___x_1964_);
lean_dec(v_fst_1962_);
lean_del_object(v___x_1960_);
lean_del_object(v___x_1956_);
lean_dec(v_fst_1954_);
lean_dec(v_snd_1953_);
lean_del_object(v___x_1940_);
lean_dec(v_tail_1938_);
lean_dec(v_x_1931_);
v_a_1989_ = lean_ctor_get(v___x_1967_, 0);
v_isSharedCheck_1996_ = !lean_is_exclusive(v___x_1967_);
if (v_isSharedCheck_1996_ == 0)
{
v___x_1991_ = v___x_1967_;
v_isShared_1992_ = v_isSharedCheck_1996_;
goto v_resetjp_1990_;
}
else
{
lean_inc(v_a_1989_);
lean_dec(v___x_1967_);
v___x_1991_ = lean_box(0);
v_isShared_1992_ = v_isSharedCheck_1996_;
goto v_resetjp_1990_;
}
v_resetjp_1990_:
{
lean_object* v___x_1994_; 
if (v_isShared_1992_ == 0)
{
v___x_1994_ = v___x_1991_;
goto v_reusejp_1993_;
}
else
{
lean_object* v_reuseFailAlloc_1995_; 
v_reuseFailAlloc_1995_ = lean_alloc_ctor(1, 1, 0);
lean_ctor_set(v_reuseFailAlloc_1995_, 0, v_a_1989_);
v___x_1994_ = v_reuseFailAlloc_1995_;
goto v_reusejp_1993_;
}
v_reusejp_1993_:
{
return v___x_1994_;
}
}
}
}
}
}
}
else
{
lean_object* v_snd_2003_; lean_object* v_fst_2004_; lean_object* v___x_2006_; uint8_t v_isShared_2007_; uint8_t v_isSharedCheck_2093_; 
v_snd_2003_ = lean_ctor_get(v_fst_1948_, 1);
lean_inc(v_snd_2003_);
lean_dec(v_fst_1948_);
v_fst_2004_ = lean_ctor_get(v_fst_1949_, 0);
v_isSharedCheck_2093_ = !lean_is_exclusive(v_fst_1949_);
if (v_isSharedCheck_2093_ == 0)
{
lean_object* v_unused_2094_; 
v_unused_2094_ = lean_ctor_get(v_fst_1949_, 1);
lean_dec(v_unused_2094_);
v___x_2006_ = v_fst_1949_;
v_isShared_2007_ = v_isSharedCheck_2093_;
goto v_resetjp_2005_;
}
else
{
lean_inc(v_fst_2004_);
lean_dec(v_fst_1949_);
v___x_2006_ = lean_box(0);
v_isShared_2007_ = v_isSharedCheck_2093_;
goto v_resetjp_2005_;
}
v_resetjp_2005_:
{
lean_object* v_fst_2008_; lean_object* v___x_2010_; uint8_t v_isShared_2011_; uint8_t v_isSharedCheck_2091_; 
v_fst_2008_ = lean_ctor_get(v_snd_1950_, 0);
v_isSharedCheck_2091_ = !lean_is_exclusive(v_snd_1950_);
if (v_isSharedCheck_2091_ == 0)
{
lean_object* v_unused_2092_; 
v_unused_2092_ = lean_ctor_get(v_snd_1950_, 1);
lean_dec(v_unused_2092_);
v___x_2010_ = v_snd_1950_;
v_isShared_2011_ = v_isSharedCheck_2091_;
goto v_resetjp_2009_;
}
else
{
lean_inc(v_fst_2008_);
lean_dec(v_snd_1950_);
v___x_2010_ = lean_box(0);
v_isShared_2011_ = v_isSharedCheck_2091_;
goto v_resetjp_2009_;
}
v_resetjp_2009_:
{
lean_object* v_fst_2012_; lean_object* v___x_2014_; uint8_t v_isShared_2015_; uint8_t v_isSharedCheck_2089_; 
v_fst_2012_ = lean_ctor_get(v_snd_1951_, 0);
v_isSharedCheck_2089_ = !lean_is_exclusive(v_snd_1951_);
if (v_isSharedCheck_2089_ == 0)
{
lean_object* v_unused_2090_; 
v_unused_2090_ = lean_ctor_get(v_snd_1951_, 1);
lean_dec(v_unused_2090_);
v___x_2014_ = v_snd_1951_;
v_isShared_2015_ = v_isSharedCheck_2089_;
goto v_resetjp_2013_;
}
else
{
lean_inc(v_fst_2012_);
lean_dec(v_snd_1951_);
v___x_2014_ = lean_box(0);
v_isShared_2015_ = v_isSharedCheck_2089_;
goto v_resetjp_2013_;
}
v_resetjp_2013_:
{
lean_object* v_val_2016_; lean_object* v___x_2017_; 
v_val_2016_ = lean_ctor_get(v_snd_1952_, 0);
lean_inc_n(v_val_2016_, 2);
lean_dec_ref_known(v_snd_1952_, 1);
v___x_2017_ = lp_zhc__lean_Zhc_BlockAlgebra_Tracer_addCt(v___x_1934_, v_fst_2004_, v_val_2016_, v___y_1932_);
lean_dec(v_fst_2004_);
if (lean_obj_tag(v___x_2017_) == 0)
{
lean_object* v_a_2018_; lean_object* v___x_2019_; lean_object* v___x_2020_; 
v_a_2018_ = lean_ctor_get(v___x_2017_, 0);
lean_inc(v_a_2018_);
lean_dec_ref_known(v___x_2017_, 1);
v___x_2019_ = lean_obj_once(&lp_zhc__lean_Zhc_Iops_HillisSteele_finalResolution___redArg___lam__8___closed__1, &lp_zhc__lean_Zhc_Iops_HillisSteele_finalResolution___redArg___lam__8___closed__1_once, _init_lp_zhc__lean_Zhc_Iops_HillisSteele_finalResolution___redArg___lam__8___closed__1);
v___x_2020_ = lp_zhc__lean_Zhc_BlockAlgebra_Tracer_pbs(v___x_1934_, v___x_2019_, v_a_2018_, v___y_1932_);
if (lean_obj_tag(v___x_2020_) == 0)
{
lean_object* v_a_2021_; lean_object* v___x_2022_; 
v_a_2021_ = lean_ctor_get(v___x_2020_, 0);
lean_inc(v_a_2021_);
lean_dec_ref_known(v___x_2020_, 1);
lean_inc(v_val_2016_);
v___x_2022_ = lp_zhc__lean_Zhc_BlockAlgebra_Tracer_addCt(v___x_1934_, v_fst_2008_, v_val_2016_, v___y_1932_);
lean_dec(v_fst_2008_);
if (lean_obj_tag(v___x_2022_) == 0)
{
lean_object* v_a_2023_; lean_object* v___x_2024_; lean_object* v___x_2025_; 
v_a_2023_ = lean_ctor_get(v___x_2022_, 0);
lean_inc(v_a_2023_);
lean_dec_ref_known(v___x_2022_, 1);
v___x_2024_ = lean_obj_once(&lp_zhc__lean_List_mapM_loop___at___00Zhc_Iops_HillisSteele_finalResolution___at___00Zhc_Iops_addHillisSteele___at___00__private_Zhc_Iops_AddHillisSteele_Impl_0__Zhc_Iops_smokeHs_spec__0_spec__4_spec__12___closed__2, &lp_zhc__lean_List_mapM_loop___at___00Zhc_Iops_HillisSteele_finalResolution___at___00Zhc_Iops_addHillisSteele___at___00__private_Zhc_Iops_AddHillisSteele_Impl_0__Zhc_Iops_smokeHs_spec__0_spec__4_spec__12___closed__2_once, _init_lp_zhc__lean_List_mapM_loop___at___00Zhc_Iops_HillisSteele_finalResolution___at___00Zhc_Iops_addHillisSteele___at___00__private_Zhc_Iops_AddHillisSteele_Impl_0__Zhc_Iops_smokeHs_spec__0_spec__4_spec__12___closed__2);
v___x_2025_ = lp_zhc__lean_Zhc_BlockAlgebra_Tracer_pbs(v___x_1934_, v___x_2024_, v_a_2023_, v___y_1932_);
if (lean_obj_tag(v___x_2025_) == 0)
{
lean_object* v_a_2026_; lean_object* v___x_2027_; 
v_a_2026_ = lean_ctor_get(v___x_2025_, 0);
lean_inc(v_a_2026_);
lean_dec_ref_known(v___x_2025_, 1);
v___x_2027_ = lp_zhc__lean_Zhc_BlockAlgebra_Tracer_addCt(v___x_1934_, v_fst_2012_, v_val_2016_, v___y_1932_);
lean_dec(v_fst_2012_);
if (lean_obj_tag(v___x_2027_) == 0)
{
lean_object* v_a_2028_; lean_object* v___x_2029_; lean_object* v___x_2030_; 
v_a_2028_ = lean_ctor_get(v___x_2027_, 0);
lean_inc(v_a_2028_);
lean_dec_ref_known(v___x_2027_, 1);
v___x_2029_ = lean_obj_once(&lp_zhc__lean_Zhc_Iops_HillisSteele_groupStates___redArg___lam__5___closed__1, &lp_zhc__lean_Zhc_Iops_HillisSteele_groupStates___redArg___lam__5___closed__1_once, _init_lp_zhc__lean_Zhc_Iops_HillisSteele_groupStates___redArg___lam__5___closed__1);
v___x_2030_ = lp_zhc__lean_Zhc_BlockAlgebra_Tracer_pbs(v___x_1934_, v___x_2029_, v_a_2028_, v___y_1932_);
if (lean_obj_tag(v___x_2030_) == 0)
{
lean_object* v_a_2031_; lean_object* v___x_2033_; 
v_a_2031_ = lean_ctor_get(v___x_2030_, 0);
lean_inc(v_a_2031_);
lean_dec_ref_known(v___x_2030_, 1);
if (v_isShared_2015_ == 0)
{
lean_ctor_set(v___x_2014_, 1, v_snd_2003_);
lean_ctor_set(v___x_2014_, 0, v_a_2031_);
v___x_2033_ = v___x_2014_;
goto v_reusejp_2032_;
}
else
{
lean_object* v_reuseFailAlloc_2040_; 
v_reuseFailAlloc_2040_ = lean_alloc_ctor(0, 2, 0);
lean_ctor_set(v_reuseFailAlloc_2040_, 0, v_a_2031_);
lean_ctor_set(v_reuseFailAlloc_2040_, 1, v_snd_2003_);
v___x_2033_ = v_reuseFailAlloc_2040_;
goto v_reusejp_2032_;
}
v_reusejp_2032_:
{
lean_object* v___x_2035_; 
if (v_isShared_2011_ == 0)
{
lean_ctor_set(v___x_2010_, 1, v___x_2033_);
lean_ctor_set(v___x_2010_, 0, v_a_2026_);
v___x_2035_ = v___x_2010_;
goto v_reusejp_2034_;
}
else
{
lean_object* v_reuseFailAlloc_2039_; 
v_reuseFailAlloc_2039_ = lean_alloc_ctor(0, 2, 0);
lean_ctor_set(v_reuseFailAlloc_2039_, 0, v_a_2026_);
lean_ctor_set(v_reuseFailAlloc_2039_, 1, v___x_2033_);
v___x_2035_ = v_reuseFailAlloc_2039_;
goto v_reusejp_2034_;
}
v_reusejp_2034_:
{
lean_object* v___x_2037_; 
if (v_isShared_2007_ == 0)
{
lean_ctor_set(v___x_2006_, 1, v___x_2035_);
lean_ctor_set(v___x_2006_, 0, v_a_2021_);
v___x_2037_ = v___x_2006_;
goto v_reusejp_2036_;
}
else
{
lean_object* v_reuseFailAlloc_2038_; 
v_reuseFailAlloc_2038_ = lean_alloc_ctor(0, 2, 0);
lean_ctor_set(v_reuseFailAlloc_2038_, 0, v_a_2021_);
lean_ctor_set(v_reuseFailAlloc_2038_, 1, v___x_2035_);
v___x_2037_ = v_reuseFailAlloc_2038_;
goto v_reusejp_2036_;
}
v_reusejp_2036_:
{
v_a_1943_ = v___x_2037_;
goto v___jp_1942_;
}
}
}
}
else
{
lean_object* v_a_2041_; lean_object* v___x_2043_; uint8_t v_isShared_2044_; uint8_t v_isSharedCheck_2048_; 
lean_dec(v_a_2026_);
lean_dec(v_a_2021_);
lean_del_object(v___x_2014_);
lean_del_object(v___x_2010_);
lean_del_object(v___x_2006_);
lean_dec(v_snd_2003_);
lean_del_object(v___x_1940_);
lean_dec(v_tail_1938_);
lean_dec(v_x_1931_);
v_a_2041_ = lean_ctor_get(v___x_2030_, 0);
v_isSharedCheck_2048_ = !lean_is_exclusive(v___x_2030_);
if (v_isSharedCheck_2048_ == 0)
{
v___x_2043_ = v___x_2030_;
v_isShared_2044_ = v_isSharedCheck_2048_;
goto v_resetjp_2042_;
}
else
{
lean_inc(v_a_2041_);
lean_dec(v___x_2030_);
v___x_2043_ = lean_box(0);
v_isShared_2044_ = v_isSharedCheck_2048_;
goto v_resetjp_2042_;
}
v_resetjp_2042_:
{
lean_object* v___x_2046_; 
if (v_isShared_2044_ == 0)
{
v___x_2046_ = v___x_2043_;
goto v_reusejp_2045_;
}
else
{
lean_object* v_reuseFailAlloc_2047_; 
v_reuseFailAlloc_2047_ = lean_alloc_ctor(1, 1, 0);
lean_ctor_set(v_reuseFailAlloc_2047_, 0, v_a_2041_);
v___x_2046_ = v_reuseFailAlloc_2047_;
goto v_reusejp_2045_;
}
v_reusejp_2045_:
{
return v___x_2046_;
}
}
}
}
else
{
lean_object* v_a_2049_; lean_object* v___x_2051_; uint8_t v_isShared_2052_; uint8_t v_isSharedCheck_2056_; 
lean_dec(v_a_2026_);
lean_dec(v_a_2021_);
lean_del_object(v___x_2014_);
lean_del_object(v___x_2010_);
lean_del_object(v___x_2006_);
lean_dec(v_snd_2003_);
lean_del_object(v___x_1940_);
lean_dec(v_tail_1938_);
lean_dec(v_x_1931_);
v_a_2049_ = lean_ctor_get(v___x_2027_, 0);
v_isSharedCheck_2056_ = !lean_is_exclusive(v___x_2027_);
if (v_isSharedCheck_2056_ == 0)
{
v___x_2051_ = v___x_2027_;
v_isShared_2052_ = v_isSharedCheck_2056_;
goto v_resetjp_2050_;
}
else
{
lean_inc(v_a_2049_);
lean_dec(v___x_2027_);
v___x_2051_ = lean_box(0);
v_isShared_2052_ = v_isSharedCheck_2056_;
goto v_resetjp_2050_;
}
v_resetjp_2050_:
{
lean_object* v___x_2054_; 
if (v_isShared_2052_ == 0)
{
v___x_2054_ = v___x_2051_;
goto v_reusejp_2053_;
}
else
{
lean_object* v_reuseFailAlloc_2055_; 
v_reuseFailAlloc_2055_ = lean_alloc_ctor(1, 1, 0);
lean_ctor_set(v_reuseFailAlloc_2055_, 0, v_a_2049_);
v___x_2054_ = v_reuseFailAlloc_2055_;
goto v_reusejp_2053_;
}
v_reusejp_2053_:
{
return v___x_2054_;
}
}
}
}
else
{
lean_object* v_a_2057_; lean_object* v___x_2059_; uint8_t v_isShared_2060_; uint8_t v_isSharedCheck_2064_; 
lean_dec(v_a_2021_);
lean_dec(v_val_2016_);
lean_del_object(v___x_2014_);
lean_dec(v_fst_2012_);
lean_del_object(v___x_2010_);
lean_del_object(v___x_2006_);
lean_dec(v_snd_2003_);
lean_del_object(v___x_1940_);
lean_dec(v_tail_1938_);
lean_dec(v_x_1931_);
v_a_2057_ = lean_ctor_get(v___x_2025_, 0);
v_isSharedCheck_2064_ = !lean_is_exclusive(v___x_2025_);
if (v_isSharedCheck_2064_ == 0)
{
v___x_2059_ = v___x_2025_;
v_isShared_2060_ = v_isSharedCheck_2064_;
goto v_resetjp_2058_;
}
else
{
lean_inc(v_a_2057_);
lean_dec(v___x_2025_);
v___x_2059_ = lean_box(0);
v_isShared_2060_ = v_isSharedCheck_2064_;
goto v_resetjp_2058_;
}
v_resetjp_2058_:
{
lean_object* v___x_2062_; 
if (v_isShared_2060_ == 0)
{
v___x_2062_ = v___x_2059_;
goto v_reusejp_2061_;
}
else
{
lean_object* v_reuseFailAlloc_2063_; 
v_reuseFailAlloc_2063_ = lean_alloc_ctor(1, 1, 0);
lean_ctor_set(v_reuseFailAlloc_2063_, 0, v_a_2057_);
v___x_2062_ = v_reuseFailAlloc_2063_;
goto v_reusejp_2061_;
}
v_reusejp_2061_:
{
return v___x_2062_;
}
}
}
}
else
{
lean_object* v_a_2065_; lean_object* v___x_2067_; uint8_t v_isShared_2068_; uint8_t v_isSharedCheck_2072_; 
lean_dec(v_a_2021_);
lean_dec(v_val_2016_);
lean_del_object(v___x_2014_);
lean_dec(v_fst_2012_);
lean_del_object(v___x_2010_);
lean_del_object(v___x_2006_);
lean_dec(v_snd_2003_);
lean_del_object(v___x_1940_);
lean_dec(v_tail_1938_);
lean_dec(v_x_1931_);
v_a_2065_ = lean_ctor_get(v___x_2022_, 0);
v_isSharedCheck_2072_ = !lean_is_exclusive(v___x_2022_);
if (v_isSharedCheck_2072_ == 0)
{
v___x_2067_ = v___x_2022_;
v_isShared_2068_ = v_isSharedCheck_2072_;
goto v_resetjp_2066_;
}
else
{
lean_inc(v_a_2065_);
lean_dec(v___x_2022_);
v___x_2067_ = lean_box(0);
v_isShared_2068_ = v_isSharedCheck_2072_;
goto v_resetjp_2066_;
}
v_resetjp_2066_:
{
lean_object* v___x_2070_; 
if (v_isShared_2068_ == 0)
{
v___x_2070_ = v___x_2067_;
goto v_reusejp_2069_;
}
else
{
lean_object* v_reuseFailAlloc_2071_; 
v_reuseFailAlloc_2071_ = lean_alloc_ctor(1, 1, 0);
lean_ctor_set(v_reuseFailAlloc_2071_, 0, v_a_2065_);
v___x_2070_ = v_reuseFailAlloc_2071_;
goto v_reusejp_2069_;
}
v_reusejp_2069_:
{
return v___x_2070_;
}
}
}
}
else
{
lean_object* v_a_2073_; lean_object* v___x_2075_; uint8_t v_isShared_2076_; uint8_t v_isSharedCheck_2080_; 
lean_dec(v_val_2016_);
lean_del_object(v___x_2014_);
lean_dec(v_fst_2012_);
lean_del_object(v___x_2010_);
lean_dec(v_fst_2008_);
lean_del_object(v___x_2006_);
lean_dec(v_snd_2003_);
lean_del_object(v___x_1940_);
lean_dec(v_tail_1938_);
lean_dec(v_x_1931_);
v_a_2073_ = lean_ctor_get(v___x_2020_, 0);
v_isSharedCheck_2080_ = !lean_is_exclusive(v___x_2020_);
if (v_isSharedCheck_2080_ == 0)
{
v___x_2075_ = v___x_2020_;
v_isShared_2076_ = v_isSharedCheck_2080_;
goto v_resetjp_2074_;
}
else
{
lean_inc(v_a_2073_);
lean_dec(v___x_2020_);
v___x_2075_ = lean_box(0);
v_isShared_2076_ = v_isSharedCheck_2080_;
goto v_resetjp_2074_;
}
v_resetjp_2074_:
{
lean_object* v___x_2078_; 
if (v_isShared_2076_ == 0)
{
v___x_2078_ = v___x_2075_;
goto v_reusejp_2077_;
}
else
{
lean_object* v_reuseFailAlloc_2079_; 
v_reuseFailAlloc_2079_ = lean_alloc_ctor(1, 1, 0);
lean_ctor_set(v_reuseFailAlloc_2079_, 0, v_a_2073_);
v___x_2078_ = v_reuseFailAlloc_2079_;
goto v_reusejp_2077_;
}
v_reusejp_2077_:
{
return v___x_2078_;
}
}
}
}
else
{
lean_object* v_a_2081_; lean_object* v___x_2083_; uint8_t v_isShared_2084_; uint8_t v_isSharedCheck_2088_; 
lean_dec(v_val_2016_);
lean_del_object(v___x_2014_);
lean_dec(v_fst_2012_);
lean_del_object(v___x_2010_);
lean_dec(v_fst_2008_);
lean_del_object(v___x_2006_);
lean_dec(v_snd_2003_);
lean_del_object(v___x_1940_);
lean_dec(v_tail_1938_);
lean_dec(v_x_1931_);
v_a_2081_ = lean_ctor_get(v___x_2017_, 0);
v_isSharedCheck_2088_ = !lean_is_exclusive(v___x_2017_);
if (v_isSharedCheck_2088_ == 0)
{
v___x_2083_ = v___x_2017_;
v_isShared_2084_ = v_isSharedCheck_2088_;
goto v_resetjp_2082_;
}
else
{
lean_inc(v_a_2081_);
lean_dec(v___x_2017_);
v___x_2083_ = lean_box(0);
v_isShared_2084_ = v_isSharedCheck_2088_;
goto v_resetjp_2082_;
}
v_resetjp_2082_:
{
lean_object* v___x_2086_; 
if (v_isShared_2084_ == 0)
{
v___x_2086_ = v___x_2083_;
goto v_reusejp_2085_;
}
else
{
lean_object* v_reuseFailAlloc_2087_; 
v_reuseFailAlloc_2087_ = lean_alloc_ctor(1, 1, 0);
lean_ctor_set(v_reuseFailAlloc_2087_, 0, v_a_2081_);
v___x_2086_ = v_reuseFailAlloc_2087_;
goto v_reusejp_2085_;
}
v_reusejp_2085_:
{
return v___x_2086_;
}
}
}
}
}
}
}
v___jp_1942_:
{
lean_object* v___x_1945_; 
if (v_isShared_1941_ == 0)
{
lean_ctor_set(v___x_1940_, 1, v_x_1931_);
lean_ctor_set(v___x_1940_, 0, v_a_1943_);
v___x_1945_ = v___x_1940_;
goto v_reusejp_1944_;
}
else
{
lean_object* v_reuseFailAlloc_1947_; 
v_reuseFailAlloc_1947_ = lean_alloc_ctor(1, 2, 0);
lean_ctor_set(v_reuseFailAlloc_1947_, 0, v_a_1943_);
lean_ctor_set(v_reuseFailAlloc_1947_, 1, v_x_1931_);
v___x_1945_ = v_reuseFailAlloc_1947_;
goto v_reusejp_1944_;
}
v_reusejp_1944_:
{
v_x_1930_ = v_tail_1938_;
v_x_1931_ = v___x_1945_;
goto _start;
}
}
}
}
}
}
LEAN_EXPORT lean_object* lp_zhc__lean_List_mapM_loop___at___00Zhc_Iops_HillisSteele_finalResolution___at___00Zhc_Iops_addHillisSteele___at___00Zhc_Iops_addHillisSteeleCircuit_spec__0_spec__4_spec__10___boxed(lean_object* v_x_2096_, lean_object* v_x_2097_, lean_object* v___y_2098_, lean_object* v___y_2099_){
_start:
{
lean_object* v_res_2100_; 
v_res_2100_ = lp_zhc__lean_List_mapM_loop___at___00Zhc_Iops_HillisSteele_finalResolution___at___00Zhc_Iops_addHillisSteele___at___00Zhc_Iops_addHillisSteeleCircuit_spec__0_spec__4_spec__10(v_x_2096_, v_x_2097_, v___y_2098_);
lean_dec(v___y_2098_);
return v_res_2100_;
}
}
LEAN_EXPORT lean_object* lp_zhc__lean_Zhc_Iops_HillisSteele_finalResolution___at___00Zhc_Iops_addHillisSteele___at___00Zhc_Iops_addHillisSteeleCircuit_spec__0_spec__4(lean_object* v_groups_2101_, lean_object* v_carries_2102_, lean_object* v___y_2103_){
_start:
{
lean_object* v___x_2105_; lean_object* v___x_2106_; lean_object* v___x_2107_; lean_object* v_prevs_2108_; lean_object* v___x_2109_; lean_object* v___x_2110_; lean_object* v___x_2111_; lean_object* v___x_2112_; 
v___x_2105_ = lean_box(0);
v___x_2106_ = lean_box(0);
lean_inc(v_carries_2102_);
v___x_2107_ = lp_zhc__lean_List_mapTR_loop___at___00Zhc_Iops_HillisSteele_finalResolution___at___00Zhc_Iops_addHillisSteele___at___00__private_Zhc_Iops_AddHillisSteele_Impl_0__Zhc_Iops_smokeHs_spec__0_spec__4_spec__11(v_carries_2102_, v___x_2106_);
v_prevs_2108_ = lean_alloc_ctor(1, 2, 0);
lean_ctor_set(v_prevs_2108_, 0, v___x_2105_);
lean_ctor_set(v_prevs_2108_, 1, v___x_2107_);
v___x_2109_ = lp_zhc__lean_Zhc_Iops_HillisSteele_chunk4___redArg(v_groups_2101_);
v___x_2110_ = l_List_zipWith___at___00List_zip_spec__0___redArg(v___x_2109_, v_carries_2102_);
v___x_2111_ = l_List_zipWith___at___00List_zip_spec__0___redArg(v___x_2110_, v_prevs_2108_);
v___x_2112_ = lp_zhc__lean_List_mapM_loop___at___00Zhc_Iops_HillisSteele_finalResolution___at___00Zhc_Iops_addHillisSteele___at___00Zhc_Iops_addHillisSteeleCircuit_spec__0_spec__4_spec__10(v___x_2111_, v___x_2106_, v___y_2103_);
if (lean_obj_tag(v___x_2112_) == 0)
{
lean_object* v_a_2113_; lean_object* v___x_2115_; uint8_t v_isShared_2116_; uint8_t v_isSharedCheck_2121_; 
v_a_2113_ = lean_ctor_get(v___x_2112_, 0);
v_isSharedCheck_2121_ = !lean_is_exclusive(v___x_2112_);
if (v_isSharedCheck_2121_ == 0)
{
v___x_2115_ = v___x_2112_;
v_isShared_2116_ = v_isSharedCheck_2121_;
goto v_resetjp_2114_;
}
else
{
lean_inc(v_a_2113_);
lean_dec(v___x_2112_);
v___x_2115_ = lean_box(0);
v_isShared_2116_ = v_isSharedCheck_2121_;
goto v_resetjp_2114_;
}
v_resetjp_2114_:
{
lean_object* v___x_2117_; lean_object* v___x_2119_; 
v___x_2117_ = lp_zhc__lean_Zhc_Iops_HillisSteele_unchunk4___redArg(v_a_2113_);
if (v_isShared_2116_ == 0)
{
lean_ctor_set(v___x_2115_, 0, v___x_2117_);
v___x_2119_ = v___x_2115_;
goto v_reusejp_2118_;
}
else
{
lean_object* v_reuseFailAlloc_2120_; 
v_reuseFailAlloc_2120_ = lean_alloc_ctor(0, 1, 0);
lean_ctor_set(v_reuseFailAlloc_2120_, 0, v___x_2117_);
v___x_2119_ = v_reuseFailAlloc_2120_;
goto v_reusejp_2118_;
}
v_reusejp_2118_:
{
return v___x_2119_;
}
}
}
else
{
lean_object* v_a_2122_; lean_object* v___x_2124_; uint8_t v_isShared_2125_; uint8_t v_isSharedCheck_2129_; 
v_a_2122_ = lean_ctor_get(v___x_2112_, 0);
v_isSharedCheck_2129_ = !lean_is_exclusive(v___x_2112_);
if (v_isSharedCheck_2129_ == 0)
{
v___x_2124_ = v___x_2112_;
v_isShared_2125_ = v_isSharedCheck_2129_;
goto v_resetjp_2123_;
}
else
{
lean_inc(v_a_2122_);
lean_dec(v___x_2112_);
v___x_2124_ = lean_box(0);
v_isShared_2125_ = v_isSharedCheck_2129_;
goto v_resetjp_2123_;
}
v_resetjp_2123_:
{
lean_object* v___x_2127_; 
if (v_isShared_2125_ == 0)
{
v___x_2127_ = v___x_2124_;
goto v_reusejp_2126_;
}
else
{
lean_object* v_reuseFailAlloc_2128_; 
v_reuseFailAlloc_2128_ = lean_alloc_ctor(1, 1, 0);
lean_ctor_set(v_reuseFailAlloc_2128_, 0, v_a_2122_);
v___x_2127_ = v_reuseFailAlloc_2128_;
goto v_reusejp_2126_;
}
v_reusejp_2126_:
{
return v___x_2127_;
}
}
}
}
}
LEAN_EXPORT lean_object* lp_zhc__lean_Zhc_Iops_HillisSteele_finalResolution___at___00Zhc_Iops_addHillisSteele___at___00Zhc_Iops_addHillisSteeleCircuit_spec__0_spec__4___boxed(lean_object* v_groups_2130_, lean_object* v_carries_2131_, lean_object* v___y_2132_, lean_object* v___y_2133_){
_start:
{
lean_object* v_res_2134_; 
v_res_2134_ = lp_zhc__lean_Zhc_Iops_HillisSteele_finalResolution___at___00Zhc_Iops_addHillisSteele___at___00Zhc_Iops_addHillisSteeleCircuit_spec__0_spec__4(v_groups_2130_, v_carries_2131_, v___y_2132_);
lean_dec(v___y_2132_);
return v_res_2134_;
}
}
LEAN_EXPORT lean_object* lp_zhc__lean_List_mapM_loop___at___00Zhc_Iops_HillisSteele_rawSums___at___00Zhc_Iops_addHillisSteele___at___00Zhc_Iops_addHillisSteeleCircuit_spec__0_spec__0_spec__1___redArg(lean_object* v_x_2135_, lean_object* v_y_2136_, lean_object* v_x_2137_, lean_object* v_x_2138_, lean_object* v___y_2139_){
_start:
{
lean_object* v___x_2141_; 
v___x_2141_ = ((lean_object*)(lp_zhc__lean_Zhc_Iops_HillisSteele_extractPropGroupLut___closed__0));
if (lean_obj_tag(v_x_2137_) == 0)
{
lean_object* v___x_2142_; lean_object* v___x_2143_; 
lean_dec(v_y_2136_);
lean_dec(v_x_2135_);
v___x_2142_ = l_List_reverse___redArg(v_x_2138_);
v___x_2143_ = lean_alloc_ctor(0, 1, 0);
lean_ctor_set(v___x_2143_, 0, v___x_2142_);
return v___x_2143_;
}
else
{
lean_object* v_head_2144_; lean_object* v_tail_2145_; lean_object* v___x_2147_; uint8_t v_isShared_2148_; uint8_t v_isSharedCheck_2169_; 
v_head_2144_ = lean_ctor_get(v_x_2137_, 0);
v_tail_2145_ = lean_ctor_get(v_x_2137_, 1);
v_isSharedCheck_2169_ = !lean_is_exclusive(v_x_2137_);
if (v_isSharedCheck_2169_ == 0)
{
v___x_2147_ = v_x_2137_;
v_isShared_2148_ = v_isSharedCheck_2169_;
goto v_resetjp_2146_;
}
else
{
lean_inc(v_tail_2145_);
lean_inc(v_head_2144_);
lean_dec(v_x_2137_);
v___x_2147_ = lean_box(0);
v_isShared_2148_ = v_isSharedCheck_2169_;
goto v_resetjp_2146_;
}
v_resetjp_2146_:
{
lean_object* v___y_2150_; lean_object* v___x_2164_; 
lean_inc(v_head_2144_);
lean_inc(v_x_2135_);
v___x_2164_ = lp_zhc__lean_Zhc_BlockAlgebra_Tracer_extractCtBlock___redArg(v_x_2135_, v_head_2144_, v___y_2139_);
if (lean_obj_tag(v___x_2164_) == 0)
{
lean_object* v_a_2165_; lean_object* v___x_2166_; 
v_a_2165_ = lean_ctor_get(v___x_2164_, 0);
lean_inc(v_a_2165_);
lean_dec_ref_known(v___x_2164_, 1);
lean_inc(v_y_2136_);
v___x_2166_ = lp_zhc__lean_Zhc_BlockAlgebra_Tracer_extractCtBlock___redArg(v_y_2136_, v_head_2144_, v___y_2139_);
if (lean_obj_tag(v___x_2166_) == 0)
{
lean_object* v_a_2167_; lean_object* v___x_2168_; 
v_a_2167_ = lean_ctor_get(v___x_2166_, 0);
lean_inc(v_a_2167_);
lean_dec_ref_known(v___x_2166_, 1);
v___x_2168_ = lp_zhc__lean_Zhc_BlockAlgebra_Tracer_addCt(v___x_2141_, v_a_2165_, v_a_2167_, v___y_2139_);
lean_dec(v_a_2165_);
v___y_2150_ = v___x_2168_;
goto v___jp_2149_;
}
else
{
lean_dec(v_a_2165_);
v___y_2150_ = v___x_2166_;
goto v___jp_2149_;
}
}
else
{
lean_dec(v_head_2144_);
v___y_2150_ = v___x_2164_;
goto v___jp_2149_;
}
v___jp_2149_:
{
if (lean_obj_tag(v___y_2150_) == 0)
{
lean_object* v_a_2151_; lean_object* v___x_2153_; 
v_a_2151_ = lean_ctor_get(v___y_2150_, 0);
lean_inc(v_a_2151_);
lean_dec_ref_known(v___y_2150_, 1);
if (v_isShared_2148_ == 0)
{
lean_ctor_set(v___x_2147_, 1, v_x_2138_);
lean_ctor_set(v___x_2147_, 0, v_a_2151_);
v___x_2153_ = v___x_2147_;
goto v_reusejp_2152_;
}
else
{
lean_object* v_reuseFailAlloc_2155_; 
v_reuseFailAlloc_2155_ = lean_alloc_ctor(1, 2, 0);
lean_ctor_set(v_reuseFailAlloc_2155_, 0, v_a_2151_);
lean_ctor_set(v_reuseFailAlloc_2155_, 1, v_x_2138_);
v___x_2153_ = v_reuseFailAlloc_2155_;
goto v_reusejp_2152_;
}
v_reusejp_2152_:
{
v_x_2137_ = v_tail_2145_;
v_x_2138_ = v___x_2153_;
goto _start;
}
}
else
{
lean_object* v_a_2156_; lean_object* v___x_2158_; uint8_t v_isShared_2159_; uint8_t v_isSharedCheck_2163_; 
lean_del_object(v___x_2147_);
lean_dec(v_tail_2145_);
lean_dec(v_x_2138_);
lean_dec(v_y_2136_);
lean_dec(v_x_2135_);
v_a_2156_ = lean_ctor_get(v___y_2150_, 0);
v_isSharedCheck_2163_ = !lean_is_exclusive(v___y_2150_);
if (v_isSharedCheck_2163_ == 0)
{
v___x_2158_ = v___y_2150_;
v_isShared_2159_ = v_isSharedCheck_2163_;
goto v_resetjp_2157_;
}
else
{
lean_inc(v_a_2156_);
lean_dec(v___y_2150_);
v___x_2158_ = lean_box(0);
v_isShared_2159_ = v_isSharedCheck_2163_;
goto v_resetjp_2157_;
}
v_resetjp_2157_:
{
lean_object* v___x_2161_; 
if (v_isShared_2159_ == 0)
{
v___x_2161_ = v___x_2158_;
goto v_reusejp_2160_;
}
else
{
lean_object* v_reuseFailAlloc_2162_; 
v_reuseFailAlloc_2162_ = lean_alloc_ctor(1, 1, 0);
lean_ctor_set(v_reuseFailAlloc_2162_, 0, v_a_2156_);
v___x_2161_ = v_reuseFailAlloc_2162_;
goto v_reusejp_2160_;
}
v_reusejp_2160_:
{
return v___x_2161_;
}
}
}
}
}
}
}
}
LEAN_EXPORT lean_object* lp_zhc__lean_List_mapM_loop___at___00Zhc_Iops_HillisSteele_rawSums___at___00Zhc_Iops_addHillisSteele___at___00Zhc_Iops_addHillisSteeleCircuit_spec__0_spec__0_spec__1___redArg___boxed(lean_object* v_x_2170_, lean_object* v_y_2171_, lean_object* v_x_2172_, lean_object* v_x_2173_, lean_object* v___y_2174_, lean_object* v___y_2175_){
_start:
{
lean_object* v_res_2176_; 
v_res_2176_ = lp_zhc__lean_List_mapM_loop___at___00Zhc_Iops_HillisSteele_rawSums___at___00Zhc_Iops_addHillisSteele___at___00Zhc_Iops_addHillisSteeleCircuit_spec__0_spec__0_spec__1___redArg(v_x_2170_, v_y_2171_, v_x_2172_, v_x_2173_, v___y_2174_);
lean_dec(v___y_2174_);
return v_res_2176_;
}
}
LEAN_EXPORT lean_object* lp_zhc__lean_Zhc_Iops_HillisSteele_rawSums___at___00Zhc_Iops_addHillisSteele___at___00Zhc_Iops_addHillisSteeleCircuit_spec__0_spec__0(lean_object* v_si_2177_, lean_object* v_x_2178_, lean_object* v_y_2179_, lean_object* v___y_2180_){
_start:
{
lean_object* v___x_2182_; lean_object* v___x_2183_; lean_object* v___x_2184_; lean_object* v___x_2185_; 
v___x_2182_ = ((lean_object*)(lp_zhc__lean_Zhc_Iops_HillisSteele_extractPropGroupLut___closed__0));
lean_inc(v_si_2177_);
v___x_2183_ = l_List_finRange(v_si_2177_);
v___x_2184_ = lean_box(0);
v___x_2185_ = lp_zhc__lean_List_mapM_loop___at___00Zhc_Iops_HillisSteele_rawSums___at___00Zhc_Iops_addHillisSteele___at___00Zhc_Iops_addHillisSteeleCircuit_spec__0_spec__0_spec__1___redArg(v_x_2178_, v_y_2179_, v___x_2183_, v___x_2184_, v___y_2180_);
if (lean_obj_tag(v___x_2185_) == 0)
{
lean_object* v_a_2186_; lean_object* v___x_2187_; lean_object* v___x_2188_; 
v_a_2186_ = lean_ctor_get(v___x_2185_, 0);
lean_inc(v_a_2186_);
lean_dec_ref_known(v___x_2185_, 1);
v___x_2187_ = lean_unsigned_to_nat(0u);
v___x_2188_ = lp_zhc__lean_Zhc_BlockAlgebra_Tracer_letCtBlock(v___x_2182_, v___x_2187_, v___y_2180_);
if (lean_obj_tag(v___x_2188_) == 0)
{
lean_object* v_a_2189_; lean_object* v___x_2191_; uint8_t v_isShared_2192_; uint8_t v_isSharedCheck_2200_; 
v_a_2189_ = lean_ctor_get(v___x_2188_, 0);
v_isSharedCheck_2200_ = !lean_is_exclusive(v___x_2188_);
if (v_isSharedCheck_2200_ == 0)
{
v___x_2191_ = v___x_2188_;
v_isShared_2192_ = v_isSharedCheck_2200_;
goto v_resetjp_2190_;
}
else
{
lean_inc(v_a_2189_);
lean_dec(v___x_2188_);
v___x_2191_ = lean_box(0);
v_isShared_2192_ = v_isSharedCheck_2200_;
goto v_resetjp_2190_;
}
v_resetjp_2190_:
{
lean_object* v___x_2193_; lean_object* v___x_2194_; lean_object* v___x_2195_; lean_object* v___x_2196_; lean_object* v___x_2198_; 
v___x_2193_ = lp_zhc__lean_Zhc_Iops_HillisSteele_computeSize(v_si_2177_);
v___x_2194_ = lean_nat_sub(v___x_2193_, v_si_2177_);
lean_dec(v_si_2177_);
lean_dec(v___x_2193_);
v___x_2195_ = l_List_replicateTR___redArg(v___x_2194_, v_a_2189_);
v___x_2196_ = l_List_appendTR___redArg(v_a_2186_, v___x_2195_);
if (v_isShared_2192_ == 0)
{
lean_ctor_set(v___x_2191_, 0, v___x_2196_);
v___x_2198_ = v___x_2191_;
goto v_reusejp_2197_;
}
else
{
lean_object* v_reuseFailAlloc_2199_; 
v_reuseFailAlloc_2199_ = lean_alloc_ctor(0, 1, 0);
lean_ctor_set(v_reuseFailAlloc_2199_, 0, v___x_2196_);
v___x_2198_ = v_reuseFailAlloc_2199_;
goto v_reusejp_2197_;
}
v_reusejp_2197_:
{
return v___x_2198_;
}
}
}
else
{
lean_object* v_a_2201_; lean_object* v___x_2203_; uint8_t v_isShared_2204_; uint8_t v_isSharedCheck_2208_; 
lean_dec(v_a_2186_);
lean_dec(v_si_2177_);
v_a_2201_ = lean_ctor_get(v___x_2188_, 0);
v_isSharedCheck_2208_ = !lean_is_exclusive(v___x_2188_);
if (v_isSharedCheck_2208_ == 0)
{
v___x_2203_ = v___x_2188_;
v_isShared_2204_ = v_isSharedCheck_2208_;
goto v_resetjp_2202_;
}
else
{
lean_inc(v_a_2201_);
lean_dec(v___x_2188_);
v___x_2203_ = lean_box(0);
v_isShared_2204_ = v_isSharedCheck_2208_;
goto v_resetjp_2202_;
}
v_resetjp_2202_:
{
lean_object* v___x_2206_; 
if (v_isShared_2204_ == 0)
{
v___x_2206_ = v___x_2203_;
goto v_reusejp_2205_;
}
else
{
lean_object* v_reuseFailAlloc_2207_; 
v_reuseFailAlloc_2207_ = lean_alloc_ctor(1, 1, 0);
lean_ctor_set(v_reuseFailAlloc_2207_, 0, v_a_2201_);
v___x_2206_ = v_reuseFailAlloc_2207_;
goto v_reusejp_2205_;
}
v_reusejp_2205_:
{
return v___x_2206_;
}
}
}
}
else
{
lean_dec(v_si_2177_);
return v___x_2185_;
}
}
}
LEAN_EXPORT lean_object* lp_zhc__lean_Zhc_Iops_HillisSteele_rawSums___at___00Zhc_Iops_addHillisSteele___at___00Zhc_Iops_addHillisSteeleCircuit_spec__0_spec__0___boxed(lean_object* v_si_2209_, lean_object* v_x_2210_, lean_object* v_y_2211_, lean_object* v___y_2212_, lean_object* v___y_2213_){
_start:
{
lean_object* v_res_2214_; 
v_res_2214_ = lp_zhc__lean_Zhc_Iops_HillisSteele_rawSums___at___00Zhc_Iops_addHillisSteele___at___00Zhc_Iops_addHillisSteeleCircuit_spec__0_spec__0(v_si_2209_, v_x_2210_, v_y_2211_, v___y_2212_);
lean_dec(v___y_2212_);
return v_res_2214_;
}
}
LEAN_EXPORT lean_object* lp_zhc__lean_Zhc_Iops_addHillisSteele___at___00Zhc_Iops_addHillisSteeleCircuit_spec__0(lean_object* v_si_2215_, lean_object* v_x_2216_, lean_object* v_y_2217_, lean_object* v___y_2218_){
_start:
{
lean_object* v___x_2220_; 
lean_inc(v_si_2215_);
v___x_2220_ = lp_zhc__lean_Zhc_Iops_HillisSteele_rawSums___at___00Zhc_Iops_addHillisSteele___at___00Zhc_Iops_addHillisSteeleCircuit_spec__0_spec__0(v_si_2215_, v_x_2216_, v_y_2217_, v___y_2218_);
if (lean_obj_tag(v___x_2220_) == 0)
{
lean_object* v_a_2221_; lean_object* v___x_2222_; 
v_a_2221_ = lean_ctor_get(v___x_2220_, 0);
lean_inc_n(v_a_2221_, 2);
lean_dec_ref_known(v___x_2220_, 1);
v___x_2222_ = lp_zhc__lean_Zhc_Iops_HillisSteele_blockStates___at___00Zhc_Iops_addHillisSteele___at___00Zhc_Iops_addHillisSteeleCircuit_spec__0_spec__1(v_a_2221_, v___y_2218_);
if (lean_obj_tag(v___x_2222_) == 0)
{
lean_object* v_a_2223_; lean_object* v___x_2224_; 
v_a_2223_ = lean_ctor_get(v___x_2222_, 0);
lean_inc(v_a_2223_);
lean_dec_ref_known(v___x_2222_, 1);
v___x_2224_ = lp_zhc__lean_Zhc_Iops_HillisSteele_groupStates___at___00Zhc_Iops_addHillisSteele___at___00Zhc_Iops_addHillisSteeleCircuit_spec__0_spec__2(v_a_2223_, v___y_2218_);
if (lean_obj_tag(v___x_2224_) == 0)
{
lean_object* v_a_2225_; lean_object* v___x_2226_; 
v_a_2225_ = lean_ctor_get(v___x_2224_, 0);
lean_inc_n(v_a_2225_, 2);
lean_dec_ref_known(v___x_2224_, 1);
v___x_2226_ = lp_zhc__lean_Zhc_Iops_HillisSteele_groupCarries___at___00Zhc_Iops_addHillisSteele___at___00Zhc_Iops_addHillisSteeleCircuit_spec__0_spec__3(v_a_2225_, v___y_2218_);
if (lean_obj_tag(v___x_2226_) == 0)
{
lean_object* v_a_2227_; lean_object* v___x_2228_; 
v_a_2227_ = lean_ctor_get(v___x_2226_, 0);
lean_inc(v_a_2227_);
lean_dec_ref_known(v___x_2226_, 1);
v___x_2228_ = lp_zhc__lean_Zhc_Iops_HillisSteele_finalResolution___at___00Zhc_Iops_addHillisSteele___at___00Zhc_Iops_addHillisSteeleCircuit_spec__0_spec__4(v_a_2225_, v_a_2227_, v___y_2218_);
if (lean_obj_tag(v___x_2228_) == 0)
{
lean_object* v_a_2229_; lean_object* v___x_2230_; 
v_a_2229_ = lean_ctor_get(v___x_2228_, 0);
lean_inc(v_a_2229_);
lean_dec_ref_known(v___x_2228_, 1);
v___x_2230_ = lp_zhc__lean_Zhc_Iops_HillisSteele_propagate___at___00Zhc_Iops_addHillisSteele___at___00Zhc_Iops_addHillisSteeleCircuit_spec__0_spec__5(v_a_2221_, v_a_2229_, v___y_2218_);
if (lean_obj_tag(v___x_2230_) == 0)
{
lean_object* v_a_2231_; lean_object* v___x_2232_; 
v_a_2231_ = lean_ctor_get(v___x_2230_, 0);
lean_inc(v_a_2231_);
lean_dec_ref_known(v___x_2230_, 1);
v___x_2232_ = lp_zhc__lean_Zhc_Iops_HillisSteele_joinBlocks___at___00Zhc_Iops_addHillisSteele___at___00Zhc_Iops_addHillisSteeleCircuit_spec__0_spec__6(v_si_2215_, v_a_2231_, v___y_2218_);
return v___x_2232_;
}
else
{
lean_object* v_a_2233_; lean_object* v___x_2235_; uint8_t v_isShared_2236_; uint8_t v_isSharedCheck_2240_; 
lean_dec(v_si_2215_);
v_a_2233_ = lean_ctor_get(v___x_2230_, 0);
v_isSharedCheck_2240_ = !lean_is_exclusive(v___x_2230_);
if (v_isSharedCheck_2240_ == 0)
{
v___x_2235_ = v___x_2230_;
v_isShared_2236_ = v_isSharedCheck_2240_;
goto v_resetjp_2234_;
}
else
{
lean_inc(v_a_2233_);
lean_dec(v___x_2230_);
v___x_2235_ = lean_box(0);
v_isShared_2236_ = v_isSharedCheck_2240_;
goto v_resetjp_2234_;
}
v_resetjp_2234_:
{
lean_object* v___x_2238_; 
if (v_isShared_2236_ == 0)
{
v___x_2238_ = v___x_2235_;
goto v_reusejp_2237_;
}
else
{
lean_object* v_reuseFailAlloc_2239_; 
v_reuseFailAlloc_2239_ = lean_alloc_ctor(1, 1, 0);
lean_ctor_set(v_reuseFailAlloc_2239_, 0, v_a_2233_);
v___x_2238_ = v_reuseFailAlloc_2239_;
goto v_reusejp_2237_;
}
v_reusejp_2237_:
{
return v___x_2238_;
}
}
}
}
else
{
lean_object* v_a_2241_; lean_object* v___x_2243_; uint8_t v_isShared_2244_; uint8_t v_isSharedCheck_2248_; 
lean_dec(v_a_2221_);
lean_dec(v_si_2215_);
v_a_2241_ = lean_ctor_get(v___x_2228_, 0);
v_isSharedCheck_2248_ = !lean_is_exclusive(v___x_2228_);
if (v_isSharedCheck_2248_ == 0)
{
v___x_2243_ = v___x_2228_;
v_isShared_2244_ = v_isSharedCheck_2248_;
goto v_resetjp_2242_;
}
else
{
lean_inc(v_a_2241_);
lean_dec(v___x_2228_);
v___x_2243_ = lean_box(0);
v_isShared_2244_ = v_isSharedCheck_2248_;
goto v_resetjp_2242_;
}
v_resetjp_2242_:
{
lean_object* v___x_2246_; 
if (v_isShared_2244_ == 0)
{
v___x_2246_ = v___x_2243_;
goto v_reusejp_2245_;
}
else
{
lean_object* v_reuseFailAlloc_2247_; 
v_reuseFailAlloc_2247_ = lean_alloc_ctor(1, 1, 0);
lean_ctor_set(v_reuseFailAlloc_2247_, 0, v_a_2241_);
v___x_2246_ = v_reuseFailAlloc_2247_;
goto v_reusejp_2245_;
}
v_reusejp_2245_:
{
return v___x_2246_;
}
}
}
}
else
{
lean_object* v_a_2249_; lean_object* v___x_2251_; uint8_t v_isShared_2252_; uint8_t v_isSharedCheck_2256_; 
lean_dec(v_a_2225_);
lean_dec(v_a_2221_);
lean_dec(v_si_2215_);
v_a_2249_ = lean_ctor_get(v___x_2226_, 0);
v_isSharedCheck_2256_ = !lean_is_exclusive(v___x_2226_);
if (v_isSharedCheck_2256_ == 0)
{
v___x_2251_ = v___x_2226_;
v_isShared_2252_ = v_isSharedCheck_2256_;
goto v_resetjp_2250_;
}
else
{
lean_inc(v_a_2249_);
lean_dec(v___x_2226_);
v___x_2251_ = lean_box(0);
v_isShared_2252_ = v_isSharedCheck_2256_;
goto v_resetjp_2250_;
}
v_resetjp_2250_:
{
lean_object* v___x_2254_; 
if (v_isShared_2252_ == 0)
{
v___x_2254_ = v___x_2251_;
goto v_reusejp_2253_;
}
else
{
lean_object* v_reuseFailAlloc_2255_; 
v_reuseFailAlloc_2255_ = lean_alloc_ctor(1, 1, 0);
lean_ctor_set(v_reuseFailAlloc_2255_, 0, v_a_2249_);
v___x_2254_ = v_reuseFailAlloc_2255_;
goto v_reusejp_2253_;
}
v_reusejp_2253_:
{
return v___x_2254_;
}
}
}
}
else
{
lean_object* v_a_2257_; lean_object* v___x_2259_; uint8_t v_isShared_2260_; uint8_t v_isSharedCheck_2264_; 
lean_dec(v_a_2221_);
lean_dec(v_si_2215_);
v_a_2257_ = lean_ctor_get(v___x_2224_, 0);
v_isSharedCheck_2264_ = !lean_is_exclusive(v___x_2224_);
if (v_isSharedCheck_2264_ == 0)
{
v___x_2259_ = v___x_2224_;
v_isShared_2260_ = v_isSharedCheck_2264_;
goto v_resetjp_2258_;
}
else
{
lean_inc(v_a_2257_);
lean_dec(v___x_2224_);
v___x_2259_ = lean_box(0);
v_isShared_2260_ = v_isSharedCheck_2264_;
goto v_resetjp_2258_;
}
v_resetjp_2258_:
{
lean_object* v___x_2262_; 
if (v_isShared_2260_ == 0)
{
v___x_2262_ = v___x_2259_;
goto v_reusejp_2261_;
}
else
{
lean_object* v_reuseFailAlloc_2263_; 
v_reuseFailAlloc_2263_ = lean_alloc_ctor(1, 1, 0);
lean_ctor_set(v_reuseFailAlloc_2263_, 0, v_a_2257_);
v___x_2262_ = v_reuseFailAlloc_2263_;
goto v_reusejp_2261_;
}
v_reusejp_2261_:
{
return v___x_2262_;
}
}
}
}
else
{
lean_object* v_a_2265_; lean_object* v___x_2267_; uint8_t v_isShared_2268_; uint8_t v_isSharedCheck_2272_; 
lean_dec(v_a_2221_);
lean_dec(v_si_2215_);
v_a_2265_ = lean_ctor_get(v___x_2222_, 0);
v_isSharedCheck_2272_ = !lean_is_exclusive(v___x_2222_);
if (v_isSharedCheck_2272_ == 0)
{
v___x_2267_ = v___x_2222_;
v_isShared_2268_ = v_isSharedCheck_2272_;
goto v_resetjp_2266_;
}
else
{
lean_inc(v_a_2265_);
lean_dec(v___x_2222_);
v___x_2267_ = lean_box(0);
v_isShared_2268_ = v_isSharedCheck_2272_;
goto v_resetjp_2266_;
}
v_resetjp_2266_:
{
lean_object* v___x_2270_; 
if (v_isShared_2268_ == 0)
{
v___x_2270_ = v___x_2267_;
goto v_reusejp_2269_;
}
else
{
lean_object* v_reuseFailAlloc_2271_; 
v_reuseFailAlloc_2271_ = lean_alloc_ctor(1, 1, 0);
lean_ctor_set(v_reuseFailAlloc_2271_, 0, v_a_2265_);
v___x_2270_ = v_reuseFailAlloc_2271_;
goto v_reusejp_2269_;
}
v_reusejp_2269_:
{
return v___x_2270_;
}
}
}
}
else
{
lean_object* v_a_2273_; lean_object* v___x_2275_; uint8_t v_isShared_2276_; uint8_t v_isSharedCheck_2280_; 
lean_dec(v_si_2215_);
v_a_2273_ = lean_ctor_get(v___x_2220_, 0);
v_isSharedCheck_2280_ = !lean_is_exclusive(v___x_2220_);
if (v_isSharedCheck_2280_ == 0)
{
v___x_2275_ = v___x_2220_;
v_isShared_2276_ = v_isSharedCheck_2280_;
goto v_resetjp_2274_;
}
else
{
lean_inc(v_a_2273_);
lean_dec(v___x_2220_);
v___x_2275_ = lean_box(0);
v_isShared_2276_ = v_isSharedCheck_2280_;
goto v_resetjp_2274_;
}
v_resetjp_2274_:
{
lean_object* v___x_2278_; 
if (v_isShared_2276_ == 0)
{
v___x_2278_ = v___x_2275_;
goto v_reusejp_2277_;
}
else
{
lean_object* v_reuseFailAlloc_2279_; 
v_reuseFailAlloc_2279_ = lean_alloc_ctor(1, 1, 0);
lean_ctor_set(v_reuseFailAlloc_2279_, 0, v_a_2273_);
v___x_2278_ = v_reuseFailAlloc_2279_;
goto v_reusejp_2277_;
}
v_reusejp_2277_:
{
return v___x_2278_;
}
}
}
}
}
LEAN_EXPORT lean_object* lp_zhc__lean_Zhc_Iops_addHillisSteele___at___00Zhc_Iops_addHillisSteeleCircuit_spec__0___boxed(lean_object* v_si_2281_, lean_object* v_x_2282_, lean_object* v_y_2283_, lean_object* v___y_2284_, lean_object* v___y_2285_){
_start:
{
lean_object* v_res_2286_; 
v_res_2286_ = lp_zhc__lean_Zhc_Iops_addHillisSteele___at___00Zhc_Iops_addHillisSteeleCircuit_spec__0(v_si_2281_, v_x_2282_, v_y_2283_, v___y_2284_);
lean_dec(v___y_2284_);
return v_res_2286_;
}
}
LEAN_EXPORT lean_object* lp_zhc__lean_Zhc_Iops_addHillisSteeleCircuit___lam__0(lean_object* v___x_2287_, lean_object* v_si_2288_, lean_object* v___y_2289_){
_start:
{
lean_object* v___x_2291_; 
v___x_2291_ = lp_zhc__lean_Zhc_BlockAlgebra_Tracer_inputCtInteger(v___x_2287_, v_si_2288_, v___y_2289_);
if (lean_obj_tag(v___x_2291_) == 0)
{
lean_object* v_a_2292_; lean_object* v___x_2293_; 
v_a_2292_ = lean_ctor_get(v___x_2291_, 0);
lean_inc(v_a_2292_);
lean_dec_ref_known(v___x_2291_, 1);
v___x_2293_ = lp_zhc__lean_Zhc_BlockAlgebra_Tracer_inputCtInteger(v___x_2287_, v_si_2288_, v___y_2289_);
if (lean_obj_tag(v___x_2293_) == 0)
{
lean_object* v_a_2294_; lean_object* v___x_2295_; 
v_a_2294_ = lean_ctor_get(v___x_2293_, 0);
lean_inc(v_a_2294_);
lean_dec_ref_known(v___x_2293_, 1);
v___x_2295_ = lp_zhc__lean_Zhc_Iops_addHillisSteele___at___00Zhc_Iops_addHillisSteeleCircuit_spec__0(v_si_2288_, v_a_2292_, v_a_2294_, v___y_2289_);
if (lean_obj_tag(v___x_2295_) == 0)
{
lean_object* v_a_2296_; lean_object* v___x_2297_; 
v_a_2296_ = lean_ctor_get(v___x_2295_, 0);
lean_inc(v_a_2296_);
lean_dec_ref_known(v___x_2295_, 1);
v___x_2297_ = lp_zhc__lean_Zhc_BlockAlgebra_Tracer_outputCtInteger___redArg(v_a_2296_, v___y_2289_);
lean_dec(v_a_2296_);
return v___x_2297_;
}
else
{
lean_object* v_a_2298_; lean_object* v___x_2300_; uint8_t v_isShared_2301_; uint8_t v_isSharedCheck_2305_; 
v_a_2298_ = lean_ctor_get(v___x_2295_, 0);
v_isSharedCheck_2305_ = !lean_is_exclusive(v___x_2295_);
if (v_isSharedCheck_2305_ == 0)
{
v___x_2300_ = v___x_2295_;
v_isShared_2301_ = v_isSharedCheck_2305_;
goto v_resetjp_2299_;
}
else
{
lean_inc(v_a_2298_);
lean_dec(v___x_2295_);
v___x_2300_ = lean_box(0);
v_isShared_2301_ = v_isSharedCheck_2305_;
goto v_resetjp_2299_;
}
v_resetjp_2299_:
{
lean_object* v___x_2303_; 
if (v_isShared_2301_ == 0)
{
v___x_2303_ = v___x_2300_;
goto v_reusejp_2302_;
}
else
{
lean_object* v_reuseFailAlloc_2304_; 
v_reuseFailAlloc_2304_ = lean_alloc_ctor(1, 1, 0);
lean_ctor_set(v_reuseFailAlloc_2304_, 0, v_a_2298_);
v___x_2303_ = v_reuseFailAlloc_2304_;
goto v_reusejp_2302_;
}
v_reusejp_2302_:
{
return v___x_2303_;
}
}
}
}
else
{
lean_object* v_a_2306_; lean_object* v___x_2308_; uint8_t v_isShared_2309_; uint8_t v_isSharedCheck_2313_; 
lean_dec(v_a_2292_);
lean_dec(v_si_2288_);
v_a_2306_ = lean_ctor_get(v___x_2293_, 0);
v_isSharedCheck_2313_ = !lean_is_exclusive(v___x_2293_);
if (v_isSharedCheck_2313_ == 0)
{
v___x_2308_ = v___x_2293_;
v_isShared_2309_ = v_isSharedCheck_2313_;
goto v_resetjp_2307_;
}
else
{
lean_inc(v_a_2306_);
lean_dec(v___x_2293_);
v___x_2308_ = lean_box(0);
v_isShared_2309_ = v_isSharedCheck_2313_;
goto v_resetjp_2307_;
}
v_resetjp_2307_:
{
lean_object* v___x_2311_; 
if (v_isShared_2309_ == 0)
{
v___x_2311_ = v___x_2308_;
goto v_reusejp_2310_;
}
else
{
lean_object* v_reuseFailAlloc_2312_; 
v_reuseFailAlloc_2312_ = lean_alloc_ctor(1, 1, 0);
lean_ctor_set(v_reuseFailAlloc_2312_, 0, v_a_2306_);
v___x_2311_ = v_reuseFailAlloc_2312_;
goto v_reusejp_2310_;
}
v_reusejp_2310_:
{
return v___x_2311_;
}
}
}
}
else
{
lean_object* v_a_2314_; lean_object* v___x_2316_; uint8_t v_isShared_2317_; uint8_t v_isSharedCheck_2321_; 
lean_dec(v_si_2288_);
v_a_2314_ = lean_ctor_get(v___x_2291_, 0);
v_isSharedCheck_2321_ = !lean_is_exclusive(v___x_2291_);
if (v_isSharedCheck_2321_ == 0)
{
v___x_2316_ = v___x_2291_;
v_isShared_2317_ = v_isSharedCheck_2321_;
goto v_resetjp_2315_;
}
else
{
lean_inc(v_a_2314_);
lean_dec(v___x_2291_);
v___x_2316_ = lean_box(0);
v_isShared_2317_ = v_isSharedCheck_2321_;
goto v_resetjp_2315_;
}
v_resetjp_2315_:
{
lean_object* v___x_2319_; 
if (v_isShared_2317_ == 0)
{
v___x_2319_ = v___x_2316_;
goto v_reusejp_2318_;
}
else
{
lean_object* v_reuseFailAlloc_2320_; 
v_reuseFailAlloc_2320_ = lean_alloc_ctor(1, 1, 0);
lean_ctor_set(v_reuseFailAlloc_2320_, 0, v_a_2314_);
v___x_2319_ = v_reuseFailAlloc_2320_;
goto v_reusejp_2318_;
}
v_reusejp_2318_:
{
return v___x_2319_;
}
}
}
}
}
LEAN_EXPORT lean_object* lp_zhc__lean_Zhc_Iops_addHillisSteeleCircuit___lam__0___boxed(lean_object* v___x_2322_, lean_object* v_si_2323_, lean_object* v___y_2324_, lean_object* v___y_2325_){
_start:
{
lean_object* v_res_2326_; 
v_res_2326_ = lp_zhc__lean_Zhc_Iops_addHillisSteeleCircuit___lam__0(v___x_2322_, v_si_2323_, v___y_2324_);
lean_dec(v___y_2324_);
lean_dec_ref(v___x_2322_);
return v_res_2326_;
}
}
LEAN_EXPORT lean_object* lp_zhc__lean_Zhc_Iops_addHillisSteeleCircuit(lean_object* v_si_2327_){
_start:
{
lean_object* v___x_2329_; lean_object* v___f_2330_; lean_object* v___x_2331_; 
v___x_2329_ = ((lean_object*)(lp_zhc__lean_Zhc_Iops_HillisSteele_extractPropGroupLut___closed__0));
v___f_2330_ = lean_alloc_closure((void*)(lp_zhc__lean_Zhc_Iops_addHillisSteeleCircuit___lam__0___boxed), 4, 2);
lean_closure_set(v___f_2330_, 0, v___x_2329_);
lean_closure_set(v___f_2330_, 1, v_si_2327_);
v___x_2331_ = lp_zhc__lean_Zhc_BlockAlgebra_Tracer_emit(v___x_2329_, v___f_2330_);
return v___x_2331_;
}
}
LEAN_EXPORT lean_object* lp_zhc__lean_Zhc_Iops_addHillisSteeleCircuit___boxed(lean_object* v_si_2332_, lean_object* v_a_2333_){
_start:
{
lean_object* v_res_2334_; 
v_res_2334_ = lp_zhc__lean_Zhc_Iops_addHillisSteeleCircuit(v_si_2332_);
return v_res_2334_;
}
}
LEAN_EXPORT lean_object* lp_zhc__lean_List_mapM_loop___at___00Zhc_Iops_HillisSteele_rawSums___at___00Zhc_Iops_addHillisSteele___at___00Zhc_Iops_addHillisSteeleCircuit_spec__0_spec__0_spec__1(lean_object* v_si_2335_, lean_object* v_x_2336_, lean_object* v_y_2337_, lean_object* v_x_2338_, lean_object* v_x_2339_, lean_object* v___y_2340_){
_start:
{
lean_object* v___x_2342_; 
v___x_2342_ = lp_zhc__lean_List_mapM_loop___at___00Zhc_Iops_HillisSteele_rawSums___at___00Zhc_Iops_addHillisSteele___at___00Zhc_Iops_addHillisSteeleCircuit_spec__0_spec__0_spec__1___redArg(v_x_2336_, v_y_2337_, v_x_2338_, v_x_2339_, v___y_2340_);
return v___x_2342_;
}
}
LEAN_EXPORT lean_object* lp_zhc__lean_List_mapM_loop___at___00Zhc_Iops_HillisSteele_rawSums___at___00Zhc_Iops_addHillisSteele___at___00Zhc_Iops_addHillisSteeleCircuit_spec__0_spec__0_spec__1___boxed(lean_object* v_si_2343_, lean_object* v_x_2344_, lean_object* v_y_2345_, lean_object* v_x_2346_, lean_object* v_x_2347_, lean_object* v___y_2348_, lean_object* v___y_2349_){
_start:
{
lean_object* v_res_2350_; 
v_res_2350_ = lp_zhc__lean_List_mapM_loop___at___00Zhc_Iops_HillisSteele_rawSums___at___00Zhc_Iops_addHillisSteele___at___00Zhc_Iops_addHillisSteeleCircuit_spec__0_spec__0_spec__1(v_si_2343_, v_x_2344_, v_y_2345_, v_x_2346_, v_x_2347_, v___y_2348_);
lean_dec(v___y_2348_);
lean_dec(v_si_2343_);
return v_res_2350_;
}
}
LEAN_EXPORT lean_object* lp_zhc__lean_List_forIn_x27_loop___at___00Zhc_Iops_HillisSteele_groupCarries___at___00Zhc_Iops_addHillisSteele___at___00Zhc_Iops_addHillisSteeleCircuit_spec__0_spec__3_spec__8(lean_object* v_as_2351_, lean_object* v_as_x27_2352_, lean_object* v_b_2353_, lean_object* v_a_2354_, lean_object* v___y_2355_){
_start:
{
lean_object* v___x_2357_; 
v___x_2357_ = lp_zhc__lean_List_forIn_x27_loop___at___00Zhc_Iops_HillisSteele_groupCarries___at___00Zhc_Iops_addHillisSteele___at___00Zhc_Iops_addHillisSteeleCircuit_spec__0_spec__3_spec__8___redArg(v_as_x27_2352_, v_b_2353_, v___y_2355_);
return v___x_2357_;
}
}
LEAN_EXPORT lean_object* lp_zhc__lean_List_forIn_x27_loop___at___00Zhc_Iops_HillisSteele_groupCarries___at___00Zhc_Iops_addHillisSteele___at___00Zhc_Iops_addHillisSteeleCircuit_spec__0_spec__3_spec__8___boxed(lean_object* v_as_2358_, lean_object* v_as_x27_2359_, lean_object* v_b_2360_, lean_object* v_a_2361_, lean_object* v___y_2362_, lean_object* v___y_2363_){
_start:
{
lean_object* v_res_2364_; 
v_res_2364_ = lp_zhc__lean_List_forIn_x27_loop___at___00Zhc_Iops_HillisSteele_groupCarries___at___00Zhc_Iops_addHillisSteele___at___00Zhc_Iops_addHillisSteeleCircuit_spec__0_spec__3_spec__8(v_as_2358_, v_as_x27_2359_, v_b_2360_, v_a_2361_, v___y_2362_);
lean_dec(v___y_2362_);
lean_dec(v_as_x27_2359_);
lean_dec(v_as_2358_);
return v_res_2364_;
}
}
LEAN_EXPORT lean_object* lp_zhc__lean_List_forIn_x27_loop___at___00Zhc_Iops_HillisSteele_joinBlocks___at___00Zhc_Iops_addHillisSteele___at___00Zhc_Iops_addHillisSteeleCircuit_spec__0_spec__6_spec__15(lean_object* v_si_2365_, lean_object* v_as_2366_, lean_object* v_as_x27_2367_, lean_object* v_b_2368_, lean_object* v_a_2369_, lean_object* v___y_2370_){
_start:
{
lean_object* v___x_2372_; 
v___x_2372_ = lp_zhc__lean_List_forIn_x27_loop___at___00Zhc_Iops_HillisSteele_joinBlocks___at___00Zhc_Iops_addHillisSteele___at___00Zhc_Iops_addHillisSteeleCircuit_spec__0_spec__6_spec__15___redArg(v_as_x27_2367_, v_b_2368_, v___y_2370_);
return v___x_2372_;
}
}
LEAN_EXPORT lean_object* lp_zhc__lean_List_forIn_x27_loop___at___00Zhc_Iops_HillisSteele_joinBlocks___at___00Zhc_Iops_addHillisSteele___at___00Zhc_Iops_addHillisSteeleCircuit_spec__0_spec__6_spec__15___boxed(lean_object* v_si_2373_, lean_object* v_as_2374_, lean_object* v_as_x27_2375_, lean_object* v_b_2376_, lean_object* v_a_2377_, lean_object* v___y_2378_, lean_object* v___y_2379_){
_start:
{
lean_object* v_res_2380_; 
v_res_2380_ = lp_zhc__lean_List_forIn_x27_loop___at___00Zhc_Iops_HillisSteele_joinBlocks___at___00Zhc_Iops_addHillisSteele___at___00Zhc_Iops_addHillisSteeleCircuit_spec__0_spec__6_spec__15(v_si_2373_, v_as_2374_, v_as_x27_2375_, v_b_2376_, v_a_2377_, v___y_2378_);
lean_dec(v___y_2378_);
lean_dec(v_as_x27_2375_);
lean_dec(v_as_2374_);
lean_dec(v_si_2373_);
return v_res_2380_;
}
}
lean_object* initialize_Init(uint8_t builtin);
lean_object* initialize_Init(uint8_t builtin);
lean_object* initialize_zhc__lean_Zhc_BlockAlgebra_Class(uint8_t builtin);
lean_object* initialize_zhc__lean_Zhc_BlockAlgebra_Model_Instance(uint8_t builtin);
lean_object* initialize_zhc__lean_Zhc_BlockAlgebra_Tracer_Instance(uint8_t builtin);
lean_object* initialize_zhc__lean_Zhc_Iops_AddRippleCarry_Impl(uint8_t builtin);
void lean_initialize_runtime_module();
static bool _G_initialized = false;
LEAN_EXPORT lean_object* initialize_zhc__lean_Zhc_Iops_AddHillisSteele_Impl(uint8_t builtin) {
lean_object * res;
if (_G_initialized) return lean_io_result_mk_ok(lean_box(0));
_G_initialized = true;
lean_initialize_runtime_module();
res = initialize_Init(builtin);
if (lean_io_result_is_error(res)) return res;
lean_dec_ref(res);
res = initialize_Init(builtin);
if (lean_io_result_is_error(res)) return res;
lean_dec_ref(res);
res = initialize_zhc__lean_Zhc_BlockAlgebra_Class(builtin);
if (lean_io_result_is_error(res)) return res;
lean_dec_ref(res);
res = initialize_zhc__lean_Zhc_BlockAlgebra_Model_Instance(builtin);
if (lean_io_result_is_error(res)) return res;
lean_dec_ref(res);
res = initialize_zhc__lean_Zhc_BlockAlgebra_Tracer_Instance(builtin);
if (lean_io_result_is_error(res)) return res;
lean_dec_ref(res);
res = initialize_zhc__lean_Zhc_Iops_AddRippleCarry_Impl(builtin);
if (lean_io_result_is_error(res)) return res;
lean_dec_ref(res);
lp_zhc__lean_Zhc_Iops_HillisSteele_reduceCarryPadLut = _init_lp_zhc__lean_Zhc_Iops_HillisSteele_reduceCarryPadLut();
lean_mark_persistent(lp_zhc__lean_Zhc_Iops_HillisSteele_reduceCarryPadLut);
lp_zhc__lean_Zhc_Iops_HillisSteele_solvePropLut = _init_lp_zhc__lean_Zhc_Iops_HillisSteele_solvePropLut();
lean_mark_persistent(lp_zhc__lean_Zhc_Iops_HillisSteele_solvePropLut);
lp_zhc__lean_Zhc_Iops_HillisSteele_solvePropCarryLut = _init_lp_zhc__lean_Zhc_Iops_HillisSteele_solvePropCarryLut();
lean_mark_persistent(lp_zhc__lean_Zhc_Iops_HillisSteele_solvePropCarryLut);
lp_zhc__lean_Zhc_Iops_HillisSteele_msgOnlyLut = _init_lp_zhc__lean_Zhc_Iops_HillisSteele_msgOnlyLut();
lean_mark_persistent(lp_zhc__lean_Zhc_Iops_HillisSteele_msgOnlyLut);
return lean_io_result_mk_ok(lean_box(0));
}
#ifdef __cplusplus
}
#endif
