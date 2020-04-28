use std::fmt;

#[derive(Debug)]
pub enum ExceptionType {
    UndefinedException,
    WarningException,
    ResourceLimitWarning,
    TypeWarning,
    OptionWarning,
    DelegateWarning,
    MissingDelegateWarning,
    CorruptImageWarning,
    FileOpenWarning,
    BlobWarning,
    StreamWarning,
    CacheWarning,
    CoderWarning,
    FilterWarning,
    ModuleWarning,
    DrawWarning,
    ImageWarning,
    WandWarning,
    RandomWarning,
    XServerWarning,
    MonitorWarning,
    RegistryWarning,
    ConfigureWarning,
    PolicyWarning,
    ErrorException,
    ResourceLimitError,
    TypeError,
    OptionError,
    DelegateError,
    MissingDelegateError,
    CorruptImageError,
    FileOpenError,
    BlobError,
    StreamError,
    CacheError,
    CoderError,
    FilterError,
    ModuleError,
    DrawError,
    ImageError,
    WandError,
    RandomError,
    XServerError,
    MonitorError,
    RegistryError,
    ConfigureError,
    PolicyError,
    FatalErrorException,
    ResourceLimitFatalError,
    TypeFatalError,
    OptionFatalError,
    DelegateFatalError,
    MissingDelegateFatalError,
    CorruptImageFatalError,
    FileOpenFatalError,
    BlobFatalError,
    StreamFatalError,
    CacheFatalError,
    CoderFatalError,
    FilterFatalError,
    ModuleFatalError,
    DrawFatalError,
    ImageFatalError,
    WandFatalError,
    RandomFatalError,
    XServerFatalError,
    MonitorFatalError,
    RegistryFatalError,
    ConfigureFatalError,
    PolicyFatalError,
}

