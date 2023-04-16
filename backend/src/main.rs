//!
//! Afmelder-App, the fancy way to skip the daily
//!
//! This is a _very_ basic actix back-end. It does so little, yet so much.
//! Just do a GET request to https://<site>/get_reason and a nice reason
//! to skip the daily (in Dutch) will be returned.
//!
//! To make this work, you also need a load balancer or proxy on you VPS.
//! I used nginx for this, but you can use whatever.
//!
//! Mentally replace <<servername>> in this file everywhere for the fqdn of your site.
//!
//! Example nginx site configuration for proxying.
//! Put the front-end in /var/www/afmelder-app and run this back-end as a service on the host.
//! (beware! all the main security settings like SSL versions etcetera are omitted here 
//! and are in a main config file, so this is not a complete config).
//!
//! ```
//! server {
//!     root /var/www/afmelder-app;
//!     server_name <<servername>>;
//!    
//!     listen 443 ssl http2;
//!     listen [::]:443 ssl http2;
//!    
//!     # RSA certificate
//!     ssl_certificate /path/to/fullchain.pem;
//!     ssl_certificate_key /path/to/privkey.pem;
//!    
//!     location /api {
//!         proxy_pass http://localhost:8089;
//!         proxy_set_header X-Forwarded-Host $server_name;
//!         proxy_set_header X-Real-IP $remote_addr;
//!         proxy_set_header X-Forwarded-Proto https;
//!    
//!     }
//! }
//! ```

// We need actix to, well, run actix. And Serde for JSON.
// and rand for the random response generator(tm)
use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};
use serde::Serialize;
use rand::Rng;

// All the reasons are in a vector. Is a lot more flexible than static slices
#[derive(Clone)]
struct Reasons {
     reasons: Vec<String>,
}

// The response is in a JSON, so we need a JSON structure for that
#[derive(Serialize)]
struct Reasonresponse {
    reason: String,
    meeting: String,
}

// The api root just respond with a health response,
// it's at https://<<servername>>/api because of the load balancer
// you can use this in the fontend to check if the api is working.
// I didn't. but it could.
#[get("/")]
async fn health() -> impl Responder {
    HttpResponse::Ok().body("OK")
}

// the bread and butter. this function takes a random string from the reasons vector and formats the response
// output is an actix JSON response thingy object something something
async fn afmelder(data: web::Data<Reasons>) -> impl Responder {
    let mut rng = rand::thread_rng();
    let chosen = format!("{} en kan daarom niet bij de daily zijn", &data.reasons[rng.gen_range(0..&data.reasons.len() - 1)]);
    let reply = Reasonresponse { reason: chosen, meeting: "daily".to_string() };
    web::Json(reply)
}

// just the routing, https://<<servername>>/api/get_reason is the api endpoint for the reasons
fn api_config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::resource("/get_reason")
             .route(web::get().to(afmelder)),
    );
}

// Main is where the party's at.
#[actix_web::main]
async fn main() -> std::io::Result<()> {

    // List of reasons. a vector of strings. could be made more fancy with a database
    // or text file which is loaded at startup, but for this quick hack... nope, keep it KISS.
    // And yes it's in Dutch, because this was used in/for a Dutch spoken daily.
    let data = Reasons {
        reasons: vec![
          "Ik keek naar de sterren en vind nu alles nietig en insignificant".to_string(),
          "Ik zag een hele goede aanbieding voor een tosti ijzer bij de Lidl".to_string(),
          "Ik mag meedoen aan Holland got Talent".to_string(),
          "Ik heb een cavia en die heeft mijn laptop opgegeten".to_string(),
          "Ik ontdekte een UFO in mijn achtertuin en ben nu ontvoerd door aliens".to_string(),
          "Ik ben gebeld door Rutte voor een zeer geheime missie in het Staatsbelang".to_string(),
          "Ik zie dat al mijn boeken in de kast nog niet correct op kleur en titel gesorteerd staan".to_string(),
          "Ik heb een cursus mindfullnes gedaan en leef nu alleen nog maar in het hier en nu".to_string(),
          "Ik ben een Netflix serie met drie seizoenen aan het bingen en ben nu pas bij S2E06".to_string(),
          "Ik was bezig thuis met kernfusie te experimenteren maar heb per ongeluk de hele straat gesublimeerd".to_string(),
          "Ik ben wakker geworden in een Taiwanees hotel met een tatoo op mn gezicht en heb geen idee wat er gebeurd is".to_string(),
          "Ik ben net per e-mail benaderd over een Nigeriaanse prins met een nalatenschap van miljoenen".to_string(),
          "Ik heb bij mijn omelet als ontbijt per ongeluk de overgebleven paddos van Awakenings verward met de champignons".to_string(),
          "Ik heb Unhacked van Rian van Rijbroek gelezen en heb nu echt het licht gezien".to_string(),
          "Ik heb mijzelf voor een klimaatprotest aan de Nachtwacht ge-seconde-lijmd".to_string(),
          "Ik las op internet dat morgen de zombie apocalypse uitbreekt, en ik moet nog preppen".to_string(),
          "Ik heb een Facebookje gedaan met mn BGP route regex en mijn thuis internet ligt eruit".to_string(),
          "Ik heb gisteren een frikandellen eetwedstrijd gewonnen maar zit nu met de nasleep daarvan".to_string(),
          "Ik had eigenlijk Troubadour willen zijn en ga nu echt mijn passie achterna".to_string(),
          "Ik zat bij de Starbucks aan de A12 te wachten tot de file weg was".to_string(),
          "Ik something something projectleiders".to_string(),
          "Ik zit in een andere, veel belangrijkere meeting met veel leukere mensen".to_string(),
          "Ik ben druk druk druk".to_string(),
        ],
    };

    // Well, let's serve the shizzle. this is boilerplate from the actix tutorial, no bling here.
    // Don't forget to configure a load balancer or frontend of some sort. I used nginx, but 
    // the choice is giant in all kinds of load balancers/proxies.
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(data.to_owned()))
            .service(health)
            .service(web::scope("/api").configure(api_config))
    })
    .bind(("127.0.0.1", 8089))?
    .run()
    .await
}
