use dropshot::endpoint;
use dropshot::ApiDescription;
use dropshot::ConfigDropshot;
use dropshot::ConfigLogging;
use dropshot::ConfigLoggingLevel;
use dropshot::HttpError;
use dropshot::HttpResponseOk;
use dropshot::HttpServerStarter;
use dropshot::RequestContext;
use dropshot::TypedBody;
use http::StatusCode;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::sync::Arc;

use super::game::{Action, Game, PrivateState, PublicState};

#[derive(Deserialize, JsonSchema)]
struct InitRequest {
    n_players: u8,
}

#[derive(Deserialize, JsonSchema)]
struct StepRequest {
    game: Game,
    action: Action,
}

#[derive(Serialize, JsonSchema)]
struct Response {
    game: Game,
    public_state: PublicState,
    private_states: Vec<PrivateState>,
}

#[endpoint {
    method = POST,
    path = "/init",
}]
async fn init_game(
    _ctx: Arc<RequestContext<()>>,
    body_params: TypedBody<InitRequest>,
) -> Result<HttpResponseOk<Response>, HttpError> {
    let params = body_params.into_inner();
    let result = Game::new(params.n_players);
    match result {
        Err(err) => Err(HttpError {
            status_code: StatusCode::BAD_REQUEST,
            error_code: None,
            external_message: err.clone(),
            internal_message: err,
        }),
        Ok((game, public_state, private_states)) => Ok(HttpResponseOk(Response {
            game,
            public_state,
            private_states,
        })),
    }
}

#[endpoint {
    method = POST,
    path = "/step",
}]
async fn step_game(
    _ctx: Arc<RequestContext<()>>,
    body_params: TypedBody<StepRequest>,
) -> Result<HttpResponseOk<Response>, HttpError> {
    let mut params = body_params.into_inner();
    let result = params.game.step(params.action);
    match result {
        Err(err) => Err(HttpError {
            status_code: StatusCode::BAD_REQUEST,
            error_code: None,
            external_message: err.clone(),
            internal_message: err,
        }),
        Ok((public_state, private_states)) => Ok(HttpResponseOk(Response {
            game: params.game,
            public_state,
            private_states,
        })),
    }
}

pub async fn start() -> Result<(), String> {
    // Set up a logger.
    let log = ConfigLogging::StderrTerminal {
        level: ConfigLoggingLevel::Info,
    }
    .to_logger("minimal-example")
    .map_err(|e| e.to_string())?;

    // Describe the API.
    let mut api = ApiDescription::new();
    api.register(init_game).unwrap();
    api.register(step_game).unwrap();

    // Start the server.
    let server = HttpServerStarter::new(
        &ConfigDropshot {
            bind_address: "127.0.0.1:0".parse().unwrap(),
            request_body_max_bytes: 2048 * 1024,
        },
        api,
        *Arc::new(()),
        &log,
    )
    .map_err(|error| format!("failed to start server: {}", error))?
    .start();

    server.await
}
