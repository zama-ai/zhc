// Lean compiler output
// Module: Zhc.Ffi.L2r.SpecBlock
// Imports: public import Init public meta import Init public import Zhc.Model.SpecBlock
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
uint8_t lean_uint8_dec_eq(uint8_t, uint8_t);
uint8_t lean_uint8_of_nat(lean_object*);
LEAN_EXPORT lean_object* lp_zhc__lean_Zhc_Ffi_L2r_SpecBlockPointed;
lean_object* zhc_lean_spec_block_new(uint8_t, uint8_t);
LEAN_EXPORT lean_object* lp_zhc__lean_Zhc_Ffi_L2r_SpecBlock_new___boxed(lean_object*, lean_object*, lean_object*);
lean_object* zhc_lean_spec_block_carry(lean_object*);
LEAN_EXPORT lean_object* lp_zhc__lean_Zhc_Ffi_L2r_SpecBlock_carry___boxed(lean_object*, lean_object*);
lean_object* zhc_lean_spec_block_message(lean_object*);
LEAN_EXPORT lean_object* lp_zhc__lean_Zhc_Ffi_L2r_SpecBlock_message___boxed(lean_object*, lean_object*);
LEAN_EXPORT lean_object* lp_zhc__lean_Zhc_Ffi_L2r_SpecBlock_ofModel(lean_object*);
LEAN_EXPORT lean_object* lp_zhc__lean_Zhc_Ffi_L2r_SpecBlock_ofModel___boxed(lean_object*, lean_object*);
LEAN_EXPORT lean_object* lp_zhc__lean_Zhc_Ffi_L2r_SpecBlock_same(lean_object*, lean_object*);
LEAN_EXPORT lean_object* lp_zhc__lean_Zhc_Ffi_L2r_SpecBlock_same___boxed(lean_object*, lean_object*, lean_object*);
static lean_object* _init_lp_zhc__lean_Zhc_Ffi_L2r_SpecBlockPointed(void){
_start:
{
lean_object* v___x_1_; 
v___x_1_ = lean_box(0);
return v___x_1_;
}
}
LEAN_EXPORT lean_object* lp_zhc__lean_Zhc_Ffi_L2r_SpecBlock_new___boxed(lean_object* v_carry_5_, lean_object* v_message_6_, lean_object* v_a_00___x40___internal___hyg_7_){
_start:
{
uint8_t v_carry_boxed_8_; uint8_t v_message_boxed_9_; lean_object* v_res_10_; 
v_carry_boxed_8_ = lean_unbox(v_carry_5_);
v_message_boxed_9_ = lean_unbox(v_message_6_);
v_res_10_ = zhc_lean_spec_block_new(v_carry_boxed_8_, v_message_boxed_9_);
return v_res_10_;
}
}
LEAN_EXPORT lean_object* lp_zhc__lean_Zhc_Ffi_L2r_SpecBlock_carry___boxed(lean_object* v_self_13_, lean_object* v_a_00___x40___internal___hyg_14_){
_start:
{
lean_object* v_res_15_; 
v_res_15_ = zhc_lean_spec_block_carry(v_self_13_);
lean_dec(v_self_13_);
return v_res_15_;
}
}
LEAN_EXPORT lean_object* lp_zhc__lean_Zhc_Ffi_L2r_SpecBlock_message___boxed(lean_object* v_self_18_, lean_object* v_a_00___x40___internal___hyg_19_){
_start:
{
lean_object* v_res_20_; 
v_res_20_ = zhc_lean_spec_block_message(v_self_18_);
lean_dec(v_self_18_);
return v_res_20_;
}
}
LEAN_EXPORT lean_object* lp_zhc__lean_Zhc_Ffi_L2r_SpecBlock_ofModel(lean_object* v_sb_21_){
_start:
{
lean_object* v_carrySize_23_; lean_object* v_messageSize_24_; uint8_t v___x_25_; uint8_t v___x_26_; lean_object* v___x_27_; 
v_carrySize_23_ = lean_ctor_get(v_sb_21_, 0);
v_messageSize_24_ = lean_ctor_get(v_sb_21_, 1);
v___x_25_ = lean_uint8_of_nat(v_carrySize_23_);
v___x_26_ = lean_uint8_of_nat(v_messageSize_24_);
v___x_27_ = zhc_lean_spec_block_new(v___x_25_, v___x_26_);
return v___x_27_;
}
}
LEAN_EXPORT lean_object* lp_zhc__lean_Zhc_Ffi_L2r_SpecBlock_ofModel___boxed(lean_object* v_sb_28_, lean_object* v_a_29_){
_start:
{
lean_object* v_res_30_; 
v_res_30_ = lp_zhc__lean_Zhc_Ffi_L2r_SpecBlock_ofModel(v_sb_28_);
lean_dec_ref(v_sb_28_);
return v_res_30_;
}
}
LEAN_EXPORT lean_object* lp_zhc__lean_Zhc_Ffi_L2r_SpecBlock_same(lean_object* v_a_31_, lean_object* v_b_32_){
_start:
{
lean_object* v___x_34_; 
v___x_34_ = zhc_lean_spec_block_carry(v_a_31_);
if (lean_obj_tag(v___x_34_) == 0)
{
lean_object* v_a_35_; lean_object* v___x_36_; 
v_a_35_ = lean_ctor_get(v___x_34_, 0);
lean_inc(v_a_35_);
lean_dec_ref_known(v___x_34_, 1);
v___x_36_ = zhc_lean_spec_block_carry(v_b_32_);
if (lean_obj_tag(v___x_36_) == 0)
{
lean_object* v_a_37_; lean_object* v___x_38_; 
v_a_37_ = lean_ctor_get(v___x_36_, 0);
lean_inc(v_a_37_);
lean_dec_ref_known(v___x_36_, 1);
v___x_38_ = zhc_lean_spec_block_message(v_a_31_);
if (lean_obj_tag(v___x_38_) == 0)
{
lean_object* v_a_39_; lean_object* v___x_40_; 
v_a_39_ = lean_ctor_get(v___x_38_, 0);
lean_inc(v_a_39_);
lean_dec_ref_known(v___x_38_, 1);
v___x_40_ = zhc_lean_spec_block_message(v_b_32_);
if (lean_obj_tag(v___x_40_) == 0)
{
lean_object* v_a_41_; lean_object* v___x_43_; uint8_t v_isShared_44_; uint8_t v_isSharedCheck_59_; 
v_a_41_ = lean_ctor_get(v___x_40_, 0);
v_isSharedCheck_59_ = !lean_is_exclusive(v___x_40_);
if (v_isSharedCheck_59_ == 0)
{
v___x_43_ = v___x_40_;
v_isShared_44_ = v_isSharedCheck_59_;
goto v_resetjp_42_;
}
else
{
lean_inc(v_a_41_);
lean_dec(v___x_40_);
v___x_43_ = lean_box(0);
v_isShared_44_ = v_isSharedCheck_59_;
goto v_resetjp_42_;
}
v_resetjp_42_:
{
uint8_t v___x_45_; uint8_t v___x_46_; uint8_t v___x_47_; 
v___x_45_ = lean_unbox(v_a_35_);
lean_dec(v_a_35_);
v___x_46_ = lean_unbox(v_a_37_);
lean_dec(v_a_37_);
v___x_47_ = lean_uint8_dec_eq(v___x_45_, v___x_46_);
if (v___x_47_ == 0)
{
lean_object* v___x_48_; lean_object* v___x_50_; 
lean_dec(v_a_41_);
lean_dec(v_a_39_);
v___x_48_ = lean_box(v___x_47_);
if (v_isShared_44_ == 0)
{
lean_ctor_set(v___x_43_, 0, v___x_48_);
v___x_50_ = v___x_43_;
goto v_reusejp_49_;
}
else
{
lean_object* v_reuseFailAlloc_51_; 
v_reuseFailAlloc_51_ = lean_alloc_ctor(0, 1, 0);
lean_ctor_set(v_reuseFailAlloc_51_, 0, v___x_48_);
v___x_50_ = v_reuseFailAlloc_51_;
goto v_reusejp_49_;
}
v_reusejp_49_:
{
return v___x_50_;
}
}
else
{
uint8_t v___x_52_; uint8_t v___x_53_; uint8_t v___x_54_; lean_object* v___x_55_; lean_object* v___x_57_; 
v___x_52_ = lean_unbox(v_a_39_);
lean_dec(v_a_39_);
v___x_53_ = lean_unbox(v_a_41_);
lean_dec(v_a_41_);
v___x_54_ = lean_uint8_dec_eq(v___x_52_, v___x_53_);
v___x_55_ = lean_box(v___x_54_);
if (v_isShared_44_ == 0)
{
lean_ctor_set(v___x_43_, 0, v___x_55_);
v___x_57_ = v___x_43_;
goto v_reusejp_56_;
}
else
{
lean_object* v_reuseFailAlloc_58_; 
v_reuseFailAlloc_58_ = lean_alloc_ctor(0, 1, 0);
lean_ctor_set(v_reuseFailAlloc_58_, 0, v___x_55_);
v___x_57_ = v_reuseFailAlloc_58_;
goto v_reusejp_56_;
}
v_reusejp_56_:
{
return v___x_57_;
}
}
}
}
else
{
lean_object* v_a_60_; lean_object* v___x_62_; uint8_t v_isShared_63_; uint8_t v_isSharedCheck_67_; 
lean_dec(v_a_39_);
lean_dec(v_a_37_);
lean_dec(v_a_35_);
v_a_60_ = lean_ctor_get(v___x_40_, 0);
v_isSharedCheck_67_ = !lean_is_exclusive(v___x_40_);
if (v_isSharedCheck_67_ == 0)
{
v___x_62_ = v___x_40_;
v_isShared_63_ = v_isSharedCheck_67_;
goto v_resetjp_61_;
}
else
{
lean_inc(v_a_60_);
lean_dec(v___x_40_);
v___x_62_ = lean_box(0);
v_isShared_63_ = v_isSharedCheck_67_;
goto v_resetjp_61_;
}
v_resetjp_61_:
{
lean_object* v___x_65_; 
if (v_isShared_63_ == 0)
{
v___x_65_ = v___x_62_;
goto v_reusejp_64_;
}
else
{
lean_object* v_reuseFailAlloc_66_; 
v_reuseFailAlloc_66_ = lean_alloc_ctor(1, 1, 0);
lean_ctor_set(v_reuseFailAlloc_66_, 0, v_a_60_);
v___x_65_ = v_reuseFailAlloc_66_;
goto v_reusejp_64_;
}
v_reusejp_64_:
{
return v___x_65_;
}
}
}
}
else
{
lean_object* v_a_68_; lean_object* v___x_70_; uint8_t v_isShared_71_; uint8_t v_isSharedCheck_75_; 
lean_dec(v_a_37_);
lean_dec(v_a_35_);
v_a_68_ = lean_ctor_get(v___x_38_, 0);
v_isSharedCheck_75_ = !lean_is_exclusive(v___x_38_);
if (v_isSharedCheck_75_ == 0)
{
v___x_70_ = v___x_38_;
v_isShared_71_ = v_isSharedCheck_75_;
goto v_resetjp_69_;
}
else
{
lean_inc(v_a_68_);
lean_dec(v___x_38_);
v___x_70_ = lean_box(0);
v_isShared_71_ = v_isSharedCheck_75_;
goto v_resetjp_69_;
}
v_resetjp_69_:
{
lean_object* v___x_73_; 
if (v_isShared_71_ == 0)
{
v___x_73_ = v___x_70_;
goto v_reusejp_72_;
}
else
{
lean_object* v_reuseFailAlloc_74_; 
v_reuseFailAlloc_74_ = lean_alloc_ctor(1, 1, 0);
lean_ctor_set(v_reuseFailAlloc_74_, 0, v_a_68_);
v___x_73_ = v_reuseFailAlloc_74_;
goto v_reusejp_72_;
}
v_reusejp_72_:
{
return v___x_73_;
}
}
}
}
else
{
lean_object* v_a_76_; lean_object* v___x_78_; uint8_t v_isShared_79_; uint8_t v_isSharedCheck_83_; 
lean_dec(v_a_35_);
v_a_76_ = lean_ctor_get(v___x_36_, 0);
v_isSharedCheck_83_ = !lean_is_exclusive(v___x_36_);
if (v_isSharedCheck_83_ == 0)
{
v___x_78_ = v___x_36_;
v_isShared_79_ = v_isSharedCheck_83_;
goto v_resetjp_77_;
}
else
{
lean_inc(v_a_76_);
lean_dec(v___x_36_);
v___x_78_ = lean_box(0);
v_isShared_79_ = v_isSharedCheck_83_;
goto v_resetjp_77_;
}
v_resetjp_77_:
{
lean_object* v___x_81_; 
if (v_isShared_79_ == 0)
{
v___x_81_ = v___x_78_;
goto v_reusejp_80_;
}
else
{
lean_object* v_reuseFailAlloc_82_; 
v_reuseFailAlloc_82_ = lean_alloc_ctor(1, 1, 0);
lean_ctor_set(v_reuseFailAlloc_82_, 0, v_a_76_);
v___x_81_ = v_reuseFailAlloc_82_;
goto v_reusejp_80_;
}
v_reusejp_80_:
{
return v___x_81_;
}
}
}
}
else
{
lean_object* v_a_84_; lean_object* v___x_86_; uint8_t v_isShared_87_; uint8_t v_isSharedCheck_91_; 
v_a_84_ = lean_ctor_get(v___x_34_, 0);
v_isSharedCheck_91_ = !lean_is_exclusive(v___x_34_);
if (v_isSharedCheck_91_ == 0)
{
v___x_86_ = v___x_34_;
v_isShared_87_ = v_isSharedCheck_91_;
goto v_resetjp_85_;
}
else
{
lean_inc(v_a_84_);
lean_dec(v___x_34_);
v___x_86_ = lean_box(0);
v_isShared_87_ = v_isSharedCheck_91_;
goto v_resetjp_85_;
}
v_resetjp_85_:
{
lean_object* v___x_89_; 
if (v_isShared_87_ == 0)
{
v___x_89_ = v___x_86_;
goto v_reusejp_88_;
}
else
{
lean_object* v_reuseFailAlloc_90_; 
v_reuseFailAlloc_90_ = lean_alloc_ctor(1, 1, 0);
lean_ctor_set(v_reuseFailAlloc_90_, 0, v_a_84_);
v___x_89_ = v_reuseFailAlloc_90_;
goto v_reusejp_88_;
}
v_reusejp_88_:
{
return v___x_89_;
}
}
}
}
}
LEAN_EXPORT lean_object* lp_zhc__lean_Zhc_Ffi_L2r_SpecBlock_same___boxed(lean_object* v_a_92_, lean_object* v_b_93_, lean_object* v_a_94_){
_start:
{
lean_object* v_res_95_; 
v_res_95_ = lp_zhc__lean_Zhc_Ffi_L2r_SpecBlock_same(v_a_92_, v_b_93_);
lean_dec(v_b_93_);
lean_dec(v_a_92_);
return v_res_95_;
}
}
lean_object* initialize_Init(uint8_t builtin);
lean_object* initialize_Init(uint8_t builtin);
lean_object* initialize_zhc__lean_Zhc_Model_SpecBlock(uint8_t builtin);
void lean_initialize_runtime_module();
static bool _G_initialized = false;
LEAN_EXPORT lean_object* initialize_zhc__lean_Zhc_Ffi_L2r_SpecBlock(uint8_t builtin) {
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
res = initialize_zhc__lean_Zhc_Model_SpecBlock(builtin);
if (lean_io_result_is_error(res)) return res;
lean_dec_ref(res);
lp_zhc__lean_Zhc_Ffi_L2r_SpecBlockPointed = _init_lp_zhc__lean_Zhc_Ffi_L2r_SpecBlockPointed();
return lean_io_result_mk_ok(lean_box(0));
}
#ifdef __cplusplus
}
#endif
