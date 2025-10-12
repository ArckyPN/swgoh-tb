use crate::Planet;

use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Phase {
    #[serde(rename = "Dark")]
    pub dark: Planet,
    #[serde(rename = "Mixed")]
    pub mixed: Planet,
    #[serde(rename = "Light")]
    pub light: Planet,
    #[serde(rename = "Bonus")]
    pub bonus: Option<Planet>,
}

impl Phase {
    pub fn num(&self) -> usize {
        match self.bonus {
            Some(_) => 4,
            None => 3,
        }
    }
}

impl<'a> Phase {
    pub fn iter(&'a self) -> PhaseIterator<'a> {
        <&Self as IntoIterator>::into_iter(self)
    }
}

impl<'a> IntoIterator for &'a Phase {
    type Item = &'a Planet;
    type IntoIter = PhaseIterator<'a>;
    fn into_iter(self) -> Self::IntoIter {
        PhaseIterator {
            phase: self,
            idx: 0,
        }
    }
}

pub struct PhaseIterator<'a> {
    phase: &'a Phase,
    idx: usize,
}

impl<'a> PhaseIterator<'a> {
    fn ret(&mut self, next: &'a Planet) -> Option<&'a Planet> {
        if self.idx > 3 {
            return None;
        }
        self.idx += 1;
        Some(next)
    }
}

impl<'a> Iterator for PhaseIterator<'a> {
    type Item = &'a Planet;
    fn next(&mut self) -> Option<Self::Item> {
        match (self.idx, &self.phase.bonus) {
            (0, None | Some(_)) => self.ret(&self.phase.dark),
            (1, None) => self.ret(&self.phase.mixed),
            (2, None) | (3, Some(_)) => self.ret(&self.phase.light),
            (1, Some(bonus)) if bonus.is_mandalore() => self.ret(bonus),
            (2, Some(bonus)) if bonus.is_mandalore() => self.ret(&self.phase.mixed),
            (1, Some(bonus)) if bonus.is_zeffo() => self.ret(&self.phase.mixed),
            (2, Some(bonus)) if bonus.is_zeffo() => self.ret(bonus),
            _ => None,
        }
    }
}
