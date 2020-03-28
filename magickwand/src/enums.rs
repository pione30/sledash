use magickwand_bindgen;

pub enum CompositeOperator {
    UndefinedCompositeOp,
    NoCompositeOp,
    ModulusAddCompositeOp,
    AtopCompositeOp,
    BlendCompositeOp,
    BumpmapCompositeOp,
    ChangeMaskCompositeOp,
    ClearCompositeOp,
    ColorBurnCompositeOp,
    ColorDodgeCompositeOp,
    ColorizeCompositeOp,
    CopyBlackCompositeOp,
    CopyBlueCompositeOp,
    CopyCompositeOp,
    CopyCyanCompositeOp,
    CopyGreenCompositeOp,
    CopyMagentaCompositeOp,
    CopyOpacityCompositeOp,
    CopyRedCompositeOp,
    CopyYellowCompositeOp,
    DarkenCompositeOp,
    DstAtopCompositeOp,
    DstCompositeOp,
    DstInCompositeOp,
    DstOutCompositeOp,
    DstOverCompositeOp,
    DifferenceCompositeOp,
    DisplaceCompositeOp,
    DissolveCompositeOp,
    ExclusionCompositeOp,
    HardLightCompositeOp,
    HueCompositeOp,
    InCompositeOp,
    LightenCompositeOp,
    LinearLightCompositeOp,
    LuminizeCompositeOp,
    MinusDstCompositeOp,
    ModulateCompositeOp,
    MultiplyCompositeOp,
    OutCompositeOp,
    OverCompositeOp,
    OverlayCompositeOp,
    PlusCompositeOp,
    ReplaceCompositeOp,
    SaturateCompositeOp,
    ScreenCompositeOp,
    SoftLightCompositeOp,
    SrcAtopCompositeOp,
    SrcCompositeOp,
    SrcInCompositeOp,
    SrcOutCompositeOp,
    SrcOverCompositeOp,
    ModulusSubtractCompositeOp,
    ThresholdCompositeOp,
    XorCompositeOp,
    DivideDstCompositeOp,
    DistortCompositeOp,
    BlurCompositeOp,
    PegtopLightCompositeOp,
    VividLightCompositeOp,
    PinLightCompositeOp,
    LinearDodgeCompositeOp,
    LinearBurnCompositeOp,
    MathematicsCompositeOp,
    DivideSrcCompositeOp,
    MinusSrcCompositeOp,
    DarkenIntensityCompositeOp,
    LightenIntensityCompositeOp,
    HardMixCompositeOp,
}

