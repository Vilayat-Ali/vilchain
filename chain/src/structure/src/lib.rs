pub mod block;
pub mod chain;
pub mod txn;

use serde::{Deserialize, Serialize};
use std::fmt::Display;


#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum Sign {
    Positive,
    Negative
}

impl Default for Sign {
    fn default() -> Self {
        Self::Positive
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct BigNum {
    sign: Sign,
    int_val: Vec<u8>,
    frac_val: [u8; 4],
}

impl Default for BigNum {
    fn default() -> Self {
        Self {
            sign: Sign::default(),
            int_val: vec![0],
            frac_val: [0; 4]
        }
    }
}

impl Display for BigNum {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}{}.{}",
            match &self.sign {
                &Sign::Positive => "+",
                &Sign::Negative => "-",
            },
            &self
                .int_val
                .iter()
                .map(|x| x.to_string())
                .collect::<String>(),
            &self
                .frac_val
                .iter()
                .map(|x| x.to_string())
                .collect::<String>(),
        )
    }
}

impl BigNum {
    pub fn from_string<S>(num: S) -> Result<Self, String> 
        where S: Into<String>,
        {
            let num_str: String = num.into();
            let num_arr = num_str.trim().split('.').map(|x| x.to_string()).collect::<Vec<String>>();

            match num_arr.len() {
                0 => Err("Invalid Big Number Format: The string passed is empty".to_string()),
                1 => {
                    let mut int_val_vec: Vec<u8> = Vec::with_capacity(num_arr[0].len());
                    for digit_char in num_arr[0].chars() {
                        match digit_char.to_string().parse::<u8>() {
                            Ok(num_val) => int_val_vec.push(num_val),
                            Err(_) => return Err("Invalid Big Number Format: The string contains non-numeric characters".to_string())
                        }
                    }
                    Ok(Self {
                        sign: Sign::default(),
                        int_val: int_val_vec,
                        frac_val: [0; 4]
                    })
                },
                2 => {
                    if num_arr[1].len() > 4 {
                        return Err("Invalid Big Number Format: Fractional values cannot be more than 4 digits long".to_string());
                    }
                    let mut int_val_vec: Vec<u8> = Vec::with_capacity(num_arr[0].len());
                    let mut frac_val_vec: [u8; 4] = [0; 4];
                    let mut frac_idx = 0;
                    for digit_char in num_arr[0].chars() {
                        match digit_char.to_string().parse::<u8>() {
                            Ok(num_val) => int_val_vec.push(num_val),
                            Err(_) => return Err("Invalid Big Number Format: The string contains non-numeric characters".to_string())
                        }
                    }
                    for digit_char in num_arr[1].chars() {
                        match digit_char.to_string().parse::<u8>() {
                            Ok(num_val) => {
                                frac_val_vec[frac_idx] = num_val;
                                frac_idx += 1;
                            },
                            Err(_) => return Err("Invalid Big Number Format: The string contains non-numeric characters".to_string())
                        }
                    }
                    return Ok(Self {
                        sign: Sign::Positive,
                        int_val: int_val_vec,
                        frac_val: frac_val_vec
                    });
                },
                _ => return Err("Invalid Big Number Format: The string passed contains multiple decimals.".to_string()),
            }
    }

    pub fn add(&mut self, big_num: &BigNum) {
        let mut carry_forward = false;

        for idx in (0..4).rev() {
            let added_digit = {
                let sum = self.frac_val[idx] + big_num.frac_val[idx] + {
                    match carry_forward {
                        true => 1,
                        false => 0
                    }
                };

                if sum > 9 {
                    carry_forward = true;
                    sum % 10
                } else {
                    carry_forward = false;
                    sum
                }
            };
            
            self.frac_val[idx] = added_digit;
        }

        let mut new_int_vec: Vec<u8> = Vec::with_capacity(self.int_val.len().max(big_num.int_val.len()));

        let mut self_idx: usize = self.int_val.len();
        let mut big_num_idx: usize = big_num.int_val.len();

        loop {
            if self_idx == 0 && big_num_idx == 0 {
                break;
            }

            let d1 = {
                if self_idx != 0 {
                    self.int_val[self_idx - 1]
                } else {
                    0
                }
            };

            let d2 = {
                if big_num_idx != 0 {
                    big_num.int_val[big_num_idx - 1]
                } else {
                    0
                }
            };

            let added_digit = {
                let sum = d1 + d2 + {
                    match carry_forward {
                        true => 1,
                        false => 0
                    }
                };

                if sum > 9 {
                    carry_forward = true;
                    sum % 10
                } else {
                    carry_forward = false;
                    sum
                }
            };

            new_int_vec.push(added_digit);

            if self_idx != 0 {
                self_idx -= 1;
            }

            if big_num_idx != 0 {
                big_num_idx -= 1;
            }
        }

        if carry_forward {
            new_int_vec.push(1);
        }

        new_int_vec.reverse();
        self.int_val = new_int_vec;
    }

    pub fn substract(&mut self, big_num: &BigNum) {
        let mut is_neg: bool = false;
        #[allow(unused_variables)]
        let mut min_num_arr: Vec<u8> = Vec::default();
        let max_num_arr: Vec<u8> = {
            match self.int_val.len() > big_num.int_val.len() {
                true => {
                    min_num_arr = big_num.int_val.clone();
                    self.int_val.clone()
                },
                false => {
                    min_num_arr = self.int_val.clone();
                    is_neg = true;
                    big_num.int_val.clone()
                }
            }
        };

        let mut carry_forward: bool = false;
        let mut max_idx: usize = max_num_arr.len();
        let mut min_idx: usize = min_num_arr.len();

        let mut result: Vec<u8> = Vec::with_capacity(max_idx);

        loop {
            if max_idx == 0 {
                break;
            }

            let d1 = max_num_arr[max_idx - 1] - {
                match carry_forward {
                    true => 1,
                    false => 0
                }
            };

            let d2 = match min_idx {
                0 => 0,
                _ => min_num_arr[min_idx - 1]
            };

            let diff_digit = {

                if min_idx == 0 {
                    d1
                } else {
                    match d1 > d2 {
                        true => {
                            carry_forward = false;
                            d1 - d2
                        },
                        false =>{
                            carry_forward = true;
                            (d1 + 10) - d2
                        }
                    }
                }
            };

            result.push(diff_digit);

            max_idx -= 1;

            if min_idx > 0 {
                min_idx -= 1;
            }
        }
        
        result.reverse();
        self.int_val = result;
        
        match is_neg {
            true => self.sign = Sign::Negative,
            false => self.sign = Sign::Positive
        }
    }
}

#[cfg(test)]
mod tests {
    use super::BigNum;

    #[test]
    fn big_num_addition() {
        let mut b1 = BigNum::from_string("88.0025").unwrap();
        let b2 = BigNum::from_string("93.0025").unwrap();

        b1.add(&b2);
        assert_eq!(b1.int_val, BigNum::from_string("181.0050").unwrap().int_val);
    }

    // #[test]
    // fn big_num_substraction() {
    //     let mut b1 = BigNum::from_string("88.0000").unwrap();
    //     let b2 = BigNum::from_string("33.0000").unwrap();

    //     b1.substract(&b2);
    //     assert_eq!(vec![4, 5], b1.int_val);
    // }
}