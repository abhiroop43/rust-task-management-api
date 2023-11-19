use mysql::prelude::*;
use mysql::*;
// use native_tls::Certificate;
// use native_tls::TlsConnector;
// use std::fs;
use std::env;

struct Server {
    id: String,
    name: String,
    image_url: String,
}

// Implement Debug trait manually
impl std::fmt::Debug for Server {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "YourObject {{ id: {}, name: {}, image_url: {} }}",
            self.id, self.name, self.image_url
        )
    }
}

// fn encode_text<'a, T>(_text: T)
// where
//     T: Into<Cow<'a, str>>,
// {}

// fn encode_texts<'a, I>(texts: I)
// where
//     I: IntoIterator,
//     I::Item: Into<Cow<'a, str>>,
// {
//     for text in texts {
//         encode_text(text);
//     }
// }

fn main() {
    // db url with ssl enabled
    // let db_url = "mysql://abhiroop:Arun1956@voxagora-db.mysql.database.azure.com/VoxAgora?";

    // // Path to the CA certificate file
    // let ca_cert_path = "DigiCertGlobalRootCA.crt.pem";

    // // Load the CA certificate
    // let ca_cert_data =
    //     fs::read_to_string(ca_cert_path).expect("Failed to read CA certificate file");

    // // Convert the CA PEM data into a Certificate object
    // let ca_cert =
    //     Certificate::from_pem(ca_cert_data.as_bytes()).expect("Failed to parse CA certificate");

    let builder = OptsBuilder::new();
    let opts = builder
        .ip_or_hostname(Some(env::var("SERVER_NAME").unwrap()))
        .db_name(Some(env::var("DB_NAME").unwrap()))
        .user(Some(env::var("DB_USERNAME").unwrap()))
        .pass(Some(env::var("DB_PASSWORD").unwrap()))
        // .ssl_opts(SslOpts::default().with_root_cert_path(Some("DigiCertGlobalRootCA.crt.pem")))
        .ssl_opts(SslOpts::with_danger_accept_invalid_certs(
            SslOpts::default(),
            true,
        ));
    let db_pool = Pool::new(opts).unwrap();

    // // Create a TlsConnector with the CA certificate
    // let mut tls_connector = TlsConnector::builder();
    // tls_connector.add_root_certificate(ca_cert);

    // create a DB connection
    // let mut conn = Conn::new(db_url).unwrap();
    let mut conn = db_pool.get_conn().expect("Failed to open DB Connection");

    let result = conn
        .query_map(
            "select id, name, imageUrl from voxagora.server",
            |(id, name, image_url)| Server {
                id,
                name,
                image_url,
            },
        )
        .expect("Failed to execute query");

    // Print out the vector using Debug format
    println!("{:?}", result);
}
