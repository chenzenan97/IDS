
use lambda_http::{
    aws_lambda_events::serde_json::json, run, service_fn, Body, Error, IntoResponse, Request,
    RequestExt, Response,
};

use serde::Serialize;

struct FactList {
    facts: Vec<&'static str>,
}

impl FactList {
    fn new() -> FactList {
        FactList { facts: vec![
            "The US central bank has raised interest rates again, despite fears that the move could add to financial turmoil after a string of bank failures..",
            "Tyson Fury v Oleksandr Usyk: Promoter Frank Warren says fight is off.",
            "Oldest most complete Hebrew Bible goes on display in Israel before sale",
            "Jack Daniel's and dog toy in Supreme Court showdown",
            "Why workplace drinking culture is fading fast",
        ] }
    }
}

fn get_random_fact(fact_list: &FactList) -> &'static str {
    use rand::Rng;
    let mut rng = rand::thread_rng();

    // let f = rng.gen_range(0..4);
    let l: usize = fact_list.facts.len().try_into().unwrap();
    fact_list.facts[rng.gen_range(0..l)]
}

async fn build_success_response(fact: &'static str) -> Response<Body> {
    json!({ "fact": fact }).into_response().await
}

async fn build_failure_response(error_message: &str) -> Response<Body> {
    Response::builder()
        .status(400)
        .header("content-type", "application/json")
        .body(Body::from(json!({ "error": error_message }).to_string()))
        .expect("could not build the error response")
}

fn process_event(fact_list: &FactList) -> &'static str {
    get_random_fact(fact_list)
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    let all_facts = &FactList::new();
    let handler_func = |event: Request| async move {
        let response = build_success_response(process_event(all_facts)).await;
        Result::<Response<Body>, Error>::Ok(response)
    };
    run(service_fn(handler_func)).await?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn new_fact_list_test() {
        let all_facts: FactList = FactList::new();
        assert_eq!(5, all_facts.facts.len());
    }

    #[tokio::test]
    async fn build_success_response_test() {
        let test_fact = "This is a test BBCNEWS.";
        let result = build_success_response(test_fact).await;
        let (parts, body) = result.into_parts();
        assert_eq!(200, parts.status.as_u16());
        assert_eq!(
            "application/json",
            parts.headers.get("content-type").unwrap()
        );
        assert_eq!(
            "{\"fact\":\"This is a test BBCNEWS.\"}",
            String::from_utf8(body.to_ascii_lowercase()).unwrap()
        );
    }

    #[tokio::test]
    async fn build_failure_response_test() {
        let result = build_failure_response("test error message").await;
        let (parts, body) = result.into_parts();
        assert_eq!(400, parts.status.as_u16());
        assert_eq!(
            "application/json",
            parts.headers.get("content-type").unwrap()
        );
        assert_eq!(
            "{\"error\":\"test error message\"}",
            String::from_utf8(body.to_ascii_lowercase()).unwrap()
        );
    }

    #[test]
    fn process_event_test() {
        let fact_list = FactList::new();
        let res = process_event(Some("history"), &fact_list);
        assert!(res.is_ok());
        let fact = res.unwrap();
        assert_eq!(fact.name, "history");
        assert_eq!(fact.description, "The study of past BBCNEWS");
    }
}