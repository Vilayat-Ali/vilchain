pub mod block;
pub mod chain;
pub mod txn;

use serde::{Deserialize, Serialize};
use std::fmt::Display;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct BigNum {
    int_val: Vec<u8>,
    frac_val: [u8; 4],
}

impl Default for BigNum {
    fn default() -> Self {
        Self {
            int_val: vec![0],
            frac_val: [0; 4]
        }
    }
}

impl Display for BigNum {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}.{}",
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
                0 => return Err("Invalid Big Number Format: The string passed is empty".to_string()),
                1 => {
                    let mut int_val_vec: Vec<u8> = Vec::with_capacity(num_arr[0].len());
                    for digit_char in num_arr[0].chars() {
                        match digit_char.to_string().parse::<u8>() {
                            Ok(num_val) => int_val_vec.push(num_val),
                            Err(_) => return Err("Invalid Big Number Format: The string contains non-numeric characters".to_string())
                        }
                    }
                    return Ok(Self {
                        int_val: int_val_vec,
                        frac_val: [0; 4]
                    });
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
        todo!()
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
        assert_eq!(2 + 2, 4);
    }
}