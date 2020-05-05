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
    exception_type: sledash_magickwand_bindgen::ExceptionType,
) -> ExceptionType {
    match exception_type {
        sledash_magickwand_bindgen::ExceptionType_UndefinedException => {
            ExceptionType::UndefinedException
        }
        sledash_magickwand_bindgen::ExceptionType_WarningException => {
            ExceptionType::WarningException
        }
        sledash_magickwand_bindgen::ExceptionType_ResourceLimitWarning => {
            ExceptionType::ResourceLimitWarning
        }
        sledash_magickwand_bindgen::ExceptionType_TypeWarning => ExceptionType::TypeWarning,
        sledash_magickwand_bindgen::ExceptionType_OptionWarning => ExceptionType::OptionWarning,
        sledash_magickwand_bindgen::ExceptionType_DelegateWarning => ExceptionType::DelegateWarning,
        sledash_magickwand_bindgen::ExceptionType_MissingDelegateWarning => {
            ExceptionType::MissingDelegateWarning
        }
        sledash_magickwand_bindgen::ExceptionType_CorruptImageWarning => {
            ExceptionType::CorruptImageWarning
        }
        sledash_magickwand_bindgen::ExceptionType_FileOpenWarning => ExceptionType::FileOpenWarning,
        sledash_magickwand_bindgen::ExceptionType_BlobWarning => ExceptionType::BlobWarning,
        sledash_magickwand_bindgen::ExceptionType_StreamWarning => ExceptionType::StreamWarning,
        sledash_magickwand_bindgen::ExceptionType_CacheWarning => ExceptionType::CacheWarning,
        sledash_magickwand_bindgen::ExceptionType_CoderWarning => ExceptionType::CoderWarning,
        sledash_magickwand_bindgen::ExceptionType_FilterWarning => ExceptionType::FilterWarning,
        sledash_magickwand_bindgen::ExceptionType_ModuleWarning => ExceptionType::ModuleWarning,
        sledash_magickwand_bindgen::ExceptionType_DrawWarning => ExceptionType::DrawWarning,
        sledash_magickwand_bindgen::ExceptionType_ImageWarning => ExceptionType::ImageWarning,
        sledash_magickwand_bindgen::ExceptionType_WandWarning => ExceptionType::WandWarning,
        sledash_magickwand_bindgen::ExceptionType_RandomWarning => ExceptionType::RandomWarning,
        sledash_magickwand_bindgen::ExceptionType_XServerWarning => ExceptionType::XServerWarning,
        sledash_magickwand_bindgen::ExceptionType_MonitorWarning => ExceptionType::MonitorWarning,
        sledash_magickwand_bindgen::ExceptionType_RegistryWarning => ExceptionType::RegistryWarning,
        sledash_magickwand_bindgen::ExceptionType_ConfigureWarning => {
            ExceptionType::ConfigureWarning
        }
        sledash_magickwand_bindgen::ExceptionType_PolicyWarning => ExceptionType::PolicyWarning,
        sledash_magickwand_bindgen::ExceptionType_ErrorException => ExceptionType::ErrorException,
        sledash_magickwand_bindgen::ExceptionType_ResourceLimitError => {
            ExceptionType::ResourceLimitError
        }
        sledash_magickwand_bindgen::ExceptionType_TypeError => ExceptionType::TypeError,
        sledash_magickwand_bindgen::ExceptionType_OptionError => ExceptionType::OptionError,
        sledash_magickwand_bindgen::ExceptionType_DelegateError => ExceptionType::DelegateError,
        sledash_magickwand_bindgen::ExceptionType_MissingDelegateError => {
            ExceptionType::MissingDelegateError
        }
        sledash_magickwand_bindgen::ExceptionType_CorruptImageError => {
            ExceptionType::CorruptImageError
        }
        sledash_magickwand_bindgen::ExceptionType_FileOpenError => ExceptionType::FileOpenError,
        sledash_magickwand_bindgen::ExceptionType_BlobError => ExceptionType::BlobError,
        sledash_magickwand_bindgen::ExceptionType_StreamError => ExceptionType::StreamError,
        sledash_magickwand_bindgen::ExceptionType_CacheError => ExceptionType::CacheError,
        sledash_magickwand_bindgen::ExceptionType_CoderError => ExceptionType::CoderError,
        sledash_magickwand_bindgen::ExceptionType_FilterError => ExceptionType::FilterError,
        sledash_magickwand_bindgen::ExceptionType_ModuleError => ExceptionType::ModuleError,
        sledash_magickwand_bindgen::ExceptionType_DrawError => ExceptionType::DrawError,
        sledash_magickwand_bindgen::ExceptionType_ImageError => ExceptionType::ImageError,
        sledash_magickwand_bindgen::ExceptionType_WandError => ExceptionType::WandError,
        sledash_magickwand_bindgen::ExceptionType_RandomError => ExceptionType::RandomError,
        sledash_magickwand_bindgen::ExceptionType_XServerError => ExceptionType::XServerError,
        sledash_magickwand_bindgen::ExceptionType_MonitorError => ExceptionType::MonitorError,
        sledash_magickwand_bindgen::ExceptionType_RegistryError => ExceptionType::RegistryError,
        sledash_magickwand_bindgen::ExceptionType_ConfigureError => ExceptionType::ConfigureError,
        sledash_magickwand_bindgen::ExceptionType_PolicyError => ExceptionType::PolicyError,
        sledash_magickwand_bindgen::ExceptionType_FatalErrorException => {
            ExceptionType::FatalErrorException
        }
        sledash_magickwand_bindgen::ExceptionType_ResourceLimitFatalError => {
            ExceptionType::ResourceLimitFatalError
        }
        sledash_magickwand_bindgen::ExceptionType_TypeFatalError => ExceptionType::TypeFatalError,
        sledash_magickwand_bindgen::ExceptionType_OptionFatalError => {
            ExceptionType::OptionFatalError
        }
        sledash_magickwand_bindgen::ExceptionType_DelegateFatalError => {
            ExceptionType::DelegateFatalError
        }
        sledash_magickwand_bindgen::ExceptionType_MissingDelegateFatalError => {
            ExceptionType::MissingDelegateFatalError
        }
        sledash_magickwand_bindgen::ExceptionType_CorruptImageFatalError => {
            ExceptionType::CorruptImageFatalError
        }
        sledash_magickwand_bindgen::ExceptionType_FileOpenFatalError => {
            ExceptionType::FileOpenFatalError
        }
        sledash_magickwand_bindgen::ExceptionType_BlobFatalError => ExceptionType::BlobFatalError,
        sledash_magickwand_bindgen::ExceptionType_StreamFatalError => {
            ExceptionType::StreamFatalError
        }
        sledash_magickwand_bindgen::ExceptionType_CacheFatalError => ExceptionType::CacheFatalError,
        sledash_magickwand_bindgen::ExceptionType_CoderFatalError => ExceptionType::CoderFatalError,
        sledash_magickwand_bindgen::ExceptionType_FilterFatalError => {
            ExceptionType::FilterFatalError
        }
        sledash_magickwand_bindgen::ExceptionType_ModuleFatalError => {
            ExceptionType::ModuleFatalError
        }
        sledash_magickwand_bindgen::ExceptionType_DrawFatalError => ExceptionType::DrawFatalError,
        sledash_magickwand_bindgen::ExceptionType_ImageFatalError => ExceptionType::ImageFatalError,
        sledash_magickwand_bindgen::ExceptionType_WandFatalError => ExceptionType::WandFatalError,
        sledash_magickwand_bindgen::ExceptionType_RandomFatalError => {
            ExceptionType::RandomFatalError
        }
        sledash_magickwand_bindgen::ExceptionType_XServerFatalError => {
            ExceptionType::XServerFatalError
        }
        sledash_magickwand_bindgen::ExceptionType_MonitorFatalError => {
            ExceptionType::MonitorFatalError
        }
        sledash_magickwand_bindgen::ExceptionType_RegistryFatalError => {
            ExceptionType::RegistryFatalError
        }
        sledash_magickwand_bindgen::ExceptionType_ConfigureFatalError => {
            ExceptionType::ConfigureFatalError
        }
        sledash_magickwand_bindgen::ExceptionType_PolicyFatalError => {
            ExceptionType::PolicyFatalError
        }
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
