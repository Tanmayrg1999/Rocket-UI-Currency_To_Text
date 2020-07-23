#![allow(non_snake_case)] //for allowing snake case
#![allow(unused)] //for allowing unused variables or functions
use std::collections::HashMap;
use std::i32; //library fir i32
use std::io; //library for standard input output //library   for Hashmap

pub fn Bengoli(digits: u32) -> String {
    let mut text: HashMap<u32, &str> = HashMap::new();
    text.insert(0, "");
    text.insert(1, "এক ");
    text.insert(2, "দুই ");
    text.insert(3, "তিন ");
    text.insert(4, "চার ");
    text.insert(5, "পাঁচ ");
    text.insert(6, "ছয় ");
    text.insert(7, "সাত ");
    text.insert(8, "আট ");
    text.insert(9, "নয় ");
    text.insert(10, "দশ ");
    text.insert(11, "এগারো ");
    text.insert(12, "বারো ");
    text.insert(13, "তেরো ");
    text.insert(14, "চোদ্দো ");
    text.insert(15, "পনেরো ");
    text.insert(16, "ষোলো ");
    text.insert(17, "সতেরো ");
    text.insert(18, "আঠারো ");
    text.insert(19, "উনিশ ");
    text.insert(20, "বিশ ");
    text.insert(21, "একুশ ");
    text.insert(22, "বাইশ ");
    text.insert(23, "তেইশ ");
    text.insert(24, "চব্বিশ ");
    text.insert(25, "পঁচিশ ");
    text.insert(26, "ছাব্বিশ ");
    text.insert(27, "সাতাশ ");
    text.insert(28, "আটাশ");
    text.insert(29, "ঊনত্রিশ");
    text.insert(30, "ত্রিশ");
    text.insert(31, "একত্রিশ ");
    text.insert(32, "বত্রিশ ");
    text.insert(33, "তেত্রিশ ");
    text.insert(34, "চৌত্রিশ ");
    text.insert(35, "পৈন্তীরিশ ");
    text.insert(36, "ছোতিরিশ ");
    text.insert(37, "সাইত্রিশ ");
    text.insert(38, "আটত্রিশ  ");
    text.insert(39, "ঊনচল্লিশ ");
    text.insert(40, "চল্লিশ ");
    text.insert(41, "একচল্লিশ ");
    text.insert(42, "বিয়াল্লিশ ");
    text.insert(43, "তেতাল্লিশ ");
    text.insert(44, "চুয়াল্লিশ ");
    text.insert(45, "পঁয়তাল্লিশ ");
    text.insert(46, "ছিচল্লিশ ");
    text.insert(47, "সাতচল্লিশ ");
    text.insert(48, "আটচল্লিশ ");
    text.insert(49, "ঊনপঞ্চাশ ");
    text.insert(50, "পঞ্চাশ ");
    text.insert(51, "একান্নো ");
    text.insert(52, "বাহান্নো ");
    text.insert(53, "তেপ্পান্নো ");
    text.insert(54, "চুযান্নো ");
    text.insert(55, "পঞ্চান্নো ");
    text.insert(56, "ছাপ্পান্নো ");
    text.insert(57, "সাতান্নো ");
    text.insert(58, "আটান্নো ");
    text.insert(59, "ঊনষাট ");
    text.insert(60, "ষাট ");
    text.insert(61, "একষট্টি ");
    text.insert(62, "বাষট্টি ");
    text.insert(63, "তেষট্টি");
    text.insert(64, "চৌষট্টি");
    text.insert(65, "পৈন্ষট্টি ");
    text.insert(66, "ছেষট্টি ");
    text.insert(67, "সাতষট্টি ");
    text.insert(68, "আটষট্টি ");
    text.insert(69, "উনসত্তর ");
    text.insert(70, "সত্তর ");
    text.insert(71, "একাত্তর ");
    text.insert(72, "বাহাত্তর ");
    text.insert(73, "তেহাত্তর ");
    text.insert(74, "চহত্তর ");
    text.insert(75, "পচাত্তর ");
    text.insert(76, "ছিয়াত্তর ");
    text.insert(77, "সাতাত্তর ");
    text.insert(78, "আটাত্তর ");
    text.insert(79, "উনাশি ");
    text.insert(80, "আশি ");
    text.insert(81, "একাশি ");
    text.insert(82, "বিরাশি ");
    text.insert(83, "তিরাশি ");
    text.insert(84, "চুরাশি ");
    text.insert(85, "পঁচাশি ");
    text.insert(86, "ছিয়াশি ");
    text.insert(87, "সাতাশি ");
    text.insert(88, "আটাশি ");
    text.insert(89, "ঊনোনব্বই ");
    text.insert(90, "নব্বুই ");
    text.insert(91, "একানব্বই ");
    text.insert(92, "বিরানব্বই ");
    text.insert(93, "তিরানব্বই ");
    text.insert(94, "চুরানব্বই ");
    text.insert(95, "পঁচানব্বই ");
    text.insert(96, "ছিয়ানব্বই ");
    text.insert(97, "সাতানব্বই ");
    text.insert(98, "আটানব্বই ");
    text.insert(99, "নিরানব্বই ");
    let mut wordstr = String::new();
    wordstr.push_str(text.get(&digits).unwrap());
    return wordstr;
}

