use std::io::{self};

use clap::{Arg, Command};

fn main() {
    // Создаем флаги, используемые в программе, при помощи библиотеки "clap"
    let matches = Command::new("My Program")
        .version("1.0")
        .about("cut analog")
        .arg(
            Arg::new("fields")
                .short('f')
                .help("select only these fields")
                .required(true) // флаг является обязательным
                .value_parser(clap::builder::NonEmptyStringValueParser::new()), // после флага должно быть указано значение, являющееся непустой строкой
        )
        .arg(
            Arg::new("delimiter")
                .short('d')
                .help("use DELIM instead of TAB for field delimiter")
                .value_parser(clap::builder::NonEmptyStringValueParser::new()), //
        )
        .arg(
            Arg::new("separated")
                .short('s')
                .help("do not print lines not containing delimiters")
                .action(clap::ArgAction::SetTrue), // .required(false),
        )
        .get_matches();

    let fields_str = matches.get_one::<String>("fields").unwrap(); // получем значения колонок которые нужно вывести в виде строки

    // Получаем разделитель, если флаг отсутствует - устанавливаем TAB
    let delimiter = match matches.contains_id("delimiter") {
        true => matches.get_one::<String>("delimiter").unwrap(),
        false => &'\t'.to_string(),
    };
    println!("fields: {}", fields_str);

    println!("delimeter: {}", delimiter);
    // Парсим значение указанное при флаге -f, получаем массив с номерами колонок
    let fields = match validate_input(fields_str) {
        Ok(fields) => fields,
        Err(error) => {
            println!("{}", error);
            return;
        }
    };
    println!("{:?}", fields);
    println!("{}", fields.len());

    let mut input = String::new(); // здесь храним ввод

    loop {
        input.clear();
        io::stdin().read_line(&mut input).unwrap();
        let mut count: u32 = 1;

        if matches.get_flag("separated") {
            // если установлен флаг -s
            if !input.contains(delimiter) {
                // если данная строка не содержит разделителя - пропускаем ее
                continue;
            }
        }

        // обрабатываем случай когда в качестве номеров колонок указан дипазон чисел с N до конца строки
        if fields[0] == 0 {
            // разбиваем строку по разделителю и проходим по каждой подстроке
            for substr in input.split_inclusive(delimiter) {
                // если номер подстроки больше или равен началу диапазона - выводим
                if count >= fields[1] {
                    print!("{}", substr);
                }
                count += 1;
            }
        } else {
            // все остальные варианты
            for substr in input.split_inclusive(delimiter) {
                if fields.contains(&count) {
                    // если номер колонки содержится в массиве - выводим
                    print!("{}", substr);
                }
                count += 1;
            }
        }

        print!("\n");
    }
}

// Функция для обработки аргумента указанного при флаге "-f"
fn validate_input(val: &str) -> Result<Vec<u32>, &str> {
    let mut res = vec![];

    // Если указанная строка парсится в число - добавляем его в массив и возвращаем в качестве результата
    if let Ok(value) = val.parse::<u32>() {
        res.push(value);
        return Ok(res);
    }
    // Если указанная строка разбивается по разделителю: '-'
    if let Some((start, end)) = val.split_once('-') {
        // Если начало и конец разбиения пустые строки - указан только символ '-', некорректный случай
        if start == "" && end == "" {
            return Err("не является допустимым форматом (число, диапазон или перечисление)");
        }
        //Если начало пустая строка - указан диапазон от начала строки до N
        else if start == "" {
            if let Ok(end) = end.parse::<u32>() {
                // если конец парсится в число - диапазон указан верно, формируем массив от 1 до N, помещаем в результат
                for i in 1..end {
                    res.push(i);
                }
                return Ok(res);
            }
        }
        //Если конец пустая строка - указан диапазон от N до конца строки
        else if end == "" {
            if let Ok(start) = start.parse::<u32>() {
                // если начало парсится в число - диапазон указан верно, в качестве первого элемента помещаем 0, второй элемент - начало диапазона. Будем обрабатывать этот случай отдельно
                res.push(0);
                res.push(start);
                return Ok(res);
            }
        }
        // Если конец и начало не пустые строки - проверяем, парсятся ли начало и конец в числа, если да то формируем массив
        else {
            if let Ok(start) = start.parse::<u32>() {
                if let Ok(end) = end.parse::<u32>() {
                    if start <= end {
                        for i in start..=end {
                            res.push(i);
                        }
                        return Ok(res);
                    } else {
                        return Err("invalid decreasing range"); // обрабатываем ошибку если начало больше конца
                    }
                }
            }
        }
    }

    // Если указано не одно число и не диапазон, проверяем что указано перечисление
    if let Ok(result) = val
        .split(',') //разбиваем строку по разделителю ','
        .map(|num| num.parse::<u32>()) // проверяем что все подстроки представляют собой числа
        .collect::<Result<Vec<u32>, _>>()
    // собираем в вектор
    {
        return Ok(result);
    }

    // В других случаях выводим ошибку
    Err("не является допустимым форматом (число, диапазон или перечисление)")
}
