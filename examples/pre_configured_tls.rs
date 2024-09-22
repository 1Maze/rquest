use boring::ssl::{SslConnector, SslMethod};
use http::HeaderValue;
use rquest::{
    tls::{Http2Settings, ImpersonateSettings, SslExtension},
    HttpVersionPref,
};
use rquest::{PseudoOrder::*, SettingsOrder::*};
use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // Create a pre-configured TLS settings
    let settings = ImpersonateSettings::builder()
        .tls((
            SslConnector::builder(SslMethod::tls_client())?,
            SslExtension::builder()
                .tls_sni(true)
                .http_version_pref(HttpVersionPref::All)
                .application_settings(true)
                .pre_shared_key(true)
                .enable_ech_grease(true)
                .permute_extensions(true)
                .build(),
        ))
        .http2(
            Http2Settings::builder()
                .initial_stream_window_size(6291456)
                .initial_connection_window_size(15728640)
                .max_concurrent_streams(1000)
                .max_header_list_size(262144)
                .header_table_size(65536)
                .enable_push(false)
                .headers_priority((0, 255, true))
                .headers_pseudo_order([Method, Scheme, Authority, Path])
                .settings_order(vec![
                    HeaderTableSize,
                    EnablePush,
                    MaxConcurrentStreams,
                    InitialWindowSize,
                    MaxFrameSize,
                    MaxHeaderListSize,
                    EnableConnectProtocol,
                ])
                .build(),
        )
        .headers(Box::new(|headers| {
            headers.insert("user-agent", HeaderValue::from_static("rquest"));
        }))
        .build();

    // Build a client with pre-configured TLS settings
    let client = rquest::Client::builder()
        .use_preconfigured_tls(settings)
        .enable_ech_grease()
        .permute_extensions()
        .build()?;

    // Use the API you're already familiar with
    let resp = client.get("https://tls.peet.ws/api/all").send().await?;
    println!("{}", resp.text().await?);

    Ok(())
}
