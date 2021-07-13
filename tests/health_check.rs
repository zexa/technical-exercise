use technical_exercise::{build_app, ApplicationState};
use tide::StatusCode;
use tide_testing::TideTestingExt;

// I suppose you could do something like in the zero2prod example such as bellow and then use sync
// tests everywhere. You could decouple the tests from the implementation itself and that's cool.
// Tide is a bit weird with its listener implementation though and it's not that elegant to get it
// working that way though. We could chat about this in the interview if you'd like c:
// pub fn spawn_app() {
//     let address = TcpListener::bind("127.0.0.1:0").expect("Could not bind to random port");
//     let port = address.local_addr().unwrap().port();
//
//     let app = build_app(ApplicationState::new(env::var("TRANSLATIONS_API_KEY").ok()));
//
//     let app_clone = app.clone();
//     let address_clone = address.try_clone().unwrap();
//     let port_clone = port.clone();
//     let _ = async_std::task::spawn(async move {
//         app_clone
//             .listen(address_clone)
//             .await
//             .expect(&format!("Could not listen to port {}", port_clone))
//     });
// }

// Regardless for the sake of showing off what I know, lets use a more tide idiomatic approach:
#[async_std::test]
async fn test_health_check_works() -> tide::Result<()> {
    Ok(assert_eq!(
        build_app(ApplicationState::default())
            .get("/health_check")
            .await?
            .status(),
        StatusCode::Ok
    ))
}
