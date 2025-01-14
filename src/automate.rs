use std::{error::Error, str::from_utf8};

#[derive(Clone, Debug)]
pub struct Automate<'a> {
    inner: Vec<Vec<(&'a [u8], usize)>>,
    accepteur: Vec<usize>,
    start: usize,
}
fn str_to_char_slice(s: &str) -> Result<&[char], &'static str> {
    // Vérifie que tous les caractères sont des caractères ASCII
    if s.bytes().all(|b| b.is_ascii()) {
        // Conversion sécurisée
        unsafe {
            Ok(std::slice::from_raw_parts(
                s.as_ptr() as *const char,
                s.len(),
            ))
        }
    } else {
        Err("La chaîne contient des caractères non-ASCII")
    }
}

impl<'a> Automate<'a> {
    fn new(n: usize, accepteur: Vec<usize>, start: usize) -> Self {
        Self {
            inner: vec![vec![]; n],
            accepteur,
            start,
        }
    }
    pub fn from_file(file: &str) -> Result<Automate, Box<dyn Error>> {
        let mut lines = file.lines();
        let size = lines.next().expect("first line missing").parse::<usize>()?;
        let start = lines.next().expect("can't get start").parse::<usize>()?;

        let accepteur: Vec<_> = lines
            .next()
            .unwrap()
            .split(';')
            .filter_map(|s| s.parse::<usize>().ok())
            .collect();
        let mut automate = Automate::new(size, accepteur, start);
        for line in lines {
            let string_numbers: Vec<_> = line.split(';').collect();
            let src: usize = string_numbers[0].parse()?;
            let transition = string_numbers[1];
            let dest = string_numbers[2].parse()?;
            automate.inner[src].push((transition.as_bytes(), dest));
        }

        Ok(automate)
    }
    fn is_accepted(&self, to_test: &[u8], state: usize) -> bool {
        if to_test.len() == 0 {
            if self.accepteur.contains(&state) {
                /*  println!(
                    "finish{} with state: {}",
                    from_utf8(to_test).unwrap(),
                    state
                );*/
                return true;
            } else {
                return false;
            }
        }
        for &(transition, dest) in &self.inner[state] {
            let len = transition.len();
            /*println!(
                "testing {} with {}",
                from_utf8(transition).unwrap(),
                from_utf8(to_test).unwrap()
            );*/
            if len <= to_test.len() && to_test.starts_with(transition) {
                /*println!(
                    "match: {} with {}",
                    from_utf8(transition).unwrap(),
                    from_utf8(to_test).unwrap()
                );*/
                if self.is_accepted(&to_test[len..], dest) {
                    return true;
                }
            }
        }
        return false;
    }
    pub fn is_word_accepted(&self, word: &str) -> bool {
        return self.is_accepted(word.as_bytes(), self.start);
    }
}
