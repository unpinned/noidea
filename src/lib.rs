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

static PART1: &str = r#"<!DOCTYPE html>
<html lang="en">
  <head>
    <meta charset="utf-8" />
    <meta name="viewport" content="width=device-width, initial-scale=1" />
    <link
      rel="icon"
      type="image/png"
      href="https://em-content.zobj.net/thumbs/120/google/350/stopwatch_23f1-fe0f.png"
    />
    <link
      rel="stylesheet"
      href="https://unpkg.com/@picocss/pico@1.5.7/css/pico.min.css"
    />
    <title>Fedora Silverblue Tracking</title>
  </head>
  <body>
    <main class="container">
      <img
        src="https://raw.githubusercontent.com/unpinned/noidea/main/static/sbkn.png"
        style="
          display: block;
          margin-left: auto;
          margin-right: auto;
          width: 20%;
        "
      />
      <table>
        <h1 style="text-align: center">
          Track last update status of Silverblue and Kinoite
        </h1>
        <thead>
          <tr>
            <th scope="col">Edition</th>
            <th scope="col">Current Version</th>
          </tr>
        </thead>
        <tbody>
          <tr>
            <th scope="row">Silverblue 39</th>
            <td><mark>"#;

static PART2: &str = r#" UTC</mark></td>
</tr>
<tr>
  <th scope="row">Kinoite 39</th>
  <td><mark>"#;

static PART3: &str = r#" UTC</mark></td>
  </tr>
  <tr>
    <th scope="row">Testing Silverblue 38</th>
    <td><mark>"#;
static PART4: &str = r#" UTC</mark></td>
</tr>
<tr>
  <th scope="row">Testing Kinoite 38</th>
  <td><mark>"#;
static PART5: &str = r#" UTC</mark></td>
</tr>
<tr>
  <th scope="row">Rawhide Silverblue</th>
  <td><mark>"#;

static PART6: &str = r#" UTC</mark></td>
</tr>
<tr>
  <th scope="row">Rawhide Kinoite</th>
  <td><mark>"#;

static PART7: &str = r#" UTC</mark></td>
</tr>
</tbody>
</table>
</main>
</body>
<footer>
<main class="container">
<p style="text-align: center">Powered by Cloudflare Workers</p>
<div style="text-align: center">
<a href="https://github.com/unpinned/noidea">Source Code</a>
</div>
</main>
</footer>
</html>"#;

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
            let rsb = rsb_parse_data().await;
            let rkn = rkn_parse_data().await;

            let deneme = PART1.to_owned()
                + &sb
                + PART2
                + &kn
                + PART3
                + &tsb
                + PART4
                + &tkn
                + PART5
                + &rsb
                + PART6
                + &rkn
                + PART7;
            Response::from_html(deneme)
        })
        .run(req, env)
        .await
}

async fn testing_fetch_website() -> String {
    let website = reqwest::get(
        "https://kojipkgs.fedoraproject.org/ostree/repo/refs/heads/fedora/38/x86_64/testing//",
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

async fn fetch_f39_website() -> String {
    let website =
        reqwest::get("https://kojipkgs.fedoraproject.org/ostree/repo/refs/heads/fedora/39/x86_64/")
            .await
            .unwrap()
            .text()
            .await
            .unwrap();
    website
}

async fn sb_parse_data() -> String {
    let html = fetch_f39_website().await;
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
    let html = fetch_f39_website().await;
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

async fn rawhidefetch_website() -> String {
    let website = reqwest::get(
        "https://kojipkgs.fedoraproject.org/ostree/repo/refs/heads/fedora/rawhide/x86_64/",
    )
    .await
    .unwrap()
    .text()
    .await
    .unwrap();
    website
}

async fn rsb_parse_data() -> String {
    let html = rawhidefetch_website().await;
    //console_log!("{}", html);

    let re = regex::Regex::new(
        r"ue</a>                              [0-9]{4}-[0-9]{2}-[0-9]{2} \d\d:\d\d",
    )
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

async fn rkn_parse_data() -> String {
    let html = rawhidefetch_website().await;
    //console_log!("{}", html);

    let re = regex::Regex::new(
        r"te</a>                                 [0-9]{4}-[0-9]{2}-[0-9]{2} \d\d:\d\d",
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
