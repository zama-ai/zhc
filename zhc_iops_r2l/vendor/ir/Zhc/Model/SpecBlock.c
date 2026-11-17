// Lean compiler output
// Module: Zhc.Model.SpecBlock
// Imports: public import Init public meta import Init public import Zhc.Util.Theorems
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
lean_object* lean_nat_add(lean_object*, lean_object*);
lean_object* lean_nat_to_int(lean_object*);
lean_object* l_Nat_reprFast(lean_object*);
lean_object* lean_string_length(lean_object*);
uint8_t lean_nat_dec_eq(lean_object*, lean_object*);
LEAN_EXPORT uint8_t lp_zhc__lean_Zhc_Model_instDecidableEqSpecBlock_decEq(lean_object*, lean_object*);
LEAN_EXPORT lean_object* lp_zhc__lean_Zhc_Model_instDecidableEqSpecBlock_decEq___boxed(lean_object*, lean_object*);
LEAN_EXPORT uint8_t lp_zhc__lean_Zhc_Model_instDecidableEqSpecBlock(lean_object*, lean_object*);
LEAN_EXPORT lean_object* lp_zhc__lean_Zhc_Model_instDecidableEqSpecBlock___boxed(lean_object*, lean_object*);
static const lean_string_object lp_zhc__lean_Zhc_Model_instReprSpecBlock_repr___redArg___closed__0_value = {.m_header = {.m_rc = 0, .m_cs_sz = 0, .m_other = 0, .m_tag = 249}, .m_size = 3, .m_capacity = 3, .m_length = 2, .m_data = "{ "};
static const lean_object* lp_zhc__lean_Zhc_Model_instReprSpecBlock_repr___redArg___closed__0 = (const lean_object*)&lp_zhc__lean_Zhc_Model_instReprSpecBlock_repr___redArg___closed__0_value;
static const lean_string_object lp_zhc__lean_Zhc_Model_instReprSpecBlock_repr___redArg___closed__1_value = {.m_header = {.m_rc = 0, .m_cs_sz = 0, .m_other = 0, .m_tag = 249}, .m_size = 10, .m_capacity = 10, .m_length = 9, .m_data = "carrySize"};
static const lean_object* lp_zhc__lean_Zhc_Model_instReprSpecBlock_repr___redArg___closed__1 = (const lean_object*)&lp_zhc__lean_Zhc_Model_instReprSpecBlock_repr___redArg___closed__1_value;
static const lean_ctor_object lp_zhc__lean_Zhc_Model_instReprSpecBlock_repr___redArg___closed__2_value = {.m_header = {.m_rc = 0, .m_cs_sz = sizeof(lean_ctor_object) + sizeof(void*)*1 + 0, .m_other = 1, .m_tag = 3}, .m_objs = {((lean_object*)&lp_zhc__lean_Zhc_Model_instReprSpecBlock_repr___redArg___closed__1_value)}};
static const lean_object* lp_zhc__lean_Zhc_Model_instReprSpecBlock_repr___redArg___closed__2 = (const lean_object*)&lp_zhc__lean_Zhc_Model_instReprSpecBlock_repr___redArg___closed__2_value;
static const lean_ctor_object lp_zhc__lean_Zhc_Model_instReprSpecBlock_repr___redArg___closed__3_value = {.m_header = {.m_rc = 0, .m_cs_sz = sizeof(lean_ctor_object) + sizeof(void*)*2 + 0, .m_other = 2, .m_tag = 5}, .m_objs = {((lean_object*)(((size_t)(0) << 1) | 1)),((lean_object*)&lp_zhc__lean_Zhc_Model_instReprSpecBlock_repr___redArg___closed__2_value)}};
static const lean_object* lp_zhc__lean_Zhc_Model_instReprSpecBlock_repr___redArg___closed__3 = (const lean_object*)&lp_zhc__lean_Zhc_Model_instReprSpecBlock_repr___redArg___closed__3_value;
static const lean_string_object lp_zhc__lean_Zhc_Model_instReprSpecBlock_repr___redArg___closed__4_value = {.m_header = {.m_rc = 0, .m_cs_sz = 0, .m_other = 0, .m_tag = 249}, .m_size = 5, .m_capacity = 5, .m_length = 4, .m_data = " := "};
static const lean_object* lp_zhc__lean_Zhc_Model_instReprSpecBlock_repr___redArg___closed__4 = (const lean_object*)&lp_zhc__lean_Zhc_Model_instReprSpecBlock_repr___redArg___closed__4_value;
static const lean_ctor_object lp_zhc__lean_Zhc_Model_instReprSpecBlock_repr___redArg___closed__5_value = {.m_header = {.m_rc = 0, .m_cs_sz = sizeof(lean_ctor_object) + sizeof(void*)*1 + 0, .m_other = 1, .m_tag = 3}, .m_objs = {((lean_object*)&lp_zhc__lean_Zhc_Model_instReprSpecBlock_repr___redArg___closed__4_value)}};
static const lean_object* lp_zhc__lean_Zhc_Model_instReprSpecBlock_repr___redArg___closed__5 = (const lean_object*)&lp_zhc__lean_Zhc_Model_instReprSpecBlock_repr___redArg___closed__5_value;
static const lean_ctor_object lp_zhc__lean_Zhc_Model_instReprSpecBlock_repr___redArg___closed__6_value = {.m_header = {.m_rc = 0, .m_cs_sz = sizeof(lean_ctor_object) + sizeof(void*)*2 + 0, .m_other = 2, .m_tag = 5}, .m_objs = {((lean_object*)&lp_zhc__lean_Zhc_Model_instReprSpecBlock_repr___redArg___closed__3_value),((lean_object*)&lp_zhc__lean_Zhc_Model_instReprSpecBlock_repr___redArg___closed__5_value)}};
static const lean_object* lp_zhc__lean_Zhc_Model_instReprSpecBlock_repr___redArg___closed__6 = (const lean_object*)&lp_zhc__lean_Zhc_Model_instReprSpecBlock_repr___redArg___closed__6_value;
static lean_once_cell_t lp_zhc__lean_Zhc_Model_instReprSpecBlock_repr___redArg___closed__7_once = LEAN_ONCE_CELL_INITIALIZER;
static lean_object* lp_zhc__lean_Zhc_Model_instReprSpecBlock_repr___redArg___closed__7;
static const lean_string_object lp_zhc__lean_Zhc_Model_instReprSpecBlock_repr___redArg___closed__8_value = {.m_header = {.m_rc = 0, .m_cs_sz = 0, .m_other = 0, .m_tag = 249}, .m_size = 2, .m_capacity = 2, .m_length = 1, .m_data = ","};
static const lean_object* lp_zhc__lean_Zhc_Model_instReprSpecBlock_repr___redArg___closed__8 = (const lean_object*)&lp_zhc__lean_Zhc_Model_instReprSpecBlock_repr___redArg___closed__8_value;
static const lean_ctor_object lp_zhc__lean_Zhc_Model_instReprSpecBlock_repr___redArg___closed__9_value = {.m_header = {.m_rc = 0, .m_cs_sz = sizeof(lean_ctor_object) + sizeof(void*)*1 + 0, .m_other = 1, .m_tag = 3}, .m_objs = {((lean_object*)&lp_zhc__lean_Zhc_Model_instReprSpecBlock_repr___redArg___closed__8_value)}};
static const lean_object* lp_zhc__lean_Zhc_Model_instReprSpecBlock_repr___redArg___closed__9 = (const lean_object*)&lp_zhc__lean_Zhc_Model_instReprSpecBlock_repr___redArg___closed__9_value;
static const lean_string_object lp_zhc__lean_Zhc_Model_instReprSpecBlock_repr___redArg___closed__10_value = {.m_header = {.m_rc = 0, .m_cs_sz = 0, .m_other = 0, .m_tag = 249}, .m_size = 12, .m_capacity = 12, .m_length = 11, .m_data = "messageSize"};
static const lean_object* lp_zhc__lean_Zhc_Model_instReprSpecBlock_repr___redArg___closed__10 = (const lean_object*)&lp_zhc__lean_Zhc_Model_instReprSpecBlock_repr___redArg___closed__10_value;
static const lean_ctor_object lp_zhc__lean_Zhc_Model_instReprSpecBlock_repr___redArg___closed__11_value = {.m_header = {.m_rc = 0, .m_cs_sz = sizeof(lean_ctor_object) + sizeof(void*)*1 + 0, .m_other = 1, .m_tag = 3}, .m_objs = {((lean_object*)&lp_zhc__lean_Zhc_Model_instReprSpecBlock_repr___redArg___closed__10_value)}};
static const lean_object* lp_zhc__lean_Zhc_Model_instReprSpecBlock_repr___redArg___closed__11 = (const lean_object*)&lp_zhc__lean_Zhc_Model_instReprSpecBlock_repr___redArg___closed__11_value;
static lean_once_cell_t lp_zhc__lean_Zhc_Model_instReprSpecBlock_repr___redArg___closed__12_once = LEAN_ONCE_CELL_INITIALIZER;
static lean_object* lp_zhc__lean_Zhc_Model_instReprSpecBlock_repr___redArg___closed__12;
static const lean_string_object lp_zhc__lean_Zhc_Model_instReprSpecBlock_repr___redArg___closed__13_value = {.m_header = {.m_rc = 0, .m_cs_sz = 0, .m_other = 0, .m_tag = 249}, .m_size = 3, .m_capacity = 3, .m_length = 2, .m_data = " }"};
static const lean_object* lp_zhc__lean_Zhc_Model_instReprSpecBlock_repr___redArg___closed__13 = (const lean_object*)&lp_zhc__lean_Zhc_Model_instReprSpecBlock_repr___redArg___closed__13_value;
static lean_once_cell_t lp_zhc__lean_Zhc_Model_instReprSpecBlock_repr___redArg___closed__14_once = LEAN_ONCE_CELL_INITIALIZER;
static lean_object* lp_zhc__lean_Zhc_Model_instReprSpecBlock_repr___redArg___closed__14;
static lean_once_cell_t lp_zhc__lean_Zhc_Model_instReprSpecBlock_repr___redArg___closed__15_once = LEAN_ONCE_CELL_INITIALIZER;
static lean_object* lp_zhc__lean_Zhc_Model_instReprSpecBlock_repr___redArg___closed__15;
static const lean_ctor_object lp_zhc__lean_Zhc_Model_instReprSpecBlock_repr___redArg___closed__16_value = {.m_header = {.m_rc = 0, .m_cs_sz = sizeof(lean_ctor_object) + sizeof(void*)*1 + 0, .m_other = 1, .m_tag = 3}, .m_objs = {((lean_object*)&lp_zhc__lean_Zhc_Model_instReprSpecBlock_repr___redArg___closed__0_value)}};
static const lean_object* lp_zhc__lean_Zhc_Model_instReprSpecBlock_repr___redArg___closed__16 = (const lean_object*)&lp_zhc__lean_Zhc_Model_instReprSpecBlock_repr___redArg___closed__16_value;
static const lean_ctor_object lp_zhc__lean_Zhc_Model_instReprSpecBlock_repr___redArg___closed__17_value = {.m_header = {.m_rc = 0, .m_cs_sz = sizeof(lean_ctor_object) + sizeof(void*)*1 + 0, .m_other = 1, .m_tag = 3}, .m_objs = {((lean_object*)&lp_zhc__lean_Zhc_Model_instReprSpecBlock_repr___redArg___closed__13_value)}};
static const lean_object* lp_zhc__lean_Zhc_Model_instReprSpecBlock_repr___redArg___closed__17 = (const lean_object*)&lp_zhc__lean_Zhc_Model_instReprSpecBlock_repr___redArg___closed__17_value;
LEAN_EXPORT lean_object* lp_zhc__lean_Zhc_Model_instReprSpecBlock_repr___redArg(lean_object*);
LEAN_EXPORT lean_object* lp_zhc__lean_Zhc_Model_instReprSpecBlock_repr(lean_object*, lean_object*);
LEAN_EXPORT lean_object* lp_zhc__lean_Zhc_Model_instReprSpecBlock_repr___boxed(lean_object*, lean_object*);
static const lean_closure_object lp_zhc__lean_Zhc_Model_instReprSpecBlock___closed__0_value = {.m_header = {.m_rc = 0, .m_cs_sz = sizeof(lean_closure_object) + sizeof(void*)*0, .m_other = 0, .m_tag = 245}, .m_fun = (void*)lp_zhc__lean_Zhc_Model_instReprSpecBlock_repr___boxed, .m_arity = 2, .m_num_fixed = 0, .m_objs = {} };
static const lean_object* lp_zhc__lean_Zhc_Model_instReprSpecBlock___closed__0 = (const lean_object*)&lp_zhc__lean_Zhc_Model_instReprSpecBlock___closed__0_value;
LEAN_EXPORT const lean_object* lp_zhc__lean_Zhc_Model_instReprSpecBlock = (const lean_object*)&lp_zhc__lean_Zhc_Model_instReprSpecBlock___closed__0_value;
LEAN_EXPORT lean_object* lp_zhc__lean_Zhc_Model_SpecBlock_paddingSize(lean_object*);
LEAN_EXPORT lean_object* lp_zhc__lean_Zhc_Model_SpecBlock_paddingSize___boxed(lean_object*);
LEAN_EXPORT lean_object* lp_zhc__lean_Zhc_Model_SpecBlock_dataSize(lean_object*);
LEAN_EXPORT lean_object* lp_zhc__lean_Zhc_Model_SpecBlock_dataSize___boxed(lean_object*);
LEAN_EXPORT lean_object* lp_zhc__lean_Zhc_Model_SpecBlock_completeSize(lean_object*);
LEAN_EXPORT lean_object* lp_zhc__lean_Zhc_Model_SpecBlock_completeSize___boxed(lean_object*);
LEAN_EXPORT uint8_t lp_zhc__lean_Zhc_Model_instDecidableEqSpecBlock_decEq(lean_object* v_x_1_, lean_object* v_x_2_){
_start:
{
lean_object* v_carrySize_3_; lean_object* v_messageSize_4_; lean_object* v_carrySize_5_; lean_object* v_messageSize_6_; uint8_t v___x_7_; 
v_carrySize_3_ = lean_ctor_get(v_x_1_, 0);
v_messageSize_4_ = lean_ctor_get(v_x_1_, 1);
v_carrySize_5_ = lean_ctor_get(v_x_2_, 0);
v_messageSize_6_ = lean_ctor_get(v_x_2_, 1);
v___x_7_ = lean_nat_dec_eq(v_carrySize_3_, v_carrySize_5_);
if (v___x_7_ == 0)
{
return v___x_7_;
}
else
{
uint8_t v___x_8_; 
v___x_8_ = lean_nat_dec_eq(v_messageSize_4_, v_messageSize_6_);
return v___x_8_;
}
}
}
LEAN_EXPORT lean_object* lp_zhc__lean_Zhc_Model_instDecidableEqSpecBlock_decEq___boxed(lean_object* v_x_9_, lean_object* v_x_10_){
_start:
{
uint8_t v_res_11_; lean_object* v_r_12_; 
v_res_11_ = lp_zhc__lean_Zhc_Model_instDecidableEqSpecBlock_decEq(v_x_9_, v_x_10_);
lean_dec_ref(v_x_10_);
lean_dec_ref(v_x_9_);
v_r_12_ = lean_box(v_res_11_);
return v_r_12_;
}
}
LEAN_EXPORT uint8_t lp_zhc__lean_Zhc_Model_instDecidableEqSpecBlock(lean_object* v_x_13_, lean_object* v_x_14_){
_start:
{
uint8_t v___x_15_; 
v___x_15_ = lp_zhc__lean_Zhc_Model_instDecidableEqSpecBlock_decEq(v_x_13_, v_x_14_);
return v___x_15_;
}
}
LEAN_EXPORT lean_object* lp_zhc__lean_Zhc_Model_instDecidableEqSpecBlock___boxed(lean_object* v_x_16_, lean_object* v_x_17_){
_start:
{
uint8_t v_res_18_; lean_object* v_r_19_; 
v_res_18_ = lp_zhc__lean_Zhc_Model_instDecidableEqSpecBlock(v_x_16_, v_x_17_);
lean_dec_ref(v_x_17_);
lean_dec_ref(v_x_16_);
v_r_19_ = lean_box(v_res_18_);
return v_r_19_;
}
}
static lean_object* _init_lp_zhc__lean_Zhc_Model_instReprSpecBlock_repr___redArg___closed__7(void){
_start:
{
lean_object* v___x_33_; lean_object* v___x_34_; 
v___x_33_ = lean_unsigned_to_nat(13u);
v___x_34_ = lean_nat_to_int(v___x_33_);
return v___x_34_;
}
}
static lean_object* _init_lp_zhc__lean_Zhc_Model_instReprSpecBlock_repr___redArg___closed__12(void){
_start:
{
lean_object* v___x_41_; lean_object* v___x_42_; 
v___x_41_ = lean_unsigned_to_nat(15u);
v___x_42_ = lean_nat_to_int(v___x_41_);
return v___x_42_;
}
}
static lean_object* _init_lp_zhc__lean_Zhc_Model_instReprSpecBlock_repr___redArg___closed__14(void){
_start:
{
lean_object* v___x_44_; lean_object* v___x_45_; 
v___x_44_ = ((lean_object*)(lp_zhc__lean_Zhc_Model_instReprSpecBlock_repr___redArg___closed__0));
v___x_45_ = lean_string_length(v___x_44_);
return v___x_45_;
}
}
static lean_object* _init_lp_zhc__lean_Zhc_Model_instReprSpecBlock_repr___redArg___closed__15(void){
_start:
{
lean_object* v___x_46_; lean_object* v___x_47_; 
v___x_46_ = lean_obj_once(&lp_zhc__lean_Zhc_Model_instReprSpecBlock_repr___redArg___closed__14, &lp_zhc__lean_Zhc_Model_instReprSpecBlock_repr___redArg___closed__14_once, _init_lp_zhc__lean_Zhc_Model_instReprSpecBlock_repr___redArg___closed__14);
v___x_47_ = lean_nat_to_int(v___x_46_);
return v___x_47_;
}
}
LEAN_EXPORT lean_object* lp_zhc__lean_Zhc_Model_instReprSpecBlock_repr___redArg(lean_object* v_x_52_){
_start:
{
lean_object* v_carrySize_53_; lean_object* v_messageSize_54_; lean_object* v___x_56_; uint8_t v_isShared_57_; uint8_t v_isSharedCheck_89_; 
v_carrySize_53_ = lean_ctor_get(v_x_52_, 0);
v_messageSize_54_ = lean_ctor_get(v_x_52_, 1);
v_isSharedCheck_89_ = !lean_is_exclusive(v_x_52_);
if (v_isSharedCheck_89_ == 0)
{
v___x_56_ = v_x_52_;
v_isShared_57_ = v_isSharedCheck_89_;
goto v_resetjp_55_;
}
else
{
lean_inc(v_messageSize_54_);
lean_inc(v_carrySize_53_);
lean_dec(v_x_52_);
v___x_56_ = lean_box(0);
v_isShared_57_ = v_isSharedCheck_89_;
goto v_resetjp_55_;
}
v_resetjp_55_:
{
lean_object* v___x_58_; lean_object* v___x_59_; lean_object* v___x_60_; lean_object* v___x_61_; lean_object* v___x_62_; lean_object* v___x_64_; 
v___x_58_ = ((lean_object*)(lp_zhc__lean_Zhc_Model_instReprSpecBlock_repr___redArg___closed__5));
v___x_59_ = ((lean_object*)(lp_zhc__lean_Zhc_Model_instReprSpecBlock_repr___redArg___closed__6));
v___x_60_ = lean_obj_once(&lp_zhc__lean_Zhc_Model_instReprSpecBlock_repr___redArg___closed__7, &lp_zhc__lean_Zhc_Model_instReprSpecBlock_repr___redArg___closed__7_once, _init_lp_zhc__lean_Zhc_Model_instReprSpecBlock_repr___redArg___closed__7);
v___x_61_ = l_Nat_reprFast(v_carrySize_53_);
v___x_62_ = lean_alloc_ctor(3, 1, 0);
lean_ctor_set(v___x_62_, 0, v___x_61_);
if (v_isShared_57_ == 0)
{
lean_ctor_set_tag(v___x_56_, 4);
lean_ctor_set(v___x_56_, 1, v___x_62_);
lean_ctor_set(v___x_56_, 0, v___x_60_);
v___x_64_ = v___x_56_;
goto v_reusejp_63_;
}
else
{
lean_object* v_reuseFailAlloc_88_; 
v_reuseFailAlloc_88_ = lean_alloc_ctor(4, 2, 0);
lean_ctor_set(v_reuseFailAlloc_88_, 0, v___x_60_);
lean_ctor_set(v_reuseFailAlloc_88_, 1, v___x_62_);
v___x_64_ = v_reuseFailAlloc_88_;
goto v_reusejp_63_;
}
v_reusejp_63_:
{
uint8_t v___x_65_; lean_object* v___x_66_; lean_object* v___x_67_; lean_object* v___x_68_; lean_object* v___x_69_; lean_object* v___x_70_; lean_object* v___x_71_; lean_object* v___x_72_; lean_object* v___x_73_; lean_object* v___x_74_; lean_object* v___x_75_; lean_object* v___x_76_; lean_object* v___x_77_; lean_object* v___x_78_; lean_object* v___x_79_; lean_object* v___x_80_; lean_object* v___x_81_; lean_object* v___x_82_; lean_object* v___x_83_; lean_object* v___x_84_; lean_object* v___x_85_; lean_object* v___x_86_; lean_object* v___x_87_; 
v___x_65_ = 0;
v___x_66_ = lean_alloc_ctor(6, 1, 1);
lean_ctor_set(v___x_66_, 0, v___x_64_);
lean_ctor_set_uint8(v___x_66_, sizeof(void*)*1, v___x_65_);
v___x_67_ = lean_alloc_ctor(5, 2, 0);
lean_ctor_set(v___x_67_, 0, v___x_59_);
lean_ctor_set(v___x_67_, 1, v___x_66_);
v___x_68_ = ((lean_object*)(lp_zhc__lean_Zhc_Model_instReprSpecBlock_repr___redArg___closed__9));
v___x_69_ = lean_alloc_ctor(5, 2, 0);
lean_ctor_set(v___x_69_, 0, v___x_67_);
lean_ctor_set(v___x_69_, 1, v___x_68_);
v___x_70_ = lean_box(1);
v___x_71_ = lean_alloc_ctor(5, 2, 0);
lean_ctor_set(v___x_71_, 0, v___x_69_);
lean_ctor_set(v___x_71_, 1, v___x_70_);
v___x_72_ = ((lean_object*)(lp_zhc__lean_Zhc_Model_instReprSpecBlock_repr___redArg___closed__11));
v___x_73_ = lean_alloc_ctor(5, 2, 0);
lean_ctor_set(v___x_73_, 0, v___x_71_);
lean_ctor_set(v___x_73_, 1, v___x_72_);
v___x_74_ = lean_alloc_ctor(5, 2, 0);
lean_ctor_set(v___x_74_, 0, v___x_73_);
lean_ctor_set(v___x_74_, 1, v___x_58_);
v___x_75_ = lean_obj_once(&lp_zhc__lean_Zhc_Model_instReprSpecBlock_repr___redArg___closed__12, &lp_zhc__lean_Zhc_Model_instReprSpecBlock_repr___redArg___closed__12_once, _init_lp_zhc__lean_Zhc_Model_instReprSpecBlock_repr___redArg___closed__12);
v___x_76_ = l_Nat_reprFast(v_messageSize_54_);
v___x_77_ = lean_alloc_ctor(3, 1, 0);
lean_ctor_set(v___x_77_, 0, v___x_76_);
v___x_78_ = lean_alloc_ctor(4, 2, 0);
lean_ctor_set(v___x_78_, 0, v___x_75_);
lean_ctor_set(v___x_78_, 1, v___x_77_);
v___x_79_ = lean_alloc_ctor(6, 1, 1);
lean_ctor_set(v___x_79_, 0, v___x_78_);
lean_ctor_set_uint8(v___x_79_, sizeof(void*)*1, v___x_65_);
v___x_80_ = lean_alloc_ctor(5, 2, 0);
lean_ctor_set(v___x_80_, 0, v___x_74_);
lean_ctor_set(v___x_80_, 1, v___x_79_);
v___x_81_ = lean_obj_once(&lp_zhc__lean_Zhc_Model_instReprSpecBlock_repr___redArg___closed__15, &lp_zhc__lean_Zhc_Model_instReprSpecBlock_repr___redArg___closed__15_once, _init_lp_zhc__lean_Zhc_Model_instReprSpecBlock_repr___redArg___closed__15);
v___x_82_ = ((lean_object*)(lp_zhc__lean_Zhc_Model_instReprSpecBlock_repr___redArg___closed__16));
v___x_83_ = lean_alloc_ctor(5, 2, 0);
lean_ctor_set(v___x_83_, 0, v___x_82_);
lean_ctor_set(v___x_83_, 1, v___x_80_);
v___x_84_ = ((lean_object*)(lp_zhc__lean_Zhc_Model_instReprSpecBlock_repr___redArg___closed__17));
v___x_85_ = lean_alloc_ctor(5, 2, 0);
lean_ctor_set(v___x_85_, 0, v___x_83_);
lean_ctor_set(v___x_85_, 1, v___x_84_);
v___x_86_ = lean_alloc_ctor(4, 2, 0);
lean_ctor_set(v___x_86_, 0, v___x_81_);
lean_ctor_set(v___x_86_, 1, v___x_85_);
v___x_87_ = lean_alloc_ctor(6, 1, 1);
lean_ctor_set(v___x_87_, 0, v___x_86_);
lean_ctor_set_uint8(v___x_87_, sizeof(void*)*1, v___x_65_);
return v___x_87_;
}
}
}
}
LEAN_EXPORT lean_object* lp_zhc__lean_Zhc_Model_instReprSpecBlock_repr(lean_object* v_x_90_, lean_object* v_prec_91_){
_start:
{
lean_object* v___x_92_; 
v___x_92_ = lp_zhc__lean_Zhc_Model_instReprSpecBlock_repr___redArg(v_x_90_);
return v___x_92_;
}
}
LEAN_EXPORT lean_object* lp_zhc__lean_Zhc_Model_instReprSpecBlock_repr___boxed(lean_object* v_x_93_, lean_object* v_prec_94_){
_start:
{
lean_object* v_res_95_; 
v_res_95_ = lp_zhc__lean_Zhc_Model_instReprSpecBlock_repr(v_x_93_, v_prec_94_);
lean_dec(v_prec_94_);
return v_res_95_;
}
}
LEAN_EXPORT lean_object* lp_zhc__lean_Zhc_Model_SpecBlock_paddingSize(lean_object* v___self_98_){
_start:
{
lean_object* v___x_99_; 
v___x_99_ = lean_unsigned_to_nat(1u);
return v___x_99_;
}
}
LEAN_EXPORT lean_object* lp_zhc__lean_Zhc_Model_SpecBlock_paddingSize___boxed(lean_object* v___self_100_){
_start:
{
lean_object* v_res_101_; 
v_res_101_ = lp_zhc__lean_Zhc_Model_SpecBlock_paddingSize(v___self_100_);
lean_dec_ref(v___self_100_);
return v_res_101_;
}
}
LEAN_EXPORT lean_object* lp_zhc__lean_Zhc_Model_SpecBlock_dataSize(lean_object* v_self_102_){
_start:
{
lean_object* v_carrySize_103_; lean_object* v_messageSize_104_; lean_object* v___x_105_; 
v_carrySize_103_ = lean_ctor_get(v_self_102_, 0);
v_messageSize_104_ = lean_ctor_get(v_self_102_, 1);
v___x_105_ = lean_nat_add(v_carrySize_103_, v_messageSize_104_);
return v___x_105_;
}
}
LEAN_EXPORT lean_object* lp_zhc__lean_Zhc_Model_SpecBlock_dataSize___boxed(lean_object* v_self_106_){
_start:
{
lean_object* v_res_107_; 
v_res_107_ = lp_zhc__lean_Zhc_Model_SpecBlock_dataSize(v_self_106_);
lean_dec_ref(v_self_106_);
return v_res_107_;
}
}
LEAN_EXPORT lean_object* lp_zhc__lean_Zhc_Model_SpecBlock_completeSize(lean_object* v_self_108_){
_start:
{
lean_object* v___x_109_; lean_object* v___x_110_; lean_object* v___x_111_; 
v___x_109_ = lp_zhc__lean_Zhc_Model_SpecBlock_dataSize(v_self_108_);
v___x_110_ = lean_unsigned_to_nat(1u);
v___x_111_ = lean_nat_add(v___x_109_, v___x_110_);
lean_dec(v___x_109_);
return v___x_111_;
}
}
LEAN_EXPORT lean_object* lp_zhc__lean_Zhc_Model_SpecBlock_completeSize___boxed(lean_object* v_self_112_){
_start:
{
lean_object* v_res_113_; 
v_res_113_ = lp_zhc__lean_Zhc_Model_SpecBlock_completeSize(v_self_112_);
lean_dec_ref(v_self_112_);
return v_res_113_;
}
}
lean_object* initialize_Init(uint8_t builtin);
lean_object* initialize_Init(uint8_t builtin);
lean_object* initialize_zhc__lean_Zhc_Util_Theorems(uint8_t builtin);
void lean_initialize_runtime_module();
static bool _G_initialized = false;
LEAN_EXPORT lean_object* initialize_zhc__lean_Zhc_Model_SpecBlock(uint8_t builtin) {
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
res = initialize_zhc__lean_Zhc_Util_Theorems(builtin);
if (lean_io_result_is_error(res)) return res;
lean_dec_ref(res);
return lean_io_result_mk_ok(lean_box(0));
}
#ifdef __cplusplus
}
#endif
