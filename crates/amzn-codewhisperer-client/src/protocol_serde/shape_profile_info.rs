// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub(crate) fn de_profile_info<'a, I>(
    tokens: &mut ::std::iter::Peekable<I>,
) -> ::std::result::Result<Option<crate::types::ProfileInfo>, ::aws_smithy_json::deserialize::error::DeserializeError>
where
    I: Iterator<
        Item = Result<
            ::aws_smithy_json::deserialize::Token<'a>,
            ::aws_smithy_json::deserialize::error::DeserializeError,
        >,
    >,
{
    match tokens.next().transpose()? {
        Some(::aws_smithy_json::deserialize::Token::ValueNull { .. }) => Ok(None),
        Some(::aws_smithy_json::deserialize::Token::StartObject { .. }) => {
            #[allow(unused_mut)]
            let mut builder = crate::types::builders::ProfileInfoBuilder::default();
            loop {
                match tokens.next().transpose()? {
                    Some(::aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
                    Some(::aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => match key
                        .to_unescaped()?
                        .as_ref()
                    {
                        "arn" => {
                            builder = builder.set_arn(
                                ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
                                    .map(|s| s.to_unescaped().map(|u| u.into_owned()))
                                    .transpose()?,
                            );
                        },
                        "profileName" => {
                            builder = builder.set_profile_name(
                                ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
                                    .map(|s| s.to_unescaped().map(|u| u.into_owned()))
                                    .transpose()?,
                            );
                        },
                        "description" => {
                            builder = builder.set_description(
                                ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
                                    .map(|s| s.to_unescaped().map(|u| u.into_owned()))
                                    .transpose()?,
                            );
                        },
                        "status" => {
                            builder = builder.set_status(
                                ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
                                    .map(|s| s.to_unescaped().map(|u| crate::types::ProfileStatus::from(u.as_ref())))
                                    .transpose()?,
                            );
                        },
                        "profileType" => {
                            builder = builder.set_profile_type(
                                ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
                                    .map(|s| s.to_unescaped().map(|u| crate::types::ProfileType::from(u.as_ref())))
                                    .transpose()?,
                            );
                        },
                        "optInFeatures" => {
                            builder = builder.set_opt_in_features(
                                crate::protocol_serde::shape_opt_in_features::de_opt_in_features(tokens)?,
                            );
                        },
                        _ => ::aws_smithy_json::deserialize::token::skip_value(tokens)?,
                    },
                    other => {
                        return Err(::aws_smithy_json::deserialize::error::DeserializeError::custom(
                            format!("expected object key or end object, found: {:?}", other),
                        ));
                    },
                }
            }
            Ok(Some(
                crate::serde_util::profile_info_correct_errors(builder)
                    .build()
                    .map_err(|err| {
                        ::aws_smithy_json::deserialize::error::DeserializeError::custom_source(
                            "Response was invalid",
                            err,
                        )
                    })?,
            ))
        },
        _ => Err(::aws_smithy_json::deserialize::error::DeserializeError::custom(
            "expected start object or null",
        )),
    }
}
