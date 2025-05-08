crate::utils::enum_wrapper!(obd2, FuelTypeCoding, FuelTypeCodingByte, display = @"8724297537048401983");

/// Fuel type coding for PID 51
#[allow(non_camel_case_types)]
#[repr(u8)]
#[derive(strum::FromRepr, Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[cfg_attr(feature = "display", derive(displaydoc::Display))]
#[cfg_attr(feature = "iter", derive(strum::EnumIter))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[allow(clippy::upper_case_acronyms)]
pub enum FuelTypeCoding {
    /// Fuel type unavailable
    NotAvailable = 0,
    /// Gasoline engine
    Gasoline = 1,
    /// Methanol engine
    Methanol = 2,
    /// Ethanol engine
    Ethanol = 3,
    /// Diesel engine
    Diesel = 4,
    /// LPG engine
    LPG = 5,
    /// CNG engine
    CNG = 6,
    /// Propane engine
    Propane = 7,
    /// Electric engine
    Electric = 8,
    /// Bifuel engine running gasoline as primary
    BifuelGasoline = 9,
    /// Bifuel engine running methanol as primary
    BifuelMethanol = 10,
    /// Bifuel engine running ethanol as primary
    BifuelEthanol = 11,
    /// Bifuel engine running LPG as primary
    BifuelLPG = 12,
    /// Bifuel engine running CNG as primary
    BifuelCNG = 13,
    /// Bifuel engine running propane as primary
    BifuelPropane = 14,
    /// Bifuel engine running electricity as primary
    BifuelElectricity = 15,
    /// Bifuel engine running an electric and combustion engine as primary
    BifuelElectricAndCombustion = 16,
    /// Hybrid gasoline engine
    HybridGasoline = 17,
    /// Hybrid ethanol engine
    HybridEthanol = 18,
    /// Hybrid diesel engine
    HybridDiesel = 19,
    /// Hybrid electric engine
    HybridElectric = 20,
    /// Hybrid electric and combustion engine
    HybridElectricAndCombustion = 21,
    /// Hybrid regenerative engine
    HybridRegen = 22,
    /// Bifuel engine running diesel as primary
    BifuelDiesel = 23,
}
