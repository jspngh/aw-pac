#[doc = "Register `EMAC_EPHY_CLK0` reader"]
pub struct R(crate::R<EMAC_EPHY_CLK0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<EMAC_EPHY_CLK0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<EMAC_EPHY_CLK0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<EMAC_EPHY_CLK0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `EMAC_EPHY_CLK0` writer"]
pub struct W(crate::W<EMAC_EPHY_CLK0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<EMAC_EPHY_CLK0_SPEC>;
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
impl From<crate::W<EMAC_EPHY_CLK0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<EMAC_EPHY_CLK0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `BPS_EFUSE` reader - "]
pub type BPS_EFUSE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `BPS_EFUSE` writer - "]
pub type BPS_EFUSE_W<'a> = crate::FieldWriter<'a, u32, EMAC_EPHY_CLK0_SPEC, u8, u8, 4, 28>;
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum XMII_SEL_A {
    #[doc = "0: `0`"]
    INTERNAL = 0,
    #[doc = "1: `1`"]
    EXTERNAL = 1,
}
impl From<XMII_SEL_A> for bool {
    #[inline(always)]
    fn from(variant: XMII_SEL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `XMII_SEL` reader - "]
pub type XMII_SEL_R = crate::BitReader<XMII_SEL_A>;
impl XMII_SEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> XMII_SEL_A {
        match self.bits {
            false => XMII_SEL_A::INTERNAL,
            true => XMII_SEL_A::EXTERNAL,
        }
    }
    #[doc = "Checks if the value of the field is `INTERNAL`"]
    #[inline(always)]
    pub fn is_internal(&self) -> bool {
        *self == XMII_SEL_A::INTERNAL
    }
    #[doc = "Checks if the value of the field is `EXTERNAL`"]
    #[inline(always)]
    pub fn is_external(&self) -> bool {
        *self == XMII_SEL_A::EXTERNAL
    }
}
#[doc = "Field `XMII_SEL` writer - "]
pub type XMII_SEL_W<'a> = crate::BitWriter<'a, u32, EMAC_EPHY_CLK0_SPEC, XMII_SEL_A, 27>;
impl<'a> XMII_SEL_W<'a> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn internal(self) -> &'a mut W {
        self.variant(XMII_SEL_A::INTERNAL)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn external(self) -> &'a mut W {
        self.variant(XMII_SEL_A::EXTERNAL)
    }
}
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum EPHY_MODE_A {
    #[doc = "0: `0`"]
    NORMAL = 0,
    #[doc = "1: `1`"]
    SIMULATION = 1,
    #[doc = "2: `10`"]
    AFE_TEST = 2,
}
impl From<EPHY_MODE_A> for u8 {
    #[inline(always)]
    fn from(variant: EPHY_MODE_A) -> Self {
        variant as _
    }
}
#[doc = "Field `EPHY_MODE` reader - "]
pub type EPHY_MODE_R = crate::FieldReader<u8, EPHY_MODE_A>;
impl EPHY_MODE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<EPHY_MODE_A> {
        match self.bits {
            0 => Some(EPHY_MODE_A::NORMAL),
            1 => Some(EPHY_MODE_A::SIMULATION),
            2 => Some(EPHY_MODE_A::AFE_TEST),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `NORMAL`"]
    #[inline(always)]
    pub fn is_normal(&self) -> bool {
        *self == EPHY_MODE_A::NORMAL
    }
    #[doc = "Checks if the value of the field is `SIMULATION`"]
    #[inline(always)]
    pub fn is_simulation(&self) -> bool {
        *self == EPHY_MODE_A::SIMULATION
    }
    #[doc = "Checks if the value of the field is `AFE_TEST`"]
    #[inline(always)]
    pub fn is_afe_test(&self) -> bool {
        *self == EPHY_MODE_A::AFE_TEST
    }
}
#[doc = "Field `EPHY_MODE` writer - "]
pub type EPHY_MODE_W<'a> = crate::FieldWriter<'a, u32, EMAC_EPHY_CLK0_SPEC, u8, EPHY_MODE_A, 2, 25>;
impl<'a> EPHY_MODE_W<'a> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn normal(self) -> &'a mut W {
        self.variant(EPHY_MODE_A::NORMAL)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn simulation(self) -> &'a mut W {
        self.variant(EPHY_MODE_A::SIMULATION)
    }
    #[doc = "`10`"]
    #[inline(always)]
    pub fn afe_test(self) -> &'a mut W {
        self.variant(EPHY_MODE_A::AFE_TEST)
    }
}
#[doc = "Field `PHY_ADDR` reader - "]
pub type PHY_ADDR_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PHY_ADDR` writer - "]
pub type PHY_ADDR_W<'a> = crate::FieldWriter<'a, u32, EMAC_EPHY_CLK0_SPEC, u8, u8, 5, 20>;
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CLK_SEL_A {
    #[doc = "0: `0`"]
    F25M = 0,
    #[doc = "1: `1`"]
    F24M = 1,
}
impl From<CLK_SEL_A> for bool {
    #[inline(always)]
    fn from(variant: CLK_SEL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CLK_SEL` reader - "]
