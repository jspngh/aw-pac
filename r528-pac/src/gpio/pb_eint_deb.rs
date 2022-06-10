#[doc = "Register `pb_eint_deb` reader"]
pub struct R(crate::R<PB_EINT_DEB_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PB_EINT_DEB_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PB_EINT_DEB_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PB_EINT_DEB_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `pb_eint_deb` writer"]
pub struct W(crate::W<PB_EINT_DEB_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PB_EINT_DEB_SPEC>;
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
impl From<crate::W<PB_EINT_DEB_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PB_EINT_DEB_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DEB_CLK_PRE_SCALE` reader - Debounce Clock Pre_scale n"]
pub type DEB_CLK_PRE_SCALE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DEB_CLK_PRE_SCALE` writer - Debounce Clock Pre_scale n"]
pub type DEB_CLK_PRE_SCALE_W<'a> = crate::FieldWriter<'a, u32, PB_EINT_DEB_SPEC, u8, u8, 3, 4>;
#[doc = "PIO Interrupt Clock Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PIO_INT_CLK_SELECT_A {
    #[doc = "0: `0`"]
    LOSC_32KHZ = 0,
    #[doc = "1: `1`"]
    HOSC_24MHZ = 1,
}
impl From<PIO_INT_CLK_SELECT_A> for bool {
    #[inline(always)]
    fn from(variant: PIO_INT_CLK_SELECT_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PIO_INT_CLK_SELECT` reader - PIO Interrupt Clock Select"]
pub type PIO_INT_CLK_SELECT_R = crate::BitReader<PIO_INT_CLK_SELECT_A>;
impl PIO_INT_CLK_SELECT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PIO_INT_CLK_SELECT_A {
        match self.bits {
            false => PIO_INT_CLK_SELECT_A::LOSC_32KHZ,
            true => PIO_INT_CLK_SELECT_A::HOSC_24MHZ,
        }
    }
    #[doc = "Checks if the value of the field is `LOSC_32KHZ`"]
    #[inline(always)]
    pub fn is_losc_32khz(&self) -> bool {
        *self == PIO_INT_CLK_SELECT_A::LOSC_32KHZ
    }
    #[doc = "Checks if the value of the field is `HOSC_24MHZ`"]
    #[inline(always)]
    pub fn is_hosc_24mhz(&self) -> bool {
        *self == PIO_INT_CLK_SELECT_A::HOSC_24MHZ
    }
}
#[doc = "Field `PIO_INT_CLK_SELECT` writer - PIO Interrupt Clock Select"]
pub type PIO_INT_CLK_SELECT_W<'a> =
    crate::BitWriter<'a, u32, PB_EINT_DEB_SPEC, PIO_INT_CLK_SELECT_A, 0>;
impl<'a> PIO_INT_CLK_SELECT_W<'a> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn losc_32khz(self) -> &'a mut W {
        self.variant(PIO_INT_CLK_SELECT_A::LOSC_32KHZ)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn hosc_24mhz(self) -> &'a mut W {
        self.variant(PIO_INT_CLK_SELECT_A::HOSC_24MHZ)
    }
}
impl R {
    #[doc = "Bits 4:6 - Debounce Clock Pre_scale n"]
    #[inline(always)]
    pub fn deb_clk_pre_scale(&self) -> DEB_CLK_PRE_SCALE_R {
        DEB_CLK_PRE_SCALE_R::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bit 0 - PIO Interrupt Clock Select"]
    #[inline(always)]
    pub fn pio_int_clk_select(&self) -> PIO_INT_CLK_SELECT_R {
        PIO_INT_CLK_SELECT_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 4:6 - Debounce Clock Pre_scale n"]
    #[inline(always)]
    pub fn deb_clk_pre_scale(&mut self) -> DEB_CLK_PRE_SCALE_W {
        DEB_CLK_PRE_SCALE_W::new(self)
    }
    #[doc = "Bit 0 - PIO Interrupt Clock Select"]
    #[inline(always)]
    pub fn pio_int_clk_select(&mut self) -> PIO_INT_CLK_SELECT_W {
        PIO_INT_CLK_SELECT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "PB External Interrupt Debounce Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pb_eint_deb](index.html) module"]
pub struct PB_EINT_DEB_SPEC;
impl crate::RegisterSpec for PB_EINT_DEB_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pb_eint_deb::R](R) reader structure"]
impl crate::Readable for PB_EINT_DEB_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pb_eint_deb::W](W) writer structure"]
impl crate::Writable for PB_EINT_DEB_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets pb_eint_deb to value 0"]
impl crate::Resettable for PB_EINT_DEB_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
