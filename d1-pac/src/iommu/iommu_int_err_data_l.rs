#[doc = "Register `iommu_int_err_data_l%s` reader"]
pub struct R(crate::R<IOMMU_INT_ERR_DATA_L_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<IOMMU_INT_ERR_DATA_L_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<IOMMU_INT_ERR_DATA_L_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<IOMMU_INT_ERR_DATA_L_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `int_err_data` reader - Corresponding page table of virtual address that caused L\\[i\\] page table to interrupt"]
pub type INT_ERR_DATA_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - Corresponding page table of virtual address that caused L\\[i\\] page table to interrupt"]
    #[inline(always)]
    pub fn int_err_data(&self) -> INT_ERR_DATA_R {
        INT_ERR_DATA_R::new(self.bits)
    }
}
#[doc = "IOMMU Interrupt Error Data L\\[i\\] Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [iommu_int_err_data_l](index.html) module"]
pub struct IOMMU_INT_ERR_DATA_L_SPEC;
impl crate::RegisterSpec for IOMMU_INT_ERR_DATA_L_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [iommu_int_err_data_l::R](R) reader structure"]
impl crate::Readable for IOMMU_INT_ERR_DATA_L_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets iommu_int_err_data_l%s to value 0"]
impl crate::Resettable for IOMMU_INT_ERR_DATA_L_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