pub fn BengoliWords(int_currency: u32, mut Complete_string: String) -> String {
    let mut string = " ".to_string();
    let mut x = int_currency;
    let mut currency_length = 0;
    while x != 0
    //Finding the length of the integer value
    {
        x /= 10;
        currency_length += 1;
    }
    let mut lengthcopy = currency_length;
    if currency_length > 9 {
        string =
            ["Number should have less than 9 digits before decimal point".to_string()].join(" ");
             Complete_string=["".to_string()].join("");
    }
    if (int_currency) == 0 {
        string = ["શૂન્ય".to_string()].join("");
    }
    while currency_length != 0
    //spilts the digits from number
    {
        let mut lengthpow = u32::pow(10, currency_length - 1);
        let mut p = int_currency / lengthpow;
        let mut digits = p % 10;
        lengthpow = lengthpow / 10;
        currency_length -= 1;
        let mut digitcopy = digits;
        //let mut digitcopy2: u32 = 0;
        if (currency_length == 1
            || currency_length == 4
            || currency_length == 6
            || currency_length == 8)
            && digits != 0
        //digits!=0 for 101 number bcaz if we dont take it it will simply print hundred
        {
            p = int_currency / lengthpow;
            digits = digits * 10 + p % 10;
            lengthpow = lengthpow / 10;
            currency_length -= 1;

            /*if digits==0
            {
                currency_length=0;
            }*/
        } //println!("{}",digits);
        if digitcopy == 0
        //if dont use this condition it will give result like 100001 One  Lakh  One Thousand
        {
            lengthcopy -= 1;
        }
        match lengthcopy {
            1..=2 => {
                if digitcopy != 0
                //digitcopy!=0   For 100,1000,10000 values
                {
                    if lengthcopy == 2
                    //for 2 digit numbers
                    {
                        string = [
                            string,
                            // Marathi(digitcopy2).to_string(),
                            Bengoli(digits).to_string(),
                        ]
                        .join(" ");
                        lengthcopy -= 2
                    } else {
                        string = [string, Bengoli(digits).to_string()].join(" ");
                        lengthcopy -= 1
                    } //for single digit numbers
                }
            }
            3 => {
                if digitcopy != 0 {
                    string = [string, Bengoli(digits).to_string(), "শো".to_string()].join(" ");
                    lengthcopy -= 1
                }
            } //for 3 digit numbers
            4..=5 => {
                if digitcopy != 0 {
                    if lengthcopy == 5
                    //for 4 digit numbers
                    {
                        string = [
                            string,
                            //Marathi(digitcopy2).to_string(),
                            Bengoli(digits).to_string(),
                            "হাজার ".to_string(),
                        ]
                        .join(" ");
                        lengthcopy -= 2
                    } else {
                        string =
                            [string, Bengoli(digits).to_string(), "হাজার ".to_string()].join(" ");
                        lengthcopy -= 1
                    }
                }
            } //for 5 digit numbers
            6..=7 => {
                if digitcopy != 0 {
                    if lengthcopy == 7
                    //for 7 digit numbers
                    {
                        string = [
                            string,
                            //Marathi(digitcopy2).to_string(),
                            Bengoli(digits).to_string(),
                            "লাখ".to_string(),
                        ]
                        .join(" ");
                        lengthcopy -= 2
                    } else {
                        string = [string, Bengoli(digits).to_string(), "লাখ".to_string()].join(" ");
                        lengthcopy -= 1
                    }
                }
            } //for 6 digit numbers
            8..=9 => {
                if digitcopy != 0 {
                    if lengthcopy == 9
                    //for 8 digit numbers
                    {
                        string = [
                            string,
                            // Marathi(digitcopy2).to_string(),
                            Bengoli(digits).to_string(),
                            "কোটি ".to_string(),
                        ]
                        .join(" ");
                        lengthcopy -= 2
                    } else {
                        string =
                            [string, Bengoli(digits).to_string(), "কোটি ".to_string()].join(" ");
                        lengthcopy -= 1
                    }
                }
            } //for 9 digit numbers
            _ => print!(""),
        }
    }
    string = [string, Complete_string.to_string()].join(" ");
    //println!("{}",string);
    return string;
}
