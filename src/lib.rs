use serde_json::json;
use worker::*;

mod utils;

fn log_request(req: &Request) {
    console_log!(
        "{} - [{}], located at: {:?}, within: {}",
        Date::now().to_string(),
        req.path(),
        req.cf().coordinates().unwrap_or_default(),
        req.cf().region().unwrap_or_else(|| "unknown region".into())
    );
}

#[event(fetch)]
pub async fn main(req: Request, env: Env, _ctx: worker::Context) -> Result<Response> {
    log_request(&req);

    // Optionally, get more helpful error messages written to the console in the case of a panic.
    utils::set_panic_hook();

    // Optionally, use the Router to handle matching endpoints, use ":name" placeholders, or "*name"
    // catch-alls to match on specific patterns. Alternatively, use `Router::with_data(D)` to
    // provide arbitrary data that will be accessible in each route via the `ctx.data()` method.
    let router = Router::new();

    // Add as many routes as your Worker needs! Each route will get a `Request` for handling HTTP
    // functionality and a `RouteContext` which you can use to  and get route parameters and
    // Environment bindings like KV Stores, Durable Objects, Secrets, and Variables.
    router
        //.get("/", |_, _| Response::ok("Hello from Workers!"))
        .post_async("/form/:field", |mut req, ctx| async move {
            if let Some(name) = ctx.param("field") {
                let form = req.form_data().await?;
                match form.get(name) {
                    Some(FormEntry::Field(value)) => {
                        return Response::from_json(&json!({ name: value }))
                    }
                    Some(FormEntry::File(_)) => {
                        return Response::error("`field` param in form shouldn't be a File", 422);
                    }
                    None => return Response::error("Bad Request", 400),
                }
            }

            Response::error("Bad Request", 400)
        })
        .get("/worker-version", |_, ctx| {
            let version = ctx.var("WORKERS_RS_VERSION")?.to_string();
            Response::ok(version)
        })
        .get_async("/", |_, _| async {
            let tsb = tsb_parse_data().await;
            let tkn = tkn_parse_data().await;
            let sb = sb_parse_data().await;
            let kn = kn_parse_data().await;

            let deneme = "Silverblue 37: ".to_owned()
                + &sb
                + "\n"
                + "Kinoite 37: "
                + &kn
                + "\n"
                + "Testing Silverblue 37: "
                + &tsb
                + "\n"
                + "Testing Kinoite 37: "
                + &tkn;

            Response::ok(deneme)
        })
        .run(req, env)
        .await
}

async fn testing_fetch_website() -> String {
    let website = reqwest::get(
        "https://kojipkgs.fedoraproject.org/ostree/repo/refs/heads/fedora/37/x86_64/testing/",
    )
    .await
    .unwrap()
    .text()
    .await
    .unwrap();
    website
}

async fn tsb_parse_data() -> String {
    let html = testing_fetch_website().await;
    //console_log!("{}", html);

    let re = regex::Regex::new(
        r"ue</a>                                [0-9]{4}-[0-9]{2}-[0-9]{2} \d\d:\d\d",
    )
    .unwrap();

    let mut output = re.find(&html).unwrap().as_str().to_owned();

    let mut counter = 0;

    while counter < 38 {
        output.remove(0);
        counter += 1;
    }

    console_log!("{}", output);
    output.to_owned()
}

async fn tkn_parse_data() -> String {
    let html = testing_fetch_website().await;
    //console_log!("{}", html);

    let re = regex::Regex::new(
        r"te</a>                                   [0-9]{4}-[0-9]{2}-[0-9]{2} \d\d:\d\d",
    )
    .unwrap();

    let mut output = re.find(&html).unwrap().as_str().to_owned();

    let mut counter = 0;

    while counter < 41 {
        output.remove(0);
        counter += 1;
    }

    console_log!("{}", output);
    output.to_owned()
}

async fn fetch_website() -> String {
    let website =
        reqwest::get("https://kojipkgs.fedoraproject.org/ostree/repo/refs/heads/fedora/37/x86_64/")
            .await
            .unwrap()
            .text()
            .await
            .unwrap();
    website
}

async fn sb_parse_data() -> String {
    let html = fetch_website().await;
    //console_log!("{}", html);

    let re =
        regex::Regex::new(r"ue</a>                         [0-9]{4}-[0-9]{2}-[0-9]{2} \d\d:\d\d")
            .unwrap();

    let mut output = re.find(&html).unwrap().as_str().to_owned();

    let mut counter = 0;

    console_log!("{}", output.len());

    while counter < 31 {
        output.remove(0);
        counter += 1;
    }

    console_log!("{}", output);
    output.to_owned()
}

async fn kn_parse_data() -> String {
    let html = fetch_website().await;
    //console_log!("{}", html);

    let re = regex::Regex::new(
        r"te</a>                            [0-9]{4}-[0-9]{2}-[0-9]{2} \d\d:\d\d",
    )
    .unwrap();

    let mut output = re.find(&html).unwrap().as_str().to_owned();

    let mut counter = 0;

    while counter < 34 {
        output.remove(0);
        counter += 1;
    }

    console_log!("{}", output);
    output.to_owned()
}
