use anyhow::Result;
use dotenv::dotenv;
use sqlx::{FromRow, SqlitePool};
use std::env;

// Struct que representa uma linha da tabela `paths`
#[derive(Debug, FromRow)]
struct Path {
    id: i32,
    path: String,
    score: i32,
}

// Conecta ao banco de dados SQLite
async fn connect_db() -> Result<SqlitePool> {
    dotenv().ok(); // Carrega variáveis de ambiente do arquivo .env
    let database_url = env::var("DATABASE_URL")?;
    let pool = SqlitePool::connect(&database_url).await?;
    println!("{:?}", pool);
    Ok(pool)
}

// Obtém todos os registros da tabela `paths`
async fn get_all_paths(pool: &SqlitePool) -> Result<Vec<Path>> {
    let paths = sqlx::query_as::<_, Path>("SELECT * FROM paths")
        .fetch_all(pool)
        .await?;

    Ok(paths)
}

// Obtém um registro pelo ID
async fn get_path_by_id(pool: &SqlitePool, id: i32) -> Result<Option<Path>> {
    let path = sqlx::query_as::<_, Path>("SELECT * FROM paths WHERE id = ?")
        .bind(id)
        .fetch_optional(pool)
        .await?;
    Ok(path)
}

// Obtém um registro pela coluna `path`
async fn get_path_by_path(pool: &SqlitePool, path_name: &str) -> Result<Option<Path>> {
    let path = sqlx::query_as::<_, Path>("SELECT * FROM paths WHERE path = ?")
        .bind(path_name)
        .fetch_optional(pool)
        .await?;
    Ok(path)
}

// Obtém um registro pela coluna `score`
async fn get_path_by_score(pool: &SqlitePool, score: i32) -> Result<Option<Path>> {
    let path = sqlx::query_as::<_, Path>("SELECT * FROM paths WHERE score = ?")
        .bind(score)
        .fetch_optional(pool)
        .await?;
    Ok(path)
}

// Insere um novo registro na tabela `paths`
async fn insert_path(pool: &SqlitePool, path_name: &str, score: i32) -> Result<i64> {
    let result = sqlx::query("INSERT INTO paths (path, score) VALUES (?, ?)")
        .bind(path_name)
        .bind(score)
        .execute(pool)
        .await?;

    //println!("{:?}", result);
    Ok(result.last_insert_rowid())
}

// Atualiza um registro na tabela `paths`
async fn update_path(pool: &SqlitePool, id: i32, new_path: &str, new_score: i32) -> Result<()> {
    sqlx::query("UPDATE paths SET path = ?, score = ? WHERE id = ?")
        .bind(new_path)
        .bind(new_score)
        .bind(id)
        .execute(pool)
        .await?;
    Ok(())
}

// Remove um registro da tabela `paths`
async fn delete_path(pool: &SqlitePool, id: i32) -> Result<()> {
    sqlx::query("DELETE FROM paths WHERE id = ?")
        .bind(id)
        .execute(pool)
        .await?;
    Ok(())
}

#[tokio::main]
pub async fn main() -> Result<()> {
    let pool = connect_db().await?;

    // ---------------
    // --- TESTING ---
    // ---------------
    //let new_id: i64;
    //
    //match insert_path(&pool, "/new/path", 100).await {
    //    Ok(id) => {
    //        println!("Novo registro inserido com ID: {}", id);
    //        new_id = id;
    //    }
    //    Err(e) => {
    //        eprintln!("Erro ao inserir registro: {:?}", e);
    //        panic!("At the disco...");
    //    }
    //}
    //
    //let all_paths: &str;
    //match get_all_paths(&pool).await {
    //    Ok(paths) => {
    //        println!("All paths: {:?}", paths)
    //    }
    //    Err(e) => {
    //        eprintln!("Erro ao buscar tudo: {:?}", e);
    //        panic!("At the disco...");
    //    }
    //}

    // Exemplo de uso das funções:

    // Obter um registro pelo ID
    if let Some(path) = get_path_by_id(&pool, 1).await? {
        println!("Registro encontrado pelo ID: {:?}", path);
    }

    // Obter um registro pela coluna `path`
    if let Some(path) = get_path_by_path(&pool, "/new/path").await? {
        println!("Registro encontrado pelo path: {:?}", path);
    }

    // Obter um registro pela coluna `score`
    if let Some(path) = get_path_by_score(&pool, 100).await? {
        println!("Registro encontrado pelo score: {:?}", path);
    }

    //// Atualizar um registro
    //update_path(&pool, new_id.try_into().unwrap(), "/updated/path", 200).await?;
    //println!("Registro atualizado com sucesso!");
    //
    //// Remover um registro
    //delete_path(&pool, new_id.try_into().unwrap()).await?;
    //println!("Registro removido com sucesso!");

    Ok(())
}
