pub enum CompositeOperator {
    UndefinedCompositeOp,
    AlphaCompositeOp,
    AtopCompositeOp,
    BlendCompositeOp,
    BlurCompositeOp,
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
    CopyAlphaCompositeOp,
    CopyRedCompositeOp,
    CopyYellowCompositeOp,
    DarkenCompositeOp,
    DarkenIntensityCompositeOp,
    DifferenceCompositeOp,
    DisplaceCompositeOp,
    DissolveCompositeOp,
    DistortCompositeOp,
    DivideDstCompositeOp,
    DivideSrcCompositeOp,
    DstAtopCompositeOp,
    DstCompositeOp,
    DstInCompositeOp,
    DstOutCompositeOp,
    DstOverCompositeOp,
    ExclusionCompositeOp,
    HardLightCompositeOp,
    HardMixCompositeOp,
    HueCompositeOp,
    InCompositeOp,
    IntensityCompositeOp,
    LightenCompositeOp,
    LightenIntensityCompositeOp,
    LinearBurnCompositeOp,
    LinearDodgeCompositeOp,
    LinearLightCompositeOp,
    LuminizeCompositeOp,
    MathematicsCompositeOp,
    MinusDstCompositeOp,
    MinusSrcCompositeOp,
    ModulateCompositeOp,
    ModulusAddCompositeOp,
    ModulusSubtractCompositeOp,
    MultiplyCompositeOp,
    NoCompositeOp,
    OutCompositeOp,
    OverCompositeOp,
    OverlayCompositeOp,
    PegtopLightCompositeOp,
    PinLightCompositeOp,
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
    ThresholdCompositeOp,
    VividLightCompositeOp,
    XorCompositeOp,
    StereoCompositeOp,
    FreezeCompositeOp,
    InterpolateCompositeOp,
    NegateCompositeOp,
    ReflectCompositeOp,
    SoftBurnCompositeOp,
    SoftDodgeCompositeOp,
    StampCompositeOp,
}

