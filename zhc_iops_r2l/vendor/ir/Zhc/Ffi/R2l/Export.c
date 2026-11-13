// Lean compiler output
// Module: Zhc.Ffi.R2l.Export
// Imports: public import Init public meta import Init public import Zhc.Ffi.R2l.Macro public import Zhc.Iops.AddRippleCarry.Theorems public import Zhc.Iops.AddHillisSteele.Theorems
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
lean_object* lp_zhc__lean_Zhc_Ffi_R2l_inputCt(lean_object*, lean_object*, lean_object*);
lean_object* lp_zhc__lean_Zhc_Ffi_R2l_bindCt(lean_object*, lean_object*);
lean_object* lp_zhc__lean_Zhc_Ffi_R2l_checkCt(lean_object*, lean_object*, lean_object*);
lean_object* lp_zhc__lean_Zhc_Ffi_R2l_checkBuilder(lean_object*, lean_object*);
lean_object* lp_zhc__lean_Zhc_Ffi_R2l_tracedCt(lean_object*, lean_object*, lean_object*);
lean_object* lp_zhc__lean_Zhc_Iops_addRippleCarry___at___00Zhc_Iops_addRippleCarryCircuit_spec__0(lean_object*, lean_object*, lean_object*, lean_object*);
lean_object* zhc_lean_builder_ciphertext_integer_output(lean_object*, lean_object*);
lean_object* lp_zhc__lean_Zhc_Ffi_L2r_SpecBlock_ofModel(lean_object*);
lean_object* zhc_lean_builder_new(lean_object*);
lean_object* lp_zhc__lean_Zhc_Iops_addHillisSteele___at___00Zhc_Iops_addHillisSteeleCircuit_spec__0(lean_object*, lean_object*, lean_object*, lean_object*);
lean_object* zhc_lean_ciphertext_integer_unbox(lean_object*, size_t);
static const lean_ctor_object lp_zhc__lean_Zhc_Ffi_R2l_add__ripple__carry_run___closed__0_value = {.m_header = {.m_rc = 0, .m_cs_sz = sizeof(lean_ctor_object) + sizeof(void*)*2 + 0, .m_other = 2, .m_tag = 0}, .m_objs = {((lean_object*)(((size_t)(2) << 1) | 1)),((lean_object*)(((size_t)(2) << 1) | 1))}};
static const lean_object* lp_zhc__lean_Zhc_Ffi_R2l_add__ripple__carry_run___closed__0 = (const lean_object*)&lp_zhc__lean_Zhc_Ffi_R2l_add__ripple__carry_run___closed__0_value;
LEAN_EXPORT lean_object* lp_zhc__lean_Zhc_Ffi_R2l_add__ripple__carry_run(lean_object*, lean_object*, lean_object*);
LEAN_EXPORT lean_object* lp_zhc__lean_Zhc_Ffi_R2l_add__ripple__carry_run___boxed(lean_object*, lean_object*, lean_object*, lean_object*);
LEAN_EXPORT lean_object* zhc_lean_export_add_ripple_carry(lean_object*, lean_object*, lean_object*, size_t);
LEAN_EXPORT lean_object* lp_zhc__lean_Zhc_Ffi_R2l_add__ripple__carry_export___boxed(lean_object*, lean_object*, lean_object*, lean_object*, lean_object*);
LEAN_EXPORT lean_object* lp_zhc__lean_Zhc_Ffi_R2l_add__ripple__carry_circuit(lean_object*);
LEAN_EXPORT lean_object* lp_zhc__lean_Zhc_Ffi_R2l_add__ripple__carry_circuit___boxed(lean_object*, lean_object*);
LEAN_EXPORT lean_object* lp_zhc__lean_Zhc_Ffi_R2l_add__hillis__steele_run(lean_object*, lean_object*, lean_object*);
LEAN_EXPORT lean_object* lp_zhc__lean_Zhc_Ffi_R2l_add__hillis__steele_run___boxed(lean_object*, lean_object*, lean_object*, lean_object*);
LEAN_EXPORT lean_object* zhc_lean_export_add_hillis_steele(lean_object*, lean_object*, lean_object*, size_t);
LEAN_EXPORT lean_object* lp_zhc__lean_Zhc_Ffi_R2l_add__hillis__steele_export___boxed(lean_object*, lean_object*, lean_object*, lean_object*, lean_object*);
LEAN_EXPORT lean_object* lp_zhc__lean_Zhc_Ffi_R2l_add__hillis__steele_circuit(lean_object*);
LEAN_EXPORT lean_object* lp_zhc__lean_Zhc_Ffi_R2l_add__hillis__steele_circuit___boxed(lean_object*, lean_object*);
LEAN_EXPORT lean_object* lp_zhc__lean_Zhc_Ffi_R2l_add__ripple__carry_run(lean_object* v_b_3_, lean_object* v_x_4_, lean_object* v_y_5_){
_start:
{
lean_object* v___x_7_; lean_object* v___x_8_; 
v___x_7_ = ((lean_object*)(lp_zhc__lean_Zhc_Ffi_R2l_add__ripple__carry_run___closed__0));
v___x_8_ = lp_zhc__lean_Zhc_Ffi_R2l_bindCt(v___x_7_, v_x_4_);
if (lean_obj_tag(v___x_8_) == 0)
{
lean_object* v_a_9_; lean_object* v___x_10_; 
v_a_9_ = lean_ctor_get(v___x_8_, 0);
lean_inc(v_a_9_);
lean_dec_ref_known(v___x_8_, 1);
v___x_10_ = lp_zhc__lean_Zhc_Ffi_R2l_checkCt(v___x_7_, v_a_9_, v_x_4_);
if (lean_obj_tag(v___x_10_) == 0)
{
lean_object* v___x_11_; 
lean_dec_ref_known(v___x_10_, 1);
v___x_11_ = lp_zhc__lean_Zhc_Ffi_R2l_checkCt(v___x_7_, v_a_9_, v_y_5_);
if (lean_obj_tag(v___x_11_) == 0)
{
lean_object* v___x_12_; 
lean_dec_ref_known(v___x_11_, 1);
v___x_12_ = lp_zhc__lean_Zhc_Ffi_R2l_checkBuilder(v_b_3_, v___x_7_);
if (lean_obj_tag(v___x_12_) == 0)
{
lean_object* v___x_13_; lean_object* v___x_14_; lean_object* v___x_15_; 
lean_dec_ref_known(v___x_12_, 1);
v___x_13_ = lp_zhc__lean_Zhc_Ffi_R2l_tracedCt(v___x_7_, v_a_9_, v_x_4_);
v___x_14_ = lp_zhc__lean_Zhc_Ffi_R2l_tracedCt(v___x_7_, v_a_9_, v_y_5_);
v___x_15_ = lp_zhc__lean_Zhc_Iops_addRippleCarry___at___00Zhc_Iops_addRippleCarryCircuit_spec__0(v_a_9_, v___x_13_, v___x_14_, v_b_3_);
if (lean_obj_tag(v___x_15_) == 0)
{
lean_object* v_a_16_; lean_object* v___x_18_; uint8_t v_isShared_19_; uint8_t v_isSharedCheck_24_; 
v_a_16_ = lean_ctor_get(v___x_15_, 0);
v_isSharedCheck_24_ = !lean_is_exclusive(v___x_15_);
if (v_isSharedCheck_24_ == 0)
{
v___x_18_ = v___x_15_;
v_isShared_19_ = v_isSharedCheck_24_;
goto v_resetjp_17_;
}
else
{
lean_inc(v_a_16_);
lean_dec(v___x_15_);
v___x_18_ = lean_box(0);
v_isShared_19_ = v_isSharedCheck_24_;
goto v_resetjp_17_;
}
v_resetjp_17_:
{
lean_object* v_handle_20_; lean_object* v___x_22_; 
v_handle_20_ = lean_ctor_get(v_a_16_, 0);
lean_inc(v_handle_20_);
lean_dec(v_a_16_);
if (v_isShared_19_ == 0)
{
lean_ctor_set(v___x_18_, 0, v_handle_20_);
v___x_22_ = v___x_18_;
goto v_reusejp_21_;
}
else
{
lean_object* v_reuseFailAlloc_23_; 
v_reuseFailAlloc_23_ = lean_alloc_ctor(0, 1, 0);
lean_ctor_set(v_reuseFailAlloc_23_, 0, v_handle_20_);
v___x_22_ = v_reuseFailAlloc_23_;
goto v_reusejp_21_;
}
v_reusejp_21_:
{
return v___x_22_;
}
}
}
else
{
lean_object* v_a_25_; lean_object* v___x_27_; uint8_t v_isShared_28_; uint8_t v_isSharedCheck_32_; 
v_a_25_ = lean_ctor_get(v___x_15_, 0);
v_isSharedCheck_32_ = !lean_is_exclusive(v___x_15_);
if (v_isSharedCheck_32_ == 0)
{
v___x_27_ = v___x_15_;
v_isShared_28_ = v_isSharedCheck_32_;
goto v_resetjp_26_;
}
else
{
lean_inc(v_a_25_);
lean_dec(v___x_15_);
v___x_27_ = lean_box(0);
v_isShared_28_ = v_isSharedCheck_32_;
goto v_resetjp_26_;
}
v_resetjp_26_:
{
lean_object* v___x_30_; 
if (v_isShared_28_ == 0)
{
v___x_30_ = v___x_27_;
goto v_reusejp_29_;
}
else
{
lean_object* v_reuseFailAlloc_31_; 
v_reuseFailAlloc_31_ = lean_alloc_ctor(1, 1, 0);
lean_ctor_set(v_reuseFailAlloc_31_, 0, v_a_25_);
v___x_30_ = v_reuseFailAlloc_31_;
goto v_reusejp_29_;
}
v_reusejp_29_:
{
return v___x_30_;
}
}
}
}
else
{
lean_object* v_a_33_; lean_object* v___x_35_; uint8_t v_isShared_36_; uint8_t v_isSharedCheck_40_; 
lean_dec(v_a_9_);
lean_dec(v_y_5_);
lean_dec(v_x_4_);
v_a_33_ = lean_ctor_get(v___x_12_, 0);
v_isSharedCheck_40_ = !lean_is_exclusive(v___x_12_);
if (v_isSharedCheck_40_ == 0)
{
v___x_35_ = v___x_12_;
v_isShared_36_ = v_isSharedCheck_40_;
goto v_resetjp_34_;
}
else
{
lean_inc(v_a_33_);
lean_dec(v___x_12_);
v___x_35_ = lean_box(0);
v_isShared_36_ = v_isSharedCheck_40_;
goto v_resetjp_34_;
}
v_resetjp_34_:
{
lean_object* v___x_38_; 
if (v_isShared_36_ == 0)
{
v___x_38_ = v___x_35_;
goto v_reusejp_37_;
}
else
{
lean_object* v_reuseFailAlloc_39_; 
v_reuseFailAlloc_39_ = lean_alloc_ctor(1, 1, 0);
lean_ctor_set(v_reuseFailAlloc_39_, 0, v_a_33_);
v___x_38_ = v_reuseFailAlloc_39_;
goto v_reusejp_37_;
}
v_reusejp_37_:
{
return v___x_38_;
}
}
}
}
else
{
lean_object* v_a_41_; lean_object* v___x_43_; uint8_t v_isShared_44_; uint8_t v_isSharedCheck_48_; 
lean_dec(v_a_9_);
lean_dec(v_y_5_);
lean_dec(v_x_4_);
v_a_41_ = lean_ctor_get(v___x_11_, 0);
v_isSharedCheck_48_ = !lean_is_exclusive(v___x_11_);
if (v_isSharedCheck_48_ == 0)
{
v___x_43_ = v___x_11_;
v_isShared_44_ = v_isSharedCheck_48_;
goto v_resetjp_42_;
}
else
{
lean_inc(v_a_41_);
lean_dec(v___x_11_);
v___x_43_ = lean_box(0);
v_isShared_44_ = v_isSharedCheck_48_;
goto v_resetjp_42_;
}
v_resetjp_42_:
{
lean_object* v___x_46_; 
if (v_isShared_44_ == 0)
{
v___x_46_ = v___x_43_;
goto v_reusejp_45_;
}
else
{
lean_object* v_reuseFailAlloc_47_; 
v_reuseFailAlloc_47_ = lean_alloc_ctor(1, 1, 0);
lean_ctor_set(v_reuseFailAlloc_47_, 0, v_a_41_);
v___x_46_ = v_reuseFailAlloc_47_;
goto v_reusejp_45_;
}
v_reusejp_45_:
{
return v___x_46_;
}
}
}
}
else
{
lean_object* v_a_49_; lean_object* v___x_51_; uint8_t v_isShared_52_; uint8_t v_isSharedCheck_56_; 
lean_dec(v_a_9_);
lean_dec(v_y_5_);
lean_dec(v_x_4_);
v_a_49_ = lean_ctor_get(v___x_10_, 0);
v_isSharedCheck_56_ = !lean_is_exclusive(v___x_10_);
if (v_isSharedCheck_56_ == 0)
{
v___x_51_ = v___x_10_;
v_isShared_52_ = v_isSharedCheck_56_;
goto v_resetjp_50_;
}
else
{
lean_inc(v_a_49_);
lean_dec(v___x_10_);
v___x_51_ = lean_box(0);
v_isShared_52_ = v_isSharedCheck_56_;
goto v_resetjp_50_;
}
v_resetjp_50_:
{
lean_object* v___x_54_; 
if (v_isShared_52_ == 0)
{
v___x_54_ = v___x_51_;
goto v_reusejp_53_;
}
else
{
lean_object* v_reuseFailAlloc_55_; 
v_reuseFailAlloc_55_ = lean_alloc_ctor(1, 1, 0);
lean_ctor_set(v_reuseFailAlloc_55_, 0, v_a_49_);
v___x_54_ = v_reuseFailAlloc_55_;
goto v_reusejp_53_;
}
v_reusejp_53_:
{
return v___x_54_;
}
}
}
}
else
{
lean_object* v_a_57_; lean_object* v___x_59_; uint8_t v_isShared_60_; uint8_t v_isSharedCheck_64_; 
lean_dec(v_y_5_);
lean_dec(v_x_4_);
v_a_57_ = lean_ctor_get(v___x_8_, 0);
v_isSharedCheck_64_ = !lean_is_exclusive(v___x_8_);
if (v_isSharedCheck_64_ == 0)
{
v___x_59_ = v___x_8_;
v_isShared_60_ = v_isSharedCheck_64_;
goto v_resetjp_58_;
}
else
{
lean_inc(v_a_57_);
lean_dec(v___x_8_);
v___x_59_ = lean_box(0);
v_isShared_60_ = v_isSharedCheck_64_;
goto v_resetjp_58_;
}
v_resetjp_58_:
{
lean_object* v___x_62_; 
if (v_isShared_60_ == 0)
{
v___x_62_ = v___x_59_;
goto v_reusejp_61_;
}
else
{
lean_object* v_reuseFailAlloc_63_; 
v_reuseFailAlloc_63_ = lean_alloc_ctor(1, 1, 0);
lean_ctor_set(v_reuseFailAlloc_63_, 0, v_a_57_);
v___x_62_ = v_reuseFailAlloc_63_;
goto v_reusejp_61_;
}
v_reusejp_61_:
{
return v___x_62_;
}
}
}
}
}
LEAN_EXPORT lean_object* lp_zhc__lean_Zhc_Ffi_R2l_add__ripple__carry_run___boxed(lean_object* v_b_65_, lean_object* v_x_66_, lean_object* v_y_67_, lean_object* v_a_68_){
_start:
{
lean_object* v_res_69_; 
v_res_69_ = lp_zhc__lean_Zhc_Ffi_R2l_add__ripple__carry_run(v_b_65_, v_x_66_, v_y_67_);
lean_dec(v_b_65_);
return v_res_69_;
}
}
LEAN_EXPORT lean_object* zhc_lean_export_add_ripple_carry(lean_object* v_b_70_, lean_object* v_x_71_, lean_object* v_y_72_, size_t v_out0_73_){
_start:
{
lean_object* v___x_75_; 
v___x_75_ = lp_zhc__lean_Zhc_Ffi_R2l_add__ripple__carry_run(v_b_70_, v_x_71_, v_y_72_);
lean_dec(v_b_70_);
if (lean_obj_tag(v___x_75_) == 0)
{
lean_object* v_a_76_; lean_object* v___x_77_; 
v_a_76_ = lean_ctor_get(v___x_75_, 0);
lean_inc(v_a_76_);
lean_dec_ref_known(v___x_75_, 1);
v___x_77_ = zhc_lean_ciphertext_integer_unbox(v_a_76_, v_out0_73_);
lean_dec(v_a_76_);
return v___x_77_;
}
else
{
lean_object* v_a_78_; lean_object* v___x_80_; uint8_t v_isShared_81_; uint8_t v_isSharedCheck_85_; 
v_a_78_ = lean_ctor_get(v___x_75_, 0);
v_isSharedCheck_85_ = !lean_is_exclusive(v___x_75_);
if (v_isSharedCheck_85_ == 0)
{
v___x_80_ = v___x_75_;
v_isShared_81_ = v_isSharedCheck_85_;
goto v_resetjp_79_;
}
else
{
lean_inc(v_a_78_);
lean_dec(v___x_75_);
v___x_80_ = lean_box(0);
v_isShared_81_ = v_isSharedCheck_85_;
goto v_resetjp_79_;
}
v_resetjp_79_:
{
lean_object* v___x_83_; 
if (v_isShared_81_ == 0)
{
v___x_83_ = v___x_80_;
goto v_reusejp_82_;
}
else
{
lean_object* v_reuseFailAlloc_84_; 
v_reuseFailAlloc_84_ = lean_alloc_ctor(1, 1, 0);
lean_ctor_set(v_reuseFailAlloc_84_, 0, v_a_78_);
v___x_83_ = v_reuseFailAlloc_84_;
goto v_reusejp_82_;
}
v_reusejp_82_:
{
return v___x_83_;
}
}
}
}
}
LEAN_EXPORT lean_object* lp_zhc__lean_Zhc_Ffi_R2l_add__ripple__carry_export___boxed(lean_object* v_b_86_, lean_object* v_x_87_, lean_object* v_y_88_, lean_object* v_out0_89_, lean_object* v_a_90_){
_start:
{
size_t v_out0_49__boxed_91_; lean_object* v_res_92_; 
v_out0_49__boxed_91_ = lean_unbox_usize(v_out0_89_);
lean_dec(v_out0_89_);
v_res_92_ = zhc_lean_export_add_ripple_carry(v_b_86_, v_x_87_, v_y_88_, v_out0_49__boxed_91_);
return v_res_92_;
}
}
LEAN_EXPORT lean_object* lp_zhc__lean_Zhc_Ffi_R2l_add__ripple__carry_circuit(lean_object* v_si_93_){
_start:
{
lean_object* v___x_95_; lean_object* v___y_97_; lean_object* v___x_122_; 
v___x_95_ = ((lean_object*)(lp_zhc__lean_Zhc_Ffi_R2l_add__ripple__carry_run___closed__0));
v___x_122_ = lp_zhc__lean_Zhc_Ffi_L2r_SpecBlock_ofModel(v___x_95_);
if (lean_obj_tag(v___x_122_) == 0)
{
lean_object* v_a_123_; lean_object* v___x_124_; 
v_a_123_ = lean_ctor_get(v___x_122_, 0);
lean_inc(v_a_123_);
lean_dec_ref_known(v___x_122_, 1);
v___x_124_ = zhc_lean_builder_new(v_a_123_);
lean_dec(v_a_123_);
v___y_97_ = v___x_124_;
goto v___jp_96_;
}
else
{
v___y_97_ = v___x_122_;
goto v___jp_96_;
}
v___jp_96_:
{
if (lean_obj_tag(v___y_97_) == 0)
{
lean_object* v_a_98_; lean_object* v___x_99_; 
v_a_98_ = lean_ctor_get(v___y_97_, 0);
lean_inc(v_a_98_);
lean_dec_ref_known(v___y_97_, 1);
v___x_99_ = lp_zhc__lean_Zhc_Ffi_R2l_inputCt(v___x_95_, v_a_98_, v_si_93_);
if (lean_obj_tag(v___x_99_) == 0)
{
lean_object* v_a_100_; lean_object* v___x_101_; 
v_a_100_ = lean_ctor_get(v___x_99_, 0);
lean_inc(v_a_100_);
lean_dec_ref_known(v___x_99_, 1);
v___x_101_ = lp_zhc__lean_Zhc_Ffi_R2l_inputCt(v___x_95_, v_a_98_, v_si_93_);
if (lean_obj_tag(v___x_101_) == 0)
{
lean_object* v_a_102_; lean_object* v___x_103_; 
v_a_102_ = lean_ctor_get(v___x_101_, 0);
lean_inc(v_a_102_);
lean_dec_ref_known(v___x_101_, 1);
v___x_103_ = lp_zhc__lean_Zhc_Ffi_R2l_add__ripple__carry_run(v_a_98_, v_a_100_, v_a_102_);
if (lean_obj_tag(v___x_103_) == 0)
{
lean_object* v_a_104_; lean_object* v___x_105_; 
v_a_104_ = lean_ctor_get(v___x_103_, 0);
lean_inc(v_a_104_);
lean_dec_ref_known(v___x_103_, 1);
v___x_105_ = zhc_lean_builder_ciphertext_integer_output(v_a_98_, v_a_104_);
lean_dec(v_a_104_);
if (lean_obj_tag(v___x_105_) == 0)
{
lean_object* v___x_107_; uint8_t v_isShared_108_; uint8_t v_isSharedCheck_112_; 
v_isSharedCheck_112_ = !lean_is_exclusive(v___x_105_);
if (v_isSharedCheck_112_ == 0)
{
lean_object* v_unused_113_; 
v_unused_113_ = lean_ctor_get(v___x_105_, 0);
lean_dec(v_unused_113_);
v___x_107_ = v___x_105_;
v_isShared_108_ = v_isSharedCheck_112_;
goto v_resetjp_106_;
}
else
{
lean_dec(v___x_105_);
v___x_107_ = lean_box(0);
v_isShared_108_ = v_isSharedCheck_112_;
goto v_resetjp_106_;
}
v_resetjp_106_:
{
lean_object* v___x_110_; 
if (v_isShared_108_ == 0)
{
lean_ctor_set(v___x_107_, 0, v_a_98_);
v___x_110_ = v___x_107_;
goto v_reusejp_109_;
}
else
{
lean_object* v_reuseFailAlloc_111_; 
v_reuseFailAlloc_111_ = lean_alloc_ctor(0, 1, 0);
lean_ctor_set(v_reuseFailAlloc_111_, 0, v_a_98_);
v___x_110_ = v_reuseFailAlloc_111_;
goto v_reusejp_109_;
}
v_reusejp_109_:
{
return v___x_110_;
}
}
}
else
{
lean_object* v_a_114_; lean_object* v___x_116_; uint8_t v_isShared_117_; uint8_t v_isSharedCheck_121_; 
lean_dec(v_a_98_);
v_a_114_ = lean_ctor_get(v___x_105_, 0);
v_isSharedCheck_121_ = !lean_is_exclusive(v___x_105_);
if (v_isSharedCheck_121_ == 0)
{
v___x_116_ = v___x_105_;
v_isShared_117_ = v_isSharedCheck_121_;
goto v_resetjp_115_;
}
else
{
lean_inc(v_a_114_);
lean_dec(v___x_105_);
v___x_116_ = lean_box(0);
v_isShared_117_ = v_isSharedCheck_121_;
goto v_resetjp_115_;
}
v_resetjp_115_:
{
lean_object* v___x_119_; 
if (v_isShared_117_ == 0)
{
v___x_119_ = v___x_116_;
goto v_reusejp_118_;
}
else
{
lean_object* v_reuseFailAlloc_120_; 
v_reuseFailAlloc_120_ = lean_alloc_ctor(1, 1, 0);
lean_ctor_set(v_reuseFailAlloc_120_, 0, v_a_114_);
v___x_119_ = v_reuseFailAlloc_120_;
goto v_reusejp_118_;
}
v_reusejp_118_:
{
return v___x_119_;
}
}
}
}
else
{
lean_dec(v_a_98_);
return v___x_103_;
}
}
else
{
lean_dec(v_a_100_);
lean_dec(v_a_98_);
return v___x_101_;
}
}
else
{
lean_dec(v_a_98_);
return v___x_99_;
}
}
else
{
return v___y_97_;
}
}
}
}
LEAN_EXPORT lean_object* lp_zhc__lean_Zhc_Ffi_R2l_add__ripple__carry_circuit___boxed(lean_object* v_si_125_, lean_object* v_a_126_){
_start:
{
lean_object* v_res_127_; 
v_res_127_ = lp_zhc__lean_Zhc_Ffi_R2l_add__ripple__carry_circuit(v_si_125_);
lean_dec(v_si_125_);
return v_res_127_;
}
}
LEAN_EXPORT lean_object* lp_zhc__lean_Zhc_Ffi_R2l_add__hillis__steele_run(lean_object* v_b_128_, lean_object* v_x_129_, lean_object* v_y_130_){
_start:
{
lean_object* v___x_132_; lean_object* v___x_133_; 
v___x_132_ = ((lean_object*)(lp_zhc__lean_Zhc_Ffi_R2l_add__ripple__carry_run___closed__0));
v___x_133_ = lp_zhc__lean_Zhc_Ffi_R2l_bindCt(v___x_132_, v_x_129_);
if (lean_obj_tag(v___x_133_) == 0)
{
lean_object* v_a_134_; lean_object* v___x_135_; 
v_a_134_ = lean_ctor_get(v___x_133_, 0);
lean_inc(v_a_134_);
lean_dec_ref_known(v___x_133_, 1);
v___x_135_ = lp_zhc__lean_Zhc_Ffi_R2l_checkCt(v___x_132_, v_a_134_, v_x_129_);
if (lean_obj_tag(v___x_135_) == 0)
{
lean_object* v___x_136_; 
lean_dec_ref_known(v___x_135_, 1);
v___x_136_ = lp_zhc__lean_Zhc_Ffi_R2l_checkCt(v___x_132_, v_a_134_, v_y_130_);
if (lean_obj_tag(v___x_136_) == 0)
{
lean_object* v___x_137_; 
lean_dec_ref_known(v___x_136_, 1);
v___x_137_ = lp_zhc__lean_Zhc_Ffi_R2l_checkBuilder(v_b_128_, v___x_132_);
if (lean_obj_tag(v___x_137_) == 0)
{
lean_object* v___x_138_; lean_object* v___x_139_; lean_object* v___x_140_; 
lean_dec_ref_known(v___x_137_, 1);
v___x_138_ = lp_zhc__lean_Zhc_Ffi_R2l_tracedCt(v___x_132_, v_a_134_, v_x_129_);
v___x_139_ = lp_zhc__lean_Zhc_Ffi_R2l_tracedCt(v___x_132_, v_a_134_, v_y_130_);
v___x_140_ = lp_zhc__lean_Zhc_Iops_addHillisSteele___at___00Zhc_Iops_addHillisSteeleCircuit_spec__0(v_a_134_, v___x_138_, v___x_139_, v_b_128_);
if (lean_obj_tag(v___x_140_) == 0)
{
lean_object* v_a_141_; lean_object* v___x_143_; uint8_t v_isShared_144_; uint8_t v_isSharedCheck_149_; 
v_a_141_ = lean_ctor_get(v___x_140_, 0);
v_isSharedCheck_149_ = !lean_is_exclusive(v___x_140_);
if (v_isSharedCheck_149_ == 0)
{
v___x_143_ = v___x_140_;
v_isShared_144_ = v_isSharedCheck_149_;
goto v_resetjp_142_;
}
else
{
lean_inc(v_a_141_);
lean_dec(v___x_140_);
v___x_143_ = lean_box(0);
v_isShared_144_ = v_isSharedCheck_149_;
goto v_resetjp_142_;
}
v_resetjp_142_:
{
lean_object* v_handle_145_; lean_object* v___x_147_; 
v_handle_145_ = lean_ctor_get(v_a_141_, 0);
lean_inc(v_handle_145_);
lean_dec(v_a_141_);
if (v_isShared_144_ == 0)
{
lean_ctor_set(v___x_143_, 0, v_handle_145_);
v___x_147_ = v___x_143_;
goto v_reusejp_146_;
}
else
{
lean_object* v_reuseFailAlloc_148_; 
v_reuseFailAlloc_148_ = lean_alloc_ctor(0, 1, 0);
lean_ctor_set(v_reuseFailAlloc_148_, 0, v_handle_145_);
v___x_147_ = v_reuseFailAlloc_148_;
goto v_reusejp_146_;
}
v_reusejp_146_:
{
return v___x_147_;
}
}
}
else
{
lean_object* v_a_150_; lean_object* v___x_152_; uint8_t v_isShared_153_; uint8_t v_isSharedCheck_157_; 
v_a_150_ = lean_ctor_get(v___x_140_, 0);
v_isSharedCheck_157_ = !lean_is_exclusive(v___x_140_);
if (v_isSharedCheck_157_ == 0)
{
v___x_152_ = v___x_140_;
v_isShared_153_ = v_isSharedCheck_157_;
goto v_resetjp_151_;
}
else
{
lean_inc(v_a_150_);
lean_dec(v___x_140_);
v___x_152_ = lean_box(0);
v_isShared_153_ = v_isSharedCheck_157_;
goto v_resetjp_151_;
}
v_resetjp_151_:
{
lean_object* v___x_155_; 
if (v_isShared_153_ == 0)
{
v___x_155_ = v___x_152_;
goto v_reusejp_154_;
}
else
{
lean_object* v_reuseFailAlloc_156_; 
v_reuseFailAlloc_156_ = lean_alloc_ctor(1, 1, 0);
lean_ctor_set(v_reuseFailAlloc_156_, 0, v_a_150_);
v___x_155_ = v_reuseFailAlloc_156_;
goto v_reusejp_154_;
}
v_reusejp_154_:
{
return v___x_155_;
}
}
}
}
else
{
lean_object* v_a_158_; lean_object* v___x_160_; uint8_t v_isShared_161_; uint8_t v_isSharedCheck_165_; 
lean_dec(v_a_134_);
lean_dec(v_y_130_);
lean_dec(v_x_129_);
v_a_158_ = lean_ctor_get(v___x_137_, 0);
v_isSharedCheck_165_ = !lean_is_exclusive(v___x_137_);
if (v_isSharedCheck_165_ == 0)
{
v___x_160_ = v___x_137_;
v_isShared_161_ = v_isSharedCheck_165_;
goto v_resetjp_159_;
}
else
{
lean_inc(v_a_158_);
lean_dec(v___x_137_);
v___x_160_ = lean_box(0);
v_isShared_161_ = v_isSharedCheck_165_;
goto v_resetjp_159_;
}
v_resetjp_159_:
{
lean_object* v___x_163_; 
if (v_isShared_161_ == 0)
{
v___x_163_ = v___x_160_;
goto v_reusejp_162_;
}
else
{
lean_object* v_reuseFailAlloc_164_; 
v_reuseFailAlloc_164_ = lean_alloc_ctor(1, 1, 0);
lean_ctor_set(v_reuseFailAlloc_164_, 0, v_a_158_);
v___x_163_ = v_reuseFailAlloc_164_;
goto v_reusejp_162_;
}
v_reusejp_162_:
{
return v___x_163_;
}
}
}
}
else
{
lean_object* v_a_166_; lean_object* v___x_168_; uint8_t v_isShared_169_; uint8_t v_isSharedCheck_173_; 
lean_dec(v_a_134_);
lean_dec(v_y_130_);
lean_dec(v_x_129_);
v_a_166_ = lean_ctor_get(v___x_136_, 0);
v_isSharedCheck_173_ = !lean_is_exclusive(v___x_136_);
if (v_isSharedCheck_173_ == 0)
{
v___x_168_ = v___x_136_;
v_isShared_169_ = v_isSharedCheck_173_;
goto v_resetjp_167_;
}
else
{
lean_inc(v_a_166_);
lean_dec(v___x_136_);
v___x_168_ = lean_box(0);
v_isShared_169_ = v_isSharedCheck_173_;
goto v_resetjp_167_;
}
v_resetjp_167_:
{
lean_object* v___x_171_; 
if (v_isShared_169_ == 0)
{
v___x_171_ = v___x_168_;
goto v_reusejp_170_;
}
else
{
lean_object* v_reuseFailAlloc_172_; 
v_reuseFailAlloc_172_ = lean_alloc_ctor(1, 1, 0);
lean_ctor_set(v_reuseFailAlloc_172_, 0, v_a_166_);
v___x_171_ = v_reuseFailAlloc_172_;
goto v_reusejp_170_;
}
v_reusejp_170_:
{
return v___x_171_;
}
}
}
}
else
{
lean_object* v_a_174_; lean_object* v___x_176_; uint8_t v_isShared_177_; uint8_t v_isSharedCheck_181_; 
lean_dec(v_a_134_);
lean_dec(v_y_130_);
lean_dec(v_x_129_);
v_a_174_ = lean_ctor_get(v___x_135_, 0);
v_isSharedCheck_181_ = !lean_is_exclusive(v___x_135_);
if (v_isSharedCheck_181_ == 0)
{
v___x_176_ = v___x_135_;
v_isShared_177_ = v_isSharedCheck_181_;
goto v_resetjp_175_;
}
else
{
lean_inc(v_a_174_);
lean_dec(v___x_135_);
v___x_176_ = lean_box(0);
v_isShared_177_ = v_isSharedCheck_181_;
goto v_resetjp_175_;
}
v_resetjp_175_:
{
lean_object* v___x_179_; 
if (v_isShared_177_ == 0)
{
v___x_179_ = v___x_176_;
goto v_reusejp_178_;
}
else
{
lean_object* v_reuseFailAlloc_180_; 
v_reuseFailAlloc_180_ = lean_alloc_ctor(1, 1, 0);
lean_ctor_set(v_reuseFailAlloc_180_, 0, v_a_174_);
v___x_179_ = v_reuseFailAlloc_180_;
goto v_reusejp_178_;
}
v_reusejp_178_:
{
return v___x_179_;
}
}
}
}
else
{
lean_object* v_a_182_; lean_object* v___x_184_; uint8_t v_isShared_185_; uint8_t v_isSharedCheck_189_; 
lean_dec(v_y_130_);
lean_dec(v_x_129_);
v_a_182_ = lean_ctor_get(v___x_133_, 0);
v_isSharedCheck_189_ = !lean_is_exclusive(v___x_133_);
if (v_isSharedCheck_189_ == 0)
{
v___x_184_ = v___x_133_;
v_isShared_185_ = v_isSharedCheck_189_;
goto v_resetjp_183_;
}
else
{
lean_inc(v_a_182_);
lean_dec(v___x_133_);
v___x_184_ = lean_box(0);
v_isShared_185_ = v_isSharedCheck_189_;
goto v_resetjp_183_;
}
v_resetjp_183_:
{
lean_object* v___x_187_; 
if (v_isShared_185_ == 0)
{
v___x_187_ = v___x_184_;
goto v_reusejp_186_;
}
else
{
lean_object* v_reuseFailAlloc_188_; 
v_reuseFailAlloc_188_ = lean_alloc_ctor(1, 1, 0);
lean_ctor_set(v_reuseFailAlloc_188_, 0, v_a_182_);
v___x_187_ = v_reuseFailAlloc_188_;
goto v_reusejp_186_;
}
v_reusejp_186_:
{
return v___x_187_;
}
}
}
}
}
LEAN_EXPORT lean_object* lp_zhc__lean_Zhc_Ffi_R2l_add__hillis__steele_run___boxed(lean_object* v_b_190_, lean_object* v_x_191_, lean_object* v_y_192_, lean_object* v_a_193_){
_start:
{
lean_object* v_res_194_; 
v_res_194_ = lp_zhc__lean_Zhc_Ffi_R2l_add__hillis__steele_run(v_b_190_, v_x_191_, v_y_192_);
lean_dec(v_b_190_);
return v_res_194_;
}
}
LEAN_EXPORT lean_object* zhc_lean_export_add_hillis_steele(lean_object* v_b_195_, lean_object* v_x_196_, lean_object* v_y_197_, size_t v_out0_198_){
_start:
{
lean_object* v___x_200_; 
v___x_200_ = lp_zhc__lean_Zhc_Ffi_R2l_add__hillis__steele_run(v_b_195_, v_x_196_, v_y_197_);
lean_dec(v_b_195_);
if (lean_obj_tag(v___x_200_) == 0)
{
lean_object* v_a_201_; lean_object* v___x_202_; 
v_a_201_ = lean_ctor_get(v___x_200_, 0);
lean_inc(v_a_201_);
lean_dec_ref_known(v___x_200_, 1);
v___x_202_ = zhc_lean_ciphertext_integer_unbox(v_a_201_, v_out0_198_);
lean_dec(v_a_201_);
return v___x_202_;
}
else
{
lean_object* v_a_203_; lean_object* v___x_205_; uint8_t v_isShared_206_; uint8_t v_isSharedCheck_210_; 
v_a_203_ = lean_ctor_get(v___x_200_, 0);
v_isSharedCheck_210_ = !lean_is_exclusive(v___x_200_);
if (v_isSharedCheck_210_ == 0)
{
v___x_205_ = v___x_200_;
v_isShared_206_ = v_isSharedCheck_210_;
goto v_resetjp_204_;
}
else
{
lean_inc(v_a_203_);
lean_dec(v___x_200_);
v___x_205_ = lean_box(0);
v_isShared_206_ = v_isSharedCheck_210_;
goto v_resetjp_204_;
}
v_resetjp_204_:
{
lean_object* v___x_208_; 
if (v_isShared_206_ == 0)
{
v___x_208_ = v___x_205_;
goto v_reusejp_207_;
}
else
{
lean_object* v_reuseFailAlloc_209_; 
v_reuseFailAlloc_209_ = lean_alloc_ctor(1, 1, 0);
lean_ctor_set(v_reuseFailAlloc_209_, 0, v_a_203_);
v___x_208_ = v_reuseFailAlloc_209_;
goto v_reusejp_207_;
}
v_reusejp_207_:
{
return v___x_208_;
}
}
}
}
}
LEAN_EXPORT lean_object* lp_zhc__lean_Zhc_Ffi_R2l_add__hillis__steele_export___boxed(lean_object* v_b_211_, lean_object* v_x_212_, lean_object* v_y_213_, lean_object* v_out0_214_, lean_object* v_a_215_){
_start:
{
size_t v_out0_49__boxed_216_; lean_object* v_res_217_; 
v_out0_49__boxed_216_ = lean_unbox_usize(v_out0_214_);
lean_dec(v_out0_214_);
v_res_217_ = zhc_lean_export_add_hillis_steele(v_b_211_, v_x_212_, v_y_213_, v_out0_49__boxed_216_);
return v_res_217_;
}
}
LEAN_EXPORT lean_object* lp_zhc__lean_Zhc_Ffi_R2l_add__hillis__steele_circuit(lean_object* v_si_218_){
_start:
{
lean_object* v___x_220_; lean_object* v___y_222_; lean_object* v___x_247_; 
v___x_220_ = ((lean_object*)(lp_zhc__lean_Zhc_Ffi_R2l_add__ripple__carry_run___closed__0));
v___x_247_ = lp_zhc__lean_Zhc_Ffi_L2r_SpecBlock_ofModel(v___x_220_);
if (lean_obj_tag(v___x_247_) == 0)
{
lean_object* v_a_248_; lean_object* v___x_249_; 
v_a_248_ = lean_ctor_get(v___x_247_, 0);
lean_inc(v_a_248_);
lean_dec_ref_known(v___x_247_, 1);
v___x_249_ = zhc_lean_builder_new(v_a_248_);
lean_dec(v_a_248_);
v___y_222_ = v___x_249_;
goto v___jp_221_;
}
else
{
v___y_222_ = v___x_247_;
goto v___jp_221_;
}
v___jp_221_:
{
if (lean_obj_tag(v___y_222_) == 0)
{
lean_object* v_a_223_; lean_object* v___x_224_; 
v_a_223_ = lean_ctor_get(v___y_222_, 0);
lean_inc(v_a_223_);
lean_dec_ref_known(v___y_222_, 1);
v___x_224_ = lp_zhc__lean_Zhc_Ffi_R2l_inputCt(v___x_220_, v_a_223_, v_si_218_);
if (lean_obj_tag(v___x_224_) == 0)
{
lean_object* v_a_225_; lean_object* v___x_226_; 
v_a_225_ = lean_ctor_get(v___x_224_, 0);
lean_inc(v_a_225_);
lean_dec_ref_known(v___x_224_, 1);
v___x_226_ = lp_zhc__lean_Zhc_Ffi_R2l_inputCt(v___x_220_, v_a_223_, v_si_218_);
if (lean_obj_tag(v___x_226_) == 0)
{
lean_object* v_a_227_; lean_object* v___x_228_; 
v_a_227_ = lean_ctor_get(v___x_226_, 0);
lean_inc(v_a_227_);
lean_dec_ref_known(v___x_226_, 1);
v___x_228_ = lp_zhc__lean_Zhc_Ffi_R2l_add__hillis__steele_run(v_a_223_, v_a_225_, v_a_227_);
if (lean_obj_tag(v___x_228_) == 0)
{
lean_object* v_a_229_; lean_object* v___x_230_; 
v_a_229_ = lean_ctor_get(v___x_228_, 0);
lean_inc(v_a_229_);
lean_dec_ref_known(v___x_228_, 1);
v___x_230_ = zhc_lean_builder_ciphertext_integer_output(v_a_223_, v_a_229_);
lean_dec(v_a_229_);
if (lean_obj_tag(v___x_230_) == 0)
{
lean_object* v___x_232_; uint8_t v_isShared_233_; uint8_t v_isSharedCheck_237_; 
v_isSharedCheck_237_ = !lean_is_exclusive(v___x_230_);
if (v_isSharedCheck_237_ == 0)
{
lean_object* v_unused_238_; 
v_unused_238_ = lean_ctor_get(v___x_230_, 0);
lean_dec(v_unused_238_);
v___x_232_ = v___x_230_;
v_isShared_233_ = v_isSharedCheck_237_;
goto v_resetjp_231_;
}
else
{
lean_dec(v___x_230_);
v___x_232_ = lean_box(0);
v_isShared_233_ = v_isSharedCheck_237_;
goto v_resetjp_231_;
}
v_resetjp_231_:
{
lean_object* v___x_235_; 
if (v_isShared_233_ == 0)
{
lean_ctor_set(v___x_232_, 0, v_a_223_);
v___x_235_ = v___x_232_;
goto v_reusejp_234_;
}
else
{
lean_object* v_reuseFailAlloc_236_; 
v_reuseFailAlloc_236_ = lean_alloc_ctor(0, 1, 0);
lean_ctor_set(v_reuseFailAlloc_236_, 0, v_a_223_);
v___x_235_ = v_reuseFailAlloc_236_;
goto v_reusejp_234_;
}
v_reusejp_234_:
{
return v___x_235_;
}
}
}
else
{
lean_object* v_a_239_; lean_object* v___x_241_; uint8_t v_isShared_242_; uint8_t v_isSharedCheck_246_; 
lean_dec(v_a_223_);
v_a_239_ = lean_ctor_get(v___x_230_, 0);
v_isSharedCheck_246_ = !lean_is_exclusive(v___x_230_);
if (v_isSharedCheck_246_ == 0)
{
v___x_241_ = v___x_230_;
v_isShared_242_ = v_isSharedCheck_246_;
goto v_resetjp_240_;
}
else
{
lean_inc(v_a_239_);
lean_dec(v___x_230_);
v___x_241_ = lean_box(0);
v_isShared_242_ = v_isSharedCheck_246_;
goto v_resetjp_240_;
}
v_resetjp_240_:
{
lean_object* v___x_244_; 
if (v_isShared_242_ == 0)
{
v___x_244_ = v___x_241_;
goto v_reusejp_243_;
}
else
{
lean_object* v_reuseFailAlloc_245_; 
v_reuseFailAlloc_245_ = lean_alloc_ctor(1, 1, 0);
lean_ctor_set(v_reuseFailAlloc_245_, 0, v_a_239_);
v___x_244_ = v_reuseFailAlloc_245_;
goto v_reusejp_243_;
}
v_reusejp_243_:
{
return v___x_244_;
}
}
}
}
else
{
lean_dec(v_a_223_);
return v___x_228_;
}
}
else
{
lean_dec(v_a_225_);
lean_dec(v_a_223_);
return v___x_226_;
}
}
else
{
lean_dec(v_a_223_);
return v___x_224_;
}
}
else
{
return v___y_222_;
}
}
}
}
LEAN_EXPORT lean_object* lp_zhc__lean_Zhc_Ffi_R2l_add__hillis__steele_circuit___boxed(lean_object* v_si_250_, lean_object* v_a_251_){
_start:
{
lean_object* v_res_252_; 
v_res_252_ = lp_zhc__lean_Zhc_Ffi_R2l_add__hillis__steele_circuit(v_si_250_);
lean_dec(v_si_250_);
return v_res_252_;
}
}
lean_object* initialize_Init(uint8_t builtin);
lean_object* initialize_Init(uint8_t builtin);
lean_object* initialize_zhc__lean_Zhc_Ffi_R2l_Macro(uint8_t builtin);
lean_object* initialize_zhc__lean_Zhc_Iops_AddRippleCarry_Theorems(uint8_t builtin);
lean_object* initialize_zhc__lean_Zhc_Iops_AddHillisSteele_Theorems(uint8_t builtin);
void lean_initialize_runtime_module();
static bool _G_initialized = false;
LEAN_EXPORT lean_object* initialize_zhc__lean_Zhc_Ffi_R2l_Export(uint8_t builtin) {
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
res = initialize_zhc__lean_Zhc_Ffi_R2l_Macro(builtin);
if (lean_io_result_is_error(res)) return res;
lean_dec_ref(res);
res = initialize_zhc__lean_Zhc_Iops_AddRippleCarry_Theorems(builtin);
if (lean_io_result_is_error(res)) return res;
lean_dec_ref(res);
res = initialize_zhc__lean_Zhc_Iops_AddHillisSteele_Theorems(builtin);
if (lean_io_result_is_error(res)) return res;
lean_dec_ref(res);
return lean_io_result_mk_ok(lean_box(0));
}
#ifdef __cplusplus
}
#endif
