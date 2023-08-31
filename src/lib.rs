use std::env;
use std::error::Error;
use std::io::Write;
use std::io;

const MAX_RESULTS: usize = 4;

pub type EResult<T> = Result<T, Box<dyn Error>>;

pub fn read_cmdline_args() -> EResult<()>
{
    let args = env::args();

    if args.len() != 3
    {
        return Err(Box::new(io::Error::new(
            io::ErrorKind::InvalidInput,
            "Usage: vff <target> <source>."
        )));
    }

    let arg_values: Vec<String> = args.collect();

    fuzzy_find(&arg_values[1], &arg_values[2])
}

pub fn fuzzy_find(target: &str, source: &str) -> EResult<()>
{
    let mut stdout = io::stdout();

    let lines: Vec<&str> = source.split('\n').collect();

    let mut distances = Vec::<usize>::with_capacity(lines.len());

    for line in lines.iter()
    {
        distances.push(get_distance(target, line));
    }

    let order = order_distances(&distances);

    for i in 0..MAX_RESULTS
    {
        match order.get(i)
        {
            None => { break; }
            Some(idx) => { writeln!(stdout, "{}: {}", distances[*idx], lines[*idx])?; }
        }
    }

    stdout.flush()?;

    Ok(())
}

pub fn get_distance(target: &str, value: &str) -> usize
{
    let mut distance: usize = 0;

    let mut target_chars = target.chars();
    let mut value_chars = value.chars();

    let mut next_target: Option<char> = target_chars.next();
    let mut next_value: Option<char> = value_chars.next();

    let mut found_all: bool = false;

    while let Some(next_target_char) = next_target
    {
        if next_value == None
        {
            break;
        }

        if next_value != next_target
        {
            distance += 1;

            if value_chars.position(|c| {
                if c == next_target_char
                {
                    true
                }
                else
                {
                    distance += 1;
                    false
                }
            }) == None
            {
                distance += 1;
            }
            else if target_chars.clone().peekable().peek() == None
            {
                found_all = true;
            }
        }

        next_target = target_chars.next();
        next_value = value_chars.next();
    }

    if let Some(_) = next_value
    {
        distance += value_chars.count() + 1;

        if next_target == None
        {
            found_all = true;
        }
    }

    if let Some(_) = next_target
    {
        distance += target_chars.count() + 1;
    }

    if found_all
    {
        distance /= 2;
    }

    distance
}

pub fn order_distances(distances: &Vec<usize>) -> Vec<usize>
{
    let mut order: Vec<usize> = (0..distances.len()).collect();

    order.sort_by(|a, b| distances[*a].cmp(&distances[*b]));

    order
}

#[cfg(test)]
mod tests
{
    use crate::*;

    #[test]
    fn exact_matches()
    {
        assert_eq!(get_distance("greek", "greek"), 0);
        assert_eq!(get_distance("banan", "banan"), 0);
        assert_eq!(get_distance("pterodactyl", "pterodactyl"), 0);
        assert_eq!(get_distance("monstertruck", "monstertruck"), 0);
        assert_eq!(get_distance("a", "a"), 0);
    }

    #[test]
    fn off_by_one_values()
    {
        assert_eq!(get_distance("greek", "gree"), 1);
        assert_eq!(get_distance("banan", "bana"), 1);
        assert_eq!(get_distance("pterodactyl", "pterodacty"), 1);
        assert_eq!(get_distance("monstertruck", "monstertruc"), 1);
    }

    #[test]
    fn off_by_one_targets()
    {
        assert_eq!(get_distance("gree", "greek"), 1);
        assert_eq!(get_distance("bana", "banan"), 1);
        assert_eq!(get_distance("pterodacty", "pterodactyl"), 1);
        assert_eq!(get_distance("monstertruc", "monstertruck"), 1);
    }

    #[test]
    fn zero_match()
    {
        assert_eq!(get_distance("abc", "def"), 6);
        assert_eq!(get_distance("ghi", "jkl"), 6);
        assert_eq!(get_distance("xxxxx", "yyyyy"), 10);
    }

    #[test]
    fn mid_scrambled()
    {
        assert_eq!(get_distance("xxx", "xxyx"), 1);
        assert_eq!(get_distance("xxx", "xxyzx"), 2);
        assert_eq!(get_distance("xxx", "xxyzwx"), 3);
    }

    #[test]
    fn half_right()
    {
        assert_eq!(get_distance("xxxxxx", "xxxyyy"), 6);
        assert_eq!(get_distance("xxxxxx", "yyyxxx"), 6);
        assert_eq!(get_distance("xxxxxx", "xyxyxy"), 6);
    }
}