#[allow(unreachable_patterns)]
pub(crate) fn get_exception_type(
    exception_type: magickwand_bindgen::ExceptionType,
) -> ExceptionType {
    match exception_type {
        magickwand_bindgen::ExceptionType_UndefinedException => ExceptionType::UndefinedException,
        magickwand_bindgen::ExceptionType_WarningException => ExceptionType::WarningException,
        magickwand_bindgen::ExceptionType_ResourceLimitWarning => {
            ExceptionType::ResourceLimitWarning
        }
        magickwand_bindgen::ExceptionType_TypeWarning => ExceptionType::TypeWarning,
        magickwand_bindgen::ExceptionType_OptionWarning => ExceptionType::OptionWarning,
        magickwand_bindgen::ExceptionType_DelegateWarning => ExceptionType::DelegateWarning,
        magickwand_bindgen::ExceptionType_MissingDelegateWarning => {
            ExceptionType::MissingDelegateWarning
        }
        magickwand_bindgen::ExceptionType_CorruptImageWarning => ExceptionType::CorruptImageWarning,
        magickwand_bindgen::ExceptionType_FileOpenWarning => ExceptionType::FileOpenWarning,
        magickwand_bindgen::ExceptionType_BlobWarning => ExceptionType::BlobWarning,
        magickwand_bindgen::ExceptionType_StreamWarning => ExceptionType::StreamWarning,
        magickwand_bindgen::ExceptionType_CacheWarning => ExceptionType::CacheWarning,
        magickwand_bindgen::ExceptionType_CoderWarning => ExceptionType::CoderWarning,
        magickwand_bindgen::ExceptionType_FilterWarning => ExceptionType::FilterWarning,
        magickwand_bindgen::ExceptionType_ModuleWarning => ExceptionType::ModuleWarning,
        magickwand_bindgen::ExceptionType_DrawWarning => ExceptionType::DrawWarning,
        magickwand_bindgen::ExceptionType_ImageWarning => ExceptionType::ImageWarning,
        magickwand_bindgen::ExceptionType_WandWarning => ExceptionType::WandWarning,
        magickwand_bindgen::ExceptionType_RandomWarning => ExceptionType::RandomWarning,
        magickwand_bindgen::ExceptionType_XServerWarning => ExceptionType::XServerWarning,
        magickwand_bindgen::ExceptionType_MonitorWarning => ExceptionType::MonitorWarning,
        magickwand_bindgen::ExceptionType_RegistryWarning => ExceptionType::RegistryWarning,
        magickwand_bindgen::ExceptionType_ConfigureWarning => ExceptionType::ConfigureWarning,
        magickwand_bindgen::ExceptionType_PolicyWarning => ExceptionType::PolicyWarning,
        magickwand_bindgen::ExceptionType_ErrorException => ExceptionType::ErrorException,
        magickwand_bindgen::ExceptionType_ResourceLimitError => ExceptionType::ResourceLimitError,
        magickwand_bindgen::ExceptionType_TypeError => ExceptionType::TypeError,
        magickwand_bindgen::ExceptionType_OptionError => ExceptionType::OptionError,
        magickwand_bindgen::ExceptionType_DelegateError => ExceptionType::DelegateError,
        magickwand_bindgen::ExceptionType_MissingDelegateError => {
            ExceptionType::MissingDelegateError
        }
        magickwand_bindgen::ExceptionType_CorruptImageError => ExceptionType::CorruptImageError,
        magickwand_bindgen::ExceptionType_FileOpenError => ExceptionType::FileOpenError,
        magickwand_bindgen::ExceptionType_BlobError => ExceptionType::BlobError,
        magickwand_bindgen::ExceptionType_StreamError => ExceptionType::StreamError,
        magickwand_bindgen::ExceptionType_CacheError => ExceptionType::CacheError,
        magickwand_bindgen::ExceptionType_CoderError => ExceptionType::CoderError,
        magickwand_bindgen::ExceptionType_FilterError => ExceptionType::FilterError,
        magickwand_bindgen::ExceptionType_ModuleError => ExceptionType::ModuleError,
        magickwand_bindgen::ExceptionType_DrawError => ExceptionType::DrawError,
        magickwand_bindgen::ExceptionType_ImageError => ExceptionType::ImageError,
        magickwand_bindgen::ExceptionType_WandError => ExceptionType::WandError,
        magickwand_bindgen::ExceptionType_RandomError => ExceptionType::RandomError,
        magickwand_bindgen::ExceptionType_XServerError => ExceptionType::XServerError,
        magickwand_bindgen::ExceptionType_MonitorError => ExceptionType::MonitorError,
        magickwand_bindgen::ExceptionType_RegistryError => ExceptionType::RegistryError,
        magickwand_bindgen::ExceptionType_ConfigureError => ExceptionType::ConfigureError,
        magickwand_bindgen::ExceptionType_PolicyError => ExceptionType::PolicyError,
        magickwand_bindgen::ExceptionType_FatalErrorException => ExceptionType::FatalErrorException,
        magickwand_bindgen::ExceptionType_ResourceLimitFatalError => {
            ExceptionType::ResourceLimitFatalError
        }
        magickwand_bindgen::ExceptionType_TypeFatalError => ExceptionType::TypeFatalError,
        magickwand_bindgen::ExceptionType_OptionFatalError => ExceptionType::OptionFatalError,
        magickwand_bindgen::ExceptionType_DelegateFatalError => ExceptionType::DelegateFatalError,
        magickwand_bindgen::ExceptionType_MissingDelegateFatalError => {
            ExceptionType::MissingDelegateFatalError
        }
        magickwand_bindgen::ExceptionType_CorruptImageFatalError => {
            ExceptionType::CorruptImageFatalError
        }
        magickwand_bindgen::ExceptionType_FileOpenFatalError => ExceptionType::FileOpenFatalError,
        magickwand_bindgen::ExceptionType_BlobFatalError => ExceptionType::BlobFatalError,
        magickwand_bindgen::ExceptionType_StreamFatalError => ExceptionType::StreamFatalError,
        magickwand_bindgen::ExceptionType_CacheFatalError => ExceptionType::CacheFatalError,
        magickwand_bindgen::ExceptionType_CoderFatalError => ExceptionType::CoderFatalError,
        magickwand_bindgen::ExceptionType_FilterFatalError => ExceptionType::FilterFatalError,
        magickwand_bindgen::ExceptionType_ModuleFatalError => ExceptionType::ModuleFatalError,
        magickwand_bindgen::ExceptionType_DrawFatalError => ExceptionType::DrawFatalError,
        magickwand_bindgen::ExceptionType_ImageFatalError => ExceptionType::ImageFatalError,
        magickwand_bindgen::ExceptionType_WandFatalError => ExceptionType::WandFatalError,
        magickwand_bindgen::ExceptionType_RandomFatalError => ExceptionType::RandomFatalError,
        magickwand_bindgen::ExceptionType_XServerFatalError => ExceptionType::XServerFatalError,
        magickwand_bindgen::ExceptionType_MonitorFatalError => ExceptionType::MonitorFatalError,
        magickwand_bindgen::ExceptionType_RegistryFatalError => ExceptionType::RegistryFatalError,
        magickwand_bindgen::ExceptionType_ConfigureFatalError => ExceptionType::ConfigureFatalError,
        magickwand_bindgen::ExceptionType_PolicyFatalError => ExceptionType::PolicyFatalError,
        _ => ExceptionType::UndefinedException,
    }
}

impl fmt::Display for ExceptionType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "ExceptionType::{:?}", self)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use std::fmt::Write;

    #[test]
    fn exception_type_displayed() {
        let exception_type = ExceptionType::WandWarning;

        let mut output = String::new();
        write!(&mut output, "{}", exception_type).unwrap();
        assert_eq!(output, "ExceptionType::WandWarning");
    }
}
