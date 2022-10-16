mod attribute;
mod derive_struct;

use attribute::ContainerAttributes;
use virtue::prelude::*;

#[proc_macro_derive(HiveEncode, attributes(hive_encode))]
pub fn derive_encode(input: TokenStream) -> TokenStream {
    derive_encode_inner(input).unwrap_or_else(|e| e.into_token_stream())
}

fn derive_encode_inner(input: TokenStream) -> Result<TokenStream> {
    let parse = Parse::new(input)?;
    let (mut generator, attributes, body) = parse.into_generator();
    let attributes = attributes
        .get_attribute::<ContainerAttributes>()?
        .unwrap_or_default();

    if let Body::Struct(body) = body {
        derive_struct::DeriveStruct {
            fields: body.fields,
            attributes,
        }
        .generate_encode(&mut generator)?;
    }

    generator.export_to_file("HiveEncode");
    generator.finish()
}
