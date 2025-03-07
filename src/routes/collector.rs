use rocket::{
    State, get,
    http::Status,
    response::content::{self},
};
use std::net::{IpAddr, Ipv4Addr};

use crate::{
    AppState, DbConn, UserAgentInfo,
    models::{City, Collector, CollectorQuery},
};

/// # `stats_js`
/// Generates the JavaScript code for analytics tracking.
///
/// ## Arguments
/// * `ip` - The IP address of the visitor
/// * `state` - The application state
/// * `user_agent_info` - The user agent information
/// * `conn` - The database connection
///
/// ## Panics
/// * If there is an error inserting the city into the database.
///
/// ## Returns
/// * `(Status, content::RawJavaScript<String>)`
#[get("/")]
pub async fn collector_stats_js(
    ip: IpAddr,
    state: &State<AppState>,
    user_agent_info: UserAgentInfo,
    conn: DbConn,
) -> (Status, content::RawJavaScript<String>) {
    let ip = if state.dev_mode {
        IpAddr::V4(Ipv4Addr::new(215, 204, 222, 212))
    } else {
        ip
    };

    let city_to_create = City::from_ip(&ip.to_string()).await.unwrap_or_default();

    let existing_city = City::find_by_name_and_country(
        city_to_create.name.clone(),
        city_to_create.country.clone(),
        &conn,
    )
    .await;

    let new_id_res = match existing_city.unwrap() {
        Some(found) => Ok(found.id.unwrap_or_default()),
        None => City::insert(city_to_create, ip, &conn).await,
    };

    let new_id = match new_id_res {
        Ok(id) => id,
        Err(e) => {
            panic!("Error inserting city: {e:?}");
        }
    };

    let collector_query = CollectorQuery {
        origin: ip.to_string(),
        city_id: new_id,
        os: Some(user_agent_info.os),
        browser: Some(user_agent_info.browser),
    };
    let collector: Collector = collector_query.into();

    match Collector::insert(collector, &conn).await {
        Ok(collector_id) => {
            let analytics_js = generate_analytics_js(&collector_id.to_string(), &state.address);
            (Status::Ok, content::RawJavaScript(analytics_js))
        }
        Err(e) => {
            println!("Error inserting collector: {e:?}");

            // Return a minimal fallback script with error details
            let error_js = format!(
                r#""use strict";
                (function() {{
                    console.error("Analytics initialization failed: {}", {});
                    window.stats_collect = function() {{
                        console.warn("Analytics disabled due to initialization error");
                    }};
                }})();"#,
                e,
                Status::InternalServerError
            );

            (
                Status::InternalServerError,
                content::RawJavaScript(error_js),
            )
        }
    }
}

/// # `generate_analytics_js`
/// Generates the JavaScript code for analytics tracking.
///
/// ## Arguments
/// * `collector_id` - Collector ID
/// * `app_url` - Application URL
///
/// ## Returns
/// * `String` - JavaScript code for analytics tracking
fn generate_analytics_js(collector_id: &str, app_url: &str) -> String {
    format!(
        r#""use strict";
(function() {{
    var collectorId = "{collector_id}";
    var appUrl = "{app_url}";

    function init() {{
        document.addEventListener('click', function(event) {{
            if (event.target.tagName === 'A') {{
                var target = event.target.getAttribute('target');
                var href = event.target.getAttribute('href');

                if (target === '_blank') {{
                    stats_collect('leave', href);
                }}
            }}
        }});

        window.addEventListener("beforeunload", function(event) {{
            stats_collect('exit');
        }});

        // Listen for history changes
        function wrapHistoryMethod(method) {{
            var original = history[method];
            history[method] = function(state, title, url) {{
                var fullUrl = new URL(url, window.location.origin).href;
                console.log("📼 history", method, url, fullUrl);
                original.apply(this, arguments);
                stats_collect('visit', fullUrl);
            }};
        }}

        wrapHistoryMethod('pushState');
        wrapHistoryMethod('replaceState');

        // Listen for popstate event
        window.addEventListener('popstate', function(event) {{
            stats_collect('visit', location.href);
        }});
    }}

    async function send(type = "pageview", url_override = null, referrer = document.referrer) {{
        var url = new URL(appUrl + "/collect");

        url.searchParams.set('collector_id', collectorId);
        url.searchParams.set('name', type);
        url.searchParams.set('url', url_override || window.location.href);
        url.searchParams.set('referrer', referrer);

        fetch(url)
        .then(res => res.json())
        .then(data => {{
            // console.log("📼", data);
        }})
        .catch(rejected => {{
            console.log("📼", "failed to collect");
            console.error(rejected);
        }});
    }}

    async function stats_collect(type, url = null) {{
        await send(type, url);
    }}

    window.stats_collect = stats_collect;
    stats_collect('enter');

    window.addEventListener('load', function() {{
        init();
    }});
}})();
"#
    )
}
