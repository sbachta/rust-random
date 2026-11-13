use axum::{Json, Router, extract::Query, http::StatusCode, routing::get};
use serde::{Deserialize, Serialize};

/// Query string params, e.g. `/random?min=1&max=6`.
/// Spring equivalent: `@RequestParam(required = false) Long min, ...`
#[derive(Deserialize)]
struct RangeParams {
    min: Option<i64>,
    max: Option<i64>,
}

/// JSON response body. Spring equivalent: a DTO that Jackson serializes.
#[derive(Serialize)]
struct RandomResponse {
    value: i64,
    min: i64,
    max: i64,
}

/// Spring equivalent: `@GetMapping("/random")` method on a `@RestController`.
/// Returning `Err(...)` becomes an HTTP error response, like throwing
/// `ResponseStatusException(HttpStatus.BAD_REQUEST, msg)`.
async fn random_handler(Query(params): Query<RangeParams>, ) -> Result<Json<RandomResponse>, (StatusCode, String)> {
    let min = params.min.unwrap_or(1);
    let max = params.max.unwrap_or(100);

    let value = pick_random(min, max).map_err(|msg| (StatusCode::BAD_REQUEST, msg))?;

    Ok(Json(RandomResponse { value, min, max }))
}

/// Core logic: return a random number in the inclusive range [min, max],
/// or an error message if the range is invalid.
fn pick_random(min: i64, max: i64) -> Result<i64, String> {
    return Ok(rand::random_range(min..=max));
    // TODO(human): validate the range and generate the random number.
    todo!()
}

/// Spring equivalent: `SpringApplication.run(...)` + embedded Tomcat on port 8080.
#[tokio::main]
async fn main() {
    let app = Router::new().route("/random", get(random_handler));

    // 0.0.0.0 (not 127.0.0.1) so it's reachable from outside a Docker container.
    let listener = tokio::net::TcpListener::bind("0.0.0.0:8080").await.unwrap();
    println!("listening on http://{}", listener.local_addr().unwrap());

    axum::serve(listener, app).await.unwrap();
}
