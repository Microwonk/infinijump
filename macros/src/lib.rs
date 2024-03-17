use proc_macro::TokenStream;
use quote::quote;
use syn::{
    parse::{Parse, ParseStream},
    parse_macro_input,
    punctuated::Punctuated,
    DeriveInput, Ident, LitStr, Token,
};

struct Extensions {
    extensions: Vec<LitStr>,
}

impl Extensions {
    const NAME: &'static str = "extensions";
}

impl Parse for Extensions {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        // Parse the ident and '='
        let ident: Ident = input.parse()?;
        input.parse::<Token![=]>()?;

        if ident != Self::NAME {
            panic!()
        }

        // Parse the '['
        let content;
        syn::bracketed!(content in input);

        // Parse the extensions
        let extensions = Punctuated::<LitStr, Token![,]>::parse_terminated(&content)?;

        Ok(Extensions {
            extensions: extensions.into_iter().collect(),
        })
    }
}

/// automagically generates all you would need for making your own custom assets
///
/// # How To
/// ```
/// // need to import these:
/// use bevy::asset::AsyncReadExt;
/// use bevy::utils::thiserror;
///
/// // then use this macro on a struct like this:
/// #[auto_asset_loader(extensions = ["some.ron", "some.else.ron"])]
/// #[derive(Deserialize, Debug)]
/// pub struct SomeAsset {
///     health: u32,
/// }
///
/// // this will generate code like this:
///
/// #[derive(Asset, TypePath, Deserialize, Debug)]
/// pub struct SomeAsset {
///     health: u32,
/// }
///
/// #[non_exhaustive] // Yoinked from bevy's examples
/// #[derive(Debug, thiserror::Error)]
/// pub enum SomeAssetLoaderError {
///    /// An [IO](std::io) Error
///    #[error("Could not load asset: {0}")]
///    Io(#[from] std::io::Error),
///    /// A [RON](ron) Error
///    #[error("Could not parse RON: {0}")]
///    RonSpannedError(#[from] ron::error::SpannedError),
/// }
///
/// // a unit struct for the AssetLoader
/// #[derive(Default)]
/// pub struct SomeAssetLoader;
///
/// impl bevy::asset::AssetLoader for SomeAssetLoader {
///     type Asset = SomeAsset;
///     type Settings = ();
///     type Error = SomeAssetLoaderError;
///     fn load<'a>(
///        &'a self,
///        reader: &'a mut bevy::asset::io::Reader,
///        settings: &'a Self::Settings,
///        load_context: &'a mut bevy::asset::LoadContext,
///    ) -> bevy::utils::BoxedFuture<'a, Result<Self::Asset, Self::Error>> {
///            Box::pin(async move {
///                let mut bytes = Vec::new();
///                reader.read_to_end(&mut bytes).await?;
///             let custom_asset = ron::de::from_bytes::<SomeAsset>(&bytes)?;
///             Ok(custom_asset)
///         })
///     }
///     fn extensions(&self) -> &[&str] {
///         &["some.ron", "some.else.ron"]
///     }
/// }
/// // plugin parts
/// pub struct SomeAssetPlugin;
/// impl bevy::prelude::Plugin for SomeAssetPlugin {
///     fn build(&self, app: &mut bevy::prelude::App) {
///         app.init_asset::<SomeAsset>()
///             .init_asset_loader::<SomeAssetLoader>();
///     }
/// }
/// ```
#[proc_macro_attribute]
pub fn auto_ron_asset_loader(args: TokenStream, input: TokenStream) -> TokenStream {
    let args = parse_macro_input!(args as Extensions);
    let input = parse_macro_input!(input as DeriveInput);

    let extensions_values = args
        .extensions
        .iter()
        .map(|l| l.value())
        .collect::<Vec<String>>();

    let asset_type = &input.ident;

    let loader_name = quote::format_ident!("{}Loader", asset_type);
    let loader_error_name = quote::format_ident!("{}Error", loader_name);
    let plugin_name = quote::format_ident!("{}Plugin", asset_type);

    let expanded = quote! {

        #[derive(Asset, TypePath)] // auto derive necessary derives
        #input // the struct this attribute is used on, still need to return this struct as a TokenStream

        #[non_exhaustive] // Yoinked from bevy's examples
        #[derive(Debug, thiserror::Error)]
        pub enum #loader_error_name {
            /// An [IO](std::io) Error
            #[error("Could not load asset: {0}")]
            Io(#[from] std::io::Error),
            /// A [RON](ron) Error
            #[error("Could not parse RON: {0}")]
            RonSpannedError(#[from] ron::error::SpannedError),
        }

        // a unit struct for the AssetLoader
        #[derive(Default)]
        pub struct #loader_name;

        impl bevy::asset::AssetLoader for #loader_name {
            type Asset = #asset_type;
            type Settings = ();
            type Error = #loader_error_name;

            fn load<'a>(
                &'a self,
                reader: &'a mut bevy::asset::io::Reader,
                settings: &'a Self::Settings,
                load_context: &'a mut bevy::asset::LoadContext,
            ) -> bevy::utils::BoxedFuture<'a, Result<Self::Asset, Self::Error>> {
                Box::pin(async move {
                    let mut bytes = Vec::new();
                    reader.read_to_end(&mut bytes).await?;
                    let custom_asset = ron::de::from_bytes::<#asset_type>(&bytes)?;
                    Ok(custom_asset)
                })
            }

            fn extensions(&self) -> &[&str] {
                &[#(#extensions_values),*]
            }
        }


        // plugin parts
        pub struct #plugin_name;

        impl bevy::prelude::Plugin for #plugin_name {
            fn build(&self, app: &mut bevy::prelude::App) {
                app.init_asset::<#asset_type>()
                    .init_asset_loader::<#loader_name>();
            }
        }
    };

    expanded.into()
}
