use actix_web::{get, web, App, HttpRequest, HttpResponse, HttpServer, Responder, Result};
use actix_files::Files;
use serde::Serialize;
use serde_json::json;
use handlebars::Handlebars;

// Estrutura da Notícia (mesma de antes)
#[derive(Serialize)]
struct Noticia {
    titulo: String,
}

// Handler da API (mesmo de antes)
#[get("/api/latest-news")]
async fn get_latest_news() -> Result<impl Responder> {
    let noticias: Vec<Noticia> = vec![]; 
    Ok(web::Json(noticias))
}

// Handler para a Home Page (/)
#[get("/")]
async fn index(hb: web::Data<Handlebars<'static>>) -> impl Responder {
    let data = json!({
        "page_title": "Home",
        "content_message": "Home page temporária"
    });
    let body = hb.render("index", &data).unwrap();
    HttpResponse::Ok().body(body)
}

// Handler genérico para todas as páginas de categoria (Tarefa 03)
async fn generic_category_page(
    hb: web::Data<Handlebars<'static>>,
    req: HttpRequest,
) -> impl Responder {
    // Pega o caminho (ex: "/policial")
    let path = req.path(); 
    
    // Define o título da página com base no caminho
    let category_name = match path {
        "/geral" => "Notícias Gerais",
        "/policial" => "Policial",
        "/politica" => "Política",
        "/entretenimento" => "Entretenimento",
        "/educacao" => "Educação",
        "/saude" => "Saúde",
        "/esportes" => "Esportes",
        _ => "Página", // Fallback
    };

    let data = json!({
        "page_title": category_name,
        "category_name": category_name,
        "content_message": "Esta página está em construção."
    });
    
    // Renderiza o template "category.hbs"
    let body = hb.render("category", &data).unwrap();
    HttpResponse::Ok().body(body)
}


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    
    // --- Configuração do Handlebars (Tarefa 01) ---
    let mut hb = Handlebars::new();
    
    // Registra os templates parciais (header/footer)
    hb.register_partial("header", 
        std::fs::read_to_string("templates/partials/header.hbs").unwrap()
    ).unwrap();
    hb.register_partial("footer", 
        std::fs::read_to_string("templates/partials/footer.hbs").unwrap()
    ).unwrap();

    // Registra os templates de página principais
    hb.register_templates_directory(".hbs", "./templates")
       .unwrap();
    
    // Coloca o Handlebars em um estado gerenciado pelo Actix (web::Data)
    let hb_data = web::Data::new(hb);
    // ---------------------------------------------
    
    println!("🚀 Servidor iniciado em http://127.0.0.1:8080");

    HttpServer::new(move || {
        App::new()
            // 1. Disponibiliza o Handlebars para os handlers
            .app_data(hb_data.clone())

            // 2. Rota da API
            .service(get_latest_news)
            
            // 3. Rota da Home
            .service(index)

            // 4. Rotas das Categorias (Tarefa 03)
            .route("/geral", web::get().to(generic_category_page))
            .route("/policial", web::get().to(generic_category_page))
            .route("/politica", web::get().to(generic_category_page))
            .route("/entretenimento", web::get().to(generic_category_page))
            .route("/educacao", web::get().to(generic_category_page))
            .route("/saude", web::get().to(generic_category_page))
            .route("/esportes", web::get().to(generic_category_page))
            
            // 5. Servir arquivos estáticos (CSS/JS)
            // DEVE ser registrado por último!
            .service(
                Files::new("/static", "./static")
                    .use_last_modified(true),
            )
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}