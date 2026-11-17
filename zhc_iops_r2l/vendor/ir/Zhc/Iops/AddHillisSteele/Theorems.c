// Lean compiler output
// Module: Zhc.Iops.AddHillisSteele.Theorems
// Imports: public import Init public meta import Init public import Zhc.BlockAlgebra.Model.Theorems public import Zhc.BlockAlgebra.Tracer.Theorems public import Zhc.Iops.AddRippleCarry.Theorems public import Zhc.Iops.AddHillisSteele.Impl
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
uint8_t lean_nat_dec_eq(lean_object*, lean_object*);
lean_object* l_Lean_Name_mkStr4(lean_object*, lean_object*, lean_object*, lean_object*);
lean_object* l_Lean_Name_mkStr1(lean_object*);
lean_object* l_Lean_Name_mkStr5(lean_object*, lean_object*, lean_object*, lean_object*, lean_object*);
lean_object* lean_nat_sub(lean_object*, lean_object*);
uint8_t lean_nat_dec_le(lean_object*, lean_object*);
lean_object* l_String_toRawSubstring_x27(lean_object*);
lean_object* lean_nat_pow(lean_object*, lean_object*);
lean_object* lean_nat_div(lean_object*, lean_object*);
lean_object* lean_nat_mod(lean_object*, lean_object*);
lean_object* lean_nat_add(lean_object*, lean_object*);
lean_object* lean_nat_mul(lean_object*, lean_object*);
uint8_t l_Lean_Syntax_isOfKind(lean_object*, lean_object*);
lean_object* l_Lean_SourceInfo_fromRef(lean_object*, uint8_t);
lean_object* l_Lean_Name_mkStr2(lean_object*, lean_object*);
lean_object* l_Lean_addMacroScope(lean_object*, lean_object*, lean_object*);
lean_object* l_Lean_Syntax_node1(lean_object*, lean_object*, lean_object*);
lean_object* l_Lean_Syntax_node2(lean_object*, lean_object*, lean_object*, lean_object*);
lean_object* l_Lean_Syntax_node5(lean_object*, lean_object*, lean_object*, lean_object*, lean_object*, lean_object*, lean_object*);
lean_object* l_Array_mkArray0(lean_object*);
lean_object* l_Lean_Syntax_node4(lean_object*, lean_object*, lean_object*, lean_object*, lean_object*, lean_object*);
lean_object* l_Lean_Syntax_node3(lean_object*, lean_object*, lean_object*, lean_object*, lean_object*);
LEAN_EXPORT lean_object* lp_zhc__lean_Zhc_Iops_HillisSteele_digit(lean_object*, lean_object*);
LEAN_EXPORT lean_object* lp_zhc__lean_Zhc_Iops_HillisSteele_digit___boxed(lean_object*, lean_object*);
LEAN_EXPORT lean_object* lp_zhc__lean_Zhc_Iops_HillisSteele_carryIn(lean_object*, lean_object*, lean_object*);
LEAN_EXPORT lean_object* lp_zhc__lean_Zhc_Iops_HillisSteele_carryIn___boxed(lean_object*, lean_object*, lean_object*);
LEAN_EXPORT lean_object* lp_zhc__lean_Zhc_Iops_HillisSteele_windowStatus(lean_object*, lean_object*, lean_object*, lean_object*);
LEAN_EXPORT lean_object* lp_zhc__lean_Zhc_Iops_HillisSteele_windowStatus___boxed(lean_object*, lean_object*, lean_object*, lean_object*);
LEAN_EXPORT lean_object* lp_zhc__lean_Zhc_Iops_HillisSteele_blockStatus(lean_object*);
LEAN_EXPORT lean_object* lp_zhc__lean_Zhc_Iops_HillisSteele_blockStatus___boxed(lean_object*);
LEAN_EXPORT lean_object* lp_zhc__lean_Zhc_Iops_HillisSteele_solve(lean_object*, lean_object*);
LEAN_EXPORT lean_object* lp_zhc__lean_Zhc_Iops_HillisSteele_solve___boxed(lean_object*, lean_object*);
LEAN_EXPORT lean_object* lp_zhc__lean_Zhc_Iops_HillisSteele_weightedStatus(lean_object*, lean_object*, lean_object*, lean_object*);
LEAN_EXPORT lean_object* lp_zhc__lean_Zhc_Iops_HillisSteele_weightedStatus___boxed(lean_object*, lean_object*, lean_object*, lean_object*);
LEAN_EXPORT lean_object* lp_zhc__lean___private_Zhc_Iops_AddHillisSteele_Theorems_0__Zhc_Iops_HillisSteele_weightedStatus_match__1_splitter___redArg(lean_object*, lean_object*, lean_object*);
LEAN_EXPORT lean_object* lp_zhc__lean___private_Zhc_Iops_AddHillisSteele_Theorems_0__Zhc_Iops_HillisSteele_weightedStatus_match__1_splitter___redArg___boxed(lean_object*, lean_object*, lean_object*);
LEAN_EXPORT lean_object* lp_zhc__lean___private_Zhc_Iops_AddHillisSteele_Theorems_0__Zhc_Iops_HillisSteele_weightedStatus_match__1_splitter(lean_object*, lean_object*, lean_object*, lean_object*);
LEAN_EXPORT lean_object* lp_zhc__lean___private_Zhc_Iops_AddHillisSteele_Theorems_0__Zhc_Iops_HillisSteele_weightedStatus_match__1_splitter___boxed(lean_object*, lean_object*, lean_object*, lean_object*);
LEAN_EXPORT lean_object* lp_zhc__lean_Zhc_Iops_HillisSteele_sel4___redArg(lean_object*, lean_object*);
LEAN_EXPORT lean_object* lp_zhc__lean_Zhc_Iops_HillisSteele_sel4___redArg___boxed(lean_object*, lean_object*);
LEAN_EXPORT lean_object* lp_zhc__lean_Zhc_Iops_HillisSteele_sel4(lean_object*, lean_object*, lean_object*);
LEAN_EXPORT lean_object* lp_zhc__lean_Zhc_Iops_HillisSteele_sel4___boxed(lean_object*, lean_object*, lean_object*);
LEAN_EXPORT lean_object* lp_zhc__lean___private_Zhc_Iops_AddHillisSteele_Theorems_0__Zhc_Iops_HillisSteele_chunk4_match__1_splitter___redArg(lean_object*, lean_object*, lean_object*);
LEAN_EXPORT lean_object* lp_zhc__lean___private_Zhc_Iops_AddHillisSteele_Theorems_0__Zhc_Iops_HillisSteele_chunk4_match__1_splitter(lean_object*, lean_object*, lean_object*, lean_object*, lean_object*);
LEAN_EXPORT lean_object* lp_zhc__lean___private_Zhc_Iops_AddHillisSteele_Theorems_0__Zhc_Iops_HillisSteele_unchunk4_match__1_splitter___redArg(lean_object*, lean_object*, lean_object*);
LEAN_EXPORT lean_object* lp_zhc__lean___private_Zhc_Iops_AddHillisSteele_Theorems_0__Zhc_Iops_HillisSteele_unchunk4_match__1_splitter(lean_object*, lean_object*, lean_object*, lean_object*, lean_object*);
LEAN_EXPORT lean_object* lp_zhc__lean___private_Zhc_Iops_AddHillisSteele_Theorems_0__Zhc_Iops_HillisSteele_sel4_match__1_splitter___redArg(lean_object*, lean_object*, lean_object*, lean_object*, lean_object*);
LEAN_EXPORT lean_object* lp_zhc__lean___private_Zhc_Iops_AddHillisSteele_Theorems_0__Zhc_Iops_HillisSteele_sel4_match__1_splitter(lean_object*, lean_object*, lean_object*, lean_object*, lean_object*, lean_object*);
LEAN_EXPORT lean_object* lp_zhc__lean___private_Zhc_Iops_AddHillisSteele_Theorems_0__Zhc_Iops_HillisSteele_finalResolution__run_match__1_splitter___redArg(lean_object*, lean_object*, lean_object*);
LEAN_EXPORT lean_object* lp_zhc__lean___private_Zhc_Iops_AddHillisSteele_Theorems_0__Zhc_Iops_HillisSteele_finalResolution__run_match__1_splitter(lean_object*, lean_object*, lean_object*, lean_object*);
LEAN_EXPORT lean_object* lp_zhc__lean___private_Zhc_Iops_AddHillisSteele_Theorems_0__Zhc_Iops_HillisSteele_propagate_match__3_splitter___redArg(lean_object*, lean_object*, lean_object*);
LEAN_EXPORT lean_object* lp_zhc__lean___private_Zhc_Iops_AddHillisSteele_Theorems_0__Zhc_Iops_HillisSteele_propagate_match__3_splitter(lean_object*, lean_object*, lean_object*, lean_object*, lean_object*, lean_object*, lean_object*);
LEAN_EXPORT lean_object* lp_zhc__lean___private_Zhc_Iops_AddHillisSteele_Theorems_0__Zhc_Iops_HillisSteele_propagate_match__3_splitter___boxed(lean_object*, lean_object*, lean_object*, lean_object*, lean_object*, lean_object*, lean_object*);
static const lean_string_object lp_zhc__lean_Zhc_Iops_HillisSteele_tacticProjects__step___closed__0_value = {.m_header = {.m_rc = 0, .m_cs_sz = 0, .m_other = 0, .m_tag = 249}, .m_size = 4, .m_capacity = 4, .m_length = 3, .m_data = "Zhc"};
static const lean_object* lp_zhc__lean_Zhc_Iops_HillisSteele_tacticProjects__step___closed__0 = (const lean_object*)&lp_zhc__lean_Zhc_Iops_HillisSteele_tacticProjects__step___closed__0_value;
static const lean_string_object lp_zhc__lean_Zhc_Iops_HillisSteele_tacticProjects__step___closed__1_value = {.m_header = {.m_rc = 0, .m_cs_sz = 0, .m_other = 0, .m_tag = 249}, .m_size = 5, .m_capacity = 5, .m_length = 4, .m_data = "Iops"};
static const lean_object* lp_zhc__lean_Zhc_Iops_HillisSteele_tacticProjects__step___closed__1 = (const lean_object*)&lp_zhc__lean_Zhc_Iops_HillisSteele_tacticProjects__step___closed__1_value;
static const lean_string_object lp_zhc__lean_Zhc_Iops_HillisSteele_tacticProjects__step___closed__2_value = {.m_header = {.m_rc = 0, .m_cs_sz = 0, .m_other = 0, .m_tag = 249}, .m_size = 13, .m_capacity = 13, .m_length = 12, .m_data = "HillisSteele"};
static const lean_object* lp_zhc__lean_Zhc_Iops_HillisSteele_tacticProjects__step___closed__2 = (const lean_object*)&lp_zhc__lean_Zhc_Iops_HillisSteele_tacticProjects__step___closed__2_value;
static const lean_string_object lp_zhc__lean_Zhc_Iops_HillisSteele_tacticProjects__step___closed__3_value = {.m_header = {.m_rc = 0, .m_cs_sz = 0, .m_other = 0, .m_tag = 249}, .m_size = 20, .m_capacity = 20, .m_length = 19, .m_data = "tacticProjects_step"};
static const lean_object* lp_zhc__lean_Zhc_Iops_HillisSteele_tacticProjects__step___closed__3 = (const lean_object*)&lp_zhc__lean_Zhc_Iops_HillisSteele_tacticProjects__step___closed__3_value;
static const lean_ctor_object lp_zhc__lean_Zhc_Iops_HillisSteele_tacticProjects__step___closed__4_value_aux_0 = {.m_header = {.m_rc = 0, .m_cs_sz = sizeof(lean_ctor_object) + sizeof(void*)*2 + 8, .m_other = 2, .m_tag = 1}, .m_objs = {((lean_object*)(((size_t)(0) << 1) | 1)),((lean_object*)&lp_zhc__lean_Zhc_Iops_HillisSteele_tacticProjects__step___closed__0_value),LEAN_SCALAR_PTR_LITERAL(204, 210, 61, 107, 112, 216, 72, 96)}};
static const lean_ctor_object lp_zhc__lean_Zhc_Iops_HillisSteele_tacticProjects__step___closed__4_value_aux_1 = {.m_header = {.m_rc = 0, .m_cs_sz = sizeof(lean_ctor_object) + sizeof(void*)*2 + 8, .m_other = 2, .m_tag = 1}, .m_objs = {((lean_object*)&lp_zhc__lean_Zhc_Iops_HillisSteele_tacticProjects__step___closed__4_value_aux_0),((lean_object*)&lp_zhc__lean_Zhc_Iops_HillisSteele_tacticProjects__step___closed__1_value),LEAN_SCALAR_PTR_LITERAL(244, 252, 233, 116, 44, 222, 42, 52)}};
static const lean_ctor_object lp_zhc__lean_Zhc_Iops_HillisSteele_tacticProjects__step___closed__4_value_aux_2 = {.m_header = {.m_rc = 0, .m_cs_sz = sizeof(lean_ctor_object) + sizeof(void*)*2 + 8, .m_other = 2, .m_tag = 1}, .m_objs = {((lean_object*)&lp_zhc__lean_Zhc_Iops_HillisSteele_tacticProjects__step___closed__4_value_aux_1),((lean_object*)&lp_zhc__lean_Zhc_Iops_HillisSteele_tacticProjects__step___closed__2_value),LEAN_SCALAR_PTR_LITERAL(87, 206, 43, 155, 83, 74, 150, 212)}};
static const lean_ctor_object lp_zhc__lean_Zhc_Iops_HillisSteele_tacticProjects__step___closed__4_value = {.m_header = {.m_rc = 0, .m_cs_sz = sizeof(lean_ctor_object) + sizeof(void*)*2 + 8, .m_other = 2, .m_tag = 1}, .m_objs = {((lean_object*)&lp_zhc__lean_Zhc_Iops_HillisSteele_tacticProjects__step___closed__4_value_aux_2),((lean_object*)&lp_zhc__lean_Zhc_Iops_HillisSteele_tacticProjects__step___closed__3_value),LEAN_SCALAR_PTR_LITERAL(36, 84, 43, 11, 209, 209, 89, 56)}};
static const lean_object* lp_zhc__lean_Zhc_Iops_HillisSteele_tacticProjects__step___closed__4 = (const lean_object*)&lp_zhc__lean_Zhc_Iops_HillisSteele_tacticProjects__step___closed__4_value;
static const lean_string_object lp_zhc__lean_Zhc_Iops_HillisSteele_tacticProjects__step___closed__5_value = {.m_header = {.m_rc = 0, .m_cs_sz = 0, .m_other = 0, .m_tag = 249}, .m_size = 14, .m_capacity = 14, .m_length = 13, .m_data = "projects_step"};
static const lean_object* lp_zhc__lean_Zhc_Iops_HillisSteele_tacticProjects__step___closed__5 = (const lean_object*)&lp_zhc__lean_Zhc_Iops_HillisSteele_tacticProjects__step___closed__5_value;
static const lean_ctor_object lp_zhc__lean_Zhc_Iops_HillisSteele_tacticProjects__step___closed__6_value = {.m_header = {.m_rc = 0, .m_cs_sz = sizeof(lean_ctor_object) + sizeof(void*)*1 + 8, .m_other = 1, .m_tag = 6}, .m_objs = {((lean_object*)&lp_zhc__lean_Zhc_Iops_HillisSteele_tacticProjects__step___closed__5_value),LEAN_SCALAR_PTR_LITERAL(0, 0, 0, 0, 0, 0, 0, 0)}};
static const lean_object* lp_zhc__lean_Zhc_Iops_HillisSteele_tacticProjects__step___closed__6 = (const lean_object*)&lp_zhc__lean_Zhc_Iops_HillisSteele_tacticProjects__step___closed__6_value;
static const lean_ctor_object lp_zhc__lean_Zhc_Iops_HillisSteele_tacticProjects__step___closed__7_value = {.m_header = {.m_rc = 0, .m_cs_sz = sizeof(lean_ctor_object) + sizeof(void*)*3 + 0, .m_other = 3, .m_tag = 3}, .m_objs = {((lean_object*)&lp_zhc__lean_Zhc_Iops_HillisSteele_tacticProjects__step___closed__4_value),((lean_object*)(((size_t)(1024) << 1) | 1)),((lean_object*)&lp_zhc__lean_Zhc_Iops_HillisSteele_tacticProjects__step___closed__6_value)}};
static const lean_object* lp_zhc__lean_Zhc_Iops_HillisSteele_tacticProjects__step___closed__7 = (const lean_object*)&lp_zhc__lean_Zhc_Iops_HillisSteele_tacticProjects__step___closed__7_value;
LEAN_EXPORT const lean_object* lp_zhc__lean_Zhc_Iops_HillisSteele_tacticProjects__step = (const lean_object*)&lp_zhc__lean_Zhc_Iops_HillisSteele_tacticProjects__step___closed__7_value;
static const lean_string_object lp_zhc__lean_Zhc_Iops_HillisSteele___aux__Zhc__Iops__AddHillisSteele__Theorems______macroRules__Zhc__Iops__HillisSteele__tacticProjects__step__1___closed__0_value = {.m_header = {.m_rc = 0, .m_cs_sz = 0, .m_other = 0, .m_tag = 249}, .m_size = 5, .m_capacity = 5, .m_length = 4, .m_data = "Lean"};
static const lean_object* lp_zhc__lean_Zhc_Iops_HillisSteele___aux__Zhc__Iops__AddHillisSteele__Theorems______macroRules__Zhc__Iops__HillisSteele__tacticProjects__step__1___closed__0 = (const lean_object*)&lp_zhc__lean_Zhc_Iops_HillisSteele___aux__Zhc__Iops__AddHillisSteele__Theorems______macroRules__Zhc__Iops__HillisSteele__tacticProjects__step__1___closed__0_value;
static const lean_string_object lp_zhc__lean_Zhc_Iops_HillisSteele___aux__Zhc__Iops__AddHillisSteele__Theorems______macroRules__Zhc__Iops__HillisSteele__tacticProjects__step__1___closed__1_value = {.m_header = {.m_rc = 0, .m_cs_sz = 0, .m_other = 0, .m_tag = 249}, .m_size = 7, .m_capacity = 7, .m_length = 6, .m_data = "Parser"};
static const lean_object* lp_zhc__lean_Zhc_Iops_HillisSteele___aux__Zhc__Iops__AddHillisSteele__Theorems______macroRules__Zhc__Iops__HillisSteele__tacticProjects__step__1___closed__1 = (const lean_object*)&lp_zhc__lean_Zhc_Iops_HillisSteele___aux__Zhc__Iops__AddHillisSteele__Theorems______macroRules__Zhc__Iops__HillisSteele__tacticProjects__step__1___closed__1_value;
static const lean_string_object lp_zhc__lean_Zhc_Iops_HillisSteele___aux__Zhc__Iops__AddHillisSteele__Theorems______macroRules__Zhc__Iops__HillisSteele__tacticProjects__step__1___closed__2_value = {.m_header = {.m_rc = 0, .m_cs_sz = 0, .m_other = 0, .m_tag = 249}, .m_size = 7, .m_capacity = 7, .m_length = 6, .m_data = "Tactic"};
static const lean_object* lp_zhc__lean_Zhc_Iops_HillisSteele___aux__Zhc__Iops__AddHillisSteele__Theorems______macroRules__Zhc__Iops__HillisSteele__tacticProjects__step__1___closed__2 = (const lean_object*)&lp_zhc__lean_Zhc_Iops_HillisSteele___aux__Zhc__Iops__AddHillisSteele__Theorems______macroRules__Zhc__Iops__HillisSteele__tacticProjects__step__1___closed__2_value;
static const lean_string_object lp_zhc__lean_Zhc_Iops_HillisSteele___aux__Zhc__Iops__AddHillisSteele__Theorems______macroRules__Zhc__Iops__HillisSteele__tacticProjects__step__1___closed__3_value = {.m_header = {.m_rc = 0, .m_cs_sz = 0, .m_other = 0, .m_tag = 249}, .m_size = 6, .m_capacity = 6, .m_length = 5, .m_data = "first"};
static const lean_object* lp_zhc__lean_Zhc_Iops_HillisSteele___aux__Zhc__Iops__AddHillisSteele__Theorems______macroRules__Zhc__Iops__HillisSteele__tacticProjects__step__1___closed__3 = (const lean_object*)&lp_zhc__lean_Zhc_Iops_HillisSteele___aux__Zhc__Iops__AddHillisSteele__Theorems______macroRules__Zhc__Iops__HillisSteele__tacticProjects__step__1___closed__3_value;
static const lean_ctor_object lp_zhc__lean_Zhc_Iops_HillisSteele___aux__Zhc__Iops__AddHillisSteele__Theorems______macroRules__Zhc__Iops__HillisSteele__tacticProjects__step__1___closed__4_value_aux_0 = {.m_header = {.m_rc = 0, .m_cs_sz = sizeof(lean_ctor_object) + sizeof(void*)*2 + 8, .m_other = 2, .m_tag = 1}, .m_objs = {((lean_object*)(((size_t)(0) << 1) | 1)),((lean_object*)&lp_zhc__lean_Zhc_Iops_HillisSteele___aux__Zhc__Iops__AddHillisSteele__Theorems______macroRules__Zhc__Iops__HillisSteele__tacticProjects__step__1___closed__0_value),LEAN_SCALAR_PTR_LITERAL(70, 193, 83, 126, 233, 67, 208, 165)}};
static const lean_ctor_object lp_zhc__lean_Zhc_Iops_HillisSteele___aux__Zhc__Iops__AddHillisSteele__Theorems______macroRules__Zhc__Iops__HillisSteele__tacticProjects__step__1___closed__4_value_aux_1 = {.m_header = {.m_rc = 0, .m_cs_sz = sizeof(lean_ctor_object) + sizeof(void*)*2 + 8, .m_other = 2, .m_tag = 1}, .m_objs = {((lean_object*)&lp_zhc__lean_Zhc_Iops_HillisSteele___aux__Zhc__Iops__AddHillisSteele__Theorems______macroRules__Zhc__Iops__HillisSteele__tacticProjects__step__1___closed__4_value_aux_0),((lean_object*)&lp_zhc__lean_Zhc_Iops_HillisSteele___aux__Zhc__Iops__AddHillisSteele__Theorems______macroRules__Zhc__Iops__HillisSteele__tacticProjects__step__1___closed__1_value),LEAN_SCALAR_PTR_LITERAL(103, 136, 125, 166, 167, 98, 71, 111)}};
static const lean_ctor_object lp_zhc__lean_Zhc_Iops_HillisSteele___aux__Zhc__Iops__AddHillisSteele__Theorems______macroRules__Zhc__Iops__HillisSteele__tacticProjects__step__1___closed__4_value_aux_2 = {.m_header = {.m_rc = 0, .m_cs_sz = sizeof(lean_ctor_object) + sizeof(void*)*2 + 8, .m_other = 2, .m_tag = 1}, .m_objs = {((lean_object*)&lp_zhc__lean_Zhc_Iops_HillisSteele___aux__Zhc__Iops__AddHillisSteele__Theorems______macroRules__Zhc__Iops__HillisSteele__tacticProjects__step__1___closed__4_value_aux_1),((lean_object*)&lp_zhc__lean_Zhc_Iops_HillisSteele___aux__Zhc__Iops__AddHillisSteele__Theorems______macroRules__Zhc__Iops__HillisSteele__tacticProjects__step__1___closed__2_value),LEAN_SCALAR_PTR_LITERAL(166, 58, 35, 182, 187, 130, 147, 254)}};
static const lean_ctor_object lp_zhc__lean_Zhc_Iops_HillisSteele___aux__Zhc__Iops__AddHillisSteele__Theorems______macroRules__Zhc__Iops__HillisSteele__tacticProjects__step__1___closed__4_value = {.m_header = {.m_rc = 0, .m_cs_sz = sizeof(lean_ctor_object) + sizeof(void*)*2 + 8, .m_other = 2, .m_tag = 1}, .m_objs = {((lean_object*)&lp_zhc__lean_Zhc_Iops_HillisSteele___aux__Zhc__Iops__AddHillisSteele__Theorems______macroRules__Zhc__Iops__HillisSteele__tacticProjects__step__1___closed__4_value_aux_2),((lean_object*)&lp_zhc__lean_Zhc_Iops_HillisSteele___aux__Zhc__Iops__AddHillisSteele__Theorems______macroRules__Zhc__Iops__HillisSteele__tacticProjects__step__1___closed__3_value),LEAN_SCALAR_PTR_LITERAL(59, 232, 35, 17, 172, 62, 48, 174)}};
static const lean_object* lp_zhc__lean_Zhc_Iops_HillisSteele___aux__Zhc__Iops__AddHillisSteele__Theorems______macroRules__Zhc__Iops__HillisSteele__tacticProjects__step__1___closed__4 = (const lean_object*)&lp_zhc__lean_Zhc_Iops_HillisSteele___aux__Zhc__Iops__AddHillisSteele__Theorems______macroRules__Zhc__Iops__HillisSteele__tacticProjects__step__1___closed__4_value;
static const lean_string_object lp_zhc__lean_Zhc_Iops_HillisSteele___aux__Zhc__Iops__AddHillisSteele__Theorems______macroRules__Zhc__Iops__HillisSteele__tacticProjects__step__1___closed__5_value = {.m_header = {.m_rc = 0, .m_cs_sz = 0, .m_other = 0, .m_tag = 249}, .m_size = 5, .m_capacity = 5, .m_length = 4, .m_data = "null"};
static const lean_object* lp_zhc__lean_Zhc_Iops_HillisSteele___aux__Zhc__Iops__AddHillisSteele__Theorems______macroRules__Zhc__Iops__HillisSteele__tacticProjects__step__1___closed__5 = (const lean_object*)&lp_zhc__lean_Zhc_Iops_HillisSteele___aux__Zhc__Iops__AddHillisSteele__Theorems______macroRules__Zhc__Iops__HillisSteele__tacticProjects__step__1___closed__5_value;
static const lean_ctor_object lp_zhc__lean_Zhc_Iops_HillisSteele___aux__Zhc__Iops__AddHillisSteele__Theorems______macroRules__Zhc__Iops__HillisSteele__tacticProjects__step__1___closed__6_value = {.m_header = {.m_rc = 0, .m_cs_sz = sizeof(lean_ctor_object) + sizeof(void*)*2 + 8, .m_other = 2, .m_tag = 1}, .m_objs = {((lean_object*)(((size_t)(0) << 1) | 1)),((lean_object*)&lp_zhc__lean_Zhc_Iops_HillisSteele___aux__Zhc__Iops__AddHillisSteele__Theorems______macroRules__Zhc__Iops__HillisSteele__tacticProjects__step__1___closed__5_value),LEAN_SCALAR_PTR_LITERAL(24, 58, 49, 223, 146, 207, 197, 136)}};
static const lean_object* lp_zhc__lean_Zhc_Iops_HillisSteele___aux__Zhc__Iops__AddHillisSteele__Theorems______macroRules__Zhc__Iops__HillisSteele__tacticProjects__step__1___closed__6 = (const lean_object*)&lp_zhc__lean_Zhc_Iops_HillisSteele___aux__Zhc__Iops__AddHillisSteele__Theorems______macroRules__Zhc__Iops__HillisSteele__tacticProjects__step__1___closed__6_value;
static const lean_string_object lp_zhc__lean_Zhc_Iops_HillisSteele___aux__Zhc__Iops__AddHillisSteele__Theorems______macroRules__Zhc__Iops__HillisSteele__tacticProjects__step__1___closed__7_value = {.m_header = {.m_rc = 0, .m_cs_sz = 0, .m_other = 0, .m_tag = 249}, .m_size = 6, .m_capacity = 6, .m_length = 5, .m_data = "group"};
static const lean_object* lp_zhc__lean_Zhc_Iops_HillisSteele___aux__Zhc__Iops__AddHillisSteele__Theorems______macroRules__Zhc__Iops__HillisSteele__tacticProjects__step__1___closed__7 = (const lean_object*)&lp_zhc__lean_Zhc_Iops_HillisSteele___aux__Zhc__Iops__AddHillisSteele__Theorems______macroRules__Zhc__Iops__HillisSteele__tacticProjects__step__1___closed__7_value;
static const lean_ctor_object lp_zhc__lean_Zhc_Iops_HillisSteele___aux__Zhc__Iops__AddHillisSteele__Theorems______macroRules__Zhc__Iops__HillisSteele__tacticProjects__step__1___closed__8_value = {.m_header = {.m_rc = 0, .m_cs_sz = sizeof(lean_ctor_object) + sizeof(void*)*2 + 8, .m_other = 2, .m_tag = 1}, .m_objs = {((lean_object*)(((size_t)(0) << 1) | 1)),((lean_object*)&lp_zhc__lean_Zhc_Iops_HillisSteele___aux__Zhc__Iops__AddHillisSteele__Theorems______macroRules__Zhc__Iops__HillisSteele__tacticProjects__step__1___closed__7_value),LEAN_SCALAR_PTR_LITERAL(206, 113, 20, 57, 188, 177, 187, 30)}};
static const lean_object* lp_zhc__lean_Zhc_Iops_HillisSteele___aux__Zhc__Iops__AddHillisSteele__Theorems______macroRules__Zhc__Iops__HillisSteele__tacticProjects__step__1___closed__8 = (const lean_object*)&lp_zhc__lean_Zhc_Iops_HillisSteele___aux__Zhc__Iops__AddHillisSteele__Theorems______macroRules__Zhc__Iops__HillisSteele__tacticProjects__step__1___closed__8_value;
static const lean_string_object lp_zhc__lean_Zhc_Iops_HillisSteele___aux__Zhc__Iops__AddHillisSteele__Theorems______macroRules__Zhc__Iops__HillisSteele__tacticProjects__step__1___closed__9_value = {.m_header = {.m_rc = 0, .m_cs_sz = 0, .m_other = 0, .m_tag = 249}, .m_size = 2, .m_capacity = 2, .m_length = 1, .m_data = "|"};
static const lean_object* lp_zhc__lean_Zhc_Iops_HillisSteele___aux__Zhc__Iops__AddHillisSteele__Theorems______macroRules__Zhc__Iops__HillisSteele__tacticProjects__step__1___closed__9 = (const lean_object*)&lp_zhc__lean_Zhc_Iops_HillisSteele___aux__Zhc__Iops__AddHillisSteele__Theorems______macroRules__Zhc__Iops__HillisSteele__tacticProjects__step__1___closed__9_value;
static const lean_string_object lp_zhc__lean_Zhc_Iops_HillisSteele___aux__Zhc__Iops__AddHillisSteele__Theorems______macroRules__Zhc__Iops__HillisSteele__tacticProjects__step__1___closed__10_value = {.m_header = {.m_rc = 0, .m_cs_sz = 0, .m_other = 0, .m_tag = 249}, .m_size = 10, .m_capacity = 10, .m_length = 9, .m_data = "tacticSeq"};
static const lean_object* lp_zhc__lean_Zhc_Iops_HillisSteele___aux__Zhc__Iops__AddHillisSteele__Theorems______macroRules__Zhc__Iops__HillisSteele__tacticProjects__step__1___closed__10 = (const lean_object*)&lp_zhc__lean_Zhc_Iops_HillisSteele___aux__Zhc__Iops__AddHillisSteele__Theorems______macroRules__Zhc__Iops__HillisSteele__tacticProjects__step__1___closed__10_value;
static const lean_ctor_object lp_zhc__lean_Zhc_Iops_HillisSteele___aux__Zhc__Iops__AddHillisSteele__Theorems______macroRules__Zhc__Iops__HillisSteele__tacticProjects__step__1___closed__11_value_aux_0 = {.m_header = {.m_rc = 0, .m_cs_sz = sizeof(lean_ctor_object) + sizeof(void*)*2 + 8, .m_other = 2, .m_tag = 1}, .m_objs = {((lean_object*)(((size_t)(0) << 1) | 1)),((lean_object*)&lp_zhc__lean_Zhc_Iops_HillisSteele___aux__Zhc__Iops__AddHillisSteele__Theorems______macroRules__Zhc__Iops__HillisSteele__tacticProjects__step__1___closed__0_value),LEAN_SCALAR_PTR_LITERAL(70, 193, 83, 126, 233, 67, 208, 165)}};
static const lean_ctor_object lp_zhc__lean_Zhc_Iops_HillisSteele___aux__Zhc__Iops__AddHillisSteele__Theorems______macroRules__Zhc__Iops__HillisSteele__tacticProjects__step__1___closed__11_value_aux_1 = {.m_header = {.m_rc = 0, .m_cs_sz = sizeof(lean_ctor_object) + sizeof(void*)*2 + 8, .m_other = 2, .m_tag = 1}, .m_objs = {((lean_object*)&lp_zhc__lean_Zhc_Iops_HillisSteele___aux__Zhc__Iops__AddHillisSteele__Theorems______macroRules__Zhc__Iops__HillisSteele__tacticProjects__step__1___closed__11_value_aux_0),((lean_object*)&lp_zhc__lean_Zhc_Iops_HillisSteele___aux__Zhc__Iops__AddHillisSteele__Theorems______macroRules__Zhc__Iops__HillisSteele__tacticProjects__step__1___closed__1_value),LEAN_SCALAR_PTR_LITERAL(103, 136, 125, 166, 167, 98, 71, 111)}};
static const lean_ctor_object lp_zhc__lean_Zhc_Iops_HillisSteele___aux__Zhc__Iops__AddHillisSteele__Theorems______macroRules__Zhc__Iops__HillisSteele__tacticProjects__step__1___closed__11_value_aux_2 = {.m_header = {.m_rc = 0, .m_cs_sz = sizeof(lean_ctor_object) + sizeof(void*)*2 + 8, .m_other = 2, .m_tag = 1}, .m_objs = {((lean_object*)&lp_zhc__lean_Zhc_Iops_HillisSteele___aux__Zhc__Iops__AddHillisSteele__Theorems______macroRules__Zhc__Iops__HillisSteele__tacticProjects__step__1___closed__11_value_aux_1),((lean_object*)&lp_zhc__lean_Zhc_Iops_HillisSteele___aux__Zhc__Iops__AddHillisSteele__Theorems______macroRules__Zhc__Iops__HillisSteele__tacticProjects__step__1___closed__2_value),LEAN_SCALAR_PTR_LITERAL(166, 58, 35, 182, 187, 130, 147, 254)}};
static const lean_ctor_object lp_zhc__lean_Zhc_Iops_HillisSteele___aux__Zhc__Iops__AddHillisSteele__Theorems______macroRules__Zhc__Iops__HillisSteele__tacticProjects__step__1___closed__11_value = {.m_header = {.m_rc = 0, .m_cs_sz = sizeof(lean_ctor_object) + sizeof(void*)*2 + 8, .m_other = 2, .m_tag = 1}, .m_objs = {((lean_object*)&lp_zhc__lean_Zhc_Iops_HillisSteele___aux__Zhc__Iops__AddHillisSteele__Theorems______macroRules__Zhc__Iops__HillisSteele__tacticProjects__step__1___closed__11_value_aux_2),((lean_object*)&lp_zhc__lean_Zhc_Iops_HillisSteele___aux__Zhc__Iops__AddHillisSteele__Theorems______macroRules__Zhc__Iops__HillisSteele__tacticProjects__step__1___closed__10_value),LEAN_SCALAR_PTR_LITERAL(212, 140, 85, 215, 241, 69, 7, 118)}};
static const lean_object* lp_zhc__lean_Zhc_Iops_HillisSteele___aux__Zhc__Iops__AddHillisSteele__Theorems______macroRules__Zhc__Iops__HillisSteele__tacticProjects__step__1___closed__11 = (const lean_object*)&lp_zhc__lean_Zhc_Iops_HillisSteele___aux__Zhc__Iops__AddHillisSteele__Theorems______macroRules__Zhc__Iops__HillisSteele__tacticProjects__step__1___closed__11_value;
static const lean_string_object lp_zhc__lean_Zhc_Iops_HillisSteele___aux__Zhc__Iops__AddHillisSteele__Theorems______macroRules__Zhc__Iops__HillisSteele__tacticProjects__step__1___closed__12_value = {.m_header = {.m_rc = 0, .m_cs_sz = 0, .m_other = 0, .m_tag = 249}, .m_size = 19, .m_capacity = 19, .m_length = 18, .m_data = "tacticSeq1Indented"};
static const lean_object* lp_zhc__lean_Zhc_Iops_HillisSteele___aux__Zhc__Iops__AddHillisSteele__Theorems______macroRules__Zhc__Iops__HillisSteele__tacticProjects__step__1___closed__12 = (const lean_object*)&lp_zhc__lean_Zhc_Iops_HillisSteele___aux__Zhc__Iops__AddHillisSteele__Theorems______macroRules__Zhc__Iops__HillisSteele__tacticProjects__step__1___closed__12_value;
static const lean_ctor_object lp_zhc__lean_Zhc_Iops_HillisSteele___aux__Zhc__Iops__AddHillisSteele__Theorems______macroRules__Zhc__Iops__HillisSteele__tacticProjects__step__1___closed__13_value_aux_0 = {.m_header = {.m_rc = 0, .m_cs_sz = sizeof(lean_ctor_object) + sizeof(void*)*2 + 8, .m_other = 2, .m_tag = 1}, .m_objs = {((lean_object*)(((size_t)(0) << 1) | 1)),((lean_object*)&lp_zhc__lean_Zhc_Iops_HillisSteele___aux__Zhc__Iops__AddHillisSteele__Theorems______macroRules__Zhc__Iops__HillisSteele__tacticProjects__step__1___closed__0_value),LEAN_SCALAR_PTR_LITERAL(70, 193, 83, 126, 233, 67, 208, 165)}};
static const lean_ctor_object lp_zhc__lean_Zhc_Iops_HillisSteele___aux__Zhc__Iops__AddHillisSteele__Theorems______macroRules__Zhc__Iops__HillisSteele__tacticProjects__step__1___closed__13_value_aux_1 = {.m_header = {.m_rc = 0, .m_cs_sz = sizeof(lean_ctor_object) + sizeof(void*)*2 + 8, .m_other = 2, .m_tag = 1}, .m_objs = {((lean_object*)&lp_zhc__lean_Zhc_Iops_HillisSteele___aux__Zhc__Iops__AddHillisSteele__Theorems______macroRules__Zhc__Iops__HillisSteele__tacticProjects__step__1___closed__13_value_aux_0),((lean_object*)&lp_zhc__lean_Zhc_Iops_HillisSteele___aux__Zhc__Iops__AddHillisSteele__Theorems______macroRules__Zhc__Iops__HillisSteele__tacticProjects__step__1___closed__1_value),LEAN_SCALAR_PTR_LITERAL(103, 136, 125, 166, 167, 98, 71, 111)}};
static const lean_ctor_object lp_zhc__lean_Zhc_Iops_HillisSteele___aux__Zhc__Iops__AddHillisSteele__Theorems______macroRules__Zhc__Iops__HillisSteele__tacticProjects__step__1___closed__13_value_aux_2 = {.m_header = {.m_rc = 0, .m_cs_sz = sizeof(lean_ctor_object) + sizeof(void*)*2 + 8, .m_other = 2, .m_tag = 1}, .m_objs = {((lean_object*)&lp_zhc__lean_Zhc_Iops_HillisSteele___aux__Zhc__Iops__AddHillisSteele__Theorems______macroRules__Zhc__Iops__HillisSteele__tacticProjects__step__1___closed__13_value_aux_1),((lean_object*)&lp_zhc__lean_Zhc_Iops_HillisSteele___aux__Zhc__Iops__AddHillisSteele__Theorems______macroRules__Zhc__Iops__HillisSteele__tacticProjects__step__1___closed__2_value),LEAN_SCALAR_PTR_LITERAL(166, 58, 35, 182, 187, 130, 147, 254)}};
static const lean_ctor_object lp_zhc__lean_Zhc_Iops_HillisSteele___aux__Zhc__Iops__AddHillisSteele__Theorems______macroRules__Zhc__Iops__HillisSteele__tacticProjects__step__1___closed__13_value = {.m_header = {.m_rc = 0, .m_cs_sz = sizeof(lean_ctor_object) + sizeof(void*)*2 + 8, .m_other = 2, .m_tag = 1}, .m_objs = {((lean_object*)&lp_zhc__lean_Zhc_Iops_HillisSteele___aux__Zhc__Iops__AddHillisSteele__Theorems______macroRules__Zhc__Iops__HillisSteele__tacticProjects__step__1___closed__13_value_aux_2),((lean_object*)&lp_zhc__lean_Zhc_Iops_HillisSteele___aux__Zhc__Iops__AddHillisSteele__Theorems______macroRules__Zhc__Iops__HillisSteele__tacticProjects__step__1___closed__12_value),LEAN_SCALAR_PTR_LITERAL(223, 90, 160, 238, 133, 180, 23, 239)}};
static const lean_object* lp_zhc__lean_Zhc_Iops_HillisSteele___aux__Zhc__Iops__AddHillisSteele__Theorems______macroRules__Zhc__Iops__HillisSteele__tacticProjects__step__1___closed__13 = (const lean_object*)&lp_zhc__lean_Zhc_Iops_HillisSteele___aux__Zhc__Iops__AddHillisSteele__Theorems______macroRules__Zhc__Iops__HillisSteele__tacticProjects__step__1___closed__13_value;
static const lean_string_object lp_zhc__lean_Zhc_Iops_HillisSteele___aux__Zhc__Iops__AddHillisSteele__Theorems______macroRules__Zhc__Iops__HillisSteele__tacticProjects__step__1___closed__14_value = {.m_header = {.m_rc = 0, .m_cs_sz = 0, .m_other = 0, .m_tag = 249}, .m_size = 6, .m_capacity = 6, .m_length = 5, .m_data = "exact"};
static const lean_object* lp_zhc__lean_Zhc_Iops_HillisSteele___aux__Zhc__Iops__AddHillisSteele__Theorems______macroRules__Zhc__Iops__HillisSteele__tacticProjects__step__1___closed__14 = (const lean_object*)&lp_zhc__lean_Zhc_Iops_HillisSteele___aux__Zhc__Iops__AddHillisSteele__Theorems______macroRules__Zhc__Iops__HillisSteele__tacticProjects__step__1___closed__14_value;
static const lean_ctor_object lp_zhc__lean_Zhc_Iops_HillisSteele___aux__Zhc__Iops__AddHillisSteele__Theorems______macroRules__Zhc__Iops__HillisSteele__tacticProjects__step__1___closed__15_value_aux_0 = {.m_header = {.m_rc = 0, .m_cs_sz = sizeof(lean_ctor_object) + sizeof(void*)*2 + 8, .m_other = 2, .m_tag = 1}, .m_objs = {((lean_object*)(((size_t)(0) << 1) | 1)),((lean_object*)&lp_zhc__lean_Zhc_Iops_HillisSteele___aux__Zhc__Iops__AddHillisSteele__Theorems______macroRules__Zhc__Iops__HillisSteele__tacticProjects__step__1___closed__0_value),LEAN_SCALAR_PTR_LITERAL(70, 193, 83, 126, 233, 67, 208, 165)}};
static const lean_ctor_object lp_zhc__lean_Zhc_Iops_HillisSteele___aux__Zhc__Iops__AddHillisSteele__Theorems______macroRules__Zhc__Iops__HillisSteele__tacticProjects__step__1___closed__15_value_aux_1 = {.m_header = {.m_rc = 0, .m_cs_sz = sizeof(lean_ctor_object) + sizeof(void*)*2 + 8, .m_other = 2, .m_tag = 1}, .m_objs = {((lean_object*)&lp_zhc__lean_Zhc_Iops_HillisSteele___aux__Zhc__Iops__AddHillisSteele__Theorems______macroRules__Zhc__Iops__HillisSteele__tacticProjects__step__1___closed__15_value_aux_0),((lean_object*)&lp_zhc__lean_Zhc_Iops_HillisSteele___aux__Zhc__Iops__AddHillisSteele__Theorems______macroRules__Zhc__Iops__HillisSteele__tacticProjects__step__1___closed__1_value),LEAN_SCALAR_PTR_LITERAL(103, 136, 125, 166, 167, 98, 71, 111)}};
static const lean_ctor_object lp_zhc__lean_Zhc_Iops_HillisSteele___aux__Zhc__Iops__AddHillisSteele__Theorems______macroRules__Zhc__Iops__HillisSteele__tacticProjects__step__1___closed__15_value_aux_2 = {.m_header = {.m_rc = 0, .m_cs_sz = sizeof(lean_ctor_object) + sizeof(void*)*2 + 8, .m_other = 2, .m_tag = 1}, .m_objs = {((lean_object*)&lp_zhc__lean_Zhc_Iops_HillisSteele___aux__Zhc__Iops__AddHillisSteele__Theorems______macroRules__Zhc__Iops__HillisSteele__tacticProjects__step__1___closed__15_value_aux_1),((lean_object*)&lp_zhc__lean_Zhc_Iops_HillisSteele___aux__Zhc__Iops__AddHillisSteele__Theorems______macroRules__Zhc__Iops__HillisSteele__tacticProjects__step__1___closed__2_value),LEAN_SCALAR_PTR_LITERAL(166, 58, 35, 182, 187, 130, 147, 254)}};
static const lean_ctor_object lp_zhc__lean_Zhc_Iops_HillisSteele___aux__Zhc__Iops__AddHillisSteele__Theorems______macroRules__Zhc__Iops__HillisSteele__tacticProjects__step__1___closed__15_value = {.m_header = {.m_rc = 0, .m_cs_sz = sizeof(lean_ctor_object) + sizeof(void*)*2 + 8, .m_other = 2, .m_tag = 1}, .m_objs = {((lean_object*)&lp_zhc__lean_Zhc_Iops_HillisSteele___aux__Zhc__Iops__AddHillisSteele__Theorems______macroRules__Zhc__Iops__HillisSteele__tacticProjects__step__1___closed__15_value_aux_2),((lean_object*)&lp_zhc__lean_Zhc_Iops_HillisSteele___aux__Zhc__Iops__AddHillisSteele__Theorems______macroRules__Zhc__Iops__HillisSteele__tacticProjects__step__1___closed__14_value),LEAN_SCALAR_PTR_LITERAL(108, 106, 111, 83, 219, 207, 32, 208)}};
static const lean_object* lp_zhc__lean_Zhc_Iops_HillisSteele___aux__Zhc__Iops__AddHillisSteele__Theorems______macroRules__Zhc__Iops__HillisSteele__tacticProjects__step__1___closed__15 = (const lean_object*)&lp_zhc__lean_Zhc_Iops_HillisSteele___aux__Zhc__Iops__AddHillisSteele__Theorems______macroRules__Zhc__Iops__HillisSteele__tacticProjects__step__1___closed__15_value;
static const lean_string_object lp_zhc__lean_Zhc_Iops_HillisSteele___aux__Zhc__Iops__AddHillisSteele__Theorems______macroRules__Zhc__Iops__HillisSteele__tacticProjects__step__1___closed__16_value = {.m_header = {.m_rc = 0, .m_cs_sz = 0, .m_other = 0, .m_tag = 249}, .m_size = 5, .m_capacity = 5, .m_length = 4, .m_data = "Term"};
static const lean_object* lp_zhc__lean_Zhc_Iops_HillisSteele___aux__Zhc__Iops__AddHillisSteele__Theorems______macroRules__Zhc__Iops__HillisSteele__tacticProjects__step__1___closed__16 = (const lean_object*)&lp_zhc__lean_Zhc_Iops_HillisSteele___aux__Zhc__Iops__AddHillisSteele__Theorems______macroRules__Zhc__Iops__HillisSteele__tacticProjects__step__1___closed__16_value;
static const lean_string_object lp_zhc__lean_Zhc_Iops_HillisSteele___aux__Zhc__Iops__AddHillisSteele__Theorems______macroRules__Zhc__Iops__HillisSteele__tacticProjects__step__1___closed__17_value = {.m_header = {.m_rc = 0, .m_cs_sz = 0, .m_other = 0, .m_tag = 249}, .m_size = 4, .m_capacity = 4, .m_length = 3, .m_data = "app"};
static const lean_object* lp_zhc__lean_Zhc_Iops_HillisSteele___aux__Zhc__Iops__AddHillisSteele__Theorems______macroRules__Zhc__Iops__HillisSteele__tacticProjects__step__1___closed__17 = (const lean_object*)&lp_zhc__lean_Zhc_Iops_HillisSteele___aux__Zhc__Iops__AddHillisSteele__Theorems______macroRules__Zhc__Iops__HillisSteele__tacticProjects__step__1___closed__17_value;
static const lean_ctor_object lp_zhc__lean_Zhc_Iops_HillisSteele___aux__Zhc__Iops__AddHillisSteele__Theorems______macroRules__Zhc__Iops__HillisSteele__tacticProjects__step__1___closed__18_value_aux_0 = {.m_header = {.m_rc = 0, .m_cs_sz = sizeof(lean_ctor_object) + sizeof(void*)*2 + 8, .m_other = 2, .m_tag = 1}, .m_objs = {((lean_object*)(((size_t)(0) << 1) | 1)),((lean_object*)&lp_zhc__lean_Zhc_Iops_HillisSteele___aux__Zhc__Iops__AddHillisSteele__Theorems______macroRules__Zhc__Iops__HillisSteele__tacticProjects__step__1___closed__0_value),LEAN_SCALAR_PTR_LITERAL(70, 193, 83, 126, 233, 67, 208, 165)}};
static const lean_ctor_object lp_zhc__lean_Zhc_Iops_HillisSteele___aux__Zhc__Iops__AddHillisSteele__Theorems______macroRules__Zhc__Iops__HillisSteele__tacticProjects__step__1___closed__18_value_aux_1 = {.m_header = {.m_rc = 0, .m_cs_sz = sizeof(lean_ctor_object) + sizeof(void*)*2 + 8, .m_other = 2, .m_tag = 1}, .m_objs = {((lean_object*)&lp_zhc__lean_Zhc_Iops_HillisSteele___aux__Zhc__Iops__AddHillisSteele__Theorems______macroRules__Zhc__Iops__HillisSteele__tacticProjects__step__1___closed__18_value_aux_0),((lean_object*)&lp_zhc__lean_Zhc_Iops_HillisSteele___aux__Zhc__Iops__AddHillisSteele__Theorems______macroRules__Zhc__Iops__HillisSteele__tacticProjects__step__1___closed__1_value),LEAN_SCALAR_PTR_LITERAL(103, 136, 125, 166, 167, 98, 71, 111)}};
static const lean_ctor_object lp_zhc__lean_Zhc_Iops_HillisSteele___aux__Zhc__Iops__AddHillisSteele__Theorems______macroRules__Zhc__Iops__HillisSteele__tacticProjects__step__1___closed__18_value_aux_2 = {.m_header = {.m_rc = 0, .m_cs_sz = sizeof(lean_ctor_object) + sizeof(void*)*2 + 8, .m_other = 2, .m_tag = 1}, .m_objs = {((lean_object*)&lp_zhc__lean_Zhc_Iops_HillisSteele___aux__Zhc__Iops__AddHillisSteele__Theorems______macroRules__Zhc__Iops__HillisSteele__tacticProjects__step__1___closed__18_value_aux_1),((lean_object*)&lp_zhc__lean_Zhc_Iops_HillisSteele___aux__Zhc__Iops__AddHillisSteele__Theorems______macroRules__Zhc__Iops__HillisSteele__tacticProjects__step__1___closed__16_value),LEAN_SCALAR_PTR_LITERAL(75, 170, 162, 138, 136, 204, 251, 229)}};
static const lean_ctor_object lp_zhc__lean_Zhc_Iops_HillisSteele___aux__Zhc__Iops__AddHillisSteele__Theorems______macroRules__Zhc__Iops__HillisSteele__tacticProjects__step__1___closed__18_value = {.m_header = {.m_rc = 0, .m_cs_sz = sizeof(lean_ctor_object) + sizeof(void*)*2 + 8, .m_other = 2, .m_tag = 1}, .m_objs = {((lean_object*)&lp_zhc__lean_Zhc_Iops_HillisSteele___aux__Zhc__Iops__AddHillisSteele__Theorems______macroRules__Zhc__Iops__HillisSteele__tacticProjects__step__1___closed__18_value_aux_2),((lean_object*)&lp_zhc__lean_Zhc_Iops_HillisSteele___aux__Zhc__Iops__AddHillisSteele__Theorems______macroRules__Zhc__Iops__HillisSteele__tacticProjects__step__1___closed__17_value),LEAN_SCALAR_PTR_LITERAL(69, 118, 10, 41, 220, 156, 243, 179)}};
static const lean_object* lp_zhc__lean_Zhc_Iops_HillisSteele___aux__Zhc__Iops__AddHillisSteele__Theorems______macroRules__Zhc__Iops__HillisSteele__tacticProjects__step__1___closed__18 = (const lean_object*)&lp_zhc__lean_Zhc_Iops_HillisSteele___aux__Zhc__Iops__AddHillisSteele__Theorems______macroRules__Zhc__Iops__HillisSteele__tacticProjects__step__1___closed__18_value;
static const lean_string_object lp_zhc__lean_Zhc_Iops_HillisSteele___aux__Zhc__Iops__AddHillisSteele__Theorems______macroRules__Zhc__Iops__HillisSteele__tacticProjects__step__1___closed__19_value = {.m_header = {.m_rc = 0, .m_cs_sz = 0, .m_other = 0, .m_tag = 249}, .m_size = 14, .m_capacity = 14, .m_length = 13, .m_data = "Projects.pure"};
static const lean_object* lp_zhc__lean_Zhc_Iops_HillisSteele___aux__Zhc__Iops__AddHillisSteele__Theorems______macroRules__Zhc__Iops__HillisSteele__tacticProjects__step__1___closed__19 = (const lean_object*)&lp_zhc__lean_Zhc_Iops_HillisSteele___aux__Zhc__Iops__AddHillisSteele__Theorems______macroRules__Zhc__Iops__HillisSteele__tacticProjects__step__1___closed__19_value;
static lean_once_cell_t lp_zhc__lean_Zhc_Iops_HillisSteele___aux__Zhc__Iops__AddHillisSteele__Theorems______macroRules__Zhc__Iops__HillisSteele__tacticProjects__step__1___closed__20_once = LEAN_ONCE_CELL_INITIALIZER;
static lean_object* lp_zhc__lean_Zhc_Iops_HillisSteele___aux__Zhc__Iops__AddHillisSteele__Theorems______macroRules__Zhc__Iops__HillisSteele__tacticProjects__step__1___closed__20;
static const lean_string_object lp_zhc__lean_Zhc_Iops_HillisSteele___aux__Zhc__Iops__AddHillisSteele__Theorems______macroRules__Zhc__Iops__HillisSteele__tacticProjects__step__1___closed__21_value = {.m_header = {.m_rc = 0, .m_cs_sz = 0, .m_other = 0, .m_tag = 249}, .m_size = 9, .m_capacity = 9, .m_length = 8, .m_data = "Projects"};
static const lean_object* lp_zhc__lean_Zhc_Iops_HillisSteele___aux__Zhc__Iops__AddHillisSteele__Theorems______macroRules__Zhc__Iops__HillisSteele__tacticProjects__step__1___closed__21 = (const lean_object*)&lp_zhc__lean_Zhc_Iops_HillisSteele___aux__Zhc__Iops__AddHillisSteele__Theorems______macroRules__Zhc__Iops__HillisSteele__tacticProjects__step__1___closed__21_value;
static const lean_string_object lp_zhc__lean_Zhc_Iops_HillisSteele___aux__Zhc__Iops__AddHillisSteele__Theorems______macroRules__Zhc__Iops__HillisSteele__tacticProjects__step__1___closed__22_value = {.m_header = {.m_rc = 0, .m_cs_sz = 0, .m_other = 0, .m_tag = 249}, .m_size = 5, .m_capacity = 5, .m_length = 4, .m_data = "pure"};
static const lean_object* lp_zhc__lean_Zhc_Iops_HillisSteele___aux__Zhc__Iops__AddHillisSteele__Theorems______macroRules__Zhc__Iops__HillisSteele__tacticProjects__step__1___closed__22 = (const lean_object*)&lp_zhc__lean_Zhc_Iops_HillisSteele___aux__Zhc__Iops__AddHillisSteele__Theorems______macroRules__Zhc__Iops__HillisSteele__tacticProjects__step__1___closed__22_value;
static const lean_ctor_object lp_zhc__lean_Zhc_Iops_HillisSteele___aux__Zhc__Iops__AddHillisSteele__Theorems______macroRules__Zhc__Iops__HillisSteele__tacticProjects__step__1___closed__23_value_aux_0 = {.m_header = {.m_rc = 0, .m_cs_sz = sizeof(lean_ctor_object) + sizeof(void*)*2 + 8, .m_other = 2, .m_tag = 1}, .m_objs = {((lean_object*)(((size_t)(0) << 1) | 1)),((lean_object*)&lp_zhc__lean_Zhc_Iops_HillisSteele___aux__Zhc__Iops__AddHillisSteele__Theorems______macroRules__Zhc__Iops__HillisSteele__tacticProjects__step__1___closed__21_value),LEAN_SCALAR_PTR_LITERAL(96, 117, 157, 119, 138, 122, 129, 92)}};
static const lean_ctor_object lp_zhc__lean_Zhc_Iops_HillisSteele___aux__Zhc__Iops__AddHillisSteele__Theorems______macroRules__Zhc__Iops__HillisSteele__tacticProjects__step__1___closed__23_value = {.m_header = {.m_rc = 0, .m_cs_sz = sizeof(lean_ctor_object) + sizeof(void*)*2 + 8, .m_other = 2, .m_tag = 1}, .m_objs = {((lean_object*)&lp_zhc__lean_Zhc_Iops_HillisSteele___aux__Zhc__Iops__AddHillisSteele__Theorems______macroRules__Zhc__Iops__HillisSteele__tacticProjects__step__1___closed__23_value_aux_0),((lean_object*)&lp_zhc__lean_Zhc_Iops_HillisSteele___aux__Zhc__Iops__AddHillisSteele__Theorems______macroRules__Zhc__Iops__HillisSteele__tacticProjects__step__1___closed__22_value),LEAN_SCALAR_PTR_LITERAL(105, 200, 82, 235, 92, 136, 60, 50)}};
static const lean_object* lp_zhc__lean_Zhc_Iops_HillisSteele___aux__Zhc__Iops__AddHillisSteele__Theorems______macroRules__Zhc__Iops__HillisSteele__tacticProjects__step__1___closed__23 = (const lean_object*)&lp_zhc__lean_Zhc_Iops_HillisSteele___aux__Zhc__Iops__AddHillisSteele__Theorems______macroRules__Zhc__Iops__HillisSteele__tacticProjects__step__1___closed__23_value;
static const lean_string_object lp_zhc__lean_Zhc_Iops_HillisSteele___aux__Zhc__Iops__AddHillisSteele__Theorems______macroRules__Zhc__Iops__HillisSteele__tacticProjects__step__1___closed__24_value = {.m_header = {.m_rc = 0, .m_cs_sz = 0, .m_other = 0, .m_tag = 249}, .m_size = 13, .m_capacity = 13, .m_length = 12, .m_data = "BlockAlgebra"};
static const lean_object* lp_zhc__lean_Zhc_Iops_HillisSteele___aux__Zhc__Iops__AddHillisSteele__Theorems______macroRules__Zhc__Iops__HillisSteele__tacticProjects__step__1___closed__24 = (const lean_object*)&lp_zhc__lean_Zhc_Iops_HillisSteele___aux__Zhc__Iops__AddHillisSteele__Theorems______macroRules__Zhc__Iops__HillisSteele__tacticProjects__step__1___closed__24_value;
static const lean_string_object lp_zhc__lean_Zhc_Iops_HillisSteele___aux__Zhc__Iops__AddHillisSteele__Theorems______macroRules__Zhc__Iops__HillisSteele__tacticProjects__step__1___closed__25_value = {.m_header = {.m_rc = 0, .m_cs_sz = 0, .m_other = 0, .m_tag = 249}, .m_size = 7, .m_capacity = 7, .m_length = 6, .m_data = "Tracer"};
static const lean_object* lp_zhc__lean_Zhc_Iops_HillisSteele___aux__Zhc__Iops__AddHillisSteele__Theorems______macroRules__Zhc__Iops__HillisSteele__tacticProjects__step__1___closed__25 = (const lean_object*)&lp_zhc__lean_Zhc_Iops_HillisSteele___aux__Zhc__Iops__AddHillisSteele__Theorems______macroRules__Zhc__Iops__HillisSteele__tacticProjects__step__1___closed__25_value;
static const lean_ctor_object lp_zhc__lean_Zhc_Iops_HillisSteele___aux__Zhc__Iops__AddHillisSteele__Theorems______macroRules__Zhc__Iops__HillisSteele__tacticProjects__step__1___closed__26_value_aux_0 = {.m_header = {.m_rc = 0, .m_cs_sz = sizeof(lean_ctor_object) + sizeof(void*)*2 + 8, .m_other = 2, .m_tag = 1}, .m_objs = {((lean_object*)(((size_t)(0) << 1) | 1)),((lean_object*)&lp_zhc__lean_Zhc_Iops_HillisSteele_tacticProjects__step___closed__0_value),LEAN_SCALAR_PTR_LITERAL(204, 210, 61, 107, 112, 216, 72, 96)}};
static const lean_ctor_object lp_zhc__lean_Zhc_Iops_HillisSteele___aux__Zhc__Iops__AddHillisSteele__Theorems______macroRules__Zhc__Iops__HillisSteele__tacticProjects__step__1___closed__26_value_aux_1 = {.m_header = {.m_rc = 0, .m_cs_sz = sizeof(lean_ctor_object) + sizeof(void*)*2 + 8, .m_other = 2, .m_tag = 1}, .m_objs = {((lean_object*)&lp_zhc__lean_Zhc_Iops_HillisSteele___aux__Zhc__Iops__AddHillisSteele__Theorems______macroRules__Zhc__Iops__HillisSteele__tacticProjects__step__1___closed__26_value_aux_0),((lean_object*)&lp_zhc__lean_Zhc_Iops_HillisSteele___aux__Zhc__Iops__AddHillisSteele__Theorems______macroRules__Zhc__Iops__HillisSteele__tacticProjects__step__1___closed__24_value),LEAN_SCALAR_PTR_LITERAL(191, 108, 44, 3, 254, 30, 252, 26)}};
static const lean_ctor_object lp_zhc__lean_Zhc_Iops_HillisSteele___aux__Zhc__Iops__AddHillisSteele__Theorems______macroRules__Zhc__Iops__HillisSteele__tacticProjects__step__1___closed__26_value_aux_2 = {.m_header = {.m_rc = 0, .m_cs_sz = sizeof(lean_ctor_object) + sizeof(void*)*2 + 8, .m_other = 2, .m_tag = 1}, .m_objs = {((lean_object*)&lp_zhc__lean_Zhc_Iops_HillisSteele___aux__Zhc__Iops__AddHillisSteele__Theorems______macroRules__Zhc__Iops__HillisSteele__tacticProjects__step__1___closed__26_value_aux_1),((lean_object*)&lp_zhc__lean_Zhc_Iops_HillisSteele___aux__Zhc__Iops__AddHillisSteele__Theorems______macroRules__Zhc__Iops__HillisSteele__tacticProjects__step__1___closed__25_value),LEAN_SCALAR_PTR_LITERAL(29, 36, 229, 118, 140, 245, 224, 135)}};
static const lean_ctor_object lp_zhc__lean_Zhc_Iops_HillisSteele___aux__Zhc__Iops__AddHillisSteele__Theorems______macroRules__Zhc__Iops__HillisSteele__tacticProjects__step__1___closed__26_value_aux_3 = {.m_header = {.m_rc = 0, .m_cs_sz = sizeof(lean_ctor_object) + sizeof(void*)*2 + 8, .m_other = 2, .m_tag = 1}, .m_objs = {((lean_object*)&lp_zhc__lean_Zhc_Iops_HillisSteele___aux__Zhc__Iops__AddHillisSteele__Theorems______macroRules__Zhc__Iops__HillisSteele__tacticProjects__step__1___closed__26_value_aux_2),((lean_object*)&lp_zhc__lean_Zhc_Iops_HillisSteele___aux__Zhc__Iops__AddHillisSteele__Theorems______macroRules__Zhc__Iops__HillisSteele__tacticProjects__step__1___closed__21_value),LEAN_SCALAR_PTR_LITERAL(190, 103, 215, 136, 201, 42, 240, 117)}};
static const lean_ctor_object lp_zhc__lean_Zhc_Iops_HillisSteele___aux__Zhc__Iops__AddHillisSteele__Theorems______macroRules__Zhc__Iops__HillisSteele__tacticProjects__step__1___closed__26_value = {.m_header = {.m_rc = 0, .m_cs_sz = sizeof(lean_ctor_object) + sizeof(void*)*2 + 8, .m_other = 2, .m_tag = 1}, .m_objs = {((lean_object*)&lp_zhc__lean_Zhc_Iops_HillisSteele___aux__Zhc__Iops__AddHillisSteele__Theorems______macroRules__Zhc__Iops__HillisSteele__tacticProjects__step__1___closed__26_value_aux_3),((lean_object*)&lp_zhc__lean_Zhc_Iops_HillisSteele___aux__Zhc__Iops__AddHillisSteele__Theorems______macroRules__Zhc__Iops__HillisSteele__tacticProjects__step__1___closed__22_value),LEAN_SCALAR_PTR_LITERAL(247, 251, 179, 251, 6, 120, 98, 128)}};
static const lean_object* lp_zhc__lean_Zhc_Iops_HillisSteele___aux__Zhc__Iops__AddHillisSteele__Theorems______macroRules__Zhc__Iops__HillisSteele__tacticProjects__step__1___closed__26 = (const lean_object*)&lp_zhc__lean_Zhc_Iops_HillisSteele___aux__Zhc__Iops__AddHillisSteele__Theorems______macroRules__Zhc__Iops__HillisSteele__tacticProjects__step__1___closed__26_value;
static const lean_ctor_object lp_zhc__lean_Zhc_Iops_HillisSteele___aux__Zhc__Iops__AddHillisSteele__Theorems______macroRules__Zhc__Iops__HillisSteele__tacticProjects__step__1___closed__27_value = {.m_header = {.m_rc = 0, .m_cs_sz = sizeof(lean_ctor_object) + sizeof(void*)*2 + 0, .m_other = 2, .m_tag = 1}, .m_objs = {((lean_object*)&lp_zhc__lean_Zhc_Iops_HillisSteele___aux__Zhc__Iops__AddHillisSteele__Theorems______macroRules__Zhc__Iops__HillisSteele__tacticProjects__step__1___closed__26_value),((lean_object*)(((size_t)(0) << 1) | 1))}};
static const lean_object* lp_zhc__lean_Zhc_Iops_HillisSteele___aux__Zhc__Iops__AddHillisSteele__Theorems______macroRules__Zhc__Iops__HillisSteele__tacticProjects__step__1___closed__27 = (const lean_object*)&lp_zhc__lean_Zhc_Iops_HillisSteele___aux__Zhc__Iops__AddHillisSteele__Theorems______macroRules__Zhc__Iops__HillisSteele__tacticProjects__step__1___closed__27_value;
static const lean_ctor_object lp_zhc__lean_Zhc_Iops_HillisSteele___aux__Zhc__Iops__AddHillisSteele__Theorems______macroRules__Zhc__Iops__HillisSteele__tacticProjects__step__1___closed__28_value = {.m_header = {.m_rc = 0, .m_cs_sz = sizeof(lean_ctor_object) + sizeof(void*)*2 + 0, .m_other = 2, .m_tag = 1}, .m_objs = {((lean_object*)&lp_zhc__lean_Zhc_Iops_HillisSteele___aux__Zhc__Iops__AddHillisSteele__Theorems______macroRules__Zhc__Iops__HillisSteele__tacticProjects__step__1___closed__27_value),((lean_object*)(((size_t)(0) << 1) | 1))}};
static const lean_object* lp_zhc__lean_Zhc_Iops_HillisSteele___aux__Zhc__Iops__AddHillisSteele__Theorems______macroRules__Zhc__Iops__HillisSteele__tacticProjects__step__1___closed__28 = (const lean_object*)&lp_zhc__lean_Zhc_Iops_HillisSteele___aux__Zhc__Iops__AddHillisSteele__Theorems______macroRules__Zhc__Iops__HillisSteele__tacticProjects__step__1___closed__28_value;
static const lean_string_object lp_zhc__lean_Zhc_Iops_HillisSteele___aux__Zhc__Iops__AddHillisSteele__Theorems______macroRules__Zhc__Iops__HillisSteele__tacticProjects__step__1___closed__29_value = {.m_header = {.m_rc = 0, .m_cs_sz = 0, .m_other = 0, .m_tag = 249}, .m_size = 5, .m_capacity = 5, .m_length = 4, .m_data = "hole"};
static const lean_object* lp_zhc__lean_Zhc_Iops_HillisSteele___aux__Zhc__Iops__AddHillisSteele__Theorems______macroRules__Zhc__Iops__HillisSteele__tacticProjects__step__1___closed__29 = (const lean_object*)&lp_zhc__lean_Zhc_Iops_HillisSteele___aux__Zhc__Iops__AddHillisSteele__Theorems______macroRules__Zhc__Iops__HillisSteele__tacticProjects__step__1___closed__29_value;
static const lean_ctor_object lp_zhc__lean_Zhc_Iops_HillisSteele___aux__Zhc__Iops__AddHillisSteele__Theorems______macroRules__Zhc__Iops__HillisSteele__tacticProjects__step__1___closed__30_value_aux_0 = {.m_header = {.m_rc = 0, .m_cs_sz = sizeof(lean_ctor_object) + sizeof(void*)*2 + 8, .m_other = 2, .m_tag = 1}, .m_objs = {((lean_object*)(((size_t)(0) << 1) | 1)),((lean_object*)&lp_zhc__lean_Zhc_Iops_HillisSteele___aux__Zhc__Iops__AddHillisSteele__Theorems______macroRules__Zhc__Iops__HillisSteele__tacticProjects__step__1___closed__0_value),LEAN_SCALAR_PTR_LITERAL(70, 193, 83, 126, 233, 67, 208, 165)}};
static const lean_ctor_object lp_zhc__lean_Zhc_Iops_HillisSteele___aux__Zhc__Iops__AddHillisSteele__Theorems______macroRules__Zhc__Iops__HillisSteele__tacticProjects__step__1___closed__30_value_aux_1 = {.m_header = {.m_rc = 0, .m_cs_sz = sizeof(lean_ctor_object) + sizeof(void*)*2 + 8, .m_other = 2, .m_tag = 1}, .m_objs = {((lean_object*)&lp_zhc__lean_Zhc_Iops_HillisSteele___aux__Zhc__Iops__AddHillisSteele__Theorems______macroRules__Zhc__Iops__HillisSteele__tacticProjects__step__1___closed__30_value_aux_0),((lean_object*)&lp_zhc__lean_Zhc_Iops_HillisSteele___aux__Zhc__Iops__AddHillisSteele__Theorems______macroRules__Zhc__Iops__HillisSteele__tacticProjects__step__1___closed__1_value),LEAN_SCALAR_PTR_LITERAL(103, 136, 125, 166, 167, 98, 71, 111)}};
static const lean_ctor_object lp_zhc__lean_Zhc_Iops_HillisSteele___aux__Zhc__Iops__AddHillisSteele__Theorems______macroRules__Zhc__Iops__HillisSteele__tacticProjects__step__1___closed__30_value_aux_2 = {.m_header = {.m_rc = 0, .m_cs_sz = sizeof(lean_ctor_object) + sizeof(void*)*2 + 8, .m_other = 2, .m_tag = 1}, .m_objs = {((lean_object*)&lp_zhc__lean_Zhc_Iops_HillisSteele___aux__Zhc__Iops__AddHillisSteele__Theorems______macroRules__Zhc__Iops__HillisSteele__tacticProjects__step__1___closed__30_value_aux_1),((lean_object*)&lp_zhc__lean_Zhc_Iops_HillisSteele___aux__Zhc__Iops__AddHillisSteele__Theorems______macroRules__Zhc__Iops__HillisSteele__tacticProjects__step__1___closed__16_value),LEAN_SCALAR_PTR_LITERAL(75, 170, 162, 138, 136, 204, 251, 229)}};
static const lean_ctor_object lp_zhc__lean_Zhc_Iops_HillisSteele___aux__Zhc__Iops__AddHillisSteele__Theorems______macroRules__Zhc__Iops__HillisSteele__tacticProjects__step__1___closed__30_value = {.m_header = {.m_rc = 0, .m_cs_sz = sizeof(lean_ctor_object) + sizeof(void*)*2 + 8, .m_other = 2, .m_tag = 1}, .m_objs = {((lean_object*)&lp_zhc__lean_Zhc_Iops_HillisSteele___aux__Zhc__Iops__AddHillisSteele__Theorems______macroRules__Zhc__Iops__HillisSteele__tacticProjects__step__1___closed__30_value_aux_2),((lean_object*)&lp_zhc__lean_Zhc_Iops_HillisSteele___aux__Zhc__Iops__AddHillisSteele__Theorems______macroRules__Zhc__Iops__HillisSteele__tacticProjects__step__1___closed__29_value),LEAN_SCALAR_PTR_LITERAL(135, 134, 219, 115, 97, 130, 74, 55)}};
static const lean_object* lp_zhc__lean_Zhc_Iops_HillisSteele___aux__Zhc__Iops__AddHillisSteele__Theorems______macroRules__Zhc__Iops__HillisSteele__tacticProjects__step__1___closed__30 = (const lean_object*)&lp_zhc__lean_Zhc_Iops_HillisSteele___aux__Zhc__Iops__AddHillisSteele__Theorems______macroRules__Zhc__Iops__HillisSteele__tacticProjects__step__1___closed__30_value;
static const lean_string_object lp_zhc__lean_Zhc_Iops_HillisSteele___aux__Zhc__Iops__AddHillisSteele__Theorems______macroRules__Zhc__Iops__HillisSteele__tacticProjects__step__1___closed__31_value = {.m_header = {.m_rc = 0, .m_cs_sz = 0, .m_other = 0, .m_tag = 249}, .m_size = 2, .m_capacity = 2, .m_length = 1, .m_data = "_"};
static const lean_object* lp_zhc__lean_Zhc_Iops_HillisSteele___aux__Zhc__Iops__AddHillisSteele__Theorems______macroRules__Zhc__Iops__HillisSteele__tacticProjects__step__1___closed__31 = (const lean_object*)&lp_zhc__lean_Zhc_Iops_HillisSteele___aux__Zhc__Iops__AddHillisSteele__Theorems______macroRules__Zhc__Iops__HillisSteele__tacticProjects__step__1___closed__31_value;
static const lean_string_object lp_zhc__lean_Zhc_Iops_HillisSteele___aux__Zhc__Iops__AddHillisSteele__Theorems______macroRules__Zhc__Iops__HillisSteele__tacticProjects__step__1___closed__32_value = {.m_header = {.m_rc = 0, .m_cs_sz = 0, .m_other = 0, .m_tag = 249}, .m_size = 18, .m_capacity = 18, .m_length = 17, .m_data = "tacticProjects_op"};
static const lean_object* lp_zhc__lean_Zhc_Iops_HillisSteele___aux__Zhc__Iops__AddHillisSteele__Theorems______macroRules__Zhc__Iops__HillisSteele__tacticProjects__step__1___closed__32 = (const lean_object*)&lp_zhc__lean_Zhc_Iops_HillisSteele___aux__Zhc__Iops__AddHillisSteele__Theorems______macroRules__Zhc__Iops__HillisSteele__tacticProjects__step__1___closed__32_value;
static const lean_ctor_object lp_zhc__lean_Zhc_Iops_HillisSteele___aux__Zhc__Iops__AddHillisSteele__Theorems______macroRules__Zhc__Iops__HillisSteele__tacticProjects__step__1___closed__33_value_aux_0 = {.m_header = {.m_rc = 0, .m_cs_sz = sizeof(lean_ctor_object) + sizeof(void*)*2 + 8, .m_other = 2, .m_tag = 1}, .m_objs = {((lean_object*)(((size_t)(0) << 1) | 1)),((lean_object*)&lp_zhc__lean_Zhc_Iops_HillisSteele_tacticProjects__step___closed__0_value),LEAN_SCALAR_PTR_LITERAL(204, 210, 61, 107, 112, 216, 72, 96)}};
static const lean_ctor_object lp_zhc__lean_Zhc_Iops_HillisSteele___aux__Zhc__Iops__AddHillisSteele__Theorems______macroRules__Zhc__Iops__HillisSteele__tacticProjects__step__1___closed__33_value_aux_1 = {.m_header = {.m_rc = 0, .m_cs_sz = sizeof(lean_ctor_object) + sizeof(void*)*2 + 8, .m_other = 2, .m_tag = 1}, .m_objs = {((lean_object*)&lp_zhc__lean_Zhc_Iops_HillisSteele___aux__Zhc__Iops__AddHillisSteele__Theorems______macroRules__Zhc__Iops__HillisSteele__tacticProjects__step__1___closed__33_value_aux_0),((lean_object*)&lp_zhc__lean_Zhc_Iops_HillisSteele___aux__Zhc__Iops__AddHillisSteele__Theorems______macroRules__Zhc__Iops__HillisSteele__tacticProjects__step__1___closed__24_value),LEAN_SCALAR_PTR_LITERAL(191, 108, 44, 3, 254, 30, 252, 26)}};
static const lean_ctor_object lp_zhc__lean_Zhc_Iops_HillisSteele___aux__Zhc__Iops__AddHillisSteele__Theorems______macroRules__Zhc__Iops__HillisSteele__tacticProjects__step__1___closed__33_value_aux_2 = {.m_header = {.m_rc = 0, .m_cs_sz = sizeof(lean_ctor_object) + sizeof(void*)*2 + 8, .m_other = 2, .m_tag = 1}, .m_objs = {((lean_object*)&lp_zhc__lean_Zhc_Iops_HillisSteele___aux__Zhc__Iops__AddHillisSteele__Theorems______macroRules__Zhc__Iops__HillisSteele__tacticProjects__step__1___closed__33_value_aux_1),((lean_object*)&lp_zhc__lean_Zhc_Iops_HillisSteele___aux__Zhc__Iops__AddHillisSteele__Theorems______macroRules__Zhc__Iops__HillisSteele__tacticProjects__step__1___closed__25_value),LEAN_SCALAR_PTR_LITERAL(29, 36, 229, 118, 140, 245, 224, 135)}};
static const lean_ctor_object lp_zhc__lean_Zhc_Iops_HillisSteele___aux__Zhc__Iops__AddHillisSteele__Theorems______macroRules__Zhc__Iops__HillisSteele__tacticProjects__step__1___closed__33_value = {.m_header = {.m_rc = 0, .m_cs_sz = sizeof(lean_ctor_object) + sizeof(void*)*2 + 8, .m_other = 2, .m_tag = 1}, .m_objs = {((lean_object*)&lp_zhc__lean_Zhc_Iops_HillisSteele___aux__Zhc__Iops__AddHillisSteele__Theorems______macroRules__Zhc__Iops__HillisSteele__tacticProjects__step__1___closed__33_value_aux_2),((lean_object*)&lp_zhc__lean_Zhc_Iops_HillisSteele___aux__Zhc__Iops__AddHillisSteele__Theorems______macroRules__Zhc__Iops__HillisSteele__tacticProjects__step__1___closed__32_value),LEAN_SCALAR_PTR_LITERAL(103, 65, 204, 67, 207, 238, 44, 240)}};
static const lean_object* lp_zhc__lean_Zhc_Iops_HillisSteele___aux__Zhc__Iops__AddHillisSteele__Theorems______macroRules__Zhc__Iops__HillisSteele__tacticProjects__step__1___closed__33 = (const lean_object*)&lp_zhc__lean_Zhc_Iops_HillisSteele___aux__Zhc__Iops__AddHillisSteele__Theorems______macroRules__Zhc__Iops__HillisSteele__tacticProjects__step__1___closed__33_value;
static const lean_string_object lp_zhc__lean_Zhc_Iops_HillisSteele___aux__Zhc__Iops__AddHillisSteele__Theorems______macroRules__Zhc__Iops__HillisSteele__tacticProjects__step__1___closed__34_value = {.m_header = {.m_rc = 0, .m_cs_sz = 0, .m_other = 0, .m_tag = 249}, .m_size = 12, .m_capacity = 12, .m_length = 11, .m_data = "projects_op"};
static const lean_object* lp_zhc__lean_Zhc_Iops_HillisSteele___aux__Zhc__Iops__AddHillisSteele__Theorems______macroRules__Zhc__Iops__HillisSteele__tacticProjects__step__1___closed__34 = (const lean_object*)&lp_zhc__lean_Zhc_Iops_HillisSteele___aux__Zhc__Iops__AddHillisSteele__Theorems______macroRules__Zhc__Iops__HillisSteele__tacticProjects__step__1___closed__34_value;
static const lean_string_object lp_zhc__lean_Zhc_Iops_HillisSteele___aux__Zhc__Iops__AddHillisSteele__Theorems______macroRules__Zhc__Iops__HillisSteele__tacticProjects__step__1___closed__35_value = {.m_header = {.m_rc = 0, .m_cs_sz = 0, .m_other = 0, .m_tag = 249}, .m_size = 6, .m_capacity = 6, .m_length = 5, .m_data = "paren"};
static const lean_object* lp_zhc__lean_Zhc_Iops_HillisSteele___aux__Zhc__Iops__AddHillisSteele__Theorems______macroRules__Zhc__Iops__HillisSteele__tacticProjects__step__1___closed__35 = (const lean_object*)&lp_zhc__lean_Zhc_Iops_HillisSteele___aux__Zhc__Iops__AddHillisSteele__Theorems______macroRules__Zhc__Iops__HillisSteele__tacticProjects__step__1___closed__35_value;
static const lean_ctor_object lp_zhc__lean_Zhc_Iops_HillisSteele___aux__Zhc__Iops__AddHillisSteele__Theorems______macroRules__Zhc__Iops__HillisSteele__tacticProjects__step__1___closed__36_value_aux_0 = {.m_header = {.m_rc = 0, .m_cs_sz = sizeof(lean_ctor_object) + sizeof(void*)*2 + 8, .m_other = 2, .m_tag = 1}, .m_objs = {((lean_object*)(((size_t)(0) << 1) | 1)),((lean_object*)&lp_zhc__lean_Zhc_Iops_HillisSteele___aux__Zhc__Iops__AddHillisSteele__Theorems______macroRules__Zhc__Iops__HillisSteele__tacticProjects__step__1___closed__0_value),LEAN_SCALAR_PTR_LITERAL(70, 193, 83, 126, 233, 67, 208, 165)}};
static const lean_ctor_object lp_zhc__lean_Zhc_Iops_HillisSteele___aux__Zhc__Iops__AddHillisSteele__Theorems______macroRules__Zhc__Iops__HillisSteele__tacticProjects__step__1___closed__36_value_aux_1 = {.m_header = {.m_rc = 0, .m_cs_sz = sizeof(lean_ctor_object) + sizeof(void*)*2 + 8, .m_other = 2, .m_tag = 1}, .m_objs = {((lean_object*)&lp_zhc__lean_Zhc_Iops_HillisSteele___aux__Zhc__Iops__AddHillisSteele__Theorems______macroRules__Zhc__Iops__HillisSteele__tacticProjects__step__1___closed__36_value_aux_0),((lean_object*)&lp_zhc__lean_Zhc_Iops_HillisSteele___aux__Zhc__Iops__AddHillisSteele__Theorems______macroRules__Zhc__Iops__HillisSteele__tacticProjects__step__1___closed__1_value),LEAN_SCALAR_PTR_LITERAL(103, 136, 125, 166, 167, 98, 71, 111)}};
static const lean_ctor_object lp_zhc__lean_Zhc_Iops_HillisSteele___aux__Zhc__Iops__AddHillisSteele__Theorems______macroRules__Zhc__Iops__HillisSteele__tacticProjects__step__1___closed__36_value_aux_2 = {.m_header = {.m_rc = 0, .m_cs_sz = sizeof(lean_ctor_object) + sizeof(void*)*2 + 8, .m_other = 2, .m_tag = 1}, .m_objs = {((lean_object*)&lp_zhc__lean_Zhc_Iops_HillisSteele___aux__Zhc__Iops__AddHillisSteele__Theorems______macroRules__Zhc__Iops__HillisSteele__tacticProjects__step__1___closed__36_value_aux_1),((lean_object*)&lp_zhc__lean_Zhc_Iops_HillisSteele___aux__Zhc__Iops__AddHillisSteele__Theorems______macroRules__Zhc__Iops__HillisSteele__tacticProjects__step__1___closed__2_value),LEAN_SCALAR_PTR_LITERAL(166, 58, 35, 182, 187, 130, 147, 254)}};
static const lean_ctor_object lp_zhc__lean_Zhc_Iops_HillisSteele___aux__Zhc__Iops__AddHillisSteele__Theorems______macroRules__Zhc__Iops__HillisSteele__tacticProjects__step__1___closed__36_value = {.m_header = {.m_rc = 0, .m_cs_sz = sizeof(lean_ctor_object) + sizeof(void*)*2 + 8, .m_other = 2, .m_tag = 1}, .m_objs = {((lean_object*)&lp_zhc__lean_Zhc_Iops_HillisSteele___aux__Zhc__Iops__AddHillisSteele__Theorems______macroRules__Zhc__Iops__HillisSteele__tacticProjects__step__1___closed__36_value_aux_2),((lean_object*)&lp_zhc__lean_Zhc_Iops_HillisSteele___aux__Zhc__Iops__AddHillisSteele__Theorems______macroRules__Zhc__Iops__HillisSteele__tacticProjects__step__1___closed__35_value),LEAN_SCALAR_PTR_LITERAL(117, 253, 122, 28, 77, 248, 149, 120)}};
static const lean_object* lp_zhc__lean_Zhc_Iops_HillisSteele___aux__Zhc__Iops__AddHillisSteele__Theorems______macroRules__Zhc__Iops__HillisSteele__tacticProjects__step__1___closed__36 = (const lean_object*)&lp_zhc__lean_Zhc_Iops_HillisSteele___aux__Zhc__Iops__AddHillisSteele__Theorems______macroRules__Zhc__Iops__HillisSteele__tacticProjects__step__1___closed__36_value;
static const lean_string_object lp_zhc__lean_Zhc_Iops_HillisSteele___aux__Zhc__Iops__AddHillisSteele__Theorems______macroRules__Zhc__Iops__HillisSteele__tacticProjects__step__1___closed__37_value = {.m_header = {.m_rc = 0, .m_cs_sz = 0, .m_other = 0, .m_tag = 249}, .m_size = 2, .m_capacity = 2, .m_length = 1, .m_data = "("};
static const lean_object* lp_zhc__lean_Zhc_Iops_HillisSteele___aux__Zhc__Iops__AddHillisSteele__Theorems______macroRules__Zhc__Iops__HillisSteele__tacticProjects__step__1___closed__37 = (const lean_object*)&lp_zhc__lean_Zhc_Iops_HillisSteele___aux__Zhc__Iops__AddHillisSteele__Theorems______macroRules__Zhc__Iops__HillisSteele__tacticProjects__step__1___closed__37_value;
static const lean_string_object lp_zhc__lean_Zhc_Iops_HillisSteele___aux__Zhc__Iops__AddHillisSteele__Theorems______macroRules__Zhc__Iops__HillisSteele__tacticProjects__step__1___closed__38_value = {.m_header = {.m_rc = 0, .m_cs_sz = 0, .m_other = 0, .m_tag = 249}, .m_size = 7, .m_capacity = 7, .m_length = 6, .m_data = "refine"};
static const lean_object* lp_zhc__lean_Zhc_Iops_HillisSteele___aux__Zhc__Iops__AddHillisSteele__Theorems______macroRules__Zhc__Iops__HillisSteele__tacticProjects__step__1___closed__38 = (const lean_object*)&lp_zhc__lean_Zhc_Iops_HillisSteele___aux__Zhc__Iops__AddHillisSteele__Theorems______macroRules__Zhc__Iops__HillisSteele__tacticProjects__step__1___closed__38_value;
static const lean_ctor_object lp_zhc__lean_Zhc_Iops_HillisSteele___aux__Zhc__Iops__AddHillisSteele__Theorems______macroRules__Zhc__Iops__HillisSteele__tacticProjects__step__1___closed__39_value_aux_0 = {.m_header = {.m_rc = 0, .m_cs_sz = sizeof(lean_ctor_object) + sizeof(void*)*2 + 8, .m_other = 2, .m_tag = 1}, .m_objs = {((lean_object*)(((size_t)(0) << 1) | 1)),((lean_object*)&lp_zhc__lean_Zhc_Iops_HillisSteele___aux__Zhc__Iops__AddHillisSteele__Theorems______macroRules__Zhc__Iops__HillisSteele__tacticProjects__step__1___closed__0_value),LEAN_SCALAR_PTR_LITERAL(70, 193, 83, 126, 233, 67, 208, 165)}};
static const lean_ctor_object lp_zhc__lean_Zhc_Iops_HillisSteele___aux__Zhc__Iops__AddHillisSteele__Theorems______macroRules__Zhc__Iops__HillisSteele__tacticProjects__step__1___closed__39_value_aux_1 = {.m_header = {.m_rc = 0, .m_cs_sz = sizeof(lean_ctor_object) + sizeof(void*)*2 + 8, .m_other = 2, .m_tag = 1}, .m_objs = {((lean_object*)&lp_zhc__lean_Zhc_Iops_HillisSteele___aux__Zhc__Iops__AddHillisSteele__Theorems______macroRules__Zhc__Iops__HillisSteele__tacticProjects__step__1___closed__39_value_aux_0),((lean_object*)&lp_zhc__lean_Zhc_Iops_HillisSteele___aux__Zhc__Iops__AddHillisSteele__Theorems______macroRules__Zhc__Iops__HillisSteele__tacticProjects__step__1___closed__1_value),LEAN_SCALAR_PTR_LITERAL(103, 136, 125, 166, 167, 98, 71, 111)}};
static const lean_ctor_object lp_zhc__lean_Zhc_Iops_HillisSteele___aux__Zhc__Iops__AddHillisSteele__Theorems______macroRules__Zhc__Iops__HillisSteele__tacticProjects__step__1___closed__39_value_aux_2 = {.m_header = {.m_rc = 0, .m_cs_sz = sizeof(lean_ctor_object) + sizeof(void*)*2 + 8, .m_other = 2, .m_tag = 1}, .m_objs = {((lean_object*)&lp_zhc__lean_Zhc_Iops_HillisSteele___aux__Zhc__Iops__AddHillisSteele__Theorems______macroRules__Zhc__Iops__HillisSteele__tacticProjects__step__1___closed__39_value_aux_1),((lean_object*)&lp_zhc__lean_Zhc_Iops_HillisSteele___aux__Zhc__Iops__AddHillisSteele__Theorems______macroRules__Zhc__Iops__HillisSteele__tacticProjects__step__1___closed__2_value),LEAN_SCALAR_PTR_LITERAL(166, 58, 35, 182, 187, 130, 147, 254)}};
static const lean_ctor_object lp_zhc__lean_Zhc_Iops_HillisSteele___aux__Zhc__Iops__AddHillisSteele__Theorems______macroRules__Zhc__Iops__HillisSteele__tacticProjects__step__1___closed__39_value = {.m_header = {.m_rc = 0, .m_cs_sz = sizeof(lean_ctor_object) + sizeof(void*)*2 + 8, .m_other = 2, .m_tag = 1}, .m_objs = {((lean_object*)&lp_zhc__lean_Zhc_Iops_HillisSteele___aux__Zhc__Iops__AddHillisSteele__Theorems______macroRules__Zhc__Iops__HillisSteele__tacticProjects__step__1___closed__39_value_aux_2),((lean_object*)&lp_zhc__lean_Zhc_Iops_HillisSteele___aux__Zhc__Iops__AddHillisSteele__Theorems______macroRules__Zhc__Iops__HillisSteele__tacticProjects__step__1___closed__38_value),LEAN_SCALAR_PTR_LITERAL(49, 130, 130, 160, 131, 48, 178, 245)}};
static const lean_object* lp_zhc__lean_Zhc_Iops_HillisSteele___aux__Zhc__Iops__AddHillisSteele__Theorems______macroRules__Zhc__Iops__HillisSteele__tacticProjects__step__1___closed__39 = (const lean_object*)&lp_zhc__lean_Zhc_Iops_HillisSteele___aux__Zhc__Iops__AddHillisSteele__Theorems______macroRules__Zhc__Iops__HillisSteele__tacticProjects__step__1___closed__39_value;
static const lean_string_object lp_zhc__lean_Zhc_Iops_HillisSteele___aux__Zhc__Iops__AddHillisSteele__Theorems______macroRules__Zhc__Iops__HillisSteele__tacticProjects__step__1___closed__40_value = {.m_header = {.m_rc = 0, .m_cs_sz = 0, .m_other = 0, .m_tag = 249}, .m_size = 14, .m_capacity = 14, .m_length = 13, .m_data = "Projects.bind"};
static const lean_object* lp_zhc__lean_Zhc_Iops_HillisSteele___aux__Zhc__Iops__AddHillisSteele__Theorems______macroRules__Zhc__Iops__HillisSteele__tacticProjects__step__1___closed__40 = (const lean_object*)&lp_zhc__lean_Zhc_Iops_HillisSteele___aux__Zhc__Iops__AddHillisSteele__Theorems______macroRules__Zhc__Iops__HillisSteele__tacticProjects__step__1___closed__40_value;
static lean_once_cell_t lp_zhc__lean_Zhc_Iops_HillisSteele___aux__Zhc__Iops__AddHillisSteele__Theorems______macroRules__Zhc__Iops__HillisSteele__tacticProjects__step__1___closed__41_once = LEAN_ONCE_CELL_INITIALIZER;
static lean_object* lp_zhc__lean_Zhc_Iops_HillisSteele___aux__Zhc__Iops__AddHillisSteele__Theorems______macroRules__Zhc__Iops__HillisSteele__tacticProjects__step__1___closed__41;
static const lean_string_object lp_zhc__lean_Zhc_Iops_HillisSteele___aux__Zhc__Iops__AddHillisSteele__Theorems______macroRules__Zhc__Iops__HillisSteele__tacticProjects__step__1___closed__42_value = {.m_header = {.m_rc = 0, .m_cs_sz = 0, .m_other = 0, .m_tag = 249}, .m_size = 5, .m_capacity = 5, .m_length = 4, .m_data = "bind"};
static const lean_object* lp_zhc__lean_Zhc_Iops_HillisSteele___aux__Zhc__Iops__AddHillisSteele__Theorems______macroRules__Zhc__Iops__HillisSteele__tacticProjects__step__1___closed__42 = (const lean_object*)&lp_zhc__lean_Zhc_Iops_HillisSteele___aux__Zhc__Iops__AddHillisSteele__Theorems______macroRules__Zhc__Iops__HillisSteele__tacticProjects__step__1___closed__42_value;
static const lean_ctor_object lp_zhc__lean_Zhc_Iops_HillisSteele___aux__Zhc__Iops__AddHillisSteele__Theorems______macroRules__Zhc__Iops__HillisSteele__tacticProjects__step__1___closed__43_value_aux_0 = {.m_header = {.m_rc = 0, .m_cs_sz = sizeof(lean_ctor_object) + sizeof(void*)*2 + 8, .m_other = 2, .m_tag = 1}, .m_objs = {((lean_object*)(((size_t)(0) << 1) | 1)),((lean_object*)&lp_zhc__lean_Zhc_Iops_HillisSteele___aux__Zhc__Iops__AddHillisSteele__Theorems______macroRules__Zhc__Iops__HillisSteele__tacticProjects__step__1___closed__21_value),LEAN_SCALAR_PTR_LITERAL(96, 117, 157, 119, 138, 122, 129, 92)}};
static const lean_ctor_object lp_zhc__lean_Zhc_Iops_HillisSteele___aux__Zhc__Iops__AddHillisSteele__Theorems______macroRules__Zhc__Iops__HillisSteele__tacticProjects__step__1___closed__43_value = {.m_header = {.m_rc = 0, .m_cs_sz = sizeof(lean_ctor_object) + sizeof(void*)*2 + 8, .m_other = 2, .m_tag = 1}, .m_objs = {((lean_object*)&lp_zhc__lean_Zhc_Iops_HillisSteele___aux__Zhc__Iops__AddHillisSteele__Theorems______macroRules__Zhc__Iops__HillisSteele__tacticProjects__step__1___closed__43_value_aux_0),((lean_object*)&lp_zhc__lean_Zhc_Iops_HillisSteele___aux__Zhc__Iops__AddHillisSteele__Theorems______macroRules__Zhc__Iops__HillisSteele__tacticProjects__step__1___closed__42_value),LEAN_SCALAR_PTR_LITERAL(165, 208, 13, 2, 51, 204, 85, 196)}};
static const lean_object* lp_zhc__lean_Zhc_Iops_HillisSteele___aux__Zhc__Iops__AddHillisSteele__Theorems______macroRules__Zhc__Iops__HillisSteele__tacticProjects__step__1___closed__43 = (const lean_object*)&lp_zhc__lean_Zhc_Iops_HillisSteele___aux__Zhc__Iops__AddHillisSteele__Theorems______macroRules__Zhc__Iops__HillisSteele__tacticProjects__step__1___closed__43_value;
static const lean_ctor_object lp_zhc__lean_Zhc_Iops_HillisSteele___aux__Zhc__Iops__AddHillisSteele__Theorems______macroRules__Zhc__Iops__HillisSteele__tacticProjects__step__1___closed__44_value_aux_0 = {.m_header = {.m_rc = 0, .m_cs_sz = sizeof(lean_ctor_object) + sizeof(void*)*2 + 8, .m_other = 2, .m_tag = 1}, .m_objs = {((lean_object*)(((size_t)(0) << 1) | 1)),((lean_object*)&lp_zhc__lean_Zhc_Iops_HillisSteele_tacticProjects__step___closed__0_value),LEAN_SCALAR_PTR_LITERAL(204, 210, 61, 107, 112, 216, 72, 96)}};
static const lean_ctor_object lp_zhc__lean_Zhc_Iops_HillisSteele___aux__Zhc__Iops__AddHillisSteele__Theorems______macroRules__Zhc__Iops__HillisSteele__tacticProjects__step__1___closed__44_value_aux_1 = {.m_header = {.m_rc = 0, .m_cs_sz = sizeof(lean_ctor_object) + sizeof(void*)*2 + 8, .m_other = 2, .m_tag = 1}, .m_objs = {((lean_object*)&lp_zhc__lean_Zhc_Iops_HillisSteele___aux__Zhc__Iops__AddHillisSteele__Theorems______macroRules__Zhc__Iops__HillisSteele__tacticProjects__step__1___closed__44_value_aux_0),((lean_object*)&lp_zhc__lean_Zhc_Iops_HillisSteele___aux__Zhc__Iops__AddHillisSteele__Theorems______macroRules__Zhc__Iops__HillisSteele__tacticProjects__step__1___closed__24_value),LEAN_SCALAR_PTR_LITERAL(191, 108, 44, 3, 254, 30, 252, 26)}};
static const lean_ctor_object lp_zhc__lean_Zhc_Iops_HillisSteele___aux__Zhc__Iops__AddHillisSteele__Theorems______macroRules__Zhc__Iops__HillisSteele__tacticProjects__step__1___closed__44_value_aux_2 = {.m_header = {.m_rc = 0, .m_cs_sz = sizeof(lean_ctor_object) + sizeof(void*)*2 + 8, .m_other = 2, .m_tag = 1}, .m_objs = {((lean_object*)&lp_zhc__lean_Zhc_Iops_HillisSteele___aux__Zhc__Iops__AddHillisSteele__Theorems______macroRules__Zhc__Iops__HillisSteele__tacticProjects__step__1___closed__44_value_aux_1),((lean_object*)&lp_zhc__lean_Zhc_Iops_HillisSteele___aux__Zhc__Iops__AddHillisSteele__Theorems______macroRules__Zhc__Iops__HillisSteele__tacticProjects__step__1___closed__25_value),LEAN_SCALAR_PTR_LITERAL(29, 36, 229, 118, 140, 245, 224, 135)}};
static const lean_ctor_object lp_zhc__lean_Zhc_Iops_HillisSteele___aux__Zhc__Iops__AddHillisSteele__Theorems______macroRules__Zhc__Iops__HillisSteele__tacticProjects__step__1___closed__44_value_aux_3 = {.m_header = {.m_rc = 0, .m_cs_sz = sizeof(lean_ctor_object) + sizeof(void*)*2 + 8, .m_other = 2, .m_tag = 1}, .m_objs = {((lean_object*)&lp_zhc__lean_Zhc_Iops_HillisSteele___aux__Zhc__Iops__AddHillisSteele__Theorems______macroRules__Zhc__Iops__HillisSteele__tacticProjects__step__1___closed__44_value_aux_2),((lean_object*)&lp_zhc__lean_Zhc_Iops_HillisSteele___aux__Zhc__Iops__AddHillisSteele__Theorems______macroRules__Zhc__Iops__HillisSteele__tacticProjects__step__1___closed__21_value),LEAN_SCALAR_PTR_LITERAL(190, 103, 215, 136, 201, 42, 240, 117)}};
static const lean_ctor_object lp_zhc__lean_Zhc_Iops_HillisSteele___aux__Zhc__Iops__AddHillisSteele__Theorems______macroRules__Zhc__Iops__HillisSteele__tacticProjects__step__1___closed__44_value = {.m_header = {.m_rc = 0, .m_cs_sz = sizeof(lean_ctor_object) + sizeof(void*)*2 + 8, .m_other = 2, .m_tag = 1}, .m_objs = {((lean_object*)&lp_zhc__lean_Zhc_Iops_HillisSteele___aux__Zhc__Iops__AddHillisSteele__Theorems______macroRules__Zhc__Iops__HillisSteele__tacticProjects__step__1___closed__44_value_aux_3),((lean_object*)&lp_zhc__lean_Zhc_Iops_HillisSteele___aux__Zhc__Iops__AddHillisSteele__Theorems______macroRules__Zhc__Iops__HillisSteele__tacticProjects__step__1___closed__42_value),LEAN_SCALAR_PTR_LITERAL(187, 243, 174, 169, 236, 115, 91, 59)}};
static const lean_object* lp_zhc__lean_Zhc_Iops_HillisSteele___aux__Zhc__Iops__AddHillisSteele__Theorems______macroRules__Zhc__Iops__HillisSteele__tacticProjects__step__1___closed__44 = (const lean_object*)&lp_zhc__lean_Zhc_Iops_HillisSteele___aux__Zhc__Iops__AddHillisSteele__Theorems______macroRules__Zhc__Iops__HillisSteele__tacticProjects__step__1___closed__44_value;
static const lean_ctor_object lp_zhc__lean_Zhc_Iops_HillisSteele___aux__Zhc__Iops__AddHillisSteele__Theorems______macroRules__Zhc__Iops__HillisSteele__tacticProjects__step__1___closed__45_value = {.m_header = {.m_rc = 0, .m_cs_sz = sizeof(lean_ctor_object) + sizeof(void*)*2 + 0, .m_other = 2, .m_tag = 1}, .m_objs = {((lean_object*)&lp_zhc__lean_Zhc_Iops_HillisSteele___aux__Zhc__Iops__AddHillisSteele__Theorems______macroRules__Zhc__Iops__HillisSteele__tacticProjects__step__1___closed__44_value),((lean_object*)(((size_t)(0) << 1) | 1))}};
static const lean_object* lp_zhc__lean_Zhc_Iops_HillisSteele___aux__Zhc__Iops__AddHillisSteele__Theorems______macroRules__Zhc__Iops__HillisSteele__tacticProjects__step__1___closed__45 = (const lean_object*)&lp_zhc__lean_Zhc_Iops_HillisSteele___aux__Zhc__Iops__AddHillisSteele__Theorems______macroRules__Zhc__Iops__HillisSteele__tacticProjects__step__1___closed__45_value;
static const lean_ctor_object lp_zhc__lean_Zhc_Iops_HillisSteele___aux__Zhc__Iops__AddHillisSteele__Theorems______macroRules__Zhc__Iops__HillisSteele__tacticProjects__step__1___closed__46_value = {.m_header = {.m_rc = 0, .m_cs_sz = sizeof(lean_ctor_object) + sizeof(void*)*2 + 0, .m_other = 2, .m_tag = 1}, .m_objs = {((lean_object*)&lp_zhc__lean_Zhc_Iops_HillisSteele___aux__Zhc__Iops__AddHillisSteele__Theorems______macroRules__Zhc__Iops__HillisSteele__tacticProjects__step__1___closed__45_value),((lean_object*)(((size_t)(0) << 1) | 1))}};
static const lean_object* lp_zhc__lean_Zhc_Iops_HillisSteele___aux__Zhc__Iops__AddHillisSteele__Theorems______macroRules__Zhc__Iops__HillisSteele__tacticProjects__step__1___closed__46 = (const lean_object*)&lp_zhc__lean_Zhc_Iops_HillisSteele___aux__Zhc__Iops__AddHillisSteele__Theorems______macroRules__Zhc__Iops__HillisSteele__tacticProjects__step__1___closed__46_value;
static const lean_string_object lp_zhc__lean_Zhc_Iops_HillisSteele___aux__Zhc__Iops__AddHillisSteele__Theorems______macroRules__Zhc__Iops__HillisSteele__tacticProjects__step__1___closed__47_value = {.m_header = {.m_rc = 0, .m_cs_sz = 0, .m_other = 0, .m_tag = 249}, .m_size = 14, .m_capacity = 14, .m_length = 13, .m_data = "namedArgument"};
static const lean_object* lp_zhc__lean_Zhc_Iops_HillisSteele___aux__Zhc__Iops__AddHillisSteele__Theorems______macroRules__Zhc__Iops__HillisSteele__tacticProjects__step__1___closed__47 = (const lean_object*)&lp_zhc__lean_Zhc_Iops_HillisSteele___aux__Zhc__Iops__AddHillisSteele__Theorems______macroRules__Zhc__Iops__HillisSteele__tacticProjects__step__1___closed__47_value;
static const lean_ctor_object lp_zhc__lean_Zhc_Iops_HillisSteele___aux__Zhc__Iops__AddHillisSteele__Theorems______macroRules__Zhc__Iops__HillisSteele__tacticProjects__step__1___closed__48_value_aux_0 = {.m_header = {.m_rc = 0, .m_cs_sz = sizeof(lean_ctor_object) + sizeof(void*)*2 + 8, .m_other = 2, .m_tag = 1}, .m_objs = {((lean_object*)(((size_t)(0) << 1) | 1)),((lean_object*)&lp_zhc__lean_Zhc_Iops_HillisSteele___aux__Zhc__Iops__AddHillisSteele__Theorems______macroRules__Zhc__Iops__HillisSteele__tacticProjects__step__1___closed__0_value),LEAN_SCALAR_PTR_LITERAL(70, 193, 83, 126, 233, 67, 208, 165)}};
static const lean_ctor_object lp_zhc__lean_Zhc_Iops_HillisSteele___aux__Zhc__Iops__AddHillisSteele__Theorems______macroRules__Zhc__Iops__HillisSteele__tacticProjects__step__1___closed__48_value_aux_1 = {.m_header = {.m_rc = 0, .m_cs_sz = sizeof(lean_ctor_object) + sizeof(void*)*2 + 8, .m_other = 2, .m_tag = 1}, .m_objs = {((lean_object*)&lp_zhc__lean_Zhc_Iops_HillisSteele___aux__Zhc__Iops__AddHillisSteele__Theorems______macroRules__Zhc__Iops__HillisSteele__tacticProjects__step__1___closed__48_value_aux_0),((lean_object*)&lp_zhc__lean_Zhc_Iops_HillisSteele___aux__Zhc__Iops__AddHillisSteele__Theorems______macroRules__Zhc__Iops__HillisSteele__tacticProjects__step__1___closed__1_value),LEAN_SCALAR_PTR_LITERAL(103, 136, 125, 166, 167, 98, 71, 111)}};
static const lean_ctor_object lp_zhc__lean_Zhc_Iops_HillisSteele___aux__Zhc__Iops__AddHillisSteele__Theorems______macroRules__Zhc__Iops__HillisSteele__tacticProjects__step__1___closed__48_value_aux_2 = {.m_header = {.m_rc = 0, .m_cs_sz = sizeof(lean_ctor_object) + sizeof(void*)*2 + 8, .m_other = 2, .m_tag = 1}, .m_objs = {((lean_object*)&lp_zhc__lean_Zhc_Iops_HillisSteele___aux__Zhc__Iops__AddHillisSteele__Theorems______macroRules__Zhc__Iops__HillisSteele__tacticProjects__step__1___closed__48_value_aux_1),((lean_object*)&lp_zhc__lean_Zhc_Iops_HillisSteele___aux__Zhc__Iops__AddHillisSteele__Theorems______macroRules__Zhc__Iops__HillisSteele__tacticProjects__step__1___closed__16_value),LEAN_SCALAR_PTR_LITERAL(75, 170, 162, 138, 136, 204, 251, 229)}};
static const lean_ctor_object lp_zhc__lean_Zhc_Iops_HillisSteele___aux__Zhc__Iops__AddHillisSteele__Theorems______macroRules__Zhc__Iops__HillisSteele__tacticProjects__step__1___closed__48_value = {.m_header = {.m_rc = 0, .m_cs_sz = sizeof(lean_ctor_object) + sizeof(void*)*2 + 8, .m_other = 2, .m_tag = 1}, .m_objs = {((lean_object*)&lp_zhc__lean_Zhc_Iops_HillisSteele___aux__Zhc__Iops__AddHillisSteele__Theorems______macroRules__Zhc__Iops__HillisSteele__tacticProjects__step__1___closed__48_value_aux_2),((lean_object*)&lp_zhc__lean_Zhc_Iops_HillisSteele___aux__Zhc__Iops__AddHillisSteele__Theorems______macroRules__Zhc__Iops__HillisSteele__tacticProjects__step__1___closed__47_value),LEAN_SCALAR_PTR_LITERAL(226, 89, 129, 113, 173, 121, 169, 188)}};
static const lean_object* lp_zhc__lean_Zhc_Iops_HillisSteele___aux__Zhc__Iops__AddHillisSteele__Theorems______macroRules__Zhc__Iops__HillisSteele__tacticProjects__step__1___closed__48 = (const lean_object*)&lp_zhc__lean_Zhc_Iops_HillisSteele___aux__Zhc__Iops__AddHillisSteele__Theorems______macroRules__Zhc__Iops__HillisSteele__tacticProjects__step__1___closed__48_value;
static const lean_string_object lp_zhc__lean_Zhc_Iops_HillisSteele___aux__Zhc__Iops__AddHillisSteele__Theorems______macroRules__Zhc__Iops__HillisSteele__tacticProjects__step__1___closed__49_value = {.m_header = {.m_rc = 0, .m_cs_sz = 0, .m_other = 0, .m_tag = 249}, .m_size = 2, .m_capacity = 2, .m_length = 1, .m_data = "f"};
static const lean_object* lp_zhc__lean_Zhc_Iops_HillisSteele___aux__Zhc__Iops__AddHillisSteele__Theorems______macroRules__Zhc__Iops__HillisSteele__tacticProjects__step__1___closed__49 = (const lean_object*)&lp_zhc__lean_Zhc_Iops_HillisSteele___aux__Zhc__Iops__AddHillisSteele__Theorems______macroRules__Zhc__Iops__HillisSteele__tacticProjects__step__1___closed__49_value;
static lean_once_cell_t lp_zhc__lean_Zhc_Iops_HillisSteele___aux__Zhc__Iops__AddHillisSteele__Theorems______macroRules__Zhc__Iops__HillisSteele__tacticProjects__step__1___closed__50_once = LEAN_ONCE_CELL_INITIALIZER;
static lean_object* lp_zhc__lean_Zhc_Iops_HillisSteele___aux__Zhc__Iops__AddHillisSteele__Theorems______macroRules__Zhc__Iops__HillisSteele__tacticProjects__step__1___closed__50;
static const lean_ctor_object lp_zhc__lean_Zhc_Iops_HillisSteele___aux__Zhc__Iops__AddHillisSteele__Theorems______macroRules__Zhc__Iops__HillisSteele__tacticProjects__step__1___closed__51_value = {.m_header = {.m_rc = 0, .m_cs_sz = sizeof(lean_ctor_object) + sizeof(void*)*2 + 8, .m_other = 2, .m_tag = 1}, .m_objs = {((lean_object*)(((size_t)(0) << 1) | 1)),((lean_object*)&lp_zhc__lean_Zhc_Iops_HillisSteele___aux__Zhc__Iops__AddHillisSteele__Theorems______macroRules__Zhc__Iops__HillisSteele__tacticProjects__step__1___closed__49_value),LEAN_SCALAR_PTR_LITERAL(29, 68, 183, 24, 128, 148, 178, 23)}};
static const lean_object* lp_zhc__lean_Zhc_Iops_HillisSteele___aux__Zhc__Iops__AddHillisSteele__Theorems______macroRules__Zhc__Iops__HillisSteele__tacticProjects__step__1___closed__51 = (const lean_object*)&lp_zhc__lean_Zhc_Iops_HillisSteele___aux__Zhc__Iops__AddHillisSteele__Theorems______macroRules__Zhc__Iops__HillisSteele__tacticProjects__step__1___closed__51_value;
static const lean_string_object lp_zhc__lean_Zhc_Iops_HillisSteele___aux__Zhc__Iops__AddHillisSteele__Theorems______macroRules__Zhc__Iops__HillisSteele__tacticProjects__step__1___closed__52_value = {.m_header = {.m_rc = 0, .m_cs_sz = 0, .m_other = 0, .m_tag = 249}, .m_size = 3, .m_capacity = 3, .m_length = 2, .m_data = ":="};
static const lean_object* lp_zhc__lean_Zhc_Iops_HillisSteele___aux__Zhc__Iops__AddHillisSteele__Theorems______macroRules__Zhc__Iops__HillisSteele__tacticProjects__step__1___closed__52 = (const lean_object*)&lp_zhc__lean_Zhc_Iops_HillisSteele___aux__Zhc__Iops__AddHillisSteele__Theorems______macroRules__Zhc__Iops__HillisSteele__tacticProjects__step__1___closed__52_value;
static const lean_string_object lp_zhc__lean_Zhc_Iops_HillisSteele___aux__Zhc__Iops__AddHillisSteele__Theorems______macroRules__Zhc__Iops__HillisSteele__tacticProjects__step__1___closed__53_value = {.m_header = {.m_rc = 0, .m_cs_sz = 0, .m_other = 0, .m_tag = 249}, .m_size = 6, .m_capacity = 6, .m_length = 5, .m_data = "model"};
static const lean_object* lp_zhc__lean_Zhc_Iops_HillisSteele___aux__Zhc__Iops__AddHillisSteele__Theorems______macroRules__Zhc__Iops__HillisSteele__tacticProjects__step__1___closed__53 = (const lean_object*)&lp_zhc__lean_Zhc_Iops_HillisSteele___aux__Zhc__Iops__AddHillisSteele__Theorems______macroRules__Zhc__Iops__HillisSteele__tacticProjects__step__1___closed__53_value;
static lean_once_cell_t lp_zhc__lean_Zhc_Iops_HillisSteele___aux__Zhc__Iops__AddHillisSteele__Theorems______macroRules__Zhc__Iops__HillisSteele__tacticProjects__step__1___closed__54_once = LEAN_ONCE_CELL_INITIALIZER;
static lean_object* lp_zhc__lean_Zhc_Iops_HillisSteele___aux__Zhc__Iops__AddHillisSteele__Theorems______macroRules__Zhc__Iops__HillisSteele__tacticProjects__step__1___closed__54;
static const lean_ctor_object lp_zhc__lean_Zhc_Iops_HillisSteele___aux__Zhc__Iops__AddHillisSteele__Theorems______macroRules__Zhc__Iops__HillisSteele__tacticProjects__step__1___closed__55_value = {.m_header = {.m_rc = 0, .m_cs_sz = sizeof(lean_ctor_object) + sizeof(void*)*2 + 8, .m_other = 2, .m_tag = 1}, .m_objs = {((lean_object*)(((size_t)(0) << 1) | 1)),((lean_object*)&lp_zhc__lean_Zhc_Iops_HillisSteele___aux__Zhc__Iops__AddHillisSteele__Theorems______macroRules__Zhc__Iops__HillisSteele__tacticProjects__step__1___closed__53_value),LEAN_SCALAR_PTR_LITERAL(75, 116, 149, 162, 246, 98, 210, 113)}};
static const lean_object* lp_zhc__lean_Zhc_Iops_HillisSteele___aux__Zhc__Iops__AddHillisSteele__Theorems______macroRules__Zhc__Iops__HillisSteele__tacticProjects__step__1___closed__55 = (const lean_object*)&lp_zhc__lean_Zhc_Iops_HillisSteele___aux__Zhc__Iops__AddHillisSteele__Theorems______macroRules__Zhc__Iops__HillisSteele__tacticProjects__step__1___closed__55_value;
static const lean_string_object lp_zhc__lean_Zhc_Iops_HillisSteele___aux__Zhc__Iops__AddHillisSteele__Theorems______macroRules__Zhc__Iops__HillisSteele__tacticProjects__step__1___closed__56_value = {.m_header = {.m_rc = 0, .m_cs_sz = 0, .m_other = 0, .m_tag = 249}, .m_size = 9, .m_capacity = 9, .m_length = 8, .m_data = "HasModel"};
static const lean_object* lp_zhc__lean_Zhc_Iops_HillisSteele___aux__Zhc__Iops__AddHillisSteele__Theorems______macroRules__Zhc__Iops__HillisSteele__tacticProjects__step__1___closed__56 = (const lean_object*)&lp_zhc__lean_Zhc_Iops_HillisSteele___aux__Zhc__Iops__AddHillisSteele__Theorems______macroRules__Zhc__Iops__HillisSteele__tacticProjects__step__1___closed__56_value;
static const lean_ctor_object lp_zhc__lean_Zhc_Iops_HillisSteele___aux__Zhc__Iops__AddHillisSteele__Theorems______macroRules__Zhc__Iops__HillisSteele__tacticProjects__step__1___closed__57_value_aux_0 = {.m_header = {.m_rc = 0, .m_cs_sz = sizeof(lean_ctor_object) + sizeof(void*)*2 + 8, .m_other = 2, .m_tag = 1}, .m_objs = {((lean_object*)(((size_t)(0) << 1) | 1)),((lean_object*)&lp_zhc__lean_Zhc_Iops_HillisSteele_tacticProjects__step___closed__0_value),LEAN_SCALAR_PTR_LITERAL(204, 210, 61, 107, 112, 216, 72, 96)}};
static const lean_ctor_object lp_zhc__lean_Zhc_Iops_HillisSteele___aux__Zhc__Iops__AddHillisSteele__Theorems______macroRules__Zhc__Iops__HillisSteele__tacticProjects__step__1___closed__57_value_aux_1 = {.m_header = {.m_rc = 0, .m_cs_sz = sizeof(lean_ctor_object) + sizeof(void*)*2 + 8, .m_other = 2, .m_tag = 1}, .m_objs = {((lean_object*)&lp_zhc__lean_Zhc_Iops_HillisSteele___aux__Zhc__Iops__AddHillisSteele__Theorems______macroRules__Zhc__Iops__HillisSteele__tacticProjects__step__1___closed__57_value_aux_0),((lean_object*)&lp_zhc__lean_Zhc_Iops_HillisSteele___aux__Zhc__Iops__AddHillisSteele__Theorems______macroRules__Zhc__Iops__HillisSteele__tacticProjects__step__1___closed__24_value),LEAN_SCALAR_PTR_LITERAL(191, 108, 44, 3, 254, 30, 252, 26)}};
static const lean_ctor_object lp_zhc__lean_Zhc_Iops_HillisSteele___aux__Zhc__Iops__AddHillisSteele__Theorems______macroRules__Zhc__Iops__HillisSteele__tacticProjects__step__1___closed__57_value_aux_2 = {.m_header = {.m_rc = 0, .m_cs_sz = sizeof(lean_ctor_object) + sizeof(void*)*2 + 8, .m_other = 2, .m_tag = 1}, .m_objs = {((lean_object*)&lp_zhc__lean_Zhc_Iops_HillisSteele___aux__Zhc__Iops__AddHillisSteele__Theorems______macroRules__Zhc__Iops__HillisSteele__tacticProjects__step__1___closed__57_value_aux_1),((lean_object*)&lp_zhc__lean_Zhc_Iops_HillisSteele___aux__Zhc__Iops__AddHillisSteele__Theorems______macroRules__Zhc__Iops__HillisSteele__tacticProjects__step__1___closed__25_value),LEAN_SCALAR_PTR_LITERAL(29, 36, 229, 118, 140, 245, 224, 135)}};
static const lean_ctor_object lp_zhc__lean_Zhc_Iops_HillisSteele___aux__Zhc__Iops__AddHillisSteele__Theorems______macroRules__Zhc__Iops__HillisSteele__tacticProjects__step__1___closed__57_value_aux_3 = {.m_header = {.m_rc = 0, .m_cs_sz = sizeof(lean_ctor_object) + sizeof(void*)*2 + 8, .m_other = 2, .m_tag = 1}, .m_objs = {((lean_object*)&lp_zhc__lean_Zhc_Iops_HillisSteele___aux__Zhc__Iops__AddHillisSteele__Theorems______macroRules__Zhc__Iops__HillisSteele__tacticProjects__step__1___closed__57_value_aux_2),((lean_object*)&lp_zhc__lean_Zhc_Iops_HillisSteele___aux__Zhc__Iops__AddHillisSteele__Theorems______macroRules__Zhc__Iops__HillisSteele__tacticProjects__step__1___closed__56_value),LEAN_SCALAR_PTR_LITERAL(11, 116, 121, 229, 4, 188, 35, 115)}};
static const lean_ctor_object lp_zhc__lean_Zhc_Iops_HillisSteele___aux__Zhc__Iops__AddHillisSteele__Theorems______macroRules__Zhc__Iops__HillisSteele__tacticProjects__step__1___closed__57_value = {.m_header = {.m_rc = 0, .m_cs_sz = sizeof(lean_ctor_object) + sizeof(void*)*2 + 8, .m_other = 2, .m_tag = 1}, .m_objs = {((lean_object*)&lp_zhc__lean_Zhc_Iops_HillisSteele___aux__Zhc__Iops__AddHillisSteele__Theorems______macroRules__Zhc__Iops__HillisSteele__tacticProjects__step__1___closed__57_value_aux_3),((lean_object*)&lp_zhc__lean_Zhc_Iops_HillisSteele___aux__Zhc__Iops__AddHillisSteele__Theorems______macroRules__Zhc__Iops__HillisSteele__tacticProjects__step__1___closed__53_value),LEAN_SCALAR_PTR_LITERAL(27, 158, 244, 167, 159, 121, 64, 93)}};
static const lean_object* lp_zhc__lean_Zhc_Iops_HillisSteele___aux__Zhc__Iops__AddHillisSteele__Theorems______macroRules__Zhc__Iops__HillisSteele__tacticProjects__step__1___closed__57 = (const lean_object*)&lp_zhc__lean_Zhc_Iops_HillisSteele___aux__Zhc__Iops__AddHillisSteele__Theorems______macroRules__Zhc__Iops__HillisSteele__tacticProjects__step__1___closed__57_value;
static const lean_ctor_object lp_zhc__lean_Zhc_Iops_HillisSteele___aux__Zhc__Iops__AddHillisSteele__Theorems______macroRules__Zhc__Iops__HillisSteele__tacticProjects__step__1___closed__58_value = {.m_header = {.m_rc = 0, .m_cs_sz = sizeof(lean_ctor_object) + sizeof(void*)*2 + 0, .m_other = 2, .m_tag = 1}, .m_objs = {((lean_object*)&lp_zhc__lean_Zhc_Iops_HillisSteele___aux__Zhc__Iops__AddHillisSteele__Theorems______macroRules__Zhc__Iops__HillisSteele__tacticProjects__step__1___closed__57_value),((lean_object*)(((size_t)(0) << 1) | 1))}};
static const lean_object* lp_zhc__lean_Zhc_Iops_HillisSteele___aux__Zhc__Iops__AddHillisSteele__Theorems______macroRules__Zhc__Iops__HillisSteele__tacticProjects__step__1___closed__58 = (const lean_object*)&lp_zhc__lean_Zhc_Iops_HillisSteele___aux__Zhc__Iops__AddHillisSteele__Theorems______macroRules__Zhc__Iops__HillisSteele__tacticProjects__step__1___closed__58_value;
static const lean_ctor_object lp_zhc__lean_Zhc_Iops_HillisSteele___aux__Zhc__Iops__AddHillisSteele__Theorems______macroRules__Zhc__Iops__HillisSteele__tacticProjects__step__1___closed__59_value = {.m_header = {.m_rc = 0, .m_cs_sz = sizeof(lean_ctor_object) + sizeof(void*)*2 + 0, .m_other = 2, .m_tag = 1}, .m_objs = {((lean_object*)&lp_zhc__lean_Zhc_Iops_HillisSteele___aux__Zhc__Iops__AddHillisSteele__Theorems______macroRules__Zhc__Iops__HillisSteele__tacticProjects__step__1___closed__58_value),((lean_object*)(((size_t)(0) << 1) | 1))}};
static const lean_object* lp_zhc__lean_Zhc_Iops_HillisSteele___aux__Zhc__Iops__AddHillisSteele__Theorems______macroRules__Zhc__Iops__HillisSteele__tacticProjects__step__1___closed__59 = (const lean_object*)&lp_zhc__lean_Zhc_Iops_HillisSteele___aux__Zhc__Iops__AddHillisSteele__Theorems______macroRules__Zhc__Iops__HillisSteele__tacticProjects__step__1___closed__59_value;
static const lean_string_object lp_zhc__lean_Zhc_Iops_HillisSteele___aux__Zhc__Iops__AddHillisSteele__Theorems______macroRules__Zhc__Iops__HillisSteele__tacticProjects__step__1___closed__60_value = {.m_header = {.m_rc = 0, .m_cs_sz = 0, .m_other = 0, .m_tag = 249}, .m_size = 2, .m_capacity = 2, .m_length = 1, .m_data = ")"};
static const lean_object* lp_zhc__lean_Zhc_Iops_HillisSteele___aux__Zhc__Iops__AddHillisSteele__Theorems______macroRules__Zhc__Iops__HillisSteele__tacticProjects__step__1___closed__60 = (const lean_object*)&lp_zhc__lean_Zhc_Iops_HillisSteele___aux__Zhc__Iops__AddHillisSteele__Theorems______macroRules__Zhc__Iops__HillisSteele__tacticProjects__step__1___closed__60_value;
static const lean_string_object lp_zhc__lean_Zhc_Iops_HillisSteele___aux__Zhc__Iops__AddHillisSteele__Theorems______macroRules__Zhc__Iops__HillisSteele__tacticProjects__step__1___closed__61_value = {.m_header = {.m_rc = 0, .m_cs_sz = 0, .m_other = 0, .m_tag = 249}, .m_size = 14, .m_capacity = 14, .m_length = 13, .m_data = "syntheticHole"};
static const lean_object* lp_zhc__lean_Zhc_Iops_HillisSteele___aux__Zhc__Iops__AddHillisSteele__Theorems______macroRules__Zhc__Iops__HillisSteele__tacticProjects__step__1___closed__61 = (const lean_object*)&lp_zhc__lean_Zhc_Iops_HillisSteele___aux__Zhc__Iops__AddHillisSteele__Theorems______macroRules__Zhc__Iops__HillisSteele__tacticProjects__step__1___closed__61_value;
static const lean_ctor_object lp_zhc__lean_Zhc_Iops_HillisSteele___aux__Zhc__Iops__AddHillisSteele__Theorems______macroRules__Zhc__Iops__HillisSteele__tacticProjects__step__1___closed__62_value_aux_0 = {.m_header = {.m_rc = 0, .m_cs_sz = sizeof(lean_ctor_object) + sizeof(void*)*2 + 8, .m_other = 2, .m_tag = 1}, .m_objs = {((lean_object*)(((size_t)(0) << 1) | 1)),((lean_object*)&lp_zhc__lean_Zhc_Iops_HillisSteele___aux__Zhc__Iops__AddHillisSteele__Theorems______macroRules__Zhc__Iops__HillisSteele__tacticProjects__step__1___closed__0_value),LEAN_SCALAR_PTR_LITERAL(70, 193, 83, 126, 233, 67, 208, 165)}};
static const lean_ctor_object lp_zhc__lean_Zhc_Iops_HillisSteele___aux__Zhc__Iops__AddHillisSteele__Theorems______macroRules__Zhc__Iops__HillisSteele__tacticProjects__step__1___closed__62_value_aux_1 = {.m_header = {.m_rc = 0, .m_cs_sz = sizeof(lean_ctor_object) + sizeof(void*)*2 + 8, .m_other = 2, .m_tag = 1}, .m_objs = {((lean_object*)&lp_zhc__lean_Zhc_Iops_HillisSteele___aux__Zhc__Iops__AddHillisSteele__Theorems______macroRules__Zhc__Iops__HillisSteele__tacticProjects__step__1___closed__62_value_aux_0),((lean_object*)&lp_zhc__lean_Zhc_Iops_HillisSteele___aux__Zhc__Iops__AddHillisSteele__Theorems______macroRules__Zhc__Iops__HillisSteele__tacticProjects__step__1___closed__1_value),LEAN_SCALAR_PTR_LITERAL(103, 136, 125, 166, 167, 98, 71, 111)}};
static const lean_ctor_object lp_zhc__lean_Zhc_Iops_HillisSteele___aux__Zhc__Iops__AddHillisSteele__Theorems______macroRules__Zhc__Iops__HillisSteele__tacticProjects__step__1___closed__62_value_aux_2 = {.m_header = {.m_rc = 0, .m_cs_sz = sizeof(lean_ctor_object) + sizeof(void*)*2 + 8, .m_other = 2, .m_tag = 1}, .m_objs = {((lean_object*)&lp_zhc__lean_Zhc_Iops_HillisSteele___aux__Zhc__Iops__AddHillisSteele__Theorems______macroRules__Zhc__Iops__HillisSteele__tacticProjects__step__1___closed__62_value_aux_1),((lean_object*)&lp_zhc__lean_Zhc_Iops_HillisSteele___aux__Zhc__Iops__AddHillisSteele__Theorems______macroRules__Zhc__Iops__HillisSteele__tacticProjects__step__1___closed__16_value),LEAN_SCALAR_PTR_LITERAL(75, 170, 162, 138, 136, 204, 251, 229)}};
static const lean_ctor_object lp_zhc__lean_Zhc_Iops_HillisSteele___aux__Zhc__Iops__AddHillisSteele__Theorems______macroRules__Zhc__Iops__HillisSteele__tacticProjects__step__1___closed__62_value = {.m_header = {.m_rc = 0, .m_cs_sz = sizeof(lean_ctor_object) + sizeof(void*)*2 + 8, .m_other = 2, .m_tag = 1}, .m_objs = {((lean_object*)&lp_zhc__lean_Zhc_Iops_HillisSteele___aux__Zhc__Iops__AddHillisSteele__Theorems______macroRules__Zhc__Iops__HillisSteele__tacticProjects__step__1___closed__62_value_aux_2),((lean_object*)&lp_zhc__lean_Zhc_Iops_HillisSteele___aux__Zhc__Iops__AddHillisSteele__Theorems______macroRules__Zhc__Iops__HillisSteele__tacticProjects__step__1___closed__61_value),LEAN_SCALAR_PTR_LITERAL(218, 189, 67, 60, 211, 196, 112, 165)}};
static const lean_object* lp_zhc__lean_Zhc_Iops_HillisSteele___aux__Zhc__Iops__AddHillisSteele__Theorems______macroRules__Zhc__Iops__HillisSteele__tacticProjects__step__1___closed__62 = (const lean_object*)&lp_zhc__lean_Zhc_Iops_HillisSteele___aux__Zhc__Iops__AddHillisSteele__Theorems______macroRules__Zhc__Iops__HillisSteele__tacticProjects__step__1___closed__62_value;
static const lean_string_object lp_zhc__lean_Zhc_Iops_HillisSteele___aux__Zhc__Iops__AddHillisSteele__Theorems______macroRules__Zhc__Iops__HillisSteele__tacticProjects__step__1___closed__63_value = {.m_header = {.m_rc = 0, .m_cs_sz = 0, .m_other = 0, .m_tag = 249}, .m_size = 2, .m_capacity = 2, .m_length = 1, .m_data = "\?"};
static const lean_object* lp_zhc__lean_Zhc_Iops_HillisSteele___aux__Zhc__Iops__AddHillisSteele__Theorems______macroRules__Zhc__Iops__HillisSteele__tacticProjects__step__1___closed__63 = (const lean_object*)&lp_zhc__lean_Zhc_Iops_HillisSteele___aux__Zhc__Iops__AddHillisSteele__Theorems______macroRules__Zhc__Iops__HillisSteele__tacticProjects__step__1___closed__63_value;
static const lean_string_object lp_zhc__lean_Zhc_Iops_HillisSteele___aux__Zhc__Iops__AddHillisSteele__Theorems______macroRules__Zhc__Iops__HillisSteele__tacticProjects__step__1___closed__64_value = {.m_header = {.m_rc = 0, .m_cs_sz = 0, .m_other = 0, .m_tag = 249}, .m_size = 4, .m_capacity = 4, .m_length = 3, .m_data = "fun"};
static const lean_object* lp_zhc__lean_Zhc_Iops_HillisSteele___aux__Zhc__Iops__AddHillisSteele__Theorems______macroRules__Zhc__Iops__HillisSteele__tacticProjects__step__1___closed__64 = (const lean_object*)&lp_zhc__lean_Zhc_Iops_HillisSteele___aux__Zhc__Iops__AddHillisSteele__Theorems______macroRules__Zhc__Iops__HillisSteele__tacticProjects__step__1___closed__64_value;
static const lean_ctor_object lp_zhc__lean_Zhc_Iops_HillisSteele___aux__Zhc__Iops__AddHillisSteele__Theorems______macroRules__Zhc__Iops__HillisSteele__tacticProjects__step__1___closed__65_value_aux_0 = {.m_header = {.m_rc = 0, .m_cs_sz = sizeof(lean_ctor_object) + sizeof(void*)*2 + 8, .m_other = 2, .m_tag = 1}, .m_objs = {((lean_object*)(((size_t)(0) << 1) | 1)),((lean_object*)&lp_zhc__lean_Zhc_Iops_HillisSteele___aux__Zhc__Iops__AddHillisSteele__Theorems______macroRules__Zhc__Iops__HillisSteele__tacticProjects__step__1___closed__0_value),LEAN_SCALAR_PTR_LITERAL(70, 193, 83, 126, 233, 67, 208, 165)}};
static const lean_ctor_object lp_zhc__lean_Zhc_Iops_HillisSteele___aux__Zhc__Iops__AddHillisSteele__Theorems______macroRules__Zhc__Iops__HillisSteele__tacticProjects__step__1___closed__65_value_aux_1 = {.m_header = {.m_rc = 0, .m_cs_sz = sizeof(lean_ctor_object) + sizeof(void*)*2 + 8, .m_other = 2, .m_tag = 1}, .m_objs = {((lean_object*)&lp_zhc__lean_Zhc_Iops_HillisSteele___aux__Zhc__Iops__AddHillisSteele__Theorems______macroRules__Zhc__Iops__HillisSteele__tacticProjects__step__1___closed__65_value_aux_0),((lean_object*)&lp_zhc__lean_Zhc_Iops_HillisSteele___aux__Zhc__Iops__AddHillisSteele__Theorems______macroRules__Zhc__Iops__HillisSteele__tacticProjects__step__1___closed__1_value),LEAN_SCALAR_PTR_LITERAL(103, 136, 125, 166, 167, 98, 71, 111)}};
static const lean_ctor_object lp_zhc__lean_Zhc_Iops_HillisSteele___aux__Zhc__Iops__AddHillisSteele__Theorems______macroRules__Zhc__Iops__HillisSteele__tacticProjects__step__1___closed__65_value_aux_2 = {.m_header = {.m_rc = 0, .m_cs_sz = sizeof(lean_ctor_object) + sizeof(void*)*2 + 8, .m_other = 2, .m_tag = 1}, .m_objs = {((lean_object*)&lp_zhc__lean_Zhc_Iops_HillisSteele___aux__Zhc__Iops__AddHillisSteele__Theorems______macroRules__Zhc__Iops__HillisSteele__tacticProjects__step__1___closed__65_value_aux_1),((lean_object*)&lp_zhc__lean_Zhc_Iops_HillisSteele___aux__Zhc__Iops__AddHillisSteele__Theorems______macroRules__Zhc__Iops__HillisSteele__tacticProjects__step__1___closed__16_value),LEAN_SCALAR_PTR_LITERAL(75, 170, 162, 138, 136, 204, 251, 229)}};
static const lean_ctor_object lp_zhc__lean_Zhc_Iops_HillisSteele___aux__Zhc__Iops__AddHillisSteele__Theorems______macroRules__Zhc__Iops__HillisSteele__tacticProjects__step__1___closed__65_value = {.m_header = {.m_rc = 0, .m_cs_sz = sizeof(lean_ctor_object) + sizeof(void*)*2 + 8, .m_other = 2, .m_tag = 1}, .m_objs = {((lean_object*)&lp_zhc__lean_Zhc_Iops_HillisSteele___aux__Zhc__Iops__AddHillisSteele__Theorems______macroRules__Zhc__Iops__HillisSteele__tacticProjects__step__1___closed__65_value_aux_2),((lean_object*)&lp_zhc__lean_Zhc_Iops_HillisSteele___aux__Zhc__Iops__AddHillisSteele__Theorems______macroRules__Zhc__Iops__HillisSteele__tacticProjects__step__1___closed__64_value),LEAN_SCALAR_PTR_LITERAL(249, 155, 133, 242, 71, 132, 191, 97)}};
static const lean_object* lp_zhc__lean_Zhc_Iops_HillisSteele___aux__Zhc__Iops__AddHillisSteele__Theorems______macroRules__Zhc__Iops__HillisSteele__tacticProjects__step__1___closed__65 = (const lean_object*)&lp_zhc__lean_Zhc_Iops_HillisSteele___aux__Zhc__Iops__AddHillisSteele__Theorems______macroRules__Zhc__Iops__HillisSteele__tacticProjects__step__1___closed__65_value;
static const lean_string_object lp_zhc__lean_Zhc_Iops_HillisSteele___aux__Zhc__Iops__AddHillisSteele__Theorems______macroRules__Zhc__Iops__HillisSteele__tacticProjects__step__1___closed__66_value = {.m_header = {.m_rc = 0, .m_cs_sz = 0, .m_other = 0, .m_tag = 249}, .m_size = 9, .m_capacity = 9, .m_length = 8, .m_data = "basicFun"};
static const lean_object* lp_zhc__lean_Zhc_Iops_HillisSteele___aux__Zhc__Iops__AddHillisSteele__Theorems______macroRules__Zhc__Iops__HillisSteele__tacticProjects__step__1___closed__66 = (const lean_object*)&lp_zhc__lean_Zhc_Iops_HillisSteele___aux__Zhc__Iops__AddHillisSteele__Theorems______macroRules__Zhc__Iops__HillisSteele__tacticProjects__step__1___closed__66_value;
static const lean_ctor_object lp_zhc__lean_Zhc_Iops_HillisSteele___aux__Zhc__Iops__AddHillisSteele__Theorems______macroRules__Zhc__Iops__HillisSteele__tacticProjects__step__1___closed__67_value_aux_0 = {.m_header = {.m_rc = 0, .m_cs_sz = sizeof(lean_ctor_object) + sizeof(void*)*2 + 8, .m_other = 2, .m_tag = 1}, .m_objs = {((lean_object*)(((size_t)(0) << 1) | 1)),((lean_object*)&lp_zhc__lean_Zhc_Iops_HillisSteele___aux__Zhc__Iops__AddHillisSteele__Theorems______macroRules__Zhc__Iops__HillisSteele__tacticProjects__step__1___closed__0_value),LEAN_SCALAR_PTR_LITERAL(70, 193, 83, 126, 233, 67, 208, 165)}};
static const lean_ctor_object lp_zhc__lean_Zhc_Iops_HillisSteele___aux__Zhc__Iops__AddHillisSteele__Theorems______macroRules__Zhc__Iops__HillisSteele__tacticProjects__step__1___closed__67_value_aux_1 = {.m_header = {.m_rc = 0, .m_cs_sz = sizeof(lean_ctor_object) + sizeof(void*)*2 + 8, .m_other = 2, .m_tag = 1}, .m_objs = {((lean_object*)&lp_zhc__lean_Zhc_Iops_HillisSteele___aux__Zhc__Iops__AddHillisSteele__Theorems______macroRules__Zhc__Iops__HillisSteele__tacticProjects__step__1___closed__67_value_aux_0),((lean_object*)&lp_zhc__lean_Zhc_Iops_HillisSteele___aux__Zhc__Iops__AddHillisSteele__Theorems______macroRules__Zhc__Iops__HillisSteele__tacticProjects__step__1___closed__1_value),LEAN_SCALAR_PTR_LITERAL(103, 136, 125, 166, 167, 98, 71, 111)}};
static const lean_ctor_object lp_zhc__lean_Zhc_Iops_HillisSteele___aux__Zhc__Iops__AddHillisSteele__Theorems______macroRules__Zhc__Iops__HillisSteele__tacticProjects__step__1___closed__67_value_aux_2 = {.m_header = {.m_rc = 0, .m_cs_sz = sizeof(lean_ctor_object) + sizeof(void*)*2 + 8, .m_other = 2, .m_tag = 1}, .m_objs = {((lean_object*)&lp_zhc__lean_Zhc_Iops_HillisSteele___aux__Zhc__Iops__AddHillisSteele__Theorems______macroRules__Zhc__Iops__HillisSteele__tacticProjects__step__1___closed__67_value_aux_1),((lean_object*)&lp_zhc__lean_Zhc_Iops_HillisSteele___aux__Zhc__Iops__AddHillisSteele__Theorems______macroRules__Zhc__Iops__HillisSteele__tacticProjects__step__1___closed__16_value),LEAN_SCALAR_PTR_LITERAL(75, 170, 162, 138, 136, 204, 251, 229)}};
static const lean_ctor_object lp_zhc__lean_Zhc_Iops_HillisSteele___aux__Zhc__Iops__AddHillisSteele__Theorems______macroRules__Zhc__Iops__HillisSteele__tacticProjects__step__1___closed__67_value = {.m_header = {.m_rc = 0, .m_cs_sz = sizeof(lean_ctor_object) + sizeof(void*)*2 + 8, .m_other = 2, .m_tag = 1}, .m_objs = {((lean_object*)&lp_zhc__lean_Zhc_Iops_HillisSteele___aux__Zhc__Iops__AddHillisSteele__Theorems______macroRules__Zhc__Iops__HillisSteele__tacticProjects__step__1___closed__67_value_aux_2),((lean_object*)&lp_zhc__lean_Zhc_Iops_HillisSteele___aux__Zhc__Iops__AddHillisSteele__Theorems______macroRules__Zhc__Iops__HillisSteele__tacticProjects__step__1___closed__66_value),LEAN_SCALAR_PTR_LITERAL(209, 134, 40, 160, 122, 195, 31, 223)}};
static const lean_object* lp_zhc__lean_Zhc_Iops_HillisSteele___aux__Zhc__Iops__AddHillisSteele__Theorems______macroRules__Zhc__Iops__HillisSteele__tacticProjects__step__1___closed__67 = (const lean_object*)&lp_zhc__lean_Zhc_Iops_HillisSteele___aux__Zhc__Iops__AddHillisSteele__Theorems______macroRules__Zhc__Iops__HillisSteele__tacticProjects__step__1___closed__67_value;
static lean_once_cell_t lp_zhc__lean_Zhc_Iops_HillisSteele___aux__Zhc__Iops__AddHillisSteele__Theorems______macroRules__Zhc__Iops__HillisSteele__tacticProjects__step__1___closed__68_once = LEAN_ONCE_CELL_INITIALIZER;
static lean_object* lp_zhc__lean_Zhc_Iops_HillisSteele___aux__Zhc__Iops__AddHillisSteele__Theorems______macroRules__Zhc__Iops__HillisSteele__tacticProjects__step__1___closed__68;
static const lean_string_object lp_zhc__lean_Zhc_Iops_HillisSteele___aux__Zhc__Iops__AddHillisSteele__Theorems______macroRules__Zhc__Iops__HillisSteele__tacticProjects__step__1___closed__69_value = {.m_header = {.m_rc = 0, .m_cs_sz = 0, .m_other = 0, .m_tag = 249}, .m_size = 3, .m_capacity = 3, .m_length = 2, .m_data = "=>"};
static const lean_object* lp_zhc__lean_Zhc_Iops_HillisSteele___aux__Zhc__Iops__AddHillisSteele__Theorems______macroRules__Zhc__Iops__HillisSteele__tacticProjects__step__1___closed__69 = (const lean_object*)&lp_zhc__lean_Zhc_Iops_HillisSteele___aux__Zhc__Iops__AddHillisSteele__Theorems______macroRules__Zhc__Iops__HillisSteele__tacticProjects__step__1___closed__69_value;
static const lean_string_object lp_zhc__lean_Zhc_Iops_HillisSteele___aux__Zhc__Iops__AddHillisSteele__Theorems______macroRules__Zhc__Iops__HillisSteele__tacticProjects__step__1___closed__70_value = {.m_header = {.m_rc = 0, .m_cs_sz = 0, .m_other = 0, .m_tag = 249}, .m_size = 6, .m_capacity = 6, .m_length = 5, .m_data = "focus"};
static const lean_object* lp_zhc__lean_Zhc_Iops_HillisSteele___aux__Zhc__Iops__AddHillisSteele__Theorems______macroRules__Zhc__Iops__HillisSteele__tacticProjects__step__1___closed__70 = (const lean_object*)&lp_zhc__lean_Zhc_Iops_HillisSteele___aux__Zhc__Iops__AddHillisSteele__Theorems______macroRules__Zhc__Iops__HillisSteele__tacticProjects__step__1___closed__70_value;
static const lean_ctor_object lp_zhc__lean_Zhc_Iops_HillisSteele___aux__Zhc__Iops__AddHillisSteele__Theorems______macroRules__Zhc__Iops__HillisSteele__tacticProjects__step__1___closed__71_value_aux_0 = {.m_header = {.m_rc = 0, .m_cs_sz = sizeof(lean_ctor_object) + sizeof(void*)*2 + 8, .m_other = 2, .m_tag = 1}, .m_objs = {((lean_object*)(((size_t)(0) << 1) | 1)),((lean_object*)&lp_zhc__lean_Zhc_Iops_HillisSteele___aux__Zhc__Iops__AddHillisSteele__Theorems______macroRules__Zhc__Iops__HillisSteele__tacticProjects__step__1___closed__0_value),LEAN_SCALAR_PTR_LITERAL(70, 193, 83, 126, 233, 67, 208, 165)}};
static const lean_ctor_object lp_zhc__lean_Zhc_Iops_HillisSteele___aux__Zhc__Iops__AddHillisSteele__Theorems______macroRules__Zhc__Iops__HillisSteele__tacticProjects__step__1___closed__71_value_aux_1 = {.m_header = {.m_rc = 0, .m_cs_sz = sizeof(lean_ctor_object) + sizeof(void*)*2 + 8, .m_other = 2, .m_tag = 1}, .m_objs = {((lean_object*)&lp_zhc__lean_Zhc_Iops_HillisSteele___aux__Zhc__Iops__AddHillisSteele__Theorems______macroRules__Zhc__Iops__HillisSteele__tacticProjects__step__1___closed__71_value_aux_0),((lean_object*)&lp_zhc__lean_Zhc_Iops_HillisSteele___aux__Zhc__Iops__AddHillisSteele__Theorems______macroRules__Zhc__Iops__HillisSteele__tacticProjects__step__1___closed__1_value),LEAN_SCALAR_PTR_LITERAL(103, 136, 125, 166, 167, 98, 71, 111)}};
static const lean_ctor_object lp_zhc__lean_Zhc_Iops_HillisSteele___aux__Zhc__Iops__AddHillisSteele__Theorems______macroRules__Zhc__Iops__HillisSteele__tacticProjects__step__1___closed__71_value_aux_2 = {.m_header = {.m_rc = 0, .m_cs_sz = sizeof(lean_ctor_object) + sizeof(void*)*2 + 8, .m_other = 2, .m_tag = 1}, .m_objs = {((lean_object*)&lp_zhc__lean_Zhc_Iops_HillisSteele___aux__Zhc__Iops__AddHillisSteele__Theorems______macroRules__Zhc__Iops__HillisSteele__tacticProjects__step__1___closed__71_value_aux_1),((lean_object*)&lp_zhc__lean_Zhc_Iops_HillisSteele___aux__Zhc__Iops__AddHillisSteele__Theorems______macroRules__Zhc__Iops__HillisSteele__tacticProjects__step__1___closed__2_value),LEAN_SCALAR_PTR_LITERAL(166, 58, 35, 182, 187, 130, 147, 254)}};
static const lean_ctor_object lp_zhc__lean_Zhc_Iops_HillisSteele___aux__Zhc__Iops__AddHillisSteele__Theorems______macroRules__Zhc__Iops__HillisSteele__tacticProjects__step__1___closed__71_value = {.m_header = {.m_rc = 0, .m_cs_sz = sizeof(lean_ctor_object) + sizeof(void*)*2 + 8, .m_other = 2, .m_tag = 1}, .m_objs = {((lean_object*)&lp_zhc__lean_Zhc_Iops_HillisSteele___aux__Zhc__Iops__AddHillisSteele__Theorems______macroRules__Zhc__Iops__HillisSteele__tacticProjects__step__1___closed__71_value_aux_2),((lean_object*)&lp_zhc__lean_Zhc_Iops_HillisSteele___aux__Zhc__Iops__AddHillisSteele__Theorems______macroRules__Zhc__Iops__HillisSteele__tacticProjects__step__1___closed__70_value),LEAN_SCALAR_PTR_LITERAL(198, 223, 207, 6, 131, 57, 182, 221)}};
static const lean_object* lp_zhc__lean_Zhc_Iops_HillisSteele___aux__Zhc__Iops__AddHillisSteele__Theorems______macroRules__Zhc__Iops__HillisSteele__tacticProjects__step__1___closed__71 = (const lean_object*)&lp_zhc__lean_Zhc_Iops_HillisSteele___aux__Zhc__Iops__AddHillisSteele__Theorems______macroRules__Zhc__Iops__HillisSteele__tacticProjects__step__1___closed__71_value;
LEAN_EXPORT lean_object* lp_zhc__lean_Zhc_Iops_HillisSteele___aux__Zhc__Iops__AddHillisSteele__Theorems______macroRules__Zhc__Iops__HillisSteele__tacticProjects__step__1(lean_object*, lean_object*, lean_object*);
LEAN_EXPORT lean_object* lp_zhc__lean_Zhc_Iops_HillisSteele___aux__Zhc__Iops__AddHillisSteele__Theorems______macroRules__Zhc__Iops__HillisSteele__tacticProjects__step__1___boxed(lean_object*, lean_object*, lean_object*);
LEAN_EXPORT lean_object* lp_zhc__lean___private_Zhc_Iops_AddHillisSteele_Theorems_0__Zhc_Iops_HillisSteele_finalResolution_match__1_splitter___redArg(lean_object*, lean_object*, lean_object*);
LEAN_EXPORT lean_object* lp_zhc__lean___private_Zhc_Iops_AddHillisSteele_Theorems_0__Zhc_Iops_HillisSteele_finalResolution_match__1_splitter(lean_object*, lean_object*, lean_object*, lean_object*, lean_object*, lean_object*, lean_object*);
LEAN_EXPORT lean_object* lp_zhc__lean___private_Zhc_Iops_AddHillisSteele_Theorems_0__Zhc_Iops_HillisSteele_finalResolution_match__1_splitter___boxed(lean_object*, lean_object*, lean_object*, lean_object*, lean_object*, lean_object*, lean_object*);
LEAN_EXPORT lean_object* lp_zhc__lean_Zhc_Iops_HillisSteele_digit(lean_object* v_v_1_, lean_object* v_i_2_){
_start:
{
lean_object* v___x_3_; lean_object* v___x_4_; lean_object* v___x_5_; lean_object* v___x_6_; 
v___x_3_ = lean_unsigned_to_nat(4u);
v___x_4_ = lean_nat_pow(v___x_3_, v_i_2_);
v___x_5_ = lean_nat_div(v_v_1_, v___x_4_);
lean_dec(v___x_4_);
v___x_6_ = lean_nat_mod(v___x_5_, v___x_3_);
lean_dec(v___x_5_);
return v___x_6_;
}
}
LEAN_EXPORT lean_object* lp_zhc__lean_Zhc_Iops_HillisSteele_digit___boxed(lean_object* v_v_7_, lean_object* v_i_8_){
_start:
{
lean_object* v_res_9_; 
v_res_9_ = lp_zhc__lean_Zhc_Iops_HillisSteele_digit(v_v_7_, v_i_8_);
lean_dec(v_i_8_);
lean_dec(v_v_7_);
return v_res_9_;
}
}
LEAN_EXPORT lean_object* lp_zhc__lean_Zhc_Iops_HillisSteele_carryIn(lean_object* v_x_10_, lean_object* v_y_11_, lean_object* v_i_12_){
_start:
{
lean_object* v___x_13_; lean_object* v___x_14_; lean_object* v___x_15_; lean_object* v___x_16_; lean_object* v___x_17_; lean_object* v___x_18_; 
v___x_13_ = lean_unsigned_to_nat(4u);
v___x_14_ = lean_nat_pow(v___x_13_, v_i_12_);
v___x_15_ = lean_nat_mod(v_x_10_, v___x_14_);
v___x_16_ = lean_nat_mod(v_y_11_, v___x_14_);
v___x_17_ = lean_nat_add(v___x_15_, v___x_16_);
lean_dec(v___x_16_);
lean_dec(v___x_15_);
v___x_18_ = lean_nat_div(v___x_17_, v___x_14_);
lean_dec(v___x_14_);
lean_dec(v___x_17_);
return v___x_18_;
}
}
LEAN_EXPORT lean_object* lp_zhc__lean_Zhc_Iops_HillisSteele_carryIn___boxed(lean_object* v_x_19_, lean_object* v_y_20_, lean_object* v_i_21_){
_start:
{
lean_object* v_res_22_; 
v_res_22_ = lp_zhc__lean_Zhc_Iops_HillisSteele_carryIn(v_x_19_, v_y_20_, v_i_21_);
lean_dec(v_i_21_);
lean_dec(v_y_20_);
lean_dec(v_x_19_);
return v_res_22_;
}
}
LEAN_EXPORT lean_object* lp_zhc__lean_Zhc_Iops_HillisSteele_windowStatus(lean_object* v_x_23_, lean_object* v_y_24_, lean_object* v_lo_25_, lean_object* v_len_26_){
_start:
{
lean_object* v___x_27_; lean_object* v___x_28_; lean_object* v___x_29_; lean_object* v___x_30_; lean_object* v_a_31_; lean_object* v___x_32_; lean_object* v_b_33_; lean_object* v___x_34_; uint8_t v___x_35_; 
v___x_27_ = lean_unsigned_to_nat(4u);
v___x_28_ = lean_nat_pow(v___x_27_, v_lo_25_);
v___x_29_ = lean_nat_div(v_x_23_, v___x_28_);
v___x_30_ = lean_nat_pow(v___x_27_, v_len_26_);
v_a_31_ = lean_nat_mod(v___x_29_, v___x_30_);
lean_dec(v___x_29_);
v___x_32_ = lean_nat_div(v_y_24_, v___x_28_);
lean_dec(v___x_28_);
v_b_33_ = lean_nat_mod(v___x_32_, v___x_30_);
lean_dec(v___x_32_);
v___x_34_ = lean_nat_add(v_a_31_, v_b_33_);
lean_dec(v_b_33_);
lean_dec(v_a_31_);
v___x_35_ = lean_nat_dec_le(v___x_30_, v___x_34_);
if (v___x_35_ == 0)
{
lean_object* v___x_36_; lean_object* v___x_37_; uint8_t v___x_38_; 
v___x_36_ = lean_unsigned_to_nat(1u);
v___x_37_ = lean_nat_add(v___x_34_, v___x_36_);
lean_dec(v___x_34_);
v___x_38_ = lean_nat_dec_eq(v___x_37_, v___x_30_);
lean_dec(v___x_30_);
lean_dec(v___x_37_);
if (v___x_38_ == 0)
{
lean_object* v___x_39_; 
v___x_39_ = lean_unsigned_to_nat(0u);
return v___x_39_;
}
else
{
return v___x_36_;
}
}
else
{
lean_object* v___x_40_; 
lean_dec(v___x_34_);
lean_dec(v___x_30_);
v___x_40_ = lean_unsigned_to_nat(2u);
return v___x_40_;
}
}
}
LEAN_EXPORT lean_object* lp_zhc__lean_Zhc_Iops_HillisSteele_windowStatus___boxed(lean_object* v_x_41_, lean_object* v_y_42_, lean_object* v_lo_43_, lean_object* v_len_44_){
_start:
{
lean_object* v_res_45_; 
v_res_45_ = lp_zhc__lean_Zhc_Iops_HillisSteele_windowStatus(v_x_41_, v_y_42_, v_lo_43_, v_len_44_);
lean_dec(v_len_44_);
lean_dec(v_lo_43_);
lean_dec(v_y_42_);
lean_dec(v_x_41_);
return v_res_45_;
}
}
LEAN_EXPORT lean_object* lp_zhc__lean_Zhc_Iops_HillisSteele_blockStatus(lean_object* v_s_46_){
_start:
{
lean_object* v___x_47_; uint8_t v___x_48_; 
v___x_47_ = lean_unsigned_to_nat(4u);
v___x_48_ = lean_nat_dec_le(v___x_47_, v_s_46_);
if (v___x_48_ == 0)
{
lean_object* v___x_49_; uint8_t v___x_50_; 
v___x_49_ = lean_unsigned_to_nat(3u);
v___x_50_ = lean_nat_dec_eq(v_s_46_, v___x_49_);
if (v___x_50_ == 0)
{
lean_object* v___x_51_; 
v___x_51_ = lean_unsigned_to_nat(0u);
return v___x_51_;
}
else
{
lean_object* v___x_52_; 
v___x_52_ = lean_unsigned_to_nat(1u);
return v___x_52_;
}
}
else
{
lean_object* v___x_53_; 
v___x_53_ = lean_unsigned_to_nat(2u);
return v___x_53_;
}
}
}
LEAN_EXPORT lean_object* lp_zhc__lean_Zhc_Iops_HillisSteele_blockStatus___boxed(lean_object* v_s_54_){
_start:
{
lean_object* v_res_55_; 
v_res_55_ = lp_zhc__lean_Zhc_Iops_HillisSteele_blockStatus(v_s_54_);
lean_dec(v_s_54_);
return v_res_55_;
}
}
LEAN_EXPORT lean_object* lp_zhc__lean_Zhc_Iops_HillisSteele_solve(lean_object* v_hi_56_, lean_object* v_lo_57_){
_start:
{
lean_object* v___x_58_; uint8_t v___x_59_; 
v___x_58_ = lean_unsigned_to_nat(1u);
v___x_59_ = lean_nat_dec_eq(v_hi_56_, v___x_58_);
if (v___x_59_ == 0)
{
lean_inc(v_hi_56_);
return v_hi_56_;
}
else
{
lean_inc(v_lo_57_);
return v_lo_57_;
}
}
}
LEAN_EXPORT lean_object* lp_zhc__lean_Zhc_Iops_HillisSteele_solve___boxed(lean_object* v_hi_60_, lean_object* v_lo_61_){
_start:
{
lean_object* v_res_62_; 
v_res_62_ = lp_zhc__lean_Zhc_Iops_HillisSteele_solve(v_hi_60_, v_lo_61_);
lean_dec(v_lo_61_);
lean_dec(v_hi_60_);
return v_res_62_;
}
}
LEAN_EXPORT lean_object* lp_zhc__lean_Zhc_Iops_HillisSteele_weightedStatus(lean_object* v_x_63_, lean_object* v_y_64_, lean_object* v_lo_65_, lean_object* v_x_66_){
_start:
{
lean_object* v_zero_67_; uint8_t v_isZero_68_; 
v_zero_67_ = lean_unsigned_to_nat(0u);
v_isZero_68_ = lean_nat_dec_eq(v_x_66_, v_zero_67_);
if (v_isZero_68_ == 1)
{
return v_zero_67_;
}
else
{
lean_object* v_one_69_; lean_object* v_n_70_; lean_object* v___x_71_; lean_object* v___x_72_; lean_object* v___x_73_; lean_object* v___x_74_; lean_object* v___x_75_; lean_object* v___x_76_; lean_object* v___x_77_; 
v_one_69_ = lean_unsigned_to_nat(1u);
v_n_70_ = lean_nat_sub(v_x_66_, v_one_69_);
v___x_71_ = lp_zhc__lean_Zhc_Iops_HillisSteele_weightedStatus(v_x_63_, v_y_64_, v_lo_65_, v_n_70_);
v___x_72_ = lean_nat_add(v_lo_65_, v_n_70_);
v___x_73_ = lp_zhc__lean_Zhc_Iops_HillisSteele_windowStatus(v_x_63_, v_y_64_, v___x_72_, v_one_69_);
lean_dec(v___x_72_);
v___x_74_ = lean_unsigned_to_nat(2u);
v___x_75_ = lean_nat_pow(v___x_74_, v_n_70_);
lean_dec(v_n_70_);
v___x_76_ = lean_nat_mul(v___x_73_, v___x_75_);
lean_dec(v___x_75_);
lean_dec(v___x_73_);
v___x_77_ = lean_nat_add(v___x_71_, v___x_76_);
lean_dec(v___x_76_);
lean_dec(v___x_71_);
return v___x_77_;
}
}
}
LEAN_EXPORT lean_object* lp_zhc__lean_Zhc_Iops_HillisSteele_weightedStatus___boxed(lean_object* v_x_78_, lean_object* v_y_79_, lean_object* v_lo_80_, lean_object* v_x_81_){
_start:
{
lean_object* v_res_82_; 
v_res_82_ = lp_zhc__lean_Zhc_Iops_HillisSteele_weightedStatus(v_x_78_, v_y_79_, v_lo_80_, v_x_81_);
lean_dec(v_x_81_);
lean_dec(v_lo_80_);
lean_dec(v_y_79_);
lean_dec(v_x_78_);
return v_res_82_;
}
}
LEAN_EXPORT lean_object* lp_zhc__lean___private_Zhc_Iops_AddHillisSteele_Theorems_0__Zhc_Iops_HillisSteele_weightedStatus_match__1_splitter___redArg(lean_object* v_x_83_, lean_object* v_h__1_84_, lean_object* v_h__2_85_){
_start:
{
lean_object* v_zero_86_; uint8_t v_isZero_87_; 
v_zero_86_ = lean_unsigned_to_nat(0u);
v_isZero_87_ = lean_nat_dec_eq(v_x_83_, v_zero_86_);
if (v_isZero_87_ == 1)
{
lean_object* v___x_88_; lean_object* v___x_89_; 
lean_dec(v_h__2_85_);
v___x_88_ = lean_box(0);
v___x_89_ = lean_apply_1(v_h__1_84_, v___x_88_);
return v___x_89_;
}
else
{
lean_object* v_one_90_; lean_object* v_n_91_; lean_object* v___x_92_; 
lean_dec(v_h__1_84_);
v_one_90_ = lean_unsigned_to_nat(1u);
v_n_91_ = lean_nat_sub(v_x_83_, v_one_90_);
v___x_92_ = lean_apply_1(v_h__2_85_, v_n_91_);
return v___x_92_;
}
}
}
LEAN_EXPORT lean_object* lp_zhc__lean___private_Zhc_Iops_AddHillisSteele_Theorems_0__Zhc_Iops_HillisSteele_weightedStatus_match__1_splitter___redArg___boxed(lean_object* v_x_93_, lean_object* v_h__1_94_, lean_object* v_h__2_95_){
_start:
{
lean_object* v_res_96_; 
v_res_96_ = lp_zhc__lean___private_Zhc_Iops_AddHillisSteele_Theorems_0__Zhc_Iops_HillisSteele_weightedStatus_match__1_splitter___redArg(v_x_93_, v_h__1_94_, v_h__2_95_);
lean_dec(v_x_93_);
return v_res_96_;
}
}
LEAN_EXPORT lean_object* lp_zhc__lean___private_Zhc_Iops_AddHillisSteele_Theorems_0__Zhc_Iops_HillisSteele_weightedStatus_match__1_splitter(lean_object* v_motive_97_, lean_object* v_x_98_, lean_object* v_h__1_99_, lean_object* v_h__2_100_){
_start:
{
lean_object* v_zero_101_; uint8_t v_isZero_102_; 
v_zero_101_ = lean_unsigned_to_nat(0u);
v_isZero_102_ = lean_nat_dec_eq(v_x_98_, v_zero_101_);
if (v_isZero_102_ == 1)
{
lean_object* v___x_103_; lean_object* v___x_104_; 
lean_dec(v_h__2_100_);
v___x_103_ = lean_box(0);
v___x_104_ = lean_apply_1(v_h__1_99_, v___x_103_);
return v___x_104_;
}
else
{
lean_object* v_one_105_; lean_object* v_n_106_; lean_object* v___x_107_; 
lean_dec(v_h__1_99_);
v_one_105_ = lean_unsigned_to_nat(1u);
v_n_106_ = lean_nat_sub(v_x_98_, v_one_105_);
v___x_107_ = lean_apply_1(v_h__2_100_, v_n_106_);
return v___x_107_;
}
}
}
LEAN_EXPORT lean_object* lp_zhc__lean___private_Zhc_Iops_AddHillisSteele_Theorems_0__Zhc_Iops_HillisSteele_weightedStatus_match__1_splitter___boxed(lean_object* v_motive_108_, lean_object* v_x_109_, lean_object* v_h__1_110_, lean_object* v_h__2_111_){
_start:
{
lean_object* v_res_112_; 
v_res_112_ = lp_zhc__lean___private_Zhc_Iops_AddHillisSteele_Theorems_0__Zhc_Iops_HillisSteele_weightedStatus_match__1_splitter(v_motive_108_, v_x_109_, v_h__1_110_, v_h__2_111_);
lean_dec(v_x_109_);
return v_res_112_;
}
}
LEAN_EXPORT lean_object* lp_zhc__lean_Zhc_Iops_HillisSteele_sel4___redArg(lean_object* v_t_113_, lean_object* v_x_114_){
_start:
{
lean_object* v___x_115_; uint8_t v___x_116_; 
v___x_115_ = lean_unsigned_to_nat(0u);
v___x_116_ = lean_nat_dec_eq(v_x_114_, v___x_115_);
if (v___x_116_ == 0)
{
lean_object* v___x_117_; uint8_t v___x_118_; 
v___x_117_ = lean_unsigned_to_nat(1u);
v___x_118_ = lean_nat_dec_eq(v_x_114_, v___x_117_);
if (v___x_118_ == 0)
{
lean_object* v___x_119_; uint8_t v___x_120_; 
v___x_119_ = lean_unsigned_to_nat(2u);
v___x_120_ = lean_nat_dec_eq(v_x_114_, v___x_119_);
if (v___x_120_ == 0)
{
lean_object* v_snd_121_; lean_object* v_snd_122_; lean_object* v_snd_123_; 
v_snd_121_ = lean_ctor_get(v_t_113_, 1);
v_snd_122_ = lean_ctor_get(v_snd_121_, 1);
v_snd_123_ = lean_ctor_get(v_snd_122_, 1);
lean_inc(v_snd_123_);
return v_snd_123_;
}
else
{
lean_object* v_snd_124_; lean_object* v_snd_125_; lean_object* v_fst_126_; 
v_snd_124_ = lean_ctor_get(v_t_113_, 1);
v_snd_125_ = lean_ctor_get(v_snd_124_, 1);
v_fst_126_ = lean_ctor_get(v_snd_125_, 0);
lean_inc(v_fst_126_);
return v_fst_126_;
}
}
else
{
lean_object* v_snd_127_; lean_object* v_fst_128_; 
v_snd_127_ = lean_ctor_get(v_t_113_, 1);
v_fst_128_ = lean_ctor_get(v_snd_127_, 0);
lean_inc(v_fst_128_);
return v_fst_128_;
}
}
else
{
lean_object* v_fst_129_; 
v_fst_129_ = lean_ctor_get(v_t_113_, 0);
lean_inc(v_fst_129_);
return v_fst_129_;
}
}
}
LEAN_EXPORT lean_object* lp_zhc__lean_Zhc_Iops_HillisSteele_sel4___redArg___boxed(lean_object* v_t_130_, lean_object* v_x_131_){
_start:
{
lean_object* v_res_132_; 
v_res_132_ = lp_zhc__lean_Zhc_Iops_HillisSteele_sel4___redArg(v_t_130_, v_x_131_);
lean_dec(v_x_131_);
lean_dec_ref(v_t_130_);
return v_res_132_;
}
}
LEAN_EXPORT lean_object* lp_zhc__lean_Zhc_Iops_HillisSteele_sel4(lean_object* v_00_u03b1_133_, lean_object* v_t_134_, lean_object* v_x_135_){
_start:
{
lean_object* v___x_136_; 
v___x_136_ = lp_zhc__lean_Zhc_Iops_HillisSteele_sel4___redArg(v_t_134_, v_x_135_);
return v___x_136_;
}
}
LEAN_EXPORT lean_object* lp_zhc__lean_Zhc_Iops_HillisSteele_sel4___boxed(lean_object* v_00_u03b1_137_, lean_object* v_t_138_, lean_object* v_x_139_){
_start:
{
lean_object* v_res_140_; 
v_res_140_ = lp_zhc__lean_Zhc_Iops_HillisSteele_sel4(v_00_u03b1_137_, v_t_138_, v_x_139_);
lean_dec(v_x_139_);
lean_dec_ref(v_t_138_);
return v_res_140_;
}
}
LEAN_EXPORT lean_object* lp_zhc__lean___private_Zhc_Iops_AddHillisSteele_Theorems_0__Zhc_Iops_HillisSteele_chunk4_match__1_splitter___redArg(lean_object* v_x_141_, lean_object* v_h__1_142_, lean_object* v_h__2_143_){
_start:
{
if (lean_obj_tag(v_x_141_) == 1)
{
lean_object* v_tail_144_; 
v_tail_144_ = lean_ctor_get(v_x_141_, 1);
if (lean_obj_tag(v_tail_144_) == 1)
{
lean_object* v_tail_145_; 
v_tail_145_ = lean_ctor_get(v_tail_144_, 1);
if (lean_obj_tag(v_tail_145_) == 1)
{
lean_object* v_tail_146_; 
v_tail_146_ = lean_ctor_get(v_tail_145_, 1);
if (lean_obj_tag(v_tail_146_) == 1)
{
lean_object* v_head_147_; lean_object* v_head_148_; lean_object* v_head_149_; lean_object* v_head_150_; lean_object* v_tail_151_; lean_object* v___x_152_; 
lean_inc_ref(v_tail_146_);
lean_inc_ref(v_tail_145_);
lean_inc_ref(v_tail_144_);
lean_dec(v_h__2_143_);
v_head_147_ = lean_ctor_get(v_x_141_, 0);
lean_inc(v_head_147_);
lean_dec_ref_known(v_x_141_, 2);
v_head_148_ = lean_ctor_get(v_tail_144_, 0);
lean_inc(v_head_148_);
lean_dec_ref_known(v_tail_144_, 2);
v_head_149_ = lean_ctor_get(v_tail_145_, 0);
lean_inc(v_head_149_);
lean_dec_ref_known(v_tail_145_, 2);
v_head_150_ = lean_ctor_get(v_tail_146_, 0);
lean_inc(v_head_150_);
v_tail_151_ = lean_ctor_get(v_tail_146_, 1);
lean_inc(v_tail_151_);
lean_dec_ref_known(v_tail_146_, 2);
v___x_152_ = lean_apply_5(v_h__1_142_, v_head_147_, v_head_148_, v_head_149_, v_head_150_, v_tail_151_);
return v___x_152_;
}
else
{
lean_object* v___x_153_; 
lean_dec(v_h__1_142_);
v___x_153_ = lean_apply_2(v_h__2_143_, v_x_141_, lean_box(0));
return v___x_153_;
}
}
else
{
lean_object* v___x_154_; 
lean_dec(v_h__1_142_);
v___x_154_ = lean_apply_2(v_h__2_143_, v_x_141_, lean_box(0));
return v___x_154_;
}
}
else
{
lean_object* v___x_155_; 
lean_dec(v_h__1_142_);
v___x_155_ = lean_apply_2(v_h__2_143_, v_x_141_, lean_box(0));
return v___x_155_;
}
}
else
{
lean_object* v___x_156_; 
lean_dec(v_h__1_142_);
v___x_156_ = lean_apply_2(v_h__2_143_, v_x_141_, lean_box(0));
return v___x_156_;
}
}
}
LEAN_EXPORT lean_object* lp_zhc__lean___private_Zhc_Iops_AddHillisSteele_Theorems_0__Zhc_Iops_HillisSteele_chunk4_match__1_splitter(lean_object* v_00_u03b1_157_, lean_object* v_motive_158_, lean_object* v_x_159_, lean_object* v_h__1_160_, lean_object* v_h__2_161_){
_start:
{
if (lean_obj_tag(v_x_159_) == 1)
{
lean_object* v_tail_162_; 
v_tail_162_ = lean_ctor_get(v_x_159_, 1);
if (lean_obj_tag(v_tail_162_) == 1)
{
lean_object* v_tail_163_; 
v_tail_163_ = lean_ctor_get(v_tail_162_, 1);
if (lean_obj_tag(v_tail_163_) == 1)
{
lean_object* v_tail_164_; 
v_tail_164_ = lean_ctor_get(v_tail_163_, 1);
if (lean_obj_tag(v_tail_164_) == 1)
{
lean_object* v_head_165_; lean_object* v_head_166_; lean_object* v_head_167_; lean_object* v_head_168_; lean_object* v_tail_169_; lean_object* v___x_170_; 
lean_inc_ref(v_tail_164_);
lean_inc_ref(v_tail_163_);
lean_inc_ref(v_tail_162_);
lean_dec(v_h__2_161_);
v_head_165_ = lean_ctor_get(v_x_159_, 0);
lean_inc(v_head_165_);
lean_dec_ref_known(v_x_159_, 2);
v_head_166_ = lean_ctor_get(v_tail_162_, 0);
lean_inc(v_head_166_);
lean_dec_ref_known(v_tail_162_, 2);
v_head_167_ = lean_ctor_get(v_tail_163_, 0);
lean_inc(v_head_167_);
lean_dec_ref_known(v_tail_163_, 2);
v_head_168_ = lean_ctor_get(v_tail_164_, 0);
lean_inc(v_head_168_);
v_tail_169_ = lean_ctor_get(v_tail_164_, 1);
lean_inc(v_tail_169_);
lean_dec_ref_known(v_tail_164_, 2);
v___x_170_ = lean_apply_5(v_h__1_160_, v_head_165_, v_head_166_, v_head_167_, v_head_168_, v_tail_169_);
return v___x_170_;
}
else
{
lean_object* v___x_171_; 
lean_dec(v_h__1_160_);
v___x_171_ = lean_apply_2(v_h__2_161_, v_x_159_, lean_box(0));
return v___x_171_;
}
}
else
{
lean_object* v___x_172_; 
lean_dec(v_h__1_160_);
v___x_172_ = lean_apply_2(v_h__2_161_, v_x_159_, lean_box(0));
return v___x_172_;
}
}
else
{
lean_object* v___x_173_; 
lean_dec(v_h__1_160_);
v___x_173_ = lean_apply_2(v_h__2_161_, v_x_159_, lean_box(0));
return v___x_173_;
}
}
else
{
lean_object* v___x_174_; 
lean_dec(v_h__1_160_);
v___x_174_ = lean_apply_2(v_h__2_161_, v_x_159_, lean_box(0));
return v___x_174_;
}
}
}
LEAN_EXPORT lean_object* lp_zhc__lean___private_Zhc_Iops_AddHillisSteele_Theorems_0__Zhc_Iops_HillisSteele_unchunk4_match__1_splitter___redArg(lean_object* v_x_175_, lean_object* v_h__1_176_, lean_object* v_h__2_177_){
_start:
{
if (lean_obj_tag(v_x_175_) == 0)
{
lean_object* v___x_178_; lean_object* v___x_179_; 
lean_dec(v_h__2_177_);
v___x_178_ = lean_box(0);
v___x_179_ = lean_apply_1(v_h__1_176_, v___x_178_);
return v___x_179_;
}
else
{
lean_object* v_head_180_; lean_object* v_snd_181_; lean_object* v_snd_182_; lean_object* v_tail_183_; lean_object* v_fst_184_; lean_object* v_fst_185_; lean_object* v_fst_186_; lean_object* v_snd_187_; lean_object* v___x_188_; 
lean_dec(v_h__1_176_);
v_head_180_ = lean_ctor_get(v_x_175_, 0);
lean_inc(v_head_180_);
v_snd_181_ = lean_ctor_get(v_head_180_, 1);
lean_inc(v_snd_181_);
v_snd_182_ = lean_ctor_get(v_snd_181_, 1);
lean_inc(v_snd_182_);
v_tail_183_ = lean_ctor_get(v_x_175_, 1);
lean_inc(v_tail_183_);
lean_dec_ref_known(v_x_175_, 2);
v_fst_184_ = lean_ctor_get(v_head_180_, 0);
lean_inc(v_fst_184_);
lean_dec(v_head_180_);
v_fst_185_ = lean_ctor_get(v_snd_181_, 0);
lean_inc(v_fst_185_);
lean_dec(v_snd_181_);
v_fst_186_ = lean_ctor_get(v_snd_182_, 0);
lean_inc(v_fst_186_);
v_snd_187_ = lean_ctor_get(v_snd_182_, 1);
lean_inc(v_snd_187_);
lean_dec(v_snd_182_);
v___x_188_ = lean_apply_5(v_h__2_177_, v_fst_184_, v_fst_185_, v_fst_186_, v_snd_187_, v_tail_183_);
return v___x_188_;
}
}
}
LEAN_EXPORT lean_object* lp_zhc__lean___private_Zhc_Iops_AddHillisSteele_Theorems_0__Zhc_Iops_HillisSteele_unchunk4_match__1_splitter(lean_object* v_00_u03b1_189_, lean_object* v_motive_190_, lean_object* v_x_191_, lean_object* v_h__1_192_, lean_object* v_h__2_193_){
_start:
{
if (lean_obj_tag(v_x_191_) == 0)
{
lean_object* v___x_194_; lean_object* v___x_195_; 
lean_dec(v_h__2_193_);
v___x_194_ = lean_box(0);
v___x_195_ = lean_apply_1(v_h__1_192_, v___x_194_);
return v___x_195_;
}
else
{
lean_object* v_head_196_; lean_object* v_snd_197_; lean_object* v_snd_198_; lean_object* v_tail_199_; lean_object* v_fst_200_; lean_object* v_fst_201_; lean_object* v_fst_202_; lean_object* v_snd_203_; lean_object* v___x_204_; 
lean_dec(v_h__1_192_);
v_head_196_ = lean_ctor_get(v_x_191_, 0);
lean_inc(v_head_196_);
v_snd_197_ = lean_ctor_get(v_head_196_, 1);
lean_inc(v_snd_197_);
v_snd_198_ = lean_ctor_get(v_snd_197_, 1);
lean_inc(v_snd_198_);
v_tail_199_ = lean_ctor_get(v_x_191_, 1);
lean_inc(v_tail_199_);
lean_dec_ref_known(v_x_191_, 2);
v_fst_200_ = lean_ctor_get(v_head_196_, 0);
lean_inc(v_fst_200_);
lean_dec(v_head_196_);
v_fst_201_ = lean_ctor_get(v_snd_197_, 0);
lean_inc(v_fst_201_);
lean_dec(v_snd_197_);
v_fst_202_ = lean_ctor_get(v_snd_198_, 0);
lean_inc(v_fst_202_);
v_snd_203_ = lean_ctor_get(v_snd_198_, 1);
lean_inc(v_snd_203_);
lean_dec(v_snd_198_);
v___x_204_ = lean_apply_5(v_h__2_193_, v_fst_200_, v_fst_201_, v_fst_202_, v_snd_203_, v_tail_199_);
return v___x_204_;
}
}
}
LEAN_EXPORT lean_object* lp_zhc__lean___private_Zhc_Iops_AddHillisSteele_Theorems_0__Zhc_Iops_HillisSteele_sel4_match__1_splitter___redArg(lean_object* v_x_205_, lean_object* v_h__1_206_, lean_object* v_h__2_207_, lean_object* v_h__3_208_, lean_object* v_h__4_209_){
_start:
{
lean_object* v___x_210_; uint8_t v___x_211_; 
v___x_210_ = lean_unsigned_to_nat(0u);
v___x_211_ = lean_nat_dec_eq(v_x_205_, v___x_210_);
if (v___x_211_ == 0)
{
lean_object* v___x_212_; uint8_t v___x_213_; 
lean_dec(v_h__1_206_);
v___x_212_ = lean_unsigned_to_nat(1u);
v___x_213_ = lean_nat_dec_eq(v_x_205_, v___x_212_);
if (v___x_213_ == 0)
{
lean_object* v___x_214_; uint8_t v___x_215_; 
lean_dec(v_h__2_207_);
v___x_214_ = lean_unsigned_to_nat(2u);
v___x_215_ = lean_nat_dec_eq(v_x_205_, v___x_214_);
if (v___x_215_ == 0)
{
lean_object* v___x_216_; 
lean_dec(v_h__3_208_);
v___x_216_ = lean_apply_4(v_h__4_209_, v_x_205_, lean_box(0), lean_box(0), lean_box(0));
return v___x_216_;
}
else
{
lean_object* v___x_217_; lean_object* v___x_218_; 
lean_dec(v_h__4_209_);
lean_dec(v_x_205_);
v___x_217_ = lean_box(0);
v___x_218_ = lean_apply_1(v_h__3_208_, v___x_217_);
return v___x_218_;
}
}
else
{
lean_object* v___x_219_; lean_object* v___x_220_; 
lean_dec(v_h__4_209_);
lean_dec(v_h__3_208_);
lean_dec(v_x_205_);
v___x_219_ = lean_box(0);
v___x_220_ = lean_apply_1(v_h__2_207_, v___x_219_);
return v___x_220_;
}
}
else
{
lean_object* v___x_221_; lean_object* v___x_222_; 
lean_dec(v_h__4_209_);
lean_dec(v_h__3_208_);
lean_dec(v_h__2_207_);
lean_dec(v_x_205_);
v___x_221_ = lean_box(0);
v___x_222_ = lean_apply_1(v_h__1_206_, v___x_221_);
return v___x_222_;
}
}
}
LEAN_EXPORT lean_object* lp_zhc__lean___private_Zhc_Iops_AddHillisSteele_Theorems_0__Zhc_Iops_HillisSteele_sel4_match__1_splitter(lean_object* v_motive_223_, lean_object* v_x_224_, lean_object* v_h__1_225_, lean_object* v_h__2_226_, lean_object* v_h__3_227_, lean_object* v_h__4_228_){
_start:
{
lean_object* v___x_229_; uint8_t v___x_230_; 
v___x_229_ = lean_unsigned_to_nat(0u);
v___x_230_ = lean_nat_dec_eq(v_x_224_, v___x_229_);
if (v___x_230_ == 0)
{
lean_object* v___x_231_; uint8_t v___x_232_; 
lean_dec(v_h__1_225_);
v___x_231_ = lean_unsigned_to_nat(1u);
v___x_232_ = lean_nat_dec_eq(v_x_224_, v___x_231_);
if (v___x_232_ == 0)
{
lean_object* v___x_233_; uint8_t v___x_234_; 
lean_dec(v_h__2_226_);
v___x_233_ = lean_unsigned_to_nat(2u);
v___x_234_ = lean_nat_dec_eq(v_x_224_, v___x_233_);
if (v___x_234_ == 0)
{
lean_object* v___x_235_; 
lean_dec(v_h__3_227_);
v___x_235_ = lean_apply_4(v_h__4_228_, v_x_224_, lean_box(0), lean_box(0), lean_box(0));
return v___x_235_;
}
else
{
lean_object* v___x_236_; lean_object* v___x_237_; 
lean_dec(v_h__4_228_);
lean_dec(v_x_224_);
v___x_236_ = lean_box(0);
v___x_237_ = lean_apply_1(v_h__3_227_, v___x_236_);
return v___x_237_;
}
}
else
{
lean_object* v___x_238_; lean_object* v___x_239_; 
lean_dec(v_h__4_228_);
lean_dec(v_h__3_227_);
lean_dec(v_x_224_);
v___x_238_ = lean_box(0);
v___x_239_ = lean_apply_1(v_h__2_226_, v___x_238_);
return v___x_239_;
}
}
else
{
lean_object* v___x_240_; lean_object* v___x_241_; 
lean_dec(v_h__4_228_);
lean_dec(v_h__3_227_);
lean_dec(v_h__2_226_);
lean_dec(v_x_224_);
v___x_240_ = lean_box(0);
v___x_241_ = lean_apply_1(v_h__1_225_, v___x_240_);
return v___x_241_;
}
}
}
LEAN_EXPORT lean_object* lp_zhc__lean___private_Zhc_Iops_AddHillisSteele_Theorems_0__Zhc_Iops_HillisSteele_finalResolution__run_match__1_splitter___redArg(lean_object* v_x_242_, lean_object* v_h__1_243_, lean_object* v_h__2_244_){
_start:
{
if (lean_obj_tag(v_x_242_) == 0)
{
lean_object* v___x_245_; lean_object* v___x_246_; 
lean_dec(v_h__2_244_);
v___x_245_ = lean_box(0);
v___x_246_ = lean_apply_1(v_h__1_243_, v___x_245_);
return v___x_246_;
}
else
{
lean_object* v_val_247_; lean_object* v___x_248_; 
lean_dec(v_h__1_243_);
v_val_247_ = lean_ctor_get(v_x_242_, 0);
lean_inc(v_val_247_);
lean_dec_ref_known(v_x_242_, 1);
v___x_248_ = lean_apply_1(v_h__2_244_, v_val_247_);
return v___x_248_;
}
}
}
LEAN_EXPORT lean_object* lp_zhc__lean___private_Zhc_Iops_AddHillisSteele_Theorems_0__Zhc_Iops_HillisSteele_finalResolution__run_match__1_splitter(lean_object* v_motive_249_, lean_object* v_x_250_, lean_object* v_h__1_251_, lean_object* v_h__2_252_){
_start:
{
if (lean_obj_tag(v_x_250_) == 0)
{
lean_object* v___x_253_; lean_object* v___x_254_; 
lean_dec(v_h__2_252_);
v___x_253_ = lean_box(0);
v___x_254_ = lean_apply_1(v_h__1_251_, v___x_253_);
return v___x_254_;
}
else
{
lean_object* v_val_255_; lean_object* v___x_256_; 
lean_dec(v_h__1_251_);
v_val_255_ = lean_ctor_get(v_x_250_, 0);
lean_inc(v_val_255_);
lean_dec_ref_known(v_x_250_, 1);
v___x_256_ = lean_apply_1(v_h__2_252_, v_val_255_);
return v___x_256_;
}
}
}
LEAN_EXPORT lean_object* lp_zhc__lean___private_Zhc_Iops_AddHillisSteele_Theorems_0__Zhc_Iops_HillisSteele_propagate_match__3_splitter___redArg(lean_object* v_sums_257_, lean_object* v_h__1_258_, lean_object* v_h__2_259_){
_start:
{
if (lean_obj_tag(v_sums_257_) == 0)
{
lean_object* v___x_260_; lean_object* v___x_261_; 
lean_dec(v_h__2_259_);
v___x_260_ = lean_box(0);
v___x_261_ = lean_apply_1(v_h__1_258_, v___x_260_);
return v___x_261_;
}
else
{
lean_object* v_head_262_; lean_object* v_tail_263_; lean_object* v___x_264_; 
lean_dec(v_h__1_258_);
v_head_262_ = lean_ctor_get(v_sums_257_, 0);
lean_inc(v_head_262_);
v_tail_263_ = lean_ctor_get(v_sums_257_, 1);
lean_inc(v_tail_263_);
lean_dec_ref_known(v_sums_257_, 2);
v___x_264_ = lean_apply_2(v_h__2_259_, v_head_262_, v_tail_263_);
return v___x_264_;
}
}
}
LEAN_EXPORT lean_object* lp_zhc__lean___private_Zhc_Iops_AddHillisSteele_Theorems_0__Zhc_Iops_HillisSteele_propagate_match__3_splitter(lean_object* v_M_265_, lean_object* v_inst_266_, lean_object* v_A_267_, lean_object* v_motive_268_, lean_object* v_sums_269_, lean_object* v_h__1_270_, lean_object* v_h__2_271_){
_start:
{
if (lean_obj_tag(v_sums_269_) == 0)
{
lean_object* v___x_272_; lean_object* v___x_273_; 
lean_dec(v_h__2_271_);
v___x_272_ = lean_box(0);
v___x_273_ = lean_apply_1(v_h__1_270_, v___x_272_);
return v___x_273_;
}
else
{
lean_object* v_head_274_; lean_object* v_tail_275_; lean_object* v___x_276_; 
lean_dec(v_h__1_270_);
v_head_274_ = lean_ctor_get(v_sums_269_, 0);
lean_inc(v_head_274_);
v_tail_275_ = lean_ctor_get(v_sums_269_, 1);
lean_inc(v_tail_275_);
lean_dec_ref_known(v_sums_269_, 2);
v___x_276_ = lean_apply_2(v_h__2_271_, v_head_274_, v_tail_275_);
return v___x_276_;
}
}
}
LEAN_EXPORT lean_object* lp_zhc__lean___private_Zhc_Iops_AddHillisSteele_Theorems_0__Zhc_Iops_HillisSteele_propagate_match__3_splitter___boxed(lean_object* v_M_277_, lean_object* v_inst_278_, lean_object* v_A_279_, lean_object* v_motive_280_, lean_object* v_sums_281_, lean_object* v_h__1_282_, lean_object* v_h__2_283_){
_start:
{
lean_object* v_res_284_; 
v_res_284_ = lp_zhc__lean___private_Zhc_Iops_AddHillisSteele_Theorems_0__Zhc_Iops_HillisSteele_propagate_match__3_splitter(v_M_277_, v_inst_278_, v_A_279_, v_motive_280_, v_sums_281_, v_h__1_282_, v_h__2_283_);
lean_dec_ref(v_A_279_);
lean_dec_ref(v_inst_278_);
return v_res_284_;
}
}
static lean_object* _init_lp_zhc__lean_Zhc_Iops_HillisSteele___aux__Zhc__Iops__AddHillisSteele__Theorems______macroRules__Zhc__Iops__HillisSteele__tacticProjects__step__1___closed__20(void){
_start:
{
lean_object* v___x_345_; lean_object* v___x_346_; 
v___x_345_ = ((lean_object*)(lp_zhc__lean_Zhc_Iops_HillisSteele___aux__Zhc__Iops__AddHillisSteele__Theorems______macroRules__Zhc__Iops__HillisSteele__tacticProjects__step__1___closed__19));
v___x_346_ = l_String_toRawSubstring_x27(v___x_345_);
return v___x_346_;
}
}
static lean_object* _init_lp_zhc__lean_Zhc_Iops_HillisSteele___aux__Zhc__Iops__AddHillisSteele__Theorems______macroRules__Zhc__Iops__HillisSteele__tacticProjects__step__1___closed__41(void){
_start:
{
lean_object* v___x_394_; lean_object* v___x_395_; 
v___x_394_ = ((lean_object*)(lp_zhc__lean_Zhc_Iops_HillisSteele___aux__Zhc__Iops__AddHillisSteele__Theorems______macroRules__Zhc__Iops__HillisSteele__tacticProjects__step__1___closed__40));
v___x_395_ = l_String_toRawSubstring_x27(v___x_394_);
return v___x_395_;
}
}
static lean_object* _init_lp_zhc__lean_Zhc_Iops_HillisSteele___aux__Zhc__Iops__AddHillisSteele__Theorems______macroRules__Zhc__Iops__HillisSteele__tacticProjects__step__1___closed__50(void){
_start:
{
lean_object* v___x_419_; lean_object* v___x_420_; 
v___x_419_ = ((lean_object*)(lp_zhc__lean_Zhc_Iops_HillisSteele___aux__Zhc__Iops__AddHillisSteele__Theorems______macroRules__Zhc__Iops__HillisSteele__tacticProjects__step__1___closed__49));
v___x_420_ = l_String_toRawSubstring_x27(v___x_419_);
return v___x_420_;
}
}
static lean_object* _init_lp_zhc__lean_Zhc_Iops_HillisSteele___aux__Zhc__Iops__AddHillisSteele__Theorems______macroRules__Zhc__Iops__HillisSteele__tacticProjects__step__1___closed__54(void){
_start:
{
lean_object* v___x_425_; lean_object* v___x_426_; 
v___x_425_ = ((lean_object*)(lp_zhc__lean_Zhc_Iops_HillisSteele___aux__Zhc__Iops__AddHillisSteele__Theorems______macroRules__Zhc__Iops__HillisSteele__tacticProjects__step__1___closed__53));
v___x_426_ = l_String_toRawSubstring_x27(v___x_425_);
return v___x_426_;
}
}
static lean_object* _init_lp_zhc__lean_Zhc_Iops_HillisSteele___aux__Zhc__Iops__AddHillisSteele__Theorems______macroRules__Zhc__Iops__HillisSteele__tacticProjects__step__1___closed__68(void){
_start:
{
lean_object* v___x_462_; 
v___x_462_ = l_Array_mkArray0(lean_box(0));
return v___x_462_;
}
}
LEAN_EXPORT lean_object* lp_zhc__lean_Zhc_Iops_HillisSteele___aux__Zhc__Iops__AddHillisSteele__Theorems______macroRules__Zhc__Iops__HillisSteele__tacticProjects__step__1(lean_object* v_x_470_, lean_object* v_a_471_, lean_object* v_a_472_){
_start:
{
lean_object* v___x_473_; uint8_t v___x_474_; 
v___x_473_ = ((lean_object*)(lp_zhc__lean_Zhc_Iops_HillisSteele_tacticProjects__step___closed__4));
v___x_474_ = l_Lean_Syntax_isOfKind(v_x_470_, v___x_473_);
if (v___x_474_ == 0)
{
lean_object* v___x_475_; lean_object* v___x_476_; 
v___x_475_ = lean_box(1);
v___x_476_ = lean_alloc_ctor(1, 2, 0);
lean_ctor_set(v___x_476_, 0, v___x_475_);
lean_ctor_set(v___x_476_, 1, v_a_472_);
return v___x_476_;
}
else
{
lean_object* v_quotContext_477_; lean_object* v_currMacroScope_478_; lean_object* v_ref_479_; uint8_t v___x_480_; lean_object* v___x_481_; lean_object* v___x_482_; lean_object* v___x_483_; lean_object* v___x_484_; lean_object* v___x_485_; lean_object* v___x_486_; lean_object* v___x_487_; lean_object* v___x_488_; lean_object* v___x_489_; lean_object* v___x_490_; lean_object* v___x_491_; lean_object* v___x_492_; lean_object* v___x_493_; lean_object* v___x_494_; lean_object* v___x_495_; lean_object* v___x_496_; lean_object* v___x_497_; lean_object* v___x_498_; lean_object* v___x_499_; lean_object* v___x_500_; lean_object* v___x_501_; lean_object* v___x_502_; lean_object* v___x_503_; lean_object* v___x_504_; lean_object* v___x_505_; lean_object* v___x_506_; lean_object* v___x_507_; lean_object* v___x_508_; lean_object* v___x_509_; lean_object* v___x_510_; lean_object* v___x_511_; lean_object* v___x_512_; lean_object* v___x_513_; lean_object* v___x_514_; lean_object* v___x_515_; lean_object* v___x_516_; lean_object* v___x_517_; lean_object* v___x_518_; lean_object* v___x_519_; lean_object* v___x_520_; lean_object* v___x_521_; lean_object* v___x_522_; lean_object* v___x_523_; lean_object* v___x_524_; lean_object* v___x_525_; lean_object* v___x_526_; lean_object* v___x_527_; lean_object* v___x_528_; lean_object* v___x_529_; lean_object* v___x_530_; lean_object* v___x_531_; lean_object* v___x_532_; lean_object* v___x_533_; lean_object* v___x_534_; lean_object* v___x_535_; lean_object* v___x_536_; lean_object* v___x_537_; lean_object* v___x_538_; lean_object* v___x_539_; lean_object* v___x_540_; lean_object* v___x_541_; lean_object* v___x_542_; lean_object* v___x_543_; lean_object* v___x_544_; lean_object* v___x_545_; lean_object* v___x_546_; lean_object* v___x_547_; lean_object* v___x_548_; lean_object* v___x_549_; lean_object* v___x_550_; lean_object* v___x_551_; lean_object* v___x_552_; lean_object* v___x_553_; lean_object* v___x_554_; lean_object* v___x_555_; lean_object* v___x_556_; lean_object* v___x_557_; lean_object* v___x_558_; lean_object* v___x_559_; lean_object* v___x_560_; lean_object* v___x_561_; lean_object* v___x_562_; lean_object* v___x_563_; lean_object* v___x_564_; lean_object* v___x_565_; lean_object* v___x_566_; lean_object* v___x_567_; lean_object* v___x_568_; lean_object* v___x_569_; lean_object* v___x_570_; lean_object* v___x_571_; lean_object* v___x_572_; lean_object* v___x_573_; lean_object* v___x_574_; lean_object* v___x_575_; lean_object* v___x_576_; lean_object* v___x_577_; lean_object* v___x_578_; lean_object* v___x_579_; lean_object* v___x_580_; lean_object* v___x_581_; 
v_quotContext_477_ = lean_ctor_get(v_a_471_, 1);
v_currMacroScope_478_ = lean_ctor_get(v_a_471_, 2);
v_ref_479_ = lean_ctor_get(v_a_471_, 5);
v___x_480_ = 0;
v___x_481_ = l_Lean_SourceInfo_fromRef(v_ref_479_, v___x_480_);
v___x_482_ = ((lean_object*)(lp_zhc__lean_Zhc_Iops_HillisSteele___aux__Zhc__Iops__AddHillisSteele__Theorems______macroRules__Zhc__Iops__HillisSteele__tacticProjects__step__1___closed__3));
v___x_483_ = ((lean_object*)(lp_zhc__lean_Zhc_Iops_HillisSteele___aux__Zhc__Iops__AddHillisSteele__Theorems______macroRules__Zhc__Iops__HillisSteele__tacticProjects__step__1___closed__4));
lean_inc_n(v___x_481_, 51);
v___x_484_ = lean_alloc_ctor(2, 2, 0);
lean_ctor_set(v___x_484_, 0, v___x_481_);
lean_ctor_set(v___x_484_, 1, v___x_482_);
v___x_485_ = ((lean_object*)(lp_zhc__lean_Zhc_Iops_HillisSteele___aux__Zhc__Iops__AddHillisSteele__Theorems______macroRules__Zhc__Iops__HillisSteele__tacticProjects__step__1___closed__6));
v___x_486_ = ((lean_object*)(lp_zhc__lean_Zhc_Iops_HillisSteele___aux__Zhc__Iops__AddHillisSteele__Theorems______macroRules__Zhc__Iops__HillisSteele__tacticProjects__step__1___closed__8));
v___x_487_ = ((lean_object*)(lp_zhc__lean_Zhc_Iops_HillisSteele___aux__Zhc__Iops__AddHillisSteele__Theorems______macroRules__Zhc__Iops__HillisSteele__tacticProjects__step__1___closed__9));
v___x_488_ = lean_alloc_ctor(2, 2, 0);
lean_ctor_set(v___x_488_, 0, v___x_481_);
lean_ctor_set(v___x_488_, 1, v___x_487_);
v___x_489_ = ((lean_object*)(lp_zhc__lean_Zhc_Iops_HillisSteele___aux__Zhc__Iops__AddHillisSteele__Theorems______macroRules__Zhc__Iops__HillisSteele__tacticProjects__step__1___closed__11));
v___x_490_ = ((lean_object*)(lp_zhc__lean_Zhc_Iops_HillisSteele___aux__Zhc__Iops__AddHillisSteele__Theorems______macroRules__Zhc__Iops__HillisSteele__tacticProjects__step__1___closed__13));
v___x_491_ = ((lean_object*)(lp_zhc__lean_Zhc_Iops_HillisSteele___aux__Zhc__Iops__AddHillisSteele__Theorems______macroRules__Zhc__Iops__HillisSteele__tacticProjects__step__1___closed__14));
v___x_492_ = ((lean_object*)(lp_zhc__lean_Zhc_Iops_HillisSteele___aux__Zhc__Iops__AddHillisSteele__Theorems______macroRules__Zhc__Iops__HillisSteele__tacticProjects__step__1___closed__15));
v___x_493_ = lean_alloc_ctor(2, 2, 0);
lean_ctor_set(v___x_493_, 0, v___x_481_);
lean_ctor_set(v___x_493_, 1, v___x_491_);
v___x_494_ = ((lean_object*)(lp_zhc__lean_Zhc_Iops_HillisSteele___aux__Zhc__Iops__AddHillisSteele__Theorems______macroRules__Zhc__Iops__HillisSteele__tacticProjects__step__1___closed__18));
v___x_495_ = lean_obj_once(&lp_zhc__lean_Zhc_Iops_HillisSteele___aux__Zhc__Iops__AddHillisSteele__Theorems______macroRules__Zhc__Iops__HillisSteele__tacticProjects__step__1___closed__20, &lp_zhc__lean_Zhc_Iops_HillisSteele___aux__Zhc__Iops__AddHillisSteele__Theorems______macroRules__Zhc__Iops__HillisSteele__tacticProjects__step__1___closed__20_once, _init_lp_zhc__lean_Zhc_Iops_HillisSteele___aux__Zhc__Iops__AddHillisSteele__Theorems______macroRules__Zhc__Iops__HillisSteele__tacticProjects__step__1___closed__20);
v___x_496_ = ((lean_object*)(lp_zhc__lean_Zhc_Iops_HillisSteele___aux__Zhc__Iops__AddHillisSteele__Theorems______macroRules__Zhc__Iops__HillisSteele__tacticProjects__step__1___closed__23));
lean_inc_n(v_currMacroScope_478_, 4);
lean_inc_n(v_quotContext_477_, 4);
v___x_497_ = l_Lean_addMacroScope(v_quotContext_477_, v___x_496_, v_currMacroScope_478_);
v___x_498_ = lean_box(0);
v___x_499_ = ((lean_object*)(lp_zhc__lean_Zhc_Iops_HillisSteele___aux__Zhc__Iops__AddHillisSteele__Theorems______macroRules__Zhc__Iops__HillisSteele__tacticProjects__step__1___closed__28));
v___x_500_ = lean_alloc_ctor(3, 4, 0);
lean_ctor_set(v___x_500_, 0, v___x_481_);
lean_ctor_set(v___x_500_, 1, v___x_495_);
lean_ctor_set(v___x_500_, 2, v___x_497_);
lean_ctor_set(v___x_500_, 3, v___x_499_);
v___x_501_ = ((lean_object*)(lp_zhc__lean_Zhc_Iops_HillisSteele___aux__Zhc__Iops__AddHillisSteele__Theorems______macroRules__Zhc__Iops__HillisSteele__tacticProjects__step__1___closed__30));
v___x_502_ = ((lean_object*)(lp_zhc__lean_Zhc_Iops_HillisSteele___aux__Zhc__Iops__AddHillisSteele__Theorems______macroRules__Zhc__Iops__HillisSteele__tacticProjects__step__1___closed__31));
v___x_503_ = lean_alloc_ctor(2, 2, 0);
lean_ctor_set(v___x_503_, 0, v___x_481_);
lean_ctor_set(v___x_503_, 1, v___x_502_);
lean_inc_ref(v___x_503_);
v___x_504_ = l_Lean_Syntax_node1(v___x_481_, v___x_501_, v___x_503_);
lean_inc_n(v___x_504_, 2);
v___x_505_ = l_Lean_Syntax_node2(v___x_481_, v___x_485_, v___x_504_, v___x_504_);
v___x_506_ = l_Lean_Syntax_node2(v___x_481_, v___x_494_, v___x_500_, v___x_505_);
v___x_507_ = l_Lean_Syntax_node2(v___x_481_, v___x_492_, v___x_493_, v___x_506_);
v___x_508_ = l_Lean_Syntax_node1(v___x_481_, v___x_485_, v___x_507_);
v___x_509_ = l_Lean_Syntax_node1(v___x_481_, v___x_490_, v___x_508_);
v___x_510_ = l_Lean_Syntax_node1(v___x_481_, v___x_489_, v___x_509_);
lean_inc_ref_n(v___x_488_, 2);
v___x_511_ = l_Lean_Syntax_node2(v___x_481_, v___x_486_, v___x_488_, v___x_510_);
v___x_512_ = ((lean_object*)(lp_zhc__lean_Zhc_Iops_HillisSteele___aux__Zhc__Iops__AddHillisSteele__Theorems______macroRules__Zhc__Iops__HillisSteele__tacticProjects__step__1___closed__33));
v___x_513_ = ((lean_object*)(lp_zhc__lean_Zhc_Iops_HillisSteele___aux__Zhc__Iops__AddHillisSteele__Theorems______macroRules__Zhc__Iops__HillisSteele__tacticProjects__step__1___closed__34));
v___x_514_ = lean_alloc_ctor(2, 2, 0);
lean_ctor_set(v___x_514_, 0, v___x_481_);
lean_ctor_set(v___x_514_, 1, v___x_513_);
v___x_515_ = l_Lean_Syntax_node1(v___x_481_, v___x_512_, v___x_514_);
v___x_516_ = l_Lean_Syntax_node1(v___x_481_, v___x_485_, v___x_515_);
v___x_517_ = l_Lean_Syntax_node1(v___x_481_, v___x_490_, v___x_516_);
v___x_518_ = l_Lean_Syntax_node1(v___x_481_, v___x_489_, v___x_517_);
lean_inc(v___x_518_);
v___x_519_ = l_Lean_Syntax_node2(v___x_481_, v___x_486_, v___x_488_, v___x_518_);
v___x_520_ = ((lean_object*)(lp_zhc__lean_Zhc_Iops_HillisSteele___aux__Zhc__Iops__AddHillisSteele__Theorems______macroRules__Zhc__Iops__HillisSteele__tacticProjects__step__1___closed__36));
v___x_521_ = ((lean_object*)(lp_zhc__lean_Zhc_Iops_HillisSteele___aux__Zhc__Iops__AddHillisSteele__Theorems______macroRules__Zhc__Iops__HillisSteele__tacticProjects__step__1___closed__37));
v___x_522_ = lean_alloc_ctor(2, 2, 0);
lean_ctor_set(v___x_522_, 0, v___x_481_);
lean_ctor_set(v___x_522_, 1, v___x_521_);
v___x_523_ = ((lean_object*)(lp_zhc__lean_Zhc_Iops_HillisSteele___aux__Zhc__Iops__AddHillisSteele__Theorems______macroRules__Zhc__Iops__HillisSteele__tacticProjects__step__1___closed__38));
v___x_524_ = ((lean_object*)(lp_zhc__lean_Zhc_Iops_HillisSteele___aux__Zhc__Iops__AddHillisSteele__Theorems______macroRules__Zhc__Iops__HillisSteele__tacticProjects__step__1___closed__39));
v___x_525_ = lean_alloc_ctor(2, 2, 0);
lean_ctor_set(v___x_525_, 0, v___x_481_);
lean_ctor_set(v___x_525_, 1, v___x_523_);
v___x_526_ = lean_obj_once(&lp_zhc__lean_Zhc_Iops_HillisSteele___aux__Zhc__Iops__AddHillisSteele__Theorems______macroRules__Zhc__Iops__HillisSteele__tacticProjects__step__1___closed__41, &lp_zhc__lean_Zhc_Iops_HillisSteele___aux__Zhc__Iops__AddHillisSteele__Theorems______macroRules__Zhc__Iops__HillisSteele__tacticProjects__step__1___closed__41_once, _init_lp_zhc__lean_Zhc_Iops_HillisSteele___aux__Zhc__Iops__AddHillisSteele__Theorems______macroRules__Zhc__Iops__HillisSteele__tacticProjects__step__1___closed__41);
v___x_527_ = ((lean_object*)(lp_zhc__lean_Zhc_Iops_HillisSteele___aux__Zhc__Iops__AddHillisSteele__Theorems______macroRules__Zhc__Iops__HillisSteele__tacticProjects__step__1___closed__43));
v___x_528_ = l_Lean_addMacroScope(v_quotContext_477_, v___x_527_, v_currMacroScope_478_);
v___x_529_ = ((lean_object*)(lp_zhc__lean_Zhc_Iops_HillisSteele___aux__Zhc__Iops__AddHillisSteele__Theorems______macroRules__Zhc__Iops__HillisSteele__tacticProjects__step__1___closed__46));
v___x_530_ = lean_alloc_ctor(3, 4, 0);
lean_ctor_set(v___x_530_, 0, v___x_481_);
lean_ctor_set(v___x_530_, 1, v___x_526_);
lean_ctor_set(v___x_530_, 2, v___x_528_);
lean_ctor_set(v___x_530_, 3, v___x_529_);
v___x_531_ = ((lean_object*)(lp_zhc__lean_Zhc_Iops_HillisSteele___aux__Zhc__Iops__AddHillisSteele__Theorems______macroRules__Zhc__Iops__HillisSteele__tacticProjects__step__1___closed__48));
v___x_532_ = lean_obj_once(&lp_zhc__lean_Zhc_Iops_HillisSteele___aux__Zhc__Iops__AddHillisSteele__Theorems______macroRules__Zhc__Iops__HillisSteele__tacticProjects__step__1___closed__50, &lp_zhc__lean_Zhc_Iops_HillisSteele___aux__Zhc__Iops__AddHillisSteele__Theorems______macroRules__Zhc__Iops__HillisSteele__tacticProjects__step__1___closed__50_once, _init_lp_zhc__lean_Zhc_Iops_HillisSteele___aux__Zhc__Iops__AddHillisSteele__Theorems______macroRules__Zhc__Iops__HillisSteele__tacticProjects__step__1___closed__50);
v___x_533_ = ((lean_object*)(lp_zhc__lean_Zhc_Iops_HillisSteele___aux__Zhc__Iops__AddHillisSteele__Theorems______macroRules__Zhc__Iops__HillisSteele__tacticProjects__step__1___closed__51));
v___x_534_ = l_Lean_addMacroScope(v_quotContext_477_, v___x_533_, v_currMacroScope_478_);
v___x_535_ = lean_alloc_ctor(3, 4, 0);
lean_ctor_set(v___x_535_, 0, v___x_481_);
lean_ctor_set(v___x_535_, 1, v___x_532_);
lean_ctor_set(v___x_535_, 2, v___x_534_);
lean_ctor_set(v___x_535_, 3, v___x_498_);
v___x_536_ = ((lean_object*)(lp_zhc__lean_Zhc_Iops_HillisSteele___aux__Zhc__Iops__AddHillisSteele__Theorems______macroRules__Zhc__Iops__HillisSteele__tacticProjects__step__1___closed__52));
v___x_537_ = lean_alloc_ctor(2, 2, 0);
lean_ctor_set(v___x_537_, 0, v___x_481_);
lean_ctor_set(v___x_537_, 1, v___x_536_);
v___x_538_ = lean_obj_once(&lp_zhc__lean_Zhc_Iops_HillisSteele___aux__Zhc__Iops__AddHillisSteele__Theorems______macroRules__Zhc__Iops__HillisSteele__tacticProjects__step__1___closed__54, &lp_zhc__lean_Zhc_Iops_HillisSteele___aux__Zhc__Iops__AddHillisSteele__Theorems______macroRules__Zhc__Iops__HillisSteele__tacticProjects__step__1___closed__54_once, _init_lp_zhc__lean_Zhc_Iops_HillisSteele___aux__Zhc__Iops__AddHillisSteele__Theorems______macroRules__Zhc__Iops__HillisSteele__tacticProjects__step__1___closed__54);
v___x_539_ = ((lean_object*)(lp_zhc__lean_Zhc_Iops_HillisSteele___aux__Zhc__Iops__AddHillisSteele__Theorems______macroRules__Zhc__Iops__HillisSteele__tacticProjects__step__1___closed__55));
v___x_540_ = l_Lean_addMacroScope(v_quotContext_477_, v___x_539_, v_currMacroScope_478_);
v___x_541_ = ((lean_object*)(lp_zhc__lean_Zhc_Iops_HillisSteele___aux__Zhc__Iops__AddHillisSteele__Theorems______macroRules__Zhc__Iops__HillisSteele__tacticProjects__step__1___closed__59));
v___x_542_ = lean_alloc_ctor(3, 4, 0);
lean_ctor_set(v___x_542_, 0, v___x_481_);
lean_ctor_set(v___x_542_, 1, v___x_538_);
lean_ctor_set(v___x_542_, 2, v___x_540_);
lean_ctor_set(v___x_542_, 3, v___x_541_);
v___x_543_ = ((lean_object*)(lp_zhc__lean_Zhc_Iops_HillisSteele___aux__Zhc__Iops__AddHillisSteele__Theorems______macroRules__Zhc__Iops__HillisSteele__tacticProjects__step__1___closed__60));
v___x_544_ = lean_alloc_ctor(2, 2, 0);
lean_ctor_set(v___x_544_, 0, v___x_481_);
lean_ctor_set(v___x_544_, 1, v___x_543_);
lean_inc_ref(v___x_544_);
lean_inc_ref(v___x_522_);
v___x_545_ = l_Lean_Syntax_node5(v___x_481_, v___x_531_, v___x_522_, v___x_535_, v___x_537_, v___x_542_, v___x_544_);
v___x_546_ = ((lean_object*)(lp_zhc__lean_Zhc_Iops_HillisSteele___aux__Zhc__Iops__AddHillisSteele__Theorems______macroRules__Zhc__Iops__HillisSteele__tacticProjects__step__1___closed__62));
v___x_547_ = ((lean_object*)(lp_zhc__lean_Zhc_Iops_HillisSteele___aux__Zhc__Iops__AddHillisSteele__Theorems______macroRules__Zhc__Iops__HillisSteele__tacticProjects__step__1___closed__63));
v___x_548_ = lean_alloc_ctor(2, 2, 0);
lean_ctor_set(v___x_548_, 0, v___x_481_);
lean_ctor_set(v___x_548_, 1, v___x_547_);
v___x_549_ = l_Lean_Syntax_node2(v___x_481_, v___x_546_, v___x_548_, v___x_503_);
v___x_550_ = ((lean_object*)(lp_zhc__lean_Zhc_Iops_HillisSteele___aux__Zhc__Iops__AddHillisSteele__Theorems______macroRules__Zhc__Iops__HillisSteele__tacticProjects__step__1___closed__64));
v___x_551_ = ((lean_object*)(lp_zhc__lean_Zhc_Iops_HillisSteele___aux__Zhc__Iops__AddHillisSteele__Theorems______macroRules__Zhc__Iops__HillisSteele__tacticProjects__step__1___closed__65));
v___x_552_ = lean_alloc_ctor(2, 2, 0);
lean_ctor_set(v___x_552_, 0, v___x_481_);
lean_ctor_set(v___x_552_, 1, v___x_550_);
v___x_553_ = ((lean_object*)(lp_zhc__lean_Zhc_Iops_HillisSteele___aux__Zhc__Iops__AddHillisSteele__Theorems______macroRules__Zhc__Iops__HillisSteele__tacticProjects__step__1___closed__67));
v___x_554_ = l_Lean_Syntax_node1(v___x_481_, v___x_485_, v___x_504_);
v___x_555_ = lean_obj_once(&lp_zhc__lean_Zhc_Iops_HillisSteele___aux__Zhc__Iops__AddHillisSteele__Theorems______macroRules__Zhc__Iops__HillisSteele__tacticProjects__step__1___closed__68, &lp_zhc__lean_Zhc_Iops_HillisSteele___aux__Zhc__Iops__AddHillisSteele__Theorems______macroRules__Zhc__Iops__HillisSteele__tacticProjects__step__1___closed__68_once, _init_lp_zhc__lean_Zhc_Iops_HillisSteele___aux__Zhc__Iops__AddHillisSteele__Theorems______macroRules__Zhc__Iops__HillisSteele__tacticProjects__step__1___closed__68);
v___x_556_ = lean_alloc_ctor(1, 3, 0);
lean_ctor_set(v___x_556_, 0, v___x_481_);
lean_ctor_set(v___x_556_, 1, v___x_485_);
lean_ctor_set(v___x_556_, 2, v___x_555_);
v___x_557_ = ((lean_object*)(lp_zhc__lean_Zhc_Iops_HillisSteele___aux__Zhc__Iops__AddHillisSteele__Theorems______macroRules__Zhc__Iops__HillisSteele__tacticProjects__step__1___closed__69));
v___x_558_ = lean_alloc_ctor(2, 2, 0);
lean_ctor_set(v___x_558_, 0, v___x_481_);
lean_ctor_set(v___x_558_, 1, v___x_557_);
lean_inc(v___x_549_);
lean_inc_ref_n(v___x_556_, 2);
v___x_559_ = l_Lean_Syntax_node4(v___x_481_, v___x_553_, v___x_554_, v___x_556_, v___x_558_, v___x_549_);
v___x_560_ = l_Lean_Syntax_node2(v___x_481_, v___x_551_, v___x_552_, v___x_559_);
v___x_561_ = l_Lean_Syntax_node3(v___x_481_, v___x_485_, v___x_545_, v___x_549_, v___x_560_);
v___x_562_ = l_Lean_Syntax_node2(v___x_481_, v___x_494_, v___x_530_, v___x_561_);
v___x_563_ = l_Lean_Syntax_node2(v___x_481_, v___x_524_, v___x_525_, v___x_562_);
v___x_564_ = ((lean_object*)(lp_zhc__lean_Zhc_Iops_HillisSteele___aux__Zhc__Iops__AddHillisSteele__Theorems______macroRules__Zhc__Iops__HillisSteele__tacticProjects__step__1___closed__70));
v___x_565_ = ((lean_object*)(lp_zhc__lean_Zhc_Iops_HillisSteele___aux__Zhc__Iops__AddHillisSteele__Theorems______macroRules__Zhc__Iops__HillisSteele__tacticProjects__step__1___closed__71));
v___x_566_ = lean_alloc_ctor(2, 2, 0);
lean_ctor_set(v___x_566_, 0, v___x_481_);
lean_ctor_set(v___x_566_, 1, v___x_564_);
v___x_567_ = l_Lean_Syntax_node2(v___x_481_, v___x_565_, v___x_566_, v___x_518_);
v___x_568_ = ((lean_object*)(lp_zhc__lean_Zhc_Iops_HillisSteele_tacticProjects__step___closed__5));
v___x_569_ = lean_alloc_ctor(2, 2, 0);
lean_ctor_set(v___x_569_, 0, v___x_481_);
lean_ctor_set(v___x_569_, 1, v___x_568_);
v___x_570_ = l_Lean_Syntax_node1(v___x_481_, v___x_473_, v___x_569_);
v___x_571_ = l_Lean_Syntax_node5(v___x_481_, v___x_485_, v___x_563_, v___x_556_, v___x_567_, v___x_556_, v___x_570_);
v___x_572_ = l_Lean_Syntax_node1(v___x_481_, v___x_490_, v___x_571_);
v___x_573_ = l_Lean_Syntax_node1(v___x_481_, v___x_489_, v___x_572_);
v___x_574_ = l_Lean_Syntax_node3(v___x_481_, v___x_520_, v___x_522_, v___x_573_, v___x_544_);
v___x_575_ = l_Lean_Syntax_node1(v___x_481_, v___x_485_, v___x_574_);
v___x_576_ = l_Lean_Syntax_node1(v___x_481_, v___x_490_, v___x_575_);
v___x_577_ = l_Lean_Syntax_node1(v___x_481_, v___x_489_, v___x_576_);
v___x_578_ = l_Lean_Syntax_node2(v___x_481_, v___x_486_, v___x_488_, v___x_577_);
v___x_579_ = l_Lean_Syntax_node3(v___x_481_, v___x_485_, v___x_511_, v___x_519_, v___x_578_);
v___x_580_ = l_Lean_Syntax_node2(v___x_481_, v___x_483_, v___x_484_, v___x_579_);
v___x_581_ = lean_alloc_ctor(0, 2, 0);
lean_ctor_set(v___x_581_, 0, v___x_580_);
lean_ctor_set(v___x_581_, 1, v_a_472_);
return v___x_581_;
}
}
}
LEAN_EXPORT lean_object* lp_zhc__lean_Zhc_Iops_HillisSteele___aux__Zhc__Iops__AddHillisSteele__Theorems______macroRules__Zhc__Iops__HillisSteele__tacticProjects__step__1___boxed(lean_object* v_x_582_, lean_object* v_a_583_, lean_object* v_a_584_){
_start:
{
lean_object* v_res_585_; 
v_res_585_ = lp_zhc__lean_Zhc_Iops_HillisSteele___aux__Zhc__Iops__AddHillisSteele__Theorems______macroRules__Zhc__Iops__HillisSteele__tacticProjects__step__1(v_x_582_, v_a_583_, v_a_584_);
lean_dec_ref(v_a_583_);
return v_res_585_;
}
}
LEAN_EXPORT lean_object* lp_zhc__lean___private_Zhc_Iops_AddHillisSteele_Theorems_0__Zhc_Iops_HillisSteele_finalResolution_match__1_splitter___redArg(lean_object* v_prev_586_, lean_object* v_h__1_587_, lean_object* v_h__2_588_){
_start:
{
if (lean_obj_tag(v_prev_586_) == 0)
{
lean_object* v___x_589_; lean_object* v___x_590_; 
lean_dec(v_h__2_588_);
v___x_589_ = lean_box(0);
v___x_590_ = lean_apply_1(v_h__1_587_, v___x_589_);
return v___x_590_;
}
else
{
lean_object* v_val_591_; lean_object* v___x_592_; 
lean_dec(v_h__1_587_);
v_val_591_ = lean_ctor_get(v_prev_586_, 0);
lean_inc(v_val_591_);
lean_dec_ref_known(v_prev_586_, 1);
v___x_592_ = lean_apply_1(v_h__2_588_, v_val_591_);
return v___x_592_;
}
}
}
LEAN_EXPORT lean_object* lp_zhc__lean___private_Zhc_Iops_AddHillisSteele_Theorems_0__Zhc_Iops_HillisSteele_finalResolution_match__1_splitter(lean_object* v_M_593_, lean_object* v_inst_594_, lean_object* v_A_595_, lean_object* v_motive_596_, lean_object* v_prev_597_, lean_object* v_h__1_598_, lean_object* v_h__2_599_){
_start:
{
if (lean_obj_tag(v_prev_597_) == 0)
{
lean_object* v___x_600_; lean_object* v___x_601_; 
lean_dec(v_h__2_599_);
v___x_600_ = lean_box(0);
v___x_601_ = lean_apply_1(v_h__1_598_, v___x_600_);
return v___x_601_;
}
else
{
lean_object* v_val_602_; lean_object* v___x_603_; 
lean_dec(v_h__1_598_);
v_val_602_ = lean_ctor_get(v_prev_597_, 0);
lean_inc(v_val_602_);
lean_dec_ref_known(v_prev_597_, 1);
v___x_603_ = lean_apply_1(v_h__2_599_, v_val_602_);
return v___x_603_;
}
}
}
LEAN_EXPORT lean_object* lp_zhc__lean___private_Zhc_Iops_AddHillisSteele_Theorems_0__Zhc_Iops_HillisSteele_finalResolution_match__1_splitter___boxed(lean_object* v_M_604_, lean_object* v_inst_605_, lean_object* v_A_606_, lean_object* v_motive_607_, lean_object* v_prev_608_, lean_object* v_h__1_609_, lean_object* v_h__2_610_){
_start:
{
lean_object* v_res_611_; 
v_res_611_ = lp_zhc__lean___private_Zhc_Iops_AddHillisSteele_Theorems_0__Zhc_Iops_HillisSteele_finalResolution_match__1_splitter(v_M_604_, v_inst_605_, v_A_606_, v_motive_607_, v_prev_608_, v_h__1_609_, v_h__2_610_);
lean_dec_ref(v_A_606_);
lean_dec_ref(v_inst_605_);
return v_res_611_;
}
}
lean_object* initialize_Init(uint8_t builtin);
lean_object* initialize_Init(uint8_t builtin);
lean_object* initialize_zhc__lean_Zhc_BlockAlgebra_Model_Theorems(uint8_t builtin);
lean_object* initialize_zhc__lean_Zhc_BlockAlgebra_Tracer_Theorems(uint8_t builtin);
lean_object* initialize_zhc__lean_Zhc_Iops_AddRippleCarry_Theorems(uint8_t builtin);
lean_object* initialize_zhc__lean_Zhc_Iops_AddHillisSteele_Impl(uint8_t builtin);
void lean_initialize_runtime_module();
static bool _G_initialized = false;
LEAN_EXPORT lean_object* initialize_zhc__lean_Zhc_Iops_AddHillisSteele_Theorems(uint8_t builtin) {
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
res = initialize_zhc__lean_Zhc_BlockAlgebra_Model_Theorems(builtin);
if (lean_io_result_is_error(res)) return res;
lean_dec_ref(res);
res = initialize_zhc__lean_Zhc_BlockAlgebra_Tracer_Theorems(builtin);
if (lean_io_result_is_error(res)) return res;
lean_dec_ref(res);
res = initialize_zhc__lean_Zhc_Iops_AddRippleCarry_Theorems(builtin);
if (lean_io_result_is_error(res)) return res;
lean_dec_ref(res);
res = initialize_zhc__lean_Zhc_Iops_AddHillisSteele_Impl(builtin);
if (lean_io_result_is_error(res)) return res;
lean_dec_ref(res);
return lean_io_result_mk_ok(lean_box(0));
}
#ifdef __cplusplus
}
#endif
