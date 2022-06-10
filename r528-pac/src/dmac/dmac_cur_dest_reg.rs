#[doc = "Register `DMAC_CUR_DEST_REG%s` reader"]
pub struct R(crate::R<DMAC_CUR_DEST_REG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DMAC_CUR_DEST_REG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DMAC_CUR_DEST_REG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DMAC_CUR_DEST_REG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "DMAC Channel Current Destination Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dmac_cur_dest_reg](index.html) module"]
pub struct DMAC_CUR_DEST_REG_SPEC;
impl crate::RegisterSpec for DMAC_CUR_DEST_REG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dmac_cur_dest_reg::R](R) reader structure"]
impl crate::Readable for DMAC_CUR_DEST_REG_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets DMAC_CUR_DEST_REG%s to value 0"]
impl crate::Resettable for DMAC_CUR_DEST_REG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
