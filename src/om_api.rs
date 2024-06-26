/*******************************************************************************
 * Copyright (c) 2023, 2024  Noah Dominic Miranda Silvio.  All rights reserved
 * Licensed under the EUPL v1.2
 ******************************************************************************/

pub(crate) fn get_location_info(query: &str) -> Result<Vec<u8>, curl::Error> {
    let url = format!("https://geocoding-api.open-meteo.com/v1/search?name={query}");
    call_api(&url)
}

pub(crate) fn call_api(url: &str) -> Result<Vec<u8>, curl::Error> {
    let mut api_caller = curl::easy::Easy::new();
    #[cfg(windows)]
    api_caller.ssl_options(curl::easy::SslOpt::new().no_revoke(true))?;
    api_caller.url(url)?;
    let mut api_response_bytes = Vec::new();
    {
        let mut transfer = api_caller.transfer();
        transfer.write_function(|received_data| {
            api_response_bytes.extend_from_slice(received_data);
            Ok(received_data.len())
        })?;
        transfer.perform()?;
    }
    Ok(api_response_bytes)
}
