use serde::Deserialize;
use crate::entity::{Username, Credentials, Author};
use crate::{request, coder::decoder};
use futures::prelude::*;
use seed::fetch;

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct RootDecoder {
    profile: decoder::Author
}

pub fn load<Ms: 'static>(
    credentials: Option<Credentials>,
    username: Username<'static>,
    f: fn(Result<Author<'static>, (Username<'static>, Vec<String>)>) -> Ms,
) -> impl Future<Item=Ms, Error=Ms>  {
    request::new_api_request(
        &format!("profiles/{}", username.as_str()),
        credentials.as_ref()
    )
        .fetch_json_data(move |data_result: fetch::ResponseDataResult<RootDecoder>| {
            f(data_result
                .map(move |root_decoder| root_decoder.profile.into_author(credentials))
                .map_err(request::fail_reason_into_errors)
                .map_err(move |errors| (username, errors))
            )
        })
}