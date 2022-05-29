#[derive(Debug, PartialEq)]
pub enum Comparison {
    Equal,
    Sublist,
    Superlist,
    Unequal,
}

pub fn sublist<T: PartialEq>(_first_list: &[T], _second_list: &[T]) -> Comparison {
    if _first_list == _second_list {
        Comparison::Equal
    } else if _first_list.len() == 0 {
        Comparison::Sublist
    } else if _second_list.len() == 0 {
        Comparison::Superlist
    } else {
        // If the first list contains all elements of the second list, then returns Superlist
        // If the second list contains all elements of the first list, then returns Sublist
        // Else returns Unequal
        let first_larger = _first_list.len() > _second_list.len();
        if first_larger {
            if _first_list.windows(_second_list.len()).any(|w| w == _second_list) {
                return Comparison::Superlist;
            } else {
                return Comparison::Unequal;
            }
        } else {
            if _second_list.windows(_first_list.len()).any(|w| w == _first_list) {
                return Comparison::Sublist;
            } else {
                return Comparison::Unequal;
            }
        }
    }
}