pub type CLK_SEL_R = crate::BitReader<CLK_SEL_A>;
impl CLK_SEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CLK_SEL_A {
        match self.bits {
            false => CLK_SEL_A::F25M,
            true => CLK_SEL_A::F24M,
        }
    }
    #[doc = "Checks if the value of the field is `F25M`"]
    #[inline(always)]
    pub fn is_f25m(&self) -> bool {
        *self == CLK_SEL_A::F25M
    }
    #[doc = "Checks if the value of the field is `F24M`"]
    #[inline(always)]
    pub fn is_f24m(&self) -> bool {
        *self == CLK_SEL_A::F24M
    }
}
#[doc = "Field `CLK_SEL` writer - "]
pub type CLK_SEL_W<'a> = crate::BitWriter<'a, u32, EMAC_EPHY_CLK0_SPEC, CLK_SEL_A, 18>;
impl<'a> CLK_SEL_W<'a> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn f25m(self) -> &'a mut W {
        self.variant(CLK_SEL_A::F25M)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn f24m(self) -> &'a mut W {
        self.variant(CLK_SEL_A::F24M)
    }
}
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LED_POL_A {
    #[doc = "0: `0`"]
    HIGH = 0,
    #[doc = "1: `1`"]
    LOW = 1,
}
impl From<LED_POL_A> for bool {
    #[inline(always)]
    fn from(variant: LED_POL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LED_POL` reader - "]
pub type LED_POL_R = crate::BitReader<LED_POL_A>;
impl LED_POL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LED_POL_A {
        match self.bits {
            false => LED_POL_A::HIGH,
            true => LED_POL_A::LOW,
        }
    }
    #[doc = "Checks if the value of the field is `HIGH`"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == LED_POL_A::HIGH
    }
    #[doc = "Checks if the value of the field is `LOW`"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == LED_POL_A::LOW
    }
}
#[doc = "Field `LED_POL` writer - "]
pub type LED_POL_W<'a> = crate::BitWriter<'a, u32, EMAC_EPHY_CLK0_SPEC, LED_POL_A, 17>;
impl<'a> LED_POL_W<'a> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn high(self) -> &'a mut W {
        self.variant(LED_POL_A::HIGH)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn low(self) -> &'a mut W {
        self.variant(LED_POL_A::LOW)
    }
}
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SHUTDOWN_A {
    #[doc = "0: `0`"]
    POWER_UP = 0,
    #[doc = "1: `1`"]
    SHUT_DOWN = 1,
}
impl From<SHUTDOWN_A> for bool {
    #[inline(always)]
    fn from(variant: SHUTDOWN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SHUTDOWN` reader - "]
pub type SHUTDOWN_R = crate::BitReader<SHUTDOWN_A>;
impl SHUTDOWN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SHUTDOWN_A {
        match self.bits {
            false => SHUTDOWN_A::POWER_UP,
            true => SHUTDOWN_A::SHUT_DOWN,
        }
    }
    #[doc = "Checks if the value of the field is `POWER_UP`"]
    #[inline(always)]
    pub fn is_power_up(&self) -> bool {
        *self == SHUTDOWN_A::POWER_UP
    }
    #[doc = "Checks if the value of the field is `SHUT_DOWN`"]
    #[inline(always)]
    pub fn is_shut_down(&self) -> bool {
        *self == SHUTDOWN_A::SHUT_DOWN
    }
}
#[doc = "Field `SHUTDOWN` writer - "]
pub type SHUTDOWN_W<'a> = crate::BitWriter<'a, u32, EMAC_EPHY_CLK0_SPEC, SHUTDOWN_A, 16>;
impl<'a> SHUTDOWN_W<'a> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn power_up(self) -> &'a mut W {
        self.variant(SHUTDOWN_A::POWER_UP)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn shut_down(self) -> &'a mut W {
        self.variant(SHUTDOWN_A::SHUT_DOWN)
    }
}
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PHY_SELECT_A {
    #[doc = "0: `0`"]
    EXTERNAL = 0,
    #[doc = "1: `1`"]
    INTERNAL = 1,
}
impl From<PHY_SELECT_A> for bool {
    #[inline(always)]
    fn from(variant: PHY_SELECT_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PHY_SELECT` reader - "]
pub type PHY_SELECT_R = crate::BitReader<PHY_SELECT_A>;
impl PHY_SELECT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PHY_SELECT_A {
        match self.bits {
            false => PHY_SELECT_A::EXTERNAL,
            true => PHY_SELECT_A::INTERNAL,
        }
    }
    #[doc = "Checks if the value of the field is `EXTERNAL`"]
    #[inline(always)]
    pub fn is_external(&self) -> bool {
        *self == PHY_SELECT_A::EXTERNAL
    }
    #[doc = "Checks if the value of the field is `INTERNAL`"]
    #[inline(always)]
    pub fn is_internal(&self) -> bool {
        *self == PHY_SELECT_A::INTERNAL
    }
}
#[doc = "Field `PHY_SELECT` writer - "]
pub type PHY_SELECT_W<'a> = crate::BitWriter<'a, u32, EMAC_EPHY_CLK0_SPEC, PHY_SELECT_A, 15>;
impl<'a> PHY_SELECT_W<'a> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn external(self) -> &'a mut W {
        self.variant(PHY_SELECT_A::EXTERNAL)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn internal(self) -> &'a mut W {
        self.variant(PHY_SELECT_A::INTERNAL)
    }
}
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RMII_EN_A {
    #[doc = "0: `0`"]
    DISABLE = 0,
    #[doc = "1: `1`"]
    ENABLE = 1,
}
impl From<RMII_EN_A> for bool {
    #[inline(always)]
    fn from(variant: RMII_EN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RMII_EN` reader - "]
pub type RMII_EN_R = crate::BitReader<RMII_EN_A>;
impl RMII_EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RMII_EN_A {
        match self.bits {
            false => RMII_EN_A::DISABLE,
            true => RMII_EN_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == RMII_EN_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == RMII_EN_A::ENABLE
    }
}
#[doc = "Field `RMII_EN` writer - "]
pub type RMII_EN_W<'a> = crate::BitWriter<'a, u32, EMAC_EPHY_CLK0_SPEC, RMII_EN_A, 13>;
impl<'a> RMII_EN_W<'a> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(RMII_EN_A::DISABLE)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(RMII_EN_A::ENABLE)
    }
}
#[doc = "Field `ETXDC` reader - "]
pub type ETXDC_R = crate::FieldReader<u8, u8>;
#[doc = "Field `ETXDC` writer - "]
pub type ETXDC_W<'a> = crate::FieldWriter<'a, u32, EMAC_EPHY_CLK0_SPEC, u8, u8, 3, 10>;
#[doc = "Field `ERXDC` reader - "]
pub type ERXDC_R = crate::FieldReader<u8, u8>;
#[doc = "Field `ERXDC` writer - "]
pub type ERXDC_W<'a> = crate::FieldWriter<'a, u32, EMAC_EPHY_CLK0_SPEC, u8, u8, 5, 5>;
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ERXIE_A {
    #[doc = "0: `0`"]
    DISABLE = 0,
    #[doc = "1: `1`"]
    ENABLE = 1,
}
impl From<ERXIE_A> for bool {
    #[inline(always)]
    fn from(variant: ERXIE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ERXIE` reader - "]
pub type ERXIE_R = crate::BitReader<ERXIE_A>;
impl ERXIE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ERXIE_A {
        match self.bits {
            false => ERXIE_A::DISABLE,
            true => ERXIE_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == ERXIE_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == ERXIE_A::ENABLE
    }
}
#[doc = "Field `ERXIE` writer - "]
pub type ERXIE_W<'a> = crate::BitWriter<'a, u32, EMAC_EPHY_CLK0_SPEC, ERXIE_A, 4>;
impl<'a> ERXIE_W<'a> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(ERXIE_A::DISABLE)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(ERXIE_A::ENABLE)
    }
}
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ETXIE_A {
    #[doc = "0: `0`"]
    DISABLE = 0,
    #[doc = "1: `1`"]
    ENABLE = 1,
}
impl From<ETXIE_A> for bool {
    #[inline(always)]
    fn from(variant: ETXIE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ETXIE` reader - "]
pub type ETXIE_R = crate::BitReader<ETXIE_A>;
impl ETXIE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ETXIE_A {
        match self.bits {
            false => ETXIE_A::DISABLE,
            true => ETXIE_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == ETXIE_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == ETXIE_A::ENABLE
    }
}
#[doc = "Field `ETXIE` writer - "]
pub type ETXIE_W<'a> = crate::BitWriter<'a, u32, EMAC_EPHY_CLK0_SPEC, ETXIE_A, 3>;
impl<'a> ETXIE_W<'a> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(ETXIE_A::DISABLE)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(ETXIE_A::ENABLE)
    }
}
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EPIT_A {
    #[doc = "0: `0`"]
    MII = 0,
    #[doc = "1: `1`"]
    RGMII = 1,
}
impl From<EPIT_A> for bool {
    #[inline(always)]
    fn from(variant: EPIT_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EPIT` reader - "]
pub type EPIT_R = crate::BitReader<EPIT_A>;
impl EPIT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EPIT_A {
        match self.bits {
            false => EPIT_A::MII,
            true => EPIT_A::RGMII,
        }
    }
    #[doc = "Checks if the value of the field is `MII`"]
    #[inline(always)]
    pub fn is_mii(&self) -> bool {
        *self == EPIT_A::MII
    }
    #[doc = "Checks if the value of the field is `RGMII`"]
    #[inline(always)]
    pub fn is_rgmii(&self) -> bool {
        *self == EPIT_A::RGMII
    }
}
#[doc = "Field `EPIT` writer - "]
pub type EPIT_W<'a> = crate::BitWriter<'a, u32, EMAC_EPHY_CLK0_SPEC, EPIT_A, 2>;
impl<'a> EPIT_W<'a> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn mii(self) -> &'a mut W {
        self.variant(EPIT_A::MII)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn rgmii(self) -> &'a mut W {
        self.variant(EPIT_A::RGMII)
    }
}
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum ETCS_A {
    #[doc = "0: `0`"]
    MII = 0,
    #[doc = "1: `1`"]
    EXTERNAL_GMII_RGMII = 1,
    #[doc = "2: `10`"]
    INTERNAL_GMII_RGMII = 2,
}
impl From<ETCS_A> for u8 {
    #[inline(always)]
    fn from(variant: ETCS_A) -> Self {
        variant as _
    }
}
#[doc = "Field `ETCS` reader - "]
pub type ETCS_R = crate::FieldReader<u8, ETCS_A>;
impl ETCS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<ETCS_A> {
        match self.bits {
            0 => Some(ETCS_A::MII),
            1 => Some(ETCS_A::EXTERNAL_GMII_RGMII),
            2 => Some(ETCS_A::INTERNAL_GMII_RGMII),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `MII`"]
    #[inline(always)]
    pub fn is_mii(&self) -> bool {
        *self == ETCS_A::MII
    }
    #[doc = "Checks if the value of the field is `EXTERNAL_GMII_RGMII`"]
    #[inline(always)]
    pub fn is_external_gmii_rgmii(&self) -> bool {
        *self == ETCS_A::EXTERNAL_GMII_RGMII
    }
    #[doc = "Checks if the value of the field is `INTERNAL_GMII_RGMII`"]
    #[inline(always)]
    pub fn is_internal_gmii_rgmii(&self) -> bool {
        *self == ETCS_A::INTERNAL_GMII_RGMII
    }
}
#[doc = "Field `ETCS` writer - "]
pub type ETCS_W<'a> = crate::FieldWriter<'a, u32, EMAC_EPHY_CLK0_SPEC, u8, ETCS_A, 2, 0>;
impl<'a> ETCS_W<'a> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn mii(self) -> &'a mut W {
        self.variant(ETCS_A::MII)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn external_gmii_rgmii(self) -> &'a mut W {
        self.variant(ETCS_A::EXTERNAL_GMII_RGMII)
    }
    #[doc = "`10`"]
    #[inline(always)]
    pub fn internal_gmii_rgmii(self) -> &'a mut W {
        self.variant(ETCS_A::INTERNAL_GMII_RGMII)
    }
}
impl R {
    #[doc = "Bits 28:31"]
    #[inline(always)]
    pub fn bps_efuse(&self) -> BPS_EFUSE_R {
        BPS_EFUSE_R::new(((self.bits >> 28) & 0x0f) as u8)
    }
    #[doc = "Bit 27"]
    #[inline(always)]
    pub fn xmii_sel(&self) -> XMII_SEL_R {
        XMII_SEL_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bits 25:26"]
    #[inline(always)]
    pub fn ephy_mode(&self) -> EPHY_MODE_R {
        EPHY_MODE_R::new(((self.bits >> 25) & 3) as u8)
    }
    #[doc = "Bits 20:24"]
    #[inline(always)]
    pub fn phy_addr(&self) -> PHY_ADDR_R {
        PHY_ADDR_R::new(((self.bits >> 20) & 0x1f) as u8)
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    pub fn clk_sel(&self) -> CLK_SEL_R {
        CLK_SEL_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    pub fn led_pol(&self) -> LED_POL_R {
        LED_POL_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn shutdown(&self) -> SHUTDOWN_R {
        SHUTDOWN_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn phy_select(&self) -> PHY_SELECT_R {
        PHY_SELECT_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn rmii_en(&self) -> RMII_EN_R {
        RMII_EN_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bits 10:12"]
    #[inline(always)]
    pub fn etxdc(&self) -> ETXDC_R {
        ETXDC_R::new(((self.bits >> 10) & 7) as u8)
    }
    #[doc = "Bits 5:9"]
    #[inline(always)]
    pub fn erxdc(&self) -> ERXDC_R {
        ERXDC_R::new(((self.bits >> 5) & 0x1f) as u8)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn erxie(&self) -> ERXIE_R {
        ERXIE_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn etxie(&self) -> ETXIE_R {
        ETXIE_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn epit(&self) -> EPIT_R {
        EPIT_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 0:1"]
    #[inline(always)]
    pub fn etcs(&self) -> ETCS_R {
        ETCS_R::new((self.bits & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 28:31"]
    #[inline(always)]
    pub fn bps_efuse(&mut self) -> BPS_EFUSE_W {
        BPS_EFUSE_W::new(self)
    }
    #[doc = "Bit 27"]
    #[inline(always)]
    pub fn xmii_sel(&mut self) -> XMII_SEL_W {
        XMII_SEL_W::new(self)
    }
    #[doc = "Bits 25:26"]
    #[inline(always)]
    pub fn ephy_mode(&mut self) -> EPHY_MODE_W {
        EPHY_MODE_W::new(self)
    }
    #[doc = "Bits 20:24"]
    #[inline(always)]
    pub fn phy_addr(&mut self) -> PHY_ADDR_W {
        PHY_ADDR_W::new(self)
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    pub fn clk_sel(&mut self) -> CLK_SEL_W {
        CLK_SEL_W::new(self)
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    pub fn led_pol(&mut self) -> LED_POL_W {
        LED_POL_W::new(self)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn shutdown(&mut self) -> SHUTDOWN_W {
        SHUTDOWN_W::new(self)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn phy_select(&mut self) -> PHY_SELECT_W {
        PHY_SELECT_W::new(self)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn rmii_en(&mut self) -> RMII_EN_W {
        RMII_EN_W::new(self)
    }
    #[doc = "Bits 10:12"]
    #[inline(always)]
    pub fn etxdc(&mut self) -> ETXDC_W {
        ETXDC_W::new(self)
    }
    #[doc = "Bits 5:9"]
    #[inline(always)]
    pub fn erxdc(&mut self) -> ERXDC_W {
        ERXDC_W::new(self)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn erxie(&mut self) -> ERXIE_W {
        ERXIE_W::new(self)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn etxie(&mut self) -> ETXIE_W {
        ETXIE_W::new(self)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn epit(&mut self) -> EPIT_W {
        EPIT_W::new(self)
    }
    #[doc = "Bits 0:1"]
    #[inline(always)]
    pub fn etcs(&mut self) -> ETCS_W {
        ETCS_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "EMAC-EPHY Clock Register 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [emac_ephy_clk0](index.html) module"]
pub struct EMAC_EPHY_CLK0_SPEC;
impl crate::RegisterSpec for EMAC_EPHY_CLK0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [emac_ephy_clk0::R](R) reader structure"]
impl crate::Readable for EMAC_EPHY_CLK0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [emac_ephy_clk0::W](W) writer structure"]
impl crate::Writable for EMAC_EPHY_CLK0_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets EMAC_EPHY_CLK0 to value 0"]
impl crate::Resettable for EMAC_EPHY_CLK0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
