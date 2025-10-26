document.addEventListener("DOMContentLoaded", () => {
    // 1. Carrega o Ticker de Notícias (Função antiga)
    loadLatestNews();
    
    // 2. Controla o Menu Mobile (Tarefa 02)
    const menuToggle = document.getElementById("menu-toggle");
    const mainMenu = document.getElementById("main-menu");
    const toggleIcon = menuToggle ? menuToggle.querySelector("i") : null;

    if (menuToggle && mainMenu && toggleIcon) {
        menuToggle.addEventListener("click", () => {
            // Verifica se está expandido (classe 'active')
            const isExpanded = mainMenu.classList.contains("active");
            
            // Alterna a classe 'active' no menu
            mainMenu.classList.toggle("active");
            
            // Atualiza os atributos de acessibilidade (ARIA)
            menuToggle.setAttribute("aria-expanded", !isExpanded);
            
            // Alterna o ícone (Hamburguer / X)
            if (!isExpanded) {
                toggleIcon.classList.remove("fa-bars");
                toggleIcon.classList.add("fa-times"); // Ícone 'X'
            } else {
                toggleIcon.classList.remove("fa-times");
                toggleIcon.classList.add("fa-bars"); // Ícone 'Barras'
            }
        });
    }
});

// Função para carregar as últimas notícias (igual à versão anterior)
async function loadLatestNews() {
    const ticker = document.getElementById("news-ticker");
    if (!ticker) return;

    try {
        // O endpoint da API continua o mesmo
        const response = await fetch('/api/latest-news');
        if (!response.ok) {
            throw new Error(`Erro HTTP: ${response.status}`);
        }
        
        let noticias = await response.json();

        if (noticias.length === 0) {
            noticias = [
                { titulo: "inserir sua notícia aqui" },
                { titulo: "inserir sua notícia aqui" },
                { titulo: "inserir sua notícia aqui" },
                { titulo: "inserir sua notícia aqui" },
                { titulo: "inserir sua notícia aqui" },
            ];
        }

        const separator = " --- ";
        let tickerHTML = "";
        const itemsToDisplay = [...noticias, ...noticias]; 

        itemsToDisplay.forEach(noticia => {
            tickerHTML += `<span>${noticia.titulo}</span>${separator}`;
        });

        ticker.innerHTML = tickerHTML;

    } catch (error) {
        console.error("Falha ao buscar notícias:", error);
        ticker.innerHTML = `<span>Erro ao carregar notícias.</span>`;
    }
}