impl Into<magickwand_bindgen::CompositeOperator> for CompositeOperator {
    fn into(self) -> magickwand_bindgen::CompositeOperator {
        match self {
            CompositeOperator::UndefinedCompositeOp => {
                magickwand_bindgen::CompositeOperator_UndefinedCompositeOp
            }
            CompositeOperator::NoCompositeOp => magickwand_bindgen::CompositeOperator_NoCompositeOp,
            CompositeOperator::ModulusAddCompositeOp => {
                magickwand_bindgen::CompositeOperator_ModulusAddCompositeOp
            }
            CompositeOperator::AtopCompositeOp => {
                magickwand_bindgen::CompositeOperator_AtopCompositeOp
            }
            CompositeOperator::BlendCompositeOp => {
                magickwand_bindgen::CompositeOperator_BlendCompositeOp
            }
            CompositeOperator::BumpmapCompositeOp => {
                magickwand_bindgen::CompositeOperator_BumpmapCompositeOp
            }
            CompositeOperator::ChangeMaskCompositeOp => {
                magickwand_bindgen::CompositeOperator_ChangeMaskCompositeOp
            }
            CompositeOperator::ClearCompositeOp => {
                magickwand_bindgen::CompositeOperator_ClearCompositeOp
            }
            CompositeOperator::ColorBurnCompositeOp => {
                magickwand_bindgen::CompositeOperator_ColorBurnCompositeOp
            }
            CompositeOperator::ColorDodgeCompositeOp => {
                magickwand_bindgen::CompositeOperator_ColorDodgeCompositeOp
            }
            CompositeOperator::ColorizeCompositeOp => {
                magickwand_bindgen::CompositeOperator_ColorizeCompositeOp
            }
            CompositeOperator::CopyBlackCompositeOp => {
                magickwand_bindgen::CompositeOperator_CopyBlackCompositeOp
            }
            CompositeOperator::CopyBlueCompositeOp => {
                magickwand_bindgen::CompositeOperator_CopyBlueCompositeOp
            }
            CompositeOperator::CopyCompositeOp => {
                magickwand_bindgen::CompositeOperator_CopyCompositeOp
            }
            CompositeOperator::CopyCyanCompositeOp => {
                magickwand_bindgen::CompositeOperator_CopyCyanCompositeOp
            }
            CompositeOperator::CopyGreenCompositeOp => {
                magickwand_bindgen::CompositeOperator_CopyGreenCompositeOp
            }
            CompositeOperator::CopyMagentaCompositeOp => {
                magickwand_bindgen::CompositeOperator_CopyMagentaCompositeOp
            }
            CompositeOperator::CopyOpacityCompositeOp => {
                magickwand_bindgen::CompositeOperator_CopyOpacityCompositeOp
            }
            CompositeOperator::CopyRedCompositeOp => {
                magickwand_bindgen::CompositeOperator_CopyRedCompositeOp
            }
            CompositeOperator::CopyYellowCompositeOp => {
                magickwand_bindgen::CompositeOperator_CopyYellowCompositeOp
            }
            CompositeOperator::DarkenCompositeOp => {
                magickwand_bindgen::CompositeOperator_DarkenCompositeOp
            }
            CompositeOperator::DstAtopCompositeOp => {
                magickwand_bindgen::CompositeOperator_DstAtopCompositeOp
            }
            CompositeOperator::DstCompositeOp => {
                magickwand_bindgen::CompositeOperator_DstCompositeOp
            }
            CompositeOperator::DstInCompositeOp => {
                magickwand_bindgen::CompositeOperator_DstInCompositeOp
            }
            CompositeOperator::DstOutCompositeOp => {
                magickwand_bindgen::CompositeOperator_DstOutCompositeOp
            }
            CompositeOperator::DstOverCompositeOp => {
                magickwand_bindgen::CompositeOperator_DstOverCompositeOp
            }
            CompositeOperator::DifferenceCompositeOp => {
                magickwand_bindgen::CompositeOperator_DifferenceCompositeOp
            }
            CompositeOperator::DisplaceCompositeOp => {
                magickwand_bindgen::CompositeOperator_DisplaceCompositeOp
            }
            CompositeOperator::DissolveCompositeOp => {
                magickwand_bindgen::CompositeOperator_DissolveCompositeOp
            }
            CompositeOperator::ExclusionCompositeOp => {
                magickwand_bindgen::CompositeOperator_ExclusionCompositeOp
            }
            CompositeOperator::HardLightCompositeOp => {
                magickwand_bindgen::CompositeOperator_HardLightCompositeOp
            }
            CompositeOperator::HueCompositeOp => {
                magickwand_bindgen::CompositeOperator_HueCompositeOp
            }
            CompositeOperator::InCompositeOp => magickwand_bindgen::CompositeOperator_InCompositeOp,
            CompositeOperator::LightenCompositeOp => {
                magickwand_bindgen::CompositeOperator_LightenCompositeOp
            }
            CompositeOperator::LinearLightCompositeOp => {
                magickwand_bindgen::CompositeOperator_LinearLightCompositeOp
            }
            CompositeOperator::LuminizeCompositeOp => {
                magickwand_bindgen::CompositeOperator_LuminizeCompositeOp
            }
            CompositeOperator::MinusDstCompositeOp => {
                magickwand_bindgen::CompositeOperator_MinusDstCompositeOp
            }
            CompositeOperator::ModulateCompositeOp => {
                magickwand_bindgen::CompositeOperator_ModulateCompositeOp
            }
            CompositeOperator::MultiplyCompositeOp => {
                magickwand_bindgen::CompositeOperator_MultiplyCompositeOp
            }
            CompositeOperator::OutCompositeOp => {
                magickwand_bindgen::CompositeOperator_OutCompositeOp
            }
            CompositeOperator::OverCompositeOp => {
                magickwand_bindgen::CompositeOperator_OverCompositeOp
            }
            CompositeOperator::OverlayCompositeOp => {
                magickwand_bindgen::CompositeOperator_OverlayCompositeOp
            }
            CompositeOperator::PlusCompositeOp => {
                magickwand_bindgen::CompositeOperator_PlusCompositeOp
            }
            CompositeOperator::ReplaceCompositeOp => {
                magickwand_bindgen::CompositeOperator_ReplaceCompositeOp
            }
            CompositeOperator::SaturateCompositeOp => {
                magickwand_bindgen::CompositeOperator_SaturateCompositeOp
            }
            CompositeOperator::ScreenCompositeOp => {
                magickwand_bindgen::CompositeOperator_ScreenCompositeOp
            }
            CompositeOperator::SoftLightCompositeOp => {
                magickwand_bindgen::CompositeOperator_SoftLightCompositeOp
            }
            CompositeOperator::SrcAtopCompositeOp => {
                magickwand_bindgen::CompositeOperator_SrcAtopCompositeOp
            }
            CompositeOperator::SrcCompositeOp => {
                magickwand_bindgen::CompositeOperator_SrcCompositeOp
            }
            CompositeOperator::SrcInCompositeOp => {
                magickwand_bindgen::CompositeOperator_SrcInCompositeOp
            }
            CompositeOperator::SrcOutCompositeOp => {
                magickwand_bindgen::CompositeOperator_SrcOutCompositeOp
            }
            CompositeOperator::SrcOverCompositeOp => {
                magickwand_bindgen::CompositeOperator_SrcOverCompositeOp
            }
            CompositeOperator::ModulusSubtractCompositeOp => {
                magickwand_bindgen::CompositeOperator_ModulusSubtractCompositeOp
            }
            CompositeOperator::ThresholdCompositeOp => {
                magickwand_bindgen::CompositeOperator_ThresholdCompositeOp
            }
            CompositeOperator::XorCompositeOp => {
                magickwand_bindgen::CompositeOperator_XorCompositeOp
            }
            CompositeOperator::DivideDstCompositeOp => {
                magickwand_bindgen::CompositeOperator_DivideDstCompositeOp
            }
            CompositeOperator::DistortCompositeOp => {
                magickwand_bindgen::CompositeOperator_DistortCompositeOp
            }
            CompositeOperator::BlurCompositeOp => {
                magickwand_bindgen::CompositeOperator_BlurCompositeOp
            }
            CompositeOperator::PegtopLightCompositeOp => {
                magickwand_bindgen::CompositeOperator_PegtopLightCompositeOp
            }
            CompositeOperator::VividLightCompositeOp => {
                magickwand_bindgen::CompositeOperator_VividLightCompositeOp
            }
            CompositeOperator::PinLightCompositeOp => {
                magickwand_bindgen::CompositeOperator_PinLightCompositeOp
            }
            CompositeOperator::LinearDodgeCompositeOp => {
                magickwand_bindgen::CompositeOperator_LinearDodgeCompositeOp
            }
            CompositeOperator::LinearBurnCompositeOp => {
                magickwand_bindgen::CompositeOperator_LinearBurnCompositeOp
            }
            CompositeOperator::MathematicsCompositeOp => {
                magickwand_bindgen::CompositeOperator_MathematicsCompositeOp
            }
            CompositeOperator::DivideSrcCompositeOp => {
                magickwand_bindgen::CompositeOperator_DivideSrcCompositeOp
            }
            CompositeOperator::MinusSrcCompositeOp => {
                magickwand_bindgen::CompositeOperator_MinusSrcCompositeOp
            }
            CompositeOperator::DarkenIntensityCompositeOp => {
                magickwand_bindgen::CompositeOperator_DarkenIntensityCompositeOp
            }
            CompositeOperator::LightenIntensityCompositeOp => {
                magickwand_bindgen::CompositeOperator_LightenIntensityCompositeOp
            }
            CompositeOperator::HardMixCompositeOp => {
                magickwand_bindgen::CompositeOperator_HardMixCompositeOp
            }
        }
    }
}

