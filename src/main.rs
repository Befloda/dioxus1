use dioxus::prelude::*;

fn main() {
    // Point d'entrée : lance l'application
    dioxus::launch(App);
}

// Un composant simple (PascalCase obligatoire)
#[component]
fn Carte() -> Element {
    rsx! {
        div {
            class: "carte",
            style: "border: 1px solid #ccc; padding: 16px;",
            h2 { "Titre de la carte" }
            p { "Contenu de la carte." }
        }
    }
}

#[component]
fn Carte1() -> Element {
    rsx! {
        div {
            class: "carte",
            style: "border: 1px solid #ccc; padding: 16px;",
            h2 { "Titre de la carte 1" }
            p { "Contenu de la carte." }
        }
    }
}

// Utilisation dans App
#[component]
fn App() -> Element {
    rsx! {
        // On utilise le composant comme un élément HTML
        Carte {}
        Carte1 {}
        Carte {}
    }
}