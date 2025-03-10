// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

#[diplomat::bridge]
#[diplomat::abi_rename = "icu4x_{0}_mv1"]
#[diplomat::attr(auto, namespace = "icu4x")]
pub mod ffi {
    use alloc::boxed::Box;

    use crate::errors::ffi::DataError;
    use crate::locale_core::ffi::Locale;
    use crate::provider::ffi::DataProvider;

    #[diplomat::opaque]
    /// An ICU4X Unicode Set Property object, capable of querying whether a code point is contained in a set based on a Unicode property.
    #[diplomat::rust_link(icu::locale, Mod)]
    #[diplomat::rust_link(icu::locale::exemplar_chars::ExemplarCharacters, Struct)]
    #[diplomat::rust_link(icu::locale::exemplar_chars::ExemplarCharactersBorrowed, Struct)]
    pub struct ExemplarCharacters(pub icu_locale::exemplar_chars::ExemplarCharacters);

    impl ExemplarCharacters {
        /// Checks whether the string is in the set.
        #[diplomat::rust_link(
            icu::collections::codepointinvliststringlist::CodePointInversionListAndStringList::contains_str,
            FnInStruct
        )]
        #[diplomat::attr(supports = method_overloading, rename = "contains")]
        pub fn contains_str(&self, s: &DiplomatStr) -> bool {
            let Ok(s) = core::str::from_utf8(s) else {
                return false;
            };
            self.0.as_borrowed().contains_str(s)
        }
        /// Checks whether the code point is in the set.
        #[diplomat::rust_link(
            icu::collections::codepointinvliststringlist::CodePointInversionListAndStringList::contains,
            FnInStruct
        )]
        #[diplomat::rust_link(
            icu::collections::codepointinvliststringlist::CodePointInversionListAndStringList::contains32,
            FnInStruct,
            hidden
        )]
        pub fn contains(&self, cp: DiplomatChar) -> bool {
            self.0.as_borrowed().contains32(cp)
        }

        #[diplomat::rust_link(
            icu::locale::exemplar_chars::ExemplarCharacters::try_new_main,
            FnInStruct
        )]
        #[diplomat::attr(supports = fallible_constructors, named_constructor = "main")]
        #[cfg(feature = "compiled_data")]
        pub fn create_main(locale: &Locale) -> Result<Box<ExemplarCharacters>, DataError> {
            let locale = locale.to_datalocale();
            Ok(Box::new(ExemplarCharacters(
                icu_locale::exemplar_chars::ExemplarCharacters::try_new_main(&locale)?
                    .static_to_owned(),
            )))
        }

        #[diplomat::rust_link(
            icu::locale::exemplar_chars::ExemplarCharacters::try_new_main,
            FnInStruct
        )]
        #[diplomat::attr(supports = fallible_constructors, named_constructor = "main_with_provider")]
        pub fn create_main_with_provider(
            provider: &DataProvider,
            locale: &Locale,
        ) -> Result<Box<ExemplarCharacters>, DataError> {
            let locale = locale.to_datalocale();
            Ok(Box::new(ExemplarCharacters(call_constructor_unstable!(
                icu_locale::exemplar_chars::ExemplarCharacters::try_new_main [r => r.map(|r| r.static_to_owned())],
                icu_locale::exemplar_chars::ExemplarCharacters::try_new_main_unstable,
                provider,
                &locale
            )?)))
        }

        #[diplomat::rust_link(
            icu::locale::exemplar_chars::ExemplarCharacters::try_new_auxiliary,
            FnInStruct
        )]
        #[diplomat::attr(supports = fallible_constructors, named_constructor = "auxiliary")]
        #[cfg(feature = "compiled_data")]
        pub fn create_auxiliary(locale: &Locale) -> Result<Box<ExemplarCharacters>, DataError> {
            let locale = locale.to_datalocale();
            Ok(Box::new(ExemplarCharacters(
                icu_locale::exemplar_chars::ExemplarCharacters::try_new_auxiliary(&locale)?
                    .static_to_owned(),
            )))
        }

        #[diplomat::rust_link(
            icu::locale::exemplar_chars::ExemplarCharacters::try_new_auxiliary,
            FnInStruct
        )]
        #[diplomat::attr(supports = fallible_constructors, named_constructor = "auxiliary_with_provider")]
        pub fn create_auxiliary_with_provider(
            provider: &DataProvider,
            locale: &Locale,
        ) -> Result<Box<ExemplarCharacters>, DataError> {
            let locale = locale.to_datalocale();
            Ok(Box::new(ExemplarCharacters(call_constructor_unstable!(
                icu_locale::exemplar_chars::ExemplarCharacters::try_new_auxiliary [r => r.map(|r| r.static_to_owned())],
                icu_locale::exemplar_chars::ExemplarCharacters::try_new_auxiliary_unstable,
                provider,
                &locale
            )?)))
        }

        #[diplomat::rust_link(
            icu::locale::exemplar_chars::ExemplarCharacters::try_new_punctuation,
            FnInStruct
        )]
        #[diplomat::attr(supports = fallible_constructors, named_constructor = "punctuation")]
        #[cfg(feature = "compiled_data")]
        pub fn create_punctuation(locale: &Locale) -> Result<Box<ExemplarCharacters>, DataError> {
            let locale = locale.to_datalocale();
            Ok(Box::new(ExemplarCharacters(
                icu_locale::exemplar_chars::ExemplarCharacters::try_new_punctuation(&locale)?
                    .static_to_owned(),
            )))
        }

        #[diplomat::rust_link(
            icu::locale::exemplar_chars::ExemplarCharacters::try_new_punctuation,
            FnInStruct
        )]
        #[diplomat::attr(supports = fallible_constructors, named_constructor = "punctuation_with_provider")]
        pub fn create_punctuation_with_provider(
            provider: &DataProvider,
            locale: &Locale,
        ) -> Result<Box<ExemplarCharacters>, DataError> {
            let locale = locale.to_datalocale();
            Ok(Box::new(ExemplarCharacters(call_constructor_unstable!(
                icu_locale::exemplar_chars::ExemplarCharacters::try_new_punctuation [r => r.map(|r| r.static_to_owned())],
                icu_locale::exemplar_chars::ExemplarCharacters::try_new_punctuation_unstable,
                provider,
                &locale
            )?)))
        }
        #[diplomat::rust_link(
            icu::locale::exemplar_chars::ExemplarCharacters::try_new_numbers,
            FnInStruct
        )]
        #[diplomat::attr(supports = fallible_constructors, named_constructor = "numbers")]
        #[cfg(feature = "compiled_data")]
        pub fn create_numbers(locale: &Locale) -> Result<Box<ExemplarCharacters>, DataError> {
            let locale = locale.to_datalocale();
            Ok(Box::new(ExemplarCharacters(
                icu_locale::exemplar_chars::ExemplarCharacters::try_new_numbers(&locale)?
                    .static_to_owned(),
            )))
        }
        #[diplomat::rust_link(
            icu::locale::exemplar_chars::ExemplarCharacters::try_new_numbers,
            FnInStruct
        )]
        #[diplomat::attr(supports = fallible_constructors, named_constructor = "numbers_with_provider")]
        pub fn create_numbers_with_provider(
            provider: &DataProvider,
            locale: &Locale,
        ) -> Result<Box<ExemplarCharacters>, DataError> {
            let locale = locale.to_datalocale();
            Ok(Box::new(ExemplarCharacters(call_constructor_unstable!(
                icu_locale::exemplar_chars::ExemplarCharacters::try_new_numbers [r => r.map(|r| r.static_to_owned())],
                icu_locale::exemplar_chars::ExemplarCharacters::try_new_numbers_unstable,
                provider,
                &locale
            )?)))
        }

        #[diplomat::rust_link(
            icu::locale::exemplar_chars::ExemplarCharacters::try_new_index,
            FnInStruct
        )]
        #[diplomat::attr(supports = fallible_constructors, named_constructor = "index")]
        #[cfg(feature = "compiled_data")]
        pub fn create_index(locale: &Locale) -> Result<Box<ExemplarCharacters>, DataError> {
            let locale = locale.to_datalocale();
            Ok(Box::new(ExemplarCharacters(
                icu_locale::exemplar_chars::ExemplarCharacters::try_new_index(&locale)?
                    .static_to_owned(),
            )))
        }

        #[diplomat::rust_link(
            icu::locale::exemplar_chars::ExemplarCharacters::try_new_index,
            FnInStruct
        )]
        #[diplomat::attr(supports = fallible_constructors, named_constructor = "index_with_provider")]
        pub fn create_index_with_provider(
            provider: &DataProvider,
            locale: &Locale,
        ) -> Result<Box<ExemplarCharacters>, DataError> {
            let locale = locale.to_datalocale();
            Ok(Box::new(ExemplarCharacters(call_constructor_unstable!(
                icu_locale::exemplar_chars::ExemplarCharacters::try_new_index [r => r.map(|r| r.static_to_owned())],
                icu_locale::exemplar_chars::ExemplarCharacters::try_new_index_unstable,
                provider,
                &locale
            )?)))
        }
    }
}
