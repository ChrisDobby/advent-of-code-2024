use std::fs;

#[derive(Debug)]
struct CharPoints {
    x: Vec<(usize, usize)>,
    m: Vec<(usize, usize)>,
    a: Vec<(usize, usize)>,
    s: Vec<(usize, usize)>,
}

fn has_point(point: &(usize, usize), points: &Vec<(usize, usize)>) -> bool {
    points.iter().any(|p| p.0 == point.0 && p.1 == point.1)
}

fn find_up_for_point(point: &(usize, usize), data_points: &CharPoints) -> bool {
    point.0 > 2
        && has_point(&(point.0 - 1, point.1), &data_points.m)
        && has_point(&(point.0 - 2, point.1), &data_points.a)
        && has_point(&(point.0 - 3, point.1), &data_points.s)
}

fn find_down_for_point(point: &(usize, usize), data_points: &CharPoints) -> bool {
    has_point(&(point.0 + 1, point.1), &data_points.m)
        && has_point(&(point.0 + 2, point.1), &data_points.a)
        && has_point(&(point.0 + 3, point.1), &data_points.s)
}

fn find_forward_for_point(point: &(usize, usize), data_points: &CharPoints) -> bool {
    has_point(&(point.0, point.1 + 1), &data_points.m)
        && has_point(&(point.0, point.1 + 2), &data_points.a)
        && has_point(&(point.0, point.1 + 3), &data_points.s)
}

fn find_backward_for_point(point: &(usize, usize), data_points: &CharPoints) -> bool {
    point.1 > 2
        && has_point(&(point.0, point.1 - 1), &data_points.m)
        && has_point(&(point.0, point.1 - 2), &data_points.a)
        && has_point(&(point.0, point.1 - 3), &data_points.s)
}

fn find_diagonal_left_down_for_point(point: &(usize, usize), data_points: &CharPoints) -> bool {
    point.1 > 2
        && has_point(&(point.0 + 1, point.1 - 1), &data_points.m)
        && has_point(&(point.0 + 2, point.1 - 2), &data_points.a)
        && has_point(&(point.0 + 3, point.1 - 3), &data_points.s)
}

fn find_diagonal_right_down_for_point(point: &(usize, usize), data_points: &CharPoints) -> bool {
    has_point(&(point.0 + 1, point.1 + 1), &data_points.m)
        && has_point(&(point.0 + 2, point.1 + 2), &data_points.a)
        && has_point(&(point.0 + 3, point.1 + 3), &data_points.s)
}

fn find_diagonal_left_up_for_point(point: &(usize, usize), data_points: &CharPoints) -> bool {
    point.0 > 2
        && point.1 > 2
        && has_point(&(point.0 - 1, point.1 - 1), &data_points.m)
        && has_point(&(point.0 - 2, point.1 - 2), &data_points.a)
        && has_point(&(point.0 - 3, point.1 - 3), &data_points.s)
}

fn find_diagonal_right_up_for_point(point: &(usize, usize), data_points: &CharPoints) -> bool {
    point.0 > 2
        && has_point(&(point.0 - 1, point.1 + 1), &data_points.m)
        && has_point(&(point.0 - 2, point.1 + 2), &data_points.a)
        && has_point(&(point.0 - 3, point.1 + 3), &data_points.s)
}

fn find_xmas_for_point(point: &(usize, usize), data_points: &CharPoints) -> u32 {
    let functions = [
        find_up_for_point,
        find_down_for_point,
        find_forward_for_point,
        find_backward_for_point,
        find_diagonal_left_down_for_point,
        find_diagonal_right_down_for_point,
        find_diagonal_left_up_for_point,
        find_diagonal_right_up_for_point,
    ];

    functions
        .iter()
        .map(|f| f(point, data_points))
        .filter(|b| *b)
        .count() as u32
}

fn find_x_mas_for_point(point: &(usize, usize), data_points: &CharPoints) -> bool {
    if point.0 == 0 || point.1 == 0 {
        return false;
    }

    let has_left_to_right = (has_point(&(point.0 - 1, point.1 - 1), &data_points.m)
        && has_point(&(point.0 + 1, point.1 + 1), &data_points.s))
        || (has_point(&(point.0 - 1, point.1 - 1), &data_points.s)
            && has_point(&(point.0 + 1, point.1 + 1), &data_points.m));

    let has_right_to_left = (has_point(&(point.0 - 1, point.1 + 1), &data_points.m)
        && has_point(&(point.0 + 1, point.1 - 1), &data_points.s))
        || (has_point(&(point.0 - 1, point.1 + 1), &data_points.s)
            && has_point(&(point.0 + 1, point.1 - 1), &data_points.m));

    has_left_to_right && has_right_to_left
}

fn find_x_mas(data_points: &CharPoints) -> u32 {
    data_points
        .a
        .iter()
        .map(|point| find_x_mas_for_point(point, &data_points))
        .filter(|has_x_mas| *has_x_mas)
        .count() as u32
}

fn find_xmas(data_points: &CharPoints) -> u32 {
    data_points
        .x
        .iter()
        .map(|point| find_xmas_for_point(point, &data_points))
        .sum()
}

fn process_line(data_points: CharPoints, line: &str, line_number: usize) -> CharPoints {
    line.chars()
        .enumerate()
        .fold(data_points, |mut data_points, (col_number, c)| match c {
            'X' => CharPoints {
                x: {
                    data_points.x.push((line_number, col_number));
                    data_points.x
                },
                ..data_points
            },
            'M' => CharPoints {
                m: {
                    data_points.m.push((line_number, col_number));
                    data_points.m
                },
                ..data_points
            },
            'A' => CharPoints {
                a: {
                    data_points.a.push((line_number, col_number));
                    data_points.a
                },
                ..data_points
            },
            'S' => CharPoints {
                s: {
                    data_points.s.push((line_number, col_number));
                    data_points.s
                },
                ..data_points
            },
            _ => data_points,
        })
}

fn main() {
    let content =
        fs::read_to_string("./input.txt").expect("Should have been able to read the file");

    let data_points = content.lines().filter(|s| !s.is_empty()).enumerate().fold(
        CharPoints {
            x: vec![],
            m: vec![],
            a: vec![],
            s: vec![],
        },
        |data_points, (line_number, line)| process_line(data_points, line, line_number),
    );

    let xmas_count = find_xmas(&data_points);
    let x_mas_count = find_x_mas(&data_points);

    println!(
        "Number of XMAS: {} Number of X-MAS: {}",
        xmas_count, x_mas_count
    );
}
