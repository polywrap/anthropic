mod wrap;
use wrap::*;
use polywrap_wasm_rs::{Map, JSON, BigNumber};
use polywrap_wasm_rs::JSON::json;
use crate::wrap::imported::ArgsPost;

static CLIENT_ID: &str = "anthropic-typescript/0.4.3";
static DEFAULT_API_URL: &str = "https://api.anthropic.com";

impl ModuleTrait for Module {
    fn complete(args: ArgsComplete, env: Env) -> Result<CompletionResponse, String> {
        let api_url: String = env.api_url.unwrap_or_else(|| String::from(DEFAULT_API_URL));

        let mut headers = Map::new();
        headers.insert(String::from("Accept"), String::from("application/json"));
        headers.insert(String::from("Content-Type"), String::from("application/json"));
        headers.insert(String::from("Client"), String::from(CLIENT_ID));
        headers.insert(String::from("X-API-Key"), String::from(env.api_key));

        let mut data: JSON::Value = json!({
            "params": {
                "prompt": args.params.prompt,
                "max_tokens_to_sample": args.params.max_tokens_to_sample,
                "stop_sequences": args.params.stop_sequences, // vec<string>
                "model": args.params.model,
            },
            "stream": false
        });

        if let Some(temperature) = args.params.temperature {
            let temperature_num =  temperature.to_string().parse::<f64>().unwrap();
            data["params"]["temperature"] = temperature_num.into()
        }

        if let Some(top_k) = args.params.top_k {
            let top_k_num = top_k.to_string().parse::<u64>().unwrap();
            data["params"]["top_k"] = top_k_num.into();
        }

        if let Some(top_p) = args.params.top_p {
            let top_p_num = top_p.to_string().parse::<f64>().unwrap();
            data["params"]["top_p"] = top_p_num.into();
        }

        if let Some(tags) = args.params.tags {
            data["params"]["tags"] = JSON::to_value(tags).unwrap();
        }

        let request = HttpRequest {
            headers: Some(headers),
            url_params: None,
            response_type: HttpResponseType::BINARY,
            body: Some(data.to_string()),
            form_data: None,
            timeout: None,
        };

        let response: HttpResponse = HttpModule::post(&ArgsPost {
            url: format!("{}/v1/complete", api_url),
            request: Some(request),
        })?.ok_or("No response from server".to_string())?;

        if response.status == 200 {
            let body = response.body.ok_or("Response from server contains empty body")?;
            let completion: CompletionResponse = JSON::from_str(&body).map_err(|e| e.to_string())?;
            Ok(completion)
        } else {
            Err(format!("Http error with status code: {}, text: {}", response.status, response.status_text))
        }
    }
}
