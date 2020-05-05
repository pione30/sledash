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

impl Into<sledash_magickwand_bindgen::CompositeOperator> for CompositeOperator {
    fn into(self) -> sledash_magickwand_bindgen::CompositeOperator {
        match self {
            CompositeOperator::UndefinedCompositeOp => {
                sledash_magickwand_bindgen::CompositeOperator_UndefinedCompositeOp
            }
            CompositeOperator::AlphaCompositeOp => {
                sledash_magickwand_bindgen::CompositeOperator_AlphaCompositeOp
            }
            CompositeOperator::AtopCompositeOp => {
                sledash_magickwand_bindgen::CompositeOperator_AtopCompositeOp
            }
            CompositeOperator::BlendCompositeOp => {
                sledash_magickwand_bindgen::CompositeOperator_BlendCompositeOp
            }
            CompositeOperator::BlurCompositeOp => {
                sledash_magickwand_bindgen::CompositeOperator_BlurCompositeOp
            }
            CompositeOperator::BumpmapCompositeOp => {
                sledash_magickwand_bindgen::CompositeOperator_BumpmapCompositeOp
            }
            CompositeOperator::ChangeMaskCompositeOp => {
                sledash_magickwand_bindgen::CompositeOperator_ChangeMaskCompositeOp
            }
            CompositeOperator::ClearCompositeOp => {
                sledash_magickwand_bindgen::CompositeOperator_ClearCompositeOp
            }
            CompositeOperator::ColorBurnCompositeOp => {
                sledash_magickwand_bindgen::CompositeOperator_ColorBurnCompositeOp
            }
            CompositeOperator::ColorDodgeCompositeOp => {
                sledash_magickwand_bindgen::CompositeOperator_ColorDodgeCompositeOp
            }
            CompositeOperator::ColorizeCompositeOp => {
                sledash_magickwand_bindgen::CompositeOperator_ColorizeCompositeOp
            }
            CompositeOperator::CopyBlackCompositeOp => {
                sledash_magickwand_bindgen::CompositeOperator_CopyBlackCompositeOp
            }
            CompositeOperator::CopyBlueCompositeOp => {
                sledash_magickwand_bindgen::CompositeOperator_CopyBlueCompositeOp
            }
            CompositeOperator::CopyCompositeOp => {
                sledash_magickwand_bindgen::CompositeOperator_CopyCompositeOp
            }
            CompositeOperator::CopyCyanCompositeOp => {
                sledash_magickwand_bindgen::CompositeOperator_CopyCyanCompositeOp
            }
            CompositeOperator::CopyGreenCompositeOp => {
                sledash_magickwand_bindgen::CompositeOperator_CopyGreenCompositeOp
            }
            CompositeOperator::CopyMagentaCompositeOp => {
                sledash_magickwand_bindgen::CompositeOperator_CopyMagentaCompositeOp
            }
            CompositeOperator::CopyAlphaCompositeOp => {
                sledash_magickwand_bindgen::CompositeOperator_CopyAlphaCompositeOp
            }
            CompositeOperator::CopyRedCompositeOp => {
                sledash_magickwand_bindgen::CompositeOperator_CopyRedCompositeOp
            }
            CompositeOperator::CopyYellowCompositeOp => {
                sledash_magickwand_bindgen::CompositeOperator_CopyYellowCompositeOp
            }
            CompositeOperator::DarkenCompositeOp => {
                sledash_magickwand_bindgen::CompositeOperator_DarkenCompositeOp
            }
            CompositeOperator::DarkenIntensityCompositeOp => {
                sledash_magickwand_bindgen::CompositeOperator_DarkenIntensityCompositeOp
            }
            CompositeOperator::DifferenceCompositeOp => {
                sledash_magickwand_bindgen::CompositeOperator_DifferenceCompositeOp
            }
            CompositeOperator::DisplaceCompositeOp => {
                sledash_magickwand_bindgen::CompositeOperator_DisplaceCompositeOp
            }
            CompositeOperator::DissolveCompositeOp => {
                sledash_magickwand_bindgen::CompositeOperator_DissolveCompositeOp
            }
            CompositeOperator::DistortCompositeOp => {
                sledash_magickwand_bindgen::CompositeOperator_DistortCompositeOp
            }
            CompositeOperator::DivideDstCompositeOp => {
                sledash_magickwand_bindgen::CompositeOperator_DivideDstCompositeOp
            }
            CompositeOperator::DivideSrcCompositeOp => {
                sledash_magickwand_bindgen::CompositeOperator_DivideSrcCompositeOp
            }
            CompositeOperator::DstAtopCompositeOp => {
                sledash_magickwand_bindgen::CompositeOperator_DstAtopCompositeOp
            }
            CompositeOperator::DstCompositeOp => {
                sledash_magickwand_bindgen::CompositeOperator_DstCompositeOp
            }
            CompositeOperator::DstInCompositeOp => {
                sledash_magickwand_bindgen::CompositeOperator_DstInCompositeOp
            }
            CompositeOperator::DstOutCompositeOp => {
                sledash_magickwand_bindgen::CompositeOperator_DstOutCompositeOp
            }
            CompositeOperator::DstOverCompositeOp => {
                sledash_magickwand_bindgen::CompositeOperator_DstOverCompositeOp
            }
            CompositeOperator::ExclusionCompositeOp => {
                sledash_magickwand_bindgen::CompositeOperator_ExclusionCompositeOp
            }
            CompositeOperator::HardLightCompositeOp => {
                sledash_magickwand_bindgen::CompositeOperator_HardLightCompositeOp
            }
            CompositeOperator::HardMixCompositeOp => {
                sledash_magickwand_bindgen::CompositeOperator_HardMixCompositeOp
            }
            CompositeOperator::HueCompositeOp => {
                sledash_magickwand_bindgen::CompositeOperator_HueCompositeOp
            }
            CompositeOperator::InCompositeOp => {
                sledash_magickwand_bindgen::CompositeOperator_InCompositeOp
            }
            CompositeOperator::IntensityCompositeOp => {
                sledash_magickwand_bindgen::CompositeOperator_IntensityCompositeOp
            }
            CompositeOperator::LightenCompositeOp => {
                sledash_magickwand_bindgen::CompositeOperator_LightenCompositeOp
            }
            CompositeOperator::LightenIntensityCompositeOp => {
                sledash_magickwand_bindgen::CompositeOperator_LightenIntensityCompositeOp
            }
            CompositeOperator::LinearBurnCompositeOp => {
                sledash_magickwand_bindgen::CompositeOperator_LinearBurnCompositeOp
            }
            CompositeOperator::LinearDodgeCompositeOp => {
                sledash_magickwand_bindgen::CompositeOperator_LinearDodgeCompositeOp
            }
            CompositeOperator::LinearLightCompositeOp => {
                sledash_magickwand_bindgen::CompositeOperator_LinearLightCompositeOp
            }
            CompositeOperator::LuminizeCompositeOp => {
                sledash_magickwand_bindgen::CompositeOperator_LuminizeCompositeOp
            }
            CompositeOperator::MathematicsCompositeOp => {
                sledash_magickwand_bindgen::CompositeOperator_MathematicsCompositeOp
            }
            CompositeOperator::MinusDstCompositeOp => {
                sledash_magickwand_bindgen::CompositeOperator_MinusDstCompositeOp
            }
            CompositeOperator::MinusSrcCompositeOp => {
                sledash_magickwand_bindgen::CompositeOperator_MinusSrcCompositeOp
            }
            CompositeOperator::ModulateCompositeOp => {
                sledash_magickwand_bindgen::CompositeOperator_ModulateCompositeOp
            }
            CompositeOperator::ModulusAddCompositeOp => {
                sledash_magickwand_bindgen::CompositeOperator_ModulusAddCompositeOp
            }
            CompositeOperator::ModulusSubtractCompositeOp => {
                sledash_magickwand_bindgen::CompositeOperator_ModulusSubtractCompositeOp
            }
            CompositeOperator::MultiplyCompositeOp => {
                sledash_magickwand_bindgen::CompositeOperator_MultiplyCompositeOp
            }
            CompositeOperator::NoCompositeOp => {
                sledash_magickwand_bindgen::CompositeOperator_NoCompositeOp
            }
            CompositeOperator::OutCompositeOp => {
                sledash_magickwand_bindgen::CompositeOperator_OutCompositeOp
            }
            CompositeOperator::OverCompositeOp => {
                sledash_magickwand_bindgen::CompositeOperator_OverCompositeOp
            }
            CompositeOperator::OverlayCompositeOp => {
                sledash_magickwand_bindgen::CompositeOperator_OverlayCompositeOp
            }
            CompositeOperator::PegtopLightCompositeOp => {
                sledash_magickwand_bindgen::CompositeOperator_PegtopLightCompositeOp
            }
            CompositeOperator::PinLightCompositeOp => {
                sledash_magickwand_bindgen::CompositeOperator_PinLightCompositeOp
            }
            CompositeOperator::PlusCompositeOp => {
                sledash_magickwand_bindgen::CompositeOperator_PlusCompositeOp
            }
            CompositeOperator::ReplaceCompositeOp => {
                sledash_magickwand_bindgen::CompositeOperator_ReplaceCompositeOp
            }
            CompositeOperator::SaturateCompositeOp => {
                sledash_magickwand_bindgen::CompositeOperator_SaturateCompositeOp
            }
            CompositeOperator::ScreenCompositeOp => {
                sledash_magickwand_bindgen::CompositeOperator_ScreenCompositeOp
            }
            CompositeOperator::SoftLightCompositeOp => {
                sledash_magickwand_bindgen::CompositeOperator_SoftLightCompositeOp
            }
            CompositeOperator::SrcAtopCompositeOp => {
                sledash_magickwand_bindgen::CompositeOperator_SrcAtopCompositeOp
            }
            CompositeOperator::SrcCompositeOp => {
                sledash_magickwand_bindgen::CompositeOperator_SrcCompositeOp
            }
            CompositeOperator::SrcInCompositeOp => {
                sledash_magickwand_bindgen::CompositeOperator_SrcInCompositeOp
            }
            CompositeOperator::SrcOutCompositeOp => {
                sledash_magickwand_bindgen::CompositeOperator_SrcOutCompositeOp
            }
            CompositeOperator::SrcOverCompositeOp => {
                sledash_magickwand_bindgen::CompositeOperator_SrcOverCompositeOp
            }
            CompositeOperator::ThresholdCompositeOp => {
                sledash_magickwand_bindgen::CompositeOperator_ThresholdCompositeOp
            }
            CompositeOperator::VividLightCompositeOp => {
                sledash_magickwand_bindgen::CompositeOperator_VividLightCompositeOp
            }
            CompositeOperator::XorCompositeOp => {
                sledash_magickwand_bindgen::CompositeOperator_XorCompositeOp
            }
            CompositeOperator::StereoCompositeOp => {
                sledash_magickwand_bindgen::CompositeOperator_StereoCompositeOp
            }
            CompositeOperator::FreezeCompositeOp => {
                sledash_magickwand_bindgen::CompositeOperator_FreezeCompositeOp
            }
            CompositeOperator::InterpolateCompositeOp => {
                sledash_magickwand_bindgen::CompositeOperator_InterpolateCompositeOp
            }
            CompositeOperator::NegateCompositeOp => {
                sledash_magickwand_bindgen::CompositeOperator_NegateCompositeOp
            }
            CompositeOperator::ReflectCompositeOp => {
                sledash_magickwand_bindgen::CompositeOperator_ReflectCompositeOp
            }
            CompositeOperator::SoftBurnCompositeOp => {
                sledash_magickwand_bindgen::CompositeOperator_SoftBurnCompositeOp
            }
            CompositeOperator::SoftDodgeCompositeOp => {
                sledash_magickwand_bindgen::CompositeOperator_SoftDodgeCompositeOp
            }
            CompositeOperator::StampCompositeOp => {
                sledash_magickwand_bindgen::CompositeOperator_StampCompositeOp
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

impl Into<sledash_magickwand_bindgen::GravityType> for GravityType {
    fn into(self) -> sledash_magickwand_bindgen::GravityType {
        match self {
            GravityType::UndefinedGravity => {
                sledash_magickwand_bindgen::GravityType_UndefinedGravity
            }
            GravityType::ForgetGravity => sledash_magickwand_bindgen::GravityType_ForgetGravity,
            GravityType::NorthWestGravity => {
                sledash_magickwand_bindgen::GravityType_NorthWestGravity
            }
            GravityType::NorthGravity => sledash_magickwand_bindgen::GravityType_NorthGravity,
            GravityType::NorthEastGravity => {
                sledash_magickwand_bindgen::GravityType_NorthEastGravity
            }
            GravityType::WestGravity => sledash_magickwand_bindgen::GravityType_WestGravity,
            GravityType::CenterGravity => sledash_magickwand_bindgen::GravityType_CenterGravity,
            GravityType::EastGravity => sledash_magickwand_bindgen::GravityType_EastGravity,
            GravityType::SouthWestGravity => {
                sledash_magickwand_bindgen::GravityType_SouthWestGravity
            }
            GravityType::SouthGravity => sledash_magickwand_bindgen::GravityType_SouthGravity,
            GravityType::SouthEastGravity => {
                sledash_magickwand_bindgen::GravityType_SouthEastGravity
            }
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

impl Into<sledash_magickwand_bindgen::FilterType> for FilterType {
    fn into(self) -> sledash_magickwand_bindgen::FilterType {
        match self {
            FilterType::UndefinedFilter => sledash_magickwand_bindgen::FilterType_UndefinedFilter,
            FilterType::PointFilter => sledash_magickwand_bindgen::FilterType_PointFilter,
            FilterType::BoxFilter => sledash_magickwand_bindgen::FilterType_BoxFilter,
            FilterType::TriangleFilter => sledash_magickwand_bindgen::FilterType_TriangleFilter,
            FilterType::HermiteFilter => sledash_magickwand_bindgen::FilterType_HermiteFilter,
            FilterType::HannFilter => sledash_magickwand_bindgen::FilterType_HannFilter,
            FilterType::HammingFilter => sledash_magickwand_bindgen::FilterType_HammingFilter,
            FilterType::BlackmanFilter => sledash_magickwand_bindgen::FilterType_BlackmanFilter,
            FilterType::GaussianFilter => sledash_magickwand_bindgen::FilterType_GaussianFilter,
            FilterType::QuadraticFilter => sledash_magickwand_bindgen::FilterType_QuadraticFilter,
            FilterType::CubicFilter => sledash_magickwand_bindgen::FilterType_CubicFilter,
            FilterType::CatromFilter => sledash_magickwand_bindgen::FilterType_CatromFilter,
            FilterType::MitchellFilter => sledash_magickwand_bindgen::FilterType_MitchellFilter,
            FilterType::JincFilter => sledash_magickwand_bindgen::FilterType_JincFilter,
            FilterType::SincFilter => sledash_magickwand_bindgen::FilterType_SincFilter,
            FilterType::SincFastFilter => sledash_magickwand_bindgen::FilterType_SincFastFilter,
            FilterType::KaiserFilter => sledash_magickwand_bindgen::FilterType_KaiserFilter,
            FilterType::WelchFilter => sledash_magickwand_bindgen::FilterType_WelchFilter,
            FilterType::ParzenFilter => sledash_magickwand_bindgen::FilterType_ParzenFilter,
            FilterType::BohmanFilter => sledash_magickwand_bindgen::FilterType_BohmanFilter,
            FilterType::BartlettFilter => sledash_magickwand_bindgen::FilterType_BartlettFilter,
            FilterType::LagrangeFilter => sledash_magickwand_bindgen::FilterType_LagrangeFilter,
            FilterType::LanczosFilter => sledash_magickwand_bindgen::FilterType_LanczosFilter,
            FilterType::LanczosSharpFilter => {
                sledash_magickwand_bindgen::FilterType_LanczosSharpFilter
            }
            FilterType::Lanczos2Filter => sledash_magickwand_bindgen::FilterType_Lanczos2Filter,
            FilterType::Lanczos2SharpFilter => {
                sledash_magickwand_bindgen::FilterType_Lanczos2SharpFilter
            }
            FilterType::RobidouxFilter => sledash_magickwand_bindgen::FilterType_RobidouxFilter,
            FilterType::RobidouxSharpFilter => {
                sledash_magickwand_bindgen::FilterType_RobidouxSharpFilter
            }
            FilterType::CosineFilter => sledash_magickwand_bindgen::FilterType_CosineFilter,
            FilterType::SplineFilter => sledash_magickwand_bindgen::FilterType_SplineFilter,
            FilterType::LanczosRadiusFilter => {
                sledash_magickwand_bindgen::FilterType_LanczosRadiusFilter
            }
            FilterType::CubicSplineFilter => {
                sledash_magickwand_bindgen::FilterType_CubicSplineFilter
            }
            FilterType::SentinelFilter => sledash_magickwand_bindgen::FilterType_SentinelFilter,
        }
    }
}