impl Into<magickwand_bindgen::CompositeOperator> for CompositeOperator {
    fn into(self) -> magickwand_bindgen::CompositeOperator {
        match self {
            CompositeOperator::UndefinedCompositeOp => {
                magickwand_bindgen::CompositeOperator_UndefinedCompositeOp
            }
            CompositeOperator::AlphaCompositeOp => {
                magickwand_bindgen::CompositeOperator_AlphaCompositeOp
            }
            CompositeOperator::AtopCompositeOp => {
                magickwand_bindgen::CompositeOperator_AtopCompositeOp
            }
            CompositeOperator::BlendCompositeOp => {
                magickwand_bindgen::CompositeOperator_BlendCompositeOp
            }
            CompositeOperator::BlurCompositeOp => {
                magickwand_bindgen::CompositeOperator_BlurCompositeOp
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
            CompositeOperator::CopyAlphaCompositeOp => {
                magickwand_bindgen::CompositeOperator_CopyAlphaCompositeOp
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
            CompositeOperator::DarkenIntensityCompositeOp => {
                magickwand_bindgen::CompositeOperator_DarkenIntensityCompositeOp
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
            CompositeOperator::DistortCompositeOp => {
                magickwand_bindgen::CompositeOperator_DistortCompositeOp
            }
            CompositeOperator::DivideDstCompositeOp => {
                magickwand_bindgen::CompositeOperator_DivideDstCompositeOp
            }
            CompositeOperator::DivideSrcCompositeOp => {
                magickwand_bindgen::CompositeOperator_DivideSrcCompositeOp
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
            CompositeOperator::ExclusionCompositeOp => {
                magickwand_bindgen::CompositeOperator_ExclusionCompositeOp
            }
            CompositeOperator::HardLightCompositeOp => {
                magickwand_bindgen::CompositeOperator_HardLightCompositeOp
            }
            CompositeOperator::HardMixCompositeOp => {
                magickwand_bindgen::CompositeOperator_HardMixCompositeOp
            }
            CompositeOperator::HueCompositeOp => {
                magickwand_bindgen::CompositeOperator_HueCompositeOp
            }
            CompositeOperator::InCompositeOp => magickwand_bindgen::CompositeOperator_InCompositeOp,
            CompositeOperator::IntensityCompositeOp => {
                magickwand_bindgen::CompositeOperator_IntensityCompositeOp
            }
            CompositeOperator::LightenCompositeOp => {
                magickwand_bindgen::CompositeOperator_LightenCompositeOp
            }
            CompositeOperator::LightenIntensityCompositeOp => {
                magickwand_bindgen::CompositeOperator_LightenIntensityCompositeOp
            }
            CompositeOperator::LinearBurnCompositeOp => {
                magickwand_bindgen::CompositeOperator_LinearBurnCompositeOp
            }
            CompositeOperator::LinearDodgeCompositeOp => {
                magickwand_bindgen::CompositeOperator_LinearDodgeCompositeOp
            }
            CompositeOperator::LinearLightCompositeOp => {
                magickwand_bindgen::CompositeOperator_LinearLightCompositeOp
            }
            CompositeOperator::LuminizeCompositeOp => {
                magickwand_bindgen::CompositeOperator_LuminizeCompositeOp
            }
            CompositeOperator::MathematicsCompositeOp => {
                magickwand_bindgen::CompositeOperator_MathematicsCompositeOp
            }
            CompositeOperator::MinusDstCompositeOp => {
                magickwand_bindgen::CompositeOperator_MinusDstCompositeOp
            }
            CompositeOperator::MinusSrcCompositeOp => {
                magickwand_bindgen::CompositeOperator_MinusSrcCompositeOp
            }
            CompositeOperator::ModulateCompositeOp => {
                magickwand_bindgen::CompositeOperator_ModulateCompositeOp
            }
            CompositeOperator::ModulusAddCompositeOp => {
                magickwand_bindgen::CompositeOperator_ModulusAddCompositeOp
            }
            CompositeOperator::ModulusSubtractCompositeOp => {
                magickwand_bindgen::CompositeOperator_ModulusSubtractCompositeOp
            }
            CompositeOperator::MultiplyCompositeOp => {
                magickwand_bindgen::CompositeOperator_MultiplyCompositeOp
            }
            CompositeOperator::NoCompositeOp => magickwand_bindgen::CompositeOperator_NoCompositeOp,
            CompositeOperator::OutCompositeOp => {
                magickwand_bindgen::CompositeOperator_OutCompositeOp
            }
            CompositeOperator::OverCompositeOp => {
                magickwand_bindgen::CompositeOperator_OverCompositeOp
            }
            CompositeOperator::OverlayCompositeOp => {
                magickwand_bindgen::CompositeOperator_OverlayCompositeOp
            }
            CompositeOperator::PegtopLightCompositeOp => {
                magickwand_bindgen::CompositeOperator_PegtopLightCompositeOp
            }
            CompositeOperator::PinLightCompositeOp => {
                magickwand_bindgen::CompositeOperator_PinLightCompositeOp
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
            CompositeOperator::ThresholdCompositeOp => {
                magickwand_bindgen::CompositeOperator_ThresholdCompositeOp
            }
            CompositeOperator::VividLightCompositeOp => {
                magickwand_bindgen::CompositeOperator_VividLightCompositeOp
            }
            CompositeOperator::XorCompositeOp => {
                magickwand_bindgen::CompositeOperator_XorCompositeOp
            }
            CompositeOperator::StereoCompositeOp => {
                magickwand_bindgen::CompositeOperator_StereoCompositeOp
            }
            CompositeOperator::FreezeCompositeOp => {
                magickwand_bindgen::CompositeOperator_FreezeCompositeOp
            }
            CompositeOperator::InterpolateCompositeOp => {
                magickwand_bindgen::CompositeOperator_InterpolateCompositeOp
            }
            CompositeOperator::NegateCompositeOp => {
                magickwand_bindgen::CompositeOperator_NegateCompositeOp
            }
            CompositeOperator::ReflectCompositeOp => {
                magickwand_bindgen::CompositeOperator_ReflectCompositeOp
            }
            CompositeOperator::SoftBurnCompositeOp => {
                magickwand_bindgen::CompositeOperator_SoftBurnCompositeOp
            }
            CompositeOperator::SoftDodgeCompositeOp => {
                magickwand_bindgen::CompositeOperator_SoftDodgeCompositeOp
            }
            CompositeOperator::StampCompositeOp => {
                magickwand_bindgen::CompositeOperator_StampCompositeOp
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
        }
    }
}

pub enum FilterType {
    UndefinedFilter,
    PointFilter,
    BoxFilter,
    TriangleFilter,
    HermiteFilter,
    HannFilter,
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
    WelchFilter,
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
    CubicSplineFilter,
    SentinelFilter,
}

impl Into<magickwand_bindgen::FilterType> for FilterType {
    fn into(self) -> magickwand_bindgen::FilterType {
        match self {
            FilterType::UndefinedFilter => magickwand_bindgen::FilterType_UndefinedFilter,
            FilterType::PointFilter => magickwand_bindgen::FilterType_PointFilter,
            FilterType::BoxFilter => magickwand_bindgen::FilterType_BoxFilter,
            FilterType::TriangleFilter => magickwand_bindgen::FilterType_TriangleFilter,
            FilterType::HermiteFilter => magickwand_bindgen::FilterType_HermiteFilter,
            FilterType::HannFilter => magickwand_bindgen::FilterType_HannFilter,
            FilterType::HammingFilter => magickwand_bindgen::FilterType_HammingFilter,
            FilterType::BlackmanFilter => magickwand_bindgen::FilterType_BlackmanFilter,
            FilterType::GaussianFilter => magickwand_bindgen::FilterType_GaussianFilter,
            FilterType::QuadraticFilter => magickwand_bindgen::FilterType_QuadraticFilter,
            FilterType::CubicFilter => magickwand_bindgen::FilterType_CubicFilter,
            FilterType::CatromFilter => magickwand_bindgen::FilterType_CatromFilter,
            FilterType::MitchellFilter => magickwand_bindgen::FilterType_MitchellFilter,
            FilterType::JincFilter => magickwand_bindgen::FilterType_JincFilter,
            FilterType::SincFilter => magickwand_bindgen::FilterType_SincFilter,
            FilterType::SincFastFilter => magickwand_bindgen::FilterType_SincFastFilter,
            FilterType::KaiserFilter => magickwand_bindgen::FilterType_KaiserFilter,
            FilterType::WelchFilter => magickwand_bindgen::FilterType_WelchFilter,
            FilterType::ParzenFilter => magickwand_bindgen::FilterType_ParzenFilter,
            FilterType::BohmanFilter => magickwand_bindgen::FilterType_BohmanFilter,
            FilterType::BartlettFilter => magickwand_bindgen::FilterType_BartlettFilter,
            FilterType::LagrangeFilter => magickwand_bindgen::FilterType_LagrangeFilter,
            FilterType::LanczosFilter => magickwand_bindgen::FilterType_LanczosFilter,
            FilterType::LanczosSharpFilter => magickwand_bindgen::FilterType_LanczosSharpFilter,
            FilterType::Lanczos2Filter => magickwand_bindgen::FilterType_Lanczos2Filter,
            FilterType::Lanczos2SharpFilter => magickwand_bindgen::FilterType_Lanczos2SharpFilter,
            FilterType::RobidouxFilter => magickwand_bindgen::FilterType_RobidouxFilter,
            FilterType::RobidouxSharpFilter => magickwand_bindgen::FilterType_RobidouxSharpFilter,
            FilterType::CosineFilter => magickwand_bindgen::FilterType_CosineFilter,
            FilterType::SplineFilter => magickwand_bindgen::FilterType_SplineFilter,
            FilterType::LanczosRadiusFilter => magickwand_bindgen::FilterType_LanczosRadiusFilter,
            FilterType::CubicSplineFilter => magickwand_bindgen::FilterType_CubicSplineFilter,
            FilterType::SentinelFilter => magickwand_bindgen::FilterType_SentinelFilter,
        }
    }
}
