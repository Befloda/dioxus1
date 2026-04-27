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

#[component]
fn Salutation() -> Element {
    let nom = "Alice";
    let annee = 2026;

    rsx! {
        p { "Bonjour {nom} ! Nous sommes en {annee}." }
        p { "2 + 2 = {2 + 2}" }
    }
}

#[component]
fn Statut() -> Element {
    let connecte = true;

    rsx! {
        if connecte {
            p { "✅ Vous êtes connecté" }
        } else {
            p { "❌ Veuillez vous connecter" }
        }
    }
}

#[component]
fn ListeFruits() -> Element {
    let fruits = vec!["🍎 Pomme", "🍌 Banane", "🍇 Raisin"];

    rsx! {
        ul {
            // for dans RSX
            p { "Liste de fruits :" }
            for fruit in fruits {
                li { "{fruit}" }
            }
        }
    }
}

#[component]
fn Temperature() -> Element {
    let mut celsius = use_signal(|| 0.0f64);

    // Se recalcule automatiquement quand celsius change
    let fahrenheit = use_memo(move || celsius() * 9.0 / 5.0 + 32.0);

    rsx! {
        p { "Température :" }
        input {
            r#type: "range",
            min: "-50", max: "100",
            oninput: move |e| {
                celsius.set(e.value().parse().unwrap_or(0.0))
            }
        }
        p { "{celsius:.1}°C = {fahrenheit:.1}°F" }
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
        Salutation {}
        Statut {}
        ListeFruits {}
        Temperature {}
    }
}