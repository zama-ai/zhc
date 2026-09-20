// Lean compiler output
// Module: Zhc.Model.Lookup
// Imports: public import Init public meta import Init public import Zhc.Model.SpecBlock public import Zhc.Model.CiphertextBlock public import Zhc.Util.Theorems
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
lean_object* lp_zhc__lean_Zhc_Model_CiphertextBlock_toDataNat(lean_object*, lean_object*);
lean_object* lp_zhc__lean_Zhc_Model_CiphertextBlock_toPaddingNat(lean_object*, lean_object*);
uint8_t lean_nat_dec_eq(lean_object*, lean_object*);
lean_object* lp_zhc__lean_Zhc_Model_CiphertextBlock_doNeg(lean_object*, lean_object*);
lean_object* lp_zhc__lean_Zhc_Model_SpecBlock_dataSize(lean_object*);
lean_object* lean_nat_sub(lean_object*, lean_object*);
lean_object* lean_nat_pow(lean_object*, lean_object*);
lean_object* lean_mk_empty_array_with_capacity(lean_object*);
lean_object* lean_array_push(lean_object*, lean_object*);
lean_object* lean_nat_div(lean_object*, lean_object*);
lean_object* lean_nat_mod(lean_object*, lean_object*);
lean_object* lp_zhc__lean_Zhc_Model_CiphertextBlock_ofNat(lean_object*, lean_object*);
lean_object* lean_array_fget_borrowed(lean_object*, lean_object*);
lean_object* lean_nat_mul(lean_object*, lean_object*);
lean_object* lean_nat_add(lean_object*, lean_object*);
LEAN_EXPORT lean_object* lp_zhc__lean_Zhc_Model_pbs(lean_object*, lean_object*, lean_object*);
LEAN_EXPORT lean_object* lp_zhc__lean_Zhc_Model_pbs___boxed(lean_object*, lean_object*, lean_object*);
LEAN_EXPORT lean_object* lp_zhc__lean_Zhc_Model_ManyLut_subTableSize(lean_object*, lean_object*);
LEAN_EXPORT lean_object* lp_zhc__lean_Zhc_Model_ManyLut_subTableSize___boxed(lean_object*, lean_object*);
LEAN_EXPORT lean_object* lp_zhc__lean_Zhc_Model_ManyLut_fromFns___lam__0(lean_object*, lean_object*, lean_object*, lean_object*);
LEAN_EXPORT lean_object* lp_zhc__lean_Zhc_Model_ManyLut_fromFns___lam__0___boxed(lean_object*, lean_object*, lean_object*, lean_object*);
LEAN_EXPORT lean_object* lp_zhc__lean_Zhc_Model_ManyLut_fromFns(lean_object*, lean_object*, lean_object*, lean_object*);
LEAN_EXPORT lean_object* lp_zhc__lean_Zhc_Model_ManyLut_entry(lean_object*, lean_object*, lean_object*, lean_object*, lean_object*);
LEAN_EXPORT lean_object* lp_zhc__lean_Zhc_Model_ManyLut_entry___boxed(lean_object*, lean_object*, lean_object*, lean_object*, lean_object*);
LEAN_EXPORT lean_object* lp_zhc__lean_Zhc_Model_ManyLut_output(lean_object*, lean_object*, lean_object*, lean_object*, lean_object*);
LEAN_EXPORT lean_object* lp_zhc__lean_Zhc_Model_ManyLut_output___boxed(lean_object*, lean_object*, lean_object*, lean_object*, lean_object*);
LEAN_EXPORT lean_object* lp_zhc__lean_Zhc_Model_Lut1_fromFn(lean_object*, lean_object*, lean_object*);
LEAN_EXPORT lean_object* lp_zhc__lean_Zhc_Model_Lut2_fromFn(lean_object*, lean_object*, lean_object*, lean_object*);
LEAN_EXPORT lean_object* lp_zhc__lean_Zhc_Model_Lut4_fromFn(lean_object*, lean_object*, lean_object*, lean_object*, lean_object*, lean_object*);
LEAN_EXPORT lean_object* lp_zhc__lean_Zhc_Model_Lut8_fromFn(lean_object*, lean_object*, lean_object*, lean_object*, lean_object*, lean_object*, lean_object*, lean_object*, lean_object*, lean_object*);
LEAN_EXPORT lean_object* lp_zhc__lean_Zhc_Model_pbs(lean_object* v_sb_1_, lean_object* v_self_2_, lean_object* v_inp_3_){
_start:
{
lean_object* v_inp__nat_4_; lean_object* v_oup_5_; lean_object* v___x_6_; lean_object* v___x_7_; uint8_t v___x_8_; 
v_inp__nat_4_ = lp_zhc__lean_Zhc_Model_CiphertextBlock_toDataNat(v_sb_1_, v_inp_3_);
v_oup_5_ = lean_apply_1(v_self_2_, v_inp__nat_4_);
v___x_6_ = lp_zhc__lean_Zhc_Model_CiphertextBlock_toPaddingNat(v_sb_1_, v_inp_3_);
v___x_7_ = lean_unsigned_to_nat(1u);
v___x_8_ = lean_nat_dec_eq(v___x_6_, v___x_7_);
lean_dec(v___x_6_);
if (v___x_8_ == 0)
{
return v_oup_5_;
}
else
{
lean_object* v___x_9_; 
v___x_9_ = lp_zhc__lean_Zhc_Model_CiphertextBlock_doNeg(v_sb_1_, v_oup_5_);
lean_dec(v_oup_5_);
return v___x_9_;
}
}
}
LEAN_EXPORT lean_object* lp_zhc__lean_Zhc_Model_pbs___boxed(lean_object* v_sb_10_, lean_object* v_self_11_, lean_object* v_inp_12_){
_start:
{
lean_object* v_res_13_; 
v_res_13_ = lp_zhc__lean_Zhc_Model_pbs(v_sb_10_, v_self_11_, v_inp_12_);
lean_dec(v_inp_12_);
lean_dec_ref(v_sb_10_);
return v_res_13_;
}
}
LEAN_EXPORT lean_object* lp_zhc__lean_Zhc_Model_ManyLut_subTableSize(lean_object* v_sb_14_, lean_object* v_n_15_){
_start:
{
lean_object* v___x_16_; lean_object* v___x_17_; lean_object* v___x_18_; lean_object* v___x_19_; 
v___x_16_ = lean_unsigned_to_nat(2u);
v___x_17_ = lp_zhc__lean_Zhc_Model_SpecBlock_dataSize(v_sb_14_);
v___x_18_ = lean_nat_sub(v___x_17_, v_n_15_);
lean_dec(v___x_17_);
v___x_19_ = lean_nat_pow(v___x_16_, v___x_18_);
lean_dec(v___x_18_);
return v___x_19_;
}
}
LEAN_EXPORT lean_object* lp_zhc__lean_Zhc_Model_ManyLut_subTableSize___boxed(lean_object* v_sb_20_, lean_object* v_n_21_){
_start:
{
lean_object* v_res_22_; 
v_res_22_ = lp_zhc__lean_Zhc_Model_ManyLut_subTableSize(v_sb_20_, v_n_21_);
lean_dec(v_n_21_);
lean_dec_ref(v_sb_20_);
return v_res_22_;
}
}
LEAN_EXPORT lean_object* lp_zhc__lean_Zhc_Model_ManyLut_fromFns___lam__0(lean_object* v_sb_23_, lean_object* v_n_24_, lean_object* v_fs_25_, lean_object* v_i_26_){
_start:
{
lean_object* v___x_27_; lean_object* v___x_28_; lean_object* v___x_29_; lean_object* v___x_30_; lean_object* v___x_31_; lean_object* v_fnInput_32_; lean_object* v___x_33_; lean_object* v___x_93__overap_34_; lean_object* v___x_35_; 
v___x_27_ = lean_unsigned_to_nat(2u);
v___x_28_ = lp_zhc__lean_Zhc_Model_SpecBlock_dataSize(v_sb_23_);
v___x_29_ = lean_nat_sub(v___x_28_, v_n_24_);
lean_dec(v___x_28_);
v___x_30_ = lean_nat_pow(v___x_27_, v___x_29_);
lean_dec(v___x_29_);
v___x_31_ = lean_nat_div(v_i_26_, v___x_30_);
v_fnInput_32_ = lean_nat_mod(v_i_26_, v___x_30_);
lean_dec(v___x_30_);
v___x_33_ = lp_zhc__lean_Zhc_Model_CiphertextBlock_ofNat(v_sb_23_, v_fnInput_32_);
lean_dec(v_fnInput_32_);
v___x_93__overap_34_ = lean_array_fget_borrowed(v_fs_25_, v___x_31_);
lean_dec(v___x_31_);
lean_inc(v___x_93__overap_34_);
v___x_35_ = lean_apply_1(v___x_93__overap_34_, v___x_33_);
return v___x_35_;
}
}
LEAN_EXPORT lean_object* lp_zhc__lean_Zhc_Model_ManyLut_fromFns___lam__0___boxed(lean_object* v_sb_36_, lean_object* v_n_37_, lean_object* v_fs_38_, lean_object* v_i_39_){
_start:
{
lean_object* v_res_40_; 
v_res_40_ = lp_zhc__lean_Zhc_Model_ManyLut_fromFns___lam__0(v_sb_36_, v_n_37_, v_fs_38_, v_i_39_);
lean_dec(v_i_39_);
lean_dec_ref(v_fs_38_);
lean_dec(v_n_37_);
lean_dec_ref(v_sb_36_);
return v_res_40_;
}
}
LEAN_EXPORT lean_object* lp_zhc__lean_Zhc_Model_ManyLut_fromFns(lean_object* v_sb_41_, lean_object* v_n_42_, lean_object* v_name_43_, lean_object* v_fs_44_){
_start:
{
lean_object* v_table_45_; lean_object* v___x_46_; 
v_table_45_ = lean_alloc_closure((void*)(lp_zhc__lean_Zhc_Model_ManyLut_fromFns___lam__0___boxed), 4, 3);
lean_closure_set(v_table_45_, 0, v_sb_41_);
lean_closure_set(v_table_45_, 1, v_n_42_);
lean_closure_set(v_table_45_, 2, v_fs_44_);
v___x_46_ = lean_alloc_ctor(0, 2, 0);
lean_ctor_set(v___x_46_, 0, v_name_43_);
lean_ctor_set(v___x_46_, 1, v_table_45_);
return v___x_46_;
}
}
LEAN_EXPORT lean_object* lp_zhc__lean_Zhc_Model_ManyLut_entry(lean_object* v_sb_47_, lean_object* v_n_48_, lean_object* v_self_49_, lean_object* v_k_50_, lean_object* v_r_51_){
_start:
{
lean_object* v_lut_52_; lean_object* v___x_53_; lean_object* v___x_54_; lean_object* v___x_55_; lean_object* v___x_56_; lean_object* v___x_57_; lean_object* v_i_58_; lean_object* v___x_59_; lean_object* v___x_60_; lean_object* v___x_61_; 
v_lut_52_ = lean_ctor_get(v_self_49_, 1);
lean_inc_ref(v_lut_52_);
lean_dec_ref(v_self_49_);
v___x_53_ = lean_unsigned_to_nat(2u);
v___x_54_ = lp_zhc__lean_Zhc_Model_SpecBlock_dataSize(v_sb_47_);
v___x_55_ = lean_nat_sub(v___x_54_, v_n_48_);
v___x_56_ = lean_nat_pow(v___x_53_, v___x_55_);
lean_dec(v___x_55_);
v___x_57_ = lean_nat_mul(v_k_50_, v___x_56_);
lean_dec(v___x_56_);
v_i_58_ = lean_nat_add(v_r_51_, v___x_57_);
lean_dec(v___x_57_);
v___x_59_ = lean_nat_pow(v___x_53_, v___x_54_);
lean_dec(v___x_54_);
v___x_60_ = lean_nat_mod(v_i_58_, v___x_59_);
lean_dec(v___x_59_);
lean_dec(v_i_58_);
v___x_61_ = lean_apply_1(v_lut_52_, v___x_60_);
return v___x_61_;
}
}
LEAN_EXPORT lean_object* lp_zhc__lean_Zhc_Model_ManyLut_entry___boxed(lean_object* v_sb_62_, lean_object* v_n_63_, lean_object* v_self_64_, lean_object* v_k_65_, lean_object* v_r_66_){
_start:
{
lean_object* v_res_67_; 
v_res_67_ = lp_zhc__lean_Zhc_Model_ManyLut_entry(v_sb_62_, v_n_63_, v_self_64_, v_k_65_, v_r_66_);
lean_dec(v_r_66_);
lean_dec(v_k_65_);
lean_dec(v_n_63_);
lean_dec_ref(v_sb_62_);
return v_res_67_;
}
}
LEAN_EXPORT lean_object* lp_zhc__lean_Zhc_Model_ManyLut_output(lean_object* v_sb_68_, lean_object* v_n_69_, lean_object* v_self_70_, lean_object* v_inp_71_, lean_object* v_j_72_){
_start:
{
lean_object* v_lut_73_; lean_object* v___x_74_; lean_object* v___x_75_; lean_object* v___x_76_; lean_object* v___x_77_; lean_object* v___x_78_; lean_object* v___x_79_; lean_object* v_shifted_80_; lean_object* v___x_81_; 
v_lut_73_ = lean_ctor_get(v_self_70_, 1);
lean_inc_ref(v_lut_73_);
lean_dec_ref(v_self_70_);
v___x_74_ = lean_unsigned_to_nat(2u);
v___x_75_ = lp_zhc__lean_Zhc_Model_SpecBlock_dataSize(v_sb_68_);
v___x_76_ = lean_nat_sub(v___x_75_, v_n_69_);
lean_dec(v___x_75_);
v___x_77_ = lean_nat_pow(v___x_74_, v___x_76_);
lean_dec(v___x_76_);
v___x_78_ = lean_nat_mul(v_j_72_, v___x_77_);
lean_dec(v___x_77_);
v___x_79_ = lean_nat_add(v_inp_71_, v___x_78_);
lean_dec(v___x_78_);
v_shifted_80_ = lp_zhc__lean_Zhc_Model_CiphertextBlock_ofNat(v_sb_68_, v___x_79_);
lean_dec(v___x_79_);
v___x_81_ = lp_zhc__lean_Zhc_Model_pbs(v_sb_68_, v_lut_73_, v_shifted_80_);
lean_dec(v_shifted_80_);
return v___x_81_;
}
}
LEAN_EXPORT lean_object* lp_zhc__lean_Zhc_Model_ManyLut_output___boxed(lean_object* v_sb_82_, lean_object* v_n_83_, lean_object* v_self_84_, lean_object* v_inp_85_, lean_object* v_j_86_){
_start:
{
lean_object* v_res_87_; 
v_res_87_ = lp_zhc__lean_Zhc_Model_ManyLut_output(v_sb_82_, v_n_83_, v_self_84_, v_inp_85_, v_j_86_);
lean_dec(v_j_86_);
lean_dec(v_inp_85_);
lean_dec(v_n_83_);
lean_dec_ref(v_sb_82_);
return v_res_87_;
}
}
LEAN_EXPORT lean_object* lp_zhc__lean_Zhc_Model_Lut1_fromFn(lean_object* v_sb_88_, lean_object* v_name_89_, lean_object* v_f_90_){
_start:
{
lean_object* v___x_91_; lean_object* v___x_92_; lean_object* v___x_93_; lean_object* v___x_94_; lean_object* v___x_95_; 
v___x_91_ = lean_unsigned_to_nat(0u);
v___x_92_ = lean_unsigned_to_nat(1u);
v___x_93_ = lean_mk_empty_array_with_capacity(v___x_92_);
v___x_94_ = lean_array_push(v___x_93_, v_f_90_);
v___x_95_ = lp_zhc__lean_Zhc_Model_ManyLut_fromFns(v_sb_88_, v___x_91_, v_name_89_, v___x_94_);
return v___x_95_;
}
}
LEAN_EXPORT lean_object* lp_zhc__lean_Zhc_Model_Lut2_fromFn(lean_object* v_sb_96_, lean_object* v_name_97_, lean_object* v_f1_98_, lean_object* v_f2_99_){
_start:
{
lean_object* v___x_100_; lean_object* v___x_101_; lean_object* v___x_102_; lean_object* v___x_103_; lean_object* v___x_104_; lean_object* v___x_105_; 
v___x_100_ = lean_unsigned_to_nat(1u);
v___x_101_ = lean_unsigned_to_nat(2u);
v___x_102_ = lean_mk_empty_array_with_capacity(v___x_101_);
v___x_103_ = lean_array_push(v___x_102_, v_f1_98_);
v___x_104_ = lean_array_push(v___x_103_, v_f2_99_);
v___x_105_ = lp_zhc__lean_Zhc_Model_ManyLut_fromFns(v_sb_96_, v___x_100_, v_name_97_, v___x_104_);
return v___x_105_;
}
}
LEAN_EXPORT lean_object* lp_zhc__lean_Zhc_Model_Lut4_fromFn(lean_object* v_sb_106_, lean_object* v_name_107_, lean_object* v_f1_108_, lean_object* v_f2_109_, lean_object* v_f3_110_, lean_object* v_f4_111_){
_start:
{
lean_object* v___x_112_; lean_object* v___x_113_; lean_object* v___x_114_; lean_object* v___x_115_; lean_object* v___x_116_; lean_object* v___x_117_; lean_object* v___x_118_; lean_object* v___x_119_; 
v___x_112_ = lean_unsigned_to_nat(2u);
v___x_113_ = lean_unsigned_to_nat(4u);
v___x_114_ = lean_mk_empty_array_with_capacity(v___x_113_);
v___x_115_ = lean_array_push(v___x_114_, v_f1_108_);
v___x_116_ = lean_array_push(v___x_115_, v_f2_109_);
v___x_117_ = lean_array_push(v___x_116_, v_f3_110_);
v___x_118_ = lean_array_push(v___x_117_, v_f4_111_);
v___x_119_ = lp_zhc__lean_Zhc_Model_ManyLut_fromFns(v_sb_106_, v___x_112_, v_name_107_, v___x_118_);
return v___x_119_;
}
}
LEAN_EXPORT lean_object* lp_zhc__lean_Zhc_Model_Lut8_fromFn(lean_object* v_sb_120_, lean_object* v_name_121_, lean_object* v_f1_122_, lean_object* v_f2_123_, lean_object* v_f3_124_, lean_object* v_f4_125_, lean_object* v_f5_126_, lean_object* v_f6_127_, lean_object* v_f7_128_, lean_object* v_f8_129_){
_start:
{
lean_object* v___x_130_; lean_object* v___x_131_; lean_object* v___x_132_; lean_object* v___x_133_; lean_object* v___x_134_; lean_object* v___x_135_; lean_object* v___x_136_; lean_object* v___x_137_; lean_object* v___x_138_; lean_object* v___x_139_; lean_object* v___x_140_; lean_object* v___x_141_; 
v___x_130_ = lean_unsigned_to_nat(3u);
v___x_131_ = lean_unsigned_to_nat(8u);
v___x_132_ = lean_mk_empty_array_with_capacity(v___x_131_);
v___x_133_ = lean_array_push(v___x_132_, v_f1_122_);
v___x_134_ = lean_array_push(v___x_133_, v_f2_123_);
v___x_135_ = lean_array_push(v___x_134_, v_f3_124_);
v___x_136_ = lean_array_push(v___x_135_, v_f4_125_);
v___x_137_ = lean_array_push(v___x_136_, v_f5_126_);
v___x_138_ = lean_array_push(v___x_137_, v_f6_127_);
v___x_139_ = lean_array_push(v___x_138_, v_f7_128_);
v___x_140_ = lean_array_push(v___x_139_, v_f8_129_);
v___x_141_ = lp_zhc__lean_Zhc_Model_ManyLut_fromFns(v_sb_120_, v___x_130_, v_name_121_, v___x_140_);
return v___x_141_;
}
}
lean_object* initialize_Init(uint8_t builtin);
lean_object* initialize_Init(uint8_t builtin);
lean_object* initialize_zhc__lean_Zhc_Model_SpecBlock(uint8_t builtin);
lean_object* initialize_zhc__lean_Zhc_Model_CiphertextBlock(uint8_t builtin);
lean_object* initialize_zhc__lean_Zhc_Util_Theorems(uint8_t builtin);
void lean_initialize_runtime_module();
static bool _G_initialized = false;
LEAN_EXPORT lean_object* initialize_zhc__lean_Zhc_Model_Lookup(uint8_t builtin) {
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
res = initialize_zhc__lean_Zhc_Model_CiphertextBlock(builtin);
if (lean_io_result_is_error(res)) return res;
lean_dec_ref(res);
res = initialize_zhc__lean_Zhc_Util_Theorems(builtin);
if (lean_io_result_is_error(res)) return res;
lean_dec_ref(res);
return lean_io_result_mk_ok(lean_box(0));
}
#ifdef __cplusplus
}
#endif
