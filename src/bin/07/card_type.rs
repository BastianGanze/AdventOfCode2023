#[derive(Debug, Hash, Eq, PartialEq, Clone, Copy)]
pub enum CardType {
    A,
    K,
    Q,
    J,
    T,
    Nine,
    Eight,
    Seven,
    Six,
    Five,
    Four,
    Three,
    Two,
}

impl PartialOrd for CardType {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for CardType {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        use std::cmp::Ordering;
        use CardType::*;

        match (self, other) {
            (x, y) if x == y => Ordering::Equal,
            (A, _) => Ordering::Greater,
            (_, A) => Ordering::Less,
            (K, _) => Ordering::Greater,
            (_, K) => Ordering::Less,
            (Q, _) => Ordering::Greater,
            (_, Q) => Ordering::Less,
            (J, _) => Ordering::Greater,
            (_, J) => Ordering::Less,
            (T, _) => Ordering::Greater,
            (_, T) => Ordering::Less,
            (Nine, _) => Ordering::Greater,
            (_, Nine) => Ordering::Less,
            (Eight, _) => Ordering::Greater,
            (_, Eight) => Ordering::Less,
            (Seven, _) => Ordering::Greater,
            (_, Seven) => Ordering::Less,
            (Six, _) => Ordering::Greater,
            (_, Six) => Ordering::Less,
            (Five, _) => Ordering::Greater,
            (_, Five) => Ordering::Less,
            (Four, _) => Ordering::Greater,
            (_, Four) => Ordering::Less,
            (Three, _) => Ordering::Greater,
            (_, Three) => Ordering::Less,
            (Two, _) => Ordering::Less,
        }
    }
}

pub fn char_to_card_type(c: char) -> Option<CardType> {
    use CardType::*;
    match c {
        'A' => Some(A),
        'K' => Some(K),
        'Q' => Some(Q),
        'J' => Some(J),
        'T' => Some(T),
        '9' => Some(Nine),
        '8' => Some(Eight),
        '7' => Some(Seven),
        '6' => Some(Six),
        '5' => Some(Five),
        '4' => Some(Four),
        '3' => Some(Three),
        '2' => Some(Two),
        _ => None,
    }
}
