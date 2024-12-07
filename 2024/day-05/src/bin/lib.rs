pub fn count_correct_pages(rules: Vec<(u32, u32)>, pages: Vec<Vec<u32>>) -> u32 {
    pages
        .into_iter()
        .filter(|page| {
            rules
                .iter()
                .filter(|(x, y)| page.contains(x) && page.contains(y))
                .all(|(x, y)| check_order(page.clone(), *x, *y))
        })
        .map(|page| {
            let middle = page.len() / 2;
            page[middle]
        })
        .sum()
}

fn check_order(page: Vec<u32>, x: u32, y: u32) -> bool {
    let x_pos = page.iter().position(|n| *n == x).unwrap();
    let y_pos = page.iter().position(|n| *n == y).unwrap();

    y_pos > x_pos
}

fn main() {
    
}