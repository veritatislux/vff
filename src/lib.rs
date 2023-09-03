use std::env;
use std::error::Error;
use std::io::Write;
use std::io;

#[macro_use] extern crate debug_here;

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

    let mut distances = Vec::<(usize, bool)>::with_capacity(lines.len());

    for line in lines.iter()
    {
        distances.push(get_distance(target, line));
    }

    let order = order_distances(&distances);

    let mut complete_output = String::new();
    let mut incomplete_output = String::new();

    for idx in order
    {
        if distances[idx].1
        {
            complete_output.push_str(&format!(
                "{}\n",
                //distances[idx].0,
                lines[idx]
            ));
        }
        else
        {
            incomplete_output.push_str(&format!(
                "{}\n",
                //distances[idx].0,
                lines[idx]
            ));
        }
    }

    write!(stdout, "{}{}", complete_output, incomplete_output)?;

    stdout.flush()?;

    Ok(())
}

pub fn get_distance(target: &str, value: &str) -> (usize, bool)
{
    let mut distance: usize = 0;

    let mut target_chars = target.chars();
    let mut value_chars = value.chars();

    loop
    {
        let target_char = target_chars.next();

        if target_char == None
        {
            return (distance + value_chars.count(), true);
        }

        if value_chars.position(|c| {
            if c.to_lowercase().to_string() == target_char.unwrap().to_lowercase().to_string()
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
            return (distance + target_chars.count() + 1, false);
        }
    }
}

pub fn order_distances(distances: &[(usize, bool)]) -> Vec<usize>
{
    let mut order: Vec<usize> = (0..distances.len()).collect();

    order.sort_by(|a, b| distances[*a].0.cmp(&distances[*b].0));

    order
}

#[cfg(test)]
mod test
{
    use crate::*;

    #[test]
    fn exact_matches()
    {
        assert_eq!(get_distance("greek", "greek").0, 0);
        assert_eq!(get_distance("banan", "banan").0, 0);
        assert_eq!(get_distance("pterodactyl", "pterodactyl").0, 0);
        assert_eq!(get_distance("monstertruck", "monstertruck").0, 0);
        assert_eq!(get_distance("a", "a").0, 0);
    }

    #[test]
    fn off_by_one_values()
    {
        assert_eq!(get_distance("greek", "gree").0, 1);
        assert_eq!(get_distance("banan", "bana").0, 1);
        assert_eq!(get_distance("pterodactyl", "pterodacty").0, 1);
        assert_eq!(get_distance("monstertruck", "monstertruc").0, 1);
    }

    #[test]
    fn off_by_one_targets()
    {
        assert_eq!(get_distance("gree", "greek").0, 1);
        assert_eq!(get_distance("bana", "banan").0, 1);
        assert_eq!(get_distance("pterodacty", "pterodactyl").0, 1);
        assert_eq!(get_distance("monstertruc", "monstertruck").0, 1);
    }

    #[test]
    fn zero_match()
    {
        assert_eq!(get_distance("abc", "def").0, 6);
        assert_eq!(get_distance("ghi", "jkl").0, 6);
        assert_eq!(get_distance("xxxxx", "yyyyy").0, 10);
    }

    #[test]
    fn mid_scrambled()
    {
        assert_eq!(get_distance("xxx", "xxyx").0, 1);
        assert_eq!(get_distance("xxx", "xxyzx").0, 2);
        assert_eq!(get_distance("xxx", "xxyzwx").0, 3);
    }

    #[test]
    fn half_right()
    {
        assert_eq!(get_distance("xxxxxx", "xxxyyy").0, 6);
        assert_eq!(get_distance("xxxxxx", "yyyxxx").0, 6);
        assert_eq!(get_distance("xxxxxx", "xyxyxy").0, 6);
    }
}
