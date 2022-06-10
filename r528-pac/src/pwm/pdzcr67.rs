#[doc = "Register `PDZCR67` reader"]
pub struct R(crate::R<PDZCR67_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PDZCR67_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PDZCR67_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PDZCR67_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PDZCR67` writer"]
pub struct W(crate::W<PDZCR67_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PDZCR67_SPEC>;
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
impl From<crate::W<PDZCR67_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PDZCR67_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PWM67_DZ_INTV` reader - PWM67 Dead Zone Interval Value"]
pub type PWM67_DZ_INTV_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PWM67_DZ_INTV` writer - PWM67 Dead Zone Interval Value"]
pub type PWM67_DZ_INTV_W<'a> = crate::FieldWriter<'a, u32, PDZCR67_SPEC, u8, u8, 8, 8>;
#[doc = "PWM67 Dead Zone Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PWM67_DZ_EN_A {
    #[doc = "0: Dead Zone disable"]
    DISABLE = 0,
    #[doc = "1: Dead Zone enable"]
    ENABLE = 1,
}
impl From<PWM67_DZ_EN_A> for bool {
    #[inline(always)]
    fn from(variant: PWM67_DZ_EN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PWM67_DZ_EN` reader - PWM67 Dead Zone Enable"]
pub type PWM67_DZ_EN_R = crate::BitReader<PWM67_DZ_EN_A>;
impl PWM67_DZ_EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PWM67_DZ_EN_A {
        match self.bits {
            false => PWM67_DZ_EN_A::DISABLE,
            true => PWM67_DZ_EN_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == PWM67_DZ_EN_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == PWM67_DZ_EN_A::ENABLE
    }
}
#[doc = "Field `PWM67_DZ_EN` writer - PWM67 Dead Zone Enable"]
pub type PWM67_DZ_EN_W<'a> = crate::BitWriter<'a, u32, PDZCR67_SPEC, PWM67_DZ_EN_A, 0>;
impl<'a> PWM67_DZ_EN_W<'a> {
    #[doc = "Dead Zone disable"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(PWM67_DZ_EN_A::DISABLE)
    }
    #[doc = "Dead Zone enable"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(PWM67_DZ_EN_A::ENABLE)
    }
}
impl R {
    #[doc = "Bits 8:15 - PWM67 Dead Zone Interval Value"]
    #[inline(always)]
    pub fn pwm67_dz_intv(&self) -> PWM67_DZ_INTV_R {
        PWM67_DZ_INTV_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bit 0 - PWM67 Dead Zone Enable"]
    #[inline(always)]
    pub fn pwm67_dz_en(&self) -> PWM67_DZ_EN_R {
        PWM67_DZ_EN_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 8:15 - PWM67 Dead Zone Interval Value"]
    #[inline(always)]
    pub fn pwm67_dz_intv(&mut self) -> PWM67_DZ_INTV_W {
        PWM67_DZ_INTV_W::new(self)
    }
    #[doc = "Bit 0 - PWM67 Dead Zone Enable"]
    #[inline(always)]
    pub fn pwm67_dz_en(&mut self) -> PWM67_DZ_EN_W {
        PWM67_DZ_EN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "PWM67 Dead Zone Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pdzcr67](index.html) module"]
pub struct PDZCR67_SPEC;
impl crate::RegisterSpec for PDZCR67_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pdzcr67::R](R) reader structure"]
impl crate::Readable for PDZCR67_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pdzcr67::W](W) writer structure"]
impl crate::Writable for PDZCR67_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PDZCR67 to value 0"]
impl crate::Resettable for PDZCR67_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
