#[doc = "Register `smhc_resp2` reader"]
pub struct R(crate::R<SMHC_RESP2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SMHC_RESP2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SMHC_RESP2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SMHC_RESP2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Response 2 Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [smhc_resp2](index.html) module"]
pub struct SMHC_RESP2_SPEC;
impl crate::RegisterSpec for SMHC_RESP2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [smhc_resp2::R](R) reader structure"]
impl crate::Readable for SMHC_RESP2_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets smhc_resp2 to value 0"]
impl crate::Resettable for SMHC_RESP2_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
