use axum::{
    response::Html,
    routing::get,
    Router,
};
use std::net::SocketAddr;

#[tokio::main]
async fn main() {
    // Créer les routes de l'application
    let app = Router::new()
        .route("/", get(home))
        .route("/health", get(health_check));

    // Définir l'adresse du serveur
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("🚀 Serveur démarré sur http://{}", addr);

    // Démarrer le serveur
    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

// Route pour la page d'accueil
async fn home() -> Html<&'static str> {
    Html(include_str!("templates/home.html"))
}

// Route de vérification de santé
async fn health_check() -> &'static str {
    "OK - Server is healthy"
}
