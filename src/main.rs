use proconio::input;
use itertools::{iproduct, Itertools};

struct Output {
    output_id:      u32,
    first_number:   f32,
    second_number:  f32,
    third_number:   f32,
    forth_number:   f32,
    first_operand:  char,
    second_operand: char,
    third_operand:  char,
}

impl Output {
    fn formula(&self) {
        if self.output_id == 0 {
            println!("(({}{}{}){}{}){}{}", self.first_number, self.first_operand, self.second_number, self.second_operand, self.third_number, self.third_operand, self.forth_number);
        }
        else if self.output_id == 1 {
            println!("({}{}({}{}{})){}{}", self.first_number, self.first_operand, self.second_number, self.second_operand, self.third_number, self.third_operand, self.forth_number);
        }
        else if self.output_id == 2 {
            println!("{}{}(({}{}{}){}{})", self.first_number, self.first_operand, self.second_number, self.second_operand, self.third_number, self.third_operand, self.forth_number);
        }
        else if self.output_id == 3 {
            println!("{}{}({}{}({}{}{}))", self.first_number, self.first_operand, self.second_number, self.second_operand, self.third_number, self.third_operand, self.forth_number);
        }
        else if self.output_id == 4 {
            println!("({}{}{}){}({}{}{})", self.first_number, self.first_operand, self.second_number, self.second_operand, self.third_number, self.third_operand, self.forth_number);
        }
    }
}
fn add(a: f32, b:f32)  -> (f32, char, bool) { (a + b, '+', false) }
fn sub(a: f32, b:f32)  -> (f32, char, bool) { (a - b, '-', false) }
fn rsub(a: f32, b:f32) -> (f32, char, bool) { (b - a, '-', true) }
fn mul(a: f32, b:f32)  -> (f32, char, bool) { (a * b, '*', false) }
fn div(a: f32, b:f32)  -> (f32, char, bool) { (a / b, '/', false) }
fn rdiv(a: f32, b:f32) -> (f32, char, bool) { (b / a, '/', true) }

fn main() {
    input! { v: [f32; 4] }
    let calc_funcs: [fn(f32, f32) -> (f32, char, bool); 6] = [add, sub, rsub, mul, div, rdiv];
    // ((a o1 b) o2 c) 03 d
    // 4P2
    for perm in v.iter().permutations(2) {
        let mut arr = perm;
        let mut tmp = v.iter().collect_vec();
        tmp.remove(tmp.iter().position(|&p| p == arr[0]).unwrap());
        tmp.remove(tmp.iter().position(|&p| p == arr[1]).unwrap());
        arr.append(&mut tmp);

        for (fn_1, fn_2, fn_3) in iproduct!(calc_funcs.iter(), calc_funcs.iter(), calc_funcs.iter()) {
            let (ans_1, ope_1, reverse_1) = fn_1(*arr[0], *arr[1]);
            let (ans_2, ope_2, reverse_2) = fn_2(ans_1, *arr[2]);
            let (ans_3, ope_3, reverse_3) = fn_3(ans_2, *arr[3]);
            if ans_3 == 10.0 {
                let (first_operand, second_operand, third_operand) = if reverse_3 {
                    if reverse_2 {
                        (ope_3, ope_2, ope_1)
                    } else {
                        (ope_3, ope_1, ope_2)
                    }
                } else {
                    if reverse_2 {
                        (ope_2, ope_1, ope_3)
                    } else {
                        (ope_1, ope_2, ope_3)
                    }
                };

                let (output_id, 
                    first_number, 
                    second_number, 
                    third_number, 
                    forth_number
                ) = if !reverse_1 && !reverse_2 && !reverse_3 {
                    (0, *arr[0], *arr[1], *arr[2], *arr[3])
                } else if reverse_1 && !reverse_2 && !reverse_3 {
                    (0, *arr[1], *arr[0], *arr[2], *arr[3])
                } else if !reverse_1 && reverse_2 && !reverse_3 {
                    (1, *arr[2], *arr[0], *arr[1], *arr[3])
                } else if reverse_1 && reverse_2 && !reverse_3 {
                    (1, *arr[2], *arr[1], *arr[0], *arr[3])
                } else if reverse_1 && !reverse_2 && reverse_3 {
                    (2, *arr[3], *arr[1], *arr[0], *arr[2])
                } else if !reverse_1 && !reverse_2 && reverse_3 {
                    (2, *arr[3], *arr[0],*arr[1],  *arr[2])
                } else if !reverse_1 && reverse_2 && reverse_3 {
                    (3, *arr[3], *arr[2], *arr[0], *arr[1])
                } else {
                    (3, *arr[3], *arr[2], *arr[1], *arr[0])
                };

                let output = Output { 
                    output_id,
                    first_number, 
                    second_number, 
                    third_number, 
                    forth_number, 
                    first_operand, 
                    second_operand, 
                    third_operand,
                };
                output.formula();
            }
        }
    }
    // (a o1 b) o3 (c o2 d)
    // ab cd
    // ac bd
    // ad cb
    let t = v.to_vec();
    let arrs = [
        [t[0], t[1], t[2], t[3]],
        [t[0], t[2], t[1], t[3]],
        [t[0], t[3], t[2], t[1]],
    ];

    for arr in arrs.iter() {
        for (fn_1, fn_2, fn_3) in iproduct!(calc_funcs.iter(), calc_funcs.iter(), calc_funcs.iter()) {
            let (ans_1, ope_1, reverse_1) = fn_1(arr[0], arr[1]);
            let (ans_2, ope_2, reverse_2) = fn_2(arr[2], arr[3]);
            let (ans_3, ope_3, reverse_3) = fn_3(ans_1, ans_2);
            if ans_3 == 10.0 {
                let output_id = 4;

                let mut ans_arr = arr.to_vec();
                let (
                    first_number, 
                    second_number, 
                    third_number, 
                    forth_number
                ) = {
                    if reverse_1 {
                    ans_arr.swap(0, 1);
                    };
                    if reverse_2 {
                        ans_arr.swap(2, 3);
                    };
                    if reverse_3 { 
                        ans_arr.swap(0, 2);
                        ans_arr.swap(1, 3);
                    };
                    (ans_arr[0], ans_arr[1], ans_arr[2], ans_arr[3])
                };

                let (
                    first_operand, 
                    second_operand, 
                    third_operand
                ) = if reverse_3 {
                    (ope_2, ope_3, ope_1)
                } else {
                    (ope_1, ope_3, ope_2)
                };

                let output = Output {
                    output_id,
                    first_number,
                    second_number,
                    third_number,
                    forth_number,
                    first_operand,
                    second_operand,
                    third_operand,
                };

                output.formula();
            }
        }
    }
}