pub enum GravityType {
    UndefinedGravity,
    ForgetGravity,
    NorthWestGravity,
    NorthGravity,
    NorthEastGravity,
    WestGravity,
    CenterGravity,
    EastGravity,
    SouthWestGravity,
    SouthGravity,
    SouthEastGravity,
    StaticGravity,
}

impl Into<magickwand_bindgen::GravityType> for GravityType {
    fn into(self) -> magickwand_bindgen::GravityType {
        match self {
            GravityType::UndefinedGravity => magickwand_bindgen::GravityType_UndefinedGravity,
            GravityType::ForgetGravity => magickwand_bindgen::GravityType_ForgetGravity,
            GravityType::NorthWestGravity => magickwand_bindgen::GravityType_NorthWestGravity,
            GravityType::NorthGravity => magickwand_bindgen::GravityType_NorthGravity,
            GravityType::NorthEastGravity => magickwand_bindgen::GravityType_NorthEastGravity,
            GravityType::WestGravity => magickwand_bindgen::GravityType_WestGravity,
            GravityType::CenterGravity => magickwand_bindgen::GravityType_CenterGravity,
            GravityType::EastGravity => magickwand_bindgen::GravityType_EastGravity,
            GravityType::SouthWestGravity => magickwand_bindgen::GravityType_SouthWestGravity,
            GravityType::SouthGravity => magickwand_bindgen::GravityType_SouthGravity,
            GravityType::SouthEastGravity => magickwand_bindgen::GravityType_SouthEastGravity,
            GravityType::StaticGravity => magickwand_bindgen::GravityType_StaticGravity,
        }
    }
}

