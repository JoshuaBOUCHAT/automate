use std::error::Error;

pub struct Automate {
    inner: Vec<Vec<Vec<usize>>>,
    accepteur: Vec<usize>,
}

impl Automate {
    fn with_capacity(n: usize) -> Self {
        Self {
            inner: vec![vec![vec![]; n]; n],
            accepteur: vec![],
        }
    }
    pub fn from_file(file: &String) -> Result<Automate, Box<dyn Error>> {
        let mut lines = file.lines();
        let size = lines
            .next()
            .expect("can't get first line")
            .parse::<usize>()?;
        let mut automate = Automate::with_capacity(size);
        let accept: Vec<_> = lines
            .next()
            .unwrap()
            .split(';')
            .filter_map(|s| s.parse::<usize>().ok())
            .collect();
        automate.accepteur.extend_from_slice(&accept);
        for line in lines {
            let string_numbers: Vec<_> = line.split(';').collect();
            let src: usize = string_numbers[0].parse()?;
            let transition: usize = string_numbers[1].parse()?;
            let dest = string_numbers[2].parse()?;
            automate.inner[src][transition].push(dest);
        }

        Ok(automate)
    }
}
