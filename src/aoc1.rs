pub struct Lists {
    list1: Vec<i32>,
    list2: Vec<i32>,
}

#[derive(Debug, thiserror::Error)]
pub enum ListsError {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    #[error("Parse error: {0}")]
    Parse(#[from] std::num::ParseIntError),
}

impl Lists {
    fn init() -> Result<Lists, ListsError> {
        let input =
            std::fs::read_to_string("/home/vaibhav/Desktop/projects/learnings/inputs/aco1.txt")?;

        let (list1, list2): (Vec<_>, Vec<_>) = input
            .lines()
            .map(|line| {
                let nums = line
                    .split_whitespace()
                    .map(str::parse::<i32>)
                    .collect::<Result<Vec<_>, _>>()?;
                Ok::<(i32, i32), ListsError>((nums[0], nums[1]))
            })
            .collect::<Result<Vec<_>, _>>()?
            .into_iter()
            .unzip();

        Ok(Lists { list1, list2 })
    }

    fn sort(&mut self) {
        self.list1.sort();
        self.list2.sort();
    }

    fn solve(&mut self) -> i32 {
        self.sort();
        let mut res = 0;
        for i in 0..self.list1.len() {
            res += (self.list1[i] - self.list2[i]).abs();
        }
        res
    }
}

pub fn solve() -> anyhow::Result<()> {
    let mut lists = Lists::init()?;
    let res = lists.solve();
    println!("{}", res);
    Ok(())
}
