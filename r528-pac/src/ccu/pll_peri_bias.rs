#[doc = "Register `PLL_PERI_BIAS` reader"]
pub struct R(crate::R<PLL_PERI_BIAS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PLL_PERI_BIAS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PLL_PERI_BIAS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PLL_PERI_BIAS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PLL_PERI_BIAS` writer"]
pub struct W(crate::W<PLL_PERI_BIAS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PLL_PERI_BIAS_SPEC>;
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
impl From<crate::W<PLL_PERI_BIAS_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PLL_PERI_BIAS_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PLL_CP` reader - PLL current bias control"]
pub type PLL_CP_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PLL_CP` writer - PLL current bias control"]
pub type PLL_CP_W<'a> = crate::FieldWriter<'a, u32, PLL_PERI_BIAS_SPEC, u8, u8, 5, 16>;
impl R {
    #[doc = "Bits 16:20 - PLL current bias control"]
    #[inline(always)]
    pub fn pll_cp(&self) -> PLL_CP_R {
        PLL_CP_R::new(((self.bits >> 16) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 16:20 - PLL current bias control"]
    #[inline(always)]
    pub fn pll_cp(&mut self) -> PLL_CP_W {
        PLL_CP_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "PLL_PERI Bias Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pll_peri_bias](index.html) module"]
pub struct PLL_PERI_BIAS_SPEC;
impl crate::RegisterSpec for PLL_PERI_BIAS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pll_peri_bias::R](R) reader structure"]
impl crate::Readable for PLL_PERI_BIAS_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pll_peri_bias::W](W) writer structure"]
impl crate::Writable for PLL_PERI_BIAS_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PLL_PERI_BIAS to value 0"]
impl crate::Resettable for PLL_PERI_BIAS_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
