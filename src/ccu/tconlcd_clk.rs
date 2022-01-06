#[doc = "Register `TCONLCD_CLK` reader"]
pub struct R(crate::R<TCONLCD_CLK_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TCONLCD_CLK_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TCONLCD_CLK_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TCONLCD_CLK_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TCONLCD_CLK` writer"]
pub struct W(crate::W<TCONLCD_CLK_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TCONLCD_CLK_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<TCONLCD_CLK_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TCONLCD_CLK_SPEC>) -> Self {
        W(writer)
    }
}
impl W {
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "TCONLCD Clock Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tconlcd_clk](index.html) module"]
pub struct TCONLCD_CLK_SPEC;
impl crate::RegisterSpec for TCONLCD_CLK_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [tconlcd_clk::R](R) reader structure"]
impl crate::Readable for TCONLCD_CLK_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [tconlcd_clk::W](W) writer structure"]
impl crate::Writable for TCONLCD_CLK_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets TCONLCD_CLK to value 0"]
impl crate::Resettable for TCONLCD_CLK_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}