use fancy_regex::{Captures, Regex};

pub async fn replace_entities_i_hate_my_life() -> std::io::Result<()> {
    let regex = Regex::new("&(?!(amp|quot|gt|lt)\\b)([a-z]+);").unwrap();
    for file in glob::glob("./.data/**/functions/**/*.xml").unwrap() {
        let file = file.unwrap();
        let file_content = tokio::fs::read_to_string(&file).await?;
        let replaced_content = regex.replace_all(&file_content, |e: &Captures| {
            format!("<constant>{}</constant>", e.get(2).unwrap().as_str())
        });

        tokio::fs::write(file, replaced_content.to_string()).await?;
    }

    Ok(())
}
