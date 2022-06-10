#[doc = "Register `CIR_IDC_L` reader"]
pub struct R(crate::R<CIR_IDC_L_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CIR_IDC_L_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CIR_IDC_L_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CIR_IDC_L_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CIR_IDC_L` writer"]
pub struct W(crate::W<CIR_IDC_L_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CIR_IDC_L_SPEC>;
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
impl From<crate::W<CIR_IDC_L_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CIR_IDC_L_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `IDC_L` reader - Idle Duration Counter Threshold (Low 8 bits)\n\nIdle Duration = 128*IDC*Ts (IDC = 0-4095)\n\nIt is used in cyclical transmission mode. When all the data in FIFO is transmitted, the signals can be transmitted after a specific time."]
pub type IDC_L_R = crate::FieldReader<u8, u8>;
#[doc = "Field `IDC_L` writer - Idle Duration Counter Threshold (Low 8 bits)\n\nIdle Duration = 128*IDC*Ts (IDC = 0-4095)\n\nIt is used in cyclical transmission mode. When all the data in FIFO is transmitted, the signals can be transmitted after a specific time."]
pub type IDC_L_W<'a> = crate::FieldWriter<'a, u32, CIR_IDC_L_SPEC, u8, u8, 8, 0>;
impl R {
    #[doc = "Bits 0:7 - Idle Duration Counter Threshold (Low 8 bits)\n\nIdle Duration = 128*IDC*Ts (IDC = 0-4095)\n\nIt is used in cyclical transmission mode. When all the data in FIFO is transmitted, the signals can be transmitted after a specific time."]
    #[inline(always)]
    pub fn idc_l(&self) -> IDC_L_R {
        IDC_L_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Idle Duration Counter Threshold (Low 8 bits)\n\nIdle Duration = 128*IDC*Ts (IDC = 0-4095)\n\nIt is used in cyclical transmission mode. When all the data in FIFO is transmitted, the signals can be transmitted after a specific time."]
    #[inline(always)]
    pub fn idc_l(&mut self) -> IDC_L_W {
        IDC_L_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "CIR Transmit Idle Duration Threshold Low Bit Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cir_idc_l](index.html) module"]
pub struct CIR_IDC_L_SPEC;
impl crate::RegisterSpec for CIR_IDC_L_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cir_idc_l::R](R) reader structure"]
impl crate::Readable for CIR_IDC_L_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cir_idc_l::W](W) writer structure"]
impl crate::Writable for CIR_IDC_L_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CIR_IDC_L to value 0"]
impl crate::Resettable for CIR_IDC_L_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
