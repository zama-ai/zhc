#include "aig/gia/gia.h"
#include "proof/cec/cec.h"

ABC_NAMESPACE_IMPL_START

Gia_Man_t *zhc_abc_gia_start(int n_objs_max) {
    Gia_Man_t *p = Gia_ManStart(n_objs_max);
    Gia_ManHashAlloc(p);
    return p;
}

void zhc_abc_gia_stop(Gia_Man_t *p) {
    Gia_ManHashStop(p);
    Gia_ManStop(p);
}

int zhc_abc_gia_append_ci(Gia_Man_t *p) {
    return Gia_ManAppendCi(p);
}

int zhc_abc_gia_hash_and(Gia_Man_t *p, int lit0, int lit1) {
    return Gia_ManHashAnd(p, lit0, lit1);
}

int zhc_abc_gia_append_co(Gia_Man_t *p, int lit) {
    return Gia_ManAppendCo(p, lit);
}

int zhc_abc_gia_ci_num(Gia_Man_t *p) {
    return Gia_ManCiNum(p);
}

int zhc_abc_gia_co_num(Gia_Man_t *p) {
    return Gia_ManCoNum(p);
}

int zhc_abc_gia_and_num(Gia_Man_t *p) {
    return Gia_ManAndNum(p);
}

int zhc_abc_gia_obj_num(Gia_Man_t *p) {
    return Gia_ManObjNum(p);
}

int zhc_abc_gia_obj_kind(Gia_Man_t *p, int id) {
    Gia_Obj_t *obj = Gia_ManObj(p, id);
    if (Gia_ObjIsConst0(obj)) {
        return 0;
    }
    if (Gia_ObjIsCi(obj)) {
        return 1;
    }
    if (Gia_ObjIsAnd(obj)) {
        return 2;
    }
    if (Gia_ObjIsCo(obj)) {
        return 3;
    }
    return -1;
}

int zhc_abc_gia_obj_cio_id(Gia_Man_t *p, int id) {
    return Gia_ObjCioId(Gia_ManObj(p, id));
}

int zhc_abc_gia_obj_fanin0_lit(Gia_Man_t *p, int id) {
    return Gia_ObjFaninLit0(Gia_ManObj(p, id), id);
}

int zhc_abc_gia_obj_fanin1_lit(Gia_Man_t *p, int id) {
    return Gia_ObjFaninLit1(Gia_ManObj(p, id), id);
}

int zhc_abc_cec_verify(
    Gia_Man_t *p0,
    Gia_Man_t *p1,
    int conflict_limit,
    int time_limit,
    int verbose,
    int *out_fail,
    unsigned char *cex_inputs,
    int cex_len
) {
    Cec_ParCec_t pars;
    Gia_Man_t *miter;
    int ret;
    int i;

    Cec_ManCecSetDefaultParams(&pars);
    pars.nBTLimit = conflict_limit;
    pars.TimeLimit = time_limit;
    pars.fVerbose = verbose;
    pars.fSilent = !verbose;

    miter = Gia_ManMiter(p0, p1, 0, 1, 0, 0, verbose);
    if (miter == NULL) {
        return -2;
    }
    ret = Cec_ManVerify(miter, &pars);
    *out_fail = -1;
    if (ret == 0 && miter->pCexComb != NULL) {
        Abc_Cex_t *cex = miter->pCexComb;
        *out_fail = pars.iOutFail >= 0 ? pars.iOutFail : cex->iPo;
        for (i = 0; i < cex_len && i < cex->nPis; i++) {
            cex_inputs[i] = (unsigned char)Abc_InfoHasBit(cex->pData, i);
        }
    }
    Gia_ManStop(miter);
    return ret;
}

ABC_NAMESPACE_IMPL_END