pub enum FilterTypes {
    UndefinedFilter,
    PointFilter,
    BoxFilter,
    TriangleFilter,
    HermiteFilter,
    HanningFilter,
    HammingFilter,
    BlackmanFilter,
    GaussianFilter,
    QuadraticFilter,
    CubicFilter,
    CatromFilter,
    MitchellFilter,
    JincFilter,
    SincFilter,
    SincFastFilter,
    KaiserFilter,
    WelshFilter,
    ParzenFilter,
    BohmanFilter,
    BartlettFilter,
    LagrangeFilter,
    LanczosFilter,
    LanczosSharpFilter,
    Lanczos2Filter,
    Lanczos2SharpFilter,
    RobidouxFilter,
    RobidouxSharpFilter,
    CosineFilter,
    SplineFilter,
    LanczosRadiusFilter,
    SentinelFilter,
}

impl Into<magickwand_bindgen::FilterTypes> for FilterTypes {
    fn into(self) -> magickwand_bindgen::FilterTypes {
        match self {
            FilterTypes::UndefinedFilter => magickwand_bindgen::FilterTypes_UndefinedFilter,
            FilterTypes::PointFilter => magickwand_bindgen::FilterTypes_PointFilter,
            FilterTypes::BoxFilter => magickwand_bindgen::FilterTypes_BoxFilter,
            FilterTypes::TriangleFilter => magickwand_bindgen::FilterTypes_TriangleFilter,
            FilterTypes::HermiteFilter => magickwand_bindgen::FilterTypes_HermiteFilter,
            FilterTypes::HanningFilter => magickwand_bindgen::FilterTypes_HanningFilter,
            FilterTypes::HammingFilter => magickwand_bindgen::FilterTypes_HammingFilter,
            FilterTypes::BlackmanFilter => magickwand_bindgen::FilterTypes_BlackmanFilter,
            FilterTypes::GaussianFilter => magickwand_bindgen::FilterTypes_GaussianFilter,
            FilterTypes::QuadraticFilter => magickwand_bindgen::FilterTypes_QuadraticFilter,
            FilterTypes::CubicFilter => magickwand_bindgen::FilterTypes_CubicFilter,
            FilterTypes::CatromFilter => magickwand_bindgen::FilterTypes_CatromFilter,
            FilterTypes::MitchellFilter => magickwand_bindgen::FilterTypes_MitchellFilter,
            FilterTypes::JincFilter => magickwand_bindgen::FilterTypes_JincFilter,
            FilterTypes::SincFilter => magickwand_bindgen::FilterTypes_SincFilter,
            FilterTypes::SincFastFilter => magickwand_bindgen::FilterTypes_SincFastFilter,
            FilterTypes::KaiserFilter => magickwand_bindgen::FilterTypes_KaiserFilter,
            FilterTypes::WelshFilter => magickwand_bindgen::FilterTypes_WelshFilter,
            FilterTypes::ParzenFilter => magickwand_bindgen::FilterTypes_ParzenFilter,
            FilterTypes::BohmanFilter => magickwand_bindgen::FilterTypes_BohmanFilter,
            FilterTypes::BartlettFilter => magickwand_bindgen::FilterTypes_BartlettFilter,
            FilterTypes::LagrangeFilter => magickwand_bindgen::FilterTypes_LagrangeFilter,
            FilterTypes::LanczosFilter => magickwand_bindgen::FilterTypes_LanczosFilter,
            FilterTypes::LanczosSharpFilter => magickwand_bindgen::FilterTypes_LanczosSharpFilter,
            FilterTypes::Lanczos2Filter => magickwand_bindgen::FilterTypes_Lanczos2Filter,
            FilterTypes::Lanczos2SharpFilter => magickwand_bindgen::FilterTypes_Lanczos2SharpFilter,
            FilterTypes::RobidouxFilter => magickwand_bindgen::FilterTypes_RobidouxFilter,
            FilterTypes::RobidouxSharpFilter => magickwand_bindgen::FilterTypes_RobidouxSharpFilter,
            FilterTypes::CosineFilter => magickwand_bindgen::FilterTypes_CosineFilter,
            FilterTypes::SplineFilter => magickwand_bindgen::FilterTypes_SplineFilter,
            FilterTypes::LanczosRadiusFilter => magickwand_bindgen::FilterTypes_LanczosRadiusFilter,
            FilterTypes::SentinelFilter => magickwand_bindgen::FilterTypes_SentinelFilter,
        }
    }
}
