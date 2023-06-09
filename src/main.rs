use std::collections::HashSet;

use anyhow::{anyhow, Context, Result};
use clap::{crate_authors, crate_description, crate_name, crate_version, Arg, Command};
use rayon::prelude::*;

fn args() -> Command {
    Command::new(crate_name!())
        .author(crate_authors!())
        .version(crate_version!())
        .about(crate_description!())
        .arg(
            Arg::new("file")
                .short('f')
                .long("file")
                .help("File to be inspected")
                .required(true),
        )
        .arg(
            Arg::new("char")
                .short('c')
                .long("chars")
                .help("Chars to be searched. Please separate them with spaces or don't separate at all. Some symbols should be shielded with backslashes, like this: \"\\ \"")
                .required(true)
                .num_args(1..),
        )
}

fn main() -> Result<()> {
    let start = std::time::Instant::now();
    let args = args().get_matches();

    let file_path = args
        .get_one::<String>("file")
        .ok_or(anyhow!("File is not porvided"))?;
    let chars = args
        .get_many::<String>("char")
        .ok_or(anyhow!("Argument error"))?
        .flat_map(|s| s.chars());

    let pattern: HashSet<char> = HashSet::from_iter(chars);

    let content = std::fs::read_to_string(file_path).context("Failed to read file content")?;

    let count = count(&content, pattern);
    let elapsed = start.elapsed();
    println!("Found {count} symbol occurences total in {elapsed:?}");

    Ok(())
}

fn count(content: &String, pattern: HashSet<char>) -> usize {
    content.par_matches(|c| pattern.contains(&c)).count()
}

#[cfg(test)]
mod test {
    use std::collections::HashSet;

    use crate::count;

    #[test]
    fn emoji() {
        let content = String::from("Lorem ipsum 🍆🍑💦🌽🌽🌽🥑");
        let mut pattern = HashSet::new();
        pattern.insert('🌽');

        assert_eq!(count(&content, pattern), 3);
    }

    #[test]
    fn empty() {
        let content = String::new();
        let mut pattern = HashSet::new();
        pattern.insert('o');

        assert_eq!(count(&content, pattern), 0);
    }

    #[test]
    fn not_alphanumeric() {
        let content = String::from("https://habr.com/ru/articles/733454/ Долгое время я жил в районе, где общественный транспорт был скорее проблемой, чем благом (привет, Кудрово!). Приходилось в любую погоду ходить пешком до метро три километра, в мороз, слякоть и зной. Спустя некоторое время я переехал и появилась возможность доезжать от офиса до дома на прямом автобусе. Это звучит очень клево, но единственный маршрут ходит с большим интервалом и как попало. Приходилось или подолгу стоять на остановке, или нервно поглядывать на Яндекс.Карты и отслеживать перемещения ближайшего автобуса последние полчаса перед выходом, что сводило на нет остатки продуктивности и неслабо раздражало.");
        let mut pattern = HashSet::new();
        pattern.insert(' ');
        pattern.insert('.');
        pattern.insert(',');
        pattern.insert('!');
        pattern.insert('(');
        pattern.insert(')');
        pattern.insert(':');
        pattern.insert('/');

        assert_eq!(count(&content, pattern), 115);
    }
}
