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
#[get("/?<ip>")]
pub async fn collector_stats_js(
    ip: Option<String>,
    state: &State<AppState>,
    user_agent_info: UserAgentInfo,
    conn: DbConn,
) -> (Status, content::RawJavaScript<String>) {
    let ip = if state.dev_mode {
        IpAddr::V4(Ipv4Addr::new(215, 204, 222, 212))
    } else {
        ip.as_ref()
            .filter(|ip_str| !ip_str.is_empty())
            .and_then(|ip_str| ip_str.parse::<IpAddr>().ok())
            .unwrap_or_else(|| IpAddr::V4(Ipv4Addr::new(66, 131, 120, 255)))
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
            eprintln!("Error inserting collector: {e:?}");

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
(() => {{
    {{
        const collectorId = "{collector_id}";
        const appUrl = "{app_url}";

        function init() {{
            {{
                document.addEventListener("click", (event) => {{
                    if (event.target.tagName === "A") {{
                        const target = event.target.getAttribute("target");
                        const href = event.target.getAttribute("href");

                        if (target === "_blank") {{
                            stats_collect("leave", href);
                        }}
                    }}
                }});

                window.addEventListener("beforeunload", (event) => {{
                    stats_collect("exit");
                }});

                // Listen for history changes
                function wrapHistoryMethod(method) {{
                    const original = history[method];
                    history[method] = function (...args) {{
                        const [state, title, url] = args;
                        const fullUrl = new URL(url, window.location.origin)
                            .href;
                        console.log("📼 history", method, url, fullUrl);
                        original.apply(this, args);
                        stats_collect("visit", fullUrl);
                    }};
                }}

                wrapHistoryMethod("pushState");
                wrapHistoryMethod("replaceState");

                // Listen for popstate event
                window.addEventListener("popstate", (event) => {{
                    stats_collect("visit", location.href);
                }});
            }}
        }}

        async function send(
            type = "pageview",
            url_override = null,
            referrer = document.referrer,
        ) {{
            const endpoint = `${{appUrl}}/event`;

            const data = {{
                collector_id: collectorId,
                name: type,
                url: url_override || window.location.href,
                referrer: referrer,
            }};

            fetch(endpoint, {{
                method: "POST",
                headers: {{
                    "Content-Type": "application/json",
                }},
                body: JSON.stringify(data),
            }})
                .then((res) => res.json())
                .then((data) => {{
                    // console.log("📼", data);
                }})
                .catch((rejected) => {{
                    console.log(`📼 [send]`, "failed to collect");
                    console.error(rejected);
                }});
        }}

        async function stats_collect(type, url = null) {{
            await send(type, url);
        }}

        window.stats_collect = stats_collect;
        stats_collect("enter");

        window.addEventListener("load", () => {{
            init();
        }});
    }}
}})();
"#
    )
}
