use serde::Deserialize;

#[derive(Deserialize, Debug)]
struct Usuario {
    login: String,
    name: Option<String>,
    public_repos: u32,
    followers: u32,
}

async fn buscar_usuario(username: &str) -> Result<Usuario, reqwest::Error> {
    let url = format!("https://api.github.com/users/{}", username);

    let usuario = reqwest::Client::new()
        .get(&url)
        .header("User-Agent", "buscador-rust")
        .send()
        .await?
        .json::<Usuario>()
        .await?;

    Ok(usuario)
}

#[tokio::main]
async fn main() {
    match buscar_usuario("torvalds").await {
        Ok(usuario) => {
            println!("Login: {}", usuario.login);
            match usuario.name {
                Some(name) => println!("Nome: {}", name),
                None => println!("Nome: Nao informado"),
            }
            println!("Repos publicos: {}", usuario.public_repos);
            println!("Seguidores: {}", usuario.followers);
        }
        Err(error) => {
            println!("Erro ao buscar usuario: {}", error);
        }
    }
}
