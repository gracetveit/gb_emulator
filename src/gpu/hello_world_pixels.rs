pub struct HWLetter {
    pub data: [[u8; 8]; 8]
}

impl HWLetter {
    fn new(letter: char) -> Self{
        match letter {
            'h' => {
                HWLetter {
                    data: [
                        [0; 8],
                        [0, 1, 0, 0, 0, 1, 1, 0],
                        [0, 1, 0, 0, 0, 1, 1, 0],
                        [0, 1, 1, 1, 1, 1, 1, 0],
                        [0, 1, 0, 0, 0, 1, 1, 0],
                        [0, 1, 0, 0, 0, 1, 1, 0],
                        [0, 1, 0, 0, 0, 1, 1, 0],
                        [0; 8]
                    ]
                }
            }
            'e' => {
                HWLetter {
                    data: [
                        [0; 8],
                        [0, 1, 1, 1, 1, 1, 1, 0],
                        [0, 1, 1, 0, 0, 0, 0, 0],
                        [0, 1, 1, 1, 1, 1, 1, 0],
                        [0, 1, 1, 0, 0, 0, 0, 0],
                        [0, 1, 1, 0, 0, 0, 0, 0],
                        [0, 1, 1, 1, 1, 1, 1, 0],
                        [0; 8],
                    ]
                }
            }
            'l' => {
                HWLetter {
                    data: [
                        [0; 8],
                        [0, 1, 1, 0, 0, 0, 0, 0],
                        [0, 1, 1, 0, 0, 0, 0, 0],
                        [0, 1, 1, 0, 0, 0, 0, 0],
                        [0, 1, 1, 0, 0, 0, 0, 0],
                        [0, 1, 1, 0, 0, 0, 0, 0],
                        [0, 1, 1, 1, 1, 1, 1, 0],
                        [0; 8]
                    ]
                }
            }
            'o' => {
                HWLetter {
                    data: [
                        [0; 8],
                        [0, 0, 1, 1, 1, 1, 0, 0],
                        [0, 1, 1, 0, 0, 1, 1, 0],
                        [0, 1, 1, 0, 0, 1, 1, 0],
                        [0, 1, 1, 0, 0, 1, 1, 0],
                        [0, 1, 1, 0, 0, 1, 1, 0],
                        [0, 0, 1, 1, 1, 1, 0, 0],
                        [0; 8]
                    ]
                }
            }
            'w' => {
                HWLetter {
                    data: [
                        [0;8],
                        [0, 1, 0, 0, 0, 1, 1, 0],
                        [0, 1, 0, 0, 0, 1, 1, 0],
                        [0, 1, 0, 1, 0, 1, 1, 0],
                        [0, 1, 1, 1, 1, 1, 1, 0],
                        [0, 1, 1, 0, 1, 1, 1, 0],
                        [0, 1, 0, 0, 0, 1, 1, 0],
                        [0;8]
                    ]
                }
            }
            'r' => {
                HWLetter {
                    data: [
                        [0;8],
                        [0, 1, 1, 1, 1, 1, 0, 0],
                        [0, 1, 1, 0, 0, 1, 1, 0],
                        [0, 1, 1, 0, 0, 1, 1, 0],
                        [0, 1, 1, 1, 1, 1, 0, 0],
                        [0, 1, 1, 0, 1, 0, 0, 0],
                        [0, 1, 1, 0, 0, 1, 1, 0],
                        [0;8]
                    ]
                }
            }
            'd' => {
                HWLetter {
                    data: [
                        [0;8],
                        [0, 1, 1, 1, 1, 1, 0, 0],
                        [0, 1, 0, 0, 1, 1, 1, 0],
                        [0, 1, 0, 0, 1, 1, 1, 0],
                        [0, 1, 0, 0, 1, 1, 1, 0],
                        [0, 1, 0, 0, 1, 1, 1, 0],
                        [0, 1, 1, 1, 1, 1, 0, 0],
                        [0;8]
                    ]
                }
            }
            _ => {
                HWLetter {
                    data: [[0; 8]; 8]
                }
            }
        }
    }

    pub fn from_string(string: &str) -> Vec<HWLetter> {
        let mut letter_vec = Vec::new();
        for char in string.chars() {
            letter_vec.push(HWLetter::new(char));
        }
        letter_vec
    }
}